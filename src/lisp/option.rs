use std::{cell::RefCell, rc::Rc, sync::Arc};

use ratatui::widgets::ListState;
use rust_lisp::{
    model::{Env, RuntimeError, Symbol, Value},
    utils::{require_arg, require_typed_arg},
};

use crate::environment::{Container, Content, ContentChar, OptionDataSingle};

pub fn option_reset(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let outside_ar = Arc::clone(&outside);
    let mut out = outside_ar.write().unwrap();

    out.options.options = vec![];
    out.options.selected = ListState::default();

    Ok(Value::NIL)
}

pub fn option_goto(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let outside_ar = Arc::clone(&outside);
    let mut out = outside_ar.write().unwrap();

    let name = require_typed_arg::<&String>("option/goto", &args, 0)?;
    let next_room = require_typed_arg::<&Symbol>("option/goto", &args, 1)?.clone();

    let content = Content(
        name.chars()
            .map(|v| out.display.to_content_char(v))
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

pub fn option_action(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let outside_ar = Arc::clone(&outside);
    let mut out = outside_ar.write().unwrap();

    let name = require_typed_arg::<&String>("option/action", &args, 0)?;
    let action = require_arg("option/action", &args, 1)?.clone();

    let content = Content(
        name.chars()
            .map(|v| out.display.to_content_char(v))
            .collect(),
    );

    out.options.options.push(OptionDataSingle {
        name: content,
        action,
    });

    Ok(Value::NIL)
}
