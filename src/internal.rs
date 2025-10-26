/// Convert an ASCII or Russian UTF-8 character to the LCD encoding.
const fn char_to_lcd(c: char) -> Option<u8> {
	match c {
		// ASCII
		'\x00'..='\x7F' => Some(c as u8),

		// Uppercase Russian
		'А' => Some('A' as u8),
		'Б' => Some(0xA0),
		'В' => Some('B' as u8),
		'Г' => Some(0xA1),
		'Д' => Some(0xE0),
		'Е' => Some('E' as u8),
		'Ё' => Some(0xA2),
		'Ж' => Some(0xA3),
		'З' => Some(0xA4),
		'И' => Some(0xA5),
		'Й' => Some(0xA6),
		'К' => Some('K' as u8),
		'Л' => Some(0xA7),
		'М' => Some('M' as u8),
		'Н' => Some('H' as u8),
		'О' => Some('O' as u8),
		'П' => Some(0xA8),
		'Р' => Some('P' as u8),
		'С' => Some('C' as u8),
		'Т' => Some('T' as u8),
		'У' => Some(0xA9),
		'Ф' => Some(0xAA),
		'Х' => Some('X' as u8),
		'Ц' => Some(0xE1),
		'Ч' => Some(0xAB),
		'Ш' => Some(0xAC),
		'Щ' => Some(0xE2),
		'Ъ' => Some(0xAD),
		'Ы' => Some(0xAE),
		'Ь' => Some('b' as u8), // I think?
		'Э' => Some(0xAF),
		'Ю' => Some(0xB0),
		'Я' => Some(0xB1),

		// Lowercase Russian
		'а' => Some('a' as u8),
		'б' => Some(0xB2),
		'в' => Some(0xB3),
		'г' => Some(0xB4),
		'д' => Some(0xE3),
		'е' => Some('e' as u8),
		'ё' => Some(0xB5),
		'ж' => Some(0xB6),
		'з' => Some(0xB7),
		'и' => Some(0xB8),
		'й' => Some(0xB9),
		'к' => Some(0xBA),
		'л' => Some(0xBB),
		'м' => Some(0xBC),
		'н' => Some(0xBD),
		'о' => Some('o' as u8),
		'п' => Some(0xBE),
		'р' => Some('p' as u8),
		'с' => Some('c' as u8),
		'т' => Some(0xBF),
		'у' => Some('y' as u8),
		'ф' => Some(0xE4),
		'х' => Some('x' as u8),
		'ц' => Some(0xE5),
		'ч' => Some(0xC0),
		'ш' => Some(0xC1),
		'щ' => Some(0xE6),
		'ъ' => Some(0xC2),
		'ы' => Some(0xC3),
		'ь' => Some(0xC4),
		'э' => Some(0xC5),
		'ю' => Some(0xC6),
		'я' => Some(0xC7),

		// Other
		_ => None,
	}
}

/// Read a UTF-8 character from a string, returning its code and length in bytes.
///
/// Results are undefined for non-UTF-8 inputs.
///
/// `start_pos` is included since slicing does not work in const contexts.
const fn decode_utf8_char(s: &str, start_pos: usize) -> (char, usize) {
	let s = s.as_bytes();
	let mut first_byte = s[start_pos];
	if first_byte.is_ascii() {
		(first_byte as char, 1usize)
	} else {
		// Use u32 for temporary values to carry out validation later
		let mut res = 0u32;
		let mut cont_byte_count = 0usize;
		while first_byte & 0x40 != 0 {
			cont_byte_count += 1;
			res = (res as u32) << 6 | (s[start_pos + cont_byte_count] & 0x3F) as u32;
			first_byte = first_byte << 1;
		}
		res = res | ((first_byte as u32 & 0x7F) << (cont_byte_count * 5));
		(
			char::from_u32(res).expect("Failed to decode a UTF-8 character"),
			cont_byte_count + 1,
		)
	}
}

/// Get length of a UTF-8 string, in code points.
pub const fn utf8_len(s: &str) -> usize {
	let mut length = 0usize;
	let mut input_pos = 0usize;
	while input_pos < s.len() {
		let (_, len) = decode_utf8_char(&s, input_pos);
		input_pos += len;
		length += 1;
	}
	length
}

/// Transcode a string to LCD encoding at compile time.
///
/// This is used by macros and should not be accessed otherwise.
pub const fn str_to_lcd_const<const LEN: usize>(s: &str) -> [u8; LEN] {
	let mut output = [0u8; LEN];
	let mut input_pos = 0usize;
	let mut output_pos = 0usize;

	while input_pos < s.len() {
		let (char, len) = decode_utf8_char(&s, input_pos);
		output[output_pos] = char_to_lcd(char).expect("Failed to transcode a character");
		input_pos += len;
		output_pos += 1;
	}

	output
}

/// See [crate::lcd_encode_runtime] for documentation.
pub fn str_to_lcd_runtime(output: &mut [u8], s: &str) -> Result<usize, usize> {
	let mut input_pos = 0usize;
	let mut output_pos = 0usize;

	while input_pos < s.len() {
		let (char, len) = decode_utf8_char(&s, input_pos);
		if output_pos >= output.len() {
			// Averted a panic from a buffer overrun
			return Err(output_pos);
		} else if let Some(char) = char_to_lcd(char) {
			// Just checked for the overrun above
			unsafe { *output.get_unchecked_mut(output_pos) = char }
		} else {
			// Failed to transcode a character
			return Err(output_pos);
		}
		input_pos += len;
		output_pos += 1;
	}

	Ok(output_pos)
}
