#include <Arduino.h>
#include <LittleFS.h>
#include <Shrike.h>

ShrikeFlash fpga;

void setup() {
  // Use 115200 since your laptop likes it better [cite: 2026-02-14]
  Serial.begin(115200); 
  delay(3000); // Give you time to open the monitor!
  
  Serial.println("\n\n=== SHRIKE DEBUG START ===");

  if (!LittleFS.begin()) {
    Serial.println("ERROR: LittleFS not mounted. Did you 'Upload Filesystem Image'?");
    return;
  }

  // Check if file exists [cite: 2026-02-14]
  if (LittleFS.exists("/fpga.bin")) {
    Serial.println("SUCCESS: fpga.bin found in memory.");
  } else {
    Serial.println("ERROR: fpga.bin NOT FOUND. Check your 'data' folder.");
  }

  if (!fpga.begin()) {
    Serial.println("ERROR: FPGA hardware handshake failed.");
    while(1);
  }

  Serial.println("Attempting to Flash FPGA...");
  if (fpga.flash("/fpga.bin")) {
    Serial.println("=== FPGA LOGIC LOADED SUCCESSFULLY ===");
  } else {
    Serial.println("=== FPGA FLASH FAILED ===");
  }

  for (int i = 0; i < 8; i++) pinMode(i, INPUT_PULLDOWN);
}

void loop() {
  uint8_t sample = (uint8_t)(gpio_get_all() & 0xFF);
  
  // Only send if data isn't zero, or send a '.' every 1000 samples to show life
  static int heartbeat = 0;
  if (sample > 0) {
    Serial.write(sample);
  } else if (heartbeat++ > 1000) {
    // This tells us the loop is running even if pins are 0 [cite: 2026-02-14]
    Serial.print("_"); 
    heartbeat = 0;
  }
  delayMicroseconds(100);
}