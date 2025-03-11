resource "kubernetes_deployment" "api" {
  depends_on = [docker_registry_image.api-image]
  metadata {
    name      = "api"
    namespace = var.namespace
  }

  spec {
    replicas = 1

    selector {
      match_labels = {
        app = "api"
      }
    }

    template {
      metadata {
        labels = {
          app = "api"
        }
      }

      spec {
        container {
          name  = "api"
          image = docker_image.api-image.name
          port {
            container_port = var.api_port
          }
          env {
            name  = "ROCKET_DATABASES"
            value = "{database={url=\"postgres://${var.database_username}:${var.database_password}@${var.database_address}/${var.database_name}\"}}"
          }
        }
      }
    }
  }
}
