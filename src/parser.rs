#![allow(dead_code)]

use crate::token::{Token, tokenize};
use crate::errors::ParserError;

/// All available objects in out language!
#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    Void,
    Number(f64),
    Bool(bool),
    Symbol(String),
    // Params, Body
    Function(Vec<String>, Vec<Object>),
    List(Vec<Object>)
}

pub fn parse<T>(content: T) -> Result<Object, Box<dyn std::error::Error>>
where
    T: AsRef<str>
{
    let mut tokens: Vec<Token> = tokenize(content)?
        .into_iter()
        .rev()
        .collect();

    let list = parse_list(&mut tokens)?;

    Ok(list)
}


/// Important, that it requires reversed token list
pub fn parse_list(tokens: &mut Vec<Token>) -> Result<Object, ParserError> {
    use Token::*;

    let token = tokens.pop();
    if token != Some(LeftParenthesa) && token != None {
        return Err(
            ParserError::ExpectedOtherToken(LeftParenthesa, token.unwrap())
        );
    }

    let mut list: Vec<Object> = vec![];
    while let Some(token) = tokens.pop() {
        match token {
            Number(n) =>
                list.push(Object::Number(n)),
            Symbol(n) =>
                list.push(Object::Symbol(n)),
            LeftParenthesa => {
                tokens.push(LeftParenthesa);
                let sub_seq = parse_list(tokens)?;
                list.push(sub_seq);
            }
            RightParenthesa => {
                return Ok(Object::List(list))
            }
        }
    }
    Ok(Object::List(list))
}




#[cfg(test)]
mod parser_test {
    use super::*;

    #[test]
    fn parse_1() {
        use Object::*;

        let list = parse("(+ 1 2)").unwrap();

        assert_eq!(
            list,
            List(vec![
                Symbol("+".to_owned()),
                Number(1.0),
                Number(2.0),
            ])
        )

    }
}
