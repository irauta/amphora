// Copyright 2015 Ilkka Rauta
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Ugly hack for ISO 8859-9 (not supported by encoding at 0.2; 2015-08).
// Otherwise the same as ISO 8859-1 except for six special characters.
// ISO 8859-1 character codes also match Unicode codepoints, so mapping is simple.
pub fn iso_8859_9(bytes: &[u8], string: &mut String) {
    for byte in &bytes[1..] {
        string.push(match *byte {
            0xd0 => 'Ğ',
            0xdd => 'İ',
            0xde => 'Ş',
            0xf0 => 'ğ',
            0xfd => 'ı',
            0xfe => 'ş',
            byte => byte as char,
        });
    }
}

pub fn iso_6937(bytes: &[u8], string: &mut String) {
    let mut it = bytes.iter();
    loop {
        if let Some(byte) = it.next() {
            let character = match *byte {
                character @ 0x20...0x7f => character as char,
                0xa0 => '\u{a0}',
                0xa1 => '¡',
                0xa2 => '¢',
                0xa3 => '£',
                0xa4 => '€',
                0xa5 => '¥',
                0xa7 => '§',
                0xa8 => '¤',
                0xa9 => '‘',
                0xaa => '“',
                0xab => '«',
                0xac => '←',
                0xad => '↑',
                0xae => '→',
                0xaf => '↓',

                0xb0 => '°',
                0xb1 => '±',
                0xb2 => '²',
                0xb3 => '³',
                0xb4 => '×',
                0xb5 => 'µ',
                0xb6 => '¶',
                0xb7 => '·',
                0xb8 => '÷',
                0xb9 => '’',
                0xba => '”',
                0xbb => '»',
                0xbc => '¼',
                0xbd => '½',
                0xbe => '¾',
                0xbf => '¿',

                accent @ 0xc0...0xcf => {
                    if let Some(next) = it.next() {
                        iso_6937_accented(accent, *next)
                    } else { '\u{0}' }
                },

                0xd0 => '―',
                0xd1 => '¹',
                0xd2 => '®',
                0xd3 => '©',
                0xd4 => '™',
                0xd5 => '♪',
                0xd6 => '¬',
                0xd7 => '¦',

                0xdc => '⅛',
                0xdd => '⅜',
                0xde => '⅝',
                0xdf => '⅞',

                0xe0 => 'Ω',
                0xe1 => 'Æ',
                0xe2 => 'Đ',
                0xe3 => 'ª',
                0xe4 => 'Ħ',

                0xe6 => 'Ĳ',
                0xe7 => 'Ŀ',
                0xe8 => 'Ł',
                0xe9 => 'Ø',
                0xea => 'Œ',
                0xeb => 'º',
                0xec => 'Þ',
                0xed => 'Ŧ',
                0xee => 'Ŋ',
                0xef => 'ŉ',

                0xf0 => 'ĸ',
                0xf1 => 'æ',
                0xf2 => 'đ',
                0xf3 => 'ð',
                0xf4 => 'ħ',
                0xf5 => 'ı',
                0xf6 => 'ĳ',
                0xf7 => 'ŀ',
                0xf8 => 'ł',
                0xf9 => 'ø',
                0xfa => 'œ',
                0xfb => 'ß',
                0xfc => 'þ',
                0xfd => 'ŧ',
                0xfe => 'ŋ',
                0xff => '\u{ad}',

                _ => '\u{0}'
            };
            if character != '\u{0}' {
                string.push(character);
            }
        } else {
            break;
        }
    }
}

fn iso_6937_accented(accent: u8, character: u8) -> char {
    let character = character as char;
    match accent {
        0xC1 => match character {
            'A' => 'À',
            'E' => 'È',
            'I' => 'Ì',
            'O' => 'Ò',
            'U' => 'Ù',
            'a' => 'à',
            'e' => 'è',
            'i' => 'ì',
            'o' => 'ò',
            'u' => 'ù',
            _ => '\u{0}'
        },
        0xC2 => match character {
            'A' => 'Á',
            'C' => 'Ć',
            'E' => 'É',
            'I' => 'Í',
            'L' => 'Ĺ',
            'N' => 'Ń',
            'O' => 'Ó',
            'R' => 'Ŕ',
            'S' => 'Ś',
            'U' => 'Ú',
            'Y' => 'Ý',
            'Z' => 'Ź',
            'a' => 'á',
            'c' => 'ć',
            'e' => 'é',
            'g' => 'ģ',
            'i' => 'í',
            'l' => 'ĺ',
            'n' => 'ń',
            'o' => 'ó',
            'r' => 'ŕ',
            's' => 'ś',
            'u' => 'ú',
            'y' => 'ý',
            'z' => 'ź',
            _ => '\u{0}'
        },
        0xC3 => match character {
            'A' => 'Â',
            'C' => 'Ĉ',
            'E' => 'Ê',
            'G' => 'Ĝ',
            'H' => 'Ĥ',
            'I' => 'Î',
            'J' => 'Ĵ',
            'O' => 'Ô',
            'S' => 'Ŝ',
            'U' => 'Û',
            'W' => 'Ŵ',
            'Y' => 'Ŷ',
            'a' => 'â',
            'c' => 'ĉ',
            'e' => 'ê',
            'g' => 'ĝ',
            'h' => 'ĥ',
            'i' => 'î',
            'j' => 'ĵ',
            'o' => 'ô',
            's' => 'ŝ',
            'u' => 'û',
            'w' => 'ŵ',
            'y' => 'ŷ',
            _ => '\u{0}'
        },
        0xC4 => match character {
            'A' => 'Ã',
            'I' => 'Ĩ',
            'N' => 'Ñ',
            'O' => 'Õ',
            'U' => 'Ũ',
            'a' => 'ã',
            'i' => 'ĩ',
            'n' => 'ñ',
            'o' => 'õ',
            'u' => 'ũ',
            _ => '\u{0}'
        },
        0xC5 => match character {
            'A' => 'Ā',
            'E' => 'Ē',
            'I' => 'Ī',
            'O' => 'Ō',
            'U' => 'Ū',
            'a' => 'ā',
            'e' => 'ē',
            'i' => 'ī',
            'o' => 'ō',
            'u' => 'ū',
            _ => '\u{0}'
        },
        0xC6 => match character {
            'A' => 'Ă',
            'G' => 'Ğ',
            'U' => 'Ŭ',
            'a' => 'ă',
            'g' => 'ğ',
            'u' => 'ŭ',
            _ => '\u{0}'
        },
        0xC7 => match character {
            'C' => 'Ċ',
            'E' => 'Ė',
            'G' => 'Ġ',
            'I' => 'İ',
            'Z' => 'Ż',
            'c' => 'ċ',
            'e' => 'ė',
            'g' => 'ġ',
            'z' => 'ż',
            _ => '\u{0}'
        },
        0xC8 => match character {
            'A' => 'Ä',
            'E' => 'Ë',
            'I' => 'Ï',
            'O' => 'Ö',
            'U' => 'Ü',
            'Y' => 'Ÿ',
            'a' => 'ä',
            'e' => 'ë',
            'i' => 'ï',
            'o' => 'ö',
            'u' => 'ü',
            'y' => 'ÿ',
            _ => '\u{0}'
        },
        0xCA => match character {
            'A' => 'Å',
            'U' => 'Ů',
            'a' => 'å',
            'u' => 'ů',
            _ => '\u{0}'
        },
        0xCB => match character {
            'C' => 'Ç',
            'G' => 'Ģ',
            'K' => 'Ķ',
            'L' => 'Ļ',
            'N' => 'Ņ',
            'R' => 'Ŗ',
            'S' => 'Ş',
            'T' => 'Ţ',
            'c' => 'ç',
            'k' => 'ķ',
            'l' => 'ļ',
            'n' => 'ņ',
            'r' => 'ŗ',
            's' => 'ş',
            't' => 'ţ',
            _ => '\u{0}'
        },
        0xCD => match character {
            'O' => 'Ő',
            'U' => 'Ű',
            'o' => 'ő',
            'u' => 'ű',
            _ => '\u{0}'
        },
        0xCE => match character {
            'A' => 'Ą',
            'E' => 'Ę',
            'I' => 'Į',
            'U' => 'Ų',
            'a' => 'ą',
            'e' => 'ę',
            'i' => 'į',
            'u' => 'ų',
            _ => '\u{0}'
        },
        0xCF => match character {
            'C' => 'Č',
            'D' => 'Ď',
            'E' => 'Ě',
            'L' => 'Ľ',
            'N' => 'Ň',
            'R' => 'Ř',
            'S' => 'Š',
            'T' => 'Ť',
            'Z' => 'Ž',
            'c' => 'č',
            'd' => 'ď',
            'e' => 'ě',
            'l' => 'ľ',
            'n' => 'ň',
            'r' => 'ř',
            's' => 'š',
            't' => 'ť',
            'z' => 'ž',
            _ => '\u{0}'
        },
        _=> '\u{0}'
    }
}
