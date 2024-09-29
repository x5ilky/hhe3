use std::{cell::RefCell, rc::Rc};

use rust_lisp::{
    model::{Env, RuntimeError, Value},
    utils::{require_arg, require_typed_arg},
};

use crate::environment::Container;

use super::color::Color;

pub fn set_name(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let name = require_typed_arg::<&String>("title/name/set", &args, 0)?;
    let mut outside = outside.write().unwrap();
    outside.title.content = name.to_string();
    return Ok(Value::NIL);
}
pub fn set_fg(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let color = require_typed_arg::<Color>("title/fg/set", &args, 0)?;
    let mut outside = outside.write().unwrap();
    outside.title.fg = color;
    return Ok(Value::NIL);
}
pub fn set_bg(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let color = require_typed_arg::<Color>("title/bg/set", &args, 0)?;
    let mut outside = outside.write().unwrap();
    outside.title.bg = color;
    return Ok(Value::NIL);
}
pub fn bold(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let mut outside = outside.write().unwrap();
    outside.title.bold = true;
    return Ok(Value::NIL);
}
pub fn italic(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let mut outside = outside.write().unwrap();
    outside.title.italic = true;
    return Ok(Value::NIL);
}
pub fn crossed(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let mut outside = outside.write().unwrap();
    outside.title.crossed = true;
    return Ok(Value::NIL);
}
pub fn underline(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let mut outside = outside.write().unwrap();
    outside.title.underline = true;
    return Ok(Value::NIL);
}
pub fn reset(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let mut outside = outside.write().unwrap();
    outside.title.bold = false;
    outside.title.italic = false;
    outside.title.crossed = false;
    outside.title.underline = false;
    return Ok(Value::NIL);
}
pub fn show(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let show = require_arg("title/show", &args, 0)?;
    let mut outside = outside.write().unwrap();
    outside.title.show = match show {
        Value::False => false,
        Value::True => true,
        Value::Int(n) => *n > 0,
        Value::Float(n) => *n > 0.,
        _ => false,
    };
    return Ok(Value::NIL);
}
