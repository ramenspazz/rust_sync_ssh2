use std::net::TcpStream;
use ssh2::Session;

/// Purpose
/// A structure that encapsules an ssh2 socket connection
pub struct SocketSSH {
    session_handle: ssh2::Session,
    connection_address: String,
    connection_port: String,
    connection_username: String,
    session_valid: bool,
}

impl SocketSSH {
    pub fn new() -> Self {
        let mut connection_address = String::new();
        let mut connection_port = String::new();
        let mut connection_username = String::new();
        loop {
            println!("Enter connection address, port, and username:");
            connection_address.clear();
            connection_port.clear();
            connection_username.clear();
        
            getln![connection_address];
            getln![connection_port];
            getln![connection_username];
        
            let address = connection_address.trim();
            let port = connection_port.trim().parse::<u16>();
        
            if let Ok(port) = port {
                let address = format!("{}:{}", address, port);
        
                match TcpStream::connect(&address) {
                    Ok(successful_tcp) => {
                        let mut session_handle: Session = Session::new().unwrap();
                        session_handle
                            .set_tcp_stream(successful_tcp);
                        println!["Connection Validated"];
                        let session_valid = false;
                        return Self { session_handle, connection_address, connection_port, connection_username, session_valid }
                    }
                    Err(e) => {
                        println!("ERROR: {}", e);
                        println!("Please try again!\n");
                        continue;
                    }
                }
            } else {
                println!("Invalid port value. Please try again!\n");
                continue;
            }
        }
    }

    /// connects to an active socket if session valid is true.
    pub fn connect(&mut self) -> bool {
        match self.session_valid {
            false => {
                self.session_handle
                    .handshake()
                    .unwrap();
                
                for i in 0..3 {
                    println!("Enter password (typed characters will not display):");
                    let attempted_login_result = self.session_handle.userauth_password(&self.connection_username.trim(),&getln_no_echo![].trim());
                    match attempted_login_result {
                        Ok(_) => {
                            if self.session_handle.authenticated() == true {
                                // authentication was sucessful, so we can go ahead and return from the fuction.
                                self.session_valid = true;
                                return true
                            }
                            else {
                                panic!["Idk wtf happened but it did here \"if self.session_handle.authenticated() == true\" on ln 70"]
                            }
                        },
                        Err(_) => {
                            println!["Incorrect user credentials. Please try again ({}/3)", i+1];
                        },
                    }
                    
                }
                // password was incorrect more than the retry limit
                return false
            },
            true => {
                // the session did not have a valid session handle, so we will not attempt to connect and will instead throw an error.
                panic!["Session has no valid handle! Please set a valid connection address, port, and username first before attempting to connect!"];
            }
        }
    }

    pub fn disconnect(&mut self) -> Result<(), ssh2::DisconnectCode> {
        match &self.session_valid {
            true => {
                let mut temp_handle = self.session_handle.channel_session().unwrap();
                for i in 1u8..=5u8 {
                    match temp_handle.close() {
                        Ok(_) => {
                            println!["Connection sucessfully closed!"];
                            self.session_valid = false;
                            return Ok(())
                        },
                        Err(_) => {
                            if i < 5 {
                                println!["Error closing connection ({}/5). Retrying in 1 seconds...\n", 1];
                                std::thread::sleep(std::time::Duration::from_millis(1000));
                                continue;
                            }
                            else if i == 5 {
                                println!["Error closing connection. Retry count reached! Could not close connection."];
                                return Err(ssh2::DisconnectCode::ServiceNotAvailable)
                            }
                        },
                    }        
                }
            },
            false => {
                // exit program as there was no active connection anyways and we are safe to exit without doing anything.
                return Ok(())
            }
        }
        Ok(())
    }

    pub fn run_command(self, command: &str) -> Result<(), ()> {
        if self.session_valid == true {
            // run commands n stuff
            match self.session_handle.channel_session().unwrap().exec(command) {
                Ok(_) => todo!(),
                Err(_) => todo!(),
            }
        }
        else {
            // session isnt connected, so return an Err
            return Err(())
        }
    }

}

impl Drop for SocketSSH {
    fn drop(&mut self) {
        println!["drop called"];
        match self.disconnect() {
            Ok(_) => println!["object dropped sucessfully!"],
            Err(_) =>println!["Error : Service not available"],
        }
    }
}