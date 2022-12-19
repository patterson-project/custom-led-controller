from fastapi import APIRouter, HTTPException, Response

from models.dtos import BrightnessDto, HsvDto, OperationDto, TemperatureDto
from utils.ledstrip import LedStripController

led_strip: LedStripController = LedStripController()
router = APIRouter()


@router.get(
    path="/on", summary="Turning on the LED Strip"
)
async def strip_on():
    await led_strip.on()
    return Response(status_code=200)


@router.get(
    path="/off", summary="Turning off the LED Strip"
)
async def strip_off():
    await led_strip.off()
    return Response(status_code=200)


@router.post(
    path="/temperature", summary="Setting a temperature", response_description="Temperature Set"
)
async def strip_set_temperature(temperature: TemperatureDto):
    await led_strip.temperature(temperature=temperature)
    return Response(status_code=200)


@router.post(
    path="/hsv", summary="Setting a HSV", response_description="HSV Set"
)
async def strip_set_Hsv(hsv: HsvDto):
    await led_strip.hsv(hsv.h, hsv.s, hsv.v)
    return Response(status_code=200)


@router.post(
    path="/brightness", summary="Setting a brightness", response_description="Brightness Set"
)
async def strip_set_brightness(brightness_dto: BrightnessDto):
    await led_strip.brightness(brightness_dto.brightness)
    return Response(status_code=200)


@router.post(
    path="/operation", summary="Setting a special operation", response_description="Operation Set"
)
async def strip_operation(operation_request: OperationDto):
    try:
        operation = getattr(led_strip, operation_request.operation)
        await operation()
        return Response(status_code=200)
    except AttributeError:
        raise HTTPException(
            status_code=404, detail="Invalid operation"
        )