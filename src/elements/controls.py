from dataclasses import dataclass
from random import choice

import flet as ft
import flet.map as f_map
from sqlalchemy.orm import Session

from .utils.db_tools import Sailboat, engine

data_container = ft.Ref[ft.Container]()


@dataclass
class Message:
    Receiver: str
    Message: str | int


def polyline_update(polyline, e):
    polyline.color = e.control.active_color
    polyline.visible = True
    polyline.use_stroke_width_in_meter = True
    polyline.border_color = e.control.active_color
    polyline.border_stroke_width = 2
    return polyline


def coords_replace(received_coordinated, range, actual_coordinates):
    # need to be reworked
    actual_coordinates.clear()
    for coord in received_coordinated[1000:-1000]:
        # if coord.time > datetime.strptime("22/09/2024::10:08:00", '%d/%m/%Y::%H:%M:%S'):
        prepared_coord = f_map.MapLatitudeLongitude(coord.lat, coord.lon)
        actual_coordinates.append(prepared_coord)


def manage_data_container(e):
    order = int(e.control.data)
    containers = e.page.overlay[0].controls[1].controls[0].controls
    polyline = e.page.controls[0].layers[1].polylines[order]
    actual_coordinates = polyline.coordinates
    identifier = e.control.label
    with Session(bind=engine) as session:
        user = session.query(Sailboat).filter(Sailboat.sail_id == identifier).one()
        received_coordinated = user.children

    if e.control.value:
        for container in containers:
            if container.content.value == identifier:
                e.page.update()
                return

        containers.append(MonitoringContainer(content=ft.Text(identifier),
                                              bgcolor=e.control.active_color))
        polyline_update(polyline, e)

        coords_replace(received_coordinated, 100, actual_coordinates)

        e.page.update()

    else:
        for i, container in enumerate(containers):
            if container.content.value == identifier:
                actual_coordinates.clear()
                containers.pop(i)
                e.page.update()


class MyCheckboxes(ft.Row):

    def __init__(self):
        super().__init__()
        with Session(bind=engine) as session:
            users = session.query(Sailboat).all()
        self.users = users
        self.checkboxes = []
        self.colours = [ft.colors.RED,
                        ft.colors.GREEN,
                        ft.colors.BLUE,
                        ft.colors.YELLOW,
                        ft.colors.ORANGE,
                        ft.colors.AMBER]

    def get_init_checkboxes(self):
        for i, user in enumerate(self.users):
            colour = choice(self.colours)
            self.colours.remove(colour)
            selector = MyCheckbox(colour,
                                  f"{user.sail_id}",
                                  i)
            self.checkboxes.append(selector)

    def build(self):
        self.get_init_checkboxes()
        self.controls = self.checkboxes
        self.alignment = ft.MainAxisAlignment.START


class MyCheckbox(ft.Checkbox):
    def __init__(self, color: ft.colors, text: str, order: int):
        super().__init__(adaptive=True, value=False)
        self.active_color = color
        self.label = text
        self.order = int(order)

    def build(self):
        self.data = self.order
        self.on_change = manage_data_container


class MySlider(ft.Slider):

    @staticmethod
    def slider_change(e):
        e.page.pubsub.send_all(Message("Slider", int(e.control.value)))

    def __init__(self):
        super().__init__(min=0, max=100)

    def build(self):
        self.height = 50,
        self.on_change = self.slider_change


class MonitoringContainer(ft.Container):

    def __init__(self, content, bgcolor):
        super().__init__(content=content,
                         bgcolor=bgcolor,
                         width=100,
                         height=100,
                         margin=10,
                         padding=10,
                         alignment=ft.alignment.center,
                         border_radius=10,
                         ink=True)

    def build(self):
        self.on_click = lambda e: print("Clickable with Ink clicked!")
