use std::{cell::RefCell, rc::Rc};

use rust_lisp::{
    interpreter::eval,
    model::{Env, RuntimeError, Symbol, Value},
    parser::parse,
    utils::{require_arg, require_typed_arg},
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

pub fn exit(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let mut outside = outside.write().unwrap();

    outside.quit = true;

    Ok(Value::NIL)
}

pub mod string {
    use std::{cell::RefCell, rc::Rc};

    use rust_lisp::{
        model::{Env, RuntimeError, Value},
        utils::require_typed_arg,
    };

    use crate::environment::Container;

    pub fn format(
        _env: Rc<RefCell<Env>>,
        args: Vec<Value>,
        _outside: Container,
    ) -> Result<Value, RuntimeError> {
        let format_string = require_typed_arg::<&String>("string/format", &args, 0)?;
        let mut format_string: Vec<char> = format_string.chars().collect();

        let mut iter = 0;
        let mut new_str = String::new();
        while !format_string.is_empty() {
            let ch = format_string.remove(0);
            if ch == '%' {
                if !format_string.is_empty() {
                    let format = format_string.remove(0);
                    match format {
                        '%' => {
                            iter += 1;
                            if args.len() < iter + 1 {
                                return Err(RuntimeError {
                                    msg: "Not enough arguments given for string/format".to_string(),
                                });
                            }
                            new_str.push_str(&args[iter].to_string());
                        }
                        's' => {
                            iter += 1;
                            if args.len() < iter + 1 {
                                return Err(RuntimeError {
                                    msg: "Not enough arguments given for string/format".to_string(),
                                });
                            }
                            new_str.push_str(match &args[iter] {
                                Value::String(value) => value,
                                _ => {
                                    return Err(RuntimeError {
                                        msg: "Expected string for %s format specifier".to_string(),
                                    })
                                }
                            });
                        }
                        _ => {}
                    }
                } else {
                    new_str.push(ch);
                }
            } else {
                new_str.push(ch);
            }
        }

        Ok(Value::String(new_str))
    }

    pub fn escape(
        _env: Rc<RefCell<Env>>,
        args: Vec<Value>,
        _outside: Container,
    ) -> Result<Value, RuntimeError> {
        let ch: &String = require_typed_arg("string/escape", &args, 0)?;
        let ch = match ch.as_str() {
            "n" => "\n",
            "t" => "\t",
            "r" => "\r",
            "0" => "\0",
            _ => ch.as_str(),
        };
        return Ok(Value::String(ch.to_string()));
    }
}

pub fn room_set(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let room = require_typed_arg::<&String>("room/set", &args, 0)?;
    let mut outside = outside.write().unwrap();

    outside.current_room = room.clone();

    Ok(Value::NIL)
}

pub fn room_get(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let outside = outside.read().unwrap();

    Ok(Value::String(outside.current_room.clone()))
}

// pub fn import(
//     env: Rc<RefCell<Env>>,
//     args: Vec<Value>,
//     _outside: Container,
// ) -> Result<Value, RuntimeError> {
//     let module: &Symbol = require_typed_arg("import", &args, 0)?;
//     let source = match module.0.as_str() {
//         "escape" => include_str!("./lisp_lib/escape.hh3"),
//         "math" => include_str!("./lisp_lib/math.hh3"),
//         "quick" => include_str!("./lisp_lib/quick.hh3"),
//         "std" => include_str!("./lisp_lib/std.hh3"),
//         _ => "",
//     };

//     let parsed: Result<Vec<Value>, _> = parse(source).collect();
//     let parsed = parsed.unwrap();
//     for root in parsed {
//         eval(env.clone(), &root).expect("Failed to evaluate module");
//     }

//     Ok(Value::NIL)
// }

pub fn math_pow(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    _outside: Container,
) -> Result<Value, RuntimeError> {
    let base: f32 = require_typed_arg("math/pow", &args, 0)?;
    let exponent: f32 = require_typed_arg("math/pow", &args, 1)?;

    return Ok(Value::Float(base.powf(exponent)));
}

pub fn intrinsic(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    _outside: Container,
) -> Result<Value, RuntimeError> {
    let module: &Symbol = require_typed_arg("intrinsic", &args, 0)?;
    let source = match module.0.as_str() {
        "time" => Value::Int(chrono::Utc::now().timestamp_millis()),
        _ => Value::NIL,
    };

    Ok(source)
}

pub fn to_string(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    _outside: Container,
) -> Result<Value, RuntimeError> {
    let value = require_arg("to-string", &args, 0)?;

    let stringified = match value {
        Value::False => "false".to_string(),
        Value::True => "true".to_string(),
        Value::Float(f) => f.to_string(),
        Value::Int(i) => i.to_string(),
        Value::String(s) => s.to_string(),
        _ => value.type_name().to_string(),
    };
    return Ok(Value::String(stringified));
}
