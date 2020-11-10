# coding: utf-8

import os
from flask import Flask
from flask_restful import Api  # type: ignore
from .routes import IndexRoutes, ConfigRoutes, StudentListRoutes, StudentRoutes  # type: ignore
from .db_factory import db  # type: ignore




def build_db_uri() -> str:
    """ Get database URI from env variables """
    db_config = {
        'DB_DRIVER': os.environ.get('DB_DRIVER', ''),
        'DB_HOST': os.environ.get('DB_HOST', ''),
        'DB_PORT': os.environ.get('DB_PORT', ''),
        'DB_NAME': os.environ.get('DB_NAME', ''),
        'DB_USERNAME': os.environ.get('DB_USERNAME', ''),
        'DB_PASSWD': os.environ.get('DB_PASSWD', '')
    }
    return "{DB_DRIVER}://{DB_USERNAME}:{DB_PASSWD}@{DB_HOST}:{DB_PORT}/{DB_NAME}".format(**db_config)



def create_app(app_name: str):
    """ Construct the core application """
    app = Flask(app_name)

    app.config.update({
        'HOSTNAME': os.environ.get('HOSTNAME', ''),
        'GREETING': os.environ.get('GREETING', 'Hello'),
        'SQLALCHEMY_DATABASE_URI': build_db_uri(),
        'SQLALCHEMY_TRACK_MODIFICATIONS': os.environ.get('SQLALCHEMY_TRACK_MODIFICATIONS', False)
    })

    db.init_app(app)
    api = Api(app)

    with app.app_context():
        api.add_resource(IndexRoutes, '/')
        api.add_resource(ConfigRoutes, '/config')
        api.add_resource(StudentListRoutes, '/student')
        api.add_resource(StudentRoutes, '/student/<int:student_id>')
        return app
