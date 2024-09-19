import random
import flet as ft
import flet.map as map
import yaml
from pathlib import Path

from flet_core.map import DottedStrokePattern

data_src = Path.cwd().parent.joinpath("data")


with open(data_src.joinpath("parsed_list.yaml"), "r+") as file:
    my_markers_list = yaml.safe_load(file)

my_markers = []
my_coord = []
for coord in my_markers_list:
    my_markers.append(
    map.Marker(
        content=ft.Icon(ft.icons.LOCATION_ON),
        coordinates=map.MapLatitudeLongitude(coord["lat"], coord["lon"])))
    my_coord.append(map.MapLatitudeLongitude(coord["lat"], coord["lon"]))

list_of_circles =[]
for coord in my_coord:
    list_of_circles.append(
        map.CircleMarker(
                            radius=5,
                            coordinates=coord,
                            color=ft.colors.RED,
                            border_color=ft.colors.BLUE,
                            border_stroke_width=1,
                        )
    )



def main(page: ft.Page):
    marker_layer_ref = ft.Ref[map.MarkerLayer]()
    circle_layer_ref = ft.Ref[map.CircleLayer]()

    def handle_tap(e: map.MapTapEvent):
        print(
            f"Name: {e.name} - coordinates: {e.coordinates} - Local: ({e.local_x}, {e.local_y}) - Global: ({e.global_x}, {e.global_y})"
        )
        if e.name == "tap":
            aprox_lat = round(e.coordinates.latitude)
            aprox_lon = round(e.coordinates.longitude)
            for i, marker in enumerate(marker_layer_ref.current.markers):
                if round(marker.coordinates.longitude) == aprox_lon and  round(marker.coordinates.latitude) == aprox_lat:
                    marker_layer_ref.current.markers.pop(i)
                    break
            else:
                marker_layer_ref.current.markers.append(
                    map.Marker(
                        content=ft.Icon(
                            ft.icons.LOCATION_ON, color=ft.cupertino_colors.DESTRUCTIVE_RED
                        ),
                        coordinates=e.coordinates,
                    )
                )
        elif e.name == "secondary_tap":
            circle_layer_ref.current.circles.append(
                map.CircleMarker(
                    radius=random.randint(5, 10),
                    coordinates=e.coordinates,
                    color=ft.colors.random_color(),
                    border_color=ft.colors.random_color(),
                    border_stroke_width=4,
                )
            )
        page.update()

    def handle_event(e: map.MapEvent):
        print(
            f"{e.name} - Source: {e.source} - Center: {e.center} - Zoom: {e.zoom} - Rotation: {e.rotation}"
        )

    page.add(
        ft.Text("Header record : Click anywhere to add a Marker, right-click to add a CircleMarker."),
        map.Map(
            expand=True,
            configuration=map.MapConfiguration(
                initial_center=map.MapLatitudeLongitude(42.703622, 3.038975),
                initial_zoom=15,
                interaction_configuration=map.MapInteractionConfiguration(
                    flags=map.MapInteractiveFlag.ALL
                ),
                on_init=lambda e: print(f"Initialized Map"),
                on_tap=handle_tap,
                on_secondary_tap=handle_tap,
                on_long_press=handle_tap,
                on_event=handle_event,
            ),
            layers=[
                map.TileLayer(
                    url_template="https://tile.openstreetmap.org/{z}/{x}/{y}.png",
                    on_image_error=lambda e: print("TileLayer Error"),
                ),
                map.RichAttribution(
                    attributions=[
                        map.TextSourceAttribution(
                            text="OpenStreetMap Contributors",
                            on_click=lambda e: e.page.launch_url(
                                "https://openstreetmap.org/copyright"
                            ),
                        ),
                        map.TextSourceAttribution(
                            text="Flet",
                            on_click=lambda e: e.page.launch_url("https://flet.dev"),
                        ),
                    ]
                ),
                map.SimpleAttribution(
                    text="Flet",
                    alignment=ft.alignment.top_right,
                    on_click=lambda e: print("Clicked SimpleAttribution"),
                ),
                # map.MarkerLayer(
                #     ref=marker_layer_ref,
                #     markers = my_markers,
                #     # markers=[
                #     #     map.Marker(
                #     #         content=ft.Icon(ft.icons.LOCATION_ON),
                #     #         coordinates=map.MapLatitudeLongitude(30, 15),
                #     #     ),
                #     #     map.Marker(
                #     #         content=ft.Icon(ft.icons.LOCATION_ON),
                #     #         coordinates=map.MapLatitudeLongitude(10, 10),
                #     #     ),
                #     #     map.Marker(
                #     #         content=ft.Icon(ft.icons.LOCATION_ON),
                #     #         coordinates=map.MapLatitudeLongitude(25, 45),
                #     #     ),
                #     # ],
                # ),
                map.CircleLayer(
                    ref=circle_layer_ref,
                    circles=list_of_circles,
                ),
                # map.PolygonLayer(
                #     polygons=[
                #         map.PolygonMarker(
                #             label="Popular Touristic Area",
                #             label_text_style=ft.TextStyle(
                #                 color=ft.colors.BLACK,
                #                 size=15,
                #                 weight=ft.FontWeight.BOLD,
                #             ),
                #             color=ft.colors.with_opacity(0.3, ft.colors.BLUE),
                #             coordinates=[
                #                 map.MapLatitudeLongitude(10, 10),
                #                 map.MapLatitudeLongitude(30, 15),
                #                 map.MapLatitudeLongitude(25, 45),
                #             ],
                #         ),
                #     ],
                # ),
                map.PolylineLayer(
                    polylines=[
                        map.PolylineMarker(
                            border_stroke_width=3,
                            border_color=ft.colors.RED,
                            gradient_colors=[ft.colors.BLACK, ft.colors.BLACK],
                            color=ft.colors.with_opacity(0.6, ft.colors.GREEN),
                            coordinates=my_coord,
                            stroke_pattern=DottedStrokePattern(1),
                        ),
                    ],
                ),
            ],
        ),
    )


ft.app(main)