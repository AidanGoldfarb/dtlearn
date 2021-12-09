use crate::attribute::{Attribute};

pub struct Tree{
    pub root: Node,
}

pub struct Node{
    pub v: Attribute,
    pub children: Vec<Node>,
}
