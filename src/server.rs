use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::calculator_server::{Calculator, CalculatorServer};
use hello_world::{HelloReply, HelloRequest, CalcRequest, CalcReply};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default, Debug)]
pub struct MyGreeter {}

#[derive(Default, Debug)]
pub struct MyCalculator {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tonic::async_trait]
impl Calculator for MyCalculator {
    async fn add(
        &self,
        request: Request<CalcRequest>,
    ) -> Result<Response<CalcReply>, Status> {
        let r = request.into_inner();
        let reply = hello_world::CalcReply { result: r.a + r.b };
        Ok(Response::new(reply))
    }

    async fn subtract(
        &self,
        request: Request<CalcRequest>,
    ) -> Result<Response<CalcReply>, Status> {
        let r = request.into_inner();
        let reply = hello_world::CalcReply { result: r.a - r.b };
        Ok(Response::new(reply))
    }

    async fn multiply(
        &self,
        request: Request<CalcRequest>,
    ) -> Result<Response<CalcReply>, Status> {
        let r = request.into_inner();
        let reply = hello_world::CalcReply { result: r.a * r.b };
        Ok(Response::new(reply))
    }

    async fn divide(
        &self,
        request: Request<CalcRequest>,
    ) -> Result<Response<CalcReply>, Status> {
        let r = request.into_inner();
        if r.b == 0 {
            return Err(Status::invalid_argument("division by zero"));
        }
        let reply = hello_world::CalcReply { result: r.a / r.b };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();
    let calculator = MyCalculator::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(CalculatorServer::new(calculator))
        .serve(addr)
        .await?;

    Ok(())
}
