from random import choice

import flet as ft
import flet.map as f_map
from sqlalchemy.orm import Session

from .utils.db_tools import Sailboat, engine

slider_ref = ft.Ref[ft.Slider]()
data_container = ft.Ref[ft.Container]()


def manage_data_container(e):
    order = int(e.control.data)
    containers = e.page.overlay[0].controls[1].controls[0].controls
    polyline = e.page.controls[0].layers[1].polylines[order]
    coordinates = polyline.coordinates
    identifier = e.control.label
    with Session(bind=engine) as session:
        user = session.query(Sailboat).filter(Sailboat.sail_id == identifier).one()
        coords = user.children

    if e.control.value:
        for container in containers:
            if container.content.value == identifier:
                e.page.update()
                return
        container = monitoring_container()
        container.content = ft.Text(identifier)
        container.bgcolor = e.control.active_color
        containers.append(container)
        polyline.color = e.control.active_color
        polyline.visible = True
        polyline.use_stroke_width_in_meter = True
        polyline.border_color = e.control.active_color
        polyline.border_stroke_width = 2
        for coord in coords:
            prepared_coord = f_map.MapLatitudeLongitude(coord.lat, coord.lon)
            coordinates.append(prepared_coord)
        e.page.update()
    else:
        for i, container in enumerate(containers):
            if container.content.value == identifier:
                coordinates.clear()
                containers.pop(i)
                e.page.update()


def checkbox(color: ft.colors, text, order: int):
    obj = ft.Checkbox(adaptive=True,
                      label=text,
                      value=False,
                      active_color=color,
                      data=int(order),
                      on_change=manage_data_container)
    return obj


def my_checkboxes():
    with Session(bind=engine) as session:
        users = session.query(Sailboat).all()
    checkboxes = []
    colours = [ft.colors.RED,
               ft.colors.GREEN,
               ft.colors.BLUE,
               ft.colors.YELLOW,
               ft.colors.ORANGE,
               ft.colors.AMBER]
    for i, user in enumerate(users):
        colour = choice(colours)
        colours.remove(colour)
        selector = checkbox(colour, f"{user.sail_id}", i)
        checkboxes.append(selector)

    return ft.Row(
        controls=checkboxes,
        alignment=ft.MainAxisAlignment.START,
        # height=50,
    )


def slider_change(e):
    e.page.update()


def my_slider(page: ft.Page) -> ft.Slider:
    return ft.Slider(
        ref=slider_ref,
        min=0,
        max=100,
        # divisions=100,
        value=0,
        label="{value}",
        width=page.width,
        height=50,
        on_change=slider_change
    )


def monitoring_container():
    return ft.Container(
        content=ft.Text("Voile #"),
        margin=10,
        padding=10,
        alignment=ft.alignment.center,
        bgcolor=ft.colors.BLUE,
        width=100,
        height=100,
        border_radius=10,
        ink=True,
        on_click=lambda e: print("Clickable with Ink clicked!"),
    )
