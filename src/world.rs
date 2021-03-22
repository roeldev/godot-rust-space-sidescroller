// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::prelude::*;
use crate::utils::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct World {}

#[methods]
impl World {
    fn new(_owner: &Node) -> Self {
        World {}
    }

    #[export]
    fn _on_player_create_bullet(&self, owner: &Node, bullet: Variant, location: Variant) {
        let bullet_instance = try_variant_to_instance(bullet, PackedScene::GEN_EDIT_STATE_DISABLED).unwrap();
        let bullet_instance = unsafe { bullet_instance.assume_safe() };
        let bullet_instance = bullet_instance.cast::<Sprite>()
            .expect("Failed to cast bullet Node to Sprite");

        owner.add_child(bullet_instance, false);

        let location = location.try_to_vector2().expect("Failed to try location as Vector2");
        let mut pos = bullet_instance.global_position();
        pos.y = location.y;
        bullet_instance.set_global_position(pos);
    }
}
