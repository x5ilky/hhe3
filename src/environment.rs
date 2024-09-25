use std::{cell::RefCell, rc::Rc};

use rust_lisp::{
    default_env,
    model::{Env, Symbol, Value},
};

use crate::lisp::{self, color::Color};

pub struct Environment {
    pub context: Rc<RefCell<Env>>,
    pub data: Rc<RefCell<EnvData>>,
}

#[derive(Clone, Default)]
pub struct EnvData {
    pub title: TitleData, 
    pub current_room: String,
    pub display: DisplayData,
}

#[derive(Clone, Default)]
pub struct TitleData {
    pub content: String,
    pub show: bool,
    pub color: Color
}
#[derive(Clone, Default)]
pub struct DisplayData {
    pub content: Content,
    pub delay: i64,
}

pub type Content = Vec<(char, Color)>;

macro_rules! insert_func {
    ($self: expr, $lisp_name: expr, $func_name: ident) => {
        {
            use rust_lisp::model::Symbol;
            let data = Rc::clone(&$self.data);
            $self.context.borrow_mut().define(
                Symbol::from($lisp_name),
                Value::NativeClosure(Rc::new(RefCell::new(move |env, args| {
                    let d = Rc::clone(&data);
                    $func_name(env, args, d)
                }))),
            );
        }
    };
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            context: Rc::new(RefCell::new(default_env())),
            data: Rc::new(RefCell::new(EnvData {
                ..Default::default()
            })),
        }
    }

    pub fn register_all(self) -> Environment {
        let mut s = self;
        s = s.register_lisp();
        s = s.register_pre();
        s
    }

    pub fn register_lisp(self) -> Self {
        let ctx = Rc::clone(&self.context);
        let mut ctx = ctx.borrow_mut();

        macro_rules! redefine {
            ($orig: expr, $new: expr) => {{
                let d = ctx.get(&Symbol::from($orig)).unwrap();
                ctx.define(Symbol::from($new), d);
                ctx.undefine(&Symbol::from($orig));
            }};
        }

        ctx.undefine(&Symbol::from("print"));

        redefine!("is_null", "is-null");
        redefine!("is_number", "is-number");
        redefine!("is_symbol", "is-symbol");
        redefine!("is_boolean", "is-boolean");
        redefine!("is_procedure", "is-procedure");
        redefine!("is_pair", "is-pair");

        redefine!("hash_get", "hash-get");
        redefine!("hash_set", "hash-set");

        ctx.define(Symbol::from("true"), Value::True);
        ctx.define(Symbol::from("false"), Value::False);


        self
    }

    pub fn register_pre(self) -> Environment {
        {
            use lisp::title::*;
            insert_func!(self, "title-set-name", set_name);
            insert_func!(self, "title-set-color", set_color);
            insert_func!(self, "title-show", show);
        }
        {
            use lisp::color::*;
            insert_func!(self, "color-new", color_new);
            insert_func!(self, "color", color);
        }
        self
    }
}
