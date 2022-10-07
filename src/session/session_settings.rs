use crate::session::*;
use std::collections::HashMap;
use std::fs;
use std::iter::{Iterator, Peekable};
use std::path::Path;
use std::str::{FromStr, Lines};

#[derive(Debug)]
pub struct Properties {
    default_session_id: SessionId,
    session_settings: HashMap<SessionId, HashMap<String, String>>,
}

impl Properties {
    pub fn new<P: AsRef<Path>>(p: P) -> Self {
        let toml_str = fs::read_to_string(p).expect("unable to open the config file");
        Self::from_str(&toml_str)
    }

    pub fn get_or_default<F: FromStr>(&self, session_id: &SessionId, name: &str) -> Option<F> {
        let default_properties = self.session_settings.get(&self.default_session_id).unwrap();
        self.session_settings
            .get(session_id)
            .and_then(|props| props.get(name))
            .or_else(|| default_properties.get(name))
            .and_then(|val| val.as_str().parse::<F>().ok())
    }

    pub fn default_property<'a, F: FromStr>(&self, name: &str) -> Option<F> {
        self.get_or_default(&self.default_session_id, name)
    }

    pub fn from_str(s: &str) -> Self {
        let mut default_found = false;
        let mut lines = s.lines().peekable();
        let mut default_session_id: Option<SessionId> = None;
        let mut setting_map = HashMap::new();
        while let Some(line) = lines.next() {
            let line = line.trim();
            if line.starts_with('[') && line.ends_with(']') && line.contains(DEFAULT_SECTION_NAME) {
                if default_found {
                    // duplicate default section
                    panic!("duplicate default section found");
                }
                let default_section = parse_table(&mut lines);
                default_found = true;
                default_session_id = Some(SessionId::default());
                setting_map.insert(default_session_id.clone().unwrap(), default_section);
            } else if line.starts_with('[') && line.ends_with(']') && default_found {
                // some other section in config file
                if !default_found {
                    panic!("default section should be first section. not found");
                }
                let section = parse_table(&mut lines);
                let defaults = setting_map.get(default_session_id.as_ref().unwrap()).unwrap();
                let session_id = SessionId::from_map(&section, defaults);
                setting_map.insert(session_id, section);
            }
        }
        if !default_found {
            panic!("default section not found");
        }
        let properties = Self {
            default_session_id: default_session_id.unwrap(),
            session_settings: setting_map,
        };
        properties.check();
        properties
    }

    pub fn session_ids(&self) -> Vec<&SessionId> {
        self.session_settings
            .keys()
            .filter(|&id| !id.eq(&self.default_session_id))
            .collect::<Vec<&SessionId>>()
    }

    fn check(&self) {
        let connection_type: String = match self.default_property(CONNECTION_TYPE_SETTING) {
            Some(s) => s,
            None => panic!("connection_type not found"),
        };
        if connection_type != ACCEPTOR_CONN_TYPE && connection_type != INITIATOR_CONN_TYPE {
            panic!("invalid connection type");
        }
        for session_id in self.session_ids() {
            // verify ports
            if connection_type == ACCEPTOR_CONN_TYPE {
                if self.get_or_default::<u16>(session_id, SOCKET_ACCEPT_PORT_SETTING).is_none() {
                    panic!("acceptor port not found");
                }
            } else {
                if self.get_or_default::<String>(session_id, SOCKET_CONNECT_HOST_SETTING).is_none()
                    || self.get_or_default::<u16>(session_id, SOCKET_CONNECT_PORT_SETTING).is_none()
                {
                    panic!("socket connect host or port is missing");
                }
            }

            // verify begin string
            let begin_string = self
                .get_or_default::<String>(session_id, BEGIN_STRING_SETTING)
                .expect("begin string is missing");
            if begin_string != FIX42_BEGIN_STR
                && begin_string != FIX43_BEGIN_STR
                && begin_string != FIX44_BEGIN_STR
            {
                panic!("invalid begin string");
            }

            // verify comp_ids
            if self.get_or_default::<String>(session_id, SENDER_COMPID_SETTING).is_none()
                || self.get_or_default::<String>(session_id, TARGET_COMPID_SETTING).is_none()
            {
                panic!("sender and/or target compid missing");
            }
        }
    }
}

fn parse_table(lines: &mut Peekable<Lines>) -> HashMap<String, String> {
    // takes only the lines between 2 sections and creates a map out of it
    // let peekable_lines = lines.peekable();
    let mut properties = HashMap::new();
    while let Some(line) = lines.next_if(|&l| !l.trim().starts_with('[')) {
        let line = line.trim();
        if !line.is_empty() {
            let (prop_key, prop_val) = line
                .split_once('=')
                .and_then(|(key, val)| {
                    Some((
                        key.trim().trim_start_matches('"').trim_end_matches('"'),
                        val.trim().trim_start_matches('"').trim_end_matches('"'),
                    ))
                })
                .unwrap();
            properties.insert(prop_key.to_string(), prop_val.to_string());
        }
    }
    properties
}

#[cfg(test)]
mod session_setting_tests {
    use super::*;

    // #[test]
    fn session_sample_config_test() {
        // println!("{:#?}", &session_config.sessions);

        let cargo_toml = r#" 
            [Default]
            connection_type = "acceptor"
            sender_comp_id = "sender"
            target_comp_id= "target"
            socket_accept_port = 10117

            [Session]
            sender_comp_id = "sender_1"
            target_comp_id = "target_1"

            [Session]
            sender_comp_id = "sender_order"
            target_comp_id = "target_order"
            session_qualifier = "order"
"#;

        let properties = Properties::from_str(cargo_toml);
        println!("{:#?}", properties);
        let accept_port = properties
            .get_or_default::<u16>(&properties.default_session_id, SOCKET_ACCEPT_PORT_SETTING);
        println!("{:?}", accept_port);
    }

    #[test]
    #[should_panic(expected = "default section not found")]
    fn test_no_default_section() {
        let cfg_toml = r#"
            [Session]
            sender_comp_id = "sender"
            target_comp_id = "target"
        "#;
        let settings = Properties::from_str(cfg_toml);
    }

    #[test]
    #[should_panic(expected = "connection_type not found")]
    fn test_default_no_connection_type() {
        let cfg_toml = r#"
            [Default]
            begin_string = "FIX.4.3"
            sender_comp_id = "sender"
            target_comp_id = "target"

            [Session]
            sender_comp_id = "sender"
            target_comp_id = "target"
        "#;
        let settings = Properties::from_str(cfg_toml);
    }

    #[test]
    fn test_no_mandatory_fields() {
        // no begin_string, no sender_compid, no target_compid
    }

    #[test]
    fn test_only_default_settings() {}

    #[test]
    fn test_default_and_session_overrides() {}
}
