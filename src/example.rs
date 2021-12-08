use crate::attribute::{*};


pub struct Example {
    pub id: usize,
    pub output: Output,
    pub attributes: Vec<Attribute>,
}

#[derive(PartialEq)]
pub enum Output{
    Yes,
    No,
}