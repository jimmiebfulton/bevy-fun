use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub mod diagnostics;
pub mod loader;

pub struct UIPlugins;

impl PluginGroup for UIPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(diagnostics::DiagnosticsUIPlugin::default());
        group.add(loader::LoaderUIPlugin::default());
    }
}
