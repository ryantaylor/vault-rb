use magnus::{value::ReprValue, RHash, Ruby, Value};
use vault::{Command, Map, Message, Player, Replay};

pub trait HashExt {
    fn to_h(ruby: &Ruby, rb_self: &Self) -> Value;
}

impl HashExt for Replay {
    fn to_h(_ruby: &Ruby, rb_self: &Self) -> Value {
        serde_magnus::serialize(rb_self).unwrap()
    }
}

impl HashExt for Map {
    fn to_h(_ruby: &Ruby, rb_self: &Self) -> Value {
        serde_magnus::serialize(rb_self).unwrap()
    }
}

impl HashExt for Player {
    fn to_h(_ruby: &Ruby, rb_self: &Self) -> Value {
        serde_magnus::serialize(rb_self).unwrap()
    }
}

impl HashExt for Message {
    fn to_h(_ruby: &Ruby, rb_self: &Self) -> Value {
        serde_magnus::serialize(rb_self).unwrap()
    }
}

impl HashExt for Command {
    fn to_h(ruby: &Ruby, rb_self: &Self) -> Value {
        match rb_self {
            Command::AITakeover(data) => {
                let hash: RHash = serde_magnus::serialize(data).unwrap();
                hash.aset(ruby.to_symbol("type"), ruby.str_new("AITakeover"))
                    .unwrap();
                hash.aset(ruby.to_symbol("action_type"), ruby.str_new("PCMD_AIPlayer"))
                    .unwrap();
                hash.as_value()
            }
            Command::BuildGlobalUpgrade(data) => {
                let hash: RHash = serde_magnus::serialize(&data).unwrap();
                hash.aset(ruby.to_symbol("type"), ruby.str_new("BuildGlobalUpgrade"))
                    .unwrap();
                hash.aset(ruby.to_symbol("action_type"), ruby.str_new("CMD_Upgrade"))
                    .unwrap();
                hash.as_value()
            }
            Command::BuildSquad(data) => {
                let hash: RHash = serde_magnus::serialize(&data).unwrap();
                hash.aset(ruby.to_symbol("type"), ruby.str_new("BuildSquad"))
                    .unwrap();
                hash.aset(
                    ruby.to_symbol("action_type"),
                    ruby.str_new("CMD_BuildSquad"),
                )
                .unwrap();
                hash.as_value()
            }
            Command::CancelConstruction(data) => {
                let hash: RHash = serde_magnus::serialize(&data).unwrap();
                hash.aset(ruby.to_symbol("type"), ruby.str_new("CancelConstruction"))
                    .unwrap();
                hash.aset(
                    ruby.to_symbol("action_type"),
                    ruby.str_new("CMD_CancelConstruction"),
                )
                .unwrap();
                hash.as_value()
            }
            Command::CancelProduction(data) => {
                let hash: RHash = serde_magnus::serialize(&data).unwrap();
                hash.aset(ruby.to_symbol("type"), ruby.str_new("CancelProduction"))
                    .unwrap();
                hash.aset(
                    ruby.to_symbol("action_type"),
                    ruby.str_new("CMD_CancelProduction"),
                )
                .unwrap();
                hash.as_value()
            }
            Command::ConstructEntity(data) => {
                let hash: RHash = serde_magnus::serialize(&data).unwrap();
                hash.aset(ruby.to_symbol("type"), ruby.str_new("ConstructEntity"))
                    .unwrap();
                hash.aset(
                    ruby.to_symbol("action_type"),
                    ruby.str_new("PCMD_PlaceAndConstructEntities"),
                )
                .unwrap();
                hash.as_value()
            }
            Command::SelectBattlegroup(data) => {
                let hash: RHash = serde_magnus::serialize(&data).unwrap();
                hash.aset(ruby.to_symbol("type"), ruby.str_new("SelectBattlegroup"))
                    .unwrap();
                hash.aset(
                    ruby.to_symbol("action_type"),
                    ruby.str_new("PCMD_InstantUpgrade"),
                )
                .unwrap();
                hash.as_value()
            }
            Command::SelectBattlegroupAbility(data) => {
                let hash: RHash = serde_magnus::serialize(&data).unwrap();
                hash.aset(
                    ruby.to_symbol("type"),
                    ruby.str_new("SelectBattlegroupAbility"),
                )
                .unwrap();
                hash.aset(
                    ruby.to_symbol("action_type"),
                    ruby.str_new("PCMD_TentativeUpgrade"),
                )
                .unwrap();
                hash.as_value()
            }
            Command::Unknown(data) => {
                let hash: RHash = serde_magnus::serialize(&data).unwrap();
                hash.aset(ruby.to_symbol("type"), ruby.str_new("Unknown"))
                    .unwrap();
                hash.as_value()
            }
            Command::UseAbility(data) => {
                let hash: RHash = serde_magnus::serialize(&data).unwrap();
                hash.aset(ruby.to_symbol("type"), ruby.str_new("UseAbility"))
                    .unwrap();
                hash.aset(ruby.to_symbol("action_type"), ruby.str_new("CMD_Ability"))
                    .unwrap();
                hash.as_value()
            }
            Command::UseBattlegroupAbility(data) => {
                let hash: RHash = serde_magnus::serialize(&data).unwrap();
                hash.aset(
                    ruby.to_symbol("type"),
                    ruby.str_new("UseBattlegroupAbility"),
                )
                .unwrap();
                hash.aset(ruby.to_symbol("action_type"), ruby.str_new("PCMD_Ability"))
                    .unwrap();
                hash.as_value()
            }
        }
    }
}
