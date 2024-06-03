use magnus::{value::ReprValue, RHash, RString, Value};
use vault::{Command, Map, Message, Player, Replay};

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

impl HashExt for Command {
    fn to_h(&self) -> Value {
        match self {
            Command::BuildGlobalUpgrade(data) => {
                let hash: RHash = serde_magnus::serialize(data).unwrap();
                hash.aset("type", RString::new("BuildGlobalUpgrade"))
                    .unwrap();
                hash.aset("action_type", RString::new("CMD_Upgrade"))
                    .unwrap();
                hash.as_value()
            }
            Command::BuildSquad(data) => {
                let hash: RHash = serde_magnus::serialize(data).unwrap();
                hash.aset("type", RString::new("BuildSquad")).unwrap();
                hash.aset("action_type", RString::new("CMD_BuildSquad"))
                    .unwrap();
                hash.as_value()
            }
            Command::CancelConstruction(data) => {
                let hash: RHash = serde_magnus::serialize(data).unwrap();
                hash.aset("type", RString::new("CancelConstruction"))
                    .unwrap();
                hash.aset("action_type", RString::new("CMD_CancelConstruction"))
                    .unwrap();
                hash.as_value()
            }
            Command::CancelProduction(data) => {
                let hash: RHash = serde_magnus::serialize(data).unwrap();
                hash.aset("type", RString::new("CancelProduction")).unwrap();
                hash.aset("action_type", RString::new("CMD_CancelProduction"))
                    .unwrap();
                hash.as_value()
            }
            Command::SelectBattlegroup(data) => {
                let hash: RHash = serde_magnus::serialize(data).unwrap();
                hash.aset("type", RString::new("SelectBattlegroup"))
                    .unwrap();
                hash.aset("action_type", RString::new("PCMD_InstantUpgrade"))
                    .unwrap();
                hash.as_value()
            }
            Command::SelectBattlegroupAbility(data) => {
                let hash: RHash = serde_magnus::serialize(data).unwrap();
                hash.aset("type", RString::new("SelectBattlegroupAbility"))
                    .unwrap();
                hash.aset("action_type", RString::new("PCMD_TentativeUpgrade"))
                    .unwrap();
                hash.as_value()
            }
            Command::Unknown(data) => {
                let hash: RHash = serde_magnus::serialize(data).unwrap();
                hash.aset("type", RString::new("Unknown")).unwrap();
                hash.as_value()
            }
            Command::UseAbility(data) => {
                let hash: RHash = serde_magnus::serialize(data).unwrap();
                hash.aset("type", RString::new("UseAbility")).unwrap();
                hash.aset("action_type", RString::new("CMD_Ability"))
                    .unwrap();
                hash.as_value()
            }
            Command::UseBattlegroupAbility(data) => {
                let hash: RHash = serde_magnus::serialize(data).unwrap();
                hash.aset("type", RString::new("UseBattlegroupAbility"))
                    .unwrap();
                hash.aset("action_type", RString::new("PCMD_Ability"))
                    .unwrap();
                hash.as_value()
            }
        }
    }
}
