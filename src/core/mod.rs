//! Version-neutral protocol core: the wire message model (`Message`/`FieldMap`/`Group` +
//! parse/serialize/validate) and the runtime `DataDictionary` it consults.

pub mod dictionary;
pub mod message;
