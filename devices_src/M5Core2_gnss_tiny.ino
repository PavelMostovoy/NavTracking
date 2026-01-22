#include <M5Core2.h>
#include <TinyGPSPlus.h>
#include <SD.h>
#include <SPI.h>

/* ================== GNSS ================== */
#define GNSS_RX   19
#define GNSS_TX   27
#define GNSS_BAUD 38400

TinyGPSPlus gps;
HardwareSerial GNSS(2);

/* ================== SD ================== */
File logFile;

/* ================== Buffer ================== */
#define BUFFER_SIZE 1024
char writeBuffer[BUFFER_SIZE];
uint16_t bufferIndex = 0;

unsigned long lastFlush = 0;
const unsigned long FLUSH_INTERVAL = 1000;

/* ================== File ================== */
char fileName[32];

/* ================== UBX RATE COMMANDS ================== */

// 1 Hz (default)
// const uint8_t UBX_RATE_1HZ[] = {
//   0xB5,0x62,0x06,0x08,0x06,0x00,
//   0xE8,0x03,0x01,0x00,0x01,0x00,
//   0x01,0x39
// };

// const uint8_t UBX_RATE_10HZ[] = {
//   0xB5,0x62,0x06,0x08,0x06,0x00,
//   0x64,0x00,0x01,0x00,0x01,0x00,
//   0x7A,0x12
// };

// const uint8_t UBX_RATE_25HZ[] = {
//   0xB5,0x62,0x06,0x08,0x06,0x00,
//   0x28,0x00,0x01,0x00,0x01,0x00,
//   0x3E,0xAA
// };

/* ================================================= */

void setup() {
  M5.begin();
  delay(1000);

  M5.Lcd.setBrightness(0);

  Serial.begin(115200);

  GNSS.begin(GNSS_BAUD, SERIAL_8N1, GNSS_RX, GNSS_TX);
  delay(300);

  // === при необходимости раскомментировать ===
  // GNSS.write(UBX_RATE_1HZ,  sizeof(UBX_RATE_1HZ));
  // GNSS.write(UBX_RATE_10HZ, sizeof(UBX_RATE_10HZ));
  // GNSS.write(UBX_RATE_25HZ, sizeof(UBX_RATE_25HZ));

  if (!SD.begin()) {
    Serial.println("SD init failed");
    while (1);
  }

  M5.Rtc.begin();

  syncRTCwithGPS();   // <<< ВАЖНО
  createLogFile();

  logFile = SD.open(fileName, FILE_APPEND);
  if (!logFile) {
    Serial.println("File open error");
    while (1);
  }

  if (logFile.size() == 0) {
    logFile.println("DATE,TIME,LAT,LON,ALT,HDOP");
  }

  Serial.print("Logging to: ");
  Serial.println(fileName);

}

/* ================================================= */

void loop() {
  while (GNSS.available()) {
    gps.encode(GNSS.read());
  }

  if (gps.location.isUpdated()) {
    logGPS();
  }

  if (millis() - lastFlush >= FLUSH_INTERVAL) {
    flushBuffer();
    lastFlush = millis();
  }
}

/* ================================================= */

void syncRTCwithGPS() {
  unsigned long start = millis();

  while ((!gps.date.isValid() || !gps.time.isValid()) &&
         millis() - start < 5000) {
    while (GNSS.available()) gps.encode(GNSS.read());
  }

  if (gps.date.isValid() && gps.time.isValid()) {
    RTC_DateTypeDef date;
    RTC_TimeTypeDef time;

    date.Year  = gps.date.year();
    date.Month = gps.date.month();
    date.Date  = gps.date.day();

    time.Hours   = gps.time.hour();
    time.Minutes = gps.time.minute();
    time.Seconds = gps.time.second();

    M5.Rtc.SetDate(&date);
    M5.Rtc.SetTime(&time);

    Serial.println("RTC synced from GPS");
  } else {
    Serial.println("GPS time not valid, RTC unchanged");
  }
}

/* ================================================= */

void logGPS() {
  if (!gps.location.isValid()) return;

  char line[96];

  sprintf(line,
    "%04d-%02d-%02d,%02d:%02d:%02d,%.6f,%.6f,%.2f,%.2f\n",
    gps.date.year(),
    gps.date.month(),
    gps.date.day(),
    gps.time.hour(),
    gps.time.minute(),
    gps.time.second(),
    gps.location.lat(),
    gps.location.lng(),
    gps.altitude.meters(),
    gps.hdop.hdop()
  );

  uint16_t len = strlen(line);

  if (bufferIndex + len >= BUFFER_SIZE) {
    flushBuffer();
  }

  memcpy(&writeBuffer[bufferIndex], line, len);
  bufferIndex += len;
}

/* ================================================= */

void flushBuffer() {
  if (bufferIndex == 0) return;

  logFile.write((uint8_t*)writeBuffer, bufferIndex);
  logFile.flush();
  bufferIndex = 0;
}

/* ================================================= */

void createLogFile() {
  RTC_DateTypeDef date;
  M5.Rtc.GetDate(&date);

  sprintf(fileName, "/%04d-%02d-%02d.txt",
          date.Year,
          date.Month,
          date.Date);
}
