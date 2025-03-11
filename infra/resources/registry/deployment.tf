resource "kubernetes_namespace" "registry" {
  metadata {
    name = var.namespace
  }
}

# resource "kubernetes_persistent_volume_claim" "registry_volume" {
#   metadata {
#     name      = "registry-volume"
#     namespace = kubernetes_namespace.registry.metadata[0].name
#   }

#   spec {
#     access_modes = ["ReadWriteOnce"]
#     resources {
#       requests = {
#         storage = "1Gi"
#       }
#     }
#   }

# }

resource "kubernetes_deployment" "docker-registry" {
  metadata {
    name      = "registry-deployment"
    namespace = kubernetes_namespace.registry.metadata[0].name
  }

  spec {
    replicas = 1

    selector {
      match_labels = {
        app = "docker-registry"
      }
    }

    template {
      metadata {
        name = "registry"
        labels = {
          app = "docker-registry"
        }
      }

      spec {
        container {
          name              = "registry-container"
          image             = "registry:2"
          image_pull_policy = "IfNotPresent"
          # volume_mount {
          #   mount_path = "/var/lib/registry"
          #   name       = "registry-volume"
          # }
          port {
            container_port = 5000
          }
        }
        # volume {
        #   name = "registry-volume"
        #   persistent_volume_claim {
        #     claim_name = kubernetes_persistent_volume_claim.registry_volume.metadata[0].name
        #   }
        # }
      }
    }
  }
}

resource "kubernetes_service" "registry" {
  metadata {
    name      = "registry"
    namespace = kubernetes_namespace.registry.metadata[0].name
  }

  spec {
    selector = {
      app = "docker-registry"
    }
    type = "NodePort"

    port {
      port        = 5000
      target_port = 5000
      node_port   = 30100
    }
  }

}
