use crate::parser::Rule;
use pest::iterators::Pairs;
use std::convert::TryInto;

pub fn eval(pairs: Pairs<Rule>) -> i64 {
    let mut op: Option<char> = None;
    // since we using Polish notation
    // we could eval numbers directly to result
    let mut numbers = Vec::with_capacity(2);
    dbg!(numbers.clone());
    for pair in pairs {
        let token = pair.as_str();
        match pair.as_rule() {
            Rule::operator => op = Some(token.chars().nth(0).unwrap()),
            Rule::number => numbers.push(eval_number(token)),
            Rule::expr => numbers.push(eval(pair.into_inner())),
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
            numbers.push(result);
        }
    }

    assert_eq!(numbers.len(), 1);
    numbers[0]
}

fn eval_number(x: &str) -> i64 {
    x.parse::<i64>().unwrap()
}

fn eval_op(x: i64, op: &char, y: i64) -> i64 {
    match op {
        '+' => return y + x,
        '-' => return y - x,
        '*' => return y * x,
        '/' => return y / x,
        '%' => return y % x,
        '^' => return y.pow(x.try_into().unwrap()),
        _ => unreachable!(),
    }
}
