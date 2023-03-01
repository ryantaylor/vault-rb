use serde::{Serialize, Deserialize};
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::combinator::{cut, eof, flat_map, map, map_parser, peek};
use nom::IResult;
use nom::multi::{length_count, many0};
use nom::number::complete::le_u32;
use nom::sequence::{terminated, tuple};
use nom_tracable::tracable_parser;
use crate::chunks::Chunk::{DATA, DATADATA, DATASDSC, FOLD};
use crate::parser::{parse_utf16_variable, parse_utf8_fixed, parse_utf8_variable};
use crate::player::Player;
use crate::span::{ParserResult, Span};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[magnus::wrap(class = "Vault::Chunk")]
pub enum Chunk {
    FOLD(FOLDChunk),
    DATA(TrashDATAChunk),
    DATADATA(DATADATAChunk),
    DATASDSC(DATASDSCChunk)
}

impl Chunk {
    pub fn parse_chunk(input: Span) -> ParserResult<Chunk> {
        let (input, header) = ChunkHeader::parse_chunk_header(input)?;
        // let (input, identifier) = Self::peek_identifier(input)?;

        // println!("parsing {}", identifier);

        return match &header.chunk_kind as &str {
            "DATA" => match &header.chunk_type as &str {
                "DATA" => DATADATAChunk::parse_chunk(input, header),
                "SDSC" => DATASDSCChunk::parse_chunk(input, header),
                _ => TrashDATAChunk::parse_chunk(input, header)
            },
            "FOLD" => FOLDChunk::parse_folder_chunk(input, header),
            _ => panic!()
        };

        // let parser = match &header.chunk_kind as &str {
        //     "DATA" => match &header.chunk_type as &str {
        //         "DATA" => parse_datadata_chunk,
        //         "SDSC" => parse_datasdsc_chunk,
        //         "PLAS" => parse_dataplas_chunk,
        //         _ => panic!()
        //     },
        //     "FOLD" => parse_folder_chunk,
        //     _ => panic!()
        // };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[magnus::wrap(class = "Vault::ChunkHeader")]
pub struct ChunkHeader {
    pub chunk_kind: String,
    pub chunk_type: String,
    pub version: u32,
    pub length: u32,
    pub name_length: u32
}

impl ChunkHeader {
    #[tracable_parser]
    pub fn parse_chunk_header(input: Span) -> ParserResult<ChunkHeader> {
        map(
            tuple((
                Self::parse_chunk_kind,
                cut(
                    tuple((
                        Self::parse_chunk_type,
                        Self::parse_version,
                        Self::parse_length,
                        Self::parse_name_length
                    ))
                )
            )),
            |(
                 chunk_kind,
                 (
                     chunk_type,
                     version,
                     length,
                     name_length
                 )
             )| {
                ChunkHeader {
                    chunk_kind,
                    chunk_type,
                    version,
                    length,
                    name_length
                }
            }
        )(input)
    }

    #[tracable_parser]
    fn parse_chunk_kind(input: Span) -> ParserResult<String> {
        map(
            alt((
                tag("DATA"), tag("FOLD")
            )),
            |s: Span| String::from_utf8_lossy(s.fragment()).into_owned()
        )(input)
    }

    #[tracable_parser]
    fn parse_chunk_type(input: Span) -> ParserResult<String> {
        parse_utf8_fixed(4usize)(input)
    }

    #[tracable_parser]
    fn parse_version(input: Span) -> ParserResult<u32> {
        le_u32(input)
    }

    #[tracable_parser]
    fn parse_length(input: Span) -> ParserResult<u32> {
        le_u32(input)
    }

    #[tracable_parser]
    fn parse_name_length(input: Span) -> ParserResult<u32> {
        le_u32(input)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[magnus::wrap(class = "Vault::FOLDChunk")]
pub struct FOLDChunk {
    pub header: ChunkHeader,
    pub chunks: Vec<Chunk>
}

impl FOLDChunk {
    pub fn parse_folder_chunk(input: Span, header: ChunkHeader) -> ParserResult<Chunk> {
        cut(
            map_parser(
                take(header.length),
                terminated(
                    map(
                        map_parser(
                            take(header.length),
                            many0(Chunk::parse_chunk)
                        ),
                        move |chunks| {
                            FOLD(FOLDChunk {
                                header: header.clone(),
                                chunks
                            })
                        }
                    ),
                    eof
                )
            )
        )(input)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[magnus::wrap(class = "Vault::TrashDATAChunk")]
pub struct TrashDATAChunk {
    pub header: ChunkHeader,
    pub data: Vec<u8>
}

impl TrashDATAChunk {
    pub fn parse_chunk(input: Span, header: ChunkHeader) -> ParserResult<Chunk> {
        map(
            take(header.length),
            |data: Span| {
                DATA(TrashDATAChunk {
                    header: header.clone(),
                    data: data.to_vec()
                })
            }
        )(input)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[magnus::wrap(class = "Vault::DATADATAChunk")]
pub struct DATADATAChunk {
    pub header: ChunkHeader,
    pub opponent_type: u32,
    pub players: Vec<Player>,
    pub section_resources: String,
    pub option_resources: String,
    pub section_tickets: String,
    pub option_tickets: String,
    pub unknown_string: String
}

impl DATADATAChunk {
    #[tracable_parser]
    pub fn parse_chunk(input: Span, header: ChunkHeader) -> ParserResult<Chunk> {
        if header.version == 1 {
            return TrashDATAChunk::parse_chunk(input, header);
        }

        cut(
            map_parser(
                take(header.length),
                map(
                    tuple((
                        Self::parse_opponent_type,
                        take(6u32),
                        Self::parse_players,
                        flat_map(le_u32, take),
                        take(12u32),
                        flat_map(le_u32, take),
                        Self::parse_resource_string,
                        take(4u32),
                        Self::parse_resource_string,
                        take(4u32),
                        Self::parse_resource_string,
                        take(4u32),
                        Self::parse_resource_string,
                        take(16u32),
                        Self::parse_resource_string
                    )),
                    |(
                         opponent_type,
                         _,
                         players,
                         _,
                         _,
                         _,
                         section_resources,
                         _,
                         option_resources,
                         _,
                         section_tickets,
                         _,
                         option_tickets,
                         _,
                         unknown_string
                     )| {
                        DATADATA(DATADATAChunk {
                            header: header.clone(),
                            opponent_type,
                            players,
                            section_resources,
                            option_resources,
                            section_tickets,
                            option_tickets,
                            unknown_string
                        })
                    }
                )
            )
        )(input)
    }

    #[tracable_parser]
    fn parse_opponent_type(input: Span) -> ParserResult<u32> { le_u32(input) }

    #[tracable_parser]
    fn parse_players(input: Span) -> ParserResult<Vec<Player>> {
        length_count(le_u32, Player::parse_player)(input)
    }

    fn parse_resource_string(input: Span) -> ParserResult<String> {
        let (input, (_, section_resources)) = parse_utf8_variable(le_u32)(input)?;
        Ok((input, section_resources))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[magnus::wrap(class = "Vault::DATASDSCChunk")]
pub struct DATASDSCChunk {
    pub header: ChunkHeader,
    pub map_file: String,
    pub map_name: String,
    pub map_description: String
}

impl DATASDSCChunk {
    #[tracable_parser]
    pub fn parse_chunk(input: Span, header: ChunkHeader) -> ParserResult<Chunk> {
        cut(
            map_parser(
                take(header.length),
                map(
                    tuple((
                        take(121u32),
                        Self::parse_map_file,
                        Self::parse_map_identifier,
                        take(4u32),
                        Self::parse_map_identifier
                    )),
                    |(
                         _,
                         map_file,
                         map_name,
                         _,
                         map_description
                     )| {
                        DATASDSC(DATASDSCChunk {
                            header: header.clone(),
                            map_name,
                            map_file,
                            map_description
                        })
                    }
                )
            )
        )(input)
    }

    fn parse_map_file(input: Span) -> ParserResult<String> {
        let (input, (_, section_resources)) = parse_utf8_variable(le_u32)(input)?;
        Ok((input, section_resources))
    }

    fn parse_map_identifier(input: Span) -> ParserResult<String> {
        let (input, (_, section_resources)) = parse_utf16_variable(le_u32)(input)?;
        Ok((input, section_resources))
    }
}
