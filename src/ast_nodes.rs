use crate::ti_interp::*;
#[derive(Debug, Clone)] //TODO REMOVE CLONE (JIT NEEDS)
pub enum Node {
    Var {name: char},
    Num {value: f32},
    Par {cont: Box<Node>},
    BinOp {op: char, left: Box<Node>, right: Box<Node>},
    If {cond: Box<Node>, t_block: Box<Node>, f_block: Box<Node>},
    While {cond: Box<Node>, t_block: Box<Node>},
    Block {cont: Box<Vec<Node>>},

    In,
    Out{cont: Box<Node>},
}

impl Node {
    pub fn disp(&self) -> String {
        match self {
            Node::Var{name} => format!("{}", name),
            Node::Num{value} => format!("{}", value),
            Node::Par{cont} => format!("({})",cont.disp()),
            Node::BinOp{op, left, right} => 
                format!("({}{}{})",left.disp(),op,right.disp()),
            Node::If{cond, t_block, f_block} =>
                format!("\nif {}: \nthen: {}\nelse: {}\nend",cond.disp(), t_block.disp(), f_block.disp()),
            Node::While{cond, t_block} =>
                format!("\nwhile {}:\ndo: {}\nend",cond.disp(), t_block.disp()),
            Node::Block{cont} => {
                let mut buf = String::new();
                for c in cont.iter() {
                    buf.push_str(c.disp().as_str());
                    buf.push_str("; ")
                }
                format!("[ {}]", buf)
            }
            Node::In => "in".to_string(),
            Node::Out{cont} => format!("out({})",cont.disp()),
        }
    }
    pub fn eval(&self, interp: &mut Interp) -> f32 {
        match self {
            Node::Var {name} => {
                interp.ld(name)
            },
            Node::Num {value} => {
                *value
            },
            Node::Par {cont} => {
                cont.eval(interp)
            },
            Node::BinOp {op, left, right} =>{ 
                let l = left.eval(interp);
                let r = right.eval(interp);

                match op {
                    '+' => l + r,
                    '-' => l - r,
                    '*' => l * r,
                    '/' => l / r,
                    '=' => if l == r {1.0} else {0.0},
                    '>' => if l > r {1.0} else {0.0},
                    '<' => if l < r {1.0} else {0.0},
                    '|' => {
                        match **right{
                            Node::Var{name} => interp.asg(name, l),
                            _ => println!("asg to non-var!"),
                        }
                        l
                    },
                    _ => 0.0,
                }
            },
            Node::If {cond, t_block, f_block} =>{
                if cond.eval(interp) != 0.0 {
                    t_block.eval(interp)
                } else {
                    f_block.eval(interp)
                }
            },
            Node::While {cond, t_block} =>{
                while cond.eval(interp) != 0.0 {
                    t_block.eval(interp);
                }
                0.0
            },
            Node::Block {cont} =>{
                for c in cont.iter() {
                    c.eval(interp);
                }
                0.0
            },
            Node::In    => { 
                interp.input()
            },
            Node::Out{cont}   => { 
                let o = cont.eval(interp);
                interp.output(o);
                o
            },

        }
    }
    /*pub fn codegen(&self) {
        match self {
            Node::Var {name} => {
                interp.ld(name)
            },
            Node::Num {value} => {
                *value
            },
            Node::Par {cont} => {
                cont.eval(interp)
            },
            Node::BinOp {op, left, right} =>{ 
                let l = left.eval(interp);
                let r = right.eval(interp);

                match op {
                    '+' => l + r,
                    '-' => l - r,
                    '*' => l * r,
                    '/' => l / r,
                    '=' => if l == r {1.0} else {0.0},
                    '>' => if l > r {1.0} else {0.0},
                    '<' => if l < r {1.0} else {0.0},
                    '|' => {
                        match **right{
                            Node::Var{name} => interp.asg(name, l),
                            _ => println!("asg to non-var!"),
                        }
                        l
                    },
                    _ => 0.0,
                }
            },
            _ => {},
        }
    }*/
}

fn ops(c: &char, l: f32, r: f32) -> f32 {
    match c {
        '+' => l + r,
        '-' => l - r,
        '*' => l * r,
        '/' => l / r,
        '=' => if l == r {1.0} else {0.0},
        '>' => if l > r {1.0} else {0.0},
        '<' => if l < r {1.0} else {0.0},
        _ => 0.0,
    }
}