import { PublicKey } from '@xoneorg/web3.js';

/** Address of the HPL Token program */
export const TOKEN_PROGRAM_ID = new PublicKey('HuiToken11111111111111111111111111111111111');

/** Address of the HPL Associated Token Account program */
export const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey('HuiATA1111111111111111111111111111111111111');

/** Address of the special mint for wrapped native HUIONE in hpl-token */
export const NATIVE_MINT = new PublicKey('Hc11111111111111111111111111111111111111111');

/** Address of the special mint for wrapped native HUIONE in hpl-token */
export const NATIVE_MINT_METS = new PublicKey('HcMeta1111111111111111111111111111111111111');


export const SYSTEM_PROGRAM_ID = new PublicKey('11111111111111111111111111111111');


// /** Check that the token program provided is not `Tokenkeg...`, useful when using extensions */
// export function programSupportsExtensions(programId: PublicKey): boolean {
//     if (programId === TOKEN_PROGRAM_ID) {
//         return false;
//     } else {
//         return true;
//     }
// }
