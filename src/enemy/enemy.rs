// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::borrow::{Borrow, BorrowMut};

use gdnative::api::Area2D;

use crate::*;
use crate::enemy::*;
use crate::utils::convert::TryInstanceFrom;
use crate::utils::singleton::SingletonInstance;
use crate::utils::utils;
use crate::player::{Player, PlayerBullet};
use crate::signals::*;
use crate::signals::instance_node::InstanceNodeEmitter;
use crate::sound::SoundController;
use crate::world::World;

#[derive(NativeClass)]
#[inherit(Sprite)]
#[register_with(Self::register_signals)]
pub struct Enemy {
    #[property(default = 1)]
    health: u8,
    #[property(default = 50.0)]
    speed: f32,
    #[property(default = 5)]
    point_value: u32,

    particles_scene: Ref<PackedScene>,
}

impl InstanceNodeEmitter<Self> for Enemy {}

#[methods]
impl Enemy {
    fn register_signals<'a>(builder: &ClassBuilder<Self>) {
        Self::add_instance_node_signal(builder);
    }

    fn new(_owner: &Sprite) -> Self {
        Enemy {
            health: 1,
            speed: 50.0,
            point_value: 5,
            particles_scene: PackedScene::new().into_shared(),
        }
    }

    #[export]
    pub fn destroy(&self, owner: &Sprite) {
        SoundController::try_do(owner, |sc, sc_node| {
            sc.play_sound(sc_node.as_ref(), sound::EXPLOSION)
        });

        self.emit_instance_node(
            owner,
            self.particles_scene.borrow(),
            owner.global_position().clone(),
        );
        owner.queue_free();
    }

    #[export]
    fn _ready(&mut self, owner: &Sprite) {
        self.particles_scene = utils::preload(RES_PARTICLES);

        World::singleton(owner).borrow()
            .map(|_, target| {
                owner.connect(
                    instance_node::SIGNAL,
                    target,
                    world::ON_INSTANCE_NODE,
                    VariantArray::new_shared(),
                    1,
                )
            })
            .unwrap()
            .expect("Failed to connect Enemy1 to World");
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
    fn _on_Hitbox_area_entered(&mut self, owner: &Sprite, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        if !area.is_in_group(GROUP_DAMAGER) {
            return;
        }

        self.health -= PlayerBullet::try_instance_from(area.get_parent())
            .expect("Failed to get bullet from Area2D")
            .borrow()
            .map(|bullet, bullet_node| {
                bullet_node.queue_free();
                bullet.get_damage()
            })
            .unwrap_or(1);

        if self.health == 0 {
            Player::singleton(owner)
                .borrow_mut()
                .map_mut(|player, _| {
                    player.add_score(self.point_value);
                })
                .expect("Failed to add score");

            self.destroy(owner);
        }
    }
}

impl TryInstanceFrom<Self, Sprite> for Enemy {}