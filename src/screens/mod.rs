//! The game's main screen states and transitions between them.

mod credits;
mod game;
mod loading;
mod main_menu;
mod splash;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        splash::plugin,
        loading::plugin,
        main_menu::plugin,
        credits::plugin,
        game::plugin,
    ));
}

#[allow(dead_code)]
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Loaded,
    MainMenu,
    Settings,
    Tutorial,
    Credits,
    Game,
    RestartGame,
    Score,
    Quit,
}
