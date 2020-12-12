use crate::error::{LiResult, LisperError};
use crate::parser::Type;
use std::convert::TryInto;

pub fn eval(lval: Type) -> LiResult<Type> {
    match lval {
        Type::Sexpr(v) => lval_eval_sexprs(v),
        _ => Ok(lval),
    }
}

fn lval_eval_sexprs(mut sexpr: Vec<Type>) -> LiResult<Type> {
    /* Empty Expression */
    if sexpr.len() == 0 {
        return Ok(Type::Sexpr(sexpr));
    }

    /* Recursively evaluate all Sexprs in list */
    for i in 0..sexpr.len() {
        // TODO: rework to avoid cloning
        sexpr[i] = eval(sexpr[i].clone())?;
    }

    /* Single Expression */
    if sexpr.len() == 1 {
        return Ok(sexpr.pop().unwrap());
    }

    /* Ensure First Element is Symbol */
    if let Type::Symbol(op) = sexpr.remove(0) {
        builtin_op(sexpr, &op)
    } else {
        Err(LisperError::SymbolNotFound)
    }
}

fn builtin_op(values: Vec<Type>, op: &str) -> LiResult<Type> {
    /* Ensure all arguments are numbers */
    let mut numbers: Vec<i64> = values
        .iter()
        .map(|v| match v {
            Type::Number(n) => *n,
            _ => unreachable!(),
        })
        .collect();

    /* Pop the first element */
    let mut x = numbers.remove(0);
    for n in numbers {
        // TODO: move op check outside of the loop
        match op {
            "+" => x += n,
            "-" => x -= n,
            "*" => x *= n,
            "/" => {
                if n == 0 {
                    return Err(LisperError::ZeroDiv);
                } else {
                    x /= n;
                }
            }
            "%" => x %= n,
            "^" => x = x.pow(n.try_into().unwrap()),
            "min" => x = x.min(n),
            "max" => x = x.max(n),
            _ => unreachable!(),
        }
    }

    Ok(Type::Number(x))
}
