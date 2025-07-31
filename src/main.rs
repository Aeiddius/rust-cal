use std::collections::HashMap;

fn main() {
    let ex: String = String::from("4 / 2 + ( 7 * 3 )");



    let postfix: String = convert_to_postfix(&ex);
    let evaluated_rpn: u32 = evaluate_rpn(&postfix);
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
            },
            Err(_err) => {
                
                if token == "(" {
                    stack.push(token);
                    println!("hello world");
                } else if token == ")" {
                    while !stack.is_empty() && stack.last().unwrap() != &"(" {
                        output.push(stack.pop().unwrap());
                    }
                    stack.pop(); // pop last parenthesis
                    continue;
                } else {
                    while !stack.is_empty() && get_priority(&priorities, stack.last().unwrap()) >= get_priority( &priorities,&token) {
                        output.push(stack.pop().unwrap());
                    }
                    // println!("{token}");
                    stack.push(token);
                }
            }
        }
    };

    while !stack.is_empty() {
        output.push(stack.pop().unwrap());
    }

    println!("{output:?}");
    println!("{stack:?}");
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
fn evaluate_rpn(expression: &String) -> u32 {
    let list: Vec<&str> = expression.split_whitespace().collect();
    
    let mut stack: Vec<u32> = Vec::new();

    for s in list {
        match s.trim().parse::<u32>() {
            Ok(n) => {
                stack.push(n);
                println!("Num: {}", n);
            },
            Err(_err) => {
                let b = stack.pop().expect("there should be two");
                let a = stack.pop().expect("there should be two");

                println!("Ope: {}", s);
                match s {
                    "+" => {println!("Add {a} + {b}");stack.push(a + b);},
                    "-" => {println!("Sub {a} - {b}");stack.push(a - b);},
                    "*" => {println!("Mul {a} * {b}");stack.push(a * b);},
                    "/" => {println!("Div {a} / {b}");stack.push(a / b);},
                    "^" => {println!("Pow {a} ^ {b}");stack.push(a.pow(b));},
                    _ => {}
                }
            }
        }
    }

    stack.pop().unwrap()
}

