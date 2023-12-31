use crate::resolution::{Env, Globe, val, Id, Module, module, globe::ValId};

mod node;
mod browser;

pub fn value(env: &Env, globe: &Globe, val_id: ValId) -> String {
    let main = format!(
        "{}()",
        id(&val_id.unwrap())
    );

    format!(
        "{}\n// MAIN\n{}",
        module_where(env.structure(), globe),
        main
    )
}


pub fn module_where(value: &module::r#where::Structure, globe: &Globe) -> String {
    let vals = value.vals().iter().map(
        |(_, id)| format!(
            "let {} = {}\n",
            self::id(&id.unwrap()),
            val_out(globe.val_out(id), globe)
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
        self::module_where(&value.output, globe),
    )
}

pub fn id(value: &Id) -> String {
    format!("v{}", value.unwrap().to_string())
}

pub fn val_out(value: &val::Out, globe: &Globe) -> String {
    use val::{Out, out};
    match value {
        Out::Apply(f, input) => format!("{}({})", val_out(&f, globe), val_out(&input, globe)),
        Out::Function(input, output) => format!("({} => {})", id(&input.unwrap()), val_out(&output, globe)),
        Out::Ref(value) => id(&value.unwrap()),
        Out::LetIn(input, output) => format!("
            (() => {{ {}; return {} }})()",
            self::module_where(input.structure(), globe),
            val_out(output, globe)
        ),
        Out::Sum(v) => match v {
            out::Sum::Init(field_id, _) => {
                format!("$ => [{}, $]", field_id)
            },
            out::Sum::Match(type_id) => {
                let len = globe.sum_type(type_id);
                let inputs = (0..*len).map(|id| format!("${id} => ")).collect::<String>();
                let branches = (0..*len).map(|id| format!(
                    "if ($sum[0] == {id}) {{ return ${id}($sum[1]) }} else "
                )).collect::<String>();
                format!(
                    "({inputs}$sum => {{ {branches}{{ throw new Error('Sum type mismatch: $sum[0] is not in range of possible branches!') }} }})"
                )
            },
        },
        Out::Enum(v) => match v {
            out::Enum::Init(field_id, _) => {
                format!("{}", field_id)
            },
            out::Enum::Match(type_id) => {
                let len = globe.sum_type(type_id);
                let inputs = (0..*len).map(|id| format!("${id} => ")).collect::<String>();
                let branches = (0..*len).map(|id| format!(
                    "if ($enum == {id}) {{ return ${id} }} else "
                )).collect::<String>();
                format!(
                    "({inputs}$enum => {{ {branches}{{ throw new Error('Enum type mismatch: $enum is not in range of possible branches!') }} }})"
                )
            },
        },
        Out::Product(v) => match v {
            out::Product::Field(field_id, _) => format!(
                "($value => $value[{field_id}])"
            ),
            out::Product::Init(type_id) => {
                let len = globe.product_type(type_id);
                let inputs = (0..*len).map(|id| format!("${id} => ")).collect::<String>();
                let fields = (0..*len).map(|id| format!(
                    "${id}, "
                )).collect::<String>();
                format!(
                    "({inputs} [{fields}])"
                )
            },
        },
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
            out::Number::Add => format!("($l => $r => $l + $r)"),
            out::Number::Sub => format!("($l => $r => $l - $r)"),
            out::Number::Mul => format!("($l => $r => $l * $r)"),
            out::Number::Div => format!("($l => $r => $l / $r)"),
            out::Number::Modulo => format!("($l => $r => $l % $r)"),
            out::Number::IsEqual => format!("($l => $r => $l == $r)"),
            out::Number::IsGreater => format!("($l => $r => $l > $r)"),
        },
        Out::Boolean(v) => match v {
            out::Boolean::Value(v) => if *v { "true" } else { "false" }.into(),
            out::Boolean::And => "($l => $r => $l && $r)".into(),
            out::Boolean::Or => "($l => $r => $l || $r)".into(),
            out::Boolean::Match => "($f => $t => $v => $v ? $t : $f)".into()
        },
        Out::Array(v) => match v {
            out::Array::Pair => "(f => s => [f, s])".into(),
            out::Array::Push => "(el => arr => [...arr, el])".into()
        }
        Out::Js(v) => match v {
            out::Js::Node(v) => node::val(v, globe),
            out::Js::Browser(v) => browser::val(v, globe),
            out::Js::Bind => format!("($1 => $2 => () => $2($1())())"),
            out::Js::Throw => "(obj => () => { throw obj })".into(),
            out::Js::Catch => "(v => f => { try { return v() } catch(e) { return f(e)() } })".into(),
            out::Js::Console(v) => match v {
                out::js::Console::Log => format!("($ => () => console.log($))"),
                out::js::Console::Warn => format!("($ => () => console.warn($))"),
                out::js::Console::Error => format!("($ => () => console.error($))"),
            },
            out::Js::Timeout(v) => match v {
                out::js::Timeout::Set => format!("($time => $f => () => setTimeout($f, $time))"),
                out::js::Timeout::Clear => format!("($id => () => clearTimeout($id))"),
            },
            out::Js::Interval(v) => match v {
                out::js::Interval::Set => format!("($time => $f => () => setInterval($f, $time))"),
                out::js::Interval::Clear => format!("($id => () => clearInterval($id))"),
            },
            out::Js::Value(v) => match v {
                out::js::value::Value::Undefined => "(() => undefined)".into(),
                out::js::value::Value::Null => "(() => null)".into(),
                out::js::value::Value::NaN => "(() => NaN)".into(),
                out::js::value::Value::String => "(v => () => v)".into(),
                out::js::value::Value::Field => "(name => obj => () => obj[name])".into(),
                out::js::value::Value::Index => "(idx => obj => () => obj[idx])".into(),
                out::js::value::Value::Eq => "(a => b => () => a == b)".into(),
                out::js::value::Value::Typeof => "(v => () => typeof v)".into()
            }
        }
    }
}
