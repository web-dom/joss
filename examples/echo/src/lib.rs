#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate malloc;
use joss;
use cstring::*;
use serde_json::{json,from_str};

#[derive(Serialize, Deserialize)]
struct CommandLineArguments {
    arguments: Vec<String>,
}

extern "C" {
    fn console_log(msg:CString);
}

#[no_mangle]
fn main() {
    let request_json = json!({
        "operation": "get_command_line_arguments"
    });
    let response = joss::syscall(request_json.to_string());
    let response_json:CommandLineArguments = from_str(&response).unwrap();
    unsafe {
        console_log(cstr(&response_json.arguments.clone().join(" ")));
    }
}
