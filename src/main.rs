#![allow(dead_code, unused_imports, unused_variables, non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/mod.rs"));

mod data_dictionary;
mod message;
mod quickfix_errors;

use data_dictionary::*;
use fields::*;
use message::*;

pub(crate) const FILE_PATH: &str = "resources/FIX43.xml";

fn main() {
    // let mut message = Message::new();
    // message.set_field(44, 45.678);
    // message.set_field(35, 'A');

    // let header = message.header_mut();
    // header.set_field(76, "gaurav");
    // println!("{:#?}", &message);
    // let price: Result<u32, String> = message.get_field(44);
    // let found_price: f32 = message.get_field(44).unwrap();
    // println!("price = {:?}", found_price);

    // println!("not parsing {:?}", message.get_field::<f32>(35));
    // println!("not found {:?}", message.get_field::<u32>(56));
    // message.set_field(34, 1);
    // println!("out dir - {:?}", env!("OUT_DIR"));
    // let price = Price::new(55.5f32);
    // let msg_typ = MsgType::new("asdf".to_string());
    // println!("{:?}", price);
    // println!("{:?}", msg_typ);
}
