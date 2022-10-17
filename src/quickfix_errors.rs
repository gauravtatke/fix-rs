use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, thiserror::Error)]
#[error("Session Level Reject Reason - {:?}", .kind)]
pub struct SessionRejectError {
    kind: SessionRejectReason,
    // tag: Option<String>,
    // value: Option<String>,
    // pub source: Option<Box<dyn Error>>,
}

impl SessionRejectError {
    pub fn invalid_tag_err() -> Self {
        // tag not parsed properly
        SessionRejectError {
            kind: SessionRejectReason::InvalidTag,
        }
    }

    pub fn required_tag_missing_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::RequiredTagMissing,
        }
    }

    pub fn undefined_tag_err() -> Self {
        // tag not defined in Xml
        SessionRejectError {
            kind: SessionRejectReason::UndefinedTag,
        }
    }

    pub fn tag_without_value_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::TagSpecifiedWithoutValue,
        }
    }

    pub fn value_out_of_range_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::ValueOutOfRange,
        }
    }

    pub fn incorrect_data_format_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::IncorrectDataFormatForValue,
        }
    }

    pub fn decryption_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::DecryptionProblem,
        }
    }

    pub fn signature_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::SignatureProblem,
        }
    }

    pub fn comp_id_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::CompIdProblem,
        }
    }

    pub fn sending_time_accuracy_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::SendingTimeAccuracyProblem,
        }
    }

    pub fn invalid_msg_type_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::InvalidMessageType,
        }
    }

    pub fn invalid_body_len_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::InvalidBodyLength,
        }
    }

    pub fn invalid_checksum() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::InvalidChecksum,
        }
    }

    pub fn tag_not_defined_for_msg() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::TagNotDefinedForMsgType,
        }
    }

    pub fn xml_validation_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::XmlValidationError,
        }
    }

    pub fn tag_appear_more_than_once() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::TagAppearsMoreThanOnce,
        }
    }

    pub fn tag_specified_out_of_order() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::TagSpecifiedOutOfOrder,
        }
    }

    pub fn repeating_grp_out_of_order() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::RepeatingGroupsOutOfOrder,
        }
    }

    pub fn incorrect_num_in_grp_count() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::IncorrectNumInGroupCountForRepeatingGroup,
        }
    }

    pub fn non_data_field_contains_soh() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::NonDataFieldIncludeSOHChar,
        }
    }
}

#[derive(Debug)]
enum SessionRejectReason {
    // #[error("Invalid tag")]
    InvalidTag,
    // #[error("Required tag missing")]
    RequiredTagMissing,
    // #[error("Undefined tag")]
    UndefinedTag,
    // #[error("Tag not defined for message, tag")]
    TagNotDefinedForMsgType,
    // #[error("No value for tag")]
    TagSpecifiedWithoutValue,
    // #[error("Value out of range for tag")]
    ValueOutOfRange,
    // #[error("Incorrect data format")]
    IncorrectDataFormatForValue,
    // #[error("Decryption problem")]
    DecryptionProblem,
    // #[error("Signature problem")]
    SignatureProblem,
    // #[error("Compid problem")]
    CompIdProblem,
    // #[error("Sending time accuracy problem")]
    SendingTimeAccuracyProblem,
    // #[error("Invalid message type")]
    InvalidMessageType,
    XmlValidationError,
    TagAppearsMoreThanOnce,
    TagSpecifiedOutOfOrder,
    RepeatingGroupsOutOfOrder,
    IncorrectNumInGroupCountForRepeatingGroup,
    NonDataFieldIncludeSOHChar,
    // #[error("Invalid body length")]
    InvalidBodyLength,
    // #[error("Invalid checksum")]
    InvalidChecksum,
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
pub enum ConfigErr<'a> {
    #[error("Config Not Present - {}", .0)]
    NotFound(&'a str),
    #[error("Could not parse - {}", .0)]
    ParseError(String),
}
