use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::path::Path;
use tar::Builder;

/// Архивирует директорию в формат .tar.gz
///
/// # Аргументы
/// * `src_dir` - путь к исходной директории.
/// * `dest_file` - путь к создаваемому файлу архива.
pub fn archive_directory<P: AsRef<Path>>(
    src_dir: P,
    dest_file: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let src_path = src_dir.as_ref();
    
    // Извлекаем имя директории для использования в качестве корневой папки внутри архива.
    // Это предотвращает извлечение файлов напрямую в текущую директорию при распаковке.
    let dir_name = src_path
        .file_name()
        .ok_or("Некорректный путь к исходной директории")?
        .to_str()
        .ok_or("Имя директории содержит невалидные символы UTF-8")?;

    // Создаем файл для записи архива
    let tar_gz = File::create(&dest_file)?;

    // Инициализируем энкодер gzip с уровнем сжатия по умолчанию
    let gz_encoder = GzEncoder::new(tar_gz, Compression::default());

    // Создаем сборщик tar-архива, передавая ему поток сжатия
    let mut tar_builder = Builder::new(gz_encoder);

    // Рекурсивно добавляем содержимое директории в архив.
    // Первый аргумент задает префикс пути внутри архива (имя самой директории).
    tar_builder.append_dir_all(dir_name, src_path)?;

    // Завершаем запись в tar и получаем доступ к нижележащему потоку (gz_encoder)
    let gz_encoder = tar_builder.into_inner()?;

    // Завершаем поток gzip, чтобы гарантировать сброс всех буферизованных данных на диск
    gz_encoder.finish()?;

    Ok(())
}
