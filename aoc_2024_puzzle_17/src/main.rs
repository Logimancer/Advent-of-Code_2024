//Puzzle 17: Chronospatial Computer, a 3-bit machine
use std::{ env, fs::File, io::{ self, BufRead }, path::Path };

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(PartialEq)]
enum State {
    Running,
    Stopped,
}

struct ChronospatialComputer {
   cpu: Cpu,
   ram: Ram,
   output: Vec<String>,
   state: State,
}
impl ChronospatialComputer {
    fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            ram: Ram::new(),
            output: Vec::new(),
            state: State::Stopped,
        }
    }

    fn initialize_from_file(&mut self, initialization_file: &String) {
        let lines = read_lines(initialization_file).unwrap();
        for line in lines {
            match line {
                Ok(line) => {
                    let line_values: Vec<&str> = line.split(": ").collect();
                    match line_values[0] {
                        "Register A" => self.cpu.a_register = line_values[1].parse().unwrap(),
                        "Register B" => self.cpu.b_register = line_values[1].parse().unwrap(),
                        "Register C" => self.cpu.c_register = line_values[1].parse().unwrap(),
                        "Program" => self.ram.contents = line_values[1].split(",").map(|x| x.parse().unwrap()).collect::<Vec<u64>>(),
                        _ => (),
                    }
                },
                Err(_e) => (),
            };
        }
    }

    fn run(&mut self) {
        self.state = State::Running;
    }

    fn halt(&mut self){
        self.state = State::Stopped;
    }

    fn cycle(&mut self) {
        if self.cpu.instruction_pointer as usize >= self.ram.contents.len() {
            self.halt();
        } else {
            let current_opcode = self.ram.contents[self.cpu.instruction_pointer as usize];
            let current_operand = self.ram.contents[self.cpu.instruction_pointer as usize + 1];
            match Instructions::opcode_to_mnumonic(current_opcode).unwrap() {
                Instructions::adv => self.cpu.adv(current_operand),
                Instructions::bxl => self.cpu.bxl(current_operand),
                Instructions::bst => self.cpu.bst(current_operand),
                Instructions::jnz => self.cpu.jnz(current_operand),
                Instructions::bxc => self.cpu.bxc(current_operand),
                Instructions::out => self.output.push(self.cpu.out(current_operand)),
                Instructions::bdv => self.cpu.bdv(current_operand),
                Instructions::cdv => self.cpu.cdv(current_operand),               
            }
            self.state = State::Running;
        }

    }

    fn print_output(&self){
        let output = self.output.iter().map(|out| out.to_owned() + &",").collect::<Vec<String>>();

        let mut output_string = String::new();
        for string in output {
            output_string = output_string.clone() + string.as_str();
        }

        if output_string.ends_with(",") {
            output_string.drain(output_string.len() - 1 ..);
        }
        println!("Output: {}", output_string);
    }

    fn print_state(&self) {
        println!("--------------------------------------------------------------");
        println!("Internal State of Computer");
        println!("Cpu:");
        println!("Register A: {}", self.cpu.a_register);
        println!("Register B: {}", self.cpu.b_register);
        println!("Register C: {}", self.cpu.c_register);
        println!("Instuction Pointer: {}", self.cpu.instruction_pointer);
        println!("Program: ");
        for element in &self.ram.contents {
            print!("{},", element)
        }
        println!("");
        println!("**************************************************************");
    }
}
struct Cpu {
    a_register: u64,
    b_register: u64,
    c_register: u64,
    instruction_pointer: u64,
}

impl Cpu {
    fn new() -> Self {
        Self {
            a_register: 0,
            b_register: 0,
            c_register: 0,
            instruction_pointer: 0,
        }
    }

    fn combo_operand(&self, operand: u64) -> Result<u64, String> {
        match operand {
            0u64..=3u64 => Ok(operand),
            4 => Ok(self.a_register),
            5 => Ok(self.b_register),
            6 => Ok(self.c_register),
            _ => Err(format!("Invalid Combo Operator")),
        }
    }
    
    fn adv(&mut self, operand: u64) {
    //The adv instruction (opcode 0) performs division. 
    //The numerator(dividend) is the value in the A register. 
    //The denominator(divisor) is found by raising 2 to the power of the instruction's combo operand. 
    //(So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) 
    //The result of the division operation is truncated to an integer and then written to the A register.
//    println!("adv, operand: {}. ", operand);
        let dividend = self.a_register;
        let divisor = 2u64.pow(self.combo_operand(operand).unwrap().try_into().unwrap());
        let quotient = dividend / divisor;
        self.a_register = quotient;

        self.instruction_pointer += 2;
    }
    
    fn bxl(&mut self, operand: u64) {
    //The bxl instruction (opcode 1) 
    //calculates the bitwise XOR of register B and the instruction's literal operand, 
    //then stores the result in register B.
//        println!("bxl, operand: {}", operand);
        self.b_register = self.b_register ^ operand;

        self.instruction_pointer += 2;
    }
    
    fn bst(&mut self, operand: u64) {
    //The bst instruction (opcode 2) 
    //calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), 
    //then writes that value to the B register.
//        println!("bst, operand: {}", operand);
        self.b_register = self.combo_operand(operand).unwrap() % 8;
        
        self.instruction_pointer += 2;
    }
    
    fn jnz(&mut self, operand: u64) {
    //The jnz instruction (opcode 3) 
    //does nothing if the A register is 0. 
    //However, if the A register is not zero, 
    //it jumps by setting the instruction pointer to the value of its literal operand; 
    //if this instruction jumps, 
    //the instruction pointer is not increased by 2 after this instruction.
//        println!("jnz, operand: {}", operand);
        if self.a_register != 0 {
            self.instruction_pointer = operand;
        } else { 
            self.instruction_pointer += 2;
        }
    }
    
    fn bxc(&mut self, operand: u64) {
    //The bxc instruction (opcode 4) 
    //calculates the bitwise XOR of register B and register C, 
    //then stores the result in register B. 
    //(For legacy reasons, this instruction reads an operand but ignores it.)
//        println!("bxc, operand: {}", operand);
        let _ignore = operand;
        self.b_register = self.b_register ^ self.c_register;

        self.instruction_pointer += 2;
    }
    
    fn out(&mut self, operand: u64) -> String {
    //The out instruction (opcode 5) 
    //calculates the value of its combo operand modulo 8, 
    //then outputs that value. 
    //(If a program outputs multiple values, they are separated by commas.)
//        println!("out, operand: {}", operand);
//        println!("OUT: {}", self.combo_operand(operand).unwrap() % 8);
        
        self.instruction_pointer += 2;
        format!("{}", self.combo_operand(operand).unwrap() % 8)
    }
    
    fn bdv(&mut self, operand: u64) {
    //The bdv instruction (opcode 6) 
    //works exactly like the adv instruction 
    //except that the result is stored in the B register. 
    //(The numerator is still read from the A register.)
//        println!("bdv, operand: {}", operand);
        let dividend = self.a_register;
        let divisor = 2u64.pow(self.combo_operand(operand).unwrap().try_into().unwrap());
        let quotient = dividend / divisor;
        self.b_register = quotient;

        self.instruction_pointer += 2;
    }
    
    fn cdv(&mut self, operand: u64) {
    //The cdv instruction (opcode 7) 
    //works exactly like the adv instruction 
    //except that the result is stored in the C register. 
    //(The numerator is still read from the A register.)
//        println!("cdv, operand: {}", operand);
        let dividend = self.a_register;
        let divisor = 2u64.pow(self.combo_operand(operand).unwrap().try_into().unwrap());
        let quotient = dividend / divisor;
        self.c_register = quotient;

        self.instruction_pointer += 2;
    }

}

struct Ram {
    contents: Vec<u64>,
}

impl Ram {
    fn new() -> Self {
        Self {
            contents: Vec::new(),
        }
    }

}
enum Instructions {
    adv,
    bxl,
    bst,
    jnz,
    bxc,
    out,
    bdv,
    cdv,
}

impl Instructions {
    fn mnumonic_to_opcode(&self) -> u64 {
        match *self {
            Instructions::adv => 0, 
            Instructions::bxl => 1,
            Instructions::bst => 2,
            Instructions::jnz => 3,
            Instructions::bxc => 4,
            Instructions::out => 5,
            Instructions::bdv => 6,
            Instructions::cdv => 7,
        }
    }

    fn opcode_to_mnumonic(opcode: u64) -> Result<Instructions, String> {
        match opcode {
            0 => Ok(Instructions::adv),
            1 => Ok(Instructions::bxl),
            2 => Ok(Instructions::bst),
            3 => Ok(Instructions::jnz),
            4 => Ok(Instructions::bxc),
            5 => Ok(Instructions::out),
            6 => Ok(Instructions::bdv),
            7 => Ok(Instructions::cdv),
            _ => Err(format!("Error: {} is out of range for an opcode. Opcodes can only be between 0 and 7", opcode))
        }
    }
}

fn main() {
    
    let args: Vec<String> = env::args().collect();

    let initial_state_file_path = &args[1];

    let mut chronopatial_computer = ChronospatialComputer::new();

    chronopatial_computer.initialize_from_file(initial_state_file_path);
    
    chronopatial_computer.print_state();
    
    chronopatial_computer.run();

    while chronopatial_computer.state == State::Running {
        chronopatial_computer.cycle();
    }

    chronopatial_computer.print_output();

    println!("Computer Halted");
}