mod comment;
mod float;
mod integer;
mod string;
mod symbol;

#[derive(Debug, PartialEq, Clone)]
pub enum AtomType {
    Integer(i64),
    Float(f64),
    Symbol(String),
    String(String),
    Null,
}
