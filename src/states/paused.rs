use amethyst::{StateData, StateEvent, Trans};
use log::trace;

use crate::game_data::SurvivalState;
use crate::states::Running;
use crate::SurvivalData;

pub struct Paused;

impl<'a, 'b> amethyst::State<SurvivalData<'a, 'b>, StateEvent> for Paused {
    fn on_start(&mut self, _: StateData<'_, SurvivalData<'_, '_>>) {
        trace!("Changed state to Paused");
    }

    fn on_pause(&mut self, _: StateData<'_, SurvivalData<'_, '_>>) {}

    fn handle_event(
        &mut self,
        data: StateData<'_, SurvivalData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<SurvivalData<'a, 'b>, StateEvent> {
        amethyst_imgui::handle_imgui_events(data.world, &event);

        // Wait for player input and trans if we get it.

        Trans::None
    }

    fn update(
        &mut self,
        data: StateData<'_, SurvivalData<'_, '_>>,
    ) -> Trans<SurvivalData<'a, 'b>, StateEvent> {
        if data.data.update(&data.world, SurvivalState::Paused) != SurvivalState::Paused {
            return Trans::Push(Box::new(Running));
        }

        Trans::None
    }
}
