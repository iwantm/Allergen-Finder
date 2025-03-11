provider "kubernetes" {
  config_path = "~/.kube/config"
}

provider "docker" {
  host = "unix:///var/run/docker.sock"
  registry_auth {
    address = "http://localhost:30100"
  }
}

terraform {
  required_providers {
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "2.36.0"
    }

    docker = {
      source  = "kreuzwerker/docker"
      version = "3.0.2"
    }
  }
}

