use steel::steel_vm::builtin::BuiltInModule;

use crate::runtime::persist::PersistStore;
use crate::{bindings, runtime, types};

pub const PRELUDE_SCM: &str = include_str!("../scheme/prelude.scm");

pub fn build_macroquad_module(persist: &PersistStore) -> BuiltInModule {
    let mut module = BuiltInModule::new("macroquad");
    types::register_types(&mut module);
    bindings::math::register(&mut module);
    bindings::color::register(&mut module);
    bindings::shapes2d::register(&mut module);
    bindings::camera::register(&mut module);
    bindings::models3d::register(&mut module);
    bindings::input::register(&mut module);
    bindings::text::register(&mut module);
    bindings::window::register(&mut module);
    runtime::persist::register(&mut module, persist);
    module
}
