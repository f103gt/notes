mod notes;
#[cfg(test)]
mod tests{
    use std::fs::{self, File};
    use crate::notes::{set_default_file_name, Note};
    use std::path::Path;
    use serde_json;
    use std::io::{Write,Read};

    const TEST_FILE_NAME: &str = "test_notes.json";


    fn remove_test_file(){
        if Path::new(TEST_FILE_NAME).exists(){
            fs::remove_file(TEST_FILE_NAME)
            .expect("Feiled to remove test file");
        }
    }

    fn create_test_file_with_notes(notes_data: &Vec<Note>){
        
        let serialized_notes = serde_json::to_string(&notes_data).expect("Failed to serialize notes");
        let mut file = File::create(TEST_FILE_NAME).expect("Failed to create test file");
        file.write_all(serialized_notes.as_bytes()).expect("Failed to write to test file");
        

        let mut file_content = String::new();
        File::open(TEST_FILE_NAME)
            .expect("Failed to open test file")
            .read_to_string(&mut file_content)
            .expect("Failed to read test file");
        println!("File content after writing: {}", file_content);
        assert!(!file_content.is_empty(),"The content must not be empty");
    }

    //#[setup]
    #[test]
    fn test_load_notes_no_file(){
        set_default_file_name(&TEST_FILE_NAME);
       
        remove_test_file();
            
        let notes = Note::load_notes().expect("Feiled to laod notes");


        assert!(notes.is_empty(),
        "Notes should be empty when the file does not exist");
        remove_test_file();
    }
    #[test]
    fn test_load_notes_empty_file() {
        remove_test_file();
        set_default_file_name(&TEST_FILE_NAME);

        let mut file = File::create(TEST_FILE_NAME).expect("Failed to create test file");
        file.write_all(b"").expect("Failed to write to test file");


        let notes = Note::load_notes().expect("Failed to load notes from empty file");

        assert!(notes.is_empty(), "Notes should be empty when the file is empty");

        remove_test_file();
    }

    #[test]
    fn test_load_notes_nonempty_file() {
        remove_test_file();

        set_default_file_name(TEST_FILE_NAME);

        let notes_data = vec![
            Note::new("1".to_string(), "First note".to_string(), "2024-06-19 12:00:00".to_string()),
            Note::new("2".to_string(), "Second note".to_string(), "2024-06-19 13:00:00".to_string()),
        ];

        create_test_file_with_notes(&notes_data);


        let loaded_notes = 
        Note::load_notes().expect("Failed to load notes from nonempty file");

        assert!(!loaded_notes.is_empty(), "Notes should not be empty when the file contains data");
        assert_eq!(loaded_notes.len(), 2, "There should be 2 notes in the file");

        assert_eq!(loaded_notes[0].id(), "1");
        assert_eq!(loaded_notes[0].text(), "First note");
        assert_eq!(loaded_notes[0].date(), "2024-06-19 12:00:00");

        assert_eq!(loaded_notes[1].id(), "2");
        assert_eq!(loaded_notes[1].text(), "Second note");
        assert_eq!(loaded_notes[1].date(), "2024-06-19 13:00:00");


        remove_test_file();
    }


    #[test]
    fn test_write_multiple_notes() {
        remove_test_file();

        set_default_file_name(TEST_FILE_NAME);

        Note::write_note("First note");
        Note::write_note("Second note");


        let mut file_content = String::new();
        File::open(TEST_FILE_NAME)
            .expect("Failed to open test file")
            .read_to_string(&mut file_content)
            .expect("Failed to read test file");



        let loaded_notes: Vec<Note> = serde_json::from_str(&file_content)
            .expect("Failed to deserialize notes from test file");

        assert_eq!(loaded_notes.len(), 2, "There should be 2 notes in the file");
        assert_eq!(loaded_notes[0].text(), "First note", "First note text should match");
        assert_eq!(loaded_notes[1].text(), "Second note", "Second note text should match");



        remove_test_file();
    }


    #[test]
    fn test_remove_note_by_id_existing_id() {
        remove_test_file();

        set_default_file_name(TEST_FILE_NAME);

        let notes_data = vec![
            Note::new("1".to_string(), "First note".to_string(), "2024-06-19 12:00:00".to_string()),
            Note::new("2".to_string(), "Second note".to_string(), "2024-06-19 13:00:00".to_string()),
        ];
        create_test_file_with_notes(&notes_data);

        Note::remove_note_by_id("1".to_string());


        let loaded_notes = Note::load_notes().expect("Failed to load notes after removal");

        assert_eq!(loaded_notes.len(), 1, "There should be 1 note remaining after removal");
        assert_eq!(loaded_notes[0].id(), "2", "Remaining note should have ID '2'");

        remove_test_file();
    }

    #[test]
    fn test_remove_note_by_id_non_existing_id() {
        remove_test_file();

        set_default_file_name(TEST_FILE_NAME);

        let notes_data = vec![
            Note::new("1".to_string(), "First note".to_string(), "2024-06-19 12:00:00".to_string()),
            Note::new("2".to_string(), "Second note".to_string(), "2024-06-19 13:00:00".to_string()),
        ];
        create_test_file_with_notes(&notes_data);

        Note::remove_note_by_id("3".to_string());

        let loaded_notes = Note::load_notes().expect("Failed to load notes after removal");

        assert_eq!(loaded_notes.len(), 2, "All notes should remain unchanged");

        remove_test_file();
    }


    #[test]
    fn test_save_notes() {
        remove_test_file();

        set_default_file_name(&TEST_FILE_NAME);

        let notes_data = vec![
            Note::new("1".to_string(), "First note".to_string(), "2024-06-19 12:00:00".to_string()),
            Note::new("2".to_string(), "Second note".to_string(), "2024-06-19 13:00:00".to_string()),
        ];

        create_test_file_with_notes(&notes_data);


        let result = Note::save_notes(&notes_data);

        assert!(result.is_ok(), "Saving notes should succeed");


        let mut file_content = String::new();
        let mut file = File::open(&TEST_FILE_NAME).expect("Failed to open test file");
        file.read_to_string(&mut file_content).expect("Failed to read test file");


        let loaded_notes: Vec<Note> = serde_json::from_str(&file_content)
            .expect("Failed to deserialize notes from test file");

        assert_eq!(loaded_notes.len(), 2, "There should be 2 notes in the loaded file");
        assert_eq!(loaded_notes[0].id(), "1");
        assert_eq!(loaded_notes[0].text(), "First note");
        assert_eq!(loaded_notes[0].date(), "2024-06-19 12:00:00");
        assert_eq!(loaded_notes[1].id(), "2");
        assert_eq!(loaded_notes[1].text(), "Second note");
        assert_eq!(loaded_notes[1].date(), "2024-06-19 13:00:00");

        remove_test_file();
    }

}
