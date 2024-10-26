output "server_ip" {
  value = hcloud_floating_ip.master.ip_address
}

output "server_status" {
  value = hcloud_server.master.status
}

output "data_volume_path" {
  value = local.data_volume_path
}

output "server_os_user" {
  value = local.server_os_user
}
