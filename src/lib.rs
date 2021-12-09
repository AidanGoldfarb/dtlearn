pub mod attribute;
pub mod example;
pub mod tree;

use crate::example::{Example};
use crate::attribute::{Attribute};
use crate::tree::{Tree, Node};
use strum::{IntoEnumIterator};

fn learn_decision_tree(examples: Vec<Example>,attributes: Vec<Attribute>,parent_examples: Vec<Example>) -> Tree{
    if examples.is_empty(){
        return plurality_value(&parent_examples);
    }
    else if unity(&examples){
        return Tree{root: Node{v: examples[0].output.clone(), children: Vec::new()}};
    }
    else if attributes.is_empty(){
        return plurality_value(&parent_examples);
    }
    else{
        let max_entropy: f32 = f32::MIN;
        for a in attributes.iter(){
            //Tree {root: Node{v: Attribute::Est,children: Vec::new()}}
            todo!();
        }
    }
    Tree {root: Node{v: Attribute::Est,children: Vec::new()}}
}

// //−(q log 2 q + (1 − q) log 2 (1 − q)) .
fn B(q: f32)-> f32{
    -(q*q.log2() + (1.0-q)*(1.0-q).log2())
}

fn Remainder(attribute: Attribute, examples: &Vec<Example>, p: usize, n: usize) -> f32{
    let k = 1;
    let mut sum = 0.0;
    for value in Attribute::iter(){
        let mut pk: usize = 0; 
        let mut nk: usize = 0;
        for example in examples{
            for attr in example.attributes.iter(){
                if *attr == value{
                    pk += 1;
                }
            }
        }
        nk = examples.len() - pk;

        sum += (pk as f32 + nk as f32)/(p as f32+n as f32) * B(pk as f32/(pk as f32 +nk as f32))
    }
    sum
}  


fn unity(examples: &Vec<Example>) -> bool{
    let litmus = examples[0].output.clone();
    for example in examples.iter(){
        if example.output != litmus{
            return false
        }
    }
    return true;
}

fn plurality_value(parent_examples: &Vec<Example>) -> Tree{
    let mut cnt = 0;
    for example in parent_examples.iter(){
        cnt = if example.output == Attribute::OutputYes { cnt+1 } else { cnt-1 };
    }
    if cnt >= 0 { 
        Tree{root: Node{v: Attribute::OutputYes, children: Vec::new()}} 
    } 
    else { 
        Tree{root: Node{v: Attribute::OutputNo, children: Vec::new()}}
    }
}