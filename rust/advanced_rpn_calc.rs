use std::io::{self, BufRead};

fn main() {
  for line in io::stdin().lock().lines() {
    let operands: Vec<String> = line
      .unwrap()
      .trim()
      .split_whitespace()
      .map(|x| x.to_string())
      .collect();

    let mut stack: Vec<i64> = Vec::new();

    for op in operands.iter() {
      let op = op.as_str();

      if let Ok(i) = op.parse::<i64>() {
        stack.push(i);
				//println!("Stack count: {}", stack.len().to_string());
      } else {
        match op {
          "+" => {
            if stack.len() < 2 {
              panic!("Incorrect operands count on stack!");
            } else {
              let mut result: i64 = 0;
              for i in 0..stack.len() {
                let opd = stack.remove(0);
                if i == 0 {
                  result = opd;
                } else {
                  result += opd;
                }
              }

              stack.push(result);
            }
          }
          "-" => {
            if stack.len() < 2 {
              panic!("Incorrect operands count on stack!");
            } else {
              let mut result: i64 = 0;
              for i in 0..stack.len() {
                let opd = stack.remove(0);
                if i == 0 {
                  result = opd;
                } else {
                  result -= opd;
                }
              }

              stack.push(result);
            }
          }
          "*" => {
            if stack.len() < 2 {
              panic!("Incorrect operands count on stack!");
            } else {
              let mut result: i64 = 0;
              for i in 0..stack.len() {
                let opd = stack.remove(0);
                if i == 0 {
                  result = opd;
                } else {
                  result *= opd;
                }
              }

              stack.push(result);
            }
          }
          "/" => {
            if stack.len() < 2 {
              panic!("Incorrect operands count on stack!");
            } else {
              let mut result: i64 = 0;
              for i in 0..stack.len() { 
								let opd = stack.remove(0);
								if i == 0 {
									result = opd;
								} else {
									if opd != 0 {
										result /= opd;
									} else {
										panic!("Division by zero is forbidden!");
									}
								}
              }

              stack.push(result);
            }
          }
          "%" => {
            if stack.len() < 2 {
              panic!("Incorrect operands count on stack!");
            } else {
              let mut result: i64 = 0;
              for i in 0..stack.len() {
                let opd = stack.remove(0);
                if i == 0 {
                  result = opd;
                } else {
                  if opd != 0 {
                    result = result % opd;
                  } else {
                    panic!("Division by zero is forbidden!");
                  }
                }
              }

              stack.push(result);
            }
          }
					"!" => {
						if stack.len() < 1 {
							panic!("Incorrect operands count on stack!");
						} else {
							let mut mul = stack.remove(0);
							let mut result: i64 = mul;
							mul -= 1;
							for i in (1..(mul+1)).rev() {
								result *= i;
							}
							stack.push(result);
						}
					}
					"^" => {
						if stack.len() != 2 {
							panic!("Incorrect operands count on stack!");
						} else {
							let opd_a = stack.remove(0);
							let opd_b = stack.remove(0);
							let mut result: i64 = 0;

							for i in 0..opd_b {
								if i == 0 {
									result = opd_a;
								} else {
									result *= opd_a;
								}
							}

							stack.push(result);
						}
					}
          _ => {
            panic!("Correct operator nor operand provided!");
          }
				}
    	}
		}
    println!("{}", stack.pop().unwrap());
  }
}
