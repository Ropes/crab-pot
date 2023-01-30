Reading ArduinoUNO serial
-------------------------

ArduinoUNO running the IMU code at: https://github.com/Ropes/arduino/blob/main/imu-viz/imu-viz.ino writes X, Y, Z values over Serial as well to the display.

eg:
`20:14:51.151 -> X: 13.0625	Y: -9.8750	Z: 11.7500`

## Running

`cargo build` writes binary to target/debug/serial-loop.

*run as root as a quick fix to access the device*

`sudo ./target/debug/serial-loop /dev/ttyACM0 9600`
```
X: 345.8125	Y: -32.5000	Z: -5.4375
X: 344.7500	Y: -16.4375	Z: -4.0625
X: 345.0625	Y: -2.1250	Z: -3.8750
X: 348.4375	Y: 2.5625	Z: -1.3125
X: 348.3750	Y: 20.8125	Z: 0.1875
X: 349.0625	Y: 35.7500	Z: -0.7500
X: 344.3750	Y: 28.3125	Z: 2.3125
X: 340.1250	Y: 8.7500	Z: 0.0625
X: 339.8125	Y: 0.8125	Z: -0.7500
X: 335.8750	Y: -11.7500	Z: -6.3750
X: 324.1875	Y: -15.0625	Z: -10.4375
X: 322.8125	Y: -15.5625	Z: -18.0000
X: 324.6250	Y: -10.5000	Z: -23.6875
X: 329.7500	Y: -5.1875	Z: -39.6250
X: 331.5000	Y: -3.9375	Z: -39.7500
X: 332.5625	Y: -3.2500	Z: -24.9375
X: 333.8125	Y: -0.5625	Z: -8.8750
X: 335.1250	Y: 1.4375	Z: 1.6875
X: 337.1250	Y: 2.0000	Z: 11.5625
X: 337.7500	Y: -0.9375	Z: 13.0000
X: 336.7500	Y: -4.3750	Z: 8.1250
X: 337.3750	Y: -9.4375	Z: 2.5000
X: 336.6250	Y: -12.1875	Z: 3.6250
```
