from struct import pack
from time import sleep
import serial
from xmodem import XMODEM
import logging

logging.basicConfig(level=logging.DEBUG)

# Timeout needs to be non zero or else it doen't work.
ser = serial.Serial('COM10', baudrate=115200, timeout=1)


def getc(size, timeout=1):
    logging.debug(f"Read {size} Bytes")
    val = ser.read(size)
    logging.debug(f"Bytes Received: {val}")
    return val or None


def putc(data, timeout=1):
    logging.debug(f"Bytes to Write: {data}")
    return ser.write(data)


modem = XMODEM(getc, putc)

stream = open("received.txt", 'wb')
b = modem.recv(stream=stream, crc_mode=1, retry=32)
print(b)
