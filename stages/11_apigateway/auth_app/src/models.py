# coding: utf-8

from typing import Sequence, Optional

from .db_factory import db  # type: ignore




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
    def get_one(cls, user_id: Optional[int] = None) -> Sequence[dict]:
        qr = db.session.query( cls ).filter(cls.id == user_id).all() if user_id is not None else db.session.query(Users).all()
        return [ r.as_dict() for r in qr ]

    @classmethod
    def find_one(cls, data: dict) -> Optional[dict]:
        username = data["username"]
        password = data["password"]
        qr = db.session.query( cls ).filter( cls.name == username ).filter( cls.password == password ).first()
        return qr.as_dict() if qr else None

    @classmethod
    def insert(cls, data: dict) -> Optional[int]:
        username = data["username"]
        password = data["password"]
        try:
            obj = Users(name=username, password=password)
            db.session.add( obj )
            db.session.commit()
            return obj.id
        except Exception:
            db.session.rollback()
            return None

    @classmethod
    def update(cls, user_id: int, data: dict) -> str:
        qr = db.session.query( cls ).filter(cls.id == user_id).update(data)
        db.session.commit()
        return str(qr)

    @classmethod
    def delete(cls, user_id: int) -> Optional[int]:
        db.session.query( cls ).filter(cls.id == user_id).delete()
        db.session.commit()
        return user_id
