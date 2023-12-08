#![feature(trait_alias)]

use std::{fs::File, io::{Write, Read}, collections::BTreeMap};
use ariadne::{Label, Source};
use chumsky::Parser;
use depending::Dependency;

pub mod parsing;
pub mod syntax;
pub mod target;
pub mod resolution;
mod depending;
pub mod project;

#[test]
fn main() {
    let mut code = String::new();
    File::open("code.rim").unwrap().read_to_string(&mut code).unwrap();

    let parser = parsing::value(Default::default());
    let syntax = parser.parse(code.clone());

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
                    .print(Source::from(code.clone()))
                    .unwrap();
            }
        }
    }
}

fn write_string_to_file(path: &str, data: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}
