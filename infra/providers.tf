provider "hcloud" {
  token = var.hcloud_token
}

terraform {
  # Make sure to keep it in sync with the version requirement on CI
  required_version = ">= 1.9"

  required_providers {
    hcloud = {
      source  = "hetznercloud/hcloud"
      version = "~> 1.48.1"
    }

    cloudinit = {
      source  = "hashicorp/cloudinit"
      version = "~> 2.3.5"
    }
  }
}
