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

    slider_ref = ft.Ref[ft.Slider]()
    map_container = ft.Ref[ft.Container]()
    data_container = ft.Ref[ft.Container]()
    circle_layer_ref = ft.Ref[map.CircleLayer]()
    polyline_layer_ref = ft.Ref[map.PolylineLayer]()

    def resized(containers: []):
        for reference in containers:
            reference.current.width = page.width
            if reference.current.data == "MAP":
                reference.current.width = page.width - 200
                reference.current.height = page.height * 0.75
        page.update()

    def get_circle(value):
        coord = map.MapLatitudeLongitude(positions_list[value]["lat"], positions_list[value]["lon"])
        circle = map.CircleMarker(
            radius=6,
            coordinates=coord,
            color=ft.colors.GREEN,
            border_color=ft.colors.BLUE,
            border_stroke_width=2,
        )
        return circle

    def get_tail(value):
        tail_length = value - 20
        if tail_length < 0:
            tail_length = 0
        tail = positions_list[tail_length:value + 1]
        tail_coord = []
        for coord in tail:
            tail_coord.append(map.MapLatitudeLongitude(coord["lat"], coord["lon"]))
        return tail_coord

    def slider_change(e):
        value = int(e.control.value)
        circle = get_circle(value)
        circle_layer_ref.current.circles = [circle]
        polyline_layer_ref.current.coordinates = get_tail(value)
        data_container.current.content = ft.Text(value=f"{round(positions_list[value]['sog'], 1)}", size=30)
        page.update()

    top_container = ft.Container(
        ref=map_container,
        content=map.Map(
            expand=True,
            configuration=map.MapConfiguration(
                initial_center=map.MapLatitudeLongitude(42.703622, 3.038975),
                initial_zoom=17,
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
                        radius=6,
                        coordinates=map.MapLatitudeLongitude(positions_list[0]["lat"], positions_list[0]["lon"]),
                        color=ft.colors.GREEN,
                        border_color=ft.colors.BLUE,
                        border_stroke_width=2,
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
        width=page.width -200,
        height=page.height -200,
        border_radius=10,
        data="MAP"
    )
    middle_container = ft.Container(
        ref=data_container,
        content=ft.Text("Voile #"),
        margin=10,
        padding=10,
        alignment=ft.alignment.center,
        bgcolor=polyline_layer_ref.current.border_color,
        width=100,
        height=100,
        border_radius=10,
        ink=True,
        on_click=lambda e: print("Clickable with Ink clicked!"),
    )
    checkboxes = [ft.Checkbox(adaptive=True, label="Voile # 5555 FRA", value=True, active_color=ft.colors.RED),
                  ft.Checkbox(adaptive=True, label="Voile # 4444 FRA", value=True, active_color=ft.colors.GREEN), ]
    check_boxes = ft.Row(
        controls=checkboxes,
        height=50,
        alignment=ft.alignment.top_center,
    )
    page.add(
        ft.Column(spacing=0,
                  controls=[
                      check_boxes,
                      ft.Row(
                          controls=[top_container,
                                    ft.Column(
                                        controls=[middle_container, ft.Container(
                                            content=ft.Text("Voile #"),
                                            margin=10,
                                            padding=10,
                                            alignment=ft.alignment.center,
                                            bgcolor=ft.colors.RED,
                                            width=100,
                                            height=100,
                                            border_radius=10,
                                            ink=True,
                                        )],
                                        alignment=ft.MainAxisAlignment.START, )],
                          alignment=ft.MainAxisAlignment.START),
                      ft.Slider(
                          ref=slider_ref,
                          min=0,
                          max=len(positions_list) - 1,
                          # divisions=100,
                          value=0,
                          label="{value}",
                          width=page.width,
                          height= 50,
                          on_change=slider_change
                      )
                  ],
                  alignment=ft.MainAxisAlignment.SPACE_BETWEEN,
                  ),
    )

    page.on_resized = lambda x: resized([map_container, slider_ref])


ft.app(main)
