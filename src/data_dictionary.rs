use std::cmp::{Eq, PartialEq};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::convert::{Infallible, Into};
use std::io::{Read, Write};
use std::iter::{FromIterator, IntoIterator, Iterator};
use std::{fmt, fs, path::Path, str::FromStr};

use crate::message::{self, Group, StringField};
use crate::{quickfix_errors::*, FILE_PATH};
use indexmap::IndexSet;
use roxmltree::{Document, Node, NodeType};

type NodeMap<'a, 'i> = HashMap<String, Node<'a, 'i>>;
type DResult<T> = Result<T, XmlError>;

pub(crate) const HEADER_ID: &str = "Header";
pub(crate) const TRAILER_ID: &str = "Trailer";

#[derive(Debug, Copy, Clone)]
pub enum FixType {
    Char,
    Boolean,
    Data,
    Float,
    Amt,
    Percentage,
    Price,
    PriceOffset,
    Qty,
    Int,
    Length,
    NumInGroup,
    Seqnum,
    Tagnum,
    Str,
    Country,
    Currency,
    Exchange,
    LocalMktDate,
    MonthYear,
    MultipleValueString,
    UtcDate,
    UtcTimeOnly,
    UtcTimestamp,
    Unknown,
}

impl FromStr for FixType {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Infallible> {
        let value = match s {
            "CHAR" => FixType::Char,
            "BOOLEAN" => FixType::Boolean,
            "DATA" => FixType::Data,
            "FLOAT" => FixType::Float,
            "AMT" => FixType::Amt,
            "PERCENTAGE" => FixType::Percentage,
            "PRICE" => FixType::Price,
            "PRICEOFFSET" => FixType::PriceOffset,
            "QTY" => FixType::Qty,
            "INT" => FixType::Int,
            "LENGTH" => FixType::Length,
            "NUMINGROUP" => FixType::NumInGroup,
            "SEQNUM" => FixType::Seqnum,
            "TAGNUM" => FixType::Tagnum,
            "STRING" => FixType::Str,
            "COUNTRY" => FixType::Country,
            "CURRENCY" => FixType::Currency,
            "EXCHANGE" => FixType::Exchange,
            "LOCALMKTDATE" => FixType::LocalMktDate,
            "MONTHYEAR" => FixType::MonthYear,
            "MULTIPLEVALUESTRING" => FixType::MultipleValueString,
            "UTCDATE" => FixType::UtcDate,
            "UTCTIMEONLY" => FixType::UtcTimeOnly,
            "UTCTIMESTAMP" => FixType::UtcTimestamp,
            _ => FixType::Unknown,
        };
        Ok(value)
    }
}

impl std::fmt::Display for FixType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ftype = match self {
            FixType::Char => "CHAR",
            FixType::Boolean => "BOOLEAN",
            FixType::Data => "DATA",
            FixType::Float => "FLOAT",
            FixType::Amt => "AMT",
            FixType::Percentage => "PERCENTAGE",
            FixType::Price => "PRICE",
            FixType::PriceOffset => "PRICEOFFSET",
            FixType::Qty => "QTY",
            FixType::Int => "INT",
            FixType::Length => "LENGTH",
            FixType::NumInGroup => "NUMINGROUP",
            FixType::Seqnum => "SEQNUM",
            FixType::Tagnum => "TAGNUM",
            FixType::Str => "STRING",
            FixType::Country => "COUNTRY",
            FixType::Currency => "CURRENCY",
            FixType::Exchange => "EXCHANGE",
            FixType::LocalMktDate => "LOCALMKTDATE",
            FixType::MonthYear => "MONTHYEAR",
            FixType::MultipleValueString => "MULTIPLEVALUESTRING",
            FixType::UtcDate => "UTCDATE",
            FixType::UtcTimeOnly => "UTCTIMEONLY",
            FixType::UtcTimestamp => "UTCTIMESTAMP",
            FixType::Unknown => "UNKNOWN",
        };
        write!(f, "{}", ftype)
    }
}

#[derive(Debug, Default)]
pub struct DataDictionary {
    begin_string: String,
    fields_by_tag: HashMap<u32, String>,
    fields_by_name: HashMap<String, u32>,
    field_values: HashMap<u32, HashSet<String>>,
    field_type: HashMap<u32, FixType>,
    // mapping of msg_type -> group field. i.e "D" -> <78, 386>
    // {"D" -> {78 -> NoAllocsGroupInfo, 386 -> NoTradingSessionGroupInfo}}
    groups: HashMap<String, HashMap<u32, GroupInfo>>, // can have "header" -> {..}
    fields_order: IndexSet<u32>, // fields of message. mostly useful in group's dd for field order
    types: HashMap<String, String>, // "NewOrderSingle" -> "D"
    category: HashMap<String, String>, // "D" -> "app"
    msg_fields: HashMap<String, HashSet<u32>>, // "D" -> <44, 54, ...>, "header" -> <..>
    msg_required_fields: HashMap<String, HashSet<u32>>,
}

impl DataDictionary {
    pub fn from_xml<P: AsRef<Path>>(xml_file: P) -> Self {
        let file_data = fs::read_to_string(xml_file).expect("xml file open/read error");
        DataDictionary::from_str(&file_data).unwrap()
    }

    pub fn get_field_type(&self, tag: u32) -> Option<&FixType> {
        self.field_type.get(&tag)
    }

    pub fn get_field_values(&self, tag: u32) -> Option<&HashSet<String>> {
        self.field_values.get(&tag)
    }

    pub fn get_msg_group(&self, msg_type: &str, group_tag: u32) -> Option<&GroupInfo> {
        self.groups.get(msg_type).and_then(|hmap| hmap.get(&group_tag))
    }

    pub fn get_msg_required_field(&self, msg_type: &str) -> Option<&HashSet<u32>> {
        self.msg_required_fields.get(msg_type)
    }

    pub fn get_msg_fields(&self, msg_type: &str) -> Option<&HashSet<u32>> {
        self.msg_fields.get(msg_type)
    }

    pub fn is_group(&self, msg_type: &str, tag: u32) -> bool {
        self.groups.get(msg_type).and_then(|val_map| val_map.get(&tag)).is_some()
    }

    pub fn get_group(&self, msg_type: &str, tag: u32) -> Option<&GroupInfo> {
        self.groups.get(msg_type).and_then(|gi| gi.get(&tag))
    }

    pub fn get_ordered_fields(&self) -> Vec<u32> {
        self.fields_order.iter().copied().collect::<Vec<u32>>()
    }

    pub fn is_msg_field(&self, msg_type: &str, tag: u32) -> bool {
        self.msg_fields.get(msg_type).and_then(|val| val.get(&tag)).is_some()
    }

    pub fn is_msg_req_field(&self, msg_type: &str, tag: u32) -> bool {
        self.msg_required_fields.get(msg_type).and_then(|val| val.get(&tag)).is_some()
    }

    pub fn is_trailer_field(&self, tag: u32) -> bool {
        self.is_msg_field(TRAILER_ID, tag)
    }

    pub fn is_header_field(&self, tag: u32) -> bool {
        self.is_msg_field(HEADER_ID, tag)
    }
    /*********************** ALL PRIVATE METHODS BELOW *************************************/
    fn set_field_name_number_type(&mut self, name: &str, number: u32, ty: &str) -> DResult<()> {
        if self.fields_by_name.contains_key(name) || self.fields_by_tag.contains_key(&number) {
            // return error
            return Err(XmlError::DuplicateField(format!("{}={}", name, number)));
        }
        self.fields_by_name.insert(name.to_string(), number);
        self.fields_by_tag.insert(number, name.to_string());
        self.field_type.entry(number).or_insert_with(|| FixType::from_str(ty).unwrap());
        Ok(())
    }

    fn set_field_values(&mut self, fnumber: u32, values: HashSet<String>) {
        self.field_values.entry(fnumber).or_insert(values);
    }

    fn add_fields(&mut self, field: u32) {
        // this adds field to fields indexSet which in tern helps provides field order
        // field order only important for groups, not messages
        self.fields_order.insert(field);
    }

    fn set_msg_name_type_cat(&mut self, msg_name: &str, msg_type: &str, cat: &str) -> DResult<()> {
        if self.category.contains_key(msg_type) || self.types.contains_key(msg_name) {
            return Err(XmlError::DuplicateMessage(msg_name.to_string()));
        }
        self.types.insert(msg_name.to_string(), msg_type.to_string());
        self.category.insert(msg_type.to_string(), cat.to_string());
        Ok(())
    }

    fn set_field_for(&mut self, msg_type: &str, fnum: u32, required: bool) -> DResult<()> {
        let msg_fields = self.msg_fields.entry(msg_type.to_string()).or_insert_with(HashSet::new);
        if msg_fields.contains(&fnum) {
            return Err(XmlError::DuplicateField(format!(
                "field {} in message {}",
                fnum, msg_type
            )));
        }
        msg_fields.insert(fnum);
        if required {
            self.msg_required_fields
                .entry(msg_type.to_owned())
                .or_insert_with(HashSet::new)
                .insert(fnum);
        }
        Ok(())
    }

    fn set_group_info(&mut self, msg_type: &str, grp_num: u32, info: GroupInfo) {
        // msg_type is value of 35 tag i.e. "D" or "AE" etc
        // for headers, its literal `header`
        self.groups.entry(msg_type.to_string()).and_modify(|hm| {
            hm.entry(grp_num).or_insert(info);
        });
    }

    fn get_field_num(&self, fname: &str) -> Option<u32> {
        self.fields_by_name.get(fname).map(|n| *n)
    }

    fn add_fields_and_values(&mut self, fields: Node) -> DResult<()> {
        for field_node in
            fields.children().filter(|node| node.is_element() && node.has_tag_name("field"))
        {
            let name = get_name_attr(&field_node)?;
            let number = get_number_attr(&field_node)?;
            let typ = get_attribute("type", &field_node)?;
            self.set_field_name_number_type(name, number, typ)?;
            let values = get_field_values(&field_node)?;
            if !values.is_empty() {
                self.set_field_values(number, values);
            }
        }
        Ok(())
    }

    #[inline]
    fn add_fields_to(
        &mut self, msg_type: &str, field_name: &str, is_required: bool, doc: &Document,
    ) -> DResult<u32> {
        let field_number = lookup_field_num_with_name(field_name, doc)?;
        self.set_field_for(msg_type, field_number, is_required)?;
        self.add_fields(field_number);
        Ok(field_number)
    }

    fn add_xml_group(
        &mut self, msg_type: &str, group_node: &Node, components: &NodeMap, doc: &Document,
    ) -> DResult<()> {
        // process the group node and add fields, components, subgroup
        // for the message name and message type
        let mut group_dd = DataDictionary::default();
        let group_required = get_required_attr(group_node)?;
        let mut delimiter = 0u32;
        for grp_child in group_node.children().filter(|&n| n.is_element()) {
            let child_name = grp_child.tag_name().name();
            let first_field: u32 = match child_name {
                "field" => {
                    let fname = get_name_attr(&grp_child)?;
                    let required = get_required_attr(&grp_child)?;
                    // add this field to group_dd for the msg_name
                    // this field is required if group is required and field is required
                    let required = required && group_required;
                    group_dd.add_fields_to(msg_type, fname, required, doc)?
                }
                "component" => {
                    // this component fields are also added in group_dd for msg_name
                    let comp_name = get_name_attr(&grp_child)?;
                    let comp_required = group_required && get_required_attr(&grp_child)?;
                    group_dd.add_xml_component(
                        msg_type,
                        &grp_child,
                        comp_required,
                        components,
                        doc,
                    )?
                }
                "group" => {
                    // this is subgroup inside group
                    let sub_group_name = get_name_attr(&grp_child)?;
                    let sub_group_required = get_required_attr(&grp_child)? && group_required;
                    // this subgroup fields should be added to group's dd but under msg_type
                    let field = group_dd.add_fields_to(
                        msg_type,
                        sub_group_name,
                        sub_group_required,
                        doc,
                    )?;
                    // process group node separately to create GroupInfo
                    // and add it to group dd. Mapping should be with msg_type
                    group_dd.add_xml_group(msg_type, &grp_child, components, doc)?;
                    field
                }
                _ => return Err(XmlError::UnknownXmlTag(child_name.to_string())),
            };
            if delimiter == 0 {
                delimiter = first_field;
            }
        }
        let group_info = GroupInfo {
            delimiter,
            group_dd,
        };
        let group_name = get_name_attr(group_node)?;
        let group_tag = lookup_field_num_with_name(group_name, doc)?;
        self.set_group_info(msg_type, group_tag, group_info);
        Ok(())
    }

    fn add_xml_component(
        &mut self, msg_type: &str, comp_node: &Node, is_required: bool, components: &NodeMap,
        doc: &Document,
    ) -> DResult<u32> {
        // first_field is the first field encountered in processing the node
        // it only useful for groups where this serves as the delimiter.
        let mut first_field = 0u32;
        for child in comp_node.children().filter(|n| n.is_element()) {
            let child_name = child.tag_name().name();
            let num = match child_name {
                "field" => {
                    let fname = get_name_attr(&child)?;
                    // if component is required and component's field is required
                    // then field is required for message
                    let required = get_required_attr(&child)? && is_required;
                    self.add_fields_to(msg_type, fname, required, doc)?
                }
                "component" => {
                    // most likely components do not contain components but
                    // adding this for completeness.
                    let comp_required = get_required_attr(&child)?;
                    let comp_name = get_name_attr(&child)?;
                    let comp_node = components
                        .get(comp_name)
                        .expect(format!("msgtype {}, component {}", msg_type, comp_name).as_str());
                    // component inside component is only required if outer comp and this are req
                    // otherwise not required
                    let is_comp_required = comp_required && is_required;
                    self.add_xml_component(msg_type, comp_node, is_comp_required, components, doc)?
                }
                "group" => {
                    // this group field is added to message fields
                    let group_name = get_name_attr(&child)?;
                    // if component is required && group inside component is required then
                    // group is required for message
                    let group_required = get_required_attr(&child)? && is_required;
                    let field = self.add_fields_to(msg_type, group_name, group_required, doc)?;
                    // process group node separately to create GroupInfo
                    // and add it to dd for the message
                    self.add_xml_group(msg_type, &child, components, doc)?;
                    field
                }
                _ => return Err(XmlError::UnknownXmlTag(child_name.to_string())),
            };
            if first_field == 0 {
                first_field = num;
            }
        }
        Ok(first_field)
    }

    fn add_all_xml_messages(
        &mut self, msgs_node: &Node, components: &NodeMap, doc: &Document,
    ) -> DResult<()> {
        for m_node in msgs_node
            .children()
            .filter(|n| n.is_element() && n.tag_name().name().eq_ignore_ascii_case("message"))
        {
            let message_name = get_name_attr(&m_node)?;
            let message_category = get_attribute("msgcat", &m_node)?;
            let message_type = get_attribute("msgtype", &m_node)?;
            self.set_msg_name_type_cat(message_name, message_type, message_category)?;
            self.add_xml_message(message_type, &m_node, &components, doc)?;
        }
        Ok(())
    }

    fn add_xml_message(
        &mut self, msg_type: &str, node: &Node, components: &NodeMap, doc: &Document,
    ) -> DResult<()> {
        // adding empty hashset for msg type so that any msg which does not have fields have
        // entres. for e.g. 35=n does not have any fields. All data is contained in header
        self.msg_fields.insert(msg_type.to_string(), HashSet::new());
        self.msg_required_fields.insert(msg_type.to_string(), HashSet::new());
        for child in node.children().filter(|n| n.is_element()) {
            let child_tag_name = child.tag_name().name();
            match child_tag_name {
                "field" => {
                    let fname = get_name_attr(&child)?;
                    let required = get_required_attr(&child)?;
                    self.add_fields_to(msg_type, fname, required, doc)?;
                }
                "component" => {
                    let comp_required = get_required_attr(&child)?;
                    let comp_name = get_name_attr(&child)?;
                    let comp_node = components
                        .get(comp_name)
                        .expect(format!("msgtype {}, component {}", msg_type, comp_name).as_str());
                    self.add_xml_component(msg_type, comp_node, comp_required, components, doc)?;
                }
                "group" => {
                    // this group field is added to message fields
                    let group_name = get_name_attr(&child)?;
                    let group_required = get_required_attr(&child)?;
                    self.add_fields_to(msg_type, group_name, group_required, doc)?;
                    // process group node separately to create GroupInfo
                    // and add it to dd for the message type
                    self.add_xml_group(msg_type, &child, components, doc)?;
                }
                _ => return Err(XmlError::UnknownXmlTag(child_tag_name.to_string())),
            };
        }
        Ok(())
    }
}

impl FromStr for DataDictionary {
    type Err = XmlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dd = DataDictionary::default();
        let doc = Document::parse(s)?;
        let begin_string = get_begin_str_from_doc(doc.root_element())?;
        dd.begin_string = begin_string;

        let fields = lookup_node("fields", &doc)?;
        dd.add_fields_and_values(fields)?;

        let component_node = lookup_node("components", &doc)?;
        let component_map: NodeMap = get_component_nodes_by_name(component_node)?;

        let header_node = lookup_node(HEADER_ID, &doc)?;
        dd.add_xml_message(&HEADER_ID.to_ascii_lowercase(), &header_node, &component_map, &doc)?;

        let trailer_node = lookup_node(TRAILER_ID, &doc)?;
        dd.add_xml_message(&TRAILER_ID.to_ascii_lowercase(), &trailer_node, &component_map, &doc)?;

        let messages = lookup_node("messages", &doc)?;
        dd.add_all_xml_messages(&messages, &component_map, &doc)?;

        Ok(dd)
    }
}

#[derive(Debug, Default)]
pub struct GroupInfo {
    delimiter: u32,
    group_dd: DataDictionary,
}

impl GroupInfo {
    pub fn get_data_dictionary(&self) -> &DataDictionary {
        &self.group_dd
    }

    pub fn get_delimiter(&self) -> u32 {
        self.delimiter
    }
}

/********************* ALL XML PARSING RELATED CODE ********************************************/
fn get_attribute<'a>(attr: &str, node: &Node<'a, '_>) -> DResult<&'a str> {
    let requested = match node.attribute(attr) {
        Some(atr) => {
            if atr.is_empty() {
                return Err(XmlError::AttributeNotFound(format!(
                    "empty {} in {}",
                    attr,
                    node.tag_name().name()
                )));
            } else {
                atr
            }
        }
        None => {
            return Err(XmlError::AttributeNotFound(format!(
                "{} in {}",
                attr,
                node.tag_name().name()
            )))
        }
    };
    Ok(requested)
}

fn get_name_attr<'a>(node: &Node<'a, '_>) -> DResult<&'a str> {
    get_attribute("name", node)
}

fn get_required_attr(node: &Node) -> DResult<bool> {
    let att = get_attribute("required", node)?;
    Ok(att.eq_ignore_ascii_case("Y"))
}

fn get_number_attr(node: &Node) -> DResult<u32> {
    let number = get_attribute("number", node)?;
    match number.parse::<u32>() {
        Ok(n) => return Ok(n),
        Err(e) => {
            return Err(XmlError::FieldNotParsed {
                source: e,
                field: number.to_string(),
            })
        }
    };
}

fn get_begin_str_from_doc(root_node: Node) -> DResult<String> {
    let dict_type = get_attribute("type", &root_node)?;
    let major_version = get_attribute("major", &root_node)?;
    let minor_verion = get_attribute("minor", &root_node)?;
    Ok(format!("{}.{}.{}", dict_type, major_version, minor_verion))
}

fn lookup_node<'a, 'input>(
    name: &str, document: &'a Document<'input>,
) -> DResult<Node<'a, 'input>> {
    // find the node in the document with given name
    // NOTE: this searches in children, not in descendents
    document
        .root_element()
        .children()
        .find(|node| node.tag_name().name().eq_ignore_ascii_case(name))
        .ok_or_else(|| XmlError::XmlNodeNotFound(name.to_string()))
}

fn get_component_nodes_by_name<'a, 'i>(components: Node<'a, 'i>) -> DResult<NodeMap<'a, 'i>> {
    let mut cmap: HashMap<String, Node> = HashMap::new();
    for node in components.children().filter(|cnode| cnode.is_element()) {
        let cname = get_name_attr(&node)?;
        cmap.insert(cname.to_string(), node);
    }
    Ok(cmap)
}

fn lookup_field_num_with_name(field_name: &str, doc: &Document) -> DResult<u32> {
    let fields = lookup_node("fields", doc)?;
    for node in fields.children().filter(|n| n.has_attribute("number") && n.has_attribute("name")) {
        let name = get_name_attr(&node)?;
        if name == field_name {
            let number = get_number_attr(&node)?;
            return Ok(number);
        }
    }
    Err(XmlError::XmlNodeNotFound(field_name.to_string()))
}

fn get_all_field_nums(doc: &Document) -> HashSet<u32> {
    // solely used in testing
    let field_node = lookup_node("fields", doc).unwrap();
    HashSet::from_iter(
        field_node
            .children()
            .filter(|node| node.is_element() && node.has_tag_name("field"))
            .map(|node| get_attribute("number", &node).unwrap().parse::<u32>().unwrap()),
    )
}

fn get_field_values(node: &Node) -> DResult<HashSet<String>> {
    let mut field_values = HashSet::new();
    for val_node in node.children().filter(|n| n.is_element() && n.has_tag_name("value")) {
        let value = get_attribute("enum", &val_node)?;
        if field_values.contains(value) {
            // duplicate value for this field
            return Err(XmlError::DuplicateField(format!(
                "value {} for field {}",
                value,
                get_name_attr(&node)?
            )));
        }
        field_values.insert(value.to_string());
    }
    Ok(field_values)
}

fn get_field_num_to_name(doc: &Document) -> HashMap<u32, String> {
    let fields = lookup_node("fields", doc).unwrap();
    let num_to_name: HashMap<u32, String> = fields
        .children()
        .filter(|node| node.is_element() && node.has_tag_name("field"))
        .map(|node| {
            (
                get_attribute("number", &node).unwrap().parse::<u32>().unwrap(),
                get_name_attr(&node).unwrap().to_string(),
            )
        })
        .collect();
    num_to_name
}

fn get_field_num_to_type(doc: &Document) -> HashMap<u32, String> {
    let fields = lookup_node("fields", doc).unwrap();
    let num_to_type: HashMap<u32, String> = fields
        .children()
        .filter(|node| node.is_element() && node.has_tag_name("field"))
        .map(|node| {
            (
                get_attribute("number", &node).unwrap().parse::<u32>().unwrap(),
                get_attribute("type", &node).unwrap().to_string(),
            )
        })
        .collect();
    num_to_type
}

#[cfg(test)]
mod tests {
    use crate::fields::{OnBehalfOfLocationID, ResetSeqNumFlag};

    use super::*;
    #[cfg(test)]
    use assert_matches::*;
    #[cfg(test)]
    use lazy_static::lazy_static;
    use roxmltree::Document;
    use std::fs;

    const XML_PATH: &str = "resources/FIX43.xml";

    const FIX_START: &str = r#"<fix type="FIX" major="4" minor="3" servicepack="0">"#;
    const HEADER_STR: &str = r#"
        <header>
            <field name="BeginString" required="Y"/>
            <field name="BodyLength" required="Y"/>
            <field name="MsgType" required="Y"/>
            <field name="SenderCompID" required="Y"/> 
            <group name="NoHops" required="N">
                <field name="HopCompID" required="N"/>
                <field name="HopSendingTime" required="N"/>
                <field name="HopRefID" required="N"/>
            </group>
        </header>
    "#;

    lazy_static! {
        static ref XML: String = fs::read_to_string(XML_PATH).expect("test file read error");
        static ref DOC: Document<'static> =
            Document::parse(&XML).expect("test document parse error");
        static ref COMPONENTS: NodeMap<'static, 'static> =
            get_component_nodes_by_name(lookup_node("components", &DOC).expect("test components"))
                .expect("test component map");
    }

    fn get_all_field_values() -> HashMap<u32, HashSet<String>> {
        let mut field_value_map: HashMap<u32, HashSet<String>> = HashMap::new();
        let fields = lookup_node("fields", &DOC).unwrap();
        for fnode in
            fields.children().filter(|node| node.is_element() && node.has_tag_name("field"))
        {
            let number = get_attribute("number", &fnode).unwrap();
            let number = number.parse::<u32>().unwrap();
            let values = get_field_values(&fnode).unwrap();
            field_value_map.insert(number, values);
        }
        field_value_map
    }

    fn add_fields_and_messages(fields: &str, msgs: &str, dd: &mut DataDictionary) -> DResult<()> {
        // adds given fields and messages and forms the mini fix xml
        // uses this xml to create Document and parse the Document to create a datadictionary
        let buff = format!("{}{}{}{}", FIX_START, msgs, fields, "</fix>");
        let doc: Document = Document::parse(&buff)?;
        let field_node = lookup_node("fields", &doc)?;
        dd.add_fields_and_values(field_node)?;
        let mesg_node = lookup_node("messages", &doc)?;
        dd.add_all_xml_messages(&mesg_node, &NodeMap::new(), &doc)
    }

    fn get_messages_and_types(doc: &Document) -> HashMap<String, String> {
        // returns map of msg_name -> msg_type from Document
        let msgs_node = lookup_node("messages", doc).unwrap();
        let msgs: HashMap<String, String> = msgs_node
            .children()
            .filter(|node| node.is_element() && node.has_tag_name("message"))
            .map(|node| {
                (
                    get_name_attr(&node).unwrap().to_string(),
                    get_attribute("msgtype", &node).unwrap().to_string(),
                )
            })
            .collect();
        msgs
    }

    fn get_only_fields_for_msg_type(msg_type: &str, doc: &Document) -> HashMap<String, bool> {
        // for a given msg_type, returns String
        let msgs_node = lookup_node("messages", doc).unwrap();
        let msg_node = msgs_node
            .children()
            .find(|node| {
                node.is_element()
                    && node.has_tag_name("message")
                    && get_attribute("msgtype", node).unwrap().eq(msg_type)
            })
            .unwrap();
        let msg_fields = msg_node
            .children()
            .filter(|node| node.is_element() && node.has_tag_name("field"))
            .map(|node| {
                (get_name_attr(&node).unwrap().to_string(), get_required_attr(&node).unwrap())
            })
            .collect();
        msg_fields
    }

    #[test]
    fn test_major_minor_type() {
        let fstr_type_missing = r#"<fix major="4" minor="3" servicepack="0">/"#;
        let fstr_type_empty = r#"<fix type="" major="4" minor="3" servicepack="0">/"#;
        let doc = Document::parse(fstr_type_missing).unwrap();
        let result = get_begin_str_from_doc(doc.root_element());
        assert!(result.is_err());
        assert_matches!(result.unwrap_err(), XmlError::AttributeNotFound(_));

        let doc = Document::parse(fstr_type_empty).unwrap();
        let result = get_begin_str_from_doc(doc.root_element());
        assert!(result.is_err());
        assert_matches!(result.unwrap_err(), XmlError::AttributeNotFound(_));

        let fstr_major_missing = r#"<fix type="FIX" minor="3" servicepack="0">/"#;
        let fstr_major_empty = r#"<fix type="FIX" major="" minor="3" servicepack="0">/"#;
        let doc = Document::parse(fstr_major_missing).unwrap();
        let result = get_begin_str_from_doc(doc.root_element());
        assert!(result.is_err());
        assert_matches!(result.unwrap_err(), XmlError::AttributeNotFound(_));

        let doc = Document::parse(fstr_major_empty).unwrap();
        let result = get_begin_str_from_doc(doc.root_element());
        assert!(result.is_err());
        assert_matches!(result.unwrap_err(), XmlError::AttributeNotFound(_));

        let fstr_minor_missing = r#"<fix type="FIX" major="4" servicepack="0">/"#;
        let fstr_minor_empty = r#"<fix type="FIX" major="4" minor="" servicepack="0">/"#;
        let doc = Document::parse(fstr_minor_missing).unwrap();
        let result = get_begin_str_from_doc(doc.root_element());
        assert!(result.is_err());
        assert_matches!(result.unwrap_err(), XmlError::AttributeNotFound(_));

        let doc = Document::parse(fstr_minor_empty).unwrap();
        let result = get_begin_str_from_doc(doc.root_element());
        assert!(result.is_err());
        assert_matches!(result.unwrap_err(), XmlError::AttributeNotFound(_));
    }

    #[test]
    fn test_number_of_fields() {
        // this tests from actual xml file
        // test correct number of fields
        let mut dict = DataDictionary::default();
        let fields = lookup_node("fields", &DOC).unwrap();
        dict.add_fields_and_values(fields).unwrap();
        let expected_fields = get_all_field_nums(&DOC);
        assert_eq!(expected_fields.len(), dict.fields_by_tag.len(), "fields_by_tag is not same");
        assert_eq!(expected_fields.len(), dict.fields_by_name.len(), "fields_by_name is not same");
        assert_eq!(expected_fields.len(), dict.field_type.len(), "field_type len is not same");
    }

    #[test]
    fn test_field_num_to_name() {
        // this tests from actual xml file
        let expected_num_to_name = get_field_num_to_name(&DOC);
        let mut dict = DataDictionary::default();
        let fields = lookup_node("fields", &DOC).unwrap();
        dict.add_fields_and_values(fields).unwrap();
        // verify size
        assert_eq!(expected_num_to_name.len(), dict.fields_by_tag.len());
        // verify entries
        for (expect_key, expect_value) in expected_num_to_name.iter() {
            let actual_val = dict.fields_by_tag.get(expect_key);
            assert!(actual_val.is_some(), "key does not exist");
            assert_eq!(expect_value.as_str(), actual_val.unwrap().as_str());

            // verify in string -> num mapping
            let actual_tag = dict.get_field_num(expect_value);
            assert!(actual_tag.is_some());
            assert_eq!(*expect_key, actual_tag.unwrap());
        }
    }

    #[test]
    fn test_field_types() {
        // testing against actual xml file
        let expected_num_type = get_field_num_to_type(&DOC);
        let mut dict = DataDictionary::default();
        let fields = lookup_node("fields", &DOC).unwrap();
        dict.add_fields_and_values(fields).unwrap();
        assert_eq!(expected_num_type.len(), dict.field_type.len());
        for (expected_key, expected_val) in expected_num_type {
            let actual_type = dict.get_field_type(expected_key);
            assert!(actual_type.is_some(), "type does not exist");
            assert_eq!(expected_val, actual_type.unwrap().to_string());
        }
    }

    #[test]
    fn test_field_values() {
        // testing against actual xml file
        let expected_vals = get_all_field_values();
        let mut dict = DataDictionary::default();
        let fields = lookup_node("fields", &DOC).unwrap();
        dict.add_fields_and_values(fields).unwrap();
        for (key, val) in expected_vals {
            if !val.is_empty() {
                let dict_val = dict.get_field_values(key);
                assert!(dict_val.is_some());
                assert_eq!(val, dict_val.unwrap().to_owned());
            }
        }
    }

    #[test]
    fn test_duplicate_field() {
        let duplicate_tag: &str = r#"
            <fields>
                <field number="639" name="PriceImprovement" type="PRICEOFFSET"/>
                <field number="640" name="Price2" type="PRICE"/>
                <field number="639" name="BidForwardPoints2" type="PRICEOFFSET"/>
            </fields> 
        "#;
        let mini_xml = format!("{}{}{}", FIX_START, duplicate_tag, "</fix>");
        let document = Document::parse(&mini_xml).unwrap();
        let mut dict = DataDictionary::default();
        let fields = lookup_node("fields", &document).unwrap();
        let result = dict.add_fields_and_values(fields);
        assert!(result.is_err());
        assert_matches!(result, Err(XmlError::DuplicateField(_)));

        let duplicate_name: &str = r#"
            <fields>
                <field number="639" name="PriceImprovement" type="PRICEOFFSET"/>
                <field number="640" name="Price2" type="PRICE"/>
                <field number="641" name="Price2" type="PRICEOFFSET"/>
            </fields> 
        "#;
        let mini_xml = format!("{}{}{}", FIX_START, duplicate_name, "</fix>");
        let document = Document::parse(&mini_xml).unwrap();
        let mut dict = DataDictionary::default();
        let fields = lookup_node("fields", &document).unwrap();
        let result = dict.add_fields_and_values(fields);
        assert!(result.is_err());
        assert_matches!(result, Err(XmlError::DuplicateField(_)));
    }

    #[test]
    fn test_duplicate_field_values() {
        let duplicate_values: &str = r#"
            <fields>
                <field number="658" name="QuoteRequestRejectReason" type="INT">
                    <value enum="1" description="UNKNOWN_SYMBOL"/>
                    <value enum="2" description="EXCHANGE"/>
                    <value enum="1" description="QUOTE_REQUEST_EXCEEDS_LIMIT"/>
                </field>
                <field number="642" name="BidForwardPoints2" type="PRICEOFFSET"/>
            </fields> 
        "#;
        let mini_xml = format!("{}{}{}", FIX_START, duplicate_values, "</fix>");
        let document = Document::parse(&mini_xml).unwrap();
        let mut dict = DataDictionary::default();
        let fields = lookup_node("fields", &document).unwrap();
        let result = dict.add_fields_and_values(fields);
        assert!(result.is_err());
        assert_matches!(result, Err(XmlError::DuplicateField(_)));
    }

    #[test]
    fn test_missing_field_number() {
        let missing_field_num = r#"
            <fields>
                <field number="658" name="QuoteRequestRejectReason" type="INT"/>
                <field name="BidForwardPoints2" type="PRICEOFFSET"/>
            </fields> 
        "#;
        let mini_xml = format!("{}{}{}", FIX_START, missing_field_num, "</fix>");
        let document = Document::parse(&mini_xml).unwrap();
        let mut dict = DataDictionary::default();
        let fields = lookup_node("fields", &document).unwrap();
        let result = dict.add_fields_and_values(fields);
        assert!(result.is_err());
        assert_matches!(result, Err(XmlError::AttributeNotFound(_)));

        let unparsable_field_num = r#"
            <fields>
                <field number="658non" name="QuoteRequestRejectReason" type="INT"/>
                <field number="660" name="BidForwardPoints2" type="PRICEOFFSET"/>
            </fields> 
        "#;
        let mini_xml = format!("{}{}{}", FIX_START, unparsable_field_num, "</fix>");
        let document = Document::parse(&mini_xml).unwrap();
        let mut dict = DataDictionary::default();
        let fields = lookup_node("fields", &document).unwrap();
        let result = dict.add_fields_and_values(fields);
        assert!(result.is_err());
        assert_matches!(result, Err(XmlError::FieldNotParsed { .. }));
    }

    fn test_missing_field_name() {}
    fn test_missing_field_type() {}

    #[test]
    fn test_msg_with_non_existent_field() {
        // message definition can have a field name that does not exists in xml
        let message = r#"
            <messages>
                <message name="ExecutionReport" msgtype="8" msgcat="app">
                    <field name="OrderID" required="Y"/>
                    <field name="SecondaryOrderID" required="N"/>
                </message>
            </messages> 
        "#;

        let fields: &str = r#"
            <fields>
                <field number="639" name="PriceImprovement" type="PRICEOFFSET"/>
                <field number="640" name="Price2" type="PRICE"/>
                <field number="641" name="PriceOffset" type="PRICEOFFSET"/>
            </fields> 
        "#;

        let mut dd = DataDictionary::default();
        let result = add_fields_and_messages(fields, message, &mut dd);
        assert!(result.is_err());
        assert_matches!(result.unwrap_err(), XmlError::XmlNodeNotFound(_));
    }

    #[test]
    fn test_msg_with_missing_name() {
        // message does not have a name attribute
        let msg_no_name = r#"
            <messages>
                <message msgtype="8" msgcat="app">
                    <field name="OrderID" required="Y"/>
                    <field name="SecondaryOrderID" required="N"/>
                </message>
            </messages> 
        "#;

        let msg_empty_name = r#"
            <messages>
                <message name="" msgtype="8" msgcat="app">
                    <field name="OrderID" required="Y"/>
                    <field name="SecondaryOrderID" required="N"/>
                </message>
            </messages> 
        "#;

        let fields: &str = r#"
            <fields>
                <field number="639" name="PriceImprovement" type="PRICEOFFSET"/>
                <field number="640" name="Price2" type="PRICE"/>
                <field number="641" name="PriceOffset" type="PRICEOFFSET"/>
            </fields> 
        "#;

        let mut dd = DataDictionary::default();
        let result = add_fields_and_messages(fields, msg_no_name, &mut dd);
        assert!(result.is_err(), "no error on msg name missing");
        assert_matches!(result.unwrap_err(), XmlError::AttributeNotFound(_));

        dd = DataDictionary::default();
        let result = add_fields_and_messages(fields, msg_empty_name, &mut dd);
        assert!(result.is_err(), "no error on empty string in msgname");
        assert_matches!(result.unwrap_err(), XmlError::AttributeNotFound(_));
    }

    #[test]
    fn test_msg_with_missing_type() {
        // message definition does not have type
        let msg_no_type = r#"
            <messages>
                <message name="ExecutionReport" msgcat="app">
                    <field name="OrderID" required="Y"/>
                    <field name="SecondaryOrderID" required="N"/>
                </message>
            </messages> 
        "#;

        let msg_empty_type = r#"
            <messages>
                <message name="ExecutionReport" msgtype="" msgcat="app">
                    <field name="OrderID" required="Y"/>
                    <field name="SecondaryOrderID" required="N"/>
                </message>
            </messages> 
        "#;

        let fields: &str = r#"
            <fields>
                <field number="639" name="PriceImprovement" type="PRICEOFFSET"/>
                <field number="640" name="Price2" type="PRICE"/>
                <field number="641" name="PriceOffset" type="PRICEOFFSET"/>
            </fields> 
        "#;

        let mut dd = DataDictionary::default();
        let result = add_fields_and_messages(fields, msg_no_type, &mut dd);
        assert!(result.is_err(), "no error on msg type missing");
        assert_matches!(result.unwrap_err(), XmlError::AttributeNotFound(_));

        dd = DataDictionary::default();
        let result = add_fields_and_messages(fields, msg_empty_type, &mut dd);
        assert!(result.is_err(), "no error on empty string in msgtype");
        assert_matches!(result.unwrap_err(), XmlError::AttributeNotFound(_));
    }

    #[test]
    fn test_all_message_type() {
        // tests all the message in fix xml are present in data dictionary
        // uses actual xml file without duplicates or missing data
        let dd = DataDictionary::from_str(&XML).unwrap();
        let expct_msgs = get_messages_and_types(&DOC);
        let expct_len = expct_msgs.len();
        assert_eq!(expct_len, dd.category.len());
        assert_eq!(expct_len, dd.types.len());
        // excluding header, trailer
        // assuming all messages have atleast one required field
        assert_eq!(expct_len, dd.msg_fields.len() - 2); // excluding header, trailer
        assert_eq!(expct_len, dd.msg_required_fields.len() - 2); // excluding header, trailer
        for (msg_name, msg_type) in expct_msgs {
            // all of these messages and types should be present in dd
            let actual_type = dd.types.get(&msg_name);
            assert!(actual_type.is_some());
            assert_eq!(&msg_type, actual_type.unwrap());

            assert!(dd.category.contains_key(&msg_type));
            assert!(dd.msg_fields.contains_key(&msg_type));
            assert!(dd.msg_required_fields.contains_key(&msg_type), "msgtype {}", &msg_type);
        }
    }

    #[test]
    fn test_msgs_with_fields() {
        // test that message's required & non required fields are correctly captured
        // msg having only fields, no groups or component is taken
        let dd = DataDictionary::from_str(&XML).unwrap();
        for (_, msg_type) in get_messages_and_types(&DOC) {
            for (name, required) in get_only_fields_for_msg_type(&msg_type, &DOC) {
                let number = dd.get_field_num(&name);
                assert!(number.is_some());
                let number = number.unwrap();
                assert!(
                    dd.is_msg_field(&msg_type, number),
                    "msg {}, name {}, number {}",
                    &msg_type,
                    &name,
                    number
                );
                if required {
                    assert!(
                        dd.is_msg_req_field(&msg_type, number),
                        "reqd: msg {}, name {}, number {}",
                        &msg_type,
                        &name,
                        number
                    );
                }
            }
        }
    }
}
