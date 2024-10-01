import datetime

import flet as ft
import uvicorn

from elements.map_objects import MyMap
from elements.controls import MySlider, MyCheckboxes, TimeSelector, DateSelector, reference_button


def main(page: ft.Page):
    page.session.set("date", datetime.datetime.now())
    page.session.set("start_time", datetime.datetime.strptime(
            f"{page.session.get("date").date()}::11:59:00", '%Y-%m-%d::%H:%M:%S'))
    page.session.set("end_time", datetime.datetime.strptime(
            f"{page.session.get("date").date()}::12:00:00", '%Y-%m-%d::%H:%M:%S'))
    page.session.set("tracks", {})

    def on_broadcast_message(message):
        # Need to be implemented
        print(message)

    date_selector = ft.ElevatedButton(
        ref = reference_button,
        text = "Pick date",
        icon=ft.icons.CALENDAR_MONTH,
        on_click=lambda e: page.open(
            DateSelector()
        )

    )

    start_btn = ft.ElevatedButton(
        "Start time",
        icon=ft.icons.START,
        on_click=lambda _: page.open(TimeSelector()),
    )


    def main_struct():
        return ft.Column(
            controls=[ft.Row(controls=[MyCheckboxes(), date_selector, start_btn]),
                      ft.Row(controls=[ft.Column(
                          # controls=[],
                          alignment=ft.MainAxisAlignment.SPACE_BETWEEN)],
                          alignment=ft.MainAxisAlignment.END),
                      ], alignment=ft.MainAxisAlignment.START, height=page.height)

    page.overlay.append(main_struct())

    page.overlay.append(ft.Column(controls=[MySlider()], alignment=ft.MainAxisAlignment.END))

    page.add(MyMap())


if __name__ == '__main__':
    # uvicorn.run(ft.app(main, export_asgi_app=True), host="127.0.0.1", port=30000)
    ft.app(main)
