from pathlib import Path
import yaml
import statistics

file_name = "2024_9_14.log"
data_location = Path.cwd().parent

file_location = data_location.joinpath("data").joinpath(file_name)
result_file = data_location.joinpath("data").joinpath("parsed.yaml")

with open(file_location, "r+") as i_file, open(result_file, "w+") as r_file:
    data = {}

    for line in i_file:
        pre_data = line.split(",")
        if data.get(pre_data[0]) is None:
            data[pre_data[0]] = []
        data[pre_data[0]].append((float(pre_data[1]), float(pre_data[2]), float(pre_data[3])))

    for key, value in data.items():
        sum_lat = []
        sum_lon = []
        sum_sog = []
        for record in value:
            sum_lat.append(record[0])
            sum_lon.append(record[1])
            sum_sog.append(record[2])
        data[key] = {"lat" : statistics.mean(sum_lat), "lon": statistics.mean(sum_lon),"sog": statistics.mean(sum_sog)}


    yaml.safe_dump(data, r_file)
