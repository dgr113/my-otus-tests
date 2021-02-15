# coding: utf-8

from typing import Sequence, Optional, Callable
from flask import jsonify, current_app, request, abort
from flask_restful import Resource  # type: ignore

from .models import Users  # type: ignore
from .jwt import JwtUtils




def check_auth(func: Callable) -> Callable:
    def wrapped(_self, user_id: Optional[int], *args, **kwargs):
        signed_user_data = {
            'id': request.headers.get('X-User-Id', None),
            'name': request.headers.get('X-User-Name', None)
        }
        signer_user_id = signed_user_data['id']
        if ( user_id is not None ) or ( user_id == signer_user_id ):
            return func(signer_user_id, *args, **kwargs)
        else:
            # Logging an unauthorized access attempt
            abort(403, "Wrong user auth data")

    return wrapped




class Index( Resource ):
    def get( self ) -> dict:
        return jsonify({ 'result': "OK" })



class Keys(Resource):
    def get( self ) -> dict:
        public_key = current_app.config['APP_CONFIG']['PUBLIC_KEY']
        result = JwtUtils.create_jwks_keyset( public_key )
        return jsonify( result )



class UserLogin( Resource ):
    def post(self) -> dict:
        req_data = request.get_json()
        if user := Users.find_one( req_data ):
            private_key = current_app.config['APP_CONFIG']['PRIVATE_KEY']
            id_token = JwtUtils.create_id_token(private_key, user)
            return jsonify({ "IDtoken": id_token })
        else:
            abort(401)



class UserRegister( Resource ):
    def post(self) -> dict:
        req_data = request.get_json()
        if user_id := Users.insert( req_data ):
            return jsonify({ 'id': user_id })
        else:
            abort(400, "Username already exists")



class UserActions(Resource):
    @check_auth
    def get(self, user_id: Optional[int] = None) -> Sequence[dict]:
        return Users.get_one( user_id )

    @check_auth
    def put(self, user_id: Optional[int] = None) -> str:
        req_data = request.get_json()
        return Users.update(user_id, req_data)

    @check_auth
    def delete(self, user_id: Optional[int] = None) -> int:
        return Users.delete(user_id)
