#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]

extern crate libc;

use libc::c_char;
use std::ptr;
use std::ffi::CString;

#[repr(C)]
pub struct HttpRequest {
	method: *const c_char,
	request: *const c_char,
	protocol: *const c_char,
	hostname: *const c_char,
}

#[repr(C)]
pub struct HttpResponse {
	message: HttpMessage,
	content: *const c_char,
	content_type: *const c_char,
}

#[repr(C)]
enum HttpMessage {
	HTTP_200_OK,
	HTTP_400_BAD_REQUEST,
	HTTP_401_UNAUTHORIZED,
	HTTP_404_NOT_FOUND,
	NUM_MESSAGES
}

type HttpCallback = extern fn(request: *const HttpRequest, target: *mut HttpServer) -> HttpResponse;

#[link(name="httpserver", kind="static")]
extern {
	fn http_server_initialize(port: u16, callback_handler: HttpCallback, target: *mut HttpServer) -> bool;
	fn http_server_shutdown();
	fn http_server_listen();
	fn http_server_add_static_directory(path: *const c_char, directory: *const c_char);
}

#[repr(C)]
pub struct HttpServer {
	port: u16,
}

impl HttpServer
{
	pub fn new(port: u16) -> HttpServer
	{
		let mut server = Box::new(HttpServer {
			port: port,
		});

		unsafe {
			http_server_initialize(port, HttpServer::callback, &mut *server);
		}

		return *server;
	}

	pub fn listen(&self)
	{
		unsafe {
			http_server_listen();
		}
	}

	pub fn add_static_directory(&self, path: &str, directory: &str)
	{
		let path_str = CString::new(path).unwrap();
		let directory_str = CString::new(directory).unwrap();

		let directory_ptr = directory_str.as_ptr();
		let path_ptr = path_str.as_ptr();

		unsafe {
			http_server_add_static_directory(path_ptr, directory_ptr);
		}
	}

	extern "C" fn callback(request: *const HttpRequest, target: *mut HttpServer) -> HttpResponse
	{
		/*println!("I'm called from C with value {0}", a);
		unsafe {
			// Update the value in RustObject with the value received from the callback:
			(*target).a = a;
		}
		*/
		HttpResponse {
			message: HttpMessage::HTTP_200_OK,
			content: ptr::null(),
			content_type: ptr::null(),
		}
	}
}

impl Drop for HttpServer
{
	fn drop(&mut self)
	{
		unsafe {
			http_server_shutdown();
		}
	}
}
