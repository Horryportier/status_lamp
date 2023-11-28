#include <ArduinoJson.h>
#include <Adafruit_NeoPixel.h>

#define NUM_PIXELS 16
#define PIXELS_PIN 6

#define DEBUG 1

#define STATUS_R 9
#define STATUS_G 10
#define STATUS_B 11

#define RELAY_1 2
#define RELAY_2 3



#if DEBUG
#define debug(x) Serial.println(x)
#else
#define debug(x)
#endif

Adafruit_NeoPixel strip(NUM_PIXELS, PIXELS_PIN, NEO_GRB + NEO_KHZ800);

void animation() {
}

void set_center(uint8_t red, uint8_t green,uint8_t blue) {
  analogWrite(STATUS_R, red);
  analogWrite(STATUS_G, green);
  analogWrite(STATUS_B, blue);
}

void set_pin(uint8_t pin, int value, bool analog) {
  if (analog) {
    analogWrite(pin, value);
  } else {
    digitalWrite(pin, value);
  }
}
  

void begin() {
  // setup led ring and flash red for 500ms 
  debug("startup!");
  strip.begin();
  strip.clear();


  // setup status led
  
  pinMode(STATUS_R, OUTPUT);
  pinMode(STATUS_G, OUTPUT);
  pinMode(STATUS_B, OUTPUT);

  pinMode(RELAY_1, OUTPUT);
  pinMode(RELAY_2, OUTPUT);
}

// msg scheme
// {
// "msg_type": "set_ring"
// msg {
//  all msg data
// }
//} 
// msg_types
// set_ring { [16 ;rgb, rgb, rgb] }
// set_center { rgb }
// switch_pin { pin, state }
// toggle_pin { pin }
void read_msg() {
 
  DynamicJsonDocument doc(1024);
  deserializeJson(doc, Serial);

  String msg_type = doc["msg"][0];
  debug(msg_type);
  if ( msg_type == "Pixel_Strip" ) {
    for(int i = 0; i < NUM_PIXELS; i++) {
      strip.setPixelColor(i, strip.Color(doc["msg"][1][i]["r"],doc["msg"][1][i]["g"],doc["msg"][1][i]["b"]));
      strip.show();
    }
  }
  if ( msg_type == "Center_Led" ) {
      String x = doc["msg"][0];
      debug(x);
      set_center(doc["msg"][1]["r"],doc["msg"][1]["g"],doc["msg"][1]["b"]);
  }
  // { "msg_type": "switch_pin", "msg": { "pin": 2, "value": 1} }
  if ( msg_type == "PIN" ) {
     set_pin(doc["msg"][1]["pin"],doc["msg"][1]["value"],doc["msg"][1]["analog"]);
  }
  if ( msg_type == "animation" ) {
  }
 
   
}

void setup() {
  Serial.begin(9600);
  begin();
}

void loop() {
  read_msg();
}
