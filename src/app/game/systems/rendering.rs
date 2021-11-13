use sdl2::{pixels::Color, rect::{Point, Rect}, render::Canvas, video::Window};

use crate::{
    app::game::{entity::Entity, Game},
    draw::ZOOM_FACTOR,
};

pub(crate) fn system(game: &Game, canvas: &mut Canvas<Window>) -> Result<(), String> {
    for entity in &game.entities {
        if let Entity {
            pos: Some(pos),
            shape: Some(cuboid),
            draw: Some(draw),
            ..
        } = &entity
        {
            canvas.set_draw_color(draw.color);
            canvas.fill_rect(Rect::new(
                ((pos.x - cuboid.half_extents.x) * ZOOM_FACTOR as f32).round() as i32,
                ((pos.y - cuboid.half_extents.y) * ZOOM_FACTOR as f32).round() as i32,
                (cuboid.half_extents.x * 2.0 * ZOOM_FACTOR as f32).round() as u32,
                (cuboid.half_extents.y * 2.0 * ZOOM_FACTOR as f32).round() as u32,
            ))?;
            // canvas.fill_rect(Rect::new(
            //     (((pos.x - cuboid.half_extents.x) + SCREEN_WIDTH as f32) * ZOOM_FACTOR as f32)
            //         as i32,
            //     ((pos.y /*cuboid.half_extents.y) * ZOOM_FACTOR as f32) as i32,
            //     (cuboid.half_extents.x * 2.0 * ZOOM_FACTOR as f32) as u32,
            //     (cuboid.half_extents.y * 2.0 * ZOOM_FACTOR as f32) as u32,
            // ))?;
            // canvas.fill_rect(Rect::new(
            //     (((pos.x - cuboid.half_extents.x) - SCREEN_WIDTH as f32) * ZOOM_FACTOR as f32)
            //         as i32,
            //     ((pos.y - cuboid.half_extents.y) * ZOOM_FACTOR as f32) as i32,
            //     (cuboid.half_extents.x * 2.0 * ZOOM_FACTOR as f32) as u32,
            //     (cuboid.half_extents.y * 2.0 * ZOOM_FACTOR as f32) as u32,
            // ))?;
        }
    }
    Ok(())
}
