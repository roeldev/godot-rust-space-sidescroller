// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::prelude::*;

mod world;
mod background;
mod player;
mod enemy;
mod particles;
mod signals;
mod sound;
mod ui;
mod utils;

fn init(handle: InitHandle) {
    handle.add_class::<world::World>();
    handle.add_class::<background::ParallaxStarBackground>();

    handle.add_class::<ui::PlayerHealth>();
    handle.add_class::<ui::PlayerScore>();

    handle.add_class::<player::Player>();
    handle.add_class::<player::PlayerBullet>();
    handle.add_class::<player::effects::PlayerDestroyParticles>();

    handle.add_class::<enemy::EnemyGenerator>();
    handle.add_class::<enemy::Enemy>();

    handle.add_class::<particles::DestroyParticles>();

    handle.add_class::<sound::SoundController>();
}

godot_init!(init);
