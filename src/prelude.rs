// prelude module to simplify common imports
#![allow(unused_imports)]

pub(crate) use crate::tween::*;
pub(crate) use crate::{
    assets::{MusicAssets, SfxAssets, SpriteAssets},
    audio::{
        music::{MusicCommands, MusicTrack},
        sfx::{Sfx, SfxCommands},
    },
    ext::*,
    math::*,
    screens::Screen,
    theme::prelude::*,
    AppSet,
};
pub(crate) use bevy::{prelude::*, utils::HashMap};
pub(crate) use bevy_tweening::{
    asset_animator_system, component_animator_system, Animator, AssetAnimator, Ease, EaseFunction,
};
pub(crate) use rand::prelude::*;
