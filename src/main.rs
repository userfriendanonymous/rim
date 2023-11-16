use std::{fs::File, io::Write};
use chumsky::Parser;

mod parsing;
mod syntax;
mod target;
mod resolution;

const CODE: &str =
r#"
let
    mod unit where
        pro new
    mod tuple where
        pro new fst snd
in let
    # amazing module about booleans! #
    mod bool where
        let
            sum match false true
        in
            let val
                match t f = match (\v = t) (\v = f)
                true = true unit.new
                false = false unit.new
            in val
                match = match
                true = true
                false = false
                not = match true false

in let
    mod either where
        let sum match left right
        in let val isleft = match (\v = bool.true) (\v = bool.false)
        in val
            match = match
            left = left
            right = right
            isleft = isleft
            isright v = bool.not (isleft v)
in
    val main = either.isleft (either.left ((\f = f f) (\f = f f)))
"#;

fn main() {
    let parser = parsing::value(0);
    let syntax = parser.parse(CODE).unwrap();

    write_string_to_file("syntax.js", &format!("{:#?}", &syntax));

    let (env, globe) = resolution::value(&syntax).unwrap();

    write_string_to_file("lazy-output.js", &target::js::lazy::value(&env, &globe));
    write_string_to_file("strict-output.js", &target::js::strict::value(&env, &globe));

    let mut file = File::create("syntax.js").unwrap();
    file.write_all(format!("{syntax:?}").as_bytes()).unwrap();
}

fn write_string_to_file(path: &str, data: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}
