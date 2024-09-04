from logging import exception

import yaml
from pydantic_core.core_schema import model_ser_schema
from pynmeagps import NMEAReader, NMEAMessage
from pathlib import Path

file_name = "result.yaml"
data_location = Path.cwd().parent

file_location = data_location.joinpath("data").joinpath(file_name)


with open(file_location, 'r+') as file:
    gps_data =  yaml.safe_load(file)


nmr = NMEAReader(gps_data, nmeaonly=True)
p_data = {}
for record in gps_data:
    msg = NMEAReader.parse(record)
    try:
        if msg.spd :
            p_data[f"{msg.time}"] = {"lat":msg.lat, "lon": msg.lon, "sog":msg.spd, "cog": msg.cog}
    except AttributeError :
        pass
with open("parsed.yaml", "w+") as file:
    yaml.safe_dump(p_data, file)