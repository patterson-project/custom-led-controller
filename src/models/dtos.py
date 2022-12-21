from pydantic import BaseModel


class BrightnessDto(BaseModel):
    brightness: int

class RgbDto(BaseModel):
    r: int
    g: int
    b: int

class HsvDto(BaseModel):
    h: int
    s: int
    v: int

class TemperatureDto(BaseModel):
    temperature: int

class OperationDto(BaseModel):
    operation: str