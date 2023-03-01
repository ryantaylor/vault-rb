use serde::{Serialize, Deserialize};
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::tuple;
use crate::span::{ParserResult, Span};
use nom_tracable::tracable_parser;
use crate::parser::verify_le_u32;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[magnus::wrap(class = "Vault::Chunky")]
pub struct Chunky {
    pub name: String,
    pub signature: u32,
    pub major_version: u32,
    pub minor_version: u32 // maybe?
}

impl Chunky {
    #[tracable_parser]
    pub fn parse_chunky(input: Span) -> ParserResult<Chunky> {
        map(
            tuple((
                Self::parse_name,
                Self::parse_signature,
                Self::parse_major_version,
                Self::parse_minor_version
            )),
            |(
                 name,
                 signature,
                 major_version,
                 minor_version
             )| {
                Chunky {
                    name,
                    signature,
                    major_version,
                    minor_version
                }
            }
        )(input)
    }

    #[tracable_parser]
    pub fn parse_chunkies(input: Span) -> ParserResult<Vec<Chunky>> {
        many1(Self::parse_chunky)(input)
    }

    #[tracable_parser]
    fn parse_name(input: Span) -> ParserResult<String> {
        map(tag("Relic Chunky"), |s: Span| String::from_utf8_lossy(s.fragment()).into_owned())(input)
    }

    #[tracable_parser]
    fn parse_signature(input: Span) -> ParserResult<u32> {
        verify_le_u32(0x1A0A0D)(input)
    }

    #[tracable_parser]
    fn parse_major_version(input: Span) -> ParserResult<u32> {
        verify_le_u32(0x4)(input)
    }

    #[tracable_parser]
    fn parse_minor_version(input: Span) -> ParserResult<u32> {
        verify_le_u32(0x1)(input)
    }
}
