A naive Docker container exposing an API to update Docker Containers with the latest image using a given `docker-compose.yaml`.
This container is used to update [this](https://github.com/pkissling/home-assistant) Docker container running Home-Assistant. [Here](https://github.com/pkissling/home-assistant/blob/master/switches/updater.yaml) you can find the corresponding caller script.

```bash
# Build the latest image from Dockerfile
$ docker-compose --file test/docker/docker-compose.yaml up --detach --build

# Call API and update container 'hello-world' with latest image
$ curl -X PUT \
    -H 'X-API-Key:SUPER_SECRET' \
    localhost:5000/docker/containers/hello-world/update
```