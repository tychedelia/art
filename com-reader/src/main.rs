use std::time::Duration;

fn main() {
    read_port();
}

fn read_port() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let mut port = serialport::new("COM4", 115_200)
        .timeout(Duration::from_millis(2000))
        .open().expect("Failed to open port");

    let mut serial_buf: Vec<u8> = vec![0; 32];
    loop {
        port.read(serial_buf.as_mut_slice()).expect("Found no data!");

        let s = String::from_utf8_lossy(&serial_buf);
        print!("{}", s);
    };
}
