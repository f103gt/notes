use clap::{command, Arg, ArgMatches, Command};
use crate::notes;
use std::error::Error;

enum Commands {
    Write,
    Show,
    Remove
}

impl Commands{
    fn as_str(&self)-> &'static str{
        match self {
            Commands::Write => "write",
            Commands::Show => "show",
            Commands::Remove => "remove",

        }
    }
}

pub fn parse_args() -> ArgMatches {
    command!()
        .subcommand(
            Command::new("notes")
                .subcommand(
                    Command::new(Commands::Write.as_str())
                        .about("Saves provided text in its storage/database")
                        .short_flag('w')
                        .long_flag(Commands::Write.as_str())
                        .arg(
                            Arg::new("text")
                                .index(1)
                                .required(true)
                                .help("Text to save"),
                        ),
                )
                .subcommand(
                    Command::new(Commands::Show.as_str())
                        .long_flag(&Commands::Show.as_str())
                        .about("Show all saved notes on the screen")
                        .short_flag('s'),
                )
                .subcommand(
                    Command::new(Commands::Remove.as_str())
                        .about("Removes note with the given ID")
                        .long_flag(Commands::Remove.as_str())
                        .short_flag('r')
                        .arg(
                            Arg::new("ID")
                                .index(1)
                                .required(true)
                                .help("ID of the note to remove"),
                        ),
                ),
        )
        .get_matches()
}


pub fn handle_matches(matches: ArgMatches) -> Result<(), Box<dyn Error>>{
    match matches.subcommand(){
        Some(("notes",notes_matches))=>{
            match notes_matches.subcommand(){
                Some(("write",write_matches))=>{
                if let Some(text) =
                 write_matches.get_one::<String>("text"){
                println!("Saving text: {}",text);
                notes::Note::write_note(text);
                Ok(())
                 }else{
                    Err("Text argument is missing.".into())
                 }
            },
            Some(("show",_)) =>{
                notes::Note::show_notes();
                Ok(())
            },
            Some(("remove",remove_matches))=>{
                let id = remove_matches.get_one::<String>("ID")
            .ok_or("ID argument is missing.")?;
        notes::Note::remove_note_by_id(id.to_string());
        Ok(())

        },
        _ => Ok(()),
            
        }
    },
    _ => Ok(()),
    }
}