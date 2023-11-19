use std::{fs::File, io::Write, collections::BTreeMap};
use ariadne::{Label, Source};
use chumsky::Parser;
use depending::Dependency;

mod parsing;
mod syntax;
mod target;
mod resolution;
mod depending;

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
in let
    val usEdCool123Wow = builtin.js.effect.console.log "
        This text will be printed because the effect is connected to the main function!
        "
in
    val main = builtin.js.effect.chain
        (builtin.js.effect.chain
            (builtin.js.effect.console.log "hello!!")
            (builtin.js.effect.console.log "lol, those effects are chained!"))
            usEdCool123Wow
"#;

const CODE_: &str =
r#"
let
 mod idk where
  let val
   so =
    5
   idk = 10
  in val
   x = 10
in val
 main = idk.x
"#;

fn main() {
    let parser = parsing::value(Default::default());
    let syntax = parser.parse(CODE);

    match syntax {
        Ok(syntax) => {
            write_string_to_file("syntax.js", &format!("{:#?}", &syntax));

            let mut deps = BTreeMap::new();
            deps.insert("builtin".into(), Dependency::BuiltIn);

            let (env, globe) = depending::resolve(&syntax, deps).unwrap();

            write_string_to_file("lazy-output.js", &target::js::lazy::value(&env, &globe));
            write_string_to_file("strict-output.js", &target::js::strict::value(&env, &globe));

            let mut file = File::create("syntax.js").unwrap();
            file.write_all(format!("{syntax:?}").as_bytes()).unwrap();
        },
        Err(errors) => {
            for error in errors {
                ariadne::Report::build(ariadne::ReportKind::Error, (), 0)
                    .with_message(&error.to_string())
                    .with_label(
                        Label::new(error.span())
                        .with_message("Here")
                    )
                    .with_note("Note!")
                    .finish()
                    .print(Source::from(CODE.clone()))
                    .unwrap();
            }
        }
    }
}

fn write_string_to_file(path: &str, data: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}
