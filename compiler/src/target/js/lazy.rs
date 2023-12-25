use std::fmt::Display;
use crate::resolution::{Env, Globe, val, Id, Module, module, globe::ValId};

mod browser;
mod node;

pub fn value(env: &Env, globe: &Globe, val_id: ValId) -> String {
    let utils = format!(
        "{}",
        format!("let $unwrap = wrapped => {{ let output = wrapped[0](); wrapped[0] = () => output; return output }}\n"),
    );

    let main = format!(
        "{}()",
        unwrap_val(id(&val_id.unwrap()))
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
            wrap_val(val_out(globe.val_out(id), globe))
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

pub fn unwrap_val<W: Display>(wrapped: W) -> String {
    format!("$unwrap({wrapped})")
}

pub fn wrap_val<U: Display>(unwrapped: U) -> String {
    format!("[() => {unwrapped}]")
}

pub fn function<I: Display, O: Display>(input: I, output: O) -> String {
    format!("({input} => {})", wrap_val(output))
}

pub fn eff<O: Display>(output: O) -> String {
    format!("() => {{ let $o = {}; return {} }}", output, wrap_val("$o"))
}

pub fn curried_function<const INPUTS: usize>(body: impl FnOnce([String; INPUTS]) -> String) -> String {
    let body = body(std::array::from_fn(|idx| unwrap_val(format!("${idx}"))));
    (0..INPUTS).rev().fold(body, |output, input_idx| format!("(${input_idx} => {})", wrap_val(output)))
}

pub fn val_out(value: &val::Out, globe: &Globe) -> String {
    use val::{Out, out};
    match value {
        Out::Apply(f, input) => unwrap_val(format!("{}({})", val_out(f, globe), wrap_val(val_out(&input, globe)))),
        Out::Function(input, output) => format!("({} => {})", id(&input.unwrap()), wrap_val(val_out(&output, globe))),
        Out::Ref(v) => unwrap_val(id(&v.unwrap())),
        Out::LetIn(input, output) => format!("
            (() => {{ {}return ({}) }})()",
            self::module_where(input.structure(), globe),
            val_out(output, globe)
        ),
        Out::Sum(v) => match v {
            out::Sum::Init(field_id, _) => format!("($ => {})", wrap_val(format!("[{field_id}, $]"))),
            out::Sum::Match(type_id) => {
                let len = globe.sum_type(type_id);
    
                let output = (0..*len).rev().fold(
                    format!("{{ throw new Error('Sum type mismatch: $value[0] is not in range of possible branches!') }}"),
                    |prev, id| format!(
                        "if ($value[0] == {id}) {{ return {} }} else {prev}",
                        unwrap_val(format!("{}($value[1])", unwrap_val(format!("${id}"))))
                    )
                );
                (0..*len).rev().fold(
                    function("$sum", format!(
                        "{{ let $value = {}; {output} }}",
                        unwrap_val("$sum")
                    )),
                    |output, input_idx| format!("(${input_idx} => {})", wrap_val(output))
                )
            }
        },
        Out::Enum(v) => match v {
            out::Enum::Init(field_id, _) => format!("{field_id}"),
            out::Enum::Match(type_id) => {
                let len = globe.enum_type(type_id);
    
                let output = (0..*len).rev().fold(
                    format!("{{ throw new Error('Enum type mismatch: $value is not in range of possible branches!') }}"),
                    |prev, id| format!(
                        "if ($value == {id}) {{ return {} }} else {prev}",
                        unwrap_val(format!("${id}"))
                    )
                );
                (0..*len).rev().fold(
                    function("$enum", format!(
                        "{{ let $value = {}; {output} }}",
                        unwrap_val("$enum")
                    )),
                    |output, input_idx| format!("(${input_idx} => {})", wrap_val(output))
                )
            }
        },
        Out::Product(v) => match v {
            out::Product::Init(type_id) => {
                let len = globe.product_type(type_id);
                let fields = (0..*len).map(|id| format!(
                    "{}, ", format!("${id}")
                )).collect::<String>();
    
                (0..*len).rev().fold(
                    format!("[{fields}]"),
                    |output, input_idx| format!("(${input_idx} => {})", wrap_val(output))
                )
            },
            out::Product::Field(field_id, _) => {
                function("$value", unwrap_val(format!(
                    "{}[{field_id}]", unwrap_val("$value")
                )))
            },
        }
        Out::String(v) => match v {
            out::String::Value(v) => {
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
        Out::Number(v) => match v {
            out::Number::Value(v) => {
                let items = v.items.iter().map(|item| item.to_str()).collect::<String>();
                let last = v.last.to_str();
                format!("{items}{last}")
            },
            out::Number::Add => curried_function(|[l, r]| format!("{l} + {r}")),
            out::Number::Sub => curried_function(|[l, r]| format!("{l} - {r}")),
            out::Number::Mul => curried_function(|[l, r]| format!("{l} * {r}")),
            out::Number::Div => curried_function(|[l, r]| format!("{l} / {r}")),
            out::Number::Modulo => curried_function(|[l, r]| format!("{l} % {r}")),
            out::Number::IsEqual => curried_function(|[l, r]| format!("{l} == {r}")),
            out::Number::IsGreater => curried_function(|[l, r]| format!("{l} > {r}")),
        },
        Out::Boolean(v) => match v {
            out::Boolean::Value(v) => if *v { format!("true") } else { format!("false") },
            out::Boolean::And => curried_function(|[l, r]| format!("{l} && {r}")),
            out::Boolean::Or => curried_function(|[l, r]| format!("{l} || {r}")),
            out::Boolean::Match => curried_function(|[f, t, v]| format!("{v} ? {t} : {f}"))
        }
        Out::Js(v) => match v {
            out::Js::Node(v) => node::val(v, globe),
            out::Js::Browser(v) => browser::val(v, globe),
            out::Js::Bind => curried_function(|[v, f]| format!("() => {{ let $o = {}(); return $o }}", unwrap_val(format!("{f}({v}())")))),
            out::Js::Console(v) => match v {
                out::js::Console::Log => curried_function(|[i]| eff(format!("console.log({i})"))),
                out::js::Console::Warn => curried_function(|[i]| eff(format!("console.warn({i})"))),
                out::js::Console::Error => curried_function(|[i]| eff(format!("console.error({i})"))),
            },
            out::Js::Timeout(v) => match v {
                out::js::Timeout::Set => curried_function(|[time, f]| eff(format!("setTimeout({f}, {time})"))),
                out::js::Timeout::Clear => curried_function(|[id]| eff(format!("clearTimeout({id})"))),
            },
            out::Js::Interval(v) => match v {
                out::js::Interval::Set => curried_function(|[time, f]| eff(format!("setInterval({f}, {time})"))),
                out::js::Interval::Clear => curried_function(|[id]| eff(format!("clearInterval({id})"))),
            }
        }
    }
}
