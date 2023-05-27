use std::net::TcpStream;
use ssh2::Session;

pub fn connect() -> Session {
    let tcp: TcpStream;
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
                    tcp = successful_tcp;
                    break;
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

    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    println!("Enter password:");
    sess.userauth_password(&connection_username.trim(), &getln_no_echo![].trim()).unwrap();

    assert!(sess.authenticated());
    sess
}


// use std::net::TcpStream;
// use ssh2::Session;

// pub fn connect () -> Session {
//     let tcp: TcpStream;
//     let mut connection_address = String::from("");
//     let mut connection_username = String::from("");
//     loop {
//         println!["Enter connection address, then username:\n"];
//         connection_address = getln![];
//         connection_username = getln![];
//         // Connect to a SSH server
//         match TcpStream::connect(&connection_address) {
//             Ok(sucessful_tcp) => {
//                 tcp = sucessful_tcp;
//                 break
//             },
//             Err(e) => {
//                 println!["ERROR : {}", e];
//                 println!["Please try again!\n"];
//                 continue
//             },
//         }
//     }
//     let mut sess = Session::new().unwrap();
//     sess.set_tcp_stream(tcp);
//     sess.handshake().unwrap();
//     println!["Enter password (input will not echo to tty): "];
//     sess.userauth_password(&connection_username, &getln_no_echo![]).unwrap();
//     assert!(sess.authenticated());
//     sess     
// }