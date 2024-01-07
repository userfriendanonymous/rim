use clap::Parser;

mod fs_utils;
mod command;
mod text_utils;

#[tokio::main]
pub async fn main() {
    command::Value::parse().run().await;
}
