use std::collections::HashMap;

pub struct Grid {
    cell_size: f32,
    cells: HashMap<(i32, i32), Vec<usize>>,
    width: f32,
    height: f32,
}

impl Grid {
    pub fn new(cell_size: f32, width: f32, height: f32) -> Grid {
        Grid {
            cell_size,
            cells: HashMap::new(),
            width,
            height,
        }
    }

    pub fn hash(&self, x: f32, y: f32) -> (i32, i32) {
        let cell_x = (x / self.cell_size).floor() as i32;
        let cell_y = (y / self.cell_size).floor() as i32;
        (cell_x, cell_y)
    }

    pub fn insert(&mut self, i: usize, x: f32, y: f32) {
        let key = self.hash(x, y);
        self.cells.entry(key).or_insert(Vec::new()).push(i);
    }

    pub fn query(&self, x: f32, y: f32) -> Vec<usize> {
        let (cx, cy) = self.hash(x, y);
        let mut res = Vec::new();

        for dx in -1..=1 {
            for dy in -1..=1 {
                let key = (
                    (cx + dx).rem_euclid((self.width / self.cell_size) as i32),
                    (cy + dy).rem_euclid((self.height / self.cell_size) as i32),
                );

                if let Some(points) = self.cells.get(&key) {
                    res.extend(points.iter());
                }
            }
        }
        res
    }
}
