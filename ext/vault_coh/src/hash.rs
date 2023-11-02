use magnus::Value;
use vault::commands::{BuildSquad, SelectBattlegroup, Unknown};
use vault::{Map, Message, Player, Replay};

pub trait HashExt {
    fn to_h(&self) -> Value;
}

impl HashExt for Replay {
    fn to_h(&self) -> Value {
        serde_magnus::serialize(self).unwrap()
    }
}

impl HashExt for Map {
    fn to_h(&self) -> Value {
        serde_magnus::serialize(self).unwrap()
    }
}

impl HashExt for Player {
    fn to_h(&self) -> Value {
        serde_magnus::serialize(self).unwrap()
    }
}

impl HashExt for Message {
    fn to_h(&self) -> Value {
        serde_magnus::serialize(self).unwrap()
    }
}

impl HashExt for BuildSquad {
    fn to_h(&self) -> Value {
        serde_magnus::serialize(self).unwrap()
    }
}

impl HashExt for SelectBattlegroup {
    fn to_h(&self) -> Value {
        serde_magnus::serialize(self).unwrap()
    }
}

impl HashExt for Unknown {
    fn to_h(&self) -> Value {
        serde_magnus::serialize(self).unwrap()
    }
}
