#[macro_use]
extern crate log;

mod example_service;
mod gen;

use crate::example_service::ExampleServiceImpl;
use crate::gen::example_service_grpc::ExampleServiceServer;
use env_logger;

#[cfg_attr(tarpaulin, skip)]
fn main() {
    env_logger::init();

    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(50051);
    server.add_service(ExampleServiceServer::new_service_def(ExampleServiceImpl));
    let server = server.build().expect("server");

    ctrlc::set_handler(move || {
        info!("exiting...");
        std::process::exit(0);
    })
    .expect("SIGTERM handler");
    info!("service started on {}", server.local_addr());
    std::thread::park();
}
