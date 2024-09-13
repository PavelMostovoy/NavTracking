/*******************************************************************************************************
  Programs for Arduino - Copyright of the author Stuart Robinson - 28/08/18

  This program is supplied as is, it is up to the user of the program to decide if the program is
  suitable for the intended purpose and free from errors.
*******************************************************************************************************/

/*******************************************************************************************************
  Program Operation - This is a simple program to configure a Ublox GPS. 

  At startup GPS characters are read and sent to the IDE serial monitor for 2 seconds. 

  The GPS configuration is then cleared and a further 2 seconds of characters are sent. 

  Then most GPS sentences are turned off, leaving only GPRMC and GPGGA that are those needed for location
  and speed data etc. The refresh rate is then changed to 10hz.

  The GPS characters are again copied to serial monitor using the new configuration.
  
  GPS baud rate set at 9600 baud, Serial monitor set at 115200 baud. If the data displayed on the serial
  terminal appears to be random text with odd symbols its very likely you have the GPS serial baud rate
  set incorrectly for the GPS.

  Note that not all pins on all Arduinos will work with software serial, see here;

  https://www.arduino.cc/en/Reference/softwareSerial

  Serial monitor baud rate is set at 115200.

*******************************************************************************************************/

#define RXpin 1              //this is the pin that the Arduino will use to receive data from the GPS
#define TXpin 2              //this is the pin that the Arduino can use to send data (commands) to the GPS

#define SD_SPI_SCK_PIN  40
#define SD_SPI_MISO_PIN 39
#define SD_SPI_MOSI_PIN 14
#define SD_SPI_CS_PIN   12


#include <HardwareSerial.h>
#include <M5Cardputer.h>
#include <SD.h>
#include <SPI.h>

HardwareSerial GPS(2);


void setup() {
  auto cfg = M5.config();
  M5Cardputer.begin(cfg, true);

  M5Cardputer.Display.setRotation(1);
  M5Cardputer.Display.setTextSize(3);
  M5Cardputer.Display.drawString("Starting",
                                   M5Cardputer.Display.width() / 8,
                                   M5Cardputer.Display.height() / 2);
  delay(1000);
  SPI.begin(SD_SPI_SCK_PIN, SD_SPI_MISO_PIN, SD_SPI_MOSI_PIN, SD_SPI_CS_PIN);
  SD.begin(SD_SPI_CS_PIN, SPI, 25000000);
  Serial.begin(115200);// put your setup code here, to run once:
  GPS.begin(9600, SERIAL_8N1, RXpin, TXpin);

  Serial.println();

  Serial.println("device started");
  Serial.println();

  if (!SD.begin()) {  // Initialize the SD card. 初始化SD卡
        Serial.println(
            "Card failed, or not present");  // Print a message if the SD card
                                             // initialization fails or if the
                                             // SD card does not exist
                                             // 如果SD卡初始化失败或者SD卡不存在，则打印消息
        while (1)
            ;
    }

  Serial.println("TF card initialized.");

  M5Cardputer.Display.clear();
  M5Cardputer.Display.drawString("Setup Done",
                                   M5Cardputer.Display.width() / 8,
                                   M5Cardputer.Display.height() / 2);
  delay(1000);


}

void loop()
{ 
  M5Cardputer.Display.clear();
  M5Cardputer.Display.drawString("GPS Reading",
                                   M5Cardputer.Display.width() / 8,
                                   M5Cardputer.Display.height() / 2);
  delay(200);
    Serial.println("gps reading");
    File myFile = SD.open("/gps_log.txt", FILE_APPEND);
    while (GPS.available())
    { myFile.write(GPS.read());

    }
    myFile.close();
    // myFile.println("");
    // myFile.close(); 
 
}


