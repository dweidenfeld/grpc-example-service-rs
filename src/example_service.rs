use crate::gen::example_service::{HelloRequest, HelloResponse};
use crate::gen::example_service_grpc::ExampleService;
use grpc::{RequestOptions, SingleResponse};

pub struct ExampleServiceImpl;

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
    use crate::gen::example_service_grpc::{ExampleServiceClient, ExampleServiceServer};
    use grpc::{ClientStub, Server};
    use rand::{thread_rng, Rng};
    use std::sync::Arc;

    fn setup() -> (Server, ExampleServiceClient) {
        let port = thread_rng().gen_range(49152, 65535);

        let mut server = grpc::ServerBuilder::new_plain();
        server.http.set_port(port);
        server.add_service(ExampleServiceServer::new_service_def(ExampleServiceImpl));
        let server = server.build().expect("server");

        let grpc_client = Arc::new(
            grpc::Client::new_plain("127.0.0.1", port, Default::default()).expect("grpc client"),
        );

        (server, ExampleServiceClient::with_client(grpc_client))
    }

    #[test]
    fn hello_success() {
        let (server, client) = setup();

        let mut req = HelloRequest::new();
        req.set_message("my message".to_string());

        assert!(server.is_alive());
        let resp: HelloResponse = client
            .hello(RequestOptions::new(), req)
            .wait_drop_metadata()
            .unwrap();
        assert_eq!("Hello my message", resp.get_message());
    }
}
