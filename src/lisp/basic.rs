use std::{cell::RefCell, rc::Rc};

use rust_lisp::{interpreter::eval, model::{Env, RuntimeError, Value}, parser::parse};

use crate::environment::Container;


pub fn run_post(_env: Rc<RefCell<Env>>, _args: Vec<Value>, outside: Container) -> Result<Value, RuntimeError> {
    let outside = outside.read().unwrap();
    let room = outside.project.rooms.get(&outside.current_room).expect("Couldn't find room for current_room somehow?");
    let parsed = parse(&room.post);
    for root in parsed {
        eval(Rc::clone(&_env), &root.expect("Failed to parse lua in post section")).expect("Failed to evaluate lua snippet");
    }
    Ok(Value::NIL)
}