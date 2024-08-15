use bevy::ecs::{system::RunSystemOnce, world::Command};

use crate::{camera::BACKGROUND_COLOR, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<ScreenTransition>()
        .add_systems(Startup, setup_transition_overlay)
        .add_systems(
            Update,
            (
                start_transition_out.run_if(state_changed::<ScreenTransition>),
                start_transition_in,
            ),
        );
}

#[allow(dead_code)]
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum ScreenTransition {
    #[default]
    Done,
    TransitioningOut(Screen),
    TransitioningIn,
}

#[derive(Debug)]
pub struct TransitionScreen(Screen);
impl Command for TransitionScreen {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(
            self,
            |In(trans_screen): In<TransitionScreen>,
             mut next: ResMut<NextState<ScreenTransition>>| {
                next.set(ScreenTransition::TransitioningOut(trans_screen.0));
            },
        );
    }
}

pub(crate) trait TransitionScreenCommandExt {
    fn transition_to_screen(&mut self, next_screen: Screen);
}

impl<'w, 's> TransitionScreenCommandExt for Commands<'w, 's> {
    fn transition_to_screen(&mut self, next_screen: Screen) {
        self.add(TransitionScreen(next_screen));
    }
}

#[derive(Component)]
struct TransitionImage;

fn setup_transition_overlay(mut cmd: Commands) {
    cmd.spawn((
        Name::new("transition"),
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Vw(100.),
                height: Val::Vw(100.),
                ..default()
            },
            background_color: BACKGROUND_COLOR.into(),
            ..default()
        },
        TransitionImage,
    ))
    .insert(Animator::new(delay_tween(
        ui_bg_color_tween(
            BACKGROUND_COLOR.with_alpha(0.0),
            800,
            EaseFunction::QuadraticInOut,
        ),
        300,
    )));
}

fn start_transition_out(
    next_transition_state: ResMut<State<ScreenTransition>>,
    mut cmd: Commands,
    transition_img_q: Query<Entity, With<TransitionImage>>,
) {
    if !matches!(
        next_transition_state.get(),
        ScreenTransition::TransitioningOut(_)
    ) {
        return;
    }

    let e = or_return!(transition_img_q.get_single());
    cmd.tween_ui_bg_color(e, BACKGROUND_COLOR, 600, EaseFunction::QuadraticInOut);
}

fn start_transition_in(
    screen_trans: Res<State<ScreenTransition>>,
    mut next_screen_trans: ResMut<NextState<ScreenTransition>>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut cmd: Commands,
    mut tween_evr: EventReader<TweenCompleted>,
    transition_img_q: Query<Entity, With<TransitionImage>>,
) {
    if let ScreenTransition::TransitioningOut(screen) = screen_trans.get() {
        let e = or_return_quiet!(tween_evr
            .read()
            .find(|ev| transition_img_q.contains(ev.entity)))
        .entity;

        next_screen_trans.set(ScreenTransition::TransitioningIn);
        next_screen.set(screen.clone());
        cmd.tween_ui_bg_color(
            e,
            BACKGROUND_COLOR.with_alpha(0.0),
            600,
            EaseFunction::QuadraticInOut,
        );
    }
}
