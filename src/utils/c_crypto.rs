use openssl::aes::AesKey;
use openssl::error::ErrorStack;
use openssl::hash::hash;
use openssl::hash::MessageDigest;
use openssl::symm::{decrypt, encrypt, Cipher, Mode};
//use openssl::rsa::{Rsa, Padding};
//use actix::fut::ok;
use base64::DecodeError;
use openssl::pkey::PKey;
use openssl::rsa::*;
use openssl::sign::*;
use std::collections::BTreeMap;
use std::fmt::Error;
use md5::Digest;
use crate::utils::c_string::bytes_to_str;

/// ****************
/// Base64_Decrypt
/// ****************
pub fn Base64_Decrypt(src: &str) -> Result<Vec<u8>, DecodeError> {
    //    hex::encode(base64::decode(src).unwrap_or_default())
    base64::decode(src)
}

/// ****************
/// Base64_Encrypt
/// ****************
pub fn Base64_Encrypt(src: &str) -> String {
    base64::encode(src)
}

/// ****************
/// Md5
/// ****************
pub fn Md5(src: &[u8]) -> Result<String, ErrorStack> {
    let secret = hash(MessageDigest::md5(), src);
    match secret {
        Ok(res) => Ok(hex::encode(res)),
        Err(e) => Err(e),
    }
}

/// ****************
/// Sha1
/// ****************
pub fn Sha1(src: &[u8]) -> Result<String, ErrorStack> {
    let secret = hash(MessageDigest::sha1(), src);
    match secret {
        Ok(res) => Ok(hex::encode(res)),
        Err(e) => Err(e),
    }
}

/// ****************
/// Sha256
/// ****************
pub fn Sha256(src: &[u8]) -> Result<String, ErrorStack> {
    let secret = hash(MessageDigest::sha256(), src);
    match secret {
        Ok(res) => Ok(hex::encode(res)),
        Err(e) => Err(e),
    }
}

// /// ****************
// /// Sha3_256
// /// ****************
// pub fn Sha3_256(src: &[u8]) -> Result<String, ErrorStack> {
//     let secret = hash(MessageDigest::sha3_256(), src);
//     match secret {
//         Ok(res) => Ok(hex::encode(res)),
//         Err(e) => Err(e),
//     }
// }

/// *************************************************************
/// AES_128_CBC 数据解密(openssl)
/// *************************************************************
pub fn AES_128_CBC_Decrypt(key: &[u8], iv: &[u8], data: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    let cipher = Cipher::aes_128_cbc();
    //    println!("key.len():{}", key.len());
    let ciphertext = decrypt(cipher, key, Some(iv), data);
    ciphertext
}

/// *************************************************************
/// AES_128_CBC 数据加密(openssl)
/// *************************************************************
pub fn AES_128_CBC_Encrypt(key: &[u8], iv: &[u8], data: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    let cipher = Cipher::aes_128_cbc();
    let ciphertext = encrypt(cipher, key, Some(iv), data);

    ciphertext
}

/// *************************************************************
/// RSA 数据加密，返回base64 (openssl)
/// *************************************************************
pub fn rsa_encrypt(pkey: &[u8], data: &[u8]) -> Result<String, ErrorStack> {
    let public_key = Rsa::public_key_from_pem(pkey)?;

    let mut result = vec![0; public_key.size() as usize];
    //    let data = b"a=123&sign_type=RSA2";
    let len = public_key.public_encrypt(data, &mut result, Padding::PKCS1)?;
    let result = base64::encode(&result);
    //    println!(" =========> result：{:#?}", result);  //TODO *********** test  *********************************
    Ok(result)
}

/// *************************************************************
/// RSA 数据解密，返回Vec<u8> (openssl)
/// *************************************************************
pub fn rsa_decrypt(privkey: &[u8], data: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    let privkey = Rsa::private_key_from_pem(privkey)?;
    let data = base64::decode(data);
    match data {
        Err(e) => {
            let err = ErrorStack::get();
            Err(err)
        }
        Ok(data) => {
            let mut result = vec![0; privkey.size() as usize];
            //    let data = b"a=123&sign_type=RSA2";
            let len = privkey.private_decrypt(&data, &mut result, Padding::PKCS1)?;
            //            let result = base64::decode(&result);
            //            println!(" =========> result：{:#?}", result);  //TODO *********** test  *********************************
            Ok(result)
        }
    }
}

/// *************************************************************
/// 签名 SHA256WithRSA ，返回base64 (openssl)
/// Alipay:使用应用私钥生成请求签名
/// *************************************************************
pub fn sign_rsa2(privkey: &[u8], data: &[u8]) -> Result<String, ErrorStack> {
    let keypair = Rsa::private_key_from_pem(privkey)?;
    let keypair = PKey::from_rsa(keypair)?;

    // Sign the data
    let mut signer = Signer::new(MessageDigest::sha256(), &keypair)?;
    signer.set_rsa_padding(Padding::PKCS1)?;
    signer.update(data)?;
    let result = signer.sign_to_vec()?;
    let result = base64::encode(&result);
    //    println!(" =========> result：{:#?}", result);  //TODO *********** test  *********************************
    Ok(result)
}

/// *************************************************************
/// 签名 HmacSHA1 ，返回base64 (openssl)
/// Alipay:
/// *************************************************************
pub fn hmac_SHA1(privkey: &[u8], data: &[u8]) -> Result<String, ErrorStack> {
    // Create a PKey
    let key = PKey::hmac(privkey)?;

    // Compute the HMAC
    let mut signer = Signer::new(MessageDigest::sha1(), &key)?;
    signer.update(data)?;
    let hmac = signer.sign_to_vec()?;
    println!("=========> 签名字符串 hmac：{:?}",hmac);  //TODO *********** test  *********************************
    let result = base64::encode(&hmac);
    Ok(result)
//    Ok(bytes_to_str(&hmac).to_string())
}

/// *************************************************************
/// 签名验证 SHA256WithRSA ，返回base64 (openssl)
/// 使用私钥验证签名
/// *************************************************************
pub fn sign_check_rsa2_private_key(
    privkey: &[u8],
    data: &[u8],
    sign: &[u8],
) -> Result<bool, ErrorStack> {
    let privkey = Rsa::private_key_from_pem(privkey)?;
    let privkey = PKey::from_rsa(privkey)?;

    //    // Sign the data
    //    let mut signer = Signer::new(MessageDigest::sha256(), &privkey)?;
    //    signer.set_rsa_padding(Padding::PKCS1)?;
    //    signer.update(data)?;
    //    let result = signer.sign_to_vec()?;
    //    let result=base64::encode(&result);
    //    println!(" =========> result：{:#?}",result);  //TODO *********** test  *********************************

    let mut verifier = Verifier::new(MessageDigest::sha256(), &privkey)?;
    verifier.set_rsa_padding(Padding::PKCS1)?;
    verifier.update(data)?;
    Ok(verifier.verify(sign)?)
}

/// *************************************************************
/// 签名验证 SHA256WithRSA ，返回base64 (openssl)
/// Alipay: 使用公钥验签
/// *************************************************************
pub fn sign_check_rsa2_public_key(
    pubkey: &[u8],
    data: &[u8],
    sign: &str,
) -> Result<bool, ErrorStack> {
    let pubkey = Rsa::public_key_from_pem(pubkey)?;
    let pubkey = PKey::from_rsa(pubkey)?;
    let sign = Base64_Decrypt(sign);
    match sign {
        Err(e) => {
            let err = ErrorStack::get();
            Err(err)
        }
        Ok(sign) => {
            let mut verifier = Verifier::new(MessageDigest::sha256(), &pubkey)?;
            verifier.set_rsa_padding(Padding::PKCS1)?;
            verifier.update(data)?;
            Ok(verifier.verify(&sign)?)
        }
    }
}

/// *************************************************************
/// 阿里urlencode_pop（特殊URL编码这个是POP特殊的一种规则，即在一般的URLEncode后再增加三种字符替换：
/// 加号 （+）替换成 %20、星号 （*）替换成 %2A、 %7E 替换回波浪号 （~））
/// *************************************************************
pub fn ali_urlencode_pop(params: &BTreeMap<&str, String>) -> String {
    let p = serde_urlencoded::to_string(params);
    debug!("urlencoded p: {:?}", p); //todo

    let str = p.unwrap_or_default();
    let str = str
        .replace("+", "%20")
        .replace("*", "%2A")
        .replace("%7E", "~");
    debug!("urlencoded str: {:?}", str); //todo

    str
}
