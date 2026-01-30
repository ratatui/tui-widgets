#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PixelSize {
    #[default]
    /// A pixel from the 8x8 font is represented by a full character cell in the terminal.
    Full,
    /// A pixel from the 8x8 font is represented by a half (upper/lower) character cell in the
    /// terminal.
    HalfHeight,
    /// A pixel from the 8x8 font is represented by a half (left/right) character cell in the
    /// terminal.
    HalfWidth,
    /// A pixel from the 8x8 font is represented by a quadrant of a character cell in the
    /// terminal.
    Quadrant,
    /// A pixel from the 8x8 font is represented by a third (top/middle/bottom) of a character
    /// cell in the terminal.
    /// *Note: depending on how the used terminal renders characters, the generated text with
    /// this PixelSize might look very strange.*
    ThirdHeight,
    /// A pixel from the 8x8 font is represented by a sextant of a character cell in the
    /// terminal.
    /// *Note: depending on how the used terminal renders characters, the generated text with
    /// this PixelSize might look very strange.*
    Sextant,
    /// A pixel from the 8x8 font is represented by a quarter
    /// (top/upper-middle/lower-middle/bottom) of a character cell in the terminal.
    /// *Note: depending on how the used terminal renders characters, the generated text with
    /// this PixelSize might look very strange.*
    QuarterHeight,
    /// A pixel from the 8x8 font is represented by an octant of a character cell in the
    /// terminal.
    /// *Note: depending on how the used terminal renders characters, the generated text with
    /// this PixelSize might look very strange.*
    Octant,
}

impl PixelSize {
    /// The number of pixels that can be displayed in a single character cell for the given
    /// pixel size.
    ///
    /// The first value is the number of pixels in the horizontal direction, the second value is
    /// the number of pixels in the vertical direction.
    pub(crate) const fn pixels_per_cell(self) -> (u16, u16) {
        match self {
            Self::Full => (1, 1),
            Self::HalfHeight => (1, 2),
            Self::HalfWidth => (2, 1),
            Self::Quadrant => (2, 2),
            Self::ThirdHeight => (1, 3),
            Self::Sextant => (2, 3),
            Self::QuarterHeight => (1, 4),
            Self::Octant => (2, 4),
        }
    }

    /// Get a symbol/char that represents the pixels at the given position with the given pixel size
    pub(crate) const fn symbol_for_position(self, glyph: &[u8; 8], row: usize, col: i32) -> char {
        match self {
            Self::Full => match glyph[row] & (1 << col) {
                0 => ' ',
                _ => '█',
            },
            Self::HalfHeight => {
                let top = glyph[row] & (1 << col);
                let bottom = glyph[row + 1] & (1 << col);
                get_symbol_half_height(top, bottom)
            }
            Self::HalfWidth => {
                let left = glyph[row] & (1 << col);
                let right = glyph[row] & (1 << (col + 1));
                get_symbol_half_width(left, right)
            }
            Self::Quadrant => {
                let top_left = glyph[row] & (1 << col);
                let top_right = glyph[row] & (1 << (col + 1));
                let bottom_left = glyph[row + 1] & (1 << col);
                let bottom_right = glyph[row + 1] & (1 << (col + 1));
                get_symbol_quadrant_size(top_left, top_right, bottom_left, bottom_right)
            }
            Self::ThirdHeight => {
                let top = glyph[row] & (1 << col);
                let is_middle_available = (row + 1) < glyph.len();
                let middle = if is_middle_available {
                    glyph[row + 1] & (1 << col)
                } else {
                    0
                };
                let is_bottom_available = (row + 2) < glyph.len();
                let bottom = if is_bottom_available {
                    glyph[row + 2] & (1 << col)
                } else {
                    0
                };
                get_symbol_third_height(top, middle, bottom)
            }
            Self::Sextant => {
                let top_left = glyph[row] & (1 << col);
                let top_right = glyph[row] & (1 << (col + 1));
                let is_middle_available = (row + 1) < glyph.len();
                let (middle_left, middle_right) = if is_middle_available {
                    (
                        glyph[row + 1] & (1 << col),
                        glyph[row + 1] & (1 << (col + 1)),
                    )
                } else {
                    (0, 0)
                };
                let is_bottom_available = (row + 2) < glyph.len();
                let (bottom_left, bottom_right) = if is_bottom_available {
                    (
                        glyph[row + 2] & (1 << col),
                        glyph[row + 2] & (1 << (col + 1)),
                    )
                } else {
                    (0, 0)
                };
                get_symbol_sextant_size(
                    top_left,
                    top_right,
                    middle_left,
                    middle_right,
                    bottom_left,
                    bottom_right,
                )
            }
            Self::QuarterHeight => {
                let top = glyph[row] & (1 << col);
                let is_upper_middle_available = (row + 1) < glyph.len();
                let upper_middle = if is_upper_middle_available {
                    glyph[row + 1] & (1 << col)
                } else {
                    0
                };
                let is_lower_middle_available = (row + 2) < glyph.len();
                let lower_middle = if is_lower_middle_available {
                    glyph[row + 2] & (1 << col)
                } else {
                    0
                };
                let is_bottom_available = (row + 3) < glyph.len();
                let bottom = if is_bottom_available {
                    glyph[row + 3] & (1 << col)
                } else {
                    0
                };
                get_symbol_quarter_height(top, upper_middle, lower_middle, bottom)
            }
            Self::Octant => {
                let top_left = glyph[row] & (1 << col);
                let top_right = glyph[row] & (1 << (col + 1));
                let is_upper_middle_available = (row + 1) < glyph.len();
                let (upper_middle_left, upper_middle_right) = if is_upper_middle_available {
                    (
                        glyph[row + 1] & (1 << col),
                        glyph[row + 1] & (1 << (col + 1)),
                    )
                } else {
                    (0, 0)
                };
                let is_lower_middle_available = (row + 2) < glyph.len();
                let (lower_middle_left, lower_middle_right) = if is_lower_middle_available {
                    (
                        glyph[row + 2] & (1 << col),
                        glyph[row + 2] & (1 << (col + 1)),
                    )
                } else {
                    (0, 0)
                };
                let is_bottom_available = (row + 3) < glyph.len();
                let (bottom_left, bottom_right) = if is_bottom_available {
                    (
                        glyph[row + 3] & (1 << col),
                        glyph[row + 3] & (1 << (col + 1)),
                    )
                } else {
                    (0, 0)
                };
                get_symbol_octant_size(
                    top_left,
                    top_right,
                    upper_middle_left,
                    upper_middle_right,
                    lower_middle_left,
                    lower_middle_right,
                    bottom_left,
                    bottom_right,
                )
            }
        }
    }
}

/// Get the correct unicode symbol for two vertical "pixels"
const fn get_symbol_half_height(top: u8, bottom: u8) -> char {
    match top {
        0 => match bottom {
            0 => ' ',
            _ => '▄',
        },
        _ => match bottom {
            0 => '▀',
            _ => '█',
        },
    }
}

/// Get the correct unicode symbol for two horizontal "pixels"
const fn get_symbol_half_width(left: u8, right: u8) -> char {
    match left {
        0 => match right {
            0 => ' ',
            _ => '▐',
        },
        _ => match right {
            0 => '▌',
            _ => '█',
        },
    }
}

/// Get the correct unicode symbol for 2x2 "pixels"
const fn get_symbol_quadrant_size(
    top_left: u8,
    top_right: u8,
    bottom_left: u8,
    bottom_right: u8,
) -> char {
    let top_left = if top_left > 0 { 1 } else { 0 };
    let top_right = if top_right > 0 { 1 } else { 0 };
    let bottom_left = if bottom_left > 0 { 1 } else { 0 };
    let bottom_right = if bottom_right > 0 { 1 } else { 0 };

    // We use an array here instead of directly indexing into the unicode symbols, because although
    // most symbols are in order in unicode, some of them are already part of another character set
    // and missing in this character set.
    const QUADRANT_SYMBOLS: [char; 16] = [
        ' ', '▘', '▝', '▀', '▖', '▌', '▞', '▛', '▗', '▚', '▐', '▜', '▄', '▙', '▟', '█',
    ];
    let character_index = top_left + (top_right << 1) + (bottom_left << 2) + (bottom_right << 3);

    QUADRANT_SYMBOLS[character_index]
}

/// Get the correct unicode symbol for 1x3 "pixels"
const fn get_symbol_third_height(top: u8, middle: u8, bottom: u8) -> char {
    get_symbol_sextant_size(top, top, middle, middle, bottom, bottom)
}

/// Get the correct unicode symbol for 2x3 "pixels"
const fn get_symbol_sextant_size(
    top_left: u8,
    top_right: u8,
    middle_left: u8,
    middle_right: u8,
    bottom_left: u8,
    bottom_right: u8,
) -> char {
    let top_left = if top_left > 0 { 1 } else { 0 };
    let top_right = if top_right > 0 { 1 } else { 0 };
    let middle_left = if middle_left > 0 { 1 } else { 0 };
    let middle_right = if middle_right > 0 { 1 } else { 0 };
    let bottom_left = if bottom_left > 0 { 1 } else { 0 };
    let bottom_right = if bottom_right > 0 { 1 } else { 0 };

    // We use an array here instead of directly indexing into the unicode symbols, because although
    // most symbols are in order in unicode, some of them are already part of another character set
    // and missing in this character set.
    const SEXANT_SYMBOLS: [char; 64] = [
        ' ', '🬀', '🬁', '🬂', '🬃', '🬄', '🬅', '🬆', '🬇', '🬈', '🬉', '🬊', '🬋', '🬌', '🬍', '🬎', '🬏', '🬐',
        '🬑', '🬒', '🬓', '▌', '🬔', '🬕', '🬖', '🬗', '🬘', '🬙', '🬚', '🬛', '🬜', '🬝', '🬞', '🬟', '🬠', '🬡',
        '🬢', '🬣', '🬤', '🬥', '🬦', '🬧', '▐', '🬨', '🬩', '🬪', '🬫', '🬬', '🬭', '🬮', '🬯', '🬰', '🬱', '🬲',
        '🬳', '🬴', '🬵', '🬶', '🬷', '🬸', '🬹', '🬺', '🬻', '█',
    ];
    let character_index = top_left
        + (top_right << 1)
        + (middle_left << 2)
        + (middle_right << 3)
        + (bottom_left << 4)
        + (bottom_right << 5);

    SEXANT_SYMBOLS[character_index]
}

/// Get the correct unicode symbol for 1x4 "pixels"
const fn get_symbol_quarter_height(
    top: u8,
    upper_middle: u8,
    lower_middle: u8,
    bottom: u8,
) -> char {
    get_symbol_octant_size(
        top,
        top,
        upper_middle,
        upper_middle,
        lower_middle,
        lower_middle,
        bottom,
        bottom,
    )
}

/// Get the correct unicode symbol for 2x4 "pixels"
#[allow(clippy::too_many_arguments)]
const fn get_symbol_octant_size(
    top_left: u8,
    top_right: u8,
    upper_middle_left: u8,
    upper_middle_right: u8,
    lower_middle_left: u8,
    lower_middle_right: u8,
    bottom_left: u8,
    bottom_right: u8,
) -> char {
    let top_left = if top_left > 0 { 1 } else { 0 };
    let top_right = if top_right > 0 { 1 } else { 0 };
    let upper_middle_left = if upper_middle_left > 0 { 1 } else { 0 };
    let upper_middle_right = if upper_middle_right > 0 { 1 } else { 0 };
    let lower_middle_left = if lower_middle_left > 0 { 1 } else { 0 };
    let lower_middle_right = if lower_middle_right > 0 { 1 } else { 0 };
    let bottom_left = if bottom_left > 0 { 1 } else { 0 };
    let bottom_right = if bottom_right > 0 { 1 } else { 0 };

    // We use an array here instead of directly indexing into the unicode symbols, because although
    // most symbols are in order in unicode, some of them are already part of another character set
    // and missing in this character set.
    const OCTANT_SYMBOLS: [char; 256] = [
        ' ', '𜺨', '𜺫', '🮂', '𜴀', '▘', '𜴁', '𜴂', '𜴃', '𜴄', '▝', '𜴅', '𜴆', '𜴇', '𜴈', '▀', '𜴉', '𜴊',
        '𜴋', '𜴌', '🯦', '𜴍', '𜴎', '𜴏', '𜴐', '𜴑', '𜴒', '𜴓', '𜴔', '𜴕', '𜴖', '𜴗', '𜴘', '𜴙', '𜴚', '𜴛',
        '𜴜', '𜴝', '𜴞', '𜴟', '🯧', '𜴠', '𜴡', '𜴢', '𜴣', '𜴤', '𜴥', '𜴦', '𜴧', '𜴨', '𜴩', '𜴪', '𜴫', '𜴬',
        '𜴭', '𜴮', '𜴯', '𜴰', '𜴱', '𜴲', '𜴳', '𜴴', '𜴵', '🮅', '𜺣', '𜴶', '𜴷', '𜴸', '𜴹', '𜴺', '𜴻', '𜴼',
        '𜴽', '𜴾', '𜴿', '𜵀', '𜵁', '𜵂', '𜵃', '𜵄', '▖', '𜵅', '𜵆', '𜵇', '𜵈', '▌', '𜵉', '𜵊', '𜵋', '𜵌',
        '▞', '𜵍', '𜵎', '𜵏', '𜵐', '▛', '𜵑', '𜵒', '𜵓', '𜵔', '𜵕', '𜵖', '𜵗', '𜵘', '𜵙', '𜵚', '𜵛', '𜵜',
        '𜵝', '𜵞', '𜵟', '𜵠', '𜵡', '𜵢', '𜵣', '𜵤', '𜵥', '𜵦', '𜵧', '𜵨', '𜵩', '𜵪', '𜵫', '𜵬', '𜵭', '𜵮',
        '𜵯', '𜵰', '𜺠', '𜵱', '𜵲', '𜵳', '𜵴', '𜵵', '𜵶', '𜵷', '𜵸', '𜵹', '𜵺', '𜵻', '𜵼', '𜵽', '𜵾', '𜵿',
        '𜶀', '𜶁', '𜶂', '𜶃', '𜶄', '𜶅', '𜶆', '𜶇', '𜶈', '𜶉', '𜶊', '𜶋', '𜶌', '𜶍', '𜶎', '𜶏', '▗', '𜶐',
        '𜶑', '𜶒', '𜶓', '▚', '𜶔', '𜶕', '𜶖', '𜶗', '▐', '𜶘', '𜶙', '𜶚', '𜶛', '▜', '𜶜', '𜶝', '𜶞', '𜶟',
        '𜶠', '𜶡', '𜶢', '𜶣', '𜶤', '𜶥', '𜶦', '𜶧', '𜶨', '𜶩', '𜶪', '𜶫', '▂', '𜶬', '𜶭', '𜶮', '𜶯', '𜶰',
        '𜶱', '𜶲', '𜶳', '𜶴', '𜶵', '𜶶', '𜶷', '𜶸', '𜶹', '𜶺', '𜶻', '𜶼', '𜶽', '𜶾', '𜶿', '𜷀', '𜷁', '𜷂',
        '𜷃', '𜷄', '𜷅', '𜷆', '𜷇', '𜷈', '𜷉', '𜷊', '𜷋', '𜷌', '𜷍', '𜷎', '𜷏', '𜷐', '𜷑', '𜷒', '𜷓', '𜷔',
        '𜷕', '𜷖', '𜷗', '𜷘', '𜷙', '𜷚', '▄', '𜷛', '𜷜', '𜷝', '𜷞', '▙', '𜷟', '𜷠', '𜷡', '𜷢', '▟', '𜷣',
        '▆', '𜷤', '𜷥', '█',
    ];
    let character_index = top_left
        + (top_right << 1)
        + (upper_middle_left << 2)
        + (upper_middle_right << 3)
        + (lower_middle_left << 4)
        + (lower_middle_right << 5)
        + (bottom_left << 6)
        + (bottom_right << 7);

    OCTANT_SYMBOLS[character_index]
}

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;

    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn core::error::Error>>;

    #[test]
    fn check_quadrant_size_symbols() -> Result<()> {
        assert_eq!(get_symbol_quadrant_size(0, 0, 0, 0), ' ');
        assert_eq!(get_symbol_quadrant_size(1, 0, 0, 0), '▘');
        assert_eq!(get_symbol_quadrant_size(0, 1, 0, 0), '▝');
        assert_eq!(get_symbol_quadrant_size(1, 1, 0, 0), '▀');
        assert_eq!(get_symbol_quadrant_size(0, 0, 1, 0), '▖');
        assert_eq!(get_symbol_quadrant_size(1, 0, 1, 0), '▌');
        assert_eq!(get_symbol_quadrant_size(0, 1, 1, 0), '▞');
        assert_eq!(get_symbol_quadrant_size(1, 1, 1, 0), '▛');
        assert_eq!(get_symbol_quadrant_size(0, 0, 0, 1), '▗');
        assert_eq!(get_symbol_quadrant_size(1, 0, 0, 1), '▚');
        assert_eq!(get_symbol_quadrant_size(0, 1, 0, 1), '▐');
        assert_eq!(get_symbol_quadrant_size(1, 1, 0, 1), '▜');
        assert_eq!(get_symbol_quadrant_size(0, 0, 1, 1), '▄');
        assert_eq!(get_symbol_quadrant_size(1, 0, 1, 1), '▙');
        assert_eq!(get_symbol_quadrant_size(0, 1, 1, 1), '▟');
        assert_eq!(get_symbol_quadrant_size(1, 1, 1, 1), '█');
        Ok(())
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn check_sextant_size_symbols() -> Result<()> {
        assert_eq!(get_symbol_sextant_size(0, 0, 0, 0, 0, 0), ' ');
        assert_eq!(get_symbol_sextant_size(1, 0, 0, 0, 0, 0), '🬀');
        assert_eq!(get_symbol_sextant_size(0, 1, 0, 0, 0, 0), '🬁');
        assert_eq!(get_symbol_sextant_size(1, 1, 0, 0, 0, 0), '🬂');
        assert_eq!(get_symbol_sextant_size(0, 0, 1, 0, 0, 0), '🬃');
        assert_eq!(get_symbol_sextant_size(1, 0, 1, 0, 0, 0), '🬄');
        assert_eq!(get_symbol_sextant_size(0, 1, 1, 0, 0, 0), '🬅');
        assert_eq!(get_symbol_sextant_size(1, 1, 1, 0, 0, 0), '🬆');
        assert_eq!(get_symbol_sextant_size(0, 0, 0, 1, 0, 0), '🬇');
        assert_eq!(get_symbol_sextant_size(1, 0, 0, 1, 0, 0), '🬈');
        assert_eq!(get_symbol_sextant_size(0, 1, 0, 1, 0, 0), '🬉');
        assert_eq!(get_symbol_sextant_size(1, 1, 0, 1, 0, 0), '🬊');
        assert_eq!(get_symbol_sextant_size(0, 0, 1, 1, 0, 0), '🬋');
        assert_eq!(get_symbol_sextant_size(1, 0, 1, 1, 0, 0), '🬌');
        assert_eq!(get_symbol_sextant_size(0, 1, 1, 1, 0, 0), '🬍');
        assert_eq!(get_symbol_sextant_size(1, 1, 1, 1, 0, 0), '🬎');
        assert_eq!(get_symbol_sextant_size(0, 0, 0, 0, 1, 0), '🬏');
        assert_eq!(get_symbol_sextant_size(1, 0, 0, 0, 1, 0), '🬐');
        assert_eq!(get_symbol_sextant_size(0, 1, 0, 0, 1, 0), '🬑');
        assert_eq!(get_symbol_sextant_size(1, 1, 0, 0, 1, 0), '🬒');
        assert_eq!(get_symbol_sextant_size(0, 0, 1, 0, 1, 0), '🬓');
        assert_eq!(get_symbol_sextant_size(1, 0, 1, 0, 1, 0), '▌');
        assert_eq!(get_symbol_sextant_size(0, 1, 1, 0, 1, 0), '🬔');
        assert_eq!(get_symbol_sextant_size(1, 1, 1, 0, 1, 0), '🬕');
        assert_eq!(get_symbol_sextant_size(0, 0, 0, 1, 1, 0), '🬖');
        assert_eq!(get_symbol_sextant_size(1, 0, 0, 1, 1, 0), '🬗');
        assert_eq!(get_symbol_sextant_size(0, 1, 0, 1, 1, 0), '🬘');
        assert_eq!(get_symbol_sextant_size(1, 1, 0, 1, 1, 0), '🬙');
        assert_eq!(get_symbol_sextant_size(0, 0, 1, 1, 1, 0), '🬚');
        assert_eq!(get_symbol_sextant_size(1, 0, 1, 1, 1, 0), '🬛');
        assert_eq!(get_symbol_sextant_size(0, 1, 1, 1, 1, 0), '🬜');
        assert_eq!(get_symbol_sextant_size(1, 1, 1, 1, 1, 0), '🬝');
        assert_eq!(get_symbol_sextant_size(0, 0, 0, 0, 0, 1), '🬞');
        assert_eq!(get_symbol_sextant_size(1, 0, 0, 0, 0, 1), '🬟');
        assert_eq!(get_symbol_sextant_size(0, 1, 0, 0, 0, 1), '🬠');
        assert_eq!(get_symbol_sextant_size(1, 1, 0, 0, 0, 1), '🬡');
        assert_eq!(get_symbol_sextant_size(0, 0, 1, 0, 0, 1), '🬢');
        assert_eq!(get_symbol_sextant_size(1, 0, 1, 0, 0, 1), '🬣');
        assert_eq!(get_symbol_sextant_size(0, 1, 1, 0, 0, 1), '🬤');
        assert_eq!(get_symbol_sextant_size(1, 1, 1, 0, 0, 1), '🬥');
        assert_eq!(get_symbol_sextant_size(0, 0, 0, 1, 0, 1), '🬦');
        assert_eq!(get_symbol_sextant_size(1, 0, 0, 1, 0, 1), '🬧');
        assert_eq!(get_symbol_sextant_size(0, 1, 0, 1, 0, 1), '▐');
        assert_eq!(get_symbol_sextant_size(1, 1, 0, 1, 0, 1), '🬨');
        assert_eq!(get_symbol_sextant_size(0, 0, 1, 1, 0, 1), '🬩');
        assert_eq!(get_symbol_sextant_size(1, 0, 1, 1, 0, 1), '🬪');
        assert_eq!(get_symbol_sextant_size(0, 1, 1, 1, 0, 1), '🬫');
        assert_eq!(get_symbol_sextant_size(1, 1, 1, 1, 0, 1), '🬬');
        assert_eq!(get_symbol_sextant_size(0, 0, 0, 0, 1, 1), '🬭');
        assert_eq!(get_symbol_sextant_size(1, 0, 0, 0, 1, 1), '🬮');
        assert_eq!(get_symbol_sextant_size(0, 1, 0, 0, 1, 1), '🬯');
        assert_eq!(get_symbol_sextant_size(1, 1, 0, 0, 1, 1), '🬰');
        assert_eq!(get_symbol_sextant_size(0, 0, 1, 0, 1, 1), '🬱');
        assert_eq!(get_symbol_sextant_size(1, 0, 1, 0, 1, 1), '🬲');
        assert_eq!(get_symbol_sextant_size(0, 1, 1, 0, 1, 1), '🬳');
        assert_eq!(get_symbol_sextant_size(1, 1, 1, 0, 1, 1), '🬴');
        assert_eq!(get_symbol_sextant_size(0, 0, 0, 1, 1, 1), '🬵');
        assert_eq!(get_symbol_sextant_size(1, 0, 0, 1, 1, 1), '🬶');
        assert_eq!(get_symbol_sextant_size(0, 1, 0, 1, 1, 1), '🬷');
        assert_eq!(get_symbol_sextant_size(1, 1, 0, 1, 1, 1), '🬸');
        assert_eq!(get_symbol_sextant_size(0, 0, 1, 1, 1, 1), '🬹');
        assert_eq!(get_symbol_sextant_size(1, 0, 1, 1, 1, 1), '🬺');
        assert_eq!(get_symbol_sextant_size(0, 1, 1, 1, 1, 1), '🬻');
        assert_eq!(get_symbol_sextant_size(1, 1, 1, 1, 1, 1), '█');
        Ok(())
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn check_octant_size_symbols() -> Result<()> {
        assert_eq!(get_symbol_octant_size(0, 0, 0, 0, 0, 0, 0, 0), ' ');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 0, 0, 0, 0, 0), '𜺨');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 0, 0, 0, 0, 0), '𜺫');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 0, 0, 0, 0, 0), '🮂');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 0, 0, 0, 0, 0), '𜴀');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 0, 0, 0, 0, 0), '▘');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 0, 0, 0, 0, 0), '𜴁');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 0, 0, 0, 0, 0), '𜴂');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 1, 0, 0, 0, 0), '𜴃');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 1, 0, 0, 0, 0), '𜴄');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 1, 0, 0, 0, 0), '▝');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 1, 0, 0, 0, 0), '𜴅');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 1, 0, 0, 0, 0), '𜴆');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 1, 0, 0, 0, 0), '𜴇');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 1, 0, 0, 0, 0), '𜴈');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 1, 0, 0, 0, 0), '▀');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 0, 1, 0, 0, 0), '𜴉');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 0, 1, 0, 0, 0), '𜴊');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 0, 1, 0, 0, 0), '𜴋');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 0, 1, 0, 0, 0), '𜴌');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 0, 1, 0, 0, 0), '🯦');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 0, 1, 0, 0, 0), '𜴍');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 0, 1, 0, 0, 0), '𜴎');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 0, 1, 0, 0, 0), '𜴏');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 1, 1, 0, 0, 0), '𜴐');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 1, 1, 0, 0, 0), '𜴑');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 1, 1, 0, 0, 0), '𜴒');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 1, 1, 0, 0, 0), '𜴓');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 1, 1, 0, 0, 0), '𜴔');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 1, 1, 0, 0, 0), '𜴕');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 1, 1, 0, 0, 0), '𜴖');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 1, 1, 0, 0, 0), '𜴗');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 0, 0, 1, 0, 0), '𜴘');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 0, 0, 1, 0, 0), '𜴙');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 0, 0, 1, 0, 0), '𜴚');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 0, 0, 1, 0, 0), '𜴛');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 0, 0, 1, 0, 0), '𜴜');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 0, 0, 1, 0, 0), '𜴝');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 0, 0, 1, 0, 0), '𜴞');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 0, 0, 1, 0, 0), '𜴟');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 1, 0, 1, 0, 0), '🯧');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 1, 0, 1, 0, 0), '𜴠');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 1, 0, 1, 0, 0), '𜴡');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 1, 0, 1, 0, 0), '𜴢');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 1, 0, 1, 0, 0), '𜴣');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 1, 0, 1, 0, 0), '𜴤');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 1, 0, 1, 0, 0), '𜴥');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 1, 0, 1, 0, 0), '𜴦');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 0, 1, 1, 0, 0), '𜴧');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 0, 1, 1, 0, 0), '𜴨');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 0, 1, 1, 0, 0), '𜴩');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 0, 1, 1, 0, 0), '𜴪');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 0, 1, 1, 0, 0), '𜴫');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 0, 1, 1, 0, 0), '𜴬');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 0, 1, 1, 0, 0), '𜴭');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 0, 1, 1, 0, 0), '𜴮');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 1, 1, 1, 0, 0), '𜴯');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 1, 1, 1, 0, 0), '𜴰');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 1, 1, 1, 0, 0), '𜴱');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 1, 1, 1, 0, 0), '𜴲');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 1, 1, 1, 0, 0), '𜴳');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 1, 1, 1, 0, 0), '𜴴');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 1, 1, 1, 0, 0), '𜴵');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 1, 1, 1, 0, 0), '🮅');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 0, 0, 0, 1, 0), '𜺣');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 0, 0, 0, 1, 0), '𜴶');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 0, 0, 0, 1, 0), '𜴷');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 0, 0, 0, 1, 0), '𜴸');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 0, 0, 0, 1, 0), '𜴹');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 0, 0, 0, 1, 0), '𜴺');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 0, 0, 0, 1, 0), '𜴻');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 0, 0, 0, 1, 0), '𜴼');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 1, 0, 0, 1, 0), '𜴽');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 1, 0, 0, 1, 0), '𜴾');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 1, 0, 0, 1, 0), '𜴿');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 1, 0, 0, 1, 0), '𜵀');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 1, 0, 0, 1, 0), '𜵁');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 1, 0, 0, 1, 0), '𜵂');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 1, 0, 0, 1, 0), '𜵃');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 1, 0, 0, 1, 0), '𜵄');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 0, 1, 0, 1, 0), '▖');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 0, 1, 0, 1, 0), '𜵅');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 0, 1, 0, 1, 0), '𜵆');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 0, 1, 0, 1, 0), '𜵇');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 0, 1, 0, 1, 0), '𜵈');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 0, 1, 0, 1, 0), '▌');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 0, 1, 0, 1, 0), '𜵉');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 0, 1, 0, 1, 0), '𜵊');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 1, 1, 0, 1, 0), '𜵋');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 1, 1, 0, 1, 0), '𜵌');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 1, 1, 0, 1, 0), '▞');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 1, 1, 0, 1, 0), '𜵍');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 1, 1, 0, 1, 0), '𜵎');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 1, 1, 0, 1, 0), '𜵏');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 1, 1, 0, 1, 0), '𜵐');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 1, 1, 0, 1, 0), '▛');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 0, 0, 1, 1, 0), '𜵑');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 0, 0, 1, 1, 0), '𜵒');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 0, 0, 1, 1, 0), '𜵓');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 0, 0, 1, 1, 0), '𜵔');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 0, 0, 1, 1, 0), '𜵕');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 0, 0, 1, 1, 0), '𜵖');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 0, 0, 1, 1, 0), '𜵗');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 0, 0, 1, 1, 0), '𜵘');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 1, 0, 1, 1, 0), '𜵙');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 1, 0, 1, 1, 0), '𜵚');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 1, 0, 1, 1, 0), '𜵛');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 1, 0, 1, 1, 0), '𜵜');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 1, 0, 1, 1, 0), '𜵝');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 1, 0, 1, 1, 0), '𜵞');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 1, 0, 1, 1, 0), '𜵟');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 1, 0, 1, 1, 0), '𜵠');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 0, 1, 1, 1, 0), '𜵡');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 0, 1, 1, 1, 0), '𜵢');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 0, 1, 1, 1, 0), '𜵣');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 0, 1, 1, 1, 0), '𜵤');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 0, 1, 1, 1, 0), '𜵥');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 0, 1, 1, 1, 0), '𜵦');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 0, 1, 1, 1, 0), '𜵧');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 0, 1, 1, 1, 0), '𜵨');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 1, 1, 1, 1, 0), '𜵩');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 1, 1, 1, 1, 0), '𜵪');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 1, 1, 1, 1, 0), '𜵫');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 1, 1, 1, 1, 0), '𜵬');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 1, 1, 1, 1, 0), '𜵭');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 1, 1, 1, 1, 0), '𜵮');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 1, 1, 1, 1, 0), '𜵯');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 1, 1, 1, 1, 0), '𜵰');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 0, 0, 0, 0, 1), '𜺠');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 0, 0, 0, 0, 1), '𜵱');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 0, 0, 0, 0, 1), '𜵲');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 0, 0, 0, 0, 1), '𜵳');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 0, 0, 0, 0, 1), '𜵴');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 0, 0, 0, 0, 1), '𜵵');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 0, 0, 0, 0, 1), '𜵶');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 0, 0, 0, 0, 1), '𜵷');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 1, 0, 0, 0, 1), '𜵸');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 1, 0, 0, 0, 1), '𜵹');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 1, 0, 0, 0, 1), '𜵺');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 1, 0, 0, 0, 1), '𜵻');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 1, 0, 0, 0, 1), '𜵼');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 1, 0, 0, 0, 1), '𜵽');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 1, 0, 0, 0, 1), '𜵾');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 1, 0, 0, 0, 1), '𜵿');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 0, 1, 0, 0, 1), '𜶀');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 0, 1, 0, 0, 1), '𜶁');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 0, 1, 0, 0, 1), '𜶂');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 0, 1, 0, 0, 1), '𜶃');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 0, 1, 0, 0, 1), '𜶄');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 0, 1, 0, 0, 1), '𜶅');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 0, 1, 0, 0, 1), '𜶆');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 0, 1, 0, 0, 1), '𜶇');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 1, 1, 0, 0, 1), '𜶈');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 1, 1, 0, 0, 1), '𜶉');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 1, 1, 0, 0, 1), '𜶊');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 1, 1, 0, 0, 1), '𜶋');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 1, 1, 0, 0, 1), '𜶌');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 1, 1, 0, 0, 1), '𜶍');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 1, 1, 0, 0, 1), '𜶎');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 1, 1, 0, 0, 1), '𜶏');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 0, 0, 1, 0, 1), '▗');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 0, 0, 1, 0, 1), '𜶐');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 0, 0, 1, 0, 1), '𜶑');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 0, 0, 1, 0, 1), '𜶒');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 0, 0, 1, 0, 1), '𜶓');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 0, 0, 1, 0, 1), '▚');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 0, 0, 1, 0, 1), '𜶔');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 0, 0, 1, 0, 1), '𜶕');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 1, 0, 1, 0, 1), '𜶖');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 1, 0, 1, 0, 1), '𜶗');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 1, 0, 1, 0, 1), '▐');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 1, 0, 1, 0, 1), '𜶘');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 1, 0, 1, 0, 1), '𜶙');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 1, 0, 1, 0, 1), '𜶚');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 1, 0, 1, 0, 1), '𜶛');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 1, 0, 1, 0, 1), '▜');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 0, 1, 1, 0, 1), '𜶜');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 0, 1, 1, 0, 1), '𜶝');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 0, 1, 1, 0, 1), '𜶞');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 0, 1, 1, 0, 1), '𜶟');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 0, 1, 1, 0, 1), '𜶠');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 0, 1, 1, 0, 1), '𜶡');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 0, 1, 1, 0, 1), '𜶢');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 0, 1, 1, 0, 1), '𜶣');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 1, 1, 1, 0, 1), '𜶤');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 1, 1, 1, 0, 1), '𜶥');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 1, 1, 1, 0, 1), '𜶦');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 1, 1, 1, 0, 1), '𜶧');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 1, 1, 1, 0, 1), '𜶨');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 1, 1, 1, 0, 1), '𜶩');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 1, 1, 1, 0, 1), '𜶪');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 1, 1, 1, 0, 1), '𜶫');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 0, 0, 0, 1, 1), '▂');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 0, 0, 0, 1, 1), '𜶬');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 0, 0, 0, 1, 1), '𜶭');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 0, 0, 0, 1, 1), '𜶮');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 0, 0, 0, 1, 1), '𜶯');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 0, 0, 0, 1, 1), '𜶰');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 0, 0, 0, 1, 1), '𜶱');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 0, 0, 0, 1, 1), '𜶲');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 1, 0, 0, 1, 1), '𜶳');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 1, 0, 0, 1, 1), '𜶴');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 1, 0, 0, 1, 1), '𜶵');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 1, 0, 0, 1, 1), '𜶶');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 1, 0, 0, 1, 1), '𜶷');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 1, 0, 0, 1, 1), '𜶸');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 1, 0, 0, 1, 1), '𜶹');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 1, 0, 0, 1, 1), '𜶺');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 0, 1, 0, 1, 1), '𜶻');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 0, 1, 0, 1, 1), '𜶼');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 0, 1, 0, 1, 1), '𜶽');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 0, 1, 0, 1, 1), '𜶾');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 0, 1, 0, 1, 1), '𜶿');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 0, 1, 0, 1, 1), '𜷀');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 0, 1, 0, 1, 1), '𜷁');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 0, 1, 0, 1, 1), '𜷂');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 1, 1, 0, 1, 1), '𜷃');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 1, 1, 0, 1, 1), '𜷄');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 1, 1, 0, 1, 1), '𜷅');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 1, 1, 0, 1, 1), '𜷆');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 1, 1, 0, 1, 1), '𜷇');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 1, 1, 0, 1, 1), '𜷈');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 1, 1, 0, 1, 1), '𜷉');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 1, 1, 0, 1, 1), '𜷊');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 0, 0, 1, 1, 1), '𜷋');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 0, 0, 1, 1, 1), '𜷌');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 0, 0, 1, 1, 1), '𜷍');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 0, 0, 1, 1, 1), '𜷎');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 0, 0, 1, 1, 1), '𜷏');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 0, 0, 1, 1, 1), '𜷐');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 0, 0, 1, 1, 1), '𜷑');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 0, 0, 1, 1, 1), '𜷒');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 1, 0, 1, 1, 1), '𜷓');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 1, 0, 1, 1, 1), '𜷔');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 1, 0, 1, 1, 1), '𜷕');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 1, 0, 1, 1, 1), '𜷖');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 1, 0, 1, 1, 1), '𜷗');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 1, 0, 1, 1, 1), '𜷘');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 1, 0, 1, 1, 1), '𜷙');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 1, 0, 1, 1, 1), '𜷚');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 0, 1, 1, 1, 1), '▄');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 0, 1, 1, 1, 1), '𜷛');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 0, 1, 1, 1, 1), '𜷜');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 0, 1, 1, 1, 1), '𜷝');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 0, 1, 1, 1, 1), '𜷞');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 0, 1, 1, 1, 1), '▙');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 0, 1, 1, 1, 1), '𜷟');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 0, 1, 1, 1, 1), '𜷠');
        assert_eq!(get_symbol_octant_size(0, 0, 0, 1, 1, 1, 1, 1), '𜷡');
        assert_eq!(get_symbol_octant_size(1, 0, 0, 1, 1, 1, 1, 1), '𜷢');
        assert_eq!(get_symbol_octant_size(0, 1, 0, 1, 1, 1, 1, 1), '▟');
        assert_eq!(get_symbol_octant_size(1, 1, 0, 1, 1, 1, 1, 1), '𜷣');
        assert_eq!(get_symbol_octant_size(0, 0, 1, 1, 1, 1, 1, 1), '▆');
        assert_eq!(get_symbol_octant_size(1, 0, 1, 1, 1, 1, 1, 1), '𜷤');
        assert_eq!(get_symbol_octant_size(0, 1, 1, 1, 1, 1, 1, 1), '𜷥');
        assert_eq!(get_symbol_octant_size(1, 1, 1, 1, 1, 1, 1, 1), '█');
        Ok(())
    }

    #[test]
    fn check_half_width_symbols() -> Result<()> {
        assert_eq!(get_symbol_half_width(0, 0), ' ');
        assert_eq!(get_symbol_half_width(1, 0), '▌');
        assert_eq!(get_symbol_half_width(0, 1), '▐');
        assert_eq!(get_symbol_half_width(1, 1), '█');
        Ok(())
    }

    #[test]
    fn check_half_height_symbols() -> Result<()> {
        assert_eq!(get_symbol_half_height(0, 0), ' ');
        assert_eq!(get_symbol_half_height(1, 0), '▀');
        assert_eq!(get_symbol_half_height(0, 1), '▄');
        assert_eq!(get_symbol_half_height(1, 1), '█');
        Ok(())
    }

    #[test]
    fn check_third_height_symbols() -> Result<()> {
        assert_eq!(get_symbol_third_height(0, 0, 0), ' ');
        assert_eq!(get_symbol_third_height(1, 0, 0), '🬂');
        assert_eq!(get_symbol_third_height(0, 1, 0), '🬋');
        assert_eq!(get_symbol_third_height(1, 1, 0), '🬎');
        assert_eq!(get_symbol_third_height(0, 0, 1), '🬭');
        assert_eq!(get_symbol_third_height(1, 0, 1), '🬰');
        assert_eq!(get_symbol_third_height(0, 1, 1), '🬹');
        assert_eq!(get_symbol_third_height(1, 1, 1), '█');
        Ok(())
    }

    #[test]
    fn check_quarter_height_symbols() -> Result<()> {
        assert_eq!(get_symbol_quarter_height(0, 0, 0, 0), ' ');
        assert_eq!(get_symbol_quarter_height(1, 0, 0, 0), '🮂');
        assert_eq!(get_symbol_quarter_height(0, 1, 0, 0), '𜴆');
        assert_eq!(get_symbol_quarter_height(1, 1, 0, 0), '▀');
        assert_eq!(get_symbol_quarter_height(0, 0, 1, 0), '𜴧');
        assert_eq!(get_symbol_quarter_height(1, 0, 1, 0), '𜴪');
        assert_eq!(get_symbol_quarter_height(0, 1, 1, 0), '𜴳');
        assert_eq!(get_symbol_quarter_height(1, 1, 1, 0), '🮅');
        assert_eq!(get_symbol_quarter_height(0, 0, 0, 1), '▂');
        assert_eq!(get_symbol_quarter_height(1, 0, 0, 1), '𜶮');
        assert_eq!(get_symbol_quarter_height(0, 1, 0, 1), '𜶷');
        assert_eq!(get_symbol_quarter_height(1, 1, 0, 1), '𜶺');
        assert_eq!(get_symbol_quarter_height(0, 0, 1, 1), '▄');
        assert_eq!(get_symbol_quarter_height(1, 0, 1, 1), '𜷝');
        assert_eq!(get_symbol_quarter_height(0, 1, 1, 1), '▆');
        assert_eq!(get_symbol_quarter_height(1, 1, 1, 1), '█');
        Ok(())
    }

    #[test]
    fn check_get_symbol_for_position_in_glyph_third_height_defensive_middle() -> Result<()> {
        // In this test, we set all pixels of the glyph to 1 (all bytes are u8-max)
        // We expect that pixels out of the glyph-bounds are not set
        // Returned character is upper third filled only

        let glyph = [0xFFu8; 8];
        assert_eq!(
            PixelSize::ThirdHeight.symbol_for_position(&glyph, 7, 0),
            '🬂'
        );
        Ok(())
    }

    #[test]
    fn check_get_symbol_for_position_in_glyph_sextant_size_defensive_middle() -> Result<()> {
        // In this test, we set all pixels of the glyph to 1 (all bytes are u8-max)
        // We expect that pixels out of the glyph-bounds are not set
        // Returned character is upper third filled only

        let glyph = [0xFFu8; 8];
        assert_eq!(PixelSize::Sextant.symbol_for_position(&glyph, 7, 0), '🬂');
        Ok(())
    }
}
