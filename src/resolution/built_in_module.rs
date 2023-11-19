use super::{Globe, val, module};

pub fn create(globe: &mut Globe) -> module::Where {
    use val::Out;
    module::Where::build(globe.store_mut())
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
