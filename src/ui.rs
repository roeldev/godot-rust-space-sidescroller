// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::prelude::*;

use crate::utils::singleton::SingletonInstance;
use crate::player::Player;
use std::borrow::Borrow;

#[derive(NativeClass)]
#[inherit(Sprite)]
pub struct PlayerHealth {}

#[methods]
impl PlayerHealth {
    fn new(_owner: &Sprite) -> Self {
        PlayerHealth {}
    }

    #[export]
    fn _process(&self, owner: &Sprite, _delta: f32) {
        owner.set_frame(
            Player::try_singleton(owner).map_or(0, |instance| {
                instance.borrow()
                    .map(|player, _| { player.get_health() as i64 })
                    .expect("Failed to get Player health")
            })
        )
    }
}

#[derive(NativeClass)]
#[inherit(Label)]
pub struct PlayerScore {}

#[methods]
impl PlayerScore {
    fn new(_owner: &Label) -> Self {
        PlayerScore {}
    }

    #[export]
    fn _process(&self, owner: &Label, _delta: f32) {
        Player::try_do(owner, |player, _| {
            owner.set_text(format!("Score: {}", player.get_score()));
        })
    }
}