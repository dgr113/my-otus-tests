# coding: utf-8

from flask import jsonify, request
from flask_restful import Resource  # type: ignore




class Index( Resource ):
    def get( self ):
        return jsonify({ 'result': "OK" })



class UserOne( Resource ):
    def get( self ) -> dict[str, str]:
        user = {
            'id': request.headers.get('X-User-Id', 'NONE'),
            'name': request.headers.get('X-User-Name', 'NONE')
        }
        return jsonify( user )
