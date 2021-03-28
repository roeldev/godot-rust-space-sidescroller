// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::*;
use crate::global::Global;
use gdnative::api::{Position2D, Area2D};
use std::borrow::Borrow;
use rand::Rng;

const RES_ENEMY1: &str = "res://Enemies/Enemy1.tscn";
const RES_PARTICLES: &str = "res://Enemies/EnemyDestroyParticles.tscn";
const GROUP_DAMAGER: &str = "enemy_damager";

#[derive(NativeClass)]
#[inherit(Position2D)]
#[register_with(Self::register_signals)]
pub struct EnemyGenerator {
    enemy_scene: Ref<PackedScene>
}

#[methods]
impl EnemyGenerator {
    fn register_signals<'a>(builder: &ClassBuilder<Self>) {
        signals::instance_node_signal::<Self>(builder);
    }

    fn new(_owner: &Position2D) -> Self {
        EnemyGenerator {
            enemy_scene: PackedScene::new().into_shared()
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Position2D) {
        self.enemy_scene = utils::preload(RES_ENEMY1);
        godot_print!("> enemy generator ready");
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_Timer_timeout(&self, owner: &Position2D) {
        let mut rng = rand::thread_rng();
        owner.emit_signal(signals::INSTANCE_NODE, &[
            Variant::from_object(self.enemy_scene.borrow()),
            Variant::from_vector2(Vector2::new(
                owner.global_position().x,
                rng.gen_range(0.0..90.0),
            ).borrow())
        ]);
    }
}


#[derive(NativeClass)]
#[inherit(Sprite)]
#[register_with(Self::register_signals)]
pub struct Enemy1 {
    #[property(default = 50.0)]
    speed: f32,

    particles_scene: Ref<PackedScene>,
}

#[methods]
impl Enemy1 {
    fn register_signals<'a>(builder: &ClassBuilder<Self>) {
        signals::instance_node_signal::<Self>(builder);
    }

    fn new(_owner: &Sprite) -> Self {
        Enemy1 {
            speed: 50.0,
            particles_scene: PackedScene::new().into_shared(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Sprite) {
        self.particles_scene = utils::preload(RES_PARTICLES);

        let world = Global::instance(owner)
            .map(|global, _| { global.get_world() })
            .expect("Failed to get Global.world");

        owner.connect(
            signals::INSTANCE_NODE,
            world,
            world::ON_INSTANCE_NODE,
            VariantArray::new_shared(),
            1,
        ).expect("Failed to connect Enemy1 to World");

        // godot_print!("> enemy1 ready");
    }

    #[export]
    fn _process(&self, owner: &Sprite, delta: f32) {
        let mut pos = owner.global_position();
        pos.x -= self.speed * delta;
        owner.set_global_position(pos);

        if pos.x < -20.0 {
            owner.queue_free();
        }
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_Hitbox_area_entered(&self, owner: &Sprite, area: Variant) {
        let area: Ref<Area2D> = area.try_to_object::<Area2D>().expect("Failed to try area Variant to Area2D");
        let area = unsafe { area.assume_safe() };
        if !area.is_in_group(GROUP_DAMAGER) {
            return;
        }

        owner.emit_signal(signals::INSTANCE_NODE, &[
            Variant::from_object(self.particles_scene.borrow()),
            Variant::from_vector2(owner.global_position().clone().borrow()),
        ]);

        let bullet = area.get_parent().unwrap();
        let bullet = unsafe { bullet.assume_safe() };
        bullet.queue_free();
        owner.queue_free();
    }
}
