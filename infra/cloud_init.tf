locals {
  data_volume_path = "/mnt/master"
  data_volume_fs   = "ext4"
  pg_data          = "${local.data_volume_path}/data/postgres"
  env_file_path    = "/var/app/.env"
  bootstrap        = "${path.module}/bootstrap"

  template_files = {
    "umami.service"       = "/etc/systemd/system/umami.service"
    "data-volume.service" = "/etc/systemd/system/data-volume.service"
    "docker-daemon.json"  = "/etc/docker/daemon.json"
  }
  data_files = merge(
    {
      "/var/app/docker-compose.yml" = file("${path.module}/docker-compose.yml")
      (local.env_file_path)         = join("\n", [for k, v in local.env_vars : "${k}=${v}"])
    },
    {
      for source, target in local.template_files :
      target => templatefile("${local.bootstrap}/${source}", local.template_vars)
    }
  )

  exec_files = {
    for file in fileset(local.bootstrap, "*.sh") :
    "/var/app/${file}" => file("${local.bootstrap}/${file}")
  }

  files_by_perms = {
    "0444" = local.data_files
    "0555" = local.exec_files
  }

  template_vars = {
    env_file_path  = local.env_file_path
    server_os_user = local.server_os_user
    server_ip      = hcloud_floating_ip.master.ip_address

    ssh_public_key = chomp(file("~/.ssh/id_rsa.pub"))

    data_volume_device = hcloud_volume.master.linux_device
    data_volume_path   = local.data_volume_path
    data_volume_fs     = local.data_volume_fs
  }

  env_vars = {
    PG_PASSWORD      = var.pg_password
    PG_DATA          = local.pg_data
    DATA_VOLUME_PATH = local.data_volume_path
    UMAMI_APP_SECRET = var.umami_app_secret
  }
}

data "cloudinit_config" "master" {
  part {
    content = templatefile(
      "${path.module}/bootstrap/user_data.yml",
      merge(
        local.template_vars,
        {
          files = merge(
            flatten([
              for perms, files in local.files_by_perms : [
                for path, content in files : {
                  (path) = { content = base64gzip(content), perms = perms }
                }
              ]
            ])
            ...
          )
        }
      )
    )
  }
}
