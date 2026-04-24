'use client'

interface Message {
  id: string
  role: 'user' | 'pet'
  content: string
  timestamp: number
  metadata?: { growth?: boolean; evolution?: boolean }
}

export default function MessageBubble({ message }: { message: Message }) {
  const isUser = message.role === 'user'

  return (
    <div className={`rounded-lg px-4 py-3 text-sm ${isUser ? 'message-user' : 'message-pet'}`}>
      <div className="mb-1 text-[10px] text-muted">
        {isUser ? 'You' : 'Pet'}
      </div>
      <p className="whitespace-pre-wrap">{message.content}</p>
      {message.metadata?.evolution && (
        <div className="mt-2 text-[10px] text-soul">
          &#10022; Evolution triggered
        </div>
      )}
    </div>
  )
}
