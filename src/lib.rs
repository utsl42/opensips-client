use jsonrpsee::proc_macros::rpc;
use serde_derive::{Serialize, Deserialize};
use serde_repr::*;

pub mod events;
pub use events::*;

#[rpc(client)]
pub trait OpenSIPS {
    /// core methods
    #[method(name="version",param_kind=map)]
    fn version(&self) -> Result<VersionResponse, Error>;

    // log_level has 3 variants:
    #[method(name="log_level",param_kind=map)]
    fn get_log_level(&self) -> Result<LogLevelResponse, Error>;
    #[method(name="log_level",param_kind=map)]
    fn set_log_level(&self, level: LogLevel) -> Result<LogLevelResponse, Error>;
    #[method(name="log_level",param_kind=map)]
    fn set_log_level_pid(&self, level: LogLevel, pid: usize) -> Result<LogLevelResponse, Error>;

    // xlog_level has 2 variants:
    #[method(name="xlog_level",param_kind=map)]
    fn get_xlog_level(&self) -> Result<XLogLevelResponse, Error>;
    #[method(name="xlog_level",param_kind=map)]
    fn set_xlog_level(&self, level: LogLevel) -> Result<XLogLevelResponse, Error>;

    #[method(name="reload_routes")]
    fn reload_routes(&self) -> Result<String, Error>;

    // cache
    #[method(name="cache_fetch",param_kind=map)]
    fn cache_fetch(&self, system: String, attr: String) -> Result<CacheResponse, Error>;
    #[method(name="cache_store",param_kind=map)]
    fn cache_store(&self, system: String, attr: String, value: String) -> Result<String, Error>;
    #[method(name="cache_store",param_kind=map)]
    fn cache_store_expires(&self, system: String, attr: String, value: String, expires: usize) -> Result<String, Error>;
    #[method(name="cache_remove",param_kind=map)]
    fn cache_remove(&self, system: String, attr: String) -> Result<String, Error>;

    #[method(name="events_list",param_kind=map)]
    fn events_list(&self) -> Result<EventsListResponse, Error>;
    #[method(name="event_subscribe",param_kind=map)]
    fn event_subscribe(&self, event: String, socket: String) -> Result<String, Error>;
    #[method(name="event_subscribe",param_kind=map)]
    fn event_subscribe_expire(&self, event: String, socket: String, expire: usize) -> Result<String, Error>;

    // dispatcher methods
    #[method(name="ds_reload",param_kind=map)]
    fn ds_reload(&self) -> Result<String, Error>;
    #[method(name="ds_list",param_kind=map)]
    fn ds_list(&self, full: usize) -> Result<dispatcher::ListResponse, Error>;

    // clusterer methods
    #[method(name="clusterer_list",param_kind=map)]
    fn clusterer_list(&self) -> Result<clusterer::ClustererListResponse, Error>;
    #[method(name="clusterer_list_shtags",param_kind=map)]
    fn clusterer_list_shtags(&self) -> Result<Vec<clusterer::SharedTagStatus>, Error>;
    #[method(name="clusterer_shtag_set_active",param_kind=map)]
    fn clusterer_shtag_set_active(&self, tag: String) -> Result<String, Error>;

    // usrloc methods
    #[method(name="ul_dump",param_kind=map)]
    fn ul_dump(&self) -> Result<usrloc::DumpResponse, Error>;
    #[method(name="ul_rm",param_kind=map)]
    fn ul_rm(&self, table_name: String, aor: String) -> Result<String, Error>;
    #[method(name="ul_rm_contact",param_kind=map)]
    fn ul_rm_contact(&self, table_name: String, aor: String, contact: String) -> Result<String, Error>;
    #[method(name="ul_flush")]
    fn ul_flush(&self) -> Result<String, Error>;
    #[method(name="ul_cluster_sync")]
    fn ul_cluster_sync(&self) -> Result<String, Error>;

    // tm methods
    #[method(name="t_uac_dlg",param_kind=map)]
    fn t_uac_dlg(&self, method: String, ruri: String, headers: String, next_hop: String, socket: String) -> Result<TUacDlgResponse, Error>;
    #[method(name="t_uac_dlg",param_kind=map)]
    fn t_uac_dlg_with_body(&self, method: String, ruri: String, headers: String, next_hop: String, socket: String, body: String) -> Result<TUacDlgResponse, Error>;

    // uac_registrant methods
    #[method(name="reg_list",param_kind=map)]
    fn reg_list(&self) -> Result<RegListResponse, Error>;
    #[method(name="reg_list",param_kind=map)]
    fn reg_list_record(&self, aor: String, contact: String, registrar: String) -> Result<RegListRecordResponse, Error>;
    #[method(name="reg_reload",param_kind=map)]
    fn reg_reload(&self) -> Result<String, Error>;
    #[method(name="reg_reload",param_kind=map)]
    fn reg_reload_record(&self, aor: String, contact: String, registrar: String) -> Result<String, Error>;
    #[method(name="reg_enable",param_kind=map)]
    fn reg_enable(&self, aor: String, contact: String, registrar: String) -> Result<String, Error>;
    #[method(name="reg_disable",param_kind=map)]
    fn reg_disable(&self, aor: String, contact: String, registrar: String) -> Result<String, Error>;

    // dialog module
    #[method(name="dlg_list",param_kind=map)]
    fn dlg_list(&self) -> Result<dialog::ListResponse, Error>;
    #[method(name="dlg_list",param_kind=map)]
    fn dlg_list_record(&self, callid: String, from_tag: String) -> Result<dialog::ListRecordResponse, Error>;

    // b2b_entities module
    #[method(name="b2be_list")]
    fn b2be_list(&self) -> Result<String, Error>;
    // OpenSIPS doesn't appear to accept `null` for optional parameters, which is what Option would send, so leaving
    // most of the optional types in every method for now.
    #[method(name="ua_session_client_start",param_kind=map)]
    fn ua_session_client_start(&self, ruri: String, to: String, from: String, proxy: String, body: String, extra_headers: Vec<String>, content_type: String, flags: String) -> Result<String, Error>;
    #[method(name="ua_session_reply",param_kind=map)]
    fn ua_session_reply(&self, key: String, method: String, code: usize, reason: String, body: String, extra_headers: Vec<String>, content_type: String) -> Result<String, Error>;
    #[method(name="ua_session_update",param_kind=map)]
    fn ua_session_update(&self, key: String, method: String, body: String, extra_headers: Vec<String>, content_type: String) -> Result<String, Error>;
    #[method(name="ua_session_terminate",param_kind=map)]
    fn ua_session_terminate(&self, key: String, extra_headers: Vec<String>) -> Result<String, Error>;
    #[method(name="ua_session_list",param_kind=map)]
    fn ua_session_list(&self) -> Result<Vec<b2b_entities::UASession>, Error>;
    #[method(name="ua_session_list",param_kind=map)]
    fn ua_session_list_with_key(&self, key: String) -> Result<b2b_entities::UASession, Error>;

}

#[derive(Debug, Deserialize, Serialize)]
pub struct VersionResponse {
     #[serde(rename = "Server")]
     pub server: String,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(i8)]
pub enum LogLevel {
    Alert = -3,
    Critical = -2,
    Error = -1,
    Warning = 1,
    Notice = 2,
    Info = 3,
    Debug = 4,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LogLevelProcesses {
    #[serde(rename="PID")]
    pub pid: usize,
    #[serde(rename="Log level")]
    pub log_level: LogLevel,
    #[serde(rename="Type")]
    pub process_type: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum LogLevelResponse {
    #[serde(rename="Processes")]
    Processes(Vec<LogLevelProcesses>),
    #[serde(rename="Log level")]
    LogLevel(usize),
    #[serde(rename="New global log level")]
    GlobalLogLevel(usize),
}
#[derive(Debug, Deserialize, Serialize)]
pub enum XLogLevelResponse {
    #[serde(rename="xLog Level")]
    LogLevel(usize),
    #[serde(rename="New xLog Level")]
    NewLogLevel(usize),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CacheResponse {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventsListItem {
    pub name: String,
    pub id: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventsListResponse {
    #[serde(rename="Events")]
    pub events: Vec<EventsListItem>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="PascalCase")]
pub struct TUacDlgResponse {
    pub status: String,
    #[serde(default)]
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="snake_case")]
pub enum RegEnabled {
    #[default]
    Yes,
    No,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum RegState {
    #[default]
    NotRegisteredState,
    RegisteringState,
    AuthenticatingState,
    RegisteredState,
    RegisterTimeoutState,
    InternalErrorState,
    WrongCredentialsState,
    RegistrarErrorState,
    UnregisteringState,
    AuthenticatingUnregisterState,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegListRecord {
    #[serde(rename = "AOR")]
    pub aor: String,
    pub expires: u64,
    pub state: RegState,
    pub enabled: RegEnabled,
    pub last_register_sent: String,
    pub registration_t_out: String,
    pub registrar: String,
    pub binding: String,
    #[serde(default, rename = "dst_IP")]
    pub dst_ip: String,
    #[serde(default)]
    pub ip: String,
    #[serde(default)]
    pub shtag: String,
    #[serde(default)]
    pub cluster_id: usize,
    #[serde(default)]
    pub binding_params: String,
    #[serde(default)]
    pub third_party_registrant: String,
    #[serde(default)]
    pub proxy: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="PascalCase")]
pub struct RegListResponse {
    pub records: Vec<RegListRecord>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="PascalCase")]
pub struct RegListRecordResponse {
    pub registrant: RegListRecord,
}


#[derive(Copy, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum DialogState {
    Unconfirmed = 1,
    Early = 2,
    ConfirmedNA = 3,
    Confirmed = 4,
    Deleted = 5,
}
impl std::fmt::Display for DialogState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DialogState::Unconfirmed => write!(f, "Unconfirmed"),
            DialogState::Early => write!(f, "Early"),
            DialogState::ConfirmedNA => write!(f, "Confirmed NoACk"),
            DialogState::Confirmed => write!(f, "Confirmed"),
            DialogState::Deleted => write!(f, "Deleted"),
        }
    }
}

// types for destination set list response
pub mod dispatcher {
    use super::*;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Partition {
        pub name: String,
        #[serde(default, rename = "SETS")]
        pub sets: Vec<Set>,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct Set {
        pub id: usize,
        #[serde(rename = "Destinations")]
        pub destinations: Vec<Destination>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub enum DestinationState {
        Active,
        Probing,
        Inactive,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Destination {
        #[serde(rename = "URI")]
        pub uri: String,
        pub state: DestinationState,
        pub resolved_addresses: Vec<String>,
        #[serde(default)]
        pub description: String,
        #[serde(default)]
        pub weight: usize,
        #[serde(default)]
        pub priority: usize,
        #[serde(default)]
        pub first_hit_counter: usize,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct ListResponse {
        #[serde(rename = "PARTITIONS")]
        pub partitions: Vec<Partition>,
    }
}

pub mod clusterer {
    use super::*;

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ClustererListResponse {
        #[serde(rename = "Clusters")]
        pub clusters: Vec<Cluster>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Cluster {
        pub cluster_id: usize,
        #[serde(rename = "Nodes")]
        pub nodes: Vec<Node>,
    }

    #[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub enum LinkState {
        #[default]
        Up,
        Down,
        Probe,
    }

    #[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum NodeState {
        #[default]
        Enabled,
        Disabled,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Node {
        pub node_id: i64,
        pub db_id: i64,
        pub url: String,
        pub link_state: LinkState,
        pub state: NodeState,
        pub next_hop: String,
        pub description: String,
    }

    #[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum TagState {
        #[default]
        Active,
        Backup,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct SharedTagStatus {
        pub tag: String,
        pub cluster: usize,
        pub state: TagState,
    }
}

pub mod usrloc {
    use super::*;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct DumpResponse {
        #[serde(rename = "Domains")]
        pub domains: Vec<DumpDomain>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct DumpDomain {
        #[serde(rename = "AORs")]
        pub aors: Vec<DumpAOR>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct DumpAOR {
        #[serde(rename = "AOR")]
        pub aor: String,
        #[serde(rename = "Contacts")]
        pub contacts: Vec<DumpContact>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum DumpExpires {
        Other(String),
        Expiration(u64),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DumpContact {
        #[serde(rename = "Contact")]
        pub contact: String,
        #[serde(rename = "ContactID")]
        pub contact_id: String,
        #[serde(rename = "Expires")]
        pub expires: DumpExpires,
        #[serde(rename = "Q")]
        pub q: String,
        #[serde(rename = "Callid")]
        pub call_id: String,
        #[serde(rename = "Cseq")]
        pub cseq: i32,
        #[serde(rename = "User-agent")]
        pub user_agent: Option<String>,
        #[serde(rename = "Received")]
        pub received: Option<String>,
        #[serde(rename = "Path")]
        pub path: Option<String>,
        #[serde(rename = "State")]
        pub state: String,
        #[serde(rename = "Flags")]
        pub flags: u32,
        #[serde(rename = "Cflags")]
        pub cflags: String,
        #[serde(rename = "Socket")]
        pub socket: Option<String>,
        #[serde(rename = "Methods")]
        pub methods: u32,
        #[serde(rename = "Attr")]
        pub attr: Option<String>,
    }
}

pub mod dialog {
    use super::*;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Dialog {
        #[serde(rename = "ID")]
        pub id: String,
        #[serde(rename = "callid")]
        pub call_id: String,
        pub state: DialogState,
        #[serde(rename = "timestart")]
        pub time_start: u64,
        pub from_uri: String,
        pub to_uri: String,
        #[serde(rename = "caller_tag")]
        pub to_tag: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct ListResponse {
        pub dialogs: Vec<Dialog>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct ListRecordResponse {
        pub dialog: Dialog,
    }
}

pub mod b2b_entities {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Contact {
        pub caller: String,
        pub callee: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Cseq {
        pub caller: i64,
        pub callee: i64,
    }

    #[derive(Copy, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug)]
    #[repr(u8)]
    pub enum DBFlag {
        NoUpdateDB = 0,
        UpdateDB = 1,
        InsertDB = 2,
    }

    #[derive(Copy, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug)]
    #[repr(u8)]
    pub enum B2BState {
        Undefined = 0,
        New = 1, /* New dialog, no reply received yet */
        NewAuth = 2, /* New dialog with auth info, no reply received yet */
        Early = 3, /* Early dialog, provisional response received */
        Confirmed = 4, /* Confirmed dialog, 2xx received */
        Established = 5, /* Established dialog, sent or received ACK received */
        Modified = 6, /* ReInvite inside dialog */
        Terminated = 7, /* Terminated dialog */
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct UASession {
        pub dlg: i64,
        pub logic_key: String,
        pub mod_name: String,
        pub state: B2BState,
        pub last_invite_cseq: usize,
        pub last_method: usize,
        pub last_reply_code: usize,
        pub db_flag: DBFlag,
        pub ruri: String,
        pub callid: String,
        pub from: String,
        pub from_uri: String,
        pub from_tag: String,
        pub to: String,
        pub to_uri: String,
        pub to_tag: String,
        pub cseq: Cseq,
        pub contact: Contact,
        pub send_sock: String,
        pub tm_tran: String,
    }
}
