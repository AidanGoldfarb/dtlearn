//use crate::example::{*};
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Clone)]
pub enum Attribute {
    OutputYes,
    OutputNo,   
    Alt,
    Bar,
    Fri,
    Hun,
    Pat,
    Price,
    Rain,
    Res,
    Type,
    Est,
}

#[derive(EnumIter, Debug)]
pub enum Alt {
    Yes,
    No,
}

#[derive(EnumIter, Debug)]
pub enum Bar {
    Yes,
    No,
}

#[derive(EnumIter, Debug)]
pub enum Fri {
    Yes,
    No,
}
#[derive(EnumIter, Debug)]
pub enum Hun {
    Yes,
    No,
}
#[derive(EnumIter, Debug)]
pub enum Pat {
    None,
    Some,
    Full,
}
#[derive(EnumIter, Debug)]
pub enum Price {
    Low,
    Mid,
    High,
}
#[derive(EnumIter, Debug)]
pub enum Rain {
    Yes,
    No,
}
#[derive(EnumIter, Debug)]
pub enum Res {
    Yes,
    No,
}
#[derive(EnumIter, Debug)]
pub enum Type {
    French,
    Burger,
    Thai,
    Italian,
}
#[derive(EnumIter, Debug)]
pub enum Est {
    Short,
    Medium,
    Long,
}