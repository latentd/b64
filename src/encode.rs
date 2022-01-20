use super::INDEX_TABLE;

fn convert_to_sextets(input: String) -> Vec<u8> {
    let octets = input.as_bytes();
    let mut sextets: Vec<u8> = Vec::new();

    if input.len() == 0 {
        return sextets;
    };

    let mut i = 0;
    loop {
        match i % 3 {
            0 => sextets.push((octets[i] & 0b11111100) >> 2),
            1 => sextets.push((octets[i - 1] & 0b00000011) << 4 | ((octets[i] & 0b11110000) >> 4)),
            2 => {
                sextets.push((octets[i - 1] & 0b00001111) << 2 | ((octets[i] & 0b11000000) >> 6));
                sextets.push(octets[i] & 0b00111111);
            }
            _ => {}
        };
        if i >= octets.len() - 1 {
            break;
        };
        i += 1;
    }
    match i % 3 {
        0 => sextets.push((octets[i] & 0b00000011) << 4),
        1 => sextets.push((octets[i] & 0b00001111) << 2),
        _ => (),
    };

    sextets
}

fn add_padding(encoded: &mut String) {
    let length = encoded.len();
    if length % 4 != 0 {
        for _ in (length % 4)..4 {
            encoded.push('=');
        }
    };
}

fn render_sextets(sextets: Vec<u8>) -> String {
    let mut encoded = String::new();
    for s in sextets {
        encoded.push(INDEX_TABLE[s as usize]);
    }
    encoded
}

pub fn encode(input: String) -> String {
    let sextets = convert_to_sextets(input);
    let mut encoded = render_sextets(sextets);
    add_padding(&mut encoded);
    encoded
}
