pub mod error;
pub mod icon2svg;
pub mod iconid;
pub mod ligature;
pub mod pathstyle;
mod pens;

/// Setup to match fontations/font-test-data because that rig works for google3
#[cfg(test)]
mod testdata {
    pub static LAN_SVG: &str = include_str!("../resources/testdata/lan.svg");
    pub static MAN_SVG: &str = include_str!("../resources/testdata/man.svg");
    pub static MAIL_SVG: &str = include_str!("../resources/testdata/mail.svg");
    pub static MAIL_OPSZ48_SVG: &str = include_str!("../resources/testdata/mail_opsz48.svg");
    pub static MOSTLY_OFF_CURVE_SVG: &str =
        include_str!("../resources/testdata/mostly_off_curve.svg");

    pub static INFO_UNCHANGED_SVG: &str = include_str!("../resources/testdata/info_unchanged.svg");
    pub static INFO_COMPACT_SVG: &str = include_str!("../resources/testdata/info_compact.svg");

    pub static ICON_FONT: &[u8] =
        include_bytes!("../resources/testdata/vf[FILL,GRAD,opsz,wght].ttf");
    pub static MOSTLY_OFF_CURVE_FONT: &[u8] =
        include_bytes!("../resources/testdata/mostly_off_curve.ttf");
    pub static MATERIAL_SYMBOLS_POPULAR: &[u8] =
        include_bytes!("../resources/testdata/MaterialSymbolsOutlinedVF-Popular.ttf");
    pub static LIGA_TESTS_FONT: &[u8] = include_bytes!("../resources/testdata/liga_test.otf");
}
