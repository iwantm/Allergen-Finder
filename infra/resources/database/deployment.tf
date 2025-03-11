resource "kubernetes_deployment" "database" {
  metadata {
    name      = "database"
    namespace = var.namespace
  }

  spec {
    replicas = 1

    selector {
      match_labels = {
        app = "database"
      }
    }

    template {
      metadata {
        name = "database"
        labels = {
          app = "database"
        }
      }

      spec {
        container {
          name  = "database"
          image = "postgres:13"
          env {
            name  = "POSTGRES_DB"
            value = var.database_name
          }
          env {
            name  = "POSTGRES_USER"
            value = var.database_username
          }
          env {
            name  = "POSTGRES_PASSWORD"
            value = var.database_password
          }
          env {
            name  = "PGPORT"
            value = var.database_port
          }
          port {
            container_port = var.database_port
          }
        }
      }
    }
  }
}

resource "kubernetes_service" "database" {
  metadata {
    name      = "database"
    namespace = var.namespace
  }

  spec {
    selector = {
      app = "database"
    }
    type = "ClusterIP"

    port {
      port        = var.database_port
      target_port = var.database_port
    }
  }
}
