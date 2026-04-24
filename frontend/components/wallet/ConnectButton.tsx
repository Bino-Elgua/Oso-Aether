'use client'

import { ConnectButton as SuiConnectButton } from '@mysten/dapp-kit'

export default function ConnectButton() {
  return (
    <SuiConnectButton
      className="!rounded-lg !bg-soul !px-4 !py-2 !text-sm !font-medium !text-white hover:!bg-soul/90 !transition-colors"
    />
  )
}
