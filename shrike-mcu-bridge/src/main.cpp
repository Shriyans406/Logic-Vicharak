#include <Arduino.h>
#include <LittleFS.h>
#include <Shrike.h>

ShrikeFlash fpga;

void setup() {
  delay(2000);
  Serial.begin(921600); 
  
  Serial.println("--- SYSTEM START ---");

  if (!LittleFS.begin()) {
    Serial.println("FATAL: LittleFS Mount Failed!");
    return;
  }
  Serial.println("LittleFS Mounted.");

  if (!fpga.begin()) {
    Serial.println("FATAL: Shrike Hardware Init Failed!");
    while(1);
  }
  Serial.println("Shrike Hardware Ready.");
  
  Serial.print("Flashing FPGA...");
  if(fpga.flash("/FPGA_bitstream_MCU.bin")) {
    Serial.println(" SUCCESS.");
  } else {
    Serial.println(" FAILED.");
  }

  for (int i = 0; i < 8; i++) {
    pinMode(i, INPUT_PULLDOWN);
  }
  Serial.println("Starting Stream...");
}

void loop() {
  uint8_t sample = (uint8_t)(gpio_get_all() & 0xFF);
  Serial.write(sample);
  delayMicroseconds(100); 
}