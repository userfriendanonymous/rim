use std::fmt::Display;

use crate::resolution::{Env, Globe, val, Id, Module, module};

pub fn value(env: &Env, globe: &Globe) -> String {
    let utils = format!(
        "{}",
        format!("let $unwrap = wrapped => {{ let output = wrapped[0](); wrapped[0] = () => output; return output }}\n"),
    );

    let main = format!(
        "{}()",
        unwrap_val_out(id(&env.val_id(&"main".into()).unwrap().unwrap()))
    );

    format!(
        "{}{}\n// MAIN\n{}",
        utils,
        module_where(env.structure(), globe),
        main,
    )
}

pub fn module_where(value: &module::r#where::Structure, globe: &Globe) -> String {
    let vals = value.vals().iter().map(
        |(_, id)| format!(
            "let {} = {}\n",
            self::id(&id.unwrap()),
            wrap_val_out(val_out(globe.val_out(id), globe))
        )
    ).collect::<String>();

    let modules = value.modules().iter().map(|(_, id)| {
        match globe.module(id) {
            Module::Ref(_) => format!(""),
            Module::Where(env) => self::module_where(env.structure(), globe)
        }
    }).collect::<String>();

    let let_ins = value.let_ins().iter().map(|value| self::let_in(value, globe)).collect::<String>();

    format!(
        "{}{}{}",
        let_ins,
        modules,
        vals
    )
}

pub fn let_in(value: &module::r#where::structure::LetIn, globe: &Globe) -> String {
    format!(
        "{}{}",
        self::module_where(&value.input, globe),
        self::module_where(&value.output, globe)
    )
}

pub fn id(value: &Id) -> String {
    format!("v{}", value.unwrap().to_string())
}

pub fn unwrap_val_out<W: Display>(wrapped: W) -> String {
    format!("$unwrap({wrapped})")
}

pub fn wrap_val_out<U: Display>(unwrapped: U) -> String {
    format!("[() => {unwrapped}]")
}

pub fn function<I: Display, O: Display>(input: I, output: O) -> String {
    format!("({input} => {})", wrap_val_out(output))
}

pub fn val_out(value: &val::Out, globe: &Globe) -> String {
    match value {
        val::Out::Apply(f, input) => unwrap_val_out(format!("{}({})", val_out(f, globe), wrap_val_out(val_out(&input, globe)))),
        val::Out::Function(input, output) => format!("({} => {})", id(&input.unwrap()), wrap_val_out(val_out(&output, globe))),
        val::Out::Ref(v) => unwrap_val_out(id(&v.unwrap())),
        val::Out::LetIn(input, output) => format!("
            (() => {{ {}return ({}) }})()",
            self::module_where(input.structure(), globe),
            val_out(output, globe)
        ),
        val::Out::Sum(v) => match v {
            val::out::Sum::Init(field_id, _) => format!("($ => {})", wrap_val_out(format!("[{field_id}, $]"))),
            val::out::Sum::Match(type_id) => {
                let len = globe.sum_type(type_id);
    
                let output = (0..*len).rev().fold(
                    format!("{{ throw new Error('Sum type mismatch: $value[0] is not in range of possible branches!') }}"),
                    |prev, id| format!(
                        "if ($value[0] == {id}) {{ return {} }} else {prev}",
                        unwrap_val_out(format!("{}($value[1])", unwrap_val_out(format!("${id}"))))
                    )
                );
                (0..*len).rev().fold(
                    function("$sum", format!(
                        "{{ let $value = {}; {output} }}",
                        unwrap_val_out("$sum")
                    )),
                    |output, input_idx| format!("(${input_idx} => {})", wrap_val_out(output))
                )
            }
        },
        val::Out::Product(v) => match v {
            val::out::Product::Init(type_id) => {
                let len = globe.product_type(type_id);
                let fields = (0..*len).map(|id| format!(
                    "{}, ", format!("${id}")
                )).collect::<String>();
    
                (0..*len).rev().fold(
                    format!("[{fields}]"),
                    |output, input_idx| format!("(${input_idx} => {})", wrap_val_out(output))
                )
            },
            val::out::Product::Field(field_id, _) => {
                function("$value", unwrap_val_out(format!(
                    "{}[{field_id}]", unwrap_val_out("$value")
                )))
            },
        }
        val::Out::String(v) => match v {
            val::out::String::Value(v) => {
                let content = v.chars()
                    .map(|ch| {
                        match ch {
                            '\\' => "\\\\".into(),
                            '"' => "\\\"".into(),
                            '\n' => "\\n".into(),
                            _ => ch.to_string()
                        }
                    })
                    .collect::<String>();
                format!(r#""{content}""#)
            },
        },
        val::Out::Number(v) => match v {
            val::out::Number::Value(v) => {
                let items = v.items.iter().map(|item| item.to_str()).collect::<String>();
                let last = v.last.to_str();
                format!("{items}{last}")
            },
            val::out::Number::Add => function("$l", function("$r", format!(
                "{} + {}",
                unwrap_val_out("$l"),
                unwrap_val_out("$r"),
            ))),
            val::out::Number::Sub => function("$l", function("$r", format!(
                "{} - {}",
                unwrap_val_out("$l"),
                unwrap_val_out("$r"),
            ))),
            val::out::Number::Mul => function("$l", function("$r", format!(
                "{} * {}",
                unwrap_val_out("$l"),
                unwrap_val_out("$r"),
            ))),
            val::out::Number::Div => function("$l", function("$r", format!(
                "{} / {}",
                unwrap_val_out("$l"),
                unwrap_val_out("$r"),
            ))),
        },
        val::Out::Js(v) => match v {
            val::out::Js::Effect(v) => match v {
                val::out::js::Effect::Console(v) => match v {
                    val::out::js::effect::Console::Log => {
                        format!("($ => {})", wrap_val_out(format!("() => console.log({})", unwrap_val_out("$"))))
                    }
                },
                val::out::js::Effect::Chain => function("$1", function("$2", format!(
                    "() => {{ {}(); {}() }}",
                    unwrap_val_out("$1"),
                    unwrap_val_out("$2"),
                )))
            },
        }
    }
}
