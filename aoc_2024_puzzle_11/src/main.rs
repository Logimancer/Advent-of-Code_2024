//plutonium pebbles
use std::{ env, fs::File, io::{ self, BufRead }, path::Path };
use std::str::FromStr;
use std::clone::Clone;
use std::fmt::Debug;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

type Pebble = u64;

fn is_even_number_of_digits(digit: u64) -> bool {
    (digit.checked_ilog10().unwrap_or(0) + 1) % 2 == 0
}

#[derive(Debug, Clone)]
struct Pebbles<T> {
    group: Vec<T>,
}

impl<T: Clone + FromStr /* + PartialEq<u64>*/> Pebbles<T> {
    fn new() -> Self {
        Pebbles { group: Vec::new() }
    }

    fn txt_to_pebbles(&mut self, file_path: &String) 
        where <T as FromStr>::Err: Debug, 
              Vec<T>: FromIterator<u64> {
        if let Ok(lines) = read_lines(file_path) {
            self.group = lines.flat_map(|x| x.unwrap()
                                                                    .split_whitespace()
                                                                    .map(|y| y.parse()
                                                                                    .unwrap())
                                                                    .collect::<Vec<u64>>())
                                .collect();
        };
    }
}

impl Pebbles<u64> {
    fn blink(&mut self) -> Pebbles<u64> {
        //rule one
        let mut pebbles = Pebbles::new();
        for pebble in self.group.clone(){
            //rule one
            if pebble == 0 {
                pebbles.group.push(1);
            } else if is_even_number_of_digits(pebble) {
                let size_of_pebble = pebble.checked_ilog10().unwrap_or(0) + 1;
                //println!("Size of pebble: {}", size_of_pebble);
                let divisor_to_get_lhs = 10u64.pow(size_of_pebble/2) as u64;
                //println!("Divisor to get lhs: {}", divisor_to_get_lhs);
                let lhs_of_pebble = pebble / divisor_to_get_lhs;
                let rhs_of_pebble = pebble - (lhs_of_pebble * 10u64.pow(size_of_pebble/2));
                pebbles.group.push(lhs_of_pebble);
                pebbles.group.push(rhs_of_pebble); 
            } else {
                pebbles.group.push(pebble * 2024);
            }
        }
        pebbles
    }
}

fn main() {
    
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let blinks = args[2].parse().unwrap();

    let mut pebbles = Pebbles::<Pebble>::new();

    pebbles.txt_to_pebbles(file_path);

    if blinks > 0 {
        for _blink in 1..=blinks {
            pebbles = pebbles.blink();
        }
    } 

    println!("{} stones.", pebbles.group.iter().count());
}