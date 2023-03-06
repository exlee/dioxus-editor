use std::fmt::Display;
use dioxus::prelude::*;

pub enum Direction {
    Forward = 1,
    Backward = -1,
}

#[derive(Debug)]
pub enum Glyph
{
    Char(char),
    Cursor,
    HTMLNode(String),
}



impl Display for Glyph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Glyph::Char(c) => write!(f,"{}", c),
            Glyph::Cursor => write!(f,"▫️"),
            HTMLNode => write!(f, "HTMLNode"),

        }
    }
}
