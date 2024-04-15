use crate::parser::Object;
use crate::errors::EvaluatorError;

use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

use Object::*;

type Result<R> = std::result::Result<R, EvaluatorError>;

#[derive(Debug, PartialEq, Default)]
pub struct Env {
    prev: Option<Rc<RefCell<Env>>>,
    vars: HashMap<String, Object>
}

impl Env {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn extend(prev: Rc<RefCell<Self>>) -> Self {
        Self {
            prev: Some(prev),
            vars: HashMap::new()
        }
    }

    pub fn get<T>(&self, name: T) -> Option<Object>
        where T: AsRef<str>
    {
        match self.vars.get(name.as_ref()) {
            Some(value) => Some(value.clone()),
            None => self
                .prev
                .as_ref()
                .and_then(|o| o.borrow().get(name).clone())
        }

    }
    pub fn set<T>(&mut self, name: T, val: Object)
        where T: AsRef<str>
    {
        self.vars.insert(name.as_ref().to_string(), val);
    }
}

pub fn eval<T: AsRef<str>>(code: T, env: &mut Rc<RefCell<Env>>) -> Result<Object> {

    match crate::parser::parse(code) {
        Ok(parsed) => {
            eval_obj(&parsed, env)
        },
        Err(e) => return Err(EvaluatorError::Error(e.to_string()))
    }
}

fn eval_obj(obj: &Object, env: &mut Rc<RefCell<Env>>) -> Result<Object> {
    match obj {
        Void => Ok(Void),
        Function(_params, _body) => Ok(Void),
        Bool(_) => Ok(obj.clone()),
        Number(n) => Ok(Number(*n)),
        Symbol(s) => eval_symbol(s, env),
        List(l) => eval_list(l, env)
    }
}


// Evals single Symbol
fn eval_symbol<T>(symbol: T, env: &mut Rc<RefCell<Env>>) -> Result<Object>
    where T: AsRef<str>
{
    let symbol = symbol.as_ref();
    let value = (*env).borrow_mut().get(symbol);

    match value {
        Some(x) => Ok(x.clone()),
        None => Err(EvaluatorError::Error("None symbol?".to_owned()))
    }
}

fn eval_list(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object> {
    let head = &list[0];
    match head {
        Symbol(s) => match s.as_str() {
            // FIXME != not in scheme.
            // Sumtypes as strings bruh
            "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" => {
                return eval_bin_op(list, env);
            },
            "if" => eval_if(list, env),
            "define" => eval_define(list, env),
            "lambda" => eval_function_definition(list),
            _ => eval_function_call(s, list, env)
        },
        _ => {
            let mut new_list = vec![];
            for obj in list {
                let result = eval_obj(obj, env)?;
                match result {
                    Void => {},
                    _ => new_list.push(result),
                }
            }
            Ok(Object::List(new_list))
        }
    }
}

fn eval_define(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object> {
    let symbol = match &list[1] {
        Symbol(s) => s.clone(),
        _ => return Err(EvaluatorError::Error("Invalid define".to_owned()))
    };
    let val = eval_obj(&list[2], env)?;
    (*env).borrow_mut().set(symbol, val);
    Ok(Void)
}



// FIXME i think this is the shittiest thing there...
// Binary ops only for numbers?
fn eval_bin_op(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object> {
    if list.len() != 3 {
        return Err(EvaluatorError::Error("Failed to evaluate binary operation. Expected 3 object.".to_owned()));
    }

    let operator = &list[0];
    let left = eval_obj(&list[1], env)?;
    let right = eval_obj(&list[2], env)?;

    let lvalue = match left {
        // FIXME: Number only??? there should be a matching of operation!
        Number(n) => n,
        _ => return Err(
            EvaluatorError::Error("Available data types in binary ops is only numbers :(".to_owned())
        )
    };

    let rvalue = match right {
        // FIXME: Number only??? there should be a matching of operation!
        Number(n) => n,
        _ => return Err(
            EvaluatorError::Error("Available data types in binary ops is only numbers :(".to_owned())
        )
    };

    Ok(match operator {
        Symbol(s) => {
            match s.as_str() {
                "+" => Number(lvalue + rvalue),
                "-" => Number(lvalue - rvalue),
                "*" => Number(lvalue * rvalue),
                "/" => Number(lvalue / rvalue),
                "<" => Bool(lvalue < rvalue),
                ">" => Bool(lvalue > rvalue),
                "=" => Bool(lvalue == rvalue),
                "!=" => Bool(lvalue != rvalue),
                _ => return Err(
                    EvaluatorError::Error("Operator not recognized!".to_owned())
                )
            }
        },
        _ => return Err(
            EvaluatorError::Error("Operator not recognized!".to_owned())
        )

    })
}

fn eval_if(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object> {
    let cond_obj = eval_obj(&list[1], env)?;
    let cond = match cond_obj {
        Bool(b) => b,
        _ => return Err(EvaluatorError::Error("Expected boolean in conditional error, found something else".to_owned()))
    };

    match cond {
        true => eval_obj(&list[2], env),
        false => eval_obj(&list[3], env)
    }
}


fn eval_function_definition(list: &Vec<Object>) -> Result<Object> {
    let params = match &list[1] {
        List(list) => {
            let mut params = vec![];
            for param in list {
                match param {
                    Symbol(s) => params.push(s.clone()),
                    _ => return Err(EvaluatorError::Error("Invalid funtion parameter".to_owned()))
                }
            }
            params
        },
        _ => return Err(EvaluatorError::Error("Expected list of parameters, found something else".to_owned()))
    };

    let body = match &list[2] {
        List(list) => list.clone(),
        _ => return Err(EvaluatorError::Error("Expected list as body of function, found something else!".to_owned()))
    };

    Ok(Function(params, body))
}


fn eval_function_call<T: AsRef<str>>(name: T, list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object> {
    let m = env.borrow_mut().get(name.as_ref());
    match m {
        Some(Function(params, body)) => {
            let mut inner_environment = Rc::new(
                RefCell::new(
                    Env::extend(env.clone())
                )
            );

            for (idx, param) in params.iter().enumerate() {
                let value = eval_obj(&list[idx + 1], env)?;
                inner_environment.borrow_mut().set(param, value);
            }

            Ok(eval_obj(&List(body.clone()), &mut inner_environment)?)
        },
        None => Err(EvaluatorError::Error("Failed to execute function! no such function is defined!".to_owned())),
        _ => Err(EvaluatorError::Error("Given a function for eval is not a function?".to_owned()))
    }
}

#[cfg(test)]
mod evaluator_tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(+ 1 2)", &mut env).unwrap();
        assert_eq!(result, Object::Number(3.0));
    }

    #[test]
    fn test_area_of_a_circle() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "(
                        (define r 10)
                        (define pi 314)
                        (* pi (* r r))
                      )";
        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Number((314.0 * 10.0 * 10.0) as f64)])
        );
    }

    #[test]
    fn test_sqr_function() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "(
                        (define sqr (lambda (r) (* r r)))
                        (sqr 10)
                       )";
        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Number((10.0 * 10.0) as f64)])
        );
    }

    #[test]
    fn test_is_borrowed() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "(
                        (define doo (lambda (x y z) (+ (+ x y) z)))
                        (doo 10 20 30)
                        )";
        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Number(10.0 + 20.0 + 30.0 as f64)])
        );
    }
}
