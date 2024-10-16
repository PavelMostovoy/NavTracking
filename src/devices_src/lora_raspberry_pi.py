"""
Raspberry Pi Lora, with sx1262 board and tft screen.
"""

import asyncio
import flet as ft
import flet.map as map

from LoRaRF import SX126x

# Begin LoRa radio and set NSS, reset, busy, IRQ, txen, and rxen pin with connected Raspberry Pi gpio pins
busId = 0
csId = 0
resetPin = 18
busyPin = 20
irqPin = 16
txenPin = 6
rxenPin = -1
LoRa = SX126x()
print("Begin LoRa radio")
if not LoRa.begin(busId, csId, resetPin, busyPin, irqPin, txenPin, rxenPin):
    raise Exception("Something wrong, can't begin LoRa radio")

LoRa.setDio2RfSwitch()
# Set frequency to 868 Mhz
print("Set frequency to 868 Mhz")
LoRa.setFrequency(868000000)

# Set RX gain to power gain
LoRa.setRxGain(LoRa.RX_GAIN_BOOSTED)

sf = 7
bw = 125000
cr = 5
LoRa.setLoRaModulation(sf, bw, cr)
headerType = LoRa.HEADER_EXPLICIT
preambleLength = 12
payloadLength = 50
crcType = True
LoRa.setLoRaPacket(headerType, preambleLength, payloadLength, crcType)

LoRa.request(LoRa.RX_CONTINUOUS)

marker_layer_ref = ft.Ref[map.MarkerLayer]()

async def timer_task(page, marker):
    while True:
        if LoRa.available():

            # Put received packet to message and counter variable
            message = ""
            while LoRa.available() > 1:
                message += chr(LoRa.read())
            counter = LoRa.read()

            # Print received message and counter in serial
            # example *;FRA5555;9:55:38:0;42.684662;3.034344;0.69;*
            # print(f" Msg :{message}  {counter}")
            msg = message.split(";")
            if len(msg) == 7:
                try:
                    lat = float(msg[3])
                    lon = float(msg[4])
                    sog = round(float(msg[5]),2)
                    marker.current.markers.clear()
                    marker.current.markers.append(map.Marker(
                        content=ft.Icon(ft.icons.MY_LOCATION_SHARP,
                                        tooltip=ft.Tooltip(f"{sog}")),
                        coordinates=map.MapLatitudeLongitude(lat, lon),
                    ))
                except Exception as e:
                    print(f"Incorrect message{msg}")
            else:
                print(msg)
            page.update()
        await asyncio.sleep(0)

def long_press_update(e):
    e.page.clean()
    coordinates = e.coordinates
    e.page.add(my_map(coordinates))


def my_map(coordinates):
    my_map = map.Map(
            expand=True,
            configuration=map.MapConfiguration(
                initial_center=coordinates,
                initial_zoom=10.0,
                interaction_configuration=map.MapInteractionConfiguration(
                    flags=map.MapInteractiveFlag.ALL
                ),
                on_init=lambda e: print("Map initialised"),
                on_long_press= long_press_update
            ),
            layers=[
                map.TileLayer(
                    url_template="https://tile.openstreetmap.org/{z}/{x}/{y}.png",
                    # on_image_error=lambda e: print("TileLayer Error"),
                ),
                map.MarkerLayer(
                    ref=marker_layer_ref,
                    markers=[
                        map.Marker(
                            content=ft.Icon(ft.icons.MY_LOCATION_SHARP),
                            coordinates=map.MapLatitudeLongitude(42, 3.03),
                        ),
                    ],
                ),
            ],
        )
    return my_map

async def main(page: ft.Page):
    page.add(
            my_map(map.MapLatitudeLongitude(42.6, 3))
    )

    await timer_task(page, marker_layer_ref)


ft.app(main)
