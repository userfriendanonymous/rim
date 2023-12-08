
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Colon,
    SemiColon,
    Dollar,
    ArrowLeft,
    ArrowRight,
    Question,
    Exclamation,
    Percent,
    Period,
    Bar,
    Ampersand
}

impl Value {
    pub fn from_char(v: char) -> Option<Self> {
        Some(match v {
            '+' => Self::Plus,
            '-' => Self::Minus,
            '*' => Self::Asterisk,
            '/' => Self::Slash,
            ':' => Self::Colon,
            ';' => Self::SemiColon,
            '$' => Self::Dollar,
            '<' => Self::ArrowLeft,
            '>' => Self::ArrowRight,
            '?' => Self::Question,
            '!' => Self::Exclamation,
            '.' => Self::Period,
            '%' => Self::Percent,
            '|' => Self::Bar,
            '&' => Self::Ampersand,
            _ => None?
        })
    }
}
