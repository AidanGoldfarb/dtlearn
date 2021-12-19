use crate::attribute::{Attribute};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct Example {
    pub id: usize,
    pub output: bool,
    pub attributes: Vec<Attribute>,
    pub amap: HashMap<Attribute,usize>,
}