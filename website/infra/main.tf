locals {
  location = "fsn1"

  hostname = "hetzner-master"

  # XXX: using the name `admin` for the user is a bad idea. It does seem to work
  # fine on Hetzner. However, when using Oracle Cloud, it was found that `admin`
  # user name causes the server to be inaccessible via SSH. The supposition is
  # that there is a conflict with the `admin` group name already present in
  # the used Oracle Ubuntu AMI.
  server_os_user = "master"
}

resource "hcloud_floating_ip" "master" {
  type          = "ipv4"
  home_location = local.location
}

resource "hcloud_floating_ip_assignment" "master" {
  floating_ip_id = hcloud_floating_ip.master.id
  server_id      = hcloud_server.master.id
}

resource "hcloud_server" "master" {
  name         = local.hostname
  image        = "ubuntu-24.04"
  server_type  = "cax21"
  location     = local.location
  user_data    = data.cloudinit_config.master.rendered
  firewall_ids = [hcloud_firewall.master.id]

  public_net {
    ipv4_enabled = true
    ipv6_enabled = true
  }
}

resource "hcloud_volume" "master" {
  name     = "master"
  size     = 50
  location = local.location
}

resource "hcloud_volume_attachment" "master" {
  server_id = hcloud_server.master.id
  volume_id = hcloud_volume.master.id

  # Automount doesn't work if server's cloud-init script contains `runcmd` module
  # <https://github.com/hetznercloud/terraform-provider-hcloud/issues/473#issuecomment-971535629>
  # instead we use systemd mount unit via fstab
  automount = false
}

# HACK: we need to gracefully shutdown our systemd service with the database
# docker container before the data volume is detached. This null resource
# depends on the volume attachment resource, so the remote-exec provisioner
# teardown script will be run before the attachment is destroyed.
#
# Unfortunately, it's not possible to do this with `systemd`. The volume detach
# sequence is undocumented in Hetzner docs. One would expect that all `systemd`
# services dependent upon the volume's mount are stopped before the volume
# is detached but this isn't true.
#
# The reality is cruel. It was experimentally found that the volume is
# detached abruptly. Therefore the database doesn't have time to flush its data
# to disk, which means potential corruption or even data loss.
resource "terraform_data" "teardown" {
  triggers_replace = [
    hcloud_volume_attachment.master.id
  ]

  input = {
    server_ip      = hcloud_server.master.ipv4_address
    server_os_user = local.server_os_user
  }


  provisioner "remote-exec" {
    when = destroy

    inline = [
      <<-SCRIPT
      #!/usr/bin/env bash
      set -euo pipefail
      sudo systemctl stop umami.service
      SCRIPT
    ]

    connection {
      host = self.input.server_ip
      user = self.input.server_os_user
    }
  }
}

resource "hcloud_firewall" "master" {
  name = "allow-inbound"
  rule {
    direction  = "in"
    protocol   = "tcp"
    port       = "22"
    source_ips = var.allowed_ssh_ips
  }
  rule {
    direction = "in"
    protocol  = "tcp"
    port      = "80"
    source_ips = [
      "0.0.0.0/0",
      "::/0"
    ]
  }
}
