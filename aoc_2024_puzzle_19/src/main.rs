//Puzzle 19: Linen Layout

use std::{ env, fmt, fs::File, io::{ self, BufRead }, path::Path };

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct DummyStruct {
    dummy_id: u64,
    data_in_struct: Vec<Vec<u64>>,
}

impl DummyStruct {
    fn new() -> Self {
        Self {
            dummy_id: 0,
            data_in_struct: Vec::new(),
        }
    }

    fn dummy_file_load(&mut self, file_path: &String) {
        let file = read_lines(file_path);
        //this gets a collection of number strings that are seperated by commas and stores them to 
        // a Vec<Vec<u64> in the dummy stuct
        // The outer vec is a collection of the lines and the 
        //inner vec is a collection of numbers seperated by commas on said line
        
        if let Ok(lines) = file {
            self.data_in_struct = lines.map(|x| x.unwrap()
                                                                             .split(",")
                                                                             .map(|y| y.parse::<u64>()
                                                                                             .unwrap())
                                                                             .collect::<Vec<u64>>()
                                                )
                                            .collect();
            
        }
    }
}

impl fmt::Display for DummyStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Id {}", self.dummy_id)
    }
}

fn main() {
    
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    
    let mut dummy_instance = DummyStruct::new();

    dummy_instance.dummy_file_load(file_path);

    println!("Dummy ID {}", dummy_instance);

    for set in dummy_instance.data_in_struct {
        for item in set {
            print!("{} ", item)
        }
        println!("")
    }



}