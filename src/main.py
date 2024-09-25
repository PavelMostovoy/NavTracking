import flet as ft
from elements.map_objects import my_map
from elements.controls import my_slider, my_checkboxes, monitoring_container, slider_ref, data_monitor
from helpers.utility import resized



def main(page: ft.Page):
    page.overlay.append(ft.Column(
                                  controls=[my_checkboxes,
                                            ft.Row(controls=[ft.Column(ref = data_monitor,
                                                                       controls=[],
                                                                       alignment=ft.MainAxisAlignment.SPACE_BETWEEN),],
                                                   alignment=ft.MainAxisAlignment.END),
                                            ], alignment=ft.MainAxisAlignment.START, height=page.height))

    page.overlay.append(ft.Column(controls=[my_slider(page)], alignment=ft.MainAxisAlignment.END))

    page.add(my_map)
    page.on_resized = lambda x: resized(page, [slider_ref])

if __name__ == '__main__':
    ft.app(main)
