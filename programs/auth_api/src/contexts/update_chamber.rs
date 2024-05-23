use crate::*;

use aes_gcm::Aes256Gcm as aesaes;
use aes_gcm::Key as aeskey;
use aes_gcm::Nonce as aesnonce;
use aes_gcm::aead::Aead as aesaed;
use aes_gcm::aead::NewAead as aesnewaed;
use aes_gcm::aead::AeadInPlace as aesinplace;


#[derive(Accounts)]
pub struct UpdateChamber<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// TODO: make sure client does not send a false identity PDA
    // possibly use issuer_id's -> find a way to use u32 in pda seeds
    #[account(has_one = authority)]
    pub identity: Account<'info, IdentityInfo>,
    #[account(mut, seeds = [b"chamber", identity.key().as_ref()], bump = chamber.bump)]
    pub chamber: Account<'info, Chamber>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> UpdateChamber<'_> {
    pub fn process(&mut self) -> Result<()> {
        let Self {authority, identity, chamber,..} = self;
    
        // set creation timestamp to now
        let clock: Clock = Clock::get().unwrap();
        chamber.ts = clock.unix_timestamp as u32;

        let clock_string = format!("{}", clock.unix_timestamp);
        let clock_bytes = clock_string.as_bytes();

        let user_key = identity.authority.to_bytes();

        let key = aeskey::from_slice(&user_key);
        let cipher = aesaes::new(key);
        
        let nonce = aesnonce::from_slice(b"thisisanonce"); // 96-bits; unique per message
        
        let mut buffer: Vec<u8> = Vec::new(); // Buffer needs 16-bytes overhead for GCM tag
        buffer.extend_from_slice(b"3Xa4NGED5MDRMYX2oBpKJyUaHEXSHdYUG9LUYwwPxtH6sf9P8CPfUPoAxJzXK8JedHfhuqUdb16ewYHzTHC66UAC");
        
        
        // Encrypt `buffer` in-place, replacing the plaintext contents with ciphertext
        cipher.encrypt_in_place(nonce, b"", &mut buffer).expect("encryption failure!");
        
        // `buffer` now contains the message ciphertext
        assert_ne!(&buffer, b"3Xa4NGED5MDRMYX2oBpKJyUaHEXSHdYUG9LUYwwPxtH6sf9P8CPfUPoAxJzXK8JedHfhuqUdb16ewYHzTHC66UAC");
        
        // Decrypt `buffer` in-place, replacing its ciphertext context with the original plaintext
        //cipher.decrypt_in_place(nonce, b"", &mut buffer).expect("decryption failure!");
        //assert_eq!(&buffer, b"3Xa4NGED5MDRMYX2oBpKJyUaHEXSHdYUG9LUYwwPxtH6sf9P8CPfUPoAxJzXK8JedHfhuqUdb16ewYHzTHC66UAC");

        //let tekst = String::from_utf8(buffer).unwrap();
        self.chamber.sec = buffer;

        // for item in buffer {
        //     self.chamber.sec.push(item.to_char());
        // }

        Ok(())

    }
}


