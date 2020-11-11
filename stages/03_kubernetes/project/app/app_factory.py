# coding: utf-8

import os
from flask import Flask
from flask.json import JSONEncoder
from flask_restful import Api  # type: ignore
from .routes import Index, Config, StudentMany, StudentOne  # type: ignore
from .db_factory import db  # type: ignore




class CustomJSONEncoder(JSONEncoder):
    def default(self, obj):
        try:
            JSONEncoder.default(self, obj)
            return obj
        except Exception:
            return str(obj)



def build_db_uri() -> str:
    """ Get database URI from env variables """

    return "{DB_DRIVER}://{DB_USERNAME}:{DB_PASSWD}@{DB_HOST}:{DB_PORT}/{DB_NAME}".format(**{
        'DB_DRIVER': os.environ.get('DB_DRIVER', ''),
        'DB_HOST': os.environ.get('DB_HOST', ''),
        'DB_PORT': os.environ.get('DB_PORT', ''),
        'DB_NAME': os.environ.get('DB_NAME', ''),
        'DB_USERNAME': os.environ.get('DB_USERNAME', ''),
        'DB_PASSWD': os.environ.get('DB_PASSWD', '')
    })



def create_app(app_name: str):
    """ Construct the core application """

    app = Flask(app_name)
    app.json_encoder = CustomJSONEncoder

    app.config.update({
        'SQLALCHEMY_DATABASE_URI': build_db_uri(),
        'SQLALCHEMY_TRACK_MODIFICATIONS': os.environ.get('SQLALCHEMY_TRACK_MODIFICATIONS', False),
        'APP_CONFIG': {
            'HOSTNAME': os.environ.get('HOSTNAME', ''),
            'GREETING': os.environ.get('GREETING', 'Hello'),
        }
    })

    db.init_app(app)
    api = Api(app)

    with app.app_context():
        api.add_resource(Index, '/')
        api.add_resource(Config, '/config')
        api.add_resource(StudentMany, '/student')
        api.add_resource(StudentOne, '/student/<int:student_id>')
        return app
