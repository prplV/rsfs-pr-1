use clap::Parser;
use anyhow::Result;

/// segments : fs , file , xml , json , zip 
/// actions : 
/// (fs) - info 
/// (file) - create, read, write, delete 
/// ((plain)) - create, read, write delete 
/// ((xml)) - create, read, write delete 
/// ((json)) - create, read, write, delete 
/// (zip) - empty, archive-file, delete
/// 
/// examples : 
/// rsfs --segment fs --action info --name D
/// rsfs --segment file --action create --file-type [ plain | json | xml ] --name filename
/// rsfs --segment zip --action empty --name filename
/// 
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, value_enum)]
    segment : Segment,
    #[arg(long)]
    action : String,
    #[arg(long, value_enum, default_value=None)]
    file_type : Option<FileType>,
    #[arg(long, default_value=None)]
    name : Option<String>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Segment {
    Fs,
    File,
    Zip,
}
#[derive(clap::ValueEnum, Clone, Debug)]
enum FileType {
    Plain,
    Json,
    Xml,
}

trait FileManagment {
    fn create_file(&self) -> Result<()>;
    fn read_file(&self) -> Result<()>;
    fn delete_file(&self) -> Result<()>;
}