use std::sync::Arc;

use futures_channel::mpsc::UnboundedSender;
use futures_util::io::ReadHalf;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::net::TcpListener;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::Mutex;
use tokio_tungstenite:: tungstenite::Message;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct TcpTunnel {}

impl TcpTunnel {

    // async fn read_tcp_in(tcpin_tx: UnboundedSender<Message>, tcp_read: ReadHalf) -> () {
    //     loop {
    //         let mut buf = [0; 4096];
    //         match tcp_read.read(&mut buf).await {
    //             Err(e) => {
    //                 println!("[TCP] error: {:?}", e);
    //                 break;
    //             },
    //             Ok(n) => {
    //                 let read = buf.to_vec().get(..n).unwrap().to_vec();
    //                 tcpin_tx.unbounded_send(Message::binary(read)).unwrap();
    //             }
    //         }
    //     }
    // }

    // async fn process(mut inbound: TcpStream) {
    //     let (mut tcp_read, tcp_write) = inbound.split();
    //     let (tcpin_tx, tcpin_rx) = futures_channel::mpsc::unbounded::<Message>();
    //     tokio::spawn(async move {
    //         loop {
    //             let mut buf = [0; 4096];
    //             match tcp_read.read(&mut buf).await {
    //                 Err(e) => {
    //                     println!("[TCP] error: {:?}", e);
    //                     break;
    //                 },
    //                 Ok(n) => {
    //                     let read = buf.to_vec().get(..n).unwrap().to_vec();
    //                     tcpin_tx.unbounded_send(Message::binary(read)).unwrap();
    //                 }
    //             }
    //         }
    //     });
    //     let (ws_stream, _) = Box::leak(Box::new(
    //         tokio_tungstenite::connect_async("ws://test-tcp-tunnel.oto.tools:9999/.well-known/otoroshi/tunnel").await.unwrap()
    //     ));
    //     let (ws_write, ws_read) = ws_stream.split();
    //     let tcpin_to_ws = tcpin_rx.map(Ok).forward(ws_write);
    //     let ws_to_tcpout = {
    //         ws_read.for_each(|message| async move {
    //             let data = message.unwrap().into_data();
    //             tcp_write.write_all(&data).await.unwrap();
    //         })
    //     };
    //     futures_util::pin_mut!(tcpin_to_ws, ws_to_tcpout);
    //     futures::future::select(tcpin_to_ws, ws_to_tcpout).await;
    // }

    pub async fn start() -> () {

        let listen_addr ="127.0.0.1:8081".to_string();

        println!("Listening on: {}", listen_addr);

        let listener = TcpListener::bind(listen_addr).await.unwrap();

        // while let Ok((mut inbound, _)) = listener.accept().await {
        //     tokio::spawn(async move {
        //         Self::process(inbound).await;
        //     });
        // }

        while let Ok((mut inbound, _)) = listener.accept().await {
            tokio::spawn(async move {
                let (mut tcp_read, tcp_write) = inbound.split();
                let (tcpin_tx, tcpin_rx) = futures_channel::mpsc::unbounded::<Message>();
                let (tcpout_tx, tcpout_rx) = futures_channel::mpsc::unbounded::<Vec<u8>>();
                tokio::spawn(async move {
                    loop {
                        let mut buf = [0; 4096];
                        match tcp_read.read(&mut buf).await {
                            Err(e) => {
                                println!("[TCP] error: {:?}", e);
                                break;
                            },
                            Ok(n) => {
                                let read = buf.to_vec().get(..n).unwrap().to_vec();
                                tcpin_tx.unbounded_send(Message::binary(read)).unwrap();
                            }
                        }
                    }
                });
                let (ws_stream, _) = Box::leak(Box::new(
                    tokio_tungstenite::connect_async("ws://test-tcp-tunnel.oto.tools:9999/.well-known/otoroshi/tunnel").await.unwrap()
                ));
                let (ws_write, ws_read) = ws_stream.split();
                let tcpin_to_ws = tcpin_rx.map(Ok).forward(ws_write);
                let tcpout_to_tcp = tcpout_rx.for_each(|vec| async move {
                    tcp_write.write(&vec).await;
                }); //.map(Ok).forward(tcp_write);
                let ws_to_tcpout = {
                    ws_read.for_each(|message| async move {
                        let data = message.unwrap().into_data();
                        tcpout_tx.clone().unbounded_send(data).unwrap();
                    })
                };
                futures_util::pin_mut!(tcpin_to_ws);
                futures_util::pin_mut!(ws_to_tcpout);
                futures_util::pin_mut!(tcpout_to_tcp);
                futures::future::join3(tcpin_to_ws, ws_to_tcpout, tcpout_to_tcp).await;
            });
        }
    }    


            
            //tokio::spawn(async move {
            //    let (ws_stream, _) = Box::leak(Box::new(tokio_tungstenite::connect_async("ws://test-tcp-tunnel.oto.tools:9999/.well-known/otoroshi/tunnel").await.unwrap()));
            //    let ws_stream_ref = Arc::new(Mutex::new(ws_stream));
            //    let ws_stream_ref_2 = ws_stream_ref.clone();
            //    let inbound_ref = Arc::new(Mutex::new(inbound));
            //    let inbound_ref_2 = inbound_ref.clone();
            //    tokio::spawn(async move {
            //        while let Some(msg) = ws_stream_ref.clone().lock().await.next().await {
            //            let msg = msg.unwrap();
            //            println!("[WS] got message: {}", msg.is_binary());
            //            if msg.is_binary() {
            //                let data = msg.into_data();
            //                println!("[WS] write data into the pipe: {:?}", String::from_utf8(data.to_vec()).unwrap());
            //                match inbound_ref.clone().lock().await.write_all(&data).await {
            //                    Err(e) => println!("[WS] error: {}", e),
            //                    Ok(_) => println!("[WS] ok")
            //                };
            //                println!("done !")
            //            }
            //        }
            //    });
            //    loop {
            //        let mut buf = [0; 4096];
            //        match inbound_ref_2.clone().lock().await.read(&mut buf).await {
            //            Err(e) => {
            //                println!("[TCP] error: {:?}", e);
            //                break;
            //            },
            //            Ok(n) => {
            //                let read = buf.to_vec().get(..n).unwrap().to_vec();
            //                println!("[TCP] read {} bytes {}: '{:?}'", read.len(), n, String::from_utf8(read.clone()).unwrap());
            //                let wut = ws_stream_ref_2.clone();
            //                let mut ws_stream = wut.lock().await;
            //                let res = ws_stream.send(Message::Binary(read));
            //                let _ = match res.await {
            //                    Err(e) => println!("[TCP] error while sending to ws: {}", e), // TODO: 
            //                    Ok(_) => println!("[TCP] sent"),// TODO: 
            //                };
            //            }
            //        }
            //        // inbound_ref_2.clone().lock().await.readable().await.unwrap();
            //        // let mut buf = [0; 4096];
            //        // match inbound_ref_2.clone().lock().await.try_read(&mut buf) {
            //        //     Ok(0) => {
            //        //         println!("[TCP] end of the stream !");
            //        //         break;
            //        //     },
            //        //     Ok(n) => {
            //        //         println!("[TCP] read {} bytes", n);
            //        //         // let ws_stream = ws_stream_ref.clone().lock().unwrap();
            //        //         let wut = ws_stream_ref_2.clone();
            //        //         let mut ws_stream = wut.lock().await;
            //        //         let res = ws_stream.send(Message::Binary(buf.to_vec()));
            //        //         let _ = match res.await {
            //        //             Err(e) => (), // TODO: 
            //        //             Ok(_) => (),// TODO: 
            //        //         };
            //        //     }
            //        //     Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
            //        //         println!("[TCP] error would block");
            //        //         continue;
            //        //     }
            //        //     Err(e) => {
            //        //         println!("[TCP] error: {:?}", e);
            //        //         break;
            //        //     }
            //        // }
            //    };
            //    ()
            //});
    
}