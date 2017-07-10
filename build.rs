extern crate gcc;

fn main()
{
	gcc::compile_library(
		"libhttpserver.a",
		&["../lib/httpserver/httpsocket.c", "../lib/httpserver/httputils.c", "../lib/httpserver/httpserver.c"]
	);
}
