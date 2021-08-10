use std::error::Error;

use concrete::LWE;
#[allow(unused_imports)]
use colored::Colorize;
use crate::params::Params;
use crate::userovo::keys::PrivKeySet;
use crate::ciphertexts::ParmCiphertext;



// =============================================================================
//
//  Encryption
//

/// Parmesan encryption
/// * splits signed integer into nibbles (bits)
/// * encrypt one-by-one
pub fn parm_encrypt(
    params: &Params,
    priv_keys: &PrivKeySet,
    m: i32,
    bits: usize,
) -> Result<ParmCiphertext, Box<dyn Error>> {
    //WISH some warning if bits is more than given type (-1 for signed)
    let mut res: ParmCiphertext = Vec::new();
    let m_abs = m.abs();
    let m_pos = m >= 0;

    for i in 0..bits {
        // calculate i-th bit with sign
        let mi = if ((m_abs >> i) & 1) == 0 {
            0i32
        } else {
            if m_pos {1i32} else {-1i32}
        };
        res.push(parm_encr_nibble(params, priv_keys, mi)?);
    }

    Ok(res)
}

fn parm_encr_nibble(
    params: &Params,
    priv_keys: &PrivKeySet,
    mut mi: i32,
) -> Result<LWE, Box<dyn Error>> {
    // little hack, how to bring mi into positive interval [0,2^pi)
    mi &= params.plaintext_mask();

    Ok(LWE::encrypt_uint(
        &priv_keys.sk,
        mi as u32,
        &priv_keys.encoder,
    )?)
}



// =============================================================================
//
//  Decryption
//

/// Parmesan decryption
/// * composes signed integer from multiple encrypted nibbles (bits)
/// * considers symmetric alphabet around zero
pub fn parm_decrypt(
    params: &Params,
    priv_keys: &PrivKeySet,
    pc: &ParmCiphertext,
) -> Result<i64, Box<dyn Error>> {
    let mut m = 0i64;

    //~ measure_duration!(
        //~ "Decrypt",
        //~ [
            for (i, ct) in pc.iter().enumerate() {
                let mi = parm_decr_nibble(params, priv_keys, ct)?;
                //~ infoln!("m[{}] = {} (pi = {})", i, mi, ct.encoder.nb_bit_precision);
                m += match mi {
                     1 => {  1i64 << i},
                     0 => {  0i64},
                    -1 => {-(1i64 << i)},
                     _ => {  0i64},   //WISH fail
                };
            }
        //~ ]
    //~ );

    Ok(m)
}

fn parm_decr_nibble(
    params: &Params,
    priv_keys: &PrivKeySet,
    ct: &LWE,
) -> Result<i32, Box<dyn Error>> {
    let mi = ct.decrypt_uint(&priv_keys.sk)? as i32;   // rounding included in Encoder
    if mi >= params.plaintext_pos_max() {Ok(mi - params.plaintext_space_size())} else {Ok(mi)}
}
