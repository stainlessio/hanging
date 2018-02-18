#include <HMC58X3.h>
#include <MS561101BA.h>
#include <I2Cdev.h>
#include <MPU60X0.h>
#include <EEPROM.h>

//#define DEBUG
#include "DebugUtils.h"
#include "IMUduino.h"
#include <Wire.h>
#include <SPI.h>

typedef enum {
  RAW_VALUES = 'R',
  CAPABILITY_REQUEST = 'C',
  CAPABILITY_RESPONSE = 'P',
  START_STREAMING = 'S',
  STOP_STREAMING = 'X'
} packet_type_enum;
#define RAW_VALUES_LENGTH 22

typedef enum {
  WAITING,
  STREAMING
} state_enum;

typedef enum {
  LED = B00000001,
  ACCEL = B00000010,
  MAG = B00000100,
  GYRO = B00001000,
  TEMP = B00010000,
  BARO = B00100000,
  SERVO = B01000000
} capability_enum;

capability_enum capabilities = ACCEL | MAG | GYRO | TEMP | BARO;
state_enum state = WAITING;

int raw_values[11];
float val[9];
unsigned long last_send_time = 0;
unsigned long current_time = 0;
#define SEND_INTERVAL_MS 1000

int i = 0;
char sendbuffersize;
byte sendbuffer[24];
byte crc = 0;

// Set the FreeIMU object
IMUduino my3IMU = IMUduino();

void setup()
{
  Wire.begin();
  Serial.begin(115200);

  delay(500);
  my3IMU.init(true);
}

void loop()
{
  switch (state)
  {
  case STREAMING:
    current_time = millis();
    if (current_time - last_send_time > SEND_INTERVAL_MS)
    {
      my3IMU.getRawValues(raw_values);
      sendRawValues();
      last_send_time = millis();
    }
    break;
  case WAITING:
    break;
  default:
    Serial.println("!");
  }

  processInput();
  delay(200);
}

void processInput()
{
  while (Serial.available() > 0)
  {
    crc = Serial.read();
    switch (crc)
    {
    case CAPABILITY_REQUEST:
      sendCapabilityResponse();
      break;
    case START_STREAMING:
      Serial.println("S");
      state = STREAMING;
      break;
    case STOP_STREAMING:
      Serial.println("X");
      state = WAITING;
      break;
    }
  }
}

void sendRawValues()
{
  for (i = 0; i < RAW_VALUES_LENGTH / 2; i++)
  {
    sendbuffer[i * 2 + 1] = lowByte(raw_values[i]);
    sendbuffer[i * 2 + 2] = highByte(raw_values[i]);
  }
  writePacket(RAW_VALUES, RAW_VALUES_LENGTH);
}

void sendCapabilityResponse()
{
  sendbuffer[1] = capabilities;
  writePacket(CAPABILITY_RESPONSE, 1);
}

void writePacket(byte packet_type, byte len)
{
  crc = 0;
  sendbuffer[0] = packet_type;
  for (i = 0; i < len; i++)
  {
    crc ^= sendbuffer[i];
  }
  sendbuffer[len + 1] = crc;
  Serial.write(sendbuffer, len + 2);
  Serial.flush();
}
