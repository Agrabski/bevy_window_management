# Bevy Window Management

A simple crate that allows you to set the window icon and taskbar progress indicator (currently only on windows).

Currently it is only compatible with Bevy 0.11.3

Modyfing taskbar progress indicator is only supported on windows and requires the `taskbar feature`

Example usage:
```rs
use bevy::{
    app::{App, Startup, Update},
    asset::AssetServer,
    ecs::system::{Res, ResMut},
    DefaultPlugins,
};
use bevy_window_management::{WindowManagement, WindowManagementPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins((WindowManagementPlugin, DefaultPlugins))
        .add_systems(
            Startup,
            |assets: Res<AssetServer>, mut window: ResMut<WindowManagement>| {
                window.window_icon = Some(assets.load("my_icon.png"));
            },
        )
        .add_systems(Update, |mut window: ResMut<WindowManagement>| {
            window.taskbar_progress =
                window
                    .taskbar_progress
                    .as_ref()
                    .map(|p| bevy_window_management::TaskbarProgress {
                        progress: p.progress + 1,
                        max: 100,
                    });
        });
    app.run();
}
```