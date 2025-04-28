pub fn decode_dns_name<'a>(mut input: &'a [u8], mut backlog: &'a [u8]) -> Option<Vec<u8>> {
    let mut result = Vec::with_capacity(256);
    loop {
        match usize::from(*input.first()?) {
            0 => break,
            prefix @ ..=0x3F if result.len() + prefix <= 255 => {
                let part;
                (part, input) = input[1..].split_at_checked(prefix)?;
                result.extend_from_slice(part);
                result.push(b'.');
            }
            0xC0.. => {
                let (offset_bytes, _) = input.split_first_chunk()?;
                let offset = u16::from_be_bytes(*offset_bytes) & !0xC000;
                (backlog, input) = backlog.split_at_checked(usize::from(offset))?;
            }
            _ => return None,
        }
    }
    result.pop()?;
    Some(result)
}

fn main() {}
