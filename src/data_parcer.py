from pathlib import Path
import yaml

file_name = "gps_log.txt"
data_location = Path.cwd().parent

file_location = data_location.joinpath("data").joinpath(file_name)
result_file = data_location.joinpath("data").joinpath("result.yaml")
with open(file_location, "r+") as i_file, open(result_file, "w+") as r_file:
    data = []
    for line in i_file:
        # if line[:6] == "$GPTXT":
        #     continue
        if line[:6] == "$GNRMC" and line.split(",")[2] == "A":
            data.append(line[:-1])
        else:
            continue
    yaml.safe_dump(data, r_file)
