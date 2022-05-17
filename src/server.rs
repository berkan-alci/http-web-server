use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

//Self is alias for struct
impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {} ", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            // same as while true

            match listener.accept() {
                // cover every possible pattern, for enums or as switch statement
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer))
                        }
                        Err(err) => println!("Failed to read from connection: {}", err),
                    }
                }
                Err(err) => println!("Failed to establish a connection: {}", err),
            }
        }
    }
}
