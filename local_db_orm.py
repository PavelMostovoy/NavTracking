from typing import List

from sqlalchemy import Column, ForeignKey, Integer, String, Text, Date, \
    DateTime, create_engine, Float, select
from sqlalchemy.ext.hybrid import hybrid_property
from sqlalchemy.orm import relationship, Mapped, mapped_column
import sqlalchemy
from sqlalchemy.orm import Session
from datetime import datetime

Base = sqlalchemy.orm.declarative_base()


class Sailboat(Base):
    __tablename__ = "sailboats"
    __tableargs__ = {
        'comment': 'Sailboats'
    }

    _sail_id: Mapped[str] = mapped_column(primary_key=True)

    @hybrid_property
    def sail_id(self):
        return self._sail_id

    @sail_id.inplace.setter
    def sail_id(self, value):
        if not self._sail_id:
            self._sail_id = value

    children: Mapped[List["NavData"]] = relationship(back_populates="parent",
                                                     cascade="all, delete-orphan")
    name: Mapped[str] = mapped_column(comment='Racer')
    birth_date: Mapped[datetime] = mapped_column(comment='Birth date')
    sclass: Mapped[str] = mapped_column(String(5), default="OPTI", comment='Class')
    license: Mapped[str] = mapped_column(nullable=True)

    def __repr__(self):
        return f'{self.sail_id} {self.sclass} {self.name} {self.birth_date}'


class NavData(Base):
    __tablename__ = "nav_data"
    __tableargs__ = {
        'comment': 'Navigation data',
    }

    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True)
    parent_id: Mapped[str] = mapped_column(ForeignKey("sailboats._sail_id",
                                                      ondelete="CASCADE", onupdate="CASCADE"), )
    parent: Mapped["Sailboat"] = relationship(back_populates="children")
    time: Mapped[datetime] = mapped_column(comment='Time')
    lat: Mapped[float] = mapped_column(comment='Latitude')
    lon: Mapped[float] = mapped_column(comment='Longitude')
    sog: Mapped[float] = mapped_column(comment='Speed Over Ground', nullable=True)

    def __repr__(self):
        return f'{self.id} {self.parent_id} {self.time} {self.lat} {self.lon} {self.sog}'


engine = create_engine('sqlite:///forein_app.sqlite')
Base.metadata.create_all(engine)

# with Session(bind=engine) as session:
#
#
#     user = session.query(Sailboat).all()[1]
#     print(user.children)


with Session(bind=engine) as session:
    user = session.query(Sailboat).all()
    if user:
        user = session.query(Sailboat).all()[0]
    else:
        user = session.query(Sailboat).first()

    if not user:
        user = Sailboat()
    user.sail_id = "3333FRA"
    user.name = "Jule"
    user.birth_date = datetime.today()
    user.sclass = "OPTI1"

    coord = NavData()
    coord.time = datetime.now()
    coord.lat = 1.24
    coord.lon = 22.5
    coord.sog = 1

    user.children.append(coord)

    session.add(user)
    print(session.query(Sailboat).first())
    print(session.query(NavData).all())
    session.commit()

    # session.delete(user)
    print(session.query(Sailboat).first())
    print(user.children)

    session.commit()
