use kube::{Client, Api};
use kube::api::ListParams;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Pod;
use prettytable::{Table, format, row};
use colored::*;

pub async fn list_pods() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);

    println!("{}", "Listing Pods:".green());

    for p in pods.list(&ListParams::default()).await? {
        let pod_name = p.metadata.name.unwrap_or_else(|| "Unknown".to_string());
        let status = p
            .status
            .as_ref()
            .and_then(|s| s.phase.clone())
            .unwrap_or_else(|| "Unknown".to_string());
        let node_name = p
            .spec
            .as_ref()
            .and_then(|spec| spec.node_name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        println!("Pod: {}, Status: {}, Node: {}", pod_name, status, node_name);
    }
    Ok(())
}

pub async fn list_deployments() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    let deployments: Api<Deployment> = Api::default_namespaced(client);

    println!("{}", "Fetching deployments...".green());

    let dp_list = deployments.list(&ListParams::default()).await?;
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.add_row(row!["NAME".yellow(), "IMAGE".cyan(), "REPLICAS".magenta()]);

    for dp in dp_list {
        let name = dp.metadata.name.unwrap_or_default();
        let replicas = dp
            .spec
            .as_ref()
            .and_then(|spec| spec.replicas)
            .map(|r| r.to_string())
            .unwrap_or_else(|| "N/A".to_string());
        let containers = dp
            .spec
            .as_ref()
            .and_then(|spec| spec.template.spec.as_ref())
            .map(|spec| {
                spec.containers
                    .iter()
                    .map(|c| c.image.clone().unwrap_or_else(|| "Unknown".to_string()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_else(Vec::new);

        let image_str = containers.join(", ");
        table.add_row(row![name.blue(), image_str.white(), replicas.green()]);
    }
    table.printstd();
    Ok(())
}