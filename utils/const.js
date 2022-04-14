import { clusterApiUrl, PublicKey } from '@solana/web3.js'
import facebook_sol from './programs.json'

export const CLUSTER = 'devnet'
export const SOLANA_HOST = 'https://api.devnet.solana.com'
export const STABLE_POOL_PROGRAM_ID = new PublicKey(
  'ARmGAxXNSbpKb4DpHrZpjPinPf47sKZu2YnuEG9S4gf4'
)

export const STABLE_POOL_IDL = facebook_sol
