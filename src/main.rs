use bracket_lib::prelude::*;
use legion::prelude::*;
mod components;
use components::{Position, Renderable};
use gui::draw_ui;
use turn_history::{TurnState, TurnsHistory};
mod glyphs;
mod gui;
mod level;
mod map;
mod player;
mod systems;
mod turn_history;

pub const TERM_WIDTH: i32 = 40;
pub const TERM_HEIGHT: i32 = 30;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RunState {
    Menu,
    LoadLevel(u64),
    GameAwaitingInput,
    GameTurn,
    GameEndDead,
    GameLevelEnd,
}
pub struct State {
    pub ecs: World,
    pub rsrc: Resources,
    schedule: Schedule,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let runstate = *(self.rsrc.get::<RunState>().unwrap());
        let newrunstate;
        match runstate {
            RunState::Menu => {
                ctx.cls();
                gui::draw_menu(ctx);
                newrunstate = gui::menu_input(self, ctx);
            }
            RunState::LoadLevel(level) => {
                level::load_level(self, level);
                self.run_game_systems();
                ctx.cls();
                self.draw_game(ctx);
                newrunstate = RunState::GameAwaitingInput;
            }
            RunState::GameAwaitingInput => {
                newrunstate = player::game_turn_input(self, ctx);
            }
            RunState::GameTurn => {
                self.run_game_systems();
                ctx.cls();
                self.draw_game(ctx);
                let history = self.rsrc.get::<TurnsHistory>().unwrap();
                newrunstate = match history.state {
                    TurnState::PlayerDead => RunState::GameEndDead,
                    TurnState::PlayerAtExit => RunState::GameLevelEnd,
                    TurnState::Running => RunState::GameAwaitingInput,
                };
            }
            RunState::GameEndDead => {
                ctx.cls();
                self.draw_game(ctx);
                gui::draw_dead(self, ctx);
                newrunstate = gui::game_end_dead_input(self, ctx);
            }
            RunState::GameLevelEnd => {
                ctx.cls();
                self.draw_game(ctx);
                gui::draw_level_solved(self, ctx);
                newrunstate = gui::game_level_end_input(self, ctx);
            }
        }
        self.rsrc.insert(newrunstate);
    }
}
impl State {
    fn new() -> State {
        let universe = Universe::new();
        let world = universe.create_world();
        let resources = Resources::default();
        State {
            ecs: world,
            rsrc: resources,
            schedule: systems::build_systems(),
        }
    }
    fn run_game_systems(&mut self) {
        self.schedule.execute(&mut self.ecs, &mut self.rsrc);
    }
    fn draw_game(&self, ctx: &mut BTerm) {
        let map = self.rsrc.get::<map::Map>().unwrap();
        let start_x = (TERM_WIDTH - map.width) / 2;
        let start_y = 11;
        map.draw(ctx, start_x, start_y);
        let mut data = <(Read<Position>, Read<Renderable>)>::query()
            .iter(&self.ecs)
            .collect::<Vec<_>>();
        data.sort_by(|d1, d2| d2.1.render_order.cmp(&d1.1.render_order));
        for (pos, render) in data.iter() {
            ctx.set(
                start_x + pos.x,
                start_y + pos.y,
                render.fg,
                render.bg,
                render.glyph,
            );
        }
        draw_ui(self, ctx);
    }
}

pub const TERM_UI_FONT: &str = "Bisasam_20x20.png";
embedded_resource!(UI_FONT, "../resources/Bisasam_20x20.png");

fn main() -> BError {
    link_resource!(UI_FONT, format!("resources/{}", TERM_UI_FONT));

    let context = BTermBuilder::new()
        .with_dimensions(TERM_WIDTH as u32, TERM_HEIGHT as u32)
        .with_tile_dimensions(20u32, 20u32)
        .with_title("Griphus")
        .with_font(TERM_UI_FONT, 20u32, 20u32)
        .with_simple_console(TERM_WIDTH as u32, TERM_HEIGHT as u32, TERM_UI_FONT)
        //.with_sparse_console_no_bg(TERM_WIDTH as u32, TERM_HEIGHT as u32, "Bisasam_16x16.png")
        //.with_sparse_console_no_bg(TERM_WIDTH as u32, TERM_HEIGHT as u32, "Bisasam_16x16.png")
        .build()?;

    let mut gs = State::new();
    gs.rsrc.insert(RunState::Menu);
    gs.rsrc.insert(map::Map::empty());
    main_loop(context, gs)
}
