use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::video::Window;
use sdl2::render::WindowCanvas;

type Board = Vec<Vec<u16>>;

fn init_cells(width: usize, height: usize) -> Board {
    let mut cells: Board =
        vec![vec![0; width]; height];
    cells[10][10] = 1;
    cells[10][11] = 1;
    cells[10][12] = 1;
    cells[10][13] = 1;
    cells[11][13] = 1;

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
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1), (0, 1),
            (1, -1), (1, 0), (1, 1),
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

fn draw_line(
    canvas: &mut WindowCanvas,
    point: Point,
    pixels: i32) {
    for i in 0..pixels {
        let x = std::cmp::max(0, point.x + i);
        let y = point.y;
        canvas.draw_point(Point::new(x, y));
    }
}

fn draw_box(
    canvas: &mut WindowCanvas,
    point: Point,
    pixels: i32) {
    canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
    for i in 0..pixels {
        let y = std::cmp::max(0, point.y + i);
        let x = point.x;
        draw_line(canvas, Point::new(x, y), pixels);
    }
}

pub fn draw_cells(canvas: &mut WindowCanvas, cells: &Board, pixels: i32) {
    for i in 0..cells.len() {
        let x = i as i32 * pixels;
        for j in 0..cells[0].len() {
            let y = j as i32 * pixels;
            if cells[i][j] != 0 {
                draw_box(canvas, Point::new(x, y), pixels);
            }
        }
    }
}

fn main() -> Result<(), String> {
    let SQUARE_SIZE = 8;
    let PLAYGROUND_WIDTH = 50;
    let PLAYGROUND_HEIGHT = 50;
    let width = 50;
    let height = 50;
    let mut cells = init_cells(width, height);
    cells[10][10] = 1;
    cells[10][11] = 1;
    cells[10][12] = 1;
    cells[10][13] = 1;
    cells[11][13] = 1;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rust-sdl2 demo: Game of Life",
                SQUARE_SIZE * PLAYGROUND_WIDTH,
                SQUARE_SIZE * PLAYGROUND_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // the canvas allows us to both manipulate the property of the window and to change its content
    // via hardware or software rendering. See CanvasBuilder for more info.
    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    // clears the canvas with the color we set in `set_draw_color`.
    canvas.clear();
    canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
    canvas.draw_point(Point::new(10, 10));
    // However the canvas has not been updated to the window yet, everything has been processed to
    // an internal buffer, but if we want our buffer to be displayed on the window, we need to call
    // `present`. We need to call this everytime we want to render a new frame on the window.
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    let mut frame: u32 = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. }
                => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Space), .. }
                => {
                    cells = update_cells(&cells);
                },
                _ => ()
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        // clears the canvas with the color we set in `set_draw_color`.
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
        draw_cells(&mut canvas, &cells, 10);
        // However the canvas has not been updated to the window yet, everything has been processed to
        // an internal buffer, but if we want our buffer to be displayed on the window, we need to call
        // `present`. We need to call this everytime we want to render a new frame on the window.
        canvas.present();

    }

    Ok(())
}
