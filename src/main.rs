enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace { from: String, to: String },
}

impl Command {
    fn serialize(&self) -> String {
        let json_string = match self {
            Command::Undo => String::from("{\"cmd\":\"undo\"}"),
            Command::Redo => String::from("{\"cmd\":\"redo\"}"),
            Command::AddText(s) => {
                format!("{{\"cmd\":\"add_text\",\"text\":\"{}\"}}", s)
            }
            Command::MoveCursor(line, column) => {
                format!(
                    "{{ \
                        \"cmd\": \"move_cursor\", \
                        \"line\": \"{line}\", \
                        \"column\": \"{column}\", \
                    }}"
                )
            }
            Command::Replace { from, to } => {
                format!(
                    "{{ \
                    \"cmd\": \"replace\", \
                    \"from\": \"{from}\", \
                    \"to\": \"{to}\", \
                }}"
                )
            }
        };
        json_string
    }
}

fn main() {
    let cmd1 = Command::Undo;
    let cmd2 = Command::AddText(String::from("test"));
    let cmd3 = Command::MoveCursor(10, 20);
    let cmd4 = Command::Replace {
        from: String::from("foo"),
        to: String::from("bar"),
    };

    println!("{}", cmd1.serialize());
    println!("{}", cmd2.serialize());
    println!("{}", cmd3.serialize());
    println!("{}", cmd4.serialize());

    // let age = 35;

    // match age {
    //     1 => println!("Happy first birthday!"),
    //     13..=19 => println!("You're a teenager!"),
    //     x => println!("You are {x} years old!"),
    // }
}
