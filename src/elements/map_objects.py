import flet.map as f_map


def my_map():
    return f_map.Map(
        expand=True,
        configuration=f_map.MapConfiguration(
            initial_center=f_map.MapLatitudeLongitude(42.703622, 3.038975),
            initial_zoom=15,
            interaction_configuration=f_map.MapInteractionConfiguration(
                flags=f_map.MapInteractiveFlag.ALL
            ),
            on_init=lambda e: print(f"Initialized Map"),
        ),
        layers=[
            f_map.TileLayer(
                url_template="https://tile.openstreetmap.org/{z}/{x}/{y}.png",
                on_image_error=lambda e: print("TileLayer Error"),
            ),

        ],
    )
