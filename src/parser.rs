#[derive(Debug)]
pub struct Get {
    pub key: String,
}

#[derive(Debug)]
pub struct Delete {
    pub key: String,
}

#[derive(Debug)]
pub struct Insert {
    pub key: String,
    pub val: String,
}

#[derive(Debug)]
pub enum Command {
    Get(Get),
    Delete(Delete),
    Insert(Insert),
}

#[derive(Debug)]
pub enum ParseError {
    Other,
    MissingSemicolon,
    EmptyParameters,
    IncorrectParameters,
    Unknown,
}

// parse line, returns Command
// if not ending in ; error
// if first word not GET/DELETE/INSERT error
// depending on the command, read the next tokens. tokens need to be either 1 word or multiple words
// in " "

pub fn parse(input: &str) -> Result<Command, ParseError> {
    let input = input
        .trim()
        .strip_suffix(';')
        .ok_or(ParseError::MissingSemicolon)?;

    let (cmd, mut rest) = input.split_once(char::is_whitespace).unwrap_or((input, ""));

    // process the parameters of the command
    rest = rest.trim();
    if rest.is_empty() {
        return Err(ParseError::EmptyParameters);
    }
    let (first_param, second_param): (&str, Option<&str>) = match rest {
        r if r.starts_with('"') => {
            let end = r[1..].find('"').ok_or(ParseError::IncorrectParameters)?;

            let first = &r[1..1 + end];
            let tail = r[1 + end + 1..].trim();

            (first, if tail.is_empty() { None } else { Some(tail) })
        }
        _ => match rest.split_once(char::is_whitespace) {
            Some((first, second)) => {
                let second = second.trim();
                (
                    first,
                    if second.is_empty() {
                        None
                    } else {
                        Some(second)
                    },
                )
            }
            None => (rest, None),
        },
    };

    let prepared_command = match (cmd, first_param, second_param) {
        ("GET", first_param, _) => Command::Get(Get {
            key: first_param.to_string(),
        }),
        ("DELETE", first_param, _) => Command::Delete(Delete {
            key: first_param.to_string(),
        }),
        ("INSERT", _, None) => return Err(ParseError::IncorrectParameters),
        ("INSERT", first_param, Some(second_param)) => Command::Insert(Insert {
            key: first_param.to_string(),
            val: second_param.to_string(),
        }),
        (_, _, _) => return Err(ParseError::Unknown),
    };

    Ok(prepared_command)
}
