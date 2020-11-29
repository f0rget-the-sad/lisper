use pest::error::Error;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct GrammarParser;

pub fn parse(line: &str) -> Result<&str, Error<Rule>> {
    let pairs = GrammarParser::parse(Rule::expr_list, line)?;

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        /*
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::alpha => println!("Letter:  {}", inner_pair.as_str()),
                Rule::digit => println!("Digit:   {}", inner_pair.as_str()),
                _ => unreachable!(),
            };
        }
        */
        dbg!(pair);
    }
    Ok(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_ok() {
        let line = "+ 5 (* 2 2)";
        let result = parse(line);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), line);
    }

    #[test]
    fn test_parser_err() {
        let line = "fuck+ 5 (* 2 2)";
        let result = parse(line);
        assert!(!result.is_ok());
    }
}
