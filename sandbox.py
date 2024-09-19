import json
import time
from datetime import datetime
from pathlib import Path

import geopy.distance
import yaml



data_src = Path.cwd().joinpath("data")

with open(data_src.joinpath("parsed.yaml"), "r+") as file:
    data_dict = yaml.safe_load(file)

data = []
for key, value in data_dict.items():
    value.update({"time" : key})
    data.append(value)

with open(data_src.joinpath("parsed_list.yaml"), "w+") as file:
    yaml.dump(data, file)

with open(data_src.joinpath("parsed_list.yaml"), "r+") as file:
    data_list = yaml.safe_load(file)

data_list.sort(key = lambda x : datetime.strptime(x["time"], '%H:%M:%S.%f').time())
# data_list.sort(key = lambda x : x["sog"])
coef = 1

data_list[0].update({"valid" : True})
data_list[-1].update({"valid" : True})

for i,position in enumerate(data_list):
    if i == len(data_list) - 2:
        break
    current_position = (position["lat"], position["lon"])
    next_position = (data_list[i+1]["lat"], data_list[i+1]["lon"])
    third_position = (data_list[i+2]["lat"], data_list[i+1]["lon"])
    close_distance = geopy.distance.distance(current_position, next_position).m
    far_distance = geopy.distance.distance(current_position, third_position).m
    if close_distance/coef < far_distance:
        data_list[i+1].update({"valid" : True})
    else:
        data_list[i+1].update({"valid" : False})

data_list = list(filter(lambda x : x["valid"] == True, data_list))

data_list.reverse()

for i,position in enumerate(data_list):
    if i == len(data_list) - 2:
        break
    current_position = (position["lat"], position["lon"])
    next_position = (data_list[i+1]["lat"], data_list[i+1]["lon"])
    third_position = (data_list[i+2]["lat"], data_list[i+1]["lon"])
    close_distance = geopy.distance.distance(current_position, next_position).m
    far_distance = geopy.distance.distance(current_position, third_position).m
    if close_distance/coef < far_distance:
        data_list[i+1].update({"valid" : True})
    else:
        data_list[i+1].update({"valid" : False})

data_list = list(filter(lambda x : x["valid"] == True, data_list))

data_list.reverse()

with open(data_src.joinpath("parsed_list.yaml"), "w+") as file:
    yaml.dump(data_list,file)

