use ::libc;
use core::{ffi::c_char, ptr};
pub(crate) use parser_impl::{parse_integer, ParseIntegerError};

use self::lexer::Lexer;
use self::parser_impl::{ByteStack, ParseState};

use crate::{
    buffer::Buffer,
    yajl_alloc::{yajl_alloc_funcs, yajl_set_default_alloc_funcs},
    Status,
};

pub use self::rparser::RParser;

mod lexer;
mod parser_impl;
mod rparser;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Parser {
    pub callbacks: *const yajl_callbacks,
    pub ctx: *mut libc::c_void,
    pub lexer: yajl_lexer,
    pub parseError: Option<ParseError>,
    pub bytesConsumed: usize,
    pub decodeBuf: *mut Buffer,
    pub stateStack: ByteStack,
    pub alloc: yajl_alloc_funcs,
    pub flags: libc::c_uint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ParseError {
    ClientCancelled = 1,
    FloatingPointOverflow,
    IntegerOverflow,
    InvalidArraySeparator,
    InvalidKeyValueSeparator,
    InvalidObjectKey,
    InvalidObjectSeparator,
    InvalidToken,
    PrematureEof,
    TrailingGarbage,
    UnallowedToken,
}

impl ParseError {
    fn as_c_str_ptr(&self) -> *const c_char {
        match self {
            Self::ClientCancelled => {
                b"client cancelled parse via callback return value\0" as *const u8 as *const c_char
            }
            Self::FloatingPointOverflow => {
                b"numeric (floating point) overflow\0" as *const u8 as *const c_char
            }
            Self::IntegerOverflow => b"integer overflow\0" as *const u8 as *const c_char,
            Self::InvalidArraySeparator => {
                b"after array element, I expect ',' or ']'\0" as *const u8 as *const c_char
            }
            Self::InvalidKeyValueSeparator => {
                b"object key and value must be separated by a colon (':')\0" as *const u8
                    as *const c_char
            }
            Self::InvalidObjectKey => {
                b"invalid object key (must be a string)\0" as *const u8 as *const c_char
            }
            Self::InvalidObjectSeparator => {
                b"after key and value, inside map, I expect ',' or '}'\0" as *const u8
                    as *const c_char
            }
            Self::InvalidToken => b"invalid token, internal error\0" as *const u8 as *const c_char,
            Self::PrematureEof => b"premature EOF\0" as *const u8 as *const c_char,
            Self::TrailingGarbage => b"trailing garbage\0" as *const u8 as *const c_char,
            Self::UnallowedToken => {
                b"unallowed token at this point in JSON text\0" as *const u8 as *const c_char
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ParserOption {
    AllowComments = 1,
    DontValidateStrings = 2,
    AllowTrailingGarbage = 4,
    AllowMultipleValues = 8,
    AllowPartialValues = 16,
}

impl ParserOption {
    pub fn from_repr(x: u32) -> Option<ParserOption> {
        match x {
            1 => Some(ParserOption::AllowComments),
            2 => Some(ParserOption::DontValidateStrings),
            4 => Some(ParserOption::AllowTrailingGarbage),
            8 => Some(ParserOption::AllowMultipleValues),
            16 => Some(ParserOption::AllowPartialValues),
            _ => None,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_callbacks {
    pub yajl_null: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_boolean: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> libc::c_int>,
    pub yajl_integer:
        Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_longlong) -> libc::c_int>,
    pub yajl_double: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_double) -> libc::c_int>,
    pub yajl_number:
        Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, usize) -> libc::c_int>,
    pub yajl_string:
        Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_uchar, usize) -> libc::c_int>,
    pub yajl_start_map: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_map_key:
        Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_uchar, usize) -> libc::c_int>,
    pub yajl_end_map: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_start_array: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_end_array: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
}

pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}

pub type yajl_lexer = *mut Lexer;

pub unsafe extern "C" fn yajl_status_to_string(stat: Status) -> *const libc::c_char {
    let mut statStr: *const libc::c_char = b"unknown\0" as *const u8 as *const libc::c_char;
    match stat as libc::c_uint {
        0 => {
            statStr = b"ok, no error\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            statStr = b"client canceled parse\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            statStr = b"parse error\0" as *const u8 as *const libc::c_char;
        }
        _ => {}
    }
    statStr
}

impl Parser {
    /// allocate a parser handle
    ///
    /// # Arguments
    ///
    /// * `callbacks` - a yajl callbacks structure specifying the
    ///                    functions to call when different JSON entities
    ///                    are encountered in the input text.  May be NULL,
    ///                    which is only useful for validation.
    /// * `afs` - memory allocation functions, may be NULL for to use
    ///                    C runtime library routines (malloc and friends)
    /// * `ctx` - a context pointer that will be passed to callbacks.
    ///
    /// # Safety
    ///
    /// The caller is responsible for free the handle by calling `Parser::free`
    pub unsafe fn alloc(
        mut callbacks: *const yajl_callbacks,
        mut afs: *mut yajl_alloc_funcs,
        mut ctx: *mut libc::c_void,
    ) -> *mut Parser {
        let mut hand: *mut Parser = ptr::null_mut();
        let mut afsBuffer: yajl_alloc_funcs = yajl_alloc_funcs {
            malloc: None,
            realloc: None,
            free: None,
            ctx: ptr::null_mut::<libc::c_void>(),
        };
        if !afs.is_null() {
            if ((*afs).malloc).is_none() || ((*afs).realloc).is_none() || ((*afs).free).is_none() {
                return ptr::null_mut();
            }
        } else {
            yajl_set_default_alloc_funcs(&mut afsBuffer);
            afs = &mut afsBuffer;
        }
        hand = ((*afs).malloc).expect("non-null function pointer")(
            (*afs).ctx,
            ::core::mem::size_of::<Parser>(),
        ) as *mut Parser;
        libc::memcpy(
            &mut (*hand).alloc as *mut yajl_alloc_funcs as *mut libc::c_void,
            afs as *mut libc::c_void,
            ::core::mem::size_of::<yajl_alloc_funcs>(),
        );
        (*hand).callbacks = callbacks;
        (*hand).ctx = ctx;
        (*hand).lexer = ptr::null_mut();
        (*hand).bytesConsumed = 0;
        (*hand).decodeBuf = Buffer::alloc(&mut (*hand).alloc);
        (*hand).flags = 0;
        (*hand).stateStack = ByteStack::new(&mut (*hand).alloc);

        (*hand).stateStack.push(ParseState::Start);
        hand
    }
    // pub fn new(
    //     mut callbacks: *const yajl_callbacks,
    //     mut afs: *mut yajl_alloc_funcs,
    //     mut ctx: *mut libc::c_void,
    // ) -> Result<Self, String> {
    //     let mut afsBuffer: yajl_alloc_funcs = yajl_alloc_funcs {
    //         malloc: None,
    //         realloc: None,
    //         free: None,
    //         ctx: ptr::null_mut::<libc::c_void>(),
    //     };
    //     let mut alloc: yajl_alloc_funcs = yajl_alloc_funcs {
    //         malloc: None,
    //         realloc: None,
    //         free: None,
    //         ctx: ptr::null_mut::<libc::c_void>(),
    //     };
    //     unsafe {
    //         if !afs.is_null() {
    //             if ((*afs).malloc).is_none()
    //                 || ((*afs).realloc).is_none()
    //                 || ((*afs).free).is_none()
    //             {
    //                 return Err("Given afs is not valid".into());
    //             }
    //         } else {
    //             yajl_set_default_alloc_funcs(&mut afsBuffer);
    //             afs = &mut afsBuffer;
    //         }
    //         libc::memcpy(
    //             addr_of_mut!(alloc) as *mut yajl_alloc_funcs as *mut libc::c_void,
    //             afs as *mut libc::c_void,
    //             ::core::mem::size_of::<yajl_alloc_funcs>(),
    //         );
    //     }

    //     let mut res = Self {
    //         callbacks,
    //         ctx,
    //         lexer: ptr::null_mut(),
    //         parseError: ptr::null(),
    //         bytesConsumed: 0,
    //         decodeBuf: ptr::null_mut(),
    //         stateStack: ByteStack::new(addr_of_mut!(alloc)),
    //         alloc,
    //         flags: 0,
    //     };
    //     res.decodeBuf = unsafe { yajl_buf_alloc(addr_of_mut!(res.alloc)) };
    //     // This line is needed, must take the address of res.alloc
    //     res.stateStack = ByteStack::new(addr_of_mut!(res.alloc));
    //     Ok(res)
    // }

    pub unsafe fn free(mut handle: *mut Parser) {
        (*handle).stateStack.free();
        Buffer::free((*handle).decodeBuf);
        if !((*handle).lexer).is_null() {
            Lexer::free((*handle).lexer);
            (*handle).lexer = ptr::null_mut();
        }
        ((*handle).alloc.free).expect("non-null function pointer")(
            (*handle).alloc.ctx,
            handle as *mut libc::c_void,
        );
    }
}

// impl Drop for Parser {
//     fn drop(&mut self) {
//         self.stateStack.free();
//         unsafe { yajl_buf_free(self.decodeBuf) };
//         if !(self.lexer).is_null() {
//             unsafe { yajl_lex_free(self.lexer) };
//             self.lexer = ptr::null_mut();
//         }
//     }
// }
impl Parser {
    pub fn config(&mut self, opt: ParserOption, arg: bool) -> bool {
        if arg {
            self.flags |= opt as u32;
        } else {
            self.flags &= !(opt as u32);
        }
        true
    }
    pub unsafe fn parse(
        &mut self,
        mut jsonText: *const libc::c_uchar,
        mut jsonTextLen: usize,
    ) -> Status {
        self.ensure_lexer();
        self.do_parse(jsonText, jsonTextLen)
    }
    fn ensure_lexer(&mut self) {
        if self.lexer.is_null() {
            unsafe {
                self.lexer = Lexer::alloc(
                    &mut self.alloc,
                    self.flags & ParserOption::AllowComments as u32,
                    (self.flags & ParserOption::DontValidateStrings as u32 == 0) as libc::c_int
                        as libc::c_uint,
                );
            }
        }
    }
    pub fn complete_parse(&mut self) -> Status {
        self.ensure_lexer();
        unsafe { self.do_finish() }
    }

    pub unsafe fn get_error(
        &mut self,
        mut verbose: bool,
        mut jsonText: *const libc::c_uchar,
        mut jsonTextLen: usize,
    ) -> *mut libc::c_uchar {
        self.render_error_string(jsonText, jsonTextLen, verbose)
    }

    pub fn get_bytes_consumed(&self) -> usize {
        self.bytesConsumed
    }
    pub unsafe fn free_error(&self, mut str: *mut libc::c_uchar) {
        (self.alloc.free).expect("non-null function pointer")(
            self.alloc.ctx,
            str as *mut libc::c_void,
        );
    }
}
