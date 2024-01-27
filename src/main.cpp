#include "Adafruit_NeoPixel.h"
#include "HardwareSerial.h"
#include "api.h"
#include "peripherals.h"
#include "pins_arduino.h"
#include <Arduino.h>
#include <ArduinoJson.h>

JsonDocument doc;

Adafruit_NeoPixel pixels(NUMPIXELS, PINPIXELS, NEO_GRB + NEO_KHZ800);
void set_pixels(uint8_t r, uint8_t g, uint8_t b) {
  pixels.fill(pixels.Color(r, g, b), 0, 16);
  pixels.show();
}

void setup() {
  Serial.begin(9600);
  pixels.begin();
  set_pixels(0, 0, 0);

  pinMode(LED_BUILTIN, OUTPUT);

  pinMode(MIDDLE_LED.r, OUTPUT);
  pinMode(MIDDLE_LED.g, OUTPUT);
  pinMode(MIDDLE_LED.b, OUTPUT);
}

void send_response(Response *response) {
  Serial.println(serialize_response(response));
}

void parse_msg() {
  deserializeJson(doc, Serial);
  uint8_t op = doc["op"];
  Response def_response = {0, 200, "none"};
  Response *ok = &def_response;
  switch (op) {
  case HELLO: {
    hello_action(&pixels);
    ok->op = HELLO * -1;
    ok->job_name = "HELLO";
    send_response(ok);
    break;
  }
  case SET_RING: {
    set_ring(deserialize_set_ring(doc), &pixels);
    ok->op = SET_RING * -1;
    ok->job_name = "SET_RING";
    send_response(ok);
    break;
  }
  case SET_MIDDLE: {
    set_middle(deserialize_set_middle(doc), &pixels);
    ok->op = SET_MIDDLE * -1;
    ok->job_name = "SET_MIDDLE";
    send_response(ok);
    break;
  }
  case SET_PIN: {
    set_pin(deserialize_set_pin(doc));
    ok->op = SET_PIN * -1;
    ok->job_name = "SET_PIN";
    send_response(ok);
    break;
  }
  case GET_PIN: {
    get_pin_data(deserialize_get_pin(doc), ok);
    ok->op = GET_PIN * -1;
    ok->job_name = "GET_PIN";
    send_response(ok);
    break;
  }
  default:
    break;
  }
}

void loop() {
  digitalWrite(LED_BUILTIN, HIGH);
  parse_msg();
  digitalWrite(LED_BUILTIN, LOW);
}
