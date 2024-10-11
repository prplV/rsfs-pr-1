use clap::Parser;
use crate::core::{Diskmng, Filemng, Segment, Zipmng, FileTp};
use anyhow::{Result, Ok};

/// segments : fs , file , xml , json , zip 
/// actions : 
/// (fs) - info 
/// (file) - create, read, write, delete 
/// ((plain)) - create, read, write delete 
/// ((xml)) - create, read, write delete 
/// ((json)) - create, read, write, delete 
/// (zip) - empty, archive, delete
/// 
/// examples : 
/// rsfs --segment fs --action info --name D
/// rsfs --segment file --action create --file-type [ plain | json | xml ] --name filename
/// rsfs --segment zip --action empty --zipname filename
/// rsfs --segment zip --action archive --zipname zipname --name filename
/// 
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, value_enum)]
    segment : Segment,
    #[arg(long)]
    action : String,
    #[arg(long, value_enum, default_value=None)]
    file_type : Option<FileTp>,
    #[arg(long, default_value=None)]
    zipname : Option<String>,
    #[arg(long, default_value=None)]
    name : Option<String>,
    #[arg(long, default_value=None)]
    text : Option<String>,
}


impl Args {
    pub fn exec(&mut self) -> Result<()> {
        match self.segment {
            Segment::Fs => {
                if self.action != "" && self.name != None
                    && self.file_type.is_none() && self.zipname.is_none()
                    && !self.name.is_none() {
                        let _ = self.disk_info();
                } else {
                    eprintln!("Bad args!\nExample: rsfs --segment fs --action info --name <NAME>");
                }
            },
            Segment::File => {
                if self.file_type.is_some() && self.name.is_some()
                    && self.zipname.is_none() {
                        match self.action.as_str() {
                            "create" => {
                                let _ = self.create();
                            },
                            "read" => {
                                let _ = self.read();
                            },
                            "write" => {
                                let _ = self.write();
                            },
                            "delete" => {
                                let _ = self.delete();
                            },
                            _ => {
                                eprintln!("Bad args!\nExample: rsfs --segment file --action [ create | read | write | delete] --file-type [ plain | json | xml ] --name filename");
                                
                            }
                        }
                    }
                else {
                    eprintln!("Bad args!\nExample: rsfs --segment file --action [ create | read | write | delete] --file-type [ plain | json | xml ] --name filename");
                }
                
            },
            Segment::Zip => {
                match self.action.as_str() {
                    "empty" => {
                        if self.zipname.is_some() && self.file_type.is_none()
                            && self.name.is_none() && self.text.is_none() {
                                let _ = self.empty();
                            } else {
                                eprintln!("Bad args!\nExample: rsfs --segment zip --action empty --zipname filename");
                            }
                        }
                    "archive" => {
                        if self.zipname.is_some() && self.file_type.is_none()
                            && self.name.is_some() && self.text.is_none() {
                                let _ = self.zip_add();
                        } else {
                            eprintln!("Bad args!\nExample: rsfs --segment zip --action archive --zipname zipname --name filename");
                        }
                    },
                    _ => {
                        eprintln!("Bad action: Use <empty|archive>");
                    }
                }
            },
        }
        Ok(())
    }
    // file 
    fn disk_info(&mut self) -> Result<()> {
        let _ = self.segment.show_info(self.name.as_ref().unwrap());
        Ok(())
    }
    // fs 
    fn create(&mut self) -> Result<()> {
        let _ = self.segment.create_file(
            self.file_type.as_ref().unwrap().clone(),
            self.name.as_ref().unwrap()
            );
        Ok(()) 
    }
    fn read(&mut self) -> Result<()> {
        let _ = self.segment.read_file(
            self.file_type.as_ref().unwrap().clone(),
            self.name.as_ref().unwrap());
        Ok(())
    }
    fn write(&mut self) -> Result<()> {
        let _ = self.segment.write_file(
            self.file_type.as_ref().unwrap().clone(),
            &self.name.as_ref().unwrap(),
             &self.text.as_ref().unwrap()
            );
        Ok(())
    }
    fn delete(&self) -> Result<()> {
        let _ = self.segment.delete_file(
            self.file_type.as_ref().unwrap().clone(),
            self.name.as_ref().unwrap()
        );
        Ok(())
    }
    // zip 
    fn empty(&self) -> Result<()> {
        let _= self.segment.zip_empty(self.zipname.as_ref().unwrap());
        Ok(())
    }
    fn zip_add(&self) -> Result<()> {
        let _ = self.segment.zip_file(self.zipname.as_ref().unwrap(), self.name.as_ref().unwrap());
        Ok(())
    }
}