//Puzzle 19: Linen Layout
//Don't forget to bring a towel

use std::{ env, fmt, fs::File, io::{ self, BufRead }, path::Path, };

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

type StripedTowel = String;
type Design = String;

struct Towels {
    towel_patterns: Vec<StripedTowel>,
    desired_designs: Vec<Design>,
    possible_designs: Vec<Design>,
}

impl Towels {
    fn new() -> Self {
        Self {
            towel_patterns: Vec::new(),
            desired_designs: Vec::new(),
            possible_designs: Vec::new(),
        }
    }

    fn load_from_file(&mut self, file_path: &String) {
        let file = read_lines(file_path);
        
        if let Ok(lines) = file {
           for line in lines {
                if line.as_ref().unwrap().contains(","){
                    //it is a towel pattern
                    for towel in line.unwrap().split(",") {
                        self.towel_patterns.push(towel.trim().to_string());
                    }
                    //re order them from largest to smallest
                    self.towel_patterns.sort_by(
                        |a , b|  a.len().cmp(&b.len()).reverse());
                } else if !line.as_ref().unwrap().is_empty() {
                    //it is a desired design
                    self.desired_designs.push(line.unwrap().trim().to_string());
                }
           } 
        }
    }

    fn number_of_possible_designs(&mut self) {
        for mut desired_design in self.desired_designs.clone() {
            let mut possible_design = String::new();
            let mut last_possible_design = String::new();
            while !desired_design.is_empty(){ 
                for towel in self.towel_patterns.clone() {
                    match desired_design.find(&towel) {
                        Some(position) => {
                            possible_design.push_str(desired_design.drain(position..position+towel.len()).as_str());
                        }
                        None => {
                            ()
                        }
                    }
                }
                if last_possible_design != possible_design {
                    last_possible_design = possible_design.clone();
                } else {
                    break;
                }
            }
            if desired_design.trim().to_string().is_empty() {
                self.possible_designs.push(possible_design);
            }
        }
    }

}

impl fmt::Display for Towels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //I think per idiom, that these are suppose to be for loops since they writing as a side effect
        let _ = writeln!(f, "Available Towel Patterns:");
        let _ = self.towel_patterns.iter().map(|x| write!(f, "{} ", x)).collect::<Vec<_>>();
        let _ = writeln!(f, "\nAvailable Towel Patterns Count: {}", self.towel_patterns.len());
        let _ = writeln!(f,"Desired Designs:");
        let _ = self.desired_designs.iter().map(|x| writeln!(f, "{}", x)).collect::<Vec<_>>();
        let _ = writeln!(f,"Possible Designs:");
        let _ = self.possible_designs.iter().map(|x| writeln!(f,"{}", x)).collect::<Vec<_>>();
        let _ = writeln!(f,"Number of Possible Designs: {}", self.possible_designs.len());
        Ok(())
    }
}

fn main() {
    
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    
    let mut towels = Towels::new();

    towels.load_from_file(file_path);

    towels.number_of_possible_designs();

    println!("{}", towels);

}