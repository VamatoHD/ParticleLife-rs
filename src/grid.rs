use std::collections::HashMap;

pub struct Grid {
    pub cell_size: f32,
    cells: HashMap<(i32, i32), Vec<usize>>,
    pub cols: i32,
    pub rows: i32,
}

impl Grid {
    pub fn new(cell_size: f32, width: f32, height: f32) -> Grid {
        Grid {
            cell_size,
            cells: HashMap::new(),
            cols: ((width / cell_size) as i32).max(1),
            rows: ((height / cell_size) as i32).max(1),
        }
    }

    pub fn hash(&self, x: f32, y: f32) -> (i32, i32) {
        let cell_x = (x / self.cell_size).floor() as i32;
        let cell_y = (y / self.cell_size).floor() as i32;
        (cell_x.rem_euclid(self.cols), cell_y.rem_euclid(self.rows))
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
                    (cx + dx).rem_euclid(self.cols),
                    (cy + dy).rem_euclid(self.rows),
                );

                if let Some(points) = self.cells.get(&key) {
                    res.extend(points.iter());
                }
            }
        }
        res
    }
}
