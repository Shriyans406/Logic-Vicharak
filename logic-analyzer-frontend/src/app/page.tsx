import LogicCanvas from "@/components/LogicCanvas";

export default function Home() {
  return (
    <main className="flex flex-col items-center justify-center min-h-screen bg-zinc-950 p-4">
      <h1 className="text-2xl font-bold text-white mb-4">
        LOGIC ANALYZER CORE
      </h1>
      <div className="w-full max-w-4xl bg-black border border-zinc-800 p-2">
        <LogicCanvas />
      </div>
    </main>
  );
}
