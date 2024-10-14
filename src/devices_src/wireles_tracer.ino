/*
 * HelTec Wireless Tracer v1.1
 * www.heltec.org
 * */

#include "LoRaWan_APP.h"
#include "Arduino.h"
#include "HT_TinyGPS++.h"


#define RF_FREQUENCY 868000000  // Hz

#define TX_OUTPUT_POWER 10  // dBm

#define LORA_BANDWIDTH 0         // [0: 125 kHz, \
                                 //  1: 250 kHz, \
                                 //  2: 500 kHz, \
                                 //  3: Reserved]
#define LORA_SPREADING_FACTOR 7  // [SF7..SF12]
#define LORA_CODINGRATE 1        // [1: 4/5, \
                                 //  2: 4/6, \
                                 //  3: 4/7, \
                                 //  4: 4/8]
#define LORA_PREAMBLE_LENGTH 12  // Same for Tx and Rx
#define LORA_SYMBOL_TIMEOUT 0    // Symbols
#define LORA_FIX_LENGTH_PAYLOAD_ON false
#define LORA_IQ_INVERSION_ON false


#define RX_TIMEOUT_VALUE 1000
#define BUFFER_SIZE 50  // Define the payload size here


#define VBAT_READ 1
#define ADC_CTRL 2

#define VGNSS_CTRL Vext

bool GPS_DATA = false;
String time_str = "";
float latitude;
float longitude;
float sog;
float seconds;
String Identifier = "FRA5555";

char txpacket[BUFFER_SIZE];

bool lora_idle = true;

uint8_t syncWord = 0x3333;

static RadioEvents_t RadioEvents;
void OnTxDone(void);
void OnTxTimeout(void);
TinyGPSPlus gps;

void setup() {
  Serial.begin(115200);
  Mcu.begin(HELTEC_BOARD, SLOW_CLK_TPYE);

  RadioEvents.TxDone = OnTxDone;
  RadioEvents.TxTimeout = OnTxTimeout;

  Radio.Init(&RadioEvents);
  Radio.SetChannel(RF_FREQUENCY);
  Radio.SetTxConfig(MODEM_LORA, TX_OUTPUT_POWER, 0, LORA_BANDWIDTH,
                    LORA_SPREADING_FACTOR, LORA_CODINGRATE,
                    LORA_PREAMBLE_LENGTH, LORA_FIX_LENGTH_PAYLOAD_ON,
                    true, 0, 0, LORA_IQ_INVERSION_ON, 3000);
  Radio.SetSyncWord(syncWord);

  pinMode(VGNSS_CTRL, OUTPUT);
  digitalWrite(VGNSS_CTRL, HIGH);

  Serial1.begin(115200, SERIAL_8N1, 33, 34);
  // Serial1.println("$CFGMSG,0,1,1");
  // Serial1.println("$CFGNAV,1000,1000,100");
  Serial.println("Started");
  delay(1000);
  while (true) {

    if (Serial1.available()) {
      // Disable TXT messages
      Serial1.println("$CFGMSG,6,0,0");
      // Disable GSA message

      Serial1.println("$CFGMSG,0,2,0");

      // Disable GSV message
      Serial1.println("$CFGMSG,0,3,0");

      // Disable VTG message
      Serial1.println("$CFGMSG,0,5,0");

      // Disable ZDA message
      Serial1.println("$CFGMSG,0,6,0");

      // Disable GST message
      Serial1.println("$CFGMSG,0,7,0");

      // Disable GBS message
      Serial1.println("$CFGMSG,0,8,0");
      break;
    } else {
      delay(100);
      continue;
    }
  }


  while (true) {
    if (Serial1.available()) {
      // set 10Hz
      // Serial1.println("$CFGNAV,100,100,1000");
      Serial1.println("$CFGNAV,1000,1000,1000");
      break;
    } else {
      sleep(100);
      continue;
    }
  }
}



void loop() {

  if (!(int(millis() / 1000) % 10)) {
    pinMode(ADC_CTRL, INPUT_PULLUP);
    delay(1);
    int vbat = analogRead(VBAT_READ) * 4.8;
    pinMode(ADC_CTRL, INPUT_PULLDOWN);
    if (vbat < 3000) {
      enter_deepsleep();
    }
  }

  while (!GPS_DATA) {
    delay(10);
    if (Serial1.available() > 0) {

      if (Serial1.peek() != '\n') {
        gps.encode(Serial1.read());
      } else {
        Serial1.read();
        if (!gps.location.isValid()) {
          GPS_DATA = false;
          continue;
        }
        Serial.println(gps.time.second());
        if (gps.time.second() == seconds) {
          GPS_DATA = false;
          continue;
        }
        time_str = (String)gps.time.hour() + ":" + (String)gps.time.minute() + ":" + (String)gps.time.second() + ":" + (String)gps.time.centisecond();
        latitude = gps.location.lat();
        longitude = gps.location.lng();
        sog = gps.speed.kmph();
        seconds = gps.time.second();
        GPS_DATA = true;
      }
    }
  }




  if (lora_idle == true and GPS_DATA == true) {
    delay(random(100));
    sprintf(txpacket, "**;%s;%s;%.6f;%.6f:%.2f **", Identifier, time_str, latitude, longitude, sog);  //start a package

    Serial.printf("\r\nsending packet \"%s\" , length %d\r\n", txpacket, strlen(txpacket));

    Radio.Send((uint8_t *)txpacket, 50);
    GPS_DATA = false;
    lora_idle = false;
  }
  Radio.IrqProcess();
}

void OnTxDone(void) {
  Serial.println("TX done......");
  lora_idle = true;
}

void OnTxTimeout(void) {
  Radio.Sleep();
  Serial.println("TX Timeout......");
  lora_idle = true;
}

void enter_deepsleep(void) {
  Radio.Sleep();
  SPI.end();
  pinMode(RADIO_DIO_1, ANALOG);
  pinMode(RADIO_NSS, ANALOG);
  pinMode(RADIO_RESET, ANALOG);
  pinMode(RADIO_BUSY, ANALOG);
  pinMode(LORA_CLK, ANALOG);
  pinMode(LORA_MISO, ANALOG);
  pinMode(LORA_MOSI, ANALOG);
  digitalWrite(VGNSS_CTRL, LOW);
  Serial1.end();
  // esp_sleep_enable_timer_wakeup(600*1000*(uint64_t)1000);
  esp_deep_sleep_start();
}