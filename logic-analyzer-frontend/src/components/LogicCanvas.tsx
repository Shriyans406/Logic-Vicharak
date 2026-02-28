"use client";
import React, { useEffect, useRef, useState } from "react";

export default function LogicCanvas() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [dataBuffer, setDataBuffer] = useState<number[][]>([]);

  useEffect(() => {
    // Connect to the Rust backend [cite: 2026-02-19]
    const socket = new WebSocket("ws://127.0.0.1:4000/ws");
    socket.binaryType = "arraybuffer";

    socket.onmessage = (event) => {
      const bytes = new Uint8Array(event.data);
      const byte = bytes[0];

      const bits: number[] = [];
      for (let i = 0; i < 8; i++) {
        bits.push((byte >> i) & 1);
      }
      // Update the buffer with a max of 800 samples for performance [cite: 2026-02-14]
      setDataBuffer((prev) => [...prev.slice(-800), bits]);
    };

    return () => socket.close();
  }, []);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas || !canvas.getContext) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const render = () => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      ctx.strokeStyle = "#4ade80"; // Bright Green [cite: 2026-02-14]
      ctx.lineWidth = 2;

      const rowHeight = 35;
      const rowGap = 15;

      for (let ch = 0; ch < 8; ch++) {
        ctx.beginPath();
        dataBuffer.forEach((sample, x) => {
          const yBase = ch * (rowHeight + rowGap) + 40;
          const y = sample[ch] === 1 ? yBase - rowHeight : yBase;

          if (x === 0) ctx.moveTo(x, y);
          else ctx.lineTo(x, y);
        });
        ctx.stroke();
      }
      requestAnimationFrame(render);
    };

    render();
  }, [dataBuffer]);

  return (
    <div className="bg-zinc-950 p-6 rounded-lg border border-zinc-800">
      <h2 className="text-green-400 font-mono text-sm mb-4">
        LIVE LOGIC STREAM
      </h2>
      <canvas
        ref={canvasRef}
        width={800}
        height={420}
        className="w-full h-auto bg-black"
      />
    </div>
  );
}
