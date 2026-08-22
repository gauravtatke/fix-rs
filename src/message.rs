use getset::{CopyGetters, Getters, MutGetters};
use indexmap::IndexMap;
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use crate::data_dictionary::{DataDictionary, FixType, HEADER_ID};
use crate::fields::*;
use crate::quickfix_errors::{FieldError, SessionRejectError};
use crate::session::{SessionId, SessionIdBuilder};

type SessResult<T> = Result<T, SessionRejectError>;

/*
derive a macro which will create impl fns for each of the items in this enum
 and then delete this comment
 */
#[derive(Debug)]
pub enum Type {
    Int(i64),
    Length(u32),
    TagNum(u32),
    DayOfMonth(u32),
    SeqNum(u64),
    NumInGroup(u32),
    Float(f64),
    Price(f64),
    PriceOffset(f64),
    Amt(f64),
    Percent(f64),
    Qty(f64),
    Char(char),
    Bool(bool),
    Str(String),
    Currency(String),
    Country(String),
    Exchange(String),
    LocalMktDate(String),
    MonthYear(String),
    MultiValueStr(String),
    UtcDate(String),
    UtcTimeOnly(String),
    UtcTimestamp(String),
}

type Tag = u32;
pub const SOH: char = '\u{01}';
// pub const SOH: char = '|';

#[derive(Debug, Default, Clone, CopyGetters, Getters)]
pub struct StringField {
    #[getset(get_copy = "pub")]
    tag: Tag,

    #[getset(get = "pub")]
    value: String,
}

impl StringField {
    pub fn new(tag: Tag, value: &str) -> Self {
        Self {
            tag,
            value: value.to_string(),
        }
    }
}

impl Display for StringField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}{}", self.tag, self.value, SOH)
    }
}

#[derive(Debug, Default, Clone)]
pub struct FieldMap {
    // Flat (tag -> value) fields belonging directly to this section (header, trailer, or a
    // message body / one group instance). IndexMap keeps *insertion* order, which is what
    // makes `Message::to_string()` round-trip a parsed message byte-for-byte when no
    // explicit `field_order` below overrides it.
    fields: IndexMap<Tag, StringField>,
    // Repeating groups nested under this FieldMap, keyed by the group's "NoXXX" count tag
    // (e.g. tag 268 for MDEntries). Each Group owns its own Vec<FieldMap>, one per instance.
    group: HashMap<Tag, Group>,

    // Dictionary-declared field order for *this* section (set via `set_field_order`), used
    // to sort output when the spec cares about order (header's first 3 fields, and every
    // repeating group's field order). Empty for sections where wire order is unconstrained.
    field_order: Vec<Tag>,
}

impl FieldMap {
    #[inline]
    fn new() -> Self {
        Self::default()
    }

    fn with_field_order(field_order: &[u32]) -> Self {
        Self {
            field_order: field_order.to_vec(),
            ..Default::default()
        }
    }

    pub fn set_field(&mut self, field: StringField) {
        self.fields.insert(field.tag(), field);
    }

    pub fn get_field<T: FromStr>(&self, tag: u32) -> Result<T, FieldError> {
        if let Some(field) = self.fields.get(&tag) {
            return field.value.parse::<T>().map_err(|_| FieldError::InvalidFormat);
        }
        Err(FieldError::TagNotFound)
    }

    pub fn set_group(&mut self, tag: Tag, value: u32, rep_grp_delimiter: Tag) -> &mut Group {
        let grp_field = StringField::new(tag, value.to_string().as_str());
        self.set_field(grp_field);
        let group =
            self.group.entry(tag).or_insert_with(|| Group::new(rep_grp_delimiter, tag, value));
        // create group instances and insert into group
        for i in 0..value {
            group.add_group(FieldMap::new());
        }
        group
    }

    pub fn get_group(&self, tag: Tag) -> Option<&Group> {
        self.group.get(&tag)
    }

    pub fn set_field_order(&mut self, f_order: &[Tag]) {
        self.field_order = f_order.to_vec();
    }

    pub fn iter(&self) -> FieldMapIter<'_> {
        let mut map_iter = FieldMapIter::default();
        map_iter.fieldmap_to_vec(self);
        map_iter
    }

    /// Position of `tag` in this map's declared `field_order`, or `usize::MAX` if the tag
    /// has no declared position (such fields sort last during serialization).
    fn field_rank(&self, tag: Tag) -> usize {
        self.field_order.iter().position(|needle| *needle == tag).map_or(usize::MAX, |pos| pos)
    }
}

impl std::fmt::Display for FieldMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from_iter(self.iter().into_iter().map(|sfield| sfield.to_string()));
        write!(f, "{}", s)
    }
}

#[derive(Debug, Default)]
pub struct FieldMapIter<'a> {
    vec_str_field: Vec<&'a StringField>,
}

impl<'a> FieldMapIter<'a> {
    fn fieldmap_to_vec(&mut self, field_map: &'a FieldMap) {
        let mut temp_vec: Vec<&StringField> = field_map.fields.values().collect();
        if !field_map.field_order.is_empty() {
            temp_vec.sort_by_cached_key(|&field| field_map.field_rank(field.tag()))
        }
        for str_field in temp_vec {
            let tag = str_field.tag();
            self.vec_str_field.push(str_field);
            if let Some(grp) = field_map.get_group(tag) {
                for grp_field_map in grp.fields.iter() {
                    self.fieldmap_to_vec(grp_field_map);
                }
            }
        }
    }
}

impl<'a> IntoIterator for FieldMapIter<'a> {
    type Item = &'a StringField;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec_str_field.into_iter()
    }
}

#[derive(Debug, Default, Clone, CopyGetters, Getters)]
pub struct Group {
    #[getset(get_copy)]
    delim: u32,

    #[getset(get_copy)]
    tag: Tag,

    #[getset(get_copy)]
    value: u32,

    fields: Vec<FieldMap>,
}

impl Group {
    pub fn new(delimiter: Tag, tag: Tag, value: u32) -> Self {
        Self {
            delim: delimiter,
            tag,
            value,
            ..Default::default()
        }
    }

    pub fn add_group(&mut self, grp: FieldMap) {
        self.fields.push(grp);
    }

    pub fn size(&self) -> u32 {
        self.fields.len() as u32
    }
}

impl Index<usize> for Group {
    type Output = FieldMap;

    fn index(&self, idx: usize) -> &Self::Output {
        self.fields.index(idx)
    }
}

impl IndexMut<usize> for Group {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        self.fields.index_mut(idx)
    }
}

type Header = FieldMap;

#[derive(Debug, Default, Clone, MutGetters, Getters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct Message {
    header: Header,
    body: FieldMap,
    trailer: FieldMap,
}

impl Message {
    pub fn new() -> Self {
        Self {
            header: FieldMap::with_field_order(&[8, 9, 35]),
            ..Default::default()
        }
    }

    pub fn set_field(&mut self, fld: StringField) {
        self.body.set_field(fld);
    }

    pub fn get_field<T: FromStr>(&self, tag: Tag) -> Result<T, FieldError> {
        self.body.get_field(tag)
    }

    pub fn set_group(&mut self, tag: Tag, value: u32, rep_grp_delimiter: Tag) -> &mut Group {
        self.body.set_group(tag, value, rep_grp_delimiter)
    }

    pub fn get_group(&self, tag: Tag) -> Option<&Group> {
        self.body.get_group(tag)
    }

    fn add_group(&mut self, tag: Tag, grp: Group) {
        self.body.group.insert(tag, grp);
    }

    fn calc_checksum(&self) -> u32 {
        let mut byte_sum = 0u32;
        for sfield in
            self.header.iter().into_iter().chain(self.body.iter()).chain(self.trailer.iter())
        {
            if sfield.tag() != 10 {
                for byt in sfield.to_string().as_bytes() {
                    byte_sum = byte_sum + *byt as u32;
                }
            }
        }
        byte_sum % 256
    }

    pub fn set_checksum(&mut self) {
        let checksum_str = format!("{:0>3}", self.calc_checksum());
        self.trailer_mut().set_field(StringField::new(10, &checksum_str));
    }

    fn calc_body_len(&self) -> usize {
        self.header
            .iter()
            .into_iter()
            .chain(self.body.iter())
            .chain(self.trailer.iter())
            .filter_map(|sfield| {
                if sfield.tag() != 8 && sfield.tag() != 9 && sfield.tag() != 10 {
                    Some(sfield.to_string().as_bytes().len())
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn set_body_len(&mut self) {
        let body_len = self.calc_body_len();
        self.header_mut().set_field(StringField::new(9, &body_len.to_string()))
    }

    pub fn get_msg_type(&self) -> Result<String, FieldError> {
        self.header.get_field::<String>(35)
    }

    pub fn set_sending_time(&mut self) {
        let curr_time = chrono::Utc::now();
        let sending_time = curr_time.format("%Y%m%d-%T%.3f").to_string();
        self.header_mut().set_field(StringField::new(52, &sending_time));
    }

    // Entry point for turning a raw wire string into a `Message`. Two passes:
    // 1. Tokenize: split on SOH, split each "tag=value" chunk, push every field into a
    //    VecDeque in wire order (a plain Vec would do for this pass alone, but VecDeque is
    //    what the second pass needs — see `from_vec`).
    // 2. Structure: `from_vec` walks that queue and sorts fields into header/body/trailer
    //    (and nested groups) according to the dictionary.
    pub fn from_str(s: &str, dd: &DataDictionary) -> SessResult<Self> {
        let mut vdeq: VecDeque<StringField> = VecDeque::with_capacity(16);
        for field in s.split_terminator(SOH) {
            let (tag, value) = match field.split_once('=') {
                Some((t, v)) => {
                    let parse_result = t.parse::<u32>();
                    if parse_result.is_err() {
                        return Err(SessionRejectError::invalid_tag_err());
                    }
                    if v.is_empty() {
                        return Err(SessionRejectError::tag_without_value_err());
                    }
                    (parse_result.unwrap(), v)
                }
                None => return Err(SessionRejectError::invalid_tag_err()),
            };
            vdeq.push_back(StringField::new(tag, value));
        }

        from_vec(vdeq, dd)
    }

    pub fn get_session_id(&self) -> SessionId {
        let begin_str = self.header.get_field::<String>(8).unwrap();
        let sender_comp = self.header.get_field::<String>(49).unwrap();
        let target_comp = self.header.get_field::<String>(56).unwrap();
        SessionIdBuilder::new(&begin_str, &sender_comp, &target_comp)
            .sender_sub_id(self.header.get_field::<String>(50).ok().as_deref())
            .sender_location_id(self.header.get_field::<String>(142).ok().as_deref())
            .target_sub_id(self.header.get_field::<String>(57).ok().as_deref())
            .target_location_id(self.header.get_field::<String>(143).ok().as_deref())
            .build()
    }

    pub fn get_reverse_session_id(&self) -> SessionId {
        // sender values from message is put into target & vice-versa
        let begin_str = self.header.get_field::<String>(8).unwrap();
        let sender_comp = self.header.get_field::<String>(49).unwrap();
        let target_comp = self.header.get_field::<String>(56).unwrap();
        SessionIdBuilder::new(&begin_str, &target_comp, &sender_comp)
            .sender_sub_id(self.header.get_field::<String>(57).ok().as_deref())
            .sender_location_id(self.header.get_field::<String>(143).ok().as_deref())
            .target_sub_id(self.header.get_field::<String>(50).ok().as_deref())
            .target_location_id(self.header.get_field::<String>(142).ok().as_deref())
            .build()
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.header(), self.body, self.trailer())
    }
}

// Pulls a single field's value straight out of the raw wire string, without doing a full
// parse — used by get_session_id/get_reverse_session_id to identify which session a raw
// message belongs to *before* we know which DataDictionary (and therefore which
// `Message::from_str`) to parse it with.
//
// Searching for a bare "9=" would false-match the "9" inside "49=..." or "109=...", so
// every tag except 8 is searched for as SOH + tag + "=" to guarantee we're matching a
// tag boundary, not a substring of some other tag/value. Tag 8 (BeginString) is exempt
// because it is always the literal first bytes of the message, with no leading SOH to
// require.
fn extract_field_value<'a>(tag: &str, s: &'a str) -> &'a str {
    let pat_prefix = match tag {
        "8" => "",
        _ => std::str::from_utf8(&[SOH as u8]).unwrap(),
    };
    let pat = format!("{}{}=", pat_prefix, tag);
    if let Some(indx) = s.find(pat.as_str()) {
        let field_start_pos = indx + pat_prefix.len();
        // ignore the first SOH prefix, if any, and start from tag
        let end_pos = s[field_start_pos..].find(SOH).unwrap();
        let start_pos = s[field_start_pos..].find('=').unwrap();
        return &s[field_start_pos + start_pos + 1..field_start_pos + end_pos];
    }
    ""
}

// Consumes the flat, wire-ordered queue of fields into a structured `Message`, in the
// only order a FIX message can actually appear on the wire: header, then body, then
// trailer. `v` is shared, mutable state across all three calls — each `parse_*` function
// pops fields off the *front* of the queue for as long as they belong to its section,
// then leaves whatever's left (starting with the first field it doesn't recognize) for
// the next call. That's why they take `&mut VecDeque` rather than returning a remainder.
fn from_vec(mut v: VecDeque<StringField>, dd: &DataDictionary) -> SessResult<Message> {
    let mut message = Message::new();
    // Too few fields to possibly hold BeginString/BodyLength/MsgType as v[0]/v[1]/v[2] —
    // bail before any indexing below, which would otherwise panic on out-of-bounds access
    // for a short/malformed message instead of rejecting it cleanly.
    if v.len() < 3 {
        return Err(SessionRejectError::tag_specified_out_of_order());
    }
    // validate the first 3 fields and the last one
    if v[0].tag() != BeginString::field()
        || v[1].tag() != BodyLength::field()
        || v[2].tag() != MsgType::field()
        || v[v.len() - 1].tag() != CheckSum::field()
    {
        return Err(SessionRejectError::tag_specified_out_of_order());
    }
    let body_len_field = &v[1];
    let expected_body_len = body_len_field
        .value()
        .parse::<u32>()
        .map_err(|_| SessionRejectError::invalid_body_len_err())?;
    // Body length excludes BeginString(8)/BodyLength(9)/CheckSum(10) — same rule as
    // Message::calc_body_len, just computed over the raw incoming fields instead of an
    // already-built Message's header/body/trailer.
    let actual_body_len: usize = v
        .iter()
        .filter(|sfield| sfield.tag() != 8 && sfield.tag() != 9 && sfield.tag() != 10)
        .map(|sfield| sfield.to_string().as_bytes().len())
        .sum();
    if actual_body_len as u32 != expected_body_len {
        return Err(SessionRejectError::invalid_body_len_err());
    }
    let expected_checksum = v[v.len() - 1]
        .value()
        .parse::<u32>()
        .map_err(|_| SessionRejectError::invalid_checksum())?;
    let actual_total_bytes_sum: u32 = v
        .iter()
        .filter(|sfield| sfield.tag() != 10)
        .flat_map(|sfield| sfield.to_string().into_bytes())
        .map(|b| b as u32)
        .sum();
    let actual_check_sum = actual_total_bytes_sum % 256;
    if actual_check_sum != expected_checksum {
        return Err(SessionRejectError::invalid_checksum());
    }
    parse_header(&mut v, message.header_mut(), dd)?;
    validate_required_tag_missing(HEADER_ID, message.header(), dd)?;
    parse_body(&mut v, &mut message, dd)?;
    validate_required_tag_missing(&message.get_msg_type().unwrap(), message.body(), dd)?;
    parse_trailer(&mut v, message.trailer_mut(), dd)?;
    // One pass over every field in the fully-built message — header, body, trailer, and
    // every group instance at any nesting depth (FieldMap::iter() already recurses into
    // groups) — checking each field's value against its declared type/enum-values. Kept
    // as a single post-parse sweep rather than interleaved into parse_header/parse_body/
    // parse_group: this way there's exactly one DataDictionary in scope (the true
    // top-level one), so no need to thread a second dictionary reference through group
    // recursion just for these checks.
    validate_field_values(&message, dd)?;
    Ok(message)
}

fn validate_field_values(message: &Message, dd: &DataDictionary) -> SessResult<()> {
    for field in message
        .header()
        .iter()
        .into_iter()
        .chain(message.body().iter().into_iter())
        .chain(message.trailer().iter().into_iter())
    {
        validate_tag_value_for_type(field.tag(), field.value(), dd)?;
        validate_tag_for_value_range(field.tag(), field.value(), dd)?;
    }
    Ok(())
}

fn validate_tag_for_msgtype(tag: Tag, msg_type: &str, dd: &DataDictionary) -> SessResult<()> {
    if dd.is_msg_field(msg_type, tag) {
        return Ok(());
    } else if dd.get_field_type(tag).is_some() {
        // this field exist, since the field_type is defined. But may be not for this message type
        return Err(SessionRejectError::tag_not_defined_for_msg());
    }
    Err(SessionRejectError::undefined_tag_err())
}

fn validate_tag_for_value_range(tag: Tag, value: &String, dd: &DataDictionary) -> SessResult<()> {
    match dd.get_field_values(tag) {
        Some(valueSet) => {
            if valueSet.contains(value) {
                return Ok(());
            }
            Err(SessionRejectError::value_out_of_range_err())
        }
        None => Ok(()),
    }
}

fn validate_tag_value_for_type(tag: Tag, value: &String, dd: &DataDictionary) -> SessResult<()> {
    match dd.get_field_type(tag) {
        Some(fix_type) => {
            // Grouped by which Rust primitive the FIX spec's "Data Types" section (Vol 1)
            // says each category is built from, not by FIX category name — several
            // categories share the same underlying parse rule.
            let parsed_correctly = match fix_type {
                // "int field (see definition of int above)" for all four of these.
                FixType::Int
                | FixType::Length
                | FixType::NumInGroup
                | FixType::Seqnum
                | FixType::Tagnum => value.parse::<i64>().is_ok(),
                // "float field (see definition of float above)" for all five of these.
                // f64, not f32: spec requires accommodating up to 15 significant digits,
                // which f32 (~7 significant decimal digits) can't reliably hold.
                FixType::Float
                | FixType::Amt
                | FixType::Percentage
                | FixType::Price
                | FixType::PriceOffset
                | FixType::Qty => value.parse::<f64>().is_ok(),
                FixType::Char => value.chars().count() == 1,
                // Boolean is a char restricted to exactly 'Y'/'N' per the spec — not
                // Rust's bool parsing, which accepts "true"/"false", neither valid here.
                FixType::Boolean => value == "Y" || value == "N",
                // String-shaped categories (each has its own date/ISO-code sub-grammar,
                // e.g. UtcTimestamp's YYYYMMDD-HH:MM:SS[.sss]) — not validated here, out
                // of scope for this pass per TASKS.md's "numeric/Boolean, etc." wording.
                FixType::Data
                | FixType::Str
                | FixType::Country
                | FixType::Currency
                | FixType::Exchange
                | FixType::LocalMktDate
                | FixType::MonthYear
                | FixType::MultipleValueString
                | FixType::UtcDate
                | FixType::UtcTimeOnly
                | FixType::UtcTimestamp
                | FixType::Unknown => true,
            };
            if parsed_correctly {
                Ok(())
            } else {
                Err(SessionRejectError::incorrect_data_format_err())
            }
        }
        // Same as the earlier tag-membership check: by the time this runs, the tag's
        // membership has already been validated upstream, so get_field_type returning
        // None here shouldn't actually be reachable in practice.
        None => Err(SessionRejectError::undefined_tag_err()),
    }
}

fn validate_required_tag_missing(
    msg_type: &str,
    field_map: &FieldMap,
    dd: &DataDictionary,
) -> SessResult<()> {
    let required_fields = dd.get_msg_required_field(msg_type);
    match required_fields {
        Some(req_fields) => {
            let current_fields: Vec<u32> =
                field_map.iter().into_iter().map(|s: &StringField| s.tag()).collect();
            let all_pressent = req_fields.iter().all(|f| current_fields.contains(f));
            if all_pressent {
                return Ok(());
            }
            Err(SessionRejectError::required_tag_missing_err())
        }
        None => Ok(()),
    }
}

// Parses one repeating group (e.g. `268=2` NoMDEntries followed by two MDEntry
// instances) starting right after its count field `fld` (268=2) has already been popped
// by the caller. `fld.tag()` is the group's count tag; `fld.value()` is the declared
// number of instances (`declared_count`) — NOT yet verified against how many instances
// actually show up, that's what this function checks.
//
// Every group has its own delimiter tag: the first field of each instance (e.g. tag 269,
// MDEntryType, for NoMDEntries) — seeing that tag again is how we know a new instance
// has started, since groups aren't wrapped in any explicit begin/end marker on the wire.
fn parse_group(
    v: &mut VecDeque<StringField>,
    msg_type: &str,
    fld: &StringField,
    fmap: &mut FieldMap,
    dd: &DataDictionary,
) -> SessResult<()> {
    let rg = dd
        .get_msg_group(msg_type, fld.tag())
        .ok_or_else(SessionRejectError::tag_not_defined_for_msg)?;
    // Groups can nest (a group whose instances themselves contain a group), so each group
    // gets its own little DataDictionary scoped to just its own fields/sub-groups.
    let rg_dd = rg.data_dictionary();
    // The dictionary-declared field order *within one instance* of this group — used both
    // to enforce "fields inside a group instance must appear in declared order" below, and
    // (via `set_field_order`) to sort each instance back into that order on output.
    let field_order = rg_dd.get_ordered_fields();
    let group_count_tag = fld.tag();
    let declared_count = match fld.value().parse::<u32>() {
        Ok(c) => c,
        Err(e) => return Err(SessionRejectError::incorrect_data_format_err()),
    };
    let delimiter = rg.delimiter();
    // Pre-allocates `declared_count` empty FieldMap slots up front; the loop below fills
    // them in by index as instances are recognized on the wire.
    let group = fmap.set_group(fld.tag(), declared_count, delimiter);
    // `actual_count`: number of group instances started so far, bumped each time the
    // delimiter tag reappears. `0` means "no instance started yet" — a real, in-range
    // usize value, since this counts instances rather than indexing into `group[..]`
    // directly (indexing uses `actual_count - 1`, only reached after the `actual_count
    // == 0` guards below have already ruled out "no instance yet").
    let mut actual_count: usize = 0;
    // `previous_offset`: 1-based rank (declared `field_order` position + 1) of the last
    // field seen *within the current instance*; reset to `0` (nothing seen yet) each time
    // a new instance starts. The `+1` shift means a real rank can never collide with the
    // "nothing seen yet" value, so plain `usize` works here without needing `Option`.
    // Used to detect a field appearing out of its declared order inside one instance.
    let mut previous_offset: usize = 0;
    // Consume fields off the shared queue one at a time. This loop doesn't know in advance
    // how many fields belong to the group overall (only how many *instances* are
    // declared) — it keeps going until it pops a field that isn't part of this group at
    // all, at which point it must push that field back (see the final `else` below) so
    // whichever caller invoked us (parse_header/parse_body/an outer parse_group) can
    // process it instead.
    while let Some(next_field) = v.pop_front() {
        if next_field.tag() == delimiter {
            // Start of a new instance.
            actual_count += 1;
            if actual_count > declared_count as usize {
                // We've seen more instances than the count field declared.
                // incorrect NumInGroups
                return Err(SessionRejectError::incorrect_num_in_grp_count());
            }
            // resetting previous offset
            previous_offset = 0;
            let group_instance = &mut group[actual_count - 1];
            group_instance.set_field_order(&field_order);
            if rg_dd.is_msg_group(msg_type, next_field.tag()) {
                // The delimiter field itself starts a nested group (rare, but structurally
                // possible) — recurse into it instead of just storing it as a plain field.
                parse_group(v, msg_type, &next_field, group_instance, rg_dd)?;
            } else {
                group_instance.set_field(next_field);
            }
        } else if rg_dd.is_msg_group(msg_type, next_field.tag()) {
            // A nested group's count field, appearing partway through the current instance.
            if actual_count == 0 {
                // delimiter not found but other tag is encountered
                return Err(SessionRejectError::required_tag_missing_err());
            }
            let group_instance = &mut group[actual_count - 1];
            parse_group(v, msg_type, &next_field, group_instance, rg_dd)?;
        } else if rg_dd.is_msg_field(msg_type, next_field.tag()) {
            // An ordinary (non-delimiter, non-group) field belonging to the current instance.
            // Deliberately not calling validate_tag_for_msgtype here (or in the delimiter/
            // nested-group branches above): this `else if` condition already *is* that
            // check — `rg_dd.is_msg_field(...)` just evaluated to true to reach this
            // branch, so re-validating membership here would only ever hit the Ok(())
            // path; the tag_not_defined_for_msg/undefined_tag_err branches are provably
            // unreachable. Same reasoning covers the delimiter branch above: the
            // delimiter tag is always the group's own first declared field (that's how
            // `rg.delimiter()` is derived from the XML), so it's unconditionally a member
            // too. The one place a field genuinely hasn't been validated yet by anything
            // is the final `else` below — and that's exactly why it doesn't try to
            // validate it itself, just hands it back to the caller (parse_header/
            // parse_body/an outer parse_group) to check in its own context instead.
            if actual_count == 0 {
                // means first field not found i.e. delimiter
                return Err(SessionRejectError::required_tag_missing_err());
            }
            // verify the order of fields, 1-based rank
            let offset = field_order.iter().position(|f| *f == next_field.tag()).unwrap() + 1;
            if offset < previous_offset {
                // in groups, fields have an order. if a next_field (under-process) ranks is less than last offset
                // means the field is out of order. for e.g. if the order [W, Z] and we have already gotten
                // Z (previous_offset = 2), and then we get W whose offset/rank is 1 in order then its out of order
                return Err(SessionRejectError::repeating_grp_out_of_order());
            }
            let group_instance = &mut group[actual_count - 1];
            group_instance.set_field(next_field);
            previous_offset = offset;
        } else {
            // This field belongs to whatever comes after the group (the rest of the body, or
            // the trailer) — not part of this group at all. Put it back and stop; the caller
            // that invoked parse_group resumes consuming the queue from here.
            // its not a group field, push back and come out
            v.push_front(next_field);
            break;
        }
    }
    // Now that we've stopped (either the queue ran dry, or we hit a non-group field),
    // confirm the number of instances we actually built matches what was declared —
    // catches a declared count that's too *high* (too-low was already caught above).
    if actual_count != declared_count as usize {
        // means actual repeating groups are less then declared count
        return Err(SessionRejectError::incorrect_num_in_grp_count());
    }
    // at here, the group's parsing is complete, check if any required tag is missing for each instance
    for count in 0..declared_count as usize {
        let group_instance = &group[count];
        validate_required_tag_missing(msg_type, &group_instance, rg_dd)?
    }
    Ok(())
}

// Consumes header fields from the front of the queue until it sees the first field that
// *isn't* a header field, at which point it pushes that field back (see `from_vec`'s
// comment on this hand-off pattern) and returns — leaving the rest of the queue for
// `parse_body`.
fn parse_header(
    v: &mut VecDeque<StringField>,
    header: &mut FieldMap,
    dd: &DataDictionary,
) -> SessResult<()> {
    // Spec rule: BeginString(8), BodyLength(9), MsgType(35) must be the literal first three
    // fields of every message, in this order — already checked in from_vec (which also
    // needs it to safely index into the first-three/last fields for body-length/checksum
    // verification, so it runs before this function is even called).
    while let Some(fld) = v.pop_front() {
        if !dd.is_header_field(fld.tag()) {
            // start of body
            v.push_front(fld);
            return Ok(());
        } else if dd.is_msg_group(HEADER_ID, fld.tag()) {
            // The header's own repeating group (NoHops) — HEADER_ID is used here as the
            // "message type" so parse_group can look group info up in the header's own section
            // of the dictionary rather than a real message type's.
            parse_group(v, HEADER_ID, &fld, header, dd)?;
        } else {
            header.set_field(fld);
        }
    }
    Ok(())
}

// Consumes body fields (and body-level repeating groups) until it sees the first
// trailer field, then pushes that field back and returns — same hand-off pattern as
// parse_header, this time at the body/trailer boundary.
fn parse_body(
    v: &mut VecDeque<StringField>,
    msg: &mut Message,
    dd: &DataDictionary,
) -> SessResult<()> {
    // MsgType (35) was already parsed into the header by parse_header; read it back out to
    // know which message-type-specific dictionary rules apply to everything below.
    let msg_type = match msg.get_msg_type() {
        Ok(s) => s,
        Err(_) => return Err(SessionRejectError::required_tag_missing_err()),
    };
    while let Some(fld) = v.pop_front() {
        if dd.is_header_field(fld.tag()) {
            // A header field showing up after the body has started is never valid — header
            // fields only belong at the very front of the message (parse_header already
            // consumed all of them), so seeing one here is a genuine ordering violation, not
            // just "not part of the body" (contrast with the trailer case just below, which is
            // a normal, expected end-of-body signal).
            return Err(SessionRejectError::tag_specified_out_of_order());
        }
        if dd.is_trailer_field(fld.tag()) {
            v.push_front(fld);
            return Ok(());
        }
        if dd.is_msg_group(msg_type.as_str(), fld.tag()) {
            parse_group(v, &msg_type, &fld, &mut msg.body, dd)?;
        } else {
            validate_tag_for_msgtype(fld.tag(), &msg_type, dd)?;
            msg.set_field(fld);
        }
    }
    Ok(())
}

// Consumes everything left in the queue as trailer fields. Unlike parse_header/
// parse_body, there's no "next section" to hand off to — the trailer is always last —
// so any field here that *isn't* a trailer field is simply invalid, not a hand-off
// signal.
fn parse_trailer(
    v: &mut VecDeque<StringField>,
    trailer: &mut FieldMap,
    dd: &DataDictionary,
) -> SessResult<()> {
    while let Some(fld) = v.pop_front() {
        if !dd.is_trailer_field(fld.tag()) {
            return Err(SessionRejectError::tag_specified_out_of_order());
        }
        trailer.set_field(fld);
    }
    Ok(())
}

// pub const SAMPLE_MSG: &str = "8=FIX.4.2|9=251|35=D|49=AFUNDMGR|56=ABROKER|34=2|52=2003061501:14:49|11=12345|1=111111|63=0|64=20030621|21=3|110=1000|111=50000|55=IBM|48=459200101|22=1|54=1|60=2003061501:14:49|38=5000|40=1|44=15.75|15=USD|59=0|10=127|";

pub fn test_logon() -> Message {
    let mut heartbeat = Message::new();
    heartbeat.header_mut().set_field(StringField::new(8, "FIX.4.3"));
    heartbeat.header_mut().set_field(StringField::new(35, "A"));
    heartbeat.header_mut().set_field(StringField::new(34, "1"));
    heartbeat.header_mut().set_field(StringField::new(49, "FIXIMULATOR"));
    heartbeat.header_mut().set_field(StringField::new(56, "BANZAI"));
    heartbeat.set_field(StringField::new(98, "0"));
    heartbeat.set_field(StringField::new(108, "30"));
    heartbeat.set_sending_time();
    heartbeat.set_body_len();
    heartbeat.set_checksum();
    heartbeat
}

#[cfg(test)]
mod message_test {
    use super::*;
    #[cfg(test)]
    use crate::data_dictionary::*;
    use crate::quickfix_errors::SessionRejectReason;
    use assert_matches::*;
    use lazy_static::*;

    const MSG_STR: &str = "8=FIX.4.3|9=72|35=A|34=0|49=BANZAI|52=20221006-08:43:36.522|56=FIXIMULATOR|98=0|108=30|10=004|";
    lazy_static! {
        static ref DD: DataDictionary = DataDictionary::from_xml("resources/FIX43.xml");
    }

    fn soh_replaced_str(s: &str) -> String {
        let mut buff = [0u8; 1];
        s.replace('|', SOH.encode_utf8(&mut buff))
    }

    #[test]
    fn msg_test_simple_no_group() {
        let msg = Message::from_str(&soh_replaced_str(MSG_STR), &DD);
        assert!(msg.is_ok());
        let msg = msg.unwrap();
        assert_eq!(msg.get_msg_type().unwrap(), "A");
        assert_eq!(msg.header().get_field::<String>(8).unwrap(), "FIX.4.3");
    }

    #[test]
    fn msg_test_with_header_group() {
        // header having a group, verify that its parsed
        // header with NoHops repeating group
        let msg_with_header: &str = "8=FIX.4.3|9=124|35=A|34=0|49=BANZAI|52=20221006-08:43:36.522|56=FIXIMULATOR|627=1|628=hopcompid|629=20221006-08:43:36.522|630=0|98=0|108=30|10=244|";
        let msg = Message::from_str(&soh_replaced_str(msg_with_header), &DD);
        assert!(msg.is_ok());
        let msg = msg.unwrap();
        assert!(msg.header().get_group(627).is_some());
        let header_group = msg.header().get_group(627).unwrap();
        assert_eq!(header_group.delim(), 628);
        assert_eq!(header_group.size(), 1);
        assert_eq!(header_group[0].get_field::<String>(628).unwrap(), "hopcompid");
        assert_eq!(header_group[0].get_field::<String>(629).unwrap(), "20221006-08:43:36.522");
        assert_eq!(header_group[0].get_field::<u32>(630).unwrap(), 0);
    }

    #[test]
    fn msg_test_with_body_group() {
        // message body having groups
        let msg_body_with_repeating_group = "8=FIX.4.4|9=134|35=W|34=2|49=GEMINI|52=20180425-17:51:40.787|56=TRADEBOTMD002|55=BTCUSD|262=2|268=2|269=0|270=8490.07|271=10|269=1|270=8519.57|271=20|10=013|";
        let msg = Message::from_str(&soh_replaced_str(msg_body_with_repeating_group), &DD);
        assert!(msg.is_ok());
        let msg = msg.unwrap();
        assert_eq!(msg.get_msg_type().unwrap(), "W");
        assert!(msg.get_group(268).is_some());
        let md_entries_grp = msg.get_group(268).unwrap();
        assert_eq!(md_entries_grp.delim(), 269);
        assert_eq!(md_entries_grp.size(), 2);
        assert_eq!(md_entries_grp[0].get_field::<u32>(269).unwrap(), 0);
        assert_eq!(md_entries_grp[0].get_field::<f32>(270).unwrap(), 8490.07);
        assert_eq!(md_entries_grp[0].get_field::<u32>(271).unwrap(), 10);

        assert_eq!(md_entries_grp[1].get_field::<u32>(269).unwrap(), 1);
        assert_eq!(md_entries_grp[1].get_field::<f32>(270).unwrap(), 8519.57);
        assert_eq!(md_entries_grp[1].get_field::<u32>(271).unwrap(), 20);
    }

    #[test]
    fn msg_test_with_group_and_subgroups() {
        // body having repeating groups having subgroups
        let new_order_list = "8=FIX.4.4|9=215|35=E|34=2|49=GEMINI|52=20180425-17:51:40.787|56=TRADEBOTMD002|66=list_id|394=1|68=2|73=2|11=ClientOrderId1|67=1|78=2|79=AllocAct11|80=10|79=AllocAct12|80=20|54=1|11=ClientOrderId2|67=2|78=1|79=AllocAct21|80=30|54=1|10=222";
        let msg = Message::from_str(&soh_replaced_str(new_order_list), &DD);
        assert!(msg.is_ok());
        let msg = msg.unwrap();
        assert_eq!(msg.get_msg_type().unwrap(), "E");
        assert!(msg.get_group(73).is_some());
        let no_orders_grp = msg.get_group(73).unwrap();
        assert_eq!(no_orders_grp.delim(), 11);
        assert_eq!(no_orders_grp.size(), 2);
        assert_eq!(no_orders_grp[0].get_field::<String>(11).unwrap(), "ClientOrderId1");
        assert_eq!(no_orders_grp[0].get_field::<u32>(67).unwrap(), 1);

        let no_alloc_subgrp = no_orders_grp[0].get_group(78);
        assert!(no_alloc_subgrp.is_some());
        let no_alloc_subgrp = no_alloc_subgrp.unwrap();
        assert_eq!(no_alloc_subgrp.size(), 2);
        assert_eq!(no_alloc_subgrp[0].get_field::<String>(79).unwrap(), "AllocAct11");
        assert_eq!(no_alloc_subgrp[0].get_field::<u32>(80).unwrap(), 10);

        assert_eq!(no_alloc_subgrp[1].get_field::<String>(79).unwrap(), "AllocAct12");
        assert_eq!(no_alloc_subgrp[1].get_field::<u32>(80).unwrap(), 20);

        assert_eq!(no_orders_grp[1].get_field::<String>(11).unwrap(), "ClientOrderId2");
        assert_eq!(no_orders_grp[1].get_field::<u32>(67).unwrap(), 2);

        let no_alloc_subgrp2 = no_orders_grp[1].get_group(78);
        assert!(no_alloc_subgrp2.is_some());
        let no_alloc_subgrp2 = no_alloc_subgrp2.unwrap();
        assert_eq!(no_alloc_subgrp2.size(), 1);
        assert_eq!(no_alloc_subgrp2[0].get_field::<String>(79).unwrap(), "AllocAct21");
        assert_eq!(no_alloc_subgrp2[0].get_field::<u32>(80).unwrap(), 30);
    }

    fn assert_round_trip(fixture: &str) {
        // wire format always ends with an SOH-terminated checksum field; a couple of the
        // fixtures above omit the trailing delimiter, so normalize before comparing.
        let mut expected = soh_replaced_str(fixture);
        if !expected.ends_with(SOH) {
            expected.push(SOH);
        }
        let msg = Message::from_str(&expected, &DD);
        assert!(msg.is_ok());
        assert_eq!(msg.unwrap().to_string(), expected);
    }

    #[test]
    fn msg_test_round_trip_no_group() {
        assert_round_trip(MSG_STR);
    }

    #[test]
    fn msg_test_round_trip_header_group() {
        assert_round_trip(
            "8=FIX.4.3|9=124|35=A|34=0|49=BANZAI|52=20221006-08:43:36.522|56=FIXIMULATOR|627=1|628=hopcompid|629=20221006-08:43:36.522|630=0|98=0|108=30|10=244|",
        );
    }

    #[test]
    fn msg_test_round_trip_body_group() {
        assert_round_trip(
            "8=FIX.4.4|9=134|35=W|34=2|49=GEMINI|52=20180425-17:51:40.787|56=TRADEBOTMD002|55=BTCUSD|262=2|268=2|269=0|270=8490.07|271=10|269=1|270=8519.57|271=20|10=013|",
        );
    }

    #[test]
    fn msg_test_round_trip_group_and_subgroups() {
        assert_round_trip(
            "8=FIX.4.4|9=215|35=E|34=2|49=GEMINI|52=20180425-17:51:40.787|56=TRADEBOTMD002|66=list_id|394=1|68=2|73=2|11=ClientOrderId1|67=1|78=2|79=AllocAct11|80=10|79=AllocAct12|80=20|54=1|11=ClientOrderId2|67=2|78=1|79=AllocAct21|80=30|54=1|10=222",
        );
    }

    #[test]
    fn msg_test_required_field_missing_at_group_level() {
        // MassQuote (35=i): NoQuoteSets(296) group's own required fields are
        // QuoteSetID(302) and TotQuoteEntries(304), on top of its nested NoQuoteEntries(295)
        // group. TotQuoteEntries is omitted here, so the check on this NoQuoteSets instance
        // (rg_dd's own required set, not the top-level message's) should catch it.
        let mass_quote = "8=FIX.4.3|9=101|35=i|34=1|49=BANZAI|52=20221006-08:43:36.522|56=FIXIMULATOR|117=QID1|296=1|302=QSID1|295=1|299=QEID1|10=173";
        let msg = Message::from_str(&soh_replaced_str(mass_quote), &DD);
        assert!(msg.is_err());
        assert_matches!(msg.unwrap_err().kind(), SessionRejectReason::RequiredTagMissing);
    }

    #[test]
    fn msg_test_required_field_missing_at_subgroup_level() {
        // Same message type; NoQuoteSets' own required fields (QuoteSetID, TotQuoteEntries)
        // are all present, but the nested NoQuoteEntries(295) instance is missing its field
        // QuoteEntryID(299). Checked the whole FIX43.xml dictionary: every group nested two
        // or more levels deep has either no required fields at all, or exactly one, and that
        // one is always the group's own delimiter (NoQuoteEntries here is no exception —
        // QuoteEntryID is both its only required field and its delimiter). So omitting it
        // doesn't surface as "instance present but missing a required field" — the instance
        // never gets recognized as started at all, which is IncorrectNumInGroupCountForRepeatingGroup
        // (0 instances found vs. 1 declared), not RequiredTagMissing. This still exercises
        // the recursive parse_group call reaching NoQuoteEntries correctly and propagating a
        // real error back up through NoQuoteSets — just not this specific reason.
        let mass_quote = "8=FIX.4.3|9=97|35=i|34=1|49=BANZAI|52=20221006-08:43:36.522|56=FIXIMULATOR|117=QID1|296=1|302=QSID1|304=1|295=1|10=091";
        let msg = Message::from_str(&soh_replaced_str(mass_quote), &DD);
        assert!(msg.is_err());
        assert_matches!(
            msg.unwrap_err().kind(),
            SessionRejectReason::IncorrectNumInGroupCountForRepeatingGroup
        );
    }

    #[test]
    fn msg_test_required_field_missing_no_group() {
        // TestRequest (35=1) has no groups at all — its only field, TestReqID(112), is
        // required and omitted here. This exercises the plain from_vec-level body check
        // (validate_required_tag_missing called directly on msg.body(), not via parse_group),
        // for a message type that never goes anywhere near a repeating group.
        let test_request =
            "8=FIX.4.3|9=60|35=1|34=1|49=BANZAI|52=20221006-08:43:36.522|56=FIXIMULATOR|10=217";
        let msg = Message::from_str(&soh_replaced_str(test_request), &DD);
        assert!(msg.is_err());
        assert_matches!(msg.unwrap_err().kind(), SessionRejectReason::RequiredTagMissing);
    }

    #[test]
    fn msg_test_trailer_with_more_fields() {
        // trailer having all of the trailer's declared fields (SignatureLength,
        // Signature, CheckSum), not just the mandatory CheckSum, and verifying they're
        // all parsed into the trailer correctly.
        let test_request = "8=FIX.4.3|9=97|35=1|34=1|49=BANZAI|52=20221006-08:43:36.522|56=FIXIMULATOR|112=TESTID1|93=15|89=abc123signature|10=000";
        let msg = Message::from_str(&soh_replaced_str(test_request), &DD);
        assert!(msg.is_ok());
        let msg = msg.unwrap();
        assert_eq!(msg.trailer().get_field::<u32>(93).unwrap(), 15);
        assert_eq!(msg.trailer().get_field::<String>(89).unwrap(), "abc123signature");
        assert_eq!(msg.trailer().get_field::<String>(10).unwrap(), "000");
    }

    #[test]
    fn msg_test_undefined_tag() {
        // TestRequest(35=1) with an extra tag (99999) that isn't defined anywhere in
        // the FIX43.xml dictionary at all — validate_tag_for_msgtype's fallback branch.
        let test_request = "8=FIX.4.3|9=86|35=1|34=1|49=BANZAI|52=20221006-08:43:36.522|56=FIXIMULATOR|112=TESTID1|99999=garbage|10=213";
        let msg = Message::from_str(&soh_replaced_str(test_request), &DD);
        assert!(msg.is_err());
        assert_matches!(msg.unwrap_err().kind(), SessionRejectReason::UndefinedTag);
    }

    #[test]
    fn msg_test_tag_not_defined_for_msgtype() {
        // TestRequest(35=1) with Price(44) added — a real FIX tag (used by plenty of
        // other message types, e.g. NewOrderSingle) but not one of TestRequest's own
        // declared fields, so it should be rejected as "known tag, wrong message type"
        // rather than "undefined tag".
        let test_request = "8=FIX.4.3|9=81|35=1|34=1|49=BANZAI|52=20221006-08:43:36.522|56=FIXIMULATOR|112=TESTID1|44=15.75|10=082";
        let msg = Message::from_str(&soh_replaced_str(test_request), &DD);
        assert!(msg.is_err());
        assert_matches!(msg.unwrap_err().kind(), SessionRejectReason::TagNotDefinedForMsgType);
    }

    #[test]
    fn msg_test_value_out_of_enum_range() {
        // Logon(35=A) with EncryptMethod(98) set to 9 — a well-formed int, but not one
        // of the dictionary's declared enum values for this tag (0-6 per FIX43.xml).
        let logon = "8=FIX.4.3|9=72|35=A|34=0|49=BANZAI|52=20221006-08:43:36.522|56=FIXIMULATOR|98=9|108=30|10=013";
        let msg = Message::from_str(&soh_replaced_str(logon), &DD);
        assert!(msg.is_err());
        assert_matches!(msg.unwrap_err().kind(), SessionRejectReason::ValueOutOfRange);
    }

    #[test]
    fn msg_test_incorrect_data_format() {
        // Logon(35=A) with MsgSeqNum(34) — a header field of type SEQNUM — set to a
        // non-numeric string instead of an integer.
        let logon = "8=FIX.4.3|9=74|35=A|34=abc|49=BANZAI|52=20221006-08:43:36.522|56=FIXIMULATOR|98=0|108=30|10=252";
        let msg = Message::from_str(&soh_replaced_str(logon), &DD);
        assert!(msg.is_err());
        assert_matches!(msg.unwrap_err().kind(), SessionRejectReason::IncorrectDataFormatForValue);
    }

    #[test]
    fn msg_test_invalid_checksum() {
        // Same Logon fixture as MSG_STR, with the checksum's last digit flipped
        // (004 -> 005) — body length (9=72) is still correct, so this should be caught
        // specifically by the checksum comparison, not the body-length one.
        let logon = "8=FIX.4.3|9=72|35=A|34=0|49=BANZAI|52=20221006-08:43:36.522|56=FIXIMULATOR|98=0|108=30|10=005";
        let msg = Message::from_str(&soh_replaced_str(logon), &DD);
        assert!(msg.is_err());
        assert_matches!(msg.unwrap_err().kind(), SessionRejectReason::InvalidChecksum);
    }

    #[test]
    fn msg_test_invalid_body_length() {
        // Same Logon fixture, with the body length's last digit flipped (72 -> 73).
        // Body length is checked before checksum in from_vec, so this is caught first
        // even though the checksum field itself is left correct for the original body.
        let logon = "8=FIX.4.3|9=73|35=A|34=0|49=BANZAI|52=20221006-08:43:36.522|56=FIXIMULATOR|98=0|108=30|10=004";
        let msg = Message::from_str(&soh_replaced_str(logon), &DD);
        assert!(msg.is_err());
        assert_matches!(msg.unwrap_err().kind(), SessionRejectReason::InvalidBodyLength);
    }

    fn msg_test_soh_in_data_field() {}

    fn msg_test_soh_in_non_data_field() {}
}
