use clap::Parser;

mod fs_utils;
mod command;
mod text_utils;

#[tokio::main]
async fn main() {
    command::Value::parse().run().await;
}
