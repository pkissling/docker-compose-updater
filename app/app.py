from flask import Flask
import os
app = Flask(__name__)

@app.route('/')
def hello_world():
    os.system('docker-compose -f ../resources/docker-compose.yaml up')
    return 'Hello, World!'

