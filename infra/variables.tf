variable "database_name" {
  description = "The name of the database"
  type        = string
  default     = "allergens"
}
variable "database_username" {
  description = "The username for the database"
  type        = string
  default     = "api_user"

}

variable "database_password" {
  description = "The password for the database"
  type        = string

}

variable "database_port" {
  description = "The port for the database"
  type        = string
  default     = "5432"

}

variable "namespace" {
  description = "The namespace for the resources"
  type        = string
  default     = "allergens"
}

variable "registry_namespace" {
  description = "The namespace for the registry"
  type        = string
  default     = "registry"
}
