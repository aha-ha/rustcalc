
use std::{io};

#[derive(Debug, PartialEq, Clone)]
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
            '*' => { tokens.push(Token::Multiply); chars.next(); }
            '/' => { tokens.push(Token::Divide); chars.next(); }
            '(' => { tokens.push(Token::LeftParen); chars.next(); }
            ')' => { tokens.push(Token::RightParen); chars.next(); }

            '0'..='9' | '-' => {

                if ch == '-' {
                    let is_operator = !tokens.is_empty() && !matches!(tokens.last(), Some(Token::Plus) | Some(Token::Minus) | Some(Token::Multiply) | Some(Token::Divide) | Some(Token::LeftParen));
                    if is_operator {
                        tokens.push(Token::Minus);
                        chars.next();
                        continue;
                    }
                }
                
                let mut num_text = String::new();

                num_text.push(ch);
                chars.next();

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
                } else if num_text == "-" {
                    tokens.push(Token::Minus);
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

fn precedence(token: &Token) -> i32 {
    match token {
        Token::Plus | Token::Minus => 1,
        Token::Multiply | Token::Divide => 2,
        _ => 0,
    }
}

fn apply_operator(numbers: &mut Vec<f64>, op: &Token) -> Result<(), String> {
    let right = numbers.pop().ok_or("Missing Operand")?;
    let left = numbers.pop().ok_or("Missing Operand")?;
    let result = match op {
        Token::Plus => left + right,
        Token::Minus => left - right,
        Token::Multiply => left * right,
        Token::Divide => {
            if right == 0.0 { return Err("Seriously? You really want to see the universe explode? You cannot divide by zero".to_string()); }
            left / right
        },
        _ => return Err("Invalid Operator".to_string()),
    };
    numbers.push(result);
    Ok(())
}

fn calc(tokens: &[Token]) -> Result<f64, String> {
    let mut numbers: Vec<f64> = Vec::new();
    let mut operators: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(n) => numbers.push(*n),
            Token::LeftParen => operators.push(Token::LeftParen),
            Token::RightParen => {
                let mut found = false;
                while let Some(op) = operators.pop() {
                    if op == Token::LeftParen {
                        found = true;
                        break;
                    }
                    apply_operator(&mut numbers, &op)?;
                }
                if !found { return Err("Mismatched Parentheses".to_string()); }
            }
            op @ (Token::Plus | Token::Minus | Token::Multiply | Token::Divide) => {
                while let Some(top_op) = operators.last() {
                    if precedence(top_op) >= precedence(op) {
                        let top = operators.pop().unwrap();
                        apply_operator(&mut numbers, &top)?;
                    } else {
                        break;
                    }
                }
                operators.push(op.clone());
            }
        }
    }

    while let Some(op) = operators.pop() {
        if op == Token::LeftParen { 
            return Err("Parenthesis opened but never closed".to_string()); 
        }
        apply_operator(&mut numbers, &op)?;
    }


    numbers.pop().ok_or("Calculation error: No result on stack".to_string())
}
fn main() {
    println!("Please enter calculation:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let tokens = tokenize(&input);
    println!("{:?}", tokens);
    match calc(&tokens) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    };
}
