from fastapi import APIRouter, HTTPException, Response

from models.dtos import BrightnessDto, HsvDto, OperationDto, TemperatureDto
from utils.ledstrip import LedStripController

led_strip: LedStripController = LedStripController()
router = APIRouter()


@router.get(
    path="/on", summary="Turning on the LED Strip"
)
async def strip_on():
    led_strip.on()
    return Response(status_code=200)


@router.get(
    path="/off", summary="Turning off the LED Strip"
)
async def strip_off():
    led_strip.off()
    return Response(status_code=200)


@router.post(
    path="/temperature", summary="Setting a temperature", response_description="Temperature Set"
)
async def strip_set_temperature(temperature: TemperatureDto):
    led_strip.temperature(temperature=temperature)
    return Response(status_code=200)


@router.post(
    path="/hsv", summary="Setting a HSV", response_description="HSV Set"
)
async def strip_set_Hsv(Hsv: HsvDto):
    led_strip.hsv(h=Hsv.h, s=Hsv.s, v=Hsv.v)
    return Response(status_code=200)


@router.post(
    path="/brightness", summary="Setting a brightness", response_description="Brightness Set"
)
async def strip_set_brightness(brightness: BrightnessDto):
    led_strip.brightness(brightness=brightness)
    return Response(status_code=200)


@router.post(
    path="/operation", summary="Setting a special operation", response_description="Operation Set"
)
async def strip_operation(self, operation_request: OperationDto):
    try:
        operation = getattr(self, operation_request)
        await operation(operation_request)
    except AttributeError:
        raise HTTPException(
            status_code=400, detail="Invalid Operation"
        )
    if operation == "rainbow_cycle_loop":
        led_strip.rainbow_cycle_loop()
        return Response(status_code=200)

    if operation == "rainbow_loop":
        led_strip.rainbow_loop()
        return Response(status_code=200)


    
