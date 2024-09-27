import flet as ft
import uvicorn

from elements.map_objects import MyMap
from elements.controls import MySlider, MyCheckboxes


def main(page: ft.Page):
    def on_broadcast_message(message):
        # Need to be implemented
        print(message)

    def main_struct():
        return ft.Column(
            controls=[MyCheckboxes(),
                      ft.Row(controls=[ft.Column(
                          # controls=[],
                          alignment=ft.MainAxisAlignment.SPACE_BETWEEN), ],
                          alignment=ft.MainAxisAlignment.END),
                      ], alignment=ft.MainAxisAlignment.START, height=page.height)

    page.pubsub.subscribe(on_broadcast_message)

    page.overlay.append(main_struct())

    page.overlay.append(ft.Column(controls=[MySlider()], alignment=ft.MainAxisAlignment.END))

    page.add(MyMap())


if __name__ == '__main__':
    # uvicorn.run(ft.app(main, export_asgi_app=True), host="127.0.0.1", port=30000)
    ft.app(main)
