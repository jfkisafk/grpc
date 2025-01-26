use proto::calculator_server::{Calculator, CalculatorServer};
use tonic::transport::Server;

mod proto {
    tonic::include_proto!("calculator");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("calculator_descriptor");
}

#[derive(Debug, Default)]
struct CalculatorService {}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<proto::CalculationResult>, tonic::Status> {
        let req = request.get_ref();
        let result = req.a + req.b;
        Ok(tonic::Response::new(proto::CalculationResult { result }))
    }

    async fn subtract(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
        ) -> Result<tonic::Response<proto::CalculationResult>, tonic::Status> {
        let req = request.get_ref();
        let result = req.a - req.b;
        Ok(tonic::Response::new(proto::CalculationResult { result }))
    }

    async fn multiply(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<proto::CalculationResult>, tonic::Status> {
        let req = request.get_ref();
        let result = req.a * req.b;
        Ok(tonic::Response::new(proto::CalculationResult { result }))
    }

    async fn divide(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<proto::CalculationResult>, tonic::Status> {
        let req = request.get_ref();
        if req.b == 0 {
            return Err(tonic::Status::invalid_argument("Cannot divide by zero"));
        }
        let result = req.a / req.b;
        Ok(tonic::Response::new(proto::CalculationResult { result }))
    }

    async fn sum(
        &self,
        request: tonic::Request<proto::NumbersRequest>,
    ) -> Result<tonic::Response<proto::CalculationResult>, tonic::Status> {
        let req = request.get_ref();
        let result = req.numbers.iter().sum();
        Ok(tonic::Response::new(proto::CalculationResult { result }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::0]:8080".parse()?;

    let calc = CalculatorService::default();

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    Server::builder()
        .add_service(service)
        .add_service(CalculatorServer::new(calc))
        .serve(addr)
        .await?;

    Ok(())
}
