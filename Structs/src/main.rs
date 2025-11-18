use std::io::{self, Read, Write};
use std::time::Duration;

fn main() {
    let ports = serialport::available_ports().expect("Không tìm thấy cổng nối tiếp nào!");
    if ports.is_empty() {
        println!("Không tìm thấy cổng nối tiếp nào.");
        return;
    }

    println!("Các cổng nối tiếp có sẵn:");
    for (i, port) in ports.iter().enumerate() {
        println!("{}: {}", i, port.port_name);
    }

    println!("\nVui lòng chọn một cổng (nhập số thứ tự):");
    let mut port_index_str = String::new();
    io::stdin().read_line(&mut port_index_str).unwrap();
    let port_index: usize = port_index_str.trim().parse().expect("Vui lòng nhập một số hợp lệ.");

    println!("Vui lòng nhập tốc độ baud (ví dụ: 9600, 115200):");
    let mut baud_rate_str = String::new();
    io::stdin().read_line(&mut baud_rate_str).unwrap();
    let baud_rate: u32 = baud_rate_str.trim().parse().expect("Vui lòng nhập một số hợp lệ.");

    let port_name = &ports[port_index].port_name;
    println!("\nĐang mở cổng '{}' với tốc độ baud {}...", port_name, baud_rate);

    let mut port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Không thể mở cổng nối tiếp.");

    println!("Đang lắng nghe dữ liệu... (sẽ được in dưới dạng mã HEX)");

    let mut serial_buf: [u8; 128] = [0; 128];
    loop {
        match port.read(&mut serial_buf) {
            Ok(bytes_read) => {
                for i in 0..bytes_read {
                    print!("{:02X} ", serial_buf[i]);
                }
                io::stdout().flush().unwrap();
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => {
                eprintln!("\nLỗi: {:?}", e);
                break;
            },
        }
    }
}