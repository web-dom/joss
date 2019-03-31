use cstring::*;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate malloc;
extern crate joss;
use serde_json::{json,from_str};

#[derive(Serialize, Deserialize)]
struct CommandLineArguments {
    arguments: Vec<String>,
}

extern "C" {
    fn console_log(msg:CString);
    fn joss_syscall(request:CString)->CString;
}

#[no_mangle]
fn main() {
    // The type of `john` is `serde_json::Value`
    let request_json = json!({
        "operation": "get_command_line_arguments"
    });
    let response = joss::syscall(request_json.to_string())
    let response_json:CommandLineArguments = from_str(&cstr_to_string(response)).unwrap();
    unsafe {
        console_log(cstr(&response_json.arguments.clone().join(" ")));
    }
}
