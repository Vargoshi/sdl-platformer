use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::{
    app::game::{entity::Entity, Game},
    draw::ZOOM_FACTOR,
};

pub(crate) fn system(game: &Game, canvas: &mut Canvas<Window>) -> Result<(), String> {
    for entity in &game.entities {
        if let Entity {
            pos: Some(pos),
            shape: Some(cuboid),
            vel: vel,
            physics: physics,
            draw: Some(draw),
            ..
        } = &entity
        {
            canvas.set_draw_color(Color::RGBA(255 - draw.color.r, 255 - draw.color.g, 255 - draw.color.b, 255));

            if let Some(vel) = vel {
                let p2 = pos + vel * 4.0;
                canvas.draw_line(
                    Point::new(
                        (pos.x * ZOOM_FACTOR as f32) as i32,
                        (pos.y * ZOOM_FACTOR as f32) as i32,
                    ),
                    Point::new(
                        (p2.x * ZOOM_FACTOR as f32) as i32,
                        (p2.y * ZOOM_FACTOR as f32) as i32,
                    ),
                )?;

            }

            if let Some(physics) = physics {
                if physics.on_floor {
                    canvas.draw_line(
                        Point::new(
                            ((pos.x - cuboid.half_extents.x + 2.0) * ZOOM_FACTOR as f32) as i32,
                            ((pos.y + cuboid.half_extents.y - 2.0) * ZOOM_FACTOR as f32) as i32,
                        ),
                        Point::new(
                            ((pos.x + cuboid.half_extents.x - 2.0) * ZOOM_FACTOR as f32) as i32,
                            ((pos.y + cuboid.half_extents.y - 2.0) * ZOOM_FACTOR as f32) as i32,
                        ),
                    )?;
                }
                if physics.on_left_wall {
                    canvas.draw_line(
                        Point::new(
                            ((pos.x - cuboid.half_extents.x + 2.0) * ZOOM_FACTOR as f32) as i32,
                            ((pos.y - cuboid.half_extents.y + 2.0) * ZOOM_FACTOR as f32) as i32,
                        ),
                        Point::new(
                            ((pos.x - cuboid.half_extents.x + 2.0) * ZOOM_FACTOR as f32) as i32,
                            ((pos.y + cuboid.half_extents.y - 2.0) * ZOOM_FACTOR as f32) as i32,
                        ),
                    )?;
                }
                if physics.on_right_wall {
                    canvas.draw_line(
                        Point::new(
                            ((pos.x + cuboid.half_extents.x - 2.0) * ZOOM_FACTOR as f32) as i32,
                            ((pos.y - cuboid.half_extents.y + 2.0) * ZOOM_FACTOR as f32) as i32,
                        ),
                        Point::new(
                            ((pos.x + cuboid.half_extents.x - 2.0) * ZOOM_FACTOR as f32) as i32,
                            ((pos.y + cuboid.half_extents.y - 2.0) * ZOOM_FACTOR as f32) as i32,
                        ),
                    )?;
                }
            }
        }
    }
    Ok(())
}
