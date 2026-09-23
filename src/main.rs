mod diread;

use std::env;
use diread::{scan_directory, FileSystemEntryType};

fn main() {
    let current_dir = env::current_dir().expect("Не удалось получить рабочую директорию");
    
    println!("Сканирование директории: {:?}", current_dir);

    match scan_directory(&current_dir) {
        Ok(entries) => {
            for entry in entries {
                match entry.entry_type {
                    FileSystemEntryType::Directory => {
                        println!("[DIR]  {:?}", entry.path);
                    }
                    FileSystemEntryType::Archive(fmt) => {
                        println!("[ARCH] {:?} (Формат: {:?})", entry.path, fmt);
                    }
                    FileSystemEntryType::RegularFile => {
                        println!("[FILE] {:?}", entry.path);
                    }
                }
            }
        }
        Err(e) => eprintln!("Ошибка сканирования: {}", e),
    }
}
