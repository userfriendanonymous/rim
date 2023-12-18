use crate::resolution::{Env, Globe, val, Id, Module, module, globe::ValId};

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
    match value {
        val::Out::Apply(f, input) => format!("{}({})", val_out(&f, globe), val_out(&input, globe)),
        val::Out::Function(input, output) => format!("({} => {})", id(&input.unwrap()), val_out(&output, globe)),
        val::Out::Ref(value) => id(&value.unwrap()),
        val::Out::LetIn(input, output) => format!("
            (() => {{ {}; return {} }})()",
            self::module_where(input.structure(), globe),
            val_out(output, globe)
        ),
        val::Out::Sum(v) => match v {
            val::out::Sum::Init(field_id, _) => {
                format!("$ => [{}, $]", field_id)
            },
            val::out::Sum::Match(type_id) => {
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
        val::Out::Enum(v) => match v {
            val::out::Enum::Init(field_id, _) => {
                format!("{}", field_id)
            },
            val::out::Enum::Match(type_id) => {
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
        val::Out::Product(v) => match v {
            val::out::Product::Field(field_id, _) => format!(
                "($value => $value[{field_id}])"
            ),
            val::out::Product::Init(type_id) => {
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
            val::out::Number::Add => format!("($l => $r => $l + $r)"),
            val::out::Number::Sub => format!("($l => $r => $l - $r)"),
            val::out::Number::Mul => format!("($l => $r => $l * $r)"),
            val::out::Number::Div => format!("($l => $r => $l / $r)"),
            val::out::Number::Modulo => format!("($l => $r => $l % $r)"),
            val::out::Number::IsEqual => format!("($l => $r => $l == $r)"),
            val::out::Number::IsGreater => format!("($l => $r => $l > $r)"),
        },
        val::Out::Boolean(v) => match v {
            val::out::Boolean::Value(v) => if *v { "true" } else { "false" }.into(),
            val::out::Boolean::And => "($l => $r => $l && $r)".into(),
            val::out::Boolean::Or => "($l => $r => $l || $r)".into(),
            val::out::Boolean::Match => "($f => $t => $v => $v ? $t : $f)".into()
        },
        val::Out::Js(v) => match v {
            val::out::Js::Effect(v) => match v {
                val::out::js::Effect::Chain => format!("($1 => $2 => () => {{ $1(); $2() }})")
            },
            val::out::Js::Console(v) => match v {
                val::out::js::Console::Log => format!("($ => () => console.log($))")
            },
            val::out::Js::SetTimeout => format!("($time => $f => () => {{ setTimeout($f, $time) }})"),
            val::out::Js::Alert => format!("($i => () => {{ alert($i) }})")
        }
    }
}
