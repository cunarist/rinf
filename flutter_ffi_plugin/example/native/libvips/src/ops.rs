// (c) Copyright 2019-2024 OLX
use crate::bindings;
use crate::error::*;
use crate::utils;
use crate::Result;
use crate::VipsBlob;
use crate::VipsImage;
use crate::VipsInterpolate;
use crate::VipsSource;
use crate::VipsTarget;
use std::convert::TryInto;
use std::ffi::*;
use std::ptr::null_mut;

const NULL: *const c_void = null_mut();

include!("manual.rs");

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Access {
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0
    Random = 0,
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    Sequential = 1,
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    SequentialUnbuffered = 2,
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    Last = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Align {
    ///  `Low` -> VIPS_ALIGN_LOW = 0
    Low = 0,
    ///  `Centre` -> VIPS_ALIGN_CENTRE = 1
    Centre = 1,
    ///  `High` -> VIPS_ALIGN_HIGH = 2
    High = 2,
    ///  `Last` -> VIPS_ALIGN_LAST = 3
    Last = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Angle {
    ///  `D0` -> VIPS_ANGLE_D0 = 0
    D0 = 0,
    ///  `D90` -> VIPS_ANGLE_D90 = 1
    D90 = 1,
    ///  `D180` -> VIPS_ANGLE_D180 = 2
    D180 = 2,
    ///  `D270` -> VIPS_ANGLE_D270 = 3
    D270 = 3,
    ///  `Last` -> VIPS_ANGLE_LAST = 4
    Last = 4,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Angle45 {
    ///  `D0` -> VIPS_ANGLE45_D0 = 0
    D0 = 0,
    ///  `D45` -> VIPS_ANGLE45_D45 = 1
    D45 = 1,
    ///  `D90` -> VIPS_ANGLE45_D90 = 2
    D90 = 2,
    ///  `D135` -> VIPS_ANGLE45_D135 = 3
    D135 = 3,
    ///  `D180` -> VIPS_ANGLE45_D180 = 4
    D180 = 4,
    ///  `D225` -> VIPS_ANGLE45_D225 = 5
    D225 = 5,
    ///  `D270` -> VIPS_ANGLE45_D270 = 6
    D270 = 6,
    ///  `D315` -> VIPS_ANGLE45_D315 = 7
    D315 = 7,
    ///  `Last` -> VIPS_ANGLE45_LAST = 8
    Last = 8,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum BandFormat {
    ///  `Notset` -> VIPS_FORMAT_NOTSET = -1
    Notset = -1,
    ///  `Uchar` -> VIPS_FORMAT_UCHAR = 0
    Uchar = 0,
    ///  `Char` -> VIPS_FORMAT_CHAR = 1
    Char = 1,
    ///  `Ushort` -> VIPS_FORMAT_USHORT = 2
    Ushort = 2,
    ///  `Short` -> VIPS_FORMAT_SHORT = 3
    Short = 3,
    ///  `Uint` -> VIPS_FORMAT_UINT = 4
    Uint = 4,
    ///  `Int` -> VIPS_FORMAT_INT = 5
    Int = 5,
    ///  `Float` -> VIPS_FORMAT_FLOAT = 6
    Float = 6,
    ///  `Complex` -> VIPS_FORMAT_COMPLEX = 7
    Complex = 7,
    ///  `Double` -> VIPS_FORMAT_DOUBLE = 8
    Double = 8,
    ///  `Dpcomplex` -> VIPS_FORMAT_DPCOMPLEX = 9
    Dpcomplex = 9,
    ///  `Last` -> VIPS_FORMAT_LAST = 10
    Last = 10,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum BlendMode {
    ///  `Clear` -> VIPS_BLEND_MODE_CLEAR = 0
    Clear = 0,
    ///  `Source` -> VIPS_BLEND_MODE_SOURCE = 1
    Source = 1,
    ///  `Over` -> VIPS_BLEND_MODE_OVER = 2
    Over = 2,
    ///  `In` -> VIPS_BLEND_MODE_IN = 3
    In = 3,
    ///  `Out` -> VIPS_BLEND_MODE_OUT = 4
    Out = 4,
    ///  `Atop` -> VIPS_BLEND_MODE_ATOP = 5
    Atop = 5,
    ///  `Dest` -> VIPS_BLEND_MODE_DEST = 6
    Dest = 6,
    ///  `DestOver` -> VIPS_BLEND_MODE_DEST_OVER = 7
    DestOver = 7,
    ///  `DestIn` -> VIPS_BLEND_MODE_DEST_IN = 8
    DestIn = 8,
    ///  `DestOut` -> VIPS_BLEND_MODE_DEST_OUT = 9
    DestOut = 9,
    ///  `DestAtop` -> VIPS_BLEND_MODE_DEST_ATOP = 10
    DestAtop = 10,
    ///  `Xor` -> VIPS_BLEND_MODE_XOR = 11
    Xor = 11,
    ///  `Add` -> VIPS_BLEND_MODE_ADD = 12
    Add = 12,
    ///  `Saturate` -> VIPS_BLEND_MODE_SATURATE = 13
    Saturate = 13,
    ///  `Multiply` -> VIPS_BLEND_MODE_MULTIPLY = 14
    Multiply = 14,
    ///  `Screen` -> VIPS_BLEND_MODE_SCREEN = 15
    Screen = 15,
    ///  `Overlay` -> VIPS_BLEND_MODE_OVERLAY = 16
    Overlay = 16,
    ///  `Darken` -> VIPS_BLEND_MODE_DARKEN = 17
    Darken = 17,
    ///  `Lighten` -> VIPS_BLEND_MODE_LIGHTEN = 18
    Lighten = 18,
    ///  `ColourDodge` -> VIPS_BLEND_MODE_COLOUR_DODGE = 19
    ColourDodge = 19,
    ///  `ColourBurn` -> VIPS_BLEND_MODE_COLOUR_BURN = 20
    ColourBurn = 20,
    ///  `HardLight` -> VIPS_BLEND_MODE_HARD_LIGHT = 21
    HardLight = 21,
    ///  `SoftLight` -> VIPS_BLEND_MODE_SOFT_LIGHT = 22
    SoftLight = 22,
    ///  `Difference` -> VIPS_BLEND_MODE_DIFFERENCE = 23
    Difference = 23,
    ///  `Exclusion` -> VIPS_BLEND_MODE_EXCLUSION = 24
    Exclusion = 24,
    ///  `Last` -> VIPS_BLEND_MODE_LAST = 25
    Last = 25,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Coding {
    ///  `Error` -> VIPS_CODING_ERROR = -1
    Error = -1,
    ///  `None` -> VIPS_CODING_NONE = 0
    None = 0,
    ///  `Labq` -> VIPS_CODING_LABQ = 2
    Labq = 2,
    ///  `Rad` -> VIPS_CODING_RAD = 6
    Rad = 6,
    ///  `Last` -> VIPS_CODING_LAST = 7
    Last = 7,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Combine {
    ///  `Max` -> VIPS_COMBINE_MAX = 0
    Max = 0,
    ///  `Sum` -> VIPS_COMBINE_SUM = 1
    Sum = 1,
    ///  `Min` -> VIPS_COMBINE_MIN = 2
    Min = 2,
    ///  `Last` -> VIPS_COMBINE_LAST = 3
    Last = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum CombineMode {
    ///  `Set` -> VIPS_COMBINE_MODE_SET = 0
    Set = 0,
    ///  `Add` -> VIPS_COMBINE_MODE_ADD = 1
    Add = 1,
    ///  `Last` -> VIPS_COMBINE_MODE_LAST = 2
    Last = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum CompassDirection {
    ///  `Centre` -> VIPS_COMPASS_DIRECTION_CENTRE = 0
    Centre = 0,
    ///  `North` -> VIPS_COMPASS_DIRECTION_NORTH = 1
    North = 1,
    ///  `East` -> VIPS_COMPASS_DIRECTION_EAST = 2
    East = 2,
    ///  `South` -> VIPS_COMPASS_DIRECTION_SOUTH = 3
    South = 3,
    ///  `West` -> VIPS_COMPASS_DIRECTION_WEST = 4
    West = 4,
    ///  `NorthEast` -> VIPS_COMPASS_DIRECTION_NORTH_EAST = 5
    NorthEast = 5,
    ///  `SouthEast` -> VIPS_COMPASS_DIRECTION_SOUTH_EAST = 6
    SouthEast = 6,
    ///  `SouthWest` -> VIPS_COMPASS_DIRECTION_SOUTH_WEST = 7
    SouthWest = 7,
    ///  `NorthWest` -> VIPS_COMPASS_DIRECTION_NORTH_WEST = 8
    NorthWest = 8,
    ///  `Last` -> VIPS_COMPASS_DIRECTION_LAST = 9
    Last = 9,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Direction {
    ///  `Horizontal` -> VIPS_DIRECTION_HORIZONTAL = 0
    Horizontal = 0,
    ///  `Vertical` -> VIPS_DIRECTION_VERTICAL = 1
    Vertical = 1,
    ///  `Last` -> VIPS_DIRECTION_LAST = 2
    Last = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Extend {
    ///  `Black` -> VIPS_EXTEND_BLACK = 0
    Black = 0,
    ///  `Copy` -> VIPS_EXTEND_COPY = 1
    Copy = 1,
    ///  `Repeat` -> VIPS_EXTEND_REPEAT = 2
    Repeat = 2,
    ///  `Mirror` -> VIPS_EXTEND_MIRROR = 3
    Mirror = 3,
    ///  `White` -> VIPS_EXTEND_WHITE = 4
    White = 4,
    ///  `Background` -> VIPS_EXTEND_BACKGROUND = 5
    Background = 5,
    ///  `Last` -> VIPS_EXTEND_LAST = 6
    Last = 6,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum FailOn {
    ///  `None` -> VIPS_FAIL_ON_NONE = 0
    None = 0,
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    Truncated = 1,
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    Error = 2,
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    Warning = 3,
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    Last = 4,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignDzDepth {
    ///  `Onepixel` -> VIPS_FOREIGN_DZ_DEPTH_ONEPIXEL = 0
    Onepixel = 0,
    ///  `Onetile` -> VIPS_FOREIGN_DZ_DEPTH_ONETILE = 1
    Onetile = 1,
    ///  `One` -> VIPS_FOREIGN_DZ_DEPTH_ONE = 2
    One = 2,
    ///  `Last` -> VIPS_FOREIGN_DZ_DEPTH_LAST = 3
    Last = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignFlags {
    ///  `None` -> VIPS_FOREIGN_NONE = 0
    None = 0,
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    Partial = 1,
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    Bigendian = 2,
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    Sequential = 4,
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    All = 7,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignHeifCompression {
    ///  `Hevc` -> VIPS_FOREIGN_HEIF_COMPRESSION_HEVC = 1
    Hevc = 1,
    ///  `Avc` -> VIPS_FOREIGN_HEIF_COMPRESSION_AVC = 2
    Avc = 2,
    ///  `Jpeg` -> VIPS_FOREIGN_HEIF_COMPRESSION_JPEG = 3
    Jpeg = 3,
    ///  `Av1` -> VIPS_FOREIGN_HEIF_COMPRESSION_AV1 = 4
    Av1 = 4,
    ///  `Last` -> VIPS_FOREIGN_HEIF_COMPRESSION_LAST = 5
    Last = 5,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignHeifEncoder {
    ///  `Auto` -> VIPS_FOREIGN_HEIF_ENCODER_AUTO = 0
    Auto = 0,
    ///  `Aom` -> VIPS_FOREIGN_HEIF_ENCODER_AOM = 1
    Aom = 1,
    ///  `Rav1E` -> VIPS_FOREIGN_HEIF_ENCODER_RAV1E = 2
    Rav1E = 2,
    ///  `Svt` -> VIPS_FOREIGN_HEIF_ENCODER_SVT = 3
    Svt = 3,
    ///  `X265` -> VIPS_FOREIGN_HEIF_ENCODER_X265 = 4
    X265 = 4,
    ///  `Last` -> VIPS_FOREIGN_HEIF_ENCODER_LAST = 5
    Last = 5,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignKeep {
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    None = 0,
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    Exif = 1,
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    Xmp = 2,
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    Iptc = 4,
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    Icc = 8,
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    Other = 16,
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31
    All = 31,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignPngFilter {
    ///  `None` -> VIPS_FOREIGN_PNG_FILTER_NONE = 8
    None = 8,
    ///  `Sub` -> VIPS_FOREIGN_PNG_FILTER_SUB = 16
    Sub = 16,
    ///  `Up` -> VIPS_FOREIGN_PNG_FILTER_UP = 32
    Up = 32,
    ///  `Avg` -> VIPS_FOREIGN_PNG_FILTER_AVG = 64
    Avg = 64,
    ///  `Paeth` -> VIPS_FOREIGN_PNG_FILTER_PAETH = 128
    Paeth = 128,
    ///  `All` -> VIPS_FOREIGN_PNG_FILTER_ALL = 248
    All = 248,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignPpmFormat {
    ///  `Pbm` -> VIPS_FOREIGN_PPM_FORMAT_PBM = 0
    Pbm = 0,
    ///  `Pgm` -> VIPS_FOREIGN_PPM_FORMAT_PGM = 1
    Pgm = 1,
    ///  `Ppm` -> VIPS_FOREIGN_PPM_FORMAT_PPM = 2
    Ppm = 2,
    ///  `Pfm` -> VIPS_FOREIGN_PPM_FORMAT_PFM = 3
    Pfm = 3,
    ///  `Pnm` -> VIPS_FOREIGN_PPM_FORMAT_PNM = 4
    Pnm = 4,
    ///  `Last` -> VIPS_FOREIGN_PPM_FORMAT_LAST = 5
    Last = 5,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignSubsample {
    ///  `Auto` -> VIPS_FOREIGN_SUBSAMPLE_AUTO = 0
    Auto = 0,
    ///  `On` -> VIPS_FOREIGN_SUBSAMPLE_ON = 1
    On = 1,
    ///  `Off` -> VIPS_FOREIGN_SUBSAMPLE_OFF = 2
    Off = 2,
    ///  `Last` -> VIPS_FOREIGN_SUBSAMPLE_LAST = 3
    Last = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignTiffCompression {
    ///  `None` -> VIPS_FOREIGN_TIFF_COMPRESSION_NONE = 0
    None = 0,
    ///  `Jpeg` -> VIPS_FOREIGN_TIFF_COMPRESSION_JPEG = 1
    Jpeg = 1,
    ///  `Deflate` -> VIPS_FOREIGN_TIFF_COMPRESSION_DEFLATE = 2
    Deflate = 2,
    ///  `Packbit` -> VIPS_FOREIGN_TIFF_COMPRESSION_PACKBITS = 3
    Packbit = 3,
    ///  `Ccittfax4` -> VIPS_FOREIGN_TIFF_COMPRESSION_CCITTFAX4 = 4
    Ccittfax4 = 4,
    ///  `Lzw` -> VIPS_FOREIGN_TIFF_COMPRESSION_LZW = 5
    Lzw = 5,
    ///  `Webp` -> VIPS_FOREIGN_TIFF_COMPRESSION_WEBP = 6
    Webp = 6,
    ///  `Zstd` -> VIPS_FOREIGN_TIFF_COMPRESSION_ZSTD = 7
    Zstd = 7,
    ///  `Jp2K` -> VIPS_FOREIGN_TIFF_COMPRESSION_JP2K = 8
    Jp2K = 8,
    ///  `Last` -> VIPS_FOREIGN_TIFF_COMPRESSION_LAST = 9
    Last = 9,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignTiffPredictor {
    ///  `None` -> VIPS_FOREIGN_TIFF_PREDICTOR_NONE = 1
    None = 1,
    ///  `Horizontal` -> VIPS_FOREIGN_TIFF_PREDICTOR_HORIZONTAL = 2
    Horizontal = 2,
    ///  `Float` -> VIPS_FOREIGN_TIFF_PREDICTOR_FLOAT = 3
    Float = 3,
    ///  `Last` -> VIPS_FOREIGN_TIFF_PREDICTOR_LAST = 4
    Last = 4,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignTiffResunit {
    ///  `Cm` -> VIPS_FOREIGN_TIFF_RESUNIT_CM = 0
    Cm = 0,
    ///  `Inch` -> VIPS_FOREIGN_TIFF_RESUNIT_INCH = 1
    Inch = 1,
    ///  `Last` -> VIPS_FOREIGN_TIFF_RESUNIT_LAST = 2
    Last = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignWebpPreset {
    ///  `Default` -> VIPS_FOREIGN_WEBP_PRESET_DEFAULT = 0
    Default = 0,
    ///  `Picture` -> VIPS_FOREIGN_WEBP_PRESET_PICTURE = 1
    Picture = 1,
    ///  `Photo` -> VIPS_FOREIGN_WEBP_PRESET_PHOTO = 2
    Photo = 2,
    ///  `Drawing` -> VIPS_FOREIGN_WEBP_PRESET_DRAWING = 3
    Drawing = 3,
    ///  `Icon` -> VIPS_FOREIGN_WEBP_PRESET_ICON = 4
    Icon = 4,
    ///  `Text` -> VIPS_FOREIGN_WEBP_PRESET_TEXT = 5
    Text = 5,
    ///  `Last` -> VIPS_FOREIGN_WEBP_PRESET_LAST = 6
    Last = 6,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Intent {
    ///  `Perceptual` -> VIPS_INTENT_PERCEPTUAL = 0
    Perceptual = 0,
    ///  `Relative` -> VIPS_INTENT_RELATIVE = 1
    Relative = 1,
    ///  `Saturation` -> VIPS_INTENT_SATURATION = 2
    Saturation = 2,
    ///  `Absolute` -> VIPS_INTENT_ABSOLUTE = 3
    Absolute = 3,
    ///  `Last` -> VIPS_INTENT_LAST = 4
    Last = 4,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Interesting {
    ///  `None` -> VIPS_INTERESTING_NONE = 0
    None = 0,
    ///  `Centre` -> VIPS_INTERESTING_CENTRE = 1
    Centre = 1,
    ///  `Entropy` -> VIPS_INTERESTING_ENTROPY = 2
    Entropy = 2,
    ///  `Attention` -> VIPS_INTERESTING_ATTENTION = 3
    Attention = 3,
    ///  `Low` -> VIPS_INTERESTING_LOW = 4
    Low = 4,
    ///  `High` -> VIPS_INTERESTING_HIGH = 5
    High = 5,
    ///  `All` -> VIPS_INTERESTING_ALL = 6
    All = 6,
    ///  `Last` -> VIPS_INTERESTING_LAST = 7
    Last = 7,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Interpretation {
    ///  `Error` -> VIPS_INTERPRETATION_ERROR = -1
    Error = -1,
    ///  `Multiband` -> VIPS_INTERPRETATION_MULTIBAND = 0
    Multiband = 0,
    ///  `BW` -> VIPS_INTERPRETATION_B_W = 1
    BW = 1,
    ///  `Histogram` -> VIPS_INTERPRETATION_HISTOGRAM = 10
    Histogram = 10,
    ///  `Xyz` -> VIPS_INTERPRETATION_XYZ = 12
    Xyz = 12,
    ///  `Lab` -> VIPS_INTERPRETATION_LAB = 13
    Lab = 13,
    ///  `Cmyk` -> VIPS_INTERPRETATION_CMYK = 15
    Cmyk = 15,
    ///  `Labq` -> VIPS_INTERPRETATION_LABQ = 16
    Labq = 16,
    ///  `Rgb` -> VIPS_INTERPRETATION_RGB = 17
    Rgb = 17,
    ///  `Cmc` -> VIPS_INTERPRETATION_CMC = 18
    Cmc = 18,
    ///  `Lch` -> VIPS_INTERPRETATION_LCH = 19
    Lch = 19,
    ///  `Lab` -> VIPS_INTERPRETATION_LABS = 21
    Labs = 21,
    ///  `Srgb` -> VIPS_INTERPRETATION_sRGB = 22
    Srgb = 22,
    ///  `Yxy` -> VIPS_INTERPRETATION_YXY = 23
    Yxy = 23,
    ///  `Fourier` -> VIPS_INTERPRETATION_FOURIER = 24
    Fourier = 24,
    ///  `Rgb16` -> VIPS_INTERPRETATION_RGB16 = 25
    Rgb16 = 25,
    ///  `Grey16` -> VIPS_INTERPRETATION_GREY16 = 26
    Grey16 = 26,
    ///  `Matrix` -> VIPS_INTERPRETATION_MATRIX = 27
    Matrix = 27,
    ///  `Scrgb` -> VIPS_INTERPRETATION_scRGB = 28
    Scrgb = 28,
    ///  `Hsv` -> VIPS_INTERPRETATION_HSV = 29
    Hsv = 29,
    ///  `Last` -> VIPS_INTERPRETATION_LAST = 30
    Last = 30,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Kernel {
    ///  `Nearest` -> VIPS_KERNEL_NEAREST = 0
    Nearest = 0,
    ///  `Linear` -> VIPS_KERNEL_LINEAR = 1
    Linear = 1,
    ///  `Cubic` -> VIPS_KERNEL_CUBIC = 2
    Cubic = 2,
    ///  `Mitchell` -> VIPS_KERNEL_MITCHELL = 3
    Mitchell = 3,
    ///  `Lanczos2` -> VIPS_KERNEL_LANCZOS2 = 4
    Lanczos2 = 4,
    ///  `Lanczos3` -> VIPS_KERNEL_LANCZOS3 = 5
    Lanczos3 = 5,
    ///  `Last` -> VIPS_KERNEL_LAST = 6
    Last = 6,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum OperationBoolean {
    ///  `And` -> VIPS_OPERATION_BOOLEAN_AND = 0
    And = 0,
    ///  `Or` -> VIPS_OPERATION_BOOLEAN_OR = 1
    Or = 1,
    ///  `Eor` -> VIPS_OPERATION_BOOLEAN_EOR = 2
    Eor = 2,
    ///  `Lshift` -> VIPS_OPERATION_BOOLEAN_LSHIFT = 3
    Lshift = 3,
    ///  `Rshift` -> VIPS_OPERATION_BOOLEAN_RSHIFT = 4
    Rshift = 4,
    ///  `Last` -> VIPS_OPERATION_BOOLEAN_LAST = 5
    Last = 5,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum OperationComplex {
    ///  `Polar` -> VIPS_OPERATION_COMPLEX_POLAR = 0
    Polar = 0,
    ///  `Rect` -> VIPS_OPERATION_COMPLEX_RECT = 1
    Rect = 1,
    ///  `Conj` -> VIPS_OPERATION_COMPLEX_CONJ = 2
    Conj = 2,
    ///  `Last` -> VIPS_OPERATION_COMPLEX_LAST = 3
    Last = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum OperationComplex2 {
    ///  `CrossPhase` -> VIPS_OPERATION_COMPLEX2_CROSS_PHASE = 0
    CrossPhase = 0,
    ///  `Last` -> VIPS_OPERATION_COMPLEX2_LAST = 1
    Last = 1,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum OperationComplexget {
    ///  `Real` -> VIPS_OPERATION_COMPLEXGET_REAL = 0
    Real = 0,
    ///  `Imag` -> VIPS_OPERATION_COMPLEXGET_IMAG = 1
    Imag = 1,
    ///  `Last` -> VIPS_OPERATION_COMPLEXGET_LAST = 2
    Last = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum OperationMath {
    ///  `Sin` -> VIPS_OPERATION_MATH_SIN = 0
    Sin = 0,
    ///  `Co` -> VIPS_OPERATION_MATH_COS = 1
    Co = 1,
    ///  `Tan` -> VIPS_OPERATION_MATH_TAN = 2
    Tan = 2,
    ///  `Asin` -> VIPS_OPERATION_MATH_ASIN = 3
    Asin = 3,
    ///  `Aco` -> VIPS_OPERATION_MATH_ACOS = 4
    Aco = 4,
    ///  `Atan` -> VIPS_OPERATION_MATH_ATAN = 5
    Atan = 5,
    ///  `Log` -> VIPS_OPERATION_MATH_LOG = 6
    Log = 6,
    ///  `Log10` -> VIPS_OPERATION_MATH_LOG10 = 7
    Log10 = 7,
    ///  `Exp` -> VIPS_OPERATION_MATH_EXP = 8
    Exp = 8,
    ///  `Exp10` -> VIPS_OPERATION_MATH_EXP10 = 9
    Exp10 = 9,
    ///  `Sinh` -> VIPS_OPERATION_MATH_SINH = 10
    Sinh = 10,
    ///  `Cosh` -> VIPS_OPERATION_MATH_COSH = 11
    Cosh = 11,
    ///  `Tanh` -> VIPS_OPERATION_MATH_TANH = 12
    Tanh = 12,
    ///  `Asinh` -> VIPS_OPERATION_MATH_ASINH = 13
    Asinh = 13,
    ///  `Acosh` -> VIPS_OPERATION_MATH_ACOSH = 14
    Acosh = 14,
    ///  `Atanh` -> VIPS_OPERATION_MATH_ATANH = 15
    Atanh = 15,
    ///  `Last` -> VIPS_OPERATION_MATH_LAST = 16
    Last = 16,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum OperationMath2 {
    ///  `Pow` -> VIPS_OPERATION_MATH2_POW = 0
    Pow = 0,
    ///  `Wop` -> VIPS_OPERATION_MATH2_WOP = 1
    Wop = 1,
    ///  `Atan2` -> VIPS_OPERATION_MATH2_ATAN2 = 2
    Atan2 = 2,
    ///  `Last` -> VIPS_OPERATION_MATH2_LAST = 3
    Last = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum OperationMorphology {
    ///  `Erode` -> VIPS_OPERATION_MORPHOLOGY_ERODE = 0
    Erode = 0,
    ///  `Dilate` -> VIPS_OPERATION_MORPHOLOGY_DILATE = 1
    Dilate = 1,
    ///  `Last` -> VIPS_OPERATION_MORPHOLOGY_LAST = 2
    Last = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum OperationRelational {
    ///  `Equal` -> VIPS_OPERATION_RELATIONAL_EQUAL = 0
    Equal = 0,
    ///  `Noteq` -> VIPS_OPERATION_RELATIONAL_NOTEQ = 1
    Noteq = 1,
    ///  `Less` -> VIPS_OPERATION_RELATIONAL_LESS = 2
    Less = 2,
    ///  `Lesseq` -> VIPS_OPERATION_RELATIONAL_LESSEQ = 3
    Lesseq = 3,
    ///  `More` -> VIPS_OPERATION_RELATIONAL_MORE = 4
    More = 4,
    ///  `Moreeq` -> VIPS_OPERATION_RELATIONAL_MOREEQ = 5
    Moreeq = 5,
    ///  `Last` -> VIPS_OPERATION_RELATIONAL_LAST = 6
    Last = 6,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum OperationRound {
    ///  `Rint` -> VIPS_OPERATION_ROUND_RINT = 0
    Rint = 0,
    ///  `Ceil` -> VIPS_OPERATION_ROUND_CEIL = 1
    Ceil = 1,
    ///  `Floor` -> VIPS_OPERATION_ROUND_FLOOR = 2
    Floor = 2,
    ///  `Last` -> VIPS_OPERATION_ROUND_LAST = 3
    Last = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum PCS {
    ///  `Lab` -> VIPS_PCS_LAB = 0
    Lab = 0,
    ///  `Xyz` -> VIPS_PCS_XYZ = 1
    Xyz = 1,
    ///  `Last` -> VIPS_PCS_LAST = 2
    Last = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Precision {
    ///  `Integer` -> VIPS_PRECISION_INTEGER = 0
    Integer = 0,
    ///  `Float` -> VIPS_PRECISION_FLOAT = 1
    Float = 1,
    ///  `Approximate` -> VIPS_PRECISION_APPROXIMATE = 2
    Approximate = 2,
    ///  `Last` -> VIPS_PRECISION_LAST = 3
    Last = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum RegionShrink {
    ///  `Mean` -> VIPS_REGION_SHRINK_MEAN = 0
    Mean = 0,
    ///  `Median` -> VIPS_REGION_SHRINK_MEDIAN = 1
    Median = 1,
    ///  `Mode` -> VIPS_REGION_SHRINK_MODE = 2
    Mode = 2,
    ///  `Max` -> VIPS_REGION_SHRINK_MAX = 3
    Max = 3,
    ///  `Min` -> VIPS_REGION_SHRINK_MIN = 4
    Min = 4,
    ///  `Nearest` -> VIPS_REGION_SHRINK_NEAREST = 5
    Nearest = 5,
    ///  `Last` -> VIPS_REGION_SHRINK_LAST = 6
    Last = 6,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Size {
    ///  `Both` -> VIPS_SIZE_BOTH = 0
    Both = 0,
    ///  `Up` -> VIPS_SIZE_UP = 1
    Up = 1,
    ///  `Down` -> VIPS_SIZE_DOWN = 2
    Down = 2,
    ///  `Force` -> VIPS_SIZE_FORCE = 3
    Force = 3,
    ///  `Last` -> VIPS_SIZE_LAST = 4
    Last = 4,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum TextWrap {
    ///  `Word` -> VIPS_TEXT_WRAP_WORD = 0
    Word = 0,
    ///  `Char` -> VIPS_TEXT_WRAP_CHAR = 1
    Char = 1,
    ///  `WordChar` -> VIPS_TEXT_WRAP_WORD_CHAR = 2
    WordChar = 2,
    ///  `None` -> VIPS_TEXT_WRAP_NONE = 3
    None = 3,
    ///  `Last` -> VIPS_TEXT_WRAP_LAST = 4
    Last = 4,
}

/// VipsSystem (system), run an external command
/// cmd_format: `&str` -> Command to run

pub fn system(cmd_format: &str) -> Result<()> {
    unsafe {
        let cmd_format_in: CString = utils::new_c_string(cmd_format)?;

        let vips_op_response = bindings::vips_system(
            cmd_format_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::SystemError,
        )
    }
}

/// Options for system operation
#[derive(Clone, Debug)]
pub struct SystemOptions {
    /// inp: `Vec<VipsImage>` -> Array of input images
    pub inp: Vec<VipsImage>,
    /// out: `VipsImage` -> Output image
    pub out: VipsImage,
    /// log: `String` -> Command log
    pub log: String,
    /// out_format: `String` -> Format for output filename
    pub out_format: String,
    /// in_format: `String` -> Format for input filename
    pub in_format: String,
}

impl std::default::Default for SystemOptions {
    fn default() -> Self {
        SystemOptions {
            inp: Vec::new(),
            out: VipsImage::new(),
            log: String::new(),
            out_format: String::new(),
            in_format: String::new(),
        }
    }
}

/// VipsSystem (system), run an external command
/// cmd_format: `&str` -> Command to run
/// system_options: `&SystemOptions` -> optional arguments

pub fn system_with_opts(cmd_format: &str, system_options: &SystemOptions) -> Result<()> {
    unsafe {
        let cmd_format_in: CString = utils::new_c_string(cmd_format)?;

        let inp_wrapper = utils::VipsArrayImageWrapper::from(&system_options.inp[..]);
        let inp_in = inp_wrapper.ctx;
        let inp_in_name = utils::new_c_string("inp")?;

        let out_in: *mut bindings::VipsImage = system_options
            .out
            .ctx;
        let out_in_name = utils::new_c_string("out")?;

        let log_in: CString = utils::new_c_string(&system_options.log)?;
        let log_in_name = utils::new_c_string("log")?;

        let out_format_in: CString = utils::new_c_string(&system_options.out_format)?;
        let out_format_in_name = utils::new_c_string("out-format")?;

        let in_format_in: CString = utils::new_c_string(&system_options.in_format)?;
        let in_format_in_name = utils::new_c_string("in-format")?;

        let vips_op_response = bindings::vips_system(
            cmd_format_in.as_ptr(),
            inp_in_name.as_ptr(),
            inp_in,
            out_in_name.as_ptr(),
            out_in,
            log_in_name.as_ptr(),
            log_in.as_ptr(),
            out_format_in_name.as_ptr(),
            out_format_in.as_ptr(),
            in_format_in_name.as_ptr(),
            in_format_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::SystemError,
        )
    }
}

/// VipsAdd (add), add two images
/// left: `&VipsImage` -> Left-hand image argument
/// right: `&VipsImage` -> Right-hand image argument
/// returns `VipsImage` - Output image
pub fn add(left: &VipsImage, right: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let left_in: *mut bindings::VipsImage = left.ctx;
        let right_in: *mut bindings::VipsImage = right.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_add(
            left_in,
            right_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::AddError,
        )
    }
}

/// VipsSubtract (subtract), subtract two images
/// left: `&VipsImage` -> Left-hand image argument
/// right: `&VipsImage` -> Right-hand image argument
/// returns `VipsImage` - Output image
pub fn subtract(left: &VipsImage, right: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let left_in: *mut bindings::VipsImage = left.ctx;
        let right_in: *mut bindings::VipsImage = right.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_subtract(
            left_in,
            right_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SubtractError,
        )
    }
}

/// VipsMultiply (multiply), multiply two images
/// left: `&VipsImage` -> Left-hand image argument
/// right: `&VipsImage` -> Right-hand image argument
/// returns `VipsImage` - Output image
pub fn multiply(left: &VipsImage, right: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let left_in: *mut bindings::VipsImage = left.ctx;
        let right_in: *mut bindings::VipsImage = right.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_multiply(
            left_in,
            right_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MultiplyError,
        )
    }
}

/// VipsDivide (divide), divide two images
/// left: `&VipsImage` -> Left-hand image argument
/// right: `&VipsImage` -> Right-hand image argument
/// returns `VipsImage` - Output image
pub fn divide(left: &VipsImage, right: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let left_in: *mut bindings::VipsImage = left.ctx;
        let right_in: *mut bindings::VipsImage = right.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_divide(
            left_in,
            right_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::DivideError,
        )
    }
}

/// VipsRelational (relational), relational operation on two images
/// left: `&VipsImage` -> Left-hand image argument
/// right: `&VipsImage` -> Right-hand image argument
/// relational: `OperationRelational` -> Relational to perform
///  `Equal` -> VIPS_OPERATION_RELATIONAL_EQUAL = 0 [DEFAULT]
///  `Noteq` -> VIPS_OPERATION_RELATIONAL_NOTEQ = 1
///  `Less` -> VIPS_OPERATION_RELATIONAL_LESS = 2
///  `Lesseq` -> VIPS_OPERATION_RELATIONAL_LESSEQ = 3
///  `More` -> VIPS_OPERATION_RELATIONAL_MORE = 4
///  `Moreeq` -> VIPS_OPERATION_RELATIONAL_MOREEQ = 5
///  `Last` -> VIPS_OPERATION_RELATIONAL_LAST = 6
/// returns `VipsImage` - Output image
pub fn relational(
    left: &VipsImage,
    right: &VipsImage,
    relational: OperationRelational,
) -> Result<VipsImage> {
    unsafe {
        let left_in: *mut bindings::VipsImage = left.ctx;
        let right_in: *mut bindings::VipsImage = right.ctx;
        let relational_in: i32 = relational as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_relational(
            left_in,
            right_in,
            &mut out_out,
            relational_in
                .try_into()
                .unwrap(),
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RelationalError,
        )
    }
}

/// VipsRemainder (remainder), remainder after integer division of two images
/// left: `&VipsImage` -> Left-hand image argument
/// right: `&VipsImage` -> Right-hand image argument
/// returns `VipsImage` - Output image
pub fn remainder(left: &VipsImage, right: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let left_in: *mut bindings::VipsImage = left.ctx;
        let right_in: *mut bindings::VipsImage = right.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_remainder(
            left_in,
            right_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RemainderError,
        )
    }
}

/// VipsBoolean (boolean), boolean operation on two images
/// left: `&VipsImage` -> Left-hand image argument
/// right: `&VipsImage` -> Right-hand image argument
/// boolean: `OperationBoolean` -> Boolean to perform
///  `And` -> VIPS_OPERATION_BOOLEAN_AND = 0 [DEFAULT]
///  `Or` -> VIPS_OPERATION_BOOLEAN_OR = 1
///  `Eor` -> VIPS_OPERATION_BOOLEAN_EOR = 2
///  `Lshift` -> VIPS_OPERATION_BOOLEAN_LSHIFT = 3
///  `Rshift` -> VIPS_OPERATION_BOOLEAN_RSHIFT = 4
///  `Last` -> VIPS_OPERATION_BOOLEAN_LAST = 5
/// returns `VipsImage` - Output image
pub fn boolean(
    left: &VipsImage,
    right: &VipsImage,
    boolean: OperationBoolean,
) -> Result<VipsImage> {
    unsafe {
        let left_in: *mut bindings::VipsImage = left.ctx;
        let right_in: *mut bindings::VipsImage = right.ctx;
        let boolean_in: i32 = boolean as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_boolean(
            left_in,
            right_in,
            &mut out_out,
            boolean_in
                .try_into()
                .unwrap(),
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BooleanError,
        )
    }
}

/// VipsMath2 (math2), binary math operations
/// left: `&VipsImage` -> Left-hand image argument
/// right: `&VipsImage` -> Right-hand image argument
/// math_2: `OperationMath2` -> Math to perform
///  `Pow` -> VIPS_OPERATION_MATH2_POW = 0 [DEFAULT]
///  `Wop` -> VIPS_OPERATION_MATH2_WOP = 1
///  `Atan2` -> VIPS_OPERATION_MATH2_ATAN2 = 2
///  `Last` -> VIPS_OPERATION_MATH2_LAST = 3
/// returns `VipsImage` - Output image
pub fn math_2(left: &VipsImage, right: &VipsImage, math_2: OperationMath2) -> Result<VipsImage> {
    unsafe {
        let left_in: *mut bindings::VipsImage = left.ctx;
        let right_in: *mut bindings::VipsImage = right.ctx;
        let math_2_in: i32 = math_2 as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_math2(
            left_in,
            right_in,
            &mut out_out,
            math_2_in
                .try_into()
                .unwrap(),
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Math2Error,
        )
    }
}

/// VipsComplex2 (complex2), complex binary operations on two images
/// left: `&VipsImage` -> Left-hand image argument
/// right: `&VipsImage` -> Right-hand image argument
/// cmplx: `OperationComplex2` -> Binary complex operation to perform
///  `CrossPhase` -> VIPS_OPERATION_COMPLEX2_CROSS_PHASE = 0 [DEFAULT]
///  `Last` -> VIPS_OPERATION_COMPLEX2_LAST = 1
/// returns `VipsImage` - Output image
pub fn complex_2(
    left: &VipsImage,
    right: &VipsImage,
    cmplx: OperationComplex2,
) -> Result<VipsImage> {
    unsafe {
        let left_in: *mut bindings::VipsImage = left.ctx;
        let right_in: *mut bindings::VipsImage = right.ctx;
        let cmplx_in: i32 = cmplx as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_complex2(
            left_in,
            right_in,
            &mut out_out,
            cmplx_in
                .try_into()
                .unwrap(),
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Complex2Error,
        )
    }
}

/// VipsComplexform (complexform), form a complex image from two real images
/// left: `&VipsImage` -> Left-hand image argument
/// right: `&VipsImage` -> Right-hand image argument
/// returns `VipsImage` - Output image
pub fn complexform(left: &VipsImage, right: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let left_in: *mut bindings::VipsImage = left.ctx;
        let right_in: *mut bindings::VipsImage = right.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_complexform(
            left_in,
            right_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ComplexformError,
        )
    }
}

/// VipsSum (sum), sum an array of images
/// inp: `&mut [VipsImage]` -> Array of input images
/// returns `VipsImage` - Output image
pub fn sum(inp: &mut [VipsImage]) -> Result<VipsImage> {
    unsafe {
        let (inp_len, mut inp_in) = {
            let len = inp.len();
            let mut input = Vec::new();
            for img in inp {
                input.push(img.ctx)
            }
            (
                len as i32,
                input,
            )
        };
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_sum(
            inp_in.as_mut_ptr(),
            &mut out_out,
            inp_len,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SumError,
        )
    }
}

/// VipsInvert (invert), invert an image
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn invert(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_invert(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::InvertError,
        )
    }
}

/// VipsMath (math), apply a math operation to an image
/// inp: `&VipsImage` -> Input image
/// math: `OperationMath` -> Math to perform
///  `Sin` -> VIPS_OPERATION_MATH_SIN = 0 [DEFAULT]
///  `Co` -> VIPS_OPERATION_MATH_COS = 1
///  `Tan` -> VIPS_OPERATION_MATH_TAN = 2
///  `Asin` -> VIPS_OPERATION_MATH_ASIN = 3
///  `Aco` -> VIPS_OPERATION_MATH_ACOS = 4
///  `Atan` -> VIPS_OPERATION_MATH_ATAN = 5
///  `Log` -> VIPS_OPERATION_MATH_LOG = 6
///  `Log10` -> VIPS_OPERATION_MATH_LOG10 = 7
///  `Exp` -> VIPS_OPERATION_MATH_EXP = 8
///  `Exp10` -> VIPS_OPERATION_MATH_EXP10 = 9
///  `Sinh` -> VIPS_OPERATION_MATH_SINH = 10
///  `Cosh` -> VIPS_OPERATION_MATH_COSH = 11
///  `Tanh` -> VIPS_OPERATION_MATH_TANH = 12
///  `Asinh` -> VIPS_OPERATION_MATH_ASINH = 13
///  `Acosh` -> VIPS_OPERATION_MATH_ACOSH = 14
///  `Atanh` -> VIPS_OPERATION_MATH_ATANH = 15
///  `Last` -> VIPS_OPERATION_MATH_LAST = 16
/// returns `VipsImage` - Output image
pub fn math(inp: &VipsImage, math: OperationMath) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let math_in: i32 = math as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_math(
            inp_in,
            &mut out_out,
            math_in
                .try_into()
                .unwrap(),
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MathError,
        )
    }
}

/// VipsAbs (abs), absolute value of an image
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn abs(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_abs(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::AbError,
        )
    }
}

/// VipsSign (sign), unit vector of pixel
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn sign(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_sign(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SignError,
        )
    }
}

/// VipsRound (round), perform a round function on an image
/// inp: `&VipsImage` -> Input image
/// round: `OperationRound` -> Rounding operation to perform
///  `Rint` -> VIPS_OPERATION_ROUND_RINT = 0 [DEFAULT]
///  `Ceil` -> VIPS_OPERATION_ROUND_CEIL = 1
///  `Floor` -> VIPS_OPERATION_ROUND_FLOOR = 2
///  `Last` -> VIPS_OPERATION_ROUND_LAST = 3
/// returns `VipsImage` - Output image
pub fn round(inp: &VipsImage, round: OperationRound) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let round_in: i32 = round as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_round(
            inp_in,
            &mut out_out,
            round_in
                .try_into()
                .unwrap(),
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RoundError,
        )
    }
}

/// VipsRelationalConst (relational_const), relational operations against a constant
/// inp: `&VipsImage` -> Input image
/// relational: `OperationRelational` -> Relational to perform
///  `Equal` -> VIPS_OPERATION_RELATIONAL_EQUAL = 0 [DEFAULT]
///  `Noteq` -> VIPS_OPERATION_RELATIONAL_NOTEQ = 1
///  `Less` -> VIPS_OPERATION_RELATIONAL_LESS = 2
///  `Lesseq` -> VIPS_OPERATION_RELATIONAL_LESSEQ = 3
///  `More` -> VIPS_OPERATION_RELATIONAL_MORE = 4
///  `Moreeq` -> VIPS_OPERATION_RELATIONAL_MOREEQ = 5
///  `Last` -> VIPS_OPERATION_RELATIONAL_LAST = 6
/// c: `&mut [f64]` -> Array of constants
/// returns `VipsImage` - Output image
pub fn relational_const(
    inp: &VipsImage,
    relational: OperationRelational,
    c: &mut [f64],
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let relational_in: i32 = relational as i32;
        let c_in: *mut f64 = c.as_mut_ptr();
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_relational_const(
            inp_in,
            &mut out_out,
            relational_in
                .try_into()
                .unwrap(),
            c_in,
            c.len() as i32,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RelationalConstError,
        )
    }
}

/// VipsRemainderConst (remainder_const), remainder after integer division of an image and a constant
/// inp: `&VipsImage` -> Input image
/// c: `&mut [f64]` -> Array of constants
/// returns `VipsImage` - Output image
pub fn remainder_const(inp: &VipsImage, c: &mut [f64]) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let c_in: *mut f64 = c.as_mut_ptr();
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_remainder_const(
            inp_in,
            &mut out_out,
            c_in,
            c.len() as i32,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RemainderConstError,
        )
    }
}

/// VipsBooleanConst (boolean_const), boolean operations against a constant
/// inp: `&VipsImage` -> Input image
/// boolean: `OperationBoolean` -> Boolean to perform
///  `And` -> VIPS_OPERATION_BOOLEAN_AND = 0 [DEFAULT]
///  `Or` -> VIPS_OPERATION_BOOLEAN_OR = 1
///  `Eor` -> VIPS_OPERATION_BOOLEAN_EOR = 2
///  `Lshift` -> VIPS_OPERATION_BOOLEAN_LSHIFT = 3
///  `Rshift` -> VIPS_OPERATION_BOOLEAN_RSHIFT = 4
///  `Last` -> VIPS_OPERATION_BOOLEAN_LAST = 5
/// c: `&mut [f64]` -> Array of constants
/// returns `VipsImage` - Output image
pub fn boolean_const(
    inp: &VipsImage,
    boolean: OperationBoolean,
    c: &mut [f64],
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let boolean_in: i32 = boolean as i32;
        let c_in: *mut f64 = c.as_mut_ptr();
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_boolean_const(
            inp_in,
            &mut out_out,
            boolean_in
                .try_into()
                .unwrap(),
            c_in,
            c.len() as i32,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BooleanConstError,
        )
    }
}

/// VipsMath2Const (math2_const), binary math operations with a constant
/// inp: `&VipsImage` -> Input image
/// math_2: `OperationMath2` -> Math to perform
///  `Pow` -> VIPS_OPERATION_MATH2_POW = 0 [DEFAULT]
///  `Wop` -> VIPS_OPERATION_MATH2_WOP = 1
///  `Atan2` -> VIPS_OPERATION_MATH2_ATAN2 = 2
///  `Last` -> VIPS_OPERATION_MATH2_LAST = 3
/// c: `&mut [f64]` -> Array of constants
/// returns `VipsImage` - Output image
pub fn math_2_const(inp: &VipsImage, math_2: OperationMath2, c: &mut [f64]) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let math_2_in: i32 = math_2 as i32;
        let c_in: *mut f64 = c.as_mut_ptr();
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_math2_const(
            inp_in,
            &mut out_out,
            math_2_in
                .try_into()
                .unwrap(),
            c_in,
            c.len() as i32,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Math2ConstError,
        )
    }
}

/// VipsComplex (complex), perform a complex operation on an image
/// inp: `&VipsImage` -> Input image
/// cmplx: `OperationComplex` -> Complex to perform
///  `Polar` -> VIPS_OPERATION_COMPLEX_POLAR = 0 [DEFAULT]
///  `Rect` -> VIPS_OPERATION_COMPLEX_RECT = 1
///  `Conj` -> VIPS_OPERATION_COMPLEX_CONJ = 2
///  `Last` -> VIPS_OPERATION_COMPLEX_LAST = 3
/// returns `VipsImage` - Output image
pub fn complex(inp: &VipsImage, cmplx: OperationComplex) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let cmplx_in: i32 = cmplx as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_complex(
            inp_in,
            &mut out_out,
            cmplx_in
                .try_into()
                .unwrap(),
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ComplexError,
        )
    }
}

/// VipsComplexget (complexget), get a component from a complex image
/// inp: `&VipsImage` -> Input image
/// get: `OperationComplexget` -> Complex to perform
///  `Real` -> VIPS_OPERATION_COMPLEXGET_REAL = 0 [DEFAULT]
///  `Imag` -> VIPS_OPERATION_COMPLEXGET_IMAG = 1
///  `Last` -> VIPS_OPERATION_COMPLEXGET_LAST = 2
/// returns `VipsImage` - Output image
pub fn complexget(inp: &VipsImage, get: OperationComplexget) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let get_in: i32 = get as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_complexget(
            inp_in,
            &mut out_out,
            get_in
                .try_into()
                .unwrap(),
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ComplexgetError,
        )
    }
}

/// VipsAvg (avg), find image average
/// inp: `&VipsImage` -> Input image
/// returns `f64` - Output value
pub fn avg(inp: &VipsImage) -> Result<f64> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: f64 = f64::from(0);

        let vips_op_response = bindings::vips_avg(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::AvgError,
        )
    }
}

/// VipsMin (min), find image minimum
/// inp: `&VipsImage` -> Input image
/// returns `f64` - Output value
pub fn min(inp: &VipsImage) -> Result<f64> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: f64 = f64::from(0);

        let vips_op_response = bindings::vips_min(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::MinError,
        )
    }
}

/// Options for min operation
#[derive(Clone, Debug)]
pub struct MinOptions {
    /// x: `i32` -> Horizontal position of minimum
    /// min: 0, max: 10000000, default: 0
    pub x: i32,
    /// y: `i32` -> Vertical position of minimum
    /// min: 0, max: 10000000, default: 0
    pub y: i32,
    /// size: `i32` -> Number of minimum values to find
    /// min: 1, max: 1000000, default: 10
    pub size: i32,
    /// out_array: `Vec<f64>` -> Array of output values
    pub out_array: Vec<f64>,
    /// x_array: `Vec<i32>` -> Array of horizontal positions
    pub x_array: Vec<i32>,
    /// y_array: `Vec<i32>` -> Array of vertical positions
    pub y_array: Vec<i32>,
}

impl std::default::Default for MinOptions {
    fn default() -> Self {
        MinOptions {
            x: i32::from(0),
            y: i32::from(0),
            size: i32::from(10),
            out_array: Vec::new(),
            x_array: Vec::new(),
            y_array: Vec::new(),
        }
    }
}

/// VipsMin (min), find image minimum
/// inp: `&VipsImage` -> Input image
/// min_options: `&MinOptions` -> optional arguments
/// returns `f64` - Output value
pub fn min_with_opts(inp: &VipsImage, min_options: &MinOptions) -> Result<f64> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: f64 = f64::from(0);

        let x_in: i32 = min_options.x;
        let x_in_name = utils::new_c_string("x")?;

        let y_in: i32 = min_options.y;
        let y_in_name = utils::new_c_string("y")?;

        let size_in: i32 = min_options.size;
        let size_in_name = utils::new_c_string("size")?;

        let out_array_wrapper = utils::VipsArrayDoubleWrapper::from(&min_options.out_array[..]);
        let out_array_in = out_array_wrapper.ctx;
        let out_array_in_name = utils::new_c_string("out-array")?;

        let x_array_wrapper = utils::VipsArrayIntWrapper::from(&min_options.x_array[..]);
        let x_array_in = x_array_wrapper.ctx;
        let x_array_in_name = utils::new_c_string("x-array")?;

        let y_array_wrapper = utils::VipsArrayIntWrapper::from(&min_options.y_array[..]);
        let y_array_in = y_array_wrapper.ctx;
        let y_array_in_name = utils::new_c_string("y-array")?;

        let vips_op_response = bindings::vips_min(
            inp_in,
            &mut out_out,
            x_in_name.as_ptr(),
            x_in,
            y_in_name.as_ptr(),
            y_in,
            size_in_name.as_ptr(),
            size_in,
            out_array_in_name.as_ptr(),
            out_array_in,
            x_array_in_name.as_ptr(),
            x_array_in,
            y_array_in_name.as_ptr(),
            y_array_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::MinError,
        )
    }
}

/// VipsMax (max), find image maximum
/// inp: `&VipsImage` -> Input image
/// returns `f64` - Output value
pub fn max(inp: &VipsImage) -> Result<f64> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: f64 = f64::from(0);

        let vips_op_response = bindings::vips_max(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::MaxError,
        )
    }
}

/// Options for max operation
#[derive(Clone, Debug)]
pub struct MaxOptions {
    /// x: `i32` -> Horizontal position of maximum
    /// min: 0, max: 10000000, default: 0
    pub x: i32,
    /// y: `i32` -> Vertical position of maximum
    /// min: 0, max: 10000000, default: 0
    pub y: i32,
    /// size: `i32` -> Number of maximum values to find
    /// min: 1, max: 1000000, default: 10
    pub size: i32,
    /// out_array: `Vec<f64>` -> Array of output values
    pub out_array: Vec<f64>,
    /// x_array: `Vec<i32>` -> Array of horizontal positions
    pub x_array: Vec<i32>,
    /// y_array: `Vec<i32>` -> Array of vertical positions
    pub y_array: Vec<i32>,
}

impl std::default::Default for MaxOptions {
    fn default() -> Self {
        MaxOptions {
            x: i32::from(0),
            y: i32::from(0),
            size: i32::from(10),
            out_array: Vec::new(),
            x_array: Vec::new(),
            y_array: Vec::new(),
        }
    }
}

/// VipsMax (max), find image maximum
/// inp: `&VipsImage` -> Input image
/// max_options: `&MaxOptions` -> optional arguments
/// returns `f64` - Output value
pub fn max_with_opts(inp: &VipsImage, max_options: &MaxOptions) -> Result<f64> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: f64 = f64::from(0);

        let x_in: i32 = max_options.x;
        let x_in_name = utils::new_c_string("x")?;

        let y_in: i32 = max_options.y;
        let y_in_name = utils::new_c_string("y")?;

        let size_in: i32 = max_options.size;
        let size_in_name = utils::new_c_string("size")?;

        let out_array_wrapper = utils::VipsArrayDoubleWrapper::from(&max_options.out_array[..]);
        let out_array_in = out_array_wrapper.ctx;
        let out_array_in_name = utils::new_c_string("out-array")?;

        let x_array_wrapper = utils::VipsArrayIntWrapper::from(&max_options.x_array[..]);
        let x_array_in = x_array_wrapper.ctx;
        let x_array_in_name = utils::new_c_string("x-array")?;

        let y_array_wrapper = utils::VipsArrayIntWrapper::from(&max_options.y_array[..]);
        let y_array_in = y_array_wrapper.ctx;
        let y_array_in_name = utils::new_c_string("y-array")?;

        let vips_op_response = bindings::vips_max(
            inp_in,
            &mut out_out,
            x_in_name.as_ptr(),
            x_in,
            y_in_name.as_ptr(),
            y_in,
            size_in_name.as_ptr(),
            size_in,
            out_array_in_name.as_ptr(),
            out_array_in,
            x_array_in_name.as_ptr(),
            x_array_in,
            y_array_in_name.as_ptr(),
            y_array_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::MaxError,
        )
    }
}

/// VipsDeviate (deviate), find image standard deviation
/// inp: `&VipsImage` -> Input image
/// returns `f64` - Output value
pub fn deviate(inp: &VipsImage) -> Result<f64> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: f64 = f64::from(0);

        let vips_op_response = bindings::vips_deviate(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::DeviateError,
        )
    }
}

/// VipsStats (stats), find many image stats
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output array of statistics
pub fn stats(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_stats(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::StatError,
        )
    }
}

/// VipsHistFind (hist_find), find image histogram
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output histogram
pub fn hist_find(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_hist_find(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistFindError,
        )
    }
}

/// Options for hist_find operation
#[derive(Clone, Debug)]
pub struct HistFindOptions {
    /// band: `i32` -> Find histogram of band
    /// min: -1, max: 100000, default: -1
    pub band: i32,
}

impl std::default::Default for HistFindOptions {
    fn default() -> Self {
        HistFindOptions {
            band: i32::from(-1),
        }
    }
}

/// VipsHistFind (hist_find), find image histogram
/// inp: `&VipsImage` -> Input image
/// hist_find_options: `&HistFindOptions` -> optional arguments
/// returns `VipsImage` - Output histogram
pub fn hist_find_with_opts(
    inp: &VipsImage,
    hist_find_options: &HistFindOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let band_in: i32 = hist_find_options.band;
        let band_in_name = utils::new_c_string("band")?;

        let vips_op_response = bindings::vips_hist_find(
            inp_in,
            &mut out_out,
            band_in_name.as_ptr(),
            band_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistFindError,
        )
    }
}

/// VipsHistFindNDim (hist_find_ndim), find n-dimensional image histogram
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output histogram
pub fn hist_find_ndim(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_hist_find_ndim(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistFindNdimError,
        )
    }
}

/// Options for hist_find_ndim operation
#[derive(Clone, Debug)]
pub struct HistFindNdimOptions {
    /// bins: `i32` -> Number of bins in each dimension
    /// min: 1, max: 65536, default: 10
    pub bins: i32,
}

impl std::default::Default for HistFindNdimOptions {
    fn default() -> Self {
        HistFindNdimOptions {
            bins: i32::from(10),
        }
    }
}

/// VipsHistFindNDim (hist_find_ndim), find n-dimensional image histogram
/// inp: `&VipsImage` -> Input image
/// hist_find_ndim_options: `&HistFindNdimOptions` -> optional arguments
/// returns `VipsImage` - Output histogram
pub fn hist_find_ndim_with_opts(
    inp: &VipsImage,
    hist_find_ndim_options: &HistFindNdimOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let bins_in: i32 = hist_find_ndim_options.bins;
        let bins_in_name = utils::new_c_string("bins")?;

        let vips_op_response = bindings::vips_hist_find_ndim(
            inp_in,
            &mut out_out,
            bins_in_name.as_ptr(),
            bins_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistFindNdimError,
        )
    }
}

/// VipsHistFindIndexed (hist_find_indexed), find indexed image histogram
/// inp: `&VipsImage` -> Input image
/// index: `&VipsImage` -> Index image
/// returns `VipsImage` - Output histogram
pub fn hist_find_indexed(inp: &VipsImage, index: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let index_in: *mut bindings::VipsImage = index.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_hist_find_indexed(
            inp_in,
            index_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistFindIndexedError,
        )
    }
}

/// Options for hist_find_indexed operation
#[derive(Clone, Debug)]
pub struct HistFindIndexedOptions {
    /// combine: `Combine` -> Combine bins like this
    ///  `Max` -> VIPS_COMBINE_MAX = 0
    ///  `Sum` -> VIPS_COMBINE_SUM = 1 [DEFAULT]
    ///  `Min` -> VIPS_COMBINE_MIN = 2
    ///  `Last` -> VIPS_COMBINE_LAST = 3
    pub combine: Combine,
}

impl std::default::Default for HistFindIndexedOptions {
    fn default() -> Self {
        HistFindIndexedOptions {
            combine: Combine::Sum,
        }
    }
}

/// VipsHistFindIndexed (hist_find_indexed), find indexed image histogram
/// inp: `&VipsImage` -> Input image
/// index: `&VipsImage` -> Index image
/// hist_find_indexed_options: `&HistFindIndexedOptions` -> optional arguments
/// returns `VipsImage` - Output histogram
pub fn hist_find_indexed_with_opts(
    inp: &VipsImage,
    index: &VipsImage,
    hist_find_indexed_options: &HistFindIndexedOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let index_in: *mut bindings::VipsImage = index.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let combine_in: i32 = hist_find_indexed_options.combine as i32;
        let combine_in_name = utils::new_c_string("combine")?;

        let vips_op_response = bindings::vips_hist_find_indexed(
            inp_in,
            index_in,
            &mut out_out,
            combine_in_name.as_ptr(),
            combine_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistFindIndexedError,
        )
    }
}

/// VipsHoughLine (hough_line), find hough line transform
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn hough_line(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_hough_line(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HoughLineError,
        )
    }
}

/// Options for hough_line operation
#[derive(Clone, Debug)]
pub struct HoughLineOptions {
    /// width: `i32` -> Horizontal size of parameter space
    /// min: 1, max: 100000, default: 256
    pub width: i32,
    /// height: `i32` -> Vertical size of parameter space
    /// min: 1, max: 100000, default: 256
    pub height: i32,
}

impl std::default::Default for HoughLineOptions {
    fn default() -> Self {
        HoughLineOptions {
            width: i32::from(256),
            height: i32::from(256),
        }
    }
}

/// VipsHoughLine (hough_line), find hough line transform
/// inp: `&VipsImage` -> Input image
/// hough_line_options: `&HoughLineOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn hough_line_with_opts(
    inp: &VipsImage,
    hough_line_options: &HoughLineOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let width_in: i32 = hough_line_options.width;
        let width_in_name = utils::new_c_string("width")?;

        let height_in: i32 = hough_line_options.height;
        let height_in_name = utils::new_c_string("height")?;

        let vips_op_response = bindings::vips_hough_line(
            inp_in,
            &mut out_out,
            width_in_name.as_ptr(),
            width_in,
            height_in_name.as_ptr(),
            height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HoughLineError,
        )
    }
}

/// VipsHoughCircle (hough_circle), find hough circle transform
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn hough_circle(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_hough_circle(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HoughCircleError,
        )
    }
}

/// Options for hough_circle operation
#[derive(Clone, Debug)]
pub struct HoughCircleOptions {
    /// scale: `i32` -> Scale down dimensions by this factor
    /// min: 1, max: 100000, default: 3
    pub scale: i32,
    /// min_radius: `i32` -> Smallest radius to search for
    /// min: 1, max: 100000, default: 10
    pub min_radius: i32,
    /// max_radius: `i32` -> Largest radius to search for
    /// min: 1, max: 100000, default: 20
    pub max_radius: i32,
}

impl std::default::Default for HoughCircleOptions {
    fn default() -> Self {
        HoughCircleOptions {
            scale: i32::from(3),
            min_radius: i32::from(10),
            max_radius: i32::from(20),
        }
    }
}

/// VipsHoughCircle (hough_circle), find hough circle transform
/// inp: `&VipsImage` -> Input image
/// hough_circle_options: `&HoughCircleOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn hough_circle_with_opts(
    inp: &VipsImage,
    hough_circle_options: &HoughCircleOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let scale_in: i32 = hough_circle_options.scale;
        let scale_in_name = utils::new_c_string("scale")?;

        let min_radius_in: i32 = hough_circle_options.min_radius;
        let min_radius_in_name = utils::new_c_string("min-radius")?;

        let max_radius_in: i32 = hough_circle_options.max_radius;
        let max_radius_in_name = utils::new_c_string("max-radius")?;

        let vips_op_response = bindings::vips_hough_circle(
            inp_in,
            &mut out_out,
            scale_in_name.as_ptr(),
            scale_in,
            min_radius_in_name.as_ptr(),
            min_radius_in,
            max_radius_in_name.as_ptr(),
            max_radius_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HoughCircleError,
        )
    }
}

/// VipsProject (project), find image projections
/// inp: `&VipsImage` -> Input image
/// Tuple (
/// VipsImage - Sums of columns
/// VipsImage - Sums of rows
///)
pub fn project(
    inp: &VipsImage,
) -> Result<(
    VipsImage,
    VipsImage,
)> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut columns_out: *mut bindings::VipsImage = null_mut();
        let mut rows_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_project(
            inp_in,
            &mut columns_out,
            &mut rows_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            (
                VipsImage {
                    ctx: columns_out,
                },
                VipsImage {
                    ctx: rows_out,
                },
            ),
            Error::ProjectError,
        )
    }
}

/// VipsProfile (profile), find image profiles
/// inp: `&VipsImage` -> Input image
/// Tuple (
/// VipsImage - First non-zero pixel in column
/// VipsImage - First non-zero pixel in row
///)
pub fn profile(
    inp: &VipsImage,
) -> Result<(
    VipsImage,
    VipsImage,
)> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut columns_out: *mut bindings::VipsImage = null_mut();
        let mut rows_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_profile(
            inp_in,
            &mut columns_out,
            &mut rows_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            (
                VipsImage {
                    ctx: columns_out,
                },
                VipsImage {
                    ctx: rows_out,
                },
            ),
            Error::ProfileError,
        )
    }
}

/// VipsMeasure (measure), measure a set of patches on a color chart
/// inp: `&VipsImage` -> Image to measure
/// h: `i32` -> Number of patches across chart
/// min: 1, max: 10000000, default: 1
/// v: `i32` -> Number of patches down chart
/// min: 1, max: 10000000, default: 1
/// returns `VipsImage` - Output array of statistics
pub fn measure(inp: &VipsImage, h: i32, v: i32) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let h_in: i32 = h;
        let v_in: i32 = v;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_measure(
            inp_in,
            &mut out_out,
            h_in,
            v_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MeasureError,
        )
    }
}

/// Options for measure operation
#[derive(Clone, Debug)]
pub struct MeasureOptions {
    /// left: `i32` -> Left edge of extract area
    /// min: 0, max: 10000000, default: 0
    pub left: i32,
    /// top: `i32` -> Top edge of extract area
    /// min: 0, max: 10000000, default: 0
    pub top: i32,
    /// width: `i32` -> Width of extract area
    /// min: 1, max: 10000000, default: 1
    pub width: i32,
    /// height: `i32` -> Height of extract area
    /// min: 1, max: 10000000, default: 1
    pub height: i32,
}

impl std::default::Default for MeasureOptions {
    fn default() -> Self {
        MeasureOptions {
            left: i32::from(0),
            top: i32::from(0),
            width: i32::from(1),
            height: i32::from(1),
        }
    }
}

/// VipsMeasure (measure), measure a set of patches on a color chart
/// inp: `&VipsImage` -> Image to measure
/// h: `i32` -> Number of patches across chart
/// min: 1, max: 10000000, default: 1
/// v: `i32` -> Number of patches down chart
/// min: 1, max: 10000000, default: 1
/// measure_options: `&MeasureOptions` -> optional arguments
/// returns `VipsImage` - Output array of statistics
pub fn measure_with_opts(
    inp: &VipsImage,
    h: i32,
    v: i32,
    measure_options: &MeasureOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let h_in: i32 = h;
        let v_in: i32 = v;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let left_in: i32 = measure_options.left;
        let left_in_name = utils::new_c_string("left")?;

        let top_in: i32 = measure_options.top;
        let top_in_name = utils::new_c_string("top")?;

        let width_in: i32 = measure_options.width;
        let width_in_name = utils::new_c_string("width")?;

        let height_in: i32 = measure_options.height;
        let height_in_name = utils::new_c_string("height")?;

        let vips_op_response = bindings::vips_measure(
            inp_in,
            &mut out_out,
            h_in,
            v_in,
            left_in_name.as_ptr(),
            left_in,
            top_in_name.as_ptr(),
            top_in,
            width_in_name.as_ptr(),
            width_in,
            height_in_name.as_ptr(),
            height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MeasureError,
        )
    }
}

/// VipsFindTrim (find_trim), search an image for non-edge areas
/// inp: `&VipsImage` -> Image to find_trim
/// Tuple (
/// i32 - Left edge of image
/// i32 - Top edge of extract area
/// i32 - Width of extract area
/// i32 - Height of extract area
///)
pub fn find_trim(
    inp: &VipsImage,
) -> Result<(
    i32,
    i32,
    i32,
    i32,
)> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut left_out: i32 = i32::from(1);
        let mut top_out: i32 = i32::from(0);
        let mut width_out: i32 = i32::from(1);
        let mut height_out: i32 = i32::from(1);

        let vips_op_response = bindings::vips_find_trim(
            inp_in,
            &mut left_out,
            &mut top_out,
            &mut width_out,
            &mut height_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            (
                left_out,
                top_out,
                width_out,
                height_out,
            ),
            Error::FindTrimError,
        )
    }
}

/// Options for find_trim operation
#[derive(Clone, Debug)]
pub struct FindTrimOptions {
    /// threshold: `f64` -> Object threshold
    /// min: 0, max: inf, default: 10
    pub threshold: f64,
    /// background: `Vec<f64>` -> Color for background pixels
    pub background: Vec<f64>,
    /// line_art: `bool` -> Enable line art mode
    /// default: false
    pub line_art: bool,
}

impl std::default::Default for FindTrimOptions {
    fn default() -> Self {
        FindTrimOptions {
            threshold: f64::from(10),
            background: Vec::new(),
            line_art: false,
        }
    }
}

/// VipsFindTrim (find_trim), search an image for non-edge areas
/// inp: `&VipsImage` -> Image to find_trim
/// find_trim_options: `&FindTrimOptions` -> optional arguments
/// Tuple (
/// i32 - Left edge of image
/// i32 - Top edge of extract area
/// i32 - Width of extract area
/// i32 - Height of extract area
///)
pub fn find_trim_with_opts(
    inp: &VipsImage,
    find_trim_options: &FindTrimOptions,
) -> Result<(
    i32,
    i32,
    i32,
    i32,
)> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut left_out: i32 = i32::from(1);
        let mut top_out: i32 = i32::from(0);
        let mut width_out: i32 = i32::from(1);
        let mut height_out: i32 = i32::from(1);

        let threshold_in: f64 = find_trim_options.threshold;
        let threshold_in_name = utils::new_c_string("threshold")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&find_trim_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let line_art_in: i32 = if find_trim_options.line_art { 1 } else { 0 };
        let line_art_in_name = utils::new_c_string("line-art")?;

        let vips_op_response = bindings::vips_find_trim(
            inp_in,
            &mut left_out,
            &mut top_out,
            &mut width_out,
            &mut height_out,
            threshold_in_name.as_ptr(),
            threshold_in,
            background_in_name.as_ptr(),
            background_in,
            line_art_in_name.as_ptr(),
            line_art_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (
                left_out,
                top_out,
                width_out,
                height_out,
            ),
            Error::FindTrimError,
        )
    }
}

/// VipsCopy (copy), copy an image
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn copy(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_copy(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CopyError,
        )
    }
}

/// Options for copy operation
#[derive(Clone, Debug)]
pub struct CopyOptions {
    /// width: `i32` -> Image width in pixels
    /// min: 0, max: 10000000, default: 0
    pub width: i32,
    /// height: `i32` -> Image height in pixels
    /// min: 0, max: 10000000, default: 0
    pub height: i32,
    /// bands: `i32` -> Number of bands in image
    /// min: 0, max: 10000000, default: 0
    pub bands: i32,
    /// format: `BandFormat` -> Pixel format in image
    ///  `Notset` -> VIPS_FORMAT_NOTSET = -1
    ///  `Uchar` -> VIPS_FORMAT_UCHAR = 0 [DEFAULT]
    ///  `Char` -> VIPS_FORMAT_CHAR = 1
    ///  `Ushort` -> VIPS_FORMAT_USHORT = 2
    ///  `Short` -> VIPS_FORMAT_SHORT = 3
    ///  `Uint` -> VIPS_FORMAT_UINT = 4
    ///  `Int` -> VIPS_FORMAT_INT = 5
    ///  `Float` -> VIPS_FORMAT_FLOAT = 6
    ///  `Complex` -> VIPS_FORMAT_COMPLEX = 7
    ///  `Double` -> VIPS_FORMAT_DOUBLE = 8
    ///  `Dpcomplex` -> VIPS_FORMAT_DPCOMPLEX = 9
    ///  `Last` -> VIPS_FORMAT_LAST = 10
    pub format: BandFormat,
    /// coding: `Coding` -> Pixel coding
    ///  `Error` -> VIPS_CODING_ERROR = -1
    ///  `None` -> VIPS_CODING_NONE = 0 [DEFAULT]
    ///  `Labq` -> VIPS_CODING_LABQ = 2
    ///  `Rad` -> VIPS_CODING_RAD = 6
    ///  `Last` -> VIPS_CODING_LAST = 7
    pub coding: Coding,
    /// interpretation: `Interpretation` -> Pixel interpretation
    ///  `Error` -> VIPS_INTERPRETATION_ERROR = -1
    ///  `Multiband` -> VIPS_INTERPRETATION_MULTIBAND = 0 [DEFAULT]
    ///  `BW` -> VIPS_INTERPRETATION_B_W = 1
    ///  `Histogram` -> VIPS_INTERPRETATION_HISTOGRAM = 10
    ///  `Xyz` -> VIPS_INTERPRETATION_XYZ = 12
    ///  `Lab` -> VIPS_INTERPRETATION_LAB = 13
    ///  `Cmyk` -> VIPS_INTERPRETATION_CMYK = 15
    ///  `Labq` -> VIPS_INTERPRETATION_LABQ = 16
    ///  `Rgb` -> VIPS_INTERPRETATION_RGB = 17
    ///  `Cmc` -> VIPS_INTERPRETATION_CMC = 18
    ///  `Lch` -> VIPS_INTERPRETATION_LCH = 19
    ///  `Lab` -> VIPS_INTERPRETATION_LABS = 21
    ///  `Srgb` -> VIPS_INTERPRETATION_sRGB = 22
    ///  `Yxy` -> VIPS_INTERPRETATION_YXY = 23
    ///  `Fourier` -> VIPS_INTERPRETATION_FOURIER = 24
    ///  `Rgb16` -> VIPS_INTERPRETATION_RGB16 = 25
    ///  `Grey16` -> VIPS_INTERPRETATION_GREY16 = 26
    ///  `Matrix` -> VIPS_INTERPRETATION_MATRIX = 27
    ///  `Scrgb` -> VIPS_INTERPRETATION_scRGB = 28
    ///  `Hsv` -> VIPS_INTERPRETATION_HSV = 29
    ///  `Last` -> VIPS_INTERPRETATION_LAST = 30
    pub interpretation: Interpretation,
    /// xres: `f64` -> Horizontal resolution in pixels/mm
    /// min: -0, max: 1000000, default: 0
    pub xres: f64,
    /// yres: `f64` -> Vertical resolution in pixels/mm
    /// min: -0, max: 1000000, default: 0
    pub yres: f64,
    /// xoffset: `i32` -> Horizontal offset of origin
    /// min: -10000000, max: 10000000, default: 0
    pub xoffset: i32,
    /// yoffset: `i32` -> Vertical offset of origin
    /// min: -10000000, max: 10000000, default: 0
    pub yoffset: i32,
}

impl std::default::Default for CopyOptions {
    fn default() -> Self {
        CopyOptions {
            width: i32::from(0),
            height: i32::from(0),
            bands: i32::from(0),
            format: BandFormat::Uchar,
            coding: Coding::None,
            interpretation: Interpretation::Multiband,
            xres: f64::from(0),
            yres: f64::from(0),
            xoffset: i32::from(0),
            yoffset: i32::from(0),
        }
    }
}

/// VipsCopy (copy), copy an image
/// inp: `&VipsImage` -> Input image
/// copy_options: `&CopyOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn copy_with_opts(inp: &VipsImage, copy_options: &CopyOptions) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let width_in: i32 = copy_options.width;
        let width_in_name = utils::new_c_string("width")?;

        let height_in: i32 = copy_options.height;
        let height_in_name = utils::new_c_string("height")?;

        let bands_in: i32 = copy_options.bands;
        let bands_in_name = utils::new_c_string("bands")?;

        let format_in: i32 = copy_options.format as i32;
        let format_in_name = utils::new_c_string("format")?;

        let coding_in: i32 = copy_options.coding as i32;
        let coding_in_name = utils::new_c_string("coding")?;

        let interpretation_in: i32 = copy_options.interpretation as i32;
        let interpretation_in_name = utils::new_c_string("interpretation")?;

        let xres_in: f64 = copy_options.xres;
        let xres_in_name = utils::new_c_string("xres")?;

        let yres_in: f64 = copy_options.yres;
        let yres_in_name = utils::new_c_string("yres")?;

        let xoffset_in: i32 = copy_options.xoffset;
        let xoffset_in_name = utils::new_c_string("xoffset")?;

        let yoffset_in: i32 = copy_options.yoffset;
        let yoffset_in_name = utils::new_c_string("yoffset")?;

        let vips_op_response = bindings::vips_copy(
            inp_in,
            &mut out_out,
            width_in_name.as_ptr(),
            width_in,
            height_in_name.as_ptr(),
            height_in,
            bands_in_name.as_ptr(),
            bands_in,
            format_in_name.as_ptr(),
            format_in,
            coding_in_name.as_ptr(),
            coding_in,
            interpretation_in_name.as_ptr(),
            interpretation_in,
            xres_in_name.as_ptr(),
            xres_in,
            yres_in_name.as_ptr(),
            yres_in,
            xoffset_in_name.as_ptr(),
            xoffset_in,
            yoffset_in_name.as_ptr(),
            yoffset_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CopyError,
        )
    }
}

/// VipsTileCache (tilecache), cache an image as a set of tiles
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn tilecache(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_tilecache(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::TilecacheError,
        )
    }
}

/// Options for tilecache operation
#[derive(Clone, Debug)]
pub struct TilecacheOptions {
    /// tile_width: `i32` -> Tile width in pixels
    /// min: 1, max: 1000000, default: 128
    pub tile_width: i32,
    /// tile_height: `i32` -> Tile height in pixels
    /// min: 1, max: 1000000, default: 128
    pub tile_height: i32,
    /// max_tiles: `i32` -> Maximum number of tiles to cache
    /// min: -1, max: 1000000, default: 1000
    pub max_tiles: i32,
    /// access: `Access` -> Expected access pattern
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// threaded: `bool` -> Allow threaded access
    /// default: false
    pub threaded: bool,
    /// persistent: `bool` -> Keep cache between evaluations
    /// default: false
    pub persistent: bool,
}

impl std::default::Default for TilecacheOptions {
    fn default() -> Self {
        TilecacheOptions {
            tile_width: i32::from(128),
            tile_height: i32::from(128),
            max_tiles: i32::from(1000),
            access: Access::Random,
            threaded: false,
            persistent: false,
        }
    }
}

/// VipsTileCache (tilecache), cache an image as a set of tiles
/// inp: `&VipsImage` -> Input image
/// tilecache_options: `&TilecacheOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn tilecache_with_opts(
    inp: &VipsImage,
    tilecache_options: &TilecacheOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let tile_width_in: i32 = tilecache_options.tile_width;
        let tile_width_in_name = utils::new_c_string("tile-width")?;

        let tile_height_in: i32 = tilecache_options.tile_height;
        let tile_height_in_name = utils::new_c_string("tile-height")?;

        let max_tiles_in: i32 = tilecache_options.max_tiles;
        let max_tiles_in_name = utils::new_c_string("max-tiles")?;

        let access_in: i32 = tilecache_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let threaded_in: i32 = if tilecache_options.threaded { 1 } else { 0 };
        let threaded_in_name = utils::new_c_string("threaded")?;

        let persistent_in: i32 = if tilecache_options.persistent { 1 } else { 0 };
        let persistent_in_name = utils::new_c_string("persistent")?;

        let vips_op_response = bindings::vips_tilecache(
            inp_in,
            &mut out_out,
            tile_width_in_name.as_ptr(),
            tile_width_in,
            tile_height_in_name.as_ptr(),
            tile_height_in,
            max_tiles_in_name.as_ptr(),
            max_tiles_in,
            access_in_name.as_ptr(),
            access_in,
            threaded_in_name.as_ptr(),
            threaded_in,
            persistent_in_name.as_ptr(),
            persistent_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::TilecacheError,
        )
    }
}

/// VipsLineCache (linecache), cache an image as a set of lines
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn linecache(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_linecache(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LinecacheError,
        )
    }
}

/// Options for linecache operation
#[derive(Clone, Debug)]
pub struct LinecacheOptions {
    /// tile_height: `i32` -> Tile height in pixels
    /// min: 1, max: 1000000, default: 128
    pub tile_height: i32,
    /// access: `Access` -> Expected access pattern
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// threaded: `bool` -> Allow threaded access
    /// default: false
    pub threaded: bool,
    /// persistent: `bool` -> Keep cache between evaluations
    /// default: false
    pub persistent: bool,
}

impl std::default::Default for LinecacheOptions {
    fn default() -> Self {
        LinecacheOptions {
            tile_height: i32::from(128),
            access: Access::Random,
            threaded: false,
            persistent: false,
        }
    }
}

/// VipsLineCache (linecache), cache an image as a set of lines
/// inp: `&VipsImage` -> Input image
/// linecache_options: `&LinecacheOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn linecache_with_opts(
    inp: &VipsImage,
    linecache_options: &LinecacheOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let tile_height_in: i32 = linecache_options.tile_height;
        let tile_height_in_name = utils::new_c_string("tile-height")?;

        let access_in: i32 = linecache_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let threaded_in: i32 = if linecache_options.threaded { 1 } else { 0 };
        let threaded_in_name = utils::new_c_string("threaded")?;

        let persistent_in: i32 = if linecache_options.persistent { 1 } else { 0 };
        let persistent_in_name = utils::new_c_string("persistent")?;

        let vips_op_response = bindings::vips_linecache(
            inp_in,
            &mut out_out,
            tile_height_in_name.as_ptr(),
            tile_height_in,
            access_in_name.as_ptr(),
            access_in,
            threaded_in_name.as_ptr(),
            threaded_in,
            persistent_in_name.as_ptr(),
            persistent_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LinecacheError,
        )
    }
}

/// VipsSequential (sequential), check sequential access
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn sequential(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_sequential(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SequentialError,
        )
    }
}

/// Options for sequential operation
#[derive(Clone, Debug)]
pub struct SequentialOptions {
    /// tile_height: `i32` -> Tile height in pixels
    /// min: 1, max: 1000000, default: 1
    pub tile_height: i32,
}

impl std::default::Default for SequentialOptions {
    fn default() -> Self {
        SequentialOptions {
            tile_height: i32::from(1),
        }
    }
}

/// VipsSequential (sequential), check sequential access
/// inp: `&VipsImage` -> Input image
/// sequential_options: `&SequentialOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn sequential_with_opts(
    inp: &VipsImage,
    sequential_options: &SequentialOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let tile_height_in: i32 = sequential_options.tile_height;
        let tile_height_in_name = utils::new_c_string("tile-height")?;

        let vips_op_response = bindings::vips_sequential(
            inp_in,
            &mut out_out,
            tile_height_in_name.as_ptr(),
            tile_height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SequentialError,
        )
    }
}

/// VipsCache (cache), cache an image
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn cache(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_cache(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CacheError,
        )
    }
}

/// Options for cache operation
#[derive(Clone, Debug)]
pub struct CacheOptions {
    /// max_tiles: `i32` -> Maximum number of tiles to cache
    /// min: -1, max: 1000000, default: 1000
    pub max_tiles: i32,
    /// tile_height: `i32` -> Tile height in pixels
    /// min: 1, max: 1000000, default: 128
    pub tile_height: i32,
    /// tile_width: `i32` -> Tile width in pixels
    /// min: 1, max: 1000000, default: 128
    pub tile_width: i32,
}

impl std::default::Default for CacheOptions {
    fn default() -> Self {
        CacheOptions {
            max_tiles: i32::from(1000),
            tile_height: i32::from(128),
            tile_width: i32::from(128),
        }
    }
}

/// VipsCache (cache), cache an image
/// inp: `&VipsImage` -> Input image
/// cache_options: `&CacheOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn cache_with_opts(inp: &VipsImage, cache_options: &CacheOptions) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let max_tiles_in: i32 = cache_options.max_tiles;
        let max_tiles_in_name = utils::new_c_string("max-tiles")?;

        let tile_height_in: i32 = cache_options.tile_height;
        let tile_height_in_name = utils::new_c_string("tile-height")?;

        let tile_width_in: i32 = cache_options.tile_width;
        let tile_width_in_name = utils::new_c_string("tile-width")?;

        let vips_op_response = bindings::vips_cache(
            inp_in,
            &mut out_out,
            max_tiles_in_name.as_ptr(),
            max_tiles_in,
            tile_height_in_name.as_ptr(),
            tile_height_in,
            tile_width_in_name.as_ptr(),
            tile_width_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CacheError,
        )
    }
}

/// VipsEmbed (embed), embed an image in a larger image
/// inp: `&VipsImage` -> Input image
/// x: `i32` -> Left edge of input in output
/// min: -1000000000, max: 1000000000, default: 0
/// y: `i32` -> Top edge of input in output
/// min: -1000000000, max: 1000000000, default: 0
/// width: `i32` -> Image width in pixels
/// min: 1, max: 1000000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 1000000000, default: 1
/// returns `VipsImage` - Output image
pub fn embed(inp: &VipsImage, x: i32, y: i32, width: i32, height: i32) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let x_in: i32 = x;
        let y_in: i32 = y;
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_embed(
            inp_in,
            &mut out_out,
            x_in,
            y_in,
            width_in,
            height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::EmbedError,
        )
    }
}

/// Options for embed operation
#[derive(Clone, Debug)]
pub struct EmbedOptions {
    /// extend: `Extend` -> How to generate the extra pixels
    ///  `Black` -> VIPS_EXTEND_BLACK = 0 [DEFAULT]
    ///  `Copy` -> VIPS_EXTEND_COPY = 1
    ///  `Repeat` -> VIPS_EXTEND_REPEAT = 2
    ///  `Mirror` -> VIPS_EXTEND_MIRROR = 3
    ///  `White` -> VIPS_EXTEND_WHITE = 4
    ///  `Background` -> VIPS_EXTEND_BACKGROUND = 5
    ///  `Last` -> VIPS_EXTEND_LAST = 6
    pub extend: Extend,
    /// background: `Vec<f64>` -> Color for background pixels
    pub background: Vec<f64>,
}

impl std::default::Default for EmbedOptions {
    fn default() -> Self {
        EmbedOptions {
            extend: Extend::Black,
            background: Vec::new(),
        }
    }
}

/// VipsEmbed (embed), embed an image in a larger image
/// inp: `&VipsImage` -> Input image
/// x: `i32` -> Left edge of input in output
/// min: -1000000000, max: 1000000000, default: 0
/// y: `i32` -> Top edge of input in output
/// min: -1000000000, max: 1000000000, default: 0
/// width: `i32` -> Image width in pixels
/// min: 1, max: 1000000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 1000000000, default: 1
/// embed_options: `&EmbedOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn embed_with_opts(
    inp: &VipsImage,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    embed_options: &EmbedOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let x_in: i32 = x;
        let y_in: i32 = y;
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let extend_in: i32 = embed_options.extend as i32;
        let extend_in_name = utils::new_c_string("extend")?;

        let background_wrapper = utils::VipsArrayDoubleWrapper::from(&embed_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let vips_op_response = bindings::vips_embed(
            inp_in,
            &mut out_out,
            x_in,
            y_in,
            width_in,
            height_in,
            extend_in_name.as_ptr(),
            extend_in,
            background_in_name.as_ptr(),
            background_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::EmbedError,
        )
    }
}

/// VipsGravity (gravity), place an image within a larger image with a certain gravity
/// inp: `&VipsImage` -> Input image
/// direction: `CompassDirection` -> Direction to place image within width/height
///  `Centre` -> VIPS_COMPASS_DIRECTION_CENTRE = 0 [DEFAULT]
///  `North` -> VIPS_COMPASS_DIRECTION_NORTH = 1
///  `East` -> VIPS_COMPASS_DIRECTION_EAST = 2
///  `South` -> VIPS_COMPASS_DIRECTION_SOUTH = 3
///  `West` -> VIPS_COMPASS_DIRECTION_WEST = 4
///  `NorthEast` -> VIPS_COMPASS_DIRECTION_NORTH_EAST = 5
///  `SouthEast` -> VIPS_COMPASS_DIRECTION_SOUTH_EAST = 6
///  `SouthWest` -> VIPS_COMPASS_DIRECTION_SOUTH_WEST = 7
///  `NorthWest` -> VIPS_COMPASS_DIRECTION_NORTH_WEST = 8
///  `Last` -> VIPS_COMPASS_DIRECTION_LAST = 9
/// width: `i32` -> Image width in pixels
/// min: 1, max: 1000000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 1000000000, default: 1
/// returns `VipsImage` - Output image
pub fn gravity(
    inp: &VipsImage,
    direction: CompassDirection,
    width: i32,
    height: i32,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let direction_in: i32 = direction as i32;
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_gravity(
            inp_in,
            &mut out_out,
            direction_in
                .try_into()
                .unwrap(),
            width_in,
            height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GravityError,
        )
    }
}

/// Options for gravity operation
#[derive(Clone, Debug)]
pub struct GravityOptions {
    /// extend: `Extend` -> How to generate the extra pixels
    ///  `Black` -> VIPS_EXTEND_BLACK = 0 [DEFAULT]
    ///  `Copy` -> VIPS_EXTEND_COPY = 1
    ///  `Repeat` -> VIPS_EXTEND_REPEAT = 2
    ///  `Mirror` -> VIPS_EXTEND_MIRROR = 3
    ///  `White` -> VIPS_EXTEND_WHITE = 4
    ///  `Background` -> VIPS_EXTEND_BACKGROUND = 5
    ///  `Last` -> VIPS_EXTEND_LAST = 6
    pub extend: Extend,
    /// background: `Vec<f64>` -> Color for background pixels
    pub background: Vec<f64>,
}

impl std::default::Default for GravityOptions {
    fn default() -> Self {
        GravityOptions {
            extend: Extend::Black,
            background: Vec::new(),
        }
    }
}

/// VipsGravity (gravity), place an image within a larger image with a certain gravity
/// inp: `&VipsImage` -> Input image
/// direction: `CompassDirection` -> Direction to place image within width/height
///  `Centre` -> VIPS_COMPASS_DIRECTION_CENTRE = 0 [DEFAULT]
///  `North` -> VIPS_COMPASS_DIRECTION_NORTH = 1
///  `East` -> VIPS_COMPASS_DIRECTION_EAST = 2
///  `South` -> VIPS_COMPASS_DIRECTION_SOUTH = 3
///  `West` -> VIPS_COMPASS_DIRECTION_WEST = 4
///  `NorthEast` -> VIPS_COMPASS_DIRECTION_NORTH_EAST = 5
///  `SouthEast` -> VIPS_COMPASS_DIRECTION_SOUTH_EAST = 6
///  `SouthWest` -> VIPS_COMPASS_DIRECTION_SOUTH_WEST = 7
///  `NorthWest` -> VIPS_COMPASS_DIRECTION_NORTH_WEST = 8
///  `Last` -> VIPS_COMPASS_DIRECTION_LAST = 9
/// width: `i32` -> Image width in pixels
/// min: 1, max: 1000000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 1000000000, default: 1
/// gravity_options: `&GravityOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn gravity_with_opts(
    inp: &VipsImage,
    direction: CompassDirection,
    width: i32,
    height: i32,
    gravity_options: &GravityOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let direction_in: i32 = direction as i32;
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let extend_in: i32 = gravity_options.extend as i32;
        let extend_in_name = utils::new_c_string("extend")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&gravity_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let vips_op_response = bindings::vips_gravity(
            inp_in,
            &mut out_out,
            direction_in
                .try_into()
                .unwrap(),
            width_in,
            height_in,
            extend_in_name.as_ptr(),
            extend_in,
            background_in_name.as_ptr(),
            background_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GravityError,
        )
    }
}

/// VipsFlip (flip), flip an image
/// inp: `&VipsImage` -> Input image
/// direction: `Direction` -> Direction to flip image
///  `Horizontal` -> VIPS_DIRECTION_HORIZONTAL = 0 [DEFAULT]
///  `Vertical` -> VIPS_DIRECTION_VERTICAL = 1
///  `Last` -> VIPS_DIRECTION_LAST = 2
/// returns `VipsImage` - Output image
pub fn flip(inp: &VipsImage, direction: Direction) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let direction_in: i32 = direction as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_flip(
            inp_in,
            &mut out_out,
            direction_in
                .try_into()
                .unwrap(),
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::FlipError,
        )
    }
}

/// VipsInsert (insert), insert image @sub into @main at @x, @y
/// main: `&VipsImage` -> Main input image
/// sub: `&VipsImage` -> Sub-image to insert into main image
/// x: `i32` -> Left edge of sub in main
/// min: -10000000, max: 10000000, default: 0
/// y: `i32` -> Top edge of sub in main
/// min: -10000000, max: 10000000, default: 0
/// returns `VipsImage` - Output image
pub fn insert(main: &VipsImage, sub: &VipsImage, x: i32, y: i32) -> Result<VipsImage> {
    unsafe {
        let main_in: *mut bindings::VipsImage = main.ctx;
        let sub_in: *mut bindings::VipsImage = sub.ctx;
        let x_in: i32 = x;
        let y_in: i32 = y;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_insert(
            main_in,
            sub_in,
            &mut out_out,
            x_in,
            y_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::InsertError,
        )
    }
}

/// Options for insert operation
#[derive(Clone, Debug)]
pub struct InsertOptions {
    /// expand: `bool` -> Expand output to hold all of both inputs
    /// default: false
    pub expand: bool,
    /// background: `Vec<f64>` -> Color for new pixels
    pub background: Vec<f64>,
}

impl std::default::Default for InsertOptions {
    fn default() -> Self {
        InsertOptions {
            expand: false,
            background: Vec::new(),
        }
    }
}

/// VipsInsert (insert), insert image @sub into @main at @x, @y
/// main: `&VipsImage` -> Main input image
/// sub: `&VipsImage` -> Sub-image to insert into main image
/// x: `i32` -> Left edge of sub in main
/// min: -10000000, max: 10000000, default: 0
/// y: `i32` -> Top edge of sub in main
/// min: -10000000, max: 10000000, default: 0
/// insert_options: `&InsertOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn insert_with_opts(
    main: &VipsImage,
    sub: &VipsImage,
    x: i32,
    y: i32,
    insert_options: &InsertOptions,
) -> Result<VipsImage> {
    unsafe {
        let main_in: *mut bindings::VipsImage = main.ctx;
        let sub_in: *mut bindings::VipsImage = sub.ctx;
        let x_in: i32 = x;
        let y_in: i32 = y;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let expand_in: i32 = if insert_options.expand { 1 } else { 0 };
        let expand_in_name = utils::new_c_string("expand")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&insert_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let vips_op_response = bindings::vips_insert(
            main_in,
            sub_in,
            &mut out_out,
            x_in,
            y_in,
            expand_in_name.as_ptr(),
            expand_in,
            background_in_name.as_ptr(),
            background_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::InsertError,
        )
    }
}

/// VipsJoin (join), join a pair of images
/// in_1: `&VipsImage` -> First input image
/// in_2: `&VipsImage` -> Second input image
/// direction: `Direction` -> Join left-right or up-down
///  `Horizontal` -> VIPS_DIRECTION_HORIZONTAL = 0 [DEFAULT]
///  `Vertical` -> VIPS_DIRECTION_VERTICAL = 1
///  `Last` -> VIPS_DIRECTION_LAST = 2
/// returns `VipsImage` - Output image
pub fn join(in_1: &VipsImage, in_2: &VipsImage, direction: Direction) -> Result<VipsImage> {
    unsafe {
        let in_1_in: *mut bindings::VipsImage = in_1.ctx;
        let in_2_in: *mut bindings::VipsImage = in_2.ctx;
        let direction_in: i32 = direction as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_join(
            in_1_in,
            in_2_in,
            &mut out_out,
            direction_in
                .try_into()
                .unwrap(),
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::JoinError,
        )
    }
}

/// Options for join operation
#[derive(Clone, Debug)]
pub struct JoinOptions {
    /// expand: `bool` -> Expand output to hold all of both inputs
    /// default: false
    pub expand: bool,
    /// shim: `i32` -> Pixels between images
    /// min: 0, max: 1000000, default: 0
    pub shim: i32,
    /// background: `Vec<f64>` -> Colour for new pixels
    pub background: Vec<f64>,
    /// align: `Align` -> Align on the low, centre or high coordinate edge
    ///  `Low` -> VIPS_ALIGN_LOW = 0 [DEFAULT]
    ///  `Centre` -> VIPS_ALIGN_CENTRE = 1
    ///  `High` -> VIPS_ALIGN_HIGH = 2
    ///  `Last` -> VIPS_ALIGN_LAST = 3
    pub align: Align,
}

impl std::default::Default for JoinOptions {
    fn default() -> Self {
        JoinOptions {
            expand: false,
            shim: i32::from(0),
            background: Vec::new(),
            align: Align::Low,
        }
    }
}

/// VipsJoin (join), join a pair of images
/// in_1: `&VipsImage` -> First input image
/// in_2: `&VipsImage` -> Second input image
/// direction: `Direction` -> Join left-right or up-down
///  `Horizontal` -> VIPS_DIRECTION_HORIZONTAL = 0 [DEFAULT]
///  `Vertical` -> VIPS_DIRECTION_VERTICAL = 1
///  `Last` -> VIPS_DIRECTION_LAST = 2
/// join_options: `&JoinOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn join_with_opts(
    in_1: &VipsImage,
    in_2: &VipsImage,
    direction: Direction,
    join_options: &JoinOptions,
) -> Result<VipsImage> {
    unsafe {
        let in_1_in: *mut bindings::VipsImage = in_1.ctx;
        let in_2_in: *mut bindings::VipsImage = in_2.ctx;
        let direction_in: i32 = direction as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let expand_in: i32 = if join_options.expand { 1 } else { 0 };
        let expand_in_name = utils::new_c_string("expand")?;

        let shim_in: i32 = join_options.shim;
        let shim_in_name = utils::new_c_string("shim")?;

        let background_wrapper = utils::VipsArrayDoubleWrapper::from(&join_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let align_in: i32 = join_options.align as i32;
        let align_in_name = utils::new_c_string("align")?;

        let vips_op_response = bindings::vips_join(
            in_1_in,
            in_2_in,
            &mut out_out,
            direction_in
                .try_into()
                .unwrap(),
            expand_in_name.as_ptr(),
            expand_in,
            shim_in_name.as_ptr(),
            shim_in,
            background_in_name.as_ptr(),
            background_in,
            align_in_name.as_ptr(),
            align_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::JoinError,
        )
    }
}

/// VipsArrayjoin (arrayjoin), join an array of images
/// inp: `&mut [VipsImage]` -> Array of input images
/// returns `VipsImage` - Output image
pub fn arrayjoin(inp: &mut [VipsImage]) -> Result<VipsImage> {
    unsafe {
        let (inp_len, mut inp_in) = {
            let len = inp.len();
            let mut input = Vec::new();
            for img in inp {
                input.push(img.ctx)
            }
            (
                len as i32,
                input,
            )
        };
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_arrayjoin(
            inp_in.as_mut_ptr(),
            &mut out_out,
            inp_len,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ArrayjoinError,
        )
    }
}

/// Options for arrayjoin operation
#[derive(Clone, Debug)]
pub struct ArrayjoinOptions {
    /// across: `i32` -> Number of images across grid
    /// min: 1, max: 1000000, default: 1
    pub across: i32,
    /// shim: `i32` -> Pixels between images
    /// min: 0, max: 1000000, default: 0
    pub shim: i32,
    /// background: `Vec<f64>` -> Colour for new pixels
    pub background: Vec<f64>,
    /// halign: `Align` -> Align on the left, centre or right
    ///  `Low` -> VIPS_ALIGN_LOW = 0 [DEFAULT]
    ///  `Centre` -> VIPS_ALIGN_CENTRE = 1
    ///  `High` -> VIPS_ALIGN_HIGH = 2
    ///  `Last` -> VIPS_ALIGN_LAST = 3
    pub halign: Align,
    /// valign: `Align` -> Align on the top, centre or bottom
    ///  `Low` -> VIPS_ALIGN_LOW = 0 [DEFAULT]
    ///  `Centre` -> VIPS_ALIGN_CENTRE = 1
    ///  `High` -> VIPS_ALIGN_HIGH = 2
    ///  `Last` -> VIPS_ALIGN_LAST = 3
    pub valign: Align,
    /// hspacing: `i32` -> Horizontal spacing between images
    /// min: 1, max: 1000000, default: 1
    pub hspacing: i32,
    /// vspacing: `i32` -> Vertical spacing between images
    /// min: 1, max: 1000000, default: 1
    pub vspacing: i32,
}

impl std::default::Default for ArrayjoinOptions {
    fn default() -> Self {
        ArrayjoinOptions {
            across: i32::from(1),
            shim: i32::from(0),
            background: Vec::new(),
            halign: Align::Low,
            valign: Align::Low,
            hspacing: i32::from(1),
            vspacing: i32::from(1),
        }
    }
}

/// VipsArrayjoin (arrayjoin), join an array of images
/// inp: `&mut [VipsImage]` -> Array of input images
/// arrayjoin_options: `&ArrayjoinOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn arrayjoin_with_opts(
    inp: &mut [VipsImage],
    arrayjoin_options: &ArrayjoinOptions,
) -> Result<VipsImage> {
    unsafe {
        let (inp_len, mut inp_in) = {
            let len = inp.len();
            let mut input = Vec::new();
            for img in inp {
                input.push(img.ctx)
            }
            (
                len as i32,
                input,
            )
        };
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let across_in: i32 = arrayjoin_options.across;
        let across_in_name = utils::new_c_string("across")?;

        let shim_in: i32 = arrayjoin_options.shim;
        let shim_in_name = utils::new_c_string("shim")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&arrayjoin_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let halign_in: i32 = arrayjoin_options.halign as i32;
        let halign_in_name = utils::new_c_string("halign")?;

        let valign_in: i32 = arrayjoin_options.valign as i32;
        let valign_in_name = utils::new_c_string("valign")?;

        let hspacing_in: i32 = arrayjoin_options.hspacing;
        let hspacing_in_name = utils::new_c_string("hspacing")?;

        let vspacing_in: i32 = arrayjoin_options.vspacing;
        let vspacing_in_name = utils::new_c_string("vspacing")?;

        let vips_op_response = bindings::vips_arrayjoin(
            inp_in.as_mut_ptr(),
            &mut out_out,
            inp_len,
            across_in_name.as_ptr(),
            across_in,
            shim_in_name.as_ptr(),
            shim_in,
            background_in_name.as_ptr(),
            background_in,
            halign_in_name.as_ptr(),
            halign_in,
            valign_in_name.as_ptr(),
            valign_in,
            hspacing_in_name.as_ptr(),
            hspacing_in,
            vspacing_in_name.as_ptr(),
            vspacing_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ArrayjoinError,
        )
    }
}

/// VipsExtractArea (extract_area), extract an area from an image
/// input: `&VipsImage` -> Input image
/// left: `i32` -> Left edge of extract area
/// min: -10000000, max: 10000000, default: 0
/// top: `i32` -> Top edge of extract area
/// min: -10000000, max: 10000000, default: 0
/// width: `i32` -> Width of extract area
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Height of extract area
/// min: 1, max: 10000000, default: 1
/// returns `VipsImage` - Output image
pub fn extract_area(
    input: &VipsImage,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
) -> Result<VipsImage> {
    unsafe {
        let input_in: *mut bindings::VipsImage = input.ctx;
        let left_in: i32 = left;
        let top_in: i32 = top;
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_extract_area(
            input_in,
            &mut out_out,
            left_in,
            top_in,
            width_in,
            height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ExtractAreaError,
        )
    }
}

/// VipsSmartcrop (smartcrop), extract an area from an image
/// input: `&VipsImage` -> Input image
/// width: `i32` -> Width of extract area
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Height of extract area
/// min: 1, max: 10000000, default: 1
/// returns `VipsImage` - Output image
pub fn smartcrop(input: &VipsImage, width: i32, height: i32) -> Result<VipsImage> {
    unsafe {
        let input_in: *mut bindings::VipsImage = input.ctx;
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_smartcrop(
            input_in,
            &mut out_out,
            width_in,
            height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SmartcropError,
        )
    }
}

/// Options for smartcrop operation
#[derive(Clone, Debug)]
pub struct SmartcropOptions {
    /// attention_x: `i32` -> Horizontal position of attention centre
    /// min: 0, max: 10000000, default: 0
    pub attention_x: i32,
    /// attention_y: `i32` -> Vertical position of attention centre
    /// min: 0, max: 10000000, default: 0
    pub attention_y: i32,
    /// interesting: `Interesting` -> How to measure interestingness
    ///  `None` -> VIPS_INTERESTING_NONE = 0
    ///  `Centre` -> VIPS_INTERESTING_CENTRE = 1
    ///  `Entropy` -> VIPS_INTERESTING_ENTROPY = 2
    ///  `Attention` -> VIPS_INTERESTING_ATTENTION = 3 [DEFAULT]
    ///  `Low` -> VIPS_INTERESTING_LOW = 4
    ///  `High` -> VIPS_INTERESTING_HIGH = 5
    ///  `All` -> VIPS_INTERESTING_ALL = 6
    ///  `Last` -> VIPS_INTERESTING_LAST = 7
    pub interesting: Interesting,
    /// premultiplied: `bool` -> Input image already has premultiplied alpha
    /// default: false
    pub premultiplied: bool,
}

impl std::default::Default for SmartcropOptions {
    fn default() -> Self {
        SmartcropOptions {
            attention_x: i32::from(0),
            attention_y: i32::from(0),
            interesting: Interesting::Attention,
            premultiplied: false,
        }
    }
}

/// VipsSmartcrop (smartcrop), extract an area from an image
/// input: `&VipsImage` -> Input image
/// width: `i32` -> Width of extract area
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Height of extract area
/// min: 1, max: 10000000, default: 1
/// smartcrop_options: `&SmartcropOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn smartcrop_with_opts(
    input: &VipsImage,
    width: i32,
    height: i32,
    smartcrop_options: &SmartcropOptions,
) -> Result<VipsImage> {
    unsafe {
        let input_in: *mut bindings::VipsImage = input.ctx;
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let attention_x_in: i32 = smartcrop_options.attention_x;
        let attention_x_in_name = utils::new_c_string("attention-x")?;

        let attention_y_in: i32 = smartcrop_options.attention_y;
        let attention_y_in_name = utils::new_c_string("attention-y")?;

        let interesting_in: i32 = smartcrop_options.interesting as i32;
        let interesting_in_name = utils::new_c_string("interesting")?;

        let premultiplied_in: i32 = if smartcrop_options.premultiplied { 1 } else { 0 };
        let premultiplied_in_name = utils::new_c_string("premultiplied")?;

        let vips_op_response = bindings::vips_smartcrop(
            input_in,
            &mut out_out,
            width_in,
            height_in,
            attention_x_in_name.as_ptr(),
            attention_x_in,
            attention_y_in_name.as_ptr(),
            attention_y_in,
            interesting_in_name.as_ptr(),
            interesting_in,
            premultiplied_in_name.as_ptr(),
            premultiplied_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SmartcropError,
        )
    }
}

/// VipsExtractBand (extract_band), extract band from an image
/// inp: `&VipsImage` -> Input image
/// band: `i32` -> Band to extract
/// min: 0, max: 10000000, default: 0
/// returns `VipsImage` - Output image
pub fn extract_band(inp: &VipsImage, band: i32) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let band_in: i32 = band;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_extract_band(
            inp_in,
            &mut out_out,
            band_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ExtractBandError,
        )
    }
}

/// Options for extract_band operation
#[derive(Clone, Debug)]
pub struct ExtractBandOptions {
    /// n: `i32` -> Number of bands to extract
    /// min: 1, max: 10000000, default: 1
    pub n: i32,
}

impl std::default::Default for ExtractBandOptions {
    fn default() -> Self {
        ExtractBandOptions {
            n: i32::from(1),
        }
    }
}

/// VipsExtractBand (extract_band), extract band from an image
/// inp: `&VipsImage` -> Input image
/// band: `i32` -> Band to extract
/// min: 0, max: 10000000, default: 0
/// extract_band_options: `&ExtractBandOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn extract_band_with_opts(
    inp: &VipsImage,
    band: i32,
    extract_band_options: &ExtractBandOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let band_in: i32 = band;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let n_in: i32 = extract_band_options.n;
        let n_in_name = utils::new_c_string("n")?;

        let vips_op_response = bindings::vips_extract_band(
            inp_in,
            &mut out_out,
            band_in,
            n_in_name.as_ptr(),
            n_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ExtractBandError,
        )
    }
}

/// VipsBandjoin (bandjoin), bandwise join a set of images
/// inp: `&mut [VipsImage]` -> Array of input images
/// returns `VipsImage` - Output image
pub fn bandjoin(inp: &mut [VipsImage]) -> Result<VipsImage> {
    unsafe {
        let (inp_len, mut inp_in) = {
            let len = inp.len();
            let mut input = Vec::new();
            for img in inp {
                input.push(img.ctx)
            }
            (
                len as i32,
                input,
            )
        };
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_bandjoin(
            inp_in.as_mut_ptr(),
            &mut out_out,
            inp_len,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BandjoinError,
        )
    }
}

/// VipsBandjoinConst (bandjoin_const), append a constant band to an image
/// inp: `&VipsImage` -> Input image
/// c: `&mut [f64]` -> Array of constants to add
/// returns `VipsImage` - Output image
pub fn bandjoin_const(inp: &VipsImage, c: &mut [f64]) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let c_in: *mut f64 = c.as_mut_ptr();
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_bandjoin_const(
            inp_in,
            &mut out_out,
            c_in,
            c.len() as i32,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BandjoinConstError,
        )
    }
}

/// VipsBandrank (bandrank), band-wise rank of a set of images
/// inp: `&mut [VipsImage]` -> Array of input images
/// returns `VipsImage` - Output image
pub fn bandrank(inp: &mut [VipsImage]) -> Result<VipsImage> {
    unsafe {
        let (inp_len, mut inp_in) = {
            let len = inp.len();
            let mut input = Vec::new();
            for img in inp {
                input.push(img.ctx)
            }
            (
                len as i32,
                input,
            )
        };
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_bandrank(
            inp_in.as_mut_ptr(),
            &mut out_out,
            inp_len,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BandrankError,
        )
    }
}

/// Options for bandrank operation
#[derive(Clone, Debug)]
pub struct BandrankOptions {
    /// index: `i32` -> Select this band element from sorted list
    /// min: -1, max: 1000000, default: -1
    pub index: i32,
}

impl std::default::Default for BandrankOptions {
    fn default() -> Self {
        BandrankOptions {
            index: i32::from(-1),
        }
    }
}

/// VipsBandrank (bandrank), band-wise rank of a set of images
/// inp: `&mut [VipsImage]` -> Array of input images
/// bandrank_options: `&BandrankOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn bandrank_with_opts(
    inp: &mut [VipsImage],
    bandrank_options: &BandrankOptions,
) -> Result<VipsImage> {
    unsafe {
        let (inp_len, mut inp_in) = {
            let len = inp.len();
            let mut input = Vec::new();
            for img in inp {
                input.push(img.ctx)
            }
            (
                len as i32,
                input,
            )
        };
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let index_in: i32 = bandrank_options.index;
        let index_in_name = utils::new_c_string("index")?;

        let vips_op_response = bindings::vips_bandrank(
            inp_in.as_mut_ptr(),
            &mut out_out,
            inp_len,
            index_in_name.as_ptr(),
            index_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BandrankError,
        )
    }
}

/// VipsBandmean (bandmean), band-wise average
/// inp: `&VipsImage` -> Input image argument
/// returns `VipsImage` - Output image
pub fn bandmean(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_bandmean(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BandmeanError,
        )
    }
}

/// VipsBandbool (bandbool), boolean operation across image bands
/// inp: `&VipsImage` -> Input image argument
/// boolean: `OperationBoolean` -> Boolean to perform
///  `And` -> VIPS_OPERATION_BOOLEAN_AND = 0 [DEFAULT]
///  `Or` -> VIPS_OPERATION_BOOLEAN_OR = 1
///  `Eor` -> VIPS_OPERATION_BOOLEAN_EOR = 2
///  `Lshift` -> VIPS_OPERATION_BOOLEAN_LSHIFT = 3
///  `Rshift` -> VIPS_OPERATION_BOOLEAN_RSHIFT = 4
///  `Last` -> VIPS_OPERATION_BOOLEAN_LAST = 5
/// returns `VipsImage` - Output image
pub fn bandbool(inp: &VipsImage, boolean: OperationBoolean) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let boolean_in: i32 = boolean as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_bandbool(
            inp_in,
            &mut out_out,
            boolean_in
                .try_into()
                .unwrap(),
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BandboolError,
        )
    }
}

/// VipsReplicate (replicate), replicate an image
/// inp: `&VipsImage` -> Input image
/// across: `i32` -> Repeat this many times horizontally
/// min: 1, max: 1000000, default: 1
/// down: `i32` -> Repeat this many times vertically
/// min: 1, max: 1000000, default: 1
/// returns `VipsImage` - Output image
pub fn replicate(inp: &VipsImage, across: i32, down: i32) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let across_in: i32 = across;
        let down_in: i32 = down;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_replicate(
            inp_in,
            &mut out_out,
            across_in,
            down_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ReplicateError,
        )
    }
}

/// VipsCast (cast), cast an image
/// inp: `&VipsImage` -> Input image
/// format: `BandFormat` -> Format to cast to
///  `Notset` -> VIPS_FORMAT_NOTSET = -1
///  `Uchar` -> VIPS_FORMAT_UCHAR = 0 [DEFAULT]
///  `Char` -> VIPS_FORMAT_CHAR = 1
///  `Ushort` -> VIPS_FORMAT_USHORT = 2
///  `Short` -> VIPS_FORMAT_SHORT = 3
///  `Uint` -> VIPS_FORMAT_UINT = 4
///  `Int` -> VIPS_FORMAT_INT = 5
///  `Float` -> VIPS_FORMAT_FLOAT = 6
///  `Complex` -> VIPS_FORMAT_COMPLEX = 7
///  `Double` -> VIPS_FORMAT_DOUBLE = 8
///  `Dpcomplex` -> VIPS_FORMAT_DPCOMPLEX = 9
///  `Last` -> VIPS_FORMAT_LAST = 10
/// returns `VipsImage` - Output image
pub fn cast(inp: &VipsImage, format: BandFormat) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let format_in: i32 = format as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_cast(
            inp_in,
            &mut out_out,
            format_in
                .try_into()
                .unwrap(),
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CastError,
        )
    }
}

/// Options for cast operation
#[derive(Clone, Debug)]
pub struct CastOptions {
    /// shift: `bool` -> Shift integer values up and down
    /// default: false
    pub shift: bool,
}

impl std::default::Default for CastOptions {
    fn default() -> Self {
        CastOptions {
            shift: false,
        }
    }
}

/// VipsCast (cast), cast an image
/// inp: `&VipsImage` -> Input image
/// format: `BandFormat` -> Format to cast to
///  `Notset` -> VIPS_FORMAT_NOTSET = -1
///  `Uchar` -> VIPS_FORMAT_UCHAR = 0 [DEFAULT]
///  `Char` -> VIPS_FORMAT_CHAR = 1
///  `Ushort` -> VIPS_FORMAT_USHORT = 2
///  `Short` -> VIPS_FORMAT_SHORT = 3
///  `Uint` -> VIPS_FORMAT_UINT = 4
///  `Int` -> VIPS_FORMAT_INT = 5
///  `Float` -> VIPS_FORMAT_FLOAT = 6
///  `Complex` -> VIPS_FORMAT_COMPLEX = 7
///  `Double` -> VIPS_FORMAT_DOUBLE = 8
///  `Dpcomplex` -> VIPS_FORMAT_DPCOMPLEX = 9
///  `Last` -> VIPS_FORMAT_LAST = 10
/// cast_options: `&CastOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn cast_with_opts(
    inp: &VipsImage,
    format: BandFormat,
    cast_options: &CastOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let format_in: i32 = format as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let shift_in: i32 = if cast_options.shift { 1 } else { 0 };
        let shift_in_name = utils::new_c_string("shift")?;

        let vips_op_response = bindings::vips_cast(
            inp_in,
            &mut out_out,
            format_in
                .try_into()
                .unwrap(),
            shift_in_name.as_ptr(),
            shift_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CastError,
        )
    }
}

/// VipsRot (rot), rotate an image
/// inp: `&VipsImage` -> Input image
/// angle: `Angle` -> Angle to rotate image
///  `D0` -> VIPS_ANGLE_D0 = 0
///  `D90` -> VIPS_ANGLE_D90 = 1 [DEFAULT]
///  `D180` -> VIPS_ANGLE_D180 = 2
///  `D270` -> VIPS_ANGLE_D270 = 3
///  `Last` -> VIPS_ANGLE_LAST = 4
/// returns `VipsImage` - Output image
pub fn rot(inp: &VipsImage, angle: Angle) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let angle_in: i32 = angle as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_rot(
            inp_in,
            &mut out_out,
            angle_in
                .try_into()
                .unwrap(),
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RotError,
        )
    }
}

/// VipsRot45 (rot45), rotate an image
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn rot_45(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_rot45(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Rot45Error,
        )
    }
}

/// Options for rot_45 operation
#[derive(Clone, Debug)]
pub struct Rot45Options {
    /// angle: `Angle45` -> Angle to rotate image
    ///  `D0` -> VIPS_ANGLE45_D0 = 0
    ///  `D45` -> VIPS_ANGLE45_D45 = 1 [DEFAULT]
    ///  `D90` -> VIPS_ANGLE45_D90 = 2
    ///  `D135` -> VIPS_ANGLE45_D135 = 3
    ///  `D180` -> VIPS_ANGLE45_D180 = 4
    ///  `D225` -> VIPS_ANGLE45_D225 = 5
    ///  `D270` -> VIPS_ANGLE45_D270 = 6
    ///  `D315` -> VIPS_ANGLE45_D315 = 7
    ///  `Last` -> VIPS_ANGLE45_LAST = 8
    pub angle: Angle45,
}

impl std::default::Default for Rot45Options {
    fn default() -> Self {
        Rot45Options {
            angle: Angle45::D45,
        }
    }
}

/// VipsRot45 (rot45), rotate an image
/// inp: `&VipsImage` -> Input image
/// rot_45_options: `&Rot45Options` -> optional arguments
/// returns `VipsImage` - Output image
pub fn rot_45_with_opts(inp: &VipsImage, rot_45_options: &Rot45Options) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let angle_in: i32 = rot_45_options.angle as i32;
        let angle_in_name = utils::new_c_string("angle")?;

        let vips_op_response = bindings::vips_rot45(
            inp_in,
            &mut out_out,
            angle_in_name.as_ptr(),
            angle_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Rot45Error,
        )
    }
}

/// VipsAutorot (autorot), autorotate image by exif tag
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn autorot(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_autorot(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::AutorotError,
        )
    }
}

/// Options for autorot operation
#[derive(Clone, Debug)]
pub struct AutorotOptions {
    /// angle: `Angle` -> Angle image was rotated by
    ///  `D0` -> VIPS_ANGLE_D0 = 0 [DEFAULT]
    ///  `D90` -> VIPS_ANGLE_D90 = 1
    ///  `D180` -> VIPS_ANGLE_D180 = 2
    ///  `D270` -> VIPS_ANGLE_D270 = 3
    ///  `Last` -> VIPS_ANGLE_LAST = 4
    pub angle: Angle,
    /// flip: `bool` -> Whether the image was flipped or not
    /// default: false
    pub flip: bool,
}

impl std::default::Default for AutorotOptions {
    fn default() -> Self {
        AutorotOptions {
            angle: Angle::D0,
            flip: false,
        }
    }
}

/// VipsAutorot (autorot), autorotate image by exif tag
/// inp: `&VipsImage` -> Input image
/// autorot_options: `&AutorotOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn autorot_with_opts(inp: &VipsImage, autorot_options: &AutorotOptions) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let angle_in: i32 = autorot_options.angle as i32;
        let angle_in_name = utils::new_c_string("angle")?;

        let flip_in: i32 = if autorot_options.flip { 1 } else { 0 };
        let flip_in_name = utils::new_c_string("flip")?;

        let vips_op_response = bindings::vips_autorot(
            inp_in,
            &mut out_out,
            angle_in_name.as_ptr(),
            angle_in,
            flip_in_name.as_ptr(),
            flip_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::AutorotError,
        )
    }
}

/// VipsIfthenelse (ifthenelse), ifthenelse an image
/// cond: `&VipsImage` -> Condition input image
/// in_1: `&VipsImage` -> Source for TRUE pixels
/// in_2: `&VipsImage` -> Source for FALSE pixels
/// returns `VipsImage` - Output image
pub fn ifthenelse(cond: &VipsImage, in_1: &VipsImage, in_2: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let cond_in: *mut bindings::VipsImage = cond.ctx;
        let in_1_in: *mut bindings::VipsImage = in_1.ctx;
        let in_2_in: *mut bindings::VipsImage = in_2.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_ifthenelse(
            cond_in,
            in_1_in,
            in_2_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::IfthenelseError,
        )
    }
}

/// Options for ifthenelse operation
#[derive(Clone, Debug)]
pub struct IfthenelseOptions {
    /// blend: `bool` -> Blend smoothly between then and else parts
    /// default: false
    pub blend: bool,
}

impl std::default::Default for IfthenelseOptions {
    fn default() -> Self {
        IfthenelseOptions {
            blend: false,
        }
    }
}

/// VipsIfthenelse (ifthenelse), ifthenelse an image
/// cond: `&VipsImage` -> Condition input image
/// in_1: `&VipsImage` -> Source for TRUE pixels
/// in_2: `&VipsImage` -> Source for FALSE pixels
/// ifthenelse_options: `&IfthenelseOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn ifthenelse_with_opts(
    cond: &VipsImage,
    in_1: &VipsImage,
    in_2: &VipsImage,
    ifthenelse_options: &IfthenelseOptions,
) -> Result<VipsImage> {
    unsafe {
        let cond_in: *mut bindings::VipsImage = cond.ctx;
        let in_1_in: *mut bindings::VipsImage = in_1.ctx;
        let in_2_in: *mut bindings::VipsImage = in_2.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let blend_in: i32 = if ifthenelse_options.blend { 1 } else { 0 };
        let blend_in_name = utils::new_c_string("blend")?;

        let vips_op_response = bindings::vips_ifthenelse(
            cond_in,
            in_1_in,
            in_2_in,
            &mut out_out,
            blend_in_name.as_ptr(),
            blend_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::IfthenelseError,
        )
    }
}

/// VipsRecomb (recomb), linear recombination with matrix
/// inp: `&VipsImage` -> Input image argument
/// m: `&VipsImage` -> Matrix of coefficients
/// returns `VipsImage` - Output image
pub fn recomb(inp: &VipsImage, m: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let m_in: *mut bindings::VipsImage = m.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_recomb(
            inp_in,
            &mut out_out,
            m_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RecombError,
        )
    }
}

/// VipsBandfold (bandfold), fold up x axis into bands
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn bandfold(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_bandfold(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BandfoldError,
        )
    }
}

/// Options for bandfold operation
#[derive(Clone, Debug)]
pub struct BandfoldOptions {
    /// factor: `i32` -> Fold by this factor
    /// min: 0, max: 10000000, default: 0
    pub factor: i32,
}

impl std::default::Default for BandfoldOptions {
    fn default() -> Self {
        BandfoldOptions {
            factor: i32::from(0),
        }
    }
}

/// VipsBandfold (bandfold), fold up x axis into bands
/// inp: `&VipsImage` -> Input image
/// bandfold_options: `&BandfoldOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn bandfold_with_opts(
    inp: &VipsImage,
    bandfold_options: &BandfoldOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let factor_in: i32 = bandfold_options.factor;
        let factor_in_name = utils::new_c_string("factor")?;

        let vips_op_response = bindings::vips_bandfold(
            inp_in,
            &mut out_out,
            factor_in_name.as_ptr(),
            factor_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BandfoldError,
        )
    }
}

/// VipsBandunfold (bandunfold), unfold image bands into x axis
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn bandunfold(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_bandunfold(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BandunfoldError,
        )
    }
}

/// Options for bandunfold operation
#[derive(Clone, Debug)]
pub struct BandunfoldOptions {
    /// factor: `i32` -> Unfold by this factor
    /// min: 0, max: 10000000, default: 0
    pub factor: i32,
}

impl std::default::Default for BandunfoldOptions {
    fn default() -> Self {
        BandunfoldOptions {
            factor: i32::from(0),
        }
    }
}

/// VipsBandunfold (bandunfold), unfold image bands into x axis
/// inp: `&VipsImage` -> Input image
/// bandunfold_options: `&BandunfoldOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn bandunfold_with_opts(
    inp: &VipsImage,
    bandunfold_options: &BandunfoldOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let factor_in: i32 = bandunfold_options.factor;
        let factor_in_name = utils::new_c_string("factor")?;

        let vips_op_response = bindings::vips_bandunfold(
            inp_in,
            &mut out_out,
            factor_in_name.as_ptr(),
            factor_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BandunfoldError,
        )
    }
}

/// VipsFlatten (flatten), flatten alpha out of an image
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn flatten(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_flatten(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::FlattenError,
        )
    }
}

/// Options for flatten operation
#[derive(Clone, Debug)]
pub struct FlattenOptions {
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// max_alpha: `f64` -> Maximum value of alpha channel
    /// min: 0, max: 100000000, default: 255
    pub max_alpha: f64,
}

impl std::default::Default for FlattenOptions {
    fn default() -> Self {
        FlattenOptions {
            background: Vec::new(),
            max_alpha: f64::from(255),
        }
    }
}

/// VipsFlatten (flatten), flatten alpha out of an image
/// inp: `&VipsImage` -> Input image
/// flatten_options: `&FlattenOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn flatten_with_opts(inp: &VipsImage, flatten_options: &FlattenOptions) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&flatten_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let max_alpha_in: f64 = flatten_options.max_alpha;
        let max_alpha_in_name = utils::new_c_string("max-alpha")?;

        let vips_op_response = bindings::vips_flatten(
            inp_in,
            &mut out_out,
            background_in_name.as_ptr(),
            background_in,
            max_alpha_in_name.as_ptr(),
            max_alpha_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::FlattenError,
        )
    }
}

/// VipsPremultiply (premultiply), premultiply image alpha
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn premultiply(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_premultiply(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PremultiplyError,
        )
    }
}

/// Options for premultiply operation
#[derive(Clone, Debug)]
pub struct PremultiplyOptions {
    /// max_alpha: `f64` -> Maximum value of alpha channel
    /// min: 0, max: 100000000, default: 255
    pub max_alpha: f64,
}

impl std::default::Default for PremultiplyOptions {
    fn default() -> Self {
        PremultiplyOptions {
            max_alpha: f64::from(255),
        }
    }
}

/// VipsPremultiply (premultiply), premultiply image alpha
/// inp: `&VipsImage` -> Input image
/// premultiply_options: `&PremultiplyOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn premultiply_with_opts(
    inp: &VipsImage,
    premultiply_options: &PremultiplyOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let max_alpha_in: f64 = premultiply_options.max_alpha;
        let max_alpha_in_name = utils::new_c_string("max-alpha")?;

        let vips_op_response = bindings::vips_premultiply(
            inp_in,
            &mut out_out,
            max_alpha_in_name.as_ptr(),
            max_alpha_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PremultiplyError,
        )
    }
}

/// VipsUnpremultiply (unpremultiply), unpremultiply image alpha
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn unpremultiply(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_unpremultiply(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::UnpremultiplyError,
        )
    }
}

/// Options for unpremultiply operation
#[derive(Clone, Debug)]
pub struct UnpremultiplyOptions {
    /// max_alpha: `f64` -> Maximum value of alpha channel
    /// min: 0, max: 100000000, default: 255
    pub max_alpha: f64,
    /// alpha_band: `i32` -> Unpremultiply with this alpha
    /// min: 0, max: 100000000, default: 3
    pub alpha_band: i32,
}

impl std::default::Default for UnpremultiplyOptions {
    fn default() -> Self {
        UnpremultiplyOptions {
            max_alpha: f64::from(255),
            alpha_band: i32::from(3),
        }
    }
}

/// VipsUnpremultiply (unpremultiply), unpremultiply image alpha
/// inp: `&VipsImage` -> Input image
/// unpremultiply_options: `&UnpremultiplyOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn unpremultiply_with_opts(
    inp: &VipsImage,
    unpremultiply_options: &UnpremultiplyOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let max_alpha_in: f64 = unpremultiply_options.max_alpha;
        let max_alpha_in_name = utils::new_c_string("max-alpha")?;

        let alpha_band_in: i32 = unpremultiply_options.alpha_band;
        let alpha_band_in_name = utils::new_c_string("alpha-band")?;

        let vips_op_response = bindings::vips_unpremultiply(
            inp_in,
            &mut out_out,
            max_alpha_in_name.as_ptr(),
            max_alpha_in,
            alpha_band_in_name.as_ptr(),
            alpha_band_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::UnpremultiplyError,
        )
    }
}

/// VipsGrid (grid), grid an image
/// inp: `&VipsImage` -> Input image
/// tile_height: `i32` -> Chop into tiles this high
/// min: 1, max: 10000000, default: 128
/// across: `i32` -> Number of tiles across
/// min: 1, max: 10000000, default: 1
/// down: `i32` -> Number of tiles down
/// min: 1, max: 10000000, default: 1
/// returns `VipsImage` - Output image
pub fn grid(inp: &VipsImage, tile_height: i32, across: i32, down: i32) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let tile_height_in: i32 = tile_height;
        let across_in: i32 = across;
        let down_in: i32 = down;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_grid(
            inp_in,
            &mut out_out,
            tile_height_in,
            across_in,
            down_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GridError,
        )
    }
}

/// VipsTranspose3d (transpose3d), transpose3d an image
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn transpose_3d(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_transpose3d(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Transpose3DError,
        )
    }
}

/// Options for transpose_3d operation
#[derive(Clone, Debug)]
pub struct Transpose3DOptions {
    /// page_height: `i32` -> Height of each input page
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
}

impl std::default::Default for Transpose3DOptions {
    fn default() -> Self {
        Transpose3DOptions {
            page_height: i32::from(0),
        }
    }
}

/// VipsTranspose3d (transpose3d), transpose3d an image
/// inp: `&VipsImage` -> Input image
/// transpose_3d_options: `&Transpose3DOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn transpose_3d_with_opts(
    inp: &VipsImage,
    transpose_3d_options: &Transpose3DOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let page_height_in: i32 = transpose_3d_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let vips_op_response = bindings::vips_transpose3d(
            inp_in,
            &mut out_out,
            page_height_in_name.as_ptr(),
            page_height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Transpose3DError,
        )
    }
}

/// VipsScale (scale), scale an image to uchar
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn scale(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_scale(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ScaleError,
        )
    }
}

/// Options for scale operation
#[derive(Clone, Debug)]
pub struct ScaleOptions {
    /// exp: `f64` -> Exponent for log scale
    /// min: 0.00001, max: 10000, default: 0.25
    pub exp: f64,
    /// log: `bool` -> Log scale
    /// default: false
    pub log: bool,
}

impl std::default::Default for ScaleOptions {
    fn default() -> Self {
        ScaleOptions {
            exp: f64::from(0.25),
            log: false,
        }
    }
}

/// VipsScale (scale), scale an image to uchar
/// inp: `&VipsImage` -> Input image
/// scale_options: `&ScaleOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn scale_with_opts(inp: &VipsImage, scale_options: &ScaleOptions) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let exp_in: f64 = scale_options.exp;
        let exp_in_name = utils::new_c_string("exp")?;

        let log_in: i32 = if scale_options.log { 1 } else { 0 };
        let log_in_name = utils::new_c_string("log")?;

        let vips_op_response = bindings::vips_scale(
            inp_in,
            &mut out_out,
            exp_in_name.as_ptr(),
            exp_in,
            log_in_name.as_ptr(),
            log_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ScaleError,
        )
    }
}

/// VipsWrap (wrap), wrap image origin
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn wrap(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_wrap(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::WrapError,
        )
    }
}

/// Options for wrap operation
#[derive(Clone, Debug)]
pub struct WrapOptions {
    /// x: `i32` -> Left edge of input in output
    /// min: -10000000, max: 10000000, default: 0
    pub x: i32,
    /// y: `i32` -> Top edge of input in output
    /// min: -10000000, max: 10000000, default: 0
    pub y: i32,
}

impl std::default::Default for WrapOptions {
    fn default() -> Self {
        WrapOptions {
            x: i32::from(0),
            y: i32::from(0),
        }
    }
}

/// VipsWrap (wrap), wrap image origin
/// inp: `&VipsImage` -> Input image
/// wrap_options: `&WrapOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn wrap_with_opts(inp: &VipsImage, wrap_options: &WrapOptions) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let x_in: i32 = wrap_options.x;
        let x_in_name = utils::new_c_string("x")?;

        let y_in: i32 = wrap_options.y;
        let y_in_name = utils::new_c_string("y")?;

        let vips_op_response = bindings::vips_wrap(
            inp_in,
            &mut out_out,
            x_in_name.as_ptr(),
            x_in,
            y_in_name.as_ptr(),
            y_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::WrapError,
        )
    }
}

/// VipsZoom (zoom), zoom an image
/// input: `&VipsImage` -> Input image
/// xfac: `i32` -> Horizontal zoom factor
/// min: 1, max: 10000000, default: 1
/// yfac: `i32` -> Vertical zoom factor
/// min: 1, max: 10000000, default: 1
/// returns `VipsImage` - Output image
pub fn zoom(input: &VipsImage, xfac: i32, yfac: i32) -> Result<VipsImage> {
    unsafe {
        let input_in: *mut bindings::VipsImage = input.ctx;
        let xfac_in: i32 = xfac;
        let yfac_in: i32 = yfac;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_zoom(
            input_in,
            &mut out_out,
            xfac_in,
            yfac_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ZoomError,
        )
    }
}

/// VipsSubsample (subsample), subsample an image
/// input: `&VipsImage` -> Input image
/// xfac: `i32` -> Horizontal subsample factor
/// min: 1, max: 10000000, default: 1
/// yfac: `i32` -> Vertical subsample factor
/// min: 1, max: 10000000, default: 1
/// returns `VipsImage` - Output image
pub fn subsample(input: &VipsImage, xfac: i32, yfac: i32) -> Result<VipsImage> {
    unsafe {
        let input_in: *mut bindings::VipsImage = input.ctx;
        let xfac_in: i32 = xfac;
        let yfac_in: i32 = yfac;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_subsample(
            input_in,
            &mut out_out,
            xfac_in,
            yfac_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SubsampleError,
        )
    }
}

/// Options for subsample operation
#[derive(Clone, Debug)]
pub struct SubsampleOptions {
    /// point: `bool` -> Point sample
    /// default: false
    pub point: bool,
}

impl std::default::Default for SubsampleOptions {
    fn default() -> Self {
        SubsampleOptions {
            point: false,
        }
    }
}

/// VipsSubsample (subsample), subsample an image
/// input: `&VipsImage` -> Input image
/// xfac: `i32` -> Horizontal subsample factor
/// min: 1, max: 10000000, default: 1
/// yfac: `i32` -> Vertical subsample factor
/// min: 1, max: 10000000, default: 1
/// subsample_options: `&SubsampleOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn subsample_with_opts(
    input: &VipsImage,
    xfac: i32,
    yfac: i32,
    subsample_options: &SubsampleOptions,
) -> Result<VipsImage> {
    unsafe {
        let input_in: *mut bindings::VipsImage = input.ctx;
        let xfac_in: i32 = xfac;
        let yfac_in: i32 = yfac;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let point_in: i32 = if subsample_options.point { 1 } else { 0 };
        let point_in_name = utils::new_c_string("point")?;

        let vips_op_response = bindings::vips_subsample(
            input_in,
            &mut out_out,
            xfac_in,
            yfac_in,
            point_in_name.as_ptr(),
            point_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SubsampleError,
        )
    }
}

/// VipsMsb (msb), pick most-significant byte from an image
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn msb(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_msb(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MsbError,
        )
    }
}

/// Options for msb operation
#[derive(Clone, Debug)]
pub struct MsbOptions {
    /// band: `i32` -> Band to msb
    /// min: 0, max: 100000000, default: 0
    pub band: i32,
}

impl std::default::Default for MsbOptions {
    fn default() -> Self {
        MsbOptions {
            band: i32::from(0),
        }
    }
}

/// VipsMsb (msb), pick most-significant byte from an image
/// inp: `&VipsImage` -> Input image
/// msb_options: `&MsbOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn msb_with_opts(inp: &VipsImage, msb_options: &MsbOptions) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let band_in: i32 = msb_options.band;
        let band_in_name = utils::new_c_string("band")?;

        let vips_op_response = bindings::vips_msb(
            inp_in,
            &mut out_out,
            band_in_name.as_ptr(),
            band_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MsbError,
        )
    }
}

/// VipsByteswap (byteswap), byteswap an image
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn byteswap(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_byteswap(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ByteswapError,
        )
    }
}

/// VipsFalsecolour (falsecolour), false-color an image
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn falsecolour(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_falsecolour(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::FalsecolourError,
        )
    }
}

/// VipsGamma (gamma), gamma an image
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn gamma(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_gamma(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GammaError,
        )
    }
}

/// Options for gamma operation
#[derive(Clone, Debug)]
pub struct GammaOptions {
    /// exponent: `f64` -> Gamma factor
    /// min: 0.000001, max: 1000, default: 2.4
    pub exponent: f64,
}

impl std::default::Default for GammaOptions {
    fn default() -> Self {
        GammaOptions {
            exponent: f64::from(2.4),
        }
    }
}

/// VipsGamma (gamma), gamma an image
/// inp: `&VipsImage` -> Input image
/// gamma_options: `&GammaOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn gamma_with_opts(inp: &VipsImage, gamma_options: &GammaOptions) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let exponent_in: f64 = gamma_options.exponent;
        let exponent_in_name = utils::new_c_string("exponent")?;

        let vips_op_response = bindings::vips_gamma(
            inp_in,
            &mut out_out,
            exponent_in_name.as_ptr(),
            exponent_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GammaError,
        )
    }
}

/// VipsComposite (composite), blend an array of images with an array of blend modes
/// inp: `&mut [VipsImage]` -> Array of input images
/// mode: `&mut [i32]` -> Array of VipsBlendMode to join with
/// returns `VipsImage` - Output image
pub fn composite(inp: &mut [VipsImage], mode: &mut [i32]) -> Result<VipsImage> {
    unsafe {
        let (inp_len, mut inp_in) = {
            let len = inp.len();
            let mut input = Vec::new();
            for img in inp {
                input.push(img.ctx)
            }
            (
                len as i32,
                input,
            )
        };
        let mode_in: *mut i32 = mode.as_mut_ptr();
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_composite(
            inp_in.as_mut_ptr(),
            &mut out_out,
            inp_len,
            mode_in,
            mode.len() as i32,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CompositeError,
        )
    }
}

/// Options for composite operation
#[derive(Clone, Debug)]
pub struct CompositeOptions {
    /// x: `Vec<i32>` -> Array of x coordinates to join at
    pub x: Vec<i32>,
    /// y: `Vec<i32>` -> Array of y coordinates to join at
    pub y: Vec<i32>,
    /// compositing_space: `Interpretation` -> Composite images in this colour space
    ///  `Error` -> VIPS_INTERPRETATION_ERROR = -1
    ///  `Multiband` -> VIPS_INTERPRETATION_MULTIBAND = 0
    ///  `BW` -> VIPS_INTERPRETATION_B_W = 1
    ///  `Histogram` -> VIPS_INTERPRETATION_HISTOGRAM = 10
    ///  `Xyz` -> VIPS_INTERPRETATION_XYZ = 12
    ///  `Lab` -> VIPS_INTERPRETATION_LAB = 13
    ///  `Cmyk` -> VIPS_INTERPRETATION_CMYK = 15
    ///  `Labq` -> VIPS_INTERPRETATION_LABQ = 16
    ///  `Rgb` -> VIPS_INTERPRETATION_RGB = 17
    ///  `Cmc` -> VIPS_INTERPRETATION_CMC = 18
    ///  `Lch` -> VIPS_INTERPRETATION_LCH = 19
    ///  `Lab` -> VIPS_INTERPRETATION_LABS = 21
    ///  `Srgb` -> VIPS_INTERPRETATION_sRGB = 22 [DEFAULT]
    ///  `Yxy` -> VIPS_INTERPRETATION_YXY = 23
    ///  `Fourier` -> VIPS_INTERPRETATION_FOURIER = 24
    ///  `Rgb16` -> VIPS_INTERPRETATION_RGB16 = 25
    ///  `Grey16` -> VIPS_INTERPRETATION_GREY16 = 26
    ///  `Matrix` -> VIPS_INTERPRETATION_MATRIX = 27
    ///  `Scrgb` -> VIPS_INTERPRETATION_scRGB = 28
    ///  `Hsv` -> VIPS_INTERPRETATION_HSV = 29
    ///  `Last` -> VIPS_INTERPRETATION_LAST = 30
    pub compositing_space: Interpretation,
    /// premultiplied: `bool` -> Images have premultiplied alpha
    /// default: false
    pub premultiplied: bool,
}

impl std::default::Default for CompositeOptions {
    fn default() -> Self {
        CompositeOptions {
            x: Vec::new(),
            y: Vec::new(),
            compositing_space: Interpretation::Srgb,
            premultiplied: false,
        }
    }
}

/// VipsComposite (composite), blend an array of images with an array of blend modes
/// inp: `&mut [VipsImage]` -> Array of input images
/// mode: `&mut [i32]` -> Array of VipsBlendMode to join with
/// composite_options: `&CompositeOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn composite_with_opts(
    inp: &mut [VipsImage],
    mode: &mut [i32],
    composite_options: &CompositeOptions,
) -> Result<VipsImage> {
    unsafe {
        let (inp_len, mut inp_in) = {
            let len = inp.len();
            let mut input = Vec::new();
            for img in inp {
                input.push(img.ctx)
            }
            (
                len as i32,
                input,
            )
        };
        let mode_in: *mut i32 = mode.as_mut_ptr();
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let x_wrapper = utils::VipsArrayIntWrapper::from(&composite_options.x[..]);
        let x_in = x_wrapper.ctx;
        let x_in_name = utils::new_c_string("x")?;

        let y_wrapper = utils::VipsArrayIntWrapper::from(&composite_options.y[..]);
        let y_in = y_wrapper.ctx;
        let y_in_name = utils::new_c_string("y")?;

        let compositing_space_in: i32 = composite_options.compositing_space as i32;
        let compositing_space_in_name = utils::new_c_string("compositing-space")?;

        let premultiplied_in: i32 = if composite_options.premultiplied { 1 } else { 0 };
        let premultiplied_in_name = utils::new_c_string("premultiplied")?;

        let vips_op_response = bindings::vips_composite(
            inp_in.as_mut_ptr(),
            &mut out_out,
            inp_len,
            mode_in,
            mode.len() as i32,
            x_in_name.as_ptr(),
            x_in,
            y_in_name.as_ptr(),
            y_in,
            compositing_space_in_name.as_ptr(),
            compositing_space_in,
            premultiplied_in_name.as_ptr(),
            premultiplied_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CompositeError,
        )
    }
}

/// VipsComposite2 (composite2), blend a pair of images with a blend mode
/// base: `&VipsImage` -> Base image
/// overlay: `&VipsImage` -> Overlay image
/// mode: `BlendMode` -> VipsBlendMode to join with
///  `Clear` -> VIPS_BLEND_MODE_CLEAR = 0
///  `Source` -> VIPS_BLEND_MODE_SOURCE = 1
///  `Over` -> VIPS_BLEND_MODE_OVER = 2 [DEFAULT]
///  `In` -> VIPS_BLEND_MODE_IN = 3
///  `Out` -> VIPS_BLEND_MODE_OUT = 4
///  `Atop` -> VIPS_BLEND_MODE_ATOP = 5
///  `Dest` -> VIPS_BLEND_MODE_DEST = 6
///  `DestOver` -> VIPS_BLEND_MODE_DEST_OVER = 7
///  `DestIn` -> VIPS_BLEND_MODE_DEST_IN = 8
///  `DestOut` -> VIPS_BLEND_MODE_DEST_OUT = 9
///  `DestAtop` -> VIPS_BLEND_MODE_DEST_ATOP = 10
///  `Xor` -> VIPS_BLEND_MODE_XOR = 11
///  `Add` -> VIPS_BLEND_MODE_ADD = 12
///  `Saturate` -> VIPS_BLEND_MODE_SATURATE = 13
///  `Multiply` -> VIPS_BLEND_MODE_MULTIPLY = 14
///  `Screen` -> VIPS_BLEND_MODE_SCREEN = 15
///  `Overlay` -> VIPS_BLEND_MODE_OVERLAY = 16
///  `Darken` -> VIPS_BLEND_MODE_DARKEN = 17
///  `Lighten` -> VIPS_BLEND_MODE_LIGHTEN = 18
///  `ColourDodge` -> VIPS_BLEND_MODE_COLOUR_DODGE = 19
///  `ColourBurn` -> VIPS_BLEND_MODE_COLOUR_BURN = 20
///  `HardLight` -> VIPS_BLEND_MODE_HARD_LIGHT = 21
///  `SoftLight` -> VIPS_BLEND_MODE_SOFT_LIGHT = 22
///  `Difference` -> VIPS_BLEND_MODE_DIFFERENCE = 23
///  `Exclusion` -> VIPS_BLEND_MODE_EXCLUSION = 24
///  `Last` -> VIPS_BLEND_MODE_LAST = 25
/// returns `VipsImage` - Output image
pub fn composite_2(base: &VipsImage, overlay: &VipsImage, mode: BlendMode) -> Result<VipsImage> {
    unsafe {
        let base_in: *mut bindings::VipsImage = base.ctx;
        let overlay_in: *mut bindings::VipsImage = overlay.ctx;
        let mode_in: i32 = mode as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_composite2(
            base_in,
            overlay_in,
            &mut out_out,
            mode_in
                .try_into()
                .unwrap(),
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Composite2Error,
        )
    }
}

/// Options for composite_2 operation
#[derive(Clone, Debug)]
pub struct Composite2Options {
    /// x: `i32` -> x position of overlay
    /// min: -10000000, max: 10000000, default: 0
    pub x: i32,
    /// y: `i32` -> y position of overlay
    /// min: -10000000, max: 10000000, default: 0
    pub y: i32,
    /// compositing_space: `Interpretation` -> Composite images in this colour space
    ///  `Error` -> VIPS_INTERPRETATION_ERROR = -1
    ///  `Multiband` -> VIPS_INTERPRETATION_MULTIBAND = 0
    ///  `BW` -> VIPS_INTERPRETATION_B_W = 1
    ///  `Histogram` -> VIPS_INTERPRETATION_HISTOGRAM = 10
    ///  `Xyz` -> VIPS_INTERPRETATION_XYZ = 12
    ///  `Lab` -> VIPS_INTERPRETATION_LAB = 13
    ///  `Cmyk` -> VIPS_INTERPRETATION_CMYK = 15
    ///  `Labq` -> VIPS_INTERPRETATION_LABQ = 16
    ///  `Rgb` -> VIPS_INTERPRETATION_RGB = 17
    ///  `Cmc` -> VIPS_INTERPRETATION_CMC = 18
    ///  `Lch` -> VIPS_INTERPRETATION_LCH = 19
    ///  `Lab` -> VIPS_INTERPRETATION_LABS = 21
    ///  `Srgb` -> VIPS_INTERPRETATION_sRGB = 22 [DEFAULT]
    ///  `Yxy` -> VIPS_INTERPRETATION_YXY = 23
    ///  `Fourier` -> VIPS_INTERPRETATION_FOURIER = 24
    ///  `Rgb16` -> VIPS_INTERPRETATION_RGB16 = 25
    ///  `Grey16` -> VIPS_INTERPRETATION_GREY16 = 26
    ///  `Matrix` -> VIPS_INTERPRETATION_MATRIX = 27
    ///  `Scrgb` -> VIPS_INTERPRETATION_scRGB = 28
    ///  `Hsv` -> VIPS_INTERPRETATION_HSV = 29
    ///  `Last` -> VIPS_INTERPRETATION_LAST = 30
    pub compositing_space: Interpretation,
    /// premultiplied: `bool` -> Images have premultiplied alpha
    /// default: false
    pub premultiplied: bool,
}

impl std::default::Default for Composite2Options {
    fn default() -> Self {
        Composite2Options {
            x: i32::from(0),
            y: i32::from(0),
            compositing_space: Interpretation::Srgb,
            premultiplied: false,
        }
    }
}

/// VipsComposite2 (composite2), blend a pair of images with a blend mode
/// base: `&VipsImage` -> Base image
/// overlay: `&VipsImage` -> Overlay image
/// mode: `BlendMode` -> VipsBlendMode to join with
///  `Clear` -> VIPS_BLEND_MODE_CLEAR = 0
///  `Source` -> VIPS_BLEND_MODE_SOURCE = 1
///  `Over` -> VIPS_BLEND_MODE_OVER = 2 [DEFAULT]
///  `In` -> VIPS_BLEND_MODE_IN = 3
///  `Out` -> VIPS_BLEND_MODE_OUT = 4
///  `Atop` -> VIPS_BLEND_MODE_ATOP = 5
///  `Dest` -> VIPS_BLEND_MODE_DEST = 6
///  `DestOver` -> VIPS_BLEND_MODE_DEST_OVER = 7
///  `DestIn` -> VIPS_BLEND_MODE_DEST_IN = 8
///  `DestOut` -> VIPS_BLEND_MODE_DEST_OUT = 9
///  `DestAtop` -> VIPS_BLEND_MODE_DEST_ATOP = 10
///  `Xor` -> VIPS_BLEND_MODE_XOR = 11
///  `Add` -> VIPS_BLEND_MODE_ADD = 12
///  `Saturate` -> VIPS_BLEND_MODE_SATURATE = 13
///  `Multiply` -> VIPS_BLEND_MODE_MULTIPLY = 14
///  `Screen` -> VIPS_BLEND_MODE_SCREEN = 15
///  `Overlay` -> VIPS_BLEND_MODE_OVERLAY = 16
///  `Darken` -> VIPS_BLEND_MODE_DARKEN = 17
///  `Lighten` -> VIPS_BLEND_MODE_LIGHTEN = 18
///  `ColourDodge` -> VIPS_BLEND_MODE_COLOUR_DODGE = 19
///  `ColourBurn` -> VIPS_BLEND_MODE_COLOUR_BURN = 20
///  `HardLight` -> VIPS_BLEND_MODE_HARD_LIGHT = 21
///  `SoftLight` -> VIPS_BLEND_MODE_SOFT_LIGHT = 22
///  `Difference` -> VIPS_BLEND_MODE_DIFFERENCE = 23
///  `Exclusion` -> VIPS_BLEND_MODE_EXCLUSION = 24
///  `Last` -> VIPS_BLEND_MODE_LAST = 25
/// composite_2_options: `&Composite2Options` -> optional arguments
/// returns `VipsImage` - Output image
pub fn composite_2_with_opts(
    base: &VipsImage,
    overlay: &VipsImage,
    mode: BlendMode,
    composite_2_options: &Composite2Options,
) -> Result<VipsImage> {
    unsafe {
        let base_in: *mut bindings::VipsImage = base.ctx;
        let overlay_in: *mut bindings::VipsImage = overlay.ctx;
        let mode_in: i32 = mode as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let x_in: i32 = composite_2_options.x;
        let x_in_name = utils::new_c_string("x")?;

        let y_in: i32 = composite_2_options.y;
        let y_in_name = utils::new_c_string("y")?;

        let compositing_space_in: i32 = composite_2_options.compositing_space as i32;
        let compositing_space_in_name = utils::new_c_string("compositing-space")?;

        let premultiplied_in: i32 = if composite_2_options.premultiplied { 1 } else { 0 };
        let premultiplied_in_name = utils::new_c_string("premultiplied")?;

        let vips_op_response = bindings::vips_composite2(
            base_in,
            overlay_in,
            &mut out_out,
            mode_in
                .try_into()
                .unwrap(),
            x_in_name.as_ptr(),
            x_in,
            y_in_name.as_ptr(),
            y_in,
            compositing_space_in_name.as_ptr(),
            compositing_space_in,
            premultiplied_in_name.as_ptr(),
            premultiplied_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Composite2Error,
        )
    }
}

/// VipsBlack (black), make a black image
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// returns `VipsImage` - Output image
pub fn black(width: i32, height: i32) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_black(
            &mut out_out,
            width_in,
            height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BlackError,
        )
    }
}

/// Options for black operation
#[derive(Clone, Debug)]
pub struct BlackOptions {
    /// bands: `i32` -> Number of bands in image
    /// min: 1, max: 10000000, default: 1
    pub bands: i32,
}

impl std::default::Default for BlackOptions {
    fn default() -> Self {
        BlackOptions {
            bands: i32::from(1),
        }
    }
}

/// VipsBlack (black), make a black image
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// black_options: `&BlackOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn black_with_opts(width: i32, height: i32, black_options: &BlackOptions) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let bands_in: i32 = black_options.bands;
        let bands_in_name = utils::new_c_string("bands")?;

        let vips_op_response = bindings::vips_black(
            &mut out_out,
            width_in,
            height_in,
            bands_in_name.as_ptr(),
            bands_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BlackError,
        )
    }
}

/// VipsGaussnoise (gaussnoise), make a gaussnoise image
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// returns `VipsImage` - Output image
pub fn gaussnoise(width: i32, height: i32) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_gaussnoise(
            &mut out_out,
            width_in,
            height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GaussnoiseError,
        )
    }
}

/// Options for gaussnoise operation
#[derive(Clone, Debug)]
pub struct GaussnoiseOptions {
    /// sigma: `f64` -> Standard deviation of pixels in generated image
    /// min: 0, max: 100000, default: 30
    pub sigma: f64,
    /// mean: `f64` -> Mean of pixels in generated image
    /// min: -10000000, max: 1000000, default: 128
    pub mean: f64,
    /// seed: `i32` -> Random number seed
    /// min: -2147483648, max: 2147483647, default: 0
    pub seed: i32,
}

impl std::default::Default for GaussnoiseOptions {
    fn default() -> Self {
        GaussnoiseOptions {
            sigma: f64::from(30),
            mean: f64::from(128),
            seed: i32::from(0),
        }
    }
}

/// VipsGaussnoise (gaussnoise), make a gaussnoise image
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// gaussnoise_options: `&GaussnoiseOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn gaussnoise_with_opts(
    width: i32,
    height: i32,
    gaussnoise_options: &GaussnoiseOptions,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let sigma_in: f64 = gaussnoise_options.sigma;
        let sigma_in_name = utils::new_c_string("sigma")?;

        let mean_in: f64 = gaussnoise_options.mean;
        let mean_in_name = utils::new_c_string("mean")?;

        let seed_in: i32 = gaussnoise_options.seed;
        let seed_in_name = utils::new_c_string("seed")?;

        let vips_op_response = bindings::vips_gaussnoise(
            &mut out_out,
            width_in,
            height_in,
            sigma_in_name.as_ptr(),
            sigma_in,
            mean_in_name.as_ptr(),
            mean_in,
            seed_in_name.as_ptr(),
            seed_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GaussnoiseError,
        )
    }
}

/// VipsXyz (xyz), make an image where pixel values are coordinates
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 64
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 64
/// returns `VipsImage` - Output image
pub fn xyz(width: i32, height: i32) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_xyz(
            &mut out_out,
            width_in,
            height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::XyzError,
        )
    }
}

/// Options for xyz operation
#[derive(Clone, Debug)]
pub struct XyzOptions {
    /// csize: `i32` -> Size of third dimension
    /// min: 1, max: 10000000, default: 1
    pub csize: i32,
    /// dsize: `i32` -> Size of fourth dimension
    /// min: 1, max: 10000000, default: 1
    pub dsize: i32,
    /// esize: `i32` -> Size of fifth dimension
    /// min: 1, max: 10000000, default: 1
    pub esize: i32,
}

impl std::default::Default for XyzOptions {
    fn default() -> Self {
        XyzOptions {
            csize: i32::from(1),
            dsize: i32::from(1),
            esize: i32::from(1),
        }
    }
}

/// VipsXyz (xyz), make an image where pixel values are coordinates
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 64
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 64
/// xyz_options: `&XyzOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn xyz_with_opts(width: i32, height: i32, xyz_options: &XyzOptions) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let csize_in: i32 = xyz_options.csize;
        let csize_in_name = utils::new_c_string("csize")?;

        let dsize_in: i32 = xyz_options.dsize;
        let dsize_in_name = utils::new_c_string("dsize")?;

        let esize_in: i32 = xyz_options.esize;
        let esize_in_name = utils::new_c_string("esize")?;

        let vips_op_response = bindings::vips_xyz(
            &mut out_out,
            width_in,
            height_in,
            csize_in_name.as_ptr(),
            csize_in,
            dsize_in_name.as_ptr(),
            dsize_in,
            esize_in_name.as_ptr(),
            esize_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::XyzError,
        )
    }
}

/// VipsGaussmat (gaussmat), make a gaussian image
/// sigma: `f64` -> Sigma of Gaussian
/// min: 0.000001, max: 10000, default: 1
/// min_ampl: `f64` -> Minimum amplitude of Gaussian
/// min: 0.000001, max: 10000, default: 0.1
/// returns `VipsImage` - Output image
pub fn gaussmat(sigma: f64, min_ampl: f64) -> Result<VipsImage> {
    unsafe {
        let sigma_in: f64 = sigma;
        let min_ampl_in: f64 = min_ampl;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_gaussmat(
            &mut out_out,
            sigma_in,
            min_ampl_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GaussmatError,
        )
    }
}

/// Options for gaussmat operation
#[derive(Clone, Debug)]
pub struct GaussmatOptions {
    /// separable: `bool` -> Generate separable Gaussian
    /// default: false
    pub separable: bool,
    /// precision: `Precision` -> Generate with this precision
    ///  `Integer` -> VIPS_PRECISION_INTEGER = 0 [DEFAULT]
    ///  `Float` -> VIPS_PRECISION_FLOAT = 1
    ///  `Approximate` -> VIPS_PRECISION_APPROXIMATE = 2
    ///  `Last` -> VIPS_PRECISION_LAST = 3
    pub precision: Precision,
}

impl std::default::Default for GaussmatOptions {
    fn default() -> Self {
        GaussmatOptions {
            separable: false,
            precision: Precision::Integer,
        }
    }
}

/// VipsGaussmat (gaussmat), make a gaussian image
/// sigma: `f64` -> Sigma of Gaussian
/// min: 0.000001, max: 10000, default: 1
/// min_ampl: `f64` -> Minimum amplitude of Gaussian
/// min: 0.000001, max: 10000, default: 0.1
/// gaussmat_options: `&GaussmatOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn gaussmat_with_opts(
    sigma: f64,
    min_ampl: f64,
    gaussmat_options: &GaussmatOptions,
) -> Result<VipsImage> {
    unsafe {
        let sigma_in: f64 = sigma;
        let min_ampl_in: f64 = min_ampl;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let separable_in: i32 = if gaussmat_options.separable { 1 } else { 0 };
        let separable_in_name = utils::new_c_string("separable")?;

        let precision_in: i32 = gaussmat_options.precision as i32;
        let precision_in_name = utils::new_c_string("precision")?;

        let vips_op_response = bindings::vips_gaussmat(
            &mut out_out,
            sigma_in,
            min_ampl_in,
            separable_in_name.as_ptr(),
            separable_in,
            precision_in_name.as_ptr(),
            precision_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GaussmatError,
        )
    }
}

/// VipsLogmat (logmat), make a Laplacian of Gaussian image
/// sigma: `f64` -> Radius of Gaussian
/// min: 0.000001, max: 10000, default: 1
/// min_ampl: `f64` -> Minimum amplitude of Gaussian
/// min: 0.000001, max: 10000, default: 0.1
/// returns `VipsImage` - Output image
pub fn logmat(sigma: f64, min_ampl: f64) -> Result<VipsImage> {
    unsafe {
        let sigma_in: f64 = sigma;
        let min_ampl_in: f64 = min_ampl;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_logmat(
            &mut out_out,
            sigma_in,
            min_ampl_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LogmatError,
        )
    }
}

/// Options for logmat operation
#[derive(Clone, Debug)]
pub struct LogmatOptions {
    /// separable: `bool` -> Generate separable Gaussian
    /// default: false
    pub separable: bool,
    /// precision: `Precision` -> Generate with this precision
    ///  `Integer` -> VIPS_PRECISION_INTEGER = 0 [DEFAULT]
    ///  `Float` -> VIPS_PRECISION_FLOAT = 1
    ///  `Approximate` -> VIPS_PRECISION_APPROXIMATE = 2
    ///  `Last` -> VIPS_PRECISION_LAST = 3
    pub precision: Precision,
}

impl std::default::Default for LogmatOptions {
    fn default() -> Self {
        LogmatOptions {
            separable: false,
            precision: Precision::Integer,
        }
    }
}

/// VipsLogmat (logmat), make a Laplacian of Gaussian image
/// sigma: `f64` -> Radius of Gaussian
/// min: 0.000001, max: 10000, default: 1
/// min_ampl: `f64` -> Minimum amplitude of Gaussian
/// min: 0.000001, max: 10000, default: 0.1
/// logmat_options: `&LogmatOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn logmat_with_opts(
    sigma: f64,
    min_ampl: f64,
    logmat_options: &LogmatOptions,
) -> Result<VipsImage> {
    unsafe {
        let sigma_in: f64 = sigma;
        let min_ampl_in: f64 = min_ampl;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let separable_in: i32 = if logmat_options.separable { 1 } else { 0 };
        let separable_in_name = utils::new_c_string("separable")?;

        let precision_in: i32 = logmat_options.precision as i32;
        let precision_in_name = utils::new_c_string("precision")?;

        let vips_op_response = bindings::vips_logmat(
            &mut out_out,
            sigma_in,
            min_ampl_in,
            separable_in_name.as_ptr(),
            separable_in,
            precision_in_name.as_ptr(),
            precision_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LogmatError,
        )
    }
}

/// VipsText (text), make a text image
/// text: `&str` -> Text to render
/// returns `VipsImage` - Output image
pub fn text(text: &str) -> Result<VipsImage> {
    unsafe {
        let text_in: CString = utils::new_c_string(text)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_text(
            &mut out_out,
            text_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::TextError,
        )
    }
}

/// Options for text operation
#[derive(Clone, Debug)]
pub struct TextOptions {
    /// font: `String` -> Font to render with
    pub font: String,
    /// width: `i32` -> Maximum image width in pixels
    /// min: 0, max: 10000000, default: 0
    pub width: i32,
    /// height: `i32` -> Maximum image height in pixels
    /// min: 0, max: 10000000, default: 0
    pub height: i32,
    /// align: `Align` -> Align on the low, centre or high edge
    ///  `Low` -> VIPS_ALIGN_LOW = 0 [DEFAULT]
    ///  `Centre` -> VIPS_ALIGN_CENTRE = 1
    ///  `High` -> VIPS_ALIGN_HIGH = 2
    ///  `Last` -> VIPS_ALIGN_LAST = 3
    pub align: Align,
    /// justify: `bool` -> Justify lines
    /// default: false
    pub justify: bool,
    /// dpi: `i32` -> DPI to render at
    /// min: 1, max: 1000000, default: 72
    pub dpi: i32,
    /// autofit_dpi: `i32` -> DPI selected by autofit
    /// min: 1, max: 1000000, default: 72
    pub autofit_dpi: i32,
    /// spacing: `i32` -> Line spacing
    /// min: -1000000, max: 1000000, default: 0
    pub spacing: i32,
    /// fontfile: `String` -> Load this font file
    pub fontfile: String,
    /// rgba: `bool` -> Enable RGBA output
    /// default: false
    pub rgba: bool,
    /// wrap: `TextWrap` -> Wrap lines on word or character boundaries
    ///  `Word` -> VIPS_TEXT_WRAP_WORD = 0 [DEFAULT]
    ///  `Char` -> VIPS_TEXT_WRAP_CHAR = 1
    ///  `WordChar` -> VIPS_TEXT_WRAP_WORD_CHAR = 2
    ///  `None` -> VIPS_TEXT_WRAP_NONE = 3
    ///  `Last` -> VIPS_TEXT_WRAP_LAST = 4
    pub wrap: TextWrap,
}

impl std::default::Default for TextOptions {
    fn default() -> Self {
        TextOptions {
            font: String::new(),
            width: i32::from(0),
            height: i32::from(0),
            align: Align::Low,
            justify: false,
            dpi: i32::from(72),
            autofit_dpi: i32::from(72),
            spacing: i32::from(0),
            fontfile: String::new(),
            rgba: false,
            wrap: TextWrap::Word,
        }
    }
}

/// VipsText (text), make a text image
/// text: `&str` -> Text to render
/// text_options: `&TextOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn text_with_opts(text: &str, text_options: &TextOptions) -> Result<VipsImage> {
    unsafe {
        let text_in: CString = utils::new_c_string(text)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let font_in: CString = utils::new_c_string(&text_options.font)?;
        let font_in_name = utils::new_c_string("font")?;

        let width_in: i32 = text_options.width;
        let width_in_name = utils::new_c_string("width")?;

        let height_in: i32 = text_options.height;
        let height_in_name = utils::new_c_string("height")?;

        let align_in: i32 = text_options.align as i32;
        let align_in_name = utils::new_c_string("align")?;

        let justify_in: i32 = if text_options.justify { 1 } else { 0 };
        let justify_in_name = utils::new_c_string("justify")?;

        let dpi_in: i32 = text_options.dpi;
        let dpi_in_name = utils::new_c_string("dpi")?;

        let autofit_dpi_in: i32 = text_options.autofit_dpi;
        let autofit_dpi_in_name = utils::new_c_string("autofit-dpi")?;

        let spacing_in: i32 = text_options.spacing;
        let spacing_in_name = utils::new_c_string("spacing")?;

        let fontfile_in: CString = utils::new_c_string(&text_options.fontfile)?;
        let fontfile_in_name = utils::new_c_string("fontfile")?;

        let rgba_in: i32 = if text_options.rgba { 1 } else { 0 };
        let rgba_in_name = utils::new_c_string("rgba")?;

        let wrap_in: i32 = text_options.wrap as i32;
        let wrap_in_name = utils::new_c_string("wrap")?;

        let vips_op_response = bindings::vips_text(
            &mut out_out,
            text_in.as_ptr(),
            font_in_name.as_ptr(),
            font_in.as_ptr(),
            width_in_name.as_ptr(),
            width_in,
            height_in_name.as_ptr(),
            height_in,
            align_in_name.as_ptr(),
            align_in,
            justify_in_name.as_ptr(),
            justify_in,
            dpi_in_name.as_ptr(),
            dpi_in,
            autofit_dpi_in_name.as_ptr(),
            autofit_dpi_in,
            spacing_in_name.as_ptr(),
            spacing_in,
            fontfile_in_name.as_ptr(),
            fontfile_in.as_ptr(),
            rgba_in_name.as_ptr(),
            rgba_in,
            wrap_in_name.as_ptr(),
            wrap_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::TextError,
        )
    }
}

/// VipsEye (eye), make an image showing the eye's spatial response
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// returns `VipsImage` - Output image
pub fn eye(width: i32, height: i32) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_eye(
            &mut out_out,
            width_in,
            height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::EyeError,
        )
    }
}

/// Options for eye operation
#[derive(Clone, Debug)]
pub struct EyeOptions {
    /// uchar: `bool` -> Output an unsigned char image
    /// default: false
    pub uchar: bool,
    /// factor: `f64` -> Maximum spatial frequency
    /// min: 0, max: 1, default: 0.5
    pub factor: f64,
}

impl std::default::Default for EyeOptions {
    fn default() -> Self {
        EyeOptions {
            uchar: false,
            factor: f64::from(0.5),
        }
    }
}

/// VipsEye (eye), make an image showing the eye's spatial response
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// eye_options: `&EyeOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn eye_with_opts(width: i32, height: i32, eye_options: &EyeOptions) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let uchar_in: i32 = if eye_options.uchar { 1 } else { 0 };
        let uchar_in_name = utils::new_c_string("uchar")?;

        let factor_in: f64 = eye_options.factor;
        let factor_in_name = utils::new_c_string("factor")?;

        let vips_op_response = bindings::vips_eye(
            &mut out_out,
            width_in,
            height_in,
            uchar_in_name.as_ptr(),
            uchar_in,
            factor_in_name.as_ptr(),
            factor_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::EyeError,
        )
    }
}

/// VipsGrey (grey), make a grey ramp image
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// returns `VipsImage` - Output image
pub fn grey(width: i32, height: i32) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_grey(
            &mut out_out,
            width_in,
            height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GreyError,
        )
    }
}

/// Options for grey operation
#[derive(Clone, Debug)]
pub struct GreyOptions {
    /// uchar: `bool` -> Output an unsigned char image
    /// default: false
    pub uchar: bool,
}

impl std::default::Default for GreyOptions {
    fn default() -> Self {
        GreyOptions {
            uchar: false,
        }
    }
}

/// VipsGrey (grey), make a grey ramp image
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// grey_options: `&GreyOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn grey_with_opts(width: i32, height: i32, grey_options: &GreyOptions) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let uchar_in: i32 = if grey_options.uchar { 1 } else { 0 };
        let uchar_in_name = utils::new_c_string("uchar")?;

        let vips_op_response = bindings::vips_grey(
            &mut out_out,
            width_in,
            height_in,
            uchar_in_name.as_ptr(),
            uchar_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GreyError,
        )
    }
}

/// VipsZone (zone), make a zone plate
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// returns `VipsImage` - Output image
pub fn zone(width: i32, height: i32) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_zone(
            &mut out_out,
            width_in,
            height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ZoneError,
        )
    }
}

/// Options for zone operation
#[derive(Clone, Debug)]
pub struct ZoneOptions {
    /// uchar: `bool` -> Output an unsigned char image
    /// default: false
    pub uchar: bool,
}

impl std::default::Default for ZoneOptions {
    fn default() -> Self {
        ZoneOptions {
            uchar: false,
        }
    }
}

/// VipsZone (zone), make a zone plate
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// zone_options: `&ZoneOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn zone_with_opts(width: i32, height: i32, zone_options: &ZoneOptions) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let uchar_in: i32 = if zone_options.uchar { 1 } else { 0 };
        let uchar_in_name = utils::new_c_string("uchar")?;

        let vips_op_response = bindings::vips_zone(
            &mut out_out,
            width_in,
            height_in,
            uchar_in_name.as_ptr(),
            uchar_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ZoneError,
        )
    }
}

/// VipsSines (sines), make a 2D sine wave
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// returns `VipsImage` - Output image
pub fn sines(width: i32, height: i32) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_sines(
            &mut out_out,
            width_in,
            height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SineError,
        )
    }
}

/// Options for sines operation
#[derive(Clone, Debug)]
pub struct SineOptions {
    /// uchar: `bool` -> Output an unsigned char image
    /// default: false
    pub uchar: bool,
    /// hfreq: `f64` -> Horizontal spatial frequency
    /// min: 0, max: 10000, default: 0.5
    pub hfreq: f64,
    /// vfreq: `f64` -> Vertical spatial frequency
    /// min: 0, max: 10000, default: 0.5
    pub vfreq: f64,
}

impl std::default::Default for SineOptions {
    fn default() -> Self {
        SineOptions {
            uchar: false,
            hfreq: f64::from(0.5),
            vfreq: f64::from(0.5),
        }
    }
}

/// VipsSines (sines), make a 2D sine wave
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// sines_options: `&SineOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn sines_with_opts(width: i32, height: i32, sines_options: &SineOptions) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let uchar_in: i32 = if sines_options.uchar { 1 } else { 0 };
        let uchar_in_name = utils::new_c_string("uchar")?;

        let hfreq_in: f64 = sines_options.hfreq;
        let hfreq_in_name = utils::new_c_string("hfreq")?;

        let vfreq_in: f64 = sines_options.vfreq;
        let vfreq_in_name = utils::new_c_string("vfreq")?;

        let vips_op_response = bindings::vips_sines(
            &mut out_out,
            width_in,
            height_in,
            uchar_in_name.as_ptr(),
            uchar_in,
            hfreq_in_name.as_ptr(),
            hfreq_in,
            vfreq_in_name.as_ptr(),
            vfreq_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SineError,
        )
    }
}

/// VipsMaskIdeal (mask_ideal), make an ideal filter
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// frequency_cutoff: `f64` -> Frequency cutoff
/// min: 0, max: 1000000, default: 0.5
/// returns `VipsImage` - Output image
pub fn mask_ideal(width: i32, height: i32, frequency_cutoff: f64) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let frequency_cutoff_in: f64 = frequency_cutoff;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_mask_ideal(
            &mut out_out,
            width_in,
            height_in,
            frequency_cutoff_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskIdealError,
        )
    }
}

/// Options for mask_ideal operation
#[derive(Clone, Debug)]
pub struct MaskIdealOptions {
    /// uchar: `bool` -> Output an unsigned char image
    /// default: false
    pub uchar: bool,
    /// nodc: `bool` -> Remove DC component
    /// default: false
    pub nodc: bool,
    /// reject: `bool` -> Invert the sense of the filter
    /// default: false
    pub reject: bool,
    /// optical: `bool` -> Rotate quadrants to optical space
    /// default: false
    pub optical: bool,
}

impl std::default::Default for MaskIdealOptions {
    fn default() -> Self {
        MaskIdealOptions {
            uchar: false,
            nodc: false,
            reject: false,
            optical: false,
        }
    }
}

/// VipsMaskIdeal (mask_ideal), make an ideal filter
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// frequency_cutoff: `f64` -> Frequency cutoff
/// min: 0, max: 1000000, default: 0.5
/// mask_ideal_options: `&MaskIdealOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn mask_ideal_with_opts(
    width: i32,
    height: i32,
    frequency_cutoff: f64,
    mask_ideal_options: &MaskIdealOptions,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let frequency_cutoff_in: f64 = frequency_cutoff;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let uchar_in: i32 = if mask_ideal_options.uchar { 1 } else { 0 };
        let uchar_in_name = utils::new_c_string("uchar")?;

        let nodc_in: i32 = if mask_ideal_options.nodc { 1 } else { 0 };
        let nodc_in_name = utils::new_c_string("nodc")?;

        let reject_in: i32 = if mask_ideal_options.reject { 1 } else { 0 };
        let reject_in_name = utils::new_c_string("reject")?;

        let optical_in: i32 = if mask_ideal_options.optical { 1 } else { 0 };
        let optical_in_name = utils::new_c_string("optical")?;

        let vips_op_response = bindings::vips_mask_ideal(
            &mut out_out,
            width_in,
            height_in,
            frequency_cutoff_in,
            uchar_in_name.as_ptr(),
            uchar_in,
            nodc_in_name.as_ptr(),
            nodc_in,
            reject_in_name.as_ptr(),
            reject_in,
            optical_in_name.as_ptr(),
            optical_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskIdealError,
        )
    }
}

/// VipsMaskIdealRing (mask_ideal_ring), make an ideal ring filter
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// frequency_cutoff: `f64` -> Frequency cutoff
/// min: 0, max: 1000000, default: 0.5
/// ringwidth: `f64` -> Ringwidth
/// min: 0, max: 1000000, default: 0.5
/// returns `VipsImage` - Output image
pub fn mask_ideal_ring(
    width: i32,
    height: i32,
    frequency_cutoff: f64,
    ringwidth: f64,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let frequency_cutoff_in: f64 = frequency_cutoff;
        let ringwidth_in: f64 = ringwidth;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_mask_ideal_ring(
            &mut out_out,
            width_in,
            height_in,
            frequency_cutoff_in,
            ringwidth_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskIdealRingError,
        )
    }
}

/// Options for mask_ideal_ring operation
#[derive(Clone, Debug)]
pub struct MaskIdealRingOptions {
    /// uchar: `bool` -> Output an unsigned char image
    /// default: false
    pub uchar: bool,
    /// nodc: `bool` -> Remove DC component
    /// default: false
    pub nodc: bool,
    /// reject: `bool` -> Invert the sense of the filter
    /// default: false
    pub reject: bool,
    /// optical: `bool` -> Rotate quadrants to optical space
    /// default: false
    pub optical: bool,
}

impl std::default::Default for MaskIdealRingOptions {
    fn default() -> Self {
        MaskIdealRingOptions {
            uchar: false,
            nodc: false,
            reject: false,
            optical: false,
        }
    }
}

/// VipsMaskIdealRing (mask_ideal_ring), make an ideal ring filter
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// frequency_cutoff: `f64` -> Frequency cutoff
/// min: 0, max: 1000000, default: 0.5
/// ringwidth: `f64` -> Ringwidth
/// min: 0, max: 1000000, default: 0.5
/// mask_ideal_ring_options: `&MaskIdealRingOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn mask_ideal_ring_with_opts(
    width: i32,
    height: i32,
    frequency_cutoff: f64,
    ringwidth: f64,
    mask_ideal_ring_options: &MaskIdealRingOptions,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let frequency_cutoff_in: f64 = frequency_cutoff;
        let ringwidth_in: f64 = ringwidth;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let uchar_in: i32 = if mask_ideal_ring_options.uchar { 1 } else { 0 };
        let uchar_in_name = utils::new_c_string("uchar")?;

        let nodc_in: i32 = if mask_ideal_ring_options.nodc { 1 } else { 0 };
        let nodc_in_name = utils::new_c_string("nodc")?;

        let reject_in: i32 = if mask_ideal_ring_options.reject { 1 } else { 0 };
        let reject_in_name = utils::new_c_string("reject")?;

        let optical_in: i32 = if mask_ideal_ring_options.optical { 1 } else { 0 };
        let optical_in_name = utils::new_c_string("optical")?;

        let vips_op_response = bindings::vips_mask_ideal_ring(
            &mut out_out,
            width_in,
            height_in,
            frequency_cutoff_in,
            ringwidth_in,
            uchar_in_name.as_ptr(),
            uchar_in,
            nodc_in_name.as_ptr(),
            nodc_in,
            reject_in_name.as_ptr(),
            reject_in,
            optical_in_name.as_ptr(),
            optical_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskIdealRingError,
        )
    }
}

/// VipsMaskIdealBand (mask_ideal_band), make an ideal band filter
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// frequency_cutoff_x: `f64` -> Frequency cutoff x
/// min: 0, max: 1000000, default: 0.5
/// frequency_cutoff_y: `f64` -> Frequency cutoff y
/// min: 0, max: 1000000, default: 0.5
/// radius: `f64` -> Radius of circle
/// min: 0, max: 1000000, default: 0.1
/// returns `VipsImage` - Output image
pub fn mask_ideal_band(
    width: i32,
    height: i32,
    frequency_cutoff_x: f64,
    frequency_cutoff_y: f64,
    radius: f64,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let frequency_cutoff_x_in: f64 = frequency_cutoff_x;
        let frequency_cutoff_y_in: f64 = frequency_cutoff_y;
        let radius_in: f64 = radius;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_mask_ideal_band(
            &mut out_out,
            width_in,
            height_in,
            frequency_cutoff_x_in,
            frequency_cutoff_y_in,
            radius_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskIdealBandError,
        )
    }
}

/// Options for mask_ideal_band operation
#[derive(Clone, Debug)]
pub struct MaskIdealBandOptions {
    /// uchar: `bool` -> Output an unsigned char image
    /// default: false
    pub uchar: bool,
    /// nodc: `bool` -> Remove DC component
    /// default: false
    pub nodc: bool,
    /// reject: `bool` -> Invert the sense of the filter
    /// default: false
    pub reject: bool,
    /// optical: `bool` -> Rotate quadrants to optical space
    /// default: false
    pub optical: bool,
}

impl std::default::Default for MaskIdealBandOptions {
    fn default() -> Self {
        MaskIdealBandOptions {
            uchar: false,
            nodc: false,
            reject: false,
            optical: false,
        }
    }
}

/// VipsMaskIdealBand (mask_ideal_band), make an ideal band filter
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// frequency_cutoff_x: `f64` -> Frequency cutoff x
/// min: 0, max: 1000000, default: 0.5
/// frequency_cutoff_y: `f64` -> Frequency cutoff y
/// min: 0, max: 1000000, default: 0.5
/// radius: `f64` -> Radius of circle
/// min: 0, max: 1000000, default: 0.1
/// mask_ideal_band_options: `&MaskIdealBandOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn mask_ideal_band_with_opts(
    width: i32,
    height: i32,
    frequency_cutoff_x: f64,
    frequency_cutoff_y: f64,
    radius: f64,
    mask_ideal_band_options: &MaskIdealBandOptions,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let frequency_cutoff_x_in: f64 = frequency_cutoff_x;
        let frequency_cutoff_y_in: f64 = frequency_cutoff_y;
        let radius_in: f64 = radius;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let uchar_in: i32 = if mask_ideal_band_options.uchar { 1 } else { 0 };
        let uchar_in_name = utils::new_c_string("uchar")?;

        let nodc_in: i32 = if mask_ideal_band_options.nodc { 1 } else { 0 };
        let nodc_in_name = utils::new_c_string("nodc")?;

        let reject_in: i32 = if mask_ideal_band_options.reject { 1 } else { 0 };
        let reject_in_name = utils::new_c_string("reject")?;

        let optical_in: i32 = if mask_ideal_band_options.optical { 1 } else { 0 };
        let optical_in_name = utils::new_c_string("optical")?;

        let vips_op_response = bindings::vips_mask_ideal_band(
            &mut out_out,
            width_in,
            height_in,
            frequency_cutoff_x_in,
            frequency_cutoff_y_in,
            radius_in,
            uchar_in_name.as_ptr(),
            uchar_in,
            nodc_in_name.as_ptr(),
            nodc_in,
            reject_in_name.as_ptr(),
            reject_in,
            optical_in_name.as_ptr(),
            optical_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskIdealBandError,
        )
    }
}

/// VipsMaskButterworth (mask_butterworth), make a butterworth filter
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// order: `f64` -> Filter order
/// min: 1, max: 1000000, default: 1
/// frequency_cutoff: `f64` -> Frequency cutoff
/// min: 0, max: 1000000, default: 0.5
/// amplitude_cutoff: `f64` -> Amplitude cutoff
/// min: 0, max: 1, default: 0.5
/// returns `VipsImage` - Output image
pub fn mask_butterworth(
    width: i32,
    height: i32,
    order: f64,
    frequency_cutoff: f64,
    amplitude_cutoff: f64,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let order_in: f64 = order;
        let frequency_cutoff_in: f64 = frequency_cutoff;
        let amplitude_cutoff_in: f64 = amplitude_cutoff;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_mask_butterworth(
            &mut out_out,
            width_in,
            height_in,
            order_in,
            frequency_cutoff_in,
            amplitude_cutoff_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskButterworthError,
        )
    }
}

/// Options for mask_butterworth operation
#[derive(Clone, Debug)]
pub struct MaskButterworthOptions {
    /// uchar: `bool` -> Output an unsigned char image
    /// default: false
    pub uchar: bool,
    /// nodc: `bool` -> Remove DC component
    /// default: false
    pub nodc: bool,
    /// reject: `bool` -> Invert the sense of the filter
    /// default: false
    pub reject: bool,
    /// optical: `bool` -> Rotate quadrants to optical space
    /// default: false
    pub optical: bool,
}

impl std::default::Default for MaskButterworthOptions {
    fn default() -> Self {
        MaskButterworthOptions {
            uchar: false,
            nodc: false,
            reject: false,
            optical: false,
        }
    }
}

/// VipsMaskButterworth (mask_butterworth), make a butterworth filter
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// order: `f64` -> Filter order
/// min: 1, max: 1000000, default: 1
/// frequency_cutoff: `f64` -> Frequency cutoff
/// min: 0, max: 1000000, default: 0.5
/// amplitude_cutoff: `f64` -> Amplitude cutoff
/// min: 0, max: 1, default: 0.5
/// mask_butterworth_options: `&MaskButterworthOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn mask_butterworth_with_opts(
    width: i32,
    height: i32,
    order: f64,
    frequency_cutoff: f64,
    amplitude_cutoff: f64,
    mask_butterworth_options: &MaskButterworthOptions,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let order_in: f64 = order;
        let frequency_cutoff_in: f64 = frequency_cutoff;
        let amplitude_cutoff_in: f64 = amplitude_cutoff;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let uchar_in: i32 = if mask_butterworth_options.uchar { 1 } else { 0 };
        let uchar_in_name = utils::new_c_string("uchar")?;

        let nodc_in: i32 = if mask_butterworth_options.nodc { 1 } else { 0 };
        let nodc_in_name = utils::new_c_string("nodc")?;

        let reject_in: i32 = if mask_butterworth_options.reject { 1 } else { 0 };
        let reject_in_name = utils::new_c_string("reject")?;

        let optical_in: i32 = if mask_butterworth_options.optical { 1 } else { 0 };
        let optical_in_name = utils::new_c_string("optical")?;

        let vips_op_response = bindings::vips_mask_butterworth(
            &mut out_out,
            width_in,
            height_in,
            order_in,
            frequency_cutoff_in,
            amplitude_cutoff_in,
            uchar_in_name.as_ptr(),
            uchar_in,
            nodc_in_name.as_ptr(),
            nodc_in,
            reject_in_name.as_ptr(),
            reject_in,
            optical_in_name.as_ptr(),
            optical_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskButterworthError,
        )
    }
}

/// VipsMaskButterworthRing (mask_butterworth_ring), make a butterworth ring filter
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// order: `f64` -> Filter order
/// min: 1, max: 1000000, default: 1
/// frequency_cutoff: `f64` -> Frequency cutoff
/// min: 0, max: 1000000, default: 0.5
/// amplitude_cutoff: `f64` -> Amplitude cutoff
/// min: 0, max: 1, default: 0.5
/// ringwidth: `f64` -> Ringwidth
/// min: 0, max: 1000000, default: 0.1
/// returns `VipsImage` - Output image
pub fn mask_butterworth_ring(
    width: i32,
    height: i32,
    order: f64,
    frequency_cutoff: f64,
    amplitude_cutoff: f64,
    ringwidth: f64,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let order_in: f64 = order;
        let frequency_cutoff_in: f64 = frequency_cutoff;
        let amplitude_cutoff_in: f64 = amplitude_cutoff;
        let ringwidth_in: f64 = ringwidth;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_mask_butterworth_ring(
            &mut out_out,
            width_in,
            height_in,
            order_in,
            frequency_cutoff_in,
            amplitude_cutoff_in,
            ringwidth_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskButterworthRingError,
        )
    }
}

/// Options for mask_butterworth_ring operation
#[derive(Clone, Debug)]
pub struct MaskButterworthRingOptions {
    /// uchar: `bool` -> Output an unsigned char image
    /// default: false
    pub uchar: bool,
    /// nodc: `bool` -> Remove DC component
    /// default: false
    pub nodc: bool,
    /// reject: `bool` -> Invert the sense of the filter
    /// default: false
    pub reject: bool,
    /// optical: `bool` -> Rotate quadrants to optical space
    /// default: false
    pub optical: bool,
}

impl std::default::Default for MaskButterworthRingOptions {
    fn default() -> Self {
        MaskButterworthRingOptions {
            uchar: false,
            nodc: false,
            reject: false,
            optical: false,
        }
    }
}

/// VipsMaskButterworthRing (mask_butterworth_ring), make a butterworth ring filter
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// order: `f64` -> Filter order
/// min: 1, max: 1000000, default: 1
/// frequency_cutoff: `f64` -> Frequency cutoff
/// min: 0, max: 1000000, default: 0.5
/// amplitude_cutoff: `f64` -> Amplitude cutoff
/// min: 0, max: 1, default: 0.5
/// ringwidth: `f64` -> Ringwidth
/// min: 0, max: 1000000, default: 0.1
/// mask_butterworth_ring_options: `&MaskButterworthRingOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn mask_butterworth_ring_with_opts(
    width: i32,
    height: i32,
    order: f64,
    frequency_cutoff: f64,
    amplitude_cutoff: f64,
    ringwidth: f64,
    mask_butterworth_ring_options: &MaskButterworthRingOptions,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let order_in: f64 = order;
        let frequency_cutoff_in: f64 = frequency_cutoff;
        let amplitude_cutoff_in: f64 = amplitude_cutoff;
        let ringwidth_in: f64 = ringwidth;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let uchar_in: i32 = if mask_butterworth_ring_options.uchar { 1 } else { 0 };
        let uchar_in_name = utils::new_c_string("uchar")?;

        let nodc_in: i32 = if mask_butterworth_ring_options.nodc { 1 } else { 0 };
        let nodc_in_name = utils::new_c_string("nodc")?;

        let reject_in: i32 = if mask_butterworth_ring_options.reject { 1 } else { 0 };
        let reject_in_name = utils::new_c_string("reject")?;

        let optical_in: i32 = if mask_butterworth_ring_options.optical { 1 } else { 0 };
        let optical_in_name = utils::new_c_string("optical")?;

        let vips_op_response = bindings::vips_mask_butterworth_ring(
            &mut out_out,
            width_in,
            height_in,
            order_in,
            frequency_cutoff_in,
            amplitude_cutoff_in,
            ringwidth_in,
            uchar_in_name.as_ptr(),
            uchar_in,
            nodc_in_name.as_ptr(),
            nodc_in,
            reject_in_name.as_ptr(),
            reject_in,
            optical_in_name.as_ptr(),
            optical_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskButterworthRingError,
        )
    }
}

/// VipsMaskButterworthBand (mask_butterworth_band), make a butterworth_band filter
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// order: `f64` -> Filter order
/// min: 1, max: 1000000, default: 1
/// frequency_cutoff_x: `f64` -> Frequency cutoff x
/// min: 0, max: 1000000, default: 0.5
/// frequency_cutoff_y: `f64` -> Frequency cutoff y
/// min: 0, max: 1000000, default: 0.5
/// radius: `f64` -> Radius of circle
/// min: 0, max: 1000000, default: 0.1
/// amplitude_cutoff: `f64` -> Amplitude cutoff
/// min: 0, max: 1, default: 0.5
/// returns `VipsImage` - Output image
pub fn mask_butterworth_band(
    width: i32,
    height: i32,
    order: f64,
    frequency_cutoff_x: f64,
    frequency_cutoff_y: f64,
    radius: f64,
    amplitude_cutoff: f64,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let order_in: f64 = order;
        let frequency_cutoff_x_in: f64 = frequency_cutoff_x;
        let frequency_cutoff_y_in: f64 = frequency_cutoff_y;
        let radius_in: f64 = radius;
        let amplitude_cutoff_in: f64 = amplitude_cutoff;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_mask_butterworth_band(
            &mut out_out,
            width_in,
            height_in,
            order_in,
            frequency_cutoff_x_in,
            frequency_cutoff_y_in,
            radius_in,
            amplitude_cutoff_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskButterworthBandError,
        )
    }
}

/// Options for mask_butterworth_band operation
#[derive(Clone, Debug)]
pub struct MaskButterworthBandOptions {
    /// uchar: `bool` -> Output an unsigned char image
    /// default: false
    pub uchar: bool,
    /// nodc: `bool` -> Remove DC component
    /// default: false
    pub nodc: bool,
    /// reject: `bool` -> Invert the sense of the filter
    /// default: false
    pub reject: bool,
    /// optical: `bool` -> Rotate quadrants to optical space
    /// default: false
    pub optical: bool,
}

impl std::default::Default for MaskButterworthBandOptions {
    fn default() -> Self {
        MaskButterworthBandOptions {
            uchar: false,
            nodc: false,
            reject: false,
            optical: false,
        }
    }
}

/// VipsMaskButterworthBand (mask_butterworth_band), make a butterworth_band filter
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// order: `f64` -> Filter order
/// min: 1, max: 1000000, default: 1
/// frequency_cutoff_x: `f64` -> Frequency cutoff x
/// min: 0, max: 1000000, default: 0.5
/// frequency_cutoff_y: `f64` -> Frequency cutoff y
/// min: 0, max: 1000000, default: 0.5
/// radius: `f64` -> Radius of circle
/// min: 0, max: 1000000, default: 0.1
/// amplitude_cutoff: `f64` -> Amplitude cutoff
/// min: 0, max: 1, default: 0.5
/// mask_butterworth_band_options: `&MaskButterworthBandOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn mask_butterworth_band_with_opts(
    width: i32,
    height: i32,
    order: f64,
    frequency_cutoff_x: f64,
    frequency_cutoff_y: f64,
    radius: f64,
    amplitude_cutoff: f64,
    mask_butterworth_band_options: &MaskButterworthBandOptions,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let order_in: f64 = order;
        let frequency_cutoff_x_in: f64 = frequency_cutoff_x;
        let frequency_cutoff_y_in: f64 = frequency_cutoff_y;
        let radius_in: f64 = radius;
        let amplitude_cutoff_in: f64 = amplitude_cutoff;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let uchar_in: i32 = if mask_butterworth_band_options.uchar { 1 } else { 0 };
        let uchar_in_name = utils::new_c_string("uchar")?;

        let nodc_in: i32 = if mask_butterworth_band_options.nodc { 1 } else { 0 };
        let nodc_in_name = utils::new_c_string("nodc")?;

        let reject_in: i32 = if mask_butterworth_band_options.reject { 1 } else { 0 };
        let reject_in_name = utils::new_c_string("reject")?;

        let optical_in: i32 = if mask_butterworth_band_options.optical { 1 } else { 0 };
        let optical_in_name = utils::new_c_string("optical")?;

        let vips_op_response = bindings::vips_mask_butterworth_band(
            &mut out_out,
            width_in,
            height_in,
            order_in,
            frequency_cutoff_x_in,
            frequency_cutoff_y_in,
            radius_in,
            amplitude_cutoff_in,
            uchar_in_name.as_ptr(),
            uchar_in,
            nodc_in_name.as_ptr(),
            nodc_in,
            reject_in_name.as_ptr(),
            reject_in,
            optical_in_name.as_ptr(),
            optical_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskButterworthBandError,
        )
    }
}

/// VipsMaskGaussian (mask_gaussian), make a gaussian filter
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// frequency_cutoff: `f64` -> Frequency cutoff
/// min: 0, max: 1000000, default: 0.5
/// amplitude_cutoff: `f64` -> Amplitude cutoff
/// min: 0, max: 1, default: 0.5
/// returns `VipsImage` - Output image
pub fn mask_gaussian(
    width: i32,
    height: i32,
    frequency_cutoff: f64,
    amplitude_cutoff: f64,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let frequency_cutoff_in: f64 = frequency_cutoff;
        let amplitude_cutoff_in: f64 = amplitude_cutoff;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_mask_gaussian(
            &mut out_out,
            width_in,
            height_in,
            frequency_cutoff_in,
            amplitude_cutoff_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskGaussianError,
        )
    }
}

/// Options for mask_gaussian operation
#[derive(Clone, Debug)]
pub struct MaskGaussianOptions {
    /// uchar: `bool` -> Output an unsigned char image
    /// default: false
    pub uchar: bool,
    /// nodc: `bool` -> Remove DC component
    /// default: false
    pub nodc: bool,
    /// reject: `bool` -> Invert the sense of the filter
    /// default: false
    pub reject: bool,
    /// optical: `bool` -> Rotate quadrants to optical space
    /// default: false
    pub optical: bool,
}

impl std::default::Default for MaskGaussianOptions {
    fn default() -> Self {
        MaskGaussianOptions {
            uchar: false,
            nodc: false,
            reject: false,
            optical: false,
        }
    }
}

/// VipsMaskGaussian (mask_gaussian), make a gaussian filter
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// frequency_cutoff: `f64` -> Frequency cutoff
/// min: 0, max: 1000000, default: 0.5
/// amplitude_cutoff: `f64` -> Amplitude cutoff
/// min: 0, max: 1, default: 0.5
/// mask_gaussian_options: `&MaskGaussianOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn mask_gaussian_with_opts(
    width: i32,
    height: i32,
    frequency_cutoff: f64,
    amplitude_cutoff: f64,
    mask_gaussian_options: &MaskGaussianOptions,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let frequency_cutoff_in: f64 = frequency_cutoff;
        let amplitude_cutoff_in: f64 = amplitude_cutoff;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let uchar_in: i32 = if mask_gaussian_options.uchar { 1 } else { 0 };
        let uchar_in_name = utils::new_c_string("uchar")?;

        let nodc_in: i32 = if mask_gaussian_options.nodc { 1 } else { 0 };
        let nodc_in_name = utils::new_c_string("nodc")?;

        let reject_in: i32 = if mask_gaussian_options.reject { 1 } else { 0 };
        let reject_in_name = utils::new_c_string("reject")?;

        let optical_in: i32 = if mask_gaussian_options.optical { 1 } else { 0 };
        let optical_in_name = utils::new_c_string("optical")?;

        let vips_op_response = bindings::vips_mask_gaussian(
            &mut out_out,
            width_in,
            height_in,
            frequency_cutoff_in,
            amplitude_cutoff_in,
            uchar_in_name.as_ptr(),
            uchar_in,
            nodc_in_name.as_ptr(),
            nodc_in,
            reject_in_name.as_ptr(),
            reject_in,
            optical_in_name.as_ptr(),
            optical_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskGaussianError,
        )
    }
}

/// VipsMaskGaussianRing (mask_gaussian_ring), make a gaussian ring filter
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// frequency_cutoff: `f64` -> Frequency cutoff
/// min: 0, max: 1000000, default: 0.5
/// amplitude_cutoff: `f64` -> Amplitude cutoff
/// min: 0, max: 1, default: 0.5
/// ringwidth: `f64` -> Ringwidth
/// min: 0, max: 1000000, default: 0.5
/// returns `VipsImage` - Output image
pub fn mask_gaussian_ring(
    width: i32,
    height: i32,
    frequency_cutoff: f64,
    amplitude_cutoff: f64,
    ringwidth: f64,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let frequency_cutoff_in: f64 = frequency_cutoff;
        let amplitude_cutoff_in: f64 = amplitude_cutoff;
        let ringwidth_in: f64 = ringwidth;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_mask_gaussian_ring(
            &mut out_out,
            width_in,
            height_in,
            frequency_cutoff_in,
            amplitude_cutoff_in,
            ringwidth_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskGaussianRingError,
        )
    }
}

/// Options for mask_gaussian_ring operation
#[derive(Clone, Debug)]
pub struct MaskGaussianRingOptions {
    /// uchar: `bool` -> Output an unsigned char image
    /// default: false
    pub uchar: bool,
    /// nodc: `bool` -> Remove DC component
    /// default: false
    pub nodc: bool,
    /// reject: `bool` -> Invert the sense of the filter
    /// default: false
    pub reject: bool,
    /// optical: `bool` -> Rotate quadrants to optical space
    /// default: false
    pub optical: bool,
}

impl std::default::Default for MaskGaussianRingOptions {
    fn default() -> Self {
        MaskGaussianRingOptions {
            uchar: false,
            nodc: false,
            reject: false,
            optical: false,
        }
    }
}

/// VipsMaskGaussianRing (mask_gaussian_ring), make a gaussian ring filter
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// frequency_cutoff: `f64` -> Frequency cutoff
/// min: 0, max: 1000000, default: 0.5
/// amplitude_cutoff: `f64` -> Amplitude cutoff
/// min: 0, max: 1, default: 0.5
/// ringwidth: `f64` -> Ringwidth
/// min: 0, max: 1000000, default: 0.5
/// mask_gaussian_ring_options: `&MaskGaussianRingOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn mask_gaussian_ring_with_opts(
    width: i32,
    height: i32,
    frequency_cutoff: f64,
    amplitude_cutoff: f64,
    ringwidth: f64,
    mask_gaussian_ring_options: &MaskGaussianRingOptions,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let frequency_cutoff_in: f64 = frequency_cutoff;
        let amplitude_cutoff_in: f64 = amplitude_cutoff;
        let ringwidth_in: f64 = ringwidth;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let uchar_in: i32 = if mask_gaussian_ring_options.uchar { 1 } else { 0 };
        let uchar_in_name = utils::new_c_string("uchar")?;

        let nodc_in: i32 = if mask_gaussian_ring_options.nodc { 1 } else { 0 };
        let nodc_in_name = utils::new_c_string("nodc")?;

        let reject_in: i32 = if mask_gaussian_ring_options.reject { 1 } else { 0 };
        let reject_in_name = utils::new_c_string("reject")?;

        let optical_in: i32 = if mask_gaussian_ring_options.optical { 1 } else { 0 };
        let optical_in_name = utils::new_c_string("optical")?;

        let vips_op_response = bindings::vips_mask_gaussian_ring(
            &mut out_out,
            width_in,
            height_in,
            frequency_cutoff_in,
            amplitude_cutoff_in,
            ringwidth_in,
            uchar_in_name.as_ptr(),
            uchar_in,
            nodc_in_name.as_ptr(),
            nodc_in,
            reject_in_name.as_ptr(),
            reject_in,
            optical_in_name.as_ptr(),
            optical_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskGaussianRingError,
        )
    }
}

/// VipsMaskGaussianBand (mask_gaussian_band), make a gaussian filter
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// frequency_cutoff_x: `f64` -> Frequency cutoff x
/// min: 0, max: 1000000, default: 0.5
/// frequency_cutoff_y: `f64` -> Frequency cutoff y
/// min: 0, max: 1000000, default: 0.5
/// radius: `f64` -> Radius of circle
/// min: 0, max: 1000000, default: 0.1
/// amplitude_cutoff: `f64` -> Amplitude cutoff
/// min: 0, max: 1, default: 0.5
/// returns `VipsImage` - Output image
pub fn mask_gaussian_band(
    width: i32,
    height: i32,
    frequency_cutoff_x: f64,
    frequency_cutoff_y: f64,
    radius: f64,
    amplitude_cutoff: f64,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let frequency_cutoff_x_in: f64 = frequency_cutoff_x;
        let frequency_cutoff_y_in: f64 = frequency_cutoff_y;
        let radius_in: f64 = radius;
        let amplitude_cutoff_in: f64 = amplitude_cutoff;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_mask_gaussian_band(
            &mut out_out,
            width_in,
            height_in,
            frequency_cutoff_x_in,
            frequency_cutoff_y_in,
            radius_in,
            amplitude_cutoff_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskGaussianBandError,
        )
    }
}

/// Options for mask_gaussian_band operation
#[derive(Clone, Debug)]
pub struct MaskGaussianBandOptions {
    /// uchar: `bool` -> Output an unsigned char image
    /// default: false
    pub uchar: bool,
    /// nodc: `bool` -> Remove DC component
    /// default: false
    pub nodc: bool,
    /// reject: `bool` -> Invert the sense of the filter
    /// default: false
    pub reject: bool,
    /// optical: `bool` -> Rotate quadrants to optical space
    /// default: false
    pub optical: bool,
}

impl std::default::Default for MaskGaussianBandOptions {
    fn default() -> Self {
        MaskGaussianBandOptions {
            uchar: false,
            nodc: false,
            reject: false,
            optical: false,
        }
    }
}

/// VipsMaskGaussianBand (mask_gaussian_band), make a gaussian filter
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// frequency_cutoff_x: `f64` -> Frequency cutoff x
/// min: 0, max: 1000000, default: 0.5
/// frequency_cutoff_y: `f64` -> Frequency cutoff y
/// min: 0, max: 1000000, default: 0.5
/// radius: `f64` -> Radius of circle
/// min: 0, max: 1000000, default: 0.1
/// amplitude_cutoff: `f64` -> Amplitude cutoff
/// min: 0, max: 1, default: 0.5
/// mask_gaussian_band_options: `&MaskGaussianBandOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn mask_gaussian_band_with_opts(
    width: i32,
    height: i32,
    frequency_cutoff_x: f64,
    frequency_cutoff_y: f64,
    radius: f64,
    amplitude_cutoff: f64,
    mask_gaussian_band_options: &MaskGaussianBandOptions,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let frequency_cutoff_x_in: f64 = frequency_cutoff_x;
        let frequency_cutoff_y_in: f64 = frequency_cutoff_y;
        let radius_in: f64 = radius;
        let amplitude_cutoff_in: f64 = amplitude_cutoff;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let uchar_in: i32 = if mask_gaussian_band_options.uchar { 1 } else { 0 };
        let uchar_in_name = utils::new_c_string("uchar")?;

        let nodc_in: i32 = if mask_gaussian_band_options.nodc { 1 } else { 0 };
        let nodc_in_name = utils::new_c_string("nodc")?;

        let reject_in: i32 = if mask_gaussian_band_options.reject { 1 } else { 0 };
        let reject_in_name = utils::new_c_string("reject")?;

        let optical_in: i32 = if mask_gaussian_band_options.optical { 1 } else { 0 };
        let optical_in_name = utils::new_c_string("optical")?;

        let vips_op_response = bindings::vips_mask_gaussian_band(
            &mut out_out,
            width_in,
            height_in,
            frequency_cutoff_x_in,
            frequency_cutoff_y_in,
            radius_in,
            amplitude_cutoff_in,
            uchar_in_name.as_ptr(),
            uchar_in,
            nodc_in_name.as_ptr(),
            nodc_in,
            reject_in_name.as_ptr(),
            reject_in,
            optical_in_name.as_ptr(),
            optical_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskGaussianBandError,
        )
    }
}

/// VipsMaskFractal (mask_fractal), make fractal filter
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// fractal_dimension: `f64` -> Fractal dimension
/// min: 2, max: 3, default: 2.5
/// returns `VipsImage` - Output image
pub fn mask_fractal(width: i32, height: i32, fractal_dimension: f64) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let fractal_dimension_in: f64 = fractal_dimension;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_mask_fractal(
            &mut out_out,
            width_in,
            height_in,
            fractal_dimension_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskFractalError,
        )
    }
}

/// Options for mask_fractal operation
#[derive(Clone, Debug)]
pub struct MaskFractalOptions {
    /// uchar: `bool` -> Output an unsigned char image
    /// default: false
    pub uchar: bool,
    /// nodc: `bool` -> Remove DC component
    /// default: false
    pub nodc: bool,
    /// reject: `bool` -> Invert the sense of the filter
    /// default: false
    pub reject: bool,
    /// optical: `bool` -> Rotate quadrants to optical space
    /// default: false
    pub optical: bool,
}

impl std::default::Default for MaskFractalOptions {
    fn default() -> Self {
        MaskFractalOptions {
            uchar: false,
            nodc: false,
            reject: false,
            optical: false,
        }
    }
}

/// VipsMaskFractal (mask_fractal), make fractal filter
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// fractal_dimension: `f64` -> Fractal dimension
/// min: 2, max: 3, default: 2.5
/// mask_fractal_options: `&MaskFractalOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn mask_fractal_with_opts(
    width: i32,
    height: i32,
    fractal_dimension: f64,
    mask_fractal_options: &MaskFractalOptions,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let fractal_dimension_in: f64 = fractal_dimension;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let uchar_in: i32 = if mask_fractal_options.uchar { 1 } else { 0 };
        let uchar_in_name = utils::new_c_string("uchar")?;

        let nodc_in: i32 = if mask_fractal_options.nodc { 1 } else { 0 };
        let nodc_in_name = utils::new_c_string("nodc")?;

        let reject_in: i32 = if mask_fractal_options.reject { 1 } else { 0 };
        let reject_in_name = utils::new_c_string("reject")?;

        let optical_in: i32 = if mask_fractal_options.optical { 1 } else { 0 };
        let optical_in_name = utils::new_c_string("optical")?;

        let vips_op_response = bindings::vips_mask_fractal(
            &mut out_out,
            width_in,
            height_in,
            fractal_dimension_in,
            uchar_in_name.as_ptr(),
            uchar_in,
            nodc_in_name.as_ptr(),
            nodc_in,
            reject_in_name.as_ptr(),
            reject_in,
            optical_in_name.as_ptr(),
            optical_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskFractalError,
        )
    }
}

/// VipsBuildlut (buildlut), build a look-up table
/// inp: `&VipsImage` -> Matrix of XY coordinates
/// returns `VipsImage` - Output image
pub fn buildlut(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_buildlut(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BuildlutError,
        )
    }
}

/// VipsInvertlut (invertlut), build an inverted look-up table
/// inp: `&VipsImage` -> Matrix of XY coordinates
/// returns `VipsImage` - Output image
pub fn invertlut(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_invertlut(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::InvertlutError,
        )
    }
}

/// Options for invertlut operation
#[derive(Clone, Debug)]
pub struct InvertlutOptions {
    /// size: `i32` -> LUT size to generate
    /// min: 1, max: 1000000, default: 256
    pub size: i32,
}

impl std::default::Default for InvertlutOptions {
    fn default() -> Self {
        InvertlutOptions {
            size: i32::from(256),
        }
    }
}

/// VipsInvertlut (invertlut), build an inverted look-up table
/// inp: `&VipsImage` -> Matrix of XY coordinates
/// invertlut_options: `&InvertlutOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn invertlut_with_opts(
    inp: &VipsImage,
    invertlut_options: &InvertlutOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let size_in: i32 = invertlut_options.size;
        let size_in_name = utils::new_c_string("size")?;

        let vips_op_response = bindings::vips_invertlut(
            inp_in,
            &mut out_out,
            size_in_name.as_ptr(),
            size_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::InvertlutError,
        )
    }
}

/// VipsTonelut (tonelut), build a look-up table

/// returns `VipsImage` - Output image
pub fn tonelut() -> Result<VipsImage> {
    unsafe {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_tonelut(
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::TonelutError,
        )
    }
}

/// Options for tonelut operation
#[derive(Clone, Debug)]
pub struct TonelutOptions {
    /// in_max: `i32` -> Size of LUT to build
    /// min: 1, max: 65535, default: 32767
    pub in_max: i32,
    /// out_max: `i32` -> Maximum value in output LUT
    /// min: 1, max: 65535, default: 32767
    pub out_max: i32,
    /// lb: `f64` -> Lowest value in output
    /// min: 0, max: 100, default: 0
    pub lb: f64,
    /// lw: `f64` -> Highest value in output
    /// min: 0, max: 100, default: 100
    pub lw: f64,
    /// ps: `f64` -> Position of shadow
    /// min: 0, max: 1, default: 0.2
    pub ps: f64,
    /// pm: `f64` -> Position of mid-tones
    /// min: 0, max: 1, default: 0.5
    pub pm: f64,
    /// ph: `f64` -> Position of highlights
    /// min: 0, max: 1, default: 0.8
    pub ph: f64,
    /// s: `f64` -> Adjust shadows by this much
    /// min: -30, max: 30, default: 0
    pub s: f64,
    /// m: `f64` -> Adjust mid-tones by this much
    /// min: -30, max: 30, default: 0
    pub m: f64,
    /// h: `f64` -> Adjust highlights by this much
    /// min: -30, max: 30, default: 0
    pub h: f64,
}

impl std::default::Default for TonelutOptions {
    fn default() -> Self {
        TonelutOptions {
            in_max: i32::from(32767),
            out_max: i32::from(32767),
            lb: f64::from(0),
            lw: f64::from(100),
            ps: f64::from(0.2),
            pm: f64::from(0.5),
            ph: f64::from(0.8),
            s: f64::from(0),
            m: f64::from(0),
            h: f64::from(0),
        }
    }
}

/// VipsTonelut (tonelut), build a look-up table

/// tonelut_options: `&TonelutOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn tonelut_with_opts(tonelut_options: &TonelutOptions) -> Result<VipsImage> {
    unsafe {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let in_max_in: i32 = tonelut_options.in_max;
        let in_max_in_name = utils::new_c_string("in-max")?;

        let out_max_in: i32 = tonelut_options.out_max;
        let out_max_in_name = utils::new_c_string("out-max")?;

        let lb_in: f64 = tonelut_options.lb;
        let lb_in_name = utils::new_c_string("Lb")?;

        let lw_in: f64 = tonelut_options.lw;
        let lw_in_name = utils::new_c_string("Lw")?;

        let ps_in: f64 = tonelut_options.ps;
        let ps_in_name = utils::new_c_string("Ps")?;

        let pm_in: f64 = tonelut_options.pm;
        let pm_in_name = utils::new_c_string("Pm")?;

        let ph_in: f64 = tonelut_options.ph;
        let ph_in_name = utils::new_c_string("Ph")?;

        let s_in: f64 = tonelut_options.s;
        let s_in_name = utils::new_c_string("S")?;

        let m_in: f64 = tonelut_options.m;
        let m_in_name = utils::new_c_string("M")?;

        let h_in: f64 = tonelut_options.h;
        let h_in_name = utils::new_c_string("H")?;

        let vips_op_response = bindings::vips_tonelut(
            &mut out_out,
            in_max_in_name.as_ptr(),
            in_max_in,
            out_max_in_name.as_ptr(),
            out_max_in,
            lb_in_name.as_ptr(),
            lb_in,
            lw_in_name.as_ptr(),
            lw_in,
            ps_in_name.as_ptr(),
            ps_in,
            pm_in_name.as_ptr(),
            pm_in,
            ph_in_name.as_ptr(),
            ph_in,
            s_in_name.as_ptr(),
            s_in,
            m_in_name.as_ptr(),
            m_in,
            h_in_name.as_ptr(),
            h_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::TonelutError,
        )
    }
}

/// VipsIdentity (identity), make a 1D image where pixel values are indexes

/// returns `VipsImage` - Output image
pub fn identity() -> Result<VipsImage> {
    unsafe {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_identity(
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::IdentityError,
        )
    }
}

/// Options for identity operation
#[derive(Clone, Debug)]
pub struct IdentityOptions {
    /// bands: `i32` -> Number of bands in LUT
    /// min: 1, max: 100000, default: 1
    pub bands: i32,
    /// ushort: `bool` -> Create a 16-bit LUT
    /// default: false
    pub ushort: bool,
    /// size: `i32` -> Size of 16-bit LUT
    /// min: 1, max: 65536, default: 65536
    pub size: i32,
}

impl std::default::Default for IdentityOptions {
    fn default() -> Self {
        IdentityOptions {
            bands: i32::from(1),
            ushort: false,
            size: i32::from(65536),
        }
    }
}

/// VipsIdentity (identity), make a 1D image where pixel values are indexes

/// identity_options: `&IdentityOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn identity_with_opts(identity_options: &IdentityOptions) -> Result<VipsImage> {
    unsafe {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let bands_in: i32 = identity_options.bands;
        let bands_in_name = utils::new_c_string("bands")?;

        let ushort_in: i32 = if identity_options.ushort { 1 } else { 0 };
        let ushort_in_name = utils::new_c_string("ushort")?;

        let size_in: i32 = identity_options.size;
        let size_in_name = utils::new_c_string("size")?;

        let vips_op_response = bindings::vips_identity(
            &mut out_out,
            bands_in_name.as_ptr(),
            bands_in,
            ushort_in_name.as_ptr(),
            ushort_in,
            size_in_name.as_ptr(),
            size_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::IdentityError,
        )
    }
}

/// VipsFractsurf (fractsurf), make a fractal surface
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 64
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 64
/// fractal_dimension: `f64` -> Fractal dimension
/// min: 2, max: 3, default: 2.5
/// returns `VipsImage` - Output image
pub fn fractsurf(width: i32, height: i32, fractal_dimension: f64) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let fractal_dimension_in: f64 = fractal_dimension;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_fractsurf(
            &mut out_out,
            width_in,
            height_in,
            fractal_dimension_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::FractsurfError,
        )
    }
}

/// VipsWorley (worley), make a worley noise image
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// returns `VipsImage` - Output image
pub fn worley(width: i32, height: i32) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_worley(
            &mut out_out,
            width_in,
            height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::WorleyError,
        )
    }
}

/// Options for worley operation
#[derive(Clone, Debug)]
pub struct WorleyOptions {
    /// cell_size: `i32` -> Size of Worley cells
    /// min: 1, max: 10000000, default: 256
    pub cell_size: i32,
    /// seed: `i32` -> Random number seed
    /// min: -2147483648, max: 2147483647, default: 0
    pub seed: i32,
}

impl std::default::Default for WorleyOptions {
    fn default() -> Self {
        WorleyOptions {
            cell_size: i32::from(256),
            seed: i32::from(0),
        }
    }
}

/// VipsWorley (worley), make a worley noise image
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// worley_options: `&WorleyOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn worley_with_opts(
    width: i32,
    height: i32,
    worley_options: &WorleyOptions,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let cell_size_in: i32 = worley_options.cell_size;
        let cell_size_in_name = utils::new_c_string("cell-size")?;

        let seed_in: i32 = worley_options.seed;
        let seed_in_name = utils::new_c_string("seed")?;

        let vips_op_response = bindings::vips_worley(
            &mut out_out,
            width_in,
            height_in,
            cell_size_in_name.as_ptr(),
            cell_size_in,
            seed_in_name.as_ptr(),
            seed_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::WorleyError,
        )
    }
}

/// VipsPerlin (perlin), make a perlin noise image
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// returns `VipsImage` - Output image
pub fn perlin(width: i32, height: i32) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_perlin(
            &mut out_out,
            width_in,
            height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PerlinError,
        )
    }
}

/// Options for perlin operation
#[derive(Clone, Debug)]
pub struct PerlinOptions {
    /// cell_size: `i32` -> Size of Perlin cells
    /// min: 1, max: 10000000, default: 256
    pub cell_size: i32,
    /// uchar: `bool` -> Output an unsigned char image
    /// default: false
    pub uchar: bool,
    /// seed: `i32` -> Random number seed
    /// min: -2147483648, max: 2147483647, default: 0
    pub seed: i32,
}

impl std::default::Default for PerlinOptions {
    fn default() -> Self {
        PerlinOptions {
            cell_size: i32::from(256),
            uchar: false,
            seed: i32::from(0),
        }
    }
}

/// VipsPerlin (perlin), make a perlin noise image
/// width: `i32` -> Image width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Image height in pixels
/// min: 1, max: 10000000, default: 1
/// perlin_options: `&PerlinOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn perlin_with_opts(
    width: i32,
    height: i32,
    perlin_options: &PerlinOptions,
) -> Result<VipsImage> {
    unsafe {
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let cell_size_in: i32 = perlin_options.cell_size;
        let cell_size_in_name = utils::new_c_string("cell-size")?;

        let uchar_in: i32 = if perlin_options.uchar { 1 } else { 0 };
        let uchar_in_name = utils::new_c_string("uchar")?;

        let seed_in: i32 = perlin_options.seed;
        let seed_in_name = utils::new_c_string("seed")?;

        let vips_op_response = bindings::vips_perlin(
            &mut out_out,
            width_in,
            height_in,
            cell_size_in_name.as_ptr(),
            cell_size_in,
            uchar_in_name.as_ptr(),
            uchar_in,
            seed_in_name.as_ptr(),
            seed_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PerlinError,
        )
    }
}

/// VipsSwitch (switch), find the index of the first non-zero pixel in tests
/// tests: `&mut [VipsImage]` -> Table of images to test
/// returns `VipsImage` - Output image
pub fn switch(tests: &mut [VipsImage]) -> Result<VipsImage> {
    unsafe {
        let (tests_len, mut tests_in) = {
            let len = tests.len();
            let mut input = Vec::new();
            for img in tests {
                input.push(img.ctx)
            }
            (
                len as i32,
                input,
            )
        };
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_switch(
            tests_in.as_mut_ptr(),
            &mut out_out,
            tests_len,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SwitchError,
        )
    }
}

/// VipsForeignLoadCsvFile (csvload), load csv (.csv), priority=0, untrusted, get_flags, get_flags_filename, header, load
/// filename: `&str` -> Filename to load from
/// returns `VipsImage` - Output image
pub fn csvload(filename: &str) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_csvload(
            filename_in.as_ptr(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CsvloadError,
        )
    }
}

/// Options for csvload operation
#[derive(Clone, Debug)]
pub struct CsvloadOptions {
    /// skip: `i32` -> Skip this many lines at the start of the file
    /// min: 0, max: 10000000, default: 0
    pub skip: i32,
    /// lines: `i32` -> Read this many lines from the file
    /// min: -1, max: 10000000, default: 0
    pub lines: i32,
    /// whitespace: `String` -> Set of whitespace characters
    pub whitespace: String,
    /// separator: `String` -> Set of separator characters
    pub separator: String,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for CsvloadOptions {
    fn default() -> Self {
        CsvloadOptions {
            skip: i32::from(0),
            lines: i32::from(0),
            whitespace: String::new(),
            separator: String::new(),
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadCsvFile (csvload), load csv (.csv), priority=0, untrusted, get_flags, get_flags_filename, header, load
/// filename: `&str` -> Filename to load from
/// csvload_options: `&CsvloadOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn csvload_with_opts(filename: &str, csvload_options: &CsvloadOptions) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let skip_in: i32 = csvload_options.skip;
        let skip_in_name = utils::new_c_string("skip")?;

        let lines_in: i32 = csvload_options.lines;
        let lines_in_name = utils::new_c_string("lines")?;

        let whitespace_in: CString = utils::new_c_string(&csvload_options.whitespace)?;
        let whitespace_in_name = utils::new_c_string("whitespace")?;

        let separator_in: CString = utils::new_c_string(&csvload_options.separator)?;
        let separator_in_name = utils::new_c_string("separator")?;

        let flags_in: i32 = csvload_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if csvload_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = csvload_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = csvload_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if csvload_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_csvload(
            filename_in.as_ptr(),
            &mut out_out,
            skip_in_name.as_ptr(),
            skip_in,
            lines_in_name.as_ptr(),
            lines_in,
            whitespace_in_name.as_ptr(),
            whitespace_in.as_ptr(),
            separator_in_name.as_ptr(),
            separator_in.as_ptr(),
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CsvloadError,
        )
    }
}

/// VipsForeignLoadCsvSource (csvload_source), load csv, priority=0, untrusted, is_a_source, get_flags, header, load
/// source: `&VipsSource` -> Source to load from
/// returns `VipsImage` - Output image
pub fn csvload_source(source: &VipsSource) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_csvload_source(
            source_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CsvloadSourceError,
        )
    }
}

/// Options for csvload_source operation
#[derive(Clone, Debug)]
pub struct CsvloadSourceOptions {
    /// skip: `i32` -> Skip this many lines at the start of the file
    /// min: 0, max: 10000000, default: 0
    pub skip: i32,
    /// lines: `i32` -> Read this many lines from the file
    /// min: -1, max: 10000000, default: 0
    pub lines: i32,
    /// whitespace: `String` -> Set of whitespace characters
    pub whitespace: String,
    /// separator: `String` -> Set of separator characters
    pub separator: String,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for CsvloadSourceOptions {
    fn default() -> Self {
        CsvloadSourceOptions {
            skip: i32::from(0),
            lines: i32::from(0),
            whitespace: String::new(),
            separator: String::new(),
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadCsvSource (csvload_source), load csv, priority=0, untrusted, is_a_source, get_flags, header, load
/// source: `&VipsSource` -> Source to load from
/// csvload_source_options: `&CsvloadSourceOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn csvload_source_with_opts(
    source: &VipsSource,
    csvload_source_options: &CsvloadSourceOptions,
) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let skip_in: i32 = csvload_source_options.skip;
        let skip_in_name = utils::new_c_string("skip")?;

        let lines_in: i32 = csvload_source_options.lines;
        let lines_in_name = utils::new_c_string("lines")?;

        let whitespace_in: CString = utils::new_c_string(&csvload_source_options.whitespace)?;
        let whitespace_in_name = utils::new_c_string("whitespace")?;

        let separator_in: CString = utils::new_c_string(&csvload_source_options.separator)?;
        let separator_in_name = utils::new_c_string("separator")?;

        let flags_in: i32 = csvload_source_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if csvload_source_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = csvload_source_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = csvload_source_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if csvload_source_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_csvload_source(
            source_in,
            &mut out_out,
            skip_in_name.as_ptr(),
            skip_in,
            lines_in_name.as_ptr(),
            lines_in,
            whitespace_in_name.as_ptr(),
            whitespace_in.as_ptr(),
            separator_in_name.as_ptr(),
            separator_in.as_ptr(),
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CsvloadSourceError,
        )
    }
}

/// VipsForeignLoadMatrixFile (matrixload), load matrix (.mat), priority=0, is_a, get_flags, get_flags_filename, header, load
/// filename: `&str` -> Filename to load from
/// returns `VipsImage` - Output image
pub fn matrixload(filename: &str) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_matrixload(
            filename_in.as_ptr(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MatrixloadError,
        )
    }
}

/// Options for matrixload operation
#[derive(Clone, Debug)]
pub struct MatrixloadOptions {
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for MatrixloadOptions {
    fn default() -> Self {
        MatrixloadOptions {
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadMatrixFile (matrixload), load matrix (.mat), priority=0, is_a, get_flags, get_flags_filename, header, load
/// filename: `&str` -> Filename to load from
/// matrixload_options: `&MatrixloadOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn matrixload_with_opts(
    filename: &str,
    matrixload_options: &MatrixloadOptions,
) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let flags_in: i32 = matrixload_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if matrixload_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = matrixload_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = matrixload_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if matrixload_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_matrixload(
            filename_in.as_ptr(),
            &mut out_out,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MatrixloadError,
        )
    }
}

/// VipsForeignLoadMatrixSource (matrixload_source), load matrix, priority=0, is_a_source, get_flags, header, load
/// source: `&VipsSource` -> Source to load from
/// returns `VipsImage` - Output image
pub fn matrixload_source(source: &VipsSource) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_matrixload_source(
            source_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MatrixloadSourceError,
        )
    }
}

/// Options for matrixload_source operation
#[derive(Clone, Debug)]
pub struct MatrixloadSourceOptions {
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for MatrixloadSourceOptions {
    fn default() -> Self {
        MatrixloadSourceOptions {
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadMatrixSource (matrixload_source), load matrix, priority=0, is_a_source, get_flags, header, load
/// source: `&VipsSource` -> Source to load from
/// matrixload_source_options: `&MatrixloadSourceOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn matrixload_source_with_opts(
    source: &VipsSource,
    matrixload_source_options: &MatrixloadSourceOptions,
) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let flags_in: i32 = matrixload_source_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if matrixload_source_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = matrixload_source_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = matrixload_source_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if matrixload_source_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_matrixload_source(
            source_in,
            &mut out_out,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MatrixloadSourceError,
        )
    }
}

/// VipsForeignLoadRaw (rawload), load raw data from a file, priority=0, untrusted, get_flags, get_flags_filename, header
/// filename: `&str` -> Filename to load from
/// width: `i32` -> Image width in pixels
/// min: 0, max: 10000000, default: 0
/// height: `i32` -> Image height in pixels
/// min: 0, max: 10000000, default: 0
/// bands: `i32` -> Number of bands in image
/// min: 0, max: 10000000, default: 0
/// returns `VipsImage` - Output image
pub fn rawload(filename: &str, width: i32, height: i32, bands: i32) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let width_in: i32 = width;
        let height_in: i32 = height;
        let bands_in: i32 = bands;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_rawload(
            filename_in.as_ptr(),
            &mut out_out,
            width_in,
            height_in,
            bands_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RawloadError,
        )
    }
}

/// Options for rawload operation
#[derive(Clone, Debug)]
pub struct RawloadOptions {
    /// offset: `u64` -> Offset in bytes from start of file
    /// min: 0, max: 100000000000, default: 0
    pub offset: u64,
    /// format: `BandFormat` -> Pixel format in image
    ///  `Notset` -> VIPS_FORMAT_NOTSET = -1
    ///  `Uchar` -> VIPS_FORMAT_UCHAR = 0 [DEFAULT]
    ///  `Char` -> VIPS_FORMAT_CHAR = 1
    ///  `Ushort` -> VIPS_FORMAT_USHORT = 2
    ///  `Short` -> VIPS_FORMAT_SHORT = 3
    ///  `Uint` -> VIPS_FORMAT_UINT = 4
    ///  `Int` -> VIPS_FORMAT_INT = 5
    ///  `Float` -> VIPS_FORMAT_FLOAT = 6
    ///  `Complex` -> VIPS_FORMAT_COMPLEX = 7
    ///  `Double` -> VIPS_FORMAT_DOUBLE = 8
    ///  `Dpcomplex` -> VIPS_FORMAT_DPCOMPLEX = 9
    ///  `Last` -> VIPS_FORMAT_LAST = 10
    pub format: BandFormat,
    /// interpretation: `Interpretation` -> Pixel interpretation
    ///  `Error` -> VIPS_INTERPRETATION_ERROR = -1
    ///  `Multiband` -> VIPS_INTERPRETATION_MULTIBAND = 0 [DEFAULT]
    ///  `BW` -> VIPS_INTERPRETATION_B_W = 1
    ///  `Histogram` -> VIPS_INTERPRETATION_HISTOGRAM = 10
    ///  `Xyz` -> VIPS_INTERPRETATION_XYZ = 12
    ///  `Lab` -> VIPS_INTERPRETATION_LAB = 13
    ///  `Cmyk` -> VIPS_INTERPRETATION_CMYK = 15
    ///  `Labq` -> VIPS_INTERPRETATION_LABQ = 16
    ///  `Rgb` -> VIPS_INTERPRETATION_RGB = 17
    ///  `Cmc` -> VIPS_INTERPRETATION_CMC = 18
    ///  `Lch` -> VIPS_INTERPRETATION_LCH = 19
    ///  `Lab` -> VIPS_INTERPRETATION_LABS = 21
    ///  `Srgb` -> VIPS_INTERPRETATION_sRGB = 22
    ///  `Yxy` -> VIPS_INTERPRETATION_YXY = 23
    ///  `Fourier` -> VIPS_INTERPRETATION_FOURIER = 24
    ///  `Rgb16` -> VIPS_INTERPRETATION_RGB16 = 25
    ///  `Grey16` -> VIPS_INTERPRETATION_GREY16 = 26
    ///  `Matrix` -> VIPS_INTERPRETATION_MATRIX = 27
    ///  `Scrgb` -> VIPS_INTERPRETATION_scRGB = 28
    ///  `Hsv` -> VIPS_INTERPRETATION_HSV = 29
    ///  `Last` -> VIPS_INTERPRETATION_LAST = 30
    pub interpretation: Interpretation,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for RawloadOptions {
    fn default() -> Self {
        RawloadOptions {
            offset: 0,
            format: BandFormat::Uchar,
            interpretation: Interpretation::Multiband,
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadRaw (rawload), load raw data from a file, priority=0, untrusted, get_flags, get_flags_filename, header
/// filename: `&str` -> Filename to load from
/// width: `i32` -> Image width in pixels
/// min: 0, max: 10000000, default: 0
/// height: `i32` -> Image height in pixels
/// min: 0, max: 10000000, default: 0
/// bands: `i32` -> Number of bands in image
/// min: 0, max: 10000000, default: 0
/// rawload_options: `&RawloadOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn rawload_with_opts(
    filename: &str,
    width: i32,
    height: i32,
    bands: i32,
    rawload_options: &RawloadOptions,
) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let width_in: i32 = width;
        let height_in: i32 = height;
        let bands_in: i32 = bands;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let offset_in: u64 = rawload_options.offset;
        let offset_in_name = utils::new_c_string("offset")?;

        let format_in: i32 = rawload_options.format as i32;
        let format_in_name = utils::new_c_string("format")?;

        let interpretation_in: i32 = rawload_options.interpretation as i32;
        let interpretation_in_name = utils::new_c_string("interpretation")?;

        let flags_in: i32 = rawload_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if rawload_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = rawload_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = rawload_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if rawload_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_rawload(
            filename_in.as_ptr(),
            &mut out_out,
            width_in,
            height_in,
            bands_in,
            offset_in_name.as_ptr(),
            offset_in,
            format_in_name.as_ptr(),
            format_in,
            interpretation_in_name.as_ptr(),
            interpretation_in,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RawloadError,
        )
    }
}

/// VipsForeignLoadVipsFile (vipsload), load vips from file (.v, .vips), priority=200, untrusted, is_a, get_flags, get_flags_filename, header
/// filename: `&str` -> Filename to load from
/// returns `VipsImage` - Output image
pub fn vipsload(filename: &str) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_vipsload(
            filename_in.as_ptr(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::VipsloadError,
        )
    }
}

/// Options for vipsload operation
#[derive(Clone, Debug)]
pub struct VipsloadOptions {
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for VipsloadOptions {
    fn default() -> Self {
        VipsloadOptions {
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadVipsFile (vipsload), load vips from file (.v, .vips), priority=200, untrusted, is_a, get_flags, get_flags_filename, header
/// filename: `&str` -> Filename to load from
/// vipsload_options: `&VipsloadOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn vipsload_with_opts(filename: &str, vipsload_options: &VipsloadOptions) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let flags_in: i32 = vipsload_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if vipsload_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = vipsload_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = vipsload_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if vipsload_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_vipsload(
            filename_in.as_ptr(),
            &mut out_out,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::VipsloadError,
        )
    }
}

/// VipsForeignLoadVipsSource (vipsload_source), load vips from source, priority=200, untrusted, is_a_source, get_flags, get_flags_filename, header
/// source: `&VipsSource` -> Source to load from
/// returns `VipsImage` - Output image
pub fn vipsload_source(source: &VipsSource) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_vipsload_source(
            source_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::VipsloadSourceError,
        )
    }
}

/// Options for vipsload_source operation
#[derive(Clone, Debug)]
pub struct VipsloadSourceOptions {
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for VipsloadSourceOptions {
    fn default() -> Self {
        VipsloadSourceOptions {
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadVipsSource (vipsload_source), load vips from source, priority=200, untrusted, is_a_source, get_flags, get_flags_filename, header
/// source: `&VipsSource` -> Source to load from
/// vipsload_source_options: `&VipsloadSourceOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn vipsload_source_with_opts(
    source: &VipsSource,
    vipsload_source_options: &VipsloadSourceOptions,
) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let flags_in: i32 = vipsload_source_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if vipsload_source_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = vipsload_source_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = vipsload_source_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if vipsload_source_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_vipsload_source(
            source_in,
            &mut out_out,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::VipsloadSourceError,
        )
    }
}

/// VipsForeignLoadAnalyze (analyzeload), load an Analyze6 image (.img, .hdr), priority=-50, untrusted, is_a, get_flags, get_flags_filename, header, load
/// filename: `&str` -> Filename to load from
/// returns `VipsImage` - Output image
pub fn analyzeload(filename: &str) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_analyzeload(
            filename_in.as_ptr(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::AnalyzeloadError,
        )
    }
}

/// Options for analyzeload operation
#[derive(Clone, Debug)]
pub struct AnalyzeloadOptions {
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for AnalyzeloadOptions {
    fn default() -> Self {
        AnalyzeloadOptions {
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadAnalyze (analyzeload), load an Analyze6 image (.img, .hdr), priority=-50, untrusted, is_a, get_flags, get_flags_filename, header, load
/// filename: `&str` -> Filename to load from
/// analyzeload_options: `&AnalyzeloadOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn analyzeload_with_opts(
    filename: &str,
    analyzeload_options: &AnalyzeloadOptions,
) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let flags_in: i32 = analyzeload_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if analyzeload_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = analyzeload_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = analyzeload_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if analyzeload_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_analyzeload(
            filename_in.as_ptr(),
            &mut out_out,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::AnalyzeloadError,
        )
    }
}

/// VipsForeignLoadPpmFile (ppmload), load ppm from file (.pbm, .pgm, .ppm, .pfm, .pnm), priority=200, untrusted, is_a, get_flags, header, load
/// filename: `&str` -> Filename to load from
/// returns `VipsImage` - Output image
pub fn ppmload(filename: &str) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_ppmload(
            filename_in.as_ptr(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PpmloadError,
        )
    }
}

/// Options for ppmload operation
#[derive(Clone, Debug)]
pub struct PpmloadOptions {
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for PpmloadOptions {
    fn default() -> Self {
        PpmloadOptions {
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadPpmFile (ppmload), load ppm from file (.pbm, .pgm, .ppm, .pfm, .pnm), priority=200, untrusted, is_a, get_flags, header, load
/// filename: `&str` -> Filename to load from
/// ppmload_options: `&PpmloadOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn ppmload_with_opts(filename: &str, ppmload_options: &PpmloadOptions) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let flags_in: i32 = ppmload_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if ppmload_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = ppmload_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = ppmload_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if ppmload_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_ppmload(
            filename_in.as_ptr(),
            &mut out_out,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PpmloadError,
        )
    }
}

/// VipsForeignLoadPpmSource (ppmload_source), load ppm base class (.pbm, .pgm, .ppm, .pfm, .pnm), priority=200, untrusted, is_a_source, get_flags, header, load
/// source: `&VipsSource` -> Source to load from
/// returns `VipsImage` - Output image
pub fn ppmload_source(source: &VipsSource) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_ppmload_source(
            source_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PpmloadSourceError,
        )
    }
}

/// Options for ppmload_source operation
#[derive(Clone, Debug)]
pub struct PpmloadSourceOptions {
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for PpmloadSourceOptions {
    fn default() -> Self {
        PpmloadSourceOptions {
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadPpmSource (ppmload_source), load ppm base class (.pbm, .pgm, .ppm, .pfm, .pnm), priority=200, untrusted, is_a_source, get_flags, header, load
/// source: `&VipsSource` -> Source to load from
/// ppmload_source_options: `&PpmloadSourceOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn ppmload_source_with_opts(
    source: &VipsSource,
    ppmload_source_options: &PpmloadSourceOptions,
) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let flags_in: i32 = ppmload_source_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if ppmload_source_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = ppmload_source_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = ppmload_source_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if ppmload_source_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_ppmload_source(
            source_in,
            &mut out_out,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PpmloadSourceError,
        )
    }
}

/// VipsForeignLoadRadFile (radload), load a Radiance image from a file (.hdr), priority=-50, untrusted, is_a, get_flags, get_flags_filename, header, load
/// filename: `&str` -> Filename to load from
/// returns `VipsImage` - Output image
pub fn radload(filename: &str) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_radload(
            filename_in.as_ptr(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RadloadError,
        )
    }
}

/// Options for radload operation
#[derive(Clone, Debug)]
pub struct RadloadOptions {
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for RadloadOptions {
    fn default() -> Self {
        RadloadOptions {
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadRadFile (radload), load a Radiance image from a file (.hdr), priority=-50, untrusted, is_a, get_flags, get_flags_filename, header, load
/// filename: `&str` -> Filename to load from
/// radload_options: `&RadloadOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn radload_with_opts(filename: &str, radload_options: &RadloadOptions) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let flags_in: i32 = radload_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if radload_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = radload_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = radload_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if radload_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_radload(
            filename_in.as_ptr(),
            &mut out_out,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RadloadError,
        )
    }
}

/// VipsForeignLoadRadBuffer (radload_buffer), load rad from buffer, priority=-50, untrusted, is_a_buffer, get_flags, get_flags_filename, header, load
/// buffer: `&[u8]` -> Buffer to load from
/// returns `VipsImage` - Output image
pub fn radload_buffer(buffer: &[u8]) -> Result<VipsImage> {
    unsafe {
        let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_radload_buffer(
            buffer_in,
            buffer.len(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RadloadBufferError,
        )
    }
}

/// Options for radload_buffer operation
#[derive(Clone, Debug)]
pub struct RadloadBufferOptions {
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for RadloadBufferOptions {
    fn default() -> Self {
        RadloadBufferOptions {
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadRadBuffer (radload_buffer), load rad from buffer, priority=-50, untrusted, is_a_buffer, get_flags, get_flags_filename, header, load
/// buffer: `&[u8]` -> Buffer to load from
/// radload_buffer_options: `&RadloadBufferOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn radload_buffer_with_opts(
    buffer: &[u8],
    radload_buffer_options: &RadloadBufferOptions,
) -> Result<VipsImage> {
    unsafe {
        let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let flags_in: i32 = radload_buffer_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if radload_buffer_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = radload_buffer_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = radload_buffer_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if radload_buffer_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_radload_buffer(
            buffer_in,
            buffer.len(),
            &mut out_out,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RadloadBufferError,
        )
    }
}

/// VipsForeignLoadRadSource (radload_source), load rad from source, priority=-50, untrusted, is_a_source, get_flags, get_flags_filename, header, load
/// source: `&VipsSource` -> Source to load from
/// returns `VipsImage` - Output image
pub fn radload_source(source: &VipsSource) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_radload_source(
            source_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RadloadSourceError,
        )
    }
}

/// Options for radload_source operation
#[derive(Clone, Debug)]
pub struct RadloadSourceOptions {
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for RadloadSourceOptions {
    fn default() -> Self {
        RadloadSourceOptions {
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadRadSource (radload_source), load rad from source, priority=-50, untrusted, is_a_source, get_flags, get_flags_filename, header, load
/// source: `&VipsSource` -> Source to load from
/// radload_source_options: `&RadloadSourceOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn radload_source_with_opts(
    source: &VipsSource,
    radload_source_options: &RadloadSourceOptions,
) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let flags_in: i32 = radload_source_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if radload_source_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = radload_source_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = radload_source_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if radload_source_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_radload_source(
            source_in,
            &mut out_out,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RadloadSourceError,
        )
    }
}

/// VipsForeignLoadSvgFile (svgload), load SVG with rsvg (.svg, .svgz, .svg.gz), priority=-5, untrusted, is_a, get_flags, get_flags_filename, header, load
/// filename: `&str` -> Filename to load from
/// returns `VipsImage` - Output image
pub fn svgload(filename: &str) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_svgload(
            filename_in.as_ptr(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SvgloadError,
        )
    }
}

/// Options for svgload operation
#[derive(Clone, Debug)]
pub struct SvgloadOptions {
    /// dpi: `f64` -> Render at this DPI
    /// min: 0.001, max: 100000, default: 72
    pub dpi: f64,
    /// scale: `f64` -> Scale output by this factor
    /// min: 0.001, max: 100000, default: 1
    pub scale: f64,
    /// unlimited: `bool` -> Allow SVG of any size
    /// default: false
    pub unlimited: bool,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for SvgloadOptions {
    fn default() -> Self {
        SvgloadOptions {
            dpi: f64::from(72),
            scale: f64::from(1),
            unlimited: false,
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadSvgFile (svgload), load SVG with rsvg (.svg, .svgz, .svg.gz), priority=-5, untrusted, is_a, get_flags, get_flags_filename, header, load
/// filename: `&str` -> Filename to load from
/// svgload_options: `&SvgloadOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn svgload_with_opts(filename: &str, svgload_options: &SvgloadOptions) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let dpi_in: f64 = svgload_options.dpi;
        let dpi_in_name = utils::new_c_string("dpi")?;

        let scale_in: f64 = svgload_options.scale;
        let scale_in_name = utils::new_c_string("scale")?;

        let unlimited_in: i32 = if svgload_options.unlimited { 1 } else { 0 };
        let unlimited_in_name = utils::new_c_string("unlimited")?;

        let flags_in: i32 = svgload_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if svgload_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = svgload_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = svgload_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if svgload_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_svgload(
            filename_in.as_ptr(),
            &mut out_out,
            dpi_in_name.as_ptr(),
            dpi_in,
            scale_in_name.as_ptr(),
            scale_in,
            unlimited_in_name.as_ptr(),
            unlimited_in,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SvgloadError,
        )
    }
}

/// VipsForeignLoadSvgBuffer (svgload_buffer), load SVG with rsvg, priority=-5, untrusted, is_a_buffer, get_flags, get_flags_filename, header, load
/// buffer: `&[u8]` -> Buffer to load from
/// returns `VipsImage` - Output image
pub fn svgload_buffer(buffer: &[u8]) -> Result<VipsImage> {
    unsafe {
        let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_svgload_buffer(
            buffer_in,
            buffer.len(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SvgloadBufferError,
        )
    }
}

/// Options for svgload_buffer operation
#[derive(Clone, Debug)]
pub struct SvgloadBufferOptions {
    /// dpi: `f64` -> Render at this DPI
    /// min: 0.001, max: 100000, default: 72
    pub dpi: f64,
    /// scale: `f64` -> Scale output by this factor
    /// min: 0.001, max: 100000, default: 1
    pub scale: f64,
    /// unlimited: `bool` -> Allow SVG of any size
    /// default: false
    pub unlimited: bool,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for SvgloadBufferOptions {
    fn default() -> Self {
        SvgloadBufferOptions {
            dpi: f64::from(72),
            scale: f64::from(1),
            unlimited: false,
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadSvgBuffer (svgload_buffer), load SVG with rsvg, priority=-5, untrusted, is_a_buffer, get_flags, get_flags_filename, header, load
/// buffer: `&[u8]` -> Buffer to load from
/// svgload_buffer_options: `&SvgloadBufferOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn svgload_buffer_with_opts(
    buffer: &[u8],
    svgload_buffer_options: &SvgloadBufferOptions,
) -> Result<VipsImage> {
    unsafe {
        let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let dpi_in: f64 = svgload_buffer_options.dpi;
        let dpi_in_name = utils::new_c_string("dpi")?;

        let scale_in: f64 = svgload_buffer_options.scale;
        let scale_in_name = utils::new_c_string("scale")?;

        let unlimited_in: i32 = if svgload_buffer_options.unlimited { 1 } else { 0 };
        let unlimited_in_name = utils::new_c_string("unlimited")?;

        let flags_in: i32 = svgload_buffer_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if svgload_buffer_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = svgload_buffer_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = svgload_buffer_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if svgload_buffer_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_svgload_buffer(
            buffer_in,
            buffer.len(),
            &mut out_out,
            dpi_in_name.as_ptr(),
            dpi_in,
            scale_in_name.as_ptr(),
            scale_in,
            unlimited_in_name.as_ptr(),
            unlimited_in,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SvgloadBufferError,
        )
    }
}

/// VipsForeignLoadNsgifFile (gifload), load GIF with libnsgif (.gif), priority=50, is_a, get_flags, get_flags_filename, header, load
/// filename: `&str` -> Filename to load from
/// returns `VipsImage` - Output image
pub fn gifload(filename: &str) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_gifload(
            filename_in.as_ptr(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GifloadError,
        )
    }
}

/// Options for gifload operation
#[derive(Clone, Debug)]
pub struct GifloadOptions {
    /// n: `i32` -> Number of pages to load, -1 for all
    /// min: -1, max: 100000, default: 1
    pub n: i32,
    /// page: `i32` -> First page to load
    /// min: 0, max: 100000, default: 0
    pub page: i32,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for GifloadOptions {
    fn default() -> Self {
        GifloadOptions {
            n: i32::from(1),
            page: i32::from(0),
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadNsgifFile (gifload), load GIF with libnsgif (.gif), priority=50, is_a, get_flags, get_flags_filename, header, load
/// filename: `&str` -> Filename to load from
/// gifload_options: `&GifloadOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn gifload_with_opts(filename: &str, gifload_options: &GifloadOptions) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let n_in: i32 = gifload_options.n;
        let n_in_name = utils::new_c_string("n")?;

        let page_in: i32 = gifload_options.page;
        let page_in_name = utils::new_c_string("page")?;

        let flags_in: i32 = gifload_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if gifload_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = gifload_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = gifload_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if gifload_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_gifload(
            filename_in.as_ptr(),
            &mut out_out,
            n_in_name.as_ptr(),
            n_in,
            page_in_name.as_ptr(),
            page_in,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GifloadError,
        )
    }
}

/// VipsForeignLoadNsgifBuffer (gifload_buffer), load GIF with libnsgif, priority=50, is_a_buffer, get_flags, get_flags_filename, header, load
/// buffer: `&[u8]` -> Buffer to load from
/// returns `VipsImage` - Output image
pub fn gifload_buffer(buffer: &[u8]) -> Result<VipsImage> {
    unsafe {
        let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_gifload_buffer(
            buffer_in,
            buffer.len(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GifloadBufferError,
        )
    }
}

/// Options for gifload_buffer operation
#[derive(Clone, Debug)]
pub struct GifloadBufferOptions {
    /// n: `i32` -> Number of pages to load, -1 for all
    /// min: -1, max: 100000, default: 1
    pub n: i32,
    /// page: `i32` -> First page to load
    /// min: 0, max: 100000, default: 0
    pub page: i32,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for GifloadBufferOptions {
    fn default() -> Self {
        GifloadBufferOptions {
            n: i32::from(1),
            page: i32::from(0),
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadNsgifBuffer (gifload_buffer), load GIF with libnsgif, priority=50, is_a_buffer, get_flags, get_flags_filename, header, load
/// buffer: `&[u8]` -> Buffer to load from
/// gifload_buffer_options: `&GifloadBufferOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn gifload_buffer_with_opts(
    buffer: &[u8],
    gifload_buffer_options: &GifloadBufferOptions,
) -> Result<VipsImage> {
    unsafe {
        let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let n_in: i32 = gifload_buffer_options.n;
        let n_in_name = utils::new_c_string("n")?;

        let page_in: i32 = gifload_buffer_options.page;
        let page_in_name = utils::new_c_string("page")?;

        let flags_in: i32 = gifload_buffer_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if gifload_buffer_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = gifload_buffer_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = gifload_buffer_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if gifload_buffer_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_gifload_buffer(
            buffer_in,
            buffer.len(),
            &mut out_out,
            n_in_name.as_ptr(),
            n_in,
            page_in_name.as_ptr(),
            page_in,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GifloadBufferError,
        )
    }
}

/// VipsForeignLoadNsgifSource (gifload_source), load gif from source, priority=50, is_a_source, get_flags, get_flags_filename, header, load
/// source: `&VipsSource` -> Source to load from
/// returns `VipsImage` - Output image
pub fn gifload_source(source: &VipsSource) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_gifload_source(
            source_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GifloadSourceError,
        )
    }
}

/// Options for gifload_source operation
#[derive(Clone, Debug)]
pub struct GifloadSourceOptions {
    /// n: `i32` -> Number of pages to load, -1 for all
    /// min: -1, max: 100000, default: 1
    pub n: i32,
    /// page: `i32` -> First page to load
    /// min: 0, max: 100000, default: 0
    pub page: i32,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for GifloadSourceOptions {
    fn default() -> Self {
        GifloadSourceOptions {
            n: i32::from(1),
            page: i32::from(0),
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadNsgifSource (gifload_source), load gif from source, priority=50, is_a_source, get_flags, get_flags_filename, header, load
/// source: `&VipsSource` -> Source to load from
/// gifload_source_options: `&GifloadSourceOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn gifload_source_with_opts(
    source: &VipsSource,
    gifload_source_options: &GifloadSourceOptions,
) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let n_in: i32 = gifload_source_options.n;
        let n_in_name = utils::new_c_string("n")?;

        let page_in: i32 = gifload_source_options.page;
        let page_in_name = utils::new_c_string("page")?;

        let flags_in: i32 = gifload_source_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if gifload_source_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = gifload_source_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = gifload_source_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if gifload_source_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_gifload_source(
            source_in,
            &mut out_out,
            n_in_name.as_ptr(),
            n_in,
            page_in_name.as_ptr(),
            page_in,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GifloadSourceError,
        )
    }
}

/// VipsForeignLoadPngFile (pngload), load png from file (.png), priority=200, is_a, get_flags, get_flags_filename, header, load
/// filename: `&str` -> Filename to load from
/// returns `VipsImage` - Output image
pub fn pngload(filename: &str) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_pngload(
            filename_in.as_ptr(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PngloadError,
        )
    }
}

/// Options for pngload operation
#[derive(Clone, Debug)]
pub struct PngloadOptions {
    /// unlimited: `bool` -> Remove all denial of service limits
    /// default: false
    pub unlimited: bool,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for PngloadOptions {
    fn default() -> Self {
        PngloadOptions {
            unlimited: false,
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadPngFile (pngload), load png from file (.png), priority=200, is_a, get_flags, get_flags_filename, header, load
/// filename: `&str` -> Filename to load from
/// pngload_options: `&PngloadOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn pngload_with_opts(filename: &str, pngload_options: &PngloadOptions) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let unlimited_in: i32 = if pngload_options.unlimited { 1 } else { 0 };
        let unlimited_in_name = utils::new_c_string("unlimited")?;

        let flags_in: i32 = pngload_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if pngload_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = pngload_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = pngload_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if pngload_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_pngload(
            filename_in.as_ptr(),
            &mut out_out,
            unlimited_in_name.as_ptr(),
            unlimited_in,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PngloadError,
        )
    }
}

/// VipsForeignLoadPngBuffer (pngload_buffer), load png from buffer, priority=200, is_a_buffer, get_flags, get_flags_filename, header, load
/// buffer: `&[u8]` -> Buffer to load from
/// returns `VipsImage` - Output image
pub fn pngload_buffer(buffer: &[u8]) -> Result<VipsImage> {
    unsafe {
        let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_pngload_buffer(
            buffer_in,
            buffer.len(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PngloadBufferError,
        )
    }
}

/// Options for pngload_buffer operation
#[derive(Clone, Debug)]
pub struct PngloadBufferOptions {
    /// unlimited: `bool` -> Remove all denial of service limits
    /// default: false
    pub unlimited: bool,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for PngloadBufferOptions {
    fn default() -> Self {
        PngloadBufferOptions {
            unlimited: false,
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadPngBuffer (pngload_buffer), load png from buffer, priority=200, is_a_buffer, get_flags, get_flags_filename, header, load
/// buffer: `&[u8]` -> Buffer to load from
/// pngload_buffer_options: `&PngloadBufferOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn pngload_buffer_with_opts(
    buffer: &[u8],
    pngload_buffer_options: &PngloadBufferOptions,
) -> Result<VipsImage> {
    unsafe {
        let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let unlimited_in: i32 = if pngload_buffer_options.unlimited { 1 } else { 0 };
        let unlimited_in_name = utils::new_c_string("unlimited")?;

        let flags_in: i32 = pngload_buffer_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if pngload_buffer_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = pngload_buffer_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = pngload_buffer_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if pngload_buffer_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_pngload_buffer(
            buffer_in,
            buffer.len(),
            &mut out_out,
            unlimited_in_name.as_ptr(),
            unlimited_in,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PngloadBufferError,
        )
    }
}

/// VipsForeignLoadPngSource (pngload_source), load png from source, priority=200, is_a_source, get_flags, get_flags_filename, header, load
/// source: `&VipsSource` -> Source to load from
/// returns `VipsImage` - Output image
pub fn pngload_source(source: &VipsSource) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_pngload_source(
            source_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PngloadSourceError,
        )
    }
}

/// Options for pngload_source operation
#[derive(Clone, Debug)]
pub struct PngloadSourceOptions {
    /// unlimited: `bool` -> Remove all denial of service limits
    /// default: false
    pub unlimited: bool,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for PngloadSourceOptions {
    fn default() -> Self {
        PngloadSourceOptions {
            unlimited: false,
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadPngSource (pngload_source), load png from source, priority=200, is_a_source, get_flags, get_flags_filename, header, load
/// source: `&VipsSource` -> Source to load from
/// pngload_source_options: `&PngloadSourceOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn pngload_source_with_opts(
    source: &VipsSource,
    pngload_source_options: &PngloadSourceOptions,
) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let unlimited_in: i32 = if pngload_source_options.unlimited { 1 } else { 0 };
        let unlimited_in_name = utils::new_c_string("unlimited")?;

        let flags_in: i32 = pngload_source_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if pngload_source_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = pngload_source_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = pngload_source_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if pngload_source_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_pngload_source(
            source_in,
            &mut out_out,
            unlimited_in_name.as_ptr(),
            unlimited_in,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PngloadSourceError,
        )
    }
}

/// VipsForeignLoadJpegFile (jpegload), load jpeg from file (.jpg, .jpeg, .jpe), priority=50, is_a, get_flags, get_flags_filename, header, load
/// filename: `&str` -> Filename to load from
/// returns `VipsImage` - Output image
pub fn jpegload(filename: &str) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_jpegload(
            filename_in.as_ptr(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::JpegloadError,
        )
    }
}

/// Options for jpegload operation
#[derive(Clone, Debug)]
pub struct JpegloadOptions {
    /// shrink: `i32` -> Shrink factor on load
    /// min: 1, max: 16, default: 1
    pub shrink: i32,
    /// autorotate: `bool` -> Rotate image using exif orientation
    /// default: false
    pub autorotate: bool,
    /// unlimited: `bool` -> Remove all denial of service limits
    /// default: false
    pub unlimited: bool,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for JpegloadOptions {
    fn default() -> Self {
        JpegloadOptions {
            shrink: i32::from(1),
            autorotate: false,
            unlimited: false,
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadJpegFile (jpegload), load jpeg from file (.jpg, .jpeg, .jpe), priority=50, is_a, get_flags, get_flags_filename, header, load
/// filename: `&str` -> Filename to load from
/// jpegload_options: `&JpegloadOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn jpegload_with_opts(filename: &str, jpegload_options: &JpegloadOptions) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let shrink_in: i32 = jpegload_options.shrink;
        let shrink_in_name = utils::new_c_string("shrink")?;

        let autorotate_in: i32 = if jpegload_options.autorotate { 1 } else { 0 };
        let autorotate_in_name = utils::new_c_string("autorotate")?;

        let unlimited_in: i32 = if jpegload_options.unlimited { 1 } else { 0 };
        let unlimited_in_name = utils::new_c_string("unlimited")?;

        let flags_in: i32 = jpegload_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if jpegload_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = jpegload_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = jpegload_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if jpegload_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_jpegload(
            filename_in.as_ptr(),
            &mut out_out,
            shrink_in_name.as_ptr(),
            shrink_in,
            autorotate_in_name.as_ptr(),
            autorotate_in,
            unlimited_in_name.as_ptr(),
            unlimited_in,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::JpegloadError,
        )
    }
}

/// VipsForeignLoadJpegBuffer (jpegload_buffer), load jpeg from buffer, priority=50, is_a_buffer, get_flags, get_flags_filename, header, load
/// buffer: `&[u8]` -> Buffer to load from
/// returns `VipsImage` - Output image
pub fn jpegload_buffer(buffer: &[u8]) -> Result<VipsImage> {
    unsafe {
        let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_jpegload_buffer(
            buffer_in,
            buffer.len(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::JpegloadBufferError,
        )
    }
}

/// Options for jpegload_buffer operation
#[derive(Clone, Debug)]
pub struct JpegloadBufferOptions {
    /// shrink: `i32` -> Shrink factor on load
    /// min: 1, max: 16, default: 1
    pub shrink: i32,
    /// autorotate: `bool` -> Rotate image using exif orientation
    /// default: false
    pub autorotate: bool,
    /// unlimited: `bool` -> Remove all denial of service limits
    /// default: false
    pub unlimited: bool,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for JpegloadBufferOptions {
    fn default() -> Self {
        JpegloadBufferOptions {
            shrink: i32::from(1),
            autorotate: false,
            unlimited: false,
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadJpegBuffer (jpegload_buffer), load jpeg from buffer, priority=50, is_a_buffer, get_flags, get_flags_filename, header, load
/// buffer: `&[u8]` -> Buffer to load from
/// jpegload_buffer_options: `&JpegloadBufferOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn jpegload_buffer_with_opts(
    buffer: &[u8],
    jpegload_buffer_options: &JpegloadBufferOptions,
) -> Result<VipsImage> {
    unsafe {
        let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let shrink_in: i32 = jpegload_buffer_options.shrink;
        let shrink_in_name = utils::new_c_string("shrink")?;

        let autorotate_in: i32 = if jpegload_buffer_options.autorotate { 1 } else { 0 };
        let autorotate_in_name = utils::new_c_string("autorotate")?;

        let unlimited_in: i32 = if jpegload_buffer_options.unlimited { 1 } else { 0 };
        let unlimited_in_name = utils::new_c_string("unlimited")?;

        let flags_in: i32 = jpegload_buffer_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if jpegload_buffer_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = jpegload_buffer_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = jpegload_buffer_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if jpegload_buffer_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_jpegload_buffer(
            buffer_in,
            buffer.len(),
            &mut out_out,
            shrink_in_name.as_ptr(),
            shrink_in,
            autorotate_in_name.as_ptr(),
            autorotate_in,
            unlimited_in_name.as_ptr(),
            unlimited_in,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::JpegloadBufferError,
        )
    }
}

/// VipsForeignLoadWebpFile (webpload), load webp from file (.webp), priority=200, is_a, get_flags, get_flags_filename, header, load
/// filename: `&str` -> Filename to load from
/// returns `VipsImage` - Output image
pub fn webpload(filename: &str) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_webpload(
            filename_in.as_ptr(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::WebploadError,
        )
    }
}

/// Options for webpload operation
#[derive(Clone, Debug)]
pub struct WebploadOptions {
    /// page: `i32` -> First page to load
    /// min: 0, max: 100000, default: 0
    pub page: i32,
    /// n: `i32` -> Number of pages to load, -1 for all
    /// min: -1, max: 100000, default: 1
    pub n: i32,
    /// scale: `f64` -> Factor to scale by
    /// min: 0, max: 1024, default: 1
    pub scale: f64,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for WebploadOptions {
    fn default() -> Self {
        WebploadOptions {
            page: i32::from(0),
            n: i32::from(1),
            scale: f64::from(1),
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadWebpFile (webpload), load webp from file (.webp), priority=200, is_a, get_flags, get_flags_filename, header, load
/// filename: `&str` -> Filename to load from
/// webpload_options: `&WebploadOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn webpload_with_opts(filename: &str, webpload_options: &WebploadOptions) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let page_in: i32 = webpload_options.page;
        let page_in_name = utils::new_c_string("page")?;

        let n_in: i32 = webpload_options.n;
        let n_in_name = utils::new_c_string("n")?;

        let scale_in: f64 = webpload_options.scale;
        let scale_in_name = utils::new_c_string("scale")?;

        let flags_in: i32 = webpload_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if webpload_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = webpload_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = webpload_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if webpload_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_webpload(
            filename_in.as_ptr(),
            &mut out_out,
            page_in_name.as_ptr(),
            page_in,
            n_in_name.as_ptr(),
            n_in,
            scale_in_name.as_ptr(),
            scale_in,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::WebploadError,
        )
    }
}

/// VipsForeignLoadWebpBuffer (webpload_buffer), load webp from buffer, priority=200, is_a_buffer, get_flags, get_flags_filename, header, load
/// buffer: `&[u8]` -> Buffer to load from
/// returns `VipsImage` - Output image
pub fn webpload_buffer(buffer: &[u8]) -> Result<VipsImage> {
    unsafe {
        let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_webpload_buffer(
            buffer_in,
            buffer.len(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::WebploadBufferError,
        )
    }
}

/// Options for webpload_buffer operation
#[derive(Clone, Debug)]
pub struct WebploadBufferOptions {
    /// page: `i32` -> First page to load
    /// min: 0, max: 100000, default: 0
    pub page: i32,
    /// n: `i32` -> Number of pages to load, -1 for all
    /// min: -1, max: 100000, default: 1
    pub n: i32,
    /// scale: `f64` -> Factor to scale by
    /// min: 0, max: 1024, default: 1
    pub scale: f64,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for WebploadBufferOptions {
    fn default() -> Self {
        WebploadBufferOptions {
            page: i32::from(0),
            n: i32::from(1),
            scale: f64::from(1),
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadWebpBuffer (webpload_buffer), load webp from buffer, priority=200, is_a_buffer, get_flags, get_flags_filename, header, load
/// buffer: `&[u8]` -> Buffer to load from
/// webpload_buffer_options: `&WebploadBufferOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn webpload_buffer_with_opts(
    buffer: &[u8],
    webpload_buffer_options: &WebploadBufferOptions,
) -> Result<VipsImage> {
    unsafe {
        let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let page_in: i32 = webpload_buffer_options.page;
        let page_in_name = utils::new_c_string("page")?;

        let n_in: i32 = webpload_buffer_options.n;
        let n_in_name = utils::new_c_string("n")?;

        let scale_in: f64 = webpload_buffer_options.scale;
        let scale_in_name = utils::new_c_string("scale")?;

        let flags_in: i32 = webpload_buffer_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if webpload_buffer_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = webpload_buffer_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = webpload_buffer_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if webpload_buffer_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_webpload_buffer(
            buffer_in,
            buffer.len(),
            &mut out_out,
            page_in_name.as_ptr(),
            page_in,
            n_in_name.as_ptr(),
            n_in,
            scale_in_name.as_ptr(),
            scale_in,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::WebploadBufferError,
        )
    }
}

/// VipsForeignLoadWebpSource (webpload_source), load webp from source, priority=200, is_a_source, get_flags, get_flags_filename, header, load
/// source: `&VipsSource` -> Source to load from
/// returns `VipsImage` - Output image
pub fn webpload_source(source: &VipsSource) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_webpload_source(
            source_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::WebploadSourceError,
        )
    }
}

/// Options for webpload_source operation
#[derive(Clone, Debug)]
pub struct WebploadSourceOptions {
    /// page: `i32` -> First page to load
    /// min: 0, max: 100000, default: 0
    pub page: i32,
    /// n: `i32` -> Number of pages to load, -1 for all
    /// min: -1, max: 100000, default: 1
    pub n: i32,
    /// scale: `f64` -> Factor to scale by
    /// min: 0, max: 1024, default: 1
    pub scale: f64,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for WebploadSourceOptions {
    fn default() -> Self {
        WebploadSourceOptions {
            page: i32::from(0),
            n: i32::from(1),
            scale: f64::from(1),
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadWebpSource (webpload_source), load webp from source, priority=200, is_a_source, get_flags, get_flags_filename, header, load
/// source: `&VipsSource` -> Source to load from
/// webpload_source_options: `&WebploadSourceOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn webpload_source_with_opts(
    source: &VipsSource,
    webpload_source_options: &WebploadSourceOptions,
) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let page_in: i32 = webpload_source_options.page;
        let page_in_name = utils::new_c_string("page")?;

        let n_in: i32 = webpload_source_options.n;
        let n_in_name = utils::new_c_string("n")?;

        let scale_in: f64 = webpload_source_options.scale;
        let scale_in_name = utils::new_c_string("scale")?;

        let flags_in: i32 = webpload_source_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if webpload_source_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = webpload_source_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = webpload_source_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if webpload_source_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_webpload_source(
            source_in,
            &mut out_out,
            page_in_name.as_ptr(),
            page_in,
            n_in_name.as_ptr(),
            n_in,
            scale_in_name.as_ptr(),
            scale_in,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::WebploadSourceError,
        )
    }
}

/// VipsForeignLoadTiffFile (tiffload), load tiff from file (.tif, .tiff), priority=50, is_a, get_flags, get_flags_filename, header, load
/// filename: `&str` -> Filename to load from
/// returns `VipsImage` - Output image
pub fn tiffload(filename: &str) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_tiffload(
            filename_in.as_ptr(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::TiffloadError,
        )
    }
}

/// Options for tiffload operation
#[derive(Clone, Debug)]
pub struct TiffloadOptions {
    /// page: `i32` -> First page to load
    /// min: 0, max: 100000, default: 0
    pub page: i32,
    /// subifd: `i32` -> Subifd index
    /// min: -1, max: 100000, default: -1
    pub subifd: i32,
    /// n: `i32` -> Number of pages to load, -1 for all
    /// min: -1, max: 100000, default: 1
    pub n: i32,
    /// autorotate: `bool` -> Rotate image using orientation tag
    /// default: false
    pub autorotate: bool,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for TiffloadOptions {
    fn default() -> Self {
        TiffloadOptions {
            page: i32::from(0),
            subifd: i32::from(-1),
            n: i32::from(1),
            autorotate: false,
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadTiffFile (tiffload), load tiff from file (.tif, .tiff), priority=50, is_a, get_flags, get_flags_filename, header, load
/// filename: `&str` -> Filename to load from
/// tiffload_options: `&TiffloadOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn tiffload_with_opts(filename: &str, tiffload_options: &TiffloadOptions) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let page_in: i32 = tiffload_options.page;
        let page_in_name = utils::new_c_string("page")?;

        let subifd_in: i32 = tiffload_options.subifd;
        let subifd_in_name = utils::new_c_string("subifd")?;

        let n_in: i32 = tiffload_options.n;
        let n_in_name = utils::new_c_string("n")?;

        let autorotate_in: i32 = if tiffload_options.autorotate { 1 } else { 0 };
        let autorotate_in_name = utils::new_c_string("autorotate")?;

        let flags_in: i32 = tiffload_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if tiffload_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = tiffload_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = tiffload_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if tiffload_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_tiffload(
            filename_in.as_ptr(),
            &mut out_out,
            page_in_name.as_ptr(),
            page_in,
            subifd_in_name.as_ptr(),
            subifd_in,
            n_in_name.as_ptr(),
            n_in,
            autorotate_in_name.as_ptr(),
            autorotate_in,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::TiffloadError,
        )
    }
}

/// VipsForeignLoadTiffBuffer (tiffload_buffer), load tiff from buffer, priority=50, is_a_buffer, get_flags, get_flags_filename, header, load
/// buffer: `&[u8]` -> Buffer to load from
/// returns `VipsImage` - Output image
pub fn tiffload_buffer(buffer: &[u8]) -> Result<VipsImage> {
    unsafe {
        let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_tiffload_buffer(
            buffer_in,
            buffer.len(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::TiffloadBufferError,
        )
    }
}

/// Options for tiffload_buffer operation
#[derive(Clone, Debug)]
pub struct TiffloadBufferOptions {
    /// page: `i32` -> First page to load
    /// min: 0, max: 100000, default: 0
    pub page: i32,
    /// subifd: `i32` -> Subifd index
    /// min: -1, max: 100000, default: -1
    pub subifd: i32,
    /// n: `i32` -> Number of pages to load, -1 for all
    /// min: -1, max: 100000, default: 1
    pub n: i32,
    /// autorotate: `bool` -> Rotate image using orientation tag
    /// default: false
    pub autorotate: bool,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for TiffloadBufferOptions {
    fn default() -> Self {
        TiffloadBufferOptions {
            page: i32::from(0),
            subifd: i32::from(-1),
            n: i32::from(1),
            autorotate: false,
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadTiffBuffer (tiffload_buffer), load tiff from buffer, priority=50, is_a_buffer, get_flags, get_flags_filename, header, load
/// buffer: `&[u8]` -> Buffer to load from
/// tiffload_buffer_options: `&TiffloadBufferOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn tiffload_buffer_with_opts(
    buffer: &[u8],
    tiffload_buffer_options: &TiffloadBufferOptions,
) -> Result<VipsImage> {
    unsafe {
        let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let page_in: i32 = tiffload_buffer_options.page;
        let page_in_name = utils::new_c_string("page")?;

        let subifd_in: i32 = tiffload_buffer_options.subifd;
        let subifd_in_name = utils::new_c_string("subifd")?;

        let n_in: i32 = tiffload_buffer_options.n;
        let n_in_name = utils::new_c_string("n")?;

        let autorotate_in: i32 = if tiffload_buffer_options.autorotate { 1 } else { 0 };
        let autorotate_in_name = utils::new_c_string("autorotate")?;

        let flags_in: i32 = tiffload_buffer_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if tiffload_buffer_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = tiffload_buffer_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = tiffload_buffer_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if tiffload_buffer_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_tiffload_buffer(
            buffer_in,
            buffer.len(),
            &mut out_out,
            page_in_name.as_ptr(),
            page_in,
            subifd_in_name.as_ptr(),
            subifd_in,
            n_in_name.as_ptr(),
            n_in,
            autorotate_in_name.as_ptr(),
            autorotate_in,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::TiffloadBufferError,
        )
    }
}

/// VipsForeignLoadTiffSource (tiffload_source), load tiff from source, priority=50, is_a_source, get_flags, get_flags_filename, header, load
/// source: `&VipsSource` -> Source to load from
/// returns `VipsImage` - Output image
pub fn tiffload_source(source: &VipsSource) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_tiffload_source(
            source_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::TiffloadSourceError,
        )
    }
}

/// Options for tiffload_source operation
#[derive(Clone, Debug)]
pub struct TiffloadSourceOptions {
    /// page: `i32` -> First page to load
    /// min: 0, max: 100000, default: 0
    pub page: i32,
    /// subifd: `i32` -> Subifd index
    /// min: -1, max: 100000, default: -1
    pub subifd: i32,
    /// n: `i32` -> Number of pages to load, -1 for all
    /// min: -1, max: 100000, default: 1
    pub n: i32,
    /// autorotate: `bool` -> Rotate image using orientation tag
    /// default: false
    pub autorotate: bool,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for TiffloadSourceOptions {
    fn default() -> Self {
        TiffloadSourceOptions {
            page: i32::from(0),
            subifd: i32::from(-1),
            n: i32::from(1),
            autorotate: false,
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadTiffSource (tiffload_source), load tiff from source, priority=50, is_a_source, get_flags, get_flags_filename, header, load
/// source: `&VipsSource` -> Source to load from
/// tiffload_source_options: `&TiffloadSourceOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn tiffload_source_with_opts(
    source: &VipsSource,
    tiffload_source_options: &TiffloadSourceOptions,
) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let page_in: i32 = tiffload_source_options.page;
        let page_in_name = utils::new_c_string("page")?;

        let subifd_in: i32 = tiffload_source_options.subifd;
        let subifd_in_name = utils::new_c_string("subifd")?;

        let n_in: i32 = tiffload_source_options.n;
        let n_in_name = utils::new_c_string("n")?;

        let autorotate_in: i32 = if tiffload_source_options.autorotate { 1 } else { 0 };
        let autorotate_in_name = utils::new_c_string("autorotate")?;

        let flags_in: i32 = tiffload_source_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if tiffload_source_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = tiffload_source_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = tiffload_source_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if tiffload_source_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_tiffload_source(
            source_in,
            &mut out_out,
            page_in_name.as_ptr(),
            page_in,
            subifd_in_name.as_ptr(),
            subifd_in,
            n_in_name.as_ptr(),
            n_in,
            autorotate_in_name.as_ptr(),
            autorotate_in,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::TiffloadSourceError,
        )
    }
}

/// VipsForeignLoadHeifFile (heifload), load a HEIF image (.heic, .heif, .avif), priority=0, is_a, get_flags, header, load
/// filename: `&str` -> Filename to load from
/// returns `VipsImage` - Output image
pub fn heifload(filename: &str) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_heifload(
            filename_in.as_ptr(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HeifloadError,
        )
    }
}

/// Options for heifload operation
#[derive(Clone, Debug)]
pub struct HeifloadOptions {
    /// page: `i32` -> First page to load
    /// min: 0, max: 100000, default: 0
    pub page: i32,
    /// n: `i32` -> Number of pages to load, -1 for all
    /// min: -1, max: 100000, default: 1
    pub n: i32,
    /// thumbnail: `bool` -> Fetch thumbnail image
    /// default: false
    pub thumbnail: bool,
    /// unlimited: `bool` -> Remove all denial of service limits
    /// default: false
    pub unlimited: bool,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for HeifloadOptions {
    fn default() -> Self {
        HeifloadOptions {
            page: i32::from(0),
            n: i32::from(1),
            thumbnail: false,
            unlimited: false,
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadHeifFile (heifload), load a HEIF image (.heic, .heif, .avif), priority=0, is_a, get_flags, header, load
/// filename: `&str` -> Filename to load from
/// heifload_options: `&HeifloadOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn heifload_with_opts(filename: &str, heifload_options: &HeifloadOptions) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let page_in: i32 = heifload_options.page;
        let page_in_name = utils::new_c_string("page")?;

        let n_in: i32 = heifload_options.n;
        let n_in_name = utils::new_c_string("n")?;

        let thumbnail_in: i32 = if heifload_options.thumbnail { 1 } else { 0 };
        let thumbnail_in_name = utils::new_c_string("thumbnail")?;

        let unlimited_in: i32 = if heifload_options.unlimited { 1 } else { 0 };
        let unlimited_in_name = utils::new_c_string("unlimited")?;

        let flags_in: i32 = heifload_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if heifload_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = heifload_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = heifload_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if heifload_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_heifload(
            filename_in.as_ptr(),
            &mut out_out,
            page_in_name.as_ptr(),
            page_in,
            n_in_name.as_ptr(),
            n_in,
            thumbnail_in_name.as_ptr(),
            thumbnail_in,
            unlimited_in_name.as_ptr(),
            unlimited_in,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HeifloadError,
        )
    }
}

/// VipsForeignLoadHeifBuffer (heifload_buffer), load a HEIF image, priority=0, is_a_buffer, get_flags, header, load
/// buffer: `&[u8]` -> Buffer to load from
/// returns `VipsImage` - Output image
pub fn heifload_buffer(buffer: &[u8]) -> Result<VipsImage> {
    unsafe {
        let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_heifload_buffer(
            buffer_in,
            buffer.len(),
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HeifloadBufferError,
        )
    }
}

/// Options for heifload_buffer operation
#[derive(Clone, Debug)]
pub struct HeifloadBufferOptions {
    /// page: `i32` -> First page to load
    /// min: 0, max: 100000, default: 0
    pub page: i32,
    /// n: `i32` -> Number of pages to load, -1 for all
    /// min: -1, max: 100000, default: 1
    pub n: i32,
    /// thumbnail: `bool` -> Fetch thumbnail image
    /// default: false
    pub thumbnail: bool,
    /// unlimited: `bool` -> Remove all denial of service limits
    /// default: false
    pub unlimited: bool,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for HeifloadBufferOptions {
    fn default() -> Self {
        HeifloadBufferOptions {
            page: i32::from(0),
            n: i32::from(1),
            thumbnail: false,
            unlimited: false,
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadHeifBuffer (heifload_buffer), load a HEIF image, priority=0, is_a_buffer, get_flags, header, load
/// buffer: `&[u8]` -> Buffer to load from
/// heifload_buffer_options: `&HeifloadBufferOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn heifload_buffer_with_opts(
    buffer: &[u8],
    heifload_buffer_options: &HeifloadBufferOptions,
) -> Result<VipsImage> {
    unsafe {
        let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let page_in: i32 = heifload_buffer_options.page;
        let page_in_name = utils::new_c_string("page")?;

        let n_in: i32 = heifload_buffer_options.n;
        let n_in_name = utils::new_c_string("n")?;

        let thumbnail_in: i32 = if heifload_buffer_options.thumbnail { 1 } else { 0 };
        let thumbnail_in_name = utils::new_c_string("thumbnail")?;

        let unlimited_in: i32 = if heifload_buffer_options.unlimited { 1 } else { 0 };
        let unlimited_in_name = utils::new_c_string("unlimited")?;

        let flags_in: i32 = heifload_buffer_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if heifload_buffer_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = heifload_buffer_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = heifload_buffer_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if heifload_buffer_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_heifload_buffer(
            buffer_in,
            buffer.len(),
            &mut out_out,
            page_in_name.as_ptr(),
            page_in,
            n_in_name.as_ptr(),
            n_in,
            thumbnail_in_name.as_ptr(),
            thumbnail_in,
            unlimited_in_name.as_ptr(),
            unlimited_in,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HeifloadBufferError,
        )
    }
}

/// VipsForeignLoadHeifSource (heifload_source), load a HEIF image, priority=0, is_a_source, get_flags, header, load
/// source: `&VipsSource` -> Source to load from
/// returns `VipsImage` - Output image
pub fn heifload_source(source: &VipsSource) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_heifload_source(
            source_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HeifloadSourceError,
        )
    }
}

/// Options for heifload_source operation
#[derive(Clone, Debug)]
pub struct HeifloadSourceOptions {
    /// page: `i32` -> First page to load
    /// min: 0, max: 100000, default: 0
    pub page: i32,
    /// n: `i32` -> Number of pages to load, -1 for all
    /// min: -1, max: 100000, default: 1
    pub n: i32,
    /// thumbnail: `bool` -> Fetch thumbnail image
    /// default: false
    pub thumbnail: bool,
    /// unlimited: `bool` -> Remove all denial of service limits
    /// default: false
    pub unlimited: bool,
    /// flags: `ForeignFlags` -> Flags for this file
    ///  `None` -> VIPS_FOREIGN_NONE = 0 [DEFAULT]
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    pub flags: ForeignFlags,
    /// memory: `bool` -> Force open via memory
    /// default: false
    pub memory: bool,
    /// access: `Access` -> Required access pattern for this file
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0 [DEFAULT]
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    pub access: Access,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
    /// revalidate: `bool` -> Don't use a cached result for this operation
    /// default: false
    pub revalidate: bool,
}

impl std::default::Default for HeifloadSourceOptions {
    fn default() -> Self {
        HeifloadSourceOptions {
            page: i32::from(0),
            n: i32::from(1),
            thumbnail: false,
            unlimited: false,
            flags: ForeignFlags::None,
            memory: false,
            access: Access::Random,
            fail_on: FailOn::None,
            revalidate: false,
        }
    }
}

/// VipsForeignLoadHeifSource (heifload_source), load a HEIF image, priority=0, is_a_source, get_flags, header, load
/// source: `&VipsSource` -> Source to load from
/// heifload_source_options: `&HeifloadSourceOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn heifload_source_with_opts(
    source: &VipsSource,
    heifload_source_options: &HeifloadSourceOptions,
) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let page_in: i32 = heifload_source_options.page;
        let page_in_name = utils::new_c_string("page")?;

        let n_in: i32 = heifload_source_options.n;
        let n_in_name = utils::new_c_string("n")?;

        let thumbnail_in: i32 = if heifload_source_options.thumbnail { 1 } else { 0 };
        let thumbnail_in_name = utils::new_c_string("thumbnail")?;

        let unlimited_in: i32 = if heifload_source_options.unlimited { 1 } else { 0 };
        let unlimited_in_name = utils::new_c_string("unlimited")?;

        let flags_in: i32 = heifload_source_options.flags as i32;
        let flags_in_name = utils::new_c_string("flags")?;

        let memory_in: i32 = if heifload_source_options.memory { 1 } else { 0 };
        let memory_in_name = utils::new_c_string("memory")?;

        let access_in: i32 = heifload_source_options.access as i32;
        let access_in_name = utils::new_c_string("access")?;

        let fail_on_in: i32 = heifload_source_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let revalidate_in: i32 = if heifload_source_options.revalidate { 1 } else { 0 };
        let revalidate_in_name = utils::new_c_string("revalidate")?;

        let vips_op_response = bindings::vips_heifload_source(
            source_in,
            &mut out_out,
            page_in_name.as_ptr(),
            page_in,
            n_in_name.as_ptr(),
            n_in,
            thumbnail_in_name.as_ptr(),
            thumbnail_in,
            unlimited_in_name.as_ptr(),
            unlimited_in,
            flags_in_name.as_ptr(),
            flags_in,
            memory_in_name.as_ptr(),
            memory_in,
            access_in_name.as_ptr(),
            access_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            revalidate_in_name.as_ptr(),
            revalidate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HeifloadSourceError,
        )
    }
}

/// VipsForeignSaveCsvFile (csvsave), save image to csv (.csv), priority=0, mono
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to

pub fn csvsave(inp: &VipsImage, filename: &str) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let vips_op_response = bindings::vips_csvsave(
            inp_in,
            filename_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::CsvsaveError,
        )
    }
}

/// Options for csvsave operation
#[derive(Clone, Debug)]
pub struct CsvsaveOptions {
    /// separator: `String` -> Separator characters
    pub separator: String,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for CsvsaveOptions {
    fn default() -> Self {
        CsvsaveOptions {
            separator: String::new(),
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveCsvFile (csvsave), save image to csv (.csv), priority=0, mono
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to
/// csvsave_options: `&CsvsaveOptions` -> optional arguments

pub fn csvsave_with_opts(
    inp: &VipsImage,
    filename: &str,
    csvsave_options: &CsvsaveOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let separator_in: CString = utils::new_c_string(&csvsave_options.separator)?;
        let separator_in_name = utils::new_c_string("separator")?;

        let keep_in: i32 = csvsave_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&csvsave_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = csvsave_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&csvsave_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_csvsave(
            inp_in,
            filename_in.as_ptr(),
            separator_in_name.as_ptr(),
            separator_in.as_ptr(),
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::CsvsaveError,
        )
    }
}

/// VipsForeignSaveCsvTarget (csvsave_target), save image to csv (.csv), priority=0, mono
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to

pub fn csvsave_target(inp: &VipsImage, target: &VipsTarget) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let vips_op_response = bindings::vips_csvsave_target(
            inp_in,
            target_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::CsvsaveTargetError,
        )
    }
}

/// Options for csvsave_target operation
#[derive(Clone, Debug)]
pub struct CsvsaveTargetOptions {
    /// separator: `String` -> Separator characters
    pub separator: String,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for CsvsaveTargetOptions {
    fn default() -> Self {
        CsvsaveTargetOptions {
            separator: String::new(),
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveCsvTarget (csvsave_target), save image to csv (.csv), priority=0, mono
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to
/// csvsave_target_options: `&CsvsaveTargetOptions` -> optional arguments

pub fn csvsave_target_with_opts(
    inp: &VipsImage,
    target: &VipsTarget,
    csvsave_target_options: &CsvsaveTargetOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let separator_in: CString = utils::new_c_string(&csvsave_target_options.separator)?;
        let separator_in_name = utils::new_c_string("separator")?;

        let keep_in: i32 = csvsave_target_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&csvsave_target_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = csvsave_target_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&csvsave_target_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_csvsave_target(
            inp_in,
            target_in,
            separator_in_name.as_ptr(),
            separator_in.as_ptr(),
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::CsvsaveTargetError,
        )
    }
}

/// VipsForeignSaveMatrixFile (matrixsave), save image to matrix (.mat), priority=0, mono
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to

pub fn matrixsave(inp: &VipsImage, filename: &str) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let vips_op_response = bindings::vips_matrixsave(
            inp_in,
            filename_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::MatrixsaveError,
        )
    }
}

/// Options for matrixsave operation
#[derive(Clone, Debug)]
pub struct MatrixsaveOptions {
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for MatrixsaveOptions {
    fn default() -> Self {
        MatrixsaveOptions {
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveMatrixFile (matrixsave), save image to matrix (.mat), priority=0, mono
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to
/// matrixsave_options: `&MatrixsaveOptions` -> optional arguments

pub fn matrixsave_with_opts(
    inp: &VipsImage,
    filename: &str,
    matrixsave_options: &MatrixsaveOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let keep_in: i32 = matrixsave_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&matrixsave_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = matrixsave_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&matrixsave_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_matrixsave(
            inp_in,
            filename_in.as_ptr(),
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::MatrixsaveError,
        )
    }
}

/// VipsForeignSaveMatrixTarget (matrixsave_target), save image to matrix (.mat), priority=0, mono
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to

pub fn matrixsave_target(inp: &VipsImage, target: &VipsTarget) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let vips_op_response = bindings::vips_matrixsave_target(
            inp_in,
            target_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::MatrixsaveTargetError,
        )
    }
}

/// Options for matrixsave_target operation
#[derive(Clone, Debug)]
pub struct MatrixsaveTargetOptions {
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for MatrixsaveTargetOptions {
    fn default() -> Self {
        MatrixsaveTargetOptions {
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveMatrixTarget (matrixsave_target), save image to matrix (.mat), priority=0, mono
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to
/// matrixsave_target_options: `&MatrixsaveTargetOptions` -> optional arguments

pub fn matrixsave_target_with_opts(
    inp: &VipsImage,
    target: &VipsTarget,
    matrixsave_target_options: &MatrixsaveTargetOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let keep_in: i32 = matrixsave_target_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&matrixsave_target_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = matrixsave_target_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&matrixsave_target_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_matrixsave_target(
            inp_in,
            target_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::MatrixsaveTargetError,
        )
    }
}

/// VipsForeignPrintMatrix (matrixprint), print matrix (.mat), priority=0, mono
/// inp: `&VipsImage` -> Image to save

pub fn matrixprint(inp: &VipsImage) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;

        let vips_op_response = bindings::vips_matrixprint(
            inp_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::MatrixprintError,
        )
    }
}

/// Options for matrixprint operation
#[derive(Clone, Debug)]
pub struct MatrixprintOptions {
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for MatrixprintOptions {
    fn default() -> Self {
        MatrixprintOptions {
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignPrintMatrix (matrixprint), print matrix (.mat), priority=0, mono
/// inp: `&VipsImage` -> Image to save
/// matrixprint_options: `&MatrixprintOptions` -> optional arguments

pub fn matrixprint_with_opts(
    inp: &VipsImage,
    matrixprint_options: &MatrixprintOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;

        let keep_in: i32 = matrixprint_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&matrixprint_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = matrixprint_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&matrixprint_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_matrixprint(
            inp_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::MatrixprintError,
        )
    }
}

/// VipsForeignSaveRaw (rawsave), save image to raw file (.raw), priority=0, any
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to

pub fn rawsave(inp: &VipsImage, filename: &str) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let vips_op_response = bindings::vips_rawsave(
            inp_in,
            filename_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::RawsaveError,
        )
    }
}

/// Options for rawsave operation
#[derive(Clone, Debug)]
pub struct RawsaveOptions {
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for RawsaveOptions {
    fn default() -> Self {
        RawsaveOptions {
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveRaw (rawsave), save image to raw file (.raw), priority=0, any
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to
/// rawsave_options: `&RawsaveOptions` -> optional arguments

pub fn rawsave_with_opts(
    inp: &VipsImage,
    filename: &str,
    rawsave_options: &RawsaveOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let keep_in: i32 = rawsave_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&rawsave_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = rawsave_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&rawsave_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_rawsave(
            inp_in,
            filename_in.as_ptr(),
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::RawsaveError,
        )
    }
}

/// VipsForeignSaveRawFd (rawsave_fd), write raw image to file descriptor (.raw), priority=0, any
/// inp: `&VipsImage` -> Image to save
/// fd: `i32` -> File descriptor to write to
/// min: 0, max: 10000, default: 0

pub fn rawsave_fd(inp: &VipsImage, fd: i32) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let fd_in: i32 = fd;

        let vips_op_response = bindings::vips_rawsave_fd(
            inp_in,
            fd_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::RawsaveFdError,
        )
    }
}

/// Options for rawsave_fd operation
#[derive(Clone, Debug)]
pub struct RawsaveFdOptions {
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for RawsaveFdOptions {
    fn default() -> Self {
        RawsaveFdOptions {
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveRawFd (rawsave_fd), write raw image to file descriptor (.raw), priority=0, any
/// inp: `&VipsImage` -> Image to save
/// fd: `i32` -> File descriptor to write to
/// min: 0, max: 10000, default: 0
/// rawsave_fd_options: `&RawsaveFdOptions` -> optional arguments

pub fn rawsave_fd_with_opts(
    inp: &VipsImage,
    fd: i32,
    rawsave_fd_options: &RawsaveFdOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let fd_in: i32 = fd;

        let keep_in: i32 = rawsave_fd_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&rawsave_fd_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = rawsave_fd_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&rawsave_fd_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_rawsave_fd(
            inp_in,
            fd_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::RawsaveFdError,
        )
    }
}

/// VipsForeignSaveVipsFile (vipssave), save image to file in vips format (.v, .vips), priority=0, any
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to

pub fn vipssave(inp: &VipsImage, filename: &str) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let vips_op_response = bindings::vips_vipssave(
            inp_in,
            filename_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::VipssaveError,
        )
    }
}

/// Options for vipssave operation
#[derive(Clone, Debug)]
pub struct VipssaveOptions {
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for VipssaveOptions {
    fn default() -> Self {
        VipssaveOptions {
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveVipsFile (vipssave), save image to file in vips format (.v, .vips), priority=0, any
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to
/// vipssave_options: `&VipssaveOptions` -> optional arguments

pub fn vipssave_with_opts(
    inp: &VipsImage,
    filename: &str,
    vipssave_options: &VipssaveOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let keep_in: i32 = vipssave_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&vipssave_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = vipssave_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&vipssave_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_vipssave(
            inp_in,
            filename_in.as_ptr(),
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::VipssaveError,
        )
    }
}

/// VipsForeignSaveVipsTarget (vipssave_target), save image to target in vips format (.v, .vips), priority=0, any
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to

pub fn vipssave_target(inp: &VipsImage, target: &VipsTarget) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let vips_op_response = bindings::vips_vipssave_target(
            inp_in,
            target_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::VipssaveTargetError,
        )
    }
}

/// Options for vipssave_target operation
#[derive(Clone, Debug)]
pub struct VipssaveTargetOptions {
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for VipssaveTargetOptions {
    fn default() -> Self {
        VipssaveTargetOptions {
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveVipsTarget (vipssave_target), save image to target in vips format (.v, .vips), priority=0, any
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to
/// vipssave_target_options: `&VipssaveTargetOptions` -> optional arguments

pub fn vipssave_target_with_opts(
    inp: &VipsImage,
    target: &VipsTarget,
    vipssave_target_options: &VipssaveTargetOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let keep_in: i32 = vipssave_target_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&vipssave_target_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = vipssave_target_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&vipssave_target_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_vipssave_target(
            inp_in,
            target_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::VipssaveTargetError,
        )
    }
}

/// VipsForeignSavePpmFile (ppmsave), save image to ppm file (.pbm, .pgm, .ppm, .pfm, .pnm), priority=0, rgb
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to

pub fn ppmsave(inp: &VipsImage, filename: &str) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let vips_op_response = bindings::vips_ppmsave(
            inp_in,
            filename_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::PpmsaveError,
        )
    }
}

/// Options for ppmsave operation
#[derive(Clone, Debug)]
pub struct PpmsaveOptions {
    /// format: `ForeignPpmFormat` -> Format to save in
    ///  `Pbm` -> VIPS_FOREIGN_PPM_FORMAT_PBM = 0
    ///  `Pgm` -> VIPS_FOREIGN_PPM_FORMAT_PGM = 1
    ///  `Ppm` -> VIPS_FOREIGN_PPM_FORMAT_PPM = 2 [DEFAULT]
    ///  `Pfm` -> VIPS_FOREIGN_PPM_FORMAT_PFM = 3
    ///  `Pnm` -> VIPS_FOREIGN_PPM_FORMAT_PNM = 4
    ///  `Last` -> VIPS_FOREIGN_PPM_FORMAT_LAST = 5
    pub format: ForeignPpmFormat,
    /// ascii: `bool` -> Save as ascii
    /// default: false
    pub ascii: bool,
    /// bitdepth: `i32` -> Set to 1 to write as a 1 bit image
    /// min: 0, max: 1, default: 0
    pub bitdepth: i32,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for PpmsaveOptions {
    fn default() -> Self {
        PpmsaveOptions {
            format: ForeignPpmFormat::Ppm,
            ascii: false,
            bitdepth: i32::from(0),
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSavePpmFile (ppmsave), save image to ppm file (.pbm, .pgm, .ppm, .pfm, .pnm), priority=0, rgb
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to
/// ppmsave_options: `&PpmsaveOptions` -> optional arguments

pub fn ppmsave_with_opts(
    inp: &VipsImage,
    filename: &str,
    ppmsave_options: &PpmsaveOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let format_in: i32 = ppmsave_options.format as i32;
        let format_in_name = utils::new_c_string("format")?;

        let ascii_in: i32 = if ppmsave_options.ascii { 1 } else { 0 };
        let ascii_in_name = utils::new_c_string("ascii")?;

        let bitdepth_in: i32 = ppmsave_options.bitdepth;
        let bitdepth_in_name = utils::new_c_string("bitdepth")?;

        let keep_in: i32 = ppmsave_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&ppmsave_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = ppmsave_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&ppmsave_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_ppmsave(
            inp_in,
            filename_in.as_ptr(),
            format_in_name.as_ptr(),
            format_in,
            ascii_in_name.as_ptr(),
            ascii_in,
            bitdepth_in_name.as_ptr(),
            bitdepth_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::PpmsaveError,
        )
    }
}

/// VipsForeignSavePpmTarget (ppmsave_target), save to ppm (.ppm), priority=0, rgb
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to

pub fn ppmsave_target(inp: &VipsImage, target: &VipsTarget) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let vips_op_response = bindings::vips_ppmsave_target(
            inp_in,
            target_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::PpmsaveTargetError,
        )
    }
}

/// Options for ppmsave_target operation
#[derive(Clone, Debug)]
pub struct PpmsaveTargetOptions {
    /// format: `ForeignPpmFormat` -> Format to save in
    ///  `Pbm` -> VIPS_FOREIGN_PPM_FORMAT_PBM = 0
    ///  `Pgm` -> VIPS_FOREIGN_PPM_FORMAT_PGM = 1
    ///  `Ppm` -> VIPS_FOREIGN_PPM_FORMAT_PPM = 2 [DEFAULT]
    ///  `Pfm` -> VIPS_FOREIGN_PPM_FORMAT_PFM = 3
    ///  `Pnm` -> VIPS_FOREIGN_PPM_FORMAT_PNM = 4
    ///  `Last` -> VIPS_FOREIGN_PPM_FORMAT_LAST = 5
    pub format: ForeignPpmFormat,
    /// ascii: `bool` -> Save as ascii
    /// default: false
    pub ascii: bool,
    /// bitdepth: `i32` -> Set to 1 to write as a 1 bit image
    /// min: 0, max: 1, default: 0
    pub bitdepth: i32,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for PpmsaveTargetOptions {
    fn default() -> Self {
        PpmsaveTargetOptions {
            format: ForeignPpmFormat::Ppm,
            ascii: false,
            bitdepth: i32::from(0),
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSavePpmTarget (ppmsave_target), save to ppm (.ppm), priority=0, rgb
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to
/// ppmsave_target_options: `&PpmsaveTargetOptions` -> optional arguments

pub fn ppmsave_target_with_opts(
    inp: &VipsImage,
    target: &VipsTarget,
    ppmsave_target_options: &PpmsaveTargetOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let format_in: i32 = ppmsave_target_options.format as i32;
        let format_in_name = utils::new_c_string("format")?;

        let ascii_in: i32 = if ppmsave_target_options.ascii { 1 } else { 0 };
        let ascii_in_name = utils::new_c_string("ascii")?;

        let bitdepth_in: i32 = ppmsave_target_options.bitdepth;
        let bitdepth_in_name = utils::new_c_string("bitdepth")?;

        let keep_in: i32 = ppmsave_target_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&ppmsave_target_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = ppmsave_target_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&ppmsave_target_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_ppmsave_target(
            inp_in,
            target_in,
            format_in_name.as_ptr(),
            format_in,
            ascii_in_name.as_ptr(),
            ascii_in,
            bitdepth_in_name.as_ptr(),
            bitdepth_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::PpmsaveTargetError,
        )
    }
}

/// VipsForeignSaveRadFile (radsave), save image to Radiance file (.hdr), priority=0, rgb
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to

pub fn radsave(inp: &VipsImage, filename: &str) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let vips_op_response = bindings::vips_radsave(
            inp_in,
            filename_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::RadsaveError,
        )
    }
}

/// Options for radsave operation
#[derive(Clone, Debug)]
pub struct RadsaveOptions {
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for RadsaveOptions {
    fn default() -> Self {
        RadsaveOptions {
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveRadFile (radsave), save image to Radiance file (.hdr), priority=0, rgb
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to
/// radsave_options: `&RadsaveOptions` -> optional arguments

pub fn radsave_with_opts(
    inp: &VipsImage,
    filename: &str,
    radsave_options: &RadsaveOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let keep_in: i32 = radsave_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&radsave_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = radsave_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&radsave_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_radsave(
            inp_in,
            filename_in.as_ptr(),
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::RadsaveError,
        )
    }
}

/// VipsForeignSaveRadBuffer (radsave_buffer), save image to Radiance buffer (.hdr), priority=0, rgb
/// inp: `&VipsImage` -> Image to save
/// returns `Vec<u8>` - Buffer to save to
pub fn radsave_buffer(inp: &VipsImage) -> Result<Vec<u8>> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut buffer_buf_size: u64 = 0;
        let mut buffer_out: *mut c_void = null_mut();

        let vips_op_response = bindings::vips_radsave_buffer(
            inp_in,
            &mut buffer_out,
            &mut buffer_buf_size,
            NULL,
        );
        utils::result(
            vips_op_response,
            utils::new_byte_array(
                buffer_out,
                buffer_buf_size,
            ),
            Error::RadsaveBufferError,
        )
    }
}

/// Options for radsave_buffer operation
#[derive(Clone, Debug)]
pub struct RadsaveBufferOptions {
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for RadsaveBufferOptions {
    fn default() -> Self {
        RadsaveBufferOptions {
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveRadBuffer (radsave_buffer), save image to Radiance buffer (.hdr), priority=0, rgb
/// inp: `&VipsImage` -> Image to save
/// radsave_buffer_options: `&RadsaveBufferOptions` -> optional arguments
/// returns `Vec<u8>` - Buffer to save to
pub fn radsave_buffer_with_opts(
    inp: &VipsImage,
    radsave_buffer_options: &RadsaveBufferOptions,
) -> Result<Vec<u8>> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut buffer_buf_size: u64 = 0;
        let mut buffer_out: *mut c_void = null_mut();

        let keep_in: i32 = radsave_buffer_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&radsave_buffer_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = radsave_buffer_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&radsave_buffer_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_radsave_buffer(
            inp_in,
            &mut buffer_out,
            &mut buffer_buf_size,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            utils::new_byte_array(
                buffer_out,
                buffer_buf_size,
            ),
            Error::RadsaveBufferError,
        )
    }
}

/// VipsForeignSaveRadTarget (radsave_target), save image to Radiance target (.hdr), priority=0, rgb
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to

pub fn radsave_target(inp: &VipsImage, target: &VipsTarget) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let vips_op_response = bindings::vips_radsave_target(
            inp_in,
            target_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::RadsaveTargetError,
        )
    }
}

/// Options for radsave_target operation
#[derive(Clone, Debug)]
pub struct RadsaveTargetOptions {
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for RadsaveTargetOptions {
    fn default() -> Self {
        RadsaveTargetOptions {
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveRadTarget (radsave_target), save image to Radiance target (.hdr), priority=0, rgb
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to
/// radsave_target_options: `&RadsaveTargetOptions` -> optional arguments

pub fn radsave_target_with_opts(
    inp: &VipsImage,
    target: &VipsTarget,
    radsave_target_options: &RadsaveTargetOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let keep_in: i32 = radsave_target_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&radsave_target_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = radsave_target_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&radsave_target_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_radsave_target(
            inp_in,
            target_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::RadsaveTargetError,
        )
    }
}

/// VipsForeignSaveCgifFile (gifsave), save as gif (.gif), priority=0, rgba-only
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to

pub fn gifsave(inp: &VipsImage, filename: &str) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let vips_op_response = bindings::vips_gifsave(
            inp_in,
            filename_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::GifsaveError,
        )
    }
}

/// Options for gifsave operation
#[derive(Clone, Debug)]
pub struct GifsaveOptions {
    /// dither: `f64` -> Amount of dithering
    /// min: 0, max: 1, default: 1
    pub dither: f64,
    /// effort: `i32` -> Quantisation effort
    /// min: 1, max: 10, default: 7
    pub effort: i32,
    /// bitdepth: `i32` -> Number of bits per pixel
    /// min: 1, max: 8, default: 8
    pub bitdepth: i32,
    /// interframe_maxerror: `f64` -> Maximum inter-frame error for transparency
    /// min: 0, max: 32, default: 0
    pub interframe_maxerror: f64,
    /// reuse: `bool` -> Reuse palette from input
    /// default: false
    pub reuse: bool,
    /// interpalette_maxerror: `f64` -> Maximum inter-palette error for palette reusage
    /// min: 0, max: 256, default: 3
    pub interpalette_maxerror: f64,
    /// interlace: `bool` -> Generate an interlaced (progressive) GIF
    /// default: false
    pub interlace: bool,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for GifsaveOptions {
    fn default() -> Self {
        GifsaveOptions {
            dither: f64::from(1),
            effort: i32::from(7),
            bitdepth: i32::from(8),
            interframe_maxerror: f64::from(0),
            reuse: false,
            interpalette_maxerror: f64::from(3),
            interlace: false,
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveCgifFile (gifsave), save as gif (.gif), priority=0, rgba-only
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to
/// gifsave_options: `&GifsaveOptions` -> optional arguments

pub fn gifsave_with_opts(
    inp: &VipsImage,
    filename: &str,
    gifsave_options: &GifsaveOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let dither_in: f64 = gifsave_options.dither;
        let dither_in_name = utils::new_c_string("dither")?;

        let effort_in: i32 = gifsave_options.effort;
        let effort_in_name = utils::new_c_string("effort")?;

        let bitdepth_in: i32 = gifsave_options.bitdepth;
        let bitdepth_in_name = utils::new_c_string("bitdepth")?;

        let interframe_maxerror_in: f64 = gifsave_options.interframe_maxerror;
        let interframe_maxerror_in_name = utils::new_c_string("interframe-maxerror")?;

        let reuse_in: i32 = if gifsave_options.reuse { 1 } else { 0 };
        let reuse_in_name = utils::new_c_string("reuse")?;

        let interpalette_maxerror_in: f64 = gifsave_options.interpalette_maxerror;
        let interpalette_maxerror_in_name = utils::new_c_string("interpalette-maxerror")?;

        let interlace_in: i32 = if gifsave_options.interlace { 1 } else { 0 };
        let interlace_in_name = utils::new_c_string("interlace")?;

        let keep_in: i32 = gifsave_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&gifsave_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = gifsave_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&gifsave_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_gifsave(
            inp_in,
            filename_in.as_ptr(),
            dither_in_name.as_ptr(),
            dither_in,
            effort_in_name.as_ptr(),
            effort_in,
            bitdepth_in_name.as_ptr(),
            bitdepth_in,
            interframe_maxerror_in_name.as_ptr(),
            interframe_maxerror_in,
            reuse_in_name.as_ptr(),
            reuse_in,
            interpalette_maxerror_in_name.as_ptr(),
            interpalette_maxerror_in,
            interlace_in_name.as_ptr(),
            interlace_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::GifsaveError,
        )
    }
}

/// VipsForeignSaveCgifBuffer (gifsave_buffer), save as gif (.gif), priority=0, rgba-only
/// inp: `&VipsImage` -> Image to save
/// returns `Vec<u8>` - Buffer to save to
pub fn gifsave_buffer(inp: &VipsImage) -> Result<Vec<u8>> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut buffer_buf_size: u64 = 0;
        let mut buffer_out: *mut c_void = null_mut();

        let vips_op_response = bindings::vips_gifsave_buffer(
            inp_in,
            &mut buffer_out,
            &mut buffer_buf_size,
            NULL,
        );
        utils::result(
            vips_op_response,
            utils::new_byte_array(
                buffer_out,
                buffer_buf_size,
            ),
            Error::GifsaveBufferError,
        )
    }
}

/// Options for gifsave_buffer operation
#[derive(Clone, Debug)]
pub struct GifsaveBufferOptions {
    /// dither: `f64` -> Amount of dithering
    /// min: 0, max: 1, default: 1
    pub dither: f64,
    /// effort: `i32` -> Quantisation effort
    /// min: 1, max: 10, default: 7
    pub effort: i32,
    /// bitdepth: `i32` -> Number of bits per pixel
    /// min: 1, max: 8, default: 8
    pub bitdepth: i32,
    /// interframe_maxerror: `f64` -> Maximum inter-frame error for transparency
    /// min: 0, max: 32, default: 0
    pub interframe_maxerror: f64,
    /// reuse: `bool` -> Reuse palette from input
    /// default: false
    pub reuse: bool,
    /// interpalette_maxerror: `f64` -> Maximum inter-palette error for palette reusage
    /// min: 0, max: 256, default: 3
    pub interpalette_maxerror: f64,
    /// interlace: `bool` -> Generate an interlaced (progressive) GIF
    /// default: false
    pub interlace: bool,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for GifsaveBufferOptions {
    fn default() -> Self {
        GifsaveBufferOptions {
            dither: f64::from(1),
            effort: i32::from(7),
            bitdepth: i32::from(8),
            interframe_maxerror: f64::from(0),
            reuse: false,
            interpalette_maxerror: f64::from(3),
            interlace: false,
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveCgifBuffer (gifsave_buffer), save as gif (.gif), priority=0, rgba-only
/// inp: `&VipsImage` -> Image to save
/// gifsave_buffer_options: `&GifsaveBufferOptions` -> optional arguments
/// returns `Vec<u8>` - Buffer to save to
pub fn gifsave_buffer_with_opts(
    inp: &VipsImage,
    gifsave_buffer_options: &GifsaveBufferOptions,
) -> Result<Vec<u8>> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut buffer_buf_size: u64 = 0;
        let mut buffer_out: *mut c_void = null_mut();

        let dither_in: f64 = gifsave_buffer_options.dither;
        let dither_in_name = utils::new_c_string("dither")?;

        let effort_in: i32 = gifsave_buffer_options.effort;
        let effort_in_name = utils::new_c_string("effort")?;

        let bitdepth_in: i32 = gifsave_buffer_options.bitdepth;
        let bitdepth_in_name = utils::new_c_string("bitdepth")?;

        let interframe_maxerror_in: f64 = gifsave_buffer_options.interframe_maxerror;
        let interframe_maxerror_in_name = utils::new_c_string("interframe-maxerror")?;

        let reuse_in: i32 = if gifsave_buffer_options.reuse { 1 } else { 0 };
        let reuse_in_name = utils::new_c_string("reuse")?;

        let interpalette_maxerror_in: f64 = gifsave_buffer_options.interpalette_maxerror;
        let interpalette_maxerror_in_name = utils::new_c_string("interpalette-maxerror")?;

        let interlace_in: i32 = if gifsave_buffer_options.interlace { 1 } else { 0 };
        let interlace_in_name = utils::new_c_string("interlace")?;

        let keep_in: i32 = gifsave_buffer_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&gifsave_buffer_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = gifsave_buffer_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&gifsave_buffer_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_gifsave_buffer(
            inp_in,
            &mut buffer_out,
            &mut buffer_buf_size,
            dither_in_name.as_ptr(),
            dither_in,
            effort_in_name.as_ptr(),
            effort_in,
            bitdepth_in_name.as_ptr(),
            bitdepth_in,
            interframe_maxerror_in_name.as_ptr(),
            interframe_maxerror_in,
            reuse_in_name.as_ptr(),
            reuse_in,
            interpalette_maxerror_in_name.as_ptr(),
            interpalette_maxerror_in,
            interlace_in_name.as_ptr(),
            interlace_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            utils::new_byte_array(
                buffer_out,
                buffer_buf_size,
            ),
            Error::GifsaveBufferError,
        )
    }
}

/// VipsForeignSaveCgifTarget (gifsave_target), save as gif (.gif), priority=0, rgba-only
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to

pub fn gifsave_target(inp: &VipsImage, target: &VipsTarget) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let vips_op_response = bindings::vips_gifsave_target(
            inp_in,
            target_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::GifsaveTargetError,
        )
    }
}

/// Options for gifsave_target operation
#[derive(Clone, Debug)]
pub struct GifsaveTargetOptions {
    /// dither: `f64` -> Amount of dithering
    /// min: 0, max: 1, default: 1
    pub dither: f64,
    /// effort: `i32` -> Quantisation effort
    /// min: 1, max: 10, default: 7
    pub effort: i32,
    /// bitdepth: `i32` -> Number of bits per pixel
    /// min: 1, max: 8, default: 8
    pub bitdepth: i32,
    /// interframe_maxerror: `f64` -> Maximum inter-frame error for transparency
    /// min: 0, max: 32, default: 0
    pub interframe_maxerror: f64,
    /// reuse: `bool` -> Reuse palette from input
    /// default: false
    pub reuse: bool,
    /// interpalette_maxerror: `f64` -> Maximum inter-palette error for palette reusage
    /// min: 0, max: 256, default: 3
    pub interpalette_maxerror: f64,
    /// interlace: `bool` -> Generate an interlaced (progressive) GIF
    /// default: false
    pub interlace: bool,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for GifsaveTargetOptions {
    fn default() -> Self {
        GifsaveTargetOptions {
            dither: f64::from(1),
            effort: i32::from(7),
            bitdepth: i32::from(8),
            interframe_maxerror: f64::from(0),
            reuse: false,
            interpalette_maxerror: f64::from(3),
            interlace: false,
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveCgifTarget (gifsave_target), save as gif (.gif), priority=0, rgba-only
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to
/// gifsave_target_options: `&GifsaveTargetOptions` -> optional arguments

pub fn gifsave_target_with_opts(
    inp: &VipsImage,
    target: &VipsTarget,
    gifsave_target_options: &GifsaveTargetOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let dither_in: f64 = gifsave_target_options.dither;
        let dither_in_name = utils::new_c_string("dither")?;

        let effort_in: i32 = gifsave_target_options.effort;
        let effort_in_name = utils::new_c_string("effort")?;

        let bitdepth_in: i32 = gifsave_target_options.bitdepth;
        let bitdepth_in_name = utils::new_c_string("bitdepth")?;

        let interframe_maxerror_in: f64 = gifsave_target_options.interframe_maxerror;
        let interframe_maxerror_in_name = utils::new_c_string("interframe-maxerror")?;

        let reuse_in: i32 = if gifsave_target_options.reuse { 1 } else { 0 };
        let reuse_in_name = utils::new_c_string("reuse")?;

        let interpalette_maxerror_in: f64 = gifsave_target_options.interpalette_maxerror;
        let interpalette_maxerror_in_name = utils::new_c_string("interpalette-maxerror")?;

        let interlace_in: i32 = if gifsave_target_options.interlace { 1 } else { 0 };
        let interlace_in_name = utils::new_c_string("interlace")?;

        let keep_in: i32 = gifsave_target_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&gifsave_target_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = gifsave_target_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&gifsave_target_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_gifsave_target(
            inp_in,
            target_in,
            dither_in_name.as_ptr(),
            dither_in,
            effort_in_name.as_ptr(),
            effort_in,
            bitdepth_in_name.as_ptr(),
            bitdepth_in,
            interframe_maxerror_in_name.as_ptr(),
            interframe_maxerror_in,
            reuse_in_name.as_ptr(),
            reuse_in,
            interpalette_maxerror_in_name.as_ptr(),
            interpalette_maxerror_in,
            interlace_in_name.as_ptr(),
            interlace_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::GifsaveTargetError,
        )
    }
}

/// VipsForeignSavePngFile (pngsave), save image to png file (.png), priority=0, rgba
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to

pub fn pngsave(inp: &VipsImage, filename: &str) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let vips_op_response = bindings::vips_pngsave(
            inp_in,
            filename_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::PngsaveError,
        )
    }
}

/// Options for pngsave operation
#[derive(Clone, Debug)]
pub struct PngsaveOptions {
    /// compression: `i32` -> Compression factor
    /// min: 0, max: 9, default: 6
    pub compression: i32,
    /// interlace: `bool` -> Interlace image
    /// default: false
    pub interlace: bool,
    /// filter: `ForeignPngFilter` -> libpng row filter flag(s)
    ///  `None` -> VIPS_FOREIGN_PNG_FILTER_NONE = 8 [DEFAULT]
    ///  `Sub` -> VIPS_FOREIGN_PNG_FILTER_SUB = 16
    ///  `Up` -> VIPS_FOREIGN_PNG_FILTER_UP = 32
    ///  `Avg` -> VIPS_FOREIGN_PNG_FILTER_AVG = 64
    ///  `Paeth` -> VIPS_FOREIGN_PNG_FILTER_PAETH = 128
    ///  `All` -> VIPS_FOREIGN_PNG_FILTER_ALL = 248
    pub filter: ForeignPngFilter,
    /// palette: `bool` -> Quantise to 8bpp palette
    /// default: false
    pub palette: bool,
    /// q: `i32` -> Quantisation quality
    /// min: 0, max: 100, default: 100
    pub q: i32,
    /// dither: `f64` -> Amount of dithering
    /// min: 0, max: 1, default: 1
    pub dither: f64,
    /// bitdepth: `i32` -> Write as a 1, 2, 4, 8 or 16 bit image
    /// min: 1, max: 16, default: 8
    pub bitdepth: i32,
    /// effort: `i32` -> Quantisation CPU effort
    /// min: 1, max: 10, default: 7
    pub effort: i32,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for PngsaveOptions {
    fn default() -> Self {
        PngsaveOptions {
            compression: i32::from(6),
            interlace: false,
            filter: ForeignPngFilter::None,
            palette: false,
            q: i32::from(100),
            dither: f64::from(1),
            bitdepth: i32::from(8),
            effort: i32::from(7),
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSavePngFile (pngsave), save image to png file (.png), priority=0, rgba
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to
/// pngsave_options: `&PngsaveOptions` -> optional arguments

pub fn pngsave_with_opts(
    inp: &VipsImage,
    filename: &str,
    pngsave_options: &PngsaveOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let compression_in: i32 = pngsave_options.compression;
        let compression_in_name = utils::new_c_string("compression")?;

        let interlace_in: i32 = if pngsave_options.interlace { 1 } else { 0 };
        let interlace_in_name = utils::new_c_string("interlace")?;

        let filter_in: i32 = pngsave_options.filter as i32;
        let filter_in_name = utils::new_c_string("filter")?;

        let palette_in: i32 = if pngsave_options.palette { 1 } else { 0 };
        let palette_in_name = utils::new_c_string("palette")?;

        let q_in: i32 = pngsave_options.q;
        let q_in_name = utils::new_c_string("Q")?;

        let dither_in: f64 = pngsave_options.dither;
        let dither_in_name = utils::new_c_string("dither")?;

        let bitdepth_in: i32 = pngsave_options.bitdepth;
        let bitdepth_in_name = utils::new_c_string("bitdepth")?;

        let effort_in: i32 = pngsave_options.effort;
        let effort_in_name = utils::new_c_string("effort")?;

        let keep_in: i32 = pngsave_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&pngsave_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = pngsave_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&pngsave_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_pngsave(
            inp_in,
            filename_in.as_ptr(),
            compression_in_name.as_ptr(),
            compression_in,
            interlace_in_name.as_ptr(),
            interlace_in,
            filter_in_name.as_ptr(),
            filter_in,
            palette_in_name.as_ptr(),
            palette_in,
            q_in_name.as_ptr(),
            q_in,
            dither_in_name.as_ptr(),
            dither_in,
            bitdepth_in_name.as_ptr(),
            bitdepth_in,
            effort_in_name.as_ptr(),
            effort_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::PngsaveError,
        )
    }
}

/// VipsForeignSavePngBuffer (pngsave_buffer), save image to png buffer (.png), priority=0, rgba
/// inp: `&VipsImage` -> Image to save
/// returns `Vec<u8>` - Buffer to save to
pub fn pngsave_buffer(inp: &VipsImage) -> Result<Vec<u8>> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut buffer_buf_size: u64 = 0;
        let mut buffer_out: *mut c_void = null_mut();

        let vips_op_response = bindings::vips_pngsave_buffer(
            inp_in,
            &mut buffer_out,
            &mut buffer_buf_size,
            NULL,
        );
        utils::result(
            vips_op_response,
            utils::new_byte_array(
                buffer_out,
                buffer_buf_size,
            ),
            Error::PngsaveBufferError,
        )
    }
}

/// Options for pngsave_buffer operation
#[derive(Clone, Debug)]
pub struct PngsaveBufferOptions {
    /// compression: `i32` -> Compression factor
    /// min: 0, max: 9, default: 6
    pub compression: i32,
    /// interlace: `bool` -> Interlace image
    /// default: false
    pub interlace: bool,
    /// filter: `ForeignPngFilter` -> libpng row filter flag(s)
    ///  `None` -> VIPS_FOREIGN_PNG_FILTER_NONE = 8 [DEFAULT]
    ///  `Sub` -> VIPS_FOREIGN_PNG_FILTER_SUB = 16
    ///  `Up` -> VIPS_FOREIGN_PNG_FILTER_UP = 32
    ///  `Avg` -> VIPS_FOREIGN_PNG_FILTER_AVG = 64
    ///  `Paeth` -> VIPS_FOREIGN_PNG_FILTER_PAETH = 128
    ///  `All` -> VIPS_FOREIGN_PNG_FILTER_ALL = 248
    pub filter: ForeignPngFilter,
    /// palette: `bool` -> Quantise to 8bpp palette
    /// default: false
    pub palette: bool,
    /// q: `i32` -> Quantisation quality
    /// min: 0, max: 100, default: 100
    pub q: i32,
    /// dither: `f64` -> Amount of dithering
    /// min: 0, max: 1, default: 1
    pub dither: f64,
    /// bitdepth: `i32` -> Write as a 1, 2, 4, 8 or 16 bit image
    /// min: 1, max: 16, default: 8
    pub bitdepth: i32,
    /// effort: `i32` -> Quantisation CPU effort
    /// min: 1, max: 10, default: 7
    pub effort: i32,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for PngsaveBufferOptions {
    fn default() -> Self {
        PngsaveBufferOptions {
            compression: i32::from(6),
            interlace: false,
            filter: ForeignPngFilter::None,
            palette: false,
            q: i32::from(100),
            dither: f64::from(1),
            bitdepth: i32::from(8),
            effort: i32::from(7),
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSavePngBuffer (pngsave_buffer), save image to png buffer (.png), priority=0, rgba
/// inp: `&VipsImage` -> Image to save
/// pngsave_buffer_options: `&PngsaveBufferOptions` -> optional arguments
/// returns `Vec<u8>` - Buffer to save to
pub fn pngsave_buffer_with_opts(
    inp: &VipsImage,
    pngsave_buffer_options: &PngsaveBufferOptions,
) -> Result<Vec<u8>> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut buffer_buf_size: u64 = 0;
        let mut buffer_out: *mut c_void = null_mut();

        let compression_in: i32 = pngsave_buffer_options.compression;
        let compression_in_name = utils::new_c_string("compression")?;

        let interlace_in: i32 = if pngsave_buffer_options.interlace { 1 } else { 0 };
        let interlace_in_name = utils::new_c_string("interlace")?;

        let filter_in: i32 = pngsave_buffer_options.filter as i32;
        let filter_in_name = utils::new_c_string("filter")?;

        let palette_in: i32 = if pngsave_buffer_options.palette { 1 } else { 0 };
        let palette_in_name = utils::new_c_string("palette")?;

        let q_in: i32 = pngsave_buffer_options.q;
        let q_in_name = utils::new_c_string("Q")?;

        let dither_in: f64 = pngsave_buffer_options.dither;
        let dither_in_name = utils::new_c_string("dither")?;

        let bitdepth_in: i32 = pngsave_buffer_options.bitdepth;
        let bitdepth_in_name = utils::new_c_string("bitdepth")?;

        let effort_in: i32 = pngsave_buffer_options.effort;
        let effort_in_name = utils::new_c_string("effort")?;

        let keep_in: i32 = pngsave_buffer_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&pngsave_buffer_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = pngsave_buffer_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&pngsave_buffer_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_pngsave_buffer(
            inp_in,
            &mut buffer_out,
            &mut buffer_buf_size,
            compression_in_name.as_ptr(),
            compression_in,
            interlace_in_name.as_ptr(),
            interlace_in,
            filter_in_name.as_ptr(),
            filter_in,
            palette_in_name.as_ptr(),
            palette_in,
            q_in_name.as_ptr(),
            q_in,
            dither_in_name.as_ptr(),
            dither_in,
            bitdepth_in_name.as_ptr(),
            bitdepth_in,
            effort_in_name.as_ptr(),
            effort_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            utils::new_byte_array(
                buffer_out,
                buffer_buf_size,
            ),
            Error::PngsaveBufferError,
        )
    }
}

/// VipsForeignSavePngTarget (pngsave_target), save image to target as PNG (.png), priority=0, rgba
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to

pub fn pngsave_target(inp: &VipsImage, target: &VipsTarget) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let vips_op_response = bindings::vips_pngsave_target(
            inp_in,
            target_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::PngsaveTargetError,
        )
    }
}

/// Options for pngsave_target operation
#[derive(Clone, Debug)]
pub struct PngsaveTargetOptions {
    /// compression: `i32` -> Compression factor
    /// min: 0, max: 9, default: 6
    pub compression: i32,
    /// interlace: `bool` -> Interlace image
    /// default: false
    pub interlace: bool,
    /// filter: `ForeignPngFilter` -> libpng row filter flag(s)
    ///  `None` -> VIPS_FOREIGN_PNG_FILTER_NONE = 8 [DEFAULT]
    ///  `Sub` -> VIPS_FOREIGN_PNG_FILTER_SUB = 16
    ///  `Up` -> VIPS_FOREIGN_PNG_FILTER_UP = 32
    ///  `Avg` -> VIPS_FOREIGN_PNG_FILTER_AVG = 64
    ///  `Paeth` -> VIPS_FOREIGN_PNG_FILTER_PAETH = 128
    ///  `All` -> VIPS_FOREIGN_PNG_FILTER_ALL = 248
    pub filter: ForeignPngFilter,
    /// palette: `bool` -> Quantise to 8bpp palette
    /// default: false
    pub palette: bool,
    /// q: `i32` -> Quantisation quality
    /// min: 0, max: 100, default: 100
    pub q: i32,
    /// dither: `f64` -> Amount of dithering
    /// min: 0, max: 1, default: 1
    pub dither: f64,
    /// bitdepth: `i32` -> Write as a 1, 2, 4, 8 or 16 bit image
    /// min: 1, max: 16, default: 8
    pub bitdepth: i32,
    /// effort: `i32` -> Quantisation CPU effort
    /// min: 1, max: 10, default: 7
    pub effort: i32,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for PngsaveTargetOptions {
    fn default() -> Self {
        PngsaveTargetOptions {
            compression: i32::from(6),
            interlace: false,
            filter: ForeignPngFilter::None,
            palette: false,
            q: i32::from(100),
            dither: f64::from(1),
            bitdepth: i32::from(8),
            effort: i32::from(7),
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSavePngTarget (pngsave_target), save image to target as PNG (.png), priority=0, rgba
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to
/// pngsave_target_options: `&PngsaveTargetOptions` -> optional arguments

pub fn pngsave_target_with_opts(
    inp: &VipsImage,
    target: &VipsTarget,
    pngsave_target_options: &PngsaveTargetOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let compression_in: i32 = pngsave_target_options.compression;
        let compression_in_name = utils::new_c_string("compression")?;

        let interlace_in: i32 = if pngsave_target_options.interlace { 1 } else { 0 };
        let interlace_in_name = utils::new_c_string("interlace")?;

        let filter_in: i32 = pngsave_target_options.filter as i32;
        let filter_in_name = utils::new_c_string("filter")?;

        let palette_in: i32 = if pngsave_target_options.palette { 1 } else { 0 };
        let palette_in_name = utils::new_c_string("palette")?;

        let q_in: i32 = pngsave_target_options.q;
        let q_in_name = utils::new_c_string("Q")?;

        let dither_in: f64 = pngsave_target_options.dither;
        let dither_in_name = utils::new_c_string("dither")?;

        let bitdepth_in: i32 = pngsave_target_options.bitdepth;
        let bitdepth_in_name = utils::new_c_string("bitdepth")?;

        let effort_in: i32 = pngsave_target_options.effort;
        let effort_in_name = utils::new_c_string("effort")?;

        let keep_in: i32 = pngsave_target_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&pngsave_target_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = pngsave_target_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&pngsave_target_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_pngsave_target(
            inp_in,
            target_in,
            compression_in_name.as_ptr(),
            compression_in,
            interlace_in_name.as_ptr(),
            interlace_in,
            filter_in_name.as_ptr(),
            filter_in,
            palette_in_name.as_ptr(),
            palette_in,
            q_in_name.as_ptr(),
            q_in,
            dither_in_name.as_ptr(),
            dither_in,
            bitdepth_in_name.as_ptr(),
            bitdepth_in,
            effort_in_name.as_ptr(),
            effort_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::PngsaveTargetError,
        )
    }
}

/// VipsForeignSaveJpegFile (jpegsave), save image to jpeg file (.jpg, .jpeg, .jpe), priority=0, rgb-cmyk
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to

pub fn jpegsave(inp: &VipsImage, filename: &str) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let vips_op_response = bindings::vips_jpegsave(
            inp_in,
            filename_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::JpegsaveError,
        )
    }
}

/// Options for jpegsave operation
#[derive(Clone, Debug)]
pub struct JpegsaveOptions {
    /// q: `i32` -> Q factor
    /// min: 1, max: 100, default: 75
    pub q: i32,
    /// optimize_coding: `bool` -> Compute optimal Huffman coding tables
    /// default: false
    pub optimize_coding: bool,
    /// interlace: `bool` -> Generate an interlaced (progressive) jpeg
    /// default: false
    pub interlace: bool,
    /// trellis_quant: `bool` -> Apply trellis quantisation to each 8x8 block
    /// default: false
    pub trellis_quant: bool,
    /// overshoot_deringing: `bool` -> Apply overshooting to samples with extreme values
    /// default: false
    pub overshoot_deringing: bool,
    /// optimize_scans: `bool` -> Split spectrum of DCT coefficients into separate scans
    /// default: false
    pub optimize_scans: bool,
    /// quant_table: `i32` -> Use predefined quantization table with given index
    /// min: 0, max: 8, default: 0
    pub quant_table: i32,
    /// subsample_mode: `ForeignSubsample` -> Select chroma subsample operation mode
    ///  `Auto` -> VIPS_FOREIGN_SUBSAMPLE_AUTO = 0 [DEFAULT]
    ///  `On` -> VIPS_FOREIGN_SUBSAMPLE_ON = 1
    ///  `Off` -> VIPS_FOREIGN_SUBSAMPLE_OFF = 2
    ///  `Last` -> VIPS_FOREIGN_SUBSAMPLE_LAST = 3
    pub subsample_mode: ForeignSubsample,
    /// restart_interval: `i32` -> Add restart markers every specified number of mcu
    /// min: 0, max: 2147483647, default: 0
    pub restart_interval: i32,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for JpegsaveOptions {
    fn default() -> Self {
        JpegsaveOptions {
            q: i32::from(75),
            optimize_coding: false,
            interlace: false,
            trellis_quant: false,
            overshoot_deringing: false,
            optimize_scans: false,
            quant_table: i32::from(0),
            subsample_mode: ForeignSubsample::Auto,
            restart_interval: i32::from(0),
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveJpegFile (jpegsave), save image to jpeg file (.jpg, .jpeg, .jpe), priority=0, rgb-cmyk
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to
/// jpegsave_options: `&JpegsaveOptions` -> optional arguments

pub fn jpegsave_with_opts(
    inp: &VipsImage,
    filename: &str,
    jpegsave_options: &JpegsaveOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let q_in: i32 = jpegsave_options.q;
        let q_in_name = utils::new_c_string("Q")?;

        let optimize_coding_in: i32 = if jpegsave_options.optimize_coding { 1 } else { 0 };
        let optimize_coding_in_name = utils::new_c_string("optimize-coding")?;

        let interlace_in: i32 = if jpegsave_options.interlace { 1 } else { 0 };
        let interlace_in_name = utils::new_c_string("interlace")?;

        let trellis_quant_in: i32 = if jpegsave_options.trellis_quant { 1 } else { 0 };
        let trellis_quant_in_name = utils::new_c_string("trellis-quant")?;

        let overshoot_deringing_in: i32 = if jpegsave_options.overshoot_deringing { 1 } else { 0 };
        let overshoot_deringing_in_name = utils::new_c_string("overshoot-deringing")?;

        let optimize_scans_in: i32 = if jpegsave_options.optimize_scans { 1 } else { 0 };
        let optimize_scans_in_name = utils::new_c_string("optimize-scans")?;

        let quant_table_in: i32 = jpegsave_options.quant_table;
        let quant_table_in_name = utils::new_c_string("quant-table")?;

        let subsample_mode_in: i32 = jpegsave_options.subsample_mode as i32;
        let subsample_mode_in_name = utils::new_c_string("subsample-mode")?;

        let restart_interval_in: i32 = jpegsave_options.restart_interval;
        let restart_interval_in_name = utils::new_c_string("restart-interval")?;

        let keep_in: i32 = jpegsave_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&jpegsave_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = jpegsave_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&jpegsave_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_jpegsave(
            inp_in,
            filename_in.as_ptr(),
            q_in_name.as_ptr(),
            q_in,
            optimize_coding_in_name.as_ptr(),
            optimize_coding_in,
            interlace_in_name.as_ptr(),
            interlace_in,
            trellis_quant_in_name.as_ptr(),
            trellis_quant_in,
            overshoot_deringing_in_name.as_ptr(),
            overshoot_deringing_in,
            optimize_scans_in_name.as_ptr(),
            optimize_scans_in,
            quant_table_in_name.as_ptr(),
            quant_table_in,
            subsample_mode_in_name.as_ptr(),
            subsample_mode_in,
            restart_interval_in_name.as_ptr(),
            restart_interval_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::JpegsaveError,
        )
    }
}

/// VipsForeignSaveJpegBuffer (jpegsave_buffer), save image to jpeg buffer (.jpg, .jpeg, .jpe), priority=0, rgb-cmyk
/// inp: `&VipsImage` -> Image to save
/// returns `Vec<u8>` - Buffer to save to
pub fn jpegsave_buffer(inp: &VipsImage) -> Result<Vec<u8>> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut buffer_buf_size: u64 = 0;
        let mut buffer_out: *mut c_void = null_mut();

        let vips_op_response = bindings::vips_jpegsave_buffer(
            inp_in,
            &mut buffer_out,
            &mut buffer_buf_size,
            NULL,
        );
        utils::result(
            vips_op_response,
            utils::new_byte_array(
                buffer_out,
                buffer_buf_size,
            ),
            Error::JpegsaveBufferError,
        )
    }
}

/// Options for jpegsave_buffer operation
#[derive(Clone, Debug)]
pub struct JpegsaveBufferOptions {
    /// q: `i32` -> Q factor
    /// min: 1, max: 100, default: 75
    pub q: i32,
    /// optimize_coding: `bool` -> Compute optimal Huffman coding tables
    /// default: false
    pub optimize_coding: bool,
    /// interlace: `bool` -> Generate an interlaced (progressive) jpeg
    /// default: false
    pub interlace: bool,
    /// trellis_quant: `bool` -> Apply trellis quantisation to each 8x8 block
    /// default: false
    pub trellis_quant: bool,
    /// overshoot_deringing: `bool` -> Apply overshooting to samples with extreme values
    /// default: false
    pub overshoot_deringing: bool,
    /// optimize_scans: `bool` -> Split spectrum of DCT coefficients into separate scans
    /// default: false
    pub optimize_scans: bool,
    /// quant_table: `i32` -> Use predefined quantization table with given index
    /// min: 0, max: 8, default: 0
    pub quant_table: i32,
    /// subsample_mode: `ForeignSubsample` -> Select chroma subsample operation mode
    ///  `Auto` -> VIPS_FOREIGN_SUBSAMPLE_AUTO = 0 [DEFAULT]
    ///  `On` -> VIPS_FOREIGN_SUBSAMPLE_ON = 1
    ///  `Off` -> VIPS_FOREIGN_SUBSAMPLE_OFF = 2
    ///  `Last` -> VIPS_FOREIGN_SUBSAMPLE_LAST = 3
    pub subsample_mode: ForeignSubsample,
    /// restart_interval: `i32` -> Add restart markers every specified number of mcu
    /// min: 0, max: 2147483647, default: 0
    pub restart_interval: i32,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for JpegsaveBufferOptions {
    fn default() -> Self {
        JpegsaveBufferOptions {
            q: i32::from(75),
            optimize_coding: false,
            interlace: false,
            trellis_quant: false,
            overshoot_deringing: false,
            optimize_scans: false,
            quant_table: i32::from(0),
            subsample_mode: ForeignSubsample::Auto,
            restart_interval: i32::from(0),
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveJpegBuffer (jpegsave_buffer), save image to jpeg buffer (.jpg, .jpeg, .jpe), priority=0, rgb-cmyk
/// inp: `&VipsImage` -> Image to save
/// jpegsave_buffer_options: `&JpegsaveBufferOptions` -> optional arguments
/// returns `Vec<u8>` - Buffer to save to
pub fn jpegsave_buffer_with_opts(
    inp: &VipsImage,
    jpegsave_buffer_options: &JpegsaveBufferOptions,
) -> Result<Vec<u8>> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut buffer_buf_size: u64 = 0;
        let mut buffer_out: *mut c_void = null_mut();

        let q_in: i32 = jpegsave_buffer_options.q;
        let q_in_name = utils::new_c_string("Q")?;

        let optimize_coding_in: i32 = if jpegsave_buffer_options.optimize_coding { 1 } else { 0 };
        let optimize_coding_in_name = utils::new_c_string("optimize-coding")?;

        let interlace_in: i32 = if jpegsave_buffer_options.interlace { 1 } else { 0 };
        let interlace_in_name = utils::new_c_string("interlace")?;

        let trellis_quant_in: i32 = if jpegsave_buffer_options.trellis_quant { 1 } else { 0 };
        let trellis_quant_in_name = utils::new_c_string("trellis-quant")?;

        let overshoot_deringing_in: i32 =
            if jpegsave_buffer_options.overshoot_deringing { 1 } else { 0 };
        let overshoot_deringing_in_name = utils::new_c_string("overshoot-deringing")?;

        let optimize_scans_in: i32 = if jpegsave_buffer_options.optimize_scans { 1 } else { 0 };
        let optimize_scans_in_name = utils::new_c_string("optimize-scans")?;

        let quant_table_in: i32 = jpegsave_buffer_options.quant_table;
        let quant_table_in_name = utils::new_c_string("quant-table")?;

        let subsample_mode_in: i32 = jpegsave_buffer_options.subsample_mode as i32;
        let subsample_mode_in_name = utils::new_c_string("subsample-mode")?;

        let restart_interval_in: i32 = jpegsave_buffer_options.restart_interval;
        let restart_interval_in_name = utils::new_c_string("restart-interval")?;

        let keep_in: i32 = jpegsave_buffer_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&jpegsave_buffer_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = jpegsave_buffer_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&jpegsave_buffer_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_jpegsave_buffer(
            inp_in,
            &mut buffer_out,
            &mut buffer_buf_size,
            q_in_name.as_ptr(),
            q_in,
            optimize_coding_in_name.as_ptr(),
            optimize_coding_in,
            interlace_in_name.as_ptr(),
            interlace_in,
            trellis_quant_in_name.as_ptr(),
            trellis_quant_in,
            overshoot_deringing_in_name.as_ptr(),
            overshoot_deringing_in,
            optimize_scans_in_name.as_ptr(),
            optimize_scans_in,
            quant_table_in_name.as_ptr(),
            quant_table_in,
            subsample_mode_in_name.as_ptr(),
            subsample_mode_in,
            restart_interval_in_name.as_ptr(),
            restart_interval_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            utils::new_byte_array(
                buffer_out,
                buffer_buf_size,
            ),
            Error::JpegsaveBufferError,
        )
    }
}

/// VipsForeignSaveJpegTarget (jpegsave_target), save image to jpeg target (.jpg, .jpeg, .jpe), priority=0, rgb-cmyk
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to

pub fn jpegsave_target(inp: &VipsImage, target: &VipsTarget) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let vips_op_response = bindings::vips_jpegsave_target(
            inp_in,
            target_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::JpegsaveTargetError,
        )
    }
}

/// Options for jpegsave_target operation
#[derive(Clone, Debug)]
pub struct JpegsaveTargetOptions {
    /// q: `i32` -> Q factor
    /// min: 1, max: 100, default: 75
    pub q: i32,
    /// optimize_coding: `bool` -> Compute optimal Huffman coding tables
    /// default: false
    pub optimize_coding: bool,
    /// interlace: `bool` -> Generate an interlaced (progressive) jpeg
    /// default: false
    pub interlace: bool,
    /// trellis_quant: `bool` -> Apply trellis quantisation to each 8x8 block
    /// default: false
    pub trellis_quant: bool,
    /// overshoot_deringing: `bool` -> Apply overshooting to samples with extreme values
    /// default: false
    pub overshoot_deringing: bool,
    /// optimize_scans: `bool` -> Split spectrum of DCT coefficients into separate scans
    /// default: false
    pub optimize_scans: bool,
    /// quant_table: `i32` -> Use predefined quantization table with given index
    /// min: 0, max: 8, default: 0
    pub quant_table: i32,
    /// subsample_mode: `ForeignSubsample` -> Select chroma subsample operation mode
    ///  `Auto` -> VIPS_FOREIGN_SUBSAMPLE_AUTO = 0 [DEFAULT]
    ///  `On` -> VIPS_FOREIGN_SUBSAMPLE_ON = 1
    ///  `Off` -> VIPS_FOREIGN_SUBSAMPLE_OFF = 2
    ///  `Last` -> VIPS_FOREIGN_SUBSAMPLE_LAST = 3
    pub subsample_mode: ForeignSubsample,
    /// restart_interval: `i32` -> Add restart markers every specified number of mcu
    /// min: 0, max: 2147483647, default: 0
    pub restart_interval: i32,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for JpegsaveTargetOptions {
    fn default() -> Self {
        JpegsaveTargetOptions {
            q: i32::from(75),
            optimize_coding: false,
            interlace: false,
            trellis_quant: false,
            overshoot_deringing: false,
            optimize_scans: false,
            quant_table: i32::from(0),
            subsample_mode: ForeignSubsample::Auto,
            restart_interval: i32::from(0),
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveJpegTarget (jpegsave_target), save image to jpeg target (.jpg, .jpeg, .jpe), priority=0, rgb-cmyk
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to
/// jpegsave_target_options: `&JpegsaveTargetOptions` -> optional arguments

pub fn jpegsave_target_with_opts(
    inp: &VipsImage,
    target: &VipsTarget,
    jpegsave_target_options: &JpegsaveTargetOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let q_in: i32 = jpegsave_target_options.q;
        let q_in_name = utils::new_c_string("Q")?;

        let optimize_coding_in: i32 = if jpegsave_target_options.optimize_coding { 1 } else { 0 };
        let optimize_coding_in_name = utils::new_c_string("optimize-coding")?;

        let interlace_in: i32 = if jpegsave_target_options.interlace { 1 } else { 0 };
        let interlace_in_name = utils::new_c_string("interlace")?;

        let trellis_quant_in: i32 = if jpegsave_target_options.trellis_quant { 1 } else { 0 };
        let trellis_quant_in_name = utils::new_c_string("trellis-quant")?;

        let overshoot_deringing_in: i32 =
            if jpegsave_target_options.overshoot_deringing { 1 } else { 0 };
        let overshoot_deringing_in_name = utils::new_c_string("overshoot-deringing")?;

        let optimize_scans_in: i32 = if jpegsave_target_options.optimize_scans { 1 } else { 0 };
        let optimize_scans_in_name = utils::new_c_string("optimize-scans")?;

        let quant_table_in: i32 = jpegsave_target_options.quant_table;
        let quant_table_in_name = utils::new_c_string("quant-table")?;

        let subsample_mode_in: i32 = jpegsave_target_options.subsample_mode as i32;
        let subsample_mode_in_name = utils::new_c_string("subsample-mode")?;

        let restart_interval_in: i32 = jpegsave_target_options.restart_interval;
        let restart_interval_in_name = utils::new_c_string("restart-interval")?;

        let keep_in: i32 = jpegsave_target_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&jpegsave_target_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = jpegsave_target_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&jpegsave_target_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_jpegsave_target(
            inp_in,
            target_in,
            q_in_name.as_ptr(),
            q_in,
            optimize_coding_in_name.as_ptr(),
            optimize_coding_in,
            interlace_in_name.as_ptr(),
            interlace_in,
            trellis_quant_in_name.as_ptr(),
            trellis_quant_in,
            overshoot_deringing_in_name.as_ptr(),
            overshoot_deringing_in,
            optimize_scans_in_name.as_ptr(),
            optimize_scans_in,
            quant_table_in_name.as_ptr(),
            quant_table_in,
            subsample_mode_in_name.as_ptr(),
            subsample_mode_in,
            restart_interval_in_name.as_ptr(),
            restart_interval_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::JpegsaveTargetError,
        )
    }
}

/// VipsForeignSaveJpegMime (jpegsave_mime), save image to jpeg mime (.jpg, .jpeg, .jpe), priority=0, rgb-cmyk
/// inp: `&VipsImage` -> Image to save

pub fn jpegsave_mime(inp: &VipsImage) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;

        let vips_op_response = bindings::vips_jpegsave_mime(
            inp_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::JpegsaveMimeError,
        )
    }
}

/// Options for jpegsave_mime operation
#[derive(Clone, Debug)]
pub struct JpegsaveMimeOptions {
    /// q: `i32` -> Q factor
    /// min: 1, max: 100, default: 75
    pub q: i32,
    /// optimize_coding: `bool` -> Compute optimal Huffman coding tables
    /// default: false
    pub optimize_coding: bool,
    /// interlace: `bool` -> Generate an interlaced (progressive) jpeg
    /// default: false
    pub interlace: bool,
    /// trellis_quant: `bool` -> Apply trellis quantisation to each 8x8 block
    /// default: false
    pub trellis_quant: bool,
    /// overshoot_deringing: `bool` -> Apply overshooting to samples with extreme values
    /// default: false
    pub overshoot_deringing: bool,
    /// optimize_scans: `bool` -> Split spectrum of DCT coefficients into separate scans
    /// default: false
    pub optimize_scans: bool,
    /// quant_table: `i32` -> Use predefined quantization table with given index
    /// min: 0, max: 8, default: 0
    pub quant_table: i32,
    /// subsample_mode: `ForeignSubsample` -> Select chroma subsample operation mode
    ///  `Auto` -> VIPS_FOREIGN_SUBSAMPLE_AUTO = 0 [DEFAULT]
    ///  `On` -> VIPS_FOREIGN_SUBSAMPLE_ON = 1
    ///  `Off` -> VIPS_FOREIGN_SUBSAMPLE_OFF = 2
    ///  `Last` -> VIPS_FOREIGN_SUBSAMPLE_LAST = 3
    pub subsample_mode: ForeignSubsample,
    /// restart_interval: `i32` -> Add restart markers every specified number of mcu
    /// min: 0, max: 2147483647, default: 0
    pub restart_interval: i32,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for JpegsaveMimeOptions {
    fn default() -> Self {
        JpegsaveMimeOptions {
            q: i32::from(75),
            optimize_coding: false,
            interlace: false,
            trellis_quant: false,
            overshoot_deringing: false,
            optimize_scans: false,
            quant_table: i32::from(0),
            subsample_mode: ForeignSubsample::Auto,
            restart_interval: i32::from(0),
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveJpegMime (jpegsave_mime), save image to jpeg mime (.jpg, .jpeg, .jpe), priority=0, rgb-cmyk
/// inp: `&VipsImage` -> Image to save
/// jpegsave_mime_options: `&JpegsaveMimeOptions` -> optional arguments

pub fn jpegsave_mime_with_opts(
    inp: &VipsImage,
    jpegsave_mime_options: &JpegsaveMimeOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;

        let q_in: i32 = jpegsave_mime_options.q;
        let q_in_name = utils::new_c_string("Q")?;

        let optimize_coding_in: i32 = if jpegsave_mime_options.optimize_coding { 1 } else { 0 };
        let optimize_coding_in_name = utils::new_c_string("optimize-coding")?;

        let interlace_in: i32 = if jpegsave_mime_options.interlace { 1 } else { 0 };
        let interlace_in_name = utils::new_c_string("interlace")?;

        let trellis_quant_in: i32 = if jpegsave_mime_options.trellis_quant { 1 } else { 0 };
        let trellis_quant_in_name = utils::new_c_string("trellis-quant")?;

        let overshoot_deringing_in: i32 =
            if jpegsave_mime_options.overshoot_deringing { 1 } else { 0 };
        let overshoot_deringing_in_name = utils::new_c_string("overshoot-deringing")?;

        let optimize_scans_in: i32 = if jpegsave_mime_options.optimize_scans { 1 } else { 0 };
        let optimize_scans_in_name = utils::new_c_string("optimize-scans")?;

        let quant_table_in: i32 = jpegsave_mime_options.quant_table;
        let quant_table_in_name = utils::new_c_string("quant-table")?;

        let subsample_mode_in: i32 = jpegsave_mime_options.subsample_mode as i32;
        let subsample_mode_in_name = utils::new_c_string("subsample-mode")?;

        let restart_interval_in: i32 = jpegsave_mime_options.restart_interval;
        let restart_interval_in_name = utils::new_c_string("restart-interval")?;

        let keep_in: i32 = jpegsave_mime_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&jpegsave_mime_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = jpegsave_mime_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&jpegsave_mime_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_jpegsave_mime(
            inp_in,
            q_in_name.as_ptr(),
            q_in,
            optimize_coding_in_name.as_ptr(),
            optimize_coding_in,
            interlace_in_name.as_ptr(),
            interlace_in,
            trellis_quant_in_name.as_ptr(),
            trellis_quant_in,
            overshoot_deringing_in_name.as_ptr(),
            overshoot_deringing_in,
            optimize_scans_in_name.as_ptr(),
            optimize_scans_in,
            quant_table_in_name.as_ptr(),
            quant_table_in,
            subsample_mode_in_name.as_ptr(),
            subsample_mode_in,
            restart_interval_in_name.as_ptr(),
            restart_interval_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::JpegsaveMimeError,
        )
    }
}

/// VipsForeignSaveWebpFile (webpsave), save as WebP (.webp), priority=0, rgba-only
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to

pub fn webpsave(inp: &VipsImage, filename: &str) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let vips_op_response = bindings::vips_webpsave(
            inp_in,
            filename_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::WebpsaveError,
        )
    }
}

/// Options for webpsave operation
#[derive(Clone, Debug)]
pub struct WebpsaveOptions {
    /// q: `i32` -> Q factor
    /// min: 0, max: 100, default: 75
    pub q: i32,
    /// lossless: `bool` -> Enable lossless compression
    /// default: false
    pub lossless: bool,
    /// preset: `ForeignWebpPreset` -> Preset for lossy compression
    ///  `Default` -> VIPS_FOREIGN_WEBP_PRESET_DEFAULT = 0 [DEFAULT]
    ///  `Picture` -> VIPS_FOREIGN_WEBP_PRESET_PICTURE = 1
    ///  `Photo` -> VIPS_FOREIGN_WEBP_PRESET_PHOTO = 2
    ///  `Drawing` -> VIPS_FOREIGN_WEBP_PRESET_DRAWING = 3
    ///  `Icon` -> VIPS_FOREIGN_WEBP_PRESET_ICON = 4
    ///  `Text` -> VIPS_FOREIGN_WEBP_PRESET_TEXT = 5
    ///  `Last` -> VIPS_FOREIGN_WEBP_PRESET_LAST = 6
    pub preset: ForeignWebpPreset,
    /// smart_subsample: `bool` -> Enable high quality chroma subsampling
    /// default: false
    pub smart_subsample: bool,
    /// near_lossless: `bool` -> Enable preprocessing in lossless mode (uses Q)
    /// default: false
    pub near_lossless: bool,
    /// alpha_q: `i32` -> Change alpha plane fidelity for lossy compression
    /// min: 0, max: 100, default: 100
    pub alpha_q: i32,
    /// min_size: `bool` -> Optimise for minimum size
    /// default: false
    pub min_size: bool,
    /// kmin: `i32` -> Minimum number of frames between key frames
    /// min: 0, max: 2147483647, default: 2147483646
    pub kmin: i32,
    /// kmax: `i32` -> Maximum number of frames between key frames
    /// min: 0, max: 2147483647, default: 2147483647
    pub kmax: i32,
    /// effort: `i32` -> Level of CPU effort to reduce file size
    /// min: 0, max: 6, default: 4
    pub effort: i32,
    /// mixed: `bool` -> Allow mixed encoding (might reduce file size)
    /// default: false
    pub mixed: bool,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for WebpsaveOptions {
    fn default() -> Self {
        WebpsaveOptions {
            q: i32::from(75),
            lossless: false,
            preset: ForeignWebpPreset::Default,
            smart_subsample: false,
            near_lossless: false,
            alpha_q: i32::from(100),
            min_size: false,
            kmin: i32::from(2147483646),
            kmax: i32::from(2147483647),
            effort: i32::from(4),
            mixed: false,
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveWebpFile (webpsave), save as WebP (.webp), priority=0, rgba-only
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to
/// webpsave_options: `&WebpsaveOptions` -> optional arguments

pub fn webpsave_with_opts(
    inp: &VipsImage,
    filename: &str,
    webpsave_options: &WebpsaveOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let q_in: i32 = webpsave_options.q;
        let q_in_name = utils::new_c_string("Q")?;

        let lossless_in: i32 = if webpsave_options.lossless { 1 } else { 0 };
        let lossless_in_name = utils::new_c_string("lossless")?;

        let preset_in: i32 = webpsave_options.preset as i32;
        let preset_in_name = utils::new_c_string("preset")?;

        let smart_subsample_in: i32 = if webpsave_options.smart_subsample { 1 } else { 0 };
        let smart_subsample_in_name = utils::new_c_string("smart-subsample")?;

        let near_lossless_in: i32 = if webpsave_options.near_lossless { 1 } else { 0 };
        let near_lossless_in_name = utils::new_c_string("near-lossless")?;

        let alpha_q_in: i32 = webpsave_options.alpha_q;
        let alpha_q_in_name = utils::new_c_string("alpha-q")?;

        let min_size_in: i32 = if webpsave_options.min_size { 1 } else { 0 };
        let min_size_in_name = utils::new_c_string("min-size")?;

        let kmin_in: i32 = webpsave_options.kmin;
        let kmin_in_name = utils::new_c_string("kmin")?;

        let kmax_in: i32 = webpsave_options.kmax;
        let kmax_in_name = utils::new_c_string("kmax")?;

        let effort_in: i32 = webpsave_options.effort;
        let effort_in_name = utils::new_c_string("effort")?;

        let mixed_in: i32 = if webpsave_options.mixed { 1 } else { 0 };
        let mixed_in_name = utils::new_c_string("mixed")?;

        let keep_in: i32 = webpsave_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&webpsave_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = webpsave_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&webpsave_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_webpsave(
            inp_in,
            filename_in.as_ptr(),
            q_in_name.as_ptr(),
            q_in,
            lossless_in_name.as_ptr(),
            lossless_in,
            preset_in_name.as_ptr(),
            preset_in,
            smart_subsample_in_name.as_ptr(),
            smart_subsample_in,
            near_lossless_in_name.as_ptr(),
            near_lossless_in,
            alpha_q_in_name.as_ptr(),
            alpha_q_in,
            min_size_in_name.as_ptr(),
            min_size_in,
            kmin_in_name.as_ptr(),
            kmin_in,
            kmax_in_name.as_ptr(),
            kmax_in,
            effort_in_name.as_ptr(),
            effort_in,
            mixed_in_name.as_ptr(),
            mixed_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::WebpsaveError,
        )
    }
}

/// VipsForeignSaveWebpBuffer (webpsave_buffer), save as WebP (.webp), priority=0, rgba-only
/// inp: `&VipsImage` -> Image to save
/// returns `Vec<u8>` - Buffer to save to
pub fn webpsave_buffer(inp: &VipsImage) -> Result<Vec<u8>> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut buffer_buf_size: u64 = 0;
        let mut buffer_out: *mut c_void = null_mut();

        let vips_op_response = bindings::vips_webpsave_buffer(
            inp_in,
            &mut buffer_out,
            &mut buffer_buf_size,
            NULL,
        );
        utils::result(
            vips_op_response,
            utils::new_byte_array(
                buffer_out,
                buffer_buf_size,
            ),
            Error::WebpsaveBufferError,
        )
    }
}

/// Options for webpsave_buffer operation
#[derive(Clone, Debug)]
pub struct WebpsaveBufferOptions {
    /// q: `i32` -> Q factor
    /// min: 0, max: 100, default: 75
    pub q: i32,
    /// lossless: `bool` -> Enable lossless compression
    /// default: false
    pub lossless: bool,
    /// preset: `ForeignWebpPreset` -> Preset for lossy compression
    ///  `Default` -> VIPS_FOREIGN_WEBP_PRESET_DEFAULT = 0 [DEFAULT]
    ///  `Picture` -> VIPS_FOREIGN_WEBP_PRESET_PICTURE = 1
    ///  `Photo` -> VIPS_FOREIGN_WEBP_PRESET_PHOTO = 2
    ///  `Drawing` -> VIPS_FOREIGN_WEBP_PRESET_DRAWING = 3
    ///  `Icon` -> VIPS_FOREIGN_WEBP_PRESET_ICON = 4
    ///  `Text` -> VIPS_FOREIGN_WEBP_PRESET_TEXT = 5
    ///  `Last` -> VIPS_FOREIGN_WEBP_PRESET_LAST = 6
    pub preset: ForeignWebpPreset,
    /// smart_subsample: `bool` -> Enable high quality chroma subsampling
    /// default: false
    pub smart_subsample: bool,
    /// near_lossless: `bool` -> Enable preprocessing in lossless mode (uses Q)
    /// default: false
    pub near_lossless: bool,
    /// alpha_q: `i32` -> Change alpha plane fidelity for lossy compression
    /// min: 0, max: 100, default: 100
    pub alpha_q: i32,
    /// min_size: `bool` -> Optimise for minimum size
    /// default: false
    pub min_size: bool,
    /// kmin: `i32` -> Minimum number of frames between key frames
    /// min: 0, max: 2147483647, default: 2147483646
    pub kmin: i32,
    /// kmax: `i32` -> Maximum number of frames between key frames
    /// min: 0, max: 2147483647, default: 2147483647
    pub kmax: i32,
    /// effort: `i32` -> Level of CPU effort to reduce file size
    /// min: 0, max: 6, default: 4
    pub effort: i32,
    /// mixed: `bool` -> Allow mixed encoding (might reduce file size)
    /// default: false
    pub mixed: bool,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for WebpsaveBufferOptions {
    fn default() -> Self {
        WebpsaveBufferOptions {
            q: i32::from(75),
            lossless: false,
            preset: ForeignWebpPreset::Default,
            smart_subsample: false,
            near_lossless: false,
            alpha_q: i32::from(100),
            min_size: false,
            kmin: i32::from(2147483646),
            kmax: i32::from(2147483647),
            effort: i32::from(4),
            mixed: false,
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveWebpBuffer (webpsave_buffer), save as WebP (.webp), priority=0, rgba-only
/// inp: `&VipsImage` -> Image to save
/// webpsave_buffer_options: `&WebpsaveBufferOptions` -> optional arguments
/// returns `Vec<u8>` - Buffer to save to
pub fn webpsave_buffer_with_opts(
    inp: &VipsImage,
    webpsave_buffer_options: &WebpsaveBufferOptions,
) -> Result<Vec<u8>> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut buffer_buf_size: u64 = 0;
        let mut buffer_out: *mut c_void = null_mut();

        let q_in: i32 = webpsave_buffer_options.q;
        let q_in_name = utils::new_c_string("Q")?;

        let lossless_in: i32 = if webpsave_buffer_options.lossless { 1 } else { 0 };
        let lossless_in_name = utils::new_c_string("lossless")?;

        let preset_in: i32 = webpsave_buffer_options.preset as i32;
        let preset_in_name = utils::new_c_string("preset")?;

        let smart_subsample_in: i32 = if webpsave_buffer_options.smart_subsample { 1 } else { 0 };
        let smart_subsample_in_name = utils::new_c_string("smart-subsample")?;

        let near_lossless_in: i32 = if webpsave_buffer_options.near_lossless { 1 } else { 0 };
        let near_lossless_in_name = utils::new_c_string("near-lossless")?;

        let alpha_q_in: i32 = webpsave_buffer_options.alpha_q;
        let alpha_q_in_name = utils::new_c_string("alpha-q")?;

        let min_size_in: i32 = if webpsave_buffer_options.min_size { 1 } else { 0 };
        let min_size_in_name = utils::new_c_string("min-size")?;

        let kmin_in: i32 = webpsave_buffer_options.kmin;
        let kmin_in_name = utils::new_c_string("kmin")?;

        let kmax_in: i32 = webpsave_buffer_options.kmax;
        let kmax_in_name = utils::new_c_string("kmax")?;

        let effort_in: i32 = webpsave_buffer_options.effort;
        let effort_in_name = utils::new_c_string("effort")?;

        let mixed_in: i32 = if webpsave_buffer_options.mixed { 1 } else { 0 };
        let mixed_in_name = utils::new_c_string("mixed")?;

        let keep_in: i32 = webpsave_buffer_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&webpsave_buffer_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = webpsave_buffer_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&webpsave_buffer_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_webpsave_buffer(
            inp_in,
            &mut buffer_out,
            &mut buffer_buf_size,
            q_in_name.as_ptr(),
            q_in,
            lossless_in_name.as_ptr(),
            lossless_in,
            preset_in_name.as_ptr(),
            preset_in,
            smart_subsample_in_name.as_ptr(),
            smart_subsample_in,
            near_lossless_in_name.as_ptr(),
            near_lossless_in,
            alpha_q_in_name.as_ptr(),
            alpha_q_in,
            min_size_in_name.as_ptr(),
            min_size_in,
            kmin_in_name.as_ptr(),
            kmin_in,
            kmax_in_name.as_ptr(),
            kmax_in,
            effort_in_name.as_ptr(),
            effort_in,
            mixed_in_name.as_ptr(),
            mixed_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            utils::new_byte_array(
                buffer_out,
                buffer_buf_size,
            ),
            Error::WebpsaveBufferError,
        )
    }
}

/// VipsForeignSaveWebpTarget (webpsave_target), save as WebP (.webp), priority=0, rgba-only
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to

pub fn webpsave_target(inp: &VipsImage, target: &VipsTarget) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let vips_op_response = bindings::vips_webpsave_target(
            inp_in,
            target_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::WebpsaveTargetError,
        )
    }
}

/// Options for webpsave_target operation
#[derive(Clone, Debug)]
pub struct WebpsaveTargetOptions {
    /// q: `i32` -> Q factor
    /// min: 0, max: 100, default: 75
    pub q: i32,
    /// lossless: `bool` -> Enable lossless compression
    /// default: false
    pub lossless: bool,
    /// preset: `ForeignWebpPreset` -> Preset for lossy compression
    ///  `Default` -> VIPS_FOREIGN_WEBP_PRESET_DEFAULT = 0 [DEFAULT]
    ///  `Picture` -> VIPS_FOREIGN_WEBP_PRESET_PICTURE = 1
    ///  `Photo` -> VIPS_FOREIGN_WEBP_PRESET_PHOTO = 2
    ///  `Drawing` -> VIPS_FOREIGN_WEBP_PRESET_DRAWING = 3
    ///  `Icon` -> VIPS_FOREIGN_WEBP_PRESET_ICON = 4
    ///  `Text` -> VIPS_FOREIGN_WEBP_PRESET_TEXT = 5
    ///  `Last` -> VIPS_FOREIGN_WEBP_PRESET_LAST = 6
    pub preset: ForeignWebpPreset,
    /// smart_subsample: `bool` -> Enable high quality chroma subsampling
    /// default: false
    pub smart_subsample: bool,
    /// near_lossless: `bool` -> Enable preprocessing in lossless mode (uses Q)
    /// default: false
    pub near_lossless: bool,
    /// alpha_q: `i32` -> Change alpha plane fidelity for lossy compression
    /// min: 0, max: 100, default: 100
    pub alpha_q: i32,
    /// min_size: `bool` -> Optimise for minimum size
    /// default: false
    pub min_size: bool,
    /// kmin: `i32` -> Minimum number of frames between key frames
    /// min: 0, max: 2147483647, default: 2147483646
    pub kmin: i32,
    /// kmax: `i32` -> Maximum number of frames between key frames
    /// min: 0, max: 2147483647, default: 2147483647
    pub kmax: i32,
    /// effort: `i32` -> Level of CPU effort to reduce file size
    /// min: 0, max: 6, default: 4
    pub effort: i32,
    /// mixed: `bool` -> Allow mixed encoding (might reduce file size)
    /// default: false
    pub mixed: bool,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for WebpsaveTargetOptions {
    fn default() -> Self {
        WebpsaveTargetOptions {
            q: i32::from(75),
            lossless: false,
            preset: ForeignWebpPreset::Default,
            smart_subsample: false,
            near_lossless: false,
            alpha_q: i32::from(100),
            min_size: false,
            kmin: i32::from(2147483646),
            kmax: i32::from(2147483647),
            effort: i32::from(4),
            mixed: false,
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveWebpTarget (webpsave_target), save as WebP (.webp), priority=0, rgba-only
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to
/// webpsave_target_options: `&WebpsaveTargetOptions` -> optional arguments

pub fn webpsave_target_with_opts(
    inp: &VipsImage,
    target: &VipsTarget,
    webpsave_target_options: &WebpsaveTargetOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let q_in: i32 = webpsave_target_options.q;
        let q_in_name = utils::new_c_string("Q")?;

        let lossless_in: i32 = if webpsave_target_options.lossless { 1 } else { 0 };
        let lossless_in_name = utils::new_c_string("lossless")?;

        let preset_in: i32 = webpsave_target_options.preset as i32;
        let preset_in_name = utils::new_c_string("preset")?;

        let smart_subsample_in: i32 = if webpsave_target_options.smart_subsample { 1 } else { 0 };
        let smart_subsample_in_name = utils::new_c_string("smart-subsample")?;

        let near_lossless_in: i32 = if webpsave_target_options.near_lossless { 1 } else { 0 };
        let near_lossless_in_name = utils::new_c_string("near-lossless")?;

        let alpha_q_in: i32 = webpsave_target_options.alpha_q;
        let alpha_q_in_name = utils::new_c_string("alpha-q")?;

        let min_size_in: i32 = if webpsave_target_options.min_size { 1 } else { 0 };
        let min_size_in_name = utils::new_c_string("min-size")?;

        let kmin_in: i32 = webpsave_target_options.kmin;
        let kmin_in_name = utils::new_c_string("kmin")?;

        let kmax_in: i32 = webpsave_target_options.kmax;
        let kmax_in_name = utils::new_c_string("kmax")?;

        let effort_in: i32 = webpsave_target_options.effort;
        let effort_in_name = utils::new_c_string("effort")?;

        let mixed_in: i32 = if webpsave_target_options.mixed { 1 } else { 0 };
        let mixed_in_name = utils::new_c_string("mixed")?;

        let keep_in: i32 = webpsave_target_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&webpsave_target_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = webpsave_target_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&webpsave_target_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_webpsave_target(
            inp_in,
            target_in,
            q_in_name.as_ptr(),
            q_in,
            lossless_in_name.as_ptr(),
            lossless_in,
            preset_in_name.as_ptr(),
            preset_in,
            smart_subsample_in_name.as_ptr(),
            smart_subsample_in,
            near_lossless_in_name.as_ptr(),
            near_lossless_in,
            alpha_q_in_name.as_ptr(),
            alpha_q_in,
            min_size_in_name.as_ptr(),
            min_size_in,
            kmin_in_name.as_ptr(),
            kmin_in,
            kmax_in_name.as_ptr(),
            kmax_in,
            effort_in_name.as_ptr(),
            effort_in,
            mixed_in_name.as_ptr(),
            mixed_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::WebpsaveTargetError,
        )
    }
}

/// VipsForeignSaveWebpMime (webpsave_mime), save image to webp mime (.webp), priority=0, rgba-only
/// inp: `&VipsImage` -> Image to save

pub fn webpsave_mime(inp: &VipsImage) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;

        let vips_op_response = bindings::vips_webpsave_mime(
            inp_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::WebpsaveMimeError,
        )
    }
}

/// Options for webpsave_mime operation
#[derive(Clone, Debug)]
pub struct WebpsaveMimeOptions {
    /// q: `i32` -> Q factor
    /// min: 0, max: 100, default: 75
    pub q: i32,
    /// lossless: `bool` -> Enable lossless compression
    /// default: false
    pub lossless: bool,
    /// preset: `ForeignWebpPreset` -> Preset for lossy compression
    ///  `Default` -> VIPS_FOREIGN_WEBP_PRESET_DEFAULT = 0 [DEFAULT]
    ///  `Picture` -> VIPS_FOREIGN_WEBP_PRESET_PICTURE = 1
    ///  `Photo` -> VIPS_FOREIGN_WEBP_PRESET_PHOTO = 2
    ///  `Drawing` -> VIPS_FOREIGN_WEBP_PRESET_DRAWING = 3
    ///  `Icon` -> VIPS_FOREIGN_WEBP_PRESET_ICON = 4
    ///  `Text` -> VIPS_FOREIGN_WEBP_PRESET_TEXT = 5
    ///  `Last` -> VIPS_FOREIGN_WEBP_PRESET_LAST = 6
    pub preset: ForeignWebpPreset,
    /// smart_subsample: `bool` -> Enable high quality chroma subsampling
    /// default: false
    pub smart_subsample: bool,
    /// near_lossless: `bool` -> Enable preprocessing in lossless mode (uses Q)
    /// default: false
    pub near_lossless: bool,
    /// alpha_q: `i32` -> Change alpha plane fidelity for lossy compression
    /// min: 0, max: 100, default: 100
    pub alpha_q: i32,
    /// min_size: `bool` -> Optimise for minimum size
    /// default: false
    pub min_size: bool,
    /// kmin: `i32` -> Minimum number of frames between key frames
    /// min: 0, max: 2147483647, default: 2147483646
    pub kmin: i32,
    /// kmax: `i32` -> Maximum number of frames between key frames
    /// min: 0, max: 2147483647, default: 2147483647
    pub kmax: i32,
    /// effort: `i32` -> Level of CPU effort to reduce file size
    /// min: 0, max: 6, default: 4
    pub effort: i32,
    /// mixed: `bool` -> Allow mixed encoding (might reduce file size)
    /// default: false
    pub mixed: bool,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for WebpsaveMimeOptions {
    fn default() -> Self {
        WebpsaveMimeOptions {
            q: i32::from(75),
            lossless: false,
            preset: ForeignWebpPreset::Default,
            smart_subsample: false,
            near_lossless: false,
            alpha_q: i32::from(100),
            min_size: false,
            kmin: i32::from(2147483646),
            kmax: i32::from(2147483647),
            effort: i32::from(4),
            mixed: false,
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveWebpMime (webpsave_mime), save image to webp mime (.webp), priority=0, rgba-only
/// inp: `&VipsImage` -> Image to save
/// webpsave_mime_options: `&WebpsaveMimeOptions` -> optional arguments

pub fn webpsave_mime_with_opts(
    inp: &VipsImage,
    webpsave_mime_options: &WebpsaveMimeOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;

        let q_in: i32 = webpsave_mime_options.q;
        let q_in_name = utils::new_c_string("Q")?;

        let lossless_in: i32 = if webpsave_mime_options.lossless { 1 } else { 0 };
        let lossless_in_name = utils::new_c_string("lossless")?;

        let preset_in: i32 = webpsave_mime_options.preset as i32;
        let preset_in_name = utils::new_c_string("preset")?;

        let smart_subsample_in: i32 = if webpsave_mime_options.smart_subsample { 1 } else { 0 };
        let smart_subsample_in_name = utils::new_c_string("smart-subsample")?;

        let near_lossless_in: i32 = if webpsave_mime_options.near_lossless { 1 } else { 0 };
        let near_lossless_in_name = utils::new_c_string("near-lossless")?;

        let alpha_q_in: i32 = webpsave_mime_options.alpha_q;
        let alpha_q_in_name = utils::new_c_string("alpha-q")?;

        let min_size_in: i32 = if webpsave_mime_options.min_size { 1 } else { 0 };
        let min_size_in_name = utils::new_c_string("min-size")?;

        let kmin_in: i32 = webpsave_mime_options.kmin;
        let kmin_in_name = utils::new_c_string("kmin")?;

        let kmax_in: i32 = webpsave_mime_options.kmax;
        let kmax_in_name = utils::new_c_string("kmax")?;

        let effort_in: i32 = webpsave_mime_options.effort;
        let effort_in_name = utils::new_c_string("effort")?;

        let mixed_in: i32 = if webpsave_mime_options.mixed { 1 } else { 0 };
        let mixed_in_name = utils::new_c_string("mixed")?;

        let keep_in: i32 = webpsave_mime_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&webpsave_mime_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = webpsave_mime_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&webpsave_mime_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_webpsave_mime(
            inp_in,
            q_in_name.as_ptr(),
            q_in,
            lossless_in_name.as_ptr(),
            lossless_in,
            preset_in_name.as_ptr(),
            preset_in,
            smart_subsample_in_name.as_ptr(),
            smart_subsample_in,
            near_lossless_in_name.as_ptr(),
            near_lossless_in,
            alpha_q_in_name.as_ptr(),
            alpha_q_in,
            min_size_in_name.as_ptr(),
            min_size_in,
            kmin_in_name.as_ptr(),
            kmin_in,
            kmax_in_name.as_ptr(),
            kmax_in,
            effort_in_name.as_ptr(),
            effort_in,
            mixed_in_name.as_ptr(),
            mixed_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::WebpsaveMimeError,
        )
    }
}

/// VipsForeignSaveTiffFile (tiffsave), save image to tiff file (.tif, .tiff), priority=0, any
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to

pub fn tiffsave(inp: &VipsImage, filename: &str) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let vips_op_response = bindings::vips_tiffsave(
            inp_in,
            filename_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::TiffsaveError,
        )
    }
}

/// Options for tiffsave operation
#[derive(Clone, Debug)]
pub struct TiffsaveOptions {
    /// compression: `ForeignTiffCompression` -> Compression for this file
    ///  `None` -> VIPS_FOREIGN_TIFF_COMPRESSION_NONE = 0 [DEFAULT]
    ///  `Jpeg` -> VIPS_FOREIGN_TIFF_COMPRESSION_JPEG = 1
    ///  `Deflate` -> VIPS_FOREIGN_TIFF_COMPRESSION_DEFLATE = 2
    ///  `Packbit` -> VIPS_FOREIGN_TIFF_COMPRESSION_PACKBITS = 3
    ///  `Ccittfax4` -> VIPS_FOREIGN_TIFF_COMPRESSION_CCITTFAX4 = 4
    ///  `Lzw` -> VIPS_FOREIGN_TIFF_COMPRESSION_LZW = 5
    ///  `Webp` -> VIPS_FOREIGN_TIFF_COMPRESSION_WEBP = 6
    ///  `Zstd` -> VIPS_FOREIGN_TIFF_COMPRESSION_ZSTD = 7
    ///  `Jp2K` -> VIPS_FOREIGN_TIFF_COMPRESSION_JP2K = 8
    ///  `Last` -> VIPS_FOREIGN_TIFF_COMPRESSION_LAST = 9
    pub compression: ForeignTiffCompression,
    /// q: `i32` -> Q factor
    /// min: 1, max: 100, default: 75
    pub q: i32,
    /// predictor: `ForeignTiffPredictor` -> Compression prediction
    ///  `None` -> VIPS_FOREIGN_TIFF_PREDICTOR_NONE = 1
    ///  `Horizontal` -> VIPS_FOREIGN_TIFF_PREDICTOR_HORIZONTAL = 2 [DEFAULT]
    ///  `Float` -> VIPS_FOREIGN_TIFF_PREDICTOR_FLOAT = 3
    ///  `Last` -> VIPS_FOREIGN_TIFF_PREDICTOR_LAST = 4
    pub predictor: ForeignTiffPredictor,
    /// tile: `bool` -> Write a tiled tiff
    /// default: false
    pub tile: bool,
    /// tile_width: `i32` -> Tile width in pixels
    /// min: 1, max: 32768, default: 128
    pub tile_width: i32,
    /// tile_height: `i32` -> Tile height in pixels
    /// min: 1, max: 32768, default: 128
    pub tile_height: i32,
    /// pyramid: `bool` -> Write a pyramidal tiff
    /// default: false
    pub pyramid: bool,
    /// miniswhite: `bool` -> Use 0 for white in 1-bit images
    /// default: false
    pub miniswhite: bool,
    /// bitdepth: `i32` -> Write as a 1, 2, 4 or 8 bit image
    /// min: 0, max: 8, default: 0
    pub bitdepth: i32,
    /// resunit: `ForeignTiffResunit` -> Resolution unit
    ///  `Cm` -> VIPS_FOREIGN_TIFF_RESUNIT_CM = 0 [DEFAULT]
    ///  `Inch` -> VIPS_FOREIGN_TIFF_RESUNIT_INCH = 1
    ///  `Last` -> VIPS_FOREIGN_TIFF_RESUNIT_LAST = 2
    pub resunit: ForeignTiffResunit,
    /// xres: `f64` -> Horizontal resolution in pixels/mm
    /// min: 0.001, max: 1000000, default: 1
    pub xres: f64,
    /// yres: `f64` -> Vertical resolution in pixels/mm
    /// min: 0.001, max: 1000000, default: 1
    pub yres: f64,
    /// bigtiff: `bool` -> Write a bigtiff image
    /// default: false
    pub bigtiff: bool,
    /// properties: `bool` -> Write a properties document to IMAGEDESCRIPTION
    /// default: false
    pub properties: bool,
    /// region_shrink: `RegionShrink` -> Method to shrink regions
    ///  `Mean` -> VIPS_REGION_SHRINK_MEAN = 0 [DEFAULT]
    ///  `Median` -> VIPS_REGION_SHRINK_MEDIAN = 1
    ///  `Mode` -> VIPS_REGION_SHRINK_MODE = 2
    ///  `Max` -> VIPS_REGION_SHRINK_MAX = 3
    ///  `Min` -> VIPS_REGION_SHRINK_MIN = 4
    ///  `Nearest` -> VIPS_REGION_SHRINK_NEAREST = 5
    ///  `Last` -> VIPS_REGION_SHRINK_LAST = 6
    pub region_shrink: RegionShrink,
    /// level: `i32` -> ZSTD compression level
    /// min: 1, max: 22, default: 10
    pub level: i32,
    /// lossless: `bool` -> Enable WEBP lossless mode
    /// default: false
    pub lossless: bool,
    /// depth: `ForeignDzDepth` -> Pyramid depth
    ///  `Onepixel` -> VIPS_FOREIGN_DZ_DEPTH_ONEPIXEL = 0
    ///  `Onetile` -> VIPS_FOREIGN_DZ_DEPTH_ONETILE = 1 [DEFAULT]
    ///  `One` -> VIPS_FOREIGN_DZ_DEPTH_ONE = 2
    ///  `Last` -> VIPS_FOREIGN_DZ_DEPTH_LAST = 3
    pub depth: ForeignDzDepth,
    /// subifd: `bool` -> Save pyr layers as sub-IFDs
    /// default: false
    pub subifd: bool,
    /// premultiply: `bool` -> Save with premultiplied alpha
    /// default: false
    pub premultiply: bool,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for TiffsaveOptions {
    fn default() -> Self {
        TiffsaveOptions {
            compression: ForeignTiffCompression::None,
            q: i32::from(75),
            predictor: ForeignTiffPredictor::Horizontal,
            tile: false,
            tile_width: i32::from(128),
            tile_height: i32::from(128),
            pyramid: false,
            miniswhite: false,
            bitdepth: i32::from(0),
            resunit: ForeignTiffResunit::Cm,
            xres: f64::from(1),
            yres: f64::from(1),
            bigtiff: false,
            properties: false,
            region_shrink: RegionShrink::Mean,
            level: i32::from(10),
            lossless: false,
            depth: ForeignDzDepth::Onetile,
            subifd: false,
            premultiply: false,
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveTiffFile (tiffsave), save image to tiff file (.tif, .tiff), priority=0, any
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to
/// tiffsave_options: `&TiffsaveOptions` -> optional arguments

pub fn tiffsave_with_opts(
    inp: &VipsImage,
    filename: &str,
    tiffsave_options: &TiffsaveOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let compression_in: i32 = tiffsave_options.compression as i32;
        let compression_in_name = utils::new_c_string("compression")?;

        let q_in: i32 = tiffsave_options.q;
        let q_in_name = utils::new_c_string("Q")?;

        let predictor_in: i32 = tiffsave_options.predictor as i32;
        let predictor_in_name = utils::new_c_string("predictor")?;

        let tile_in: i32 = if tiffsave_options.tile { 1 } else { 0 };
        let tile_in_name = utils::new_c_string("tile")?;

        let tile_width_in: i32 = tiffsave_options.tile_width;
        let tile_width_in_name = utils::new_c_string("tile-width")?;

        let tile_height_in: i32 = tiffsave_options.tile_height;
        let tile_height_in_name = utils::new_c_string("tile-height")?;

        let pyramid_in: i32 = if tiffsave_options.pyramid { 1 } else { 0 };
        let pyramid_in_name = utils::new_c_string("pyramid")?;

        let miniswhite_in: i32 = if tiffsave_options.miniswhite { 1 } else { 0 };
        let miniswhite_in_name = utils::new_c_string("miniswhite")?;

        let bitdepth_in: i32 = tiffsave_options.bitdepth;
        let bitdepth_in_name = utils::new_c_string("bitdepth")?;

        let resunit_in: i32 = tiffsave_options.resunit as i32;
        let resunit_in_name = utils::new_c_string("resunit")?;

        let xres_in: f64 = tiffsave_options.xres;
        let xres_in_name = utils::new_c_string("xres")?;

        let yres_in: f64 = tiffsave_options.yres;
        let yres_in_name = utils::new_c_string("yres")?;

        let bigtiff_in: i32 = if tiffsave_options.bigtiff { 1 } else { 0 };
        let bigtiff_in_name = utils::new_c_string("bigtiff")?;

        let properties_in: i32 = if tiffsave_options.properties { 1 } else { 0 };
        let properties_in_name = utils::new_c_string("properties")?;

        let region_shrink_in: i32 = tiffsave_options.region_shrink as i32;
        let region_shrink_in_name = utils::new_c_string("region-shrink")?;

        let level_in: i32 = tiffsave_options.level;
        let level_in_name = utils::new_c_string("level")?;

        let lossless_in: i32 = if tiffsave_options.lossless { 1 } else { 0 };
        let lossless_in_name = utils::new_c_string("lossless")?;

        let depth_in: i32 = tiffsave_options.depth as i32;
        let depth_in_name = utils::new_c_string("depth")?;

        let subifd_in: i32 = if tiffsave_options.subifd { 1 } else { 0 };
        let subifd_in_name = utils::new_c_string("subifd")?;

        let premultiply_in: i32 = if tiffsave_options.premultiply { 1 } else { 0 };
        let premultiply_in_name = utils::new_c_string("premultiply")?;

        let keep_in: i32 = tiffsave_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&tiffsave_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = tiffsave_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&tiffsave_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_tiffsave(
            inp_in,
            filename_in.as_ptr(),
            compression_in_name.as_ptr(),
            compression_in,
            q_in_name.as_ptr(),
            q_in,
            predictor_in_name.as_ptr(),
            predictor_in,
            tile_in_name.as_ptr(),
            tile_in,
            tile_width_in_name.as_ptr(),
            tile_width_in,
            tile_height_in_name.as_ptr(),
            tile_height_in,
            pyramid_in_name.as_ptr(),
            pyramid_in,
            miniswhite_in_name.as_ptr(),
            miniswhite_in,
            bitdepth_in_name.as_ptr(),
            bitdepth_in,
            resunit_in_name.as_ptr(),
            resunit_in,
            xres_in_name.as_ptr(),
            xres_in,
            yres_in_name.as_ptr(),
            yres_in,
            bigtiff_in_name.as_ptr(),
            bigtiff_in,
            properties_in_name.as_ptr(),
            properties_in,
            region_shrink_in_name.as_ptr(),
            region_shrink_in,
            level_in_name.as_ptr(),
            level_in,
            lossless_in_name.as_ptr(),
            lossless_in,
            depth_in_name.as_ptr(),
            depth_in,
            subifd_in_name.as_ptr(),
            subifd_in,
            premultiply_in_name.as_ptr(),
            premultiply_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::TiffsaveError,
        )
    }
}

/// VipsForeignSaveTiffBuffer (tiffsave_buffer), save image to tiff buffer (.tif, .tiff), priority=0, any
/// inp: `&VipsImage` -> Image to save
/// returns `Vec<u8>` - Buffer to save to
pub fn tiffsave_buffer(inp: &VipsImage) -> Result<Vec<u8>> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut buffer_buf_size: u64 = 0;
        let mut buffer_out: *mut c_void = null_mut();

        let vips_op_response = bindings::vips_tiffsave_buffer(
            inp_in,
            &mut buffer_out,
            &mut buffer_buf_size,
            NULL,
        );
        utils::result(
            vips_op_response,
            utils::new_byte_array(
                buffer_out,
                buffer_buf_size,
            ),
            Error::TiffsaveBufferError,
        )
    }
}

/// Options for tiffsave_buffer operation
#[derive(Clone, Debug)]
pub struct TiffsaveBufferOptions {
    /// compression: `ForeignTiffCompression` -> Compression for this file
    ///  `None` -> VIPS_FOREIGN_TIFF_COMPRESSION_NONE = 0 [DEFAULT]
    ///  `Jpeg` -> VIPS_FOREIGN_TIFF_COMPRESSION_JPEG = 1
    ///  `Deflate` -> VIPS_FOREIGN_TIFF_COMPRESSION_DEFLATE = 2
    ///  `Packbit` -> VIPS_FOREIGN_TIFF_COMPRESSION_PACKBITS = 3
    ///  `Ccittfax4` -> VIPS_FOREIGN_TIFF_COMPRESSION_CCITTFAX4 = 4
    ///  `Lzw` -> VIPS_FOREIGN_TIFF_COMPRESSION_LZW = 5
    ///  `Webp` -> VIPS_FOREIGN_TIFF_COMPRESSION_WEBP = 6
    ///  `Zstd` -> VIPS_FOREIGN_TIFF_COMPRESSION_ZSTD = 7
    ///  `Jp2K` -> VIPS_FOREIGN_TIFF_COMPRESSION_JP2K = 8
    ///  `Last` -> VIPS_FOREIGN_TIFF_COMPRESSION_LAST = 9
    pub compression: ForeignTiffCompression,
    /// q: `i32` -> Q factor
    /// min: 1, max: 100, default: 75
    pub q: i32,
    /// predictor: `ForeignTiffPredictor` -> Compression prediction
    ///  `None` -> VIPS_FOREIGN_TIFF_PREDICTOR_NONE = 1
    ///  `Horizontal` -> VIPS_FOREIGN_TIFF_PREDICTOR_HORIZONTAL = 2 [DEFAULT]
    ///  `Float` -> VIPS_FOREIGN_TIFF_PREDICTOR_FLOAT = 3
    ///  `Last` -> VIPS_FOREIGN_TIFF_PREDICTOR_LAST = 4
    pub predictor: ForeignTiffPredictor,
    /// tile: `bool` -> Write a tiled tiff
    /// default: false
    pub tile: bool,
    /// tile_width: `i32` -> Tile width in pixels
    /// min: 1, max: 32768, default: 128
    pub tile_width: i32,
    /// tile_height: `i32` -> Tile height in pixels
    /// min: 1, max: 32768, default: 128
    pub tile_height: i32,
    /// pyramid: `bool` -> Write a pyramidal tiff
    /// default: false
    pub pyramid: bool,
    /// miniswhite: `bool` -> Use 0 for white in 1-bit images
    /// default: false
    pub miniswhite: bool,
    /// bitdepth: `i32` -> Write as a 1, 2, 4 or 8 bit image
    /// min: 0, max: 8, default: 0
    pub bitdepth: i32,
    /// resunit: `ForeignTiffResunit` -> Resolution unit
    ///  `Cm` -> VIPS_FOREIGN_TIFF_RESUNIT_CM = 0 [DEFAULT]
    ///  `Inch` -> VIPS_FOREIGN_TIFF_RESUNIT_INCH = 1
    ///  `Last` -> VIPS_FOREIGN_TIFF_RESUNIT_LAST = 2
    pub resunit: ForeignTiffResunit,
    /// xres: `f64` -> Horizontal resolution in pixels/mm
    /// min: 0.001, max: 1000000, default: 1
    pub xres: f64,
    /// yres: `f64` -> Vertical resolution in pixels/mm
    /// min: 0.001, max: 1000000, default: 1
    pub yres: f64,
    /// bigtiff: `bool` -> Write a bigtiff image
    /// default: false
    pub bigtiff: bool,
    /// properties: `bool` -> Write a properties document to IMAGEDESCRIPTION
    /// default: false
    pub properties: bool,
    /// region_shrink: `RegionShrink` -> Method to shrink regions
    ///  `Mean` -> VIPS_REGION_SHRINK_MEAN = 0 [DEFAULT]
    ///  `Median` -> VIPS_REGION_SHRINK_MEDIAN = 1
    ///  `Mode` -> VIPS_REGION_SHRINK_MODE = 2
    ///  `Max` -> VIPS_REGION_SHRINK_MAX = 3
    ///  `Min` -> VIPS_REGION_SHRINK_MIN = 4
    ///  `Nearest` -> VIPS_REGION_SHRINK_NEAREST = 5
    ///  `Last` -> VIPS_REGION_SHRINK_LAST = 6
    pub region_shrink: RegionShrink,
    /// level: `i32` -> ZSTD compression level
    /// min: 1, max: 22, default: 10
    pub level: i32,
    /// lossless: `bool` -> Enable WEBP lossless mode
    /// default: false
    pub lossless: bool,
    /// depth: `ForeignDzDepth` -> Pyramid depth
    ///  `Onepixel` -> VIPS_FOREIGN_DZ_DEPTH_ONEPIXEL = 0
    ///  `Onetile` -> VIPS_FOREIGN_DZ_DEPTH_ONETILE = 1 [DEFAULT]
    ///  `One` -> VIPS_FOREIGN_DZ_DEPTH_ONE = 2
    ///  `Last` -> VIPS_FOREIGN_DZ_DEPTH_LAST = 3
    pub depth: ForeignDzDepth,
    /// subifd: `bool` -> Save pyr layers as sub-IFDs
    /// default: false
    pub subifd: bool,
    /// premultiply: `bool` -> Save with premultiplied alpha
    /// default: false
    pub premultiply: bool,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for TiffsaveBufferOptions {
    fn default() -> Self {
        TiffsaveBufferOptions {
            compression: ForeignTiffCompression::None,
            q: i32::from(75),
            predictor: ForeignTiffPredictor::Horizontal,
            tile: false,
            tile_width: i32::from(128),
            tile_height: i32::from(128),
            pyramid: false,
            miniswhite: false,
            bitdepth: i32::from(0),
            resunit: ForeignTiffResunit::Cm,
            xres: f64::from(1),
            yres: f64::from(1),
            bigtiff: false,
            properties: false,
            region_shrink: RegionShrink::Mean,
            level: i32::from(10),
            lossless: false,
            depth: ForeignDzDepth::Onetile,
            subifd: false,
            premultiply: false,
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveTiffBuffer (tiffsave_buffer), save image to tiff buffer (.tif, .tiff), priority=0, any
/// inp: `&VipsImage` -> Image to save
/// tiffsave_buffer_options: `&TiffsaveBufferOptions` -> optional arguments
/// returns `Vec<u8>` - Buffer to save to
pub fn tiffsave_buffer_with_opts(
    inp: &VipsImage,
    tiffsave_buffer_options: &TiffsaveBufferOptions,
) -> Result<Vec<u8>> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut buffer_buf_size: u64 = 0;
        let mut buffer_out: *mut c_void = null_mut();

        let compression_in: i32 = tiffsave_buffer_options.compression as i32;
        let compression_in_name = utils::new_c_string("compression")?;

        let q_in: i32 = tiffsave_buffer_options.q;
        let q_in_name = utils::new_c_string("Q")?;

        let predictor_in: i32 = tiffsave_buffer_options.predictor as i32;
        let predictor_in_name = utils::new_c_string("predictor")?;

        let tile_in: i32 = if tiffsave_buffer_options.tile { 1 } else { 0 };
        let tile_in_name = utils::new_c_string("tile")?;

        let tile_width_in: i32 = tiffsave_buffer_options.tile_width;
        let tile_width_in_name = utils::new_c_string("tile-width")?;

        let tile_height_in: i32 = tiffsave_buffer_options.tile_height;
        let tile_height_in_name = utils::new_c_string("tile-height")?;

        let pyramid_in: i32 = if tiffsave_buffer_options.pyramid { 1 } else { 0 };
        let pyramid_in_name = utils::new_c_string("pyramid")?;

        let miniswhite_in: i32 = if tiffsave_buffer_options.miniswhite { 1 } else { 0 };
        let miniswhite_in_name = utils::new_c_string("miniswhite")?;

        let bitdepth_in: i32 = tiffsave_buffer_options.bitdepth;
        let bitdepth_in_name = utils::new_c_string("bitdepth")?;

        let resunit_in: i32 = tiffsave_buffer_options.resunit as i32;
        let resunit_in_name = utils::new_c_string("resunit")?;

        let xres_in: f64 = tiffsave_buffer_options.xres;
        let xres_in_name = utils::new_c_string("xres")?;

        let yres_in: f64 = tiffsave_buffer_options.yres;
        let yres_in_name = utils::new_c_string("yres")?;

        let bigtiff_in: i32 = if tiffsave_buffer_options.bigtiff { 1 } else { 0 };
        let bigtiff_in_name = utils::new_c_string("bigtiff")?;

        let properties_in: i32 = if tiffsave_buffer_options.properties { 1 } else { 0 };
        let properties_in_name = utils::new_c_string("properties")?;

        let region_shrink_in: i32 = tiffsave_buffer_options.region_shrink as i32;
        let region_shrink_in_name = utils::new_c_string("region-shrink")?;

        let level_in: i32 = tiffsave_buffer_options.level;
        let level_in_name = utils::new_c_string("level")?;

        let lossless_in: i32 = if tiffsave_buffer_options.lossless { 1 } else { 0 };
        let lossless_in_name = utils::new_c_string("lossless")?;

        let depth_in: i32 = tiffsave_buffer_options.depth as i32;
        let depth_in_name = utils::new_c_string("depth")?;

        let subifd_in: i32 = if tiffsave_buffer_options.subifd { 1 } else { 0 };
        let subifd_in_name = utils::new_c_string("subifd")?;

        let premultiply_in: i32 = if tiffsave_buffer_options.premultiply { 1 } else { 0 };
        let premultiply_in_name = utils::new_c_string("premultiply")?;

        let keep_in: i32 = tiffsave_buffer_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&tiffsave_buffer_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = tiffsave_buffer_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&tiffsave_buffer_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_tiffsave_buffer(
            inp_in,
            &mut buffer_out,
            &mut buffer_buf_size,
            compression_in_name.as_ptr(),
            compression_in,
            q_in_name.as_ptr(),
            q_in,
            predictor_in_name.as_ptr(),
            predictor_in,
            tile_in_name.as_ptr(),
            tile_in,
            tile_width_in_name.as_ptr(),
            tile_width_in,
            tile_height_in_name.as_ptr(),
            tile_height_in,
            pyramid_in_name.as_ptr(),
            pyramid_in,
            miniswhite_in_name.as_ptr(),
            miniswhite_in,
            bitdepth_in_name.as_ptr(),
            bitdepth_in,
            resunit_in_name.as_ptr(),
            resunit_in,
            xres_in_name.as_ptr(),
            xres_in,
            yres_in_name.as_ptr(),
            yres_in,
            bigtiff_in_name.as_ptr(),
            bigtiff_in,
            properties_in_name.as_ptr(),
            properties_in,
            region_shrink_in_name.as_ptr(),
            region_shrink_in,
            level_in_name.as_ptr(),
            level_in,
            lossless_in_name.as_ptr(),
            lossless_in,
            depth_in_name.as_ptr(),
            depth_in,
            subifd_in_name.as_ptr(),
            subifd_in,
            premultiply_in_name.as_ptr(),
            premultiply_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            utils::new_byte_array(
                buffer_out,
                buffer_buf_size,
            ),
            Error::TiffsaveBufferError,
        )
    }
}

/// VipsForeignSaveTiffTarget (tiffsave_target), save image to tiff target (.tif, .tiff), priority=0, any
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to

pub fn tiffsave_target(inp: &VipsImage, target: &VipsTarget) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let vips_op_response = bindings::vips_tiffsave_target(
            inp_in,
            target_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::TiffsaveTargetError,
        )
    }
}

/// Options for tiffsave_target operation
#[derive(Clone, Debug)]
pub struct TiffsaveTargetOptions {
    /// compression: `ForeignTiffCompression` -> Compression for this file
    ///  `None` -> VIPS_FOREIGN_TIFF_COMPRESSION_NONE = 0 [DEFAULT]
    ///  `Jpeg` -> VIPS_FOREIGN_TIFF_COMPRESSION_JPEG = 1
    ///  `Deflate` -> VIPS_FOREIGN_TIFF_COMPRESSION_DEFLATE = 2
    ///  `Packbit` -> VIPS_FOREIGN_TIFF_COMPRESSION_PACKBITS = 3
    ///  `Ccittfax4` -> VIPS_FOREIGN_TIFF_COMPRESSION_CCITTFAX4 = 4
    ///  `Lzw` -> VIPS_FOREIGN_TIFF_COMPRESSION_LZW = 5
    ///  `Webp` -> VIPS_FOREIGN_TIFF_COMPRESSION_WEBP = 6
    ///  `Zstd` -> VIPS_FOREIGN_TIFF_COMPRESSION_ZSTD = 7
    ///  `Jp2K` -> VIPS_FOREIGN_TIFF_COMPRESSION_JP2K = 8
    ///  `Last` -> VIPS_FOREIGN_TIFF_COMPRESSION_LAST = 9
    pub compression: ForeignTiffCompression,
    /// q: `i32` -> Q factor
    /// min: 1, max: 100, default: 75
    pub q: i32,
    /// predictor: `ForeignTiffPredictor` -> Compression prediction
    ///  `None` -> VIPS_FOREIGN_TIFF_PREDICTOR_NONE = 1
    ///  `Horizontal` -> VIPS_FOREIGN_TIFF_PREDICTOR_HORIZONTAL = 2 [DEFAULT]
    ///  `Float` -> VIPS_FOREIGN_TIFF_PREDICTOR_FLOAT = 3
    ///  `Last` -> VIPS_FOREIGN_TIFF_PREDICTOR_LAST = 4
    pub predictor: ForeignTiffPredictor,
    /// tile: `bool` -> Write a tiled tiff
    /// default: false
    pub tile: bool,
    /// tile_width: `i32` -> Tile width in pixels
    /// min: 1, max: 32768, default: 128
    pub tile_width: i32,
    /// tile_height: `i32` -> Tile height in pixels
    /// min: 1, max: 32768, default: 128
    pub tile_height: i32,
    /// pyramid: `bool` -> Write a pyramidal tiff
    /// default: false
    pub pyramid: bool,
    /// miniswhite: `bool` -> Use 0 for white in 1-bit images
    /// default: false
    pub miniswhite: bool,
    /// bitdepth: `i32` -> Write as a 1, 2, 4 or 8 bit image
    /// min: 0, max: 8, default: 0
    pub bitdepth: i32,
    /// resunit: `ForeignTiffResunit` -> Resolution unit
    ///  `Cm` -> VIPS_FOREIGN_TIFF_RESUNIT_CM = 0 [DEFAULT]
    ///  `Inch` -> VIPS_FOREIGN_TIFF_RESUNIT_INCH = 1
    ///  `Last` -> VIPS_FOREIGN_TIFF_RESUNIT_LAST = 2
    pub resunit: ForeignTiffResunit,
    /// xres: `f64` -> Horizontal resolution in pixels/mm
    /// min: 0.001, max: 1000000, default: 1
    pub xres: f64,
    /// yres: `f64` -> Vertical resolution in pixels/mm
    /// min: 0.001, max: 1000000, default: 1
    pub yres: f64,
    /// bigtiff: `bool` -> Write a bigtiff image
    /// default: false
    pub bigtiff: bool,
    /// properties: `bool` -> Write a properties document to IMAGEDESCRIPTION
    /// default: false
    pub properties: bool,
    /// region_shrink: `RegionShrink` -> Method to shrink regions
    ///  `Mean` -> VIPS_REGION_SHRINK_MEAN = 0 [DEFAULT]
    ///  `Median` -> VIPS_REGION_SHRINK_MEDIAN = 1
    ///  `Mode` -> VIPS_REGION_SHRINK_MODE = 2
    ///  `Max` -> VIPS_REGION_SHRINK_MAX = 3
    ///  `Min` -> VIPS_REGION_SHRINK_MIN = 4
    ///  `Nearest` -> VIPS_REGION_SHRINK_NEAREST = 5
    ///  `Last` -> VIPS_REGION_SHRINK_LAST = 6
    pub region_shrink: RegionShrink,
    /// level: `i32` -> ZSTD compression level
    /// min: 1, max: 22, default: 10
    pub level: i32,
    /// lossless: `bool` -> Enable WEBP lossless mode
    /// default: false
    pub lossless: bool,
    /// depth: `ForeignDzDepth` -> Pyramid depth
    ///  `Onepixel` -> VIPS_FOREIGN_DZ_DEPTH_ONEPIXEL = 0
    ///  `Onetile` -> VIPS_FOREIGN_DZ_DEPTH_ONETILE = 1 [DEFAULT]
    ///  `One` -> VIPS_FOREIGN_DZ_DEPTH_ONE = 2
    ///  `Last` -> VIPS_FOREIGN_DZ_DEPTH_LAST = 3
    pub depth: ForeignDzDepth,
    /// subifd: `bool` -> Save pyr layers as sub-IFDs
    /// default: false
    pub subifd: bool,
    /// premultiply: `bool` -> Save with premultiplied alpha
    /// default: false
    pub premultiply: bool,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for TiffsaveTargetOptions {
    fn default() -> Self {
        TiffsaveTargetOptions {
            compression: ForeignTiffCompression::None,
            q: i32::from(75),
            predictor: ForeignTiffPredictor::Horizontal,
            tile: false,
            tile_width: i32::from(128),
            tile_height: i32::from(128),
            pyramid: false,
            miniswhite: false,
            bitdepth: i32::from(0),
            resunit: ForeignTiffResunit::Cm,
            xres: f64::from(1),
            yres: f64::from(1),
            bigtiff: false,
            properties: false,
            region_shrink: RegionShrink::Mean,
            level: i32::from(10),
            lossless: false,
            depth: ForeignDzDepth::Onetile,
            subifd: false,
            premultiply: false,
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveTiffTarget (tiffsave_target), save image to tiff target (.tif, .tiff), priority=0, any
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to
/// tiffsave_target_options: `&TiffsaveTargetOptions` -> optional arguments

pub fn tiffsave_target_with_opts(
    inp: &VipsImage,
    target: &VipsTarget,
    tiffsave_target_options: &TiffsaveTargetOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let compression_in: i32 = tiffsave_target_options.compression as i32;
        let compression_in_name = utils::new_c_string("compression")?;

        let q_in: i32 = tiffsave_target_options.q;
        let q_in_name = utils::new_c_string("Q")?;

        let predictor_in: i32 = tiffsave_target_options.predictor as i32;
        let predictor_in_name = utils::new_c_string("predictor")?;

        let tile_in: i32 = if tiffsave_target_options.tile { 1 } else { 0 };
        let tile_in_name = utils::new_c_string("tile")?;

        let tile_width_in: i32 = tiffsave_target_options.tile_width;
        let tile_width_in_name = utils::new_c_string("tile-width")?;

        let tile_height_in: i32 = tiffsave_target_options.tile_height;
        let tile_height_in_name = utils::new_c_string("tile-height")?;

        let pyramid_in: i32 = if tiffsave_target_options.pyramid { 1 } else { 0 };
        let pyramid_in_name = utils::new_c_string("pyramid")?;

        let miniswhite_in: i32 = if tiffsave_target_options.miniswhite { 1 } else { 0 };
        let miniswhite_in_name = utils::new_c_string("miniswhite")?;

        let bitdepth_in: i32 = tiffsave_target_options.bitdepth;
        let bitdepth_in_name = utils::new_c_string("bitdepth")?;

        let resunit_in: i32 = tiffsave_target_options.resunit as i32;
        let resunit_in_name = utils::new_c_string("resunit")?;

        let xres_in: f64 = tiffsave_target_options.xres;
        let xres_in_name = utils::new_c_string("xres")?;

        let yres_in: f64 = tiffsave_target_options.yres;
        let yres_in_name = utils::new_c_string("yres")?;

        let bigtiff_in: i32 = if tiffsave_target_options.bigtiff { 1 } else { 0 };
        let bigtiff_in_name = utils::new_c_string("bigtiff")?;

        let properties_in: i32 = if tiffsave_target_options.properties { 1 } else { 0 };
        let properties_in_name = utils::new_c_string("properties")?;

        let region_shrink_in: i32 = tiffsave_target_options.region_shrink as i32;
        let region_shrink_in_name = utils::new_c_string("region-shrink")?;

        let level_in: i32 = tiffsave_target_options.level;
        let level_in_name = utils::new_c_string("level")?;

        let lossless_in: i32 = if tiffsave_target_options.lossless { 1 } else { 0 };
        let lossless_in_name = utils::new_c_string("lossless")?;

        let depth_in: i32 = tiffsave_target_options.depth as i32;
        let depth_in_name = utils::new_c_string("depth")?;

        let subifd_in: i32 = if tiffsave_target_options.subifd { 1 } else { 0 };
        let subifd_in_name = utils::new_c_string("subifd")?;

        let premultiply_in: i32 = if tiffsave_target_options.premultiply { 1 } else { 0 };
        let premultiply_in_name = utils::new_c_string("premultiply")?;

        let keep_in: i32 = tiffsave_target_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&tiffsave_target_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = tiffsave_target_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&tiffsave_target_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_tiffsave_target(
            inp_in,
            target_in,
            compression_in_name.as_ptr(),
            compression_in,
            q_in_name.as_ptr(),
            q_in,
            predictor_in_name.as_ptr(),
            predictor_in,
            tile_in_name.as_ptr(),
            tile_in,
            tile_width_in_name.as_ptr(),
            tile_width_in,
            tile_height_in_name.as_ptr(),
            tile_height_in,
            pyramid_in_name.as_ptr(),
            pyramid_in,
            miniswhite_in_name.as_ptr(),
            miniswhite_in,
            bitdepth_in_name.as_ptr(),
            bitdepth_in,
            resunit_in_name.as_ptr(),
            resunit_in,
            xres_in_name.as_ptr(),
            xres_in,
            yres_in_name.as_ptr(),
            yres_in,
            bigtiff_in_name.as_ptr(),
            bigtiff_in,
            properties_in_name.as_ptr(),
            properties_in,
            region_shrink_in_name.as_ptr(),
            region_shrink_in,
            level_in_name.as_ptr(),
            level_in,
            lossless_in_name.as_ptr(),
            lossless_in,
            depth_in_name.as_ptr(),
            depth_in,
            subifd_in_name.as_ptr(),
            subifd_in,
            premultiply_in_name.as_ptr(),
            premultiply_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::TiffsaveTargetError,
        )
    }
}

/// VipsForeignSaveHeifFile (heifsave), save image in HEIF format (.heic, .heif, .avif), priority=0, rgba-only
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to

pub fn heifsave(inp: &VipsImage, filename: &str) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let vips_op_response = bindings::vips_heifsave(
            inp_in,
            filename_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::HeifsaveError,
        )
    }
}

/// Options for heifsave operation
#[derive(Clone, Debug)]
pub struct HeifsaveOptions {
    /// q: `i32` -> Q factor
    /// min: 1, max: 100, default: 50
    pub q: i32,
    /// bitdepth: `i32` -> Number of bits per pixel
    /// min: 1, max: 16, default: 12
    pub bitdepth: i32,
    /// lossless: `bool` -> Enable lossless compression
    /// default: false
    pub lossless: bool,
    /// compression: `ForeignHeifCompression` -> Compression format
    ///  `Hevc` -> VIPS_FOREIGN_HEIF_COMPRESSION_HEVC = 1 [DEFAULT]
    ///  `Avc` -> VIPS_FOREIGN_HEIF_COMPRESSION_AVC = 2
    ///  `Jpeg` -> VIPS_FOREIGN_HEIF_COMPRESSION_JPEG = 3
    ///  `Av1` -> VIPS_FOREIGN_HEIF_COMPRESSION_AV1 = 4
    ///  `Last` -> VIPS_FOREIGN_HEIF_COMPRESSION_LAST = 5
    pub compression: ForeignHeifCompression,
    /// effort: `i32` -> CPU effort
    /// min: 0, max: 9, default: 4
    pub effort: i32,
    /// subsample_mode: `ForeignSubsample` -> Select chroma subsample operation mode
    ///  `Auto` -> VIPS_FOREIGN_SUBSAMPLE_AUTO = 0 [DEFAULT]
    ///  `On` -> VIPS_FOREIGN_SUBSAMPLE_ON = 1
    ///  `Off` -> VIPS_FOREIGN_SUBSAMPLE_OFF = 2
    ///  `Last` -> VIPS_FOREIGN_SUBSAMPLE_LAST = 3
    pub subsample_mode: ForeignSubsample,
    /// encoder: `ForeignHeifEncoder` -> Select encoder to use
    ///  `Auto` -> VIPS_FOREIGN_HEIF_ENCODER_AUTO = 0 [DEFAULT]
    ///  `Aom` -> VIPS_FOREIGN_HEIF_ENCODER_AOM = 1
    ///  `Rav1E` -> VIPS_FOREIGN_HEIF_ENCODER_RAV1E = 2
    ///  `Svt` -> VIPS_FOREIGN_HEIF_ENCODER_SVT = 3
    ///  `X265` -> VIPS_FOREIGN_HEIF_ENCODER_X265 = 4
    ///  `Last` -> VIPS_FOREIGN_HEIF_ENCODER_LAST = 5
    pub encoder: ForeignHeifEncoder,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for HeifsaveOptions {
    fn default() -> Self {
        HeifsaveOptions {
            q: i32::from(50),
            bitdepth: i32::from(12),
            lossless: false,
            compression: ForeignHeifCompression::Hevc,
            effort: i32::from(4),
            subsample_mode: ForeignSubsample::Auto,
            encoder: ForeignHeifEncoder::Auto,
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveHeifFile (heifsave), save image in HEIF format (.heic, .heif, .avif), priority=0, rgba-only
/// inp: `&VipsImage` -> Image to save
/// filename: `&str` -> Filename to save to
/// heifsave_options: `&HeifsaveOptions` -> optional arguments

pub fn heifsave_with_opts(
    inp: &VipsImage,
    filename: &str,
    heifsave_options: &HeifsaveOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let filename_in: CString = utils::new_c_string(filename)?;

        let q_in: i32 = heifsave_options.q;
        let q_in_name = utils::new_c_string("Q")?;

        let bitdepth_in: i32 = heifsave_options.bitdepth;
        let bitdepth_in_name = utils::new_c_string("bitdepth")?;

        let lossless_in: i32 = if heifsave_options.lossless { 1 } else { 0 };
        let lossless_in_name = utils::new_c_string("lossless")?;

        let compression_in: i32 = heifsave_options.compression as i32;
        let compression_in_name = utils::new_c_string("compression")?;

        let effort_in: i32 = heifsave_options.effort;
        let effort_in_name = utils::new_c_string("effort")?;

        let subsample_mode_in: i32 = heifsave_options.subsample_mode as i32;
        let subsample_mode_in_name = utils::new_c_string("subsample-mode")?;

        let encoder_in: i32 = heifsave_options.encoder as i32;
        let encoder_in_name = utils::new_c_string("encoder")?;

        let keep_in: i32 = heifsave_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&heifsave_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = heifsave_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&heifsave_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_heifsave(
            inp_in,
            filename_in.as_ptr(),
            q_in_name.as_ptr(),
            q_in,
            bitdepth_in_name.as_ptr(),
            bitdepth_in,
            lossless_in_name.as_ptr(),
            lossless_in,
            compression_in_name.as_ptr(),
            compression_in,
            effort_in_name.as_ptr(),
            effort_in,
            subsample_mode_in_name.as_ptr(),
            subsample_mode_in,
            encoder_in_name.as_ptr(),
            encoder_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::HeifsaveError,
        )
    }
}

/// VipsForeignSaveHeifBuffer (heifsave_buffer), save image in HEIF format (.heic, .heif), priority=0, rgba-only
/// inp: `&VipsImage` -> Image to save
/// returns `Vec<u8>` - Buffer to save to
pub fn heifsave_buffer(inp: &VipsImage) -> Result<Vec<u8>> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut buffer_buf_size: u64 = 0;
        let mut buffer_out: *mut c_void = null_mut();

        let vips_op_response = bindings::vips_heifsave_buffer(
            inp_in,
            &mut buffer_out,
            &mut buffer_buf_size,
            NULL,
        );
        utils::result(
            vips_op_response,
            utils::new_byte_array(
                buffer_out,
                buffer_buf_size,
            ),
            Error::HeifsaveBufferError,
        )
    }
}

/// Options for heifsave_buffer operation
#[derive(Clone, Debug)]
pub struct HeifsaveBufferOptions {
    /// q: `i32` -> Q factor
    /// min: 1, max: 100, default: 50
    pub q: i32,
    /// bitdepth: `i32` -> Number of bits per pixel
    /// min: 1, max: 16, default: 12
    pub bitdepth: i32,
    /// lossless: `bool` -> Enable lossless compression
    /// default: false
    pub lossless: bool,
    /// compression: `ForeignHeifCompression` -> Compression format
    ///  `Hevc` -> VIPS_FOREIGN_HEIF_COMPRESSION_HEVC = 1 [DEFAULT]
    ///  `Avc` -> VIPS_FOREIGN_HEIF_COMPRESSION_AVC = 2
    ///  `Jpeg` -> VIPS_FOREIGN_HEIF_COMPRESSION_JPEG = 3
    ///  `Av1` -> VIPS_FOREIGN_HEIF_COMPRESSION_AV1 = 4
    ///  `Last` -> VIPS_FOREIGN_HEIF_COMPRESSION_LAST = 5
    pub compression: ForeignHeifCompression,
    /// effort: `i32` -> CPU effort
    /// min: 0, max: 9, default: 4
    pub effort: i32,
    /// subsample_mode: `ForeignSubsample` -> Select chroma subsample operation mode
    ///  `Auto` -> VIPS_FOREIGN_SUBSAMPLE_AUTO = 0 [DEFAULT]
    ///  `On` -> VIPS_FOREIGN_SUBSAMPLE_ON = 1
    ///  `Off` -> VIPS_FOREIGN_SUBSAMPLE_OFF = 2
    ///  `Last` -> VIPS_FOREIGN_SUBSAMPLE_LAST = 3
    pub subsample_mode: ForeignSubsample,
    /// encoder: `ForeignHeifEncoder` -> Select encoder to use
    ///  `Auto` -> VIPS_FOREIGN_HEIF_ENCODER_AUTO = 0 [DEFAULT]
    ///  `Aom` -> VIPS_FOREIGN_HEIF_ENCODER_AOM = 1
    ///  `Rav1E` -> VIPS_FOREIGN_HEIF_ENCODER_RAV1E = 2
    ///  `Svt` -> VIPS_FOREIGN_HEIF_ENCODER_SVT = 3
    ///  `X265` -> VIPS_FOREIGN_HEIF_ENCODER_X265 = 4
    ///  `Last` -> VIPS_FOREIGN_HEIF_ENCODER_LAST = 5
    pub encoder: ForeignHeifEncoder,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for HeifsaveBufferOptions {
    fn default() -> Self {
        HeifsaveBufferOptions {
            q: i32::from(50),
            bitdepth: i32::from(12),
            lossless: false,
            compression: ForeignHeifCompression::Hevc,
            effort: i32::from(4),
            subsample_mode: ForeignSubsample::Auto,
            encoder: ForeignHeifEncoder::Auto,
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveHeifBuffer (heifsave_buffer), save image in HEIF format (.heic, .heif), priority=0, rgba-only
/// inp: `&VipsImage` -> Image to save
/// heifsave_buffer_options: `&HeifsaveBufferOptions` -> optional arguments
/// returns `Vec<u8>` - Buffer to save to
pub fn heifsave_buffer_with_opts(
    inp: &VipsImage,
    heifsave_buffer_options: &HeifsaveBufferOptions,
) -> Result<Vec<u8>> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut buffer_buf_size: u64 = 0;
        let mut buffer_out: *mut c_void = null_mut();

        let q_in: i32 = heifsave_buffer_options.q;
        let q_in_name = utils::new_c_string("Q")?;

        let bitdepth_in: i32 = heifsave_buffer_options.bitdepth;
        let bitdepth_in_name = utils::new_c_string("bitdepth")?;

        let lossless_in: i32 = if heifsave_buffer_options.lossless { 1 } else { 0 };
        let lossless_in_name = utils::new_c_string("lossless")?;

        let compression_in: i32 = heifsave_buffer_options.compression as i32;
        let compression_in_name = utils::new_c_string("compression")?;

        let effort_in: i32 = heifsave_buffer_options.effort;
        let effort_in_name = utils::new_c_string("effort")?;

        let subsample_mode_in: i32 = heifsave_buffer_options.subsample_mode as i32;
        let subsample_mode_in_name = utils::new_c_string("subsample-mode")?;

        let encoder_in: i32 = heifsave_buffer_options.encoder as i32;
        let encoder_in_name = utils::new_c_string("encoder")?;

        let keep_in: i32 = heifsave_buffer_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&heifsave_buffer_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = heifsave_buffer_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&heifsave_buffer_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_heifsave_buffer(
            inp_in,
            &mut buffer_out,
            &mut buffer_buf_size,
            q_in_name.as_ptr(),
            q_in,
            bitdepth_in_name.as_ptr(),
            bitdepth_in,
            lossless_in_name.as_ptr(),
            lossless_in,
            compression_in_name.as_ptr(),
            compression_in,
            effort_in_name.as_ptr(),
            effort_in,
            subsample_mode_in_name.as_ptr(),
            subsample_mode_in,
            encoder_in_name.as_ptr(),
            encoder_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            utils::new_byte_array(
                buffer_out,
                buffer_buf_size,
            ),
            Error::HeifsaveBufferError,
        )
    }
}

/// VipsForeignSaveHeifTarget (heifsave_target), save image in HEIF format (.heic, .heif), priority=0, rgba-only
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to

pub fn heifsave_target(inp: &VipsImage, target: &VipsTarget) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let vips_op_response = bindings::vips_heifsave_target(
            inp_in,
            target_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::HeifsaveTargetError,
        )
    }
}

/// Options for heifsave_target operation
#[derive(Clone, Debug)]
pub struct HeifsaveTargetOptions {
    /// q: `i32` -> Q factor
    /// min: 1, max: 100, default: 50
    pub q: i32,
    /// bitdepth: `i32` -> Number of bits per pixel
    /// min: 1, max: 16, default: 12
    pub bitdepth: i32,
    /// lossless: `bool` -> Enable lossless compression
    /// default: false
    pub lossless: bool,
    /// compression: `ForeignHeifCompression` -> Compression format
    ///  `Hevc` -> VIPS_FOREIGN_HEIF_COMPRESSION_HEVC = 1 [DEFAULT]
    ///  `Avc` -> VIPS_FOREIGN_HEIF_COMPRESSION_AVC = 2
    ///  `Jpeg` -> VIPS_FOREIGN_HEIF_COMPRESSION_JPEG = 3
    ///  `Av1` -> VIPS_FOREIGN_HEIF_COMPRESSION_AV1 = 4
    ///  `Last` -> VIPS_FOREIGN_HEIF_COMPRESSION_LAST = 5
    pub compression: ForeignHeifCompression,
    /// effort: `i32` -> CPU effort
    /// min: 0, max: 9, default: 4
    pub effort: i32,
    /// subsample_mode: `ForeignSubsample` -> Select chroma subsample operation mode
    ///  `Auto` -> VIPS_FOREIGN_SUBSAMPLE_AUTO = 0 [DEFAULT]
    ///  `On` -> VIPS_FOREIGN_SUBSAMPLE_ON = 1
    ///  `Off` -> VIPS_FOREIGN_SUBSAMPLE_OFF = 2
    ///  `Last` -> VIPS_FOREIGN_SUBSAMPLE_LAST = 3
    pub subsample_mode: ForeignSubsample,
    /// encoder: `ForeignHeifEncoder` -> Select encoder to use
    ///  `Auto` -> VIPS_FOREIGN_HEIF_ENCODER_AUTO = 0 [DEFAULT]
    ///  `Aom` -> VIPS_FOREIGN_HEIF_ENCODER_AOM = 1
    ///  `Rav1E` -> VIPS_FOREIGN_HEIF_ENCODER_RAV1E = 2
    ///  `Svt` -> VIPS_FOREIGN_HEIF_ENCODER_SVT = 3
    ///  `X265` -> VIPS_FOREIGN_HEIF_ENCODER_X265 = 4
    ///  `Last` -> VIPS_FOREIGN_HEIF_ENCODER_LAST = 5
    pub encoder: ForeignHeifEncoder,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for HeifsaveTargetOptions {
    fn default() -> Self {
        HeifsaveTargetOptions {
            q: i32::from(50),
            bitdepth: i32::from(12),
            lossless: false,
            compression: ForeignHeifCompression::Hevc,
            effort: i32::from(4),
            subsample_mode: ForeignSubsample::Auto,
            encoder: ForeignHeifEncoder::Auto,
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}

/// VipsForeignSaveHeifTarget (heifsave_target), save image in HEIF format (.heic, .heif), priority=0, rgba-only
/// inp: `&VipsImage` -> Image to save
/// target: `&VipsTarget` -> Target to save to
/// heifsave_target_options: `&HeifsaveTargetOptions` -> optional arguments

pub fn heifsave_target_with_opts(
    inp: &VipsImage,
    target: &VipsTarget,
    heifsave_target_options: &HeifsaveTargetOptions,
) -> Result<()> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let target_in: *mut bindings::VipsTarget = target.ctx;

        let q_in: i32 = heifsave_target_options.q;
        let q_in_name = utils::new_c_string("Q")?;

        let bitdepth_in: i32 = heifsave_target_options.bitdepth;
        let bitdepth_in_name = utils::new_c_string("bitdepth")?;

        let lossless_in: i32 = if heifsave_target_options.lossless { 1 } else { 0 };
        let lossless_in_name = utils::new_c_string("lossless")?;

        let compression_in: i32 = heifsave_target_options.compression as i32;
        let compression_in_name = utils::new_c_string("compression")?;

        let effort_in: i32 = heifsave_target_options.effort;
        let effort_in_name = utils::new_c_string("effort")?;

        let subsample_mode_in: i32 = heifsave_target_options.subsample_mode as i32;
        let subsample_mode_in_name = utils::new_c_string("subsample-mode")?;

        let encoder_in: i32 = heifsave_target_options.encoder as i32;
        let encoder_in_name = utils::new_c_string("encoder")?;

        let keep_in: i32 = heifsave_target_options.keep as i32;
        let keep_in_name = utils::new_c_string("keep")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&heifsave_target_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let page_height_in: i32 = heifsave_target_options.page_height;
        let page_height_in_name = utils::new_c_string("page-height")?;

        let profile_in: CString = utils::new_c_string(&heifsave_target_options.profile)?;
        let profile_in_name = utils::new_c_string("profile")?;

        let vips_op_response = bindings::vips_heifsave_target(
            inp_in,
            target_in,
            q_in_name.as_ptr(),
            q_in,
            bitdepth_in_name.as_ptr(),
            bitdepth_in,
            lossless_in_name.as_ptr(),
            lossless_in,
            compression_in_name.as_ptr(),
            compression_in,
            effort_in_name.as_ptr(),
            effort_in,
            subsample_mode_in_name.as_ptr(),
            subsample_mode_in,
            encoder_in_name.as_ptr(),
            encoder_in,
            keep_in_name.as_ptr(),
            keep_in,
            background_in_name.as_ptr(),
            background_in,
            page_height_in_name.as_ptr(),
            page_height_in,
            profile_in_name.as_ptr(),
            profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::HeifsaveTargetError,
        )
    }
}

/// VipsThumbnailFile (thumbnail), generate thumbnail from file
/// filename: `&str` -> Filename to read from
/// width: `i32` -> Size to this width
/// min: 1, max: 10000000, default: 1
/// returns `VipsImage` - Output image
pub fn thumbnail(filename: &str, width: i32) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let width_in: i32 = width;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_thumbnail(
            filename_in.as_ptr(),
            &mut out_out,
            width_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ThumbnailError,
        )
    }
}

/// Options for thumbnail operation
#[derive(Clone, Debug)]
pub struct ThumbnailOptions {
    /// height: `i32` -> Size to this height
    /// min: 1, max: 10000000, default: 1
    pub height: i32,
    /// size: `Size` -> Only upsize, only downsize, or both
    ///  `Both` -> VIPS_SIZE_BOTH = 0 [DEFAULT]
    ///  `Up` -> VIPS_SIZE_UP = 1
    ///  `Down` -> VIPS_SIZE_DOWN = 2
    ///  `Force` -> VIPS_SIZE_FORCE = 3
    ///  `Last` -> VIPS_SIZE_LAST = 4
    pub size: Size,
    /// no_rotate: `bool` -> Don't use orientation tags to rotate image upright
    /// default: false
    pub no_rotate: bool,
    /// crop: `Interesting` -> Reduce to fill target rectangle, then crop
    ///  `None` -> VIPS_INTERESTING_NONE = 0 [DEFAULT]
    ///  `Centre` -> VIPS_INTERESTING_CENTRE = 1
    ///  `Entropy` -> VIPS_INTERESTING_ENTROPY = 2
    ///  `Attention` -> VIPS_INTERESTING_ATTENTION = 3
    ///  `Low` -> VIPS_INTERESTING_LOW = 4
    ///  `High` -> VIPS_INTERESTING_HIGH = 5
    ///  `All` -> VIPS_INTERESTING_ALL = 6
    ///  `Last` -> VIPS_INTERESTING_LAST = 7
    pub crop: Interesting,
    /// linear: `bool` -> Reduce in linear light
    /// default: false
    pub linear: bool,
    /// import_profile: `String` -> Fallback import profile
    pub import_profile: String,
    /// export_profile: `String` -> Fallback export profile
    pub export_profile: String,
    /// intent: `Intent` -> Rendering intent
    ///  `Perceptual` -> VIPS_INTENT_PERCEPTUAL = 0
    ///  `Relative` -> VIPS_INTENT_RELATIVE = 1 [DEFAULT]
    ///  `Saturation` -> VIPS_INTENT_SATURATION = 2
    ///  `Absolute` -> VIPS_INTENT_ABSOLUTE = 3
    ///  `Last` -> VIPS_INTENT_LAST = 4
    pub intent: Intent,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
}

impl std::default::Default for ThumbnailOptions {
    fn default() -> Self {
        ThumbnailOptions {
            height: i32::from(1),
            size: Size::Both,
            no_rotate: false,
            crop: Interesting::None,
            linear: false,
            import_profile: String::new(),
            export_profile: String::new(),
            intent: Intent::Relative,
            fail_on: FailOn::None,
        }
    }
}

/// VipsThumbnailFile (thumbnail), generate thumbnail from file
/// filename: `&str` -> Filename to read from
/// width: `i32` -> Size to this width
/// min: 1, max: 10000000, default: 1
/// thumbnail_options: `&ThumbnailOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn thumbnail_with_opts(
    filename: &str,
    width: i32,
    thumbnail_options: &ThumbnailOptions,
) -> Result<VipsImage> {
    unsafe {
        let filename_in: CString = utils::new_c_string(filename)?;
        let width_in: i32 = width;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let height_in: i32 = thumbnail_options.height;
        let height_in_name = utils::new_c_string("height")?;

        let size_in: i32 = thumbnail_options.size as i32;
        let size_in_name = utils::new_c_string("size")?;

        let no_rotate_in: i32 = if thumbnail_options.no_rotate { 1 } else { 0 };
        let no_rotate_in_name = utils::new_c_string("no-rotate")?;

        let crop_in: i32 = thumbnail_options.crop as i32;
        let crop_in_name = utils::new_c_string("crop")?;

        let linear_in: i32 = if thumbnail_options.linear { 1 } else { 0 };
        let linear_in_name = utils::new_c_string("linear")?;

        let import_profile_in: CString = utils::new_c_string(&thumbnail_options.import_profile)?;
        let import_profile_in_name = utils::new_c_string("import-profile")?;

        let export_profile_in: CString = utils::new_c_string(&thumbnail_options.export_profile)?;
        let export_profile_in_name = utils::new_c_string("export-profile")?;

        let intent_in: i32 = thumbnail_options.intent as i32;
        let intent_in_name = utils::new_c_string("intent")?;

        let fail_on_in: i32 = thumbnail_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let vips_op_response = bindings::vips_thumbnail(
            filename_in.as_ptr(),
            &mut out_out,
            width_in,
            height_in_name.as_ptr(),
            height_in,
            size_in_name.as_ptr(),
            size_in,
            no_rotate_in_name.as_ptr(),
            no_rotate_in,
            crop_in_name.as_ptr(),
            crop_in,
            linear_in_name.as_ptr(),
            linear_in,
            import_profile_in_name.as_ptr(),
            import_profile_in.as_ptr(),
            export_profile_in_name.as_ptr(),
            export_profile_in.as_ptr(),
            intent_in_name.as_ptr(),
            intent_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ThumbnailError,
        )
    }
}

/// VipsThumbnailBuffer (thumbnail_buffer), generate thumbnail from buffer
/// buffer: `&[u8]` -> Buffer to load from
/// width: `i32` -> Size to this width
/// min: 1, max: 10000000, default: 1
/// returns `VipsImage` - Output image
pub fn thumbnail_buffer(buffer: &[u8], width: i32) -> Result<VipsImage> {
    unsafe {
        let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
        let width_in: i32 = width;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_thumbnail_buffer(
            buffer_in,
            buffer.len() as u64,
            &mut out_out,
            width_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ThumbnailBufferError,
        )
    }
}

/// Options for thumbnail_buffer operation
#[derive(Clone, Debug)]
pub struct ThumbnailBufferOptions {
    /// option_string: `String` -> Options that are passed on to the underlying loader
    pub option_string: String,
    /// height: `i32` -> Size to this height
    /// min: 1, max: 10000000, default: 1
    pub height: i32,
    /// size: `Size` -> Only upsize, only downsize, or both
    ///  `Both` -> VIPS_SIZE_BOTH = 0 [DEFAULT]
    ///  `Up` -> VIPS_SIZE_UP = 1
    ///  `Down` -> VIPS_SIZE_DOWN = 2
    ///  `Force` -> VIPS_SIZE_FORCE = 3
    ///  `Last` -> VIPS_SIZE_LAST = 4
    pub size: Size,
    /// no_rotate: `bool` -> Don't use orientation tags to rotate image upright
    /// default: false
    pub no_rotate: bool,
    /// crop: `Interesting` -> Reduce to fill target rectangle, then crop
    ///  `None` -> VIPS_INTERESTING_NONE = 0 [DEFAULT]
    ///  `Centre` -> VIPS_INTERESTING_CENTRE = 1
    ///  `Entropy` -> VIPS_INTERESTING_ENTROPY = 2
    ///  `Attention` -> VIPS_INTERESTING_ATTENTION = 3
    ///  `Low` -> VIPS_INTERESTING_LOW = 4
    ///  `High` -> VIPS_INTERESTING_HIGH = 5
    ///  `All` -> VIPS_INTERESTING_ALL = 6
    ///  `Last` -> VIPS_INTERESTING_LAST = 7
    pub crop: Interesting,
    /// linear: `bool` -> Reduce in linear light
    /// default: false
    pub linear: bool,
    /// import_profile: `String` -> Fallback import profile
    pub import_profile: String,
    /// export_profile: `String` -> Fallback export profile
    pub export_profile: String,
    /// intent: `Intent` -> Rendering intent
    ///  `Perceptual` -> VIPS_INTENT_PERCEPTUAL = 0
    ///  `Relative` -> VIPS_INTENT_RELATIVE = 1 [DEFAULT]
    ///  `Saturation` -> VIPS_INTENT_SATURATION = 2
    ///  `Absolute` -> VIPS_INTENT_ABSOLUTE = 3
    ///  `Last` -> VIPS_INTENT_LAST = 4
    pub intent: Intent,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
}

impl std::default::Default for ThumbnailBufferOptions {
    fn default() -> Self {
        ThumbnailBufferOptions {
            option_string: String::new(),
            height: i32::from(1),
            size: Size::Both,
            no_rotate: false,
            crop: Interesting::None,
            linear: false,
            import_profile: String::new(),
            export_profile: String::new(),
            intent: Intent::Relative,
            fail_on: FailOn::None,
        }
    }
}

/// VipsThumbnailBuffer (thumbnail_buffer), generate thumbnail from buffer
/// buffer: `&[u8]` -> Buffer to load from
/// width: `i32` -> Size to this width
/// min: 1, max: 10000000, default: 1
/// thumbnail_buffer_options: `&ThumbnailBufferOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn thumbnail_buffer_with_opts(
    buffer: &[u8],
    width: i32,
    thumbnail_buffer_options: &ThumbnailBufferOptions,
) -> Result<VipsImage> {
    unsafe {
        let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
        let width_in: i32 = width;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let option_string_in: CString =
            utils::new_c_string(&thumbnail_buffer_options.option_string)?;
        let option_string_in_name = utils::new_c_string("option-string")?;

        let height_in: i32 = thumbnail_buffer_options.height;
        let height_in_name = utils::new_c_string("height")?;

        let size_in: i32 = thumbnail_buffer_options.size as i32;
        let size_in_name = utils::new_c_string("size")?;

        let no_rotate_in: i32 = if thumbnail_buffer_options.no_rotate { 1 } else { 0 };
        let no_rotate_in_name = utils::new_c_string("no-rotate")?;

        let crop_in: i32 = thumbnail_buffer_options.crop as i32;
        let crop_in_name = utils::new_c_string("crop")?;

        let linear_in: i32 = if thumbnail_buffer_options.linear { 1 } else { 0 };
        let linear_in_name = utils::new_c_string("linear")?;

        let import_profile_in: CString =
            utils::new_c_string(&thumbnail_buffer_options.import_profile)?;
        let import_profile_in_name = utils::new_c_string("import-profile")?;

        let export_profile_in: CString =
            utils::new_c_string(&thumbnail_buffer_options.export_profile)?;
        let export_profile_in_name = utils::new_c_string("export-profile")?;

        let intent_in: i32 = thumbnail_buffer_options.intent as i32;
        let intent_in_name = utils::new_c_string("intent")?;

        let fail_on_in: i32 = thumbnail_buffer_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let vips_op_response = bindings::vips_thumbnail_buffer(
            buffer_in,
            buffer.len() as u64,
            &mut out_out,
            width_in,
            option_string_in_name.as_ptr(),
            option_string_in.as_ptr(),
            height_in_name.as_ptr(),
            height_in,
            size_in_name.as_ptr(),
            size_in,
            no_rotate_in_name.as_ptr(),
            no_rotate_in,
            crop_in_name.as_ptr(),
            crop_in,
            linear_in_name.as_ptr(),
            linear_in,
            import_profile_in_name.as_ptr(),
            import_profile_in.as_ptr(),
            export_profile_in_name.as_ptr(),
            export_profile_in.as_ptr(),
            intent_in_name.as_ptr(),
            intent_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ThumbnailBufferError,
        )
    }
}

/// VipsThumbnailImage (thumbnail_image), generate thumbnail from image
/// inp: `&VipsImage` -> Input image argument
/// width: `i32` -> Size to this width
/// min: 1, max: 10000000, default: 1
/// returns `VipsImage` - Output image
pub fn thumbnail_image(inp: &VipsImage, width: i32) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let width_in: i32 = width;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_thumbnail_image(
            inp_in,
            &mut out_out,
            width_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ThumbnailImageError,
        )
    }
}

/// Options for thumbnail_image operation
#[derive(Clone, Debug)]
pub struct ThumbnailImageOptions {
    /// height: `i32` -> Size to this height
    /// min: 1, max: 10000000, default: 1
    pub height: i32,
    /// size: `Size` -> Only upsize, only downsize, or both
    ///  `Both` -> VIPS_SIZE_BOTH = 0 [DEFAULT]
    ///  `Up` -> VIPS_SIZE_UP = 1
    ///  `Down` -> VIPS_SIZE_DOWN = 2
    ///  `Force` -> VIPS_SIZE_FORCE = 3
    ///  `Last` -> VIPS_SIZE_LAST = 4
    pub size: Size,
    /// no_rotate: `bool` -> Don't use orientation tags to rotate image upright
    /// default: false
    pub no_rotate: bool,
    /// crop: `Interesting` -> Reduce to fill target rectangle, then crop
    ///  `None` -> VIPS_INTERESTING_NONE = 0 [DEFAULT]
    ///  `Centre` -> VIPS_INTERESTING_CENTRE = 1
    ///  `Entropy` -> VIPS_INTERESTING_ENTROPY = 2
    ///  `Attention` -> VIPS_INTERESTING_ATTENTION = 3
    ///  `Low` -> VIPS_INTERESTING_LOW = 4
    ///  `High` -> VIPS_INTERESTING_HIGH = 5
    ///  `All` -> VIPS_INTERESTING_ALL = 6
    ///  `Last` -> VIPS_INTERESTING_LAST = 7
    pub crop: Interesting,
    /// linear: `bool` -> Reduce in linear light
    /// default: false
    pub linear: bool,
    /// import_profile: `String` -> Fallback import profile
    pub import_profile: String,
    /// export_profile: `String` -> Fallback export profile
    pub export_profile: String,
    /// intent: `Intent` -> Rendering intent
    ///  `Perceptual` -> VIPS_INTENT_PERCEPTUAL = 0
    ///  `Relative` -> VIPS_INTENT_RELATIVE = 1 [DEFAULT]
    ///  `Saturation` -> VIPS_INTENT_SATURATION = 2
    ///  `Absolute` -> VIPS_INTENT_ABSOLUTE = 3
    ///  `Last` -> VIPS_INTENT_LAST = 4
    pub intent: Intent,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
}

impl std::default::Default for ThumbnailImageOptions {
    fn default() -> Self {
        ThumbnailImageOptions {
            height: i32::from(1),
            size: Size::Both,
            no_rotate: false,
            crop: Interesting::None,
            linear: false,
            import_profile: String::new(),
            export_profile: String::new(),
            intent: Intent::Relative,
            fail_on: FailOn::None,
        }
    }
}

/// VipsThumbnailImage (thumbnail_image), generate thumbnail from image
/// inp: `&VipsImage` -> Input image argument
/// width: `i32` -> Size to this width
/// min: 1, max: 10000000, default: 1
/// thumbnail_image_options: `&ThumbnailImageOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn thumbnail_image_with_opts(
    inp: &VipsImage,
    width: i32,
    thumbnail_image_options: &ThumbnailImageOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let width_in: i32 = width;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let height_in: i32 = thumbnail_image_options.height;
        let height_in_name = utils::new_c_string("height")?;

        let size_in: i32 = thumbnail_image_options.size as i32;
        let size_in_name = utils::new_c_string("size")?;

        let no_rotate_in: i32 = if thumbnail_image_options.no_rotate { 1 } else { 0 };
        let no_rotate_in_name = utils::new_c_string("no-rotate")?;

        let crop_in: i32 = thumbnail_image_options.crop as i32;
        let crop_in_name = utils::new_c_string("crop")?;

        let linear_in: i32 = if thumbnail_image_options.linear { 1 } else { 0 };
        let linear_in_name = utils::new_c_string("linear")?;

        let import_profile_in: CString =
            utils::new_c_string(&thumbnail_image_options.import_profile)?;
        let import_profile_in_name = utils::new_c_string("import-profile")?;

        let export_profile_in: CString =
            utils::new_c_string(&thumbnail_image_options.export_profile)?;
        let export_profile_in_name = utils::new_c_string("export-profile")?;

        let intent_in: i32 = thumbnail_image_options.intent as i32;
        let intent_in_name = utils::new_c_string("intent")?;

        let fail_on_in: i32 = thumbnail_image_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let vips_op_response = bindings::vips_thumbnail_image(
            inp_in,
            &mut out_out,
            width_in,
            height_in_name.as_ptr(),
            height_in,
            size_in_name.as_ptr(),
            size_in,
            no_rotate_in_name.as_ptr(),
            no_rotate_in,
            crop_in_name.as_ptr(),
            crop_in,
            linear_in_name.as_ptr(),
            linear_in,
            import_profile_in_name.as_ptr(),
            import_profile_in.as_ptr(),
            export_profile_in_name.as_ptr(),
            export_profile_in.as_ptr(),
            intent_in_name.as_ptr(),
            intent_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ThumbnailImageError,
        )
    }
}

/// VipsThumbnailSource (thumbnail_source), generate thumbnail from source
/// source: `&VipsSource` -> Source to load from
/// width: `i32` -> Size to this width
/// min: 1, max: 10000000, default: 1
/// returns `VipsImage` - Output image
pub fn thumbnail_source(source: &VipsSource, width: i32) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let width_in: i32 = width;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_thumbnail_source(
            source_in,
            &mut out_out,
            width_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ThumbnailSourceError,
        )
    }
}

/// Options for thumbnail_source operation
#[derive(Clone, Debug)]
pub struct ThumbnailSourceOptions {
    /// option_string: `String` -> Options that are passed on to the underlying loader
    pub option_string: String,
    /// height: `i32` -> Size to this height
    /// min: 1, max: 10000000, default: 1
    pub height: i32,
    /// size: `Size` -> Only upsize, only downsize, or both
    ///  `Both` -> VIPS_SIZE_BOTH = 0 [DEFAULT]
    ///  `Up` -> VIPS_SIZE_UP = 1
    ///  `Down` -> VIPS_SIZE_DOWN = 2
    ///  `Force` -> VIPS_SIZE_FORCE = 3
    ///  `Last` -> VIPS_SIZE_LAST = 4
    pub size: Size,
    /// no_rotate: `bool` -> Don't use orientation tags to rotate image upright
    /// default: false
    pub no_rotate: bool,
    /// crop: `Interesting` -> Reduce to fill target rectangle, then crop
    ///  `None` -> VIPS_INTERESTING_NONE = 0 [DEFAULT]
    ///  `Centre` -> VIPS_INTERESTING_CENTRE = 1
    ///  `Entropy` -> VIPS_INTERESTING_ENTROPY = 2
    ///  `Attention` -> VIPS_INTERESTING_ATTENTION = 3
    ///  `Low` -> VIPS_INTERESTING_LOW = 4
    ///  `High` -> VIPS_INTERESTING_HIGH = 5
    ///  `All` -> VIPS_INTERESTING_ALL = 6
    ///  `Last` -> VIPS_INTERESTING_LAST = 7
    pub crop: Interesting,
    /// linear: `bool` -> Reduce in linear light
    /// default: false
    pub linear: bool,
    /// import_profile: `String` -> Fallback import profile
    pub import_profile: String,
    /// export_profile: `String` -> Fallback export profile
    pub export_profile: String,
    /// intent: `Intent` -> Rendering intent
    ///  `Perceptual` -> VIPS_INTENT_PERCEPTUAL = 0
    ///  `Relative` -> VIPS_INTENT_RELATIVE = 1 [DEFAULT]
    ///  `Saturation` -> VIPS_INTENT_SATURATION = 2
    ///  `Absolute` -> VIPS_INTENT_ABSOLUTE = 3
    ///  `Last` -> VIPS_INTENT_LAST = 4
    pub intent: Intent,
    /// fail_on: `FailOn` -> Error level to fail on
    ///  `None` -> VIPS_FAIL_ON_NONE = 0 [DEFAULT]
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    pub fail_on: FailOn,
}

impl std::default::Default for ThumbnailSourceOptions {
    fn default() -> Self {
        ThumbnailSourceOptions {
            option_string: String::new(),
            height: i32::from(1),
            size: Size::Both,
            no_rotate: false,
            crop: Interesting::None,
            linear: false,
            import_profile: String::new(),
            export_profile: String::new(),
            intent: Intent::Relative,
            fail_on: FailOn::None,
        }
    }
}

/// VipsThumbnailSource (thumbnail_source), generate thumbnail from source
/// source: `&VipsSource` -> Source to load from
/// width: `i32` -> Size to this width
/// min: 1, max: 10000000, default: 1
/// thumbnail_source_options: `&ThumbnailSourceOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn thumbnail_source_with_opts(
    source: &VipsSource,
    width: i32,
    thumbnail_source_options: &ThumbnailSourceOptions,
) -> Result<VipsImage> {
    unsafe {
        let source_in: *mut bindings::VipsSource = source.ctx;
        let width_in: i32 = width;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let option_string_in: CString =
            utils::new_c_string(&thumbnail_source_options.option_string)?;
        let option_string_in_name = utils::new_c_string("option-string")?;

        let height_in: i32 = thumbnail_source_options.height;
        let height_in_name = utils::new_c_string("height")?;

        let size_in: i32 = thumbnail_source_options.size as i32;
        let size_in_name = utils::new_c_string("size")?;

        let no_rotate_in: i32 = if thumbnail_source_options.no_rotate { 1 } else { 0 };
        let no_rotate_in_name = utils::new_c_string("no-rotate")?;

        let crop_in: i32 = thumbnail_source_options.crop as i32;
        let crop_in_name = utils::new_c_string("crop")?;

        let linear_in: i32 = if thumbnail_source_options.linear { 1 } else { 0 };
        let linear_in_name = utils::new_c_string("linear")?;

        let import_profile_in: CString =
            utils::new_c_string(&thumbnail_source_options.import_profile)?;
        let import_profile_in_name = utils::new_c_string("import-profile")?;

        let export_profile_in: CString =
            utils::new_c_string(&thumbnail_source_options.export_profile)?;
        let export_profile_in_name = utils::new_c_string("export-profile")?;

        let intent_in: i32 = thumbnail_source_options.intent as i32;
        let intent_in_name = utils::new_c_string("intent")?;

        let fail_on_in: i32 = thumbnail_source_options.fail_on as i32;
        let fail_on_in_name = utils::new_c_string("fail-on")?;

        let vips_op_response = bindings::vips_thumbnail_source(
            source_in,
            &mut out_out,
            width_in,
            option_string_in_name.as_ptr(),
            option_string_in.as_ptr(),
            height_in_name.as_ptr(),
            height_in,
            size_in_name.as_ptr(),
            size_in,
            no_rotate_in_name.as_ptr(),
            no_rotate_in,
            crop_in_name.as_ptr(),
            crop_in,
            linear_in_name.as_ptr(),
            linear_in,
            import_profile_in_name.as_ptr(),
            import_profile_in.as_ptr(),
            export_profile_in_name.as_ptr(),
            export_profile_in.as_ptr(),
            intent_in_name.as_ptr(),
            intent_in,
            fail_on_in_name.as_ptr(),
            fail_on_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ThumbnailSourceError,
        )
    }
}

/// VipsMapim (mapim), resample with a map image
/// inp: `&VipsImage` -> Input image argument
/// index: `&VipsImage` -> Index pixels with this
/// returns `VipsImage` - Output image
pub fn mapim(inp: &VipsImage, index: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let index_in: *mut bindings::VipsImage = index.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_mapim(
            inp_in,
            &mut out_out,
            index_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MapimError,
        )
    }
}

/// Options for mapim operation
#[derive(Clone, Debug)]
pub struct MapimOptions {
    /// interpolate: `VipsInterpolate` -> Interpolate pixels with this
    pub interpolate: VipsInterpolate,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// premultiplied: `bool` -> Images have premultiplied alpha
    /// default: false
    pub premultiplied: bool,
    /// extend: `Extend` -> How to generate the extra pixels
    ///  `Black` -> VIPS_EXTEND_BLACK = 0
    ///  `Copy` -> VIPS_EXTEND_COPY = 1
    ///  `Repeat` -> VIPS_EXTEND_REPEAT = 2
    ///  `Mirror` -> VIPS_EXTEND_MIRROR = 3
    ///  `White` -> VIPS_EXTEND_WHITE = 4
    ///  `Background` -> VIPS_EXTEND_BACKGROUND = 5 [DEFAULT]
    ///  `Last` -> VIPS_EXTEND_LAST = 6
    pub extend: Extend,
}

impl std::default::Default for MapimOptions {
    fn default() -> Self {
        MapimOptions {
            interpolate: VipsInterpolate::new(),
            background: Vec::new(),
            premultiplied: false,
            extend: Extend::Background,
        }
    }
}

/// VipsMapim (mapim), resample with a map image
/// inp: `&VipsImage` -> Input image argument
/// index: `&VipsImage` -> Index pixels with this
/// mapim_options: `&MapimOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn mapim_with_opts(
    inp: &VipsImage,
    index: &VipsImage,
    mapim_options: &MapimOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let index_in: *mut bindings::VipsImage = index.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let interpolate_in: *mut bindings::VipsInterpolate = mapim_options
            .interpolate
            .ctx;
        let interpolate_in_name = utils::new_c_string("interpolate")?;

        let background_wrapper = utils::VipsArrayDoubleWrapper::from(&mapim_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let premultiplied_in: i32 = if mapim_options.premultiplied { 1 } else { 0 };
        let premultiplied_in_name = utils::new_c_string("premultiplied")?;

        let extend_in: i32 = mapim_options.extend as i32;
        let extend_in_name = utils::new_c_string("extend")?;

        let vips_op_response = bindings::vips_mapim(
            inp_in,
            &mut out_out,
            index_in,
            interpolate_in_name.as_ptr(),
            interpolate_in,
            background_in_name.as_ptr(),
            background_in,
            premultiplied_in_name.as_ptr(),
            premultiplied_in,
            extend_in_name.as_ptr(),
            extend_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MapimError,
        )
    }
}

/// VipsShrink (shrink), shrink an image
/// inp: `&VipsImage` -> Input image argument
/// hshrink: `f64` -> Horizontal shrink factor
/// min: 1, max: 1000000, default: 1
/// vshrink: `f64` -> Vertical shrink factor
/// min: 1, max: 1000000, default: 1
/// returns `VipsImage` - Output image
pub fn shrink(inp: &VipsImage, hshrink: f64, vshrink: f64) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let hshrink_in: f64 = hshrink;
        let vshrink_in: f64 = vshrink;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_shrink(
            inp_in,
            &mut out_out,
            hshrink_in,
            vshrink_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ShrinkError,
        )
    }
}

/// Options for shrink operation
#[derive(Clone, Debug)]
pub struct ShrinkOptions {
    /// ceil: `bool` -> Round-up output dimensions
    /// default: false
    pub ceil: bool,
}

impl std::default::Default for ShrinkOptions {
    fn default() -> Self {
        ShrinkOptions {
            ceil: false,
        }
    }
}

/// VipsShrink (shrink), shrink an image
/// inp: `&VipsImage` -> Input image argument
/// hshrink: `f64` -> Horizontal shrink factor
/// min: 1, max: 1000000, default: 1
/// vshrink: `f64` -> Vertical shrink factor
/// min: 1, max: 1000000, default: 1
/// shrink_options: `&ShrinkOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn shrink_with_opts(
    inp: &VipsImage,
    hshrink: f64,
    vshrink: f64,
    shrink_options: &ShrinkOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let hshrink_in: f64 = hshrink;
        let vshrink_in: f64 = vshrink;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let ceil_in: i32 = if shrink_options.ceil { 1 } else { 0 };
        let ceil_in_name = utils::new_c_string("ceil")?;

        let vips_op_response = bindings::vips_shrink(
            inp_in,
            &mut out_out,
            hshrink_in,
            vshrink_in,
            ceil_in_name.as_ptr(),
            ceil_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ShrinkError,
        )
    }
}

/// VipsShrinkh (shrinkh), shrink an image horizontally
/// inp: `&VipsImage` -> Input image argument
/// hshrink: `i32` -> Horizontal shrink factor
/// min: 1, max: 1000000, default: 1
/// returns `VipsImage` - Output image
pub fn shrinkh(inp: &VipsImage, hshrink: i32) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let hshrink_in: i32 = hshrink;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_shrinkh(
            inp_in,
            &mut out_out,
            hshrink_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ShrinkhError,
        )
    }
}

/// Options for shrinkh operation
#[derive(Clone, Debug)]
pub struct ShrinkhOptions {
    /// ceil: `bool` -> Round-up output dimensions
    /// default: false
    pub ceil: bool,
}

impl std::default::Default for ShrinkhOptions {
    fn default() -> Self {
        ShrinkhOptions {
            ceil: false,
        }
    }
}

/// VipsShrinkh (shrinkh), shrink an image horizontally
/// inp: `&VipsImage` -> Input image argument
/// hshrink: `i32` -> Horizontal shrink factor
/// min: 1, max: 1000000, default: 1
/// shrinkh_options: `&ShrinkhOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn shrinkh_with_opts(
    inp: &VipsImage,
    hshrink: i32,
    shrinkh_options: &ShrinkhOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let hshrink_in: i32 = hshrink;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let ceil_in: i32 = if shrinkh_options.ceil { 1 } else { 0 };
        let ceil_in_name = utils::new_c_string("ceil")?;

        let vips_op_response = bindings::vips_shrinkh(
            inp_in,
            &mut out_out,
            hshrink_in,
            ceil_in_name.as_ptr(),
            ceil_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ShrinkhError,
        )
    }
}

/// VipsShrinkv (shrinkv), shrink an image vertically
/// inp: `&VipsImage` -> Input image argument
/// vshrink: `i32` -> Vertical shrink factor
/// min: 1, max: 1000000, default: 1
/// returns `VipsImage` - Output image
pub fn shrinkv(inp: &VipsImage, vshrink: i32) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let vshrink_in: i32 = vshrink;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_shrinkv(
            inp_in,
            &mut out_out,
            vshrink_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ShrinkvError,
        )
    }
}

/// Options for shrinkv operation
#[derive(Clone, Debug)]
pub struct ShrinkvOptions {
    /// ceil: `bool` -> Round-up output dimensions
    /// default: false
    pub ceil: bool,
}

impl std::default::Default for ShrinkvOptions {
    fn default() -> Self {
        ShrinkvOptions {
            ceil: false,
        }
    }
}

/// VipsShrinkv (shrinkv), shrink an image vertically
/// inp: `&VipsImage` -> Input image argument
/// vshrink: `i32` -> Vertical shrink factor
/// min: 1, max: 1000000, default: 1
/// shrinkv_options: `&ShrinkvOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn shrinkv_with_opts(
    inp: &VipsImage,
    vshrink: i32,
    shrinkv_options: &ShrinkvOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let vshrink_in: i32 = vshrink;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let ceil_in: i32 = if shrinkv_options.ceil { 1 } else { 0 };
        let ceil_in_name = utils::new_c_string("ceil")?;

        let vips_op_response = bindings::vips_shrinkv(
            inp_in,
            &mut out_out,
            vshrink_in,
            ceil_in_name.as_ptr(),
            ceil_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ShrinkvError,
        )
    }
}

/// VipsReduceh (reduceh), shrink an image horizontally
/// inp: `&VipsImage` -> Input image argument
/// hshrink: `f64` -> Horizontal shrink factor
/// min: 1, max: 1000000, default: 1
/// returns `VipsImage` - Output image
pub fn reduceh(inp: &VipsImage, hshrink: f64) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let hshrink_in: f64 = hshrink;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_reduceh(
            inp_in,
            &mut out_out,
            hshrink_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ReducehError,
        )
    }
}

/// Options for reduceh operation
#[derive(Clone, Debug)]
pub struct ReducehOptions {
    /// kernel: `Kernel` -> Resampling kernel
    ///  `Nearest` -> VIPS_KERNEL_NEAREST = 0
    ///  `Linear` -> VIPS_KERNEL_LINEAR = 1
    ///  `Cubic` -> VIPS_KERNEL_CUBIC = 2
    ///  `Mitchell` -> VIPS_KERNEL_MITCHELL = 3
    ///  `Lanczos2` -> VIPS_KERNEL_LANCZOS2 = 4
    ///  `Lanczos3` -> VIPS_KERNEL_LANCZOS3 = 5 [DEFAULT]
    ///  `Last` -> VIPS_KERNEL_LAST = 6
    pub kernel: Kernel,
    /// gap: `f64` -> Reducing gap
    /// min: 0, max: 1000000, default: 0
    pub gap: f64,
}

impl std::default::Default for ReducehOptions {
    fn default() -> Self {
        ReducehOptions {
            kernel: Kernel::Lanczos3,
            gap: f64::from(0),
        }
    }
}

/// VipsReduceh (reduceh), shrink an image horizontally
/// inp: `&VipsImage` -> Input image argument
/// hshrink: `f64` -> Horizontal shrink factor
/// min: 1, max: 1000000, default: 1
/// reduceh_options: `&ReducehOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn reduceh_with_opts(
    inp: &VipsImage,
    hshrink: f64,
    reduceh_options: &ReducehOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let hshrink_in: f64 = hshrink;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let kernel_in: i32 = reduceh_options.kernel as i32;
        let kernel_in_name = utils::new_c_string("kernel")?;

        let gap_in: f64 = reduceh_options.gap;
        let gap_in_name = utils::new_c_string("gap")?;

        let vips_op_response = bindings::vips_reduceh(
            inp_in,
            &mut out_out,
            hshrink_in,
            kernel_in_name.as_ptr(),
            kernel_in,
            gap_in_name.as_ptr(),
            gap_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ReducehError,
        )
    }
}

/// VipsReducev (reducev), shrink an image vertically
/// inp: `&VipsImage` -> Input image argument
/// vshrink: `f64` -> Vertical shrink factor
/// min: 1, max: 1000000, default: 1
/// returns `VipsImage` - Output image
pub fn reducev(inp: &VipsImage, vshrink: f64) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let vshrink_in: f64 = vshrink;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_reducev(
            inp_in,
            &mut out_out,
            vshrink_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ReducevError,
        )
    }
}

/// Options for reducev operation
#[derive(Clone, Debug)]
pub struct ReducevOptions {
    /// kernel: `Kernel` -> Resampling kernel
    ///  `Nearest` -> VIPS_KERNEL_NEAREST = 0
    ///  `Linear` -> VIPS_KERNEL_LINEAR = 1
    ///  `Cubic` -> VIPS_KERNEL_CUBIC = 2
    ///  `Mitchell` -> VIPS_KERNEL_MITCHELL = 3
    ///  `Lanczos2` -> VIPS_KERNEL_LANCZOS2 = 4
    ///  `Lanczos3` -> VIPS_KERNEL_LANCZOS3 = 5 [DEFAULT]
    ///  `Last` -> VIPS_KERNEL_LAST = 6
    pub kernel: Kernel,
    /// gap: `f64` -> Reducing gap
    /// min: 0, max: 1000000, default: 0
    pub gap: f64,
}

impl std::default::Default for ReducevOptions {
    fn default() -> Self {
        ReducevOptions {
            kernel: Kernel::Lanczos3,
            gap: f64::from(0),
        }
    }
}

/// VipsReducev (reducev), shrink an image vertically
/// inp: `&VipsImage` -> Input image argument
/// vshrink: `f64` -> Vertical shrink factor
/// min: 1, max: 1000000, default: 1
/// reducev_options: `&ReducevOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn reducev_with_opts(
    inp: &VipsImage,
    vshrink: f64,
    reducev_options: &ReducevOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let vshrink_in: f64 = vshrink;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let kernel_in: i32 = reducev_options.kernel as i32;
        let kernel_in_name = utils::new_c_string("kernel")?;

        let gap_in: f64 = reducev_options.gap;
        let gap_in_name = utils::new_c_string("gap")?;

        let vips_op_response = bindings::vips_reducev(
            inp_in,
            &mut out_out,
            vshrink_in,
            kernel_in_name.as_ptr(),
            kernel_in,
            gap_in_name.as_ptr(),
            gap_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ReducevError,
        )
    }
}

/// VipsReduce (reduce), reduce an image
/// inp: `&VipsImage` -> Input image argument
/// hshrink: `f64` -> Horizontal shrink factor
/// min: 1, max: 1000000, default: 1
/// vshrink: `f64` -> Vertical shrink factor
/// min: 1, max: 1000000, default: 1
/// returns `VipsImage` - Output image
pub fn reduce(inp: &VipsImage, hshrink: f64, vshrink: f64) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let hshrink_in: f64 = hshrink;
        let vshrink_in: f64 = vshrink;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_reduce(
            inp_in,
            &mut out_out,
            hshrink_in,
            vshrink_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ReduceError,
        )
    }
}

/// Options for reduce operation
#[derive(Clone, Debug)]
pub struct ReduceOptions {
    /// kernel: `Kernel` -> Resampling kernel
    ///  `Nearest` -> VIPS_KERNEL_NEAREST = 0
    ///  `Linear` -> VIPS_KERNEL_LINEAR = 1
    ///  `Cubic` -> VIPS_KERNEL_CUBIC = 2
    ///  `Mitchell` -> VIPS_KERNEL_MITCHELL = 3
    ///  `Lanczos2` -> VIPS_KERNEL_LANCZOS2 = 4
    ///  `Lanczos3` -> VIPS_KERNEL_LANCZOS3 = 5 [DEFAULT]
    ///  `Last` -> VIPS_KERNEL_LAST = 6
    pub kernel: Kernel,
    /// gap: `f64` -> Reducing gap
    /// min: 0, max: 1000000, default: 0
    pub gap: f64,
}

impl std::default::Default for ReduceOptions {
    fn default() -> Self {
        ReduceOptions {
            kernel: Kernel::Lanczos3,
            gap: f64::from(0),
        }
    }
}

/// VipsReduce (reduce), reduce an image
/// inp: `&VipsImage` -> Input image argument
/// hshrink: `f64` -> Horizontal shrink factor
/// min: 1, max: 1000000, default: 1
/// vshrink: `f64` -> Vertical shrink factor
/// min: 1, max: 1000000, default: 1
/// reduce_options: `&ReduceOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn reduce_with_opts(
    inp: &VipsImage,
    hshrink: f64,
    vshrink: f64,
    reduce_options: &ReduceOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let hshrink_in: f64 = hshrink;
        let vshrink_in: f64 = vshrink;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let kernel_in: i32 = reduce_options.kernel as i32;
        let kernel_in_name = utils::new_c_string("kernel")?;

        let gap_in: f64 = reduce_options.gap;
        let gap_in_name = utils::new_c_string("gap")?;

        let vips_op_response = bindings::vips_reduce(
            inp_in,
            &mut out_out,
            hshrink_in,
            vshrink_in,
            kernel_in_name.as_ptr(),
            kernel_in,
            gap_in_name.as_ptr(),
            gap_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ReduceError,
        )
    }
}

/// VipsQuadratic (quadratic), resample an image with a quadratic transform
/// inp: `&VipsImage` -> Input image argument
/// coeff: `&VipsImage` -> Coefficient matrix
/// returns `VipsImage` - Output image
pub fn quadratic(inp: &VipsImage, coeff: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let coeff_in: *mut bindings::VipsImage = coeff.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_quadratic(
            inp_in,
            &mut out_out,
            coeff_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::QuadraticError,
        )
    }
}

/// Options for quadratic operation
#[derive(Clone, Debug)]
pub struct QuadraticOptions {
    /// interpolate: `VipsInterpolate` -> Interpolate values with this
    pub interpolate: VipsInterpolate,
}

impl std::default::Default for QuadraticOptions {
    fn default() -> Self {
        QuadraticOptions {
            interpolate: VipsInterpolate::new(),
        }
    }
}

/// VipsQuadratic (quadratic), resample an image with a quadratic transform
/// inp: `&VipsImage` -> Input image argument
/// coeff: `&VipsImage` -> Coefficient matrix
/// quadratic_options: `&QuadraticOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn quadratic_with_opts(
    inp: &VipsImage,
    coeff: &VipsImage,
    quadratic_options: &QuadraticOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let coeff_in: *mut bindings::VipsImage = coeff.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let interpolate_in: *mut bindings::VipsInterpolate = quadratic_options
            .interpolate
            .ctx;
        let interpolate_in_name = utils::new_c_string("interpolate")?;

        let vips_op_response = bindings::vips_quadratic(
            inp_in,
            &mut out_out,
            coeff_in,
            interpolate_in_name.as_ptr(),
            interpolate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::QuadraticError,
        )
    }
}

/// VipsAffine (affine), affine transform of an image
/// inp: `&VipsImage` -> Input image argument
/// a: `f64` -> Transformation Matrix coefficient
/// min: -inf, max: inf, default: 0
/// b: `f64` -> Transformation Matrix coefficient
/// min: -inf, max: inf, default: 0
/// c: `f64` -> Transformation Matrix coefficient
/// min: -inf, max: inf, default: 0
/// d: `f64` -> Transformation Matrix coefficient
/// min: -inf, max: inf, default: 0
/// returns `VipsImage` - Output image
pub fn affine(inp: &VipsImage, a: f64, b: f64, c: f64, d: f64) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let a_in: f64 = a;
        let b_in: f64 = b;
        let c_in: f64 = c;
        let d_in: f64 = d;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_affine(
            inp_in,
            &mut out_out,
            a_in,
            b_in,
            c_in,
            d_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::AffineError,
        )
    }
}

/// Options for affine operation
#[derive(Clone, Debug)]
pub struct AffineOptions {
    /// interpolate: `VipsInterpolate` -> Interpolate pixels with this
    pub interpolate: VipsInterpolate,
    /// oarea: `Vec<i32>` -> Area of output to generate
    pub oarea: Vec<i32>,
    /// odx: `f64` -> Horizontal output displacement
    /// min: -10000000, max: 10000000, default: 0
    pub odx: f64,
    /// ody: `f64` -> Vertical output displacement
    /// min: -10000000, max: 10000000, default: 0
    pub ody: f64,
    /// idx: `f64` -> Horizontal input displacement
    /// min: -10000000, max: 10000000, default: 0
    pub idx: f64,
    /// idy: `f64` -> Vertical input displacement
    /// min: -10000000, max: 10000000, default: 0
    pub idy: f64,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// premultiplied: `bool` -> Images have premultiplied alpha
    /// default: false
    pub premultiplied: bool,
    /// extend: `Extend` -> How to generate the extra pixels
    ///  `Black` -> VIPS_EXTEND_BLACK = 0
    ///  `Copy` -> VIPS_EXTEND_COPY = 1
    ///  `Repeat` -> VIPS_EXTEND_REPEAT = 2
    ///  `Mirror` -> VIPS_EXTEND_MIRROR = 3
    ///  `White` -> VIPS_EXTEND_WHITE = 4
    ///  `Background` -> VIPS_EXTEND_BACKGROUND = 5 [DEFAULT]
    ///  `Last` -> VIPS_EXTEND_LAST = 6
    pub extend: Extend,
}

impl std::default::Default for AffineOptions {
    fn default() -> Self {
        AffineOptions {
            interpolate: VipsInterpolate::new(),
            oarea: Vec::new(),
            odx: f64::from(0),
            ody: f64::from(0),
            idx: f64::from(0),
            idy: f64::from(0),
            background: Vec::new(),
            premultiplied: false,
            extend: Extend::Background,
        }
    }
}

/// VipsAffine (affine), affine transform of an image
/// inp: `&VipsImage` -> Input image argument
/// a: `f64` -> Transformation Matrix coefficient
/// min: -inf, max: inf, default: 0
/// b: `f64` -> Transformation Matrix coefficient
/// min: -inf, max: inf, default: 0
/// c: `f64` -> Transformation Matrix coefficient
/// min: -inf, max: inf, default: 0
/// d: `f64` -> Transformation Matrix coefficient
/// min: -inf, max: inf, default: 0
/// affine_options: `&AffineOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn affine_with_opts(
    inp: &VipsImage,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    affine_options: &AffineOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let a_in: f64 = a;
        let b_in: f64 = b;
        let c_in: f64 = c;
        let d_in: f64 = d;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let interpolate_in: *mut bindings::VipsInterpolate = affine_options
            .interpolate
            .ctx;
        let interpolate_in_name = utils::new_c_string("interpolate")?;

        let oarea_wrapper = utils::VipsArrayIntWrapper::from(&affine_options.oarea[..]);
        let oarea_in = oarea_wrapper.ctx;
        let oarea_in_name = utils::new_c_string("oarea")?;

        let odx_in: f64 = affine_options.odx;
        let odx_in_name = utils::new_c_string("odx")?;

        let ody_in: f64 = affine_options.ody;
        let ody_in_name = utils::new_c_string("ody")?;

        let idx_in: f64 = affine_options.idx;
        let idx_in_name = utils::new_c_string("idx")?;

        let idy_in: f64 = affine_options.idy;
        let idy_in_name = utils::new_c_string("idy")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&affine_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let premultiplied_in: i32 = if affine_options.premultiplied { 1 } else { 0 };
        let premultiplied_in_name = utils::new_c_string("premultiplied")?;

        let extend_in: i32 = affine_options.extend as i32;
        let extend_in_name = utils::new_c_string("extend")?;

        let vips_op_response = bindings::vips_affine(
            inp_in,
            &mut out_out,
            a_in,
            b_in,
            c_in,
            d_in,
            interpolate_in_name.as_ptr(),
            interpolate_in,
            oarea_in_name.as_ptr(),
            oarea_in,
            odx_in_name.as_ptr(),
            odx_in,
            ody_in_name.as_ptr(),
            ody_in,
            idx_in_name.as_ptr(),
            idx_in,
            idy_in_name.as_ptr(),
            idy_in,
            background_in_name.as_ptr(),
            background_in,
            premultiplied_in_name.as_ptr(),
            premultiplied_in,
            extend_in_name.as_ptr(),
            extend_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::AffineError,
        )
    }
}

/// VipsSimilarity (similarity), similarity transform of an image
/// inp: `&VipsImage` -> Input image argument
/// returns `VipsImage` - Output image
pub fn similarity(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_similarity(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SimilarityError,
        )
    }
}

/// Options for similarity operation
#[derive(Clone, Debug)]
pub struct SimilarityOptions {
    /// scale: `f64` -> Scale by this factor
    /// min: 0, max: 10000000, default: 1
    pub scale: f64,
    /// angle: `f64` -> Rotate anticlockwise by this many degrees
    /// min: -10000000, max: 10000000, default: 0
    pub angle: f64,
    /// interpolate: `VipsInterpolate` -> Interpolate pixels with this
    pub interpolate: VipsInterpolate,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// odx: `f64` -> Horizontal output displacement
    /// min: -10000000, max: 10000000, default: 0
    pub odx: f64,
    /// ody: `f64` -> Vertical output displacement
    /// min: -10000000, max: 10000000, default: 0
    pub ody: f64,
    /// idx: `f64` -> Horizontal input displacement
    /// min: -10000000, max: 10000000, default: 0
    pub idx: f64,
    /// idy: `f64` -> Vertical input displacement
    /// min: -10000000, max: 10000000, default: 0
    pub idy: f64,
}

impl std::default::Default for SimilarityOptions {
    fn default() -> Self {
        SimilarityOptions {
            scale: f64::from(1),
            angle: f64::from(0),
            interpolate: VipsInterpolate::new(),
            background: Vec::new(),
            odx: f64::from(0),
            ody: f64::from(0),
            idx: f64::from(0),
            idy: f64::from(0),
        }
    }
}

/// VipsSimilarity (similarity), similarity transform of an image
/// inp: `&VipsImage` -> Input image argument
/// similarity_options: `&SimilarityOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn similarity_with_opts(
    inp: &VipsImage,
    similarity_options: &SimilarityOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let scale_in: f64 = similarity_options.scale;
        let scale_in_name = utils::new_c_string("scale")?;

        let angle_in: f64 = similarity_options.angle;
        let angle_in_name = utils::new_c_string("angle")?;

        let interpolate_in: *mut bindings::VipsInterpolate = similarity_options
            .interpolate
            .ctx;
        let interpolate_in_name = utils::new_c_string("interpolate")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&similarity_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let odx_in: f64 = similarity_options.odx;
        let odx_in_name = utils::new_c_string("odx")?;

        let ody_in: f64 = similarity_options.ody;
        let ody_in_name = utils::new_c_string("ody")?;

        let idx_in: f64 = similarity_options.idx;
        let idx_in_name = utils::new_c_string("idx")?;

        let idy_in: f64 = similarity_options.idy;
        let idy_in_name = utils::new_c_string("idy")?;

        let vips_op_response = bindings::vips_similarity(
            inp_in,
            &mut out_out,
            scale_in_name.as_ptr(),
            scale_in,
            angle_in_name.as_ptr(),
            angle_in,
            interpolate_in_name.as_ptr(),
            interpolate_in,
            background_in_name.as_ptr(),
            background_in,
            odx_in_name.as_ptr(),
            odx_in,
            ody_in_name.as_ptr(),
            ody_in,
            idx_in_name.as_ptr(),
            idx_in,
            idy_in_name.as_ptr(),
            idy_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SimilarityError,
        )
    }
}

/// VipsRotate (rotate), rotate an image by a number of degrees
/// inp: `&VipsImage` -> Input image argument
/// angle: `f64` -> Rotate anticlockwise by this many degrees
/// min: -10000000, max: 10000000, default: 0
/// returns `VipsImage` - Output image
pub fn rotate(inp: &VipsImage, angle: f64) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let angle_in: f64 = angle;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_rotate(
            inp_in,
            &mut out_out,
            angle_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RotateError,
        )
    }
}

/// Options for rotate operation
#[derive(Clone, Debug)]
pub struct RotateOptions {
    /// interpolate: `VipsInterpolate` -> Interpolate pixels with this
    pub interpolate: VipsInterpolate,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// odx: `f64` -> Horizontal output displacement
    /// min: -10000000, max: 10000000, default: 0
    pub odx: f64,
    /// ody: `f64` -> Vertical output displacement
    /// min: -10000000, max: 10000000, default: 0
    pub ody: f64,
    /// idx: `f64` -> Horizontal input displacement
    /// min: -10000000, max: 10000000, default: 0
    pub idx: f64,
    /// idy: `f64` -> Vertical input displacement
    /// min: -10000000, max: 10000000, default: 0
    pub idy: f64,
}

impl std::default::Default for RotateOptions {
    fn default() -> Self {
        RotateOptions {
            interpolate: VipsInterpolate::new(),
            background: Vec::new(),
            odx: f64::from(0),
            ody: f64::from(0),
            idx: f64::from(0),
            idy: f64::from(0),
        }
    }
}

/// VipsRotate (rotate), rotate an image by a number of degrees
/// inp: `&VipsImage` -> Input image argument
/// angle: `f64` -> Rotate anticlockwise by this many degrees
/// min: -10000000, max: 10000000, default: 0
/// rotate_options: `&RotateOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn rotate_with_opts(
    inp: &VipsImage,
    angle: f64,
    rotate_options: &RotateOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let angle_in: f64 = angle;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let interpolate_in: *mut bindings::VipsInterpolate = rotate_options
            .interpolate
            .ctx;
        let interpolate_in_name = utils::new_c_string("interpolate")?;

        let background_wrapper =
            utils::VipsArrayDoubleWrapper::from(&rotate_options.background[..]);
        let background_in = background_wrapper.ctx;
        let background_in_name = utils::new_c_string("background")?;

        let odx_in: f64 = rotate_options.odx;
        let odx_in_name = utils::new_c_string("odx")?;

        let ody_in: f64 = rotate_options.ody;
        let ody_in_name = utils::new_c_string("ody")?;

        let idx_in: f64 = rotate_options.idx;
        let idx_in_name = utils::new_c_string("idx")?;

        let idy_in: f64 = rotate_options.idy;
        let idy_in_name = utils::new_c_string("idy")?;

        let vips_op_response = bindings::vips_rotate(
            inp_in,
            &mut out_out,
            angle_in,
            interpolate_in_name.as_ptr(),
            interpolate_in,
            background_in_name.as_ptr(),
            background_in,
            odx_in_name.as_ptr(),
            odx_in,
            ody_in_name.as_ptr(),
            ody_in,
            idx_in_name.as_ptr(),
            idx_in,
            idy_in_name.as_ptr(),
            idy_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RotateError,
        )
    }
}

/// VipsResize (resize), resize an image
/// inp: `&VipsImage` -> Input image argument
/// scale: `f64` -> Scale image by this factor
/// min: 0, max: 10000000, default: 0
/// returns `VipsImage` - Output image
pub fn resize(inp: &VipsImage, scale: f64) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let scale_in: f64 = scale;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_resize(
            inp_in,
            &mut out_out,
            scale_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ResizeError,
        )
    }
}

/// Options for resize operation
#[derive(Clone, Debug)]
pub struct ResizeOptions {
    /// kernel: `Kernel` -> Resampling kernel
    ///  `Nearest` -> VIPS_KERNEL_NEAREST = 0
    ///  `Linear` -> VIPS_KERNEL_LINEAR = 1
    ///  `Cubic` -> VIPS_KERNEL_CUBIC = 2
    ///  `Mitchell` -> VIPS_KERNEL_MITCHELL = 3
    ///  `Lanczos2` -> VIPS_KERNEL_LANCZOS2 = 4
    ///  `Lanczos3` -> VIPS_KERNEL_LANCZOS3 = 5 [DEFAULT]
    ///  `Last` -> VIPS_KERNEL_LAST = 6
    pub kernel: Kernel,
    /// gap: `f64` -> Reducing gap
    /// min: 0, max: 1000000, default: 2
    pub gap: f64,
    /// vscale: `f64` -> Vertical scale image by this factor
    /// min: 0, max: 10000000, default: 0
    pub vscale: f64,
}

impl std::default::Default for ResizeOptions {
    fn default() -> Self {
        ResizeOptions {
            kernel: Kernel::Lanczos3,
            gap: f64::from(2),
            vscale: f64::from(0),
        }
    }
}

/// VipsResize (resize), resize an image
/// inp: `&VipsImage` -> Input image argument
/// scale: `f64` -> Scale image by this factor
/// min: 0, max: 10000000, default: 0
/// resize_options: `&ResizeOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn resize_with_opts(
    inp: &VipsImage,
    scale: f64,
    resize_options: &ResizeOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let scale_in: f64 = scale;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let kernel_in: i32 = resize_options.kernel as i32;
        let kernel_in_name = utils::new_c_string("kernel")?;

        let gap_in: f64 = resize_options.gap;
        let gap_in_name = utils::new_c_string("gap")?;

        let vscale_in: f64 = resize_options.vscale;
        let vscale_in_name = utils::new_c_string("vscale")?;

        let vips_op_response = bindings::vips_resize(
            inp_in,
            &mut out_out,
            scale_in,
            kernel_in_name.as_ptr(),
            kernel_in,
            gap_in_name.as_ptr(),
            gap_in,
            vscale_in_name.as_ptr(),
            vscale_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ResizeError,
        )
    }
}

/// VipsColourspace (colourspace), convert to a new colorspace
/// inp: `&VipsImage` -> Input image
/// space: `Interpretation` -> Destination color space
///  `Error` -> VIPS_INTERPRETATION_ERROR = -1
///  `Multiband` -> VIPS_INTERPRETATION_MULTIBAND = 0
///  `BW` -> VIPS_INTERPRETATION_B_W = 1
///  `Histogram` -> VIPS_INTERPRETATION_HISTOGRAM = 10
///  `Xyz` -> VIPS_INTERPRETATION_XYZ = 12
///  `Lab` -> VIPS_INTERPRETATION_LAB = 13
///  `Cmyk` -> VIPS_INTERPRETATION_CMYK = 15
///  `Labq` -> VIPS_INTERPRETATION_LABQ = 16
///  `Rgb` -> VIPS_INTERPRETATION_RGB = 17
///  `Cmc` -> VIPS_INTERPRETATION_CMC = 18
///  `Lch` -> VIPS_INTERPRETATION_LCH = 19
///  `Lab` -> VIPS_INTERPRETATION_LABS = 21
///  `Srgb` -> VIPS_INTERPRETATION_sRGB = 22 [DEFAULT]
///  `Yxy` -> VIPS_INTERPRETATION_YXY = 23
///  `Fourier` -> VIPS_INTERPRETATION_FOURIER = 24
///  `Rgb16` -> VIPS_INTERPRETATION_RGB16 = 25
///  `Grey16` -> VIPS_INTERPRETATION_GREY16 = 26
///  `Matrix` -> VIPS_INTERPRETATION_MATRIX = 27
///  `Scrgb` -> VIPS_INTERPRETATION_scRGB = 28
///  `Hsv` -> VIPS_INTERPRETATION_HSV = 29
///  `Last` -> VIPS_INTERPRETATION_LAST = 30
/// returns `VipsImage` - Output image
pub fn colourspace(inp: &VipsImage, space: Interpretation) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let space_in: i32 = space as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_colourspace(
            inp_in,
            &mut out_out,
            space_in
                .try_into()
                .unwrap(),
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ColourspaceError,
        )
    }
}

/// Options for colourspace operation
#[derive(Clone, Debug)]
pub struct ColourspaceOptions {
    /// source_space: `Interpretation` -> Source color space
    ///  `Error` -> VIPS_INTERPRETATION_ERROR = -1
    ///  `Multiband` -> VIPS_INTERPRETATION_MULTIBAND = 0
    ///  `BW` -> VIPS_INTERPRETATION_B_W = 1
    ///  `Histogram` -> VIPS_INTERPRETATION_HISTOGRAM = 10
    ///  `Xyz` -> VIPS_INTERPRETATION_XYZ = 12
    ///  `Lab` -> VIPS_INTERPRETATION_LAB = 13
    ///  `Cmyk` -> VIPS_INTERPRETATION_CMYK = 15
    ///  `Labq` -> VIPS_INTERPRETATION_LABQ = 16
    ///  `Rgb` -> VIPS_INTERPRETATION_RGB = 17
    ///  `Cmc` -> VIPS_INTERPRETATION_CMC = 18
    ///  `Lch` -> VIPS_INTERPRETATION_LCH = 19
    ///  `Lab` -> VIPS_INTERPRETATION_LABS = 21
    ///  `Srgb` -> VIPS_INTERPRETATION_sRGB = 22 [DEFAULT]
    ///  `Yxy` -> VIPS_INTERPRETATION_YXY = 23
    ///  `Fourier` -> VIPS_INTERPRETATION_FOURIER = 24
    ///  `Rgb16` -> VIPS_INTERPRETATION_RGB16 = 25
    ///  `Grey16` -> VIPS_INTERPRETATION_GREY16 = 26
    ///  `Matrix` -> VIPS_INTERPRETATION_MATRIX = 27
    ///  `Scrgb` -> VIPS_INTERPRETATION_scRGB = 28
    ///  `Hsv` -> VIPS_INTERPRETATION_HSV = 29
    ///  `Last` -> VIPS_INTERPRETATION_LAST = 30
    pub source_space: Interpretation,
}

impl std::default::Default for ColourspaceOptions {
    fn default() -> Self {
        ColourspaceOptions {
            source_space: Interpretation::Srgb,
        }
    }
}

/// VipsColourspace (colourspace), convert to a new colorspace
/// inp: `&VipsImage` -> Input image
/// space: `Interpretation` -> Destination color space
///  `Error` -> VIPS_INTERPRETATION_ERROR = -1
///  `Multiband` -> VIPS_INTERPRETATION_MULTIBAND = 0
///  `BW` -> VIPS_INTERPRETATION_B_W = 1
///  `Histogram` -> VIPS_INTERPRETATION_HISTOGRAM = 10
///  `Xyz` -> VIPS_INTERPRETATION_XYZ = 12
///  `Lab` -> VIPS_INTERPRETATION_LAB = 13
///  `Cmyk` -> VIPS_INTERPRETATION_CMYK = 15
///  `Labq` -> VIPS_INTERPRETATION_LABQ = 16
///  `Rgb` -> VIPS_INTERPRETATION_RGB = 17
///  `Cmc` -> VIPS_INTERPRETATION_CMC = 18
///  `Lch` -> VIPS_INTERPRETATION_LCH = 19
///  `Lab` -> VIPS_INTERPRETATION_LABS = 21
///  `Srgb` -> VIPS_INTERPRETATION_sRGB = 22 [DEFAULT]
///  `Yxy` -> VIPS_INTERPRETATION_YXY = 23
///  `Fourier` -> VIPS_INTERPRETATION_FOURIER = 24
///  `Rgb16` -> VIPS_INTERPRETATION_RGB16 = 25
///  `Grey16` -> VIPS_INTERPRETATION_GREY16 = 26
///  `Matrix` -> VIPS_INTERPRETATION_MATRIX = 27
///  `Scrgb` -> VIPS_INTERPRETATION_scRGB = 28
///  `Hsv` -> VIPS_INTERPRETATION_HSV = 29
///  `Last` -> VIPS_INTERPRETATION_LAST = 30
/// colourspace_options: `&ColourspaceOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn colourspace_with_opts(
    inp: &VipsImage,
    space: Interpretation,
    colourspace_options: &ColourspaceOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let space_in: i32 = space as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let source_space_in: i32 = colourspace_options.source_space as i32;
        let source_space_in_name = utils::new_c_string("source-space")?;

        let vips_op_response = bindings::vips_colourspace(
            inp_in,
            &mut out_out,
            space_in
                .try_into()
                .unwrap(),
            source_space_in_name.as_ptr(),
            source_space_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ColourspaceError,
        )
    }
}

/// VipsLab2XYZ (Lab2XYZ), transform CIELAB to XYZ
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn lab_2xyz(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_Lab2XYZ(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Lab2XyzError,
        )
    }
}

/// Options for lab_2xyz operation
#[derive(Clone, Debug)]
pub struct Lab2XyzOptions {
    /// temp: `Vec<f64>` -> Color temperature
    pub temp: Vec<f64>,
}

impl std::default::Default for Lab2XyzOptions {
    fn default() -> Self {
        Lab2XyzOptions {
            temp: Vec::new(),
        }
    }
}

/// VipsLab2XYZ (Lab2XYZ), transform CIELAB to XYZ
/// inp: `&VipsImage` -> Input image
/// lab_2xyz_options: `&Lab2XyzOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn lab_2xyz_with_opts(inp: &VipsImage, lab_2xyz_options: &Lab2XyzOptions) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let temp_wrapper = utils::VipsArrayDoubleWrapper::from(&lab_2xyz_options.temp[..]);
        let temp_in = temp_wrapper.ctx;
        let temp_in_name = utils::new_c_string("temp")?;

        let vips_op_response = bindings::vips_Lab2XYZ(
            inp_in,
            &mut out_out,
            temp_in_name.as_ptr(),
            temp_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Lab2XyzError,
        )
    }
}

/// VipsXYZ2Lab (XYZ2Lab), transform XYZ to Lab
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn xyz2_lab(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_XYZ2Lab(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Xyz2LabError,
        )
    }
}

/// Options for xyz2_lab operation
#[derive(Clone, Debug)]
pub struct Xyz2LabOptions {
    /// temp: `Vec<f64>` -> Colour temperature
    pub temp: Vec<f64>,
}

impl std::default::Default for Xyz2LabOptions {
    fn default() -> Self {
        Xyz2LabOptions {
            temp: Vec::new(),
        }
    }
}

/// VipsXYZ2Lab (XYZ2Lab), transform XYZ to Lab
/// inp: `&VipsImage` -> Input image
/// xyz2_lab_options: `&Xyz2LabOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn xyz2_lab_with_opts(
    inp: &VipsImage,
    xyz_2_lab_options: &Xyz2LabOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let temp_wrapper = utils::VipsArrayDoubleWrapper::from(&xyz_2_lab_options.temp[..]);
        let temp_in = temp_wrapper.ctx;
        let temp_in_name = utils::new_c_string("temp")?;

        let vips_op_response = bindings::vips_XYZ2Lab(
            inp_in,
            &mut out_out,
            temp_in_name.as_ptr(),
            temp_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Xyz2LabError,
        )
    }
}

/// VipsLab2LCh (Lab2LCh), transform Lab to LCh
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn lab_2l_ch(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_Lab2LCh(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Lab2LChError,
        )
    }
}

/// VipsLCh2Lab (LCh2Lab), transform LCh to Lab
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn l_ch_2_lab(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_LCh2Lab(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LCh2LabError,
        )
    }
}

/// VipsLCh2CMC (LCh2CMC), transform LCh to CMC
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn l_ch_2cmc(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_LCh2CMC(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LCh2CmcError,
        )
    }
}

/// VipsCMC2LCh (CMC2LCh), transform LCh to CMC
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn cmc2l_ch(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_CMC2LCh(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Cmc2LChError,
        )
    }
}

/// VipsXYZ2Yxy (XYZ2Yxy), transform XYZ to Yxy
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn xyz2_yxy(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_XYZ2Yxy(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Xyz2YxyError,
        )
    }
}

/// VipsYxy2XYZ (Yxy2XYZ), transform Yxy to XYZ
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn yxy_2xyz(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_Yxy2XYZ(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Yxy2XyzError,
        )
    }
}

/// VipsLabQ2Lab (LabQ2Lab), unpack a LabQ image to float Lab
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn lab_q2_lab(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_LabQ2Lab(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LabQ2LabError,
        )
    }
}

/// VipsLab2LabQ (Lab2LabQ), transform float Lab to LabQ coding
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn lab_2_lab_q(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_Lab2LabQ(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Lab2LabQError,
        )
    }
}

/// VipsLabQ2LabS (LabQ2LabS), unpack a LabQ image to short Lab
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn lab_q2_lab_s(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_LabQ2LabS(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LabQ2LabSError,
        )
    }
}

/// VipsLabS2LabQ (LabS2LabQ), transform short Lab to LabQ coding
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn lab_s2_lab_q(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_LabS2LabQ(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LabS2LabQError,
        )
    }
}

/// VipsLabS2Lab (LabS2Lab), transform signed short Lab to float
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn lab_s2_lab(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_LabS2Lab(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LabS2LabError,
        )
    }
}

/// VipsLab2LabS (Lab2LabS), transform float Lab to signed short
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn lab_2_lab_s(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_Lab2LabS(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Lab2LabSError,
        )
    }
}

/// VipsRad2float (rad2float), unpack Radiance coding to float RGB
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn rad_2float(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_rad2float(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Rad2FloatError,
        )
    }
}

/// VipsFloat2rad (float2rad), transform float RGB to Radiance coding
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn float_2rad(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_float2rad(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Float2RadError,
        )
    }
}

/// VipsLabQ2sRGB (LabQ2sRGB), convert a LabQ image to sRGB
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn lab_q_2s_rgb(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_LabQ2sRGB(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LabQ2SRgbError,
        )
    }
}

/// VipssRGB2HSV (sRGB2HSV), transform sRGB to HSV
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn s_rgb2hsv(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_sRGB2HSV(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SRgb2HsvError,
        )
    }
}

/// VipsHSV2sRGB (HSV2sRGB), transform HSV to sRGB
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn hsv_2s_rgb(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_HSV2sRGB(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Hsv2SRgbError,
        )
    }
}

/// VipsIccImport (icc_import), import from device with ICC profile
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn icc_import(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_icc_import(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::IccImportError,
        )
    }
}

/// Options for icc_import operation
#[derive(Clone, Debug)]
pub struct IccImportOptions {
    /// pcs: `PCS` -> Set Profile Connection Space
    ///  `Lab` -> VIPS_PCS_LAB = 0 [DEFAULT]
    ///  `Xyz` -> VIPS_PCS_XYZ = 1
    ///  `Last` -> VIPS_PCS_LAST = 2
    pub pcs: PCS,
    /// intent: `Intent` -> Rendering intent
    ///  `Perceptual` -> VIPS_INTENT_PERCEPTUAL = 0
    ///  `Relative` -> VIPS_INTENT_RELATIVE = 1 [DEFAULT]
    ///  `Saturation` -> VIPS_INTENT_SATURATION = 2
    ///  `Absolute` -> VIPS_INTENT_ABSOLUTE = 3
    ///  `Last` -> VIPS_INTENT_LAST = 4
    pub intent: Intent,
    /// black_point_compensation: `bool` -> Enable black point compensation
    /// default: false
    pub black_point_compensation: bool,
    /// embedded: `bool` -> Use embedded input profile, if available
    /// default: false
    pub embedded: bool,
    /// input_profile: `String` -> Filename to load input profile from
    pub input_profile: String,
}

impl std::default::Default for IccImportOptions {
    fn default() -> Self {
        IccImportOptions {
            pcs: PCS::Lab,
            intent: Intent::Relative,
            black_point_compensation: false,
            embedded: false,
            input_profile: String::new(),
        }
    }
}

/// VipsIccImport (icc_import), import from device with ICC profile
/// inp: `&VipsImage` -> Input image
/// icc_import_options: `&IccImportOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn icc_import_with_opts(
    inp: &VipsImage,
    icc_import_options: &IccImportOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let pcs_in: i32 = icc_import_options.pcs as i32;
        let pcs_in_name = utils::new_c_string("pcs")?;

        let intent_in: i32 = icc_import_options.intent as i32;
        let intent_in_name = utils::new_c_string("intent")?;

        let black_point_compensation_in: i32 =
            if icc_import_options.black_point_compensation { 1 } else { 0 };
        let black_point_compensation_in_name = utils::new_c_string("black-point-compensation")?;

        let embedded_in: i32 = if icc_import_options.embedded { 1 } else { 0 };
        let embedded_in_name = utils::new_c_string("embedded")?;

        let input_profile_in: CString = utils::new_c_string(&icc_import_options.input_profile)?;
        let input_profile_in_name = utils::new_c_string("input-profile")?;

        let vips_op_response = bindings::vips_icc_import(
            inp_in,
            &mut out_out,
            pcs_in_name.as_ptr(),
            pcs_in,
            intent_in_name.as_ptr(),
            intent_in,
            black_point_compensation_in_name.as_ptr(),
            black_point_compensation_in,
            embedded_in_name.as_ptr(),
            embedded_in,
            input_profile_in_name.as_ptr(),
            input_profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::IccImportError,
        )
    }
}

/// VipsIccExport (icc_export), output to device with ICC profile
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn icc_export(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_icc_export(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::IccExportError,
        )
    }
}

/// Options for icc_export operation
#[derive(Clone, Debug)]
pub struct IccExportOptions {
    /// pcs: `PCS` -> Set Profile Connection Space
    ///  `Lab` -> VIPS_PCS_LAB = 0 [DEFAULT]
    ///  `Xyz` -> VIPS_PCS_XYZ = 1
    ///  `Last` -> VIPS_PCS_LAST = 2
    pub pcs: PCS,
    /// intent: `Intent` -> Rendering intent
    ///  `Perceptual` -> VIPS_INTENT_PERCEPTUAL = 0
    ///  `Relative` -> VIPS_INTENT_RELATIVE = 1 [DEFAULT]
    ///  `Saturation` -> VIPS_INTENT_SATURATION = 2
    ///  `Absolute` -> VIPS_INTENT_ABSOLUTE = 3
    ///  `Last` -> VIPS_INTENT_LAST = 4
    pub intent: Intent,
    /// black_point_compensation: `bool` -> Enable black point compensation
    /// default: false
    pub black_point_compensation: bool,
    /// output_profile: `String` -> Filename to load output profile from
    pub output_profile: String,
    /// depth: `i32` -> Output device space depth in bits
    /// min: 8, max: 16, default: 8
    pub depth: i32,
}

impl std::default::Default for IccExportOptions {
    fn default() -> Self {
        IccExportOptions {
            pcs: PCS::Lab,
            intent: Intent::Relative,
            black_point_compensation: false,
            output_profile: String::new(),
            depth: i32::from(8),
        }
    }
}

/// VipsIccExport (icc_export), output to device with ICC profile
/// inp: `&VipsImage` -> Input image
/// icc_export_options: `&IccExportOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn icc_export_with_opts(
    inp: &VipsImage,
    icc_export_options: &IccExportOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let pcs_in: i32 = icc_export_options.pcs as i32;
        let pcs_in_name = utils::new_c_string("pcs")?;

        let intent_in: i32 = icc_export_options.intent as i32;
        let intent_in_name = utils::new_c_string("intent")?;

        let black_point_compensation_in: i32 =
            if icc_export_options.black_point_compensation { 1 } else { 0 };
        let black_point_compensation_in_name = utils::new_c_string("black-point-compensation")?;

        let output_profile_in: CString = utils::new_c_string(&icc_export_options.output_profile)?;
        let output_profile_in_name = utils::new_c_string("output-profile")?;

        let depth_in: i32 = icc_export_options.depth;
        let depth_in_name = utils::new_c_string("depth")?;

        let vips_op_response = bindings::vips_icc_export(
            inp_in,
            &mut out_out,
            pcs_in_name.as_ptr(),
            pcs_in,
            intent_in_name.as_ptr(),
            intent_in,
            black_point_compensation_in_name.as_ptr(),
            black_point_compensation_in,
            output_profile_in_name.as_ptr(),
            output_profile_in.as_ptr(),
            depth_in_name.as_ptr(),
            depth_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::IccExportError,
        )
    }
}

/// VipsIccTransform (icc_transform), transform between devices with ICC profiles
/// inp: `&VipsImage` -> Input image
/// output_profile: `&str` -> Filename to load output profile from
/// returns `VipsImage` - Output image
pub fn icc_transform(inp: &VipsImage, output_profile: &str) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let output_profile_in: CString = utils::new_c_string(output_profile)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_icc_transform(
            inp_in,
            &mut out_out,
            output_profile_in.as_ptr(),
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::IccTransformError,
        )
    }
}

/// Options for icc_transform operation
#[derive(Clone, Debug)]
pub struct IccTransformOptions {
    /// pcs: `PCS` -> Set Profile Connection Space
    ///  `Lab` -> VIPS_PCS_LAB = 0 [DEFAULT]
    ///  `Xyz` -> VIPS_PCS_XYZ = 1
    ///  `Last` -> VIPS_PCS_LAST = 2
    pub pcs: PCS,
    /// intent: `Intent` -> Rendering intent
    ///  `Perceptual` -> VIPS_INTENT_PERCEPTUAL = 0
    ///  `Relative` -> VIPS_INTENT_RELATIVE = 1 [DEFAULT]
    ///  `Saturation` -> VIPS_INTENT_SATURATION = 2
    ///  `Absolute` -> VIPS_INTENT_ABSOLUTE = 3
    ///  `Last` -> VIPS_INTENT_LAST = 4
    pub intent: Intent,
    /// black_point_compensation: `bool` -> Enable black point compensation
    /// default: false
    pub black_point_compensation: bool,
    /// embedded: `bool` -> Use embedded input profile, if available
    /// default: false
    pub embedded: bool,
    /// input_profile: `String` -> Filename to load input profile from
    pub input_profile: String,
    /// depth: `i32` -> Output device space depth in bits
    /// min: 8, max: 16, default: 8
    pub depth: i32,
}

impl std::default::Default for IccTransformOptions {
    fn default() -> Self {
        IccTransformOptions {
            pcs: PCS::Lab,
            intent: Intent::Relative,
            black_point_compensation: false,
            embedded: false,
            input_profile: String::new(),
            depth: i32::from(8),
        }
    }
}

/// VipsIccTransform (icc_transform), transform between devices with ICC profiles
/// inp: `&VipsImage` -> Input image
/// output_profile: `&str` -> Filename to load output profile from
/// icc_transform_options: `&IccTransformOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn icc_transform_with_opts(
    inp: &VipsImage,
    output_profile: &str,
    icc_transform_options: &IccTransformOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let output_profile_in: CString = utils::new_c_string(output_profile)?;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let pcs_in: i32 = icc_transform_options.pcs as i32;
        let pcs_in_name = utils::new_c_string("pcs")?;

        let intent_in: i32 = icc_transform_options.intent as i32;
        let intent_in_name = utils::new_c_string("intent")?;

        let black_point_compensation_in: i32 =
            if icc_transform_options.black_point_compensation { 1 } else { 0 };
        let black_point_compensation_in_name = utils::new_c_string("black-point-compensation")?;

        let embedded_in: i32 = if icc_transform_options.embedded { 1 } else { 0 };
        let embedded_in_name = utils::new_c_string("embedded")?;

        let input_profile_in: CString = utils::new_c_string(&icc_transform_options.input_profile)?;
        let input_profile_in_name = utils::new_c_string("input-profile")?;

        let depth_in: i32 = icc_transform_options.depth;
        let depth_in_name = utils::new_c_string("depth")?;

        let vips_op_response = bindings::vips_icc_transform(
            inp_in,
            &mut out_out,
            output_profile_in.as_ptr(),
            pcs_in_name.as_ptr(),
            pcs_in,
            intent_in_name.as_ptr(),
            intent_in,
            black_point_compensation_in_name.as_ptr(),
            black_point_compensation_in,
            embedded_in_name.as_ptr(),
            embedded_in,
            input_profile_in_name.as_ptr(),
            input_profile_in.as_ptr(),
            depth_in_name.as_ptr(),
            depth_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::IccTransformError,
        )
    }
}

/// VipsdE76 (dE76), calculate dE76
/// left: `&VipsImage` -> Left-hand input image
/// right: `&VipsImage` -> Right-hand input image
/// returns `VipsImage` - Output image
pub fn d_e76(left: &VipsImage, right: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let left_in: *mut bindings::VipsImage = left.ctx;
        let right_in: *mut bindings::VipsImage = right.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_dE76(
            left_in,
            right_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::DE76Error,
        )
    }
}

/// VipsdE00 (dE00), calculate dE00
/// left: `&VipsImage` -> Left-hand input image
/// right: `&VipsImage` -> Right-hand input image
/// returns `VipsImage` - Output image
pub fn d_e00(left: &VipsImage, right: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let left_in: *mut bindings::VipsImage = left.ctx;
        let right_in: *mut bindings::VipsImage = right.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_dE00(
            left_in,
            right_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::DE00Error,
        )
    }
}

/// VipsdECMC (dECMC), calculate dECMC
/// left: `&VipsImage` -> Left-hand input image
/// right: `&VipsImage` -> Right-hand input image
/// returns `VipsImage` - Output image
pub fn d_ecmc(left: &VipsImage, right: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let left_in: *mut bindings::VipsImage = left.ctx;
        let right_in: *mut bindings::VipsImage = right.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_dECMC(
            left_in,
            right_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::DEcmcError,
        )
    }
}

/// VipssRGB2scRGB (sRGB2scRGB), convert an sRGB image to scRGB
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn s_rgb_2sc_rgb(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_sRGB2scRGB(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SRgb2ScRgbError,
        )
    }
}

/// VipsscRGB2XYZ (scRGB2XYZ), transform scRGB to XYZ
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn sc_rgb2xyz(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_scRGB2XYZ(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ScRgb2XyzError,
        )
    }
}

/// VipsscRGB2BW (scRGB2BW), convert scRGB to BW
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn sc_rgb2bw(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_scRGB2BW(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ScRgb2BwError,
        )
    }
}

/// Options for sc_rgb2bw operation
#[derive(Clone, Debug)]
pub struct ScRgb2BwOptions {
    /// depth: `i32` -> Output device space depth in bits
    /// min: 8, max: 16, default: 8
    pub depth: i32,
}

impl std::default::Default for ScRgb2BwOptions {
    fn default() -> Self {
        ScRgb2BwOptions {
            depth: i32::from(8),
        }
    }
}

/// VipsscRGB2BW (scRGB2BW), convert scRGB to BW
/// inp: `&VipsImage` -> Input image
/// sc_rgb2bw_options: `&ScRgb2BwOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn sc_rgb2bw_with_opts(
    inp: &VipsImage,
    sc_rgb_2bw_options: &ScRgb2BwOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let depth_in: i32 = sc_rgb_2bw_options.depth;
        let depth_in_name = utils::new_c_string("depth")?;

        let vips_op_response = bindings::vips_scRGB2BW(
            inp_in,
            &mut out_out,
            depth_in_name.as_ptr(),
            depth_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ScRgb2BwError,
        )
    }
}

/// VipsXYZ2scRGB (XYZ2scRGB), transform XYZ to scRGB
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn xyz_2sc_rgb(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_XYZ2scRGB(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Xyz2ScRgbError,
        )
    }
}

/// VipsscRGB2sRGB (scRGB2sRGB), convert an scRGB image to sRGB
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn sc_rgb_2s_rgb(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_scRGB2sRGB(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ScRgb2SRgbError,
        )
    }
}

/// Options for sc_rgb_2s_rgb operation
#[derive(Clone, Debug)]
pub struct ScRgb2SRgbOptions {
    /// depth: `i32` -> Output device space depth in bits
    /// min: 8, max: 16, default: 8
    pub depth: i32,
}

impl std::default::Default for ScRgb2SRgbOptions {
    fn default() -> Self {
        ScRgb2SRgbOptions {
            depth: i32::from(8),
        }
    }
}

/// VipsscRGB2sRGB (scRGB2sRGB), convert an scRGB image to sRGB
/// inp: `&VipsImage` -> Input image
/// sc_rgb_2s_rgb_options: `&ScRgb2SRgbOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn sc_rgb_2s_rgb_with_opts(
    inp: &VipsImage,
    sc_rgb_2s_rgb_options: &ScRgb2SRgbOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let depth_in: i32 = sc_rgb_2s_rgb_options.depth;
        let depth_in_name = utils::new_c_string("depth")?;

        let vips_op_response = bindings::vips_scRGB2sRGB(
            inp_in,
            &mut out_out,
            depth_in_name.as_ptr(),
            depth_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ScRgb2SRgbError,
        )
    }
}

/// VipsCMYK2XYZ (CMYK2XYZ), transform CMYK to XYZ
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn cmyk2xyz(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_CMYK2XYZ(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Cmyk2XyzError,
        )
    }
}

/// VipsXYZ2CMYK (XYZ2CMYK), transform XYZ to CMYK
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn xyz2cmyk(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_XYZ2CMYK(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Xyz2CmykError,
        )
    }
}

/// VipsProfileLoad (profile_load), load named ICC profile
/// name: `&str` -> Profile name
/// returns `Vec<u8>` - Loaded profile
pub fn profile_load(name: &str) -> Result<Vec<u8>> {
    unsafe {
        let name_in: CString = utils::new_c_string(name)?;
        let mut profile_out: *mut bindings::VipsBlob = null_mut();

        let vips_op_response = bindings::vips_profile_load(
            name_in.as_ptr(),
            &mut profile_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsBlob {
                ctx: profile_out,
            }
            .into(),
            Error::ProfileLoadError,
        )
    }
}

/// VipsMaplut (maplut), map an image though a lut
/// inp: `&VipsImage` -> Input image
/// lut: `&VipsImage` -> Look-up table image
/// returns `VipsImage` - Output image
pub fn maplut(inp: &VipsImage, lut: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let lut_in: *mut bindings::VipsImage = lut.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_maplut(
            inp_in,
            &mut out_out,
            lut_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaplutError,
        )
    }
}

/// Options for maplut operation
#[derive(Clone, Debug)]
pub struct MaplutOptions {
    /// band: `i32` -> Apply one-band lut to this band of in
    /// min: -1, max: 10000, default: -1
    pub band: i32,
}

impl std::default::Default for MaplutOptions {
    fn default() -> Self {
        MaplutOptions {
            band: i32::from(-1),
        }
    }
}

/// VipsMaplut (maplut), map an image though a lut
/// inp: `&VipsImage` -> Input image
/// lut: `&VipsImage` -> Look-up table image
/// maplut_options: `&MaplutOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn maplut_with_opts(
    inp: &VipsImage,
    lut: &VipsImage,
    maplut_options: &MaplutOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let lut_in: *mut bindings::VipsImage = lut.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let band_in: i32 = maplut_options.band;
        let band_in_name = utils::new_c_string("band")?;

        let vips_op_response = bindings::vips_maplut(
            inp_in,
            &mut out_out,
            lut_in,
            band_in_name.as_ptr(),
            band_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaplutError,
        )
    }
}

/// VipsPercent (percent), find threshold for percent of pixels
/// inp: `&VipsImage` -> Input image
/// percent: `f64` -> Percent of pixels
/// min: 0, max: 100, default: 50
/// returns `i32` - Threshold above which lie percent of pixels
pub fn percent(inp: &VipsImage, percent: f64) -> Result<i32> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let percent_in: f64 = percent;
        let mut threshold_out: i32 = i32::from(0);

        let vips_op_response = bindings::vips_percent(
            inp_in,
            percent_in,
            &mut threshold_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            threshold_out,
            Error::PercentError,
        )
    }
}

/// VipsStdif (stdif), statistical difference
/// inp: `&VipsImage` -> Input image
/// width: `i32` -> Window width in pixels
/// min: 1, max: 256, default: 11
/// height: `i32` -> Window height in pixels
/// min: 1, max: 256, default: 11
/// returns `VipsImage` - Output image
pub fn stdif(inp: &VipsImage, width: i32, height: i32) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_stdif(
            inp_in,
            &mut out_out,
            width_in,
            height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::StdifError,
        )
    }
}

/// Options for stdif operation
#[derive(Clone, Debug)]
pub struct StdifOptions {
    /// s_0: `f64` -> New deviation
    /// min: -inf, max: inf, default: 50
    pub s_0: f64,
    /// b: `f64` -> Weight of new deviation
    /// min: 0, max: 2, default: 0.5
    pub b: f64,
    /// m_0: `f64` -> New mean
    /// min: -inf, max: inf, default: 128
    pub m_0: f64,
    /// a: `f64` -> Weight of new mean
    /// min: 0, max: 1, default: 0.5
    pub a: f64,
}

impl std::default::Default for StdifOptions {
    fn default() -> Self {
        StdifOptions {
            s_0: f64::from(50),
            b: f64::from(0.5),
            m_0: f64::from(128),
            a: f64::from(0.5),
        }
    }
}

/// VipsStdif (stdif), statistical difference
/// inp: `&VipsImage` -> Input image
/// width: `i32` -> Window width in pixels
/// min: 1, max: 256, default: 11
/// height: `i32` -> Window height in pixels
/// min: 1, max: 256, default: 11
/// stdif_options: `&StdifOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn stdif_with_opts(
    inp: &VipsImage,
    width: i32,
    height: i32,
    stdif_options: &StdifOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let s_0_in: f64 = stdif_options.s_0;
        let s_0_in_name = utils::new_c_string("s0")?;

        let b_in: f64 = stdif_options.b;
        let b_in_name = utils::new_c_string("b")?;

        let m_0_in: f64 = stdif_options.m_0;
        let m_0_in_name = utils::new_c_string("m0")?;

        let a_in: f64 = stdif_options.a;
        let a_in_name = utils::new_c_string("a")?;

        let vips_op_response = bindings::vips_stdif(
            inp_in,
            &mut out_out,
            width_in,
            height_in,
            s_0_in_name.as_ptr(),
            s_0_in,
            b_in_name.as_ptr(),
            b_in,
            m_0_in_name.as_ptr(),
            m_0_in,
            a_in_name.as_ptr(),
            a_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::StdifError,
        )
    }
}

/// VipsHistCum (hist_cum), form cumulative histogram
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn hist_cum(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_hist_cum(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistCumError,
        )
    }
}

/// VipsHistMatch (hist_match), match two histograms
/// inp: `&VipsImage` -> Input histogram
/// refp: `&VipsImage` -> Reference histogram
/// returns `VipsImage` - Output image
pub fn hist_match(inp: &VipsImage, refp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let refp_in: *mut bindings::VipsImage = refp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_hist_match(
            inp_in,
            refp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistMatchError,
        )
    }
}

/// VipsHistNorm (hist_norm), normalise histogram
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn hist_norm(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_hist_norm(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistNormError,
        )
    }
}

/// VipsHistEqual (hist_equal), histogram equalisation
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn hist_equal(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_hist_equal(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistEqualError,
        )
    }
}

/// Options for hist_equal operation
#[derive(Clone, Debug)]
pub struct HistEqualOptions {
    /// band: `i32` -> Equalise with this band
    /// min: -1, max: 100000, default: -1
    pub band: i32,
}

impl std::default::Default for HistEqualOptions {
    fn default() -> Self {
        HistEqualOptions {
            band: i32::from(-1),
        }
    }
}

/// VipsHistEqual (hist_equal), histogram equalisation
/// inp: `&VipsImage` -> Input image
/// hist_equal_options: `&HistEqualOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn hist_equal_with_opts(
    inp: &VipsImage,
    hist_equal_options: &HistEqualOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let band_in: i32 = hist_equal_options.band;
        let band_in_name = utils::new_c_string("band")?;

        let vips_op_response = bindings::vips_hist_equal(
            inp_in,
            &mut out_out,
            band_in_name.as_ptr(),
            band_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistEqualError,
        )
    }
}

/// VipsHistPlot (hist_plot), plot histogram
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn hist_plot(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_hist_plot(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistPlotError,
        )
    }
}

/// VipsHistLocal (hist_local), local histogram equalisation
/// inp: `&VipsImage` -> Input image
/// width: `i32` -> Window width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Window height in pixels
/// min: 1, max: 10000000, default: 1
/// returns `VipsImage` - Output image
pub fn hist_local(inp: &VipsImage, width: i32, height: i32) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_hist_local(
            inp_in,
            &mut out_out,
            width_in,
            height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistLocalError,
        )
    }
}

/// Options for hist_local operation
#[derive(Clone, Debug)]
pub struct HistLocalOptions {
    /// max_slope: `i32` -> Maximum slope (CLAHE)
    /// min: 0, max: 100, default: 0
    pub max_slope: i32,
}

impl std::default::Default for HistLocalOptions {
    fn default() -> Self {
        HistLocalOptions {
            max_slope: i32::from(0),
        }
    }
}

/// VipsHistLocal (hist_local), local histogram equalisation
/// inp: `&VipsImage` -> Input image
/// width: `i32` -> Window width in pixels
/// min: 1, max: 10000000, default: 1
/// height: `i32` -> Window height in pixels
/// min: 1, max: 10000000, default: 1
/// hist_local_options: `&HistLocalOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn hist_local_with_opts(
    inp: &VipsImage,
    width: i32,
    height: i32,
    hist_local_options: &HistLocalOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let width_in: i32 = width;
        let height_in: i32 = height;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let max_slope_in: i32 = hist_local_options.max_slope;
        let max_slope_in_name = utils::new_c_string("max-slope")?;

        let vips_op_response = bindings::vips_hist_local(
            inp_in,
            &mut out_out,
            width_in,
            height_in,
            max_slope_in_name.as_ptr(),
            max_slope_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistLocalError,
        )
    }
}

/// VipsHistIsmonotonic (hist_ismonotonic), test for monotonicity
/// inp: `&VipsImage` -> Input histogram image
/// returns `bool` - true if in is monotonic
pub fn hist_ismonotonic(inp: &VipsImage) -> Result<bool> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut monotonic_out: i32 = 0;

        let vips_op_response = bindings::vips_hist_ismonotonic(
            inp_in,
            &mut monotonic_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            monotonic_out != 0,
            Error::HistIsmonotonicError,
        )
    }
}

/// VipsHistEntropy (hist_entropy), estimate image entropy
/// inp: `&VipsImage` -> Input histogram image
/// returns `f64` - Output value
pub fn hist_entropy(inp: &VipsImage) -> Result<f64> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: f64 = f64::from(0);

        let vips_op_response = bindings::vips_hist_entropy(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::HistEntropyError,
        )
    }
}

/// VipsConv (conv), convolution operation
/// inp: `&VipsImage` -> Input image argument
/// mask: `&VipsImage` -> Input matrix image
/// returns `VipsImage` - Output image
pub fn conv(inp: &VipsImage, mask: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mask_in: *mut bindings::VipsImage = mask.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_conv(
            inp_in,
            &mut out_out,
            mask_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ConvError,
        )
    }
}

/// Options for conv operation
#[derive(Clone, Debug)]
pub struct ConvOptions {
    /// precision: `Precision` -> Convolve with this precision
    ///  `Integer` -> VIPS_PRECISION_INTEGER = 0
    ///  `Float` -> VIPS_PRECISION_FLOAT = 1 [DEFAULT]
    ///  `Approximate` -> VIPS_PRECISION_APPROXIMATE = 2
    ///  `Last` -> VIPS_PRECISION_LAST = 3
    pub precision: Precision,
    /// layers: `i32` -> Use this many layers in approximation
    /// min: 1, max: 1000, default: 5
    pub layers: i32,
    /// cluster: `i32` -> Cluster lines closer than this in approximation
    /// min: 1, max: 100, default: 1
    pub cluster: i32,
}

impl std::default::Default for ConvOptions {
    fn default() -> Self {
        ConvOptions {
            precision: Precision::Float,
            layers: i32::from(5),
            cluster: i32::from(1),
        }
    }
}

/// VipsConv (conv), convolution operation
/// inp: `&VipsImage` -> Input image argument
/// mask: `&VipsImage` -> Input matrix image
/// conv_options: `&ConvOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn conv_with_opts(
    inp: &VipsImage,
    mask: &VipsImage,
    conv_options: &ConvOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mask_in: *mut bindings::VipsImage = mask.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let precision_in: i32 = conv_options.precision as i32;
        let precision_in_name = utils::new_c_string("precision")?;

        let layers_in: i32 = conv_options.layers;
        let layers_in_name = utils::new_c_string("layers")?;

        let cluster_in: i32 = conv_options.cluster;
        let cluster_in_name = utils::new_c_string("cluster")?;

        let vips_op_response = bindings::vips_conv(
            inp_in,
            &mut out_out,
            mask_in,
            precision_in_name.as_ptr(),
            precision_in,
            layers_in_name.as_ptr(),
            layers_in,
            cluster_in_name.as_ptr(),
            cluster_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ConvError,
        )
    }
}

/// VipsConva (conva), approximate integer convolution
/// inp: `&VipsImage` -> Input image argument
/// mask: `&VipsImage` -> Input matrix image
/// returns `VipsImage` - Output image
pub fn conva(inp: &VipsImage, mask: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mask_in: *mut bindings::VipsImage = mask.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_conva(
            inp_in,
            &mut out_out,
            mask_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ConvaError,
        )
    }
}

/// Options for conva operation
#[derive(Clone, Debug)]
pub struct ConvaOptions {
    /// layers: `i32` -> Use this many layers in approximation
    /// min: 1, max: 1000, default: 5
    pub layers: i32,
    /// cluster: `i32` -> Cluster lines closer than this in approximation
    /// min: 1, max: 100, default: 1
    pub cluster: i32,
}

impl std::default::Default for ConvaOptions {
    fn default() -> Self {
        ConvaOptions {
            layers: i32::from(5),
            cluster: i32::from(1),
        }
    }
}

/// VipsConva (conva), approximate integer convolution
/// inp: `&VipsImage` -> Input image argument
/// mask: `&VipsImage` -> Input matrix image
/// conva_options: `&ConvaOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn conva_with_opts(
    inp: &VipsImage,
    mask: &VipsImage,
    conva_options: &ConvaOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mask_in: *mut bindings::VipsImage = mask.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let layers_in: i32 = conva_options.layers;
        let layers_in_name = utils::new_c_string("layers")?;

        let cluster_in: i32 = conva_options.cluster;
        let cluster_in_name = utils::new_c_string("cluster")?;

        let vips_op_response = bindings::vips_conva(
            inp_in,
            &mut out_out,
            mask_in,
            layers_in_name.as_ptr(),
            layers_in,
            cluster_in_name.as_ptr(),
            cluster_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ConvaError,
        )
    }
}

/// VipsConvf (convf), float convolution operation
/// inp: `&VipsImage` -> Input image argument
/// mask: `&VipsImage` -> Input matrix image
/// returns `VipsImage` - Output image
pub fn convf(inp: &VipsImage, mask: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mask_in: *mut bindings::VipsImage = mask.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_convf(
            inp_in,
            &mut out_out,
            mask_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ConvfError,
        )
    }
}

/// VipsConvi (convi), int convolution operation
/// inp: `&VipsImage` -> Input image argument
/// mask: `&VipsImage` -> Input matrix image
/// returns `VipsImage` - Output image
pub fn convi(inp: &VipsImage, mask: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mask_in: *mut bindings::VipsImage = mask.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_convi(
            inp_in,
            &mut out_out,
            mask_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ConviError,
        )
    }
}

/// VipsCompass (compass), convolve with rotating mask
/// inp: `&VipsImage` -> Input image argument
/// mask: `&VipsImage` -> Input matrix image
/// returns `VipsImage` - Output image
pub fn compass(inp: &VipsImage, mask: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mask_in: *mut bindings::VipsImage = mask.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_compass(
            inp_in,
            &mut out_out,
            mask_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CompassError,
        )
    }
}

/// Options for compass operation
#[derive(Clone, Debug)]
pub struct CompassOptions {
    /// times: `i32` -> Rotate and convolve this many times
    /// min: 1, max: 1000, default: 2
    pub times: i32,
    /// angle: `Angle45` -> Rotate mask by this much between convolutions
    ///  `D0` -> VIPS_ANGLE45_D0 = 0
    ///  `D45` -> VIPS_ANGLE45_D45 = 1
    ///  `D90` -> VIPS_ANGLE45_D90 = 2 [DEFAULT]
    ///  `D135` -> VIPS_ANGLE45_D135 = 3
    ///  `D180` -> VIPS_ANGLE45_D180 = 4
    ///  `D225` -> VIPS_ANGLE45_D225 = 5
    ///  `D270` -> VIPS_ANGLE45_D270 = 6
    ///  `D315` -> VIPS_ANGLE45_D315 = 7
    ///  `Last` -> VIPS_ANGLE45_LAST = 8
    pub angle: Angle45,
    /// combine: `Combine` -> Combine convolution results like this
    ///  `Max` -> VIPS_COMBINE_MAX = 0 [DEFAULT]
    ///  `Sum` -> VIPS_COMBINE_SUM = 1
    ///  `Min` -> VIPS_COMBINE_MIN = 2
    ///  `Last` -> VIPS_COMBINE_LAST = 3
    pub combine: Combine,
    /// precision: `Precision` -> Convolve with this precision
    ///  `Integer` -> VIPS_PRECISION_INTEGER = 0
    ///  `Float` -> VIPS_PRECISION_FLOAT = 1 [DEFAULT]
    ///  `Approximate` -> VIPS_PRECISION_APPROXIMATE = 2
    ///  `Last` -> VIPS_PRECISION_LAST = 3
    pub precision: Precision,
    /// layers: `i32` -> Use this many layers in approximation
    /// min: 1, max: 1000, default: 5
    pub layers: i32,
    /// cluster: `i32` -> Cluster lines closer than this in approximation
    /// min: 1, max: 100, default: 1
    pub cluster: i32,
}

impl std::default::Default for CompassOptions {
    fn default() -> Self {
        CompassOptions {
            times: i32::from(2),
            angle: Angle45::D90,
            combine: Combine::Max,
            precision: Precision::Float,
            layers: i32::from(5),
            cluster: i32::from(1),
        }
    }
}

/// VipsCompass (compass), convolve with rotating mask
/// inp: `&VipsImage` -> Input image argument
/// mask: `&VipsImage` -> Input matrix image
/// compass_options: `&CompassOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn compass_with_opts(
    inp: &VipsImage,
    mask: &VipsImage,
    compass_options: &CompassOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mask_in: *mut bindings::VipsImage = mask.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let times_in: i32 = compass_options.times;
        let times_in_name = utils::new_c_string("times")?;

        let angle_in: i32 = compass_options.angle as i32;
        let angle_in_name = utils::new_c_string("angle")?;

        let combine_in: i32 = compass_options.combine as i32;
        let combine_in_name = utils::new_c_string("combine")?;

        let precision_in: i32 = compass_options.precision as i32;
        let precision_in_name = utils::new_c_string("precision")?;

        let layers_in: i32 = compass_options.layers;
        let layers_in_name = utils::new_c_string("layers")?;

        let cluster_in: i32 = compass_options.cluster;
        let cluster_in_name = utils::new_c_string("cluster")?;

        let vips_op_response = bindings::vips_compass(
            inp_in,
            &mut out_out,
            mask_in,
            times_in_name.as_ptr(),
            times_in,
            angle_in_name.as_ptr(),
            angle_in,
            combine_in_name.as_ptr(),
            combine_in,
            precision_in_name.as_ptr(),
            precision_in,
            layers_in_name.as_ptr(),
            layers_in,
            cluster_in_name.as_ptr(),
            cluster_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CompassError,
        )
    }
}

/// VipsConvsep (convsep), separable convolution operation
/// inp: `&VipsImage` -> Input image argument
/// mask: `&VipsImage` -> Input matrix image
/// returns `VipsImage` - Output image
pub fn convsep(inp: &VipsImage, mask: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mask_in: *mut bindings::VipsImage = mask.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_convsep(
            inp_in,
            &mut out_out,
            mask_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ConvsepError,
        )
    }
}

/// Options for convsep operation
#[derive(Clone, Debug)]
pub struct ConvsepOptions {
    /// precision: `Precision` -> Convolve with this precision
    ///  `Integer` -> VIPS_PRECISION_INTEGER = 0
    ///  `Float` -> VIPS_PRECISION_FLOAT = 1 [DEFAULT]
    ///  `Approximate` -> VIPS_PRECISION_APPROXIMATE = 2
    ///  `Last` -> VIPS_PRECISION_LAST = 3
    pub precision: Precision,
    /// layers: `i32` -> Use this many layers in approximation
    /// min: 1, max: 1000, default: 5
    pub layers: i32,
    /// cluster: `i32` -> Cluster lines closer than this in approximation
    /// min: 1, max: 100, default: 1
    pub cluster: i32,
}

impl std::default::Default for ConvsepOptions {
    fn default() -> Self {
        ConvsepOptions {
            precision: Precision::Float,
            layers: i32::from(5),
            cluster: i32::from(1),
        }
    }
}

/// VipsConvsep (convsep), separable convolution operation
/// inp: `&VipsImage` -> Input image argument
/// mask: `&VipsImage` -> Input matrix image
/// convsep_options: `&ConvsepOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn convsep_with_opts(
    inp: &VipsImage,
    mask: &VipsImage,
    convsep_options: &ConvsepOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mask_in: *mut bindings::VipsImage = mask.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let precision_in: i32 = convsep_options.precision as i32;
        let precision_in_name = utils::new_c_string("precision")?;

        let layers_in: i32 = convsep_options.layers;
        let layers_in_name = utils::new_c_string("layers")?;

        let cluster_in: i32 = convsep_options.cluster;
        let cluster_in_name = utils::new_c_string("cluster")?;

        let vips_op_response = bindings::vips_convsep(
            inp_in,
            &mut out_out,
            mask_in,
            precision_in_name.as_ptr(),
            precision_in,
            layers_in_name.as_ptr(),
            layers_in,
            cluster_in_name.as_ptr(),
            cluster_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ConvsepError,
        )
    }
}

/// VipsConvasep (convasep), approximate separable integer convolution
/// inp: `&VipsImage` -> Input image argument
/// mask: `&VipsImage` -> Input matrix image
/// returns `VipsImage` - Output image
pub fn convasep(inp: &VipsImage, mask: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mask_in: *mut bindings::VipsImage = mask.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_convasep(
            inp_in,
            &mut out_out,
            mask_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ConvasepError,
        )
    }
}

/// Options for convasep operation
#[derive(Clone, Debug)]
pub struct ConvasepOptions {
    /// layers: `i32` -> Use this many layers in approximation
    /// min: 1, max: 1000, default: 5
    pub layers: i32,
}

impl std::default::Default for ConvasepOptions {
    fn default() -> Self {
        ConvasepOptions {
            layers: i32::from(5),
        }
    }
}

/// VipsConvasep (convasep), approximate separable integer convolution
/// inp: `&VipsImage` -> Input image argument
/// mask: `&VipsImage` -> Input matrix image
/// convasep_options: `&ConvasepOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn convasep_with_opts(
    inp: &VipsImage,
    mask: &VipsImage,
    convasep_options: &ConvasepOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mask_in: *mut bindings::VipsImage = mask.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let layers_in: i32 = convasep_options.layers;
        let layers_in_name = utils::new_c_string("layers")?;

        let vips_op_response = bindings::vips_convasep(
            inp_in,
            &mut out_out,
            mask_in,
            layers_in_name.as_ptr(),
            layers_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ConvasepError,
        )
    }
}

/// VipsFastcor (fastcor), fast correlation
/// inp: `&VipsImage` -> Input image argument
/// refp: `&VipsImage` -> Input reference image
/// returns `VipsImage` - Output image
pub fn fastcor(inp: &VipsImage, refp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let refp_in: *mut bindings::VipsImage = refp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_fastcor(
            inp_in,
            refp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::FastcorError,
        )
    }
}

/// VipsSpcor (spcor), spatial correlation
/// inp: `&VipsImage` -> Input image argument
/// refp: `&VipsImage` -> Input reference image
/// returns `VipsImage` - Output image
pub fn spcor(inp: &VipsImage, refp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let refp_in: *mut bindings::VipsImage = refp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_spcor(
            inp_in,
            refp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SpcorError,
        )
    }
}

/// VipsSharpen (sharpen), unsharp masking for print
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn sharpen(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_sharpen(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SharpenError,
        )
    }
}

/// Options for sharpen operation
#[derive(Clone, Debug)]
pub struct SharpenOptions {
    /// sigma: `f64` -> Sigma of Gaussian
    /// min: 0.000001, max: 10, default: 0.5
    pub sigma: f64,
    /// x_1: `f64` -> Flat/jaggy threshold
    /// min: 0, max: 1000000, default: 2
    pub x_1: f64,
    /// y_2: `f64` -> Maximum brightening
    /// min: 0, max: 1000000, default: 10
    pub y_2: f64,
    /// y_3: `f64` -> Maximum darkening
    /// min: 0, max: 1000000, default: 20
    pub y_3: f64,
    /// m_1: `f64` -> Slope for flat areas
    /// min: 0, max: 1000000, default: 0
    pub m_1: f64,
    /// m_2: `f64` -> Slope for jaggy areas
    /// min: 0, max: 1000000, default: 3
    pub m_2: f64,
}

impl std::default::Default for SharpenOptions {
    fn default() -> Self {
        SharpenOptions {
            sigma: f64::from(0.5),
            x_1: f64::from(2),
            y_2: f64::from(10),
            y_3: f64::from(20),
            m_1: f64::from(0),
            m_2: f64::from(3),
        }
    }
}

/// VipsSharpen (sharpen), unsharp masking for print
/// inp: `&VipsImage` -> Input image
/// sharpen_options: `&SharpenOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn sharpen_with_opts(inp: &VipsImage, sharpen_options: &SharpenOptions) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let sigma_in: f64 = sharpen_options.sigma;
        let sigma_in_name = utils::new_c_string("sigma")?;

        let x_1_in: f64 = sharpen_options.x_1;
        let x_1_in_name = utils::new_c_string("x1")?;

        let y_2_in: f64 = sharpen_options.y_2;
        let y_2_in_name = utils::new_c_string("y2")?;

        let y_3_in: f64 = sharpen_options.y_3;
        let y_3_in_name = utils::new_c_string("y3")?;

        let m_1_in: f64 = sharpen_options.m_1;
        let m_1_in_name = utils::new_c_string("m1")?;

        let m_2_in: f64 = sharpen_options.m_2;
        let m_2_in_name = utils::new_c_string("m2")?;

        let vips_op_response = bindings::vips_sharpen(
            inp_in,
            &mut out_out,
            sigma_in_name.as_ptr(),
            sigma_in,
            x_1_in_name.as_ptr(),
            x_1_in,
            y_2_in_name.as_ptr(),
            y_2_in,
            y_3_in_name.as_ptr(),
            y_3_in,
            m_1_in_name.as_ptr(),
            m_1_in,
            m_2_in_name.as_ptr(),
            m_2_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SharpenError,
        )
    }
}

/// VipsGaussblur (gaussblur), gaussian blur
/// inp: `&VipsImage` -> Input image
/// sigma: `f64` -> Sigma of Gaussian
/// min: 0, max: 1000, default: 1.5
/// returns `VipsImage` - Output image
pub fn gaussblur(inp: &VipsImage, sigma: f64) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let sigma_in: f64 = sigma;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_gaussblur(
            inp_in,
            &mut out_out,
            sigma_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GaussblurError,
        )
    }
}

/// Options for gaussblur operation
#[derive(Clone, Debug)]
pub struct GaussblurOptions {
    /// min_ampl: `f64` -> Minimum amplitude of Gaussian
    /// min: 0.001, max: 1, default: 0.2
    pub min_ampl: f64,
    /// precision: `Precision` -> Convolve with this precision
    ///  `Integer` -> VIPS_PRECISION_INTEGER = 0 [DEFAULT]
    ///  `Float` -> VIPS_PRECISION_FLOAT = 1
    ///  `Approximate` -> VIPS_PRECISION_APPROXIMATE = 2
    ///  `Last` -> VIPS_PRECISION_LAST = 3
    pub precision: Precision,
}

impl std::default::Default for GaussblurOptions {
    fn default() -> Self {
        GaussblurOptions {
            min_ampl: f64::from(0.2),
            precision: Precision::Integer,
        }
    }
}

/// VipsGaussblur (gaussblur), gaussian blur
/// inp: `&VipsImage` -> Input image
/// sigma: `f64` -> Sigma of Gaussian
/// min: 0, max: 1000, default: 1.5
/// gaussblur_options: `&GaussblurOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn gaussblur_with_opts(
    inp: &VipsImage,
    sigma: f64,
    gaussblur_options: &GaussblurOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let sigma_in: f64 = sigma;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let min_ampl_in: f64 = gaussblur_options.min_ampl;
        let min_ampl_in_name = utils::new_c_string("min-ampl")?;

        let precision_in: i32 = gaussblur_options.precision as i32;
        let precision_in_name = utils::new_c_string("precision")?;

        let vips_op_response = bindings::vips_gaussblur(
            inp_in,
            &mut out_out,
            sigma_in,
            min_ampl_in_name.as_ptr(),
            min_ampl_in,
            precision_in_name.as_ptr(),
            precision_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GaussblurError,
        )
    }
}

/// VipsSobel (sobel), Sobel edge detector
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn sobel(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_sobel(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SobelError,
        )
    }
}

/// VipsScharr (scharr), Scharr edge detector
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn scharr(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_scharr(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ScharrError,
        )
    }
}

/// VipsPrewitt (prewitt), Prewitt edge detector
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn prewitt(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_prewitt(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PrewittError,
        )
    }
}

/// VipsCanny (canny), Canny edge detector
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn canny(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_canny(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CannyError,
        )
    }
}

/// Options for canny operation
#[derive(Clone, Debug)]
pub struct CannyOptions {
    /// sigma: `f64` -> Sigma of Gaussian
    /// min: 0.01, max: 1000, default: 1.4
    pub sigma: f64,
    /// precision: `Precision` -> Convolve with this precision
    ///  `Integer` -> VIPS_PRECISION_INTEGER = 0
    ///  `Float` -> VIPS_PRECISION_FLOAT = 1 [DEFAULT]
    ///  `Approximate` -> VIPS_PRECISION_APPROXIMATE = 2
    ///  `Last` -> VIPS_PRECISION_LAST = 3
    pub precision: Precision,
}

impl std::default::Default for CannyOptions {
    fn default() -> Self {
        CannyOptions {
            sigma: f64::from(1.4),
            precision: Precision::Float,
        }
    }
}

/// VipsCanny (canny), Canny edge detector
/// inp: `&VipsImage` -> Input image
/// canny_options: `&CannyOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn canny_with_opts(inp: &VipsImage, canny_options: &CannyOptions) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let sigma_in: f64 = canny_options.sigma;
        let sigma_in_name = utils::new_c_string("sigma")?;

        let precision_in: i32 = canny_options.precision as i32;
        let precision_in_name = utils::new_c_string("precision")?;

        let vips_op_response = bindings::vips_canny(
            inp_in,
            &mut out_out,
            sigma_in_name.as_ptr(),
            sigma_in,
            precision_in_name.as_ptr(),
            precision_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CannyError,
        )
    }
}

/// VipsFwfft (fwfft), forward FFT
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn fwfft(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_fwfft(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::FwfftError,
        )
    }
}

/// VipsInvfft (invfft), inverse FFT
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn invfft(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_invfft(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::InvfftError,
        )
    }
}

/// Options for invfft operation
#[derive(Clone, Debug)]
pub struct InvfftOptions {
    /// real: `bool` -> Output only the real part of the transform
    /// default: false
    pub real: bool,
}

impl std::default::Default for InvfftOptions {
    fn default() -> Self {
        InvfftOptions {
            real: false,
        }
    }
}

/// VipsInvfft (invfft), inverse FFT
/// inp: `&VipsImage` -> Input image
/// invfft_options: `&InvfftOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn invfft_with_opts(inp: &VipsImage, invfft_options: &InvfftOptions) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let real_in: i32 = if invfft_options.real { 1 } else { 0 };
        let real_in_name = utils::new_c_string("real")?;

        let vips_op_response = bindings::vips_invfft(
            inp_in,
            &mut out_out,
            real_in_name.as_ptr(),
            real_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::InvfftError,
        )
    }
}

/// VipsFreqmult (freqmult), frequency-domain filtering
/// inp: `&VipsImage` -> Input image
/// mask: `&VipsImage` -> Input mask image
/// returns `VipsImage` - Output image
pub fn freqmult(inp: &VipsImage, mask: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mask_in: *mut bindings::VipsImage = mask.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_freqmult(
            inp_in,
            mask_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::FreqmultError,
        )
    }
}

/// VipsSpectrum (spectrum), make displayable power spectrum
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn spectrum(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_spectrum(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SpectrumError,
        )
    }
}

/// VipsPhasecor (phasecor), calculate phase correlation
/// inp: `&VipsImage` -> Input image
/// in_2: `&VipsImage` -> Second input image
/// returns `VipsImage` - Output image
pub fn phasecor(inp: &VipsImage, in_2: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let in_2_in: *mut bindings::VipsImage = in_2.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_phasecor(
            inp_in,
            in_2_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PhasecorError,
        )
    }
}

/// VipsMorph (morph), morphology operation
/// inp: `&VipsImage` -> Input image argument
/// mask: `&VipsImage` -> Input matrix image
/// morph: `OperationMorphology` -> Morphological operation to perform
///  `Erode` -> VIPS_OPERATION_MORPHOLOGY_ERODE = 0 [DEFAULT]
///  `Dilate` -> VIPS_OPERATION_MORPHOLOGY_DILATE = 1
///  `Last` -> VIPS_OPERATION_MORPHOLOGY_LAST = 2
/// returns `VipsImage` - Output image
pub fn morph(inp: &VipsImage, mask: &VipsImage, morph: OperationMorphology) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mask_in: *mut bindings::VipsImage = mask.ctx;
        let morph_in: i32 = morph as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_morph(
            inp_in,
            &mut out_out,
            mask_in,
            morph_in
                .try_into()
                .unwrap(),
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MorphError,
        )
    }
}

/// VipsRank (rank), rank filter
/// inp: `&VipsImage` -> Input image argument
/// width: `i32` -> Window width in pixels
/// min: 1, max: 100000, default: 11
/// height: `i32` -> Window height in pixels
/// min: 1, max: 100000, default: 11
/// index: `i32` -> Select pixel at index
/// min: 0, max: 100000000, default: 50
/// returns `VipsImage` - Output image
pub fn rank(inp: &VipsImage, width: i32, height: i32, index: i32) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let width_in: i32 = width;
        let height_in: i32 = height;
        let index_in: i32 = index;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_rank(
            inp_in,
            &mut out_out,
            width_in,
            height_in,
            index_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RankError,
        )
    }
}

/// VipsCountlines (countlines), count lines in an image
/// inp: `&VipsImage` -> Input image argument
/// direction: `Direction` -> Countlines left-right or up-down
///  `Horizontal` -> VIPS_DIRECTION_HORIZONTAL = 0 [DEFAULT]
///  `Vertical` -> VIPS_DIRECTION_VERTICAL = 1
///  `Last` -> VIPS_DIRECTION_LAST = 2
/// returns `f64` - Number of lines
pub fn countlines(inp: &VipsImage, direction: Direction) -> Result<f64> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let direction_in: i32 = direction as i32;
        let mut nolines_out: f64 = f64::from(0);

        let vips_op_response = bindings::vips_countlines(
            inp_in,
            &mut nolines_out,
            direction_in
                .try_into()
                .unwrap(),
            NULL,
        );
        utils::result(
            vips_op_response,
            nolines_out,
            Error::CountlineError,
        )
    }
}

/// VipsLabelregions (labelregions), label regions in an image
/// inp: `&VipsImage` -> Input image argument
/// returns `VipsImage` - Mask of region labels
pub fn labelregions(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut mask_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_labelregions(
            inp_in,
            &mut mask_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: mask_out,
            },
            Error::LabelregionError,
        )
    }
}

/// Options for labelregions operation
#[derive(Clone, Debug)]
pub struct LabelregionOptions {
    /// segments: `i32` -> Number of discrete contiguous regions
    /// min: 0, max: 1000000000, default: 0
    pub segments: i32,
}

impl std::default::Default for LabelregionOptions {
    fn default() -> Self {
        LabelregionOptions {
            segments: i32::from(0),
        }
    }
}

/// VipsLabelregions (labelregions), label regions in an image
/// inp: `&VipsImage` -> Input image argument
/// labelregions_options: `&LabelregionOptions` -> optional arguments
/// returns `VipsImage` - Mask of region labels
pub fn labelregions_with_opts(
    inp: &VipsImage,
    labelregions_options: &LabelregionOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut mask_out: *mut bindings::VipsImage = null_mut();

        let segments_in: i32 = labelregions_options.segments;
        let segments_in_name = utils::new_c_string("segments")?;

        let vips_op_response = bindings::vips_labelregions(
            inp_in,
            &mut mask_out,
            segments_in_name.as_ptr(),
            segments_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: mask_out,
            },
            Error::LabelregionError,
        )
    }
}

/// VipsFillNearest (fill_nearest), fill image zeros with nearest non-zero pixel
/// inp: `&VipsImage` -> Input image argument
/// returns `VipsImage` - Value of nearest non-zero pixel
pub fn fill_nearest(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_fill_nearest(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::FillNearestError,
        )
    }
}

/// Options for fill_nearest operation
#[derive(Clone, Debug)]
pub struct FillNearestOptions {
    /// distance: `VipsImage` -> Distance to nearest non-zero pixel
    pub distance: VipsImage,
}

impl std::default::Default for FillNearestOptions {
    fn default() -> Self {
        FillNearestOptions {
            distance: VipsImage::new(),
        }
    }
}

/// VipsFillNearest (fill_nearest), fill image zeros with nearest non-zero pixel
/// inp: `&VipsImage` -> Input image argument
/// fill_nearest_options: `&FillNearestOptions` -> optional arguments
/// returns `VipsImage` - Value of nearest non-zero pixel
pub fn fill_nearest_with_opts(
    inp: &VipsImage,
    fill_nearest_options: &FillNearestOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let distance_in: *mut bindings::VipsImage = fill_nearest_options
            .distance
            .ctx;
        let distance_in_name = utils::new_c_string("distance")?;

        let vips_op_response = bindings::vips_fill_nearest(
            inp_in,
            &mut out_out,
            distance_in_name.as_ptr(),
            distance_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::FillNearestError,
        )
    }
}

/// VipsDrawRect (draw_rect), paint a rectangle on an image
/// image: `&VipsImage` -> Image to draw on
/// ink: `&mut [f64]` -> Color for pixels
/// left: `i32` -> Rect to fill
/// min: -1000000000, max: 1000000000, default: 0
/// top: `i32` -> Rect to fill
/// min: -1000000000, max: 1000000000, default: 0
/// width: `i32` -> Rect to fill
/// min: -1000000000, max: 1000000000, default: 0
/// height: `i32` -> Rect to fill
/// min: -1000000000, max: 1000000000, default: 0

pub fn draw_rect(
    image: &VipsImage,
    ink: &mut [f64],
    left: i32,
    top: i32,
    width: i32,
    height: i32,
) -> Result<()> {
    unsafe {
        let image_in: *mut bindings::VipsImage = image.ctx;
        let ink_in: *mut f64 = ink.as_mut_ptr();
        let left_in: i32 = left;
        let top_in: i32 = top;
        let width_in: i32 = width;
        let height_in: i32 = height;

        let vips_op_response = bindings::vips_draw_rect(
            image_in,
            ink_in,
            ink.len() as i32,
            left_in,
            top_in,
            width_in,
            height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::DrawRectError,
        )
    }
}

/// Options for draw_rect operation
#[derive(Clone, Debug)]
pub struct DrawRectOptions {
    /// fill: `bool` -> Draw a solid object
    /// default: false
    pub fill: bool,
}

impl std::default::Default for DrawRectOptions {
    fn default() -> Self {
        DrawRectOptions {
            fill: false,
        }
    }
}

/// VipsDrawRect (draw_rect), paint a rectangle on an image
/// image: `&VipsImage` -> Image to draw on
/// ink: `&mut [f64]` -> Color for pixels
/// left: `i32` -> Rect to fill
/// min: -1000000000, max: 1000000000, default: 0
/// top: `i32` -> Rect to fill
/// min: -1000000000, max: 1000000000, default: 0
/// width: `i32` -> Rect to fill
/// min: -1000000000, max: 1000000000, default: 0
/// height: `i32` -> Rect to fill
/// min: -1000000000, max: 1000000000, default: 0
/// draw_rect_options: `&DrawRectOptions` -> optional arguments

pub fn draw_rect_with_opts(
    image: &VipsImage,
    ink: &mut [f64],
    left: i32,
    top: i32,
    width: i32,
    height: i32,
    draw_rect_options: &DrawRectOptions,
) -> Result<()> {
    unsafe {
        let image_in: *mut bindings::VipsImage = image.ctx;
        let ink_in: *mut f64 = ink.as_mut_ptr();
        let left_in: i32 = left;
        let top_in: i32 = top;
        let width_in: i32 = width;
        let height_in: i32 = height;

        let fill_in: i32 = if draw_rect_options.fill { 1 } else { 0 };
        let fill_in_name = utils::new_c_string("fill")?;

        let vips_op_response = bindings::vips_draw_rect(
            image_in,
            ink_in,
            ink.len() as i32,
            left_in,
            top_in,
            width_in,
            height_in,
            fill_in_name.as_ptr(),
            fill_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::DrawRectError,
        )
    }
}

/// VipsDrawMask (draw_mask), draw a mask on an image
/// image: `&VipsImage` -> Image to draw on
/// ink: `&mut [f64]` -> Color for pixels
/// mask: `&VipsImage` -> Mask of pixels to draw
/// x: `i32` -> Draw mask here
/// min: -1000000000, max: 1000000000, default: 0
/// y: `i32` -> Draw mask here
/// min: -1000000000, max: 1000000000, default: 0

pub fn draw_mask(
    image: &VipsImage,
    ink: &mut [f64],
    mask: &VipsImage,
    x: i32,
    y: i32,
) -> Result<()> {
    unsafe {
        let image_in: *mut bindings::VipsImage = image.ctx;
        let ink_in: *mut f64 = ink.as_mut_ptr();
        let mask_in: *mut bindings::VipsImage = mask.ctx;
        let x_in: i32 = x;
        let y_in: i32 = y;

        let vips_op_response = bindings::vips_draw_mask(
            image_in,
            ink_in,
            ink.len() as i32,
            mask_in,
            x_in,
            y_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::DrawMaskError,
        )
    }
}

/// VipsDrawLine (draw_line), draw a line on an image
/// image: `&VipsImage` -> Image to draw on
/// ink: `&mut [f64]` -> Color for pixels
/// x_1: `i32` -> Start of draw_line
/// min: -1000000000, max: 1000000000, default: 0
/// y_1: `i32` -> Start of draw_line
/// min: -1000000000, max: 1000000000, default: 0
/// x_2: `i32` -> End of draw_line
/// min: -1000000000, max: 1000000000, default: 0
/// y_2: `i32` -> End of draw_line
/// min: -1000000000, max: 1000000000, default: 0

pub fn draw_line(
    image: &VipsImage,
    ink: &mut [f64],
    x_1: i32,
    y_1: i32,
    x_2: i32,
    y_2: i32,
) -> Result<()> {
    unsafe {
        let image_in: *mut bindings::VipsImage = image.ctx;
        let ink_in: *mut f64 = ink.as_mut_ptr();
        let x_1_in: i32 = x_1;
        let y_1_in: i32 = y_1;
        let x_2_in: i32 = x_2;
        let y_2_in: i32 = y_2;

        let vips_op_response = bindings::vips_draw_line(
            image_in,
            ink_in,
            ink.len() as i32,
            x_1_in,
            y_1_in,
            x_2_in,
            y_2_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::DrawLineError,
        )
    }
}

/// VipsDrawCircle (draw_circle), draw a circle on an image
/// image: `&VipsImage` -> Image to draw on
/// ink: `&mut [f64]` -> Color for pixels
/// cx: `i32` -> Centre of draw_circle
/// min: -1000000000, max: 1000000000, default: 0
/// cy: `i32` -> Centre of draw_circle
/// min: -1000000000, max: 1000000000, default: 0
/// radius: `i32` -> Radius in pixels
/// min: 0, max: 1000000000, default: 0

pub fn draw_circle(
    image: &VipsImage,
    ink: &mut [f64],
    cx: i32,
    cy: i32,
    radius: i32,
) -> Result<()> {
    unsafe {
        let image_in: *mut bindings::VipsImage = image.ctx;
        let ink_in: *mut f64 = ink.as_mut_ptr();
        let cx_in: i32 = cx;
        let cy_in: i32 = cy;
        let radius_in: i32 = radius;

        let vips_op_response = bindings::vips_draw_circle(
            image_in,
            ink_in,
            ink.len() as i32,
            cx_in,
            cy_in,
            radius_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::DrawCircleError,
        )
    }
}

/// Options for draw_circle operation
#[derive(Clone, Debug)]
pub struct DrawCircleOptions {
    /// fill: `bool` -> Draw a solid object
    /// default: false
    pub fill: bool,
}

impl std::default::Default for DrawCircleOptions {
    fn default() -> Self {
        DrawCircleOptions {
            fill: false,
        }
    }
}

/// VipsDrawCircle (draw_circle), draw a circle on an image
/// image: `&VipsImage` -> Image to draw on
/// ink: `&mut [f64]` -> Color for pixels
/// cx: `i32` -> Centre of draw_circle
/// min: -1000000000, max: 1000000000, default: 0
/// cy: `i32` -> Centre of draw_circle
/// min: -1000000000, max: 1000000000, default: 0
/// radius: `i32` -> Radius in pixels
/// min: 0, max: 1000000000, default: 0
/// draw_circle_options: `&DrawCircleOptions` -> optional arguments

pub fn draw_circle_with_opts(
    image: &VipsImage,
    ink: &mut [f64],
    cx: i32,
    cy: i32,
    radius: i32,
    draw_circle_options: &DrawCircleOptions,
) -> Result<()> {
    unsafe {
        let image_in: *mut bindings::VipsImage = image.ctx;
        let ink_in: *mut f64 = ink.as_mut_ptr();
        let cx_in: i32 = cx;
        let cy_in: i32 = cy;
        let radius_in: i32 = radius;

        let fill_in: i32 = if draw_circle_options.fill { 1 } else { 0 };
        let fill_in_name = utils::new_c_string("fill")?;

        let vips_op_response = bindings::vips_draw_circle(
            image_in,
            ink_in,
            ink.len() as i32,
            cx_in,
            cy_in,
            radius_in,
            fill_in_name.as_ptr(),
            fill_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::DrawCircleError,
        )
    }
}

/// VipsDrawFlood (draw_flood), flood-fill an area
/// image: `&VipsImage` -> Image to draw on
/// ink: `&mut [f64]` -> Color for pixels
/// x: `i32` -> DrawFlood start point
/// min: 0, max: 1000000000, default: 0
/// y: `i32` -> DrawFlood start point
/// min: 0, max: 1000000000, default: 0

pub fn draw_flood(image: &VipsImage, ink: &mut [f64], x: i32, y: i32) -> Result<()> {
    unsafe {
        let image_in: *mut bindings::VipsImage = image.ctx;
        let ink_in: *mut f64 = ink.as_mut_ptr();
        let x_in: i32 = x;
        let y_in: i32 = y;

        let vips_op_response = bindings::vips_draw_flood(
            image_in,
            ink_in,
            ink.len() as i32,
            x_in,
            y_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::DrawFloodError,
        )
    }
}

/// Options for draw_flood operation
#[derive(Clone, Debug)]
pub struct DrawFloodOptions {
    /// test: `VipsImage` -> Test pixels in this image
    pub test: VipsImage,
    /// equal: `bool` -> DrawFlood while equal to edge
    /// default: false
    pub equal: bool,
    /// left: `i32` -> Left edge of modified area
    /// min: 0, max: 1000000000, default: 0
    pub left: i32,
    /// top: `i32` -> Top edge of modified area
    /// min: 0, max: 1000000000, default: 0
    pub top: i32,
    /// width: `i32` -> Width of modified area
    /// min: 0, max: 1000000000, default: 0
    pub width: i32,
    /// height: `i32` -> Height of modified area
    /// min: 0, max: 1000000000, default: 0
    pub height: i32,
}

impl std::default::Default for DrawFloodOptions {
    fn default() -> Self {
        DrawFloodOptions {
            test: VipsImage::new(),
            equal: false,
            left: i32::from(0),
            top: i32::from(0),
            width: i32::from(0),
            height: i32::from(0),
        }
    }
}

/// VipsDrawFlood (draw_flood), flood-fill an area
/// image: `&VipsImage` -> Image to draw on
/// ink: `&mut [f64]` -> Color for pixels
/// x: `i32` -> DrawFlood start point
/// min: 0, max: 1000000000, default: 0
/// y: `i32` -> DrawFlood start point
/// min: 0, max: 1000000000, default: 0
/// draw_flood_options: `&DrawFloodOptions` -> optional arguments

pub fn draw_flood_with_opts(
    image: &VipsImage,
    ink: &mut [f64],
    x: i32,
    y: i32,
    draw_flood_options: &DrawFloodOptions,
) -> Result<()> {
    unsafe {
        let image_in: *mut bindings::VipsImage = image.ctx;
        let ink_in: *mut f64 = ink.as_mut_ptr();
        let x_in: i32 = x;
        let y_in: i32 = y;

        let test_in: *mut bindings::VipsImage = draw_flood_options
            .test
            .ctx;
        let test_in_name = utils::new_c_string("test")?;

        let equal_in: i32 = if draw_flood_options.equal { 1 } else { 0 };
        let equal_in_name = utils::new_c_string("equal")?;

        let left_in: i32 = draw_flood_options.left;
        let left_in_name = utils::new_c_string("left")?;

        let top_in: i32 = draw_flood_options.top;
        let top_in_name = utils::new_c_string("top")?;

        let width_in: i32 = draw_flood_options.width;
        let width_in_name = utils::new_c_string("width")?;

        let height_in: i32 = draw_flood_options.height;
        let height_in_name = utils::new_c_string("height")?;

        let vips_op_response = bindings::vips_draw_flood(
            image_in,
            ink_in,
            ink.len() as i32,
            x_in,
            y_in,
            test_in_name.as_ptr(),
            test_in,
            equal_in_name.as_ptr(),
            equal_in,
            left_in_name.as_ptr(),
            left_in,
            top_in_name.as_ptr(),
            top_in,
            width_in_name.as_ptr(),
            width_in,
            height_in_name.as_ptr(),
            height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::DrawFloodError,
        )
    }
}

/// VipsDrawImage (draw_image), paint an image into another image
/// image: `&VipsImage` -> Image to draw on
/// sub: `&VipsImage` -> Sub-image to insert into main image
/// x: `i32` -> Draw image here
/// min: -1000000000, max: 1000000000, default: 0
/// y: `i32` -> Draw image here
/// min: -1000000000, max: 1000000000, default: 0

pub fn draw_image(image: &VipsImage, sub: &VipsImage, x: i32, y: i32) -> Result<()> {
    unsafe {
        let image_in: *mut bindings::VipsImage = image.ctx;
        let sub_in: *mut bindings::VipsImage = sub.ctx;
        let x_in: i32 = x;
        let y_in: i32 = y;

        let vips_op_response = bindings::vips_draw_image(
            image_in,
            sub_in,
            x_in,
            y_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::DrawImageError,
        )
    }
}

/// Options for draw_image operation
#[derive(Clone, Debug)]
pub struct DrawImageOptions {
    /// mode: `CombineMode` -> Combining mode
    ///  `Set` -> VIPS_COMBINE_MODE_SET = 0 [DEFAULT]
    ///  `Add` -> VIPS_COMBINE_MODE_ADD = 1
    ///  `Last` -> VIPS_COMBINE_MODE_LAST = 2
    pub mode: CombineMode,
}

impl std::default::Default for DrawImageOptions {
    fn default() -> Self {
        DrawImageOptions {
            mode: CombineMode::Set,
        }
    }
}

/// VipsDrawImage (draw_image), paint an image into another image
/// image: `&VipsImage` -> Image to draw on
/// sub: `&VipsImage` -> Sub-image to insert into main image
/// x: `i32` -> Draw image here
/// min: -1000000000, max: 1000000000, default: 0
/// y: `i32` -> Draw image here
/// min: -1000000000, max: 1000000000, default: 0
/// draw_image_options: `&DrawImageOptions` -> optional arguments

pub fn draw_image_with_opts(
    image: &VipsImage,
    sub: &VipsImage,
    x: i32,
    y: i32,
    draw_image_options: &DrawImageOptions,
) -> Result<()> {
    unsafe {
        let image_in: *mut bindings::VipsImage = image.ctx;
        let sub_in: *mut bindings::VipsImage = sub.ctx;
        let x_in: i32 = x;
        let y_in: i32 = y;

        let mode_in: i32 = draw_image_options.mode as i32;
        let mode_in_name = utils::new_c_string("mode")?;

        let vips_op_response = bindings::vips_draw_image(
            image_in,
            sub_in,
            x_in,
            y_in,
            mode_in_name.as_ptr(),
            mode_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::DrawImageError,
        )
    }
}

/// VipsDrawSmudge (draw_smudge), blur a rectangle on an image
/// image: `&VipsImage` -> Image to draw on
/// left: `i32` -> Rect to fill
/// min: -1000000000, max: 1000000000, default: 0
/// top: `i32` -> Rect to fill
/// min: -1000000000, max: 1000000000, default: 0
/// width: `i32` -> Rect to fill
/// min: -1000000000, max: 1000000000, default: 0
/// height: `i32` -> Rect to fill
/// min: -1000000000, max: 1000000000, default: 0

pub fn draw_smudge(image: &VipsImage, left: i32, top: i32, width: i32, height: i32) -> Result<()> {
    unsafe {
        let image_in: *mut bindings::VipsImage = image.ctx;
        let left_in: i32 = left;
        let top_in: i32 = top;
        let width_in: i32 = width;
        let height_in: i32 = height;

        let vips_op_response = bindings::vips_draw_smudge(
            image_in,
            left_in,
            top_in,
            width_in,
            height_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            (),
            Error::DrawSmudgeError,
        )
    }
}

/// VipsMerge (merge), merge two images
/// refp: `&VipsImage` -> Reference image
/// sec: `&VipsImage` -> Secondary image
/// direction: `Direction` -> Horizontal or vertical merge
///  `Horizontal` -> VIPS_DIRECTION_HORIZONTAL = 0 [DEFAULT]
///  `Vertical` -> VIPS_DIRECTION_VERTICAL = 1
///  `Last` -> VIPS_DIRECTION_LAST = 2
/// dx: `i32` -> Horizontal displacement from sec to ref
/// min: -100000000, max: 1000000000, default: 1
/// dy: `i32` -> Vertical displacement from sec to ref
/// min: -100000000, max: 1000000000, default: 1
/// returns `VipsImage` - Output image
pub fn merge(
    refp: &VipsImage,
    sec: &VipsImage,
    direction: Direction,
    dx: i32,
    dy: i32,
) -> Result<VipsImage> {
    unsafe {
        let refp_in: *mut bindings::VipsImage = refp.ctx;
        let sec_in: *mut bindings::VipsImage = sec.ctx;
        let direction_in: i32 = direction as i32;
        let dx_in: i32 = dx;
        let dy_in: i32 = dy;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_merge(
            refp_in,
            sec_in,
            &mut out_out,
            direction_in
                .try_into()
                .unwrap(),
            dx_in,
            dy_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MergeError,
        )
    }
}

/// Options for merge operation
#[derive(Clone, Debug)]
pub struct MergeOptions {
    /// mblend: `i32` -> Maximum blend size
    /// min: 0, max: 10000, default: 10
    pub mblend: i32,
}

impl std::default::Default for MergeOptions {
    fn default() -> Self {
        MergeOptions {
            mblend: i32::from(10),
        }
    }
}

/// VipsMerge (merge), merge two images
/// refp: `&VipsImage` -> Reference image
/// sec: `&VipsImage` -> Secondary image
/// direction: `Direction` -> Horizontal or vertical merge
///  `Horizontal` -> VIPS_DIRECTION_HORIZONTAL = 0 [DEFAULT]
///  `Vertical` -> VIPS_DIRECTION_VERTICAL = 1
///  `Last` -> VIPS_DIRECTION_LAST = 2
/// dx: `i32` -> Horizontal displacement from sec to ref
/// min: -100000000, max: 1000000000, default: 1
/// dy: `i32` -> Vertical displacement from sec to ref
/// min: -100000000, max: 1000000000, default: 1
/// merge_options: `&MergeOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn merge_with_opts(
    refp: &VipsImage,
    sec: &VipsImage,
    direction: Direction,
    dx: i32,
    dy: i32,
    merge_options: &MergeOptions,
) -> Result<VipsImage> {
    unsafe {
        let refp_in: *mut bindings::VipsImage = refp.ctx;
        let sec_in: *mut bindings::VipsImage = sec.ctx;
        let direction_in: i32 = direction as i32;
        let dx_in: i32 = dx;
        let dy_in: i32 = dy;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let mblend_in: i32 = merge_options.mblend;
        let mblend_in_name = utils::new_c_string("mblend")?;

        let vips_op_response = bindings::vips_merge(
            refp_in,
            sec_in,
            &mut out_out,
            direction_in
                .try_into()
                .unwrap(),
            dx_in,
            dy_in,
            mblend_in_name.as_ptr(),
            mblend_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MergeError,
        )
    }
}

/// VipsMosaic (mosaic), mosaic two images
/// refp: `&VipsImage` -> Reference image
/// sec: `&VipsImage` -> Secondary image
/// direction: `Direction` -> Horizontal or vertical mosaic
///  `Horizontal` -> VIPS_DIRECTION_HORIZONTAL = 0 [DEFAULT]
///  `Vertical` -> VIPS_DIRECTION_VERTICAL = 1
///  `Last` -> VIPS_DIRECTION_LAST = 2
/// xref: `i32` -> Position of reference tie-point
/// min: 0, max: 1000000000, default: 1
/// yref: `i32` -> Position of reference tie-point
/// min: 0, max: 1000000000, default: 1
/// xsec: `i32` -> Position of secondary tie-point
/// min: 0, max: 1000000000, default: 1
/// ysec: `i32` -> Position of secondary tie-point
/// min: 0, max: 1000000000, default: 1
/// returns `VipsImage` - Output image
pub fn mosaic(
    refp: &VipsImage,
    sec: &VipsImage,
    direction: Direction,
    xref: i32,
    yref: i32,
    xsec: i32,
    ysec: i32,
) -> Result<VipsImage> {
    unsafe {
        let refp_in: *mut bindings::VipsImage = refp.ctx;
        let sec_in: *mut bindings::VipsImage = sec.ctx;
        let direction_in: i32 = direction as i32;
        let xref_in: i32 = xref;
        let yref_in: i32 = yref;
        let xsec_in: i32 = xsec;
        let ysec_in: i32 = ysec;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_mosaic(
            refp_in,
            sec_in,
            &mut out_out,
            direction_in
                .try_into()
                .unwrap(),
            xref_in,
            yref_in,
            xsec_in,
            ysec_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MosaicError,
        )
    }
}

/// Options for mosaic operation
#[derive(Clone, Debug)]
pub struct MosaicOptions {
    /// hwindow: `i32` -> Half window size
    /// min: 0, max: 1000000000, default: 5
    pub hwindow: i32,
    /// harea: `i32` -> Half area size
    /// min: 0, max: 1000000000, default: 15
    pub harea: i32,
    /// mblend: `i32` -> Maximum blend size
    /// min: 0, max: 10000, default: 10
    pub mblend: i32,
    /// bandno: `i32` -> Band to search for features on
    /// min: 0, max: 10000, default: 0
    pub bandno: i32,
    /// dx_0: `i32` -> Detected integer offset
    /// min: -10000000, max: 10000000, default: 0
    pub dx_0: i32,
    /// dy_0: `i32` -> Detected integer offset
    /// min: -10000000, max: 10000000, default: 0
    pub dy_0: i32,
    /// scale_1: `f64` -> Detected scale
    /// min: -10000000, max: 10000000, default: 1
    pub scale_1: f64,
    /// angle_1: `f64` -> Detected rotation
    /// min: -10000000, max: 10000000, default: 0
    pub angle_1: f64,
    /// dy_1: `f64` -> Detected first-order displacement
    /// min: -10000000, max: 10000000, default: 0
    pub dy_1: f64,
    /// dx_1: `f64` -> Detected first-order displacement
    /// min: -10000000, max: 10000000, default: 0
    pub dx_1: f64,
}

impl std::default::Default for MosaicOptions {
    fn default() -> Self {
        MosaicOptions {
            hwindow: i32::from(5),
            harea: i32::from(15),
            mblend: i32::from(10),
            bandno: i32::from(0),
            dx_0: i32::from(0),
            dy_0: i32::from(0),
            scale_1: f64::from(1),
            angle_1: f64::from(0),
            dy_1: f64::from(0),
            dx_1: f64::from(0),
        }
    }
}

/// VipsMosaic (mosaic), mosaic two images
/// refp: `&VipsImage` -> Reference image
/// sec: `&VipsImage` -> Secondary image
/// direction: `Direction` -> Horizontal or vertical mosaic
///  `Horizontal` -> VIPS_DIRECTION_HORIZONTAL = 0 [DEFAULT]
///  `Vertical` -> VIPS_DIRECTION_VERTICAL = 1
///  `Last` -> VIPS_DIRECTION_LAST = 2
/// xref: `i32` -> Position of reference tie-point
/// min: 0, max: 1000000000, default: 1
/// yref: `i32` -> Position of reference tie-point
/// min: 0, max: 1000000000, default: 1
/// xsec: `i32` -> Position of secondary tie-point
/// min: 0, max: 1000000000, default: 1
/// ysec: `i32` -> Position of secondary tie-point
/// min: 0, max: 1000000000, default: 1
/// mosaic_options: `&MosaicOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn mosaic_with_opts(
    refp: &VipsImage,
    sec: &VipsImage,
    direction: Direction,
    xref: i32,
    yref: i32,
    xsec: i32,
    ysec: i32,
    mosaic_options: &MosaicOptions,
) -> Result<VipsImage> {
    unsafe {
        let refp_in: *mut bindings::VipsImage = refp.ctx;
        let sec_in: *mut bindings::VipsImage = sec.ctx;
        let direction_in: i32 = direction as i32;
        let xref_in: i32 = xref;
        let yref_in: i32 = yref;
        let xsec_in: i32 = xsec;
        let ysec_in: i32 = ysec;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let hwindow_in: i32 = mosaic_options.hwindow;
        let hwindow_in_name = utils::new_c_string("hwindow")?;

        let harea_in: i32 = mosaic_options.harea;
        let harea_in_name = utils::new_c_string("harea")?;

        let mblend_in: i32 = mosaic_options.mblend;
        let mblend_in_name = utils::new_c_string("mblend")?;

        let bandno_in: i32 = mosaic_options.bandno;
        let bandno_in_name = utils::new_c_string("bandno")?;

        let dx_0_in: i32 = mosaic_options.dx_0;
        let dx_0_in_name = utils::new_c_string("dx0")?;

        let dy_0_in: i32 = mosaic_options.dy_0;
        let dy_0_in_name = utils::new_c_string("dy0")?;

        let scale_1_in: f64 = mosaic_options.scale_1;
        let scale_1_in_name = utils::new_c_string("scale1")?;

        let angle_1_in: f64 = mosaic_options.angle_1;
        let angle_1_in_name = utils::new_c_string("angle1")?;

        let dy_1_in: f64 = mosaic_options.dy_1;
        let dy_1_in_name = utils::new_c_string("dy1")?;

        let dx_1_in: f64 = mosaic_options.dx_1;
        let dx_1_in_name = utils::new_c_string("dx1")?;

        let vips_op_response = bindings::vips_mosaic(
            refp_in,
            sec_in,
            &mut out_out,
            direction_in
                .try_into()
                .unwrap(),
            xref_in,
            yref_in,
            xsec_in,
            ysec_in,
            hwindow_in_name.as_ptr(),
            hwindow_in,
            harea_in_name.as_ptr(),
            harea_in,
            mblend_in_name.as_ptr(),
            mblend_in,
            bandno_in_name.as_ptr(),
            bandno_in,
            dx_0_in_name.as_ptr(),
            dx_0_in,
            dy_0_in_name.as_ptr(),
            dy_0_in,
            scale_1_in_name.as_ptr(),
            scale_1_in,
            angle_1_in_name.as_ptr(),
            angle_1_in,
            dy_1_in_name.as_ptr(),
            dy_1_in,
            dx_1_in_name.as_ptr(),
            dx_1_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MosaicError,
        )
    }
}

/// VipsMosaic1 (mosaic1), first-order mosaic of two images
/// refp: `&VipsImage` -> Reference image
/// sec: `&VipsImage` -> Secondary image
/// direction: `Direction` -> Horizontal or vertical mosaic
///  `Horizontal` -> VIPS_DIRECTION_HORIZONTAL = 0 [DEFAULT]
///  `Vertical` -> VIPS_DIRECTION_VERTICAL = 1
///  `Last` -> VIPS_DIRECTION_LAST = 2
/// xr_1: `i32` -> Position of first reference tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// yr_1: `i32` -> Position of first reference tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// xs_1: `i32` -> Position of first secondary tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// ys_1: `i32` -> Position of first secondary tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// xr_2: `i32` -> Position of second reference tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// yr_2: `i32` -> Position of second reference tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// xs_2: `i32` -> Position of second secondary tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// ys_2: `i32` -> Position of second secondary tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// returns `VipsImage` - Output image
pub fn mosaic_1(
    refp: &VipsImage,
    sec: &VipsImage,
    direction: Direction,
    xr_1: i32,
    yr_1: i32,
    xs_1: i32,
    ys_1: i32,
    xr_2: i32,
    yr_2: i32,
    xs_2: i32,
    ys_2: i32,
) -> Result<VipsImage> {
    unsafe {
        let refp_in: *mut bindings::VipsImage = refp.ctx;
        let sec_in: *mut bindings::VipsImage = sec.ctx;
        let direction_in: i32 = direction as i32;
        let xr_1_in: i32 = xr_1;
        let yr_1_in: i32 = yr_1;
        let xs_1_in: i32 = xs_1;
        let ys_1_in: i32 = ys_1;
        let xr_2_in: i32 = xr_2;
        let yr_2_in: i32 = yr_2;
        let xs_2_in: i32 = xs_2;
        let ys_2_in: i32 = ys_2;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_mosaic1(
            refp_in,
            sec_in,
            &mut out_out,
            direction_in
                .try_into()
                .unwrap(),
            xr_1_in,
            yr_1_in,
            xs_1_in,
            ys_1_in,
            xr_2_in,
            yr_2_in,
            xs_2_in,
            ys_2_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Mosaic1Error,
        )
    }
}

/// Options for mosaic_1 operation
#[derive(Clone, Debug)]
pub struct Mosaic1Options {
    /// hwindow: `i32` -> Half window size
    /// min: 0, max: 1000000000, default: 5
    pub hwindow: i32,
    /// harea: `i32` -> Half area size
    /// min: 0, max: 1000000000, default: 15
    pub harea: i32,
    /// search: `bool` -> Search to improve tie-points
    /// default: false
    pub search: bool,
    /// interpolate: `VipsInterpolate` -> Interpolate pixels with this
    pub interpolate: VipsInterpolate,
    /// mblend: `i32` -> Maximum blend size
    /// min: 0, max: 10000, default: 10
    pub mblend: i32,
}

impl std::default::Default for Mosaic1Options {
    fn default() -> Self {
        Mosaic1Options {
            hwindow: i32::from(5),
            harea: i32::from(15),
            search: false,
            interpolate: VipsInterpolate::new(),
            mblend: i32::from(10),
        }
    }
}

/// VipsMosaic1 (mosaic1), first-order mosaic of two images
/// refp: `&VipsImage` -> Reference image
/// sec: `&VipsImage` -> Secondary image
/// direction: `Direction` -> Horizontal or vertical mosaic
///  `Horizontal` -> VIPS_DIRECTION_HORIZONTAL = 0 [DEFAULT]
///  `Vertical` -> VIPS_DIRECTION_VERTICAL = 1
///  `Last` -> VIPS_DIRECTION_LAST = 2
/// xr_1: `i32` -> Position of first reference tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// yr_1: `i32` -> Position of first reference tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// xs_1: `i32` -> Position of first secondary tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// ys_1: `i32` -> Position of first secondary tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// xr_2: `i32` -> Position of second reference tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// yr_2: `i32` -> Position of second reference tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// xs_2: `i32` -> Position of second secondary tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// ys_2: `i32` -> Position of second secondary tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// mosaic_1_options: `&Mosaic1Options` -> optional arguments
/// returns `VipsImage` - Output image
pub fn mosaic_1_with_opts(
    refp: &VipsImage,
    sec: &VipsImage,
    direction: Direction,
    xr_1: i32,
    yr_1: i32,
    xs_1: i32,
    ys_1: i32,
    xr_2: i32,
    yr_2: i32,
    xs_2: i32,
    ys_2: i32,
    mosaic_1_options: &Mosaic1Options,
) -> Result<VipsImage> {
    unsafe {
        let refp_in: *mut bindings::VipsImage = refp.ctx;
        let sec_in: *mut bindings::VipsImage = sec.ctx;
        let direction_in: i32 = direction as i32;
        let xr_1_in: i32 = xr_1;
        let yr_1_in: i32 = yr_1;
        let xs_1_in: i32 = xs_1;
        let ys_1_in: i32 = ys_1;
        let xr_2_in: i32 = xr_2;
        let yr_2_in: i32 = yr_2;
        let xs_2_in: i32 = xs_2;
        let ys_2_in: i32 = ys_2;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let hwindow_in: i32 = mosaic_1_options.hwindow;
        let hwindow_in_name = utils::new_c_string("hwindow")?;

        let harea_in: i32 = mosaic_1_options.harea;
        let harea_in_name = utils::new_c_string("harea")?;

        let search_in: i32 = if mosaic_1_options.search { 1 } else { 0 };
        let search_in_name = utils::new_c_string("search")?;

        let interpolate_in: *mut bindings::VipsInterpolate = mosaic_1_options
            .interpolate
            .ctx;
        let interpolate_in_name = utils::new_c_string("interpolate")?;

        let mblend_in: i32 = mosaic_1_options.mblend;
        let mblend_in_name = utils::new_c_string("mblend")?;

        let vips_op_response = bindings::vips_mosaic1(
            refp_in,
            sec_in,
            &mut out_out,
            direction_in
                .try_into()
                .unwrap(),
            xr_1_in,
            yr_1_in,
            xs_1_in,
            ys_1_in,
            xr_2_in,
            yr_2_in,
            xs_2_in,
            ys_2_in,
            hwindow_in_name.as_ptr(),
            hwindow_in,
            harea_in_name.as_ptr(),
            harea_in,
            search_in_name.as_ptr(),
            search_in,
            interpolate_in_name.as_ptr(),
            interpolate_in,
            mblend_in_name.as_ptr(),
            mblend_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Mosaic1Error,
        )
    }
}

/// VipsMatrixinvert (matrixinvert), invert an matrix
/// inp: `&VipsImage` -> An square matrix
/// returns `VipsImage` - Output matrix
pub fn matrixinvert(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_matrixinvert(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MatrixinvertError,
        )
    }
}

/// VipsMatch (match), first-order match of two images
/// refp: `&VipsImage` -> Reference image
/// sec: `&VipsImage` -> Secondary image
/// xr_1: `i32` -> Position of first reference tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// yr_1: `i32` -> Position of first reference tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// xs_1: `i32` -> Position of first secondary tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// ys_1: `i32` -> Position of first secondary tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// xr_2: `i32` -> Position of second reference tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// yr_2: `i32` -> Position of second reference tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// xs_2: `i32` -> Position of second secondary tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// ys_2: `i32` -> Position of second secondary tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// returns `VipsImage` - Output image
pub fn matches(
    refp: &VipsImage,
    sec: &VipsImage,
    xr_1: i32,
    yr_1: i32,
    xs_1: i32,
    ys_1: i32,
    xr_2: i32,
    yr_2: i32,
    xs_2: i32,
    ys_2: i32,
) -> Result<VipsImage> {
    unsafe {
        let refp_in: *mut bindings::VipsImage = refp.ctx;
        let sec_in: *mut bindings::VipsImage = sec.ctx;
        let xr_1_in: i32 = xr_1;
        let yr_1_in: i32 = yr_1;
        let xs_1_in: i32 = xs_1;
        let ys_1_in: i32 = ys_1;
        let xr_2_in: i32 = xr_2;
        let yr_2_in: i32 = yr_2;
        let xs_2_in: i32 = xs_2;
        let ys_2_in: i32 = ys_2;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_match(
            refp_in,
            sec_in,
            &mut out_out,
            xr_1_in,
            yr_1_in,
            xs_1_in,
            ys_1_in,
            xr_2_in,
            yr_2_in,
            xs_2_in,
            ys_2_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MatchError,
        )
    }
}

/// Options for matches operation
#[derive(Clone, Debug)]
pub struct MatchOptions {
    /// hwindow: `i32` -> Half window size
    /// min: 0, max: 1000000000, default: 1
    pub hwindow: i32,
    /// harea: `i32` -> Half area size
    /// min: 0, max: 1000000000, default: 1
    pub harea: i32,
    /// search: `bool` -> Search to improve tie-points
    /// default: false
    pub search: bool,
    /// interpolate: `VipsInterpolate` -> Interpolate pixels with this
    pub interpolate: VipsInterpolate,
}

impl std::default::Default for MatchOptions {
    fn default() -> Self {
        MatchOptions {
            hwindow: i32::from(1),
            harea: i32::from(1),
            search: false,
            interpolate: VipsInterpolate::new(),
        }
    }
}

/// VipsMatch (match), first-order match of two images
/// refp: `&VipsImage` -> Reference image
/// sec: `&VipsImage` -> Secondary image
/// xr_1: `i32` -> Position of first reference tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// yr_1: `i32` -> Position of first reference tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// xs_1: `i32` -> Position of first secondary tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// ys_1: `i32` -> Position of first secondary tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// xr_2: `i32` -> Position of second reference tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// yr_2: `i32` -> Position of second reference tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// xs_2: `i32` -> Position of second secondary tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// ys_2: `i32` -> Position of second secondary tie-point
/// min: -1000000000, max: 1000000000, default: 1
/// matches_options: `&MatchOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn matches_with_opts(
    refp: &VipsImage,
    sec: &VipsImage,
    xr_1: i32,
    yr_1: i32,
    xs_1: i32,
    ys_1: i32,
    xr_2: i32,
    yr_2: i32,
    xs_2: i32,
    ys_2: i32,
    matches_options: &MatchOptions,
) -> Result<VipsImage> {
    unsafe {
        let refp_in: *mut bindings::VipsImage = refp.ctx;
        let sec_in: *mut bindings::VipsImage = sec.ctx;
        let xr_1_in: i32 = xr_1;
        let yr_1_in: i32 = yr_1;
        let xs_1_in: i32 = xs_1;
        let ys_1_in: i32 = ys_1;
        let xr_2_in: i32 = xr_2;
        let yr_2_in: i32 = yr_2;
        let xs_2_in: i32 = xs_2;
        let ys_2_in: i32 = ys_2;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let hwindow_in: i32 = matches_options.hwindow;
        let hwindow_in_name = utils::new_c_string("hwindow")?;

        let harea_in: i32 = matches_options.harea;
        let harea_in_name = utils::new_c_string("harea")?;

        let search_in: i32 = if matches_options.search { 1 } else { 0 };
        let search_in_name = utils::new_c_string("search")?;

        let interpolate_in: *mut bindings::VipsInterpolate = matches_options
            .interpolate
            .ctx;
        let interpolate_in_name = utils::new_c_string("interpolate")?;

        let vips_op_response = bindings::vips_match(
            refp_in,
            sec_in,
            &mut out_out,
            xr_1_in,
            yr_1_in,
            xs_1_in,
            ys_1_in,
            xr_2_in,
            yr_2_in,
            xs_2_in,
            ys_2_in,
            hwindow_in_name.as_ptr(),
            hwindow_in,
            harea_in_name.as_ptr(),
            harea_in,
            search_in_name.as_ptr(),
            search_in,
            interpolate_in_name.as_ptr(),
            interpolate_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MatchError,
        )
    }
}

/// VipsGlobalbalance (globalbalance), global balance an image mosaic
/// inp: `&VipsImage` -> Input image
/// returns `VipsImage` - Output image
pub fn globalbalance(inp: &VipsImage) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = bindings::vips_globalbalance(
            inp_in,
            &mut out_out,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GlobalbalanceError,
        )
    }
}

/// Options for globalbalance operation
#[derive(Clone, Debug)]
pub struct GlobalbalanceOptions {
    /// gamma: `f64` -> Image gamma
    /// min: 0.00001, max: 10, default: 1.6
    pub gamma: f64,
    /// int_output: `bool` -> Integer output
    /// default: false
    pub int_output: bool,
}

impl std::default::Default for GlobalbalanceOptions {
    fn default() -> Self {
        GlobalbalanceOptions {
            gamma: f64::from(1.6),
            int_output: false,
        }
    }
}

/// VipsGlobalbalance (globalbalance), global balance an image mosaic
/// inp: `&VipsImage` -> Input image
/// globalbalance_options: `&GlobalbalanceOptions` -> optional arguments
/// returns `VipsImage` - Output image
pub fn globalbalance_with_opts(
    inp: &VipsImage,
    globalbalance_options: &GlobalbalanceOptions,
) -> Result<VipsImage> {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let gamma_in: f64 = globalbalance_options.gamma;
        let gamma_in_name = utils::new_c_string("gamma")?;

        let int_output_in: i32 = if globalbalance_options.int_output { 1 } else { 0 };
        let int_output_in_name = utils::new_c_string("int-output")?;

        let vips_op_response = bindings::vips_globalbalance(
            inp_in,
            &mut out_out,
            gamma_in_name.as_ptr(),
            gamma_in,
            int_output_in_name.as_ptr(),
            int_output_in,
            NULL,
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GlobalbalanceError,
        )
    }
}
