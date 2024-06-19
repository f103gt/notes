use std::{fs::OpenOptions, io::{self,Write}};
use serde::{Deserialize,Serialize};
use std::io::Read;
use chrono::Local;
use std::path::Path;
use uuid::Uuid;
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    static ref FILE_NAME: RwLock<String> = RwLock::new("notes.json".to_string());
}

pub fn set_default_file_name(new_name: &'static str) {
    if let Ok(mut guard) = FILE_NAME.write() {
        *guard = new_name.to_string();
    } else {
        eprintln!("Failed to acquire write lock for default file name.");
    }
}


#[derive(Serialize, Deserialize)]
pub struct Note{
    id: String,
    text: String,
    date: String
}

fn get_current_date() -> String{
    Local::now()
   .format("%Y-%m-%d %H:%M:%S")
   .to_string()
}


impl Note{
    pub fn new(id:String, text:String, date:String)->Self{
        Note{id, text,date}
    }
    pub fn id(&self)->&str{
        &self.id
    }

    pub fn text(&self)-> &str{
        &self.text
    }

    pub fn date(&self)-> &str{
        &self.date
    }

    pub fn save_notes(notes: &Vec<Note>) -> io::Result<()>{
        let data = serde_json::to_string(notes)
            .map_err(|e| io::Error::new(
                io::ErrorKind::InvalidData, e))?;
        let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&*FILE_NAME.read().unwrap())?;
    
        file.write_all(data.as_bytes())?;
    
        Ok(())
    }
    
    pub fn load_notes() -> Result<Vec<Note>,io::Error>{
        let guard = match FILE_NAME.read() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let file_name = guard.as_str();
        let path = Path::new(file_name);
     
        if !path.exists(){
            OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_name)?;
    
        let empty_notes: Vec<Note> = Vec::new();
        Note::save_notes(&empty_notes)?;
        return Ok(empty_notes);
        }
        let mut file = OpenOptions::new()
        .read(true)
        .open(file_name)?;
        
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        if data.is_empty(){
            Ok(Vec::new())
        }else{
            match serde_json::from_str(&data){
                Ok(notes) => Ok(notes),
                Err(e) => 
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,e)),
            }
        }
    }
    
    pub fn write_note(text:&str){
        let mut notes = 
        match Note::load_notes(){
            Ok(n) => n,
            Err(e) =>{
                eprintln!("An error occured during appending the json fil: {}",e);
                std::process::exit(1);
            }
        };
    
        let note = Note{
            id: Uuid::new_v4().to_string(),
            text: text.to_string(),
            date: get_current_date(),
        };
    
        notes.push(note);
    
        if let Err(e) = Note::save_notes(&notes){
            eprintln!("Failed to save notes: {}", e);
        }else{
            println!("Note saved successfully!");
        }
    }
    
    pub fn remove_note_by_id(id: String){
        let mut notes = match Note::load_notes(){
            Ok(n) => n,
            Err(e) =>{
                eprintln!("Failed to load notes: {}", e);
                return;
            }
        };
        notes.retain(|note| note.id != id);
    
        if let Err(e) = Note::save_notes(&notes) {
            println!("Failed to save notes: {}",e);
        }else{
            println!("Note removed successfully!");
        }
    }
    
    pub fn show_notes(){
        let notes = match Note::load_notes(){
            Ok(n)=> n,
            Err(e) =>{
                eprintln!("Failed to load notes: {}",e);
                return;
            }
        };
        notes
        .iter()
        .for_each(|note| println!("ID: {}, Date: {}, Text: {}", 
        note.id,note.date, note.text));
    }
}