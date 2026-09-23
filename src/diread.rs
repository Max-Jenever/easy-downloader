use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// Представляет тип обнаруженной сущности в файловой системе.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileSystemEntryType {
    Directory,
    Archive(ArchiveFormat),
    RegularFile,
}

/// Поддерживаемые форматы архивов.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchiveFormat {
    Zip,
    Tar,
    Gzip,
    SevenZ,
    Rar,
}

/// Структура, описывающая единичный элемент файловой системы.
#[derive(Debug, Clone)]
pub struct FileSystemEntry {
    pub path: PathBuf,
    pub entry_type: FileSystemEntryType,
}

/// Ошибки, возникающие при сканировании директории.
#[derive(Debug)]
pub enum ScanError {
    Io(io::Error),
    NotADirectory(PathBuf),
}

impl From<io::Error> for ScanError {
    fn from(err: io::Error) -> Self {
        ScanError::Io(err)
    }
}

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanError::Io(e) => write!(f, "Ошибка ввода-вывода: {}", e),
            ScanError::NotADirectory(p) => write!(f, "Указанный путь не является директорией: {:?}", p),
        }
    }
}

impl std::error::Error for ScanError {}

/// Определяет формат архива путем чтения сигнатур (magic bytes) файла.
fn detect_archive_by_magic_bytes(path: &Path) -> Option<ArchiveFormat> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return None,
    };

    // Буфер на 262 байта необходим для проверки TAR, 
    // сигнатура которого находится по смещению 257 байт.
    let mut buffer = [0u8; 262];
    let bytes_read = file.read(&mut buffer).unwrap_or(0);

    if bytes_read >= 4 && buffer[0..4] == [0x50, 0x4B, 0x03, 0x04] {
        return Some(ArchiveFormat::Zip);
    }
    if bytes_read >= 2 && buffer[0..2] == [0x1F, 0x8B] {
        return Some(ArchiveFormat::Gzip);
    }
    if bytes_read >= 6 && buffer[0..6] == [0x52, 0x61, 0x72, 0x21, 0x1A, 0x07] {
        return Some(ArchiveFormat::Rar);
    }
    if bytes_read >= 6 && buffer[0..6] == [0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C] {
        return Some(ArchiveFormat::SevenZ);
    }
    if bytes_read >= 262 && &buffer[257..262] == b"ustar" {
        return Some(ArchiveFormat::Tar);
    }

    None
}

/// Сканирует указанную директорию и возвращает список элементов с их типами.
pub fn scan_directory(path: &Path) -> Result<Vec<FileSystemEntry>, ScanError> {
    if !path.is_dir() {
        return Err(ScanError::NotADirectory(path.to_path_buf()));
    }

    let mut entries = Vec::new();

    for entry_result in fs::read_dir(path)? {
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        let entry_path = entry.path();

        let entry_type = if file_type.is_dir() {
            FileSystemEntryType::Directory
        } else if file_type.is_file() {
            // Проверка формата файла по его содержимому
            if let Some(archive_fmt) = detect_archive_by_magic_bytes(&entry_path) {
                FileSystemEntryType::Archive(archive_fmt)
            } else {
                FileSystemEntryType::RegularFile
            }
        } else {
            // Символические ссылки, сокеты и специфичные для ОС файлы 
            // классифицируются как обычные, если не требуется их отдельная обработка.
            FileSystemEntryType::RegularFile 
        };

        entries.push(FileSystemEntry {
            path: entry_path,
            entry_type,
        });
    }

    Ok(entries)
}
