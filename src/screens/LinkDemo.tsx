import { View, Button, StyleSheet, Platform } from 'react-native'
import { useNavigation } from '@react-navigation/native'
import { NativeStackNavigationProp } from '@react-navigation/native-stack'
import { RootStackParamsList } from '../App'
import { useCallback, useEffect, useState } from 'react'
import { LinkExit, LinkIOSPresentationStyle, LinkLogLevel, LinkSuccess, create, dismissLink, open } from 'react-native-plaid-link-sdk'
import  DeviceInfo from 'react-native-device-info'

const ADDRESS = Platform.OS === 'android' ? '10.0.2.2' : 'localhost'
const SERVER_URL = `http://${ADDRESS}:8080`

const LinkDemo = () => {
  const navigation = useNavigation<NativeStackNavigationProp<RootStackParamsList>>()

  // In the actual app, thie would be the deposit address - a unique identifier
  // for the current app instance
  // const [uniqueId, setUniqueId] = useState<string>('')
  const [linkToken, setLinkToken] = useState<string | null>(null)

  // useEffect(() => {
  //   const fetchUniqueId = async () => {
  //     const id = await DeviceInfo.getUniqueId()
  //     console.log(`id = ${id}`)
  //     setUniqueId(id)
  //   }
  //   fetchUniqueId()
  // }, [])

  useEffect(() => {
    if (linkToken === null) {
      createLinkToken()
    } else {
      const tokenConfig = createLinkTokenConfiguration(linkToken)
      create(tokenConfig)
    }
  }, [linkToken])

  const createLinkToken = useCallback(async () => {
    const serverUrl = `${SERVER_URL}/api/create_link_token`
    console.log(`serverUrl = ${serverUrl}`)
    const uniqueId = await DeviceInfo.getUniqueId()
    console.log(`uniqueId = ${uniqueId}`)

    try {
      const res = await fetch(serverUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ 
          client_user_id: uniqueId,
          address: ADDRESS 
        })
      })

    const resJson = await res.json()
    setLinkToken(resJson.link_token)
    } catch (err: any) {
      console.error(err)
    }
  }, [setLinkToken])

  const createLinkTokenConfiguration = (token: string, noLoadingState: boolean = false) => {
    return {
      token,
      noLoadingState
    }
  }

  const createLinkOpenProps = () => {
    return {
      onSuccess: async (success: LinkSuccess) => {
        try {
          const serverUrl = `${SERVER_URL}/api/exchange_public_token`
          const uniqueId = await DeviceInfo.getUniqueId()

          await fetch(serverUrl, {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json'
            },
            body: JSON.stringify({ 
              client_user_id: uniqueId,
               public_token: success.publicToken 
            })
          })
          navigation.navigate('Success', {
            uniqueId,
            serverUrl: `${SERVER_URL}/api/balance`
          })
        } catch (err: any) {
          console.error(err)
        }
      },
      onExit: (linkExit: LinkExit) => {
        console.log('Exit: ', linkExit)
        dismissLink()
      },
      iOSPresentationStyle: LinkIOSPresentationStyle.MODAL,
      logLevel: LinkLogLevel.ERROR
    }
  }

  const handleOpenLink = () => {
    console.log(`Link Token = ${linkToken}`)
    const openProps = createLinkOpenProps()
    open(openProps)
  }


  return (
    <View style={styles.container}>
      <Button title='Go Back' onPress={() => navigation.goBack() } />
      <Button title='Open Link' onPress={handleOpenLink} />
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

export default LinkDemo