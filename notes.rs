

static dirs: Vec<(i8,i8)> = generate_dirs();

trait Graph {
    type Vertex;
    fn neighbours(&self, vertex: Vertex) -> Iterator<Vertex>;
}

struct Grid2D;

impl Graph for Grid2D {
    struct GridCell {
        x: i8,
        y: i8,
        info: bool
    }
    impl Eq for GridCell {
        fn eq(&self, cell: &GridCell) {
            self.x == cell.x && self.y == cell.y
        }
    }

    type Vertex = GridCell;
    // ((1,2), true) != ((1,2), false)
    fn neighbours(&self, vertex: Vertex) -> Iterator<Vertex> {
        let ((x, y), _) = vertex;
        dirs.map(|(dx,dy)| (x + dx, y + dy))
    }
}

fn generate_dirs() -> Vec<(i8, i8)>{
    let dirs = vec![];
    for i in -1..1 {
        for j in -1..1 {
            if (i,j) != (0,0) {
                dirs.append((i,j));
            }
        }
    }
    dirs
} 