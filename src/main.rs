use crate::messages::{Input, Output, Grant};
extern crate actix;
extern crate actix_protobuf;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate prost;
#[macro_use]
extern crate prost_derive;

use actix_protobuf::*;
use actix_web::*;

pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/paybase.pip.get_policy.rs"));
}


fn index(msg: ProtoBuf<Input>) -> Result<HttpResponse> {
    println!("model: {:?}", msg);
    let resp = Output {
        id: "id".into(),
        name: "name".into(),
        grants: vec![
            Grant {
                source: "WHERE 1 = 1".into(),
                filter: "filter".into(),
                select: "select".into(),
            }
        ]
    };
    HttpResponse::Ok().protobuf(resp) // <- send response
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();
    let sys = actix::System::new("protobuf-example");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::post().to(index)))
    }).bind("127.0.0.1:8081")
    .unwrap()
    .shutdown_timeout(1)
    .start();

    println!("Started http server: 127.0.0.1:8081");
    let _ = sys.run();
}
