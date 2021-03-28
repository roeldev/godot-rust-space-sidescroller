// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::*;
use gdnative::api::{Area2D, CPUParticles2D};
use interpolation::Lerp;
use std::borrow::Borrow;

const RES_BULLET: &str = "res://Player/Bullet/PlayerBullet.tscn";
const RES_PARTICLES: &str = "res://Player/PlayerDestroyParticles.tscn";

#[derive(NativeClass)]
#[inherit(Sprite)]
#[register_with(Self::register_signals)]
pub struct Player {
    bullet_scene: Ref<PackedScene>,
    particles_scene: Ref<PackedScene>,
    reload_timer_node: Ref<Node>,
    can_shoot: bool,
}

#[methods]
impl Player {
    fn register_signals<'a>(builder: &ClassBuilder<Self>) {
        signals::instance_node_signal::<Self>(builder);
    }

    fn new(_owner: &Sprite) -> Self {
        Player {
            bullet_scene: PackedScene::new().into_shared(),
            particles_scene: PackedScene::new().into_shared(),
            reload_timer_node: unsafe { Node::new().assume_shared() },
            can_shoot: true,
        }
    }

    pub fn get_viewport(&self, owner: &Sprite) -> TRef<Viewport> {
        let viewport = owner.get_viewport().expect("Failed to get Viewport");
        unsafe { viewport.assume_safe() }
    }

    fn get_reload_timer(&self) -> TRef<Timer> {
        let reload_timer = unsafe { self.reload_timer_node.assume_safe() };
        reload_timer.cast::<Timer>().expect("Failed to cast ReloadTimer node to Timer")
    }

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
            owner.emit_signal(signals::INSTANCE_NODE, &[
                Variant::from_object(self.bullet_scene.borrow()),
                Variant::from_vector2(&pos)
            ]);

            self.can_shoot = false;
            self.get_reload_timer().start(0.25);
        }
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_Hitbox_area_entered(&self, owner: &Sprite, area: Variant) {
        let area: Ref<Area2D> = area.try_to_object::<Area2D>().expect("Failed to try area Variant to Area2D");
        let area = unsafe { area.assume_safe() };
        if !area.is_in_group("enemy") {
            return;
        }

        let enemy = area.get_parent().unwrap();
        let enemy = unsafe { enemy.assume_safe() };

        enemy.queue_free();
        owner.queue_free();

        owner.emit_signal(signals::INSTANCE_NODE, &[
            Variant::from_object(self.particles_scene.borrow()),
            Variant::from_vector2(owner.global_position().clone().borrow()),
        ]);
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_ReloadTimer_timeout(&mut self, _owner: &Sprite) {
        self.can_shoot = true;
        self.get_reload_timer().stop();
    }
}

#[derive(NativeClass)]
#[inherit(Sprite)]
pub struct PlayerBullet {
    #[property(default = 100.0)]
    speed: f32,
}

#[methods]
impl PlayerBullet {
    fn new(_owner: &Sprite) -> Self {
        PlayerBullet {
            speed: 100.0,
        }
    }

    // #[export]
    // fn _ready(&self, _owner: &Sprite) {
    //     godot_print!("> player bullet ready");
    // }

    #[export]
    fn _process(&self, owner: &Sprite, delta: f32) {
        let mut pos = owner.global_position();
        pos.x += self.speed * delta;
        owner.set_global_position(pos);

        if pos.x > 180.0 {
            owner.queue_free();
        }
    }
}

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
        let tree = owner.get_tree().expect("Failed to get SceneTree");
        let tree = unsafe { tree.assume_safe() };
        tree.reload_current_scene().expect("Failed to reload");
        godot_print!("-----------");
    }
}
