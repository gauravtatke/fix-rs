use std::collections::binary_heap::Iter;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::hash::Hash;
use std::iter::Peekable;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use crate::data_dictionary::{DataDictionary, HEADER_ID, TRAILER_ID};
use crate::fields::*;
use crate::quickfix_errors::SessionRejectError;

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
// pub const SOH: char = '\u{01}';
pub const SOH: char = '|';

#[derive(Debug, Default)]
pub struct StringField {
    tag: Tag,
    value: String,
}

impl StringField {
    pub fn new(tag: Tag, value: &str) -> Self {
        Self {
            tag,
            value: value.to_string(),
        }
    }

    pub fn tag(&self) -> u32 {
        self.tag
    }

    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}

#[derive(Debug, Default)]
pub struct FieldMap {
    fields: HashMap<Tag, StringField>,
    group: HashMap<Tag, Group>,
    field_order: Vec<Tag>,
    calc_vec_str: Vec<String>,
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

    pub fn set_field_order(&mut self, f_order: &[Tag]) {
        self.field_order = f_order.to_vec();
    }
}

#[derive(Debug, Default)]
pub struct Group {
    delim: u32,
    tag: Tag,
    value: u32,
    fields: Vec<FieldMap>,
    // groups: GroupMap,
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

#[derive(Debug, Default)]
pub struct Message {
    // fields: FieldMap,
    // groups: GroupMap,
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

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    pub fn trailer(&self) -> &FieldMap {
        &self.trailer
    }

    pub fn trailer_mut(&mut self) -> &mut FieldMap {
        &mut self.trailer
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

    pub fn set_checksum(&mut self) {
        todo!()
    }

    fn get_msg_type(&self) -> Result<String, String> {
        self.header.get_field::<String>(35)
    }

    pub fn from_str(s: &str, dd: &DataDictionary) -> SessResult<Self> {
        let mut vdeq: VecDeque<StringField> = VecDeque::with_capacity(16);
        for field in s.split_terminator('|') {
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
        .get_msg_group(HEADER_ID, fld.tag())
        .ok_or_else(SessionRejectError::tag_not_defined_for_msg)?;
    let rg_dd = rg.get_data_dictionary();
    let field_order = rg_dd.get_ordered_fields();
    let group_count_tag = fld.tag();
    let declared_count = match fld.value().parse::<u32>() {
        Ok(c) => c,
        Err(e) => return Err(SessionRejectError::incorrect_data_format_err()),
    };
    let delimiter = rg.get_delimiter();
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
        || v[3].tag() != MsgType::field()
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

pub struct MessageBuilder {}
