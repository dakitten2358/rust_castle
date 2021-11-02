use itertools::Itertools;

pub enum TextCommand {
    None,
    Some { command: String, arg: Option<String> },
}

pub fn parse_input(text_command: &String) -> TextCommand {
    let mut tokens = text_command.split_whitespace();
    if let Some(first_token) = tokens.next() {
        let args_string = tokens.format(" ").to_string();
        let args = args_string.as_str();
        if args.is_empty() {
            return TextCommand::Some {
                command: first_token.to_owned(),
                arg: None,
            };
        }

        return TextCommand::Some {
            command: first_token.to_owned(),
            arg: Some(args.to_owned()),
        };
    }

    return TextCommand::None;
}
