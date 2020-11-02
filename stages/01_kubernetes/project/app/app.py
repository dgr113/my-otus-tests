# coding: utf-8

import os
from flask import Flask, jsonify

app = Flask(__name__)




@app.route('/health')
def health():
    return jsonify({'Status': 'OKK'})



@app.route('/')
def hello():
    return "Hello world from: {}".format( os.environ['HOSTNAME'] )




if __name__ == "__main__":
    app.run(host='0.0.0.0', port='80')
