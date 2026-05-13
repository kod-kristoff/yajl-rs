#![allow(clippy::missing_safety_doc)]
#![allow(unused_unsafe)]
#![allow(clippy::nonminimal_bool)]

use core::ffi::{c_char, c_double, c_int, c_longlong, c_uchar, c_uint, c_void, CStr};
use core::{fmt, ptr};

use self::context::Context;
use crate::{
    parser::{parse_integer, yajl_callbacks, ParseIntegerError, Parser},
    yajl_alloc::yajl_alloc_funcs,
    ParserOption, Status,
};

mod context;
#[cfg(test)]
mod tests;

/// possible data types that a Value can hold
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum ValueType {
    String = 1,
    Number = 2,
    Object = 3,
    Array = 4,
    True = 5,
    False = 6,
    Null = 7,
    /// The any type isn't valid for Value.type, but can be
    /// used as an argument to routines like `yajl_tree_get`.
    Any = 8,
}

impl ValueType {
    pub fn from_repr(r: u32) -> Option<Self> {
        match r {
            1 => Some(Self::String),
            2 => Some(Self::Number),
            3 => Some(Self::Object),
            4 => Some(Self::Array),
            5 => Some(Self::True),
            6 => Some(Self::False),
            7 => Some(Self::Null),
            8 => Some(Self::Any),
            _ => None,
        }
    }
}

/// Represents a JSON value with type information and data stored in a union.
/// # Safety
/// The union field contains raw pointers which must remain valid for the lifetime
/// of the Value. The ValueType must accurately reflect which union field is active.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Value {
    pub type_0: ValueType,
    pub u: U,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union U {
    pub string: *mut c_char,
    pub number: Number,
    pub object: Object,
    pub array: Array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Array {
    pub values: *mut *mut Value,
    pub len: usize,
}
impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for i in 0..self.len {
            list.entry(unsafe { &**self.values.add(i) });
        }
        list.finish()
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Object {
    pub keys: *mut *const c_char,
    pub values: *mut *mut Value,
    pub len: usize,
}
impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = f.debug_map();
        for i in 0..self.len {
            map.key(unsafe { &CStr::from_ptr(*self.keys.add(i)) });
            map.value(unsafe { &**self.values.add(i) });
        }
        map.finish()
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Number {
    pub i: c_longlong,
    pub d: c_double,
    pub r: *mut c_char,
    pub flags: c_uint,
}
impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.flags & 1 != 0 {
            f.write_fmt(format_args!("Integer({})", self.i))
        } else if self.flags & 2 != 0 {
            f.write_fmt(format_args!("Double({})", self.d))
        } else {
            f.write_fmt(format_args!("Number({:?})", unsafe {
                CStr::from_ptr(self.r)
            }))
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.type_0 {
            ValueType::Any => f.write_str("Value::Any"),
            ValueType::Null => f.write_str("Value::Null"),
            ValueType::True => f.write_str("Value::True"),
            ValueType::False => f.write_str("Value::False"),
            ValueType::Number => f.write_fmt(format_args!("Value::Number({:?})", unsafe {
                self.u.number
            })),
            ValueType::Array => {
                f.write_fmt(format_args!("Value::Array({:?})", unsafe { self.u.array }))
            }
            ValueType::Object => f.write_fmt(format_args!("Value::Object({:?})", unsafe {
                self.u.object
            })),
            ValueType::String => f.write_fmt(format_args!("Value::String({:?})", unsafe {
                CStr::from_ptr(self.u.string)
            })),
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum ValueError {
    OutOfMemory = 12,
    ObjectKeyIsNotAString,
    CantAddValueToNonCompsiteType = 22,
    BottomOfStackReachedPrematurely,
}
impl ValueError {
    pub fn as_bytes_w_null(&self) -> &[u8] {
        match self {
            Self::OutOfMemory => &b"Out of memory\0"[..],
            _ => todo!(),
        }
    }
}
impl Value {
    const NUMBER_INVALID: c_uint = 0x00;
    const NUMBER_INT_VALID: c_uint = 0x01;
    const NUMBER_DOUBLE_VALID: c_uint = 0x02;

    unsafe fn alloc(mut type_0: ValueType) -> Result<*mut Value, ValueError> {
        let mut v: *mut Value = ptr::null_mut();
        v = libc::malloc(::core::mem::size_of::<Value>()) as *mut Value;
        if v.is_null() {
            return Err(ValueError::OutOfMemory);
        }
        ptr::write_bytes(v, 0, 1);

        (*v).type_0 = type_0;
        Ok(v)
    }
    unsafe fn object_free(mut v: *mut Value) {
        let mut i: usize = 0;
        if !(!v.is_null() && (*v).type_0 == ValueType::Object) {
            return;
        }
        i = 0 as c_int as usize;
        while i < (*v).u.object.len {
            libc::free(*((*v).u.object.keys).add(i) as *mut c_char as *mut c_void);
            let fresh0 = &mut (*((*v).u.object.keys).add(i));
            *fresh0 = ptr::null::<c_char>();
            Value::tree_free(*((*v).u.object.values).add(i));
            let fresh1 = &mut (*((*v).u.object.values).add(i));
            *fresh1 = 0 as *mut Value;
            i = i.wrapping_add(1);
        }
        libc::free((*v).u.object.keys as *mut c_void);
        libc::free((*v).u.object.values as *mut c_void);
        libc::free(v as *mut c_void);
    }
    unsafe fn array_free(mut v: *mut Value) {
        let mut i: usize = 0;
        if !(!v.is_null() && (*v).type_0 == ValueType::Array) {
            return;
        }
        i = 0 as c_int as usize;
        while i < (*v).u.array.len {
            Value::tree_free(*((*v).u.array.values).add(i));
            let fresh2 = &mut (*((*v).u.array.values).add(i));
            *fresh2 = 0 as *mut Value;
            i = i.wrapping_add(1);
        }
        libc::free((*v).u.array.values as *mut c_void);
        libc::free(v as *mut c_void);
    }
    pub fn is_string(&self) -> bool {
        self.type_0 == ValueType::String
    }
    pub fn is_number(&self) -> bool {
        self.type_0 == ValueType::Number
    }
    pub fn is_integer(&self) -> bool {
        self.is_number() && unsafe { self.u.number.flags } & Self::NUMBER_INT_VALID != 0
    }
    pub fn is_double(&self) -> bool {
        self.is_number() && unsafe { self.u.number.flags } & Self::NUMBER_DOUBLE_VALID != 0
    }
    pub fn is_object(&self) -> bool {
        self.type_0 == ValueType::Object
    }
    pub fn is_array(&self) -> bool {
        self.type_0 == ValueType::Array
    }
    pub fn is_true(&self) -> bool {
        self.type_0 == ValueType::True
    }
    pub fn is_false(&self) -> bool {
        self.type_0 == ValueType::False
    }
    pub fn is_null(&self) -> bool {
        self.type_0 == ValueType::Null
    }

    /// Return a bool if the value is a bool, otherwise `None`.
    pub fn as_bool(&self) -> Option<bool> {
        match self.type_0 {
            ValueType::True => Some(true),
            ValueType::False => Some(false),
            _ => None,
        }
    }
    /// Given a yajl_val_string return a *const ptr to the bare string it contains,
    /// or `None` if the value is not a string.
    pub fn as_string(&self) -> Option<*const c_char> {
        if self.is_string() {
            Some(unsafe { self.u.string })
        } else {
            None
        }
    }
    /// Given a yajl_val_string return a *mut ptr to the bare string it contains,
    /// or `None` if the value is not a string.
    pub fn as_string_mut(&mut self) -> Option<*mut c_char> {
        if self.is_string() {
            Some(unsafe { self.u.string })
        } else {
            None
        }
    }
    /// Get the string representation of a number.
    /// Returns None if this Value is not a number.
    pub fn as_number(&self) -> Option<*const c_char> {
        if self.is_number() {
            Some(unsafe { self.u.number.r })
        } else {
            None
        }
    }

    /// Get the double representation of a number,
    /// or `None` if the value is not a double.
    pub fn as_double(&self) -> Option<c_double> {
        if self.is_double() {
            Some(unsafe { self.u.number.d })
        } else {
            None
        }
    }
    /// Get the 64bit (long long) integer representation of a number
    /// or `None` if the value is not an integer.
    pub fn as_integer(&self) -> Option<c_longlong> {
        if self.is_integer() {
            Some(unsafe { self.u.number.i })
        } else {
            None
        }
    }
    /// Get a const pointer to a `Object` or `None` if the value is not an object.
    pub fn as_object(&self) -> Option<*const Object> {
        if self.is_object() {
            Some(unsafe { &self.u.object })
        } else {
            None
        }
    }
    /// Get a mut pointer to a `Object` or `None` if the value is not an object.
    pub fn as_object_mut(&mut self) -> Option<*mut Object> {
        if self.is_object() {
            Some(unsafe { &mut self.u.object })
        } else {
            None
        }
    }
    /// Get a const pointer to a `Array` or `None` if the value is not an object.
    pub fn as_array(&self) -> Option<*const Array> {
        if self.is_array() {
            Some(unsafe { &self.u.array })
        } else {
            None
        }
    }
    /// Get a mut pointer to a `Array` or `None` if the value is not an object.
    pub fn as_array_mut(&mut self) -> Option<*mut Array> {
        if self.is_array() {
            Some(unsafe { &mut self.u.array })
        } else {
            None
        }
    }

    unsafe fn alloc_array() -> Result<*mut Value, ValueError> {
        let v = Value::alloc(ValueType::Array)?;

        (*v).u.array.values = ptr::null_mut();
        (*v).u.array.len = 0;

        Ok(v)
    }
    unsafe fn alloc_bool(value: bool) -> Result<*mut Value, ValueError> {
        let v = Value::alloc(if value {
            ValueType::True
        } else {
            ValueType::False
        })?;
        Ok(v)
    }
    unsafe fn alloc_null() -> Result<*mut Value, ValueError> {
        let v = Value::alloc(ValueType::Null)?;
        Ok(v)
    }
    unsafe fn alloc_number(string: *const c_char, len: usize) -> Result<*mut Value, ValueError> {
        let v = Value::alloc(ValueType::Number)?;
        (*v).u.number.r = libc::malloc(len.wrapping_add(1)).cast::<c_char>();
        if ((*v).u.number.r).is_null() {
            libc::free(v.cast());
            return Err(ValueError::OutOfMemory);
        }
        libc::memcpy((*v).u.number.r.cast(), string.cast(), len);
        *((*v).u.number.r).add(len) = 0;
        (*v).u.number.flags = Self::NUMBER_INVALID;
        (*v).u.number.i = match parse_integer((*v).u.number.r.cast(), libc::strlen((*v).u.number.r))
        {
            Ok(integer) => {
                (*v).u.number.flags |= Self::NUMBER_INT_VALID;
                integer
            }
            Err(ParseIntegerError::Underflow) => i64::MIN,
            _ => i64::MAX,
        };
        if let Some(s) = CStr::from_ptr((*v).u.number.r).to_str().ok() {
            if let Some((d, d_len)) = strtod::strtod(s) {
                (*v).u.number.d = d;
                if d_len == len {
                    (*v).u.number.flags |= Self::NUMBER_DOUBLE_VALID;
                }
            } else {
                (*v).u.number.d = 0f64;
            };
        } else {
            (*v).u.number.d = 0f64;
        };
        Ok(v)
    }
    unsafe fn alloc_object() -> Result<*mut Value, ValueError> {
        let v = Value::alloc(ValueType::Object)?;

        (*v).u.object.keys = ptr::null_mut();
        (*v).u.object.values = ptr::null_mut();
        (*v).u.object.len = 0;

        Ok(v)
    }
    unsafe fn alloc_string(string: *const c_char, len: usize) -> Result<*mut Value, ValueError> {
        let v = Value::alloc(ValueType::String)?;

        (*v).u.string = libc::malloc(len + 1).cast::<c_char>();
        if (*v).u.string.is_null() {
            libc::free(v.cast());
            return Err(ValueError::OutOfMemory);
        }
        libc::memcpy((*v).u.string.cast(), string.cast(), len);
        *((*v).u.string).add(len) = 0;

        Ok(v)
    }
}
unsafe extern "C" fn handle_string(
    mut ctx: *mut c_void,
    mut string: *const c_uchar,
    mut string_length: usize,
) -> c_int {
    let ctx = ctx.cast::<Context>();
    match Value::alloc_string(string.cast(), string_length).and_then(|v| Context::add_value(ctx, v))
    {
        Ok(_) => 1,
        Err(err) => {
            Context::record_error(ctx, err.as_bytes_w_null());
            0
        }
    }
}
unsafe extern "C" fn handle_number(
    mut ctx: *mut c_void,
    mut string: *const c_char,
    mut string_length: usize,
) -> c_int {
    let ctx = ctx.cast::<Context>();
    match Value::alloc_number(string.cast(), string_length).and_then(|v| Context::add_value(ctx, v))
    {
        Ok(_) => 1,
        Err(err) => {
            Context::record_error(ctx, err.as_bytes_w_null());
            0
        }
    }
}
unsafe extern "C" fn handle_start_map(mut ctx: *mut c_void) -> c_int {
    let ctx = ctx.cast::<Context>();
    match Value::alloc_object().and_then(|v| Context::push(ctx, v)) {
        Ok(_) => 1,
        Err(err) => {
            Context::record_error(ctx, err.as_bytes_w_null());
            0
        }
    }
}

unsafe extern "C" fn handle_end_map(mut ctx: *mut c_void) -> i32 {
    let ctx = ctx.cast::<Context>();
    let Ok(v) = Context::pop(ctx) else {
        return 0;
    };
    if v.is_null() {
        return 0;
    }
    match Context::add_value(ctx, v) {
        Ok(_) => 1,
        Err(err) => {
            Context::record_error(ctx, err.as_bytes_w_null());
            0
        }
    }
}
unsafe extern "C" fn handle_start_array(mut ctx: *mut c_void) -> c_int {
    let ctx = ctx.cast::<Context>();
    match Value::alloc_array().and_then(|v| Context::push(ctx, v)) {
        Ok(_) => 1,
        Err(err) => {
            Context::record_error(ctx, err.as_bytes_w_null());
            0
        }
    }
}
unsafe extern "C" fn handle_end_array(mut ctx: *mut c_void) -> i32 {
    let ctx = ctx.cast::<Context>();
    let Ok(v) = Context::pop(ctx) else {
        return 0;
    };
    if v.is_null() {
        return 0;
    }
    match Context::add_value(ctx, v) {
        Ok(_) => 1,
        Err(err) => {
            Context::record_error(ctx, err.as_bytes_w_null());
            0
        }
    }
}
unsafe extern "C" fn handle_boolean(mut ctx: *mut c_void, mut boolean_value: c_int) -> c_int {
    let ctx = ctx.cast::<Context>();
    match Value::alloc_bool(boolean_value != 0).and_then(|v| Context::add_value(ctx, v)) {
        Ok(_) => 1,
        Err(err) => {
            Context::record_error(ctx, err.as_bytes_w_null());
            0
        }
    }
}
unsafe extern "C" fn handle_null(mut ctx: *mut c_void) -> c_int {
    let ctx = ctx.cast::<Context>();
    match Value::alloc_null().and_then(|v| Context::add_value(ctx, v)) {
        Ok(_) => 1,
        Err(err) => {
            Context::record_error(ctx, err.as_bytes_w_null());
            0
        }
    }
}

pub unsafe fn yajl_tree_parse(
    mut input: *const c_char,
    mut error_buffer: *mut c_char,
    mut error_buffer_size: usize,
) -> Option<*mut Value> {
    if input.is_null() {
        return None;
    }
    static mut callbacks: yajl_callbacks = unsafe {
        {
            yajl_callbacks {
                yajl_null: Some(handle_null as unsafe extern "C" fn(*mut c_void) -> c_int),
                yajl_boolean: Some(
                    handle_boolean as unsafe extern "C" fn(*mut c_void, c_int) -> c_int,
                ),
                yajl_integer: None,
                yajl_double: None,
                yajl_number: Some(
                    handle_number
                        as unsafe extern "C" fn(*mut c_void, *const c_char, usize) -> c_int,
                ),
                yajl_string: Some(
                    handle_string
                        as unsafe extern "C" fn(*mut c_void, *const c_uchar, usize) -> c_int,
                ),
                yajl_start_map: Some(
                    handle_start_map as unsafe extern "C" fn(*mut c_void) -> c_int,
                ),
                yajl_map_key: Some(
                    handle_string
                        as unsafe extern "C" fn(*mut c_void, *const c_uchar, usize) -> c_int,
                ),
                yajl_end_map: Some(handle_end_map as unsafe extern "C" fn(*mut c_void) -> c_int),
                yajl_start_array: Some(
                    handle_start_array as unsafe extern "C" fn(*mut c_void) -> c_int,
                ),
                yajl_end_array: Some(
                    handle_end_array as unsafe extern "C" fn(*mut c_void) -> c_int,
                ),
            }
        }
    };
    let mut ctx = Context::new(error_buffer, error_buffer_size);

    let mut handle = Parser::alloc(
        ptr::addr_of!(callbacks),
        ptr::null_mut::<yajl_alloc_funcs>(),
        &mut ctx as *mut Context as *mut c_void,
    );
    let parser = unsafe { &mut *handle };
    parser.config(ParserOption::AllowComments, true);
    let mut status = parser.parse(input as *mut c_uchar, libc::strlen(input));
    status = parser.complete_parse();
    if status != Status::Ok {
        if !error_buffer.is_null() && error_buffer_size > 0 {
            let internal_err_str =
                parser.get_error(true, input as *const c_uchar, libc::strlen(input)) as *mut c_char;
            libc::snprintf(
                error_buffer,
                error_buffer_size,
                b"%s\0" as *const u8 as *const c_char,
                internal_err_str,
            );
            ((*handle).alloc.free).expect("non-null function pointer")(
                (*handle).alloc.ctx,
                internal_err_str as *mut c_void,
            );
        }
        Parser::free(handle);
        return None;
    }
    Parser::free(handle);
    debug_assert!(!ctx.root.is_null());
    Some(ctx.root)
}

pub unsafe fn yajl_tree_get(
    mut n: *mut Value,
    mut path: *mut *const c_char,
    mut type_0: ValueType,
) -> Option<*mut Value> {
    if n.is_null() {
        return None;
    }
    if path.is_null() {
        return None;
    }
    while !n.is_null() && !(*path).is_null() {
        let mut i: usize = 0;
        let mut len: usize = 0;
        if !(*n).is_object() {
            return None;
        }
        len = (*n).u.object.len;
        i = 0 as c_int as usize;
        while i < len {
            if libc::strcmp(*path, *((*n).u.object.keys).add(i)) == 0 {
                n = *((*n).u.object.values).add(i);
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        if i == len {
            return None;
        }
        path = path.offset(1);
    }
    if !n.is_null() && type_0 != ValueType::Any && type_0 != (*n).type_0 {
        return None;
    }
    debug_assert!(!n.is_null());
    Some(n)
}
unsafe fn yajl_is_string(v: *mut Value) -> bool {
    !v.is_null() && (*v).type_0 == ValueType::String
}
unsafe fn yajl_is_number(v: *mut Value) -> bool {
    !v.is_null() && (*v).type_0 == ValueType::Number
}
unsafe fn yajl_is_object(v: *mut Value) -> bool {
    !v.is_null() && (*v).type_0 == ValueType::Object
}
unsafe fn yajl_is_array(v: *mut Value) -> bool {
    !v.is_null() && (*v).type_0 == ValueType::Array
}
unsafe fn yajl_get_object(v: *mut Value) -> *mut Object {
    if yajl_is_object(v) {
        &mut (*v).u.object as *mut Object
    } else {
        ptr::null_mut()
    }
}
unsafe fn yajl_get_array(v: *mut Value) -> *mut Array {
    if yajl_is_array(v) {
        &mut (*v).u.array as *mut Array
    } else {
        ptr::null_mut()
    }
}
impl Value {
    pub unsafe fn tree_free(mut v: *mut Value) {
        if v.is_null() {
            return;
        }
        match (*v).type_0 {
            ValueType::String => {
                libc::free((*v).u.string as *mut c_void);
                libc::free(v as *mut c_void);
            }
            ValueType::Number => {
                libc::free((*v).u.number.r as *mut c_void);
                libc::free(v as *mut c_void);
            }
            ValueType::Object => {
                Value::object_free(v);
            }
            ValueType::Array => Value::array_free(v),
            _ => {
                libc::free(v as *mut c_void);
            }
        }
    }
}
