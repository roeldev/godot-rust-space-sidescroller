// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::*;
use crate::utils::convert::TryInstanceFrom;

#[derive(NativeClass)]
#[inherit(Sprite)]
pub struct PlayerBullet {
    #[property(default = 100.0)]
    speed: f32,
    #[property(default = 1)]
    damage: u8,
}

#[methods]
impl PlayerBullet {
    fn new(_owner: &Sprite) -> Self {
        PlayerBullet {
            speed: 100.0,
            damage: 1,
        }
    }

    pub fn get_damage(&self) -> u8 { self.damage }

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

impl TryInstanceFrom<Self, Sprite> for PlayerBullet {}

// impl<'l> TryFrom<Option<Ref<Node>>> for PlayerBullet {
//     type Error = ();
//
//     fn try_instance_from(node: Option<Ref<Node>>) -> Result<RefInstance<'l, Self, Shared>, Self::Error> {
//         node.map(|node| unsafe { node.assume_safe() })
//             .and_then(|node| node.cast::<Sprite>())
//             .and_then(|node| node.cast_instance::<Self>())
//             .ok_or(Self::Error)
//     }
// }