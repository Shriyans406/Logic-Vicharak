#include <Arduino.h>
#include <LittleFS.h>
#include <Shrike.h>

ShrikeFlash fpga;
const uint32_t LOGIC_MASK = 0xFF; // Pins 0-7 from FPGA [cite: 2026-02-14]

void setup() {
  delay(2000);
  Serial.begin(921600); // High speed for logic data [cite: 2026-02-14]
  
  // 1. Mount the filesystem to find the .bin file [cite: 2026-02-14]
  if (!LittleFS.begin()) {
    Serial.println("LittleFS Mount Failed!");
    return;
  }

  // 2. Initialize the Shrike Hardware [cite: 2026-02-14]
  if (!fpga.begin()) {
    while (1) { Serial.println("Hardware Init Failed!"); delay(1000); }
  }
  
  // 3. Flash the Logic Analyzer Verilog into the FPGA [cite: 2026-02-14]
  fpga.flash("/FPGA_bitstream_MCU.bin");

  // 4. Setup GPIOs 0-7 as inputs to read the FPGA bus [cite: 2026-02-14]
  for (int i = 0; i < 8; i++) {
    pinMode(i, INPUT_PULLDOWN);
  }
}

void loop() {
  // Grab all 8 logic channels in one clock cycle [cite: 2026-02-14]
  uint8_t sample = (uint8_t)(gpio_get_all() & LOGIC_MASK);
  
  // Send the raw byte to Rust [cite: 2026-02-19]
  Serial.write(sample);
  
  // Keep the stream fast but stable
  delayMicroseconds(100); 
}