# coding: utf-8

from os import environ as env

from flask import Flask
from flask.json import JSONEncoder
from flask_restful import Api  # type: ignore

from .routes import Index, Keys, UserActions, UserLogin, UserRegister  # type: ignore
from .db_factory import db  # type: ignore




class CustomJSONEncoder( JSONEncoder ):
    def default(self, obj):
        try:
            JSONEncoder.default(self, obj)
            return obj
        except Exception:
            return str(obj)



def build_db_uri() -> str:
    """ Get database URI from env variables """

    return "{DB_DRIVER}://{DB_USERNAME}:{DB_PASSWD}@{DB_HOST}:{DB_PORT}/{DB_NAME}".format(**{
        'DB_DRIVER': env.get('DB_DRIVER', ''),
        'DB_HOST': env.get('DB_HOST', ''),
        'DB_PORT': env.get('DB_PORT', ''),
        'DB_NAME': env.get('DB_NAME', ''),
        'DB_USERNAME': env.get('DB_USERNAME', ''),
        'DB_PASSWD': env.get('DB_PASSWD', '')
    })



def create_app( app_name: str ) -> Flask:
    """ Construct the core application """

    app = Flask( app_name )
    app.json_encoder = CustomJSONEncoder

    app.config.update({
        'SQLALCHEMY_DATABASE_URI': build_db_uri(),
        'SQLALCHEMY_TRACK_MODIFICATIONS': env.get('SQLALCHEMY_TRACK_MODIFICATIONS', False),
        'APP_CONFIG': {
            'HOSTNAME': env.get('HOSTNAME', ''),
            'GREETING': env.get('GREETING', ''),
            'PUBLIC_KEY': env.get('PUBLIC_KEY', ''),
            'PRIVATE_KEY': env.get('PRIVATE_KEY', '')
        }
    })

    db.init_app( app )
    api = Api( app )

    with app.app_context():
        api.add_resource(Index, '/')
        api.add_resource(Keys, '/keys/')
        api.add_resource(UserLogin, '/login/')
        api.add_resource(UserRegister, '/register/')
        api.add_resource(UserActions, '/users/<int:user_id>/', '/users/')
        return app
