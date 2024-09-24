use std::{cell::RefCell, rc::Rc};

use rust_lisp::{
    default_env,
    model::{Env, Value},
};

use crate::{lisp::set::pre_set_title, sym};

pub struct Environment {
    pub context: Rc<RefCell<Env>>,
    pub data: Rc<RefCell<EnvData>>,
}

#[derive(Clone, Default)]
pub struct EnvData {
    pub title: String,
}

macro_rules! insert_func {
    ($self: expr, $lisp_name: expr, $func_name: ident) => {
        use rust_lisp::model::Symbol;
        {
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
        s = s.register_pre();
        s
    }

    pub fn register_lisp(self) -> Self {
        let ctx = self.context.clone();
        let mut ctx = ctx.borrow_mut();

        macro_rules! redefine {
            ($orig: expr, $new: expr) => {{
                let d = ctx.get(&sym!($orig)).unwrap();
                ctx.define(sym!($new), d);
                ctx.undefine(&sym!($orig));
            }};
        }

        redefine!("is_null", "is-null");
        redefine!("is_number", "is-number");
        redefine!("is_symbol", "is-symbol");
        redefine!("is_boolean", "is-boolean");
        redefine!("is_procedure", "is-procedure");
        redefine!("is_pair", "is-pair");

        redefine!("hash_get", "hash-get");
        redefine!("hast_set", "hash-set");

        self
    }

    pub fn register_pre(self) -> Environment {
        insert_func!(self, "set-title", pre_set_title);
        self
    }
}
