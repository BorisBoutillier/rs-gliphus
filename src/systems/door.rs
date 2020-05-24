use crate::components::{Activable, Door, Renderable};
use crate::glyphs::*;
use crate::map;
use legion::prelude::*;

pub fn door_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("door_system")
        .read_resource::<map::Map>()
        .read_component::<Activable>()
        .with_query(<(Read<Door>,)>::query())
        .with_query(<(Write<Door>,)>::query())
        .build(|cmd, mut world, _, (query1, query2)| {
            let mut opened = vec![];
            for (door,) in query1.iter(&world) {
                let mut open = true;
                for activable in door.activations.iter() {
                    open = open && world.get_component::<Activable>(*activable).unwrap().active;
                }
                opened.push(open);
            }
            for (idx, (entity, (mut door,))) in query2.iter_entities_mut(&mut world).enumerate() {
                door.opened = opened[idx];
                if door.opened {
                    cmd.remove_component::<Renderable>(entity);
                } else {
                    cmd.add_component::<Renderable>(
                        entity,
                        Renderable {
                            glyph: DOOR_H_CLOSED,
                            fg: rltk::RGB::named(rltk::RED),
                            bg: rltk::RGB::named(rltk::BLACK),
                            render_order: 1,
                        },
                    );
                }
            }
        })
}
