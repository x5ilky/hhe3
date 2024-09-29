use std::{cell::RefCell, rc::Rc, sync::Arc};

use rust_lisp::{
    interpreter::eval,
    model::{Env, RuntimeError, Symbol, Value},
    parser::parse,
    utils::require_typed_arg,
};

use crate::environment::Container;

pub fn run_post(
    env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside_arc: Container,
) -> Result<Value, RuntimeError> {
    let room = {
        let outside = outside_arc.read().unwrap();
        let room = outside
            .project
            .rooms
            .get(&outside.current_room)
            .expect("Couldn't find room for current_room somehow?")
            .clone();
        room
    };
    let parsed = parse(&room.post);
    for root in parsed {
        eval(
            Rc::clone(&env),
            &root.expect("Failed to parse lua in post section"),
        )
        .expect("Failed to evaluate post section");
    }
    Ok(Value::NIL)
}
pub fn debug(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let arg = require_typed_arg::<&String>("debug", &args, 0)?;
    let mut outside = outside.write().unwrap();
    println!("{:?}", outside);

    outside.debug.push(arg.to_owned());

    Ok(Value::NIL)
}
