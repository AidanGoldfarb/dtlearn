mod attribute;
mod example;
mod tree;

use attribute::{*};
use example::{*};
use tree::{*};

fn main() {
    println!("Hello, world!");
}


fn learn_decision_tree(examples: Vec<Example>,attributes: Vec<Attribute>,parent_examples: Vec<Example>) -> Tree{
    if examples.is_empty(){
        plurality_value(&parent_examples);
    }
    else if unity(&examples){
        Tree{root: Node{v: example[0].output, children: Vec::new()}}
    }
    
    
    Tree {root: Node{v: Attribute::Est,children: Vec::new()}}
}

fn unity(examples: &Vec<Example>) -> bool{
    let litmus = examples[0].output;
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
        cnt = if example.output == Output::Yes { cnt+1 } else { cnt-1 };
    }
    if cnt >= 0 { 
        Tree{root: Node{v: Attribute::OutputYes, children: Vec::new()}} 
    } 
    else { 
        Tree{root: Node{v: Attribute::OutputNo, children: Vec::new()}}
    }
}