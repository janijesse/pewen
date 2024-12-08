import { Pewen } from '@/components/Pewen'

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-4 bg-gradient-to-b from-green-100 to-green-200">
      <h1 className="text-3xl font-bold mb-8 text-green-800 text-center">
        Pewen: The Tree
      </h1>
      <Pewen/>
    </main>
  )
}

