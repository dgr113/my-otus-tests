# coding: utf-8

from typing import Optional, Callable, Any
from flask import current_app, request, abort
from flask_restful import Resource  # type: ignore

from .models import Users
from .jwt import JwtUtils

RESTFUL_JSON_RESPONSE = tuple[dict, int]




class Index( Resource ):
    def get( self ) -> dict:
        return { 'status': "OK" }



class Keys(Resource):
    def get( self ) -> dict:
        public_key = current_app.config['APP_CONFIG']['PUBLIC_KEY']
        return JwtUtils.create_jwks_keyset( public_key )



class UserLogin( Resource ):
    def post(self) -> dict:
        req_data = request.get_json()
        if user := Users.find_one( req_data ):
            private_key = current_app.config['APP_CONFIG']['PRIVATE_KEY']
            id_token = JwtUtils.create_id_token(private_key, user)
            return { "IDtoken": id_token }
        else:
            abort(401, "Not found user with this request data")



class UserRegister( Resource ):
    def post(self) -> dict:
        req_data = request.get_json()
        if user_id := Users.insert( req_data ):
            return { 'id': user_id }
        else:
            abort(400, "Username already exists")



class UserActions( Resource ):
    class Inner:
        @staticmethod
        def wrap_auth(user_id_field: str, header_id_field: str) -> Callable[..., Any]:
            def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
                """ Wrapper for verifying and passing trusted user data into wrapped function:
                    1. Doesn't allow the absence of trusted headers (user_id)
                    2. Compares user data and trusted data
                    3. If there is no user data, passes trusted data into wrapped function
                """
                def wrapped(_self, *args, **kwargs) -> Any:
                    user_id = kwargs.get(user_id_field, None)  # Get user_id from first argument (Use for Flask-Restful)
                    trusted_user_data = {
                        'id': request.headers.get(header_id_field, None),
                        'name': request.headers.get('X-User-Name', None)
                    }

                    if ( trusted_user_id := trusted_user_data['id'] ) and ( user_id is None or str(user_id) == str(trusted_user_id) ):
                        kwargs[user_id_field] = trusted_user_id  # Update original user_id argument to trusted if its allowed
                        return func(trusted_user_id, *args, **kwargs)
                    else:
                        # Logging an unauthorized access attempt
                        abort(403, "Wrong user auth data")

                return wrapped
            return decorator


    @Inner.wrap_auth("user_id", "X-User-Id")
    def get(self, user_id: Optional[int] = None) -> RESTFUL_JSON_RESPONSE:
        user_data = Users.get_one( user_id ) or {}
        return { "result": user_data }, 200


    @Inner.wrap_auth("user_id", "X-User-Id")
    def put(self, user_id: Optional[int] = None) -> RESTFUL_JSON_RESPONSE:
        req_data = request.get_json()
        try:
            Users.update(user_id, req_data)
            return { "result": { "id": user_id } }, 200
        except Exception as err:
            return { "error": str(err) }, 500


    @Inner.wrap_auth("user_id", "X-User-Id")
    def delete(self, user_id: Optional[int] = None) -> RESTFUL_JSON_RESPONSE:
        try:
            Users.delete( user_id )
            return { "result": { "id": user_id } }, 200
        except Exception as err:
            return { "error": str(err) }, 500
