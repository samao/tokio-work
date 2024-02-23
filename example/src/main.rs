/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-23 16:35:59
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-23 17:14:52
 */

use std::error::Error;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        println!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                println!("读取数据: {}", n);
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
