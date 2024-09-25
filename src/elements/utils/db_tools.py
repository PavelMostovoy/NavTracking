from pathlib import Path
from typing import List

from sqlalchemy import ForeignKey, create_engine
from sqlalchemy.ext.hybrid import hybrid_property
from sqlalchemy.orm import relationship, Mapped, mapped_column
import sqlalchemy
from datetime import datetime

db_path = Path.cwd().parent.joinpath("nav_app.sqlite")

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
    sclass: Mapped[str] = mapped_column(default="OPTI", comment='Class')
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

engine = create_engine(f'sqlite:///{db_path}')