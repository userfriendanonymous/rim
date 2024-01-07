
#[derive(Clone, Debug)]
pub enum Value {
    Init(bool),
    And,
    Or,
    Match,
}
