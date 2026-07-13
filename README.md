
>8-legged robot controlled via companion application.

---
# Components

1) MCU - [RaspberryPi pico 2 wh](https://www.gme.cz/v/1517388/raspberry-pico-2-wh) 260 kč
2) Servo (24) - [Micro servo SG90](https://dratek.cz/arduino-platforma/180244-servomotor-sg90-360-kontinualni.html) 60 kč/ks
3) Display - [OLED displej 0,96"](https://www.gme.cz/v/1509095/oled-displej-096-128x64-i2c-bily) 100 kč
4) Servo module [24-kanálový modul](https://www.gme.cz/v/1508859/modul-24-kanalovy-budic-servo-motoru) 496 kč
5) Battery [GNB 1100mAh 2S](https://www.rotorama.cz/product/gnb-1100mah-2s-60c-hv) 279 kč
6) Step-down convertor (2) [XL4005](https://www.laskakit.cz/step-down-menic-s-xl4005/) 48 kč

---
# Physical attributes

## Legs

>Each leg consists of 3 parts: Coxa, Femur and Tibia

$$
Coxa \approx 6,464 cm^3
$$
$$
Femur \approx 4,4516cm^3
$$
$$
Tibia \approx 10,552cm^3
$$
>From this I calculated weight for each leg.

$$
Leg = (Coxa + Femur + Tibia) \cdot MaterialDensity + 3 \cdot Servo
$$

>Servo motors used weight $9g$ and I used `PLA` as my material for parts that means:

$$
Leg \approx 54,26g
$$

$$
AllLegs \approx 434,12g
$$

Leg lengths:

| Coxa  | 5cm  |
| ----- | ---- |
| Femur | 8cm  |
| Tibia | 13cm |

---
## Electronic components

>Weight of each electrical component used.

$$
Battery = 46g
$$

$$
StepDownConvertor = 66g
$$

$$
ServoModule \approx 60g
$$

$$
MCU = 6g
$$

$$
Display = 3g
$$

>Electrical components (without servos) weight around: $247g$.

---
## Summary of Physical attributes

$$
LowerTorso \approx 128,27g
$$

$$
RoofTorso \approx 95,25g
$$

$$
Leg \approx 8 \cdot 55g
$$

$$
ElectricalParts \approx 247g
$$

---

$$
TotalWeight \approx 910,52g
$$

---
