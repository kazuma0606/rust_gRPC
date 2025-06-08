use helloworld::greeter_client::GreeterClient;
use helloworld::calculator_client::CalculatorClient;
use helloworld::{HelloRequest, CalcRequest};

pub mod helloworld {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut greeter = GreeterClient::connect("http://[::1]:50051").await?;
    let mut calculator = CalculatorClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = greeter.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    let calc_request = tonic::Request::new(CalcRequest { a: 10, b: 5 });
    let add_response = calculator.add(calc_request).await?;
    println!("ADD RESPONSE={:?}", add_response);

    Ok(())
}
