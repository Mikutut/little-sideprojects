use std::io::{self, BufRead};

fn main() {
  for line in io::stdin().lock().lines() {
    let mut keys: Vec<String> = line.unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();

    let _operator = keys.pop().unwrap();
    let operator = _operator
      .as_str();
    
    let mut operands: Vec<i64> = keys
      .iter()
      .map(|x| x.parse().unwrap())
      .collect();

    let mut result = operands[0];

    operands.remove(0);

    for op in operands.iter() {
      match operator {
        "+" => {
          result += op;
        }
        "-" => {
          result -= op;
        }
        "*" => {
          result *= op;
        }
        "/" => {
          result /= op;
        }
        "%" => {
          result = result % op;
        }
        _ => { panic!("Incorrect operator!"); }
      }
    }

    println!("{}", result);
  }
}
