use anyhow::Result;
use proto::calculator_client::CalculatorClient;

pub mod proto {
    tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "http://[::1]:50051";
    let mut client = CalculatorClient::connect(addr).await?;

    let req = proto::CalculationRequest { a: 3, b: 4 };
    let request = tonic::Request::new(req);

    let response = client.add(request).await?;

    println!("RESPONSE={:?}", response.get_ref().response);

    let req2 = proto::CalculationRequest { a: 10, b: 0 };
    let request2 = tonic::Request::new(req2);

    let response2 = client.divide(request2).await?;

    println!("RESPONSE={:?}", response2.get_ref().response);
    Ok(())
}
