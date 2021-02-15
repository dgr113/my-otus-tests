# coding: utf-8

from flask import Flask
from flask.json import JSONEncoder
from flask_restful import Api  # type: ignore
from .routes import Index, UserOne  # type: ignore




class CustomJSONEncoder( JSONEncoder ):
    def default(self, obj):
        try:
            JSONEncoder.default(self, obj)
            return obj
        except Exception:
            return str(obj)



def create_app( app_name ):
    """ Construct the core application """

    app = Flask( app_name )
    app.json_encoder = CustomJSONEncoder

    api = Api( app )

    with app.app_context():
        api.add_resource(Index, '/')
        api.add_resource(UserOne, '/users/me/')
        return app
