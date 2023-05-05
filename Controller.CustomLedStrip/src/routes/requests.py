from fastapi import APIRouter, HTTPException, Response

from models.dtos import (BrightnessDto, HsvDto, OperationDto, RgbDto,
                         TemperatureDto)
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
async def strip_set_temperature(temperature_dto: TemperatureDto):
    await led_strip.temperature(temperature_dto.temperature)
    return Response(status_code=200)


@router.post(
    path="/hsv", summary="Setting an HSV", response_description="HSV Set"
)
async def strip_set_Hsv(hsv_dto: HsvDto):
    await led_strip.hsv(hsv_dto.h, hsv_dto.s, hsv_dto.v)
    return Response(status_code=200)


@router.post(
    path="/rgb", summary="Setting an RGB", response_description="RGB Set"
)
async def strip_set_Rgb(rgb_dto: RgbDto):
    rounded_rgb_dto = [int(round(value)) for value in rgb_dto.rgb_color]
    await led_strip.rgb(rounded_rgb_dto[0], rounded_rgb_dto[1], rounded_rgb_dto[2])
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