use std::{ env, error::Error, fs::File, io::{ self, BufRead }, ops::Index, path::Path };

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Equation {
    output: u64,
    operands: Vec<u64>
}

impl Equation {
    fn new() -> Self {
        Self {
            output: 0,
            operands: Vec::new(),
        }
    }
}

fn txt_file_to_equations(file_path: &String) -> Vec<Equation> {
    let mut equations = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            let mut equation = Equation::new();
            
            //get lhs and store
            equation.output = line.split(":").nth(0).map(|x| x.parse().unwrap()).unwrap();
            
            //get rhs and store
            equation.operands = line.split(":").skip(1).map(|x| x.split_whitespace().map(|x| x.parse().unwrap()).collect()).collect();
            
            equations.push(equation);
        };
    }
    equations
}

fn main() {
    
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    
    let equations = txt_file_to_equations(file_path);

    println!("{:?}", equations);

}