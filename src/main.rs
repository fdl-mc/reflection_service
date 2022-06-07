#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(tonic::include_file_descriptor_set!("descriptor"))
        .build()
        .unwrap();

    let addr = "0.0.0.0:8000".parse().unwrap();

    tracing::info!(message = "Starting server.", %addr);

    tonic::transport::Server::builder()
        .trace_fn(|_| tracing::info_span!("reflection_service"))
        .add_service(reflection)
        .serve(addr)
        .await?;

    Ok(())
}
