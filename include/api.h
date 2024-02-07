#include "Adafruit_NeoPixel.h"
#include <ArduinoJson.h>
#include "WString.h"

// response will be negative of msg type
typedef enum {
    Hello = 1,
    SetRing = 2,
    SetMiddle = 3,
    SetPin = 4,
    GetPin = 5,
} OptCodes;

typedef struct {
  int8_t op;
  uint8_t msg_code;
  String msg;
} Response;

String serialize_response(Response * respones);

typedef struct {
  uint8_t pin;
  bool analog;
} get_pin_msg;
get_pin_msg deserialize_get_pin(const JsonDocument doc);
void get_pin_data(get_pin_msg msg , Response * res);


typedef struct {
  uint8_t pin;
  uint8_t value;
  bool analog;
} set_pin_msg;

set_pin_msg deserialize_set_pin(const JsonDocument doc);
void set_pin(set_pin_msg msg);

typedef struct {
  int op;
  String msg;
} HelloMsg;


void hello_action(Adafruit_NeoPixel * pixels);
typedef struct {
  struct {
    uint8_t r;
    uint8_t g;
    uint8_t b;
  } color;
  struct {
    uint8_t start;
    uint8_t stop;
  } fill;
} set_ring_msg;


set_ring_msg deserialize_set_ring(const JsonDocument doc);
void set_ring(set_ring_msg msg, Adafruit_NeoPixel * pixels);

typedef struct {
  struct {
    uint8_t r;
    uint8_t g;
    uint8_t b;
  } color;
} set_middle_msg;

set_middle_msg deserialize_set_middle(const JsonDocument doc);
void set_middle(set_middle_msg msg, Adafruit_NeoPixel * pixels);

