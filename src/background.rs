use bevy::prelude::*;
pub fn background_image(mut commands: Commands, file: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., -5.)),
        texture: file.load("./test.png"),
        ..default()
    });
}