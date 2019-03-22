#[macro_use]
extern crate log;
extern crate env_logger;
extern crate grpc;

mod gen;

use grpc::{RequestOptions, SingleResponse};
use gen::example_service::*;
use gen::example_service_grpc::*;

const PORT: u16 = 50051;

fn main() {
    let _ = env_logger::init();

    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(PORT);
    server.add_service(ExampleServiceServer::new_service_def(ExampleServiceImpl));
    let _server = server.build().expect("server");

    info!("service started on :{}", PORT);
    loop {
        std::thread::park();
    }
}

struct ExampleServiceImpl;

impl ExampleService for ExampleServiceImpl {
    fn hello(&self, _o: RequestOptions, req: HelloRequest) -> SingleResponse<HelloResponse> {
        let mut resp = HelloResponse::new();
        resp.set_message(format!("Hello {}", req.message));

        return SingleResponse::completed(resp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_succes() {
        let service = ExampleServiceImpl;

        let mut req = HelloRequest::new();
        req.set_message("my message".to_string());

        let resp = service.hello(RequestOptions::new(), req).wait_drop_metadata().unwrap();
        assert_eq!("Hello my message", resp.get_message());
    }
}
