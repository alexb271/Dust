mod branch;
mod builtin;
mod class;
mod error;
mod expression;
mod for_loop;
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

#[cfg(not(debug_assertions))]
const FUNCTION_CALL_LIMIT: usize = 1000;
#[cfg(debug_assertions)]
const FUNCTION_CALL_LIMIT: usize = 100;

pub use crate::session::Session;

fn process(input: &str, session: &mut Session) {
    let parse_result = parser::parse(&input, &mut session.parse_session);

    match parse_result {
        Ok(instructions) => {
            for item in instructions {
                match item.exec(&mut session.exec_session, &session.parse_session) {
                    Ok(_) => (),
                    Err(e) => {
                        print_error_message(e, session);
                        break;
                    }
                }
            }
        }

        Err(e) => {
            print_error_message(e, session);
        }
    }
}

#[inline]
fn print_error_message(e: crate::error::Error, session: &mut Session) {
    let mut error_string = e.print_to_string(
        session.parse_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    session.exec_session.clear_backtrace();
    if error_string.ends_with('\n') {
        error_string.pop();
    }
    println!("{error_string}");
}

#[cfg(test)]
pub fn process_to_string(input: &str, session: &mut Session) -> String {
    let parse_result = parser::parse(input, &mut session.parse_session);

    match parse_result {
        Ok(instructions) => {
            for item in instructions {
                match item.exec(&mut session.exec_session, &session.parse_session) {
                    Ok(_) => (),
                    Err(e) => {
                        let error_string = e.print_to_string(
                            session.parse_session.get_source_code(),
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
                session.parse_session.get_source_code(),
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
