// this one is mad ghetto fab and need ALOT of work, amazing what you'll crap out
// when your low on time!

use std::{ env, error::Error, fs::File, io::{ self, BufRead }, ops::Index, path::Path };

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

type Rule = (usize, usize);
type RuleSet = Vec<Rule>;

type PageUpdates = Vec<usize>;
#[derive(Debug, Clone)]
struct Book {
    page_update_list: Vec<PageUpdates>,
    order_correct: bool,
}

impl Book {
    fn new() -> Self {
        Book {
            page_update_list: Vec::new(),
            order_correct: false,
        }
    }

    fn check_rules(mut self, page_rules:RuleSet) -> Option<Book> {
        //let mut reordered_pages = book.page_update_list;
        for pages_to_update in self.clone().page_update_list{
            let pertinent_rules:Vec<&Rule> = pages_to_update.iter()
                                                 .map(|&page| page_rules.iter()
                                                                             .filter(move |&&rule| rule.0 == page)
                                                                             ).flatten().collect();
            let applicable_rules:Vec<&&Rule> = pertinent_rules.iter()
                                                         .filter(|rule| pages_to_update.contains(&rule.1))
                                                         .collect();
            let ordering_correct = applicable_rules.iter().all(|rule| pages_to_update.iter()
                                                                                                               .position(|&r| r == rule.0) < pages_to_update.iter().position(|&r| r == rule.1));
            self.order_correct = ordering_correct;
        }
        if self.order_correct{
            Some(self)
        } else {
            None
        }
    }

}

fn good_books(page_rules: Vec<Rule>, books: Vec<Book>) -> Option<Vec<Book>> {
    let mut good_books = Vec::new();
    for book in books {
        match  book.check_rules(page_rules.clone()) {
            Some(book) => good_books.push(book),
            None => ()
        };
    }
    Some(good_books)
}

fn answer(good_books: Vec<Book>) -> Result<usize, Box<dyn Error>> {
    let mut middle_numbers = Vec::new();
    for book in good_books {
        //this is a fix for not wanting to rewrite a badly written assc. fn!
        let mut page_update_list:PageUpdates = Vec::new();  
        for page_update in  book.page_update_list {
            page_update_list = page_update;
        }
        let middle_number_of_list_index = page_update_list.iter().count() / 2;
        middle_numbers.push(*page_update_list.index(middle_number_of_list_index));
    }

    Ok(middle_numbers.iter().sum::<usize>()) 
}  

fn process_txt_file(file_path: &String) -> Vec<String> {
    let mut file = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            file.push(line)
        };
    }
    file
}


fn process_input(file_path: &String) -> (RuleSet, Vec<Book>) {
    let input_raw = process_txt_file(file_path);
    let mut page_rules:RuleSet = Vec::new();
    let rules_raw = input_raw.iter().filter(|line| line.contains("|"));
    //build rule collection
    for rule in rules_raw {
        let rule_raw:Vec<&str> = rule.split("|").collect();
        let rule:Rule = (rule_raw[0].parse().unwrap(), rule_raw[1].parse().unwrap());
        page_rules.push(rule);
    }

    //build book and pages collection
    let mut books = Vec::new();
    let book_raw = input_raw.iter().filter(|&line| line.contains(",")); 
    for page_updates_raw in book_raw {
        let page_updates_split:Vec<&str> = page_updates_raw.split(",").collect();
        let page_updates = page_updates_split.iter().map(|page| page.parse().unwrap()).collect();
        let mut book = Book::new();
        book.page_update_list.push(page_updates);
        books.push(book);
    }

    (page_rules, books)

}

fn main() {
    
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    
    let (page_rules, books) = process_input(file_path);

    let answer = answer(good_books(page_rules, books).unwrap());

    println!("Answer: {}", answer.unwrap());    

}
