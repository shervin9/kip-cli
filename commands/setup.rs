use std::process::Command;

pub fn setup_provider(provider: &str) -> Result<(), Box<dyn std::error::Error>> {
    match provider {
        "gke" => setup_gke()?,
        _ => println!("Unsupported provider: {}", provider),
    }
    Ok(())
}

fn setup_gke() -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting up GKE...");
    Command::new("gcloud")
        .args(&["auth", "login"])
        .status()?;
    Command::new("gcloud")
        .args(&["container", "clusters", "get-credentials", "CLUSTER_NAME", "--region", "REGION"])
        .status()?;
    println!("GKE setup complete!");
    Ok(())
}