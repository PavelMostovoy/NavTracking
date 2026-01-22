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
  GNSS.write(UBX_RATE_10HZ, sizeof(UBX_RATE_10HZ));
  // GNSS.write(UBX_RATE_25HZ, sizeof(UBX_RATE_25HZ));
// === фильтрация NMEA (раскомментировать нужное) ===
  GNSS.write(UBX_DISABLE_GSV, sizeof(UBX_DISABLE_GSV));
  GNSS.write(UBX_DISABLE_GSA, sizeof(UBX_DISABLE_GSA));
//   GNSS.write(UBX_DISABLE_VTG, sizeof(UBX_DISABLE_VTG));
  GNSS.write(UBX_DISABLE_TXT, sizeof(UBX_DISABLE_TXT));


  M5.Rtc.begin();

  if (!SD.begin()) {
    Serial.println("SD init failed - logging disabled");
  } else {
    createLogFile();
    logFile = SD.open(fileName, FILE_APPEND);
    if (!logFile) {
      Serial.println("File open error");
    } else {
      Serial.print("Logging NMEA to: ");
      Serial.println(fileName);
    }
  }
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

bool getField(const char* line, int fieldIndex, char* buffer, int maxLen) {
  int currentField = 0;
  int j = 0;
  const char* p = line;

  while (*p && *p != '*') {
    if (*p == ',') {
      currentField++;
      p++;
      if (currentField > fieldIndex) break;
      continue;
    }
    if (currentField == fieldIndex) {
      if (j < maxLen - 1) buffer[j++] = *p;
    }
    p++;
  }
  buffer[j] = '\0';
  return (currentField >= fieldIndex);
}

uint8_t hexCharToByte(char c) {
  if (c >= '0' && c <= '9') return c - '0';
  if (c >= 'A' && c <= 'F') return c - 'A' + 10;
  if (c >= 'a' && c <= 'f') return c - 'a' + 10;
  return 0;
}

bool verifyNMEAChecksum(const char* line) {
  const char* star = strchr(line, '*');
  if (!star || star[1] == 0 || star[2] == 0) return false;
  uint8_t cs = 0;
  for (const char* p = line + 1; p < star; ++p) cs ^= *p;
  uint8_t received = (hexCharToByte(star[1]) << 4) | hexCharToByte(star[2]);
  return cs == received;
}

bool isNMEAMessageValid(const char* line) {
  if (!verifyNMEAChecksum(line)) return false;

  if (strstr(line, "GLL")) {
    char status[2], mode[2];
    if (getField(line, 6, status, 2) && getField(line, 7, mode, 2)) {
      if (status[0] != 'A') return false;
      if (mode[0] == 'N' || mode[0] == '\0') return false;
    } else return false;
  } 
  else if (strstr(line, "RMC")) {
    char status[2];
    if (getField(line, 2, status, 2)) {
      if (status[0] != 'A') return false;
    } else return false;
  } 
  else if (strstr(line, "GGA")) {
    char quality[2];
    if (getField(line, 6, quality, 2)) {
      if (quality[0] == '0' || quality[0] == '\0') return false;
    } else return false;
  }
  else if (strstr(line, "GSA")) {
    char fixType[2];
    if (getField(line, 2, fixType, 2)) {
      if (fixType[0] == '1' || fixType[0] == '\0') return false;
    } else return false;
  }
  else if (strstr(line, "VTG")) {
    char mode[2];
    if (getField(line, 9, mode, 2)) {
      if (mode[0] == 'N' || mode[0] == '\0') return false;
    } else return false;
  }

  return true;
}

void updateLogFileName() {
  char oldName[32];
  strncpy(oldName, fileName, sizeof(oldName));
  createLogFile();
  if (strcmp(oldName, fileName) != 0) {
    if (logFile) {
      logFile.close();
    }
    logFile = SD.open(fileName, FILE_APPEND);
    if (logFile) {
      Serial.print("Switched to new log file: ");
      Serial.println(fileName);
    }
  }
}

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
  if (!isNMEAMessageValid(line)) return;

  if (!rtcSynced && strstr(line, "RMC")) {
    trySyncRTCfromRMC(line);
    if (rtcSynced) {
      updateLogFileName();
    }
  }

  logLine(line);
}

/* ================================================= */

void logLine(const char* line) {
  if (!logFile) return;

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

  if (!logFile) {
    logFile = SD.open(fileName, FILE_APPEND);
  }

  if (logFile) {
    logFile.write((uint8_t*)writeBuffer, bufferIndex);
    logFile.flush();
  }
  bufferIndex = 0;
}

/* ================================================= */

void trySyncRTCfromRMC(const char* rmc) {
  // $GNRMC,hhmmss.sss,A,.....,ddmmyy,...
  char timeStr[16], dateStr[16];
  if (!getField(rmc, 1, timeStr, sizeof(timeStr))) return;
  if (!getField(rmc, 9, dateStr, sizeof(dateStr))) return;

  if (strlen(timeStr) < 6 || strlen(dateStr) < 6) return;

  int hh = (timeStr[0] - '0') * 10 + (timeStr[1] - '0');
  int mm = (timeStr[2] - '0') * 10 + (timeStr[3] - '0');
  int ss = (timeStr[4] - '0') * 10 + (timeStr[5] - '0');

  int dd = (dateStr[0] - '0') * 10 + (dateStr[1] - '0');
  int MM = (dateStr[2] - '0') * 10 + (dateStr[3] - '0');
  int yy = (dateStr[4] - '0') * 10 + (dateStr[5] - '0') + 2000;

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
