import { NavigationContainer } from '@react-navigation/native'
import { createNativeStackNavigator } from '@react-navigation/native-stack'
import LinkDemo from './screens/LinkDemo'
import Home from './screens/Home'
import Success, { SuccessProps } from './screens/Success'

export type RootStackParamsList = {
  Home: undefined
  LinkDemo: undefined
  Success: SuccessProps
}

const Stack = createNativeStackNavigator<RootStackParamsList>()

export function App() {
  return (
    <NavigationContainer>
      <Stack.Navigator>
        <Stack.Screen name='Home' component={Home} />
        <Stack.Screen name='LinkDemo' component={LinkDemo} />
        <Stack.Screen name='Success' component={Success} />
      </Stack.Navigator>
    </NavigationContainer>
  )
}

export default App;