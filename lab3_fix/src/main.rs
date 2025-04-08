use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn create_text_file(file_name: &str) -> io::Result<()> {
    fs::File::create(file_name)?;
    println!("File '{}' created successfully.", file_name);
    Ok(())
}

fn create_directory(dir_name: &str) -> io::Result<()> {
    fs::create_dir(dir_name)?;
    println!("Directory '{}' created successfully.", dir_name);
    Ok(())
}

fn copy_file(source: &str, destination: &str) -> io::Result<()> {
    fs::copy(source, destination)?;
    println!("File '{}' copied in '{}'.", source, destination);
    Ok(())
}

fn copy_directory(source: &str, destination: &str) -> io::Result<()> {
    if !Path::new(destination).exists() {
        fs::create_dir(destination)?;
    }

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let file_name = entry.file_name();
        let mut destination_path = PathBuf::from(destination);
        destination_path.push(&file_name);

        if source_path.is_file() {
            fs::copy(&source_path, &destination_path)?;
        } else if source_path.is_dir() {
            copy_directory(
                &source_path.to_string_lossy(),
                &destination_path.to_string_lossy(),
            )?;
        }
    }
    println!("Directory '{}' copied in '{}'.", source, destination);
    Ok(())
}

fn search_file(file_name: &str) -> io::Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(name) = path.file_name() {
            if name == Path::new(file_name).as_os_str() {
                println!("File '{}' found: {:?}", file_name, path);
                return Ok(path);
            }
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
}

fn delete_file(file_name: &str) -> io::Result<()> {
    fs::remove_file(file_name)?;
    println!("File '{}' deleted.", file_name);
    Ok(())
}

fn delete_directory(dir_name: &str) -> io::Result<()> {
    fs::remove_dir_all(dir_name)?;
    println!("Directory '{}' deleted.", dir_name);
    Ok(())
}

fn list_directory_contents(dir_name: &str) -> io::Result<Vec<String>> {
    println!("Catalog context '{}':", dir_name);
    let mut contents = Vec::new();
    for entry in fs::read_dir(dir_name)? {
        let entry = entry?;
        contents.push(entry.file_name().into_string().unwrap());
    }
    Ok(contents)
}

fn file_properties(file_name: &str) -> io::Result<()> {
    let metadata = fs::metadata(file_name)?;
    println!("File properties for '{}':", file_name);
    println!("Size: {} bytes", metadata.len());
    println!("Permissions: {:?}", metadata.permissions());
    Ok(())
}

fn directory_properties(dir_name: &str) -> io::Result<()> {
    let metadata = fs::metadata(dir_name)?;
    println!("Directory properties for '{}':", dir_name);
    println!("Permissions: {:?}", metadata.permissions());
    Ok(())
}

fn main() {
    let _ = create_text_file("file.txt");
    let _ = create_directory("new_dir");
    let _ = copy_file("file.txt", "copy_file.txt");
    let _ = create_directory("copy_dir");
    let _ = copy_directory("new_dir", "copy_dir/new_dir_copy");

    let _ = search_file("file.txt");

    let _ = file_properties("file.txt");
    let _ = directory_properties("new_dir");

    let _ = list_directory_contents(".");

    let _ = delete_file("file.txt");
    let _ = delete_file("copy_file.txt");
    let _ = delete_directory("new_dir");
    let _ = delete_directory("copy_dir");
}