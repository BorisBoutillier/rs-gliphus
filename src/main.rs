use bracket_lib::prelude::*;
use legion::prelude::*;
mod components;
use components::{Position, Renderable};
use gui::{draw_ui, MainMenuSelection};
use turn_history::{TurnState, TurnsHistory};
mod ai;
mod glyphs;
mod gui;
mod level;
mod map;
mod player;
mod systems;
mod turn_history;

pub const TERM_WIDTH: i32 = 40;
pub const TERM_HEIGHT: i32 = 30;

#[derive(Clone, Copy, PartialEq)]
pub enum RunState {
    MainMenu { menu_selection: MainMenuSelection },
    LoadLevel(u64),
    GameAwaitingInput,
    GameTurn,
    GameDraw,
}
pub struct State {
    pub ecs: World,
    pub rsrc: Resources,
    schedule: Schedule,
    ai: Option<ai::AI>,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let runstate = *(self.rsrc.get::<RunState>().unwrap());
        let newrunstate;
        match runstate {
            RunState::MainMenu {
                menu_selection: selection,
            } => {
                let map = self.rsrc.get::<map::Map>().unwrap();
                let can_continue = map.level != 0;
                ctx.cls();
                let result = gui::main_menu(ctx, selection, can_continue);
                match result {
                    gui::MainMenuResult::NoSelection { selected } => {
                        newrunstate = RunState::MainMenu {
                            menu_selection: selected,
                        }
                    }
                    gui::MainMenuResult::Selected { selected } => match selected {
                        gui::MainMenuSelection::NewPlayerGame => {
                            self.ai = None;
                            newrunstate = RunState::LoadLevel(1)
                        }
                        gui::MainMenuSelection::NewAiGame => {
                            self.ai = Some(ai::AI::new());
                            newrunstate = RunState::LoadLevel(10001)
                        }
                        gui::MainMenuSelection::Continue => newrunstate = RunState::GameDraw,
                        gui::MainMenuSelection::Quit => {
                            ::std::process::exit(0);
                        }
                    },
                }
            }
            RunState::LoadLevel(level) => {
                level::load_level(self, level);
                self.run_game_systems();
                ctx.cls();
                self.draw_game(ctx);
                newrunstate = RunState::GameAwaitingInput;
            }
            RunState::GameAwaitingInput => {
                if let Some(ai) = self.ai.as_mut() {
                    newrunstate = ai.play_next_turn(&mut self.ecs, &mut self.rsrc, ctx);
                } else {
                    newrunstate = player::game_turn_input(self, ctx);
                }
            }
            RunState::GameTurn => {
                self.run_game_systems();
                newrunstate = RunState::GameDraw;
            }
            RunState::GameDraw => {
                self.run_game_systems();
                ctx.cls();
                self.draw_game(ctx);
                let curstate = self.rsrc.get::<TurnsHistory>().unwrap().state;
                newrunstate = match curstate {
                    TurnState::PlayerDead => {
                        gui::draw_dead(self, ctx);
                        gui::game_end_dead_input(self, ctx)
                    }
                    TurnState::PlayerAtExit => {
                        gui::draw_level_solved(self, ctx);
                        gui::game_level_end_input(self, ctx)
                    }
                    TurnState::Running => RunState::GameAwaitingInput,
                };
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
            ai: None,
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
    gs.rsrc.insert(RunState::MainMenu {
        menu_selection: MainMenuSelection::NewPlayerGame,
    });
    gs.rsrc.insert(map::Map::empty());
    gs.rsrc.insert(RandomNumberGenerator::new());
    main_loop(context, gs)
}
