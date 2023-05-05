use std::fs::read_to_string;

use plantuml_state_diagram_parser::parse_state_diagram;

pub fn main() {
    let input = read_to_string("examples/game.puml").unwrap();

    match parse_state_diagram(&input) {
        Ok(result) => {
            dbg!(result);
            // traverse(&mut result, 0);
        }
        Err(e) => {
            panic!("Failed to parse PUML: {:?}", e);
        }
    }
}
