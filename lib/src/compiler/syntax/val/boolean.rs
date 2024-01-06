
#[derive(Clone, Debug)]
pub enum Value {
    Value(bool),
    And,
    Or,
    Match,
}
