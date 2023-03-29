extern crate byteorder;
extern crate nom;
extern crate nom_locate;
extern crate nom_tracable;
extern crate serde_magnus;
extern crate serde;

use nom_locate::LocatedSpan;
use nom_tracable::{cumulative_histogram, histogram, TracableInfo};
use magnus::{define_module, function, prelude::*, Error, define_class, class, method, Value};
use serde_magnus::serialize;
use crate::replay::Replay;
use crate::span::Span;

pub mod chunks;
pub mod chunky;
pub mod header;
pub mod parser;
pub mod replay;
pub mod span;
pub mod player;
pub mod item;
pub mod ticks;

fn hello(subject: String) -> String {
    format!("Hello from Rust, {}!", subject)
}

fn parse_replay(data: Vec<u8>) -> Result<Value, Error> {
    let info = TracableInfo::new().parser_width(64).fold("term");
    let input: Span = LocatedSpan::new_extra(data.as_slice(), info);
    let replay = match Replay::parse_replay(input) {
        Ok((_, replay)) => replay,
        Err(_) => return Err(Error::new(magnus::exception::runtime_error(), "Parsing failed!"))
    };

    serialize(&replay)
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Vault")?;
    let replay = module.define_class("Replay", class::object())?;
    replay.define_singleton_method("parse_replay", function!(parse_replay, 1))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use nom_locate::LocatedSpan;
    use nom_tracable::{cumulative_histogram, histogram, TracableInfo};
    use crate::span::Span;
    use super::*;

    #[test]
    fn it_works() {
        let info = TracableInfo::new().parser_width(64).fold("term");
        let data = include_bytes!("/Users/ryantaylor/Downloads/release.rec");
        let input: Span = LocatedSpan::new_extra(data, info);
        let (_, replay) = replay::Replay::parse_replay(input).unwrap();
        println!("{:#?}", replay);

        histogram();
        cumulative_histogram();
    }

    #[test]
    fn test_len() {
        let info = TracableInfo::new().parser_width(64).fold("term");
        let data = include_bytes!("/Users/ryantaylor/Downloads/release.rec");
        let input: Span = LocatedSpan::new_extra(data, info);
        let (_, replay) = replay::Replay::parse_replay(input).unwrap();

        assert_eq!(replay.len(), 21001)
    }

    #[test]
    fn test_version() {
        let info = TracableInfo::new().parser_width(64).fold("term");
        let data = include_bytes!("/Users/ryantaylor/Downloads/release.rec");
        let input: Span = LocatedSpan::new_extra(data, info);
        let (_, replay) = replay::Replay::parse_replay(input).unwrap();

        assert_eq!(replay.version(), 8369)
    }

    #[test]
    fn test_timestamp() {
        let info = TracableInfo::new().parser_width(64).fold("term");
        let data = include_bytes!("/Users/ryantaylor/Downloads/release.rec");
        let input: Span = LocatedSpan::new_extra(data, info);
        let (_, replay) = replay::Replay::parse_replay(input).unwrap();

        assert_eq!(replay.timestamp(), "2023-02-23 21:18")
    }

    #[test]
    fn test_matchhistory_id() {
        let info = TracableInfo::new().parser_width(64).fold("term");
        let data = include_bytes!("/Users/ryantaylor/Downloads/release.rec");
        let input: Span = LocatedSpan::new_extra(data, info);
        let (_, replay) = replay::Replay::parse_replay(input).unwrap();

        assert_eq!(replay.matchhistory_id(), 150656)
    }
}
