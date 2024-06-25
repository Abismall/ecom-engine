use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};
use ecom_engine::docker::DockerConfig;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;

#[derive(Parser)]
#[command(name = "Dockerfile management tool")]
#[command(about = "A tool to run various commands related to the application", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "menu")]
    command: String,
}

fn main() {
    let cli = Cli::parse();

    if cli.command == "menu" {
        menu()
    } else {
        process_command(cli.command)
    }
}

fn process_command(command: String) {
    if command == "show_compose" {
        load_docker_config().print_docker_compose();
    } else if command == "show_dockerfiles" {
        load_docker_config().print_dockerfiles();
    } else if command == "generate_schema" {
        generate_docker_file_schema();
    } else if command == "generate_files" {
        generate_docker_files();
    } else if command == "compose_up_build" {
        docker_compose_up_build();
    } else if command == "compose_up" {
        docker_compose_up();
    } else if command == "compose_build" {
        docker_compose_build();
    } else {
        eprintln!("Error: invalid command: {}", command);
    }
}

fn menu() {
    let options = &[
        "Show compose",
        "Show dockerfiles",
        "Generate schema",
        "Generate files",
        "Compose up build",
        "Compose up",
        "Compose build",
        "Exit",
    ];

    match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an action")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap()
    {
        0 => load_docker_config().print_docker_compose(),
        1 => load_docker_config().print_dockerfiles(),
        2 => generate_docker_file_schema(),
        3 => generate_docker_files(),
        4 => docker_compose_up_build(),
        5 => docker_compose_up(),
        6 => docker_compose_build(),
        7 => exit(0),
        _ => println!("Unknown command received"),
    }
}

fn load_docker_config() -> DockerConfig {
    let current_dir = env::current_dir().unwrap();
    let config_path: PathBuf = [current_dir.to_str().unwrap(), "docker", "cfg.json"]
        .iter()
        .collect();
    let config_json = fs::read_to_string(config_path).unwrap();
    let config: DockerConfig = DockerConfig::from(config_json.as_str());
    config
}

fn generate_docker_file_schema() {
    let mut config = load_docker_config();
    config.generate_dockerfile_schema_and_order();

    let output_path = PathBuf::from("./docker/generated_docker_schema.json");
    fs::write(output_path, serde_json::to_string_pretty(&config).unwrap())
        .expect("Failed to write Docker schema");
    println!("Docker file schema generated successfully");
}

fn generate_docker_files() {
    let config = load_docker_config();
    config.generate_and_save_dockerfiles();
    config.save_docker_compose();
}

fn docker_compose_up_build() {
    println!("Running: docker-compose up --build");

    if Command::new("docker-compose")
        .arg("up")
        .arg("--build")
        .status()
        .expect("Failed to execute command")
        .success()
    {
        println!("Docker Compose built and started successfully");
    } else {
        eprintln!("Failed to build and start Docker Compose");
    }
}

fn docker_compose_up() {
    println!("Running: docker-compose up");

    if Command::new("docker-compose")
        .arg("up")
        .status()
        .expect("Failed to execute command")
        .success()
    {
        println!("Docker Compose started successfully");
    } else {
        eprintln!("Failed to start Docker Compose");
    }
}

fn docker_compose_build() {
    println!("Running: docker-compose build");

    if Command::new("docker-compose")
        .arg("build")
        .status()
        .expect("Failed to execute command")
        .success()
    {
        println!("Docker Compose built successfully");
    } else {
        eprintln!("Failed to build Docker Compose");
    }
}
