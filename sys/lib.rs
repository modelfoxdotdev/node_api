#![allow(non_camel_case_types, improper_ctypes)]

use std::os::raw::{c_char, c_int, c_uint, c_void};
type size_t = usize;
type char16_t = u16;

#[derive(Debug, Copy, Clone)]
pub struct napi_env__ {
	_unused: [u8; 0],
}

pub type napi_env = *mut napi_env__;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_value__ {
	_unused: [u8; 0],
}

pub type napi_value = *mut napi_value__;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_ref__ {
	_unused: [u8; 0],
}

pub type napi_ref = *mut napi_ref__;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_handle_scope__ {
	_unused: [u8; 0],
}

pub type napi_handle_scope = *mut napi_handle_scope__;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_escapable_handle_scope__ {
	_unused: [u8; 0],
}

pub type napi_escapable_handle_scope = *mut napi_escapable_handle_scope__;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_callback_scope__ {
	_unused: [u8; 0],
}

pub type napi_callback_scope = *mut napi_callback_scope__;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_callback_info__ {
	_unused: [u8; 0],
}

pub type napi_callback_info = *mut napi_callback_info__;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_async_context__ {
	_unused: [u8; 0],
}

pub type napi_async_context = *mut napi_async_context__;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_async_work__ {
	_unused: [u8; 0],
}

pub type napi_async_work = *mut napi_async_work__;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_deferred__ {
	_unused: [u8; 0],
}

pub type napi_deferred = *mut napi_deferred__;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_threadsafe_function__ {
	_unused: [u8; 0],
}

pub type napi_threadsafe_function = *mut napi_threadsafe_function__;

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum napi_property_attributes {
	napi_default = 0,
	napi_writable = 1,
	napi_enumerable = 2,
	napi_configurable = 4,
	napi_static = 1024,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum napi_valuetype {
	napi_undefined = 0,
	napi_null = 1,
	napi_boolean = 2,
	napi_number = 3,
	napi_string = 4,
	napi_symbol = 5,
	napi_object = 6,
	napi_function = 7,
	napi_external = 8,
	napi_bigint = 9,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum napi_typedarray_type {
	napi_int8_array = 0,
	napi_uint8_array = 1,
	napi_uint8_clamped_array = 2,
	napi_int16_array = 3,
	napi_uint16_array = 4,
	napi_int32_array = 5,
	napi_uint32_array = 6,
	napi_float32_array = 7,
	napi_float64_array = 8,
	napi_bigint64_array = 9,
	napi_biguint64_array = 10,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum napi_status {
	napi_ok = 0,
	napi_invalid_arg = 1,
	napi_object_expected = 2,
	napi_string_expected = 3,
	napi_name_expected = 4,
	napi_function_expected = 5,
	napi_number_expected = 6,
	napi_boolean_expected = 7,
	napi_array_expected = 8,
	napi_generic_failure = 9,
	napi_pending_exception = 10,
	napi_cancelled = 11,
	napi_escape_called_twice = 12,
	napi_handle_scope_mismatch = 13,
	napi_callback_scope_mismatch = 14,
	napi_queue_full = 15,
	napi_closing = 16,
	napi_bigint_expected = 17,
	napi_date_expected = 18,
	napi_arraybuffer_expected = 19,
	napi_detachable_arraybuffer_expected = 20,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum napi_threadsafe_function_release_mode {
	napi_tsfn_release = 0,
	napi_tsfn_abort = 1,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum napi_threadsafe_function_call_mode {
	napi_tsfn_nonblocking = 0,
	napi_tsfn_blocking = 1,
}

pub type napi_callback =
	Option<unsafe extern "C" fn(env: napi_env, info: napi_callback_info) -> napi_value>;

pub type napi_finalize = Option<
	unsafe extern "C" fn(env: napi_env, finalize_data: *mut c_void, finalize_hint: *mut c_void),
>;

pub type napi_async_execute_callback =
	Option<unsafe extern "C" fn(env: napi_env, data: *mut c_void)>;

pub type napi_async_complete_callback =
	Option<unsafe extern "C" fn(env: napi_env, status: napi_status, data: *mut c_void)>;

pub type napi_threadsafe_function_call_js = Option<
	unsafe extern "C" fn(
		env: napi_env,
		js_callback: napi_value,
		context: *mut c_void,
		data: *mut c_void,
	),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_property_descriptor {
	pub utf8name: *const c_char,
	pub name: napi_value,
	pub method: napi_callback,
	pub getter: napi_callback,
	pub setter: napi_callback,
	pub value: napi_value,
	pub attributes: napi_property_attributes,
	pub data: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_extended_error_info {
	pub error_message: *const c_char,
	pub engine_reserved: *mut c_void,
	pub engine_error_code: u32,
	pub error_code: napi_status,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_node_version {
	pub major: u32,
	pub minor: u32,
	pub patch: u32,
	pub release: *const c_char,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum napi_key_collection_mode {
	napi_key_include_prototypes = 0,
	napi_key_own_only = 1,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum napi_key_filter {
	napi_key_all_properties = 0,
	napi_key_writable = 1,
	napi_key_enumerable = 2,
	napi_key_configurable = 4,
	napi_key_skip_strings = 8,
	napi_key_skip_symbols = 16,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum napi_key_conversion {
	napi_key_keep_numbers = 0,
	napi_key_numbers_to_strings = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uv_loop_s {
	_unused: [u8; 0],
}

pub type napi_addon_register_func =
	Option<unsafe extern "C" fn(env: napi_env, exports: napi_value) -> napi_value>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_module {
	pub nm_version: c_int,
	pub nm_flags: c_uint,
	pub nm_filename: *const c_char,
	pub nm_register_func: napi_addon_register_func,
	pub nm_modname: *const c_char,
	pub nm_priv: *mut c_void,
	pub reserved: [*mut c_void; 4usize],
}

#[cfg(feature = "v1")]
extern "C" {
	pub fn napi_module_register(mod_: *mut napi_module);
	pub fn napi_get_last_error_info(
		env: napi_env,
		result: *mut *const napi_extended_error_info,
	) -> napi_status;
	pub fn napi_fatal_error(
		location: *const c_char,
		location_len: size_t,
		message: *const c_char,
		message_len: size_t,
	);
	pub fn napi_get_undefined(env: napi_env, result: *mut napi_value) -> napi_status;
	pub fn napi_get_null(env: napi_env, result: *mut napi_value) -> napi_status;
	pub fn napi_get_global(env: napi_env, result: *mut napi_value) -> napi_status;
	pub fn napi_get_boolean(env: napi_env, value: bool, result: *mut napi_value) -> napi_status;
	pub fn napi_create_object(env: napi_env, result: *mut napi_value) -> napi_status;
	pub fn napi_create_array(env: napi_env, result: *mut napi_value) -> napi_status;
	pub fn napi_create_array_with_length(
		env: napi_env,
		length: size_t,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_create_double(env: napi_env, value: f64, result: *mut napi_value) -> napi_status;
	pub fn napi_create_int32(env: napi_env, value: i32, result: *mut napi_value) -> napi_status;
	pub fn napi_create_uint32(env: napi_env, value: u32, result: *mut napi_value) -> napi_status;
	pub fn napi_create_int64(env: napi_env, value: i64, result: *mut napi_value) -> napi_status;
	pub fn napi_create_string_latin1(
		env: napi_env,
		str_: *const c_char,
		length: size_t,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_create_string_utf8(
		env: napi_env,
		str_: *const c_char,
		length: size_t,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_create_string_utf16(
		env: napi_env,
		str_: *const char16_t,
		length: size_t,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_create_symbol(
		env: napi_env,
		description: napi_value,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_create_function(
		env: napi_env,
		utf8name: *const c_char,
		length: size_t,
		cb: napi_callback,
		data: *mut c_void,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_create_error(
		env: napi_env,
		code: napi_value,
		msg: napi_value,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_create_type_error(
		env: napi_env,
		code: napi_value,
		msg: napi_value,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_create_range_error(
		env: napi_env,
		code: napi_value,
		msg: napi_value,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_typeof(
		env: napi_env,
		value: napi_value,
		result: *mut napi_valuetype,
	) -> napi_status;
	pub fn napi_get_value_double(env: napi_env, value: napi_value, result: *mut f64)
		-> napi_status;
	pub fn napi_get_value_int32(env: napi_env, value: napi_value, result: *mut i32) -> napi_status;
	pub fn napi_get_value_uint32(env: napi_env, value: napi_value, result: *mut u32)
		-> napi_status;
	pub fn napi_get_value_int64(env: napi_env, value: napi_value, result: *mut i64) -> napi_status;
	pub fn napi_get_value_bool(env: napi_env, value: napi_value, result: *mut bool) -> napi_status;
	pub fn napi_get_value_string_latin1(
		env: napi_env,
		value: napi_value,
		buf: *mut c_char,
		bufsize: size_t,
		result: *mut size_t,
	) -> napi_status;
	pub fn napi_get_value_string_utf8(
		env: napi_env,
		value: napi_value,
		buf: *mut c_char,
		bufsize: size_t,
		result: *mut size_t,
	) -> napi_status;
	pub fn napi_get_value_string_utf16(
		env: napi_env,
		value: napi_value,
		buf: *mut char16_t,
		bufsize: size_t,
		result: *mut size_t,
	) -> napi_status;
	pub fn napi_coerce_to_bool(
		env: napi_env,
		value: napi_value,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_coerce_to_number(
		env: napi_env,
		value: napi_value,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_coerce_to_object(
		env: napi_env,
		value: napi_value,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_coerce_to_string(
		env: napi_env,
		value: napi_value,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_get_prototype(
		env: napi_env,
		object: napi_value,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_get_property_names(
		env: napi_env,
		object: napi_value,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_set_property(
		env: napi_env,
		object: napi_value,
		key: napi_value,
		value: napi_value,
	) -> napi_status;
	pub fn napi_has_property(
		env: napi_env,
		object: napi_value,
		key: napi_value,
		result: *mut bool,
	) -> napi_status;
	pub fn napi_get_property(
		env: napi_env,
		object: napi_value,
		key: napi_value,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_delete_property(
		env: napi_env,
		object: napi_value,
		key: napi_value,
		result: *mut bool,
	) -> napi_status;
	pub fn napi_has_own_property(
		env: napi_env,
		object: napi_value,
		key: napi_value,
		result: *mut bool,
	) -> napi_status;
	pub fn napi_set_named_property(
		env: napi_env,
		object: napi_value,
		utf8name: *const c_char,
		value: napi_value,
	) -> napi_status;
	pub fn napi_has_named_property(
		env: napi_env,
		object: napi_value,
		utf8name: *const c_char,
		result: *mut bool,
	) -> napi_status;
	pub fn napi_get_named_property(
		env: napi_env,
		object: napi_value,
		utf8name: *const c_char,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_set_element(
		env: napi_env,
		object: napi_value,
		index: u32,
		value: napi_value,
	) -> napi_status;
	pub fn napi_has_element(
		env: napi_env,
		object: napi_value,
		index: u32,
		result: *mut bool,
	) -> napi_status;
	pub fn napi_get_element(
		env: napi_env,
		object: napi_value,
		index: u32,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_delete_element(
		env: napi_env,
		object: napi_value,
		index: u32,
		result: *mut bool,
	) -> napi_status;
	pub fn napi_define_properties(
		env: napi_env,
		object: napi_value,
		property_count: size_t,
		properties: *const napi_property_descriptor,
	) -> napi_status;
	pub fn napi_is_array(env: napi_env, value: napi_value, result: *mut bool) -> napi_status;
	pub fn napi_get_array_length(env: napi_env, value: napi_value, result: *mut u32)
		-> napi_status;
	pub fn napi_strict_equals(
		env: napi_env,
		lhs: napi_value,
		rhs: napi_value,
		result: *mut bool,
	) -> napi_status;
	pub fn napi_call_function(
		env: napi_env,
		recv: napi_value,
		func: napi_value,
		argc: size_t,
		argv: *const napi_value,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_new_instance(
		env: napi_env,
		constructor: napi_value,
		argc: size_t,
		argv: *const napi_value,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_instanceof(
		env: napi_env,
		object: napi_value,
		constructor: napi_value,
		result: *mut bool,
	) -> napi_status;
	pub fn napi_get_cb_info(
		env: napi_env,
		cbinfo: napi_callback_info,
		argc: *mut size_t,
		argv: *mut napi_value,
		this_arg: *mut napi_value,
		data: *mut *mut c_void,
	) -> napi_status;
	pub fn napi_get_new_target(
		env: napi_env,
		cbinfo: napi_callback_info,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_define_class(
		env: napi_env,
		utf8name: *const c_char,
		length: size_t,
		constructor: napi_callback,
		data: *mut c_void,
		property_count: size_t,
		properties: *const napi_property_descriptor,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_wrap(
		env: napi_env,
		js_object: napi_value,
		native_object: *mut c_void,
		finalize_cb: napi_finalize,
		finalize_hint: *mut c_void,
		result: *mut napi_ref,
	) -> napi_status;
	pub fn napi_unwrap(
		env: napi_env,
		js_object: napi_value,
		result: *mut *mut c_void,
	) -> napi_status;
	pub fn napi_remove_wrap(
		env: napi_env,
		js_object: napi_value,
		result: *mut *mut c_void,
	) -> napi_status;
	pub fn napi_create_external(
		env: napi_env,
		data: *mut c_void,
		finalize_cb: napi_finalize,
		finalize_hint: *mut c_void,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_get_value_external(
		env: napi_env,
		value: napi_value,
		result: *mut *mut c_void,
	) -> napi_status;
	pub fn napi_create_reference(
		env: napi_env,
		value: napi_value,
		initial_refcount: u32,
		result: *mut napi_ref,
	) -> napi_status;
	pub fn napi_delete_reference(env: napi_env, ref_: napi_ref) -> napi_status;
	pub fn napi_reference_ref(env: napi_env, ref_: napi_ref, result: *mut u32) -> napi_status;
	pub fn napi_reference_unref(env: napi_env, ref_: napi_ref, result: *mut u32) -> napi_status;
	pub fn napi_get_reference_value(
		env: napi_env,
		ref_: napi_ref,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_open_handle_scope(env: napi_env, result: *mut napi_handle_scope) -> napi_status;
	pub fn napi_close_handle_scope(env: napi_env, scope: napi_handle_scope) -> napi_status;
	pub fn napi_open_escapable_handle_scope(
		env: napi_env,
		result: *mut napi_escapable_handle_scope,
	) -> napi_status;
	pub fn napi_close_escapable_handle_scope(
		env: napi_env,
		scope: napi_escapable_handle_scope,
	) -> napi_status;
	pub fn napi_escape_handle(
		env: napi_env,
		scope: napi_escapable_handle_scope,
		escapee: napi_value,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_throw(env: napi_env, error: napi_value) -> napi_status;
	pub fn napi_throw_error(env: napi_env, code: *const c_char, msg: *const c_char) -> napi_status;
	pub fn napi_throw_type_error(
		env: napi_env,
		code: *const c_char,
		msg: *const c_char,
	) -> napi_status;
	pub fn napi_throw_range_error(
		env: napi_env,
		code: *const c_char,
		msg: *const c_char,
	) -> napi_status;
	pub fn napi_is_error(env: napi_env, value: napi_value, result: *mut bool) -> napi_status;
	pub fn napi_is_exception_pending(env: napi_env, result: *mut bool) -> napi_status;
	pub fn napi_get_and_clear_last_exception(env: napi_env, result: *mut napi_value)
		-> napi_status;
	pub fn napi_create_buffer(
		env: napi_env,
		length: size_t,
		data: *mut *mut c_void,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_create_external_buffer(
		env: napi_env,
		length: size_t,
		data: *mut c_void,
		finalize_cb: napi_finalize,
		finalize_hint: *mut c_void,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_create_buffer_copy(
		env: napi_env,
		length: size_t,
		data: *const c_void,
		result_data: *mut *mut c_void,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_is_buffer(env: napi_env, value: napi_value, result: *mut bool) -> napi_status;
	pub fn napi_get_buffer_info(
		env: napi_env,
		value: napi_value,
		data: *mut *mut c_void,
		length: *mut size_t,
	) -> napi_status;
	pub fn napi_is_arraybuffer(env: napi_env, value: napi_value, result: *mut bool) -> napi_status;
	pub fn napi_create_arraybuffer(
		env: napi_env,
		byte_length: size_t,
		data: *mut *mut c_void,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_create_external_arraybuffer(
		env: napi_env,
		external_data: *mut c_void,
		byte_length: size_t,
		finalize_cb: napi_finalize,
		finalize_hint: *mut c_void,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_get_arraybuffer_info(
		env: napi_env,
		arraybuffer: napi_value,
		data: *mut *mut c_void,
		byte_length: *mut size_t,
	) -> napi_status;
	pub fn napi_is_typedarray(env: napi_env, value: napi_value, result: *mut bool) -> napi_status;
	pub fn napi_create_typedarray(
		env: napi_env,
		type_: napi_typedarray_type,
		length: size_t,
		arraybuffer: napi_value,
		byte_offset: size_t,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_get_typedarray_info(
		env: napi_env,
		typedarray: napi_value,
		type_: *mut napi_typedarray_type,
		length: *mut size_t,
		data: *mut *mut c_void,
		arraybuffer: *mut napi_value,
		byte_offset: *mut size_t,
	) -> napi_status;
	pub fn napi_create_dataview(
		env: napi_env,
		length: size_t,
		arraybuffer: napi_value,
		byte_offset: size_t,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_is_dataview(env: napi_env, value: napi_value, result: *mut bool) -> napi_status;
	pub fn napi_get_dataview_info(
		env: napi_env,
		dataview: napi_value,
		bytelength: *mut size_t,
		data: *mut *mut c_void,
		arraybuffer: *mut napi_value,
		byte_offset: *mut size_t,
	) -> napi_status;
	pub fn napi_create_async_work(
		env: napi_env,
		async_resource: napi_value,
		async_resource_name: napi_value,
		execute: napi_async_execute_callback,
		complete: napi_async_complete_callback,
		data: *mut c_void,
		result: *mut napi_async_work,
	) -> napi_status;
	pub fn napi_delete_async_work(env: napi_env, work: napi_async_work) -> napi_status;
	pub fn napi_queue_async_work(env: napi_env, work: napi_async_work) -> napi_status;
	pub fn napi_cancel_async_work(env: napi_env, work: napi_async_work) -> napi_status;
	pub fn napi_async_init(
		env: napi_env,
		async_resource: napi_value,
		async_resource_name: napi_value,
		result: *mut napi_async_context,
	) -> napi_status;
	pub fn napi_async_destroy(env: napi_env, async_context: napi_async_context) -> napi_status;
	pub fn napi_make_callback(
		env: napi_env,
		async_context: napi_async_context,
		recv: napi_value,
		func: napi_value,
		argc: size_t,
		argv: *const napi_value,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_get_version(env: napi_env, result: *mut u32) -> napi_status;
	pub fn napi_get_node_version(
		env: napi_env,
		version: *mut *const napi_node_version,
	) -> napi_status;
	pub fn napi_create_promise(
		env: napi_env,
		deferred: *mut napi_deferred,
		promise: *mut napi_value,
	) -> napi_status;
	pub fn napi_resolve_deferred(
		env: napi_env,
		deferred: napi_deferred,
		resolution: napi_value,
	) -> napi_status;
	pub fn napi_reject_deferred(
		env: napi_env,
		deferred: napi_deferred,
		rejection: napi_value,
	) -> napi_status;
	pub fn napi_is_promise(
		env: napi_env,
		promise: napi_value,
		is_promise: *mut bool,
	) -> napi_status;
	pub fn napi_adjust_external_memory(
		env: napi_env,
		change_in_bytes: i64,
		adjusted_value: *mut i64,
	) -> napi_status;
	pub fn napi_run_script(
		env: napi_env,
		script: napi_value,
		result: *mut napi_value,
	) -> napi_status;
}

#[cfg(feature = "v2")]
extern "C" {
	pub fn napi_get_uv_event_loop(env: napi_env, loop_: *mut *mut uv_loop_s) -> napi_status;
}

#[cfg(feature = "v3")]
extern "C" {
	pub fn napi_open_callback_scope(
		env: napi_env,
		resource_object: napi_value,
		context: napi_async_context,
		result: *mut napi_callback_scope,
	) -> napi_status;
	pub fn napi_close_callback_scope(env: napi_env, scope: napi_callback_scope) -> napi_status;
	pub fn napi_fatal_exception(env: napi_env, err: napi_value) -> napi_status;
	pub fn napi_add_env_cleanup_hook(
		env: napi_env,
		fun: Option<unsafe extern "C" fn(arg: *mut c_void)>,
		arg: *mut c_void,
	) -> napi_status;
	pub fn napi_remove_env_cleanup_hook(
		env: napi_env,
		fun: Option<unsafe extern "C" fn(arg: *mut c_void)>,
		arg: *mut c_void,
	) -> napi_status;
}

#[cfg(feature = "v4")]
extern "C" {
	pub fn napi_create_threadsafe_function(
		env: napi_env,
		func: napi_value,
		async_resource: napi_value,
		async_resource_name: napi_value,
		max_queue_size: size_t,
		initial_thread_count: size_t,
		thread_finalize_data: *mut c_void,
		thread_finalize_cb: napi_finalize,
		context: *mut c_void,
		call_js_cb: napi_threadsafe_function_call_js,
		result: *mut napi_threadsafe_function,
	) -> napi_status;
	pub fn napi_get_threadsafe_function_context(
		func: napi_threadsafe_function,
		result: *mut *mut c_void,
	) -> napi_status;
	pub fn napi_call_threadsafe_function(
		func: napi_threadsafe_function,
		data: *mut c_void,
		is_blocking: napi_threadsafe_function_call_mode,
	) -> napi_status;
	pub fn napi_acquire_threadsafe_function(func: napi_threadsafe_function) -> napi_status;
	pub fn napi_release_threadsafe_function(
		func: napi_threadsafe_function,
		mode: napi_threadsafe_function_release_mode,
	) -> napi_status;
	pub fn napi_unref_threadsafe_function(
		env: napi_env,
		func: napi_threadsafe_function,
	) -> napi_status;
	pub fn napi_ref_threadsafe_function(
		env: napi_env,
		func: napi_threadsafe_function,
	) -> napi_status;
}

#[cfg(feature = "v5")]
extern "C" {
	pub fn napi_create_date(env: napi_env, time: f64, result: *mut napi_value) -> napi_status;
	pub fn napi_is_date(env: napi_env, value: napi_value, is_date: *mut bool) -> napi_status;
	pub fn napi_get_date_value(env: napi_env, value: napi_value, result: *mut f64) -> napi_status;
	pub fn napi_add_finalizer(
		env: napi_env,
		js_object: napi_value,
		native_object: *mut c_void,
		finalize_cb: napi_finalize,
		finalize_hint: *mut c_void,
		result: *mut napi_ref,
	) -> napi_status;
}

#[cfg(feature = "v6")]
extern "C" {
	pub fn napi_create_bigint_int64(
		env: napi_env,
		value: i64,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_create_bigint_uint64(
		env: napi_env,
		value: u64,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_create_bigint_words(
		env: napi_env,
		sign_bit: c_int,
		word_count: size_t,
		words: *const u64,
		result: *mut napi_value,
	) -> napi_status;
	pub fn napi_get_value_bigint_int64(
		env: napi_env,
		value: napi_value,
		result: *mut i64,
		lossless: *mut bool,
	) -> napi_status;
	pub fn napi_get_value_bigint_uint64(
		env: napi_env,
		value: napi_value,
		result: *mut u64,
		lossless: *mut bool,
	) -> napi_status;
	pub fn napi_get_value_bigint_words(
		env: napi_env,
		value: napi_value,
		sign_bit: *mut c_int,
		word_count: *mut size_t,
		words: *mut u64,
	) -> napi_status;
	pub fn napi_set_instance_data(
		env: napi_env,
		data: *mut c_void,
		finalize_cb: napi_finalize,
		finalize_hint: *mut c_void,
	) -> napi_status;
	pub fn napi_get_instance_data(env: napi_env, data: *mut *mut c_void) -> napi_status;
	pub fn napi_get_all_property_names(
		env: napi_env,
		object: napi_value,
		key_mode: napi_key_collection_mode,
		key_filter: napi_key_filter,
		key_conversion: napi_key_conversion,
		result: *mut napi_value,
	) -> napi_status;
}

#[cfg(feature = "v7")]
extern "C" {
	pub fn napi_detach_arraybuffer(env: napi_env, arraybuffer: napi_value) -> napi_status;
	pub fn napi_is_detached_arraybuffer(
		env: napi_env,
		value: napi_value,
		result: *mut bool,
	) -> napi_status;
}
