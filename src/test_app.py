from pathlib import Path

import flet as ft
import flet.map as map
import yaml

data_src = Path.cwd().parent.joinpath("data")

with open(data_src.joinpath("parsed_list.yaml"), "r+") as file:
    positions_list = yaml.safe_load(file)


def main(page: ft.Page):
    page.title = "Containers - clickable and not"
    page.vertical_alignment = ft.MainAxisAlignment.CENTER
    page.horizontal_alignment = ft.CrossAxisAlignment.CENTER
    page.window.width = 1024
    page.window.height = 900

    slider_ref = ft.Ref[ft.Slider]()
    map_container = ft.Ref[ft.Container]()
    data_container = ft.Ref[ft.Container]()
    circle_layer_ref = ft.Ref[map.CircleLayer]()
    polyline_layer_ref = ft.Ref[map.PolylineLayer]()

    def resized(containers: []):
        for reference in containers:
            reference.current.width = page.width
            if reference.current.data == "MAP":
                reference.current.height = page.height * 0.75
        page.update()

    def get_circle(value):
        coord = map.MapLatitudeLongitude(positions_list[value]["lat"], positions_list[value]["lon"])
        circle = map.CircleMarker(
            radius=5,
            coordinates=coord,
            color=ft.colors.RED,
            border_color=ft.colors.BLUE,
            border_stroke_width=1,
        )
        return circle

    def get_tail(value):
        tail_length = value - 20
        if tail_length < 0:
            tail_length = 0
        tail = positions_list[tail_length:value+1]
        tail_coord = []
        for coord in tail:
            tail_coord.append(map.MapLatitudeLongitude(coord["lat"], coord["lon"]))
        return tail_coord


    def slider_change(e):
        value = int(e.control.value)
        circle = get_circle(value)
        circle_layer_ref.current.circles = [circle]
        polyline_layer_ref.current.coordinates = get_tail(value)
        page.update()

    top_container = ft.Container(
        ref=map_container,
        content=map.Map(
            expand=True,
            configuration=map.MapConfiguration(
                initial_center=map.MapLatitudeLongitude(42.703622, 3.038975),
                initial_zoom=15,
                interaction_configuration=map.MapInteractionConfiguration(
                    flags=map.MapInteractiveFlag.ALL
                ),
                on_init=lambda e: print(f"Initialized Map"),
            ),
            layers=[
                map.TileLayer(
                    url_template="https://tile.openstreetmap.org/{z}/{x}/{y}.png",
                    on_image_error=lambda e: print("TileLayer Error"),
                ),
                map.CircleLayer(
                    ref=circle_layer_ref,
                    circles=[map.CircleMarker(
                        radius=5,
                        coordinates=map.MapLatitudeLongitude(positions_list[0]["lat"], positions_list[0]["lon"]),
                        color=ft.colors.RED,
                        border_color=ft.colors.BLUE,
                        border_stroke_width=1,
                    )],
                ),
                map.PolylineLayer(
                    polylines=[
                        map.PolylineMarker(
                            ref=polyline_layer_ref,
                            border_stroke_width=2,
                            border_color=ft.colors.GREEN,
                            gradient_colors=[ft.colors.BLACK, ft.colors.BLACK],
                            color=ft.colors.with_opacity(0.6, ft.colors.GREEN),
                            coordinates=[],
                            use_stroke_width_in_meter=True,
                        ),
                    ],
                )
            ],
        ),
        margin=10,
        padding=10,
        alignment=ft.alignment.top_center,
        bgcolor=ft.colors.AMBER,
        width=page.width,
        height=page.height * 0.75,
        border_radius=10,
        data="MAP"
    )
    middle_container = ft.Container(
        ref=data_container,
        content=ft.Text("Clickable with Ink"),
        margin=10,
        padding=10,
        alignment=ft.alignment.center,
        bgcolor=ft.colors.CYAN_200,
        width=page.width,
        height=20,
        border_radius=10,
        ink=True,
        on_click=lambda e: print("Clickable with Ink clicked!"),
    )
    checkboxes = [ft.Checkbox(adaptive=True, label="Adaptive Checkbox 1", value=True),
                  ft.Checkbox(adaptive=True, label="Adaptive Checkbox2", value=True), ]
    check_boxes = ft.Row(
        controls=checkboxes
    )
    page.add(
        ft.Column(spacing=0,
                  controls=[
                      check_boxes,
                      top_container,
                      middle_container,
                      ft.Slider(
                          ref=slider_ref,
                          min=0,
                          max=len(positions_list) - 1,
                          # divisions=100,
                          value=0,
                          label="{value}",
                          width=page.width,
                          on_change=slider_change
                      )
                  ],
                  alignment=ft.MainAxisAlignment.SPACE_BETWEEN,
                  ),
    )

    page.on_resized = lambda x: resized([map_container, data_container, slider_ref])


ft.app(main)
