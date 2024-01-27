use meter_core::{start_capture, get_most_common_ip};
fn main() {
    let ip = meter_core::get_most_common_ip().unwrap();
    let port = 6040;
    let rx = meter_core::start_capture(ip, port).unwrap();
    while let Ok((op, data)) = rx.recv() {
        println!("{:?}\n\n{:?}", op, data);
    }

}