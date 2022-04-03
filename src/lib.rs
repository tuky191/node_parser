extern crate onig;
use crate::parser::ParseData;
use crate::structer::SetStructure;
use crate::structer::StructuredData;
use neon::prelude::*;
use std::collections::HashMap;
mod output;
mod parser;
mod structer;
use output::OuputFormatter;
use output::OutputFormat;
//use indicatif::{ProgressBar, ProgressStyle};
//use log::{info, warn};
use serde::{Deserialize, Serialize};
//use serde_json::Result;
use serde_json::{Map, Value};
use std::rc::Rc;
extern crate log;
/*
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map:HashMap<String, String> = HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}
*/
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum VariableType {
    List(Vec<String>),
    String(String),
    HashMap(HashMap<String, String>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonConfig {
    name: String,
    searched_key: String,
    composite_key: Vec<String>,
    key_lists: Vec<String>,
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("parse", parse)?;
    Ok(())
}

pub fn parse(mut cx: FunctionContext) -> JsResult<JsString> {
    let mut parsed_data_obj = parser::ParsedData::new();
    //let config_hashmap = load_structure_from_file(&config).unwrap();
    let config_hashmap: Vec<JsonConfig> =
        serde_json::from_str(&cx.argument::<JsString>(2)?.value(&mut cx)).unwrap();

    let value_vec: Vec<Value> =
        serde_json::from_str(&cx.argument::<JsString>(1)?.value(&mut cx)).unwrap();
    let mut regex_hashmap: HashMap<String, String> = HashMap::new();
    for item in value_vec.iter() {
        let map: Map<String, Value> = item.as_object().unwrap().clone();
        for (key, value) in map {
            regex_hashmap.insert(key.to_string(), value.as_str().unwrap().to_string());
        }
    }
    parsed_data_obj.set_source_config(&cx.argument::<JsString>(0)?.value(&mut cx));
    parsed_data_obj.set_regex_hashmap(regex_hashmap);
    parsed_data_obj.parse();

    let rc_parsed_data_obj = Rc::new(parsed_data_obj);
    let mut objects_hashmap: HashMap<&String, StructuredData> = HashMap::new();
    for element in config_hashmap.iter() {
        objects_hashmap.insert(&element.name, StructuredData::new());
        if let Some(reff) = objects_hashmap.get_mut(&element.name) {
            reff.set_parsed_data(rc_parsed_data_obj.clone());
            reff.set_searched_key(element.searched_key.clone());
            reff.set_composite_key(element.composite_key.clone());
            reff.set_key_lists(element.key_lists.clone());
            reff.calculate();
        }
    }
    let mut output_format = OutputFormat::new();
    output_format.set_map(&config_hashmap, &objects_hashmap);
    output_format.set_format(&cx.argument::<JsString>(3)?.value(&mut cx));
    //println!("{}", output_format.formatted_output);

    Ok(cx.string(output_format.formatted_output))
}
