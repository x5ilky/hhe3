use std::collections::HashMap;

use crate::parser::Metadata;


#[derive(Clone, Debug)]
pub struct Project {
    pub rooms: HashMap<String, Room>,
    pub author: String,
    pub name: String,
    pub meta: Metadata
}

#[derive(Clone, Debug)]
pub struct Room {
    pub pre: String,
    pub post: String,
    pub content: Vec<Content>,
}

#[derive(Clone, Debug)]
pub enum Content {
    Text(String),
    Lisp(String),
}