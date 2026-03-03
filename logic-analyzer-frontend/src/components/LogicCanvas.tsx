"use client";
import React, { useEffect, useRef, useState } from "react";

export default function LogicCanvas() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [dataBuffer, setDataBuffer] = useState<number[][]>([]);
  const MAX_SAMPLES = 600; // Limit this to keep the HP Laptop smooth [cite: 2026-01-19]

  useEffect(() => {
    const socket = new WebSocket("ws://127.0.0.1:4000/ws");
    socket.binaryType = "arraybuffer";

    socket.onmessage = (event) => {
      const bytes = new Uint8Array(event.data);
      const newSamples = Array.from(bytes).map((byte) => {
        const bits: number[] = [];
        for (let i = 0; i < 8; i++) bits.push((byte >> i) & 1);
        return bits;
      });

      setDataBuffer((prev) => {
        const combined = [...prev, ...newSamples];
        return combined.slice(-MAX_SAMPLES); // Keep only the latest 600 points [cite: 2026-02-14]
      });
    };

    return () => socket.close();
  }, []);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d", { alpha: false }); // Performance boost [cite: 2026-02-14]
    if (!ctx) return;

    let animationFrameId: number;

    const render = () => {
      ctx.fillStyle = "black";
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      const rowHeight = 25;
      const rowGap = 20;
      const xStep = canvas.width / MAX_SAMPLES;

      ctx.lineWidth = 2;
      ctx.strokeStyle = "#4ade80";

      for (let ch = 0; ch < 8; ch++) {
        ctx.beginPath();
        const yBase = ch * (rowHeight + rowGap) + 40;

        dataBuffer.forEach((sample, x) => {
          const xPos = x * xStep;
          const yPos = sample[ch] === 1 ? yBase - rowHeight : yBase;

          if (x === 0) ctx.moveTo(xPos, yPos);
          else ctx.lineTo(xPos, yPos);
        });
        ctx.stroke();
      }
      animationFrameId = requestAnimationFrame(render);
    };

    render();
    return () => cancelAnimationFrame(animationFrameId);
  }, [dataBuffer]);

  return (
    <div className="bg-black border border-zinc-800 p-2 rounded shadow-xl">
      <canvas
        ref={canvasRef}
        width={800}
        height={420}
        className="w-full h-auto"
      />
    </div>
  );
}
