import { PublicKey, clusterApiUrl } from '@put/web3.js';

export const SYSTEM_PROGRAM_ID = new PublicKey(
  '11111111111111111111111111111111',
);

export const RENT_PROGRAM_ID = new PublicKey(
    'SysvarRent111111111111111111111111111111111',
);

export const PROGRAM_ID = new PublicKey(
    'An2DRyUtGBKYioLhHJEQ3nPcGgzzRJQ8vgdhyjdtC14H',
);

type Cluster = {
  name: string;
  url: string;
};
export const CLUSTERS: Cluster[] = [
  {
    name: 'mainnet-beta',
    url: 'https://pubchain-rpc.xone.la/',
  },
  {
    name: 'testnet',
    url: clusterApiUrl('testnet'),
  },
  {
    name: 'devnet',
    url: 'http://localhost:8899',
  },
];
export const DEFAULT_CLUSTER = CLUSTERS[2];
