
type Board = Vec<Vec<u16>>;
fn init_cells(width: usize, height: usize) -> Board {
    let cells: Board =
        vec![vec![0; width]; height];
    return cells;
}

fn print_cells(cells: &Board) {
    println!("{}", "+".repeat(cells.len()));
    for row in 0..cells.len() {
        for cell in 0..cells[0].len() {
            let c = match cells[row][cell] {
                1 => '#',
                _ => ' ',
            };
            print!("{}", c);
        }
        println!("");
    }
}

fn update_cells(cells: &Board) -> Board {
    let mut updated_cells = cells.clone();
    let pairs: Vec<(i32, i32)> =
        vec![
            (-1, -1), (-1, 0), (-1,1),
            (0, -1), (0,1),
            (1, -1), (1, 0), (1,1),
        ];
    for row in 0..cells.len() {
        for cell in 0..cells[0].len() {
            let mut live_count = 0;
            for (x, y) in &pairs {
                let r = row as i32 + *x;
                let c = cell as i32 + *y;
                if r >= 0 && c >= 0 && r < cells.len() as i32 && c < cells[0].len() as i32 {
                    let cell_value = cells[r as usize][c as usize];
                    live_count += cell_value;
                }
            }

            match live_count {
                0..=1 => updated_cells[row][cell] = 0,
                2 =>
                    if cells[row][cell] == 1 {
                        updated_cells[row][cell] = 1;
                    },
                3 => updated_cells[row][cell] = 1,
                _ => updated_cells[row][cell] = 0,
            }
        }
    }
    return updated_cells;
}


#[cfg(test)]
pub mod tests {
    use crate::*;

    #[test]
    pub fn simple_test() {
        let width = 20;
        let height = 20;
        let mut cells = vec![vec![0; width]; height];
        cells[10][10] = 1;
        cells[10][11] = 1;
        cells[10][12] = 1;
        cells[10][13] = 1;
        cells[11][13] = 1;
        print_cells(&cells);
        let mut iterations = 10;
        loop {
            if iterations < 1 {
                break;
            }
            cells = update_cells(&cells);
            print_cells(&cells);
            iterations -= 1;
        }
    }
}

fn main() {
    let width = 5;
    let height = 5;
    let mut cells = init_cells(width, height);
    print_cells(&cells);
    let cells = update_cells(&cells);
    print_cells(&cells);
}
