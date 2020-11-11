# coding: utf-8

from typing import Sequence
from flask import jsonify, current_app, request
from flask_restful import Resource  # type: ignore
from .models import Client  # type: ignore




class Index(Resource):
    def get(self):
        return jsonify({
            'result': "{} from {}!".format(current_app.config['APP_CONFIG']['GREETING'], current_app.config['APP_CONFIG']['HOSTNAME'])
        })



class Config(Resource):
    def get(self):
        return jsonify( current_app.config['APP_CONFIG'] )



class StudentMany(Resource):
    def get(self) -> Sequence[dict]:
        return Client.find()

    def post(self) -> str:
        req_data = request.get_json()
        return Client.insert(req_data)



class StudentOne(Resource):
    def get(self, student_id: int) -> Sequence[dict]:
        return Client.find(student_id)

    def put(self, student_id: int):
        req_data = request.get_json()
        return Client.update(student_id, req_data)

    def delete(self, student_id: int):
        return Client.delete(student_id)
