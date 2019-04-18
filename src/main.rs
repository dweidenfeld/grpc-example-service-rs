#[macro_use]
extern crate log;

mod gen;

use env_logger;
use gen::example_service::*;
use gen::example_service_grpc::*;
use grpc::{RequestOptions, SingleResponse};

fn main() {
    env_logger::init();

    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(50051);
    server.add_service(ExampleServiceServer::new_service_def(ExampleServiceImpl));
    let server = server.build().expect("server");

    ctrlc::set_handler(move || {
        info!("exiting...");
        std::process::exit(0);
    }).expect("SIGTERM handler");
    info!("service started on {}", server.local_addr());
    std::thread::park();
}

struct ExampleServiceImpl;

impl ExampleService for ExampleServiceImpl {
    fn hello(&self, _o: RequestOptions, req: HelloRequest) -> SingleResponse<HelloResponse> {
        let mut resp = HelloResponse::new();
        resp.set_message(format!("Hello {}", req.message));

        SingleResponse::completed(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_success() {
        let service = ExampleServiceImpl;

        let mut req = HelloRequest::new();
        req.set_message("my message".to_string());

        let resp = service
            .hello(RequestOptions::new(), req)
            .wait_drop_metadata()
            .unwrap();
        assert_eq!("Hello my message", resp.get_message());
    }
}
