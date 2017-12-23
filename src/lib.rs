#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]

extern crate libc;

use libc::{ c_char, size_t };
use std::ptr;
use std::ffi::CString;

#[repr(C)]
pub struct ServerSettings {
	handler: HttpCallback,
	port: u16,
	max_connections: u16,
	timeout: u32,
	connection_timeout: u32,
	directories: *const ServerDirectory,
	directories_len: size_t,
}

#[repr(C)]
pub struct ServerDirectory {
	path: *const c_char,
	directory: *const c_char,
}

#[repr(C)]
enum HttpMessage {
	HTTP_200_OK,
	HTTP_400_BAD_REQUEST,
	HTTP_401_UNAUTHORIZED,
	HTTP_404_NOT_FOUND,
}

#[repr(C)]
pub struct HttpRequest {
	requester: *const c_char,
	method: *const c_char,
	request: *const c_char,
}

#[repr(C)]
pub struct HttpResponse {
	message: HttpMessage,
	content: *const c_char,
	content_type: *const c_char,
	content_length: usize,
}

type HttpCallback = extern fn(request: *const HttpRequest/*, target: *mut HttpServer*/) -> HttpResponse;

#[link(name="httpserver", kind="static")]
extern {
	fn http_server_initialize(configuration: ServerSettings) -> bool;
	fn http_server_shutdown();
	fn http_server_listen();
}

pub struct HttpServer {
	port: u16,
	max_connections: u16,
	timeout: u32,
	connection_timeout: u32,
	directories: Vec<ServerDirectory>,
}

impl HttpServer
{
	pub fn new() -> HttpServer
	{
		let server = Box::new(
			HttpServer {
				port: 80,
				max_connections: 10,
				timeout: 0,
				connection_timeout: 60,
				directories: Vec::new(),
		});

		return *server;
	}

	pub fn port(mut self, port: u16) -> HttpServer
	{
		self.port = port;
		self
	}

	pub fn max_connections(mut self, connections: u16) -> HttpServer
	{
		self.max_connections = connections;
		self
	}

	pub fn socket_timeout(mut self, millisec: u32) -> HttpServer
	{
		self.timeout = millisec;
		self
	}

	pub fn client_timeout(mut self, seconds: u32) -> HttpServer
	{
		self.connection_timeout = seconds;
		self
	}

	pub fn directory(mut self, path: &str, directory: &str) -> HttpServer
	{
		self.directories.push(
			ServerDirectory {
				path: CString::new(path).unwrap().as_ptr(),
				directory: CString::new(directory).unwrap().as_ptr(),
			});

		self
	}

	pub fn start(self) -> HttpServer
	{
		let config = ServerSettings {
			handler: HttpServer::callback,
			port: self.port,
			max_connections: self.max_connections,
			timeout: self.timeout,
			connection_timeout: self.connection_timeout,
			directories: self.directories.as_ptr(),
			directories_len: self.directories.len(),
		};

		unsafe {
			http_server_initialize(config);
		}

		self
	}

	pub fn listen(&self)
	{
		unsafe {
			http_server_listen();
		}
	}

	extern "C" fn callback(request: *const HttpRequest) -> HttpResponse
	//, target: *mut HttpServer
	{
		HttpResponse {
			message: HttpMessage::HTTP_200_OK,
			content: ptr::null(),
			content_type: ptr::null(),
			content_length: 0,
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
