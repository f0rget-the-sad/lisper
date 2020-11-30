pub fn eval(tokens: pest::iterators::Pairs) -> i64 {

    // expr = {number | "(" ~ operator ~ expr+ ~ ")"}
    // expr_list = _{ SOI ~ operator ~ expr+ ~ EOI}

    let mut op: char  = "";
    // TODO: since we using Polish notation
    // we could eval numbers directly to result
    let mut numbers: Vec<i64>;
    for pair in pairs {
        let token = pair.as_str();
        match pair.as_rule() {
            Rule::operator => {assrt_eq!(op, ""); op = token},
            Rule::number => {assert_eq!(pairs.len(), 1); result.push(eval_number(token))},
            Rule::expr => {result.push(eval(pair.into_inner()))},
            //TODO EOI
            _ => unreachable!(),
        };
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
    if numbers.len() == 1 {
        return eval_op(numbers[0]);
    }

    let result = numbers[0];
    for n in number {
        re
    }
    result
}

fn eval_expr()

fn eval_number(x: &str) -> i64 {
    x.parse::<i64>().unwrap()
}

fn eval_op(x: i64, op: &char, y: i64) -> i64{
    match op {
        '+' => return x + y,
        '-' => return x - y,
        '*' => return x * y,
        '/' => return x / y,
        _ => unreachable!(),
    }
}
