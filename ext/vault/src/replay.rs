use serde::{Serialize, Deserialize};
use std::slice::Chunks;
use nom::{
    combinator::map};
use nom::combinator::eof;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom_tracable::tracable_parser;
use crate::chunky::Chunky;
use crate::chunks::{Chunk, DATADATAChunk, DATASDSCChunk};
use crate::chunks::Chunk::{DATADATA, DATASDSC};

use crate::header::Header;
use crate::span::{ParserResult, Span};
use crate::ticks::{Tick, Ticks};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[magnus::wrap(class = "Vault::Replay")]
pub struct Replay {
    pub header: Header,
    pub chunkies: Vec<Chunky>,
    pub chunks: Vec<Chunk>,
    pub ticks: Ticks,
    pub length: usize
}

impl Replay {
    #[tracable_parser]
    pub fn parse_replay(input: Span) -> ParserResult<Replay> {
        let (input, header) = Header::parse_header(input)?;

        let mut parser = map(
            tuple((
                Chunky::parse_chunky,
                Chunk::parse_chunk(header.version),
                Chunky::parse_chunky,
                Chunk::parse_chunk(header.version),
                Chunk::parse_chunk(header.version),
                Ticks::parse_ticks
            )),
            |(
                first_chunky,
                foldpost_chunk,
                second_chunky,
                foldinfo_chunk,
                datasdsc_chunk,
                ticks
             )| {
                Replay {
                    header: header.clone(),
                    chunkies: vec![first_chunky, second_chunky],
                    chunks: vec![foldpost_chunk, foldinfo_chunk, datasdsc_chunk],
                    length: ticks.commands().len() / 8,
                    ticks
                }
            }
        );

        parser(input)
    }

    pub fn len(&self) -> usize {
        self.ticks.command_ticks.len()
    }

    pub fn version(&self) -> u16 {
        self.header.version
    }

    pub fn timestamp(&self) -> &str {
        &self.header.timestamp
    }

    fn data_chunks(&self) -> Vec<&Chunk> {
        self.chunks.iter().flat_map(|chunk| {
            match chunk {
                Chunk::FOLD(fold) => fold.chunks.iter().collect(),
                _ => vec![chunk]
            }
        }).collect()
    }

    fn game_data(&self) -> &DATADATAChunk {
        let chunks = self.data_chunks();

        let data_chunk = chunks.iter().find(|chunk| {
            match chunk {
                DATADATA(_) => true,
                _ => false
            }
        }).unwrap();

        match data_chunk {
            DATADATA(data) => data,
            _ => panic!()
        }
    }

    fn map_data(&self) -> &DATASDSCChunk {
        let chunks = self.data_chunks();

        let map_chunk = chunks.iter().find(|chunk| {
            match chunk {
                DATASDSC(_) => true,
                _ => false
            }
        }).unwrap();

        match map_chunk {
            DATASDSC(map) => map,
            _ => panic!()
        }
    }

    pub fn matchhistory_id(&self) -> u64 {
        self.game_data().matchhistory_id
    }
}
