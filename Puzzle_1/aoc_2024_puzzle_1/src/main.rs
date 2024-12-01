use std::{ env, fs::File, io::{ self, BufRead }, path::Path };

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct List<T> {
    column_1: Vec<T>,
    column_2: Vec<T>, 
}

impl<T> List<T> {
    fn new() -> Self {
        Self { column_1: Vec::new(),
               column_2: Vec::new(),
        }
    }
}

impl List<i64>{
    fn process_txt_file(&mut self, file_path: &String) {
        if let Ok(lines) = read_lines(file_path) {
            for line in lines.flatten() {
                //would have used collect to tupple in itertools, but wanted to keep it std :P    
                let numbers: Vec<&str> = line.split_whitespace().collect(); 
                self.column_1.push(numbers[0].parse().unwrap());
                self.column_2.push(numbers[1].parse().unwrap());
            }
        }
    }

    fn order(&mut self) {
        self.column_1.sort_unstable();
        self.column_2.sort_unstable();
    }

    //TODO: Make this immutable and more idiomatic
    fn difference(mut self) -> Vec<u64> {
        let mut differences = Vec::new();
        while !self.column_1.is_empty() & !self.column_2.is_empty() {
            let column_1 = self.column_1.pop().unwrap();
            let column_2 = self.column_2.pop().unwrap();
            differences.push(column_1.abs_diff(column_2));
        };
        differences
    }
}
 
fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut list: List<i64> = List::new();

    //create list of u64 numbers
    list.process_txt_file(file_path);

    //order lists
    list.order();

    //compute differences of each line
    let differences = list.difference();

    //add every number in vector
    let answer: u64 = differences.iter().sum();

    println!("Total Distance: {}", answer);
}