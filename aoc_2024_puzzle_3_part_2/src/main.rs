//AoC 2024 Puzzle 3 Part 2 
//This is a dirty version. Structually idomatic, but not very functional

use std::{ env, fmt::Debug, fs::read_to_string, iter, ops::AddAssign, str::FromStr };
use regex::Regex;

#[derive(Clone)]
struct Data<T> {
    input: String,
    muls: Vec<Mul<T>>,
}

impl<T: std::str::FromStr 
+ std::ops::Mul<Output = T> 
+ std::clone::Clone 
+ std::fmt::Debug> Data<T> where u64: AddAssign<T> {
    fn new() ->  Self {
        Self { input: String::new(),
               muls: Vec::new(), }
    }

    fn process_txt_file(&mut self, file_path: &String) {
        self.input = read_to_string(file_path).expect("file error");
    }

    //TODO: add filter for do()s and dont()s
    fn remove_donts(&self) {
        let dont_locs = self.input.match_indices("don't()");
        let do_locs = self.input.match_indices("do()");
        let mut inced_dont_locs: Vec<usize> = dont_locs.clone().map(|x| x.0 + 1).collect();
        let adj_do_locs:Vec<usize> = do_locs.clone().map(|x| x.0).filter(|&x| x > *inced_dont_locs.first().unwrap()).collect();
        let mut peekable_adj_do_locs = adj_do_locs.iter().peekable();
        
        let mut counter = 0; 
        let mut zipped_locs:Vec<(&usize, usize)> = inced_dont_locs.iter().zip(adj_do_locs).collect();
        for ziper in zipped_locs.clone() {
            counter = counter + 1;
            println!("zipped {}: {} {}", counter, ziper.0, ziper.1);
        }
        
        zipped_locs.reverse();
        let mut index = zipped_locs.len() - 2;
        let mut remove_locs = Vec::new();
        remove_locs.push(zipped_locs[index + 1]);
        loop { 
            let zip = zipped_locs.pop().unwrap();
            println!("zip {} {}", zip.0, zip.1);
            println!("zip index {} {}", zipped_locs[index].0, zipped_locs[index].1);
            if zip.1 < *zipped_locs[index].0 {
                remove_locs.push(zipped_locs[index]);
            }

            if zipped_locs.is_empty() { break; }
            println!("{}", zipped_locs.len())
            else{if zipped_locs.len()  1 {index -= 1;}}
        }

        for zip in remove_locs {
            println!("{} {}", zip.0, zip.1);
        };
    }

    fn find_muls(&self) -> Vec<Mul<T>> where T: FromStr<Err : Debug>  {
        let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
        let mut muls = Vec::new();
        //i mean... you knew we were going to regex this, right?
        for (_, [multiplicand, multiplier]) 
        in regex
        .captures_iter(self.input.as_str())
        .map(|c| c.extract()) {
            let mul= Mul::new(multiplicand.parse()
                                          .unwrap(), 
                              multiplier.parse()
                                        .unwrap());
            muls.push(mul);
        }
        muls
    }

    fn answer(&mut self) -> u64 where <T as FromStr>::Err: Debug {
        self.muls = self.find_muls();
        let mut answer:u64 = 0;
        for mul in self.muls.clone() {
            answer += <T as Into<T>>::into(mul.product);
        }
        answer
    }

}

#[derive(Clone)]
struct Mul<T> {
    _multiplicand: T,
    _multiplier: T,
    product: T,
}

impl<T: std::ops::Mul<Output = T> + std::clone::Clone> Mul<T> {
    fn new(multiplicand: T, multiplier: T) -> Self {
        Self {
            _multiplicand: multiplicand.clone(),
            _multiplier: multiplier.clone(),
            product: multiplicand * multiplier
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut data:Data<u64> = Data::new();

    data.process_txt_file(file_path);

    data.muls = data.find_muls();

    println!("Mul count: {}", data.muls.iter().count());
    println!("Answer: {}", data.answer());
    data.remove_donts();
}