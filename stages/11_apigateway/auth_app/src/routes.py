# coding: utf-8

from typing import Sequence, Optional, Callable, Any
from flask import jsonify, current_app, request, abort
from flask_restful import Resource  # type: ignore

from .models import Users  # type: ignore
from .jwt import JwtUtils




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



class UserActions( Resource ):
    class Inner:
        @staticmethod
        def check_auth(func: Callable[..., Any]) -> Callable[..., Any]:
            """ Wrapper for verifying and passing trusted user data into wrapped function:
                1. Doesn't allow the absence of trusted headers (user_id)
                2. Compares user data and trusted data
                3. If there is no user data, passes trusted data into wrapped function
            """
            def wrapped(self, user_id: Optional[int], *args, **kwargs) -> Any:
                trusted_user_data = {
                    'id': request.headers.get('X-User-Id', None),
                    'name': request.headers.get('X-User-Name', None)
                }
                if ( trusted_user_id := trusted_user_data['id'] ) and ( user_id is None or user_id == trusted_user_id ):
                    return func(self, trusted_user_id, *args, **kwargs)
                else:
                    # Logging an unauthorized access attempt
                    abort(403, "Wrong user auth data")

            return wrapped

    @Inner.check_auth
    def get(self, user_id: Optional[int] = None) -> Sequence[dict]:
        return Users.get_one( user_id )

    @Inner.check_auth
    def put(self, user_id: Optional[int] = None) -> str:
        req_data = request.get_json()
        return Users.update(user_id, req_data)

    @Inner.check_auth
    def delete(self, user_id: Optional[int] = None) -> int:
        return Users.delete( user_id )
