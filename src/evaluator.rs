//use crate::error::{LiResult, LisperError};
use crate::parser::Type;
//use pest::iterators::Pairs;
//use std::convert::TryInto;

/*
pub fn eval(pairs: Pairs<Rule>) -> LiResult<i64> {
    let mut op: Option<&str> = None;
    // since we using Polish notation
    // we could eval numbers directly to result
    let mut numbers = Vec::with_capacity(2);
    for pair in pairs {
        let token = pair.as_str();
        match pair.as_rule() {
            Rule::symbol => op = Some(token),
            Rule::number => numbers.push(eval_number(token)),
            Rule::expr => numbers.push(eval(pair.into_inner())?),
            Rule::EOI => {
                //TODO EOI: why it's not striped out as new line?
            }
            _ => {
                dbg!(pair);
                unreachable!()
            }
        };
        // check if we have enough values to evaluate
        if numbers.len() == 2 {
            let result = eval_op(numbers.pop().unwrap(), &op.unwrap(), numbers.pop().unwrap());
            numbers.push(result?);
        }
    }

    assert_eq!(numbers.len(), 1);
    Ok(numbers[0])
}
*/

pub fn eval(lval: Type) -> Type {
    match lval {
        Type::Sexpr(v) => lval_eval_sexprs(v),
        _ => lval,
    }
}

fn lval_eval_sexprs(mut sexpr: Vec<Type>) -> Type {
    /* Recursively evaluate all Sexprs in list */
    // can it shrinks?
    for i in 0..sexpr.len() {
        // TODO: rework with references to avoid cloning
        sexpr[i] = eval(sexpr[i].clone());
    }

    // TODO: move to top?
    /* Empty Expression */
    if sexpr.len() == 0 {
        // return Type::Sexpr(sexpr);
    }

    /* Single Expression */
    if sexpr.len() == 1 {
        return sexpr.pop().unwrap();
    }

    /* Ensure First Element is Symbol */
    if let Type::Symbol(op) = sexpr.remove(0) {
        builtin_op(sexpr, op)
    } else {
        //TODO handle error
        panic!("S-expression Does not start with symbol!");
    }
}

fn builtin_op(values: Vec<Type>, _op: String) -> Type {
    assert!(values.iter().all(|v| match v {
        Type::Number(_) => true, // why i can't use == to match any number ?
        _ => false
    }) == true);

    Type::Number(0)
}

/*
fn eval_number(x: &str) -> i64 {
    x.parse::<i64>().unwrap()
}

fn eval_op(x: i64, op: &str, y: i64) -> LiResult<i64> {
    Ok(match op {
        "+" => y + x,
        "-" => y - x,
        "*" => y * x,
        "/" => {
            if x == 0 {
                return Err(LisperError::ZeroDiv);
            } else {
                y / x
            }
        }
        "%" => y % x,
        "^" => y.pow(x.try_into().unwrap()),
        "min" => y.min(x),
        "max" => y.max(x),
        _ => unreachable!(),
    })
}
*/
