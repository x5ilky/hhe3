use std::{cell::RefCell, rc::Rc};

use rust_lisp::{model::{Env, RuntimeError, Symbol, Value}, utils::{require_typed_arg, TypeName}};

use crate::environment::EnvData;


#[derive(Debug, Clone, Default, Copy)]
pub struct Color (u8, u8, u8);

impl TypeName for Color {
    fn get_name() -> &'static str {
        "Color"
    }
}

impl From<&Value> for Color {
    fn from(value: &Value) -> Self {
        match value {
            Value::Foreign(r) => {
                if r.is::<Color>() {
                    return *r.clone().downcast::<Color>().unwrap()
                }
                panic!("Not Color");
            }
            _ => panic!("Value isn't color! Expected Color, found {}", value.type_name())
        }
    }
}

impl Color {
    pub fn to_ratatui_color(self) -> ratatui::style::Color {
        ratatui::style::Color::Rgb(self.0, self.1, self.2)
    }
}

pub fn color_new(_env: Rc<RefCell<Env>>, args: Vec<Value>, _outside: Rc<RefCell<EnvData>>) -> Result<Value, RuntimeError> {
    let r = require_typed_arg::<i32>("color-new", &args, 0)?;
    let g = require_typed_arg::<i32>("color-new", &args, 1)?;
    let b = require_typed_arg::<i32>("color-new", &args, 2)?;

    let r: u8 = match u8::try_from(r) {
        Ok(v) => v,
        Err(e) => return Err(RuntimeError {
            msg: e.to_string(),
        })
    };
    let g: u8 = match u8::try_from(g) {
        Ok(v) => v,
        Err(e) => return Err(RuntimeError {
            msg: e.to_string(),
        })
    };
    let b: u8 = match u8::try_from(b) {
        Ok(v) => v,
        Err(e) => return Err(RuntimeError {
            msg: e.to_string(),
        })
    };

    return Ok(Value::Foreign(Rc::new(Color (r, g, b))))
}
pub fn color(_env: Rc<RefCell<Env>>, args: Vec<Value>, _outside: Rc<RefCell<EnvData>>) -> Result<Value, RuntimeError> {
    let color = require_typed_arg::<&Symbol>("color", &args, 0)?;

    let color = match color.0.as_str() {
        "red" => Color(255, 0, 0),
        "green" => Color(0, 0, 255),
        "blue" => Color(0, 255, 0),
        "black" => Color(0, 0, 0),
        "white" => Color(255, 255, 255),
        _ => return Err(RuntimeError { msg: format!("No color called: {}", color.0) })
    };

    return Ok(Value::Foreign(Rc::new(color)));
}
