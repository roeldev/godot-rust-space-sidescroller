// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::prelude::*;
use std::borrow::Borrow;

pub const INSTANCE_NODE: &str = "instance_node";

pub fn instance_node_signal<T>(builder: &ClassBuilder<T>)
    where T: NativeClass {
    let scene = Variant::from_object::<Ref<PackedScene>>(PackedScene::new().into_shared());
    builder.add_signal(Signal {
        name: INSTANCE_NODE,
        args: &[
            SignalArgument {
                name: "node",
                default: scene.clone(),
                export_info: ExportInfo::new(scene.get_type()),
                usage: PropertyUsage::DEFAULT,
            },
            SignalArgument {
                name: "location",
                default: Variant::from_vector2(Vector2::zero().borrow()),
                export_info: ExportInfo::new(VariantType::Vector2),
                usage: PropertyUsage::DEFAULT,
            },
        ],
    });
}