//Puzzle 18: RAM Run 
use std::{ env, fmt, fs::File, io::{ self, BufRead }, path::Path, slice::Iter };
use self::Directions::*;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

//used for representing a 2 dimensional memory space
#[derive(Debug)]
enum Directions {
    Left,
    Right,
    Up,
    Down,
}

impl Directions {
    fn iterator() -> Iter<'static, Directions> {
        static DIRECTIONS: [Directions; 4] = [Left, Right, Up, Down];
        DIRECTIONS.iter()
    }
}

struct Grid<T> {
    cells: Vec<Vec<T>>
}

#[derive(Debug)]
struct Cell {
    valid: bool,
    explored: bool,
    coordinates: (isize, isize),
    parent: (isize, isize)
}

type Coordinates = (isize, isize);
impl Cell {
    fn new(coordinates: Coordinates) -> Self {
        Self {
            valid: true,
            explored: false,
            coordinates: coordinates,
            parent: (0,0),
        }
    }
    
    fn possible_adjacent_cells(&self, boundries: Coordinates) -> Vec<Coordinates> {
        let mut adjacent_edges = Vec::new();
        println!("Entered pac, cell operating on is {},{}", self.coordinates.0, self.coordinates.1);
        for direction in Directions::iterator() {
            let adjacent_cell = match direction {
                Directions::Left => (self.coordinates.0 - 1, self.coordinates.1),  
                Directions::Right => (self.coordinates.0 + 1, self.coordinates.1),
                Directions::Up => (self.coordinates.0, self.coordinates.1 - 1),
                Directions::Down => (self.coordinates.0, self.coordinates.1 + 1),
            };
            
            //I don't know what I'm doing wrong, but the ! operator isn't working for the below if statement!
            if adjacent_cell.0.is_positive() &&
               adjacent_cell.0 < boundries.0 + 1 &&
               adjacent_cell.1.is_positive() &&
               adjacent_cell.1 < boundries.1 + 1 {
                    adjacent_edges.push(adjacent_cell);
               } 
        }
        adjacent_edges
    }
}

impl Grid<Cell> {
    fn new(x: isize, y: isize) -> Self {
        let mut columns = Vec::new(); 
        for current_y in 0..=y{
            let mut row = Vec::new();
            for current_x in 0..=x {
                row.push(Cell::new((current_x, current_y)));
            }
            columns.push(row);
        }

        Self {
            cells: columns,
        }
    }

    fn update_cell_valid(&mut self, x: isize, y: isize, value: bool) {
        self.cells[y as usize ][x as usize].valid = value;
    }

    fn update_cell_explored(&mut self, x: isize, y: isize, value: bool) {
        self.cells[y as usize][x as usize].valid = value;
    }

    fn coordinates_to_cell(&self, coordinates: Coordinates) -> &Cell {
        &self.cells[coordinates.1 as usize][coordinates.0 as usize]       
    }
}

impl fmt::Display for Grid<Cell> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cells
            .iter()
            .map(|ys| { 
                writeln!(f, "").unwrap(); 
                ys.iter()
                  .map(|y| {
                    if y.valid == true {
                        write!(f, ". ")
                    } else {
                        write!(f, "# ")
                    }
                  }
                ).collect()
            })
            .collect()
    }
}

type Goal = Coordinates;
type Root = Coordinates;

const ROOT: Root = (0, 0);
struct MemorySpace {
    grid: Grid<Cell>,
    boundries: Coordinates,
}

impl MemorySpace {
    //returns a sized Grid of u64s
    fn new(x: isize, y: isize) -> Self {
        Self {
            grid: Grid::new(x, y),
            boundries: (x, y),
        }
    }

    fn load_corrupted_from_file(&mut self, file_path: &String) {
        let file = read_lines(file_path);
        if let Ok(lines) = file {
            let coordinates_pairs: Vec<Vec<isize>> = lines.map(|x| x.unwrap()
                                                                    .split(",")
                                                                    .map(|y| y.parse::<isize>()
                                                                                    .unwrap()
                                                                        ).collect::<Vec<isize>>()
                                                    )
                                                .collect();
            
            for coordinates in coordinates_pairs{
                let (x, y) = (coordinates[0], coordinates[1]);
                println!("{:?}", coordinates);
                
                self.grid.update_cell_valid(x, y, false);
            }
        }
    }

    fn find_adjacent_edges(&self, current_cell_coordinates: Coordinates) -> Vec<Coordinates> {
        let current_cell = self.grid.coordinates_to_cell(current_cell_coordinates);
        let possible_adjacent_cells = current_cell.possible_adjacent_cells(self.boundries);

        let mut adjacent_edges = Vec::new();
        for adjacent_cell_coordinates in possible_adjacent_cells {
            let possible_adjacent_cell = self.grid.coordinates_to_cell(adjacent_cell_coordinates);
            if possible_adjacent_cell.valid {
                adjacent_edges.push(adjacent_cell_coordinates);
            }
        }
        adjacent_edges
    }
//    fn breadth_first_search(&mut self, goal: Goal) -> isize{
//        let mut queue = Vec::new();
//        let (x, y) = ROOT;
//        self.grid.update_cell_explored(x, y, true);
//        queue.push((x, y));
//        while !queue.is_empty() {
//            let v = queue.last().unwrap();
//            if v == goal {
//                return v
//            }
//            //find adjacent edes of cell
//            for al
//
//        }
//
//        0
//    }
}





impl fmt::Display for MemorySpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}

fn main() {
    
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut memory = MemorySpace::new(6,6);

    memory.load_corrupted_from_file(file_path);

    println!("{}", memory);
    for cells in memory.grid.cells.clone() {
        for cell in cells {
            println!("Possible adjacent cells of {}, {}:", cell.coordinates.0, cell.coordinates.1);
            let possible_adjacent_cells = cell.possible_adjacent_cells(memory.boundries);
            for possible_adjacent_cell in possible_adjacent_cells {
                println!("possibly adjecent: {}, {}", possible_adjacent_cell.0, possible_adjacent_cell.1);
            }
            let adjacent_cells = memory.find_adjacent_edges(cell.coordinates);
        }
    }
}