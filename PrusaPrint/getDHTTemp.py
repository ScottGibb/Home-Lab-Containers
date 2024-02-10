import sys
import time
import board
import adafruit_dht
import os

TMP_RESULTS_PATH = "/tmp/oprint/enclosure_plugin/"
os.makedirs(exist_ok = True, name =TMP_RESULTS_PATH)

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
    sys.exit(2)

dhtDevice = sensor(pin)

# DHT sensor read fails quite often, causing enclosure plugin to report value of 0.
# If this happens, retry as suggested in the adafruit_dht docs.
MAX_RETRIES = 10
retry_count = 0
while retry_count <= MAX_RETRIES:
    try:
        humidity=dhtDevice.humidity
        temperature=dhtDevice.temperature

        if humidity is not None and temperature is not None:
            results_str = ('{0:0.1f} | {1:0.1f}'.format(temperature, humidity))
            print(results_str)
            with open(f"{TMP_RESULTS_PATH}/dht_results.txt", "w") as file:
                file.write(results_str)
            sys.exit(1)

    except RuntimeError as e:
        # Try Again?
        continue
    time.sleep(0.1)
    retry_count += 1

# If all else fails read the last saved result
with open(f"{TMP_RESULTS_PATH}/dht_results.txt", "r") as file:
    results_str =file.readline()
    print(results_str)
sys.exit(3)
