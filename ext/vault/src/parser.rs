use std::io::Cursor;
use std::string::String;

use byteorder::{LittleEndian, ReadBytesExt};

use nom::{
    ToUsize, InputIter, InputTake, IResult,
    error::ParseError,
    bytes::complete::{take, take_while},
    combinator::{map, verify, peek, rest},
    multi::{count, many_till},
    number::complete::{le_u16, le_u32}};
use crate::span::Span;

pub fn verify_zero_u16(input: Span) -> IResult<Span, u16> {
    verify(le_u16, |n: &u16| *n == 0)(input)
}

pub fn verify_le_u32<'a>(expected: u32) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, u32> {
    verify(le_u32, move |n: &u32| *n == expected)
}

pub fn enforce_end_of_input(input: &[u8]) -> IResult<&[u8], &[u8]> {
    verify(rest, |bytes: &[u8]| bytes.len() == 0)(input)
}

pub fn parse_utf8_fixed<'a, E, T: ToUsize>(len: T) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, String, E>
    where
        E: ParseError<Span<'a>>
{
    map(take(len), |s: Span| String::from_utf8_lossy(s.fragment()).into_owned())
}

pub fn parse_utf8_variable<'a, O, E, F>(mut f: F) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, (O, String), E>
    where
        E: ParseError<Span<'a>>,
        F: FnMut(Span<'a>) -> IResult<Span<'a>, O, E>,
        O: ToUsize + Copy
{
    move |input: Span| {
        let (input, num) = f(input)?;
        let (input, res) = parse_utf8_fixed(num)(input)?;

        Ok((input, (num, res)))
    }
}

fn bytes_to_u16(bytes: &[u8]) -> Vec<u16> {
    let mut u16_vec = Vec::with_capacity(bytes.len() / 2);
    let mut cursor = Cursor::new(bytes);

    cursor.read_u16_into::<LittleEndian>(&mut u16_vec).unwrap();

    u16_vec
}

fn bytes_to_utf16(bytes: Span) -> String {
    let mut u16_vec = Vec::with_capacity(bytes.len() / 2);
    let mut cursor = Cursor::new(bytes.fragment());

    for _ in 1..=(bytes.len() / 2) {
        let val = cursor.read_u16::<LittleEndian>().unwrap();
        u16_vec.push(val);
    }

    String::from_utf16_lossy(&u16_vec)
}

pub fn parse_utf16_terminated(input: Span) -> IResult<Span, String> {
    map(
        many_till(
            le_u16,
            peek(verify(le_u16, |n: &u16| *n == 0))
        ), |(u16s, _)| String::from_utf16_lossy(&u16s)
    )(input)
}

pub fn parse_utf16_fixed<'a, E, T>(len: T) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, String, E>
    where
        E: ParseError<Span<'a>>,
        T: ToUsize
{
    let len = len.to_usize();
    let true_len = len * 2;

    map(
        take(true_len),
        |bytes: Span| bytes_to_utf16(bytes)
    )
}

pub fn parse_utf16_variable<'a, O, E, F>(mut f: F) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, (O, String), E>
    where
        E: ParseError<Span<'a>>,
        F: FnMut(Span<'a>) -> IResult<Span<'a>, O, E>,
        O: ToUsize + Copy
{
    move |input: Span| {
        let (input, num) = f(input)?;
        let (input, res) = parse_utf16_fixed(num)(input)?;

        Ok((input, (num, res)))
    }
}

pub fn take_zeroes(input: Span) -> IResult<Span, Span> {
    take_while(|n: u8| n == 0)(input)
}

pub fn count_n<I, O, E, F, P>(count_parser: impl Fn(I) -> IResult<I, P, E>, f: F) -> impl FnMut(I) -> IResult<I, (P, Vec<O>), E>
    where
        I: Clone + PartialEq,
        P: ToUsize,
        F: Fn(I) -> IResult<I, O, E>,
        E: ParseError<I>,
{
    move |input: I| {
        let (input, num) = count_parser(input)?;
        let (input, res) = count(&f, num.to_usize())(input)?;

        Ok((input, (num, res)))
    }
}

pub fn take_n<I, O: ToUsize, E: ParseError<I>>(count_parser: impl Fn(I) -> IResult<I, O, E>) -> impl FnMut(I) -> IResult<I, (O, I), E>
    where
        I: Clone + PartialEq + InputIter + InputTake
{
    move |input: I| {
        let (input, num) = count_parser(input)?;
        let (input, res) = take(num.to_usize())(input)?;

        Ok((input, (num, res)))
    }
}

pub fn take_u16<'a>(len: usize) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Vec<u16>> {
    map(
        take(len * 2),
        |bytes| bytes_to_u16(bytes)
    )
}
