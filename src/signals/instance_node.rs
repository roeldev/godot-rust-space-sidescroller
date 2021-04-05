// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::borrow::Borrow;

use gdnative::prelude::*;

pub const SIGNAL: &str = "instance_node";

pub trait InstanceNodeEmitter<T>
    where T: NativeClass {
    fn add_instance_node_signal(builder: &ClassBuilder<T>) {
        let scene = Variant::from_object::<Ref<PackedScene>>(PackedScene::new().into_shared());
        builder.add_signal(Signal {
            name: SIGNAL,
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

    fn emit_instance_node(&self, owner: &Object, node: &Ref<PackedScene>, location: Vector2) {
        owner.emit_signal(SIGNAL, &[
            Variant::from_object(node),
            Variant::from_vector2(location.borrow()),
        ]);
    }
}
