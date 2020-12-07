use pest::iterators::Pairs;
use pest::Parser;

use crate::error::LiResult;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

use crate::evaluator;

pub enum Type {
    Number(i64),
    Symbol(String),
    Sexpr(Vec<Type>),
}

pub fn parse_and_eval(line: &str) -> LiResult<i64> {
    evaluator::eval(parse_expr_list(line)?)
}

pub fn lval_read(line: &str) -> LiResult<Type> {
    // pass to func?
    let pairs = GrammarParser::parse(Rule::expr_list, line)?;

    let mut sexprs = vec![];
    for pair in pairs {
        let token = pair.as_str();
        return Ok(
        match pair.as_rule() {
            //TODO: will it always be first
            Rule::symbol => sexprs.push(Type::Symbol(token.to_string())),
            Rule::number => sexprs.push(Type::Number(token.parse::<i64>().unwrap())),
            Rule::sexpr => {
                sexprs.push(lval_read(pair.into_inner())?)
                Type::Sexpr
            },
            Rule::EOI => {
                //TODO EOI: why it's not striped out as new line?
            }
            _ => {
                dbg!(pair);
                unreachable!()
            }
        });
    }
    /*
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
    */
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::LisperError;

    #[test]
    fn test_parser_ok() {
        let line = "+ 5 (* 2 2)";
        let result = parse_expr_list(line);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_gt_ten() {
        let line = "+ 10 (* 2 2)";
        let result = parse_expr_list(line);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_err() {
        let line = "fuck+ 5 (* 2 2)";
        let result = parse_expr_list(line);
        assert!(!result.is_ok());
    }

    // TODO: figure out how to construct Pairs<Rule> directly
    // and move eval tests to `evaluator`
    #[test]
    fn test_parser_and_eval_single() {
        let line = "+ 1";
        assert_eq!(parse_and_eval(line).unwrap(), 1);
    }

    #[test]
    fn test_parser_and_eval_division() {
        let line = "/ 121 11";
        assert_eq!(parse_and_eval(line).unwrap(), 11);
    }

    #[test]
    fn test_division_by_zero() {
        match parse_and_eval("/ 10 0") {
            Err(LisperError::ZeroDiv) => assert!(true),
            Err(_) => assert!(false),
            Ok(_) => assert!(false),
        }
    }

    #[test]
    fn test_parser_and_eval_long() {
        let line = "+ 1 2 1 4";
        assert_eq!(parse_and_eval(line).unwrap(), 8);
    }

    #[test]
    fn test_parser_and_eval_max_pow() {
        let line = "max (^ 2 3) (^ 3 2)";
        assert_eq!(parse_and_eval(line).unwrap(), 9);
    }

    #[test]
    fn test_parser_and_eval_book_example1() {
        let line = "* 10 (+ 1 51)";
        assert_eq!(parse_and_eval(line).unwrap(), 520);
    }

    #[test]
    fn test_parser_and_eval_book_example2() {
        let line = "- (* 10 10) (+ 1 1 1)";
        assert_eq!(parse_and_eval(line).unwrap(), 97);
    }

    #[test]
    fn test_parser_and_eval_mod() {
        assert_eq!(parse_and_eval("% 10 6").unwrap(), 4);
    }

    #[test]
    fn test_parser_and_eval_pow() {
        assert_eq!(parse_and_eval("^ 4 2").unwrap(), 16);
        assert_eq!(parse_and_eval("^ 2 10").unwrap(), 1024);
    }

    #[test]
    fn test_parser_and_eval_min() {
        assert_eq!(parse_and_eval("min 1 5 3").unwrap(), 1);
        assert_eq!(parse_and_eval("min 2 10 0").unwrap(), 0);
        assert_eq!(parse_and_eval("min 300 314 314 69").unwrap(), 69);
    }

    #[test]
    fn test_parser_and_eval_max() {
        assert_eq!(parse_and_eval("max 1 5 3").unwrap(), 5);
        assert_eq!(parse_and_eval("max 2 10 0").unwrap(), 10);
        assert_eq!(parse_and_eval("max 300 314 314 69").unwrap(), 314);
    }
}
