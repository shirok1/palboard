use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "unreal_struct.pest"]
pub struct UnrealSturctParser;

#[derive(Debug)]
pub enum Value {
    Primitive(String),
    String(String),
}

// pub fn is_parentheses_surrounded(s: &str) -> bool {
//     s.starts_with('(') && s.ends_with(')')
// }

pub fn parse_struct(s: &str) -> Vec<(String, Value)> {
    // assert!(is_parentheses_surrounded(s));
    assert!(s.starts_with('(') && s.ends_with(')'));
    let s = UnrealSturctParser::parse(Rule::r#struct, s)
        .unwrap()
        .next()
        .unwrap();
    let mut props: Vec<(String, Value)> = Vec::new();
    for prop in s.into_inner().filter(|p| p.as_rule() == Rule::prop) {
        let mut inner = prop.into_inner();
        let name = inner.next().unwrap().as_str();
        let value = inner.next().unwrap();

        props.push((
            name.to_string(),
            match value.as_rule() {
                Rule::primitive => Value::Primitive(value.as_str().to_string()),
                Rule::string => Value::String(value.as_str().trim_matches('"').to_string()),
                _ => unreachable!(),
            },
        ));
    }
    props
}
