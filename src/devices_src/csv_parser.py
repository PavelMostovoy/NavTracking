from pathlib import Path
import os
import csv
import sqlalchemy as db


engine = db.create_engine('sqlite:///flet_app.sqlite')
connection = engine.connect()
metadata_obj = db.MetaData()

stocks = db.Table('profile', metadata_obj)
secondary = db.Table('voiles', metadata_obj)

query = db.select(secondary.c.name)
print(query)

data = connection.execute(query)
data = data.fetchall()

print(data)

profile = db.Table(
    'voiles',
    metadata_obj,
    db.Column('sail_number', db.String, primary_key=True),
    db.Column('name', db.String),
    db.Column('another', db.Integer),
)

metadata_obj.create_all(engine)

# file_name = "gps_log.txt"
# data_location = Path(__file__).parent.parent.parent.joinpath("data")
#
#
# for root, dirs, files in os.walk(data_location):
#     for file in files:
#         if file.endswith(".csv"):
#              file_name = os.path.join(root, file)
#              with open(file_name, "r", newline="") as f:
#                  values = csv.DictReader(f, delimiter=';', quotechar='|')
#                  for row in values:
#                      print(row["Lat"].strip("NS"), row["Lon"].strip("EW"), row["Hour"])
#              break


