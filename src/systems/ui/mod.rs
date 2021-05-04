use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub mod debug;
pub mod loader;

pub struct UIPlugins;

impl PluginGroup for UIPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(debug::DebugUIPlugin::default());
        group.add(loader::LoaderUIPlugin::default());
    }
}
