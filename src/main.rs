use std::net::{SocketAddr, UdpSocket};
use std::fs::OpenOptions;
use std::io::Write;
use gran_turismo_query::constants::{PACKET_HEARTBEAT_DATA, PACKET_SIZE};
use gran_turismo_query::packet::Packet;

fn main() {
    // Create output file
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("telemetry.ndjson")
        .expect("Failed to open output file");

    // Socket setup
    let socket = UdpSocket::bind("0.0.0.0:33740").expect("Failed to bind socket");
    let destination: SocketAddr = format!("{}:{}", "192.168.178.99", 33739)
        .parse()
        .expect("Invalid address");

    // Initial handshake
    socket
        .send_to(PACKET_HEARTBEAT_DATA, destination)
        .expect("Failed to send initial heartbeat");

    let mut packets_received = 0;

    loop {
        socket
            .send_to(PACKET_HEARTBEAT_DATA, destination)
            .expect("Failed to send packet");

        let mut buf = [0u8; PACKET_SIZE];
        socket
            .recv_from(&mut buf)
            .expect("Failed to receive packet");

        let packet = Packet::try_from(&buf).expect("Failed to parse packet");

        // Write as NDJSON (one JSON object per line)
        serde_json::to_writer(&mut file, &packet).expect("Failed to serialize packet");
        file.write_all(b"\n").expect("Failed to write newline");
        file.flush().expect("Failed to flush file");

        packets_received += 1;
        print!("\rRecorded {} packets", packets_received);

        // Heartbeat every 100 packets
        if packet.packet_id % 100 == 0 {
            socket
                .send_to(PACKET_HEARTBEAT_DATA, destination)
                .expect("Failed to send heartbeat");
        }
    }
}
