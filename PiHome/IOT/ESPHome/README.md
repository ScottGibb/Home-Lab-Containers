# Outdoor Watering System ESPHome configuration

## Ambient-light wiring

The `Ambient Light` entity is an approximate lux measurement from an LDR and a
10 kOhm fixed resistor wired as a voltage divider:

```text
3.3 V --- LDR ---+--- A0
                 |
              10 kOhm
                 |
                GND
```

This configuration targets a NodeMCU ESP8266 board, whose A0 input has an
on-board divider for a 0-3.3 V external input. A bare ESP8266 ADC only accepts
0-1.0 V: do not connect the 3.3 V divider to a bare ESP8266 A0 pin without an
appropriate additional divider.

The lux calculation is an estimate: LDR response varies by part and light
source. Adjust the `500.0f` and `1.4f` constants in `watering-device.yaml`
after comparing its reading with a known lux meter, if accuracy matters.
