use std::future::Future;

mod store;

const URL: &'static str = "";

trait Fut<Output> = Future<Output = Output>;
trait HttpFut<Output> = Future<Output = reqwest::Result<Output>>;

pub struct Value {
    client: reqwest::Client,
}

impl Value {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new()
        }
    }
}