import serial
import logging

logging.basicConfig(level=logging.DEBUG)

# Timeout needs to be non zero or else it doen't work.
ser = serial.Serial('COM10', baudrate=115200, timeout=1)


while True:
    val = ser.read()
    if val:
        ser.write(val)
