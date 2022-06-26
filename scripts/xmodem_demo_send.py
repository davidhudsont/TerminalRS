from time import sleep
import serial
from xmodem import XMODEM
import logging

# logging.basicConfig(level=logging.DEBUG)

# Timeout needs to be non zero or else it doen't work.
ser = serial.Serial('COM10', baudrate=115200, timeout=1)


def getc(size, timeout=1):
    logging.debug(f"Size: {size}")
    val = ser.read(size)
    logging.debug(f"Val: {val}")
    return val or None


def putc(data, timeout=1):
    logging.debug(f"Data: {data}")
    return ser.write(data)


mode = "xmodem1k"
mode = "xmodem"
modem = XMODEM(getc, putc, mode=mode)
path = "example.txt"
path = "C:/Users/David/Documents/Programming/ESP32/ESP32_Thing/ESP_IDF/6502_ESP32_IDF/sdkconfig"
stream = open(path, 'rb')
s = modem.send(stream=stream, retry=32, quiet=True)
print(s)

stream.close()
