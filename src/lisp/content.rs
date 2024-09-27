use std::{cell::RefCell, rc::Rc};

use rust_lisp::{model::{Env, RuntimeError, Value}, utils::require_typed_arg};

use crate::environment::EnvData;

use super::color::Color;


pub fn set_delay(_env: Rc<RefCell<Env>>, args: Vec<Value>, outside: Rc<RefCell<EnvData>>) -> Result<Value, RuntimeError> {
    let delay = require_typed_arg::<i32>("delay-set", &args, 0)?;
    let mut outside = outside.borrow_mut();
    outside.display.delay = delay as i64;
    return Ok(Value::NIL);
}

pub fn set_color(_env: Rc<RefCell<Env>>, args: Vec<Value>, outside: Rc<RefCell<EnvData>>) -> Result<Value, RuntimeError> {
    let color = require_typed_arg::<Color>("color-set", &args, 0)?;
    let mut outside = outside.borrow_mut();
    outside.display.current_color = color;
    return Ok(Value::NIL);
}