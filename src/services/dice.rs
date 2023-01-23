pub struct Dice;

pub enum RollDiceResult {
    Normal(u8),
    Critical,
    Fumble
}

impl Dice {
    pub fn roll_1d20() -> u32 {
        (rand::random::<f32>() * 20f32).ceil() as u32
    }

    pub fn roll_1d10() -> u32 {
        (rand::random::<f32>() * 10f32).ceil() as u32
    }

    pub fn roll_1d100() -> u32 {
        (rand::random::<f32>() * 100f32).ceil() as u32
    }

    pub fn roll_1d6() -> u32 {
        (rand::random::<f32>() * 6f32).ceil() as u32
    }

    pub fn roll_1d4() -> u32 {
        (rand::random::<f32>() * 4f32).ceil() as u32
    }

}