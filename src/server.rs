use proto::calculator_server::CalculatorServer;

mod proto {
    tonic::include_proto!("calculator");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("calculator_descriptor");
}

#[derive(Debug, Default)]
struct CalculatorService {}

#[tonic::async_trait]
impl proto::calculator_server::Calculator for CalculatorService {
    async fn add(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> anyhow::Result<
        tonic::Response<proto::CalculationResponse>,
        tonic::Status,
    > {
        eprintln!("Received 'add' request: {:?}", request);

        let input = request.get_ref();
        eprintln!("Calculating sum: {} + {}", input.a, input.b);

        let response = proto::CalculationResponse {
            response: input.a + input.b,
        };

        eprintln!("Sending response: {:?}", response);

        Ok(tonic::Response::new(response))
    }
    async fn divide(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> anyhow::Result<
        tonic::Response<proto::CalculationResponse>,
        tonic::Status,
    > {
        eprintln!("Received 'divide' request: {:?}", request);

        let input = request.get_ref();

        if input.b == 0 {
            eprintln!("Division by zero is not allowed");
            return Err(tonic::Status::invalid_argument(
                "Division by zero is not allowed",
            ));
        }

        eprintln!("Calculating quotient: {} / {}", input.a, input.b);

        let response = proto::CalculationResponse {
            response: input.a / input.b,
        };

        eprintln!("Sending response: {:?}", response);

        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    eprintln!("Starting CalculatorService");

    let addr = "[::1]:50051".parse()?;
    eprintln!("Parsed address: {:?}", addr);

    let calc = CalculatorService::default();

    eprintln!("Setting up reflection service");

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1alpha()?;

    eprintln!("Reflection service registered successfully");

    eprintln!("Starting tonic server");
    tonic::transport::Server::builder()
        .add_service(reflection)
        .add_service(CalculatorServer::new(calc))
        .serve(addr)
        .await?;

    eprintln!("Server is running");

    Ok(())
}
