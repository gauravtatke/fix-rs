use derive_builder::Builder;
use getset::Getters;
use std::fmt;
use std::hash::{self, Hash};

#[derive(Debug, PartialEq, Eq, Getters, Clone, Builder)]
#[builder(setter(into, strip_option), default, build_fn(skip))]
#[getset(get = "pub")]
pub struct LegacySessionId {
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

impl std::default::Default for LegacySessionId {
    fn default() -> Self {
        LegacySessionIdBuilder::new("DEFAULT", "", "").build().unwrap()
    }
}

impl Hash for LegacySessionId {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl LegacySessionId {
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
}

impl fmt::Display for LegacySessionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl LegacySessionIdBuilder {
    pub fn new<S: Into<String>>(begin_string: S, sender_comp: S, target_comp: S) -> Self {
        let mut sessionid_builder = LegacySessionIdBuilder::default();
        sessionid_builder.begin_string = Some(begin_string.into());
        sessionid_builder.sender_compid = Some(sender_comp.into());
        sessionid_builder.target_compid = Some(target_comp.into());
        sessionid_builder
    }

    pub fn build(&self) -> Result<LegacySessionId, LegacySessionIdBuilderError> {
        let mut session_id = LegacySessionId {
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
