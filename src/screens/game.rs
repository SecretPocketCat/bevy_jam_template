//! The screen state for the main game loop.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{game::level::SpawnLevel, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Game), spawn_level);
    app.add_systems(OnExit(Screen::Game), stop_music);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Game).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_level(mut commands: Commands) {
    commands.add(SpawnLevel);
    commands.play_music(MusicTrack::Game);
}

fn stop_music(mut commands: Commands) {
    commands.stop_music();
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::MainMenu);
}
