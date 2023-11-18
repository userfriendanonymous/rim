use chumsky::{Parser, primitive::filter_map, prelude::Simple};
use crate::syntax::ident::item::{Value, WithCasing};

pub fn from_char(value: char) -> Option<Value> {
    Some(match value {
        '1' => Value::One,
        '2' => Value::Two,
        '3' => Value::Three,
        _ => Value::WithCasing(value.is_ascii_lowercase(), match value.to_ascii_lowercase() {
            'q' => WithCasing::Q,
            'w' => WithCasing::W,
            'e' => WithCasing::E,
            'r' => WithCasing::R,
            't' => WithCasing::T,
            'y' => WithCasing::Y,
            'u' => WithCasing::U,
            'i' => WithCasing::I,
            'o' => WithCasing::O,
            'p' => WithCasing::P,
            'a' => WithCasing::A,
            's' => WithCasing::S,
            'd' => WithCasing::D,
            'f' => WithCasing::F,
            'g' => WithCasing::G,
            'h' => WithCasing::H,
            'j' => WithCasing::J,
            'k' => WithCasing::K,
            'l' => WithCasing::L,
            'z' => WithCasing::Z,
            'x' => WithCasing::X,
            'c' => WithCasing::C,
            'v' => WithCasing::V,
            'b' => WithCasing::B,
            'n' => WithCasing::N,
            'm' => WithCasing::M,
            _ => None?
        }),
        _ => None?
    })
}

pub fn value() -> impl Parser<char, Value, Error = Simple<char>> + Clone {
    filter_map(|span, c: char| from_char(c).ok_or(Simple::custom(span, "invalid char")))
}
