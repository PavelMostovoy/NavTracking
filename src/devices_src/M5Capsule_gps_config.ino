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

#define RXpin 15              //this is the pin that the Arduino will use to receive data from the GPS
#define TXpin 13              //this is the pin that the Arduino can use to send data (commands) to the GPS



#include <HardwareSerial.h>
#include <M5Capsule.h>
#include <SD.h>
HardwareSerial GPS(2);

const PROGMEM  uint8_t ClearConfig[] = {0xB5, 0x62, 0x06, 0x09, 0x0D, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x01, 0x19, 0x98};
const PROGMEM  uint8_t GPGLLOff[] = {0xB5, 0x62, 0x06, 0x01, 0x08, 0x00, 0xF0, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x2B};
const PROGMEM  uint8_t GPGSVOff[] = {0xB5, 0x62, 0x06, 0x01, 0x08, 0x00, 0xF0, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x03, 0x39};
const PROGMEM  uint8_t GPVTGOff[] = {0xB5, 0x62, 0x06, 0x01, 0x08, 0x00, 0xF0, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x05, 0x47};
const PROGMEM  uint8_t GPGSAOff[] = {0xB5, 0x62, 0x06, 0x01, 0x08, 0x00, 0xF0, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x32};
const PROGMEM  uint8_t GPGGAOff[] = {0xB5, 0x62, 0x06, 0x01, 0x08, 0x00, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x24};
const PROGMEM  uint8_t GPRMCOff[] = {0xB5, 0x62, 0x06, 0x01, 0x08, 0x00, 0xF0, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x04, 0x40};
const PROGMEM  uint8_t Navrate10hz[] = {0xB5, 0x62, 0x06, 0x08, 0x06, 0x00, 0x64, 0x00, 0x01, 0x00, 0x01, 0x00, 0x7A, 0x12};

uint32_t startmS;

void setup() {
  auto cfg = M5.config();
  M5Capsule.begin(cfg);
  SD.begin(11, SPI, 25000000);
  Serial.begin(115200);// put your setup code here, to run once:
  GPS.begin(9600, SERIAL_8N1, RXpin, TXpin);
  Serial.println();
  Serial.println();
  Serial.println("90_UBlox_GPS_Configuration Starting");
  Serial.println();
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
  
  delay(500);
  Serial.println();
  Serial.println();
  Serial.flush();

  Serial.print("ClearConfig ");
  GPS_SendConfig(ClearConfig, 21);

  Serial.println();
  Serial.println();
  Serial.flush();

  //now turn off most of the GPS sentences and set the refresh rate to 10hz

  Serial.print("GPGLLOff ");
  GPS_SendConfig(GPGLLOff, 16);

  Serial.print("GPGSVOff ");
  GPS_SendConfig(GPGSVOff, 16);

  Serial.print("GPVTGOff ");
  GPS_SendConfig(GPVTGOff, 16);

  Serial.print("GPGSAOff ");
  GPS_SendConfig(GPGSAOff, 16);

  // Serial.print("Navrate10hz ");
  // GPS_SendConfig(Navrate10hz, 14);

  Serial.println();
  Serial.println();
  Serial.flush();

  delay(1000);

    
  if (SD.exists("/hello.txt")) {  
        Serial.println("hello.txt exists.");
        } else { Serial.println("hello.txt doesn't exist.");
    }
    
    Serial.println("Creating hello.txt");
    
    File myFile = SD.open("/hello.txt", FILE_WRITE);  // Create a new file "/hello.txt".
                                        // 创建一个新文件"/hello.txt"
    if (myFile) {  // If the file is open, then write to it.
                   // 如果文件打开,则进行写入操作
        Serial.println("Writing to test.txt...");
        myFile.println("SD test.");
        myFile.close();  // Close the file. 关闭文件
        Serial.println("done.");
    } else {
        Serial.println("error opening test.txt");
    };

}

void loop()
{ 
  delay(100);

    File myFile = SD.open("/gps_log.txt", FILE_APPEND);
    while (GPS.available())
    { myFile.write(GPS.read());

    }
    myFile.close();
    // myFile.println("");
    // myFile.close(); 


 
}


void GPS_SendConfig(const uint8_t *Progmem_ptr, uint8_t arraysize)
{
  uint8_t byteread, index;

  Serial.print(F("GPSSend  "));

  for (index = 0; index < arraysize; index++)
  {
    byteread = pgm_read_byte_near(Progmem_ptr++);
    if (byteread < 0x10)
    {
      Serial.print(F("0"));
    }
    Serial.print(byteread, HEX);
    Serial.print(F(" "));
  }

  Serial.println();
  Progmem_ptr = Progmem_ptr - arraysize;                  //set Progmem_ptr back to start

  for (index = 0; index < arraysize; index++)
  {
    byteread = pgm_read_byte_near(Progmem_ptr++);
    GPS.write(byteread);
  }
  delay(100);

}

