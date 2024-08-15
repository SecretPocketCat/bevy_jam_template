use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::MainMenu), show_title_screen);
}

fn show_title_screen(mut cmd: Commands) {
    cmd.ui_root()
        .insert(StateScoped(Screen::MainMenu))
        .with_children(|children| {
            children.button("Play").observe(enter_playing);
            children.button("Credits").observe(enter_credits);

            #[cfg(not(target_family = "wasm"))]
            children.button("Exit").observe(exit_app);
        });

    cmd.play_music(MusicTrack::MainMenu);
}

fn enter_playing(_trigger: Trigger<OnPress>, mut cmd: Commands) {
    cmd.transition_to_screen(Screen::Game);
}

fn enter_credits(_trigger: Trigger<OnPress>, mut cmd: Commands) {
    cmd.transition_to_screen(Screen::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_trigger: Trigger<OnPress>, mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit::Success);
}
