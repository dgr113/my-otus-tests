# coding: utf-8

from functools import cache
from datetime import datetime, timedelta

from authlib.jose import jwk  # type: ignore
from jwt import encode as jwt_encode  # type: ignore




class JwtUtils:
    @staticmethod
    def create_id_token(private_key: str, user: dict) -> str:
        user_data = {
            "iss": "http://arch.homework",
            "exp": datetime.utcnow() + timedelta(minutes=15),
            "sub": user["id"],
            "name": user["name"]
        }
        result = jwt_encode(user_data, private_key, algorithm='RS256', headers={'kid': '1'})
        return result

    @staticmethod
    @cache
    def create_jwks_keyset(public_key: str) -> dict:
        key = jwk.dumps(public_key, kty='RSA')
        key |= {'kid': '1', 'alg': 'RS256', 'use': 'sig'}
        return { 'keys': [ key ] }
