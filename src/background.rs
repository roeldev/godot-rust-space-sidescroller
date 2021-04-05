// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::api::ParallaxBackground;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(ParallaxBackground)]
pub struct ParallaxStarBackground {
    #[property(default = 1.0)]
    speed: f32,

    offset_scroll: Vector2,
}

#[methods]
impl ParallaxStarBackground {
    fn new(_owner: &ParallaxBackground) -> Self {
        ParallaxStarBackground {
            speed: 1.0,
            offset_scroll: Vector2::zero(),
        }
    }

    #[export]
    fn _process(&mut self, owner: &ParallaxBackground, delta: f32) {
        self.offset_scroll.x -= self.speed * delta;
        owner.set_scroll_offset(self.offset_scroll);
    }
}
