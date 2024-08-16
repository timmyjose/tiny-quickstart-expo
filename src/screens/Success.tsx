import { useCallback, useEffect, useState } from 'react'
import { View, Text, StyleSheet} from 'react-native'

export type SuccessProps = {
  serverUrl: string
}

const Success = ({ route }: SuccessProps) => {
  const [balance, setBalance] = useState<string | null>(null)

  const getBalance = useCallback(async () => {
    try {
      const { serverUrl } = route.params
      const res = await fetch(serverUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        }
      })

      const resJson = await res.json()
      setBalance(resJson)
    } catch (err: any) {
      console.error(err)
    }
  }, [setBalance])

  useEffect(() => {
    if (balance === null) {
      getBalance()
    }
  }, [balance])

  return (
    <View style={styles.container}>
      <Text>{JSON.stringify(balance)}</Text>
    </View>
  )
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center'
  }
})

export default Success