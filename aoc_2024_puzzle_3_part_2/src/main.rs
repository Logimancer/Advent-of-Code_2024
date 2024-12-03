//AoC 2024 Puzzle 3 Part 2 
//This is a dirty version. Structually idomatic, but not very functional

use std::{ env, fs::read_to_string, str::FromStr, fmt::Debug, ops::AddAssign };
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
}