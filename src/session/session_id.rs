use crate::session::*;
use derive_builder::{Builder, UninitializedFieldError};
use getset::Getters;
use std::collections::HashMap;
use std::fmt;
use std::hash::{self, Hash};

const NOT_SET: &str = "";
#[derive(Debug, Default, PartialEq, Eq, Getters, Clone, Builder)]
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

    pub fn from_map(prop_map: &HashMap<String, String>) -> Self {
        let mut builder = SessionIdBuilder::default();
        if let Some(begin_string) = prop_map.get(BEGIN_STRING_SETTING) {
            builder.begin_string(begin_string);
        }

        if let Some(sender_comp) = prop_map.get(SENDER_COMPID_SETTING) {
            builder.sender_compid(sender_comp);
        }

        if let Some(sender_sub) = prop_map.get(SENDER_SUBID_SETTING) {
            builder.sender_subid(sender_sub);
        }

        if let Some(sender_loc) = prop_map.get(SENDER_LOCATIONID_SETTING) {
            builder.sender_locationid(sender_loc);
        }
        if let Some(target_comp) = prop_map.get(TARGET_COMPID_SETTING) {
            builder.target_compid(target_comp);
        }
        if let Some(target_sub) = prop_map.get(TARGET_SUBID_SETTING) {
            builder.target_subid(target_sub);
        }
        if let Some(target_loc) = prop_map.get(TARGET_LOCATIONID_SETTING) {
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
            begin_string: self.begin_string.as_ref().unwrap_or(NOT_SET).to_string(),
            sender_compid: Clone::clone(self.sender_compid.as_ref().ok_or(
                SessionIdBuilderError::from(UninitializedFieldError::new("sender_compid")),
            )?),
            sender_subid: self.sender_subid.as_ref().and_then(|opt| opt.clone()),
            sender_locationid: self.sender_locationid.as_ref().and_then(|opt| opt.clone()),
            target_compid: Clone::clone(self.target_compid.as_ref().ok_or(
                SessionIdBuilderError::from(UninitializedFieldError::new("target_compid")),
            )?),
            target_subid: self.target_subid.as_ref().and_then(|opt| opt.clone()),
            target_locationid: self.target_locationid.as_ref().and_then(|opt| opt.clone()),
            session_qualifier: self.session_qualifier.as_ref().and_then(|opt| opt.clone()),
            id: String::new(),
        };
        session_id.set_session_id();
        Ok(session_id)
    }
}
