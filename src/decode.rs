use super::{B64Result, INDEX_TABLE};

fn strip_padding(input: String) -> String {
    input.trim_end_matches('=').to_string()
}

fn reverse_render_sextets(text: String) -> B64Result<Vec<u8>> {
    Ok(text.chars()
        .map(|c| INDEX_TABLE.iter().position(|v| v == &c).expect(&format!("unexpected character: {}", c)))
        .map(|c| c as u8)
        .collect::<Vec<u8>>())
}

fn reverse_convert_to_sextets(sextets: Vec<u8>) -> B64Result<String> {
    let mut decoded = String::new();
    for chunk in sextets.chunks(4).collect::<Vec<&[u8]>>() {
        let length = chunk.len();
        let mut bits: u32 = 0;
        for i in 0..length {
            bits |= (chunk[i] as u32) << (3 - i) * 6;
        }
        let bytes = bits.to_be_bytes()[1..length].to_vec();
        decoded.push_str(std::str::from_utf8(&bytes)?);
    }
    Ok(decoded)
}

pub fn decode(input: String) -> B64Result<String> {
    let stripped = strip_padding(input);
    let sextets = reverse_render_sextets(stripped)?;
    Ok(reverse_convert_to_sextets(sextets)?)
}
