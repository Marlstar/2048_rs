use rand::Rng;

type GridType = [[usize;4];4];
type RowType = [usize;4];

#[derive(Debug, Clone)]
pub struct Backend {
    grid: GridType,
    score: usize
}
impl Backend { // Constructors
    pub fn new() -> Self {
        let mut s = Self {
            grid: [[0;4];4],
            score: 0
        };
        s.spawn(); s.spawn();
        s
    }
    pub fn load(grid: GridType, score: usize) -> Self {
        Self {
            grid,
            score
        }
    }
    pub fn load_grid(grid: GridType) -> Self {
        Self {
            grid,
            score: 0
        }
    }

    // Getters
    pub fn grid(&self) -> GridType {
        self.grid
    }
    pub fn grid_ref(&self) -> &GridType {
        &self.grid
    }
}

impl Backend { // Cell Spawning
    fn empty_cells(&self) -> Vec<(usize, usize)> {
        let mut empty_cells: Vec<(usize, usize)> = vec![];

        for row in 0..4 {
            for col in 0..4 {
                if self.grid[row][col] == 0 {
                    empty_cells.push((col, row));
                }
            }
        }

        return empty_cells;
    }

    fn spawn(&mut self) {
        let mut rng = rand::thread_rng();

        let empty_cells = self.empty_cells();
        let count_empty = empty_cells.iter().count();
        let chosen = empty_cells[rng.gen_range(0..count_empty)];
        let (x, y) = chosen;
        self.grid[y][x] = [2, 4][rng.gen_range(0..=1)];
    }
}

impl Backend { // Movement
    pub fn shift(&mut self, direction: ShiftDirection) {
        if self.merge(direction) {
            self.spawn();
        }
    }

    fn compress(input: &RowType) -> RowType {
        let mut new: RowType = [0;4];
        let mut inserted = 0;
        for i in 0..4 {
            if input[i] != 0 {
                new[inserted] = input[i];
                inserted += 1;
            }
        };
        return new;
    }

    pub fn merge_row(input: &RowType, score: &mut usize) -> RowType {
        let mut out: RowType = input.clone();

        'merge_loop: loop {
            out = Self::compress(&out);
            for i in 0..3 {
                if out[i] != 0 && out[i] == out[i+1]{
                    out[i] *= 2;
                    *score += out[i];
                    out[i+1] = 0;
                    continue 'merge_loop;
                };
            };
            break 'merge_loop;
        };
        out
    }

    fn merge(&mut self, direction: ShiftDirection) -> bool {
        // let horo_to_vert = |input: &GridType| -> GridType {
        //     // let out: GridType = itertools::izip!(..self.grid)
        //     out
        // };
        let og_input = self.grid();

        match direction {
            ShiftDirection::Left => { // Done
                for row in 0..4 {
                    self.grid[row] = Self::merge_row(&self.grid[row], &mut self.score);
                }
            },

            ShiftDirection::Right => {
                for row in 0..4 {
                    let flipped = flip(&self.grid[row]);
                    self.grid[row] = flip(&Self::merge_row(
                        &flipped,
                        &mut self.score
                    ))
                }
            }

            ShiftDirection::Up => {
                let mut transposed: GridType = transpose(&self.grid);
                for row in 0..4 {
                    transposed[row] = Self::merge_row(&transposed[row], &mut self.score);
                }
                self.grid = transpose(&transposed);
            }

            ShiftDirection::Down => {
                let mut transposed: GridType = transpose(&self.grid);
                for row in 0..4 {
                    transposed[row] = flip(&Self::merge_row(
                        &flip(&transposed[row]),
                        &mut self.score
                    ));
                }
                self.grid = transpose(&transposed);
            }
        }

        return self.grid != og_input;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShiftDirection {
    Up,
    Down,
    Left,
    Right
}

pub fn transpose(input: &GridType) -> GridType {
    let mut transposed: GridType = [[0;4];4];
    for i in 0..4 {
        for j in 0..4 {
            transposed[j][i] = input[i][j];
        }
    }
    transposed
}
pub fn flip(input: &RowType) -> RowType {
    let mut out = input.clone();
    out.reverse();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_left() {
        assert_eq!(
            Backend::compress(&[0,2,0,2]),
            [2,2,0,0]
        );
        assert_eq!(
            Backend::compress(&[4,2,0,2]),
            [4,2,2,0]
        );
    }

    #[test]
    fn test_merge() {
        assert_eq!(
            Backend::merge_row(&[0,2,0,2], &mut 0),
            [4,0,0,0]
        );
        assert_eq!(
            Backend::merge_row(&[4,2,0,2], &mut 0),
            [8,0,0,0]
        );
    }

    #[test]
    fn test_transpose() {
        assert_eq!(
            transpose(&[[1,1,0,0];4]),
            [[1;4],[1;4],[0;4],[0;4]]
        )
    }

    #[test]
    fn test_array_flip() {
        assert_eq!(
            flip(&[0,2,0,4]),
            [4,0,2,0]
        )
    }

    #[test]
    fn test_shifts() {
        let base = [
            [2, 0, 2, 2],
            [4, 0, 0, 2],
            [0, 2, 4, 2],
            [2, 2, 4, 2]
        ];

        { // Left
            let mut b = Backend::load_grid(base);
            b.merge(ShiftDirection::Left);
            assert_eq!(
                b.grid(),
                [
                    [4, 2, 0, 0],
                    [4, 2, 0, 0],
                    [2, 4, 2, 0],
                    [8, 2, 0, 0]
                ],
                "shift left failed"
            )
        }
        { // Right
            let mut b = Backend::load_grid(base);
            b.merge(ShiftDirection::Right);
            assert_eq!(
                b.grid(),
                [
                    [0, 0, 2, 4],
                    [0, 0, 4, 2],
                    [0, 2, 4, 2],
                    [0, 0, 8, 2]
                ],
                "shift right failed"
            )
        }
        { // Up
            let mut b = Backend::load_grid(base);
            b.merge(ShiftDirection::Up);
            assert_eq!(
                b.grid(),
                [
                    [2, 4, 2, 8],
                    [4, 0, 8, 0],
                    [2, 0, 0, 0],
                    [0, 0, 0, 0]
                ],
                "shift up failed"
            )
        }
        { // Down
            let mut b = Backend::load_grid(base);
            b.merge(ShiftDirection::Down);
            assert_eq!(
                b.grid(),
                [
                    [0, 0, 0, 0],
                    [2, 0, 0, 0],
                    [4, 0, 2, 0],
                    [2, 4, 8, 8]
                ],
                "shift down failed"
            )
        }
    }
}