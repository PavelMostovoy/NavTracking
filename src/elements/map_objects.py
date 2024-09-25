import flet.map as f_map
import flet as ft
from sqlalchemy.orm import Session

from .utils.db_tools import Sailboat, engine


def my_map():
    polylines_l = []

    with Session(bind=engine) as session:
        users = session.query(Sailboat).all()
        lines_count = len(users)

    for polyline in range(lines_count):
        line = f_map.PolylineMarker(coordinates=[], visible=False)
        polylines_l.append(line)

    map_obj = f_map.Map(
        expand=True,
        configuration=f_map.MapConfiguration(
            initial_center=f_map.MapLatitudeLongitude(42.703622, 3.038975),
            initial_zoom=15,
            interaction_configuration=f_map.MapInteractionConfiguration(
                flags=f_map.MapInteractiveFlag.ALL
            ),
            on_init=lambda e: print(f"Initialized Map"),
        ),
        layers=[
            f_map.TileLayer(
                url_template="https://tile.openstreetmap.org/{z}/{x}/{y}.png",
                on_image_error=lambda e: print("TileLayer Error"),

            ),
            f_map.PolylineLayer(
                polylines=polylines_l,
            )

        ],
    )

    return map_obj
