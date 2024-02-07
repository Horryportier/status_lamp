#include "Adafruit_NeoPixel.h"
#include "HardwareSerial.h"
#include "api.h"
#include "peripherals.h"
#include "pins_arduino.h"
#include <Arduino.h>
#include <ArduinoJson.h>

JsonDocument doc;

Adafruit_NeoPixel pixels(NUMPIXELS, PINPIXELS, NEO_GRB + NEO_KHZ800);

void setup() {
  Serial.begin(9600);
  pixels.begin();
  pixels.fill(pixels.Color(0,0,0), 0, NUMPIXELS);

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
  case Hello: {
    hello_action(&pixels);
    ok->op = Hello ;
    String msg = doc["data"]["msg"];
    ok->msg = msg;
    send_response(ok);
    break;
  }
  case SetRing: {
    set_ring(deserialize_set_ring(doc), &pixels);
    ok->op = SetRing ;
    send_response(ok);
    break;
  }
  case SetMiddle: {
    set_middle(deserialize_set_middle(doc), &pixels);
    ok->op = SetMiddle ;
    send_response(ok);
    break;
  }
  case SetPin: {
    set_pin_msg msg =  deserialize_set_pin(doc);
    set_pin(msg);
    ok->op = SetPin ;
    ok->msg =  "pin: " + String(msg.pin) + " set to: " + String(msg.value) + "a/d: " + String(msg.analog);
    send_response(ok);
    break;
  }
  case GetPin: {
    get_pin_data(deserialize_get_pin(doc), ok);
    ok->op = GetPin ;
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
