import LogicCanvas from "@/components/LogicCanvas";

export default function Home() {
  return (
    <main className="min-h-screen bg-black text-white p-10 font-sans">
      <div className="max-w-5xl mx-auto">
        <h1 className="text-3xl font-bold mb-2 tracking-tight">
          WEB LOGIC ANALYZER
        </h1>
        <p className="text-zinc-500 mb-8 font-mono">
          Vicharak Shrike Lite (FPGA + RP2040)
        </p>
        <LogicCanvas />
      </div>
    </main>
  );
}
