import csv
import os
from pathlib import Path
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

def main(user_order):
    db_path = Path(__file__).parent.parent.parent.joinpath("nav_app.sqlite")

    engine = create_engine(f'sqlite:///{db_path}')

    data_location = Path(__file__).parent.parent.parent.joinpath("data").joinpath("ready_to_upload_data")


    for root, dirs, files in os.walk(data_location):
        for file in files:
            if file.endswith(".csv"):
                file_name = os.path.join(root, file)
                with open(file_name, "r", newline="") as f, Session(engine) as session:
                    user = session.query(Sailboat).all()[user_order]
                    values = csv.DictReader(f, delimiter=';', quotechar='|')
                    for row in values:
                        coord = NavData()
                        composed_time = f"{row['Date']}::{row['Hour']}"
                        # date = datetime.strptime(composed_time, '%d/%m/%YY/%H:%M:%S.%f')
                        coord.time = datetime.strptime(composed_time, '%d/%m/%Y::%H:%M:%S')
                        if "N" in row["Lat"]:
                            coord.lat = float(row["Lat"].strip("NS"))
                        else:
                            coord.lat = - float(row["Lat"].strip("NS"))
                        if "E" in row["Lon"]:
                            coord.lon = float(row["Lon"].strip("EW"))
                        else:
                            coord.lon = float(row["Lon"].strip("EW"))
                        coord.sog = float(row["Speed(km/h)"])
                        user.children.append(coord)
                    session.commit()

if __name__ == '__main__':
    main(1)
