import flet as ft
import flet.map as f_map

slider_ref = ft.Ref[ft.Slider]()
data_container = ft.Ref[ft.Container]()
polyline_layer_ref = ft.Ref[f_map.PolylineLayer]()
data_monitor = ft.Ref[ft.Column]()

def manage_data_container(e):
    if e.control.value:
        containers = data_monitor.current.controls
        for container in containers:
            if container.content.value == e.control.label:
                return
        container = monitoring_container()
        container.content = ft.Text(e.control.label)
        container.bgcolor = e.control.active_color
        data_monitor.current.controls.append(container)
        e.page.update()
    else:
        containers = data_monitor.current.controls
        for i, container in enumerate(containers):
            if container.content.value == e.control.label:
                containers.pop(i)
                e.page.update()




def checkbox(color:ft.colors,text ):
    obj = ft.Checkbox(adaptive=True,
                      label=text,
                      value=False,
                      active_color=color,
                      on_change=manage_data_container)
    return obj

my_checkboxes = ft.Row(
    controls=[checkbox(ft.colors.RED,"FRA 5555"),
              checkbox(ft.colors.GREEN,"FRA 4455"),
              checkbox(ft.colors.BLUE,"FRA 3355"),
              checkbox(ft.colors.ORANGE,"FRA 2255")],
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