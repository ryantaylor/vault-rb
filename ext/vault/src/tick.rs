use nom::bytes::complete::take;
use nom::combinator::{cut, flat_map, map, map_parser, peek};
use serde::{Serialize, Deserialize};
use nom::multi::length_count;
use nom::number::complete::{le_u32, le_u8};
use nom::Offset;
use nom::sequence::tuple;
use crate::parser::take_n;
use crate::span::{ParserResult, Span};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[magnus::wrap(class = "Vault::Tick")]
pub struct Tick {
    pub id: u32,
    pub tick_type: u32
}

impl Tick {
    pub fn parse_tick(input: Span) -> ParserResult<Tick> {
        cut(
            map(
                tuple((
                    le_u32,
                    map_parser(
                        flat_map(le_u32, take),
                        tuple((
                            le_u8,
                            le_u32,
                            le_u32
                        ))
                    )
                )),
                |(
                     tick_type,
                     (
                         _,
                         id,
                         _
                     )
                 )| {
                    Tick {
                        id,
                        tick_type
                    }
                }
            )
        )(input)
    }
}
