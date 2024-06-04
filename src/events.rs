use std::future::Future;
use serde_derive::{Serialize, Deserialize};
use serde_repr::*;
use tokio::net::UdpSocket;
use tracing::debug;
use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct UlContact {
    pub domain: String,
    pub aor: String,
    pub uri: String,
    #[serde(default)]
    pub received: String,
    #[serde(default)]
    pub path: Option<String>,
    pub qval: i64,
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
    pub partition: String,
    pub group: String,
    pub address: String,
    pub status: DispatcherState,
}

#[derive(Copy, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ClusterNodeState {
    Down = 0,
    Up = 1,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClustererNodeStateChange {
    pub cluster_id: usize,
    pub node_id: usize,
    pub new_state: ClusterNodeState,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum UAEventType {
    #[default]
    New,
    Early,
    Answered,
    Rejected,
    Updated,
    Terminated,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UASession {
    pub key: String,
    pub entity_type: String,
    pub event_type: UAEventType,
    pub status: usize,
    pub reason: String,
    pub method: String,
    pub body: String,
    pub headers: String,
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
    EUaSession(UASession),
}


pub struct UdpNotificationReceiver {
    pub socket: UdpSocket,
}

impl UdpNotificationReceiver {
    pub async fn run<F>(self, f: impl Fn(Notification) -> F) -> tokio::io::Result<()>
      where F: Future<Output = ()>
    {
        let UdpNotificationReceiver { socket, .. } = self;
        let mut buf = vec![0u8; 65536];

        debug!("starting UDP receiver loop");
        loop {
            let (size, source) = socket
                .recv_from(&mut buf)
                .await?;
            debug!("received packet {} bytes from {:?}", size, source);
            let decode_result = serde_json::from_slice(&buf[0..size]);
            match decode_result {
                Ok(notification) => {
                    debug!("received: {:?}", notification);
                    f(notification).await;
                }
                Err(e) => debug!("error decoding: {:?}", e)
            }
        }
    }
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


