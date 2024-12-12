//this is so gangly! it needs to be refactored. Once again, this was done in ignorance and speed
use std::{ env, fs::File, io::{ self, BufRead }, ops::Shr, path::Path };

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Equation {
    output: u64,
    operands: Vec<u64>,
    operators: Vec<Vec<u32>>,
    results: Vec<u64>,
}

fn to_bit_array(word: u32) -> Vec<u32> {
    let mut bits = Vec::new();
    
    //convert byte to bit array
    for i in 0..32 as u32 {
        bits.push(word.shr(i) & 1);
    }
    bits.reverse();

    bits
}

fn strip_leading_zeros(word: u32, bit_array: Vec<u32>) -> Vec<u32> {
    let bits_unwanted =  word.leading_zeros() as usize + 1;
    let bits_wanted = bit_array.iter().skip(bits_unwanted).map(|&x| x).collect();
    bits_wanted
}

impl Equation {
    fn new() -> Self {
        Self {
            output: 0,
            operands: Vec::new(),
            operators: Vec::new(),
            results: Vec::new(),
        }
    }
    
    //the big offender... this ugly!
    fn calculate_possible_calibration_results(&mut self) -> Vec<u64> {
        // mul is 0 and add is 1
        let mut results = Vec::new();
        let operators:Vec<Vec<u32>> = self.operators.iter().map(|operator_stack| operator_stack.iter().map(|&operator| operator).collect::<Vec<u32>>()).collect();
        for operator_stack in operators  {
            let mut index = 0;
            let mut result = *self.operands.iter().nth(0).unwrap();
            for op in operator_stack{
            //let mut result = self.operands[index];
                    if op == 0 {
                        result = result * self.operands[index + 1];
                    } else {
                        result = result + self.operands[index + 1];
                    }

                    index += 1;
                    let operands = &self.operands;
                    if index >= operands.iter().count() - 1 {
                        break;
                    }            
                }
            results.push(result);
        }
        results
    }
    
    fn create_operator_stack(&mut self) -> Vec<Vec<u32>> {
        // the number of combinations is 2^n (like binary)
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
        //9*7*18*13	    000
        //9*7*18+13 	001
        //9*7+18*13 	010
        //9*7+18+13 	011
        //9+7*18*13 	100
        //9+7*18+13 	101
        //9+7+18*13	    110
        //9+7+18+13	    111        
        let mut operators_stack = Vec::new();
        let number_of_operators = (self.operands.iter().count() - 1) as u32;
        let number_of_combinations = 2u32.pow(number_of_operators as u32);
        for n in 0..number_of_combinations {
            let bit_array = to_bit_array(n);
            let stripped_bit_array = strip_leading_zeros(number_of_combinations, bit_array);
            operators_stack.push(stripped_bit_array);
        }
        operators_stack 
    }

    fn return_valid_result(&self) -> Option<u64> {
        self.results.iter().find(|result| **result == self.output).copied()
    }

//    fn answer(&self, valid_results: Vec<&u64>) {
//        valid_results.
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

fn answer(equations: Vec<Equation>) -> u64 {
    let mut answer = 0;
    for mut equation in equations {
        equation.operators = equation.create_operator_stack();
        equation.results = equation.calculate_possible_calibration_results();
        answer += match equation.return_valid_result() {
            Some(i) => i,
            None => 0,
        };
    }
    answer
}

fn main() {
    
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    
    let equations = txt_file_to_equations(&file_path);

    println!("The answer is: {}", answer(equations));

}