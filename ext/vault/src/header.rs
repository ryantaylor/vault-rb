use serde::{Serialize, Deserialize};
use nom::{
    combinator::map,
    sequence::{preceded, tuple}};
use nom::number::complete::le_u16;
use nom_tracable::tracable_parser;

use crate::parser::{parse_utf8_fixed, verify_zero_u16, parse_utf16_terminated, take_zeroes};
use crate::span::{ParserResult, Span};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[magnus::wrap(class = "Vault::Header")]
pub struct Header {
    pub version: u16,
    pub game_type: String,
    pub timestamp: String
}

impl Header {
    #[tracable_parser]
    pub fn parse_header(input: Span) -> ParserResult<Header> {
        map(
            tuple((
                Self::parse_version,
                Self::parse_game_type,
                Self::parse_timestamp,
                Self::clear_zeroes
            )),
            |(
                version,
                game_type,
                timestamp,
                _
             )| {
                Header {
                    version,
                    game_type,
                    timestamp
                }
            }
        )(input)
    }

    #[tracable_parser]
    fn parse_version(input: Span) -> ParserResult<u16> {
        preceded(verify_zero_u16, le_u16)(input)
    }

    #[tracable_parser]
    fn parse_game_type(input: Span) -> ParserResult<String> {
        parse_utf8_fixed(8usize)(input)
    }

    #[tracable_parser]
    fn parse_timestamp(input: Span) -> ParserResult<String> {
        parse_utf16_terminated(input)
    }

    #[tracable_parser]
    fn clear_zeroes(input: Span) -> ParserResult<Span> {
        take_zeroes(input)
    }
}
