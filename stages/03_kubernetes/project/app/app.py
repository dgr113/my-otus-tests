# coding: utf-8

import os
import json
from flask import Flask
from sqlalchemy import create_engine  # type: ignore

app = Flask(__name__)

config = {
    'DATABASE_URI': os.environ.get('DATABASE_URI', ''),
    'HOSTNAME': os.environ['HOSTNAME'],
    'GREETING': os.environ.get('GREETING', 'Hello'),
}

engine = create_engine(config['DATABASE_URI'], echo=True)



@app.route("/")
def hello():
    return config['GREETING'] + ' from ' + config['HOSTNAME'] + '!'


@app.route("/config")
def configuration():
    # config['version'] = 'NEW'
    return json.dumps(config)


@app.route('/db')
def db():
    rows = []
    with engine.connect() as connection:
        result = connection.execute("select id, name from client;")
        rows = [ dict(r.items()) for r in result ]
    return json.dumps(rows)



if __name__ == "__main__":
    app.run(host='0.0.0.0', port='80', debug=True)
