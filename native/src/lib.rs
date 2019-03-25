#[macro_use]

extern crate neon;
extern crate sha2;
extern crate ed25519_dalek;
extern crate bincode;
extern crate curve25519_dalek;
extern crate serde;

use neon::prelude::*;
use ed25519_dalek::SecretKey;
use ed25519_dalek::PublicKey;
use ed25519_dalek::Keypair;
use ed25519_dalek::Signature;
use sha2::Sha512;

use ed25519_dalek::PUBLIC_KEY_LENGTH;
use ed25519_dalek::SIGNATURE_LENGTH;

use curve25519_dalek::edwards::EdwardsPoint;
use curve25519_dalek::scalar::Scalar;


pub fn buf_copy_from_slice(cx: FunctionContext, source: &[u8], buf: &mut Handle<JsBuffer>) {
    cx.borrow_mut(buf, |data| {
        data.as_mut_slice().copy_from_slice(source);
    });
}

/**
 *  @param secretKey
 *  @return publicKey
 */

pub fn generate_public_key(mut cx: FunctionContext) -> JsResult<JsBuffer> {

    let sk_buf: Handle<JsBuffer> = cx.argument(0)?;
    let sk_bytes = cx.borrow(&sk_buf, |data| data.as_slice());

    let sk: SecretKey = SecretKey::from_bytes(&sk_bytes).unwrap();
    let pk: PublicKey = PublicKey::from_secret::<Sha512>(&sk);

    let result: [u8; PUBLIC_KEY_LENGTH] = pk.to_bytes();
    let mut result_buf = JsBuffer::new(&mut cx, PUBLIC_KEY_LENGTH as u32)?;
    buf_copy_from_slice(cx, result.as_ref(), &mut result_buf);

    Ok(result_buf)
}

/**
 *  @param keypair
 *  @param message
 *  @return signature
 */
pub fn sign(mut cx: FunctionContext) -> JsResult<JsBuffer> {

    let mut kp_bytes: [u8; 64] = [0; 64];
    let sk_buf: Handle<JsBuffer> = cx.argument(0)?;
    cx.borrow(&sk_buf, |data| {
        &kp_bytes[0..32].copy_from_slice(data.as_slice());
    });
    let pk_buf: Handle<JsBuffer> = cx.argument(1)?;
    cx.borrow(&pk_buf, |data| {
        &kp_bytes[32..64].copy_from_slice(data.as_slice());
    });
    let kp: Keypair = Keypair::from_bytes(&kp_bytes).unwrap();

    let msg_buf: Handle<JsBuffer> = cx.argument(2)?;
    let msg_bytes = cx.borrow(&msg_buf, |data| data.as_slice());

    let signature: Signature = kp.sign::<Sha512>(&msg_bytes);
    let result: [u8; SIGNATURE_LENGTH] = signature.to_bytes();
    let mut result_buf = JsBuffer::new(&mut cx, SIGNATURE_LENGTH as u32)?;
    buf_copy_from_slice(cx, result.as_ref(), &mut result_buf);

    Ok(result_buf)
}

/**
 *  @param keypair
 *  @param message
 *  @param signature
 *  @return boolean
 */
pub fn verify(mut cx: FunctionContext) -> JsResult<JsBoolean> {

    let kp_buf: Handle<JsBuffer> = cx.argument(0)?;
    let kp_bytes = cx.borrow(&kp_buf, |data| data.as_slice());
//    let kp: Keypair = Keypair::from_bytes(&kp_bytes).unwrap();
    let kp: PublicKey = PublicKey::from_bytes(&kp_bytes).unwrap();

    let msg_buf: Handle<JsBuffer> = cx.argument(1)?;
    let msg_bytes = cx.borrow(&msg_buf, |data| data.as_slice());

    let sig_buf: Handle<JsBuffer> = cx.argument(2)?;
    let sig_bytes = cx.borrow(&sig_buf, |data| data.as_slice());
    let sig: Signature = Signature::from_bytes(&sig_bytes).unwrap();

    let result: bool;
    match kp.verify::<Sha512>(&msg_bytes, &sig) {
        Ok(_)  => {result = true},
        Err(_) => {result = false}
    }

    Ok(cx.boolean(result))
}

/**
 *  @param Buffer scalar 
 *  @param Buffer point
 *  @return Buffer point
 */

pub fn edwards_scalarmult(mut cx: FunctionContext) -> JsResult<JsBuffer> {

    let s_buf: Handle<JsBuffer> = cx.argument(0)?;
    let scalar: Scalar = cx.borrow(&s_buf, |data| {
        let mut s_temp: [u8; 32] = [0; 32];
        let (left, _right) = data.as_mut_slice().split_at_mut(32);
        s_temp.copy_from_slice(left);
        Scalar::from_bytes_mod_order(s_temp)
    });

    let p_buf: Handle<JsBuffer> = cx.argument(1)?;
    let p_bytes = cx.borrow(&p_buf, |data| data.as_slice());
    let point: EdwardsPoint = bincode::deserialize(p_bytes).unwrap();

    let res: EdwardsPoint = scalar * point;
    let res_bytes = bincode::serialize(&res).unwrap();
    let mut result_buf = JsBuffer::new(&mut cx, res_bytes.len() as u32)?;
    buf_copy_from_slice(cx, res_bytes.as_ref(), &mut result_buf);

    Ok(result_buf)
}

/**
 *  @param Buffer point1
 *  @param Buffer point2
 *  @returns Buffer sum
 */

pub fn edwards_addpoints(mut cx: FunctionContext) -> JsResult<JsBuffer> {

    let p1_buf: Handle<JsBuffer> = cx.argument(0)?;
    let p1_bytes = cx.borrow(&p1_buf, |data| data.as_slice());
    let point1: EdwardsPoint = bincode::deserialize(p1_bytes).unwrap();

    let p2_buf: Handle<JsBuffer> = cx.argument(1)?;
    let p2_bytes = cx.borrow(&p2_buf, |data| data.as_slice());
    let point2: EdwardsPoint = bincode::deserialize(p2_bytes).unwrap();

    let res: EdwardsPoint = point1 + point2;
    let res_bytes = bincode::serialize(&res).unwrap();
    let mut result_buf = JsBuffer::new(&mut cx, res_bytes.len() as u32)?;
    buf_copy_from_slice(cx, res_bytes.as_ref(), &mut result_buf);

    Ok(result_buf)
}

pub fn scalar_from_hash(mut cx: FunctionContext) -> JsResult<JsBuffer> {

    let msg_buf: Handle<JsBuffer> = cx.argument(0)?;
    let msg_bytes = cx.borrow(&msg_buf, |data| data.as_slice());

    let res: Scalar = Scalar::hash_from_bytes::<Sha512>(msg_bytes);

    let res_bytes = res.to_bytes();
    let mut result_buf = JsBuffer::new(&mut cx, res_bytes.len() as u32)?;
    buf_copy_from_slice(cx, res_bytes.as_ref(), &mut result_buf);

    Ok(result_buf)
}

pub fn scalar_add(mut cx: FunctionContext) -> JsResult<JsBuffer> {

    let s_buf1: Handle<JsBuffer> = cx.argument(0)?;
    let scalar1: Scalar = cx.borrow(&s_buf1, |data| {
        let mut s_temp: [u8; 32] = [0; 32];
        let (left, _right) = data.as_mut_slice().split_at_mut(32);
        s_temp.copy_from_slice(left);
        Scalar::from_bytes_mod_order(s_temp)
    });

    let s_buf2: Handle<JsBuffer> = cx.argument(1)?;
    let scalar2: Scalar = cx.borrow(&s_buf2, |data| {
        let mut s_temp: [u8; 32] = [0; 32];
        let (left, _right) = data.as_mut_slice().split_at_mut(32);
        s_temp.copy_from_slice(left);
        Scalar::from_bytes_mod_order(s_temp)
    });

    let res = scalar1 + scalar2;
    let res_bytes = res.to_bytes();
    let mut result_buf = JsBuffer::new(&mut cx, res_bytes.len() as u32)?;
    buf_copy_from_slice(cx, res_bytes.as_ref(), &mut result_buf);

    Ok(result_buf)
}

pub fn scalar_mult(mut cx: FunctionContext) -> JsResult<JsBuffer> {

    let s_buf1: Handle<JsBuffer> = cx.argument(0)?;
    let scalar1: Scalar = cx.borrow(&s_buf1, |data| {
        let mut s_temp: [u8; 32] = [0; 32];
        let (left, _right) = data.as_mut_slice().split_at_mut(32);
        s_temp.copy_from_slice(left);
        Scalar::from_bytes_mod_order(s_temp)
    });

    let s_buf2: Handle<JsBuffer> = cx.argument(1)?;
    let scalar2: Scalar = cx.borrow(&s_buf2, |data| {
        let mut s_temp: [u8; 32] = [0; 32];
        let (left, _right) = data.as_mut_slice().split_at_mut(32);
        s_temp.copy_from_slice(left);
        Scalar::from_bytes_mod_order(s_temp)
    });

    let res = scalar1 * scalar2;
    let res_bytes = res.to_bytes();
    let mut result_buf = JsBuffer::new(&mut cx, res_bytes.len() as u32)?;
    buf_copy_from_slice(cx, res_bytes.as_ref(), &mut result_buf);

    Ok(result_buf)
}

register_module!(mut cx, {
    cx.export_function("generatePublicKey", generate_public_key)?;
    cx.export_function("sign", sign)?;
    cx.export_function("verify", verify)?;
    cx.export_function("edwards_scalarmult", edwards_scalarmult)?;
    cx.export_function("edwards_addpoints", edwards_addpoints)?;
    cx.export_function("scalar_from_hash", scalar_from_hash)?;
    cx.export_function("scalar_add", scalar_add)?;
    cx.export_function("scalar_mult", scalar_mult)
});
