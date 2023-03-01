use serde::{Serialize, Deserialize};
use std::slice::Chunks;
use nom::{
    combinator::map};
use nom::sequence::tuple;
use nom_tracable::tracable_parser;
use crate::chunky::Chunky;
use crate::chunks::Chunk;

use crate::header::Header;
use crate::span::{ParserResult, Span};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[magnus::wrap(class = "Vault::Replay")]
pub struct Replay {
    pub header: Header,
    pub chunkies: Vec<Chunky>,
    pub chunks: Vec<Chunk>
}

impl Replay {
    #[tracable_parser]
    pub fn parse_replay(input: Span) -> ParserResult<Replay> {
        map(
            tuple((
                Header::parse_header,
                Chunky::parse_chunky,
                Chunk::parse_chunk,
                Chunky::parse_chunky,
                Chunk::parse_chunk,
                Chunk::parse_chunk
            )),
            |(
                header,
                first_chunky,
                foldpost_chunk,
                second_chunky,
                foldinfo_chunk,
                datasdsc_chunk
             )| {
                Replay {
                    header,
                    chunkies: vec![first_chunky, second_chunky],
                    chunks: vec![foldpost_chunk, foldinfo_chunk, datasdsc_chunk]
                }
            }
        )(input)
    }

    pub fn header(&self) -> Header {
        self.header.clone()
    }

    pub fn chunkies(&self) -> Vec<Chunky> {
        self.chunkies.clone()
    }

    pub fn chunks(&self) -> Vec<Chunk> {
        self.chunks.clone()
    }
}
