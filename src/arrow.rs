use bevy::prelude::*;

use crate::Arrow;
pub fn spawn_arrow(mut commands: Commands) {
    let note_size = Vec2::new(50., 50.);
    for i in 0..4 {
        commands.spawn((
            Arrow { line: i },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(note_size),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    -105. + (i as f32 * 70.),
                    -300.,
                    0.,
                )),
                ..default()
            },
        ));
    }
}

pub fn arrow_button(keyboard: Res<Input<KeyCode>>, mut arrow_query: Query<(&mut Sprite, &Arrow)>) {
    for (mut arrow_color, arrow_line) in arrow_query.iter_mut() {
        if arrow_line.line == 0 && keyboard.pressed(KeyCode::D) {
            arrow_color.color = Color::rgb_u8(160, 0, 160);
        } else if arrow_line.line == 1 && keyboard.pressed(KeyCode::F) {
            arrow_color.color = Color::rgb_u8(0, 160, 160);
        } else if arrow_line.line == 2 && keyboard.pressed(KeyCode::K) {
            arrow_color.color = Color::rgb_u8(0, 160, 0);
        } else if arrow_line.line == 3 && keyboard.pressed(KeyCode::L) {
            arrow_color.color = Color::rgb_u8(160, 0, 0);
        } else {
            arrow_color.color = Color::WHITE;
        }
    }
}
