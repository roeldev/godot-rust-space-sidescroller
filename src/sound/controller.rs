// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::ops::Deref;

use gdnative::api::AudioStreamPlayer;
use gdnative::prelude::*;

use crate::utils::singleton::SingletonInstance;
use crate::utils::utils::get_node_as;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct SoundController {}

#[methods]
impl SoundController {
    fn new(_owner: &Node) -> Self {
        SoundController {}
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("> sound controller ready");
    }

    pub fn play_sound(&self, owner: &Node, name: &str) {
        get_node_as::<AudioStreamPlayer, _>(owner, name, |sound| {
            sound.play(0.0);
        }).expect(format!("Failed to play sound `{}`", name).deref());
    }
}

impl SingletonInstance<Self, Node> for SoundController {
    fn node_path<'a>() -> &'a str { "/root/World/SoundController" }
}