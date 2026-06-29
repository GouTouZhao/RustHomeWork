use std::env;

pub struct ServiceDiscovery {
    pub namespace: String,
}

impl ServiceDiscovery {
    pub fn new(namespace: String) -> Self {
        Self { namespace }
    }

    pub fn get_service_url(&self, service_name: &str) -> String {
        // If in kubernetes, return k8s dns format
        if let Ok(env) = env::var("RUNTIME_ENV") {
            if env == "KUBERNETES" {
                return format!("http://{}.{}.svc.cluster.local:8080", service_name, self.namespace);
            }
        }
        
        // Local fallback
        format!("http://localhost:8080")
    }
}
