use kube::{Client, api::{ListParams, LogParams}, Api};
use k8s_openapi::api::core::v1::Pod;
use regex::Regex;

pub async fn fetch_logs(deployment_name: &str, keyword: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);

    let lp = LogParams {
        follow: false,
        ..Default::default()
    };

    println!("Fetching logs for deployment: {}", deployment_name);
    let list_params = ListParams::default().labels(&format!("app={}", deployment_name));
    for p in pods.list(&list_params).await? {
        if let Some(pod_name) = p.metadata.name {
            let log = pods.logs(&pod_name, &lp).await?;
            if let Some(kw) = keyword {
                let re = Regex::new(kw)?;
                for line in log.lines() {
                    if re.is_match(line) {
                        println!("[{}]: {}", pod_name, line);
                    }
                }
            } else {
                println!("[{}]: {}", pod_name, log);
            }
        }
    }
    Ok(())
}