use rand::random;

pub const GRID_SIZE: (usize, usize) = (20,20);

//index : just a pair of integers
type Point = (usize, usize);

// grid : NxM array of cells? could be booleans, representing bomb or not bomb
pub struct Minesweeper<const W: usize, const H: usize> {
    grid: [[bool; W]; H]
}


pub fn default_game() -> Minesweeper<{GRID_SIZE.0},{GRID_SIZE.1}> {
    let grid = [[false ; GRID_SIZE.0]; GRID_SIZE.1];
    Minesweeper { grid }
}

pub fn random_grid(bomb_p : f32) -> Minesweeper<{GRID_SIZE.0},{GRID_SIZE.1}> {
    let mut grid : [[bool; GRID_SIZE.0]; GRID_SIZE.1] = Default::default();
    for i in 0..GRID_SIZE.0 {
        for j in 0..GRID_SIZE.1 {
            let f : f32 = random();
            grid[i][j] = f < bomb_p;
        }
    }
    Minesweeper { grid }
}

impl<const W: usize,const H: usize> Minesweeper<W, H> {

    pub fn is_bomb(&self, p: &Point) -> bool {
        let (x,y) = *p;
        self.is_in_grid(p) && self.grid[x][y]
    }

    pub fn is_in_grid(&self, p: &Point) -> bool {
        p.0 >= 0 && p.0 < W && p.1 >= 0 && p.1 < H
    }

    // Get neighbours count : takes a grid, and an index (i,j) and returns the amount of adjacent bombs
    pub fn neighbours_count(&self, p: Point) -> u8 {
        let mut count : u8 = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i != 0 && j != 0 {
                    let (x,y) = p;
                    let neighbour = ((x as i8 + i) as usize,(y as i8 + j) as usize);
                    if self.is_bomb(&neighbour) {
                        count += 1;
                    }
                }
            }
        }
        count
    }
    
}
