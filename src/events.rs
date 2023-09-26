use serde_derive::{Serialize, Deserialize};
use serde_repr::*;
use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct UlContact {
    pub domain: String,
    pub aor: String,
    pub uri: String,
    #[serde(default)]
    pub received: String,
    #[serde(default)]
    pub path: String,
    pub qval: u64,
    pub user_agent: String,
    pub socket: String,
    pub bflags: i64,
    pub expires: u64,
    pub callid: String,
    pub cseq: u64,
    pub attr: String,
    pub latency: i64,
    pub shtag: String,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DialogChange {
    pub id: String,
    #[serde(rename = "callid")]
    pub call_id: String,
    pub from_tag: String,
    #[serde(default)]
    pub to_tag: String,
    pub old_state: DialogState,
    pub new_state: DialogState,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="snake_case")]
pub enum DispatcherState {
    #[default]
    Active,
    Inactive,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DispatcherStatus {
    partition: String,
    group: String,
    address: String,
    status: DispatcherState,
}

#[derive(Copy, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ClusterNodeState {
    Down = 0,
    Up = 1,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClustererNodeStateChange {
    cluster_id: usize,
    node_id: usize,
    new_state: ClusterNodeState,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method", content = "params", rename_all="SCREAMING_SNAKE_CASE")]
pub enum Notification {
    EUlContactInsert(UlContact),
    EUlContactDelete(UlContact),
    EUlContactUpdate(UlContact),
    EDlgStateChanged(DialogChange),
    EDispatcherStatus(DispatcherStatus),
    EClustererNodeStateChange(ClustererNodeStateChange),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_destination_decode() {
        let input = r#"[
        {"jsonrpc":"2.0","method":"E_UL_CONTACT_UPDATE","params":{"domain":"location","aor":"test@192.168.20.21","uri":"sip:40936782@192.168.10.179:57028","received":"sip:192.168.10.179:57028","path":null,"qval":-1,"user_agent":"Blink 8.9.4 (MacOSX)","socket":"","bflags":0,"expires":1695059926,"callid":"4VWWnqsOg9TIRYSyFN.08yXb-EvVtTP8","cseq":10,"attr":"","latency":0,"shtag":""}},
        {"jsonrpc":"2.0","method":"E_UL_CONTACT_DELETE","params":{"domain":"location","aor":"test@192.168.20.21","uri":"sip:40936782@192.168.10.179:57028","received":"sip:192.168.10.179:57028","path":null,"qval":-1,"user_agent":"Blink 8.9.4 (MacOSX)","socket":"","bflags":0,"expires":1695054721,"callid":"J76aTtBC290Y5VRin97hnIJqwG4Oeb-2","cseq":1,"attr":"","latency":0,"shtag":""}},
        {"jsonrpc":"2.0","method":"E_UL_CONTACT_INSERT","params":{"domain":"location","aor":"test@192.168.20.21","uri":"sip:40936782@192.168.10.179:57028","received":"sip:192.168.10.179:57028","path":null,"qval":-1,"user_agent":"Blink 8.9.4 (MacOSX)","socket":"udp:192.168.20.21:5060","bflags":0,"expires":1695054721,"callid":"4VWWnqsOg9TIRYSyFN.08yXb-EvVtTP8","cseq":1,"attr":"","latency":0,"shtag":""}}
        ]"#;
        let json: Vec<Notification> = serde_json::from_str(input).unwrap();
        println!("{:#?}", json);
    }
}


