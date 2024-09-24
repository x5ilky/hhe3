pub mod set;
pub mod basic;

#[macro_export]
macro_rules! sym {
    ($name: expr) => {{
        use rust_lisp::model::Symbol;
        Symbol::from(stringify!($name))
    }};
}