pub struct Command {
    description_long: String,
    description_short: String,
}

impl Command {
    pub fn new ( description_long: String, description_short: String) -> Command {
        Command {
            description_long,
            description_short,
        }
    }
}

pub struct ArgumentsParser {
    note: String,
}

impl ArgumentsParser{
    pub fn new(note: String) -> ArgumentsParser {
        ArgumentsParser{
            note,
        }
    }     
}
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn create_parser() {
        let parser = ArgumentsParser::new( String::from("test note"));
        println!("note: {}", parser.note);
    }
    #[test]
    fn create_command(){
        let command = Command::new( String::from("long description"),  String::from("short description") );
        assert_eq!(command.description_long, "long description");
        assert_eq!(command.description_short, "short description");
    }
}
