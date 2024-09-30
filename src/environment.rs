use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, RwLock},
};

use ratatui::{
    style::Stylize,
    text::{Line, Span, Text},
    widgets::ListState,
};
use rust_lisp::{
    default_env,
    interpreter::eval,
    model::{Env, Symbol, Value},
    parser::parse,
};

use crate::{
    lisp::{self, color::Color},
    project::{Project, Room},
};

pub struct Environment {
    pub context: Rc<RefCell<Env>>,
    pub data: Container,
    tick_passed: i64,
    pub prev_time: i64,
    pub prev_room: String,
}

pub type Container = Arc<RwLock<EnvData>>;

impl Environment {
    pub fn update(&mut self) {
        let now = chrono::offset::Utc::now().timestamp_millis();
        let dt = now - self.prev_time;
        let mut content_ticks = 0;

        let cur_room = {
            let read: std::sync::RwLockReadGuard<'_, EnvData> = self.data.read().unwrap();
            read.current_room.clone()
        };
        if self.prev_room != cur_room {
            self.load_room(&cur_room);
            self.prev_room = cur_room;
        }

        {
            let data = Arc::clone(&self.data);
            let data = data.write().unwrap();

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

    fn load_room(&mut self, room: &String) {
        {
            let data = Arc::clone(&self.data);
            let mut data = data.write().unwrap();
            data.current_room = room.clone();
            data.display.content.0 = vec![];
            data.display.displayed_index = 0;
            data.options.options = vec![];
            data.options.selected = ListState::default();
        }

        let room_data = self.current_room();
        for root in parse(&room_data.pre) {
            eval(
                Rc::clone(&self.context),
                &root.expect("Failed to parse lisp"),
            )
            .expect(&format!(
                "Failed to evaluate lisp block in pre section of room {}",
                room
            ));
        }
    }

    fn current_room(&self) -> Room {
        let data = Arc::clone(&self.data);
        let data = data.read().unwrap();
        let room_data = data
            .project
            .rooms
            .get(&data.current_room)
            .expect(&format!("Couldn't find the room {:?}", data.current_room));
        room_data.clone()
    }

    fn tick_content(&mut self) {
        let this_room = self.current_room();
        let data_arc = Arc::clone(&self.data);
        let (fg, bg, bold, italic, crossed, underline, too_far) = {
            let data = data_arc.read().unwrap();
            let too_far = data.display.displayed_index < this_room.content.len();
            (
                data.display.current_fg,
                data.display.current_bg,
                data.display.bold,
                data.display.italic,
                data.display.crossed,
                data.display.underline,
                too_far,
            )
        };

        if too_far {
            let value = {
                let data = data_arc.read().unwrap();
                &this_room.content[data.display.displayed_index]
            };
            match value {
                crate::project::Content::Char(c) => {
                    let new = ContentChar {
                        bg,
                        fg,
                        bold,
                        italic,
                        underline,
                        crossed,
                        ch: *c,
                    };
                    let mut data = data_arc.write().unwrap();
                    data.display.content.0.push(new);
                }
                crate::project::Content::Lisp(lisp) => {
                    for root in parse(&lisp) {
                        eval(
                            Rc::clone(&self.context),
                            &root.expect("Failed to parse lisp"),
                        )
                        .expect("Failed to evaluate lisp");
                    }
                }
            }
        }

        {
            let mut data = data_arc.write().unwrap();

            if data.display.displayed_index < this_room.content.len() {
                data.display.displayed_index += 1;
            }
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct EnvData {
    pub title: TitleData,
    pub options: OptionData,
    pub current_room: String,
    pub listeners: ListenerData,
    pub display: DisplayData,
    pub project: Project,
    pub debug: Vec<String>,
    pub quit: bool,
}

#[derive(Clone, Default, Debug)]
pub struct ListenerData {
    pub keyboard_char: HashMap<i32, Value>,
}
#[derive(Clone, Default, Debug)]
pub struct TitleData {
    pub content: String,
    pub show: bool,
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub crossed: bool,
}
#[derive(Clone, Default, Debug)]
pub struct DisplayData {
    pub content: Content,
    pub delay: i64,
    pub displayed_index: usize,
    pub current_fg: Color,
    pub current_bg: Color,
    pub display_fg: Color,
    pub display_bg: Color,
    pub display_ac: Color,
    pub bold: bool,
    pub italic: bool,
    pub crossed: bool,
    pub underline: bool,
    pub scroll: i32,
}

impl DisplayData {
    pub fn to_content_char(&self, ch: char) -> ContentChar {
        ContentChar {
            bg: self.current_bg,
            fg: self.current_fg,
            bold: self.bold,
            italic: self.italic,
            crossed: self.crossed,
            underline: self.underline,
            ch,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct OptionData {
    pub options: Vec<OptionDataSingle>,
    pub selected: ListState,
}
#[derive(Clone, Debug)]
pub struct OptionDataSingle {
    pub name: Content,
    pub action: Value,
}

#[derive(Clone, Debug)]
pub struct ContentChar {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub crossed: bool,
}
impl Default for ContentChar {
    fn default() -> Self {
        Self {
            ch: ' ',
            bold: false,
            fg: Color::default(),
            bg: Color::default(),
            crossed: false,
            italic: false,
            underline: false,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Content(pub Vec<ContentChar>);
impl Content {
    pub fn to_spans(&self) -> Vec<Vec<Span>> {
        let mut lines = vec![];
        let mut cur = vec![];
        for ch in &self.0 {
            if ch.ch != '\n' {
                let mut value = Span::from(ch.ch.to_string())
                    .fg(ch.fg.to_ratatui_color())
                    .bg(ch.bg.to_ratatui_color());
                if ch.bold {
                    value = value.bold()
                };
                if ch.italic {
                    value = value.italic()
                };
                if ch.crossed {
                    value = value.crossed_out()
                };
                if ch.underline {
                    value = value.underlined()
                };
                cur.push(value);
            } else {
                lines.push(cur);
                cur = vec![];
            }
        }
        if !cur.is_empty() {
            lines.push(cur);
        }
        return lines;
    }
    pub fn to_line(&self) -> Vec<Line> {
        self.to_spans().into_iter().map(|v| Line::from(v)).collect()
    }
    pub fn to_text(&self) -> Text {
        Text::from(self.to_line())
    }
    pub fn to_raw(&self) -> String {
        let chars = self.0.iter().map(|v| v.ch).collect();
        chars
    }
}

macro_rules! insert_func {
    ($self: expr, $lisp_name: expr, $func_name: ident) => {{
        use rust_lisp::model::Symbol;
        let data = Arc::clone(&$self.data);
        $self.context.borrow_mut().define(
            Symbol::from($lisp_name),
            Value::NativeClosure(Rc::new(RefCell::new(move |env, args| {
                let d = Arc::clone(&data);
                $func_name(Rc::clone(&env), args, d)
            }))),
        );
    }};
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            context: Rc::new(RefCell::new(default_env())),
            data: Arc::new(RwLock::new(EnvData {
                ..Default::default()
            })),
            prev_time: chrono::offset::Utc::now().timestamp_millis(),
            tick_passed: 0,
            prev_room: "".to_string(),
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

        redefine!("hash_get", "hash/get");
        redefine!("hash_set", "hash/set");

        ctx.define(Symbol::from("true"), Value::True);
        ctx.define(Symbol::from("false"), Value::False);
        ctx.define(Symbol::from("dq"), Value::String("\"".to_string()));

        self
    }

    pub fn register_pre(self) -> Environment {
        {
            use lisp::title::*;
            insert_func!(self, "title/name/set", set_name);
            insert_func!(self, "title/fg/set", set_fg);
            insert_func!(self, "title/bg/set", set_bg);
            insert_func!(self, "title/bold", bold);
            insert_func!(self, "title/italic", italic);
            insert_func!(self, "title/crossed", crossed);
            insert_func!(self, "title/underline", underline);
            insert_func!(self, "title/reset", reset);
            insert_func!(self, "title/show", show);
        }
        {
            use lisp::color::*;
            insert_func!(self, "color/new", color_new);
            insert_func!(self, "color", color);
        }
        {
            use lisp::content::*;
            insert_func!(self, "delay/set", set_delay);
            insert_func!(self, "fg/set", set_content_fg);
            insert_func!(self, "bg/set", set_content_bg);
            insert_func!(self, "bold", bold);
            insert_func!(self, "italic", italic);
            insert_func!(self, "crossed", crossed);
            insert_func!(self, "underline", underline);
            insert_func!(self, "reset", reset);

            insert_func!(self, "display/fg/set", set_fg);
            insert_func!(self, "display/bg/set", set_bg);
            insert_func!(self, "display/ac/set", set_ac);
            insert_func!(self, "display/fg/get", get_fg);
            insert_func!(self, "display/bg/get", get_bg);
            insert_func!(self, "display/ac/get", get_ac);

            insert_func!(self, "content/clear", content_clear);
            insert_func!(self, "content/get-raw", content_get_raw);
            insert_func!(self, "content/append", content_append);

            insert_func!(self, "content/scroll/down", content_scroll_down);
            insert_func!(self, "content/scroll/up", content_scroll_up);
        }
        {
            use lisp::option::*;
            insert_func!(self, "option/goto", option_goto);
            insert_func!(self, "option/action", option_action);
            insert_func!(self, "option/reset", option_reset);
        }
        {
            use lisp::basic::*;
            insert_func!(self, "post", run_post);
            insert_func!(self, "debug", debug);
            insert_func!(self, "exit", exit);
            {
                use lisp::basic::string::*;
                insert_func!(self, "string/format", format);
            }
        }
        {
            use lisp::listener::*;
            insert_func!(self, "listener/keyboard/char", keyboard_char_listener);
            insert_func!(self, "listener/clear", listener_clear);
        }
        self
    }
}
