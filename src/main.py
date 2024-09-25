import flet as ft
import uvicorn

from elements.map_objects import my_map
from elements.controls import my_slider, my_checkboxes, slider_ref
from helpers.utility import resized



def main(page: ft.Page):
    page.overlay.append(ft.Column(
                                  controls=[my_checkboxes(),
                                            ft.Row(controls=[ft.Column(
                                                                       # controls=[],
                                                                       alignment=ft.MainAxisAlignment.SPACE_BETWEEN),],
                                                   alignment=ft.MainAxisAlignment.END),
                                            ], alignment=ft.MainAxisAlignment.START, height=page.height))

    page.overlay.append(ft.Column(controls=[my_slider(page)], alignment=ft.MainAxisAlignment.END))

    page.add(my_map())
    page.on_resized = lambda x: resized(page, [slider_ref])

if __name__ == '__main__':
    # uvicorn.run(ft.app(main, export_asgi_app=True), host="127.0.0.1", port=30000)
    ft.app(main)

