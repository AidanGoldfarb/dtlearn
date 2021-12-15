use crate::attribute::{Attribute};

pub struct Tree{
    pub root: Node,
}

pub enum Node{
    Inner(InnerNode),
    Leaf(LeafNode)
}

pub struct InnerNode{
    pub v: Attribute,
    pub children: Vec<Node>,
}

pub struct LeafNode{
    pub result: bool,
}
