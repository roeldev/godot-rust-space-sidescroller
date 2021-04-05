// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::*;
use crate::utils::singleton::SingletonInstance;

pub const ON_INSTANCE_NODE: &str = "_on_instance_node";

#[derive(NativeClass)]
#[inherit(Node)]
pub struct World {}

#[methods]
impl World {
    fn new(_owner: &Node) -> Self {
        World {}
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("> world ready");
    }

    #[export]
    fn _on_instance_node(&self, owner: &Node, scene: Ref<PackedScene, Shared>, location: Vector2) {
        let scene = unsafe { scene.assume_safe() };
        let node = scene.instance(PackedScene::GEN_EDIT_STATE_DISABLED)
            .map(|node| unsafe { node.assume_safe() })
            .unwrap();

        owner.add_child(node, false);
        match node.cast::<Node2D>() {
            Some(node) => { node.set_global_position(location); }
            None => {}
        }
    }
}

impl SingletonInstance<Self, Node> for World {
    fn node_path<'a>() -> &'a str { "/root/World" }
}