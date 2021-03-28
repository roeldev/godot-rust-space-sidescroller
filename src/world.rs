// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::prelude::*;
use crate::*;
use crate::global::Global;

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
    fn _ready(&self, owner: &Node) {
        Global::instance(owner)
            .map_mut(|global, _| unsafe {
                global.set_world(Some(owner.assume_shared()))
            })
            .expect("Failed to set Global.world");

        godot_print!("> world ready");
    }

    #[export]
    fn _exit_tree(&self, owner: &Node) {
        Global::instance(owner)
            .map_mut(|global, _| { global.set_world(None) })
            .expect("Failed to set Global.world");
    }

    #[export]
    fn _on_instance_node(&self, owner: &Node, node: Variant, location: Variant) -> TRef<Node2D> {
        let instance = utils::try_variant_to_instance(node, PackedScene::GEN_EDIT_STATE_DISABLED).unwrap();
        let instance = unsafe { instance.assume_safe() };
        let node = instance.cast::<Node2D>()
            .expect("Failed to cast `node` to Node2D");

        owner.add_child(node, false);
        node.set_global_position(location
            .try_to_vector2()
            .expect("Failed to try `location` as Vector2")
        );
        return node;
    }
}
