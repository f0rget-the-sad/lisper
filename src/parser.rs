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
    fn test_parser_and_eval_max_pow() {
        let line = "max (^ 2 3) (^ 3 2)";
        assert_eq!(parse(line).unwrap(), 9);
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

    #[test]
    fn test_parser_and_eval_mod() {
        assert_eq!(parse("% 10 6").unwrap(), 4);
    }

    #[test]
    fn test_parser_and_eval_pow() {
        assert_eq!(parse("^ 4 2").unwrap(), 16);
        assert_eq!(parse("^ 2 10").unwrap(), 1024);
    }

    #[test]
    fn test_parser_and_eval_min() {
        assert_eq!(parse("min 1 5 3").unwrap(), 1);
        assert_eq!(parse("min 2 10 0").unwrap(), 0);
        assert_eq!(parse("min 300 314 314 69").unwrap(), 69);
    }

    #[test]
    fn test_parser_and_eval_max() {
        assert_eq!(parse("max 1 5 3").unwrap(), 5);
        assert_eq!(parse("max 2 10 0").unwrap(), 10);
        assert_eq!(parse("max 300 314 314 69").unwrap(), 314);
    }
}
