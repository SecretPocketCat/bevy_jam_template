// prelude module to simplify common imports
#![allow(unused_imports)]

pub(crate) use bevy::prelude::*;
pub(crate) use rand::prelude::*;

pub(crate) mod tween {
    pub(crate) use crate::tween::*;
    pub(crate) use bevy_tweening::{
        asset_animator_system, component_animator_system, Animator, AssetAnimator, Ease,
        EaseFunction,
    };
}
