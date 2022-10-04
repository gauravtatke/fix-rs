use crate::session::*;
use derive_builder::Builder;
use getset::Getters;
use std::collections::HashMap;
use std::fmt;
use std::hash::{self, Hash};

#[derive(Debug, PartialEq, Eq, Getters, Clone, Builder)]
#[builder(setter(into, strip_option), default, build_fn(skip))]
#[getset(get = "pub")]
pub struct SessionId {
    begin_string: String,
    sender_compid: String,
    sender_subid: Option<String>,
    sender_locationid: Option<String>,
    target_compid: String,
    target_subid: Option<String>,
    target_locationid: Option<String>,
    session_qualifier: Option<String>,
    #[builder(setter(skip))]
    id: String,
}

impl std::default::Default for SessionId {
    fn default() -> Self {
        SessionIdBuilder::new("DEFAULT", "", "").build().unwrap()
    }
}

impl Hash for SessionId {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl SessionId {
    fn set_session_id(&mut self) {
        self.id.push_str(&self.begin_string);
        self.id.push(':');
        self.id.push_str(&self.sender_compid);
        if self.sender_subid.is_some() {
            self.id.push('/');
            self.id.push_str(&self.sender_subid.clone().unwrap());
        }

        if self.sender_locationid.is_some() {
            self.id.push('/');
            self.id.push_str(&self.sender_locationid.clone().unwrap());
        }

        self.id.push_str("->");
        self.id.push_str(&self.target_compid);
        if self.target_subid.is_some() {
            self.id.push('/');
            self.id.push_str(&self.target_subid.clone().unwrap());
        }

        if self.target_locationid.is_some() {
            self.id.push('/');
            self.id.push_str(&self.target_locationid.clone().unwrap());
        }
    }

    pub fn from_map(
        prop_map: &HashMap<String, String>, defaults: &HashMap<String, String>,
    ) -> Self {
        let mut builder = SessionIdBuilder::default();
        builder
            .begin_string(
                prop_map
                    .get(BEGIN_STRING_SETTING)
                    .or_else(|| defaults.get(BEGIN_STRING_SETTING))
                    .unwrap(),
            )
            .sender_compid(
                prop_map
                    .get(SENDER_COMPID_SETTING)
                    .or_else(|| defaults.get(SENDER_COMPID_SETTING))
                    .unwrap(),
            )
            .target_compid(
                prop_map
                    .get(TARGET_COMPID_SETTING)
                    .or_else(|| defaults.get(TARGET_COMPID_SETTING))
                    .unwrap(),
            );

        if let Some(sender_sub) =
            prop_map.get(SENDER_SUBID_SETTING).or_else(|| defaults.get(SENDER_SUBID_SETTING))
        {
            builder.sender_subid(sender_sub);
        }

        if let Some(sender_loc) = prop_map
            .get(SENDER_LOCATIONID_SETTING)
            .or_else(|| defaults.get(SENDER_LOCATIONID_SETTING))
        {
            builder.sender_locationid(sender_loc);
        }

        if let Some(target_sub) =
            prop_map.get(TARGET_SUBID_SETTING).or_else(|| defaults.get(TARGET_SUBID_SETTING))
        {
            builder.target_subid(target_sub);
        }
        if let Some(target_loc) = prop_map
            .get(TARGET_LOCATIONID_SETTING)
            .or_else(|| defaults.get(TARGET_LOCATIONID_SETTING))
        {
            builder.target_locationid(target_loc);
        }

        builder.build().unwrap()
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl SessionIdBuilder {
    pub fn new<S: Into<String>>(begin_string: S, sender_comp: S, target_comp: S) -> Self {
        let mut sessionid_builder = SessionIdBuilder::default();
        sessionid_builder.begin_string = Some(begin_string.into());
        sessionid_builder.sender_compid = Some(sender_comp.into());
        sessionid_builder.target_compid = Some(target_comp.into());
        sessionid_builder
    }

    pub fn build(&self) -> Result<SessionId, SessionIdBuilderError> {
        let mut session_id = SessionId {
            begin_string: self.begin_string.as_ref().unwrap().to_string(),
            sender_compid: self.sender_compid.as_ref().unwrap().to_string(),
            sender_subid: self.sender_subid.clone().flatten().and_then(|s| {
                if !s.is_empty() && s != "" {
                    Some(s.to_owned())
                } else {
                    None
                }
            }),
            sender_locationid: self.sender_locationid.clone().flatten().and_then(|s| {
                if s.is_empty() || s == "" {
                    None
                } else {
                    Some(s.to_owned())
                }
            }),
            target_compid: self.target_compid.as_ref().unwrap().to_string(),
            target_subid: self.target_subid.clone().flatten().and_then(|s| {
                if s.is_empty() || s == "" {
                    None
                } else {
                    Some(s.to_owned())
                }
            }),
            target_locationid: self.target_locationid.clone().flatten().and_then(|s| {
                if s.is_empty() || s == "" {
                    None
                } else {
                    Some(s.to_owned())
                }
            }),

            session_qualifier: self.session_qualifier.as_ref().and_then(|opt| opt.clone()),
            id: String::new(),
        };
        session_id.set_session_id();
        Ok(session_id)
    }
}
