#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use core::time;
use std::error::Error;
use std::os::windows;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio::task::coop::has_budget_remaining;
use tokio::sync::watch;
use tokio::{
    runtime::Handle,
    time::{Duration, sleep},
};
#[derive(Debug)]

enum Message {
    Hello,
    Nooo,
    seq(u32),
    base(u32)
}
async fn calc(time: u64) -> i32 {
    println!("im sleeping bro");
    sleep(Duration::from_secs(time)).await;
    println!("im up");
    return 100;
}
async fn message_send(channel: Sender<Message>) {
    loop {
        match channel.send(Message::Hello).await {
            Ok(()) => sleep(Duration::from_secs(10)).await,
            Err(_) => {
                println!("big error");
                break;
            }
        };
    }
}
async fn getting(mut channel: Receiver<Message>) {
    let mut co = 0;
    while let Some(x) = channel.recv().await {
        println!("{:?} ", x);
        println!("nihh");
        co = co + 1;
        println!("{}", co);
    }
}
struct Header {
    seq: u32,
    flags: u8,
    size: u64,
}
struct cha{
   base:u32,
   available:u32,
   version:u64

}
struct arr{
    saving_base: Vec<[u8;1000]>,
    bytes_length:  Vec<u32>
}
struct WINDOW_SIZE {
    BASE: u64,
    Window_size: u32,
}
fn to_normal(buf: &mut [u8]) -> (u8, u32, u64) {
    let flags = buf[0];
    let sequence = u32::from_be_bytes(buf[1..5].try_into().unwrap());
    let size = u64::from_be_bytes(buf[5..13].try_into().unwrap());
    println!("flag {}", flags);
    println!("  real  number of sequence{}", sequence);
    println!(" the size of file is {} ", size);
    return (flags, sequence, size);
}
async fn ack(adrr: &str, ud: Arc<UdpSocket>, yo: watch::Sender<cha>,yo1 : watch::Sender<cha>) {
    
    let mut buf: [u8; 16] = [0u8; 16];
    let mut buf1 = [0u8; 8];
    let mut buf2 = [0u8; 8];
 let mut  base=0;
 let mut  window_base=0; 
 let mut  co:u64=3;

let mut  version:u64 =0 ;
 let mut  base_chk=0;
 let  mut available_places=0 ;
    loop {
        ud.recv_from(&mut buf).await.unwrap();
        
        buf1.copy_from_slice(&buf[0..8]);
         base  = u64::from_be_bytes(buf1);
        buf2.copy_from_slice(&buf[8..16]);
         window_base = u64::from_be_bytes(buf2);
         
        if base_chk==base {
            co=co-1;
            
        }
        if co==0 {
             if base==0 && window_base==0
        {
             available_places=999; 
        }
         if (base==window_base) && base!=0 && window_base!=0
        {
         available_places=window_base+999-window_base; 
        println!("available places {}",available_places)    
        }

        if (base<window_base) && base!=0 && window_base!=0
        {
         available_places=base-window_base-1; 
        println!("available places {}",available_places)    
        }


         if window_base<base 
        {
         available_places=window_base+(999-base);   
        println!("available places {}",available_places) 
        }


               
                 co=3;
                yo.send(cha { base: (base as u32), available: (available_places as u32),version:version }).unwrap();
                version=version+1;
        }
        else { 
            if base!=base_chk {
                co=3;
                
            }
            
        println!(
            "we got this from the sender base index in the  table {}",
            base
        );
        println!(
            "we got this from the sender reading base in the table {}",
            window_base
        );
        if base==0 && window_base==0
        {
            let available_places=999; 
        }
         if (base==window_base) && base!=0 && window_base!=0
        {
        let available_places=window_base+999-window_base; 
        println!("available places {}",available_places)    
        }
        if (base<window_base) && base!=0 && window_base!=0
        {
        let available_places=base-window_base-1; 
        println!("available places {}",available_places)    
        }

         if window_base<base 
        {
        let available_places=window_base+(999-base);   
        println!("available places {}",available_places) 
        }
      base_chk=base;
      yo1.send(cha { base: (base as u32), available: (available_places as u32),version:0 }).unwrap();
        }






    }
}
fn header_write(x: &Header, buff: &mut [u8]) -> usize {
    buff[0] = x.flags;
    buff[1..5].copy_from_slice(&x.seq.to_be_bytes());
    buff[5..13].copy_from_slice(&x.size.to_be_bytes());
    return 13;
}
use tokio::fs::File;
use tokio::net::UdpSocket;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "0.0.0.0:8080";
    let ud = Arc::new(UdpSocket::bind("0.0.0.0:8080").await?);
    let mut buf = [0u8; 1000];

    let mut file =
        File::open(r"C:\Users\Informatics\Desktop\brave-portable-win64-1.80.115-96-setup.exe")
            .await
            .unwrap();
    let mut windows = WINDOW_SIZE {
        BASE: 0,
        Window_size: 1000,
    };
    let mut saving_array=arr{saving_base:vec![[0u8;1000];1000],bytes_length:vec![0u32;1000]};
    let informations = File::metadata(&file).await.unwrap();
    let mut size = informations.len() as usize;
    let mut seq: u32 = 0;
    let mut buffer10: Vec<[u8; 1000]> = vec![[0u8; 1000]; 1000]; 

    let ud1 = ud.clone();
    let (mut rx, mut rt) = watch::channel(cha{available:0,base:0,version:0});
    let( mut rx1, mut rt1)=watch::channel(cha{available:0,base:0,version:0});
    let handle = tokio::spawn({ ack(addr, ud1, rx,rx1) });
    let mut buffer20: [i32; 1000] = [0; 1000];
    let mut co: usize = 0;
    let mut lengthhh=vec![0usize;1000];
    let mut  xxx= cha{available:0,base:0,version:0};
     let mut  max_writing:usize=0;
   
    loop {
        
      let( t,t1,t2)= {
       let   g = rt.borrow();
        ( g.available,g.base,g.version)
      }; 
 println!("available{}",t);
 if xxx.version!=t2  &&  seq!=0 {
    xxx.version=t2;
    xxx.base=(xxx.base+1)%1000;
    let  mut bufffff=vec![0u8;1000];
    loop {
        
        if xxx.base!=max_writing as u32 {
            
            let yyy1 = buffer20[xxx.base as usize] as usize;
        bufffff[0..yyy1 ].copy_from_slice(&buffer10[xxx.base as usize][0..yyy1]);
    
            ud.send_to(&bufffff[0..yyy1], "127.0.0.1:5000").await.unwrap();
            xxx.base=(xxx.base+1)%1000;
            println!("we are recovering");
        } else {
           
               break;      
}
    }

  
     
 }
 xxx.base=t1;
 println!("base{}",t1);
     let (rrr,rrr2)={
        let   g = rt1.borrow();
        ( g.available,g.base)
     };
        if (seq as u64) < (windows.Window_size as u64 + windows.BASE) {
            let bytes_readed = file.read( &mut buf[13..1000]).await.unwrap() as usize;
            if bytes_readed == 0 {
                break;
            }
            let head = Header {
                seq: seq,
                flags: 0,
                size: size as u64,
            };
            header_write(&head, &mut buf);
            ud.send_to(&mut buf[0..bytes_readed + 13], "127.0.0.1:5000")
                .await
                .unwrap();
            buffer10[co].copy_from_slice(&buf[0..bytes_readed + 13]);
            buffer20[co] = bytes_readed as i32;
            lengthhh[co]=bytes_readed + 13;
            max_writing=(max_writing+1)%1000;
            co = co + 1;
            seq = seq + 1;

            size = size - bytes_readed;

            println!("seq{}", seq);
        } else {
            
        }
        sleep(time::Duration::from_millis(10)).await;
    }
    

    Ok(())
}
