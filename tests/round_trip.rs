use std::mem::MaybeUninit;
use std::ptr;

#[derive(Debug, Clone, Copy)]
struct Header {
    header_ext: bool,
    padding: bool,
    payload_type: u8,
    marker: bool,
    sequence: u16,
    timestamp: u32,
    ssrc: u32,
}

impl Header {
    fn to_bytes(&self, payload_size: usize) -> Vec<u8> {
        let mut b1 = 0b10000000u8;
        b1 |= (self.padding as u8) << 5;
        b1 |= (self.header_ext as u8) << 4;

        let mut b2 = 0u8;
        b2 |= (self.marker as u8) << 7;
        b2 |= self.payload_type & 0x7F;

        let mut bytes = vec![b1, b2];
        bytes.extend_from_slice(&self.sequence.to_be_bytes());
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());

        bytes.extend((0..payload_size).map(|_| 0xAB));

        bytes
    }
}

unsafe fn round_trip(
    noop: bool,
    policy_fn: unsafe extern "C" fn(*mut srtp2_sys::srtp_crypto_policy_t),
) {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        srtp2_sys::srtp_init();
    });

    let mut key: Vec<_> = (0u8..50).collect();

    let mut policy: srtp2_sys::srtp_policy_t = MaybeUninit::zeroed().assume_init();
    policy_fn(&mut policy.rtp);
    policy_fn(&mut policy.rtcp);
    policy.key = key.as_mut_ptr();

    let mut inbound = ptr::null_mut();
    policy.ssrc.type_ = srtp2_sys::srtp_ssrc_type_t_ssrc_any_inbound;
    let err = srtp2_sys::srtp_create(&mut inbound, &policy);
    assert_eq!(err, srtp2_sys::srtp_err_status_t_srtp_err_status_ok);
    let mut outbound = ptr::null_mut();
    policy.ssrc.type_ = srtp2_sys::srtp_ssrc_type_t_ssrc_any_outbound;
    let err = srtp2_sys::srtp_create(&mut outbound, &policy);
    assert_eq!(err, srtp2_sys::srtp_err_status_t_srtp_err_status_ok);

    println!("Starting round trip");

    for sequence in 0x1234..0x1434 {
        let input = Header {
            header_ext: false,
            padding: false,
            payload_type: 96,
            marker: false,
            sequence,
            timestamp: 0xDECAFBAD + (sequence as u32 / 10) * 3000,
            ssrc: 0xDEADBEEF,
        }
        .to_bytes(1000);
        let mut output = input.clone();
        output.reserve(1024);

        let mut length = input.len() as _;
        let err = srtp2_sys::srtp_protect(outbound, output.as_mut_ptr() as _, &mut length);
        assert_eq!(err, srtp2_sys::srtp_err_status_t_srtp_err_status_ok);
        output.set_len(length as _);

        if noop {
            assert_eq!(&input[..], &output[..]);
        } else {
            assert_ne!(&input[..], &output[..]);
        }

        let err = srtp2_sys::srtp_unprotect(inbound, output.as_mut_ptr() as _, &mut length);
        assert_eq!(err, srtp2_sys::srtp_err_status_t_srtp_err_status_ok);
        output.set_len(length as _);

        assert_eq!(&input[..], &output[..]);
    }

    println!("done round trip")
}

#[test]
fn round_trip_crypto_policy_set_aes_cm_128_null_auth() {
    unsafe {
        round_trip(
            false,
            srtp2_sys::srtp_crypto_policy_set_aes_cm_128_null_auth,
        )
    }
}

#[test]
fn round_trip_crypto_policy_set_aes_cm_256_null_auth() {
    unsafe {
        round_trip(
            false,
            srtp2_sys::srtp_crypto_policy_set_aes_cm_256_null_auth,
        )
    }
}

#[test]
fn round_trip_crypto_policy_set_aes_cm_128_hmac_sha1_32() {
    unsafe {
        round_trip(
            false,
            srtp2_sys::srtp_crypto_policy_set_aes_cm_128_hmac_sha1_32,
        )
    }
}

#[test]
fn round_trip_crypto_policy_set_aes_cm_256_hmac_sha1_32() {
    unsafe {
        round_trip(
            false,
            srtp2_sys::srtp_crypto_policy_set_aes_cm_256_hmac_sha1_32,
        )
    }
}

#[test]
fn round_trip_crypto_policy_set_aes_cm_256_hmac_sha1_80() {
    unsafe {
        round_trip(
            false,
            srtp2_sys::srtp_crypto_policy_set_aes_cm_256_hmac_sha1_80,
        )
    }
}

#[test]
fn round_trip_crypto_policy_set_null_cipher_hmac_null() {
    unsafe {
        round_trip(
            true,
            srtp2_sys::srtp_crypto_policy_set_null_cipher_hmac_null,
        )
    }
}

#[test]
fn round_trip_crypto_policy_set_null_cipher_hmac_sha1_80() {
    unsafe {
        round_trip(
            false,
            srtp2_sys::srtp_crypto_policy_set_null_cipher_hmac_sha1_80,
        )
    }
}

#[test]
fn round_trip_crypto_policy_set_rtcp_default() {
    unsafe { round_trip(false, srtp2_sys::srtp_crypto_policy_set_rtcp_default) }
}

#[test]
fn round_trip_crypto_policy_set_rtp_default() {
    unsafe { round_trip(false, srtp2_sys::srtp_crypto_policy_set_rtp_default) }
}

#[cfg(feature = "enable-openssl")]
mod enable_openssl {
    use super::round_trip;

    #[test]
    fn round_trip_crypto_policy_set_aes_cm_192_null_auth() {
        unsafe {
            round_trip(
                false,
                srtp2_sys::srtp_crypto_policy_set_aes_cm_192_null_auth,
            )
        }
    }

    #[test]
    fn round_trip_crypto_policy_set_aes_cm_192_hmac_sha1_32() {
        unsafe {
            round_trip(
                false,
                srtp2_sys::srtp_crypto_policy_set_aes_cm_192_hmac_sha1_32,
            )
        }
    }

    #[test]
    fn round_trip_crypto_policy_set_aes_cm_192_hmac_sha1_80() {
        unsafe {
            round_trip(
                false,
                srtp2_sys::srtp_crypto_policy_set_aes_cm_192_hmac_sha1_80,
            )
        }
    }

    #[test]
    fn round_trip_crypto_policy_set_aes_gcm_128_8_auth() {
        unsafe { round_trip(false, srtp2_sys::srtp_crypto_policy_set_aes_gcm_128_8_auth) }
    }

    #[test]
    fn round_trip_crypto_policy_set_aes_gcm_128_8_only_auth() {
        unsafe {
            round_trip(
                false,
                srtp2_sys::srtp_crypto_policy_set_aes_gcm_128_8_only_auth,
            )
        }
    }

    #[test]
    fn round_trip_crypto_policy_set_aes_gcm_128_16_auth() {
        unsafe { round_trip(false, srtp2_sys::srtp_crypto_policy_set_aes_gcm_128_16_auth) }
    }

    #[test]
    fn round_trip_crypto_policy_set_aes_gcm_256_8_auth() {
        unsafe { round_trip(false, srtp2_sys::srtp_crypto_policy_set_aes_gcm_256_8_auth) }
    }

    #[test]
    fn round_trip_crypto_policy_set_aes_gcm_256_8_only_auth() {
        unsafe {
            round_trip(
                false,
                srtp2_sys::srtp_crypto_policy_set_aes_gcm_256_8_only_auth,
            )
        }
    }

    #[test]
    fn round_trip_crypto_policy_set_aes_gcm_256_16_auth() {
        unsafe { round_trip(false, srtp2_sys::srtp_crypto_policy_set_aes_gcm_256_16_auth) }
    }
}
