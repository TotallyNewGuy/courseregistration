use client::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:4222";
    let mut c = Client::connect(addr).await?;
    let mut rx = c.sub_message("test", None).await?;
    loop {
        let r = rx.recv().await;
        if r.is_none() {
            break;
        }
        let r = r.unwrap();
        println!("{} receive on test {}", "rust", unsafe {
            std::str::from_utf8_unchecked(r.as_slice())
        });
    }
    // for i in 0..10 {
    //     let r = rx.recv().await;
    //     if r.is_none() {
    //         break;
    //     }
    //     let r = r.unwrap();
    //     println!("{} receive on test {}", i, unsafe {
    //         std::str::from_utf8_unchecked(r.as_slice())
    //     });
    // }
    Ok(())
}