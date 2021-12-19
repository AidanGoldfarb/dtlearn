use crate::attribute::{Attribute};

#[derive(Default, PartialEq, Debug)]
pub struct Node{
    pub result: bool,
    pub test: Attribute,
    pub children: Vec<Node>,
}

// pub struct Tree{
//     pub root: Node,
// }

// pub enum Node{
//     Inner(InnerNode),
//     Leaf(LeafNode)
// }

// pub struct InnerNode{
//     pub v: Attribute,
//     pub children: Vec<Node>,
// }

// pub struct LeafNode{
//     pub result: bool,
// }
