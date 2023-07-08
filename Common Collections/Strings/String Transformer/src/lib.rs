pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

pub mod transformer {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output = vec![];
        for (string, cmd) in input {
            match cmd {
                Command::Uppercase => output.push(string.to_uppercase()),
                Command::Trim => output.push(string.trim().to_owned()),
                Command::Append(times) => output.push(
                    format!("{}{}", string, "bar".repeat(times))
                )
            }
        }
        output
    }
}
