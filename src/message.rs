use getset::{CopyGetters, Getters, MutGetters};
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::fmt::{write, Display};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use crate::data_dictionary::{DataDictionary, HEADER_ID};
use crate::fields::*;
use crate::quickfix_errors::SessionRejectError;
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

    // pub fn tag(&self) -> u32 {
    //     self.tag
    // }

    // pub fn value(&self) -> &str {
    //     self.value.as_str()
    // }
}

impl Display for StringField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}{}", self.tag, self.value, SOH)
    }
}

#[derive(Debug, Default, Clone)]
pub struct FieldMap {
    fields: HashMap<Tag, StringField>,
    group: HashMap<Tag, Group>,
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

    pub fn get_field<T: FromStr>(&self, tag: u32) -> Result<T, String> {
        if let Some(field) = self.fields.get(&tag) {
            return field.value.parse::<T>().map_err(|_| "could not parse".to_string());
        }
        Err("not found".to_string())
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

    pub fn iter(&self) -> FieldMapIter {
        let mut map_iter = FieldMapIter::default();
        map_iter.fieldmap_to_vec(self);
        map_iter
    }

    fn is_ordered_field(&self, tag: Tag) -> bool {
        self.field_order.contains(&tag)
    }

    fn index_comparator(&self, tag1: Tag, tag2: Tag) -> Ordering {
        let field_index = |field: Tag| {
            self.field_order
                .iter()
                .position(|needle| *needle == field)
                .map_or(usize::MAX, |pos| pos)
        };
        field_index(tag1).cmp(&field_index(tag2))
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
            temp_vec.sort_by_cached_key(|&field| {
                field_map
                    .field_order
                    .iter()
                    .position(|needle| *needle == field.tag())
                    .map_or(usize::MAX, |pos| pos)
            })
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
    pub header: Header,
    pub body: FieldMap,
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

    pub fn get_field<T: FromStr>(&self, tag: Tag) -> Result<T, String> {
        self.body.get_field(tag)
    }

    pub fn set_group(&mut self, tag: Tag, value: u32, rep_grp_delimiter: Tag) -> &mut Group {
        self.body.set_group(tag, value, rep_grp_delimiter)
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

    fn get_msg_type(&self) -> Result<String, String> {
        self.header.get_field::<String>(35)
    }

    pub fn set_sending_time(&mut self) {
        let curr_time = chrono::Utc::now();
        let sending_time = curr_time.format("%Y%m%d-%T%.3f").to_string();
        self.header_mut().set_field(StringField::new(52, &sending_time));
    }

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

    pub fn get_session_id(s: &str) -> SessionId {
        SessionIdBuilder::default()
            .begin_string(extract_field_value("8", s))
            .sender_compid(extract_field_value("49", s))
            .sender_subid(extract_field_value("50", s))
            .sender_locationid(extract_field_value("142", s))
            .target_compid(extract_field_value("56", s))
            .target_subid(extract_field_value("57", s))
            .target_locationid(extract_field_value("143", s))
            .build()
            .unwrap()
    }

    pub fn get_reverse_session_id(s: &str) -> SessionId {
        // sender values from message is put into target & vice-versa
        SessionIdBuilder::default()
            .begin_string(extract_field_value("8", s))
            .sender_compid(extract_field_value("56", s))
            .sender_subid(extract_field_value("57", s))
            .sender_locationid(extract_field_value("143", s))
            .target_compid(extract_field_value("49", s))
            .target_subid(extract_field_value("50", s))
            .target_locationid(extract_field_value("142", s))
            .build()
            .unwrap()
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.header(), self.body, self.trailer())
    }
}

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

fn from_vec(mut v: VecDeque<StringField>, dd: &DataDictionary) -> SessResult<Message> {
    let mut message = Message::new();
    parse_header(&mut v, message.header_mut(), dd)?;
    parse_body(&mut v, &mut message, dd)?;
    parse_trailer(&mut v, message.trailer_mut(), dd)?;
    Ok(message)
}

fn parse_group(
    v: &mut VecDeque<StringField>, msg_type: &str, fld: &StringField, fmap: &mut FieldMap,
    dd: &DataDictionary,
) -> SessResult<()> {
    let rg = dd
        .get_msg_group(msg_type, fld.tag())
        .ok_or_else(SessionRejectError::tag_not_defined_for_msg)?;
    let rg_dd = rg.data_dictionary();
    let field_order = rg_dd.get_ordered_fields();
    let group_count_tag = fld.tag();
    let declared_count = match fld.value().parse::<u32>() {
        Ok(c) => c,
        Err(e) => return Err(SessionRejectError::incorrect_data_format_err()),
    };
    let delimiter = rg.delimiter();
    let group = fmap.set_group(fld.tag(), declared_count, delimiter);
    let mut actual_count: i32 = -1;
    let mut previous_offset: i32 = -1;
    while let Some(next_field) = v.pop_front() {
        if next_field.tag() == delimiter {
            actual_count += 1;
            if actual_count + 1 >= declared_count as i32 {
                // incorrect NumInGroups
                return Err(SessionRejectError::incorrect_num_in_grp_count());
            }
            // resetting previous offset
            previous_offset = -1;
            let group_instance = &mut group[actual_count as usize];
            group_instance.set_field_order(&field_order);
            if rg_dd.is_msg_group(msg_type, next_field.tag()) {
                parse_group(v, msg_type, &next_field, group_instance, dd)?;
            } else {
                group_instance.set_field(next_field);
            }
        } else if rg_dd.is_msg_group(msg_type, next_field.tag()) {
            if actual_count < 0 {
                return Err(SessionRejectError::required_tag_missing_err());
            }
            let group_instance = &mut group[actual_count as usize];
            parse_group(v, msg_type, &next_field, group_instance, dd)?;
        } else if rg_dd.is_msg_field(msg_type, next_field.tag()) {
            if actual_count < 0 {
                // means first field not found i.e. delimiter
                return Err(SessionRejectError::required_tag_missing_err());
            }
            // verify the order of fields
            let offset = field_order.iter().position(|f| *f == next_field.tag()).unwrap() as i32;
            if offset < previous_offset {
                // means the field is out of order
                return Err(SessionRejectError::repeating_grp_out_of_order());
            }
            let group_instance = &mut group[actual_count as usize];
            group_instance.set_field(next_field);
            previous_offset = offset;
        } else {
            // its not a group field, push back and come out
            v.push_front(next_field);
            break;
        }
    }
    if actual_count + 1 != declared_count as i32 {
        // means actual repeating groups are less then declared count
        return Err(SessionRejectError::incorrect_num_in_grp_count());
    }
    Ok(())
}

fn parse_header(
    v: &mut VecDeque<StringField>, header: &mut FieldMap, dd: &DataDictionary,
) -> SessResult<()> {
    if v[0].tag() != BeginString::field()
        || v[1].tag() != BodyLength::field()
        || v[2].tag() != MsgType::field()
    {
        return Err(SessionRejectError::tag_specified_out_of_order());
    }
    while let Some(fld) = v.pop_front() {
        if !dd.is_header_field(fld.tag()) {
            // start of body
            v.push_front(fld);
            return Ok(());
        } else if dd.is_msg_group(HEADER_ID, fld.tag()) {
            parse_group(v, HEADER_ID, &fld, header, dd)?;
        } else {
            header.set_field(fld);
        }
    }
    Ok(())
}

fn parse_body(
    v: &mut VecDeque<StringField>, msg: &mut Message, dd: &DataDictionary,
) -> SessResult<()> {
    let msg_type = match msg.get_msg_type() {
        Ok(s) => s,
        Err(_) => return Err(SessionRejectError::required_tag_missing_err()),
    };
    while let Some(fld) = v.pop_front() {
        if dd.is_header_field(fld.tag()) {
            return Err(SessionRejectError::tag_specified_out_of_order());
        }
        if dd.is_trailer_field(fld.tag()) {
            v.push_front(fld);
            return Ok(());
        }
        if dd.is_msg_group(msg_type.as_str(), fld.tag()) {
            parse_group(v, &msg_type, &fld, &mut msg.body, dd)?;
        } else {
            msg.set_field(fld);
        }
    }
    Ok(())
}

fn parse_trailer(
    v: &mut VecDeque<StringField>, trailer: &mut FieldMap, dd: &DataDictionary,
) -> SessResult<()> {
    while let Some(fld) = v.pop_front() {
        if !dd.is_trailer_field(fld.tag()) {
            return Err(SessionRejectError::tag_specified_out_of_order());
        }
        trailer.set_field(fld);
    }
    Ok(())
}
pub const SAMPLE_MSG: &str = "8=FIX.4.2|9=251|35=D|49=AFUNDMGR|56=ABROKER|34=2|52=2003061501:14:49|11=12345|1=111111|63=0|64=20030621|21=3|110=1000|111=50000|55=IBM|48=459200101|22=1|54=1|60=2003061501:14:49|38=5000|40=1|44=15.75|15=USD|59=0|10=127|";

#[cfg(test)]
mod message_test {
    use super::*;
    #[cfg(test)]
    use crate::data_dictionary::*;
    use lazy_static::*;

    const MSG_STR: &str = "8=FIX.4.3|9=73|35=A|34=0|49=BANZAI|52=20221006-08:43:36.522|56=FIXIMULATOR|98=0|108=30|10=061|";
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
        let msg_with_header: &str =  "8=FIX.4.3|9=73|35=A|34=0|49=BANZAI|52=20221006-08:43:36.522|56=FIXIMULATOR|627=1|628=hopcompid|629=20221006-08:43:36.522|630=0|98=0|108=30|10=061|";
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

    fn msg_test_with_body_group() {
        // message body having groups
    }

    fn msg_test_with_group_and_subgroups() {
        // body having repeating groups having subgroups
    }

    fn msg_test_trailer_with_more_fields() {
        // trailer having all the fields of trailer and verify that it is parsed correctly
    }

    fn msg_test_invalid_checksum() {}

    fn msg_test_invalid_body_length() {}

    fn msg_test_soh_in_data_field() {}

    fn msg_test_soh_in_non_data_field() {}
}
