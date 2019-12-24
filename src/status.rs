use std::collections::HashMap;

/// Repressents what is returned by wpa_supplicant's STATUS command.
#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    bssid: String,
    freq: i32,
    ssid: String,
    id: String,
    mode: String,
    pairwise_cipher: String,
    group_cipher: String,
    key_mgmt: String,
    wpa_state: String,
    ip_address: String,
    address: String,
    uuid: String,
}

impl Status {
    /// Parses the text returned by wpa_supplicant's STATUS command.
    pub fn parse(status_str: String) -> Option<Self> {
        let map: HashMap<String, String> = Self::parse_hashmap(status_str);
        let freq: i32;

        if !map.contains_key("bssid") {
            return None;
        }

        if !map.contains_key("freq") {
            return None;
        } else {
            freq = map.get("freq").unwrap().parse().unwrap_or(0);
        }

        Some(Status {
            bssid: map.get("bssid").unwrap().clone(),
            freq,
            ssid: map.get("ssid").unwrap().clone(),
            id: map.get("id").unwrap().clone(),
            mode: map.get("mode").unwrap().clone(),
            pairwise_cipher: map.get("pairwise_cipher").unwrap().clone(),
            group_cipher: map.get("group_cipher").unwrap().clone(),
            key_mgmt: map.get("key_mgmt").unwrap().clone(),
            wpa_state: map.get("wpa_state").unwrap().clone(),
            ip_address: map.get("ip_address").unwrap().clone(),
            address: map.get("address").unwrap().clone(),
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
