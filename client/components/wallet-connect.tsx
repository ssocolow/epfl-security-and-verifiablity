"use client"

import { Button } from "@/components/ui/button"
import { Wallet } from "lucide-react"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu"
import { useWallet } from "@/context/wallet-context"

export default function WalletConnect() {
  const { connected, address, chainId, connectWallet, disconnectWallet } = useWallet()

  const formatAddress = (address: string) => {
    if (!address) return ""
    return `${address.substring(0, 6)}...${address.substring(address.length - 4)}`
  }

  return (
    <>
      {!connected ? (
        <Button onClick={connectWallet} className="flex items-center gap-2">
          <Wallet className="h-4 w-4" />
          Connect Wallet
        </Button>
      ) : (
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="outline" className="flex items-center gap-2">
              <Wallet className="h-4 w-4" />
              {formatAddress(address)}
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuItem className="text-xs text-muted-foreground">{address}</DropdownMenuItem>
            <DropdownMenuItem className="text-xs text-muted-foreground">Chain ID: {chainId}</DropdownMenuItem>
            <DropdownMenuItem onClick={disconnectWallet}>Disconnect</DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      )}
    </>
  )
}

