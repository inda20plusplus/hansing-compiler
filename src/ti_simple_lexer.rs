use crate::ti_tokens::Token;
//use lazy_static::lazy_static;
//use regex::Regex;

pub struct Lexer {
    source: Vec<char>,
}

impl Lexer {
    pub fn new(s: String) -> Self {
        Self {
            source: s.chars().collect(),
        }
    }
    fn char(&self, n: usize) -> char {
        if n < self.source.len() {
            self.source[n]
        } else {
            ';' // default
        }
    }
    pub fn tokenize(&mut self) -> Vec<Token>{
        //let source: Vec<char> = source.chars().collect();
        let mut ts = Vec::new();
        let mut n = 0usize;
        while n < self.source.len() {
            
            let c = self.char(n);
            //print!("{}",c);
            let c2 = self.char(n+1);
            // keywords
            match format!("{}{}",c,c2).as_str() {
                "If" => { ts.push(Token::If);       n += 3; continue; },
                "Th" => { ts.push(Token::Then);     n += 5; continue; },
                "El" => { ts.push(Token::Else);     n += 5; continue; },
                "En" => { ts.push(Token::End);      n += 4; continue; },
                "Wh" => { ts.push(Token::While);    n += 6; continue; },
                "In" => { ts.push(Token::In);       n += 3; continue; },
                "Ou" => { ts.push(Token::Out);      n += 4; continue; },
                _ => {},
            }
            // variables
            if c.is_uppercase() && c.is_alphabetic() {
                ts.push(Token::Var(c));
                n += 1;
                continue;
            }
            // closures and el
            match c {
                '('|'['|'{'     => { ts.push(Token::OpenBrace(c));  n += 1; continue; },
                ')'|']'|'}'     => { ts.push(Token::CloseBrace(c)); n += 1; continue; },
                ';'|'\n'        => { ts.push(Token::NewLine);       n += 1; continue;},
                _ => {},
            }
            // nums
            if c.is_numeric() || (c == 'n' && c2.is_numeric()) {
                n += 1;
                let mut nc = c2;
                let mut di = String::from(if c.is_numeric() {c} else {'-'});
                while nc.is_numeric() || nc == '.' {
                    n += 1;
                    di.push(nc);
                    nc = self.char(n);
                }
                ts.push(Token::Num(di.parse().unwrap()));
                continue;
            }
            // bin ops
            match c {
                '<'|'='|'>'|'+'|'-'|'*'|'/'|'|' => 
                    { ts.push(Token::BinOp(c)); n += 1; continue; },
                _ => {}
            }
            n+=1
        }
        ts
    }
}

