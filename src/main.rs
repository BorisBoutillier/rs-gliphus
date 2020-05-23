use legion::prelude::*;
use rltk::{GameState, Rltk};
mod components;
use components::{Position, Renderable};
mod level;
mod map;
mod player;
mod systems;

pub struct State {
    pub ecs: World,
    pub rsrc: Resources,
    schedule: Schedule,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        player::player_input(self, ctx);
        self.run_systems();
        let map = self.rsrc.get::<map::Map>().unwrap();
        let start_x = (80 - map.width) / 2;
        let start_y = (50 - map.height) / 2;
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
    fn run_systems(&mut self) {
        self.schedule.execute(&mut self.ecs, &mut self.rsrc);
    }
}

fn main() -> rltk::BError {
    let context = rltk::RltkBuilder::simple80x50()
        .with_title("Laseration")
        .build()?;
    let mut gs = State::new();
    level::load_level(&mut gs);
    rltk::main_loop(context, gs)
}
