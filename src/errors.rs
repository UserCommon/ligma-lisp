use std::error::Error;
use std::fmt;

use crate::token::Token;

#[derive(Debug)]
struct Details(String);

#[derive(Debug)]
pub enum TokenizeError {
    Error(Details)
}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Error(ref cause) =>
                write!(f, "failed to intepret!\n\tcause: {}\n", cause.0)
        }
    }
}

impl Error for TokenizeError {}


#[derive(Debug)]
pub enum ParserError {
    ExpectedOtherToken(Token, Token),
    Error(String)
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Error(ref cause) =>
                write!(f, "failed to parse!\n\tcause: {}\n", cause),
            Self::ExpectedOtherToken(ref expected, ref found) =>
                write!(f, "failed to parse!\n\tcause: expected {:?} token, found: {:?}\n", expected, found)
        }
    }
}

impl Error for ParserError {}


#[derive(Debug)]
pub enum EvaluatorError {
    Error(String)
}

impl fmt::Display for EvaluatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Error(ref cause) =>
                write!(f, "failed to parse!\n\tcause: {}\n", cause),
       }
    }
}

impl Error for EvaluatorError {}
