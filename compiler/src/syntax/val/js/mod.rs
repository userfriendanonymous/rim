pub use node::Value as Node;
pub use browser::Value as Browser;

pub mod node;
pub mod browser;

#[derive(Clone, Debug)]
pub enum Value {
    Timeout(Timeout),
    Interval(Interval),
    Console(Console),
    Bind,
    Node(Node),
    Browser(Browser),
}

#[derive(Clone, Debug)]
pub enum Timeout {
    Set,
    Clear,
}

#[derive(Clone, Debug)]
pub enum Interval {
    Set,
    Clear,
}

#[derive(Clone, Debug)]
pub enum Console {
    Log,
    Error,
    Warn,
}
