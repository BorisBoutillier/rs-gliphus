use bracket_lib::prelude::*;
use legion::prelude::*;
mod components;
use components::{AtExit, Dead, Player, Position, Renderable};
mod glyphs;
mod gui;
mod level;
mod map;
mod player;
mod systems;

pub const TERM_WIDTH: i32 = 40;
pub const TERM_HEIGHT: i32 = 30;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RunState {
    Menu,
    LoadLevel,
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
            RunState::LoadLevel => {
                level::load_level(self);
                ctx.cls();
                self.draw_game(ctx);
                self.run_game_systems();
                newrunstate = RunState::GameAwaitingInput;
            }
            RunState::GameAwaitingInput => {
                ctx.cls();
                self.draw_game(ctx);
                newrunstate = player::game_turn_input(self, ctx);
            }
            RunState::GameTurn => {
                self.run_game_systems();
                let dead = <(Read<Position>,)>::query()
                    .filter(tag::<Player>() & tag::<Dead>())
                    .iter(&self.ecs)
                    .count()
                    > 0;
                let solved = <(Read<Position>,)>::query()
                    .filter(tag::<Player>() & tag::<AtExit>())
                    .iter(&self.ecs)
                    .count()
                    > 0;
                if dead {
                    newrunstate = RunState::GameEndDead;
                } else if solved {
                    newrunstate = RunState::GameLevelEnd;
                } else {
                    newrunstate = RunState::GameAwaitingInput;
                }
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
        let start_y = (TERM_HEIGHT - map.height) / 2;
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
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(TERM_WIDTH, TERM_HEIGHT)?
        .with_title("Griphus")
        .build()?;
    let mut gs = State::new();
    gs.rsrc.insert(RunState::Menu);
    gs.rsrc.insert(map::Map::empty());
    main_loop(context, gs)
}
