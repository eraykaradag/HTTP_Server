use crate::http::method;
use crate::http::request;

use super::QueryString;
use super::method::Method;
use super::method::MethodError;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Result as FmtResult;
use std::str;
use std::str::from_utf8;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = from_utf8(buf)?;


        let (method,request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path,request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol,_) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return  Err(ParseError::InvalidProtocol);
        }
        let method:Method = method.parse()?;
        let mut query_string = None;
        if let Some(i) = path.find('?'){
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }
        
        Ok(Self{
            path,
            query_string,
            method
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str,&str)> {
    for (i,c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i],&request[i+1..]));
        }
    }
    None
}
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}
impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError{
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError{
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
impl Error for ParseError {}
