use crate::errors;


/// All available tokens!
#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Symbol(String),
    LeftParenthesa,
    RightParenthesa,
}


/// Produces vector of token from something, that can be borrowed as &str
pub fn tokenize<T>(content: T) -> Result<Vec<Token>, errors::TokenizeError>
    where T:
        AsRef<str>
{
    use Token::*;

    let content = content.as_ref()
        .replace("(", " ( ")
        .replace(")", " ) ");

    let words = content.split_whitespace();

    let mut tokens: Vec<Token> = vec![];

    for word in words {
        match word {
            "(" => tokens.push(LeftParenthesa),
            ")" => tokens.push(RightParenthesa),
            _ => {
                // FIXME I don't like this :\
                let p = word.parse::<f64>();
                if let Ok(res) = p {
                    tokens.push(Number(res))
                } else {
                    tokens.push(Symbol(word.to_owned()))
                }
            }
        }
    }

    Ok(tokens)
}

#[test]
fn tokenize_1() {
    use Token::*;
    let test_input = "(+ 1 2)";
    assert_eq!(
        tokenize(test_input).unwrap(),
        vec![
            LeftParenthesa,
            Symbol("+".to_owned()),
            Number(1.0),
            Number(2.0),
            RightParenthesa
        ]
    )
}
