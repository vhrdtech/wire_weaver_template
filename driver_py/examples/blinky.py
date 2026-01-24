from time import sleep

import driver_py

print(dir(driver_py))

def main():
    drv = driver_py.DriverPy()
    drv.connect()

    drv.led_on()
    sleep(1.0)
    drv.led_off()
    sleep(1.0)

if __name__ == "__main__":
    main()