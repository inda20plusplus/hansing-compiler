pub mod ti_tokens;
pub mod ti_parser;
pub mod ast_nodes;
pub mod ti_interp;
pub mod ti_simple_lexer;
pub mod jit;
fn main() {
    let mut lexer = ti_simple_lexer::Lexer::new(prgm_str());
    let ts = test_fib(1_00.0);
    let ts = lexer.tokenize(); 
    println!("{:?}",ts);
    let mut interp = ti_interp::Interp::new();
    let mut parser = ti_parser::Parser::new(ts);
    let ast = parser.parse_prgm();
    for root in ast {
        root.eval(&mut interp);
    } 
}

fn prgm_str() -> String{
    "
    100|A
    (A+1)*2|B
    While A
      If A > 0
      Then
        A - B | A
      Else
        A + B | A
      End
      B - 1 | B
      Out A
    End
    Out B
    
    
    ".to_string()
}


use ti_tokens::Token;
fn test_ts() -> Vec<Token> {
    vec![
        Token::Num(30.0), Token::BinOp('|'), Token::Var('A'),
        Token::Num(20.0), Token::BinOp('|'), Token::Var('B'),
        Token::Num(10.0), Token::BinOp('|'), Token::Var('C'),
        Token::If, Token::Var('A'), Token::BinOp('+'), Token::Var('B'), Token::BinOp('>'), Token::Var('C'),
        Token::Then,
        Token::Var('A'), Token::BinOp('+'), Token::Num(1.0), Token::BinOp('|'), Token::Var('A'),
        Token::Else,
        Token::Var('A'), Token::BinOp('|'), Token::Var('C'),
        Token::End,
        Token::Var('A'), Token::BinOp('*'), Token::OpenBrace('('), Token::Var('B'), Token::BinOp('+'),Token::Var('C'), Token::CloseBrace(')'), Token::BinOp('|'), Token::Var('D'),
        Token::NewLine,
    ]
}
fn test_ts2() -> Vec<Token> {
    vec![
        Token::Num(0.0), Token::BinOp('|'), Token::Var('A'),
        Token::In, Token::BinOp('|'), Token::Var('B'),
        Token::In, Token::BinOp('|'), Token::Var('C'),
        Token::Num(0.0), Token::BinOp('|'), Token::Var('D'),
        Token::While, Token::Var('A'), Token::BinOp('<'), Token::Num(100.0), Token::NewLine,Token::NewLine,Token::NewLine,
        Token::If, Token::Var('A'), Token::BinOp('>'), Token::Var('B'),Token::NewLine,Token::NewLine,
        Token::Then,Token::NewLine,
        Token::Var('A'), Token::BinOp('+'), Token::Num(1.0), Token::BinOp('|'), Token::Var('A'),Token::NewLine,
        Token::Else,Token::NewLine,Token::NewLine,Token::NewLine,Token::NewLine,
        Token::Var('A'), Token::BinOp('+'), Token::Var('C'), Token::BinOp('|'), Token::Var('A'),
        Token::End,Token::NewLine,
        Token::Out, Token::Var('A'),
        Token::Var('D'), Token::BinOp('+'), Token::Num(1.0), Token::BinOp('|'), Token::Var('D'),
        Token::End,
        Token::NewLine,
        Token::Out, Token::Var('D'),
        //Token::Var('A'), Token::BinOp('*'), Token::OpenBrace('('), Token::Var('B'), Token::BinOp('+'),Token::Var('C'), Token::CloseBrace(')'), Token::BinOp('|'), Token::Var('D'),
        Token::NewLine,
    ]
}
fn test_sqrt(n: f32) -> Vec<Token> {
    vec![
        Token::Num(n), Token::BinOp('|'), Token::Var('N'),
        Token::Var('N'), Token::BinOp('/'), Token::Num(2.0), Token::BinOp('|'), Token::Var('S'),
        Token::Num(0.0), Token::BinOp('|'), Token::Var('T'),
        Token::While, Token::Num(1.0), Token::BinOp('-'), 
        Token::OpenBrace('('), Token::Var('S'), Token::BinOp('='), Token::Var('T'), Token::CloseBrace(')'),
        Token::Var('S'), Token::BinOp('|'), Token::Var('T'),
        Token::OpenBrace('('), Token::Var('N'), Token::BinOp('/'), Token::Var('T'), Token::BinOp('+'), Token::Var('T'),Token::CloseBrace(')'),
        Token::BinOp('/'), Token::Num(2.0), Token::BinOp('|'), Token::Var('S'),
        Token::End, Token::NewLine,
        Token::Out, Token::Var('S'), Token::Var('S'),
        Token::NewLine,
        ]
}
fn test_fib(n: f32) -> Vec<Token> {
    vec![
        Token::Num(n), Token::BinOp('|'), Token::Var('N'),
        Token::Num(1.0), Token::BinOp('|'), Token::Var('A'),
        Token::Num(0.0), Token::BinOp('|'), Token::Var('B'),
        Token::While, Token::Num(0.0), Token::BinOp('<'), Token::Var('N'),
        Token::If, Token::Var('A'), Token::BinOp('<'), Token::Var('B'),
        Token::Then,
        Token::Var('A'), Token::BinOp('+'), Token::Var('B'), Token::BinOp('|'), Token::Var('A'), 
        Token::Else,
        Token::Var('A'), Token::BinOp('+'), Token::Var('B'), Token::BinOp('|'), Token::Var('B'), 
        Token::End,
        Token::Var('N'), Token::BinOp('-'), Token::Num(1.0), Token::BinOp('|'), Token::Var('N'), 
        Token::Out, Token::Var('N'),
        Token::End,

        Token::If, Token::Var('A'), Token::BinOp('<'), Token::Var('B'),
        Token::Then,
        Token::Out, Token::Var('B'), 
        Token::Else,
        Token::Out, Token::Var('A'), 
        Token::End,
    ]
}