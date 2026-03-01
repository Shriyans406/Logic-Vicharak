#include <Arduino.h>

// On the Shrike Lite, GPIO 0-7 are typically mapped to the FPGA [cite: 2026-02-14]
const uint32_t FPGA_PIN_MASK = 0xFF; // Bits 0 through 7

void setup() {
  // Start Serial at the highest stable speed for the Shrike [cite: 2026-02-14]
  Serial.begin(921600); 

  // Initialize all 8 pins as inputs with pull-downs
  for (int i = 0; i < 8; i++) {
    pinMode(i, INPUT_PULLDOWN);
  }
}

void loop() {
  // 1. Read the entire GPIO register (32 bits) [cite: 2026-02-14]
  uint32_t all_pins = gpio_get_all();

  // 2. Mask it so we only have the first 8 bits (the FPGA bus)
  uint8_t fpga_data = (uint8_t)(all_pins & FPGA_PIN_MASK);

  // 3. Write raw binary byte to USB (No println, no strings!) [cite: 2026-02-14]
  Serial.write(fpga_data);

  // 4. Tight delay to match the sample rate of our Next.js dashboard [cite: 2026-02-14]
  delayMicroseconds(100); 
}