use sysinfo::Networks;

/// 集計から除外するインターフェース名。
///
/// loopback は外部との通信量を表さないため、監視対象から外す。
const EXCLUDED_INTERFACES: [&str; 1] = ["lo"];

/// 直近の `refresh` 以降に、loopback を除く全NICが送受信した合計バイト数を返す。
///
/// sysinfo の `received()`/`transmitted()` は「前回の `refresh` からの差分」を返す。
/// したがって呼び出し側は、その差分を区間の経過時間で割ってレートに直す必要がある。
pub fn get_network_bytes(networks: &Networks) -> u64 {
    networks
        .iter()
        .filter(|(name, _)| !EXCLUDED_INTERFACES.contains(&name.as_str()))
        .map(|(_, data)| data.received() + data.transmitted())
        .sum()
}
