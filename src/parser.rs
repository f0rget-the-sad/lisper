use pest::iterators::Pairs;
use pest::Parser;

use crate::error::LiResult;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

use crate::evaluator;

#[derive(Debug, Clone)]
pub enum Type {
    Number(i64),
    Symbol(String),
    // TODO: use VecDec, op require pop_front?
    //Expr(Vec<Type>),
    Sexpr(Vec<Type>),
    Qexpr(Vec<Type>),
}

pub fn parse_and_eval(line: &str) -> LiResult<i64> {
    let pairs = line_to_pairs(line)?;
    dbg!(pairs.clone());
    let ast = lval_read(pairs)?;
    dbg!(ast.clone());
    match evaluator::eval(ast)? {
        Type::Number(n) => Ok(n),
        _ => unreachable!(),
    }
}

fn line_to_pairs(line: &str) -> LiResult<Pairs<Rule>> {
    Ok(GrammarParser::parse(Rule::expr_list, line)?)
}

fn lval_read(pairs: Pairs<Rule>) -> LiResult<Type> {
    let mut exprs = vec![];
    for pair in pairs {
        let token = pair.as_str();
        match pair.as_rule() {
            //TODO: will it always be first
            Rule::symbol => exprs.push(Type::Symbol(token.to_string())),
            Rule::number => exprs.push(Type::Number(token.parse::<i64>().unwrap())),
            Rule::expr | Rule::sexpr => {
                exprs.push(lval_read(pair.into_inner())?);
            }
            Rule::qexpr => {
                match lval_read(pair.into_inner())? {
                    // TODO: is this unwrapping really needed?
                    Type::Sexpr(v) => return Ok(Type::Qexpr(v)),
                    v => return Ok(Type::Qexpr(vec![v])),
                }
            }
            Rule::EOI => {
                //TODO EOI: why it's not striped out as new line?
            }
            _ => {
                dbg!(pair);
                unreachable!()
            }
        };
    }

    if exprs.len() == 1 {
        return Ok(exprs.pop().unwrap());
    }

    Ok(Type::Sexpr(exprs))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::LisperError;

    #[test]
    fn test_parser_ok() {
        let line = "+ 5 (* 2 2)";
        let result = line_to_pairs(line);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_gt_ten() {
        let line = "+ 10 (* 2 2)";
        let result = line_to_pairs(line);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_err() {
        let line = "fuck+ 5 (* 2 2)";
        let result = line_to_pairs(line);
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
