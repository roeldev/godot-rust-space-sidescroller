// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Global {
    world: Option<Ref<Node>>,
}

#[methods]
impl Global {
    pub fn instance(node: &Node) -> RefInstance<Self, Shared> {
        unsafe {
            node
                .get_node_as_instance::<Global>("/root/Global")
                .expect("Failed to get Global node instance")
        }
    }

    fn new(_owner: &Node) -> Self {
        Global {
            world: None
        }
    }

    pub fn get_world(&self) -> Ref<Node> {
        self.world.expect("Failed to get World from Global")
    }

    pub fn set_world(&mut self, world: Option<Ref<Node>>) {
        self.world = world;
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("> global ready");
    }
}
