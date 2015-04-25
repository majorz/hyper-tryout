extern crate hyper;

use std::io::Write;
use std::path::Path;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;

fn hello(_: Request, res: Response<Fresh>) {
   let mut res = res.start().unwrap();
   res.write_all(b"Hello World!").unwrap();
   res.end().unwrap();
}

fn main() {
   let server = Server::https(
      hello,
      Path::new("/opt/ssl/site.crt"),
      Path::new("/opt/ssl/site.key"),
   ).listen("0.0.0.0:443").unwrap();
}
