use crate::ti_tokens::*;
use crate::ast_nodes::Node;
pub struct Parser {
    token_stream: Vec<Token>,
    curtok: Token,
    ongoing: bool,
}

impl Parser {
    pub fn new(mut token_stream: Vec<Token>) -> Self {
        token_stream.reverse();
        let curtok =    //token_stream.pop().unwrap();
                                Token::NewLine;
        Self {
            token_stream,
            curtok,
            ongoing: true,
        }
    }

    fn next(&mut self, skip_nl: bool){
        
        loop {
            let next = self.token_stream.pop();

            if next.is_some() {
                self.curtok = next.unwrap();
            } else {
                println!("Parser has reached end of token stream.");
                self.ongoing = false;
            }

            if !skip_nl || !self.curtok.is(Token::NewLine) || !self.ongoing {
                break;
            }
        }

    }
    /***********************
        AST NODE TYPES     
    ***********************/ 

    //// Expressions
    
    // var ::= Var
    fn parse_var(&mut self) -> Option<Node>{
        if let Token::Var(c) = self.curtok {
            self.next(true);
            Some(Node::Var {name: c} )
        } else {
            None
        }
    }
    // num ::= Num
    fn parse_num(&mut self) -> Option<Node>{
        if let Token::Num(c) = self.curtok {
            self.next(true);
            Some(Node::Num {value: c} )
        } else {
            None
        }
    }
    // par ::= OpenBrace expr ( CloseBrace | NewLine )
    fn parse_par(&mut self) -> Option<Node>{
        if let Token::OpenBrace(open) = self.curtok {
            //println!("b{:?}",self.curtok);
            self.next(true);
            //println!("a{:?}",self.curtok);
            // parse inner expression
            let cont = self.parse_expr();
            //println!("{:?}",cont);
            if self.curtok.is(Token::CloseBrace(')')) {
                self.next(true);
            }
            if cont.is_some() {
                Some(Node::Par { cont: Box::new(cont.unwrap())} )
            } else {
                println!("Closure without content!");
                None
            }  
        } else {
            None
        }
    }
    // in ::= In 
    fn parse_in(&mut self) -> Option<Node>{
        if self.curtok.is(Token::In) {
            self.next(true);
            Some(Node::In)
        } else {
            None
        }
    }
    // primary ::= var | num | par | in
    fn parse_primary(&mut self) -> Option<Node>{
        let r = self.parse_var();
        if r.is_some() {
            return r;
        }
        let r = self.parse_num();
        if r.is_some() {
            return r;
        }
        let r = self.parse_par();
        if r.is_some() {
            return r;
        }
        let r = self.parse_in();
        if r.is_some() {
            return r;
        }
        None
    }
    
    // expr ::= primary (BinOp primary)*, where each BinOp has a priority higher or equal to its predicessor
    fn parse_expr(&mut self) -> Option<Node>{
       let lhs = self.parse_primary();
       //println!("expr: primary parse res {:?}",lhs);
       if let Some(lhs) = lhs {
            //self.next(true);
            self.parse_rhs(0, lhs)
       } else {
            //println!("nat an expression!");
            //self.next(true);
            None
       }

    }
    // rhs_operand ::= BinOp primary            later:    | primary
    fn parse_rhs(&mut self, expr_precidence: i32, mut lhs: Node) -> Option<Node>{
        while self.ongoing {
            let precidence = self.curtok.get_precidence();

            if precidence <= expr_precidence {
                return Some(lhs);
            }

            let op;
            if let Token::BinOp(c) = self.curtok {
                op = c;
            } else {
                op = ' ';
            }
            self.next(true); // eat binop
            let mut rhs = self.parse_primary();

            if rhs.is_none() {
                return Some(lhs); ///////////////////////////////////NO IDEA IF THIS IS RIGHT!!=!==!)E
            }   
            let next_precidence = self.curtok.get_precidence();
            if precidence < next_precidence {
                rhs = self.parse_rhs(precidence + 1, rhs.unwrap());
                if rhs.is_none() {
                    return Some(lhs);
                }
            }
                
                
            lhs = Node::BinOp{op, left: Box::new(lhs), right: Box::new(rhs.unwrap())}; 
        }
        Some(lhs)
    }
    // if ::= If expr Then block Else block End
    fn parse_if(&mut self) -> Option<Node>{
        if self.curtok.is(Token::If) {
            self.next(true);
            let expr = self.parse_expr();
            if expr.is_some() {
                let expr = expr.unwrap();
                if self.curtok.is(Token::Then) {
                    self.next(true);
                    let t_block = self.parse_block();
                    let f_block;
                    if self.curtok.is(Token::Else) {
                        self.next(true);
                        f_block = self.parse_block();
                    } else {
                        self.next(true);
                        f_block = Node::Block{cont: Box::new(Vec::with_capacity(0))};
                    }
                    Some (Node::If { cond: Box::new(expr),  t_block: Box::new(t_block), f_block: Box::new(f_block)})
                } else {
                    println!("No then for if!");
                    None
                }
            } else {
                println!("No cond for if!");
                None
            }
        } else {
            None
        }
    }

    // while ::= While expr block Else block End
    fn parse_while(&mut self) -> Option<Node>{
        if self.curtok.is(Token::While) {
            self.next(true);
            let expr = self.parse_expr();
            if expr.is_some() {
                let expr = expr.unwrap();
                let t_block = self.parse_block();
                if self.curtok.is(Token::Else) {
                    println!("Dont use Else for while loop end!");
                    self.next(true);
                } else {
                    //self.next(true);//Dont! End is alrady eaten!
                }
                Some (Node::While { cond: Box::new(expr),  t_block: Box::new(t_block)})
            } else {
                println!("No cond for While!");
                None
            }
        } else {
            None
        }
    }

    // block ::= (top)*
    fn parse_block(&mut self) -> Node{
        //println!("block parse begins at {:?}",self.curtok);
        let mut cont: Vec<Node> = Vec::new();
        while self.ongoing {
            if self.curtok.is(Token::End) {
                self.next(true);
                return Node::Block{cont: Box::new(cont)};
            } else if self.curtok.is(Token::Else) {
                return Node::Block{cont: Box::new(cont)};
            } else {
                let next = self.parse_top();
                if let Some(next) = next {
                    
                    cont.push(next)
                } else {
                    //nothing
                }

            }
        }
        return Node::Block{cont: Box::new(cont)};
    }

    // out ::= out expr
    fn parse_out(&mut self) -> Option<Node>{
        if self.curtok.is(Token::Out) {
            self.next(false);
            let expr = self.parse_expr();
            if let Some(expr) = expr {
                Some(Node::Out{cont: Box::new(expr)})
            } else {
                println!("Out with no content");
                None
            }
        } else {
            None
        }
    }

    // top ::= expr | if | while | out
    fn parse_top(&mut self) -> Option<Node> {
        while self.curtok.is(Token::NewLine) && self.ongoing {
            self.next(true);
        }
        let r = self.parse_if();
        if r.is_some() {
            return r;
        }
        let r = self.parse_while();
        if r.is_some() {
            return r;
        }
        let r = self.parse_expr();
        if r.is_some() {
            return r;
        }
        let r = self.parse_out();
        if r.is_some() {
            return r;
        }
        self.next(true);
        None
    }
    pub fn parse_prgm(&mut self) -> Vec<Node>{
        let mut ast = Vec::new();
        while self.ongoing {
            let res = self.parse_top();
            if let Some(n) = res {
                println!("{}",n.disp());
                ast.push(n);
            } else {
                //println!(">None at parse");
                break;
            }
        }
        ast
    }
}


use std::io::*;
#[test]
fn test() {
    let ts = vec!
    [
    Token::If,
    Token::Num(1.0),
    Token::Then,
    Token::Num(-10.0), 
    Token::BinOp('+'), 
    Token::Num(2.0), 
    //Token::CloseBrace('('),
    Token::BinOp('*'),
    Token::Var('B'),
    Token::BinOp('+'),
    Token::Var('C'),
    Token::BinOp('/'),
    Token::Var('D'),
    Token::BinOp('-'),
    Token::OpenBrace('('), 
    Token::Num(1.0),  
    Token::BinOp('-'), 
    Token::Var('A'), 
    Token::CloseBrace(')'),
    Token::NewLine,
    Token::Var('B'),
    Token::Else,
    Token::Num(7.0),
    Token::BinOp('*'),
    Token::Var('B'),
    Token::End,
    Token::NewLine,
    Token::Num(-10.0), 
    Token::BinOp('+'), 
    Token::Num(2.0),
    Token::NewLine,
    Token::Num(-9.0), 
    Token::BinOp('+'), 
    Token::Num(2.0), 
    Token::BinOp('*'), 
    Token::Num(2.0), 
    Token::NewLine,
    ];
    let mut parser = Parser::new(ts);
    let stdin = stdin();
    let mut inp = String::new();
    while parser.ongoing {
        let res = parser.parse_top();
        if let Some(expr) = res {
            println!(": {}",expr.disp())
        } else {
            println!(": None");
        }
        stdin.read_line(&mut inp);
    }

}





