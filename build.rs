extern crate gcc;

fn main()
{
	gcc::compile_library(
		"libhttpserver.a",
		&["httpserver/httpsocket.c", "httpserver/httputils.c", "httpserver/httpserver.c"]
	);
}
