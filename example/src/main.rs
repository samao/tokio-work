/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-23 16:35:59
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-23 17:42:21
 */

use std::error::Error;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // handle_client(socket).await.unwrap();
            loop {
                // 缓冲区不够会连续循环读取
                let mut buf = [0; 1024];
                // 读满了1024 resolved
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        println!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                println!("读取数据: {} -> have: {}", n, buf.len());
                // 1.合并成二进制切片
                // let bytes = ["rust".as_bytes(), &buf].concat().as_slice();
                // 2.合并字符串转成二进制
                let a = String::from_utf8_lossy(&buf);
                let send_str = format!("rust server: {}", a);

                if let Err(e) = socket.write_all(send_str.as_bytes()).await {
                    println!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

#[allow(dead_code)]
async fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = Vec::new();

    loop {
        let mut chunk = [0; 1024];

        let byte_read = stream.read(&mut chunk).await?;

        if byte_read == 0 {
            break;
        }

        buffer.extend_from_slice(&chunk[..byte_read]);
    }
    println!("读到的数据是: {:?}", String::from_utf8(buffer));
    Ok(())
}
