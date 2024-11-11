import flet.map
from sqlalchemy.orm import Session

from .utils.db_tools import Sailboat, engine


class MyMap(flet.map.Map):

    def __init__(self):
        layers = [
            flet.map.TileLayer(
                url_template="https://tile.openstreetmap.org/{z}/{x}/{y}.png",
            ),
            # Need to be properly displayed
            # flet.map.RichAttribution(
            #     show_flutter_map_attribution =False,
            #     attributions=[
            #         flet.map.TextSourceAttribution(
            #             text="OpenStreetMap Contributors",
            #             on_click=lambda e: e.page.launch_url(
            #                 "https://openstreetmap.org/copyright"
            #             ),
            #         ),
            #     ]
            #
            # )
        ]
        super().__init__(layers=layers,
                         expand=True,
                         scale=1.1)
        self.polylines_list = []

    #
    def build(self):
        self.configuration = flet.map.MapConfiguration(
            initial_center=flet.map.MapLatitudeLongitude(42.703622, 3.038975),
            initial_zoom=15,
            interaction_configuration=flet.map.MapInteractionConfiguration(
                flags=flet.map.MapInteractiveFlag.ALL
            ))

        with Session(bind=engine) as session:
            users = session.query(Sailboat).all()
            lines_count = len(users)
        for polyline in range(lines_count):
            line = flet.map.PolylineMarker(coordinates=[], visible=False)
            self.polylines_list.append(line)
        self.layers.append(flet.map.PolylineLayer(
            polylines=self.polylines_list,
        ))

    # def did_mount(self):

    # def will_unmount(self):
