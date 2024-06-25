import os
import glob
from typing import Any, Dict
import json


# Constants
SCHEMA_OUT_PATH = "./docker/cfg.json"
CFG_FILE = "docker/cfg.json"
OUT_DIR = "docker/"
DEFAULT_FORMAT = "json"
COMPOSE_FILE = "compose.json"

class CFG:
    def __init__(self, outfile: str = COMPOSE_FILE) -> None:
        self.compose_file = outfile
        self.path = os.path.dirname(os.path.abspath(__file__))
        self.cfg_file = self.find_config_file()
        self.config = self.load_config()
        self.services = self.load_services()

    def find_config_file(self) -> str:
        """Find the configuration file in the specified directory."""
        config_file = None
        # Search for JSON files in the specified directory
        for json_file in glob.glob(os.path.join(self.path, "*.json")):
            if not json_file.endswith(self.compose_file):
                config_file = json_file
                break

        # If no valid JSON file is found, raise an error
        if not config_file:
            raise FileNotFoundError("No valid JSON configuration file found in the 'docker' directory.")
        else:
            return config_file

    def load_config(self) -> Dict[str, Any]:
        """Load the configuration from the config file."""
        with open(self.cfg_file) as f:
            return json.load(f)

    def load_services(self) -> Dict[str, Any]:
        """Load the services from the configuration."""
        services = self.get_value("services")
        if not services:
            raise ValueError("No services found in JSON configuration file.")
        else:
            return services

    def get_value(self, key: str) -> Any:
        """Get a value from the configuration using the specified key."""
        return self.config.get(key, None)
    
    def __str__(self) -> str:
        return f"CFG(path={self.path}, config_file={self.cfg_file})"