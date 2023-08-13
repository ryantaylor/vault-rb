use magnus::{class, define_module, exception, function, method, prelude::*, Error};
use vault::{BuildSquad, Command, Faction, Map, Message, Player, Replay, Team, Unknown};

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("VaultCoh")?;

    let replay = module.define_class("Replay", class::object())?;
    replay.define_singleton_method("from_bytes", function!(from_bytes, 1))?;
    replay.define_method("version", method!(Replay::version, 0))?;
    replay.define_method("timestamp", method!(Replay::timestamp, 0))?;
    replay.define_method("matchhistory_id", method!(Replay::matchhistory_id, 0))?;
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

    let map = module.define_class("Map", class::object())?;
    map.define_method("filename", method!(Map::filename, 0))?;
    map.define_method("localized_name_id", method!(Map::localized_name_id, 0))?;
    map.define_method(
        "localized_description_id",
        method!(Map::localized_description_id, 0),
    )?;

    let player = module.define_class("Player", class::object())?;
    player.define_method("name", method!(Player::name, 0))?;
    player.define_method("faction", method!(Player::faction, 0))?;
    player.define_method("team", method!(Player::team, 0))?;
    player.define_method("steam_id", method!(Player::steam_id, 0))?;
    player.define_method("profile_id", method!(Player::profile_id, 0))?;
    player.define_method("messages", method!(Player::messages, 0))?;
    player.define_method("commands", method!(Player::commands, 0))?;
    player.define_method("build_commands", method!(Player::build_commands, 0))?;

    let message = module.define_class("Message", class::object())?;
    message.define_method("tick", method!(Message::tick, 0))?;
    message.define_method("message", method!(Message::message, 0))?;

    let faction = module.define_class("Faction", class::object())?;
    faction.define_method("value", method!(Faction::to_string, 0))?;

    let team = module.define_class("Team", class::object())?;
    team.define_method("value", method!(Team::value, 0))?;

    let command = module.define_class("Command", class::object())?;

    let commands_module = module.define_module("Commands")?;

    let build_squad_command = commands_module.define_class("BuildSquadCommand", command)?;
    build_squad_command.define_method("value", method!(Command::extract_build_squad , 0))?;

    let build_squad = commands_module.define_class("BuildSquad", class::object())?;
    build_squad.define_method("tick", method!(BuildSquad::tick, 0))?;
    build_squad.define_method("pbgid", method!(BuildSquad::pbgid, 0))?;

    let unknown_command = commands_module.define_class("UnknownCommand", command)?;
    unknown_command.define_method("value", method!(Command::extract_unknown, 0))?;

    let unknown = commands_module.define_class("Unknown", class::object())?;
    unknown.define_method("tick", method!(Unknown::tick, 0))?;
    unknown.define_method("action_type", method!(Unknown::action_type, 0))?;

    Ok(())
}

fn from_bytes(input: Vec<u8>) -> Result<Replay, Error> {
    Replay::from_bytes(&input)
        .map_err(|err| Error::new(exception::runtime_error(), err.to_string()))
}

// fn into_build_squad(rb_self: Command) -> Result<BuildSquad, Error> {
//     if let Command::BuildSquadCommand(command) = rb_self {
//         Ok(command.clone())
//     } else {
//         panic!()
//     }
// }
//
// fn into_unknown(rb_self: Command) -> Result<Unknown, Error> {
//     if let Command::UnknownCommand(command) = rb_self {
//         Ok(command.clone())
//     } else {
//         panic!()
//     }
// }
