"use client";
import React, { useEffect, useRef, useState } from "react";

export default function LogicCanvas() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [dataBuffer, setDataBuffer] = useState<number[][]>([]);

  useEffect(() => {
    const socket = new WebSocket("ws://127.0.0.1:4000/ws");
    socket.binaryType = "arraybuffer";

    socket.onmessage = (event) => {
      const bytes = new Uint8Array(event.data);
      // Logic: Process every byte in the received chunk
      const newSamples = Array.from(bytes).map((byte) => {
        const bits: number[] = [];
        for (let i = 0; i < 8; i++) {
          bits.push((byte >> i) & 1);
        }
        return bits;
      });

      setDataBuffer((prev) => [...prev.slice(-750), ...newSamples]);
    };

    socket.onerror = (err) => console.error("WebSocket Error:", err);
    return () => socket.close();
  }, []);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const render = () => {
      ctx.fillStyle = "black";
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      const rowHeight = 30;
      const rowGap = 20;

      for (let ch = 0; ch < 8; ch++) {
        ctx.beginPath();
        ctx.strokeStyle = "#4ade80"; // Neon Green
        ctx.lineWidth = 2;

        dataBuffer.forEach((sample, x) => {
          const yBase = ch * (rowHeight + rowGap) + 50;
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
    <div className="border-2 border-zinc-800 rounded bg-black p-4">
      <canvas ref={canvasRef} width={800} height={450} className="w-full" />
    </div>
  );
}
