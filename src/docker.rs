use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DockerConfig {
    pub services: HashMap<String, ServiceDefinition>,
    pub networks: HashMap<String, Network>,
    pub volumes: HashMap<String, Volume>,
    pub dockerfile: Dockerfile,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_file: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dockerfile_actions: Option<Vec<Vec<String>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Service {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_file: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<String>>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Build {
    pub context: String,
    pub dockerfile: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Network {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Volume {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dockerfile {
    pub schema: Schema,
    pub execution_order: Vec<ExecutionOrder>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Schema {
    pub set_tag: TagStep,
    pub copy: CopyStep,
    pub set_workdir: WorkdirStep,
    pub run_command: CommandStep,
    pub set_entrypoint: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagStep {
    pub image: HashMap<String, Vec<String>>,
    pub stage: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CopyStep {
    pub current: HashMap<String, Vec<String>>,
    pub from: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkdirStep {
    pub step_1: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandStep {
    pub step_1: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecutionOrder {
    pub service: String,
    pub dockerfile: String,
    pub steps: Vec<String>,
}

#[derive(Serialize)]
struct ComposeFile<'a> {
    version: &'static str,
    services: HashMap<String, Service>,
    networks: &'a HashMap<String, Network>,
    volumes: &'a HashMap<String, Volume>,
}

impl From<ServiceDefinition> for Service {
    fn from(service_def: ServiceDefinition) -> Self {
        Service {
            image: service_def.image,
            build: service_def.build,
            ports: service_def.ports,
            env_file: service_def.env_file,
            networks: service_def.networks,
            depends_on: service_def.depends_on,
            restart: service_def.restart,
            volumes: service_def.volumes,
        }
    }
}

impl Into<ServiceDefinition> for HashMap<String, Service> {
    fn into(self) -> ServiceDefinition {
        let mut service_definition = ServiceDefinition {
            image: None,
            build: None,
            ports: None,
            env_file: None,
            networks: None,
            depends_on: None,
            restart: None,
            volumes: None,
            dockerfile_actions: Some(Vec::new()),
        };

        for (key, value) in self {
            match key.as_str() {
                "image" => service_definition.image = value.image,
                "build" => service_definition.build = value.build,
                "ports" => service_definition.ports = value.ports,
                "env_file" => service_definition.env_file = value.env_file,
                "networks" => service_definition.networks = value.networks,
                "depends_on" => service_definition.depends_on = value.depends_on,
                "restart" => service_definition.restart = value.restart,
                "volumes" => service_definition.volumes = value.volumes,
                "dockerfile_actions" => service_definition.dockerfile_actions = Some(Vec::new()),
                _ => {}
            }
        }

        service_definition
    }
}

impl Into<Service> for HashMap<String, ServiceDefinition> {
    fn into(self) -> Service {
        let mut service = Service {
            image: None,
            build: None,
            ports: None,
            env_file: None,
            networks: None,
            depends_on: None,
            restart: None,
            volumes: None,
        };

        for (key, value) in self {
            match key.as_str() {
                "image" => service.image = value.image,
                "build" => service.build = value.build,
                "ports" => service.ports = value.ports,
                "env_file" => service.env_file = value.env_file,
                "networks" => service.networks = value.networks,
                "depends_on" => service.depends_on = value.depends_on,
                "restart" => service.restart = value.restart,
                "volumes" => service.volumes = value.volumes,
                _ => {}
            }
        }

        service
    }
}

impl DockerConfig {
    pub fn generate_dockerfile_actions(
        service_name: &str,
        service: &ServiceDefinition,
    ) -> Vec<Vec<String>> {
        let mut actions = Vec::new();

        // Check if specific dockerfile actions are provided
        if let Some(service_actions) = &service.dockerfile_actions {
            return service_actions.clone();
        }

        // Use image specified in the service definition or default to "rust:latest"
        let image = service.image.as_deref().unwrap_or("rust:latest");

        // Start with the initial FROM instruction for the builder stage if applicable
        actions.push(vec!["FROM".to_string(), format!("{} as builder", image)]);
        actions.push(vec![
            "WORKDIR".to_string(),
            format!("/usr/src/{}", service_name),
        ]);

        // Add COPY and RUN instructions for building the project
        actions.push(vec![
            "COPY".to_string(),
            "Cargo.toml Cargo.lock ./".to_string(),
        ]);
        actions.push(vec!["COPY".to_string(), "src ./src".to_string()]);
        actions.push(vec![
            "RUN".to_string(),
            format!("cargo build --release --bin {}", service_name),
        ]);

        // Add the final FROM instruction for the runtime stage
        actions.push(vec!["FROM".to_string(), image.to_string()]);
        actions.push(vec![
            "WORKDIR".to_string(),
            format!("/usr/src/{}", service_name),
        ]);
        actions.push(vec![
            "COPY".to_string(),
            format!(
                "--from=builder /usr/src/{}/target/release/{} /usr/local/bin/{}",
                service_name, service_name, service_name
            ),
        ]);

        // Add additional COPY and ENTRYPOINT instructions
        actions.push(vec!["COPY".to_string(), ".env ./".to_string()]);
        actions.push(vec!["COPY".to_string(), "../config.json ./".to_string()]);
        actions.push(vec![
            "ENTRYPOINT".to_string(),
            format!("[\"{}\"]", service_name),
        ]);

        actions
    }

    pub fn generate_dockerfile_schema_and_order(&mut self) {
        let services = &self.services;
        let mut changes = Vec::new();

        let mut dockerfile_schema = Schema {
            set_tag: TagStep {
                image: HashMap::new(),
                stage: HashMap::new(),
            },
            copy: CopyStep {
                current: HashMap::new(),
                from: HashMap::new(),
            },
            set_workdir: WorkdirStep { step_1: Vec::new() },
            run_command: CommandStep { step_1: Vec::new() },
            set_entrypoint: Vec::new(),
        };

        let mut execution_order = Vec::new();

        for (service_name, service) in services {
            if service.build.is_some() {
                let dockerfile_path = service.build.as_ref().unwrap().dockerfile.clone();
                let mut steps_required = Vec::new();

                // Generate dockerfile actions dynamically
                let dockerfile_actions =
                    DockerConfig::generate_dockerfile_actions(service_name, service);

                // Store changes to be made after the loop
                changes.push((service_name.clone(), dockerfile_actions));

                // Add required steps for the service
                dockerfile_schema
                    .set_tag
                    .image
                    .entry("step_1".to_string())
                    .or_insert_with(Vec::new)
                    .push(format!("Setting tag for {}", service_name));
                dockerfile_schema
                    .set_workdir
                    .step_1
                    .push(format!("Setting workdir for {}", service_name));
                dockerfile_schema
                    .copy
                    .current
                    .entry("step_1".to_string())
                    .or_insert_with(Vec::new)
                    .push(format!("Copying files for {}", service_name));
                dockerfile_schema
                    .run_command
                    .step_1
                    .push(format!("Running commands for {}", service_name));
                dockerfile_schema
                    .set_entrypoint
                    .push(format!("Setting entrypoint for {}", service_name));

                steps_required.extend_from_slice(&[
                    "set_tag".to_string(),
                    "set_workdir".to_string(),
                    "copy".to_string(),
                    "run_command".to_string(),
                    "set_entrypoint".to_string(),
                ]);

                execution_order.push(ExecutionOrder {
                    service: service_name.clone(),
                    dockerfile: dockerfile_path,
                    steps: steps_required,
                });
            }
        }

        // Apply changes after the loop to avoid borrow checker issues
        for (service_name, dockerfile_actions) in changes {
            if let Some(service) = self.services.get_mut(&service_name) {
                service.dockerfile_actions = Some(dockerfile_actions);
            }
        }

        // Remove empty step levels
        dockerfile_schema.set_tag.image.retain(|_, v| !v.is_empty());
        dockerfile_schema.set_tag.stage.retain(|_, v| !v.is_empty());
        dockerfile_schema.copy.current.retain(|_, v| !v.is_empty());
        dockerfile_schema.copy.from.retain(|_, v| !v.is_empty());
        dockerfile_schema
            .set_workdir
            .step_1
            .retain(|v| !v.is_empty());
        dockerfile_schema
            .run_command
            .step_1
            .retain(|v| !v.is_empty());

        self.dockerfile = Dockerfile {
            schema: dockerfile_schema,
            execution_order,
        };
    }

    pub fn save_docker_compose(&self) {
        let services: HashMap<String, Service> = self
            .services
            .clone()
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect();

        let compose_file = ComposeFile {
            version: "3.8",
            services,
            networks: &self.networks,
            volumes: &self.volumes,
        };

        let compose_yaml = serde_yaml::to_string(&compose_file).unwrap();
        fs::write("docker-compose.yml", compose_yaml).expect("Failed to write to file");
    }

    pub fn generate_and_save_dockerfiles(&self) {
        for (service_name, service) in &self.services {
            if let Some(actions) = &service.dockerfile_actions {
                let dockerfile_content: Vec<String> = actions
                    .iter()
                    .map(|action| format!("{} {}", action[0], action[1]))
                    .collect();
                let dockerfile_path = format!("./Docker/Dockerfile.{}", service_name);
                fs::write(dockerfile_path, dockerfile_content.join("\n"))
                    .expect("Failed to write Dockerfile");
            }
        }
    }

    pub fn print_docker_compose(&self) {
        let services: HashMap<String, Service> = self
            .services
            .clone()
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect();

        let compose_file = ComposeFile {
            version: "3.8",
            services,
            networks: &self.networks,
            volumes: &self.volumes,
        };

        let compose = serde_yaml::to_string(&compose_file).unwrap();
        println!("{}", compose);
    }

    pub fn print_dockerfiles(&self) {
        for (_service_name, service) in &self.services {
            if let Some(actions) = &service.dockerfile_actions {
                for action in actions {
                    println!("{} {}", action[0], action[1]);
                }
                println!();
            }
        }
    }
}

impl From<&str> for DockerConfig {
    fn from(json_str: &str) -> Self {
        serde_json::from_str(json_str).expect("Failed to parse JSON")
    }
}

impl Into<Value> for DockerConfig {
    fn into(self) -> Value {
        serde_json::to_value(self).expect("Failed to convert to JSON")
    }
}



pub fn read_config_from_file(file_path: &str) -> Result<DockerConfig, Box<dyn Error>> {
    let config_json = fs::read_to_string(file_path)?;
    let config: DockerConfig = DockerConfig::from(config_json.as_str());
    Ok(config)
}
