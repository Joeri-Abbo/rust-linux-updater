use ssh2::Session;
use std::net::TcpStream;
use std::fs::File;
use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize)]
// #[serde(rename_all = “camelCase”)]
struct Device {
    ip: String,
    username: String,
    password: String,
}

fn main() {
    let json_file_path = Path::new("devices.json");

    let file = File::open(json_file_path).expect("file not found");
    let devices: Vec<Device> = serde_json::from_reader(file).expect("error while reading");
    for device in devices {
        let ip = device.ip;
        let password = device.password;
        let username = device.username;

        println!("Connecting to {}", &ip);
        let tcp = TcpStream::connect(&ip).unwrap();
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();

        sess.userauth_password(&username, &password)
            .unwrap();
        let mut channel = sess.channel_session().unwrap();
        println!("Updating {}", &ip);
        //
        channel.exec("sudo apt update && sudo apt -y upgrade && sudo apt -y dist-upgrade").unwrap();
        channel.wait_close().ok();
        // println!("{}", channel.exit_status().unwrap());
        println!("Finished {}", &ip);

    }
}