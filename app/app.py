from flask import Flask, request
import docker
import os

app = Flask(__name__)

EXPECTED_API_KEY = os.environ['API_KEY']
docker_client = docker.from_env()

@app.route('/docker/containers/<container_name>/update', methods=['PUT'])
def update_container(container_name):

    # actual_api_key = request.headers.get('X-API-Key')
    # if actual_api_key != EXPECTED_API_KEY:
    #     return {}, 401

    # try:
    #     docker_client.containers.get(container_name)
    # except docker.errors.NotFound:
    #     return EXPECTED_API_KEY, 404

    os.system('docker-compose --file /resources/docker-compose.yaml pull {}'.format(container_name))
    os.system('docker stop {}'.format(container_name))
    os.system('docker rm {}'.format(container_name))
    os.system('docker-compose --file /resources/docker-compose.yaml up --detach {}'.format(container_name))
    os.system('docker image prune --force')
    return {}, 200

