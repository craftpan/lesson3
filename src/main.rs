// 引入std依赖
use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, thread, str};

fn main() {
    // 定义请求地址及端口
    let addr:String = "127.0.0.1:8899".to_string();
    // 创建Tcp监听
    let listener:TcpListener = TcpListener::bind(&addr).unwrap();
    // 接收客户端信息
    for stream in listener.incoming() {
        // 连接建立
        println!("Debug: New user is coming");
        // 模式匹配
        match stream {
            // 当Resut 匹配 Ok
            Ok(stream) => {
                // 连接成功
                thread::spawn(move || {
                    // 调用handle_client
                    handle_client(stream);
                });
            }
            // 当Result 枚举匹配错误
            Err(e) => {
                panic!("Debug: error {:?}", e)
            }
        }
    }
    // 关闭Tcp监听 
    drop(listener);
}

// 线程调用处理函数
fn handle_client(mut stream: TcpStream) {
    println!("Debug: client handle...");
    // 定义存储数组
    let mut buf = [0; 512];
    // 循环，反复读取客户的输入信息
    loop {
        // read方法读取=
        let bytes_read = stream.read(&mut buf).expect("Debug: read error -> stop script");
        println!("Debug: byte size: {}", bytes_read);

        // 如果输入字符为空，直接退出
        if bytes_read == 0 {
            break;
        }
        // 将buyte[]转换为str类型，方便对比
        let s = match str::from_utf8(&buf[..bytes_read]) {
            Ok(v) => v,
            Err(_e) => {
                stream.write(b"Need utf-8 sequence.").unwrap();
                continue;
            }
        };

        // 如果输入的前3个字符是 bye 终止程序
        if s.len() >= 3 && s[0..3] =="bye".to_string() {
            stream.write(b"bye bye\n").unwrap();
            break;
        }
        // 如果程序没有终止，返回输入的消息，也就是输入什么返回什么，unwrap() 表示不处理错误，遇到错误直接出错退出程序。
        stream.write(&buf[..bytes_read]).unwrap();
    }
}