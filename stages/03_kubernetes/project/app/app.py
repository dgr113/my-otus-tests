# coding: utf-8

import os
import json
from flask import Flask, jsonify
from sqlalchemy import create_engine  # type: ignore

app = Flask(__name__)

app_config = {
    'DATABASE_URI': os.environ.get('DATABASE_URI', ''),
    'HOSTNAME': os.environ.get('HOSTNAME', ''),
    'GREETING': os.environ.get('GREETING', 'Hello'),
}

engine = create_engine(app_config['DATABASE_URI'], echo=True)




@app.route("/")
def index():
    return "{} from {}!".format(app_config['GREETING'], app_config['HOSTNAME'])


@app.route("/config")
def config():
    return jsonify(app_config)


@app.route('/db')
def db():
    with engine.connect() as connection:
        result = connection.execute("select id, name from client;")
        rows = [ dict(r.items()) for r in result ]
    return json.dumps(rows)




if __name__ == "__main__":
    app.run(host='0.0.0.0', port='80', debug=True)
