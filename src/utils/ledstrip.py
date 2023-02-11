import asyncio
import colorsys
import time

import rpi_ws281x

from utils.color import convert_ha_temperature, convert_K_to_RGB, wheel
from utils.ledstripconfig import LedStripConfig


class LedStripController:
    def __init__(self) -> None:
        self.strip: rpi_ws281x.Adafruit_NeoPixel = self.led_strip_init()
        self.sequence_task: asyncio.Task
        self.sequence_cancel_task: asyncio.Event = asyncio.Event()
        self.last_rgb: tuple[int, int, int] = (255, 255, 255)

    def led_strip_init(self) -> rpi_ws281x.Adafruit_NeoPixel:
        try:
            strip = rpi_ws281x.Adafruit_NeoPixel(
                LedStripConfig.COUNT,
                LedStripConfig.PIN,
                LedStripConfig.FREQ_HZ,
                LedStripConfig.DMA,
                LedStripConfig.INVERT,
                LedStripConfig.BRIGHTNESS,
                LedStripConfig.CHANNEL,
            )
            strip.begin()
            print("Controller initialization completed successfully.")
            return strip
        except RuntimeError:
            print("Controller initialization failed")

    async def terminate_task(self) -> None:
        if self.sequence_task is not None:
            self.sequence_cancel_task.clear()
            await self.sequence_task
            self.sequence_task = None

    async def brightness(self, brightness: int):
        if self.sequence_task is None:
            self.strip.setBrightness(
                int(255 * (brightness / 100)))
            self.strip.show()
        else:
            last_sequence = getattr(self.sequence_task.get_name())
            await self.terminate_task()
            self.strip.setBrightness(
                int(255 * (brightness / 100)))
            self.strip.show()
            await last_sequence()

    async def temperature(self, temperature: int):
        await self.terminate_task()
        temperature = convert_ha_temperature(temperature)
        r, g, b = convert_K_to_RGB(temperature)
        self.last_rgb = (r, g, b)
        for i in range(self.strip.numPixels()):
            self.strip.setPixelColorRGB(i, r, b, g)
        self.strip.show()
        self.sequence_task.set_name("temperature")

    async def on(self):
        await self.terminate_task()
        for i in range(self.strip.numPixels()):
            self.strip.setPixelColorRGB(
                i, self.last_rgb[0], self.last_rgb[2], self.last_rgb[1]
            )
        self.strip.show()

    async def off(self):
        await self.terminate_task()
        for i in range(self.strip.numPixels()):
            self.strip.setPixelColorRGB(i, 0, 0, 0)
        self.strip.show()

    async def hsv(self, h: int, s: int, v: int):
        await self.terminate_task()
        r, g, b = tuple(
            round(i * 255)
            for i in colorsys.hsv_to_rgb(
                h / 360, s / 100, v / 100
            )
        )
        self.last_rgb = (r, g, b)
        for i in range(self.strip.numPixels()):
            self.strip.setPixelColorRGB(i, r, b, g)
        self.strip.show()
        self.sequence_task.set_name("HSV")

    async def rgb(self, r: int, g: int, b: int):
        await self.terminate_task()
        for i in range(self.strip.numPixels()):
            self.strip.setPixelColorRGB(i, r, b, g)
        self.strip.show()
        self.sequence_task.set_name("RGB")

    async def rainbow(self) -> None:
        await self.terminate_task()
        self.sequence_task = asyncio.create_task(self.rainbow_loop)
        self.sequence_task.set_name("rainbow_loop")

    async def rainbow_loop(self) -> None:
        running = True
        while running:
            for j in range(255):
                if not self.sequence_cancel_task.is_set():
                    running = False
                    break
                for i in range(self.strip.numPixels()):
                    self.strip.setPixelColor(i, wheel((i + j) & 255))
                self.strip.show()
                time.sleep(0.05)

    async def rainbow_cycle(self):
        await self.terminate_task()
        self.sequence_task = asyncio.create_task(self.rainbow_cycle_loop)
        self.sequence_task.set_name("rainbow_cycle_loop")

    async def rainbow_cycle_loop(self) -> None:
        running = True
        while running:
            for j in range(255):
                if not self.sequence_cancel_task.is_set():
                    running = False
                    break
                for i in range(self.strip.numPixels()):
                    self.strip.setPixelColor(
                        i, wheel(
                            (int(i * 256 / self.strip.numPixels()) + j) & 255)
                    )
                self.strip.show()
                time.sleep(0.05)
