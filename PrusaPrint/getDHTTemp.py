import sys
import time
import board
import adafruit_dht

# Parse command line parameters.
sensor_args =   {
                    '11': adafruit_dht.DHT11,
                    '22': adafruit_dht.DHT22,
                    '2302': adafruit_dht.DHT22
                }

if len(sys.argv) == 3 and sys.argv[1] in sensor_args:
    sensor = sensor_args[sys.argv[1]]
    pin = "D%s" % sys.argv[2]
    pin = getattr(board,pin)
else:
    sys.exit(1)

dhtDevice = sensor(pin)

# DHT sensor read fails quite often, causing enclosure plugin to report value of 0.
# If this happens, retry as suggested in the adafruit_dht docs.
max_retries = 3
retry_count = 0
while retry_count <= max_retries:
#    try:
     if True:
        humidity=dhtDevice.humidity
        temperature=dhtDevice.temperature

        if humidity is not None and temperature is not None:
            print(('{0:0.1f} | {1:0.1f}'.format(temperature, humidity)))
            sys.exit(1)
#    except Exception as e:
#        print('-1 | -1')
#        sys.exit(1)

     time.sleep(1)
     retry_count += 1

print('-1 | -1')
sys.exit(1)
