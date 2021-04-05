// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::ops::Deref;

use gdnative::prelude::*;

pub trait SingletonInstance<T, U>
    where T: NativeClass + gdnative::prelude::NativeClass<Base=U>,
          U: GodotObject + SubClass<Node>
{
    fn node_path<'a>() -> &'a str;

    // guarantees the return of a RefInstance
    fn singleton(node: &Node) -> RefInstance<T, Shared> {
        Self::try_singleton(node)
            .expect(format!("Failed to find instance from path `{}`", Self::node_path()).deref())
    }

    // tries to get a RefInstance, might return None
    fn try_singleton(node: &Node) -> Option<RefInstance<T, Shared>> {
        node.get_node_or_null(Self::node_path())
            .map(|node| unsafe { node.assume_safe() })
            .and_then(|node| { node.cast::<U>() })
            .and_then(|instance| { RefInstance::<T, _>::try_from_base(instance) })
    }

    fn try_do<F>(node: &Node, func: F)
        where F: FnOnce(&T, TRef<U, Shared>),
              <T as NativeClass>::UserData: gdnative::nativescript::Map
    {
        match Self::try_singleton(node) {
            Some(instance) => {
                match instance.map(func) {
                    Ok(_) => {}
                    Err(_) => {}
                };
            }
            None => {}
        };
    }
}
