use std::{ env, error::Error, fs::File, io::{ self, BufRead }, ops::Index, path::Path };

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Equation {
    output: u64,
    operands: Vec<u64>,
    operators: Vec<char>,
}

impl Equation {
    fn new() -> Self {
        Self {
            output: 0,
            operands: Vec::new(),
            operators: Vec::new(),
        }
    }

    // mul is "*" and add is "+"
    fn create_operator_stack(&mut self) {
        let number_or_operators = self.operands.iter().count() - 1;
        for n in 1..number_or_operators{
            self.operators.push('+');
            self.operators.push('*');
        }
    }

//number of operators - 1 to binary and then you have all the combos
//10 19 =
//10*19		00
//10+19		01
//
//81 40 27 =
//81*40*27	00
//81+40*27	01
//81*40+27	10	
//81+40+27	11
//
//9 7 18 13 =
//9*7*18*13	000
//9*7*18+13 	001
//9*7+18*13 	010
//9*7+18+13 	011
//9+7*18*13 	100
//9+7*18+13 	101
//9+7+18*13	110
//9+7+18+13	111

//    fn calibration_result(&self) -> Option<u64> {
//        let mut result: u64 = 0;
//        for operand in self.operands {
//
//        }
//    }
}

fn txt_file_to_equations(file_path: &String) -> Vec<Equation> {
    let mut equations = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            let mut equation = Equation::new();
            
            //get lhs and store
            equation.output = line.split(":").nth(0).map(|x| x.parse().unwrap()).unwrap();
            
            //get rhs and store
            equation.operands = line.split(":")
                                               .skip(1)
                                               .map(|x| x.split_whitespace()
                                                               .map(|x| x.parse()
                                                                               .unwrap()))
                                               .flatten()
                                               .collect();
            equations.push(equation);
            }
        };
    equations
}

fn main() {
    
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    
    let equations = txt_file_to_equations(file_path);

    println!("{:?}", equations);
}