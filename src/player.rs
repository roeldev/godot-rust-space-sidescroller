// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::prelude::*;
use gdnative::api::{KinematicBody2D};
use interpolation::Lerp;
use std::borrow::Borrow;
use crate::utils::*;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register_signals)]
pub struct Player {
    bullet: Ref<PackedScene>
}

#[methods]
impl Player {
    fn register_signals(builder: &ClassBuilder<Self>) {
        let scene = Variant::from_object::<Ref<PackedScene>>(PackedScene::new().into_shared());
        builder.add_signal(Signal {
            name: "create_bullet",
            args: &[
                SignalArgument {
                    name: "bullet",
                    default: scene.clone(),
                    export_info: ExportInfo::new(scene.get_type()),
                    usage: PropertyUsage::DEFAULT,
                },
                SignalArgument {
                    name: "location",
                    default: Variant::from_vector2(Vector2::zero().borrow()),
                    export_info: ExportInfo::new(VariantType::Vector2),
                    usage: PropertyUsage::DEFAULT,
                },
            ],
        });
    }

    fn new(_owner: &KinematicBody2D) -> Self {
        Player {
            bullet: PackedScene::new().into_shared()
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &KinematicBody2D) {
        self.bullet = preload("res://entities/PlayerBullet.tscn")
            .expect("Failed to preload PlayerBullet.tscn");
    }

    #[export]
    fn _process(&self, owner: &KinematicBody2D, _delta: f32) {
        let viewport = unsafe {
            owner.get_viewport()
                .expect("Failed to get Viewport")
                .assume_safe()
        };


        let mut pos = owner.global_position();
        pos.y = pos.y.lerp(&viewport.get_mouse_position().y, &0.2);
        owner.set_global_position(pos);

        let input = Input::godot_singleton();
        if input.is_action_just_pressed("shoot") {
            owner.emit_signal("create_bullet", &[
                Variant::from_object(self.bullet.borrow()),
                Variant::from_vector2(&pos)
            ]);
        }
    }
}
