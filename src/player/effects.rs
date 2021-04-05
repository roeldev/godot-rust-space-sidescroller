// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::api::CPUParticles2D;

use crate::*;

#[derive(NativeClass)]
#[inherit(CPUParticles2D)]
pub struct PlayerDestroyParticles {}

#[methods]
impl PlayerDestroyParticles {
    fn new(_owner: &CPUParticles2D) -> Self {
        PlayerDestroyParticles {}
    }

    #[export]
    fn _ready(&self, owner: &CPUParticles2D) {
        owner.set_one_shot(true);
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_Timer_timeout(&self, owner: &CPUParticles2D) {
        owner.get_tree()
            .map(|tree| unsafe { tree.assume_safe() })
            .and_then(|tree| { tree.change_scene("res://ui/TitleScreen.tscn").ok() })
            .expect("Failed to reload");

        godot_print!("-----------");
    }
}
