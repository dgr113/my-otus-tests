# coding: utf-8

import os
from flask import Flask, jsonify

app = Flask(__name__)




@app.route('/')
def index():
    return "Hello from: {}".format( os.environ['HOSTNAME'] )



@app.route('/health')
def health():
    return jsonify({'Status': 'OK'})




if __name__ == "__main__":
    app.run(host='0.0.0.0', port='80')
