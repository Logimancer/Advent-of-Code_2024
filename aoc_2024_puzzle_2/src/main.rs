//AoC 2024 Puzzle 2 Part 1 (is it safe?)
use std::{ env, fs::File, io::{ self, BufRead }, path::Path };

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

    fn safe_report_count(&self) -> usize {
        self.reports.iter().filter(|x| x.is_safe()).count()
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
        let level_iter = self.levels.iter();
        //are levels descending only?
        let asc_desc = level_iter.clone().is_sorted_by(|a, b| a > b) |
        //are levels ascending only?
        level_iter.clone().is_sorted_by(|a, b| a < b);
        
        //did any two levels adjacent to eachother differ by less than one or 
        //more than three?
        let differences: Vec<_> = self.levels
                                    .windows(2)
                                    .collect::<Vec<_>>()
                                    .iter()
                                    .map(|a| a[0].abs_diff(a[1]))
                                    .collect();                                           
        let acceptable_distances = 1..4;
        let acceptable_diffs:Vec<bool> = differences.iter().map(|x| acceptable_distances.contains(x)).collect(); 
        
        //if none of the adjacent levels are more than 3 apart and they are in ascending or decending
        //return true
        !acceptable_diffs.contains(&false) && asc_desc    
    }


}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut data = Data::new();

    //process input into list of reports
    data.process_txt_file(file_path);

    println!("{} report(s) are safe", data.safe_report_count());
}