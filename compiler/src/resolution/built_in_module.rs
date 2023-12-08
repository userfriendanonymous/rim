use super::{Globe, val, module};

pub fn create(globe: &mut Globe) -> module::Where {
    use val::Out;
    module::Where::build(globe.store_mut())
        .nest_module("number", |m| {
            use val::out::Number;
            m
                .with_val("add", Out::Number(Number::Add))
                .with_val("sub", Out::Number(Number::Sub))
                .with_val("mul", Out::Number(Number::Mul))
                .with_val("div", Out::Number(Number::Div))
                .with_val("mod", Out::Number(Number::Modulo))
                .with_val("eq", Out::Number(Number::IsEqual))
                .with_val("gt", Out::Number(Number::IsGreater))
        })
        .nest_module("bool", |m| {
            use val::out::{Boolean, boolean};
            m
                .with_val("true", Out::Boolean(Boolean::Init(true)))
                .with_val("false", Out::Boolean(Boolean::Init(false)))
                .with_val("match", Out::Boolean(Boolean::Match))
                .with_val("and", Out::Boolean(Boolean::And))
                .with_val("or", Out::Boolean(Boolean::Or))
        })
        .nest_module("js", |m| {
            use val::out::{js, Js};
            m.nest_module("effect", |m| {
                use js::{Effect, effect};
                m
                    .with_val("chain", Out::Js(Js::Effect(Effect::Chain)))
                    .nest_module("console", |m| {
                        use effect::Console;
                        m
                            .with_val("log", Out::Js(Js::Effect(Effect::Console(Console::Log))))
                    })
            })
        })
        .end()
}
