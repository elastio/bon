# umami backend

This directory contains the deployment code for our [umami](https://umami.is/) backend used for collecting anonymous statics about the usage of our documentation website. This code lives here in the open for the sake of transparency and sharing (in case if you want to self-host your own umami instance on Hetzner).

It is a simple Hetzner VPS that runs a docker-compose cluster with the umami service and Postgres. The data is stored on a separate volume. The server is allocated a static IPv4.

## Deployment

Prerequisites:
- [Terraform CLI](https://developer.hashicorp.com/terraform/install)
- [Account at hetzner.com/cloud](https://hetzner.com/cloud)

Create a `terraform.tfvars` file in this directory. Here is an example below, make sure to replace all `{...}` placeholders with your values.

```tf
hcloud_token = "{token_value}"

allowed_ssh_ips = ["{your_ip_here}/32"]

pg_password = "{pg_password_value}"

umami_app_secret = "{umami_app_secret_value}"
```

Initialize terraform plugin directory and modules:

```bash
terraform init
```

Run the deployment:

```bash
terraform apply
```
