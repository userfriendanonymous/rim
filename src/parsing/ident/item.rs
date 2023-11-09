use chumsky::{Parser, primitive::filter_map, prelude::Simple};
use crate::syntax::ident::item::Value;

pub fn from_char(value: char) -> Option<Value> {
    Some(match value {
        'q' => Value::Q,
        'w' => Value::W,
        'e' => Value::E,
        'r' => Value::R,
        't' => Value::T,
        'y' => Value::Y,
        'u' => Value::U,
        'i' => Value::I,
        'o' => Value::O,
        'p' => Value::P,
        'a' => Value::A,
        's' => Value::S,
        'd' => Value::D,
        'f' => Value::F,
        'g' => Value::G,
        'h' => Value::H,
        'j' => Value::J,
        'k' => Value::K,
        'l' => Value::L,
        'z' => Value::Z,
        'x' => Value::X,
        'c' => Value::C,
        'v' => Value::V,
        'b' => Value::B,
        'n' => Value::N,
        'm' => Value::M,
        '1' => Value::One,
        '2' => Value::Two,
        '3' => Value::Three,
        _ => None?
    })
}

pub fn value() -> impl Parser<char, Value, Error = Simple<char>> + Clone {
    filter_map(|span, c: char| from_char(c).ok_or(Simple::custom(span, "invalid char")))
}
