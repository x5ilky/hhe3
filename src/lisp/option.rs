use std::{cell::RefCell, rc::Rc, sync::Arc};

use rust_lisp::{
    model::{Env, RuntimeError, Symbol, Value},
    utils::require_typed_arg,
};

use crate::environment::{Container, Content, OptionDataSingle};

pub fn option_goto(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let outside_ar = Arc::clone(&outside);
    let mut out = outside_ar.write().unwrap();

    let name = require_typed_arg::<&String>("option-goto", &args, 0)?;
    let next_room = require_typed_arg::<&Symbol>("option-goto", &args, 1)?.clone();

    let content = Content(
        name.chars()
            .map(|v| (v, out.display.current_color.clone()))
            .collect(),
    );

    let outside_ar = Arc::clone(&outside);
    out.options.options.push(OptionDataSingle {
        name: content,
        action: Value::NativeClosure(Rc::new(RefCell::new(move |_env, _args| {
            let outside_ar = Arc::clone(&outside_ar);
            outside_ar.write().unwrap().current_room = next_room.0.clone();
            Ok(Value::NIL)
        }))),
    });

    Ok(Value::NIL)
}
