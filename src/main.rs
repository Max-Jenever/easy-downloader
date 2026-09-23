mod diread;
mod zipper;

use std::env;
use diread::{scan_directory, FileSystemEntryType};
use zipper::archive_directory;

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

    let source = "~/my_projects/easy-downloader/src/test.c";
    let destination = "~/my_projects/easy-downloader/src/test.tar.gz";

    match archive_directory(source, destination) {
        Ok(_) => println!("Директория успешно архивирована в {}", destination),
        Err(e) => eprintln!("Ошибка при архивации: {}", e),
    }
}

