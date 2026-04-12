use serde::{Deserialize, Serialize};
use std::process::Command;

fn kubectl(args: &[&str]) -> Result<String, String> {
    let output = Command::new("kubectl")
        .args(args)
        .output()
        .map_err(|e| format!("kubectl not found: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("kubectl error: {}", stderr));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

// ─── Types ───────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct K8sContext {
    pub name: String,
    pub cluster: String,
    pub user: String,
    pub namespace: String,
    pub is_current: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct K8sPod {
    pub name: String,
    pub namespace: String,
    pub status: String,
    pub ready: String,
    pub restarts: i64,
    pub age: String,
    pub node: String,
    pub containers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct K8sDeployment {
    pub name: String,
    pub namespace: String,
    pub ready: String,
    pub up_to_date: i64,
    pub available: i64,
    pub age: String,
    pub replicas: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct K8sService {
    pub name: String,
    pub namespace: String,
    pub svc_type: String,
    pub cluster_ip: String,
    pub external_ip: String,
    pub ports: String,
    pub age: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct K8sNamespace {
    pub name: String,
    pub status: String,
    pub age: String,
}

// ─── Commands ────────────────────────────────────────────

#[tauri::command]
pub fn k8s_is_available() -> bool {
    Command::new("kubectl")
        .arg("version")
        .arg("--client")
        .arg("--short")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[tauri::command]
pub fn k8s_get_contexts() -> Result<Vec<K8sContext>, String> {
    let output = kubectl(&["config", "get-contexts", "-o", "name"])?;
    let current = kubectl(&["config", "current-context"])
        .unwrap_or_default()
        .trim()
        .to_string();

    let mut contexts = Vec::new();

    for line in output.lines() {
        let name = line.trim().to_string();
        if name.is_empty() {
            continue;
        }

        let cluster = kubectl(&[
            "config", "view", "-o",
            &format!("jsonpath={{.contexts[?(@.name==\"{}\")].context.cluster}}", name),
        ]).unwrap_or_default().trim().to_string();

        let user = kubectl(&[
            "config", "view", "-o",
            &format!("jsonpath={{.contexts[?(@.name==\"{}\")].context.user}}", name),
        ]).unwrap_or_default().trim().to_string();

        let namespace = kubectl(&[
            "config", "view", "-o",
            &format!("jsonpath={{.contexts[?(@.name==\"{}\")].context.namespace}}", name),
        ]).unwrap_or_default().trim().to_string();

        contexts.push(K8sContext {
            is_current: name == current,
            cluster,
            user,
            namespace: if namespace.is_empty() { "default".to_string() } else { namespace },
            name,
        });
    }

    Ok(contexts)
}

#[tauri::command]
pub fn k8s_switch_context(context_name: String) -> Result<(), String> {
    kubectl(&["config", "use-context", &context_name])?;
    Ok(())
}

#[tauri::command]
pub fn k8s_get_namespaces() -> Result<Vec<K8sNamespace>, String> {
    let output = kubectl(&[
        "get", "namespaces", "-o",
        "jsonpath={range .items[*]}{.metadata.name}|{.status.phase}|{.metadata.creationTimestamp}{\"\\n\"}{end}",
    ])?;

    let mut namespaces = Vec::new();
    for line in output.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 3 {
            namespaces.push(K8sNamespace {
                name: parts[0].to_string(),
                status: parts[1].to_string(),
                age: format_age(parts[2]),
            });
        }
    }
    Ok(namespaces)
}

#[tauri::command]
pub fn k8s_get_pods(namespace: String) -> Result<Vec<K8sPod>, String> {
    let ns_arg = if namespace.is_empty() { "--all-namespaces".to_string() } else { format!("-n={}", namespace) };

    let output = kubectl(&[
        "get", "pods", &ns_arg, "-o", "json",
    ])?;

    let json: serde_json::Value = serde_json::from_str(&output)
        .map_err(|e| format!("JSON parse error: {}", e))?;

    let items = json["items"].as_array().ok_or("No items")?;
    let mut pods = Vec::new();

    for item in items {
        let metadata = &item["metadata"];
        let status = &item["status"];
        let spec = &item["spec"];

        let container_statuses = status["containerStatuses"].as_array();
        let total = container_statuses.map(|c| c.len()).unwrap_or(0);
        let ready_count = container_statuses
            .map(|cs| cs.iter().filter(|c| c["ready"].as_bool().unwrap_or(false)).count())
            .unwrap_or(0);

        let restarts: i64 = container_statuses
            .map(|cs| cs.iter().map(|c| c["restartCount"].as_i64().unwrap_or(0)).sum())
            .unwrap_or(0);

        let containers: Vec<String> = spec["containers"]
            .as_array()
            .map(|cs| cs.iter().filter_map(|c| c["name"].as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default();

        pods.push(K8sPod {
            name: metadata["name"].as_str().unwrap_or("").to_string(),
            namespace: metadata["namespace"].as_str().unwrap_or("").to_string(),
            status: status["phase"].as_str().unwrap_or("Unknown").to_string(),
            ready: format!("{}/{}", ready_count, total),
            restarts,
            age: format_age(metadata["creationTimestamp"].as_str().unwrap_or("")),
            node: spec["nodeName"].as_str().unwrap_or("").to_string(),
            containers,
        });
    }

    Ok(pods)
}

#[tauri::command]
pub fn k8s_get_deployments(namespace: String) -> Result<Vec<K8sDeployment>, String> {
    let ns_arg = if namespace.is_empty() { "--all-namespaces".to_string() } else { format!("-n={}", namespace) };

    let output = kubectl(&["get", "deployments", &ns_arg, "-o", "json"])?;

    let json: serde_json::Value = serde_json::from_str(&output)
        .map_err(|e| format!("JSON parse error: {}", e))?;

    let items = json["items"].as_array().ok_or("No items")?;
    let mut deployments = Vec::new();

    for item in items {
        let metadata = &item["metadata"];
        let status = &item["status"];
        let spec = &item["spec"];

        let replicas = spec["replicas"].as_i64().unwrap_or(0);
        let ready_replicas = status["readyReplicas"].as_i64().unwrap_or(0);
        let updated = status["updatedReplicas"].as_i64().unwrap_or(0);
        let available = status["availableReplicas"].as_i64().unwrap_or(0);

        deployments.push(K8sDeployment {
            name: metadata["name"].as_str().unwrap_or("").to_string(),
            namespace: metadata["namespace"].as_str().unwrap_or("").to_string(),
            ready: format!("{}/{}", ready_replicas, replicas),
            up_to_date: updated,
            available,
            age: format_age(metadata["creationTimestamp"].as_str().unwrap_or("")),
            replicas,
        });
    }

    Ok(deployments)
}

#[tauri::command]
pub fn k8s_get_services(namespace: String) -> Result<Vec<K8sService>, String> {
    let ns_arg = if namespace.is_empty() { "--all-namespaces".to_string() } else { format!("-n={}", namespace) };

    let output = kubectl(&["get", "services", &ns_arg, "-o", "json"])?;

    let json: serde_json::Value = serde_json::from_str(&output)
        .map_err(|e| format!("JSON parse error: {}", e))?;

    let items = json["items"].as_array().ok_or("No items")?;
    let mut services = Vec::new();

    for item in items {
        let metadata = &item["metadata"];
        let spec = &item["spec"];

        let ports: Vec<String> = spec["ports"]
            .as_array()
            .map(|ps| {
                ps.iter().map(|p| {
                    let port = p["port"].as_i64().unwrap_or(0);
                    let target = p["targetPort"].as_i64()
                        .map(|t| t.to_string())
                        .unwrap_or_else(|| p["targetPort"].as_str().unwrap_or("").to_string());
                    let protocol = p["protocol"].as_str().unwrap_or("TCP");
                    format!("{}:{}/{}", port, target, protocol)
                }).collect()
            })
            .unwrap_or_default();

        let external_ips: String = spec["externalIPs"]
            .as_array()
            .map(|ips| ips.iter().filter_map(|ip| ip.as_str()).collect::<Vec<_>>().join(","))
            .or_else(|| {
                item["status"]["loadBalancer"]["ingress"].as_array().map(|ings| {
                    ings.iter().filter_map(|i| i["ip"].as_str().or(i["hostname"].as_str())).collect::<Vec<_>>().join(",")
                })
            })
            .unwrap_or_default();

        services.push(K8sService {
            name: metadata["name"].as_str().unwrap_or("").to_string(),
            namespace: metadata["namespace"].as_str().unwrap_or("").to_string(),
            svc_type: spec["type"].as_str().unwrap_or("ClusterIP").to_string(),
            cluster_ip: spec["clusterIP"].as_str().unwrap_or("").to_string(),
            external_ip: if external_ips.is_empty() { "<none>".to_string() } else { external_ips },
            ports: ports.join(", "),
            age: format_age(metadata["creationTimestamp"].as_str().unwrap_or("")),
        });
    }

    Ok(services)
}

#[tauri::command]
pub fn k8s_scale_deployment(namespace: String, name: String, replicas: i64) -> Result<(), String> {
    kubectl(&[
        "scale", "deployment", &name,
        &format!("-n={}", namespace),
        &format!("--replicas={}", replicas),
    ])?;
    Ok(())
}

#[tauri::command]
pub fn k8s_restart_deployment(namespace: String, name: String) -> Result<(), String> {
    kubectl(&[
        "rollout", "restart", "deployment", &name,
        &format!("-n={}", namespace),
    ])?;
    Ok(())
}

#[tauri::command]
pub fn k8s_delete_pod(namespace: String, name: String) -> Result<(), String> {
    kubectl(&["delete", "pod", &name, &format!("-n={}", namespace)])?;
    Ok(())
}

fn format_age(timestamp: &str) -> String {
    if timestamp.is_empty() {
        return "Unknown".to_string();
    }
    let parsed = chrono::DateTime::parse_from_rfc3339(timestamp);
    match parsed {
        Ok(dt) => {
            let now = chrono::Utc::now();
            let duration = now.signed_duration_since(dt.with_timezone(&chrono::Utc));
            let secs = duration.num_seconds();
            if secs < 60 { return format!("{}s", secs); }
            if secs < 3600 { return format!("{}m", secs / 60); }
            if secs < 86400 { return format!("{}h", secs / 3600); }
            format!("{}d", secs / 86400)
        }
        Err(_) => timestamp.to_string(),
    }
}
