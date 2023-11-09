
#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
pub enum Value {
    Q, W, E, R, T, Y, U, I, O, P,
    A, S, D, F, G, H, J, K, L,
    Z, X, C, V, B, N, M,
    One, Two, Three,
}

impl Value {
    pub fn to_char(&self) -> char {
        match self {
            Self::Q => 'q',
            Self::W => 'w',
            Self::E => 'e',
            Self::R => 'r',
            Self::T => 't',
            Self::Y => 'y',
            Self::U => 'u',
            Self::I => 'i',
            Self::O => 'o',
            Self::P => 'p',
            _ => todo!()
        }
    }
}