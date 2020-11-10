# coding: utf-8

from typing import Sequence, Optional
from .db_factory import db  # type: ignore




class Client(db.Model):
    __tablename__ = 'client'

    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String, unique=True, nullable=False)

    def __repr__(self):
        return '<User %r>' % self.name

    def as_dict(self):
        return { 'id': self.id, 'name': self.name }

    @classmethod
    def find(cls, _id: Optional[int] = None) -> Sequence[dict]:
        qr = db.session.query(cls).filter(cls.id == _id).all() if _id is not None else db.session.query(Client).all()
        return [ r.as_dict() for r in qr ]

    @classmethod
    def delete(cls, _id: int) -> Sequence[str]:
        qr = db.session.query(cls).filter(cls.id == _id).delete()
        return [ str(r) for r in qr ]
