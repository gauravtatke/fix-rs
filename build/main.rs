extern crate handlebars;
extern crate serde;
// extern crate yaserde;

mod code_generator;
mod templates;

use crate::code_generator::get_fix_spec;
use code_generator::*;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fs};

pub fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let source = root.join("resources");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:warning={:?}", &out);
    let fix = get_fix_spec(&source, "FIX43.xml");
    generate_fields(&out, "fields.rs", &fix);
    let mut mod_rs = fs::File::create(out.join("mod.rs")).expect("mod rs");
    mod_rs.write_all(b"pub mod fields;").expect("pub mod");
}
