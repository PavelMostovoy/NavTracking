/*******************************************************************************************************

*******************************************************************************************************/

#define RXpin 15
#define TXpin 13
#define CSPIN 11


#include <HardwareSerial.h>
#include <M5Capsule.h>
#include <TinyGPSPlus.h>
#include <SD.h>


HardwareSerial GPS_port(2);

TinyGPSPlus gps;

void setup() {
  auto cfg = M5.config();
  M5Capsule.begin(cfg);
  SD.begin(CSPIN, SPI, 25000000);
  Serial.begin(115200);
  GPS_port.begin(9600, SERIAL_8N1, RXpin, TXpin);

  if (!SD.begin()) {
    Serial.println(
      "Card failed, or not present");
    while (1)
      ;
  }

  Serial.println("TF card initialized.");

  delay(1000);
}


void loop() {
  while (GPS_port.available() > 0)
    if (gps.encode(GPS_port.read()))
      displayInfo();

  if (millis() > 5000 && gps.charsProcessed() < 10) {
    Serial.println(F("No GPS detected: check wiring."));
    while (true)
      ;
  }
}

void displayInfo() {

  if (gps.date.isValid()) {
    String file_name = String("/") + String(gps.date.year()) + String("_") + String(gps.date.month()) + String("_") + String(gps.date.day()) + String(".log");

    if (gps.location.isValid() && (gps.location.age() < 1000)) {
      File myFile = SD.open(file_name, FILE_APPEND);

      if (gps.time.isValid()){
          if (gps.time.hour() < 10) myFile.print(F("0"));
          myFile.print(gps.time.hour());
          myFile.print(F(":"));
          if (gps.time.minute() < 10) myFile.print(F("0"));
          myFile.print(gps.time.minute());
          myFile.print(F(":"));
          if (gps.time.second() < 10) myFile.print(F("0"));
          myFile.print(gps.time.second());
          myFile.print(F("."));
          if (gps.time.centisecond() < 10) myFile.print(F("0"));
          myFile.print(gps.time.centisecond());
      } else {
          myFile.print("00:00:00.00");
      }
      myFile.print(",");
      myFile.print(gps.location.lat(), 6);
      myFile.print(",");
      myFile.print(gps.location.lng(), 6);
      myFile.print(",");
      if (gps.speed.isValid()) {
          myFile.print(gps.speed.kmph(), 3);
      } else {
          myFile.print(0.000, 3);
      }
      myFile.println();
      myFile.close();
      }
  }

  Serial.print(F("Location: "));
  if (gps.location.isValid()) {
    Serial.print(" Received");
  } else {
    Serial.print(F("INVALID"));
  }
  Serial.println();
}
