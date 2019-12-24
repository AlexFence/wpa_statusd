use std::collections::HashMap;

fn convert_option(option: Option<&String>) -> Option<String> {
    match option {
        Some(value) => Some(value.clone()),
        None => None,
    }
}

/// Repressents what is returned by wpa_supplicant's STATUS command.
#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    bssid: Option<String>,
    freq: Option<i32>,
    ssid: Option<String>,
    id: Option<String>,
    mode: Option<String>,
    pairwise_cipher: Option<String>,
    group_cipher: Option<String>,
    key_mgmt: Option<String>,
    wpa_state: Option<String>,
    ip_address: Option<String>,
    address: Option<String>,
    uuid: String,
}

impl Status {
    /// Parses the text returned by wpa_supplicant's STATUS command.
    pub fn parse(status_str: String) -> Option<Self> {
        let map: HashMap<String, String> = Self::parse_hashmap(status_str);
        let freq: Option<i32>;

        if !map.contains_key("uuid") {
            return None;
        }

        freq = match map.get("freq") {
            Some(value) => Some(value.parse().unwrap_or(0)),
            None => None,
        };

        Some(Status {
            bssid: convert_option(map.get("bssid")),
            freq,
            ssid: convert_option(map.get("ssid")),
            id: convert_option(map.get("id")),
            mode: convert_option(map.get("mode")),
            pairwise_cipher: convert_option(map.get("pairwise_cipher")),
            group_cipher: convert_option(map.get("group_cipher")),
            key_mgmt: convert_option(map.get("key_mgmt")),
            wpa_state: convert_option(map.get("wpa_state")),
            ip_address: convert_option(map.get("ip_address")),
            address: convert_option(map.get("address")),
            uuid: map.get("uuid").unwrap().clone(),
        })
    }

    fn parse_hashmap(string: String) -> HashMap<String, String> {
        let mut map: HashMap<String, String> = HashMap::new();
        let lines = string.split("\n");

        for line in lines {
            if line != String::from("") {
                let mut pair = line.split("=");
                // we should always have at least one string
                // the unwrap should be safe, maybe
                let key = pair.next().unwrap();
                let value = pair.next().unwrap_or("");

                map.insert(String::from(key), String::from(value));
            }
        }

        map
    }
}
