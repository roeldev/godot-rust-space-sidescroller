// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::ops::Deref;

use gdnative::prelude::*;
use gdnative::Ref;

#[allow(dead_code)]
pub fn preload(path: &str) -> Ref<PackedScene, Shared> {
    ResourceLoader::godot_singleton()
        .load(path, "PackedScene", false)
        .map(|resource| unsafe { resource.assume_unique() })
        .map(|resource| { resource.into_shared() })
        .and_then(|scene| { scene.cast::<PackedScene>() })
        .expect(format!("Failed to preload {}", path).deref())
}

#[allow(dead_code)]
pub fn get_node_as<'a, U, F>(owner: &'a Node, name: &'a str, mut func: F) -> Option<TRef<'a, U>>
    where U: SubClass<Node>,
          F: FnMut(TRef<U>)
{
    let node = unsafe { owner.get_node_as::<U>(name) };
    match node {
        Some(node) => {
            func(node);
            Some(node)
        }
        _ => { None }
    }
}