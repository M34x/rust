use macroquad::prelude::*;

const GRID_SIZE: usize = 8;
const CELL_SIZE: f32 = 38.0;
const CELL_GAP: f32 = 4.0;

fn shapes() -> Vec<Vec<i32>> {
    return vec![
        vec![1,1,0,0,1,0,0,1,0],
        vec![0,1,1,0,1,0,0,1,0],
        vec![0,1,0,0,1,0,1,1,0],
        vec![0,1,0,0,1,0,0,1,1],
        vec![1,0,0,1,1,1,0,0,0],
        vec![0,0,1,1,1,1,0,0,0],
        vec![0,0,0,1,1,1,1,0,0],
        vec![0,0,0,1,1,1,0,0,1],

        vec![0,1,0,1,1,0,0,1,0],
        vec![0,1,0,0,1,1,0,1,0],
        vec![0,1,0,1,1,1,0,0,0],
        vec![0,0,0,1,1,1,0,1,0],

        vec![1,1,0,1,1,0,0,0,0],
        vec![1,1,1,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,0,0,0],
        vec![1,1,0,1,1,0,1,1,0],

        vec![1,0,0,0,1,0,0,0,1],
        vec![0,0,1,0,1,0,1,0,0],

        vec![0,1,1,1,1,0,0,0,0],
        vec![1,1,0,0,1,1,0,0,0],
        vec![0,1,0,0,1,1,0,0,1],
        vec![0,0,1,0,1,1,0,1,0],

        vec![1,0,0,1,0,0,1,1,1],
        vec![0,0,1,0,0,1,1,1,1],
        vec![1,1,1,1,0,0,1,0,0],
        vec![1,1,1,0,0,1,0,0,1],

        vec![0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0]
    ]
}


struct Board {
    pos: Vec2,
    cells: Vec<Vec<Option<Color>>>,
}
impl Board {
    fn new() -> Self {
        let total_step = CELL_SIZE + CELL_GAP;
        let board_pixel_size = (GRID_SIZE as f32 * total_step) - CELL_GAP;
        let start_x = ();
        Self {
            pos:,
            cells:,
        }
    }
}


struct Block {
    pos: Vec2,
    shape: Vec<i32>,
    colour: Color
}
impl Block {
    fn new(count:u8, shapes: &[Vec<i32>]) -> Self {
        let idx = rand::gen_range(0, shapes.len());
        Self {
            pos:vec2(250.0 + (count as f32 * 150.0),100.0),
            colour: Color {r: 1.0, g: 1.0, b: 1.0, a: 1.0},
            shape: shapes[idx].clone()
        }
    }
    fn draw(&self) {
        let grid_size = self.shape.len().isqrt();
        let cell_size = 24.0;
        let gap = 2.0;
        for (i, &filled) in self.shape.iter().enumerate() {
            if filled == 1 {
                let row = (i/grid_size) as f32;
                let col = (i % grid_size) as f32;
                let x = self.pos.x + col * (cell_size + gap);
                let y = self.pos.y + row * (cell_size + gap);

                draw_rectangle(x, y, cell_size, cell_size, self.colour);
            }
        }

    }
}
fn window_conf() -> Conf {
    Conf {
        window_title: "gaem".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let slate = shapes();
    let tray = vec![
        Block::new(0, &slate),
        Block::new(1, &slate),
        Block::new(2, &slate),
    ];
    loop {
        clear_background(BLACK);
        for block in &tray {
            block.draw();
        }
        next_frame().await;
    }
}