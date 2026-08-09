use crate::quickfix_errors::ConfigParseError;
use crate::session::*;
use chrono::{NaiveTime, Weekday};
use chrono_tz::Tz;
use getset::Getters;
use log::{error, warn};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use toml::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectionType {
    #[default]
    Acceptor,
    Initiator,
}

impl TryFrom<&str> for ConnectionType {
    type Error = ConfigParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case("acceptor") {
            Ok(ConnectionType::Acceptor)
        } else if value.eq_ignore_ascii_case("initiator") {
            Ok(ConnectionType::Initiator)
        } else {
            Err(ConfigParseError::InvalidFieldValue {
                field: "connection_type".to_string(),
                value: value.to_owned(),
            })
        }
    }
}

#[derive(Debug, Deserialize, Default)]
struct FixProperties {
    begin_string: Option<String>,
    connection_type: Option<String>,
    session_qualifier: Option<String>,
    heartbeat_interval: Option<u32>,
    data_dictionary: Option<PathBuf>,
    // sender ids
    sender_comp_id: Option<String>,
    sender_sub_id: Option<String>,
    sender_location_id: Option<String>,
    // target ids
    target_comp_id: Option<String>,
    target_sub_id: Option<String>,
    target_location_id: Option<String>,
    // host & port
    socket_accept_port: Option<u16>,
    socket_connect_port: Option<u16>,
    socket_connect_host: Option<String>,
    // reset
    reset_on_logon: Option<bool>,
    reset_on_disconnect: Option<bool>,
    reset_on_logout: Option<bool>,
    // start & end
    start_day: Option<Weekday>,
    start_time: Option<NaiveTime>,
    end_day: Option<Weekday>,
    end_time: Option<NaiveTime>,
    timezone: Option<Tz>,
}

#[derive(Debug, Clone, Eq, Getters)]
#[getset(get = "pub")]
pub struct SessionId {
    begin_string: String,
    sender_comp_id: String,
    sender_sub_id: Option<String>,
    sender_location_id: Option<String>,
    target_comp_id: String,
    target_sub_id: Option<String>,
    target_location_id: Option<String>,
    session_qualifier: Option<String>,
    id: String,
}

impl SessionId {
    pub fn reverse_id(&self) -> SessionId {
        SessionId {
            begin_string: self.begin_string.clone(),
            sender_comp_id: self.target_comp_id.clone(),
            sender_sub_id: self.target_sub_id.clone(),
            sender_location_id: self.target_location_id.clone(),
            target_comp_id: self.sender_comp_id.clone(),
            target_sub_id: self.sender_sub_id.clone(),
            target_location_id: self.sender_location_id.clone(),
            session_qualifier: self.session_qualifier.clone(),
            id: create_sessionid_string(
                &self.begin_string,
                &self.target_comp_id,
                self.target_sub_id.clone(),
                self.target_location_id.clone(),
                &self.sender_comp_id,
                self.sender_sub_id.clone(),
                self.sender_location_id.clone(),
                self.session_qualifier.clone(),
            ),
        }
    }
}

// Hash and PartialEq use only the `id` field — two SessionIdV2 values with
// the same composite id string are the same session. Derived impls would hash
// all fields, breaking the Borrow<str> contract below.
impl std::hash::Hash for SessionId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for SessionId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// Allows HashMap<SessionIdV2, V>::get("some_str") without constructing a full
// SessionIdV2 for lookups. The Borrow contract requires hash(self) == hash(self.borrow()),
// which holds because Hash above hashes only self.id — the same bytes &str hashes.
impl std::borrow::Borrow<str> for SessionId {
    fn borrow(&self) -> &str {
        &self.id
    }
}

#[derive(Debug)]
pub struct SessionConfig {
    // identity (required)
    begin_string: String,
    connection_type: ConnectionType,
    sender_comp_id: String,
    sender_sub_id: Option<String>,      // optional, no default
    sender_location_id: Option<String>, // optional, no default
    target_comp_id: String,
    target_sub_id: Option<String>,      // optional, no default
    target_location_id: Option<String>, // optional, no default
    session_qualifier: Option<String>,  // optional, no default
    // host & port (required per connection_type, validated in TryFrom)
    socket_accept_port: Option<u16>,
    socket_connect_port: Option<u16>,
    socket_connect_host: Option<String>,
    heartbeat_interval: Option<u32>,
    // schedule
    start_time: NaiveTime,      // default: 00:00:00
    end_time: NaiveTime,        // default: 23:59:59
    start_day: Option<Weekday>, // optional, must pair with end_day
    end_day: Option<Weekday>,   // optional, must pair with start_day
    timezone: Tz,               // default: UTC
    // session behavior
    reset_on_logon: bool,      // default: false
    reset_on_disconnect: bool, // default: false
    reset_on_logout: bool,     // default: false
    data_dictionary: PathBuf,  // default: derived from begin_string (e.g. FIX43.xml)
}

fn create_sessionid_string(
    begin_str: &str,
    sender_compid: &str,
    sender_subid: Option<String>,
    sender_locationid: Option<String>,
    target_compid: &str,
    target_subid: Option<String>,
    target_locationid: Option<String>,
    session_qualifier: Option<String>,
) -> String {
    // Format: "BeginString:SenderCompID/SubID/LocationID->TargetCompID/SubID/LocationID:Qualifier"
    // e.g. "FIX.4.3:SENDER/sub/loc->TARGET/sub/loc:qual" (optional parts omitted when absent)
    let mut sid = format!("{}:{}", begin_str, sender_compid);
    if let Some(subid) = sender_subid {
        sid.push('/');
        sid.push_str(&subid);
    }
    if let Some(locid) = sender_locationid {
        sid.push('/');
        sid.push_str(&locid);
    }
    sid.push_str("->");
    sid.push_str(target_compid);
    if let Some(t_subid) = target_subid {
        sid.push('/');
        sid.push_str(&t_subid);
    }
    if let Some(t_locid) = target_locationid {
        sid.push('/');
        sid.push_str(&t_locid);
    }
    if let Some(qualifier) = session_qualifier {
        sid.push(':');
        sid.push_str(&qualifier);
    }
    sid
}
impl SessionConfig {
    fn to_session_id(&self) -> SessionId {
        SessionId {
            begin_string: self.begin_string.clone(),
            sender_comp_id: self.sender_comp_id.clone(),
            sender_sub_id: self.sender_sub_id.clone(),
            sender_location_id: self.sender_location_id.clone(),
            target_comp_id: self.target_comp_id.clone(),
            target_sub_id: self.target_sub_id.clone(),
            target_location_id: self.target_location_id.clone(),
            session_qualifier: self.session_qualifier.clone(),
            id: create_sessionid_string(
                &self.begin_string,
                &self.sender_comp_id,
                self.sender_sub_id.clone(),
                self.sender_location_id.clone(),
                &self.target_comp_id,
                self.target_sub_id.clone(),
                self.target_location_id.clone(),
                self.session_qualifier.clone(),
            ),
        }
    }
}

impl TryFrom<FixProperties> for SessionConfig {
    type Error = ConfigParseError;
    fn try_from(settings: FixProperties) -> Result<Self, Self::Error> {
        let begin_str = settings.begin_string.ok_or(ConfigParseError::MissingRequiredField {
            field: BEGIN_STRING_SETTING.to_string(),
        })?;
        if begin_str != FIX42_BEGIN_STR
            && begin_str != FIX43_BEGIN_STR
            && begin_str != FIX44_BEGIN_STR
        {
            return Err(ConfigParseError::InvalidFieldValue {
                field: BEGIN_STRING_SETTING.to_string(),
                value: begin_str,
            });
        }

        let connection_type = ConnectionType::try_from(
            settings
                .connection_type
                .ok_or(ConfigParseError::MissingRequiredField {
                    field: CONNECTION_TYPE_SETTING.to_string(),
                })?
                .as_str(),
        )?;
        let sender_comp_id =
            settings.sender_comp_id.ok_or(ConfigParseError::MissingRequiredField {
                field: SENDER_COMPID_SETTING.to_string(),
            })?;
        let target_comp_id =
            settings.target_comp_id.ok_or(ConfigParseError::MissingRequiredField {
                field: TARGET_COMPID_SETTING.to_string(),
            })?;

        let data_dictionary = settings.data_dictionary.unwrap_or_else(|| {
            let dict_name = begin_str.replace(".", "") + ".xml";
            PathBuf::from(dict_name)
        });

        let mut config = SessionConfig {
            begin_string: begin_str,
            connection_type,
            sender_comp_id,
            target_comp_id,
            timezone: settings.timezone.unwrap_or(chrono_tz::UTC),
            reset_on_logon: settings.reset_on_logon.unwrap_or(false),
            reset_on_disconnect: settings.reset_on_disconnect.unwrap_or(false),
            reset_on_logout: settings.reset_on_logout.unwrap_or(false),
            data_dictionary,
            session_qualifier: settings.session_qualifier,
            heartbeat_interval: settings.heartbeat_interval,
            sender_sub_id: settings.sender_sub_id,
            sender_location_id: settings.sender_location_id,
            target_sub_id: settings.target_sub_id,
            target_location_id: settings.target_location_id,
            socket_accept_port: None,
            socket_connect_port: None,
            socket_connect_host: None,
            start_day: None,
            start_time: settings.start_time.unwrap_or(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            end_day: None,
            end_time: settings.end_time.unwrap_or(NaiveTime::from_hms_opt(23, 59, 59).unwrap()),
        };

        // socket host/port validation
        if connection_type == ConnectionType::Acceptor {
            config.socket_accept_port = Some(settings.socket_accept_port.ok_or(
                ConfigParseError::MissingRequiredField {
                    field: SOCKET_ACCEPT_PORT_SETTING.to_string(),
                },
            )?);
            if settings.socket_connect_host.is_some() || settings.socket_connect_port.is_some() {
                warn!(
                    "{} and {} fields are ignored for acceptor",
                    SOCKET_CONNECT_HOST_SETTING, SOCKET_CONNECT_PORT_SETTING
                );
            }
        }
        if connection_type == ConnectionType::Initiator {
            if settings.socket_connect_host.is_none()
                || settings.socket_connect_port.is_none()
                || settings.heartbeat_interval.is_none()
            {
                let err_msg = format!(
                    "{}, {}, & {}, all should be present for initiator",
                    SOCKET_CONNECT_HOST_SETTING,
                    SOCKET_CONNECT_PORT_SETTING,
                    HEARTBEAT_INTERVAL_SETTING
                );
                error!("{}", err_msg);
                return Err(ConfigParseError::ValidationFailed { msg: err_msg });
            }
            config.socket_connect_host = settings.socket_connect_host;
            config.socket_connect_port = settings.socket_connect_port;
            config.heartbeat_interval = settings.heartbeat_interval;
            if settings.socket_accept_port.is_some() {
                warn!("{} field is ignored for initiator", SOCKET_ACCEPT_PORT_SETTING);
            }
        }

        match (settings.start_day, settings.end_day) {
            (Some(sd), Some(ed)) => {
                config.start_day = Some(sd);
                config.end_day = Some(ed);
            }
            (None, None) => {}
            _ => {
                return Err(ConfigParseError::ValidationFailed {
                    msg: format!(
                        "Either {} & {} both should be present or both should be absent",
                        START_DAY_SETTING, END_DAY_SETTING
                    ),
                });
            }
        }

        Ok(config)
    }
}

#[derive(Debug, Default)]
pub struct SessionProperties {
    default: FixProperties,
    sessions: HashMap<SessionId, SessionConfig>,
}

impl SessionProperties {
    pub fn from_str(settings: &str) -> Result<Self, ConfigParseError> {
        let parsed_toml = settings.parse::<toml::Table>()?;
        let default_value =
            parsed_toml.get("Default").ok_or(ConfigParseError::MissingDefaultSection)?;
        let default_table =
            default_value.as_table().ok_or(ConfigParseError::MissingDefaultSection)?;
        let mut sessions = HashMap::new();
        if let Some(session_tables) = parsed_toml.get("Session").and_then(|v| v.as_array()) {
            for (i, table) in session_tables.iter().enumerate() {
                let mut cloned_table = default_table.clone();
                cloned_table.extend(
                    table
                        .as_table()
                        .ok_or(ConfigParseError::InvalidSessionBlock { index: i })?
                        .clone(),
                );
                let effective_session = Value::Table(cloned_table)
                    .try_into::<FixProperties>()
                    .map_err(|e| ConfigParseError::SessionDeserialize {
                        index: i,
                        source: e,
                    })?;
                let session_config = SessionConfig::try_from(effective_session)?;
                // let session_id = session_config.get_session_id();
                let session_id = session_config.to_session_id();
                sessions.insert(session_id, session_config);
            }
        }
        let default_properties: FixProperties = default_value
            .clone()
            .try_into::<FixProperties>()
            .map_err(ConfigParseError::DefaultDeserialize)?;
        Ok(Self {
            default: default_properties,
            sessions,
        })
    }
}
#[cfg(test)]
mod session_setting_tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_fix_properties_deserialize_single_block() {
        let toml_str = r#"
            connection_type = "acceptor"
            begin_string = "FIX.4.3"
            sender_comp_id = "FIXIMULATOR"
            target_comp_id = "BANZAI"
            socket_accept_port = 10114
            heartbeat_interval = 30
            reset_on_logon = true
            reset_on_logout = false
            reset_on_disconnect = false
            data_dictionary = "resources/FIX43.xml"
            start_time = "08:00:00"
            end_time = "16:00:00"
            start_day = "Monday"
            end_day = "Friday"
            timezone = "US/Eastern"
        "#;

        let props: FixProperties = toml::from_str(toml_str).unwrap();

        assert_eq!(props.connection_type.as_deref(), Some("acceptor"));
        assert_eq!(props.begin_string.as_deref(), Some("FIX.4.3"));
        assert_eq!(props.sender_comp_id.as_deref(), Some("FIXIMULATOR"));
        assert_eq!(props.target_comp_id.as_deref(), Some("BANZAI"));
        assert_eq!(props.socket_accept_port, Some(10114));
        assert_eq!(props.heartbeat_interval, Some(30));
        assert_eq!(props.reset_on_logon, Some(true));
        assert_eq!(props.reset_on_logout, Some(false));
        assert_eq!(props.reset_on_disconnect, Some(false));
        assert_eq!(props.data_dictionary.as_deref(), Some(Path::new("resources/FIX43.xml")));
        assert_eq!(props.start_time, Some(NaiveTime::from_hms_opt(8, 0, 0).unwrap()));
        assert_eq!(props.end_time, Some(NaiveTime::from_hms_opt(16, 0, 0).unwrap()));
        assert_eq!(props.start_day, Some(Weekday::Mon));
        assert_eq!(props.end_day, Some(Weekday::Fri));
        assert_eq!(props.timezone, Some(chrono_tz::US::Eastern));
        // fields not present in the TOML should be None
        assert_eq!(props.session_qualifier, None);
        assert_eq!(props.sender_sub_id, None);
        assert_eq!(props.socket_connect_host, None);
        assert_eq!(props.socket_connect_port, None);
    }

    #[test]
    fn test_fix_properties_deserialize_partial_block() {
        let toml_str = r#"
            sender_comp_id = "sender_1"
            target_comp_id = "target_1"
            session_qualifier = "order"
        "#;

        let props: FixProperties = toml::from_str(toml_str).unwrap();

        assert_eq!(props.sender_comp_id.as_deref(), Some("sender_1"));
        assert_eq!(props.target_comp_id.as_deref(), Some("target_1"));
        assert_eq!(props.session_qualifier.as_deref(), Some("order"));
        // everything else should be None
        assert_eq!(props.connection_type, None);
        assert_eq!(props.begin_string, None);
        assert_eq!(props.socket_accept_port, None);
        assert_eq!(props.heartbeat_interval, None);
    }

    #[test]
    fn test_weekday_case_insensitive() {
        let lower = r#"
            start_day = "monday"
            end_day = "friday"
        "#;
        let upper = r#"
            start_day = "MONDAY"
            end_day = "FRIDAY"
        "#;
        let mixed = r#"
            start_day = "Monday"
            end_day = "Friday"
        "#;
        let abbrev = r#"
            start_day = "mon"
            end_day = "fri"
        "#;

        let lower_props: FixProperties = toml::from_str(lower).unwrap();
        let upper_props: FixProperties = toml::from_str(upper).unwrap();
        let mixed_props: FixProperties = toml::from_str(mixed).unwrap();
        let abbrev_props: FixProperties = toml::from_str(abbrev).unwrap();

        assert_eq!(lower_props.start_day, Some(Weekday::Mon));
        assert_eq!(upper_props.start_day, Some(Weekday::Mon));
        assert_eq!(mixed_props.start_day, Some(Weekday::Mon));
        assert_eq!(abbrev_props.start_day, Some(Weekday::Mon));

        assert_eq!(lower_props.end_day, Some(Weekday::Fri));
        assert_eq!(upper_props.end_day, Some(Weekday::Fri));
        assert_eq!(mixed_props.end_day, Some(Weekday::Fri));
        assert_eq!(abbrev_props.end_day, Some(Weekday::Fri));
    }

    #[test]
    fn test_session_overrides_default() {
        let cfg = r#"
            [Default]
            connection_type = "acceptor"
            begin_string = "FIX.4.3"
            socket_accept_port = 10114
            reset_on_logon = false

            [[Session]]
            sender_comp_id = "SENDER_1"
            target_comp_id = "TARGET_1"

            [[Session]]
            sender_comp_id = "SENDER_2"
            target_comp_id = "TARGET_2"
            socket_accept_port = 10115
            reset_on_logon = true
        "#;

        let props = SessionProperties::from_str(cfg).unwrap();

        // default preserved as-is
        assert_eq!(props.default.connection_type.as_deref(), Some("acceptor"));
        assert_eq!(props.default.begin_string.as_deref(), Some("FIX.4.3"));
        assert_eq!(props.default.socket_accept_port, Some(10114));
        assert_eq!(props.default.sender_comp_id, None);

        assert_eq!(props.sessions.len(), 2);

        // session 1: inherits defaults, no overrides
        let s1 = props.sessions.get("FIX.4.3:SENDER_1->TARGET_1").unwrap();
        assert_eq!(s1.sender_comp_id, "SENDER_1");
        assert_eq!(s1.target_comp_id, "TARGET_1");
        assert_eq!(s1.connection_type, ConnectionType::Acceptor);
        assert_eq!(s1.begin_string, "FIX.4.3");
        assert_eq!(s1.socket_accept_port, Some(10114));
        assert_eq!(s1.reset_on_logon, false);

        // session 2: overrides port and reset_on_logon
        let s2 = props.sessions.get("FIX.4.3:SENDER_2->TARGET_2").unwrap();
        assert_eq!(s2.sender_comp_id, "SENDER_2");
        assert_eq!(s2.target_comp_id, "TARGET_2");
        assert_eq!(s2.connection_type, ConnectionType::Acceptor);
        assert_eq!(s2.begin_string, "FIX.4.3");
        assert_eq!(s2.socket_accept_port, Some(10115));
        assert_eq!(s2.reset_on_logon, true);
    }

    #[test]
    fn test_session_inherits_all_default_fields() {
        let cfg = r#"
            [Default]
            connection_type = "acceptor"
            begin_string = "FIX.4.3"
            socket_accept_port = 10114
            heartbeat_interval = 30
            data_dictionary = "resources/FIX43.xml"
            start_time = "08:00:00"
            end_time = "16:00:00"
            timezone = "US/Eastern"

            [[Session]]
            sender_comp_id = "SENDER"
            target_comp_id = "TARGET"
        "#;

        let props = SessionProperties::from_str(cfg).unwrap();
        let s = props.sessions.get("FIX.4.3:SENDER->TARGET").unwrap();

        assert_eq!(s.connection_type, ConnectionType::Acceptor);
        assert_eq!(s.begin_string, "FIX.4.3");
        assert_eq!(s.socket_accept_port, Some(10114));
        assert_eq!(s.heartbeat_interval, Some(30));
        assert_eq!(s.data_dictionary, PathBuf::from("resources/FIX43.xml"));
        assert_eq!(s.start_time, NaiveTime::from_hms_opt(8, 0, 0).unwrap());
        assert_eq!(s.end_time, NaiveTime::from_hms_opt(16, 0, 0).unwrap());
        assert_eq!(s.timezone, chrono_tz::US::Eastern);
        // fields not in default or session stay None
        assert_eq!(s.session_qualifier, None);
        assert_eq!(s.start_day, None);
        assert_eq!(s.end_day, None);
    }

    #[test]
    fn test_only_default_no_sessions() {
        let cfg = r#"
            [Default]
            connection_type = "acceptor"
            begin_string = "FIX.4.3"
        "#;

        let props = SessionProperties::from_str(cfg).unwrap();

        assert_eq!(props.default.connection_type.as_deref(), Some("acceptor"));
        assert_eq!(props.sessions.len(), 0);
    }

    #[test]
    fn test_missing_default_section() {
        let cfg = r#"
            [[Session]]
            sender_comp_id = "SENDER"
            target_comp_id = "TARGET"
        "#;

        let err = SessionProperties::from_str(cfg).unwrap_err();
        assert!(matches!(err, ConfigParseError::MissingDefaultSection));
    }

    #[test]
    fn test_invalid_toml_syntax() {
        let cfg = r#"
            [Default
            connection_type = "acceptor"
        "#;

        let err = SessionProperties::from_str(cfg).unwrap_err();
        assert!(matches!(err, ConfigParseError::InvalidToml(_)));
    }

    #[test]
    fn test_session_deserialize_wrong_type() {
        let cfg = r#"
            [Default]
            connection_type = "acceptor"
            begin_string = "FIX.4.3"

            [[Session]]
            sender_comp_id = "SENDER"
            target_comp_id = "TARGET"
            socket_accept_port = "not_a_number"
        "#;

        let err = SessionProperties::from_str(cfg).unwrap_err();
        assert!(
            matches!(err, ConfigParseError::SessionDeserialize { index: 0, .. }),
            "expected SessionDeserialize for index 0, got: {:?}",
            err
        );
    }

    #[test]
    fn test_session_deserialize_error_reports_correct_index() {
        let cfg = r#"
            [Default]
            connection_type = "acceptor"
            begin_string = "FIX.4.3"
            socket_accept_port = 10114

            [[Session]]
            sender_comp_id = "SENDER_1"
            target_comp_id = "TARGET_1"

            [[Session]]
            socket_accept_port = "bad"
        "#;

        let err = SessionProperties::from_str(cfg).unwrap_err();
        assert!(
            matches!(err, ConfigParseError::SessionDeserialize { index: 1, .. }),
            "expected SessionDeserialize for index 1, got: {:?}",
            err
        );
    }

    #[test]
    fn test_default_deserialize_wrong_type() {
        let cfg = r#"
            [Default]
            socket_accept_port = "not_a_number"
        "#;

        let err = SessionProperties::from_str(cfg).unwrap_err();
        assert!(matches!(err, ConfigParseError::DefaultDeserialize(_)));
    }

    #[test]
    fn test_empty_config() {
        let cfg = "";

        let err = SessionProperties::from_str(cfg).unwrap_err();
        assert!(matches!(err, ConfigParseError::MissingDefaultSection));
    }

    #[test]
    fn test_all_fields_parsed_into_session_config() {
        let cfg = r#"
            [Default]
            connection_type = "acceptor"
            begin_string = "FIX.4.3"
            socket_accept_port = 10114

            [[Session]]
            sender_comp_id = "SENDER"
            target_comp_id = "TARGET"
            sender_sub_id = "sender_sub"
            sender_location_id = "sender_loc"
            target_sub_id = "target_sub"
            target_location_id = "target_loc"
            session_qualifier = "qual1"
            heartbeat_interval = 30
            data_dictionary = "resources/FIX43.xml"
            reset_on_logon = true
            reset_on_disconnect = true
            reset_on_logout = true
            timezone = "US/Eastern"
            start_day = "Monday"
            end_day = "Friday"
            start_time = "08:00:00"
            end_time = "16:00:00"
        "#;

        let props = SessionProperties::from_str(cfg).unwrap();
        assert_eq!(props.sessions.len(), 1);

        let sid = "FIX.4.3:SENDER/sender_sub/sender_loc->TARGET/target_sub/target_loc:qual1";
        let s = props.sessions.get(sid).expect(&format!("session not found for key: {}", sid));

        assert_eq!(s.begin_string, "FIX.4.3");
        assert_eq!(s.connection_type, ConnectionType::Acceptor);
        assert_eq!(s.sender_comp_id, "SENDER");
        assert_eq!(s.target_comp_id, "TARGET");
        assert_eq!(s.sender_sub_id.as_deref(), Some("sender_sub"));
        assert_eq!(s.sender_location_id.as_deref(), Some("sender_loc"));
        assert_eq!(s.target_sub_id.as_deref(), Some("target_sub"));
        assert_eq!(s.target_location_id.as_deref(), Some("target_loc"));
        assert_eq!(s.session_qualifier.as_deref(), Some("qual1"));
        assert_eq!(s.heartbeat_interval, Some(30));
        assert_eq!(s.data_dictionary, PathBuf::from("resources/FIX43.xml"));
        assert_eq!(s.reset_on_logon, true);
        assert_eq!(s.reset_on_disconnect, true);
        assert_eq!(s.reset_on_logout, true);
        assert_eq!(s.timezone, chrono_tz::US::Eastern);
        assert_eq!(s.socket_accept_port, Some(10114));
        assert_eq!(s.start_day, Some(Weekday::Mon));
        assert_eq!(s.end_day, Some(Weekday::Fri));
        assert_eq!(s.start_time, NaiveTime::from_hms_opt(8, 0, 0).unwrap());
        assert_eq!(s.end_time, NaiveTime::from_hms_opt(16, 0, 0).unwrap());
    }

    #[test]
    fn test_reverse_id_swaps_sender_and_target() {
        let cfg = r#"
            [Default]
            connection_type = "acceptor"
            begin_string = "FIX.4.3"
            socket_accept_port = 10114

            [[Session]]
            sender_comp_id = "SENDER"
            target_comp_id = "TARGET"
            sender_sub_id = "s_sub"
            sender_location_id = "s_loc"
            target_sub_id = "t_sub"
            target_location_id = "t_loc"
            session_qualifier = "qual1"
        "#;

        let props = SessionProperties::from_str(cfg).unwrap();
        let (sid, _) = props.sessions.iter().next().unwrap();

        assert_eq!(sid.id(), "FIX.4.3:SENDER/s_sub/s_loc->TARGET/t_sub/t_loc:qual1");

        let reversed = sid.reverse_id();
        assert_eq!(reversed.id(), "FIX.4.3:TARGET/t_sub/t_loc->SENDER/s_sub/s_loc:qual1");
        assert_eq!(reversed.sender_comp_id(), "TARGET");
        assert_eq!(reversed.target_comp_id(), "SENDER");

        // reverse of reverse == original
        assert_eq!(reversed.reverse_id(), *sid);
    }

    #[test]
    fn test_reverse_id_minimal_no_optional_fields() {
        let cfg = r#"
            [Default]
            connection_type = "acceptor"
            begin_string = "FIX.4.3"
            socket_accept_port = 10114

            [[Session]]
            sender_comp_id = "ALPHA"
            target_comp_id = "BETA"
        "#;

        let props = SessionProperties::from_str(cfg).unwrap();
        let (sid, _) = props.sessions.iter().next().unwrap();

        assert_eq!(sid.id(), "FIX.4.3:ALPHA->BETA");
        assert_eq!(sid.reverse_id().id(), "FIX.4.3:BETA->ALPHA");
        assert_eq!(sid.reverse_id().reverse_id(), *sid);
    }

    #[test]
    fn test_start_end_time_defaults_when_absent() {
        let cfg = r#"
            [Default]
            connection_type = "acceptor"
            begin_string = "FIX.4.3"
            socket_accept_port = 10114

            [[Session]]
            sender_comp_id = "SENDER"
            target_comp_id = "TARGET"
        "#;

        let props = SessionProperties::from_str(cfg).unwrap();
        let s = props.sessions.get("FIX.4.3:SENDER->TARGET").unwrap();

        assert_eq!(s.start_time, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        assert_eq!(s.end_time, NaiveTime::from_hms_opt(23, 59, 59).unwrap());
    }
}
