use std::{ env, fs, fs::File, io::{ self, BufRead }, path::Path };

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
 
fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut list: List<u64> = List::new();

    //create list of u64 numbers
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            let numbers: Vec<&str> = line.split_whitespace().collect(); //would have used collect to tupple in itertools, but wanted to keep it std :P
            list.column_1.push(numbers[0].parse().unwrap());
            list.column_2.push(numbers[1].parse().unwrap());
        }
    }

    println!("{} {}", list.column_1[0], list.column_2[0]);


}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}