use bevy::prelude::*;

use crate::state_in_game_mod::{Bird, DebugText, SnakeHead, SnakeSegment};

pub fn render_snake_head(mut snake_head_query: Query<(&mut SnakeHead, &mut Transform)>) {
    if let Ok((mut snake_head, mut transform)) = snake_head_query.single_mut() {
        if snake_head.updated {
            transform.translation.x = snake_head.position.to_bevy_x();
            transform.translation.y = snake_head.position.to_bevy_y();

            snake_head.updated = false;
        }
    }
}
pub fn render_bird(mut queried_entities: Query<(&mut Bird, &mut Transform)>) {
    if let Ok((mut bird, mut transform)) = queried_entities.single_mut() {
        if bird.updated {
            transform.translation.x = bird.position.to_bevy_x();
            transform.translation.y = bird.position.to_bevy_y();

            bird.updated = false;
        }
    }
}

pub fn render_segment(mut queried_entities: Query<(&mut SnakeSegment, &mut Transform)>) {
    for (mut segment, mut transform) in queried_entities.iter_mut() {
        if segment.updated {
            transform.translation.x = segment.position.to_bevy_x();
            transform.translation.y = segment.position.to_bevy_y();

            segment.updated = false;
        }
    }
}

pub fn render_debug_text(mut debug_text_query: Query<(&mut DebugText, &mut Text)>) {
    if let Ok((debug_text, mut text)) = debug_text_query.single_mut() {
        *text = Text::new(&debug_text.bird_position);
    }
}
