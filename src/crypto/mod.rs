use chacha20::cipher::{NewCipher, StreamCipher};
use chacha20::{ChaCha20, Key, Nonce};

struct CipherReader<T: std::io::Read> {
    cipher: ChaCha20,
    reader: T,
}

impl<T: std::io::Read> std::io::Read for CipherReader<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let count = self.reader.read(buf)?;
        if count == buf.len() {
            self.cipher.apply_keystream(buf);
        } else {
            self.cipher.apply_keystream(&mut buf[..count])
        }
        Ok(count)
    }
}

pub fn crypto_chacha20(
    key: &[u8; 32],
    nonce: &[u8; 12],
    input_file_reader: impl std::io::Read,
    mut writer: impl std::io::Write,
    progress_bar: indicatif::ProgressBar,
) {
    let key = Key::from_slice(key);
    let nonce = Nonce::from_slice(nonce);

    let cipher = ChaCha20::new(key, nonce);

    let mut read_cipher = CipherReader {
        cipher,
        reader: input_file_reader,
    };

    let pre_time = chrono::Local::now();
    let _ = std::io::copy(&mut read_cipher, &mut progress_bar.wrap_write(&mut writer));
    progress_bar.finish();
    let post_time = chrono::Local::now();
    println!("{:?}", post_time - pre_time);
}
