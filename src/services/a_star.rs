use std::collections::{HashMap, VecDeque};
use ggez::glam::Vec2;

pub fn calculate_range(pawn_position: (u16, u16), range: u16, room: &Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    let breath_result = breath_first(pawn_position, room, range);

    let mut result = vec![];
    for j in 0..room.len() {
        let mut row = vec![];
        for i in 0..room.get(0).unwrap().len() {
            if breath_result.contains(&(i as u16, j as u16)) && room.get(j).unwrap().get(i).unwrap() != &20u8 {
                row.push(true);
            } else {
                row.push(false);
            }
        }
        result.push(row);
    }

    result
}

fn get_neighbours(room_size: (u16, u16), position: (u16, u16)) -> Vec<(u16, u16)> {
    let up = calculate_min_value(position.0);
    let left = calculate_min_value(position.1);

    let down = calculate_max_value(room_size.0, position.0);
    let right = calculate_max_value(room_size.1, position.1);

    vec![
        (up, position.1),
        (position.0, left),
        (position.0, right),
        (down, position.1),
    ]
}

fn calculate_max_value(room_size_max: u16, position: u16) -> u16 {
    if position as i32 + 1 > room_size_max as i32 {
        room_size_max - 1
    } else {
        position + 1
    }
}

fn calculate_min_value(position: u16) -> u16 {
    if position as i32 - 1 < 0 {
        0
    } else {
        position - 1
    }
}

fn calculate_dist(current: Vec2, start: Vec2) -> u16 {
    start.distance_squared(current) as u16
}

fn breath_first(start: (u16, u16), room: &Vec<Vec<u8>>, range: u16) -> Vec<(u16, u16)> {
    let mut frontier = VecDeque::new();
    frontier.push_front(start);
    let mut came_from = HashMap::new();
    came_from.insert(start, None);
    while !frontier.is_empty() {
        let current = frontier.pop_front().unwrap();
        let (size_x, size_y) = (room.len() as u16, room.get(0).unwrap().len() as u16);


        if calculate_dist(Vec2::new(current.0 as f32, current.1 as f32), Vec2::new(start.0 as f32, start.1 as f32)) > range * range {
            break;
        }

        for next in get_neighbours((size_x, size_y), current.clone()) {
            if !came_from.contains_key(&next) {
                frontier.push_back(next.clone());
                came_from.insert(next.clone(), Some(current.clone()));
            }
        }
    }

    came_from.iter()
        .filter(|(k, &v)| v != None)
        .map(|(k,v)| k.clone())
        .collect::<Vec<(u16,u16)>>()
}