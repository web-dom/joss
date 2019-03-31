use cstring::*;

extern "C" {
    fn joss_syscall(request:CString)->CString;
}

pub fn syscall(request:String) -> String{
    let response = unsafe {
        joss_syscall(cstr(&request.to_string()))
    };
    cstr_to_string(response)
}
