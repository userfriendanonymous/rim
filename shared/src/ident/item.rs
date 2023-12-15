
#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
pub enum Value {
    WithCasing(bool, WithCasing),
    Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine,
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
            Self::WithCasing(is_upper, v) => {
                let lowercase = match v {
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
                    WithCasing::A => 'a',
                    WithCasing::S => 's',
                    WithCasing::D => 'd',
                    WithCasing::F => 'f',
                    WithCasing::G => 'g',
                    WithCasing::H => 'h',
                    WithCasing::J => 'j',
                    WithCasing::K => 'k',
                    WithCasing::L => 'l',
                    WithCasing::Z => 'z',
                    WithCasing::X => 'x',
                    WithCasing::C => 'c',
                    WithCasing::V => 'v',
                    WithCasing::B => 'b',
                    WithCasing::N => 'n',
                    WithCasing::M => 'm',
                };
                if *is_upper { lowercase.to_ascii_uppercase() } else { lowercase }
            },
            Self::Zero => '0',
            Self::One => '1',
            Self::Two => '2',
            Self::Three => '3',
            Self::Four => '4',
            Self::Five => '5',
            Self::Six => '6',
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine => '9',
        }
    }

    pub fn from_char(value: char) -> Option<Value> {
        Some(match value {
            '0' => Value::Zero,
            '1' => Value::One,
            '2' => Value::Two,
            '3' => Value::Three,
            '4' => Value::Four,
            '5' => Value::Five,
            '6' => Value::Six,
            '7' => Value::Seven,
            '8' => Value::Eight,
            '9' => Value::Nine,
            _ => Value::WithCasing(value.is_ascii_uppercase(), match value.to_ascii_lowercase() {
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
            })
        })
    }    
}