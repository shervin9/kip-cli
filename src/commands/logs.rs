use kube::{Client, api::{ListParams, LogParams}, Api};
use k8s_openapi::api::core::v1::Pod;
use regex::Regex;
use std::fs::File;
use std::io::Write;

pub async fn fetch_logs(
    deployment_name: &str,
    keyword: Option<&str>,
    output_file: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);

    let lp = LogParams {
        follow: false,
        ..Default::default()
    };

    println!("Fetching logs for deployment: {}", deployment_name);
    let list_params = ListParams::default().labels(&format!("app={}", deployment_name));
    let mut combined_logs = String::new();

    for pod in pods.list(&list_params).await? {
        if let Some(pod_name) = pod.metadata.name {
            let log = pods.logs(&pod_name, &lp).await?;
            if let Some(kw) = keyword {
                let re = Regex::new(kw)?;
                for line in log.lines() {
                    if re.is_match(line) {
                        combined_logs.push_str(&format!("[{}]: {}\n", pod_name, line));
                    }
                }
            } else {
                for line in log.lines() {
                    combined_logs.push_str(&format!("[{}]: {}\n", pod_name, line));
                }
            }
        }
    }

    if output_file {
        let file_name = format!("{}.txt", deployment_name);
        let mut file = File::create(&file_name)?;
        file.write_all(combined_logs.as_bytes())?;
        println!("Logs saved to file: {}", file_name);
    } else {
        println!("{}", combined_logs);
    }

    Ok(())
}