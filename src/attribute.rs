//use crate::example::{*};
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Clone)]
pub enum Attribute {   
    Alt(bool),
    Bar(bool),
    Fri(bool),
    Hun(bool),
    Pat(usize),
    Price(usize),
    Rain(bool),
    Res(bool),
    Type(usize),
    Est(usize),
}