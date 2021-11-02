use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::{
    app::game::{entity::Entity, Game},
    draw::ZOOM_FACTOR,
};

pub(crate) fn system(game: &Game, canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(Color::RGBA(200, 200, 200, 255));
    for entity in &game.entities {
        if let Entity {
            pos: Some(pos),
            size: Some(size),
            collision: Some(_),
            ..
        } = &entity
        {
            canvas.fill_rect(Rect::new(
                pos.x * ZOOM_FACTOR as i32,
                pos.y * ZOOM_FACTOR as i32,
                size.w * ZOOM_FACTOR,
                size.h * ZOOM_FACTOR,
            ))?;
        }
    }
    Ok(())
}
