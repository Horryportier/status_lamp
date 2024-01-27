#include "api.h"
#include "Adafruit_NeoPixel.h"
#include "Arduino.h"
#include "ArduinoJson/Document/JsonDocument.hpp"
#include "WString.h"
#include <ArduinoJson.h>
#include "peripherals.h"
String serialize_response(Response *respones) {
  JsonDocument doc;
  doc["op"] = respones->op;
  doc["msg_code"] = respones->msg_code;
  doc["job_name"] = respones->job_name;
  doc["msg"] = respones->msg;
  String res = "";
  serializeJson(doc, res);
  return res;
}

void hello_action(Adafruit_NeoPixel * pixels) {
  pixels->fill(pixels->Color(20, 0,0), 0, 16);
  pixels->show();
  delay(100);
  pixels->fill(pixels->Color(0, 20,0), 0, 16);
  pixels->show();
  delay(100);
  pixels->fill(pixels->Color(0, 0,20), 0, 16);
  pixels->show();
  delay(100);
  pixels->fill(pixels->Color(0, 0,0), 0, 16);
  pixels->show();
}

set_ring_msg deserialize_set_ring(const JsonDocument doc){
  set_ring_msg ring  = {};
  ring.color.r = doc["data"]["color"]["r"];
  ring.color.g = doc["data"]["color"]["g"];
  ring.color.b = doc["data"]["color"]["b"];

  ring.fill.start = doc["data"]["fill"]["start"];
  ring.fill.stop = doc["data"]["fill"]["stop"];
  return  ring;
}

void set_ring(set_ring_msg msg, Adafruit_NeoPixel * pixels) {
  pixels->fill(pixels->Color(msg.color.r, msg.color.g, msg.color.b), msg.fill.start, msg.fill.stop);
  pixels->show();
}


set_middle_msg deserialize_set_middle(const JsonDocument doc) {
  set_middle_msg middle  = {};
  middle.color.r = doc["data"]["color"]["r"];
  middle.color.g = doc["data"]["color"]["g"];
  middle.color.b = doc["data"]["color"]["b"];
  return  middle;
}
void set_middle(set_middle_msg msg, Adafruit_NeoPixel * pixels){ 
  analogWrite(MIDDLE_LED.r, msg.color.r);
  analogWrite(MIDDLE_LED.g, msg.color.g);
  analogWrite(MIDDLE_LED.b, msg.color.b);
}


set_pin_msg deserialize_set_pin(const JsonDocument doc) {
  set_pin_msg pin  = {};
  pin.pin = doc["data"]["pin"];
  pin.value = doc["data"]["value"];
  pin.analog = doc["data"]["analog"];
  return  pin;
}
void set_pin(set_pin_msg msg) {
  pinMode(msg.pin, OUTPUT);
  if (msg.analog) {
    analogWrite(msg.pin, msg.value);
  } else {
    digitalWrite(msg.pin, msg.value);
  }
}

get_pin_msg deserialize_get_pin(const JsonDocument doc) {
  get_pin_msg pin  = {};
  pin.pin = doc["data"]["pin"];
  pin.analog = doc["data"]["analog"];
  return  pin;
}

void get_pin_data(get_pin_msg msg , Response * res) {
  if (msg.analog) {
    res->msg = analogRead(msg.pin);
  } else {
    res->msg = digitalRead(msg.pin);
  }
}

