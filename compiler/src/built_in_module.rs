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
                .with_val("bind", Val::Js(Js::Bind))
                .nest("timeout", {
                    use js::Timeout;
                    Builder::new()
                        .with_val("set", Val::Js(Js::Timeout(Timeout::Set)))
                        .with_val("clear", Val::Js(Js::Timeout(Timeout::Clear)))
                })
                .nest("interval", {
                    use js::Interval;
                    Builder::new()
                        .with_val("set", Val::Js(Js::Interval(Interval::Set)))
                        .with_val("clear", Val::Js(Js::Interval(Interval::Clear)))
                })
                .nest("console", {
                    use js::Console;
                    Builder::new()
                        .with_val("log", Val::Js(Js::Console(Console::Log)))
                        .with_val("warn", Val::Js(Js::Console(Console::Warn)))
                        .with_val("error", Val::Js(Js::Console(Console::Error)))
                })
                .nest("node", {
                    use js::Node;
                    Builder::new()
                        .with_val("fromSuper", Val::Js(Js::Node(Node::FromSuper)))
                })
                .nest("browser", {
                    use js::Browser;
                    Builder::new()
                        .with_val("fromSuper", Val::Js(Js::Browser(Browser::FromSuper)))
                        .with_val("alert", Val::Js(Js::Browser(Browser::Alert)))
                })
        })
        .into()
}
