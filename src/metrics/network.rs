use sysinfo::Networks;

pub fn get_network_usage(networks: &Networks) -> u64 {
    networks
    .iter()
    .map(|(_,network)| network.received()+ network.transmitted())
    .sum()
}