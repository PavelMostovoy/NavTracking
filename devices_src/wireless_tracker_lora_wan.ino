

#include "LoRaWan_APP.h"
#include "Arduino.h"
#include "HT_st7735.h"
#include "HT_TinyGPS++.h"

TinyGPSPlus GPS;
HT_st7735 st7735;

#define VGNSS_CTRL Vext

/* OTAA para*/
uint8_t devEui[] = { 0x70, 0xB3, 0xD5, 0x7E, 0xD0, 0x06, 0x53, 0xC8 };
uint8_t appEui[] = { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 };
uint8_t appKey[] = { 0x74, 0xD6, 0x6E, 0x63, 0x45, 0x82, 0x48, 0x27, 0xFE, 0xC5, 0xB7, 0x70, 0xBA, 0x2B, 0x50, 0x45 };

/* ABP para*/
uint8_t nwkSKey[] = { 0x15, 0xb1, 0xd0, 0xef, 0xa4, 0x63, 0xdf, 0xbe, 0x3d, 0x11, 0x18, 0x1e, 0x1e, 0xc7, 0xda, 0x85 };
uint8_t appSKey[] = { 0xd7, 0x2c, 0x78, 0x75, 0x8c, 0xdc, 0xca, 0xbf, 0x55, 0xee, 0x4a, 0x77, 0x8d, 0x16, 0xef, 0x67 };
uint32_t devAddr = (uint32_t)0x007e6ae1;


/*LoraWan channelsmask, default channels 0-7*/
uint16_t userChannelsMask[6] = { 0x00FF, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000 };
// uint32_t license[4] = { 0x21701168, 0xAAAEEC0A, 0xFDA9F9D2, 0x0110E29E };
/*LoraWan region, select in arduino IDE tools*/
LoRaMacRegion_t loraWanRegion = ACTIVE_REGION;

/*LoraWan Class, Class A and Class C are supported*/
DeviceClass_t loraWanClass = CLASS_C;

/*the application data transmission duty cycle.  value in [ms].*/
uint32_t appTxDutyCycle = 5000;

/*OTAA or ABP*/
bool overTheAirActivation = true;

/*ADR enable*/
bool loraWanAdr = true;
/* Indicates if the node is sending confirmed or unconfirmed messages */
bool isTxConfirmed = true;

/* Application port */
uint8_t appPort = 2;

uint8_t confirmedNbTrials = 4;


/* Prepares the payload of the frame */
static void prepareTxFrame(uint8_t port) {

  float lat, lon, alt, course, speed, hdop, sats;

  Serial.println("Waiting for GPS FIX ...");
  st7735.st7735_write_str(0, 0, "GPS Fix vait");

  while (!GPS.location.isUpdated() || !GPS.location.isValid()) {
          if (Serial1.available()) {
        GPS.encode(Serial1.read());
      }
  }

  st7735.st7735_write_str(0, 0, "Ready to Send");

  st7735.st7735_fill_screen(ST7735_BLACK);
  lat = GPS.location.lat();
  lon = GPS.location.lng();
  uint32_t timestamp = GPS.time.value();

  unsigned char *puc;

  appDataSize = 0;

  puc = (unsigned char *)(&timestamp);
  appData[appDataSize++] = puc[0];
  appData[appDataSize++] = puc[1];
  appData[appDataSize++] = puc[2];
  appData[appDataSize++] = puc[3];

  puc = (unsigned char *)(&lat);
  appData[appDataSize++] = puc[0];
  appData[appDataSize++] = puc[1];
  appData[appDataSize++] = puc[2];
  appData[appDataSize++] = puc[3];
  puc = (unsigned char *)(&lon);
  appData[appDataSize++] = puc[0];
  appData[appDataSize++] = puc[1];
  appData[appDataSize++] = puc[2];
  appData[appDataSize++] = puc[3];

  Serial.print(", LAT: ");
  Serial.print(GPS.location.lat(), 6);
  Serial.print(", LON: ");
  Serial.print(GPS.location.lng(), 6);
  Serial.println();
}




void setup() {

  Serial.begin(115200);

  Mcu.begin(HELTEC_BOARD, SLOW_CLK_TPYE);

  Serial1.begin(115200, SERIAL_8N1, 33, 34);

  pinMode(VGNSS_CTRL, OUTPUT);
  digitalWrite(VGNSS_CTRL, HIGH);

  Serial.println("Started");

  delay(1000);

  while (true) {
    if (Serial1.available()) {
      // set 10Hz
      Serial1.println("$CFGNAV,100,100,1000");
      // Serial1.println("$CFGNAV,1000,1000,1000");
      break;
    } else {
      sleep(100);
      continue;
    }
  }

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

  st7735.st7735_init();

  st7735.st7735_fill_screen(ST7735_BLACK);
}
void loop() {
  switch (deviceState) {
    case DEVICE_STATE_INIT:
      {
#if (LORAWAN_DEVEUI_AUTO)
        LoRaWAN.generateDeveuiByChipID();
#endif
        LoRaWAN.init(loraWanClass, loraWanRegion);
        //both set join DR and DR when ADR off
        LoRaWAN.setDefaultDR(3);
        break;
      }
    case DEVICE_STATE_JOIN:
      {
        LoRaWAN.join();
        st7735.st7735_write_str(0, 0, "join>");

        break;
      }
    case DEVICE_STATE_SEND:
      {
        prepareTxFrame(appPort);
        st7735.st7735_write_str(0, 0, "send");

        LoRaWAN.send();
        st7735.st7735_write_str(0, 0, "send  ok");


        deviceState = DEVICE_STATE_CYCLE;
        break;
      }
    case DEVICE_STATE_CYCLE:
      {
        txDutyCycleTime = appTxDutyCycle + randr(-APP_TX_DUTYCYCLE_RND, APP_TX_DUTYCYCLE_RND);
        LoRaWAN.cycle(txDutyCycleTime);
        deviceState = DEVICE_STATE_SLEEP;
        break;
      }
    case DEVICE_STATE_SLEEP:
      {
        st7735.st7735_write_str(0, 0, "Sleep");
        LoRaWAN.sleep(loraWanClass);
        break;
      }
    default:
      {
        deviceState = DEVICE_STATE_INIT;

        break;
      }
  }
}
