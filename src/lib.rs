use nom::{
    bytes::complete::{tag, take_until},
    combinator::map,
    sequence::tuple,
    IResult,
};

pub mod translation_file;
pub mod ts_file;

fn fenced<'a>(start: &'a str, end: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    map(tuple((tag(start), take_until(end), tag(end))), |x| x.1)
}
