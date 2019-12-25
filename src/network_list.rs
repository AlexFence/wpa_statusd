/// This module is meant to transform the output of the wpa_supplicant's
/// LIST_NETWORKS command into a serializable format.

#[derive(Serialize, Deserialize, Debug)]
pub struct Network {
    id: i16,
    ssid: String,
    bssid: String,
    flags: String,
}

pub fn parse_list(network_list: String) -> Vec<Network> {
    let mut networks: Vec<Network> = Vec::new();
    let mut lines = network_list.lines();

    // Throw away the first line, its just a header
    // network id / ssid / bssid / flag
    lines.next();

    for line in lines {
        let mut tabs = line.split("\t");

        // count consumes the iterator
        // moving the value whihc is why we clone it first
        let tab_count = tabs.clone().count();
        if tab_count == 4 {
            let id_str = &tabs.next().unwrap();
            let id = id_str.parse();

            if id.is_ok() {
                let network = Network {
                    id: id.unwrap(),
                    ssid: tabs.next().unwrap().to_string(),
                    bssid: tabs.next().unwrap().to_string(),
                    flags: tabs.next().unwrap().to_string(),
                };

                networks.push(network)
            } else {
                error!("could not parse id of network: {}", line);
            }
        } else {
            error!("network netry does not have 4 tabs: {}", line);
        }
    }

    networks
}
