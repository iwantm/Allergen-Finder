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

variable "api_port" {
  description = "The port for the API"
  type        = number
}

variable "registry_address" {
  description = "The address for the registry"
  type        = string
}

variable "database_address" {
  description = "The address for the database"
  type        = string
}

variable "auth0_domain" {
  description = "The domain for Auth0"
  type        = string
}

variable "auth0_audience" {
  description = "The audience for Auth0"
  type        = string
}
