variable "allowed_ssh_ips" {
  nullable  = false
  type      = list(string)
  sensitive = true
}

variable "pg_password" {
  nullable  = false
  type      = string
  sensitive = true
}

variable "umami_app_secret" {
  nullable  = false
  type      = string
  sensitive = true
}

variable "hcloud_token" {
  nullable  = false
  type      = string
  sensitive = true
}
