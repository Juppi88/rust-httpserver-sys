extern crate gcc;

fn main()
{
	//println!("cargo:rustc-env=CFLAGS=-std=c99");

	gcc::Config::new()
		.flag("-std=gnu99")
		.file("httpserver/httpsocket.c")
		.file("httpserver/httputils.c")
		.file("httpserver/httpserver.c")
		.compile("libhttpserver.a"
	);

//		.compile_library(
//			"libhttpserver.a",
//			&["httpserver/httpsocket.c", "httpserver/httputils.c", "httpserver/httpserver.c"]
//	);
}
