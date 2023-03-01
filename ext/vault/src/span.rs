use nom::IResult;
use nom_locate::LocatedSpan;
use nom_tracable::TracableInfo;

pub type Span<'a> = LocatedSpan<&'a [u8], TracableInfo>;

pub type ParserResult<'a, T> = IResult<Span<'a>, T>;
