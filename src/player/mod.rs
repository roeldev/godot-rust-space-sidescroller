// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

pub use bullet::*;
pub use player::*;

mod bullet;
pub mod effects;
mod player;

pub const RES_BULLET: &str = "res://player/bullet/PlayerBullet.tscn";
pub const RES_PARTICLES: &str = "res://player/PlayerDestroyParticles.tscn";
