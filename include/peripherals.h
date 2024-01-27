#include <Arduino.h>
#define NUMPIXELS 16
#define PINPIXELS 6


typedef struct { uint8_t r; uint8_t g; uint8_t b; } middle_led;

const middle_led MIDDLE_LED = {9, 10, 11};

