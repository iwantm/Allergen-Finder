resource "kubernetes_namespace" "allergens" {
  metadata {
    name = var.namespace
  }
}

module "database" {
  source            = "./resources/database"
  namespace         = kubernetes_namespace.allergens.metadata[0].name
  database_name     = var.database_name
  database_username = var.database_username
  database_password = var.database_password
  database_port     = var.database_port
}

module "registry" {
  source    = "./resources/registry"
  namespace = var.registry_namespace
}

module "api" {
  source            = "./resources/api"
  namespace         = kubernetes_namespace.allergens.metadata[0].name
  api_port          = 8080
  database_name     = var.database_name
  database_username = var.database_username
  database_password = var.database_password
  database_port     = var.database_port
  auth0_domain      = var.auth0_domain
  auth0_audience    = var.auth0_audience
  registry_address  = module.registry.registry_address
  database_address  = module.database.database_address
  providers = {
    docker = docker
  }
}


