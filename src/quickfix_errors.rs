use std::num::ParseIntError;

// FIX-level reject reasons raised during message parsing/validation. This enum
// IS the error type (it derives `Error`) — each variant carries exactly the
// context it needs: the offending `tag` for field-level problems, a free-form
// `msg` for the catch-all cases, and nothing for whole-message problems.
//
// `InvalidBodyLength`/`InvalidChecksum` are *garbled-message* markers, not
// Reject(35=3) reasons — a garbled message can't be trusted enough to reference
// in a Reject, so the FIX-correct handling is to drop it. They have no tag-373
// wire code (see `code`).
//
// No `Copy` because `Other`/`InvalidUnsupportedAppVersion` own a `String`.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SessionRejectReason {
    #[error("Invalid tag number: {tag}")]
    InvalidTag { tag: u32 },
    #[error("Required tag missing: {tag}")]
    RequiredTagMissing { tag: u32 },
    #[error("Tag {tag} not defined for this message type")]
    TagNotDefinedForMsgType { tag: u32 },
    #[error("Undefined tag: {tag}")]
    UndefinedTag { tag: u32 },
    #[error("Tag {tag} specified without a value")]
    TagSpecifiedWithoutValue { tag: u32 },
    #[error("Value out of range for tag {tag}")]
    ValueOutOfRange { tag: u32 },
    #[error("Incorrect data format for tag {tag}")]
    IncorrectDataFormatForValue { tag: u32 },
    #[error("Decryption problem")]
    DecryptionProblem,
    #[error("Signature problem")]
    SignatureProblem,
    #[error("CompID problem")]
    CompIdProblem,
    #[error("SendingTime accuracy problem")]
    SendingTimeAccuracyProblem,
    #[error("Invalid message type")]
    InvalidMessageType,
    #[error("XML validation error")]
    XmlValidationError,
    #[error("Tag {tag} appears more than once")]
    TagAppearsMoreThanOnce { tag: u32 },
    #[error("Tag {tag} specified out of required order")]
    TagSpecifiedOutOfOrder { tag: u32 },
    #[error("Repeating group fields out of order near tag {tag}")]
    RepeatingGroupsOutOfOrder { tag: u32 },
    #[error("Incorrect NumInGroup count for repeating group (tag {tag})")]
    IncorrectNumInGroupCountForRepeatingGroup { tag: u32 },
    #[error("Non-data field {tag} contains an SOH delimiter")]
    NonDataFieldIncludeSOHChar { tag: u32 },
    #[error("Invalid or unsupported application version: {msg}")]
    InvalidUnsupportedAppVersion { msg: String },
    #[error("{msg}")]
    Other { msg: String },
    #[error("Invalid body length")]
    InvalidBodyLength,
    #[error("Invalid checksum")]
    InvalidChecksum,
}

impl SessionRejectReason {
    /// The FIX 4.3 SessionRejectReason (tag 373) wire code for this reason.
    ///
    /// Mapping is by name, not enum position. `Other` uses 99 (a real "Other"
    /// code); `InvalidUnsupportedAppVersion` uses 18 (a FIXT/4.4+ code, harmless
    /// on 4.3 since it isn't produced there).
    ///
    /// `InvalidBodyLength`/`InvalidChecksum` have no tag-373 code — they are
    /// garbled-message markers that get dropped, never rejected — so they map to
    /// a sentinel (`u32::MAX`) that should never reach the wire. The match is
    /// left exhaustive (no `_` arm) on purpose: adding a new reason will fail to
    /// compile until its code is decided here.
    pub fn code(&self) -> u32 {
        match self {
            SessionRejectReason::InvalidTag { .. } => 0,
            SessionRejectReason::RequiredTagMissing { .. } => 1,
            SessionRejectReason::TagNotDefinedForMsgType { .. } => 2,
            SessionRejectReason::UndefinedTag { .. } => 3,
            SessionRejectReason::TagSpecifiedWithoutValue { .. } => 4,
            SessionRejectReason::ValueOutOfRange { .. } => 5,
            SessionRejectReason::IncorrectDataFormatForValue { .. } => 6,
            SessionRejectReason::DecryptionProblem => 7,
            SessionRejectReason::SignatureProblem => 8,
            SessionRejectReason::CompIdProblem => 9,
            SessionRejectReason::SendingTimeAccuracyProblem => 10,
            SessionRejectReason::InvalidMessageType => 11,
            SessionRejectReason::XmlValidationError => 12,
            SessionRejectReason::TagAppearsMoreThanOnce { .. } => 13,
            SessionRejectReason::TagSpecifiedOutOfOrder { .. } => 14,
            SessionRejectReason::RepeatingGroupsOutOfOrder { .. } => 15,
            SessionRejectReason::IncorrectNumInGroupCountForRepeatingGroup { .. } => 16,
            SessionRejectReason::NonDataFieldIncludeSOHChar { .. } => 17,
            SessionRejectReason::InvalidUnsupportedAppVersion { .. } => 18,
            SessionRejectReason::Other { .. } => 99,
            SessionRejectReason::InvalidBodyLength | SessionRejectReason::InvalidChecksum => {
                u32::MAX
            }
        }
    }

    /// The offending tag, for a Reject's RefTagID (371) — `None` for reasons
    /// that aren't about a specific tag.
    pub fn ref_tag(&self) -> Option<u32> {
        match self {
            SessionRejectReason::InvalidTag { tag }
            | SessionRejectReason::RequiredTagMissing { tag }
            | SessionRejectReason::TagNotDefinedForMsgType { tag }
            | SessionRejectReason::UndefinedTag { tag }
            | SessionRejectReason::TagSpecifiedWithoutValue { tag }
            | SessionRejectReason::ValueOutOfRange { tag }
            | SessionRejectReason::IncorrectDataFormatForValue { tag }
            | SessionRejectReason::TagAppearsMoreThanOnce { tag }
            | SessionRejectReason::TagSpecifiedOutOfOrder { tag }
            | SessionRejectReason::RepeatingGroupsOutOfOrder { tag }
            | SessionRejectReason::IncorrectNumInGroupCountForRepeatingGroup { tag }
            | SessionRejectReason::NonDataFieldIncludeSOHChar { tag } => Some(*tag),
            _ => None,
        }
    }

    /// Free-form text for a Reject's Text (58) — `None` unless the reason
    /// carries a message.
    pub fn text(&self) -> Option<&str> {
        match self {
            SessionRejectReason::InvalidUnsupportedAppVersion { msg }
            | SessionRejectReason::Other { msg } => Some(msg.as_str()),
            _ => None,
        }
    }

    /// Whether this reason marks a *garbled* message (bad body length or
    /// checksum) that must be dropped rather than rejected. The parse-side
    /// classification for 7.3: `is_garbled()` → drop the message and keep the
    /// connection; anything else → send a Reject(35=3).
    pub fn is_garbled(&self) -> bool {
        matches!(
            self,
            SessionRejectReason::InvalidBodyLength | SessionRejectReason::InvalidChecksum
        )
    }
}

#[cfg(test)]
mod session_reject_reason_tests {
    use super::SessionRejectReason;

    #[test]
    fn test_is_garbled_true_for_bodylength_and_checksum() {
        assert!(SessionRejectReason::InvalidBodyLength.is_garbled());
        assert!(SessionRejectReason::InvalidChecksum.is_garbled());
    }

    #[test]
    fn test_is_garbled_false_for_reject_reasons() {
        // A representative field-level reason and a message-level one — neither
        // is garbled, both should be rejected (not dropped).
        assert!(!SessionRejectReason::ValueOutOfRange { tag: 55 }.is_garbled());
        assert!(!SessionRejectReason::InvalidMessageType.is_garbled());
        assert!(!SessionRejectReason::Other { msg: "boom".into() }.is_garbled());
    }

    #[test]
    fn test_code_maps_reason_to_tag373_wire_value() {
        // Spot-check the by-name mapping, including the two whose enum position
        // differs from their wire code (UndefinedTag=3, TagNotDefinedForMsgType=2).
        assert_eq!(SessionRejectReason::InvalidTag { tag: 1 }.code(), 0);
        assert_eq!(SessionRejectReason::TagNotDefinedForMsgType { tag: 44 }.code(), 2);
        assert_eq!(SessionRejectReason::UndefinedTag { tag: 99 }.code(), 3);
        assert_eq!(SessionRejectReason::ValueOutOfRange { tag: 98 }.code(), 5);
        assert_eq!(SessionRejectReason::Other { msg: "x".into() }.code(), 99);
    }

    #[test]
    fn test_garbled_reasons_have_sentinel_code() {
        // These never reach the wire (they're dropped), so code() is a sentinel.
        assert_eq!(SessionRejectReason::InvalidBodyLength.code(), u32::MAX);
        assert_eq!(SessionRejectReason::InvalidChecksum.code(), u32::MAX);
    }

    #[test]
    fn test_ref_tag_present_for_field_reasons_absent_otherwise() {
        assert_eq!(SessionRejectReason::RequiredTagMissing { tag: 35 }.ref_tag(), Some(35));
        assert_eq!(SessionRejectReason::UndefinedTag { tag: 99 }.ref_tag(), Some(99));
        // reasons not about a specific tag carry no RefTagID
        assert_eq!(SessionRejectReason::SignatureProblem.ref_tag(), None);
        assert_eq!(SessionRejectReason::Other { msg: "x".into() }.ref_tag(), None);
    }

    #[test]
    fn test_text_present_only_for_message_bearing_reasons() {
        assert_eq!(SessionRejectReason::Other { msg: "bad".into() }.text(), Some("bad"));
        assert_eq!(
            SessionRejectReason::InvalidUnsupportedAppVersion { msg: "v9".into() }.text(),
            Some("v9")
        );
        // tag-bearing / bare reasons have no Text
        assert_eq!(SessionRejectReason::ValueOutOfRange { tag: 55 }.text(), None);
        assert_eq!(SessionRejectReason::InvalidChecksum.text(), None);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum XmlError {
    #[error("Could not parse the document")]
    DocumentNotParsed(#[from] roxmltree::Error),
    #[error("Node {} not found", .0)]
    XmlNodeNotFound(String),
    #[error("Could not parse field {field} into u32: {:?}", .source)]
    FieldNotParsed {
        source: ParseIntError,
        field: String,
    },
    #[error("Duplicate field {}", .0)]
    DuplicateField(String),
    #[error("Duplicate message {}", .0)]
    DuplicateMessage(String),
    #[error("Attribute {} not found", .0)]
    AttributeNotFound(String),
    #[error("Unknown xml tag {}", .0)]
    UnknownXmlTag(String),
}

pub enum InvalidMessage {
    FieldDoesNotHaveDelimiter,
    MessageDoesNotHaveSOH,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigParseError {
    #[error("Invalid TOML: {0}")]
    InvalidToml(#[from] toml::de::Error),
    #[error("Missing [Default] section")]
    MissingDefaultSection,
    #[error("Session block {index} is not a valid table")]
    InvalidSessionBlock { index: usize },
    #[error("Failed to deserialize session block {index}: {source}")]
    SessionDeserialize {
        index: usize,
        source: toml::de::Error,
    },
    #[error("Failed to deserialize [Default] section: {0}")]
    DefaultDeserialize(toml::de::Error),
    #[error("Invalid field={field} value={value}")]
    InvalidFieldValue { field: String, value: String },
    #[error("Missing required field {field}")]
    MissingRequiredField { field: String },
    #[error("Validation failed: {msg}")]
    ValidationFailed { msg: String },
}

#[derive(Debug, thiserror::Error)]
pub enum FieldError {
    #[error("Tag not found")]
    TagNotFound,
    #[error("Could not parse field value into the requested type")]
    InvalidFormat,
}

// Session-level errors raised by verify_msg — distinct from SessionRejectReason,
// which covers FIX-level Reject(3) reasons during message parsing.
#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("BeginString mismatch: expected {expected} got {received}")]
    BeginStringMismatch { expected: String, received: String },
    #[error("Message type {msg_type} not valid for current session state")]
    InvalidStateForMsgType { msg_type: String },
    #[error("CompID mismatch: expected sender={expected_sender} target={expected_target}")]
    CompIdMismatch {
        expected_sender: String,
        expected_target: String,
    },
    #[error("MsgSeqNum too low: received {received} expected {expected}")]
    SeqNumTooLow { received: u32, expected: u32 },
    #[error("Missing header field tag={tag}")]
    MissingHeaderField { tag: u32 },
    #[error("Not in session time")]
    OutOfSessionTime,
}

#[derive(Debug, thiserror::Error)]
#[error("Do not send")]
pub struct DonotSend;

#[derive(Debug, thiserror::Error)]
#[error("Reject logon: {reason:?}")]
pub struct RejectLogon {
    reason: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Unsupported message type")]
    UnsupportedMessageType,
    #[error("Required field is missing: {tag}")]
    FieldNotFound { tag: u32 },
    #[error("Field value does not parse to the expected value")]
    IncorrectDataFormat,
}

// Failure modes for an outbound app-message send driven from outside the engine
// (the fix-rs analogue of QFJ's Session.sendToTarget returning false /
// throwing SessionNotFound).
#[derive(Debug, thiserror::Error)]
pub enum SendError {
    #[error("No session found for the given SessionId")]
    SessionNotFound,
    #[error("Session is not logged on; message not sent")]
    NotLoggedOn,
}
