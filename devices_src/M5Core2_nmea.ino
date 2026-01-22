#include <M5Core2.h>
#include <SD.h>
#include <SPI.h>

/* ================== GNSS ================== */
#define GNSS_RX   19
#define GNSS_TX   27
#define GNSS_BAUD 38400

HardwareSerial GNSS(2);

/* ================== SD ================== */
File logFile;

/* ================== Buffer ================== */
#define BUFFER_SIZE 2048
char writeBuffer[BUFFER_SIZE];
uint16_t bufferIndex = 0;

unsigned long lastFlush = 0;
const unsigned long FLUSH_INTERVAL = 1000;

/* ================== NMEA ================== */
char nmeaLine[128];
uint8_t nmeaIndex = 0;

/* ================== File ================== */
char fileName[32];
bool rtcSynced = false;

/* ================== UBX RATE COMMANDS ================== */
// 1 Hz
const uint8_t UBX_RATE_1HZ[] = {
  0xB5,0x62,0x06,0x08,0x06,0x00,
  0xE8,0x03,0x01,0x00,0x01,0x00,
  0x01,0x39
};

// 5 Hz
const uint8_t UBX_RATE_5HZ[] = {
  0xB5,0x62,0x06,0x08,0x06,0x00,
  0xC8,0x00,0x01,0x00,0x01,0x00,
  0xDE,0x6A
};

// 10 Hz
const uint8_t UBX_RATE_10HZ[] = {
  0xB5,0x62,0x06,0x08,0x06,0x00,
  0x64,0x00,0x01,0x00,0x01,0x00,
  0x7A,0x12
};

// 25 Hz
const uint8_t UBX_RATE_25HZ[] = {
  0xB5,0x62,0x06,0x08,0x06,0x00,
  0x28,0x00,0x01,0x00,0x01,0x00,
  0x3E,0xAA
};

/* ================== UBX BAUD RATE COMMANDS (UART1) ================== */
// 9600 baud
// const uint8_t UBX_BAUD_9600[] = {
//   0xB5,0x62,0x06,0x00,0x14,0x00,0x01,0x00,0x00,0x00,0xD0,0x08,0x00,0x00,
//   0x80,0x25,0x00,0x00,0x07,0x00,0x03,0x00,0x00,0x00,0x00,0x00,0xA0,0xA9
// };

// 115200 baud
// const uint8_t UBX_BAUD_115200[] = {
//   0xB5,0x62,0x06,0x00,0x14,0x00,0x01,0x00,0x00,0x00,0xD0,0x08,0x00,0x00,
//   0x00,0xC2,0x01,0x00,0x07,0x00,0x03,0x00,0x00,0x00,0x00,0x00,0xC0,0x7E
// };

/* ================== UBX CONFIG SAVE ================== */
// Save configuration to Flash/BBR
// const uint8_t UBX_CFG_SAVE[] = {
//   0xB5,0x62,0x06,0x09,0x0D,0x00,0x00,0x00,0x00,0x00,
//   0xFF,0xFF,0x00,0x00,0x00,0x00,0x00,0x00,0x07,0x21,0xAF
// };

/* ================================================= */

/* ================== UBX NMEA MESSAGE FILTER ================== */
/* отключение сообщений, не несущих координаты / скорость */

// GGA – Fix data
const uint8_t UBX_DISABLE_GGA[] = {
  0xB5, 0x62, 0x06, 0x01, 0x08, 0x00, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x23
};

// GLL – Geographic position
const uint8_t UBX_DISABLE_GLL[] = {
  0xB5, 0x62, 0x06, 0x01, 0x08, 0x00, 0xF0, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2A
};

// RMC – Recommended minimum data
const uint8_t UBX_DISABLE_RMC[] = {
  0xB5, 0x62, 0x06, 0x01, 0x08, 0x00, 0xF0, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x3F
};

// ZDA – Time and Date
const uint8_t UBX_DISABLE_ZDA[] = {
  0xB5, 0x62, 0x06, 0x01, 0x08, 0x00, 0xF0, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x5B
};

// GSV – satellites in view
const uint8_t UBX_DISABLE_GSV[] = {
  0xB5, 0x62, 0x06, 0x01, 0x08, 0x00, 0xF0, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x38
};


// GSA – DOP / active satellites
const uint8_t UBX_DISABLE_GSA[] = {
  0xB5, 0x62, 0x06, 0x01, 0x08, 0x00, 0xF0, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x31
};

// VTG – ground speed/course
const uint8_t UBX_DISABLE_VTG[] = {
  0xB5, 0x62, 0x06, 0x01, 0x08, 0x00, 0xF0, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x46
};

// TXT – info / warnings
const uint8_t UBX_DISABLE_TXT[] = {
  0xB5, 0x62, 0x06, 0x01, 0x08, 0x00, 0xF0, 0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0xEA
};

/* ================================================= */


void setup() {
  M5.begin();
  delay(1000);
  Serial.begin(115200);



  GNSS.begin(GNSS_BAUD, SERIAL_8N1, GNSS_RX, GNSS_TX);
  delay(300);

  // === при необходимости раскомментировать ===
  // GNSS.write(UBX_RATE_10HZ, sizeof(UBX_RATE_10HZ));
  // GNSS.write(UBX_RATE_25HZ, sizeof(UBX_RATE_25HZ));
// === фильтрация NMEA (раскомментировать нужное) ===
  GNSS.write(UBX_DISABLE_GSV, sizeof(UBX_DISABLE_GSV));
//   GNSS.write(UBX_DISABLE_GSA, sizeof(UBX_DISABLE_GSA));
//   GNSS.write(UBX_DISABLE_VTG, sizeof(UBX_DISABLE_VTG));
//   GNSS.write(UBX_DISABLE_TXT, sizeof(UBX_DISABLE_TXT));


  if (!SD.begin()) {
    Serial.println("SD init failed");
    while (1);
  }

  M5.Rtc.begin();

  createLogFile();

  logFile = SD.open(fileName, FILE_APPEND);
  if (!logFile) {
    Serial.println("File open error");
    while (1);
  }

  Serial.print("Logging NMEA to: ");
  Serial.println(fileName);
}

/* ================================================= */

void loop() {
  while (GNSS.available()) {
    char c = GNSS.read();
    processNMEAChar(c);
  }

  if (millis() - lastFlush >= FLUSH_INTERVAL) {
    flushBuffer();
    lastFlush = millis();
  }
}

/* ================================================= */

void processNMEAChar(char c) {
  if (c == '\n') {
    nmeaLine[nmeaIndex] = 0;
    handleNMEALine(nmeaLine);
    nmeaIndex = 0;
  } else if (nmeaIndex < sizeof(nmeaLine) - 1) {
    nmeaLine[nmeaIndex++] = c;
  }
}

/* ================================================= */

void handleNMEALine(const char* line) {
  logLine(line);

  if (!rtcSynced && strstr(line, "RMC")) {
    trySyncRTCfromRMC(line);
  }
}

/* ================================================= */

void logLine(const char* line) {
  uint16_t len = strlen(line);

  if (bufferIndex + len + 2 >= BUFFER_SIZE) {
    flushBuffer();
  }

  memcpy(&writeBuffer[bufferIndex], line, len);
  bufferIndex += len;

  writeBuffer[bufferIndex++] = '\n';
}

/* ================================================= */

void flushBuffer() {
  if (bufferIndex == 0) return;

  logFile.write((uint8_t*)writeBuffer, bufferIndex);
  logFile.flush();
  bufferIndex = 0;
}

/* ================================================= */

void trySyncRTCfromRMC(const char* rmc) {
  // $GNRMC,hhmmss.sss,A,.....,ddmmyy,...
  char copy[128];
  strncpy(copy, rmc, sizeof(copy));

  char* token;
  uint8_t field = 0;

  int hh, mm, ss, dd, MM, yy;

  token = strtok(copy, ",");

  while (token) {
    field++;

    if (field == 2) { // time
      if (strlen(token) < 6) return;
      hh = (token[0]-'0')*10 + (token[1]-'0');
      mm = (token[2]-'0')*10 + (token[3]-'0');
      ss = (token[4]-'0')*10 + (token[5]-'0');
    }

    if (field == 10) { // date
      if (strlen(token) != 6) return;
      dd = (token[0]-'0')*10 + (token[1]-'0');
      MM = (token[2]-'0')*10 + (token[3]-'0');
      yy = (token[4]-'0')*10 + (token[5]-'0') + 2000;
      break;
    }

    token = strtok(NULL, ",");
  }

  RTC_DateTypeDef date;
  RTC_TimeTypeDef time;

  date.Year  = yy;
  date.Month = MM;
  date.Date  = dd;

  time.Hours   = hh;
  time.Minutes = mm;
  time.Seconds = ss;

  M5.Rtc.SetDate(&date);
  M5.Rtc.SetTime(&time);

  rtcSynced = true;

  Serial.println("RTC synced from RMC");
}

/* ================================================= */

void createLogFile() {
  RTC_DateTypeDef date;
  M5.Rtc.GetDate(&date);

  sprintf(fileName, "/%04d-%02d-%02d.log",
          date.Year,
          date.Month,
          date.Date);
}
