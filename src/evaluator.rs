use crate::error::{LiResult, LisperError};
use crate::parser::Rule;
use pest::iterators::Pairs;
use std::convert::TryInto;

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
