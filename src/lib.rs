// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::prelude::*;

extern crate rand;

mod world;
mod player;
mod player_bullet;
mod utils;

fn init(handle: InitHandle) {
    handle.add_class::<world::World>();
    handle.add_class::<player::Player>();
    handle.add_class::<player_bullet::PlayerBullet>()
}

godot_init!(init);
