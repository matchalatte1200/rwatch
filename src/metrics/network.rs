use std::time::Instant;

use sysinfo::Networks;

pub struct NetworkMetrics {
    previous_bytes: u64,
    previous_time: Instant,
}

impl NetworkMetrics {
    pub fn new(networks: &Networks) -> Self {
        Self { previous_bytes: get_network_usage(networks), previous_time: Instant::now() }
    }

    pub fn get_mbps(&mut self, networks: &Networks) -> f64 {
        let now = Instant::now();
        let current_bytes = get_network_usage(networks);

        let elapsed = now.duration_since(self.previous_time).as_secs_f64();
        let bytes = current_bytes.saturating_sub(self.previous_bytes);

        self.previous_bytes = current_bytes;
        self.previous_time = now;

        if elapsed <= 0.0 {
            return 0.0;
        }

        bytes as f64 * 8.0 / elapsed / 1_000_000.0
    }
}

pub fn get_network_usage(networks: &Networks) -> u64 {
    networks
        .iter()
        .filter(|(name, _)| name.as_str() != "lo")
        .map(|(_, network)| network.received() + network.transmitted())
        .sum()
}

pub fn format_network_mbps(mbps: f64) -> String {
    if mbps == 0.0 {
        "0 bps".to_string()
    } else if mbps >= 1000.0 {
        format!("{:.2} Gbps", mbps / 1000.0)
    } else if mbps >= 1.0 {
        format!("{:.2} Mbps", mbps)
    } else if mbps >= 0.001 {
        format!("{:.2} Kbps", mbps * 1000.0)
    } else {
        format!("{:.2} bps", mbps * 1_000_000.0)
    }
}
