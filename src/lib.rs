pub mod atom;

use atom::AtomType;
#[derive(Debug, PartialEq)]
enum LispValue {
    Atom(AtomType),
    List(Vec<LispValue>),
}
