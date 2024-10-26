# umami backend

This directory contains the deployment code for our [umami](https://umami.is/) backend used for collecting anonymous statics about the usage of our documentation website. The code for this lives here in the open for the sake of transparency and sharing (in case if you want to self-host your own umami instance on Hetzner).

It is a simple Hetzner VPS that runs a docker-compose cluster with the umami service and Postgres. The data is stored on a separate volume. The server is allocated a static IPv4.
