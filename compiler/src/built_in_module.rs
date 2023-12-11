use crate::syntax::{module::{Builder, self}, Val, val};

pub fn create() -> module::Value {
    Builder::new()
        .nest("number", {
            use val::Number;
            Builder::new()
                .with_val("add", Val::Number(Number::Add))
                .with_val("sub", Val::Number(Number::Sub))
                .with_val("mul", Val::Number(Number::Mul))
                .with_val("div", Val::Number(Number::Div))
                .with_val("mod", Val::Number(Number::Modulo))
                .with_val("eq", Val::Number(Number::IsEqual))
                .with_val("gt", Val::Number(Number::IsGreater))
        })
        .nest("bool", {
            use val::Boolean;
            Builder::new()
                .with_val("true", Val::Boolean(Boolean::Value(true)))
                .with_val("false", Val::Boolean(Boolean::Value(false)))
                .with_val("match", Val::Boolean(Boolean::Match))
                .with_val("and", Val::Boolean(Boolean::And))
                .with_val("or", Val::Boolean(Boolean::Or))
        })
        .nest("js", {
            use val::{Js, js};
            Builder::new()
                .nest("effect", {
                    use js::{Effect, effect};
                    Builder::new()
                        .with_val("chain", Val::Js(Js::Effect(Effect::Chain)))
                        .nest("console", {
                            use effect::Console;
                            Builder::new()
                                .with_val("log", Val::Js(Js::Effect(Effect::Console(Console::Log))))
                        })
                })
        })
        .into()
}
