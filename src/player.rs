use bevy::prelude::*;

pub fn get_start_positions(num: usize) -> Result<Vec<Vec2>, &'static str> {
    match num {
        2 => Ok(vec![
            Vec2::new(1.0, 6.0),
            Vec2::new(13.0, 6.0),
        ]),
        3 => Ok(vec![
            Vec2::new(7.0, 9.0),
            Vec2::new(1.0, 2.0),
            Vec2::new(13.0, 2.0),
        ]),
        4 => Ok(vec![
            Vec2::new(1.0, 9.0),
            Vec2::new(13.0, 9.0),
            Vec2::new(1.0, 2.0),
            Vec2::new(13.0, 2.0),
        ]),
        5 => Ok(vec![
            Vec2::new(7.0, 15.0),
            Vec2::new(0.0, 7.0),
            Vec2::new(14.0, 7.0),
            Vec2::new(3.0, 1.0),
            Vec2::new(11.0, 1.0),
        ]),
        6 => Ok(vec![
            Vec2::new(7.0, 10.0),
            Vec2::new(0.0, 9.0),
            Vec2::new(14.0, 9.0),
            Vec2::new(0.0, 2.0),
            Vec2::new(7.0, 1.0),
            Vec2::new(14.0, 2.0),
        ]),
        7 => Ok(vec![
            Vec2::new(7.0, 10.0),
            Vec2::new(1.0, 9.0),
            Vec2::new(13.0, 9.0),
            Vec2::new(0.0, 4.0),
            Vec2::new(14.0, 5.0),
            Vec2::new(4.0, 1.0),
            Vec2::new(10.0, 1.0),
        ]),
        8 => Ok(vec![
            Vec2::new(0.0, 10.0),
            Vec2::new(7.0, 10.0),
            Vec2::new(14.0, 10.0),
            Vec2::new(0.0, 6.0),
            Vec2::new(14.0, 6.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(7.0, 1.0),
            Vec2::new(14.0, 1.0),
        ]),
        _ => Err("invalid number of players"),
    }
}