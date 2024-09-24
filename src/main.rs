mod parse;

use crate::parse::parse;

use serde_json::{self, Map, Value};
use std::fs;

const CMU: &'static str = "src/resources/cmu_dict.json";
const CONVERSION: &'static str = "src/resources/conversions.json";


fn main() {
    let parse_map = get_map_from_file(&CMU);
    let conversion_map = get_map_from_file(&CONVERSION);
    let text = "I sat up and said 'The dog is dead.' ' I'd got that far,' he said. I said, 'I think someone killed the dog.' 'How old are you?’ he asked. I didn't answer.";

    let new_text = parse(text, parse_map, conversion_map);

    println!("{new_text}");
}


fn get_map_from_file(filepath: &str) -> Map<String, Value> {
    // TODO: since we know Value will always be the Array(Vec<Value>) variant,
    //       change the Map<String, Value> to Map<String, Vec<String>> somehow
    let db = fs::read_to_string(filepath).expect("Could not read file");
    
    serde_json::from_str(&db).expect("Failed to read json")
}
