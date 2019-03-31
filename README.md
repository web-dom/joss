# joss
The goal of JOSS (JSON Operating System Schema) is to create a protocol for interacting with a UNIX-like environment via the host interface of web assembly using JSON.

# Why?

Web assembly is going down the route of running not just in the browser but also outside of the browser.  It needs a common way of communicating with the outside world. Traditionally technologies have utilized C style structs for communicating data across boundries. JOSS is a standard that utilizes JSON instead. There are several reason for this approach.


## JSON puts simpler requirements on programming languages

Not every programming language has a strong capability to do binary structs. By using JSON we reduce the requirements of interacting with a JOSS system down to JSON parsing and encoding.


## JSON is human readable

Human readability helps debugging and making introspection into how systems are behaving way easier.


## JSON is already a popular standard

It's one of the most known structured formats out there and has very strong library support already.


## JOSS does not preclude more performant binary interfaces when they are needed

Let's face it, performance of interacting with operating system isn't always the most important aspect of our program. When it is though, JOSS does not limit the ability to use alternative performant interactions. Binary structures could be considered an overeager optimization.


## JOSS can be easily used with remote systems

Most of the web already understands JSON, imagine you are writing an interface to a JOSS system in a web browser. The problem is more easily reduced to communication via JSON over a web server or web socket to a server that represents your system.

## Easier error handling

Because JSON is a more flexable structure than a binary format.  Error handling is easier and more expressive to implement into the spec.

## Web assembly currently has poor support for multiple outputs and even then it might not be enough

Because web assembly lacks an easy way to return multiple outputs, we are forced to resort to a structured format of some type. Even with the use of multiple outputs, representing return objects that are very nested or have arrays may be problematic. JSON has few if none of these complexities (asside from memory allocation) and works with the web assembly spec as it is now.

# Examples

The spec will be mirroring concepts of the POSIX interface with emphasis on human readability and simplicity http://pubs.opengroup.org/onlinepubs/9699919799/

## Getting command line arguments

### Request

```json
{
  "operation":"get_command_line_arguments"
}  
```
### Response
```json
["vim","foo.txt"]
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
12345
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
