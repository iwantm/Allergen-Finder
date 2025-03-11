variable "database_name" {
  description = "The name of the database"
  type        = string
}

variable "database_username" {
  description = "The username for the database"
  type        = string
}

variable "database_password" {
  description = "The password for the database"
  type        = string
}

variable "database_port" {
  description = "The port for the database"
  type        = number

}

variable "namespace" {
  description = "The namespace for the resources"
  type        = string
}

