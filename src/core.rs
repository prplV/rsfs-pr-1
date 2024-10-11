use std::fs;
use std::{fs::File, io::Read, io::Write};
use zip::write::FileOptions;
use std::path::Path; 
use sysinfo::{DiskExt, System, SystemExt};
use anyhow::{Ok, Result};

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Segment {
    Fs,
    File,
    Zip,
}
#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum FileTp {
    Plain,
    Json,
    Xml,
}

pub trait Filemng {
    fn create_file(&self, file_type : FileTp, file_name : &str) -> Result<()>;
    fn read_file(&self, file_type: FileTp, file_name : &str) -> Result<()>;
    fn write_file(&self, file_type: FileTp, file_name : &str, text : &str) -> Result<()>;
    fn delete_file(&self, file_type: FileTp, file_name : &str) -> Result<()>;
}

pub trait Diskmng {
    fn show_info(&self, disk_name : &str) -> Result<()>;
}

pub trait Zipmng {
    fn zip_empty(&self, zip_name : &str) -> Result<()>;
    fn zip_file(&self, zip_name : &str, file_name : &str) -> Result<()>;
}

impl Filemng for Segment {
    fn create_file(&self, file_type : FileTp, file_name : &str) -> Result<()> {
        match file_type {
            FileTp::Plain => {
                std::fs::File::create(Path::new(&file_name)).expect("cannot create plain file");
            },
            FileTp::Xml => {
                let temp = if file_name.contains(".xml") {file_name} else {&(file_name.to_owned() + ".xml")};
                std::fs::File::create(Path::new(&temp)).expect("cannot create xml file");
            },
            FileTp::Json => {
                let temp = if file_name.contains(".json") {file_name} else {&(file_name.to_owned() + ".json")};
                std::fs::File::create(Path::new(&temp)).expect("cannot create json file");
            },
        }
        Ok(())
    }
    fn write_file(&self, file_type: FileTp, file_name : &str, text : &str) -> Result<()> {
        let file_name: &str = match file_type {
            FileTp::Json => &format!("{}.json", file_name),
            FileTp::Xml => &format!("{}.xml", file_name),
            _ => file_name,
        };
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_name)
            .expect("cannot open file");
        file.write_all(text.as_bytes())?;
        Ok(())
    }
    fn read_file(&self, file_type: FileTp, file_name : &str) -> Result<()> {
        let file_name: &str = match file_type {
            FileTp::Json => &format!("{}.json", file_name),
            FileTp::Xml => &format!("{}.xml", file_name),
            _ => file_name,
        };
        let mut buffer = File::open(Path::new(file_name)).expect("cannot open file");
        let mut buf = String::new();
        buffer.read_to_string(&mut buf).expect("cannot read file");
        println!("{}:\n{}", file_name, buf);
        Ok(())
    }
    fn delete_file(&self, file_type: FileTp, file_name : &str) -> Result<()> {
        let file_name: &str = match file_type {
            FileTp::Json => &format!("{}.json", file_name),
            FileTp::Xml => &format!("{}.xml", file_name),
            _ => file_name,
        };
        fs::remove_file(Path::new(file_name)).expect("cannot delete file");
        Ok(())
    }
}

impl Diskmng for Segment {
    fn show_info(&self, disk_name : &str) -> Result<()> {
        let mut system = System::new_all();

        system.refresh_memory();
        let mut counter = 0;
        for disk in system.disks() { 
            if disk.mount_point().to_string_lossy().contains(&disk_name.to_uppercase()) {
                println!("Name: {:?}", disk.name());
                println!("File System: {}", String::from_utf8_lossy(disk.file_system()));
                println!("Size: {}/{} Gbyte", (disk.available_space() as f32 / 1024.0 / 1024.0 / 1024.0), (disk.total_space() as f32 / 1024.0 / 1024.0 / 1024.0));
                println!("Tag: {:?}\n", disk.mount_point());
                counter += 1;
                break;
            }
        }
        if counter == 0 {
            eprintln!("No such Disk !");
        }
        Ok(())
    }
}

impl Zipmng for Segment {
    fn zip_empty(&self, zip_name : &str) -> Result<()> {
        let temp = if zip_name.contains(".zip") {zip_name} else {&(zip_name.to_owned() + ".zip")};
        let path = Path::new(&temp);
        let file = File::create(&path)?;

        let mut zip = zip::ZipWriter::new(file);

        zip.finish()?;
        
        Ok(())
    }
    fn zip_file(&self, zip_name : &str, file_name : &str) -> Result<()> {
        let temp = if zip_name.contains(".zip") {zip_name} else {&(zip_name.to_owned() + ".zip")};
        let zip_file = File::options().read(true).write(true).open(&temp).expect("cannot open zip-file");

        let mut zip = zip::ZipWriter::new_append(zip_file)?;
        
        let options: FileOptions = FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);

        let mut f = File::open(file_name)?;

        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;

        zip.start_file(file_name, options)?;
        zip.write_all(&buffer)?;
        zip.finish()?;

        Ok(())
    }
}