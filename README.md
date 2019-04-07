# JOSS
The goal of JOSS (JSON Operating System Schema) is to create a protocol for interacting with a UNIX-like environment via the host interface of web assembly using JSON.

# Why?

Web assembly is going down the route of running not just in the browser but also outside of the browser.  It needs a common way of communicating with the outside world. Traditionally technologies have utilized C style structs for communicating data across boundries. JOSS is a standard that utilizes JSON instead. There are several reason for this approach.


## JSON puts simpler requirements on programming languages

Not every programming language has a strong capability to do binary structs. By using JSON we reduce the requirements of interacting with a JOSS system down to JSON parsing/encoding and C strings.


## JSON is human readable

Human readability helps debugging and making introspection into how systems are behaving way easier.


## JSON is already a popular standard

It's one of the most known structured formats out there and has very strong library support already.


## JOSS does not preclude more performant binary interfaces when they are needed

Let's face it, performance of interacting with operating system isn't always the most important aspect of our program. When it is though, JOSS does not limit the ability to use alternative performant interactions. Binary structures could be percieved as an overeager optimization in many scenarios.

## JOSS can be easily used with remote systems

Most of the web already understands JSON, imagine you are writing an interface to a JOSS system in a web browser that communicates to a server which represents your system. This situation more easily accomplished by communication with JSON over a http or web socket.

## Easier error handling

Because JSON is a less rigid structure than a binary format.  Error handling is easier and more expressive to implement into the spec.

## Web assembly currently has poor support for multiple outputs and even then it might not be enough

Because web assembly lacks an easy way to return multiple outputs, we are forced to resort to a structured format of some type. Even with the use of multiple outputs, representing return objects that are very nested or have arrays may be problematic. JSON has few if none of these complexities (asside from memory allocation) and works with the web assembly spec as it is now.

# System Calls

JOSS takes inspiration from Plan 9 and Redox system calls:

* set_current_directory
* get_process_id
* file_open
* file_close
* file_read
* file_write
* file_get_path
* file_rename
* file_get_metadata
* file_link
* file_unlink
* file_set_offset
* exit

* /dev/clock
* /proc/[id]/args


## Getting command line arguments

### Request

```json
{
  "operation":"get_command_line_arguments"
}  
```
### Response
```json
{
  "arguments":["vim","foo.txt"]
}
```

## Open a file

### Request

```json
{
  "operation":"open_file",
  "path":"/home/helloworld.txt",
  "mode":["read","write"]
}
```
### Response
```json
{
  "file_descriptor":12345
}
```

## Write to a file

### Request

```json
{
  "operation":"write_to_file",
  "file_descriptor":12345,
  "text":"hello world"
}
```

```json
{
  "operation":"write_to_file",
  "file_descriptor":12345,
  "base64":"basbfrasfs="
}
```
### Response
```json
{}
```
```json
{
  "error":"you do not own this file"
}
```

# Hello World

```rust
use joss;

#[no_mangle]
pub fn main() -> () {
    // write to stdout
    joss::syscall(r#"{
        "operation": "write_file",
        "file_descriptor": 1,
        "text":"Hello World!"
    }"#.to_owned());
}
```

# Let's write an application

This application `echo` will simply write to the console log what command line arguments it receives. This app has various small helper libraries:
* `serde` - for getting json in and output
* `malloc` - a small package exposing a function called `malloc` to let data get into your web assembly module
* `joss` - a simple rust wrapper for sending JOSS operations to the host

```rust
#[macro_use]
extern crate serde_derive;
extern crate malloc;
use joss;
use serde_json::{from_str, json};

#[derive(Serialize, Deserialize)]
struct CommandLineArguments {
    arguments: Vec<String>,
}

#[no_mangle]
fn main() {
    // get command line args
    let request_json = json!({
        "operation": "get_command_line_arguments"
    });
    let response = joss::syscall(request_json.to_string());
    let response_json: CommandLineArguments = from_str(&response).unwrap();

    let output = response_json.arguments.clone().join(" ");

    // write to stdout
    let output_json = json!({
        "operation": "write_file",
        "file_decscriptor": 1,
        "text":output
    });
    joss::syscall(output_json.to_string());
}
```
