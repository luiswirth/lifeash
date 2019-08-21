#[derive(Clone)]
enum Cell {
    Alive,
    Dead,
}

struct World {
    m_cells: Vec<Vec<Cell>>,
    m_old: Vec<Vec<Cell>>
}

impl World {
    fn create(width: usize, height: usize) -> World {
        World {
            m_cells: vec![vec![Cell::Dead; height]; width],
            m_old: vec![vec![]]
        }
    }

    fn get(&mut self, i: usize, j: usize) -> Option<&Cell> {
        match self.m_cells.get(i) {
            None => return None,
            Some(cell) => {
                return cell.get(j);
            }
        }
    }


    fn set(&mut self, i: usize, j: usize, value: Cell) {
        if let Some(col) = self.m_cells.get_mut(i) {
            if let Some(cell) = col.get_mut(j) {
                cell = value;
            }
        }
    }

    fn update(&mut self) {
        self.m_old = self.m_cells.clone();
        for (i, _) in self.m_cells.iter().enumerate() {
            for (j, _) in self.m_cells.get(i).unwrap().iter().enumerate() {
                println!("[*] i: {}, j: {}", i, j);
                self.update_single(i,j);
            }
        }
    }

    fn update_single(&mut self, i: usize, j: usize) {
        let mut neighbour_count = 0;
        for k in (-1..2) {
            for l in (-1..2) {
                if k == 0 || l == 0 {
                    continue;
                }
                if let Some(cell) = self.get(i, j) {
                    match cell {
                        Alive => neighbour_count += 1,
                        Dead => (),
                    }
                }
            }
        }
        if neighbour_count == 3 {
            self.get(i, j).unwrap() = Cell::Alive;
        }

    }

    fn render(&self) {

    }
}

fn main() {
    let mut world = World::create(32, 32);
    world.update();
}
