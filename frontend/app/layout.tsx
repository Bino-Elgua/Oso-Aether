import type { Metadata } from 'next'
import '@/styles/globals.css'

export const metadata: Metadata = {
  title: '\u1ECC\u0300\u1E63\u1ECC\u0301 — Own My Own',
  description: 'Birth, raise, and evolve your own persistent AI pet on Sui.',
}

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body className="min-h-screen bg-deep text-[var(--text-primary)] antialiased">
        <header className="border-b border-iron/20 px-6 py-4">
          <nav className="mx-auto flex max-w-5xl items-center justify-between">
            <a href="/" className="text-lg font-medium tracking-wide text-soul">
              \u1ECC\u0300\u1E63\u1ECC\u0301
            </a>
            <div className="flex gap-6 text-sm text-muted">
              <a href="/pets" className="hover:text-ember transition-colors">My Pets</a>
              <a href="/birth" className="hover:text-ember transition-colors">Birth</a>
            </div>
          </nav>
        </header>
        <main className="mx-auto max-w-5xl">{children}</main>
      </body>
    </html>
  )
}
