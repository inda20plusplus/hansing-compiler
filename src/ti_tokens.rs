#[derive(Debug, PartialEq)]
pub enum Token {
    BinOp(char),
    OpenBrace(char),
    CloseBrace(char),
    Var(char),
    Num(f32),
    NewLine,

    If,
    Then,
    Else,
    End,
    
    While,

    In,
    Out,
}

impl Token {
    pub fn is(&self, t: Token) -> bool{
        //println!("{:?} {:?}, {:?}",self, t, match self {
        //    t => true,
        //    _ => false,
        //});
        //match *self {
        //    t => true,
        //    _ => false,
        //}
        self == &t
    }
    pub fn get_precidence(&self) -> i32{
        if let Token::BinOp(op) = self {
            match op {
                '<'|'='|'>'     => 10,
                '+'|'-'         => 20,
                '*'|'/'         => 30,
                '|'             => 1,
                _ => -1,
            }
        } else {
            -1
        }
    }
}


#[test]
fn test(){
    let t = Token::Var('A');
    assert!(t.is(Token::Var('A')))
}