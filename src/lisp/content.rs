use std::{cell::RefCell, rc::Rc, sync::Arc};

use rust_lisp::{
    model::{Env, IntType, RuntimeError, Value},
    utils::require_typed_arg,
};

use crate::environment::{Container, ContentChar};

use super::color::Color;

pub fn set_delay(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let delay = require_typed_arg::<IntType>("delay/set", &args, 0)?;
    let mut outside = outside.write().unwrap();
    outside.display.delay = delay as i64;
    return Ok(Value::NIL);
}

pub fn set_content_fg(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let color = require_typed_arg::<Color>("fg/set", &args, 0)?;
    let mut outside = outside.write().unwrap();
    outside.display.current_fg = color;
    return Ok(Value::NIL);
}
pub fn set_content_bg(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let color = require_typed_arg::<Color>("bg/set", &args, 0)?;
    let mut outside = outside.write().unwrap();
    outside.display.current_bg = color;
    return Ok(Value::NIL);
}
pub fn set_fg(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let color = require_typed_arg::<Color>("display/fg/set", &args, 0)?;
    let mut outside = outside.write().unwrap();
    outside.display.display_fg = color;
    return Ok(Value::NIL);
}
pub fn get_fg(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let outside = outside.read().unwrap();
    return Ok(Value::Foreign(Rc::new(outside.display.display_fg)));
}
pub fn set_bg(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let color = require_typed_arg::<Color>("display/bg/set", &args, 0)?;
    let mut outside = outside.write().unwrap();
    outside.display.display_bg = color;
    return Ok(Value::NIL);
}
pub fn get_bg(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let outside = outside.read().unwrap();
    return Ok(Value::Foreign(Rc::new(outside.display.display_bg)));
}
pub fn set_ac(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let color = require_typed_arg::<Color>("display/ac/set", &args, 0)?;
    let mut outside = outside.write().unwrap();
    outside.display.display_ac = color;
    return Ok(Value::NIL);
}
pub fn get_ac(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let outside = outside.read().unwrap();
    return Ok(Value::Foreign(Rc::new(outside.display.display_ac)));
}
pub fn bold(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let mut outside = outside.write().unwrap();
    outside.display.bold = true;
    return Ok(Value::NIL);
}
pub fn italic(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let mut outside = outside.write().unwrap();
    outside.display.italic = true;
    return Ok(Value::NIL);
}
pub fn crossed(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let mut outside = outside.write().unwrap();
    outside.display.crossed = true;
    return Ok(Value::NIL);
}
pub fn underline(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let mut outside = outside.write().unwrap();
    outside.display.underline = true;
    return Ok(Value::NIL);
}
pub fn reset(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let mut outside = outside.write().unwrap();
    outside.display.bold = false;
    outside.display.italic = false;
    outside.display.crossed = false;
    outside.display.underline = false;
    outside.display.current_fg = Color::default();
    outside.display.current_bg = Color::default();
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

pub fn content_get_raw(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let outside = Arc::clone(&outside);
    let read = outside.read().unwrap();

    Ok(Value::String(read.display.content.to_raw()))
}

pub fn content_append(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let string = require_typed_arg::<&String>("content/append", &args, 0)?;
    let outside = Arc::clone(&outside);
    let mut write = outside.write().unwrap();
    let to_content: Vec<ContentChar> = string
        .chars()
        .map(|v| write.display.to_content_char(v))
        .collect();
    write.display.content.0.extend(to_content);

    Ok(Value::NIL)
}

pub fn content_scroll_down(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let outside = Arc::clone(&outside);
    let mut write = outside.write().unwrap();
    write.display.scroll += 1;
    Ok(Value::NIL)
}

pub fn content_scroll_up(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let outside = Arc::clone(&outside);
    let mut write = outside.write().unwrap();
    write.display.scroll -= 1;
    if write.display.scroll < 0 {
        write.display.scroll = 0;
    }
    Ok(Value::NIL)
}

pub fn content_scroll_set(
    _env: Rc<RefCell<Env>>,
    args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let mut amount = require_typed_arg::<IntType>("content/scroll/set", &args, 0)?;
    let outside = Arc::clone(&outside);
    let mut write = outside.write().unwrap();
    if amount < 0 {
        amount = 0;
    }
    write.display.scroll = amount as i32;
    Ok(Value::NIL)
}

pub fn content_scroll_get(
    _env: Rc<RefCell<Env>>,
    _args: Vec<Value>,
    outside: Container,
) -> Result<Value, RuntimeError> {
    let outside = Arc::clone(&outside);
    let read = outside.read().unwrap();
    Ok(Value::Int(read.display.scroll as IntType))
}
