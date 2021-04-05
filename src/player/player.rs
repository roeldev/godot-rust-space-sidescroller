// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::borrow::Borrow;

use gdnative::api::Area2D;
use interpolation::Lerp;

use crate::*;
use crate::enemy::Enemy;
use crate::utils::convert::TryInstanceFrom;
use crate::utils::singleton::SingletonInstance;
use crate::utils::utils;
use crate::player::*;
use crate::signals::instance_node::InstanceNodeEmitter;
use crate::sound::SoundController;

#[derive(NativeClass)]
#[inherit(Sprite)]
#[register_with(Self::register_signals)]
pub struct Player {
    bullet_scene: Ref<PackedScene>,
    particles_scene: Ref<PackedScene>,
    reload_timer_node: Ref<Node>,

    can_shoot: bool,
    health: u8,
    score: u32,
}

#[methods]
impl Player {
    fn register_signals(builder: &ClassBuilder<Self>) {
        Self::add_instance_node_signal(builder);
    }

    fn new(_owner: &Sprite) -> Self {
        Player {
            bullet_scene: PackedScene::new().into_shared(),
            particles_scene: PackedScene::new().into_shared(),
            reload_timer_node: unsafe { Node::new().assume_shared() },

            can_shoot: true,
            health: 3,
            score: 0,
        }
    }

    pub fn destroy(&self, owner: &Sprite) {
        self.emit_instance_node(
            owner,
            self.particles_scene.borrow(),
            owner.global_position().clone(),
        );
        owner.queue_free();
    }

    pub fn get_viewport(&self, owner: &Node) -> TRef<Viewport> {
        let viewport = owner.get_viewport().expect("Failed to get Viewport");
        unsafe { viewport.assume_safe() }
    }

    fn get_reload_timer(&self) -> TRef<Timer> {
        let reload_timer = unsafe { self.reload_timer_node.assume_safe() };
        reload_timer.cast::<Timer>().expect("Failed to cast ReloadTimer node to Timer")
    }

    pub fn get_health(&self) -> u8 { self.health }

    pub fn get_score(&self) -> u32 { self.score }

    pub fn add_score(&mut self, add: u32) { self.score += add; }

    #[export]
    fn _ready(&mut self, owner: &Sprite) {
        self.bullet_scene = utils::preload(RES_BULLET);
        self.particles_scene = utils::preload(RES_PARTICLES);
        self.reload_timer_node = owner
            .get_node("ReloadTimer")
            .expect("Failed to get ReloadTimer");

        let mut pos = owner.global_position();
        pos.y = self.get_viewport(owner).get_mouse_position().y;
        owner.set_global_position(pos);
        godot_print!("> player ready");
    }

    #[export]
    fn _process(&mut self, owner: &Sprite, _delta: f32) {
        let mut pos = owner.global_position();
        pos.y = pos.y.lerp(&self.get_viewport(owner).get_mouse_position().y, &0.2);
        owner.set_global_position(pos);

        if Input::godot_singleton().is_action_pressed("shoot") && self.can_shoot {
            SoundController::try_do(owner, |sc, sc_node| {
                sc.play_sound(sc_node.as_ref(), sound::SHOOT);
            });

            owner.emit_signal(signals::instance_node::SIGNAL, &[
                Variant::from_object(self.bullet_scene.borrow()),
                Variant::from_vector2(&pos)
            ]);

            self.can_shoot = false;
            self.get_reload_timer().start(0.25);
        }
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_Hitbox_area_entered(&mut self, owner: &Sprite, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        if !area.is_in_group("enemy") {
            return;
        }

        Enemy::try_instance_from(area.get_parent())
            .expect("Failed to get enemy from Area2D")
            .borrow()
            .map(|enemy, node| { enemy.destroy(node.borrow()); })
            .expect("Failed to destroy enemy");

        self.health -= 1;
        if self.health == 0 {
            SoundController::try_do(owner, |sc, sc_node| {
                sc.play_sound(sc_node.as_ref(), sound::EXPLOSION)
            });

            self.destroy(owner);
        }
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_ReloadTimer_timeout(&mut self, _owner: &Sprite) {
        self.can_shoot = true;
        self.get_reload_timer().stop();
    }
}

impl SingletonInstance<Self, Sprite> for Player {
    fn node_path<'a>() -> &'a str { "/root/World/Player" }
}

impl InstanceNodeEmitter<Self> for Player {}