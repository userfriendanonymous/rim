pub use effect::Value as Effect;
pub mod effect;

#[derive(Clone, Debug)]
pub enum Value {
    Effect(Effect),
}
