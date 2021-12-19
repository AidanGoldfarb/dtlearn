//use crate::example::{*};
// use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Clone, Hash, Eq, Default)]
pub struct Attribute{
    pub name: String,
    pub value: usize,
    pub values: Vec<usize>,
}


// #[derive(EnumIter, Debug, PartialEq, Clone)]
// pub enum Attribute {   
//     Alt(Vec<bool>),
//     Bar(Vec<bool>),
//     Fri(Vec<bool>),
//     Hun(Vec<bool>),
//     Pat(Vec<usize>),
//     Price(Vec<usize>),
//     Rain(Vec<bool>),
//     Res(Vec<bool>),
//     Type(Vec<usize>),
//     Est(Vec<usize>),
// }