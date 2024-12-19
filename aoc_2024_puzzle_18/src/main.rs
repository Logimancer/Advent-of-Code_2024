//Puzzle 18: RAM Run 
use std::{ collections:: VecDeque, env, fmt, fs::File, io::{ self, BufRead }, path::Path, slice::Iter };
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

#[derive(Debug, Clone)]
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
        let mut possible_adjacent_edges = Vec::new();
        for direction in Directions::iterator() {
            let adjacent_cell = match direction {
                Directions::Left => (self.coordinates.0 - 1, self.coordinates.1),  
                Directions::Right => (self.coordinates.0 + 1, self.coordinates.1),
                Directions::Up => (self.coordinates.0, self.coordinates.1 - 1),
                Directions::Down => (self.coordinates.0, self.coordinates.1 + 1),
            };
            
            //I don't know what I'm doing wrong, but the ! operator isn't working for the below if statement!
            if adjacent_cell.0 >= 0 &&
               adjacent_cell.0 < boundries.0 + 1 &&
               adjacent_cell.1 >= 0 &&
               adjacent_cell.1 < boundries.1 + 1 {
                    possible_adjacent_edges.push(adjacent_cell);
               } 
        }
        possible_adjacent_edges
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

    fn update_cell_valid(&mut self, coordinates: Coordinates, value: bool) {
        let x = coordinates.0;
        let y = coordinates.1;
        self.cells[y as usize ][x as usize].valid = value;
    }

    fn update_cell_explored(&mut self, coordinates: Coordinates, value: bool) {
        let x = coordinates.0;
        let y = coordinates.1;
        self.cells[y as usize][x as usize].explored = value;
    }

    fn update_cell_parent(&mut self, coordinates: Coordinates, value: Coordinates) {
        let x = coordinates.0;
        let y = coordinates.1;
        self.cells[y as usize][x as usize].parent = value;
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
    shortest_path: Vec<Coordinates>,
}

impl MemorySpace {
    //returns a sized Grid of u64s
    fn new(x: isize, y: isize) -> Self {
        Self {
            grid: Grid::new(x, y),
            boundries: (x, y),
            shortest_path: Vec::new(),
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
                
                self.grid.update_cell_valid((x, y), false);
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
    fn breadth_first_search_parents(&mut self, goal: Goal) -> Vec<Coordinates>{
        let mut queue = VecDeque::new();
        let mut parents = Vec::new();
        let (x, y) = ROOT;
        self.grid.update_cell_explored((x, y), true);
        queue.push_front((x, y));
        while !queue.is_empty() {
            let v = queue.pop_back().unwrap();
            if v == goal {
                return parents
            }
            let adjacent_edges = self.find_adjacent_edges(v);
            for w in adjacent_edges {
                let w_cell = self.grid.coordinates_to_cell(w);
                if !w_cell.explored  {
                    self.grid.update_cell_explored(w, true);
                    self.grid.update_cell_parent(w, v);
                    parents.push((v.0,v.1));       
                    queue.push_front(w);
                }
            }
        }
        vec![(0,0)]
    }

    fn find_shortest_path(&mut self, goal: Coordinates) {
        let mut parents = self.breadth_first_search_parents(goal);
        let mut deduped_parents = Vec::new();
        deduped_parents.push(parents.pop().unwrap());
        while !parents.is_empty() {
            let elem_a = parents.pop().unwrap();

            if elem_a != *deduped_parents.last().unwrap_or(&elem_a) {
                deduped_parents.push(elem_a);
            }
        }
        
        let mut valid_move_parents = Vec::new();
        deduped_parents.reverse();
        valid_move_parents.push(deduped_parents.pop().unwrap());
        while !deduped_parents.is_empty() {
            let elem_b = deduped_parents.pop().unwrap();
            let elem_a = *valid_move_parents.last().unwrap();
            //remove any coordinates that are more than one space away from eachother
            if (elem_a.0 - elem_b.0).abs() + (elem_a.1 - elem_b.1).abs() == 1 {
                valid_move_parents.push(elem_b);
            }
        }
        self.shortest_path = valid_move_parents;

    }

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

    let goal = memory.boundries;

    memory.find_shortest_path(goal);

    for coordinates in &memory.shortest_path {
        println!("{}, {}", coordinates.0, coordinates.1);
    }    
    println!("Minimum Number of moves: {}", memory.shortest_path.len());  

}