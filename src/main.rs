use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    value: u32,
}

impl Cell {
    fn get_color(&self) -> graphics::Color {
        unimplemented!();
    }
}

pub struct CellRenderer {
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

impl CellRenderer {
    fn get_index(&self, x: u32, y: u32) -> usize {
        (y * self.height + x) as usize
    }
}

impl CellRenderer {
    fn new(_ctx: &mut Context, width: u32, height: u32) -> CellRenderer {
        let cells = (0..width * height)
            .map(|i| {
                Cell { value: 0 }
            })
            .collect();

        CellRenderer {
            width,
            height,
            cells
        }
    }

    fn tick(&mut self, _ctx: &mut Context) -> GameResult<()> {

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mb = &mut graphics::MeshBuilder::new();

        for x in 0..self.width {
            for y in 0..self.height {
                let idx = self.get_index(x, y);
                let cell = self.cells[idx];
            }
        }
        Ok(())
    }

}

pub struct AppConfig {
    width: u32,
    height: u32,
}

pub struct App {
    cell_renderer: CellRenderer
}

impl App {
    pub fn new(_ctx: &mut Context, config: &AppConfig) -> App {
        let cell_renderer = CellRenderer::new(_ctx, config.width, config.height);
        App { cell_renderer }
    }
}

impl EventHandler for App {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.cell_renderer.tick(_ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        self.cell_renderer.draw(ctx);
        graphics::present(ctx)
    }
}

fn main() {
    let window_setup = ggez::conf::WindowSetup::default()
        .title("Joy");
    let window_mode = ggez::conf::WindowMode::default();

    let(mut ctx, mut event_loop) = ContextBuilder::new("joy_app", "lwirth")
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()
        .expect("ggez could not be started");

    let app_config = AppConfig {
        width: 64,
        height: 64,
    };

    let mut app = App::new(&mut ctx, &app_config);

    // start gameloop; returns error during execution
    match event::run(&mut ctx, &mut event_loop, &mut app) {
        Ok(_) => println!("clean exit"),
        Err(e) => println!("error occured during execution: {}", e),
    }
}
