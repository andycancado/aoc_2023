/**
 * This code was generated by v0 by Vercel.
 * @see https://v0.dev/t/XNXbjg6ofaY
 */
import { Button } from "@/app/components/ui/button"
import Link from "next/link"

export function Home() {
  return (
    <div className="min-h-screen bg-[#0f0f23] font-sans">
      <div className="container mx-auto px-4 sm:px-6 lg:px-8">
        <header className="mt-10 text-center">
          <h1 className="text-[#009900] text-5xl font-bold tracking-wide text-shadow-lg">Andy AoC 2023</h1>
        </header>
        <nav className="flex justify-start space-x-2 mt-10 pr-4 flex-wrap">
          <Link href="#">
            <Button className="text-[#009900]" variant="link">
              [ Day 25 ]
            </Button>
          </Link>
        </nav>
        <div className="mt-10 flex space-x-4">
          <div className="w-1/2 h-full bg-[#0f0f23] p-4 rounded-lg shadow-lg border-[#009900] border overflow-auto">
            <h2 className="text-2xl font-bold text-[#009900] mb-2">Title</h2>
            <p className="text-[#009900]">
              Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas mattis massa vitae tempus suscipit.
              Mauris ultrices lacus in purus aliquam, auctor lacinia odio facilisis. Integer vel nisl euismod, mollis
              tellus a, venenatis metus. Sed sit amet semper erat. Fusce auctor, dui at gravida facilisis, nisl justo
              egestas dolor, id ornare mauris erat id magna. Sed fermentum, erat vitae consequat scelerisque, ante mi
              posuere dui, non iaculis dolor lacus in mauris. In hac habitasse platea dictumst. Cras non erat auctor,
              tincidunt erat sed, condimentum sem. Donec in diam vestibulum, tincidunt ligula vitae, congue metus. Sed
              euismod velit nec semper consequat.
            </p>
          </div>
          <div className="w-1/2 h-full bg-[#0f0f23] p-4 rounded-lg shadow-lg border-[#009900] border overflow-auto">
            <h2 className="text-2xl font-bold text-[#009900] mb-2">Title</h2>
            <p className="text-[#009900]">
              Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas mattis massa vitae tempus suscipit.
              Mauris ultrices lacus in purus aliquam, auctor lacinia odio facilisis. Integer vel nisl euismod, mollis
              tellus a, venenatis metus. Sed sit amet semper erat. Fusce auctor, dui at gravida facilisis, nisl justo
              egestas dolor, id ornare mauris erat id magna. Sed fermentum, erat vitae consequat scelerisque, ante mi
              posuere dui, non iaculis dolor lacus in mauris. In hac habitasse platea dictumst. Cras non erat auctor,
              tincidunt erat sed, condimentum sem. Donec in diam vestibulum, tincidunt ligula vitae, congue metus. Sed
              euismod velit nec semper consequat.
            </p>
          </div>
        </div>
        <div className="mt-10 flex space-x-4 justify-end">
          <div className="w-1/4 h-full bg-[#0f0f23] p-4 rounded-lg shadow-lg border-[#009900] border overflow-auto">
            <h2 className="text-2xl font-bold text-[#009900] mb-2">Part 1</h2>
            <p className="text-[#009900]">12345678</p>
          </div>
          <div className="w-1/4 h-full bg-[#0f0f23] p-4 rounded-lg shadow-lg border-[#009900] border overflow-auto">
            <h2 className="text-2xl font-bold text-[#009900] mb-2">Part 2</h2>
            <p className="text-[#009900]">12345678</p>
          </div>
        </div>
      </div>
    </div>
  )
}
