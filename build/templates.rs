pub const FIELD_STRUCT: &str = r#"
use std::fmt::Display;

{{#each fields}}
#[derive(Debug)]
pub struct {{this.name}} {
    tag: u32,
    value: String,
} 

impl {{this.name}} {
    pub fn new<T: Into<{{fld_type}}> + Display>(val: T) -> Self {
        Self {
            tag: {{number}},
            value: val.to_string()
        }
    }

    pub fn field() -> u32 {
        {{number}}
    }
}

{{/each}}
"#;

// {{#if values}}
// #[allow(non_camel_case_types)]
// #[derive(Debug)]
// pub enum {{name}} {
//     {{#each values}}
//     {{this.enum_variant}},
//     {{/each}}
// }

// impl {{name}} {
//     fn get(&self) -> String {
//         match self {
//             {{#each values}}
//             {{this.enum_variant}} => String::from("{{this.variant_val}}"),
//             {{/each}}
//         }
//     }

// }
// {{/if}}
// {{/each}}
// "#;

// const FIELD_ENUM: &'static str = r#"
// #[derive(Debug)]
// enum {{name}} {
//     {{#each values}}
//     {{this.enum_variant}},
//     {{/each}}
// }
// "#;

const MSG_STRUCT: &str = r#"
#[derive(Debug, Default, Clone)]
pub struct {{msg_name}} {
    header: Header,
    trailer: Trailer,
    body: FieldMap
}

impl {{msg_name}} {
    pub fn new() -> Self {
        let mut msg = Self::default();
        msg.header
    }
}
"#;
