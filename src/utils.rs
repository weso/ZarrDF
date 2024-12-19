use fcsd::Set;
use oxrdf::Term as OxTerm;
use serde_json::Value;

pub fn rdf_to_value(terms: Set) -> Value {
    terms
        .iter()
        .map(|(_, term)| std::str::from_utf8(&term).unwrap().to_string())
        .collect::<Vec<_>>()
        .into()
}

pub fn value_to_rdf(value: &Value) -> Vec<String> {
    let mut terms: Vec<_> = value
        .as_array()
        .unwrap()
        .iter()
        .map(|term| term.as_str().unwrap().to_string())
        .collect();
    terms.sort();
    terms
}

pub fn serialize_term(term: impl Into<OxTerm>) -> String {
    match term.into() {
        OxTerm::BlankNode(id) => format!("_:{id}"),
        OxTerm::NamedNode(iri) => iri.to_string(),
        OxTerm::Literal(literal) => match literal.destruct() {
            (value, None, None) => value,
            (value, Some(datatype), None) => format!("\"{value}\"^^{datatype}"),
            (value, None, Some(language)) => format!("\"{value}\"@{language}"),
            _ => unreachable!(),
        },
    }
}
