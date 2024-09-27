use std::{cell::RefCell, rc::Rc};

use rust_lisp::{model::{Env, RuntimeError, Value}, utils::{require_arg, require_typed_arg}};

use crate::environment::Container;

use super::color::Color;

pub fn set_name(_env: Rc<RefCell<Env>>, args: Vec<Value>, outside: Container) -> Result<Value, RuntimeError> {
    let name = require_typed_arg::<&String>("title-set-name", &args, 0)?;
    let mut outside = outside.write().unwrap();
    outside.title.content = name.to_string();
    return Ok(Value::NIL);
}
pub fn set_color(_env: Rc<RefCell<Env>>, args: Vec<Value>, outside: Container) -> Result<Value, RuntimeError> {
    let color = require_typed_arg::<Color>("title-set-color", &args, 0)?;
    let mut outside = outside.write().unwrap();
    outside.title.color = color;
    return Ok(Value::NIL);
}
pub fn show(_env: Rc<RefCell<Env>>, args: Vec<Value>, outside: Container) -> Result<Value, RuntimeError> {
    let show = require_arg("title-show", &args, 0)?;
    let mut outside = outside.write().unwrap();
    outside.title.show = match show {
        Value::False => false,
        Value::True => true,
        Value::Int(n) => *n > 0,
        Value::Float(n) => *n > 0.,
        _ => false
    };
    return Ok(Value::NIL);
}