use crate::metadata;
use crate::metadata::EndpointMetadata;
use crate::MescCliError;
use futures::stream::FuturesUnordered;
use futures::stream::StreamExt;
use tokio::task::JoinHandle;

pub(crate) async fn ping_command() -> Result<(), MescCliError> {
    let config_data = mesc::load::load_config_data()?;
    let mut tasks = FuturesUnordered::<JoinHandle<_>>::new();

    for endpoint in config_data.endpoints.into_values() {
        let url = endpoint.url.clone();
        let task: JoinHandle<(String, Result<EndpointMetadata, MescCliError>)> =
            tokio::spawn(async move {
                let result = metadata::get_node_metadata(url.clone()).await;
                (endpoint.name, result)
            });
        tasks.push(task);
    }
    let mut metadatas = vec![];
    let mut failed_endpoints = vec![];
    while let Some(result) = tasks.next().await {
        match result {
            Ok((_name, Ok(metadata))) => metadatas.push(metadata),
            Ok((name, Err(_))) => failed_endpoints.push(name),
            Err(e) => return Err(MescCliError::JoinError(e)),
        }
    }

    println!("collected metadata for {} endpoints", metadatas.len());
    println!("failed collection for {} endpoints", failed_endpoints.len());
    Ok(())
}
