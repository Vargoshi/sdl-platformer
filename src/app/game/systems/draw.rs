use sdl2::{rect::Rect, render::Canvas, video::Window};

use crate::{app::game::{entity::Entity, Game}, draw::{SCREEN_WIDTH, ZOOM_FACTOR}};

pub(crate) fn system(game: &Game, canvas: &mut Canvas<Window>) -> Result<(), String> {
    
    for entity in &game.entities {
        if let Entity {
            pos: Some(pos),
            size: Some(size),
            draw: Some(draw),
            ..
        } = &entity
        {
            canvas.set_draw_color(draw.color);
            canvas.fill_rect(Rect::new(
                pos.x * ZOOM_FACTOR as i32,
                pos.y * ZOOM_FACTOR as i32,
                size.w * ZOOM_FACTOR,
                size.h * ZOOM_FACTOR,
            ))?;
            canvas.fill_rect(Rect::new(
                (pos.x+SCREEN_WIDTH as i32) * ZOOM_FACTOR as i32,
                pos.y * ZOOM_FACTOR as i32,
                size.w * ZOOM_FACTOR,
                size.h * ZOOM_FACTOR,
            ))?;
            canvas.fill_rect(Rect::new(
                (pos.x-SCREEN_WIDTH as i32) * ZOOM_FACTOR as i32,
                pos.y * ZOOM_FACTOR as i32,
                size.w * ZOOM_FACTOR,
                size.h * ZOOM_FACTOR,
            ))?;
        }
    }
    Ok(())
}
