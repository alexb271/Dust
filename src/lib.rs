mod branch;
mod builtin;
mod error;
mod expression;
mod function;
mod instruction;
mod operation;
mod parser;
mod session;
mod token;
mod variable;
mod while_loop;

#[cfg(test)]
mod tests;
#[cfg(test)]
const FUNCTION_CALL_LIMIT: usize = 100;

const DYN_KEYWORD: &'static str = "dyn";
const TYPEID_DYN: usize = 0;
const TYPEID_NONE: usize = 1;
const TYPEID_NUMBER: usize = 2;
const TYPEID_STRING: usize = 3;
const TYPEID_BOOL: usize = 4;

#[cfg(not(test))]
const FUNCTION_CALL_LIMIT: usize = 1000;

pub use crate::session::Session;

fn process(input: &str, session: &mut Session) {
    session.parsed_session.append_source_code(input);
    let parse_result = parser::parse(input, &mut session.parsed_session);

    match parse_result {
        Ok(instructions) => {
            for item in instructions {
                match item.exec(&mut session.exec_session, &session.parsed_session) {
                    Ok(_) => (),
                    Err(e) => {
                        let mut error_string = e.print_to_string(
                            session.parsed_session.get_source_code(),
                            session.exec_session.get_backtrace(),
                        );
                        session.exec_session.clear_backtrace();
                        if error_string.ends_with('\n') {
                            error_string.pop();
                        }
                        println!("{error_string}");
                        break;
                    }
                }
            }
        }

        Err(e) => {
            let mut error_string = e.print_to_string(
                session.parsed_session.get_source_code(),
                session.exec_session.get_backtrace(),
            );
            session.exec_session.clear_backtrace();
            if error_string.ends_with('\n') {
                error_string.pop();
            }
            println!("{error_string}");
        }
    }
}

#[cfg(test)]
pub fn process_to_string(input: &str, session: &mut Session) -> String {
    session.parsed_session.append_source_code(input);
    let parse_result = parser::parse(input, &mut session.parsed_session);

    match parse_result {
        Ok(instructions) => {
            for item in instructions {
                match item.exec(&mut session.exec_session, &session.parsed_session) {
                    Ok(_) => (),
                    Err(e) => {
                        let error_string = e.print_to_string(
                            session.parsed_session.get_source_code(),
                            session.exec_session.get_backtrace(),
                        );
                        session.exec_session.clear_backtrace();
                        session
                            .exec_session
                            .output_stream
                            .push_str(error_string.as_str());
                        break;
                    }
                }
            }

            let result = session.exec_session.output_stream.clone();
            session.exec_session.output_stream.clear();
            result
        }

        Err(e) => {
            let error_string = e.print_to_string(
                session.parsed_session.get_source_code(),
                session.exec_session.get_backtrace(),
            );
            session.exec_session.clear_backtrace();
            session
                .exec_session
                .output_stream
                .push_str(error_string.as_str());

            let result = session.exec_session.output_stream.clone();
            session.exec_session.output_stream.clear();
            result
        }
    }
}
