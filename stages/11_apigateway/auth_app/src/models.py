# coding: utf-8

from .db_factory import db  # type: ignore




class UserAlreadyExistsError(Exception):
    pass



class Users( db.Model ):
    __tablename__ = 'users'

    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String, unique=True, nullable=False)
    password = db.Column(db.String, unique=False, nullable=False)

    def __repr__( self ):
        return '<User %r>' % self.name

    def as_dict( self ):
        return { 'id': self.id, 'name': self.name }

    @classmethod
    def get_one(cls, user_id: int) -> dict:
        qr = db.session.query( cls ).filter( cls.id == user_id ).first()
        return ( qr and qr.as_dict() ) or {}

    @classmethod
    def find_one(cls, data: dict) -> dict:
        username = data["username"]
        password = data["password"]
        qr = db.session.query( cls ).filter( cls.name == username ).filter( cls.password == password ).first()
        return ( qr and qr.as_dict() ) or {}

    @classmethod
    def insert(cls, data: dict) -> int:
        username = data["username"]
        password = data["password"]
        if not db.session.query( db.exists().where( cls.name == username ) ).scalar():
            obj = Users(name=username, password=password)
            db.session.add( obj )
            db.session.commit()
            return obj.id
        else:
            raise UserAlreadyExistsError()

    @classmethod
    def update(cls, user_id: int, data: dict) -> int:
        db.session.query( cls ).filter(cls.id == user_id).update(data)
        db.session.commit()
        return user_id

    @classmethod
    def delete(cls, user_id: int) -> int:
        db.session.query( cls ).filter(cls.id == user_id).delete()
        db.session.commit()
        return user_id
