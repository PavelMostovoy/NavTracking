import datetime
from dataclasses import dataclass
from random import choice

import flet as ft
import flet.map as f_map
from flet_core import TimePickerEntryMode, DatePicker
from sqlalchemy.orm import Session

from .utils.db_tools import Sailboat, engine

reference_button = ft.Ref[ft.ElevatedButton]()

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


def coords_prepare(received_coordinated: list, limit: datetime.datetime):
    tail_length = datetime.timedelta(minutes=10)
    start_time = limit - tail_length
    prepared_coordinates = []
    last_sog = 0.0
    for coord in received_coordinated:
        if start_time < coord[0] <= limit:
            prepared_coord = f_map.MapLatitudeLongitude(coord[1], coord[2])
            prepared_coordinates.append(prepared_coord)
            last_sog = coord[3]
    return prepared_coordinates, round(last_sog,2)


def manage_data_container(e):
    tracks = e.page.session.get("tracks")
    order = int(e.control.data)
    containers = e.page.overlay[0].controls[1].controls[0].controls
    polyline = e.page.controls[0].layers[1].polylines[order]
    actual_coordinates = polyline.coordinates
    identifier = e.control.label
    with Session(bind=engine) as session:
        user = session.query(Sailboat).filter(Sailboat.sail_id == identifier).one()
        received_all_coordinated = user.children
        received_coordinates = []
        # 2024-09-23::00:00:00
        selected_date = e.page.session.get("date")
        for coord in received_all_coordinated:
            if coord.time.date() == selected_date.date():
                received_coordinates.append((coord.time, coord.lat, coord.lon,coord.sog))
        if received_coordinates:
            if received_coordinates[0][0] < e.page.session.get("start_time"):
                e.page.session.set("start_time", received_coordinates[0][0])
            if received_coordinates[-1][0] > e.page.session.get("end_time"):
                e.page.session.set("end_time", received_coordinates[-1][0])

    if e.control.value:
        for container in containers:
            if container.content.data == identifier:
                e.page.update()
                return
        container = MonitoringContainer(content=ft.Text("SOG", size=30),
                                              bgcolor=e.control.active_color)
        container.content.data = identifier
        containers.append(container)
        polyline_update(polyline, e)


        tracks[identifier] = (order, received_coordinates)



    else:
        tracks.pop(identifier)
        for i, container in enumerate(containers):
            if container.content.data == identifier:
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

    def slider_change(self, e):
        start_point = e.page.session.get("start_time").timestamp()
        end_point = e.page.session.get("end_time").timestamp()

        if self.max != end_point:
            if self.value >= end_point:
                self.value = end_point
            self.max = end_point

        if self.min != start_point:
            if self.value <= start_point:
                self.value = start_point
            self.min = start_point

        hashmap_for_containers = {}
        for i, container in enumerate(e.page.overlay[0].controls[1].controls[0].controls):
            hashmap_for_containers[container.content.data] = i

        for owner, track in e.page.session.get("tracks").items():
            container = e.page.overlay[0].controls[1].controls[0].controls[hashmap_for_containers[owner]]
            order = track[0]
            coords = track[1]
            current_coordinates = e.page.controls[0].layers[1].polylines[order].coordinates
            actual_coords, container.content.value  = coords_prepare(coords, datetime.datetime.fromtimestamp(self.value))
            current_coordinates.clear()
            current_coordinates.extend(actual_coords)

        e.page.update()

    def __init__(self):
        super().__init__(min=datetime.datetime.now().timestamp() - 1000, max=datetime.datetime.now().timestamp())

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


class TimeSelector(ft.TimePicker):

    @staticmethod
    def handle_change(e):
        start_time = datetime.datetime.strptime(
            f"{e.page.session.get("date").date()}::{e.control.value}", '%Y-%m-%d::%H:%M:%S')
        end_time = start_time + datetime.timedelta(hours=8)
        e.page.session.set("start_time", start_time)
        e.page.session.set("end_time", end_time)
        e.page.update()



    def __init__(self):
        super().__init__()
        self.value = "09:00:00"
        self.on_change = self.handle_change
        self.time_picker_entry_mode = TimePickerEntryMode.DIAL_ONLY
        self.confirm_text = "Confirm"
        self.help_text = "Select time"




class DateSelector(DatePicker):

    def handle_change(self, e):
        reference_button.current.text = f"{e.control.value.date()}"
        e.page.session.set("date", e.control.value)
        start_time = datetime.datetime.strptime(
            f"{e.control.value.date()}::{e.page.session.get('start_time').time()}", '%Y-%m-%d::%H:%M:%S')
        end_time = datetime.datetime.strptime(
            f"{e.control.value.date()}::{e.page.session.get('end_time').time()}", '%Y-%m-%d::%H:%M:%S')
        e.page.session.set("start_time", start_time)
        e.page.session.set("end_time", end_time)
        e.page.update()

    def __init__(self):
        super().__init__()
        self.first_date = datetime.datetime(year=2024, month=9, day=1)
        self.on_change = self.handle_change
