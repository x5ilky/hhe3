use std::{cell::RefCell, rc::Rc};

use rand::Rng;
use rust_lisp::{
    model::{Env, IntType, RuntimeError, Value},
    utils::require_typed_arg,
};

use crate::environment::Container;

pub fn random_int(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    _outside: Container,
) -> Result<Value, RuntimeError> {
    let a: IntType = require_typed_arg("random/int", &args, 0)?;
    let b: IntType = require_typed_arg("random/int", &args, 1)?;

    return Ok(Value::Int(rand::thread_rng().gen_range(a..b)));
}
