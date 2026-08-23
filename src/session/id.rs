use getset::Getters;

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
    pub fn new(begin_string: &str, sender_comp_id: &str, target_comp_id: &str) -> Self {
        SessionIdBuilder::new(begin_string, sender_comp_id, target_comp_id).build()
    }

    pub fn reverse_id(&self) -> SessionId {
        SessionIdBuilder::new(&self.begin_string, &self.target_comp_id, &self.sender_comp_id)
            .sender_sub_id(self.target_sub_id.as_deref())
            .sender_location_id(self.target_location_id.as_deref())
            .target_sub_id(self.sender_sub_id.as_deref())
            .target_location_id(self.sender_location_id.as_deref())
            .session_qualifier(self.session_qualifier.as_deref())
            .build()
    }
}

// Hash and PartialEq use only the `id` field — two SessionId values with
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

// Allows HashMap<SessionId, V>::get("some_str") without constructing a full
// SessionId for lookups. The Borrow contract requires hash(self) == hash(self.borrow()),
// which holds because Hash above hashes only self.id — the same bytes &str hashes.
impl std::borrow::Borrow<str> for SessionId {
    fn borrow(&self) -> &str {
        &self.id
    }
}

pub(crate) struct SessionIdBuilder {
    begin_string: String,
    sender_comp_id: String,
    sender_sub_id: Option<String>,
    sender_location_id: Option<String>,
    target_comp_id: String,
    target_sub_id: Option<String>,
    target_location_id: Option<String>,
    session_qualifier: Option<String>,
}

impl SessionIdBuilder {
    pub fn new(begin_string: &str, sender_comp_id: &str, target_comp_id: &str) -> Self {
        Self {
            begin_string: begin_string.to_owned(),
            sender_comp_id: sender_comp_id.to_owned(),
            sender_sub_id: None,
            sender_location_id: None,
            target_comp_id: target_comp_id.to_owned(),
            target_sub_id: None,
            target_location_id: None,
            session_qualifier: None,
        }
    }

    pub fn sender_sub_id(mut self, id: Option<&str>) -> Self {
        if let Some(s) = id {
            self.sender_sub_id = Some(s.to_owned());
        }
        self
    }

    pub fn sender_location_id(mut self, id: Option<&str>) -> Self {
        if let Some(s) = id {
            self.sender_location_id = Some(s.to_owned());
        }
        self
    }

    pub fn target_sub_id(mut self, id: Option<&str>) -> Self {
        if let Some(s) = id {
            self.target_sub_id = Some(s.to_owned());
        }
        self
    }

    pub fn target_location_id(mut self, id: Option<&str>) -> Self {
        if let Some(s) = id {
            self.target_location_id = Some(s.to_owned());
        }
        self
    }

    pub fn session_qualifier(mut self, qualifier: Option<&str>) -> Self {
        if let Some(s) = qualifier {
            self.session_qualifier = Some(s.to_owned());
        }
        self
    }

    pub fn build(self) -> SessionId {
        let id = create_sessionid_string(
            &self.begin_string,
            &self.sender_comp_id,
            &self.sender_sub_id,
            &self.sender_location_id,
            &self.target_comp_id,
            &self.target_sub_id,
            &self.target_location_id,
            &self.session_qualifier,
        );
        SessionId {
            begin_string: self.begin_string,
            sender_comp_id: self.sender_comp_id,
            sender_sub_id: self.sender_sub_id,
            sender_location_id: self.sender_location_id,
            target_comp_id: self.target_comp_id,
            target_sub_id: self.target_sub_id,
            target_location_id: self.target_location_id,
            session_qualifier: self.session_qualifier,
            id,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn create_sessionid_string(
    begin_str: &str,
    sender_compid: &str,
    sender_subid: &Option<String>,
    sender_locationid: &Option<String>,
    target_compid: &str,
    target_subid: &Option<String>,
    target_locationid: &Option<String>,
    session_qualifier: &Option<String>,
) -> String {
    // Format: "BeginString:SenderCompID/SubID/LocationID->TargetCompID/SubID/LocationID:Qualifier"
    // e.g. "FIX.4.3:SENDER/sub/loc->TARGET/sub/loc:qual" (optional parts omitted when absent)
    let mut sid = format!("{}:{}", begin_str, sender_compid);
    if let Some(subid) = sender_subid {
        sid.push('/');
        sid.push_str(subid);
    }
    if let Some(locid) = sender_locationid {
        sid.push('/');
        sid.push_str(locid);
    }
    sid.push_str("->");
    sid.push_str(target_compid);
    if let Some(t_subid) = target_subid {
        sid.push('/');
        sid.push_str(t_subid);
    }
    if let Some(t_locid) = target_locationid {
        sid.push('/');
        sid.push_str(t_locid);
    }
    if let Some(qualifier) = session_qualifier {
        sid.push(':');
        sid.push_str(qualifier);
    }
    sid
}

#[cfg(test)]
mod session_id_tests {
    use super::*;

    #[test]
    fn test_reverse_id_swaps_sender_and_target() {
        let sid = SessionIdBuilder::new("FIX.4.3", "SENDER", "TARGET")
            .sender_sub_id(Some("s_sub"))
            .sender_location_id(Some("s_loc"))
            .target_sub_id(Some("t_sub"))
            .target_location_id(Some("t_loc"))
            .session_qualifier(Some("qual1"))
            .build();

        assert_eq!(sid.id(), "FIX.4.3:SENDER/s_sub/s_loc->TARGET/t_sub/t_loc:qual1");

        let reversed = sid.reverse_id();
        assert_eq!(reversed.id(), "FIX.4.3:TARGET/t_sub/t_loc->SENDER/s_sub/s_loc:qual1");
        assert_eq!(reversed.sender_comp_id(), "TARGET");
        assert_eq!(reversed.target_comp_id(), "SENDER");

        assert_eq!(reversed.reverse_id(), sid);
    }

    #[test]
    fn test_reverse_id_minimal_no_optional_fields() {
        let sid = SessionId::new("FIX.4.3", "ALPHA", "BETA");

        assert_eq!(sid.id(), "FIX.4.3:ALPHA->BETA");
        assert_eq!(sid.reverse_id().id(), "FIX.4.3:BETA->ALPHA");
        assert_eq!(sid.reverse_id().reverse_id(), sid);
    }
}
