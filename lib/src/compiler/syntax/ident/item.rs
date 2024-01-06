
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
}