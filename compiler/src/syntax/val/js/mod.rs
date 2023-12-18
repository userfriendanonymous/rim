pub use effect::Value as Effect;
pub mod effect;

#[derive(Clone, Debug)]
pub enum Value {
    Effect(Effect),
    SetTimeout,
    Alert,
    Console(Console)
}

#[derive(Clone, Debug)]
pub enum Console {
    Log,
}
