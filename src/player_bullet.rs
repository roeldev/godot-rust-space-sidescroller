// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::prelude::*;

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

    #[export]
    fn _process(&self, owner: &Sprite, delta: f32) {
        let mut pos = owner.global_position();
        pos.x += self.speed * delta;
        owner.set_global_position(pos);
    }
}
