#![feature(libc)]
extern crate libc;

mod ffi {

    use libc::{c_char, c_int};

    #[derive(Debug)]
    #[repr(C)]
    pub struct PgQueryError {
        pub message: *const c_char, // exception message
        pub funcname: *const c_char, // source function of exception (e.g. SearchSysCache)
        pub filename: *const c_char, // source of exception (e.g. parse.l)
        pub lineno: c_int, // source of exception (e.g. 104)
        pub cursorpos: c_int, // char in query at which exception occurred
        pub context: *const c_char, // additional context (optional, can be NULL)
    }

    #[derive(Debug)]
    #[repr(C)]
    pub struct PgQueryParseResult {
      pub parse_tree: *const c_char,
      pub stderr_buffer: *const c_char,
      pub error: *mut PgQueryError
    }

    #[link(name = "pg_query")]
    extern "C" {

        pub fn pg_query_parse(input: *const c_char) -> PgQueryParseResult;

        pub fn pg_query_free_parse_result(result: PgQueryParseResult);
    }
}

#[derive(Debug)]
pub struct PgQueryError {
    pub message: String,
    pub funcname: String,
    pub filename: String,
    pub lineno: i32,
    pub cursorpos: i32,
    pub context: Option<String>,
}

#[derive(Debug)]
pub struct PgQueryParseResult {
      pub parse_tree: String,
      pub stderr_buffer: Option<String>,
      pub error: Option<PgQueryError>
}

pub fn pg_query_parse(input: &str) -> PgQueryParseResult {

    use std::ffi::{CString, CStr};
    use std::str;

    let c_input = CString::new(input).unwrap();

    unsafe {
        let result = ffi::pg_query_parse(c_input.as_ptr());

        let query_error = if !result.error.is_null() {

            let ref error = *(result.error);

            let message = {
                let bytes = CStr::from_ptr(error.message).to_bytes();
                str::from_utf8(bytes).unwrap().to_string()
            };

            let funcname = {
                let bytes = CStr::from_ptr(error.funcname).to_bytes();
                str::from_utf8(bytes).unwrap().to_string()
            };

            let filename = {
                let bytes = CStr::from_ptr(error.filename).to_bytes();
                str::from_utf8(bytes).unwrap().to_string()
            };

            let context = if !error.context.is_null() {
                let bytes = CStr::from_ptr(error.context).to_bytes();
                Some(str::from_utf8(bytes).unwrap().to_string())
            } else {
                None
            };

            let query_error = PgQueryError {
                message: message,
                funcname: funcname,
                filename: filename,
                lineno: error.lineno,
                cursorpos: error.cursorpos,
                context: context
            };

            Some(query_error)

        } else {
            None
        };

        let parse_tree = {
            let parse_tree_bytes = CStr::from_ptr(result.parse_tree).to_bytes();
            str::from_utf8(parse_tree_bytes).unwrap().to_string()
        };

        let stderr_buffer = if !result.stderr_buffer.is_null() {
            let stderr_buffer_bytes = CStr::from_ptr(result.stderr_buffer).to_bytes();
            Some(str::from_utf8(stderr_buffer_bytes).unwrap().to_string())
        } else {
            None
        };

        ffi::pg_query_free_parse_result(result);

        PgQueryParseResult {
            parse_tree: parse_tree,
            stderr_buffer: stderr_buffer,
            error: query_error
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{pg_query_parse};

    #[test]
    fn it_works() {

        let result = pg_query_parse("SELECT 1");

        println!("{:?}", result);
        assert!(result.error.is_none());

    }

    #[test]
    fn it_does_not_work() {

        let result = pg_query_parse("INSERT FROM DOES NOT WORK");

        println!("{:?}", result);
        assert!(result.error.is_some());

    }

    // TODO: more tests
}
