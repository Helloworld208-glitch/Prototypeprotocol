#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use core::time;
use std::error::Error;
use std::f64::consts;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

use std::net::{SocketAddr, SocketAddrV4, UdpSocket as StdUdpSocket};
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio::{
    runtime::Handle,
    time::{Duration, sleep},
};
struct Header {
    seq: u32,
    flag: u8,
}
struct PACKET_ARR {
    buff: Vec<[u8; 1000]>,
    seq: Vec<u32>,
    written: Vec<bool>,
}
struct Message {
    accseq: u32,
}

struct ack {
    base: u64,
    reading_base: u64,
}
async fn chk_seq(packer: &PACKET_ARR, mut base: usize, reading_base: usize) -> (usize, usize) {
    let mut co: usize = 0;

    if base == 0 {
        if packer.seq[base] == 0 {
            return (0, co);
        }
    }

    loop {
        base = (base + 1) % 1000;
        if base == reading_base {
            if base == 0 && co == 0 {
                return (0, co);
            }
            if base == 0 && co != 0 {
                return (999, co);
            }
            return (base - 1, co);
        }

        if packer.seq[base] == 0 {
            if base == 0 && co == 0 {
                return (0, co);
            }
            if base == 0 && co != 0 {
                return (999, co);
            }
            base = base - 1;
            return (base, co);
        }
        println!("the the base is {}", base);

        co = co + 1;
    }
}

async fn writed(packet: &mut PACKET_ARR, mut index: usize, base: usize, file: &mut File) -> usize {
    if index == base && base == 0 {
        return 0;
    }
    if index < base {
        let length = (packet.seq[index]) as usize;
        let address = &packet.buff[index][13..length];
        file.write_all(address).await.unwrap();

        packet.seq[index] = 0;
        packet.written[index] = false;
        index = (index + 1) % 1000;
        return index;
    }
    if index == base {
        return index;
    }

    if index > base {
        if packet.seq[index] == 0 {
            return index;
        }
        let length = (packet.seq[index]) as usize;
        let address = &packet.buff[index][13..length];
        file.write_all(address).await.unwrap();
        packet.seq[index] = 0;
        packet.written[index] = false;
        index = (index + 1) % 1000;

        return index;
    }
    return index as usize;
}
fn to_normal(buf: &mut [u8]) -> (u8, usize, usize) {
    let flags = buf[0];
    let sequence = u32::from_be_bytes(buf[1..5].try_into().unwrap()) as usize;
    let size = u64::from_be_bytes(buf[5..13].try_into().unwrap()) as usize;
    println!("flag {}", flags);
    println!("  real  number of sequence{}", sequence);
    println!(" the size of file is {} ", size);
    return (flags, sequence, size);
}
#[tokio::main]

async fn main() {
    let addr = "127.0.0.1:5000";
    let ud = UdpSocket::bind(addr).await.unwrap();
    let mut old_cha = ack {
        base: 0,
        reading_base: 0,
    };
    let mut buf = [0u8; 1000];

    let mut buff1 = [0u8; 13];
    let mut packets = PACKET_ARR {
        buff: vec![[0u8; 1000]; 1000],
        seq: vec![0u32; 1000],
        written: vec![false; 1000],
    };
    let mut fsize = 1000;
    let mut filee = File::create_new(r"C:\Users\Informatics\Desktop\brave-pssortabidihd0vvvxxv0ldeg-x0ddwink60040-1.80.11,,f5-96-setup.exe")
        .await
        .unwrap();
    buff1.copy_from_slice(&mut buf[0..13]);

    let mut window = 0;
    let mut base = 0;
    let mut reading_base: usize = 0;
    let mut cooo: usize = 0;
    let mut adress1: Option<SocketAddr> = None;
    let flags: u8 = 0;
    let sequence: usize = 0;
    let size: usize = 0;
    let mut yoo: u8 = 0;
    let max_writing: usize = 0;
    loop {
        if flags == 1 {
            let buff2 = [0u8; 13];
            buff1.copy_from_slice(&mut buf[0..13]);

            println!("the flag is {}", size);
            break;
        }

        tokio::select! {

             Ok((len, addr)) =  ud.recv_from(&mut buf)=>{

               adress1 = Some(addr);
               println!("  {}",addr);
                     buff1.copy_from_slice(&mut buf[0..13]);

                     let (flags, sequence, size) = to_normal(&mut buff1);
                     println!("length is {}",len);
                     println!(" sequence is {}",sequence);
                     if ( sequence as i64)-(window as i64 + 1)*1000>=0{
                         packets.buff[sequence-(1000*(window+1))][..len-13  ].copy_from_slice(buf[13..len].try_into().unwrap());
                            packets.seq[sequence-(1000*(window+1))]=len as u32 ;
                            packets.written[sequence-(1000*(window+1))]=true;

                     }
                     else {

                         packets.buff[sequence-(1000*window)][..len-13  ].copy_from_slice(buf[13..len].try_into().unwrap());
                            packets.seq[sequence-(1000*window)]=len as u32 ;
                            packets.written[sequence-(1000*window)]=true;
                     }


                }




                     read_b =writed(&mut packets,reading_base,base, & mut filee)=>{
                  reading_base=read_b;
                }



        }

        if let Some(f) = adress1.as_ref() {
            (base, cooo) = chk_seq(&packets, base, reading_base).await;
            println!("base is {}", base);
            let x = ack {
                base: base as u64,
                reading_base: reading_base as u64,
            };

            let mut acks = [0u8; 16];
            acks[0..8].copy_from_slice(&x.base.to_be_bytes());
            acks[8..16].copy_from_slice(&x.reading_base.to_be_bytes());
            ud.send_to(&acks, *f).await.unwrap();
        }

        println!("{:?}", packets.seq);
        sleep(time::Duration::from_millis(100)).await;
    }
}
