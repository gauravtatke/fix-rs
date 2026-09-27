//! Shared FIX field-tag registry: `field name -> tag number`.
//!
//! Version-neutral by design. FIX tag numbers are globally stable — a given number
//! always denotes the same field, and each FIX version is (almost entirely) a superset
//! of the previous one. So this ONE file lists every field's tag, shared across all
//! `crate::fixNN` message modules; adding a new FIX version means appending only the
//! delta of new tags here, never editing existing ones.
//!
//! Generator-owned once codegen lands (M3); hand-maintained for the reference messages.

// Header / trailer — common to every message.
pub const BEGIN_STRING: u32 = 8;
pub const BODY_LENGTH: u32 = 9;
pub const MSG_TYPE: u32 = 35;
pub const MSG_SEQ_NUM: u32 = 34;
pub const SENDER_COMP_ID: u32 = 49;
pub const TARGET_COMP_ID: u32 = 56;
pub const SENDING_TIME: u32 = 52;
pub const CHECK_SUM: u32 = 10;

// Logon (35=A).
pub const ENCRYPT_METHOD: u32 = 98;
pub const HEART_BT_INT: u32 = 108;
pub const RAW_DATA_LENGTH: u32 = 95;
pub const RAW_DATA: u32 = 96;
pub const RESET_SEQ_NUM_FLAG: u32 = 141;
pub const MAX_MESSAGE_SIZE: u32 = 383;
pub const TEST_MESSAGE_INDICATOR: u32 = 464;
pub const USERNAME: u32 = 553;
pub const PASSWORD: u32 = 554;
pub const NO_MSG_TYPES: u32 = 384;
pub const REF_MSG_TYPE: u32 = 372;
pub const MSG_DIRECTION: u32 = 385;

// Market data (MarketDataSnapshotFullRefresh 35=W, NoMDEntries group).
pub const MD_REQ_ID: u32 = 262;
pub const SYMBOL: u32 = 55;
pub const NO_MD_ENTRIES: u32 = 268;
pub const MD_ENTRY_TYPE: u32 = 269;
pub const MD_ENTRY_PX: u32 = 270;
pub const MD_ENTRY_SIZE: u32 = 271;
