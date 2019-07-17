#!/usr/bin/python
# -*- coding:utf-8 -*-

import epd2in13
import time
from PIL import Image,ImageDraw,ImageFont
import traceback

font = "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf"
epd = epd2in13.EPD()

def init():
    epd.init(epd.lut_full_update)

def draw_loop():
    image = Image.new('1', (epd2in13.EPD_HEIGHT, epd2in13.EPD_WIDTH), 255)
    draw = ImageDraw.Draw(image)
    font24 = ImageFont.truetype(font, 24)

    epd.Clear(0xFF)
    draw.text((10, 10), "eHlo oWrld!", font = font24, fill = 0)
    epd.display(epd.getbuffer(image.rotate(0)))
    epd.sleep()

init()
draw_loop()