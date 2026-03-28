
use std::{io};

#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => { chars.next(); }

            '+' => { tokens.push(Token::Plus); chars.next(); }
            '-' => { tokens.push(Token::Minus); chars.next(); }
            '*' => { tokens.push(Token::Multiply); chars.next(); }
            '/' => { tokens.push(Token::Divide); chars.next(); }
            '(' => { tokens.push(Token::LeftParen); chars.next(); }
            ')' => { tokens.push(Token::RightParen); chars.next(); }

            '0'..='9' => {
                let mut num_text = String::new();

                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        num_text.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if let Ok(number) = num_text.parse::<f64>() {
                    tokens.push(Token::Number(number))
                }
            }

            _ => {
                println!("Syntax Error '{ch}'");
                chars.next();
            }
        }
    }
    tokens
}

fn main() {
    println!("Please enter calculation:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("{:?}", tokenize(&input));
}
