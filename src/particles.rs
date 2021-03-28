// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::prelude::*;
use gdnative::api::{CPUParticles2D};

#[derive(NativeClass)]
#[inherit(CPUParticles2D)]
pub struct DestroyParticles {}

#[methods]
impl DestroyParticles {
    fn new(_owner: &CPUParticles2D) -> Self {
        DestroyParticles {}
    }

    #[export]
    fn _ready(&self, owner: &CPUParticles2D) {
        owner.set_one_shot(true);
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_Timer_timeout(&self, owner: &CPUParticles2D) {
        owner.queue_free();
    }
}
