use std::{cell::RefCell, rc::Rc};

use ratatui::{
    style::Stylize,
    text::{Line, Span, Text},
    widgets::ListState,
};
use rust_lisp::{
    default_env, interpreter::eval, model::{Env, Symbol, Value}, parser::parse
};

use crate::{lisp::{self, color::Color}, project::{Project, Room}};

pub struct Environment {
    pub context: Rc<RefCell<Env>>,
    pub data: Rc<RefCell<EnvData>>,
    tick_passed: i64,
    pub prev_time: i64,
}

impl Environment {
    pub fn update(&mut self) {
        let now = chrono::offset::Utc::now().timestamp_millis();
        let dt = now - self.prev_time;
        let mut content_ticks = 0;

        {
            let data = Rc::clone(&self.data);
            let data = data.borrow();

            if data.display.delay != 0 {
                self.tick_passed += dt;
                content_ticks = self.tick_passed / data.display.delay;
                self.tick_passed %= data.display.delay;
                
            };
        }
        for _ in 0..content_ticks {
            self.tick_content();
        }

        self.prev_time = chrono::offset::Utc::now().timestamp_millis();
    }

    pub fn load_room(&mut self, room: &String) {
        {
            let data = Rc::clone(&self.data);
            let mut data = data.borrow_mut();
            data.current_room = room.clone();
        }
        let room_data = self.current_room();
        for root in parse(&room_data.pre) {
            eval(Rc::clone(&self.context), &root.expect("Failed to parse lisp")).expect(&format!("Failed to evaluate lisp block in pre section of room {}", room));
        }
    }

    fn current_room(&self) -> Room {
        let data = Rc::clone(&self.data);
        let data = data.borrow();
        let room_data = data.project.rooms.get(&data.current_room).expect(&format!("Couldn't find the room {:?}", data.current_room));
        room_data.clone()
    }

    fn tick_content(&mut self) {
        let this_room = self.current_room();
        let data = Rc::clone(&self.data);
        let mut data = data.borrow_mut();

        if data.display.displayed_index < this_room.content.len() - 1 {
            data.display.displayed_index += 1;
        }
        match &this_room.content[data.display.displayed_index] {
            crate::project::Content::Char(c) => {
                let new = (*c, data.display.current_color.clone());
                data.display.content.0.push(new);
            }
            crate::project::Content::Lisp(lisp) => {
                for root in parse(&lisp) {
                    eval(Rc::clone(&self.context), &root.expect("Failed to parse lisp")).expect("Failed to evaluate lisp");
                }
            }
        }
        data.display.content =
            Content(data.display.content.0[0..data.display.displayed_index].to_vec());
        let d = format!("{}", data.display.delay);
        data.debug.push(d);
    }
}

#[derive(Clone, Default)]
pub struct EnvData {
    pub title: TitleData,
    pub options: OptionData,
    pub current_room: String,
    pub display: DisplayData,
    pub project: Project,
    pub debug: Vec<String>,
    pub quit: bool,
}

#[derive(Clone, Default)]
pub struct TitleData {
    pub content: String,
    pub show: bool,
    pub color: Color,
}
#[derive(Clone, Default)]
pub struct DisplayData {
    pub content: Content,
    pub delay: i64,
    pub displayed_index: usize,
    pub current_color: Color,
}
#[derive(Clone, Default)]
pub struct OptionData {
    pub options: Vec<OptionDataSingle>,
    pub selected: ListState,
}
#[derive(Clone)]
pub struct OptionDataSingle {
    pub name: Content,
    pub action: Value,
}

#[derive(Clone, Default)]
pub struct Content(pub Vec<(char, Color)>);
impl Content {
    pub fn to_spans(&self) -> Vec<Span> {
        self.0
            .iter()
            .map(|v| Span::from(v.0.to_string()).fg(v.1.to_ratatui_color()))
            .collect()
    }
    pub fn to_line(&self) -> Line {
        Line::from(self.to_spans())
    }
    pub fn to_text(&self) -> Text {
        Text::from(vec![self.to_line()])
    }
}

macro_rules! insert_func {
    ($self: expr, $lisp_name: expr, $func_name: ident) => {{
        use rust_lisp::model::Symbol;
        let data = Rc::clone(&$self.data);
        $self.context.borrow_mut().define(
            Symbol::from($lisp_name),
            Value::NativeClosure(Rc::new(RefCell::new(move |env, args| {
                let d = Rc::clone(&data);
                $func_name(env, args, d)
            }))),
        );
    }};
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            context: Rc::new(RefCell::new(default_env())),
            data: Rc::new(RefCell::new(EnvData {
                ..Default::default()
            })),
            prev_time: chrono::offset::Utc::now().timestamp_millis(),
            tick_passed: 0,
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
        {
            use lisp::content::*;
            insert_func!(self, "delay-set", set_delay);
        }
        self
    }
}
