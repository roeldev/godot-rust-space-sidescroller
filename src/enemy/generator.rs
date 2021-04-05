// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::borrow::Borrow;

use gdnative::api::Position2D;
use rand::Rng;

use crate::*;
use crate::signals::instance_node::InstanceNodeEmitter;

#[derive(NativeClass)]
#[inherit(Position2D)]
#[register_with(Self::register_signals)]
pub struct EnemyGenerator {
    #[property(default = 3)]
    enemy_number: u8,

    #[property()]
    enemy1_scene: Ref<PackedScene>,
    #[property()]
    enemy2_scene: Ref<PackedScene>,
    #[property()]
    enemy3_scene: Ref<PackedScene>,
}

impl InstanceNodeEmitter<Self> for EnemyGenerator {}

#[methods]
impl EnemyGenerator {
    fn register_signals<'a>(builder: &ClassBuilder<Self>) {
        Self::add_instance_node_signal(builder);
    }

    fn new(_owner: &Position2D) -> Self {
        EnemyGenerator {
            enemy_number: 3,
            enemy1_scene: PackedScene::new().into_shared(),
            enemy2_scene: PackedScene::new().into_shared(),
            enemy3_scene: PackedScene::new().into_shared(),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Position2D) {
        if self.enemy_number < 1 {
            self.enemy_number = 1;
        }

        godot_print!("> enemy generator ready");
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_Timer_timeout(&self, owner: &Position2D) {
        let mut rng = rand::thread_rng();
        let enemy_scene = match rng.gen_range(0..self.enemy_number) {
            1 => self.enemy2_scene.borrow(),
            2 => self.enemy3_scene.borrow(),
            _ => self.enemy1_scene.borrow(),
        };

        self.emit_instance_node(owner, enemy_scene, Vector2::new(
            owner.global_position().x,
            rng.gen_range(0.0..90.0),
        ));
    }
}

