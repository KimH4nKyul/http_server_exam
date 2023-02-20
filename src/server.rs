use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::net::TcpListener;
use std::io::Read;  // stream에서 read() 메서드를 사용하기 위해 Read 트레이트를 불러온다. 

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Listening on: {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) { // 서버에서 읽어야 할 바이트 스트림, Read 트레이트의 read() 메서드에 대한 구체적인 구현체를 포함한다. 
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            // as 키워드를 통해 참조를 바이트 슬라이스로 취급할 수 있다. 
                            // Request::try_from(&buffer as &[u8]); 
                            // 아래 방식은 위 주석처리된 문장과 동일한 방식임. (직접 바이트 슬라이스로 생성해주는 방식)
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                        
                                }, 
                                Err(e) => {
                                    println!("Faild to parse a request: {}", e);
                                }
                            }
                            
                        }, 
                        Err(e) => {
                            println!("Failed to read a buffer: {}", e);
                        }
                    }
                },
                Err(e) => {
                    println!("Failed to establish a connection: {}", e); 
                }
            }
        }
    }
}