// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::prelude::*;

pub trait TryInstanceFrom<T, U>
    where T: NativeClass + gdnative::prelude::NativeClass<Base=U>,
          U: GodotObject + SubClass<Node>
{
    fn try_instance_from<'l>(node: Option<Ref<Node>>) -> Option<RefInstance<'l, T, Shared>> {
        node.map(|node| unsafe { node.assume_safe() })
            .and_then(|node| node.cast::<U>())
            .and_then(|node| node.cast_instance::<T>())
    }
}
