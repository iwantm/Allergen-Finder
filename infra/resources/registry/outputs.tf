output "registry_address" {
  value = "localhost:${kubernetes_service.registry.spec[0].port[0].node_port}"
}
