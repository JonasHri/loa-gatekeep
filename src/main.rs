fn main() {
    let ip = meter_core::get_most_common_ip().unwrap();
    let port = 6020;
    let rx = meter_core::start_capture(ip, port).unwrap();
    while let Ok((op, data)) = rx.recv() {
        println!("op:\n{:?}\n\ndata:\n{:?}", op, data);
    }
}
