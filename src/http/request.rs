use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult, Debug};
use std::str;
use std::str::Utf8Error;

pub struct Request {
    path: String,
    query_string: Option<String>, 
    method: Method
}

// impl Request {
//     pub fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
//         unimplemented!()
//     }
// }

impl TryFrom<&[u8]> for Request {
    // type Error = String; 
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        /*
        str::from_utf8(buf)
        버퍼를 요청으로 변경하기 위해서는 버퍼 슬라이스를 문자열 슬라이스로 변경해야 한다.
        str의 from_utf8은 버퍼 슬라이스를 문자열 슬라이스로 변경함과 동시에 utf8 형태인지 확인해 준다.
         */

        // 패턴 1. 
        // match str::from_utf8(buf) {
        //     Ok(request) => {  }
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }

        // 패턴 2. 러스트에서 권장하는 패턴임.
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => { unimplemented!() }
        //     Err(e) => return Err(e), 
        // }

        // 패턴3. ? 연산자는 match 구문과 비슷한 일을 함. 
        // ? 연산자는 반환할 오류와 매칭할 오류가 명시되어 있어야 함. 그렇지 않으면 필요한 트레이트를 직접 구현해줘야 함.
        // let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
        let request = str::from_utf8(buf)?;

        // 여기서 로컬 변수 request를 재사용하고 있기 때문에 shadowing이 발생한다. 이제 이전의 request 는 사용할 수 없다.
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        unimplemented!()
    }   
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i+1..]));
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
            Self::InvalidRequest => "INVALID REQUEST", 
            Self::InvalidEncoding => "INVALID ENCODING",
            Self::InvalidProtocol => "INVALID PROTOCOL",
            Self::InvalidMethod => "INVALID METHOD",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Error for ParseError {
        
}