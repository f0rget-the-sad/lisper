use pest::error::Error;
use pest::iterators::Pairs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

use crate::evaluator;

pub fn parse(line: &str) -> Result<i64, Error<Rule>> {
    Ok(evaluator::eval(parse_expr_list(line)?))
}

fn parse_expr_list(line: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    GrammarParser::parse(Rule::expr_list, line)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(parse(line).unwrap(), 1);
    }

    #[test]
    fn test_parser_and_eval_division() {
        let line = "/ 121 11";
        assert_eq!(parse(line).unwrap(), 11);
    }

    #[test]
    fn test_parser_and_eval_long() {
        let line = "+ 1 2 1 4";
        assert_eq!(parse(line).unwrap(), 8);
    }

    #[test]
    fn test_parser_and_eval_book_example1() {
        let line = "* 10 (+ 1 51)";
        assert_eq!(parse(line).unwrap(), 520);
    }

    #[test]
    fn test_parser_and_eval_book_example2() {
        let line = "- (* 10 10) (+ 1 1 1)";
        assert_eq!(parse(line).unwrap(), 97);
    }
}
