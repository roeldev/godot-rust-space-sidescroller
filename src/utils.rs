// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::prelude::*;
use gdnative::{Ref, GodotObject};
use std::ops::Deref;

#[allow(dead_code)]
pub fn find_node<T>(owner: &Node, name: &str) -> Ref<T, Shared>
    where T: GodotObject
{
    owner
        .find_node(GodotString::from_str(name), false, false)
        .expect(format!("Failed to find `{}` node", name).deref())
        .to_variant()
        .try_to_object::<T>()
        .expect(format!("Failed to cast `{}` node", name).deref())
}

#[allow(dead_code)]
pub fn preload(path: &str) -> Ref<PackedScene, Shared> {
    let scene = ResourceLoader::godot_singleton()
        .load(path, "PackedScene", false)
        .expect(format!("Failed to preload {}", path).deref());

    let scene = unsafe { scene.assume_unique().into_shared() };
    return scene.cast::<PackedScene>()
        .expect("Failed to cast Resource to PackedScene");
}

#[allow(dead_code)]
pub fn try_variant_to_instance(variant: Variant, edit_state: i64) -> Option<Ref<Node, Shared>> {
    let scene: Ref<PackedScene, Shared> = variant.try_to_object::<PackedScene>()?;
    let scene = unsafe { scene.assume_safe() };
    return scene.instance(edit_state);
}
