#![allow(clippy::module_name_repetitions)]

use crate::actions;
use crate::actions::Action;
use crate::components;
use crate::settings::Context;
use crate::utils::{ComponentEventReader, HasChannel};
use amethyst::{
    core::components::Parent,
    core::transform::Transform,
    ecs::{Entities, Join, ReadExpect, ReadStorage, Resources, SystemData, WriteStorage},
};
use slog::slog_error;

use crate::tiles::{ReadTiles, TileEntities, Tiles};

#[derive(Default)]
pub struct System {
    action_reader: ComponentEventReader<components::Actionable, Action>,
}
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        ReadExpect<'s, crate::settings::Config>,
        ReadExpect<'s, Tiles>,
        Entities<'s>,
        ReadStorage<'s, components::Item>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, components::Actionable>,
        ReadStorage<'s, Parent>,
        ReadTiles<'s, TileEntities>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.action_reader.setup(res);
    }

    fn run(
        &mut self,
        (
            context,
            config,
            tiles,
            entities,
            items,
            transforms,
            mut actionables,
            _,
            tile_entities_map,
        ): Self::SystemData,
    ) {
        self.action_reader.maintain(&entities, &mut actionables);
        let mut events = Vec::new();
        for (entity, transform, actionable) in (&entities, &transforms, &mut actionables).join() {
            for event in self.action_reader.read(entity, actionable) {
                if let Action::TryPickup(target) = event {
                    match target {
                        actions::PickupTarget::Under => {
                            // Target there any other tile entities underneath us?
                            for entity in &tile_entities_map
                                .get(tiles.id_from_vector(
                                    tiles.world_to_tile(transform.translation(), &config),
                                ))
                                .unwrap()
                                .0
                            {
                                if let Some(_) = items.get(*entity) {
                                    // Its an item! We can get it.
                                    // TODO: allllll sorts of checks
                                    // rebroadcast the DoPickup event
                                    events.push((*entity, Action::DoPickup(*entity)));
                                }
                            }
                        }
                        actions::PickupTarget::Location(_) => {
                            slog_error!(context.logs.root, "Location Not implemented");
                        }
                        actions::PickupTarget::Entity(_) => {
                            slog_error!(context.logs.root, "Entity Not implemented");
                        }
                    }
                }
            }
        }

        // Emit all our generated events
        for event in events {
            if let Some(actionable) = actionables.get_mut(event.0) {
                actionable.channel_mut().single_write(event.1);
            }
        }
    }
}
