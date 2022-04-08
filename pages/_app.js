import dynamic from 'next/dynamic'
import '../styles/globals.css'
import { WalletBalanceProvider } from '../context/useWalletBalance'

function MyApp({ Component, pageProps }) {

  const WalletConnectionProvider = dynamic(
    () => import('../context/WalletConnectionProvider'),
    {
      ssr:false,
    },
  )

  return (
    <WalletConnectionProvider>
      <WalletBalanceProvider>
        <Component {...pageProps} />
      </WalletBalanceProvider>
    </WalletConnectionProvider>

  )
}

export default MyApp
