// https://www.shadertoy.com/view/XtGcDK
// by Wyatt
// License Creative Commons Attribution-NonCommercial-ShareAlike 3.0 Unported License

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};

use bevy_shadertoy_wgsl::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::GRAY))
        .insert_resource(ShadertoyCanvas {
            width: (960.0_f32 * 1.0).floor() as u32,
            height: (600.0_f32 * 1.0).floor() as u32,
            borders: 0.,
            position: Vec3::new(0.0, 0.0, 0.0),
			active: true
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
			window: WindowDescriptor {
				width: 960.,
				height: 600.,
				cursor_visible: true,
				monitor: MonitorSelection::Primary,
				position: WindowPosition::Centered,
				// present_mode: PresentMode::Immediate, // uncomment for unthrottled FPS
				..default()
			},
			..default()
		}))
        .add_plugin(ShadertoyPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup)
        // .add_system(update_common_uniform)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut st_res: ResMut<ShadertoyResources>,
) {
    let example = "fluid";
    st_res.include_debugger = false;

    let all_shader_handles: ShaderHandles =
        make_and_load_shaders2(example, &asset_server, st_res.include_debugger);

    commands.insert_resource(all_shader_handles);
}
