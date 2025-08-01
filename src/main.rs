use std::collections::HashMap;

fn main() {
    let ex: String = String::from("( ( 9 + 3 ) * 2 - 6 )");



    let postfix: String = convert_to_postfix(&ex);
    let evaluated_rpn: f64 = evaluate_rpn(&postfix);
    println!("{postfix}");
    println!("Final result: {evaluated_rpn}");
}

fn convert_to_postfix(infix_expression: &String) -> String {

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
        
        match token.trim().parse::<u32>() {
            Ok(_n) => {
                output.push(token);
                println!("Pushing to output: {token} | {output:?}")
            },
            Err(_err) => {
                
                if token == "(" {
                    stack.push(token);
                    println!("Pushing to stack sub: {token} | {stack:?}")
                } else if token == ")" {
                    while !stack.is_empty() && stack.last().unwrap() != &"(" {
                        output.push(stack.pop().unwrap());
                        println!("Pushing to output sub: {token} | {output:?} |           | {stack:?}")
                    }
                    stack.pop(); // pop last parenthesis
                    println!("Pushing to output sub popped: {token} | {stack:?}")
                } else {
                    if !stack.is_empty() && stack.last().unwrap() == &"(" {
                        stack.push(token);
                        println!("Pushing to stack: {token} | {stack:?}");
                    } else {
                        loop {
                            let last = stack.last().unwrap();
                            if stack.is_empty() || last == &"(" {
                                break;
                            }
                            if get_priority(&priorities, last) >= get_priority(&priorities, &token) {
                            output.push(stack.pop().unwrap());
                            println!("Pushing to ouput the stack pop: {token} | {output:?}");
                            }
                        }
                        stack.push(token);
                        println!("Pushing to stack: {token} | {stack:?}");

                    }

      
                }
            }
        }
    };

    while !stack.is_empty() {
        output.push(stack.pop().unwrap());
    }


    output
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(" ")

}

fn get_priority<'a>(operators: &'a HashMap<&'a str, u8>, key: &'a str) -> &'a u8 {
    operators.get(&key).expect(&format!("Key does not exist: {}", key))
}

// Reverse Polish Notation
fn evaluate_rpn(expression: &String) -> f64 {
    let list: Vec<&str> = expression.split_whitespace().collect();
    
    let mut stack: Vec<f64> = Vec::new();

    for s in list {
        match s.trim().parse::<f64>() {
            Ok(n) => {
                stack.push(n);
                println!("Num: {}", n);
            },
            Err(_err) => {
                let b: f64 = stack.pop().expect("there should be two");
                let a: f64 = stack.pop().expect("there should be two");

                println!("Ope: {}", s);
                match s {
                    "+" => {println!("Add {a} + {b}");stack.push(a + b);},
                    "-" => {println!("Sub {a} - {b}");stack.push(a - b);},
                    "*" => {println!("Mul {a} * {b}");stack.push(a * b);},
                    "/" => {println!("Div {a} / {b}");stack.push(a / b);},
                    "^" => {println!("Pow {a} ^ {b}");stack.push(a.powf(b));},
                    _ => {}
                }
            }
        }
    }

    stack.pop().unwrap()
}

