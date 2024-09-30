use std::{cell::RefCell, rc::Rc, sync::Arc};

use rust_lisp::{
    model::{Env, IntType, RuntimeError, Value},
    utils::{require_arg, require_typed_arg},
};

use crate::environment::Container;

pub fn keyboard_char_listener(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let cb = require_arg("listener/keyboard/char", &args, 0)?;
    let outside = Arc::clone(&outside);
    let mut outside = outside.write().unwrap();
    let i = outside.listeners.keyboard_char.len();
    outside.listeners.keyboard_char.insert(i as i32, cb.clone());
    return Ok(Value::Int(i as IntType));
}

pub fn listener_clear(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let cb = require_typed_arg::<IntType>("listener/clear", &args, 0)?;
    let outside = Arc::clone(&outside);
    let mut outside = outside.write().unwrap();
    outside.listeners.keyboard_char.remove(&(cb as i32));
    return Ok(Value::NIL);
}
