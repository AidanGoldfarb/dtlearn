use crate::attribute::{*};

pub struct Tree{
    root: Node,
}

pub struct Node{
    v: Attribute,
    children: Vec<Node>,
}
