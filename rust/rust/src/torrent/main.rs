use std::env;
use std::sync::TryLockError;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};



mod utils;
use utils::print_info;

mod peer;
use peer::print_peers;

mod handshake;
use handshake::send_handshake;

mod download;
use download::{download_cmd, download_piece_cmd};

mod state;
use crate::state::DOWNLOADED_DATA;
use crate::state::VEC_DATA;
use crate::state::GLOBAL_ARRAY;

fn main() {
   
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: <command> [args]");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "decode" => {
            if args.len() < 3 {
                eprintln!("Usage: decode <bencoded string>");
                std::process::exit(1);
            }
            let encoded_value = args[2].as_bytes();
            let decoded = utils::decode_bencoded_value(encoded_value);
            println!("{}", decoded);
        }

        "info" => {
            if args.len() < 3 {
                eprintln!("Usage: info <torrent file>");
                std::process::exit(1);
            }
            print_info(&args[2]);
        }

        "peers" => {
            if args.len() < 3 {
                eprintln!("Usage: peers <torrent file>");
                std::process::exit(1);
            }
            print_peers(&args[2]);
        }

        "handshake" => {
            if args.len() < 4 {
                eprintln!("Usage: handshake <torrent file> <peer ip:port>");
                std::process::exit(1);
            }
            send_handshake(&args[2], &args[3]);
        }

        "download_piece" => {
            if args.len() < 6 || args[2] != "-o" {
                eprintln!("Usage: download_piece -o <output file> <torrent file> <piece index>");
                std::process::exit(1);
            }

            let output_path = &args[3];
            let torrent_path = &args[4];
            let piece_index: usize = args[5].parse().expect("Invalid piece index");

            download_piece_cmd(output_path, torrent_path, piece_index);
        }

        "download" => {
            if args.len() < 5 || args[2] != "-o" {
                eprintln!("Usage: download -o <output file> <torrent file>");
                std::process::exit(1);
            }
            let output_path = &args[3];
            let torrent_path = &args[4];
            download_cmd(output_path, torrent_path);

            let datae = VEC_DATA.lock().unwrap();
            println!("Bytes en buffer: {}", datae.len());

            let data: std::sync::MutexGuard<'_, Vec<u8>> = DOWNLOADED_DATA.lock().unwrap();
            println!("Bytes descargados: {:?}", data);
        }
        "test" => {


            let mut array = GLOBAL_ARRAY.lock().unwrap();
            array.push(20);
            array.push(10);
            array.pop();
           // drop(array); // 🔓 libera el lock

           // remove_at(0); // ✅ ya no cuelga


            //if args.len() < 3 {
            //    eprintln!("Usage: info <torrent file>");
           //     std::process::exit(1);
          //  }
            
         //   let byte = 10;
          //  let mut array = GLOBAL_ARRAY.lock().unwrap();
          //  array.push(20);
          //  println!("Array actual despues de push 1 : {:?}", *array);
          //  array.push(byte);
          //  println!("Array actual despues de push 2: {:?}", *array);

           // print_info(&args[2]);


            //let mut array = GLOBAL_ARRAY.lock().unwrap();
          //  array.pop();
          //  println!("Array actual despues de pop: {:?}", *array);


         

          // test_removal_ops(); // esto si funciona



         //  wait_and_remove_at(0);
           println!("el resultado de esperar la funcion: {:?}",wait_and_remove_at(0));
          //  println!("Array actual despues de remove_at: {:?}", *array);

        }

        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(1);
        }



    }



    }



pub fn test_removal_ops() {
    let mut array = GLOBAL_ARRAY.lock().unwrap();

    array.clear();
    array.extend([10, 20, 30, 40, 50]);
    println!("Inicial: {:?}", *array);

    // remove_first
    if !array.is_empty() {
        let val = array.remove(0);
        println!("remove_first: {}", val);
    } else {
        println!("remove_first: array vacío");
    }
    println!("Después de remove_first: {:?}", *array);

    // remove_at(1)
    if array.len() > 1 {
        let val = array.remove(1);
        println!("remove_at(1): {}", val);
    } else {
        println!("remove_at(1): índice fuera de rango");
    }
    println!("Después de remove_at(1): {:?}", *array);

    // remove_range(0, 2)
    let end = (0 + 2).min(array.len());
    array.drain(0..end);
    println!("Después de remove_range(0, 2): {:?}", *array);
}





pub fn wait_and_remove_at(index: usize) -> Option<u8> {
    let mut count = 0;
    loop {
        match GLOBAL_ARRAY.try_lock() {
            Ok(mut array) => {
                if index < array.len() {
                    return Some(array.remove(index));
                
                } else {
                    println!("Índice fuera de rango: {}", index);
                    return None;
                }
            }
            Err(TryLockError::WouldBlock) => {
                // Mutex ocupado, esperamos un poco
                thread::sleep(Duration::from_millis(700));
                count += 1;
                if count > 10 {
                    println!("Tiempo de espera agotado");
                    return None;
                }
                println!("Mutex ocupado, esperamos un poco");
            }
            Err(e) => {
                println!("Error al intentar lockear: {:?}", e);
                return None;
            }
        }
    }


}

