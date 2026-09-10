use clap::Parser;
use log::{error, info};
use sysinfo::{Disks, Networks, System};

mod logging;
mod metrics;
mod signal;

const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), " (", env!("RWATCH_COMMIT"), ")");

#[derive(Parser)]
#[command(
    name = "rwatch",
    version = VERSION,
)]
struct Cli {
    /// Run once and exit
    #[arg(long)]
    once: bool,
}

fn main() {
    let cli = Cli::parse();

    logging::init_logger();

    let mut server_logger = match logging::ServerLogger::new("server.jsonl") {
        Ok(logger) => logger,
        Err(error) => {
            error!("Failed to create server logger: {error}");
            return;
        }
    };

    const NTP_SERVERS: [&str; 2] = ["169.254.169.254:123", "ntp.nict.jp:123"];

    let mut sys = System::new();
    let mut disks = Disks::new_with_refreshed_list();
    let mut network = Networks::new_with_refreshed_list();

    // ネットワークの計測区間の起点。初回は起点が無いため記録しない (null)。
    let mut last_refresh: Option<std::time::Instant> = None;

    let ntp_time = match metrics::time::NtpClock::new(&NTP_SERVERS) {
        Some(ntp_time) => ntp_time,
        None => {
            error!("Failed to get NTP time");
            return;
        }
    };

    let shutdown = signal::setup_signal_handlers();

    loop {
        let cpu_usage: f64 = metrics::cpu::get_cpu_usage(&mut sys);
        let available_memory_percentage = metrics::memory::get_memory_usage(&mut sys);
        let disk_usage = metrics::disk::get_disk_usage(&mut disks);

        // sysinfo の received()/transmitted() は「前回 refresh からの差分バイト数」。
        // 区間は refresh 間で測る。待機時間だけで割ると cpu/mem/disk の収集時間が
        // 区間から漏れ、Mbps が約 4% 過大になる。
        network.refresh(true);
        let refreshed_at = std::time::Instant::now();
        let network_bytes = metrics::network::get_network_bytes(&network);
        let network_mbps = last_refresh.map(|last_refresh| {
            let elapsed_seconds = refreshed_at.duration_since(last_refresh).as_secs_f64();
            network_bytes as f64 * 8.0 / 1_000_000.0 / elapsed_seconds.max(f64::EPSILON)
        });
        last_refresh = Some(refreshed_at);

        let network_display = match network_mbps {
            Some(network_mbps) => format!("{network_mbps:.3} Mbps"),
            None => "n/a (first sample)".to_string(),
        };

        let timestamp = ntp_time.now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();

        info!(
            "[{}] CPU: {:.2}%, Memory available: {:.2}%, Disk: {:.2}%, Network: {network_display}",
            timestamp, cpu_usage, available_memory_percentage, disk_usage,
        );

        let network_field = match network_mbps {
            Some(network_mbps) => format!("\"network_mbps\": {network_mbps:.3}"),
            None => "\"network_mbps\": null".to_string(),
        };

        let record = format!(
            "{{\"timestamp\": \"{}\", \"cpu_usage\": {:.2}, \
            \"available_memory_percentage\": {:.2}, \
            \"disk_usage\": {:.2}, {network_field}}}",
            timestamp, cpu_usage, available_memory_percentage, disk_usage,
        );

        if let Err(error) = server_logger.append(&record) {
            error!("Failed to append to server log: {error}");
        }

        if cli.once {
            break;
        }

        let (lock, cvar) = &*shutdown.condvar;
        let mut shutdown_requested = lock.lock().unwrap();

        if !*shutdown_requested {
            let (guard, _) =
                cvar.wait_timeout(shutdown_requested, std::time::Duration::from_secs(5)).unwrap();

            shutdown_requested = guard;
        }

        if *shutdown_requested {
            info!("Graceful shutdown completed");
            break;
        }
    }
}
