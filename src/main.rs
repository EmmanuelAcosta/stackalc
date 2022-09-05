use std::io;
use std::io::BufRead;

fn main() {
    
    let lines = io::stdin().lock().lines();
    let mut vector: Vec<f64> = vec![];
    //Ten NaN values are added to the new vector variables
    let mut variables: Vec<f64> = vec![std::f64::NAN; 10];
     for line in lines {
        //Save each line into vector
        let line_str = line.unwrap();




        // split the line into floats
        for instruccion in line_str.split_whitespace() {
            let vectorInstruccion = instruccion.split(":").collect::<Vec<&str>>();
            match vectorInstruccion[..]{
                ["ADD"] => {
                    let a = vector.pop().unwrap();
                    let b = vector.pop().unwrap();
                    vector.push(a+b);
                },
                ["SUB"] => {
                    let a = vector.pop().unwrap();
                    let b = vector.pop().unwrap();
                    vector.push(b-a);
                },
                ["MULT"] => {
                    let a = vector.pop().unwrap();
                    let b = vector.pop().unwrap();
                    vector.push(a*b);
                },
                ["DIV"] => {
                    let a = vector.pop().unwrap();
                    let b = vector.pop().unwrap();
                    vector.push(b/a);
                },
                // if AND is found, pop the last two elements, 0 and NaN are false, everything else is true, if true push greater value, else push 0
                ["AND"] => {
                    let a = vector.pop().unwrap();
                    let b = vector.pop().unwrap();
                    if a != 0.0 && b != 0.0 && !a.is_nan() && !b.is_nan() {
                        if a > b {
                            vector.push(a);
                        } else {
                            vector.push(b);
                        }
                    } else {
                        vector.push(0.0);
                    }
                },
                // if OR is found, pop the last two elements, 0 and NaN are false, everything else is true, if true push smaller value, else push 0
                ["OR"] => {
                    let a = vector.pop().unwrap();
                    let b = vector.pop().unwrap();
                    if a != 0.0 && b != 0.0 && !a.is_nan() && !b.is_nan() {
                        if a < b {
                            vector.push(a);
                        } else {
                            vector.push(b);
                        }
                    } else {
                        vector.push(0.0);
                    }
                },
                // if NOT is found, pop the last element, 0 and NaN are false, everything else is true, if true push 0, else push 1
                ["NOT"] => {
                    let a = vector.pop().unwrap();
                    if a != 0.0 && !a.is_nan() {
                        vector.push(0.0);
                    } else {
                        vector.push(1.0);
                    }
                },
                // if EQ is found, pop the last two elements, if equal push 1, else push 0
                ["EQ"] => {
                    let a = vector.pop().unwrap();
                    let b = vector.pop().unwrap();
                    if a == b {
                        vector.push(1.0);
                    } else {
                        vector.push(0.0);
                    }
                },
                // if GT is found, pop the last two elements, if a > b push 1, else push 0
                ["GT"] => {
                    let a = vector.pop().unwrap();
                    let b = vector.pop().unwrap();
                    if a > b {
                        vector.push(1.0);
                    } else {
                        vector.push(0.0);
                    }
                },
                // if LT is found, pop the last two elements, if a < b push 1, else push 0
                ["LT"] => {
                    let a = vector.pop().unwrap();
                    let b = vector.pop().unwrap();
                    if a < b {
                        vector.push(1.0);
                    } else {
                        vector.push(0.0);
                    }
                },
                // if GTE is found, pop the last two elements, if a >= b push 1, else push 0
                ["GTE"] => {
                    let a = vector.pop().unwrap();
                    let b = vector.pop().unwrap();
                    if a >= b {
                        vector.push(1.0);
                    } else {
                        vector.push(0.0);
                    }
                }, 
                // if LTE is found, pop the last two elements, if a <= b push 1, else push 0
                ["LTE"] => {
                    let a = vector.pop().unwrap();
                    let b = vector.pop().unwrap();
                    if a <= b {
                        vector.push(1.0);
                    } else {
                        vector.push(0.0);
                    }
                },
                // if DIFF is found, pop the last two elements, if a != b push 1, else push 0
                ["DIFF"] => {
                    let a = vector.pop().unwrap();
                    let b = vector.pop().unwrap();
                    if a != b {
                        vector.push(1.0);
                    } else {
                        vector.push(0.0);
                    }
                },
                // if SET:n is found, the case is SET and the rest, pop the top element and save it in the i position of the variables vector
                ["SET",n] => {
                    let a = vector.pop().unwrap();
                    let b = n.parse::<usize>().unwrap();
                    variables[b] = a;
                },
                // if GET:n is found, the case is GET:_ and the rest, push the i position of the variables vector
                ["GET",n] => {
                    let a = n.parse::<usize>().unwrap();
                    vector.push(variables[a]);
                },

                // if DUP is found, pop the top element and push it twice
                ["DUP"] => {
                    let a = vector.pop().unwrap();
                    vector.push(a);
                    vector.push(a);
                },
                // if POP is found, pop the top element
                ["POP"] => {
                    vector.pop();
                },
                _ => {
                    let a = instruccion.parse::<f64>().unwrap();
                    vector.push(a);
                }
            }
        }
        print!("{vector:?}");
        //print variables vector with the values i=variables[i]
        print!("|");
        for i in 0..variables.len() {
            print!(" {}={} ",i,variables[i]);
        };
        println!("");
        
    } 
}


