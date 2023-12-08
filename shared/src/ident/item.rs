
#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
pub enum Value {
    WithCasing(bool, WithCasing),
    One, Two, Three,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
pub enum WithCasing {
    Q, W, E, R, T, Y, U, I, O, P,
    A, S, D, F, G, H, J, K, L,
    Z, X, C, V, B, N, M,
}

impl Value {
    pub fn to_char(&self) -> char {
        match self {
            Self::WithCasing(is_upper, v) => match v {
                WithCasing::Q => 'q',
                WithCasing::W => 'w',
                WithCasing::E => 'e',
                WithCasing::R => 'r',
                WithCasing::T => 't',
                WithCasing::Y => 'y',
                WithCasing::U => 'u',
                WithCasing::I => 'i',
                WithCasing::O => 'o',
                WithCasing::P => 'p',
                _ => todo!()
            },
            _ => todo!()
        }
    }

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
}