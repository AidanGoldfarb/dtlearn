pub mod attribute;
pub mod example;
pub mod tree;


use crate::example::{Example};
use crate::attribute::{*};
use crate::Attribute::*;
use crate::tree::{*};
use strum::IntoEnumIterator;

fn learn_decision_tree(examples: Vec<Example>,attributes: Vec<Attribute>,parent_examples: Vec<Example>) -> Tree{
    if examples.is_empty(){
        return plurality_value(&parent_examples);
    }
    else if unity(&examples){
        return Tree{root: Node::Leaf(LeafNode{result: examples[0].output})};
    }
    else if attributes.is_empty(){
        return plurality_value(&parent_examples);
    }
    else{
        let mut max_entropy: f32 = f32::MIN;
        let mut max_entropy_attr: Attribute = Est(vec![0,1,2]);
        for a in attributes.iter(){
            let cur_imprt = Importance(a.clone(),&examples);
            if  cur_imprt > max_entropy {
                max_entropy_attr = a.clone();
                max_entropy = cur_imprt;
            }
        }
        let tree = Tree {root: Node::Inner(InnerNode{v: max_entropy_attr.clone(),children: Vec::new()})};
        match max_entropy_attr{
            Bar(arr) | Fri(arr) | Hun(arr) | Rain(arr) | Res(arr) => (),
            Est(arr) | Pat(arr) | Price(arr) | Type(arr) | Est(arr) => (),
            _ => (),
        }
    }
    Tree{root: Node::Leaf(LeafNode{result: true})}
}

#[allow(non_snake_case)]
fn Importance(attribute: Attribute, examples: &Vec<Example>)-> f32{
    let mut p = 0;
    let mut n = 0;
    for example in examples{
        if example.output == true{
            p+=1;
        }
        else{
            n+=1;
        }
    }
    B(p as f32/(p as f32 + n as f32)) - Remainder(attribute, examples, p, n)
}

// //−(q log 2 q + (1 − q) log 2 (1 − q)) .
#[allow(non_snake_case)]
fn B(q: f32)-> f32{
    -(q*q.log2() + (1.0-q)*(1.0-q).log2())
}

#[allow(non_snake_case)]
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
        cnt = if example.output == true { cnt+1 } else { cnt-1 };
    }
    if cnt >= 0 { 
        Tree{root: Node::Leaf(LeafNode{result: true})} 
    } 
    else { 
        Tree{root: Node::Leaf(LeafNode{result: false})} 
    }
}

fn max(a: f32, b: f32) ->f32 {
    if a>=b {a} else {b}
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pretty_assertions::{assert_eq};

    #[test]
    fn entropy_fair_coin(){
        assert_eq!(B(0.5), 1.0);
    }

    // #[test]
    // fn entropy_die4(){
    //     assert_eq!(B(0.25), 2.0);

    // }

    #[test]
    fn entropy_loaded(){
        assert!((B(0.99) - 0.08).abs() < 0.001);
    }
    
}