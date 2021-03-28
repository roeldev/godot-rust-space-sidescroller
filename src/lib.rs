// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::prelude::*;

mod global;
mod world;
mod player;
mod enemies;
mod particles;
mod signals;
mod utils;

fn init(handle: InitHandle) {
    handle.add_class::<global::Global>();
    handle.add_class::<world::World>();
    handle.add_class::<player::Player>();
    handle.add_class::<player::PlayerBullet>();
    handle.add_class::<player::PlayerDestroyParticles>();
    handle.add_class::<enemies::EnemyGenerator>();
    handle.add_class::<enemies::Enemy1>();
    handle.add_class::<particles::DestroyParticles>();
}

godot_init!(init);
