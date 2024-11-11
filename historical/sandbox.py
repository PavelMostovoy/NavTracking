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

def calculate_speed(coord1, coord2):
    first_time = datetime.strptime(coord1["time"], '%H:%M:%S.%f')
    second_time = datetime.strptime(coord2["time"], '%H:%M:%S.%f')
    first_position = (coord1["lat"], coord1["lon"])
    second_position = (coord2["lat"], coord2["lon"])
    distance = geopy.distance.distance(first_position, second_position).m
    difference = (second_time-first_time).total_seconds()
    sog = distance * 3.6 / difference
    return sog
    # coord2.update({"sog": sog})



data_list.sort(key = lambda x : datetime.strptime(x["time"], '%H:%M:%S.%f').time())
# data_list.sort(key = lambda x : x["sog"])
coef = 1

data_list[0].update({"valid" : True})
data_list[-1].update({"valid" : True})
temp_speed = 1
for i,position in enumerate(data_list[:-2]):
    current_position = (position["lat"], position["lon"])
    next_position = (data_list[i+1]["lat"], data_list[i+1]["lon"])
    speed = calculate_speed(position,data_list[i+1])
    third_position = (data_list[i+2]["lat"], data_list[i+1]["lon"])
    close_distance = geopy.distance.distance(current_position, next_position).m
    far_distance = geopy.distance.distance(current_position, third_position).m
    if close_distance/coef < far_distance and speed < 25 :
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
    speed = calculate_speed(position, data_list[i + 1])
    third_position = (data_list[i+2]["lat"], data_list[i+1]["lon"])
    close_distance = geopy.distance.distance(current_position, next_position).m
    far_distance = geopy.distance.distance(current_position, third_position).m
    if close_distance/coef < far_distance and speed < 25:
        data_list[i+1].update({"valid" : True})
    else:
        data_list[i+1].update({"valid" : False})

data_list = list(filter(lambda x : x["valid"] == True, data_list))

data_list.reverse()

with open(data_src.joinpath("parsed_list.yaml"), "w+") as file:
    yaml.dump(data_list,file)

