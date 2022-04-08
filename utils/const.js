import { clusterApiUrl, PublicKey } from "@solana/web3.js";
import facebook_sol from './facebook_sol.json'

export const CLUSTER = 'devnet'
export const SOLANA_HOST = 'https://api.devnet.solana.com/'

export const STABLE_POOL_PROGRAM_ID = new PublicKey (
    'EsgFRxZjRnsgDuY7retJHAqmJzpvZVkmojMaS9gQo3Ao'
)

export const STABLE_POOL_IDL = facebook_sol