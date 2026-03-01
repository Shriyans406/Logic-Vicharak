module main (
    input  osc_clk,      // This is the internal oscillator
    output [7:0] mcu_out // These are the 8 pins going to the RP2040
);

    // 24-bit counter to create different speeds for our 8 channels
    reg [23:0] count = 0;

    always @(posedge osc_clk) begin
        count <= count + 1'b1;
    end

    // Map 8 different bits of the counter to the 8 output pins
    // This gives us 8 square waves at different frequencies
    assign mcu_out[0] = count[0];  // Fastest toggle
    assign mcu_out[1] = count[1];
    assign mcu_out[2] = count[2];
    assign mcu_out[3] = count[3];
    assign mcu_out[4] = count[16]; // Slow toggle
    assign mcu_out[5] = count[17];
    assign mcu_out[6] = count[18];
    assign mcu_out[7] = count[22]; // Very slow "heartbeat"

endmodule