import serial
import logging

logging.basicConfig(level=logging.DEBUG)

# Timeout needs to be non zero or else it doen't work.
ser = serial.Serial('COM10', baudrate=115200, timeout=1)

response = b"""
 ID: 0ABERSFSE000fsdfj
 PARTNO: ABSDFKSOFAJF012312
 DATE: 6/25/2022
 TIME: 02:23

 >>
"""


while True:
    val = ser.read()
    if val:
        print(val)
        if val == b'\n':
            print("send response")
            ser.write(response)
        elif val == b'\r':
            print("send response")
            ser.write(response)
        else:
            ser.write(val)
