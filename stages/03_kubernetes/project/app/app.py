# coding: utf-8

import os
import json
from ast import literal_eval

from flask import Flask, jsonify
from flask_restful import Api, Resource  # type: ignore
from sqlalchemy import create_engine  # type: ignore


app = Flask(__name__)
api = Api(app)

app_config = {
    'DATABASE_URI': os.environ.get('DATABASE_URI', ''),
    'HOSTNAME': os.environ.get('HOSTNAME', ''),
    'GREETING': os.environ.get('GREETING', 'Hello'),
}

engine = create_engine(app_config['DATABASE_URI'], echo=True)




class Index(Resource):
    def get(self):
        return "{} from {}!".format(app_config['GREETING'], app_config['HOSTNAME'])


class Config(Resource):
    def get(self):
        return jsonify(app_config)


class StudentList(Resource):
    def get(self) -> str:
        with engine.connect() as connection:
            sql_expr = "SELECT id, name FROM client;"
            result = connection.execute(sql_expr)
            rows = [ dict(r.items()) for r in result ]
        return json.dumps(rows)


class Student(Resource):
    def get(self, student_id: str) -> str:
        with engine.connect() as connection:
            rec_id = literal_eval(student_id)
            sql_expr = "SELECT id, name FROM client WHERE id = {};".format( rec_id )
            result = connection.execute(sql_expr)
            rows = [ dict(r.items()) for r in result ]
        return json.dumps(rows)

    def delete(self, student_id: str) -> str:
        with engine.connect() as connection:
            rec_id = literal_eval(student_id)
            sql_expr = "DELETE FROM client WHERE id = {};".format( rec_id )
            connection.execute(sql_expr)
        return json.dumps({'status': 200})




if __name__ == "__main__":
    api.add_resource(Index, '/')
    api.add_resource(Config, '/config')
    api.add_resource(StudentList, '/student')
    api.add_resource(Student, '/student/<string:student_id>')
    app.run(host='0.0.0.0', port='80', debug=True)
