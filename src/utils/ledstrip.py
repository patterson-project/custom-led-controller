import colorsys
import multiprocessing
import time

import rpi_ws281x

from utils.color import convert_K_to_RGB, wheel
from utils.ledstripconfig import LedStripConfig


class LedStripController:
    def __init__(self) -> None:
        self.strip: rpi_ws281x.Adafruit_NeoPixel = self.led_strip_init()
        self.sequence_process: multiprocessing.Process = None
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

    def terminate_process(self) -> None:
        if self.sequence_process is not None:
            self.sequence_process.terminate()
            self.sequence_process.join()
            self.sequence_process = None


    def brightness(self, brightness: int):
        if self.sequence_process is None:
            self.strip.setBrightness(
                int(255 * (brightness / 100)))
            self.strip.show()
        else:
            last_sequence = self.sequence_process.name
            self.terminate_process()
            self.strip.setBrightness(
                int(255 * (brightness / 100)))
            self.strip.show()
            self.operation_callback_by_name[last_sequence]()


    def temperature(self, temperature: int):
        self.terminate_process()
        r, g, b = convert_K_to_RGB(temperature)
        self.last_rgb = (r, g, b)
        for i in range(self.strip.numPixels()):
            self.strip.setPixelColorRGB(i, r, b, g)
        self.strip.show()


    def on(self):
        self.terminate_process()
        for i in range(self.strip.numPixels()):
            self.strip.setPixelColorRGB(
                i, self.last_rgb[0], self.last_rgb[2], self.last_rgb[1]
            )
        self.strip.show()


    def off(self):
        self.terminate_process()
        for i in range(self.strip.numPixels()):
            self.strip.setPixelColorRGB(i, 0, 0, 0)
        self.strip.show()


    def hsv(self, h: int, s: int, v: int):
        self.terminate_process()
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


    def rainbow(self) -> None:
        self.terminate_process()
        self.sequence_process = multiprocessing.Process(
            target=self.rainbow_loop)
        self.sequence_process.name = "rainbow"
        self.sequence_process.start()


    def rainbow_loop(self) -> None:
        while True:
            for j in range(255):
                for i in range(self.strip.numPixels()):
                    self.strip.setPixelColor(i, wheel((i + j) & 255))
                self.strip.show()
                time.sleep(0.05)


    def rainbow_cycle(self):
        self.terminate_process()
        self.sequence_process = multiprocessing.Process(
            target=self.rainbow_cycle_loop)
        self.sequence_process.name = "rainbow_cycle"
        self.sequence_process.start()


    def rainbow_cycle_loop(self) -> None:
        while True:
            for j in range(255):
                for i in range(self.strip.numPixels()):
                    self.strip.setPixelColor(
                        i, wheel(
                            (int(i * 256 / self.strip.numPixels()) + j) & 255)
                    )
                self.strip.show()
                time.sleep(0.05)