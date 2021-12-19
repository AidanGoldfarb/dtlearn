pub mod attribute;
pub mod example;
pub mod tree;


use crate::example::{Example};
use crate::attribute::{*};
use crate::tree::{*};
//use strum::IntoEnumIterator;

fn learn_decision_tree(examples: Vec<Example>,attributes: Vec<Attribute>,parent_examples: Vec<Example>) -> Node{
    if examples.is_empty(){
        return plurality_value(&parent_examples);
    }
    else if unity(&examples){
        return Node{result: examples[0].output, ..Default::default()};
        //return Tree{root: Node::Leaf(LeafNode{result: examples[0].output})};
    }
    else if attributes.is_empty(){
        return plurality_value(&parent_examples);
    }
    else{
        let mut max_entropy: f32 = f32::MIN;
        let mut max_entropy_attr = Attribute{name: String::from(""), value: 0, values: Vec::new()};
        for a in attributes.iter(){
            let cur_imprt = Importance(a.clone(),&examples);
            if  cur_imprt > max_entropy {
                max_entropy_attr = a.clone();
                max_entropy = cur_imprt;
            }
        }
        let mut tree = Node{test: max_entropy_attr.clone(), ..Default::default()};
        //let tree = Tree {root: Node::Inner(InnerNode{v: max_entropy_attr.clone(),children: Vec::new()})};
        
        for v in max_entropy_attr.values.iter(){
            let mut exs = Vec::new();
            for e in examples.iter(){
                if e.amap.get(&max_entropy_attr).unwrap() == v{ //make it a hashsmap of attributes to values
                    exs.push(e.clone());
                }
            }
            let subtree = learn_decision_tree(exs,remove(max_entropy_attr.clone(),&attributes),examples.clone());
            tree.children.push(subtree);
        }
        tree
    }
}

fn remove(a: Attribute, attributes: &Vec<Attribute>) -> Vec<Attribute>{
    let mut res = Vec::new();
    for attr in attributes.iter(){
        if a != *attr{
            res.push(attr.clone());
        }
    }
    res
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
    for value in attribute.values.iter(){
        let mut pk: usize = 0; 
        let mut nk: usize = 0;
        for example in examples{
            for attr in example.attributes.iter(){
                if attr.value == *value{
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

fn plurality_value(parent_examples: &Vec<Example>) -> Node{
    let mut cnt = 0;
    for example in parent_examples.iter(){
        cnt = if example.output == true { cnt+1 } else { cnt-1 };
    }
    if cnt >= 0 { 
        Node{result: true, ..Default::default()}
        //Tree{root: Node::Leaf(LeafNode{result: true})} 
    } 
    else { 
        Node{result: false, ..Default::default()}
        //Tree{root: Node::Leaf(LeafNode{result: false})} 
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
    fn test_unity(){
        let e0 = Example{id: 0, output: true, ..Default::default()};
        let e1 = Example{id: 1, output: true, ..Default::default()};
        let e2 = Example{id: 2, output: false, ..Default::default()};
        let e3 = Example{id: 3, output: true, ..Default::default()};
        let v = vec![e0.clone(),e1.clone(),e2.clone(),e3.clone()];
        assert_eq!(unity(&v), false);
        let v = vec![e0.clone(),e1.clone(),e3.clone()];
        assert_eq!(unity(&v), true);
    }

    #[test]
    fn test_plurality(){
        let e0 = Example{id: 0, output: true, ..Default::default()};
        let e1 = Example{id: 1, output: true, ..Default::default()};
        let e2 = Example{id: 2, output: false, ..Default::default()};
        let e3 = Example{id: 3, output: true, ..Default::default()};
        let v = vec![e0,e1,e2,e3];
        let gt = Node{result: true, ..Default::default()};
        assert_eq!(plurality_value(&v), gt);
    }

    #[test]
    fn test_remove(){
        let a0 = Attribute{name: String::from("a0"), value: 1, values: vec![0,1,2]};
        let a1 = Attribute{name: String::from("a1"), value: 0, values: vec![0,1]};
        let a2 = Attribute{name: String::from("a2"), value: 3, values: vec![0,1,2,3]};
        let a3 = Attribute{name: String::from("a3"), value: 0, values: vec![0,1]};
        let v = vec![a0.clone(),a1.clone(),a2.clone(),a3.clone()];
        let res = remove(a0,&v);
        assert_eq!(res,vec![a1.clone(),a2.clone(),a3.clone()]);
        let res = remove(a1,&res);
        assert_eq!(res,vec![a2.clone(),a3.clone()]);
        let res = remove(a2,&res);
        assert_eq!(res,vec![a3.clone()]);
        let res = remove(a3,&res);
        assert_eq!(res,vec![]);
    }

    #[test]
    fn entropy_fair_coin(){
        assert_eq!(B(0.5), 1.0);
    }

    #[test]
    #[ignore]
    fn entropy_die4(){
        assert_eq!(B(0.25), 2.0);

    }

    #[test]
    fn entropy_loaded(){
        assert!((B(0.99) - 0.08).abs() < 0.001);
    }
    
}