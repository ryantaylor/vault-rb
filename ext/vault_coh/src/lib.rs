mod hash;

use crate::hash::HashExt;
use magnus::{function, method, prelude::*, Error, Ruby};
use vault::{Command, Map, Message, Player, Replay};

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("VaultCoh")?;

    let replay = module.define_class("Replay", ruby.class_object())?;
    replay.define_singleton_method("from_bytes", function!(from_bytes, 1))?;
    replay.define_method("version", method!(Replay::version, 0))?;
    replay.define_method("timestamp", method!(Replay::timestamp, 0))?;
    replay.define_method("game_type", method!(game_type_string, 0))?;
    replay.define_method("matchhistory_id", method!(Replay::matchhistory_id, 0))?;
    replay.define_method("mod_uuid", method!(mod_uuid_string, 0))?;
    replay.define_method("map", method!(Replay::map, 0))?;
    replay.define_method("map_filename", method!(Replay::map_filename, 0))?;
    replay.define_method(
        "map_localized_name_id",
        method!(Replay::map_localized_name_id, 0),
    )?;
    replay.define_method(
        "map_localized_description_id",
        method!(Replay::map_localized_description_id, 0),
    )?;
    replay.define_method("players", method!(Replay::players, 0))?;
    replay.define_method("length", method!(Replay::length, 0))?;
    replay.define_method("to_h", method!(Replay::to_h, 0))?;

    let map = module.define_class("Map", ruby.class_object())?;
    map.define_method("filename", method!(Map::filename, 0))?;
    map.define_method("localized_name_id", method!(Map::localized_name_id, 0))?;
    map.define_method(
        "localized_description_id",
        method!(Map::localized_description_id, 0),
    )?;
    map.define_method("to_h", method!(Map::to_h, 0))?;

    let player = module.define_class("Player", ruby.class_object())?;
    player.define_method("name", method!(Player::name, 0))?;
    player.define_method("human?", method!(Player::human, 0))?;
    player.define_method("faction", method!(faction_string, 0))?;
    player.define_method("team", method!(team_value, 0))?;
    player.define_method("battlegroup", method!(Player::battlegroup, 0))?;
    player.define_method("steam_id", method!(Player::steam_id, 0))?;
    player.define_method("profile_id", method!(Player::profile_id, 0))?;
    player.define_method("messages", method!(Player::messages, 0))?;
    player.define_method("commands", method!(Player::commands, 0))?;
    player.define_method("build_commands", method!(Player::build_commands, 0))?;
    player.define_method(
        "battlegroup_commands",
        method!(Player::battlegroup_commands, 0),
    )?;
    player.define_method("to_h", method!(Player::to_h, 0))?;

    let message = module.define_class("Message", ruby.class_object())?;
    message.define_method("tick", method!(Message::tick, 0))?;
    message.define_method("message", method!(Message::message, 0))?;
    message.define_method("to_h", method!(Message::to_h, 0))?;

    let command = module.define_class("Command", ruby.class_object())?;
    command.define_method("to_h", method!(Command::to_h, 0))?;

    Ok(())
}

fn from_bytes(ruby: &Ruby, input: Vec<u8>) -> Result<Replay, Error> {
    Replay::from_bytes(&input)
        .map_err(|err| Error::new(ruby.exception_runtime_error(), err.to_string()))
}

fn mod_uuid_string(rb_self: &Replay) -> String {
    rb_self.mod_uuid().to_string()
}

fn game_type_string(rb_self: &Replay) -> String {
    rb_self.game_type().to_string()
}

fn faction_string(rb_self: &Player) -> String {
    rb_self.faction().to_string()
}

fn team_value(rb_self: &Player) -> usize {
    rb_self.team().value()
}
