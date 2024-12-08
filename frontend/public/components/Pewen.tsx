import React, { useState, useEffect } from 'react'
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card"
import { useToast } from "@/components/ui/use-toast"
import { Smile, Frown, Meh, TreesIcon as Tree, Coins, Wallet } from 'lucide-react'

const treeNames = ["Piney", "Sprucey", "Cedric", "Oakey", "Woody"]
const treeStates = ["🌱", "🌿", "🌳", "🎄", "🎋"]

// Mock function for wallet connection
const connectWallet = async () => {
  // In a real implementation, this would connect to the Cosmos wallet
  return { address: "cosmos1..." + Math.random().toString(36).substring(2, 6) }
}

export const PewenTamagotchi = () => {
  const [treeHealth, setTreeHealth] = useState(100)
  const [isLoaned, setIsLoaned] = useState(false)
  const [tokens, setTokens] = useState(0)
  const [treeName, setTreeName] = useState("")
  const [walletConnected, setWalletConnected] = useState(false)
  const [walletAddress, setWalletAddress] = useState("")
  const { toast } = useToast()

  useEffect(() => {
    let timer: NodeJS.Timeout
    if (isLoaned) {
      timer = setInterval(() => {
        setTreeHealth(health => {
          const newHealth = Math.max(0, health - 1)
          if (newHealth === 0) {
            handleReturn()
            toast({
              title: "Tree Tantrum!",
              description: `${treeName} ran away to join a forest boy band.`,
            })
          }
          return newHealth
        })
      }, 1000)
    }
    return () => clearInterval(timer)
  }, [isLoaned, treeName])

  const handleConnect = async () => {
    try {
      const { address } = await connectWallet()
      setWalletConnected(true)
      setWalletAddress(address)
      toast({
        title: "Wallet Connected",
        description: `Your wallet is now tree-ted as part of the family!`,
      })
    } catch (error) {
      toast({
        title: "Connection Failed",
        description: "Wallet got stage fright. Try again!",
        variant: "destructive",
      })
    }
  }

  const handleLoan = () => {
    if (!walletConnected) {
      toast({
        title: "No Wallet, No Tree",
        description: "Connect your wallet first. Trees don't grow on thin air!",
        variant: "destructive",
      })
      return
    }
    setIsLoaned(true)
    const name = treeNames[Math.floor(Math.random() * treeNames.length)]
    setTreeName(name)
    toast({
      title: "New Tree Friend!",
      description: `You've adopted ${name}. No refunds!`,
    })
  }

  const handleReturn = () => {
    setIsLoaned(false)
    const earnedTokens = Math.floor(treeHealth / 10)
    setTokens(t => t + earnedTokens)
    toast({
      title: "Tree Returned",
      description: `${treeName} left you ${earnedTokens} tokens. It's not you, it's tree.`,
    })
    setTreeName("")
    setTreeHealth(100)
  }

  const getTreeState = () => {
    const index = Math.floor((treeHealth / 100) * (treeStates.length - 1))
    return treeStates[index]
  }

  const getTreeMood = () => {
    if (treeHealth > 66) return <Smile className="w-8 h-8 text-green-500" />
    if (treeHealth > 33) return <Meh className="w-8 h-8 text-yellow-500" />
    return <Frown className="w-8 h-8 text-red-500" />
  }

  return (
    <Card className="w-full max-w-sm mx-auto bg-white">
      <CardHeader>
        <CardTitle className="text-center text-2xl font-bold">Pewen Pal</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col items-center space-y-4">
        <div className="text-6xl">{isLoaned ? getTreeState() : "🪴"}</div>
        {isLoaned && (
          <>
            <div className="text-xl font-bold">{treeName}</div>
            <div className="flex items-center space-x-2">
              <Tree className="w-6 h-6" />
              <div className="text-lg">{treeHealth}%</div>
              {getTreeMood()}
            </div>
          </>
        )}
        <div className="flex items-center space-x-2">
          <Coins className="w-6 h-6 text-yellow-500" />
          <div className="text-lg font-bold">{tokens} Tokens</div>
        </div>
        {walletConnected ? (
          <div className="flex items-center space-x-2 bg-green-100 p-2 rounded-full">
            <Wallet className="w-4 h-4 text-green-500" />
            <div className="text-sm text-green-700">{walletAddress}</div>
          </div>
        ) : (
          <Button onClick={handleConnect} variant="outline" className="w-full">
            <Wallet className="w-4 h-4 mr-2" /> Connect Wallet
          </Button>
        )}
      </CardContent>
      <CardFooter className="flex justify-center">
        {isLoaned ? (
          <Button onClick={handleReturn} variant="outline">
            Return Tree (It's not you, it's tree)
          </Button>
        ) : (
          <Button onClick={handleLoan} disabled={!walletConnected}>
            Adopt a Tree (No takebacks!)
          </Button>
        )}
      </CardFooter>
    </Card>
  )
}

