# coding: utf-8

from typing import Sequence
from flask import jsonify, current_app
from flask_restful import Resource  # type: ignore
from .models import Client  # type: ignore




class IndexRoutes(Resource):
    def get(self):
        return "{} from {}!".format(current_app.config['GREETING'], current_app.config['HOSTNAME'])


class ConfigRoutes(Resource):
    def get(self):
        return jsonify(current_app.config)


class StudentListRoutes(Resource):
    def get(self) -> Sequence[dict]:
        return Client.find()


class StudentRoutes(Resource):
    def get(self, student_id: int) -> Sequence[dict]:
        return Client.find(student_id)

    def delete(self, student_id: int):
        return Client.delete(student_id)
