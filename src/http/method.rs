use::std::str::FromStr;

pub enum Method {
    GET,
    POST,
    DELETE,
    PUT,
    PATCH,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s { 
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::DELETE),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            _ => Err(MethodError), // default 
        };
        unimplemented!();
    }
}

// make custom error
pub struct MethodError;