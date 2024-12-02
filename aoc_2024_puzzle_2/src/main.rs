//AoC 2024 Puzzle 2 Part 1 (is it safe?)
use std::{ clone, collections::btree_set::Difference, env, fs::File, io::{ self, BufRead }, path::Path };

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Data<T> {
    reports: Vec<Report<T>>,
}

impl<T> Data<T>{
    fn new() -> Self {
        Self { reports: Vec::new(),
        }
    }
}

impl Data<u64> {
    fn process_txt_file(&mut self, file_path: &String) {
        if let Ok(lines) = read_lines(file_path) {
            for line in lines.flatten() {
                let mut report = Report::new();
                report.add(line);
                self.reports.push(report);
            }
        }
    }
}


struct Report<T> {
    levels: Vec<T>, 
}

impl<T> Report<T> {
    fn new() -> Self {
        Self { levels: Vec::new(),
        }
    }
}

impl Report<u64> {
    fn add(&mut self, line: String) {
        let values: Vec<&str> = line.split_whitespace().collect(); 
        for value in values {
            self.levels.push(value.parse().unwrap());
        }
    }

    fn is_safe(&self) -> bool {
        let mut level_iter = self.levels.iter().peekable();
        //are levels descending only?
        let asc_desc = (level_iter.clone().is_sorted_by(|a, b| a > b) |
        //are levels ascending only?
        level_iter.clone().is_sorted_by(|a, b| a < b)) ;
        
        //did any two levels adjacent to eachother differ by less than one or 
        //more than three?
        while let Some(distance_safe) = level_iter.peek() {
            let difference = distance_safe.abs_diff(*level_iter.next().unwrap());
            if difference > 1 
            && difference < 4 {
                println!("true"); 
            }  else {
                println!("false");
            }
        }
        let mut dist_acceptable = Some(false);
        loop {
            let dist_acceptable = match level_iter.next().unwrap().abs_diff(**level_iter.peek().unwrap(0)) {  
                            1..4 => true,
                            _ => false
                        };
            if dist_acceptable == false {break};        
            };
        println!("asc or desc: {}, dist good? {:?}", asc_desc, dist_acceptable); 
            true
    }

}
fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut data = Data::new();

    //process input into list of reports
    data.process_txt_file(file_path);

//    for report in data.reports {
//        let mut level_total = 0;
//        for level in report.levels {
//            level_total += level;
//        }
//        println!("{}", level_total);
//    }

    for report in data.reports {
        report.is_safe();
    }
}