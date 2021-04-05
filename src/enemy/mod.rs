// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

pub use enemy::Enemy;
pub use generator::EnemyGenerator;

mod enemy;
mod generator;

const RES_PARTICLES: &str = "res://enemies/EnemyDestroyParticles.tscn";
const GROUP_DAMAGER: &str = "enemy_damager";
