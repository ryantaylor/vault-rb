use nom::bytes::complete::take;
use nom::combinator::{cut, eof, flat_map, map, map_parser, peek};
use serde::{Serialize, Deserialize};
use nom::multi::{length_count, many_m_n, many_till};
use nom::number::complete::{le_u32, le_u8};
use nom::{Offset, Parser};
use nom::branch::alt;
use nom::sequence::tuple;
use crate::parser::{parse_utf16_variable, take_n, verify_le_u32};
use crate::span::{ParserResult, Span};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[magnus::wrap(class = "Vault::Tick")]
pub enum Tick {
    Command(CommandTick),
    Message(MessageTick)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[magnus::wrap(class = "Vault::CommandTick")]
pub struct CommandTick {
    pub id: u32,
    pub tick_type: u32
}

impl CommandTick {
    pub fn parse_tick(input: Span) -> ParserResult<Tick> {
        map(
            tuple((
                verify_le_u32(0),
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
                Tick::Command(CommandTick {
                    id,
                    tick_type
                })
            }
        )(input)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[magnus::wrap(class = "Vault::Message")]
pub struct Message {
    pub name: String,
    pub message: String
}

impl Message {
    pub fn parse_message(input: Span) -> ParserResult<Message> {
        cut(
            map(
                tuple((
                    parse_utf16_variable(le_u32),
                    parse_utf16_variable(le_u32)
                )),
                |(
                     (_, name),
                     (_, message)
                 )| {
                    Message {
                        name,
                        message
                    }
                }
            )
        )(input)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[magnus::wrap(class = "Vault::MessageTick")]
pub struct MessageTick {
    pub tick_type: u32,
    pub messages: Vec<Message>,
    pub position: isize
}

impl MessageTick {
    pub fn parse_tick(input: Span) -> ParserResult<Tick> {
        map(
            tuple((
                le_u32,
                map_parser(
                    flat_map(le_u32, take),
                    Self::parse_message
                )
            )),
            |(
                 tick_type,
                 messages
             )| {
                Tick::Message(MessageTick {
                    tick_type,
                    messages,
                    position: -1
                })
            }
        )(input)
    }

    fn parse_message(input: Span) -> ParserResult<Vec<Message>> {
        let (_, num_messages) = peek(le_u32)(input)?;

        if num_messages == 0 {
            Self::parse_empty_message(input)
        } else {
            Self::parse_content_message(input, num_messages)
        }
    }

    fn parse_empty_message(input: Span) -> ParserResult<Vec<Message>> {
        cut(
            map(
                tuple((
                    le_u32,
                    flat_map(le_u32, take)
                )),
                |(_, _)| { Vec::new() }
            )
        )(input)
    }

    fn parse_content_message(input: Span, num_messages: u32) -> ParserResult<Vec<Message>> {
        cut(
            map(
                tuple((
                    le_u32,
                    le_u32,
                    le_u32,
                    le_u32,
                    le_u32,
                    many_m_n(1, num_messages as usize, Message::parse_message)
                )),
                |(_, _, _, _, _, messages)| {
                    messages
                }
            )
        )(input)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[magnus::wrap(class = "Vault::Ticks")]
pub struct Ticks {
    pub command_ticks: Vec<CommandTick>,
    pub message_ticks: Vec<MessageTick>
}

impl Ticks {
    pub fn parse_ticks(input: Span) -> ParserResult<Ticks> {
        map(
            many_till(
                alt((
                    CommandTick::parse_tick,
                    MessageTick::parse_tick
                )),
                eof
            ), |(ticks, _)| {
                let (command_ticks, message_ticks): (Vec<Tick>, Vec<Tick>) =
                    ticks
                        .into_iter()
                        .partition(|tick| {
                            match tick {
                                Tick::Command(_) => true,
                                Tick::Message(_) => false
                            }
                        });
                Ticks {
                    command_ticks: command_ticks
                        .into_iter()
                        .map(|command_tick| {
                            if let Tick::Command(tick) = command_tick {
                                tick
                            } else {
                                panic!()
                            }
                        })
                        .collect(),
                    message_ticks: message_ticks
                        .into_iter()
                        .map(|message_tick| {
                            if let Tick::Message(tick) = message_tick {
                                tick
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
            }
        )(input)
    }
}
