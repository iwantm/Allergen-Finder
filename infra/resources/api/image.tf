resource "docker_image" "api-image" {
  name = "${var.registry_address}/api"
  build {
    context = "${path.root}/../backend-rs"
  }
  triggers = {
    dir_sha1 = sha1(join("", [for f in fileset("${path.root}/../backend-rs", "src/**/*") : filesha1("${path.root}/../backend-rs/${f}")]))
  }
}

resource "docker_registry_image" "api-image" {
  name                 = docker_image.api-image.name
  insecure_skip_verify = true
}
