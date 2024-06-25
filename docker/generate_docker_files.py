import json
import os

from typing import Any, Dict, List

from shared import COMPOSE_FILE, DEFAULT_FORMAT, OUT_DIR, CFG


def dict_to_yaml_str(d: Dict[str, Any], indent: int = 0) -> List[str]:
    """Convert a dictionary to a YAML-formatted string."""
    lines = []
    for key, value in d.items():
        if isinstance(value, dict):
            lines.append(f"{' ' * indent}{key}:")
            lines.extend(dict_to_yaml_str(value, indent + 2))
        elif isinstance(value, list):
            lines.append(f"{' ' * indent}{key}:")
            for item in value:
                if isinstance(item, dict):
                    lines.extend(dict_to_yaml_str(item, indent + 2))
                else:
                    lines.append(f"{' ' * (indent + 2)}- {item}")
        else:
            lines.append(f"{' ' * indent}{key}: {value}")
    return lines   

def file_write(path: str, data: Any, mode: str = "w") -> None:
    """Write data to a file."""
    with open(path, mode) as f:
        f.write(data)

def build_compose_json(config: Dict[str, Any]) -> Dict[str, Any]:
    """Build the Docker Compose configuration from the given config."""
    services = config.get("services", {})
    networks = config.get("networks", {})
    volumes = config.get("volumes", {})

    # Remove 'dockerfile_actions' from each service
    filtered_services = {}
    for service_name, service in services.items():
        filtered_services[service_name] = {k: v for k, v in service.items() if k != "dockerfile_actions"}

    compose = {
        "version": "3.8",
        "services": filtered_services,
        "networks": networks,
        "volumes": volumes
    }
   
    return compose

def create_docker_compose(compose: Dict[str, Any], outfile: str = COMPOSE_FILE, outdir: str = OUT_DIR, format: str = DEFAULT_FORMAT) -> None:
    """Create the Docker Compose file in the specified format."""
    os.makedirs(outdir, exist_ok=True)
    if format == "yaml":
        yaml_str = "\n".join(dict_to_yaml_str(compose))
        file_write(os.path.join(outdir, outfile), yaml_str)
    elif format == "json":
        with open(os.path.join(outdir, outfile), "w") as f:
            json.dump(compose, f, indent=2)
    else:
        raise ValueError(f"File format is not supported: '{format}'")

def create_dockerfile(service_name: str, service: Dict[str, Any], outdir: str = OUT_DIR) -> None:
    """Create a Dockerfile for the given service."""
    dockerfile_path = service["build"]["dockerfile"]
    dockerfile_content = generate_dockerfile_content(service.get("dockerfile_actions", []), service_name)
    file_write(path=dockerfile_path, data=dockerfile_content)
    print(f"{service_name} Dockerfile created -> {dockerfile_path}")  

def process_action(action: str, params: str, service_name: str) -> str:
    """Process a Dockerfile action."""
    if action == "FROM":
        return f"FROM {params}"
    elif action == "WORKDIR":
        return f"WORKDIR {params}"
    elif action == "COPY":
        return f"COPY {params}"
    elif action == "RUN":
        return f"RUN {params.format(service_name=service_name)}"
    elif action == "ENTRYPOINT":
        return f"ENTRYPOINT {params.format(service_name=service_name)}"
    else:
        raise ValueError(f"Unknown action: {action}")

def generate_dockerfile_content(actions: List[List[str]], service_name: str) -> str:
    """Generate the content of a Dockerfile from actions."""
    lines = []
    for action, params in actions:
        lines.append(process_action(action, params, service_name))
    return "\n".join(lines)

def main() -> None:
    """Main function to generate Docker Compose and Dockerfiles."""
    try:
        cfg = CFG()
        create_docker_compose(compose=build_compose_json(cfg.config))
        
        services = cfg.get_value("services")
        for service_name, service in services.items():
            if "build" in service:
                create_dockerfile(service_name=service_name, service=service)
    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    main()
