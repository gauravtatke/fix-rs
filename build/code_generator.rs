use crate::templates::*;
use handlebars::Handlebars;
use heck::ToUpperCamelCase;
use roxmltree::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, collections::HashSet, fs, fs::File, path::Path};

const ENUM_VARIANT_MAX_LEN: usize = 10; // max words in enum variant separated by `_`
const ENUM_VARIANT_PREFIX: &str = "ENVal_";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct XmlFixSpec {
    pub begin_string: String,
    pub header: Header,
    pub trailer: Trailer,
    pub messages: Vec<XmlMessage>,
    pub fields: Vec<XmlField>,
}

impl XmlFixSpec {}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Header {
    pub fields: HashSet<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Trailer {
    pub fields: HashSet<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct XmlMessage {
    pub msg_name: String,
    pub msg_type: String,
    pub msg_cat: String,
    pub fields: HashSet<String>,
    pub groups: HashMap<String, XmlGroup>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct XmlGroup {
    pub group_name: String,
    pub group_fields: HashSet<String>,
    pub groups: HashMap<String, XmlGroup>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct XmlField {
    pub name: String,
    pub number: u32,
    pub fld_type: String,
    pub values: Vec<XmlFieldValue>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct XmlFieldValue {
    pub enum_variant: String,
    pub variant_val: String,
}

fn lookup_node<'a, 'input>(name: &str, document: &'a Document<'input>) -> Node<'a, 'input> {
    // find the node in the document with given name
    document
        .root_element()
        .children()
        .find(|node| node.tag_name().name().eq_ignore_ascii_case(name))
        .expect("Could not lookup node")
}

fn get_primitive_type(field_type: &str) -> String {
    let primitive = match field_type.to_lowercase().as_str() {
        "char" => "char",
        "boolean" => "bool",
        "data" | "string" | "country" | "currency" | "exchange" => "String",
        "float" | "price" | "amt" | "qty" | "priceoffset" => "f32",
        "localmktdate"
        | "monthyear"
        | "multiplevaluestring"
        | "utcdate"
        | "utctimeonly"
        | "utctimestamp" => "String", // may convert it to chrono types
        "int" => "i32",
        "length" | "numingroup" | "seqnum" | "tagnum" => "u32",
        _ => "String",
    };
    primitive.to_string()
}

fn get_enum_variant(field_type: &str, enum_val: &str, description: &str) -> String {
    let enum_words = description
        .split_terminator(&['_', '-'])
        .map(|s| s.to_upper_camel_case())
        .collect::<Vec<String>>();
    let mut short_description = if enum_words.len() > ENUM_VARIANT_MAX_LEN {
        enum_words[..ENUM_VARIANT_MAX_LEN].join("")
    } else {
        enum_words[..].join("")
    };

    if short_description.chars().next().unwrap().is_numeric() {
        // if the description starts with a number, prefix it with `Val`
        short_description.insert_str(0, "Val")
    }

    let enum_variant = match field_type {
        "bool" | "char" | "u32" | "u64" | "i32" | "i64" | "f32" | "f64" => short_description,
        _ => {
            if enum_val.len() >= 2 {
                // make enum variant compatible with variant naming convention
                enum_val
                    .split_terminator(&['_', '-', ' '])
                    .map(|s| s.to_upper_camel_case())
                    .collect::<String>()
            } else {
                short_description
            }
        }
    };
    enum_variant
}

fn add_fields_to_spec(field_node: &Node, spec: &mut XmlFixSpec) {
    for field in field_node
        .children()
        .filter(|node| node.is_element() && node.has_tag_name("field"))
    {
        let ftype = field.attribute("type").unwrap();
        let fld_type = get_primitive_type(ftype);
        let name = field.attribute("name").unwrap();
        let number = field.attribute("number").and_then(|s| s.parse::<u32>().ok()).unwrap();
        let values: Vec<XmlFieldValue> = field
            .children()
            .filter(|n| n.is_element() && n.has_tag_name("value"))
            .map(|node| {
                let description = node
                    .attribute("description")
                    .unwrap()
                    .split_terminator(&['_', '-'])
                    .map(|s| s.to_upper_camel_case())
                    // .take(ENUM_VARIANT_MAX_LEN)
                    .collect::<String>();
                let enum_value = node.attribute("enum").unwrap();
                // variant's actual val will always be "enum" attribute
                // let enum_variant = get_enum_variant(&fld_type, enum_value, description);
                let mut enum_variant: String = description;
                // prefix the variant name so that it distinguishes between Rust keywords like None
                // `None` is enum values in some field's supported values
                enum_variant.insert_str(0, ENUM_VARIANT_PREFIX);
                let variant_val = enum_value.to_string();
                XmlFieldValue {
                    enum_variant,
                    variant_val,
                }
            })
            .collect();

        let xml_field = XmlField {
            name: name.to_string(),
            number,
            fld_type,
            values,
        };
        spec.fields.push(xml_field);
    }
}

pub fn get_fix_spec(src_dir: &Path, name: &str) -> XmlFixSpec {
    let mut fix_spec = XmlFixSpec::default();
    let buff = fs::read_to_string(src_dir.join(name)).unwrap();
    let document = Document::parse(&buff).expect("xml document could not be parsed");
    let begin_string = format!(
        "{}.{}.{}",
        document.root_element().attribute("type").unwrap(),
        document.root_element().attribute("major").unwrap(),
        document.root_element().attribute("minor").unwrap()
    );
    fix_spec.begin_string = begin_string;
    let component_parent = lookup_node("components", &document);
    let components: HashMap<String, Node> = component_parent
        .children()
        .filter(|node| node.is_element() && node.has_tag_name("component"))
        .map(|node| (node.attribute("name").map(|name| name.to_string()).unwrap(), node))
        .collect();
    let fields_node = lookup_node("fields", &document);
    add_fields_to_spec(&fields_node, &mut fix_spec);
    fix_spec
}

pub fn generate_fields(out_dir: &Path, name: &str, xml_spec: &XmlFixSpec) {
    let mut file = File::create(out_dir.join(name)).expect("file could not be created");
    let mut handlebar = Handlebars::new();
    handlebar.register_template_string("f_struct", FIELD_STRUCT).unwrap();
    handlebar.render_to_write("f_struct", &xml_spec, &mut file).unwrap();
}
