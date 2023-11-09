use std::{fs::File, io::Write};

use chumsky::Parser;


mod parsing;
mod syntax;
mod target;
mod resolution;

const CODE: &str =
r#"

let
    let
        mod unit where
            pro new
    in
        mod bool where
            let
                sum match true false
            in
                val match t f = match (\v = t) (\v = f)
                val true = true unit.new
                val false = false unit.new
        mod unit = unit
in
    val main = unit.new
"#;

fn main() {
    let parser = parsing::value(0);
    let syntax = parser.parse(CODE).unwrap();
    let (env, globe) = resolution::value(&syntax).unwrap();

    let mut file = File::create("output.js").unwrap();
    let output = target::js::value(&env, &globe);
    file.write_all(output.as_bytes()).unwrap();

    let mut file = File::create("syntax.js").unwrap();
    file.write_all(format!("{syntax:?}").as_bytes()).unwrap();
}
