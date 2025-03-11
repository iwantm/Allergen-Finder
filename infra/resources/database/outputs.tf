output "database_address" {
  value = "${kubernetes_service.database.metadata[0].name}.${var.namespace}.svc:${var.database_port}"

}
