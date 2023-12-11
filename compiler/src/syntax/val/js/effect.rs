
#[derive(Clone, Debug)]
pub enum Value {
    Console(Console),
    Chain
}

#[derive(Clone, Debug)]
pub enum Console {
    Log,
}
