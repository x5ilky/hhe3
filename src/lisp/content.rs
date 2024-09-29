use std::{cell::RefCell, rc::Rc, sync::Arc};

use rust_lisp::{
    model::{Env, RuntimeError, Value},
    utils::require_typed_arg,
};

use crate::environment::Container;

use super::color::Color;

pub fn set_delay(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let delay = require_typed_arg::<i32>("delay/set", &args, 0)?;
    let mut outside = outside.write().unwrap();
    outside.display.delay = delay as i64;
    return Ok(Value::NIL);
}

pub fn set_color(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let color = require_typed_arg::<Color>("color/set", &args, 0)?;
    let mut outside = outside.write().unwrap();
    outside.display.current_color = color;
    return Ok(Value::NIL);
}

pub fn content_clear(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let outside = Arc::clone(&outside);
    let mut write = outside.write().unwrap();
    write.display.content.0.clear();
    write.display.displayed_index = 0;

    Ok(Value::NIL)
}
