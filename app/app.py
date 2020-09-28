from flask import Flask
import os
app = Flask(__name__)

API_KEY = os.environ['API_KEY']

@app.route('/')
def hello_world():
    try:
        os.environ["API_KEY"]
    except KeyError:
        return "nope", 401
    os.system('docker-compose -f ../resources/docker-compose.yaml up')
    return 'Hello, World!'

