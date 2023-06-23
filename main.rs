use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Write};
//&str -> use it when we nedd a different view of an existing string e.g. slice
//String -> normal string
struct Note {
    name: String,
    text: String,
    compleeted: bool,
}

impl Note {
    fn change_name(&mut self, name: &String) {
        self.name = name.to_string();
    }

    fn change_description(&mut self, description: String) {
        self.text = description.to_string();
    }

    fn change_compleeted(&mut self, compleeted: bool) {
        self.compleeted = compleeted;
    }
}
struct NoteBook {
    notes: Vec<Note>,
}
impl NoteBook {
    fn add_note(&mut self, note: Note) {
        self.notes.push(note);
    }

    fn delete_book(&mut self, name: String) {
        for i in 0..self.notes.len() {
            if self.notes[i].name == name {
                self.notes.remove(i);
            }
        }
    }

    fn edit_note(
        &mut self,
        name_to_search: String,
        thing_to_change: String,
        text_to_be_placed: String,
    ) {
        for i in 0..self.notes.len() {
            if self.notes[i].name == name_to_search {
                self.notes[i].change_name(&text_to_be_placed); // how to make a preprocessor
            }
        }
    }
}

//Notes class
fn main() {
    let file_path = "path/to/your/file.txt";

    // Create an OpenOptions object and configure it for read and write mode
    let mut open_options = OpenOptions::new();
    open_options.read(true);
    open_options.write(true);

    // Open the file with the specified options
    let mut file = open_options.open(file_path).expect("Failed to open file");

    // Read from the file
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("Failed to read file");
    println!("File contents: {}", buffer);

    // Write to the file

    let mut notes = NoteBook { notes: Vec::new() };

    loop {
        let mut user_input = String::new();
        println!("view notes -> 1, add note -> 2, del note -> 3, edit note -> 4");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input.");

        let number: i32 = match user_input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input.");
                return;
            }
        };

        match number {
            1 => {
                for note in &notes.notes {
                    // like that cuz it doesnt take ownership
                    println!(
                        "name : {},text : {}, compleeted: {}",
                        note.name, note.text, note.compleeted
                    );
                }
            }
            2 => {
                let mut name = String::new();
                println!("name:");
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read input.");
                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read input.");
                println!("description");
                notes.add_note(Note {
                    name: name,
                    text: description,
                    compleeted: false,
                });
            }
            3 => {
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("failed name");
                notes.delete_book(name)
            }
            4 => {
                let mut name = String::new();
                let mut thing_to_change = String::new();
                let mut text = String::new();
                println!("name");
                io::stdin().read_line(&mut name).expect("failed name");
                println!("thing to change");
                io::stdin().read_line(&mut thing_to_change);
                println!("new text");
                io::stdin().read_line(&mut text);
                notes.edit_note(name, thing_to_change, text)
            }
            _ => println!("Invalid number."),
        }
    }
    let content_to_write = "Hello, World!";
    file.write_all(content_to_write.as_bytes())
        .expect("Failed to write to file");
    println!("Successfully wrote to file");
}
