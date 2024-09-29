use proto::calculator_server::{Calculator, CalculatorServer};

mod proto {
    tonic::include_proto!("calculator");
}
#[derive(Debug, Default)]

struct CalculatorService {}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> anyhow::Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
        println!("Got a request:{:?}", request);
        let input = request.get_ref();
        let response = proto::CalculationResponse {
            response: input.a + input.b,
        };

        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse()?;
    let calc = CalculatorService::default();
    println!("Bout to run this bs");
    tonic::transport::Server::builder()
        .add_service(CalculatorServer::new(calc))
        .serve(addr)
        .await?;

    Ok(())
}
