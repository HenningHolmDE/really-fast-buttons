use rfb_proto::{SensorRequest, SensorResponse};

// use serialport::SerialPort;

fn main() {
    let mut port = serialport::new("COM1", 9600).open().unwrap();

    println!("Sensor dummy waiting for requests...");
    let mut request = [0u8; 1];
    loop {
        port.read_exact(&mut request).unwrap();
        let request = rfb_proto::from_bytes(dbg!(&request));
        match dbg!(request) {
            Ok(SensorRequest::GetCount) => {
                let count = 1337; // COUNTER.swap(0, Ordering::SeqCst);
                let response = SensorResponse::Count(count as u32);
                let bytes: rfb_proto::Vec<u8, 5> = rfb_proto::to_vec(&response).unwrap();
                port.write_all(dbg!(&bytes)).unwrap();
            }
            Ok(SensorRequest::WhoAreYou) => {
                let response = SensorResponse::IAm("sensor-dummy       ");
                let bytes: rfb_proto::Vec<u8, 21> = rfb_proto::to_vec(&response).unwrap();
                port.write_all(dbg!(&bytes)).unwrap();
            }
            _ => (),
        }
    }
}
