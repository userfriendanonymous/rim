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
        # amazing module about booleans! #
        mod bool where
            let #! idk !#
                sum match true false # cool #
            in
                val match t f = match (\v = t) (\v = f)
                val true = true unit.new
                val false = false unit.new
                val coolio = "hello guys!!"
                val faeijf = 105
        mod unit = unit
in
    val main = unit.new
    val amazing #@@ cool wow impressive # @ @@# = 124
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
