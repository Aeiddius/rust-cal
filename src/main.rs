use std::collections::HashMap;

fn main() {
    let ex: String = String::from("( ( 12 * ( 3 + 5 ) ) - ( 8 / 2 ) + ( 7 - 4 ) ) / ( 6 - ( 1 + 2 ) ) + ( ( 5 + 5 ) * 2 )");

    let postfix: Vec<&str> = convert_to_postfix(&ex);
    let evaluated_rpn: f64 = evaluate_rpn(postfix);
    println!("Final result: {evaluated_rpn}");
}


fn get_priority(operators: &HashMap<&str, u8>, key: &str) -> u8 {
    *operators
        .get(key)
        .expect(&format!("Key does not exist: {}", key))
}

fn convert_to_postfix(infix_expression: &str) -> Vec<&str> {

    let priorities: HashMap<&str, u8> = HashMap::from([
        ("+", 1),
        ("-", 1),
        ("*", 2),
        ("/", 2),
        ("^", 3),
    ]);

    let mut output: Vec<&str> = Vec::new();
    let mut stack: Vec<&str> = Vec::new();

    let tokens: Vec<&str> = infix_expression.split_whitespace().collect();
    for token in tokens {
        
        if token.parse::<f64>().is_ok() {
            output.push(token);
        } else if token == "(" {
            stack.push(token);
        } else if token == ")" {

            while let Some(&top) = stack.last() {
                if top == "(" {
                    break;
                }
                output.push(stack.pop().unwrap());
            }
            stack.pop(); // pop last parenthesis
        } else {
            while let Some(&top) = stack.last() {
                if top == "(" {
                    break;
                }
                let top_prec: u8 = get_priority(&priorities, top);
                let cur_prec: u8 = get_priority(&priorities, token);
                if top_prec >= cur_prec {
                    output.push(stack.pop().unwrap());
                } else {
                    break;
                }
            }
            stack.push(token);
        }
        
    };

    while !stack.is_empty() {
        output.push(stack.pop().unwrap());
    }


    output

}


// Reverse Polish Notation
fn evaluate_rpn(tokens: Vec<&str>) -> f64 {

    let mut stack: Vec<f64> = Vec::new();

    for token in tokens {
        if let Ok(n) = token.parse::<f64>() {
            stack.push(n);
        } else {
            let b: f64 = stack.pop().expect("Missing operand");
            let a: f64 = stack.pop().expect("Missing operand");
            let res: f64 = match token {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                "^" => a.powf(b),
                _ => panic!("Unknown operator {}", token),
            };
            stack.push(res);
        }
    }
    stack.pop().unwrap()
}

