use std::{cell::RefCell, rc::Rc};

use rust_lisp::{model::{Env, RuntimeError, Value}, utils::require_typed_arg};

use crate::environment::EnvData;


pub fn pre_set_title(_env: Rc<RefCell<Env>>, args: Vec<Value>, outside: Rc<RefCell<EnvData>>) -> Result<Value, RuntimeError> {
    let name = require_typed_arg::<&String>("set-title", &args, 0)?;
    outside.borrow_mut().title = name.to_string();
    return Ok(Value::NIL);
}