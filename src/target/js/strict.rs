use crate::resolution::{Env, Globe, val, Id, Module, module};

pub fn value(env: &Env, globe: &Globe) -> String {
    let main = format!(
        "console.log({})",
        id(&env.main_val().unwrap().unwrap())
    );

    format!(
        "{}\n// MAIN\n{}",
        module(env, globe),
        main
    )
}


pub fn module(env: &Env, globe: &Globe) -> String {
    let vals = env.vals().iter().map(
        |(_, id)| format!(
            "let {} = {}\n",
            self::id(&id.unwrap()),
            val_out(globe.val_out(id), globe)
        )
    ).collect::<String>();

    let modules = env.modules().iter().map(|(_, id)| {
        match globe.module(id) {
            Module::Ref(_) => format!(""),
            Module::Where(env) => self::module(env, globe)
        }
    }).collect::<String>();

    let let_ins = env.let_ins().iter().map(|value| self::let_in(value, globe)).collect::<String>();

    format!(
        "{}{}{}",
        let_ins,
        modules,
        vals
    )
}

pub fn let_in(value: &module::LetIn, globe: &Globe) -> String {
    format!(
        "{}{}",
        self::module(&value.input, globe),
        value.output.iter().map(|value| self::let_in(value, globe)).collect::<String>()
    )
}

pub fn id(value: &Id) -> String {
    format!("v{}", value.unwrap().to_string())
}

pub fn val_out(value: &val::Out, globe: &Globe) -> String {
    match value {
        val::Out::Call(f, input) => format!("({})({})", val_out(&f, globe), val_out(&input, globe)),
        val::Out::Function(input, output) => format!("{} => {}", id(&input.unwrap()), val_out(&output, globe)),
        val::Out::Ref(value) => id(&value.unwrap()),
        val::Out::LetIn(input, output) => format!("
            (() => {{ {}; return {} }})()",
            self::module(input, globe),
            val_out(output, globe)
        ),
        val::Out::SumInit(field_id, _) => {
            format!("$ => [{}, $]", field_id)
        },
        val::Out::SumMatch(type_id) => {
            let len = globe.sum_type(type_id);
            let inputs = (0..*len).map(|id| format!("${id} => ")).collect::<String>();
            let branches = (0..*len).map(|id| format!(
                "if ($sum[0] == {id}) {{ return ${id}($sum[1]) }} else "
            )).collect::<String>();
            format!(
                "{inputs}$sum => {{ {branches}{{ throw new Error('Sum type mismatch: $sum[0] is not in range of possible branches!') }} }}"
            )
        },
        val::Out::ProductField(field_id, _) => format!(
            "$value => $value[{field_id}]"
        ),
        val::Out::ProductInit(type_id) => {
            let len = globe.product_type(type_id);
            let inputs = (0..*len).map(|id| format!("${id} => ")).collect::<String>();
            let fields = (0..*len).map(|id| format!(
                "${id}, "
            )).collect::<String>();
            format!(
                "{inputs} [{fields}]"
            )
        },
        val::Out::String(v) => {
            let content = v.chars()
                .map(|ch| {
                    if ch == '\\' {
                        "\\\\".into()
                    } else if ch == '"' {
                        "\\\"".into()
                    } else {
                        ch.to_string()
                    }
                })
                .collect::<String>();
            format!(r#""{content}""#)
        },
        val::Out::Number(v) => {
            let items = v.items.iter().map(|item| item.to_str()).collect::<String>();
            let last = v.last.to_str();
            format!("{items}{last}")
        }
    }
}
