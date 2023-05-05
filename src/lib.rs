#[derive(Debug, PartialEq, Clone)]
pub enum Directive {
    StartUml,
    EndUml,
    SkinParam,
    Style,
    Scale,
    Hide,
    Entry(String),
    Exit(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StateDiagram {
    pub elements: Vec<Element>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StateDeclaration {
    pub name: String,
    pub id: String,
    pub attributes: Option<Attributes>,
    pub state_block: Option<StateBlock>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StateData {
    pub name: String,
    pub type_name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StateBlock {
    pub elements: Vec<Element>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Directive(Directive),
    StateDiagram(StateDiagram),
    StateDeclaration(StateDeclaration),
    Transition(Transition),
    StateNote(StateNote),
    StateData(StateData),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Transition {
    pub from_state: String,
    pub to_state: String,
    pub event: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StateNote {
    pub identifier: String,
    pub content: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Attributes {
    pub color: String,
    pub line_style: Option<String>,
    pub text_style: Option<String>,
}

extern crate pest;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "plantuml-state-diagram.pest"]
pub struct StateDiagramParser;

impl StateDeclaration {
    fn from_pairs(pairs: pest::iterators::Pairs<Rule>) -> Self {
        let mut identifier = String::new();
        let mut string = String::new();
        let mut attributes = None;
        let mut state_block = None;

        for pair in pairs {
            match pair.as_rule() {
                Rule::identifier => {
                    identifier = pair.as_str().to_string();
                }
                Rule::string => {
                    string = pair.as_str().to_string();
                }
                Rule::state_block => {
                    state_block = Some(StateBlock::from_pairs(pair.into_inner()));
                }
                Rule::attributes => {
                    attributes = Some(Attributes::from_pairs(pair.into_inner()));
                }
                _ => unreachable!(),
            }
        }

        if identifier == "" && string != "" {
            identifier = string.clone();
        }

        StateDeclaration {
            id: identifier,
            name: string,
            attributes,
            state_block,
        }
    }
}

impl StateBlock {
    fn from_pairs(pairs: pest::iterators::Pairs<Rule>) -> Self {
        let elements = parse_elements(pairs);
        StateBlock { elements }
    }
}

impl StateDiagram {
    fn from_pairs(pairs: pest::iterators::Pairs<Rule>) -> Self {
        let elements = parse_elements(pairs);
        StateDiagram { elements }
    }
}

impl Transition {
    fn from_pairs(pairs: pest::iterators::Pairs<Rule>) -> Self {
        let mut from_state = String::new();
        let mut to_state = String::new();
        let mut event = None;

        for pair in pairs {
            match pair.as_rule() {
                Rule::from_state => {
                    from_state = pair.as_str().to_string();
                }
                Rule::to_state => {
                    to_state = pair.as_str().to_string();
                }
                Rule::event => {
                    event = Some(
                        pair.as_str()
                            .to_owned()
                            .trim_start()
                            .trim_start_matches(':')
                            .trim_start()
                            .trim_end()
                            .to_owned(),
                    );
                }
                Rule::tr_arrow => {
                    // TODO: Handle different kinds of arrows?
                }
                x => unreachable!("{:?}", x),
            }
        }

        Transition {
            event,
            from_state,
            to_state,
        }
    }
}

impl StateNote {
    fn from_pairs(pairs: pest::iterators::Pairs<Rule>) -> Self {
        let mut content = String::new();
        let mut identifier = String::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::identifier => {
                    identifier = pair.as_str().to_string();
                }
                Rule::string => {
                    content = pair.as_str().to_string();
                }
                _ => unreachable!(),
            }
        }
        StateNote {
            content,
            identifier,
        }
    }
}

impl Attributes {
    fn from_pairs(pairs: pest::iterators::Pairs<Rule>) -> Self {
        let mut color = String::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::color => {
                    color = pair.as_str().to_string();
                }
                _ => unreachable!(),
            }
        }
        Attributes {
            color,
            line_style: None,
            text_style: None,
        }
    }
}

impl StateData {
    fn from_pairs(pairs: pest::iterators::Pairs<Rule>) -> Self {
        let mut name = String::new();
        let mut type_name = String::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::identifier => {
                    name = pair.as_str().to_string();
                }
                Rule::string => {
                    type_name = pair.as_str().to_string();
                }
                _ => unreachable!(),
            }
        }

        StateData { name, type_name }
    }
}

fn parse_elements(pairs: pest::iterators::Pairs<Rule>) -> Vec<Element> {
    let mut elements = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::directive => {
                let directive = match pair.as_str() {
                    "@startuml" => Directive::StartUml,
                    "@enduml" => Directive::EndUml,
                    "skinparam" => Directive::SkinParam,
                    "style" => Directive::Style,
                    "scale" => Directive::Scale,
                    "hide" => Directive::Hide,
                    inner => {
                        unreachable!("{:?}", inner)
                    }
                };
                elements.push(Element::Directive(directive));
            }
            Rule::state_data => {
                let state_data = StateData::from_pairs(pair.into_inner());
                elements.push(Element::StateData(state_data));
            }
            Rule::state_declaration => {
                let state_declaration = StateDeclaration::from_pairs(pair.into_inner());
                elements.push(Element::StateDeclaration(state_declaration));
            }
            Rule::transition => {
                let transition = Transition::from_pairs(pair.into_inner());
                elements.push(Element::Transition(transition));
            }
            Rule::state_note => {
                let state_note = StateNote::from_pairs(pair.into_inner());
                elements.push(Element::StateNote(state_note));
            }
            Rule::state_diagram => {
                let state_diagram = StateDiagram::from_pairs(pair.into_inner());
                elements.push(Element::StateDiagram(state_diagram));
            }
            Rule::EOI => {
                // All good, at end
            }
            x => unreachable!("{:?}", x),
        }
    }
    elements
}

// The result type is 184 bytes. Since this is nothing performance critical,
// I choose to not do anything about this.
#[allow(clippy::result_large_err)]
pub fn parse_state_diagram(input: &str) -> Result<StateDiagram, pest::error::Error<Rule>> {
    let pairs = StateDiagramParser::parse(Rule::state_diagram, input)?;
    let diag = StateDiagram::from_pairs(pairs);
    Ok(diag)
}
