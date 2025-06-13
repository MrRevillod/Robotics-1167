pub const GOAL: &str = "M";
pub const DANGER_KEYS: [&str; 4] = ["P1", "P2", "P3", "P4"];
pub const WALL_KEYS: [&str; 10] = ["O1", "O2", "O3", "O4", "O5", "O6", "O7", "O8", "O9", "O10"];

#[derive(Debug, PartialEq)]
pub struct Status {
    pub key: &'static str,
    pub r#type: StatusType,
    pub reward: f32,
}

impl From<&'static str> for Status {
    fn from(key: &'static str) -> Self {
        let r#type = StatusType::from(key);

        let reward = match r#type {
            StatusType::Goal => 10.0,
            StatusType::Danger => -0.5,
            _ => -0.1,
        };

        Self {
            key,
            r#type,
            reward,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StatusType {
    Normal,
    Danger,
    Wall,
    Goal,
}

impl From<&'static str> for StatusType {
    fn from(key: &'static str) -> Self {
        if key == GOAL {
            StatusType::Goal
        } else if DANGER_KEYS.contains(&key) {
            StatusType::Danger
        } else if WALL_KEYS.contains(&key) {
            StatusType::Wall
        } else {
            StatusType::Normal
        }
    }
}
