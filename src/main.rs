#[macro_use]
pub mod getln_macro;
pub mod connection_socket;
pub mod connect_to_client;

fn main() {
    let mut ssh_connection = connection_socket::SocketSSH::new();
    if ssh_connection.connect() == true {println!["Auth sucess!\n"];} else { println!["Auth failed!\n"]}
    println!["Exiting program..."];

}
