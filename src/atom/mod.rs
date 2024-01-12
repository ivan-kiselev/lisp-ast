mod comment;
mod float;
mod integer;
mod string;

#[derive(Debug, PartialEq)]
pub enum AtomType {
    Integer(i64),
    Float(f64),
    Symbol(String),
    String(String),
    Char(char),
    OneLineComment(String),
}
