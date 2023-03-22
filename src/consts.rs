// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/consts.rs`)

#[allow(unused_imports)]
use crate::{Alpha2, Region, SubRegion, WorldRegion};
use hashbrown::HashMap;
use lazy_static::lazy_static;
pub const ALL_COUNTRIES_COUNT: usize = 249;
lazy_static! {
    pub static ref SUPPORTED_COUNTRIES_COUNT: usize = SUPPORTED_ALPHA2_LIST.len();
}
lazy_static! {
    pub static ref UNSUPPORTED_COUNTRIES_COUNT: usize =
        ALL_COUNTRIES_COUNT - *SUPPORTED_COUNTRIES_COUNT;
}
#[cfg(all(
    feature = "ad",
    feature = "ae",
    feature = "af",
    feature = "ag",
    feature = "ai",
    feature = "al",
    feature = "am",
    feature = "ao",
    feature = "aq",
    feature = "ar",
    feature = "as",
    feature = "at",
    feature = "au",
    feature = "aw",
    feature = "ax",
    feature = "az",
    feature = "ba",
    feature = "bb",
    feature = "bd",
    feature = "be",
    feature = "bf",
    feature = "bg",
    feature = "bh",
    feature = "bi",
    feature = "bj",
    feature = "bl",
    feature = "bm",
    feature = "bn",
    feature = "bo",
    feature = "bq",
    feature = "br",
    feature = "bs",
    feature = "bt",
    feature = "bv",
    feature = "bw",
    feature = "by",
    feature = "bz",
    feature = "ca",
    feature = "cc",
    feature = "cd",
    feature = "cf",
    feature = "cg",
    feature = "ch",
    feature = "ci",
    feature = "ck",
    feature = "cl",
    feature = "cm",
    feature = "cn",
    feature = "co",
    feature = "cr",
    feature = "cu",
    feature = "cv",
    feature = "cw",
    feature = "cx",
    feature = "cy",
    feature = "cz",
    feature = "de",
    feature = "dj",
    feature = "dk",
    feature = "dm",
    feature = "do",
    feature = "dz",
    feature = "ec",
    feature = "ee",
    feature = "eg",
    feature = "eh",
    feature = "er",
    feature = "es",
    feature = "et",
    feature = "fi",
    feature = "fj",
    feature = "fk",
    feature = "fm",
    feature = "fo",
    feature = "fr",
    feature = "ga",
    feature = "gb",
    feature = "gd",
    feature = "ge",
    feature = "gf",
    feature = "gg",
    feature = "gh",
    feature = "gi",
    feature = "gl",
    feature = "gm",
    feature = "gn",
    feature = "gp",
    feature = "gq",
    feature = "gr",
    feature = "gs",
    feature = "gt",
    feature = "gu",
    feature = "gw",
    feature = "gy",
    feature = "hk",
    feature = "hm",
    feature = "hn",
    feature = "hr",
    feature = "ht",
    feature = "hu",
    feature = "id",
    feature = "ie",
    feature = "il",
    feature = "im",
    feature = "in",
    feature = "io",
    feature = "iq",
    feature = "ir",
    feature = "is",
    feature = "it",
    feature = "je",
    feature = "jm",
    feature = "jo",
    feature = "jp",
    feature = "ke",
    feature = "kg",
    feature = "kh",
    feature = "ki",
    feature = "km",
    feature = "kn",
    feature = "kp",
    feature = "kr",
    feature = "kw",
    feature = "ky",
    feature = "kz",
    feature = "la",
    feature = "lb",
    feature = "lc",
    feature = "li",
    feature = "lk",
    feature = "lr",
    feature = "ls",
    feature = "lt",
    feature = "lu",
    feature = "lv",
    feature = "ly",
    feature = "ma",
    feature = "mc",
    feature = "md",
    feature = "me",
    feature = "mf",
    feature = "mg",
    feature = "mh",
    feature = "mk",
    feature = "ml",
    feature = "mm",
    feature = "mn",
    feature = "mo",
    feature = "mp",
    feature = "mq",
    feature = "mr",
    feature = "ms",
    feature = "mt",
    feature = "mu",
    feature = "mv",
    feature = "mw",
    feature = "mx",
    feature = "my",
    feature = "mz",
    feature = "na",
    feature = "nc",
    feature = "ne",
    feature = "nf",
    feature = "ng",
    feature = "ni",
    feature = "nl",
    feature = "no",
    feature = "np",
    feature = "nr",
    feature = "nu",
    feature = "nz",
    feature = "om",
    feature = "pa",
    feature = "pe",
    feature = "pf",
    feature = "pg",
    feature = "ph",
    feature = "pk",
    feature = "pl",
    feature = "pm",
    feature = "pn",
    feature = "pr",
    feature = "ps",
    feature = "pt",
    feature = "pw",
    feature = "py",
    feature = "qa",
    feature = "re",
    feature = "ro",
    feature = "rs",
    feature = "ru",
    feature = "rw",
    feature = "sa",
    feature = "sb",
    feature = "sc",
    feature = "sd",
    feature = "se",
    feature = "sg",
    feature = "sh",
    feature = "si",
    feature = "sj",
    feature = "sk",
    feature = "sl",
    feature = "sm",
    feature = "sn",
    feature = "so",
    feature = "sr",
    feature = "ss",
    feature = "st",
    feature = "sv",
    feature = "sx",
    feature = "sy",
    feature = "sz",
    feature = "tc",
    feature = "td",
    feature = "tf",
    feature = "tg",
    feature = "th",
    feature = "tj",
    feature = "tk",
    feature = "tl",
    feature = "tm",
    feature = "tn",
    feature = "to",
    feature = "tr",
    feature = "tt",
    feature = "tv",
    feature = "tw",
    feature = "tz",
    feature = "ua",
    feature = "ug",
    feature = "um",
    feature = "us",
    feature = "uy",
    feature = "uz",
    feature = "va",
    feature = "vc",
    feature = "ve",
    feature = "vg",
    feature = "vi",
    feature = "vn",
    feature = "vu",
    feature = "wf",
    feature = "ws",
    feature = "ye",
    feature = "yt",
    feature = "za",
    feature = "zm",
    feature = "zw",
))]
pub const SUPPORT_ALL_COUNTRIES: bool = true;
#[cfg(not(all(
    feature = "ad",
    feature = "ae",
    feature = "af",
    feature = "ag",
    feature = "ai",
    feature = "al",
    feature = "am",
    feature = "ao",
    feature = "aq",
    feature = "ar",
    feature = "as",
    feature = "at",
    feature = "au",
    feature = "aw",
    feature = "ax",
    feature = "az",
    feature = "ba",
    feature = "bb",
    feature = "bd",
    feature = "be",
    feature = "bf",
    feature = "bg",
    feature = "bh",
    feature = "bi",
    feature = "bj",
    feature = "bl",
    feature = "bm",
    feature = "bn",
    feature = "bo",
    feature = "bq",
    feature = "br",
    feature = "bs",
    feature = "bt",
    feature = "bv",
    feature = "bw",
    feature = "by",
    feature = "bz",
    feature = "ca",
    feature = "cc",
    feature = "cd",
    feature = "cf",
    feature = "cg",
    feature = "ch",
    feature = "ci",
    feature = "ck",
    feature = "cl",
    feature = "cm",
    feature = "cn",
    feature = "co",
    feature = "cr",
    feature = "cu",
    feature = "cv",
    feature = "cw",
    feature = "cx",
    feature = "cy",
    feature = "cz",
    feature = "de",
    feature = "dj",
    feature = "dk",
    feature = "dm",
    feature = "do",
    feature = "dz",
    feature = "ec",
    feature = "ee",
    feature = "eg",
    feature = "eh",
    feature = "er",
    feature = "es",
    feature = "et",
    feature = "fi",
    feature = "fj",
    feature = "fk",
    feature = "fm",
    feature = "fo",
    feature = "fr",
    feature = "ga",
    feature = "gb",
    feature = "gd",
    feature = "ge",
    feature = "gf",
    feature = "gg",
    feature = "gh",
    feature = "gi",
    feature = "gl",
    feature = "gm",
    feature = "gn",
    feature = "gp",
    feature = "gq",
    feature = "gr",
    feature = "gs",
    feature = "gt",
    feature = "gu",
    feature = "gw",
    feature = "gy",
    feature = "hk",
    feature = "hm",
    feature = "hn",
    feature = "hr",
    feature = "ht",
    feature = "hu",
    feature = "id",
    feature = "ie",
    feature = "il",
    feature = "im",
    feature = "in",
    feature = "io",
    feature = "iq",
    feature = "ir",
    feature = "is",
    feature = "it",
    feature = "je",
    feature = "jm",
    feature = "jo",
    feature = "jp",
    feature = "ke",
    feature = "kg",
    feature = "kh",
    feature = "ki",
    feature = "km",
    feature = "kn",
    feature = "kp",
    feature = "kr",
    feature = "kw",
    feature = "ky",
    feature = "kz",
    feature = "la",
    feature = "lb",
    feature = "lc",
    feature = "li",
    feature = "lk",
    feature = "lr",
    feature = "ls",
    feature = "lt",
    feature = "lu",
    feature = "lv",
    feature = "ly",
    feature = "ma",
    feature = "mc",
    feature = "md",
    feature = "me",
    feature = "mf",
    feature = "mg",
    feature = "mh",
    feature = "mk",
    feature = "ml",
    feature = "mm",
    feature = "mn",
    feature = "mo",
    feature = "mp",
    feature = "mq",
    feature = "mr",
    feature = "ms",
    feature = "mt",
    feature = "mu",
    feature = "mv",
    feature = "mw",
    feature = "mx",
    feature = "my",
    feature = "mz",
    feature = "na",
    feature = "nc",
    feature = "ne",
    feature = "nf",
    feature = "ng",
    feature = "ni",
    feature = "nl",
    feature = "no",
    feature = "np",
    feature = "nr",
    feature = "nu",
    feature = "nz",
    feature = "om",
    feature = "pa",
    feature = "pe",
    feature = "pf",
    feature = "pg",
    feature = "ph",
    feature = "pk",
    feature = "pl",
    feature = "pm",
    feature = "pn",
    feature = "pr",
    feature = "ps",
    feature = "pt",
    feature = "pw",
    feature = "py",
    feature = "qa",
    feature = "re",
    feature = "ro",
    feature = "rs",
    feature = "ru",
    feature = "rw",
    feature = "sa",
    feature = "sb",
    feature = "sc",
    feature = "sd",
    feature = "se",
    feature = "sg",
    feature = "sh",
    feature = "si",
    feature = "sj",
    feature = "sk",
    feature = "sl",
    feature = "sm",
    feature = "sn",
    feature = "so",
    feature = "sr",
    feature = "ss",
    feature = "st",
    feature = "sv",
    feature = "sx",
    feature = "sy",
    feature = "sz",
    feature = "tc",
    feature = "td",
    feature = "tf",
    feature = "tg",
    feature = "th",
    feature = "tj",
    feature = "tk",
    feature = "tl",
    feature = "tm",
    feature = "tn",
    feature = "to",
    feature = "tr",
    feature = "tt",
    feature = "tv",
    feature = "tw",
    feature = "tz",
    feature = "ua",
    feature = "ug",
    feature = "um",
    feature = "us",
    feature = "uy",
    feature = "uz",
    feature = "va",
    feature = "vc",
    feature = "ve",
    feature = "vg",
    feature = "vi",
    feature = "vn",
    feature = "vu",
    feature = "wf",
    feature = "ws",
    feature = "ye",
    feature = "yt",
    feature = "za",
    feature = "zm",
    feature = "zw",
)))]
pub const SUPPORT_ALL_COUNTRIES: bool = false;
pub const SUPPORTED_ALPHA2_LIST: &[Alpha2] = &[
    #[cfg(feature = "ad")] // The Principality of Andorra (Europe)
    Alpha2::AD,
    #[cfg(feature = "ae")] // The United Arab Emirates (Asia)
    Alpha2::AE,
    #[cfg(feature = "af")] // The Islamic Republic of Afghanistan (Asia)
    Alpha2::AF,
    #[cfg(feature = "ag")] // Antigua and Barbuda (Americas)
    Alpha2::AG,
    #[cfg(feature = "ai")] // Anguilla (Americas)
    Alpha2::AI,
    #[cfg(feature = "al")] // The Republic of Albania (Europe)
    Alpha2::AL,
    #[cfg(feature = "am")] // The Republic of Armenia (Asia)
    Alpha2::AM,
    #[cfg(feature = "ao")] // The Republic of Angola (Africa)
    Alpha2::AO,
    #[cfg(feature = "aq")] // Antarctica
    Alpha2::AQ,
    #[cfg(feature = "ar")] // The Argentine Republic (Americas)
    Alpha2::AR,
    #[cfg(feature = "as")] // The Territory of American Samoa (Oceania)
    Alpha2::AS,
    #[cfg(feature = "at")] // The Republic of Austria (Europe)
    Alpha2::AT,
    #[cfg(feature = "au")] // The Commonwealth of Australia (Oceania)
    Alpha2::AU,
    #[cfg(feature = "aw")] // Aruba (Americas)
    Alpha2::AW,
    #[cfg(feature = "ax")] // Åland (Europe)
    Alpha2::AX,
    #[cfg(feature = "az")] // The Republic of Azerbaijan (Asia)
    Alpha2::AZ,
    #[cfg(feature = "ba")] // Bosnia and Herzegovina (Europe)
    Alpha2::BA,
    #[cfg(feature = "bb")] // Barbados (Americas)
    Alpha2::BB,
    #[cfg(feature = "bd")] // The People's Republic of Bangladesh (Asia)
    Alpha2::BD,
    #[cfg(feature = "be")] // The Kingdom of Belgium (Europe)
    Alpha2::BE,
    #[cfg(feature = "bf")] // Burkina Faso (Africa)
    Alpha2::BF,
    #[cfg(feature = "bg")] // The Republic of Bulgaria (Europe)
    Alpha2::BG,
    #[cfg(feature = "bh")] // The Kingdom of Bahrain (Asia)
    Alpha2::BH,
    #[cfg(feature = "bi")] // The Republic of Burundi (Africa)
    Alpha2::BI,
    #[cfg(feature = "bj")] // The Republic of Benin (Africa)
    Alpha2::BJ,
    #[cfg(feature = "bl")] // The Collectivity of Saint-Barthélemy (Americas)
    Alpha2::BL,
    #[cfg(feature = "bm")] // Bermuda (Americas)
    Alpha2::BM,
    #[cfg(feature = "bn")] // The Nation of Brunei, the Abode of Peace (Asia)
    Alpha2::BN,
    #[cfg(feature = "bo")] // The Plurinational State of Bolivia (Americas)
    Alpha2::BO,
    #[cfg(feature = "bq")] // Bonaire, Sint Eustatius and Saba (Americas)
    Alpha2::BQ,
    #[cfg(feature = "br")] // The Federative Republic of Brazil (Americas)
    Alpha2::BR,
    #[cfg(feature = "bs")] // The Commonwealth of The Bahamas (Americas)
    Alpha2::BS,
    #[cfg(feature = "bt")] // The Kingdom of Bhutan (Asia)
    Alpha2::BT,
    #[cfg(feature = "bv")] // Bouvet Island
    Alpha2::BV,
    #[cfg(feature = "bw")] // The Republic of Botswana (Africa)
    Alpha2::BW,
    #[cfg(feature = "by")] // The Republic of Belarus (Europe)
    Alpha2::BY,
    #[cfg(feature = "bz")] // Belize (Americas)
    Alpha2::BZ,
    #[cfg(feature = "ca")] // Canada (Americas)
    Alpha2::CA,
    #[cfg(feature = "cc")] // The Territory of Cocos (Keeling) Islands (Oceania)
    Alpha2::CC,
    #[cfg(feature = "cd")] // The Democratic Republic of the Congo (Africa)
    Alpha2::CD,
    #[cfg(feature = "cf")] // The Central African Republic (Africa)
    Alpha2::CF,
    #[cfg(feature = "cg")] // The Republic of the Congo (Africa)
    Alpha2::CG,
    #[cfg(feature = "ch")] // The Swiss Confederation (Europe)
    Alpha2::CH,
    #[cfg(feature = "ci")] // The Republic of Côte d'Ivoire (Africa)
    Alpha2::CI,
    #[cfg(feature = "ck")] // The Cook Islands (Oceania)
    Alpha2::CK,
    #[cfg(feature = "cl")] // The Republic of Chile (Americas)
    Alpha2::CL,
    #[cfg(feature = "cm")] // The Republic of Cameroon (Africa)
    Alpha2::CM,
    #[cfg(feature = "cn")] // The People's Republic of China (Asia)
    Alpha2::CN,
    #[cfg(feature = "co")] // The Republic of Colombia (Americas)
    Alpha2::CO,
    #[cfg(feature = "cr")] // The Republic of Costa Rica (Americas)
    Alpha2::CR,
    #[cfg(feature = "cu")] // The Republic of Cuba (Americas)
    Alpha2::CU,
    #[cfg(feature = "cv")] // The Republic of Cabo Verde (Africa)
    Alpha2::CV,
    #[cfg(feature = "cw")] // The Country of Curaçao (Americas)
    Alpha2::CW,
    #[cfg(feature = "cx")] // The Territory of Christmas Island (Oceania)
    Alpha2::CX,
    #[cfg(feature = "cy")] // The Republic of Cyprus (Asia)
    Alpha2::CY,
    #[cfg(feature = "cz")] // The Czech Republic (Europe)
    Alpha2::CZ,
    #[cfg(feature = "de")] // The Federal Republic of Germany (Europe)
    Alpha2::DE,
    #[cfg(feature = "dj")] // The Republic of Djibouti (Africa)
    Alpha2::DJ,
    #[cfg(feature = "dk")] // The Kingdom of Denmark (Europe)
    Alpha2::DK,
    #[cfg(feature = "dm")] // The Commonwealth of Dominica (Americas)
    Alpha2::DM,
    #[cfg(feature = "do")] // The Dominican Republic (Americas)
    Alpha2::DO,
    #[cfg(feature = "dz")] // The People's Democratic Republic of Algeria (Africa)
    Alpha2::DZ,
    #[cfg(feature = "ec")] // The Republic of Ecuador (Americas)
    Alpha2::EC,
    #[cfg(feature = "ee")] // The Republic of Estonia (Europe)
    Alpha2::EE,
    #[cfg(feature = "eg")] // The Arab Republic of Egypt (Africa)
    Alpha2::EG,
    #[cfg(feature = "eh")] // The Sahrawi Arab Democratic Republic (Africa)
    Alpha2::EH,
    #[cfg(feature = "er")] // The State of Eritrea (Africa)
    Alpha2::ER,
    #[cfg(feature = "es")] // The Kingdom of Spain (Europe)
    Alpha2::ES,
    #[cfg(feature = "et")] // The Federal Democratic Republic of Ethiopia (Africa)
    Alpha2::ET,
    #[cfg(feature = "fi")] // The Republic of Finland (Europe)
    Alpha2::FI,
    #[cfg(feature = "fj")] // The Republic of Fiji (Oceania)
    Alpha2::FJ,
    #[cfg(feature = "fk")] // The Falkland Islands (Americas)
    Alpha2::FK,
    #[cfg(feature = "fm")] // The Federated States of Micronesia (Oceania)
    Alpha2::FM,
    #[cfg(feature = "fo")] // The Faroe Islands (Europe)
    Alpha2::FO,
    #[cfg(feature = "fr")] // The French Republic (Europe)
    Alpha2::FR,
    #[cfg(feature = "ga")] // The Gabonese Republic (Africa)
    Alpha2::GA,
    #[cfg(feature = "gb")] // The United Kingdom of Great Britain and Northern Ireland (Europe)
    Alpha2::GB,
    #[cfg(feature = "gd")] // Grenada (Americas)
    Alpha2::GD,
    #[cfg(feature = "ge")] // Georgia (Asia)
    Alpha2::GE,
    #[cfg(feature = "gf")] // Guyane (Americas)
    Alpha2::GF,
    #[cfg(feature = "gg")] // The Bailiwick of Guernsey (Europe)
    Alpha2::GG,
    #[cfg(feature = "gh")] // The Republic of Ghana (Africa)
    Alpha2::GH,
    #[cfg(feature = "gi")] // Gibraltar (Europe)
    Alpha2::GI,
    #[cfg(feature = "gl")] // Kalaallit Nunaat (Americas)
    Alpha2::GL,
    #[cfg(feature = "gm")] // The Republic of The Gambia (Africa)
    Alpha2::GM,
    #[cfg(feature = "gn")] // The Republic of Guinea (Africa)
    Alpha2::GN,
    #[cfg(feature = "gp")] // Guadeloupe (Americas)
    Alpha2::GP,
    #[cfg(feature = "gq")] // The Republic of Equatorial Guinea (Africa)
    Alpha2::GQ,
    #[cfg(feature = "gr")] // The Hellenic Republic (Europe)
    Alpha2::GR,
    #[cfg(feature = "gs")] // South Georgia and the South Sandwich Islands (Americas)
    Alpha2::GS,
    #[cfg(feature = "gt")] // The Republic of Guatemala (Americas)
    Alpha2::GT,
    #[cfg(feature = "gu")] // The Territory of Guam (Oceania)
    Alpha2::GU,
    #[cfg(feature = "gw")] // The Republic of Guinea-Bissau (Africa)
    Alpha2::GW,
    #[cfg(feature = "gy")] // The Co-operative Republic of Guyana (Americas)
    Alpha2::GY,
    #[cfg(feature = "hk")] // The Hong Kong Special Administrative Region of China (Asia)
    Alpha2::HK,
    #[cfg(feature = "hm")] // The Territory of Heard Island and McDonald Islands
    Alpha2::HM,
    #[cfg(feature = "hn")] // The Republic of Honduras (Americas)
    Alpha2::HN,
    #[cfg(feature = "hr")] // The Republic of Croatia (Europe)
    Alpha2::HR,
    #[cfg(feature = "ht")] // The Republic of Haiti (Americas)
    Alpha2::HT,
    #[cfg(feature = "hu")] // Hungary (Europe)
    Alpha2::HU,
    #[cfg(feature = "id")] // The Republic of Indonesia (Asia)
    Alpha2::ID,
    #[cfg(feature = "ie")] // Ireland (Europe)
    Alpha2::IE,
    #[cfg(feature = "il")] // The State of Israel (Asia)
    Alpha2::IL,
    #[cfg(feature = "im")] // The Isle of Man (Europe)
    Alpha2::IM,
    #[cfg(feature = "in")] // The Republic of India (Asia)
    Alpha2::IN,
    #[cfg(feature = "io")] // The British Indian Ocean Territory (Africa)
    Alpha2::IO,
    #[cfg(feature = "iq")] // The Republic of Iraq (Asia)
    Alpha2::IQ,
    #[cfg(feature = "ir")] // The Islamic Republic of Iran (Asia)
    Alpha2::IR,
    #[cfg(feature = "is")] // Iceland (Europe)
    Alpha2::IS,
    #[cfg(feature = "it")] // The Italian Republic (Europe)
    Alpha2::IT,
    #[cfg(feature = "je")] // The Bailiwick of Jersey (Europe)
    Alpha2::JE,
    #[cfg(feature = "jm")] // Jamaica (Americas)
    Alpha2::JM,
    #[cfg(feature = "jo")] // The Hashemite Kingdom of Jordan (Asia)
    Alpha2::JO,
    #[cfg(feature = "jp")] // Japan (Asia)
    Alpha2::JP,
    #[cfg(feature = "ke")] // The Republic of Kenya (Africa)
    Alpha2::KE,
    #[cfg(feature = "kg")] // The Kyrgyz Republic (Asia)
    Alpha2::KG,
    #[cfg(feature = "kh")] // The Kingdom of Cambodia (Asia)
    Alpha2::KH,
    #[cfg(feature = "ki")] // The Republic of Kiribati (Oceania)
    Alpha2::KI,
    #[cfg(feature = "km")] // The Union of the Comoros (Africa)
    Alpha2::KM,
    #[cfg(feature = "kn")] // Saint Kitts and Nevis (Americas)
    Alpha2::KN,
    #[cfg(feature = "kp")] // The Democratic People's Republic of Korea (Asia)
    Alpha2::KP,
    #[cfg(feature = "kr")] // The Republic of Korea (Asia)
    Alpha2::KR,
    #[cfg(feature = "kw")] // The State of Kuwait (Asia)
    Alpha2::KW,
    #[cfg(feature = "ky")] // The Cayman Islands (Americas)
    Alpha2::KY,
    #[cfg(feature = "kz")] // The Republic of Kazakhstan (Asia)
    Alpha2::KZ,
    #[cfg(feature = "la")] // The Lao People's Democratic Republic (Asia)
    Alpha2::LA,
    #[cfg(feature = "lb")] // The Lebanese Republic (Asia)
    Alpha2::LB,
    #[cfg(feature = "lc")] // Saint Lucia (Americas)
    Alpha2::LC,
    #[cfg(feature = "li")] // The Principality of Liechtenstein (Europe)
    Alpha2::LI,
    #[cfg(feature = "lk")] // The Democratic Socialist Republic of Sri Lanka (Asia)
    Alpha2::LK,
    #[cfg(feature = "lr")] // The Republic of Liberia (Africa)
    Alpha2::LR,
    #[cfg(feature = "ls")] // The Kingdom of Lesotho (Africa)
    Alpha2::LS,
    #[cfg(feature = "lt")] // The Republic of Lithuania (Europe)
    Alpha2::LT,
    #[cfg(feature = "lu")] // The Grand Duchy of Luxembourg (Europe)
    Alpha2::LU,
    #[cfg(feature = "lv")] // The Republic of Latvia (Europe)
    Alpha2::LV,
    #[cfg(feature = "ly")] // The State of Libya (Africa)
    Alpha2::LY,
    #[cfg(feature = "ma")] // The Kingdom of Morocco (Africa)
    Alpha2::MA,
    #[cfg(feature = "mc")] // The Principality of Monaco (Europe)
    Alpha2::MC,
    #[cfg(feature = "md")] // The Republic of Moldova (Europe)
    Alpha2::MD,
    #[cfg(feature = "me")] // Montenegro (Europe)
    Alpha2::ME,
    #[cfg(feature = "mf")] // The Collectivity of Saint-Martin (Americas)
    Alpha2::MF,
    #[cfg(feature = "mg")] // The Republic of Madagascar (Africa)
    Alpha2::MG,
    #[cfg(feature = "mh")] // The Republic of the Marshall Islands (Oceania)
    Alpha2::MH,
    #[cfg(feature = "mk")] // The Republic of North Macedonia (Europe)
    Alpha2::MK,
    #[cfg(feature = "ml")] // The Republic of Mali (Africa)
    Alpha2::ML,
    #[cfg(feature = "mm")] // The Republic of the Union of Myanmar (Asia)
    Alpha2::MM,
    #[cfg(feature = "mn")] // Mongolia (Asia)
    Alpha2::MN,
    #[cfg(feature = "mo")] // The Macao Special Administrative Region of China (Asia)
    Alpha2::MO,
    #[cfg(feature = "mp")] // The Commonwealth of the Northern Mariana Islands (Oceania)
    Alpha2::MP,
    #[cfg(feature = "mq")] // Martinique (Americas)
    Alpha2::MQ,
    #[cfg(feature = "mr")] // The Islamic Republic of Mauritania (Africa)
    Alpha2::MR,
    #[cfg(feature = "ms")] // Montserrat (Americas)
    Alpha2::MS,
    #[cfg(feature = "mt")] // The Republic of Malta (Europe)
    Alpha2::MT,
    #[cfg(feature = "mu")] // The Republic of Mauritius (Africa)
    Alpha2::MU,
    #[cfg(feature = "mv")] // The Republic of Maldives (Asia)
    Alpha2::MV,
    #[cfg(feature = "mw")] // The Republic of Malawi (Africa)
    Alpha2::MW,
    #[cfg(feature = "mx")] // The United Mexican States (Americas)
    Alpha2::MX,
    #[cfg(feature = "my")] // Malaysia (Asia)
    Alpha2::MY,
    #[cfg(feature = "mz")] // The Republic of Mozambique (Africa)
    Alpha2::MZ,
    #[cfg(feature = "na")] // The Republic of Namibia (Africa)
    Alpha2::NA,
    #[cfg(feature = "nc")] // New Caledonia (Oceania)
    Alpha2::NC,
    #[cfg(feature = "ne")] // The Republic of the Niger (Africa)
    Alpha2::NE,
    #[cfg(feature = "nf")] // The Territory of Norfolk Island (Oceania)
    Alpha2::NF,
    #[cfg(feature = "ng")] // The Federal Republic of Nigeria (Africa)
    Alpha2::NG,
    #[cfg(feature = "ni")] // The Republic of Nicaragua (Americas)
    Alpha2::NI,
    #[cfg(feature = "nl")] // The Kingdom of the Netherlands (Europe)
    Alpha2::NL,
    #[cfg(feature = "no")] // The Kingdom of Norway (Europe)
    Alpha2::NO,
    #[cfg(feature = "np")] // The Federal Democratic Republic of Nepal (Asia)
    Alpha2::NP,
    #[cfg(feature = "nr")] // The Republic of Nauru (Oceania)
    Alpha2::NR,
    #[cfg(feature = "nu")] // Niue (Oceania)
    Alpha2::NU,
    #[cfg(feature = "nz")] // New Zealand (Oceania)
    Alpha2::NZ,
    #[cfg(feature = "om")] // The Sultanate of Oman (Asia)
    Alpha2::OM,
    #[cfg(feature = "pa")] // The Republic of Panamá (Americas)
    Alpha2::PA,
    #[cfg(feature = "pe")] // The Republic of Perú (Americas)
    Alpha2::PE,
    #[cfg(feature = "pf")] // French Polynesia (Oceania)
    Alpha2::PF,
    #[cfg(feature = "pg")] // The Independent State of Papua New Guinea (Oceania)
    Alpha2::PG,
    #[cfg(feature = "ph")] // The Republic of the Philippines (Asia)
    Alpha2::PH,
    #[cfg(feature = "pk")] // The Islamic Republic of Pakistan (Asia)
    Alpha2::PK,
    #[cfg(feature = "pl")] // The Republic of Poland (Europe)
    Alpha2::PL,
    #[cfg(feature = "pm")] // The Overseas Collectivity of Saint-Pierre and Miquelon (Americas)
    Alpha2::PM,
    #[cfg(feature = "pn")] // The Pitcairn, Henderson, Ducie and Oeno Islands (Oceania)
    Alpha2::PN,
    #[cfg(feature = "pr")] // The Commonwealth of Puerto Rico (Americas)
    Alpha2::PR,
    #[cfg(feature = "ps")] // The State of Palestine (Asia)
    Alpha2::PS,
    #[cfg(feature = "pt")] // The Portuguese Republic (Europe)
    Alpha2::PT,
    #[cfg(feature = "pw")] // The Republic of Palau (Oceania)
    Alpha2::PW,
    #[cfg(feature = "py")] // The Republic of Paraguay (Americas)
    Alpha2::PY,
    #[cfg(feature = "qa")] // The State of Qatar (Asia)
    Alpha2::QA,
    #[cfg(feature = "re")] // Réunion (Africa)
    Alpha2::RE,
    #[cfg(feature = "ro")] // Romania (Europe)
    Alpha2::RO,
    #[cfg(feature = "rs")] // The Republic of Serbia (Europe)
    Alpha2::RS,
    #[cfg(feature = "ru")] // The Russian Federation (Europe)
    Alpha2::RU,
    #[cfg(feature = "rw")] // The Republic of Rwanda (Africa)
    Alpha2::RW,
    #[cfg(feature = "sa")] // The Kingdom of Saudi Arabia (Asia)
    Alpha2::SA,
    #[cfg(feature = "sb")] // The Solomon Islands (Oceania)
    Alpha2::SB,
    #[cfg(feature = "sc")] // The Republic of Seychelles (Africa)
    Alpha2::SC,
    #[cfg(feature = "sd")] // The Republic of the Sudan (Africa)
    Alpha2::SD,
    #[cfg(feature = "se")] // The Kingdom of Sweden (Europe)
    Alpha2::SE,
    #[cfg(feature = "sg")] // The Republic of Singapore (Asia)
    Alpha2::SG,
    #[cfg(feature = "sh")] // Saint Helena, Ascension and Tristan da Cunha (Africa)
    Alpha2::SH,
    #[cfg(feature = "si")] // The Republic of Slovenia (Europe)
    Alpha2::SI,
    #[cfg(feature = "sj")] // Svalbard and Jan Mayen (Europe)
    Alpha2::SJ,
    #[cfg(feature = "sk")] // The Slovak Republic (Europe)
    Alpha2::SK,
    #[cfg(feature = "sl")] // The Republic of Sierra Leone (Africa)
    Alpha2::SL,
    #[cfg(feature = "sm")] // The Republic of San Marino (Europe)
    Alpha2::SM,
    #[cfg(feature = "sn")] // The Republic of Senegal (Africa)
    Alpha2::SN,
    #[cfg(feature = "so")] // The Federal Republic of Somalia (Africa)
    Alpha2::SO,
    #[cfg(feature = "sr")] // The Republic of Suriname (Americas)
    Alpha2::SR,
    #[cfg(feature = "ss")] // The Republic of South Sudan (Africa)
    Alpha2::SS,
    #[cfg(feature = "st")] // The Democratic Republic of São Tomé and Príncipe (Africa)
    Alpha2::ST,
    #[cfg(feature = "sv")] // The Republic of El Salvador (Americas)
    Alpha2::SV,
    #[cfg(feature = "sx")] // Sint Maarten (Americas)
    Alpha2::SX,
    #[cfg(feature = "sy")] // The Syrian Arab Republic (Asia)
    Alpha2::SY,
    #[cfg(feature = "sz")] // The Kingdom of Eswatini (Africa)
    Alpha2::SZ,
    #[cfg(feature = "tc")] // The Turks and Caicos Islands (Americas)
    Alpha2::TC,
    #[cfg(feature = "td")] // The Republic of Chad (Africa)
    Alpha2::TD,
    #[cfg(feature = "tf")] // The French Southern and Antarctic Lands (Africa)
    Alpha2::TF,
    #[cfg(feature = "tg")] // The Togolese Republic (Africa)
    Alpha2::TG,
    #[cfg(feature = "th")] // The Kingdom of Thailand (Asia)
    Alpha2::TH,
    #[cfg(feature = "tj")] // The Republic of Tajikistan (Asia)
    Alpha2::TJ,
    #[cfg(feature = "tk")] // Tokelau (Oceania)
    Alpha2::TK,
    #[cfg(feature = "tl")] // The Democratic Republic of Timor-Leste (Asia)
    Alpha2::TL,
    #[cfg(feature = "tm")] // Turkmenistan (Asia)
    Alpha2::TM,
    #[cfg(feature = "tn")] // The Republic of Tunisia (Africa)
    Alpha2::TN,
    #[cfg(feature = "to")] // The Kingdom of Tonga (Oceania)
    Alpha2::TO,
    #[cfg(feature = "tr")] // The Republic of Turkey (Asia)
    Alpha2::TR,
    #[cfg(feature = "tt")] // The Republic of Trinidad and Tobago (Americas)
    Alpha2::TT,
    #[cfg(feature = "tv")] // Tuvalu (Oceania)
    Alpha2::TV,
    #[cfg(feature = "tw")] // The Republic of China (Asia)
    Alpha2::TW,
    #[cfg(feature = "tz")] // The United Republic of Tanzania (Africa)
    Alpha2::TZ,
    #[cfg(feature = "ua")] // Ukraine (Europe)
    Alpha2::UA,
    #[cfg(feature = "ug")] // The Republic of Uganda (Africa)
    Alpha2::UG,
    #[cfg(feature = "um")] // United States Minor Outlying Islands (Americas)
    Alpha2::UM,
    #[cfg(feature = "us")] // The United States of America (Americas)
    Alpha2::US,
    #[cfg(feature = "uy")] // The Oriental Republic of Uruguay (Americas)
    Alpha2::UY,
    #[cfg(feature = "uz")] // The Republic of Uzbekistan (Asia)
    Alpha2::UZ,
    #[cfg(feature = "va")] // The Holy See (Europe)
    Alpha2::VA,
    #[cfg(feature = "vc")] // Saint Vincent and the Grenadines (Americas)
    Alpha2::VC,
    #[cfg(feature = "ve")] // The Bolivarian Republic of Venezuela (Americas)
    Alpha2::VE,
    #[cfg(feature = "vg")] // The Virgin Islands (Americas)
    Alpha2::VG,
    #[cfg(feature = "vi")] // The Virgin Islands of the United States (Americas)
    Alpha2::VI,
    #[cfg(feature = "vn")] // The Socialist Republic of Viet Nam (Asia)
    Alpha2::VN,
    #[cfg(feature = "vu")] // The Republic of Vanuatu (Oceania)
    Alpha2::VU,
    #[cfg(feature = "wf")] // The Territory of the Wallis and Futuna Islands (Oceania)
    Alpha2::WF,
    #[cfg(feature = "ws")] // The Independent State of Samoa (Oceania)
    Alpha2::WS,
    #[cfg(feature = "ye")] // The Republic of Yemen (Asia)
    Alpha2::YE,
    #[cfg(feature = "yt")] // The Department of Mayotte (Africa)
    Alpha2::YT,
    #[cfg(feature = "za")] // The Republic of South Africa (Africa)
    Alpha2::ZA,
    #[cfg(feature = "zm")] // The Republic of Zambia (Africa)
    Alpha2::ZM,
    #[cfg(feature = "zw")] // The Republic of Zimbabwe (Africa)
    Alpha2::ZW,
];
lazy_static! {
    pub static ref SUPPORTED_REGION_LIST: &'static [Region] = &[
        #[cfg(all())]
        Region::Antarctica,
        #[cfg(all(
            feature = "ae",
            feature = "af",
            feature = "am",
            feature = "az",
            feature = "bd",
            feature = "bh",
            feature = "bn",
            feature = "bt",
            feature = "cn",
            feature = "cy",
            feature = "ge",
            feature = "hk",
            feature = "id",
            feature = "il",
            feature = "in",
            feature = "iq",
            feature = "ir",
            feature = "jo",
            feature = "jp",
            feature = "kg",
            feature = "kh",
            feature = "kp",
            feature = "kr",
            feature = "kw",
            feature = "kz",
            feature = "la",
            feature = "lb",
            feature = "lk",
            feature = "mm",
            feature = "mn",
            feature = "mo",
            feature = "mv",
            feature = "my",
            feature = "np",
            feature = "om",
            feature = "ph",
            feature = "pk",
            feature = "ps",
            feature = "qa",
            feature = "sa",
            feature = "sg",
            feature = "sy",
            feature = "th",
            feature = "tj",
            feature = "tl",
            feature = "tm",
            feature = "tr",
            feature = "tw",
            feature = "uz",
            feature = "vn",
            feature = "ye"
        ))]
        Region::Asia,
        #[cfg(all(
            feature = "ag",
            feature = "ai",
            feature = "ar",
            feature = "aw",
            feature = "bb",
            feature = "bl",
            feature = "bm",
            feature = "bo",
            feature = "bq",
            feature = "br",
            feature = "bs",
            feature = "bz",
            feature = "ca",
            feature = "cl",
            feature = "co",
            feature = "cr",
            feature = "cu",
            feature = "cw",
            feature = "dm",
            feature = "do",
            feature = "ec",
            feature = "fk",
            feature = "gd",
            feature = "gf",
            feature = "gl",
            feature = "gp",
            feature = "gs",
            feature = "gt",
            feature = "gy",
            feature = "hn",
            feature = "ht",
            feature = "jm",
            feature = "kn",
            feature = "ky",
            feature = "lc",
            feature = "mf",
            feature = "mq",
            feature = "ms",
            feature = "mx",
            feature = "ni",
            feature = "pa",
            feature = "pe",
            feature = "pm",
            feature = "pr",
            feature = "py",
            feature = "sr",
            feature = "sv",
            feature = "sx",
            feature = "tc",
            feature = "tt",
            feature = "um",
            feature = "us",
            feature = "uy",
            feature = "vc",
            feature = "ve",
            feature = "vg",
            feature = "vi"
        ))]
        Region::Americas,
        #[cfg(all(
            feature = "ao",
            feature = "bf",
            feature = "bi",
            feature = "bj",
            feature = "bw",
            feature = "cd",
            feature = "cf",
            feature = "cg",
            feature = "ci",
            feature = "cm",
            feature = "cv",
            feature = "dj",
            feature = "dz",
            feature = "eg",
            feature = "eh",
            feature = "er",
            feature = "et",
            feature = "ga",
            feature = "gh",
            feature = "gm",
            feature = "gn",
            feature = "gq",
            feature = "gw",
            feature = "io",
            feature = "ke",
            feature = "km",
            feature = "lr",
            feature = "ls",
            feature = "ly",
            feature = "ma",
            feature = "mg",
            feature = "ml",
            feature = "mr",
            feature = "mu",
            feature = "mw",
            feature = "mz",
            feature = "na",
            feature = "ne",
            feature = "ng",
            feature = "re",
            feature = "rw",
            feature = "sc",
            feature = "sd",
            feature = "sh",
            feature = "sl",
            feature = "sn",
            feature = "so",
            feature = "ss",
            feature = "st",
            feature = "sz",
            feature = "td",
            feature = "tf",
            feature = "tg",
            feature = "tn",
            feature = "tz",
            feature = "ug",
            feature = "yt",
            feature = "za",
            feature = "zm",
            feature = "zw"
        ))]
        Region::Africa,
        #[cfg(all(
            feature = "ad",
            feature = "al",
            feature = "at",
            feature = "ax",
            feature = "ba",
            feature = "be",
            feature = "bg",
            feature = "by",
            feature = "ch",
            feature = "cz",
            feature = "de",
            feature = "dk",
            feature = "ee",
            feature = "es",
            feature = "fi",
            feature = "fo",
            feature = "fr",
            feature = "gb",
            feature = "gg",
            feature = "gi",
            feature = "gr",
            feature = "hr",
            feature = "hu",
            feature = "ie",
            feature = "im",
            feature = "is",
            feature = "it",
            feature = "je",
            feature = "li",
            feature = "lt",
            feature = "lu",
            feature = "lv",
            feature = "mc",
            feature = "md",
            feature = "me",
            feature = "mk",
            feature = "mt",
            feature = "nl",
            feature = "no",
            feature = "pl",
            feature = "pt",
            feature = "ro",
            feature = "rs",
            feature = "ru",
            feature = "se",
            feature = "si",
            feature = "sj",
            feature = "sk",
            feature = "sm",
            feature = "ua",
            feature = "va"
        ))]
        Region::Europe,
        #[cfg(all(
            feature = "as",
            feature = "au",
            feature = "cc",
            feature = "ck",
            feature = "cx",
            feature = "fj",
            feature = "fm",
            feature = "gu",
            feature = "ki",
            feature = "mh",
            feature = "mp",
            feature = "nc",
            feature = "nf",
            feature = "nr",
            feature = "nu",
            feature = "nz",
            feature = "pf",
            feature = "pg",
            feature = "pn",
            feature = "pw",
            feature = "sb",
            feature = "tk",
            feature = "to",
            feature = "tv",
            feature = "vu",
            feature = "wf",
            feature = "ws"
        ))]
        Region::Oceania,
    ];
}
lazy_static! {
    pub static ref SUPPORTED_SUBREGION_LIST: &'static [SubRegion] = &[
        #[cfg(all(
            feature = "bw",
            feature = "ls",
            feature = "na",
            feature = "sz",
            feature = "za"
        ))]
        SubRegion::SouthernAfrica,
        #[cfg(all(
            feature = "fm",
            feature = "gu",
            feature = "ki",
            feature = "mh",
            feature = "mp",
            feature = "nr",
            feature = "pw"
        ))]
        SubRegion::Micronesia,
        #[cfg(all(
            feature = "ad",
            feature = "al",
            feature = "ba",
            feature = "es",
            feature = "gi",
            feature = "gr",
            feature = "hr",
            feature = "it",
            feature = "me",
            feature = "mk",
            feature = "mt",
            feature = "pt",
            feature = "rs",
            feature = "si",
            feature = "sm",
            feature = "va"
        ))]
        SubRegion::SouthernEurope,
        #[cfg(all(
            feature = "ao",
            feature = "cd",
            feature = "cf",
            feature = "cg",
            feature = "cm",
            feature = "ga",
            feature = "gq",
            feature = "st",
            feature = "td"
        ))]
        SubRegion::MiddleAfrica,
        #[cfg(all(
            feature = "bf",
            feature = "bj",
            feature = "ci",
            feature = "cv",
            feature = "gh",
            feature = "gm",
            feature = "gn",
            feature = "gw",
            feature = "lr",
            feature = "ml",
            feature = "mr",
            feature = "ne",
            feature = "ng",
            feature = "sh",
            feature = "sl",
            feature = "sn",
            feature = "tg"
        ))]
        SubRegion::WesternAfrica,
        #[cfg(all(
            feature = "bg",
            feature = "by",
            feature = "cz",
            feature = "hu",
            feature = "md",
            feature = "pl",
            feature = "ro",
            feature = "ru",
            feature = "sk",
            feature = "ua"
        ))]
        SubRegion::EasternEurope,
        #[cfg(all(
            feature = "bz",
            feature = "cr",
            feature = "gt",
            feature = "hn",
            feature = "mx",
            feature = "ni",
            feature = "pa",
            feature = "sv"
        ))]
        SubRegion::CentralAmerica,
        #[cfg(all(
            feature = "kg",
            feature = "kz",
            feature = "tj",
            feature = "tm",
            feature = "uz"
        ))]
        SubRegion::CentralAsia,
        #[cfg(all(
            feature = "af",
            feature = "bd",
            feature = "bt",
            feature = "in",
            feature = "ir",
            feature = "lk",
            feature = "mv",
            feature = "np",
            feature = "pk"
        ))]
        SubRegion::SouthernAsia,
        #[cfg(all(
            feature = "at",
            feature = "be",
            feature = "ch",
            feature = "de",
            feature = "fr",
            feature = "li",
            feature = "lu",
            feature = "mc",
            feature = "nl"
        ))]
        SubRegion::WesternEurope,
        #[cfg(all(
            feature = "dz",
            feature = "eg",
            feature = "eh",
            feature = "ly",
            feature = "ma",
            feature = "sd",
            feature = "ss",
            feature = "tn"
        ))]
        SubRegion::NorthernAfrica,
        #[cfg(all(
            feature = "au",
            feature = "cc",
            feature = "cx",
            feature = "nf",
            feature = "nz"
        ))]
        SubRegion::AustraliaAndNewZealand,
        #[cfg(all(
            feature = "as",
            feature = "ck",
            feature = "nu",
            feature = "pf",
            feature = "pn",
            feature = "tk",
            feature = "to",
            feature = "tv",
            feature = "wf",
            feature = "ws"
        ))]
        SubRegion::Polynesia,
        #[cfg(all(
            feature = "bi",
            feature = "dj",
            feature = "er",
            feature = "et",
            feature = "io",
            feature = "ke",
            feature = "km",
            feature = "mg",
            feature = "mu",
            feature = "mw",
            feature = "mz",
            feature = "re",
            feature = "rw",
            feature = "sc",
            feature = "so",
            feature = "tf",
            feature = "tz",
            feature = "ug",
            feature = "yt",
            feature = "zm",
            feature = "zw"
        ))]
        SubRegion::EasternAfrica,
        #[cfg(all(
            feature = "bn",
            feature = "id",
            feature = "kh",
            feature = "la",
            feature = "mm",
            feature = "my",
            feature = "ph",
            feature = "sg",
            feature = "th",
            feature = "tl",
            feature = "vn"
        ))]
        SubRegion::SouthEasternAsia,
        #[cfg(all(
            feature = "bm",
            feature = "ca",
            feature = "gl",
            feature = "pm",
            feature = "um",
            feature = "us"
        ))]
        SubRegion::NorthernAmerica,
        #[cfg(all(
            feature = "ar",
            feature = "bo",
            feature = "br",
            feature = "cl",
            feature = "co",
            feature = "ec",
            feature = "fk",
            feature = "gf",
            feature = "gs",
            feature = "gy",
            feature = "pe",
            feature = "py",
            feature = "sr",
            feature = "uy",
            feature = "ve"
        ))]
        SubRegion::SouthAmerica,
        #[cfg(all(
            feature = "fj",
            feature = "nc",
            feature = "pg",
            feature = "sb",
            feature = "vu"
        ))]
        SubRegion::Melanesia,
        #[cfg(all(
            feature = "ae",
            feature = "am",
            feature = "az",
            feature = "bh",
            feature = "cy",
            feature = "ge",
            feature = "il",
            feature = "iq",
            feature = "jo",
            feature = "kw",
            feature = "lb",
            feature = "om",
            feature = "ps",
            feature = "qa",
            feature = "sa",
            feature = "sy",
            feature = "tr",
            feature = "ye"
        ))]
        SubRegion::WesternAsia,
        #[cfg(all(
            feature = "cn",
            feature = "hk",
            feature = "jp",
            feature = "kp",
            feature = "kr",
            feature = "mn",
            feature = "mo",
            feature = "tw"
        ))]
        SubRegion::EasternAsia,
        #[cfg(all(
            feature = "ag",
            feature = "ai",
            feature = "aw",
            feature = "bb",
            feature = "bl",
            feature = "bq",
            feature = "bs",
            feature = "cu",
            feature = "cw",
            feature = "dm",
            feature = "do",
            feature = "gd",
            feature = "gp",
            feature = "ht",
            feature = "jm",
            feature = "kn",
            feature = "ky",
            feature = "lc",
            feature = "mf",
            feature = "mq",
            feature = "ms",
            feature = "pr",
            feature = "sx",
            feature = "tc",
            feature = "tt",
            feature = "vc",
            feature = "vg",
            feature = "vi"
        ))]
        SubRegion::Caribbean,
        #[cfg(all(
            feature = "ax",
            feature = "dk",
            feature = "ee",
            feature = "fi",
            feature = "fo",
            feature = "gb",
            feature = "gg",
            feature = "ie",
            feature = "im",
            feature = "is",
            feature = "je",
            feature = "lt",
            feature = "lv",
            feature = "no",
            feature = "se",
            feature = "sj"
        ))]
        SubRegion::NorthernEurope,
    ];
}
lazy_static! {
    pub static ref SUPPORTED_WORLD_REGION_LIST: &'static [WorldRegion] = &[
        #[cfg(all(
            feature = "ag",
            feature = "ai",
            feature = "aq",
            feature = "ar",
            feature = "aw",
            feature = "bb",
            feature = "bm",
            feature = "bo",
            feature = "br",
            feature = "bs",
            feature = "bz",
            feature = "ca",
            feature = "cl",
            feature = "co",
            feature = "cr",
            feature = "cu",
            feature = "cw",
            feature = "dm",
            feature = "do",
            feature = "ec",
            feature = "fk",
            feature = "gd",
            feature = "gf",
            feature = "gp",
            feature = "gs",
            feature = "gt",
            feature = "gy",
            feature = "hn",
            feature = "ht",
            feature = "jm",
            feature = "kn",
            feature = "ky",
            feature = "lc",
            feature = "mf",
            feature = "mq",
            feature = "mx",
            feature = "ni",
            feature = "pa",
            feature = "pe",
            feature = "pm",
            feature = "pr",
            feature = "py",
            feature = "sr",
            feature = "sv",
            feature = "sx",
            feature = "tt",
            feature = "um",
            feature = "us",
            feature = "uy",
            feature = "vc",
            feature = "ve",
            feature = "vg",
            feature = "vi"
        ))]
        WorldRegion::AMER,
        #[cfg(all(
            feature = "ad",
            feature = "ae",
            feature = "al",
            feature = "am",
            feature = "ao",
            feature = "at",
            feature = "ax",
            feature = "az",
            feature = "ba",
            feature = "be",
            feature = "bf",
            feature = "bg",
            feature = "bh",
            feature = "bi",
            feature = "bj",
            feature = "bw",
            feature = "by",
            feature = "cd",
            feature = "cf",
            feature = "cg",
            feature = "ch",
            feature = "ci",
            feature = "cm",
            feature = "cv",
            feature = "cy",
            feature = "cz",
            feature = "de",
            feature = "dj",
            feature = "dk",
            feature = "dz",
            feature = "ee",
            feature = "eg",
            feature = "eh",
            feature = "er",
            feature = "es",
            feature = "et",
            feature = "fi",
            feature = "fo",
            feature = "fr",
            feature = "ga",
            feature = "gb",
            feature = "ge",
            feature = "gg",
            feature = "gh",
            feature = "gi",
            feature = "gl",
            feature = "gm",
            feature = "gn",
            feature = "gq",
            feature = "gr",
            feature = "gw",
            feature = "hr",
            feature = "hu",
            feature = "ie",
            feature = "il",
            feature = "im",
            feature = "iq",
            feature = "ir",
            feature = "is",
            feature = "it",
            feature = "je",
            feature = "jo",
            feature = "ke",
            feature = "kg",
            feature = "km",
            feature = "kw",
            feature = "kz",
            feature = "lb",
            feature = "li",
            feature = "lr",
            feature = "ls",
            feature = "lt",
            feature = "lu",
            feature = "lv",
            feature = "ly",
            feature = "ma",
            feature = "mc",
            feature = "md",
            feature = "me",
            feature = "mg",
            feature = "mk",
            feature = "ml",
            feature = "mr",
            feature = "ms",
            feature = "mt",
            feature = "mu",
            feature = "mw",
            feature = "mz",
            feature = "na",
            feature = "ne",
            feature = "ng",
            feature = "nl",
            feature = "no",
            feature = "om",
            feature = "pl",
            feature = "ps",
            feature = "pt",
            feature = "qa",
            feature = "re",
            feature = "ro",
            feature = "rs",
            feature = "ru",
            feature = "rw",
            feature = "sa",
            feature = "sc",
            feature = "sd",
            feature = "se",
            feature = "si",
            feature = "sj",
            feature = "sk",
            feature = "sl",
            feature = "sm",
            feature = "sn",
            feature = "so",
            feature = "ss",
            feature = "st",
            feature = "sy",
            feature = "sz",
            feature = "td",
            feature = "tf",
            feature = "tg",
            feature = "tj",
            feature = "tm",
            feature = "tn",
            feature = "tr",
            feature = "tz",
            feature = "ua",
            feature = "ug",
            feature = "uz",
            feature = "va",
            feature = "ye",
            feature = "yt",
            feature = "za",
            feature = "zm",
            feature = "zw"
        ))]
        WorldRegion::EMEA,
        #[cfg(all(
            feature = "af",
            feature = "as",
            feature = "au",
            feature = "bd",
            feature = "bl",
            feature = "bn",
            feature = "bq",
            feature = "bt",
            feature = "bv",
            feature = "cc",
            feature = "ck",
            feature = "cn",
            feature = "cx",
            feature = "fj",
            feature = "fm",
            feature = "gu",
            feature = "hk",
            feature = "hm",
            feature = "id",
            feature = "in",
            feature = "io",
            feature = "jp",
            feature = "kh",
            feature = "ki",
            feature = "kp",
            feature = "kr",
            feature = "la",
            feature = "lk",
            feature = "mh",
            feature = "mm",
            feature = "mn",
            feature = "mo",
            feature = "mp",
            feature = "mv",
            feature = "my",
            feature = "nc",
            feature = "nf",
            feature = "np",
            feature = "nr",
            feature = "nu",
            feature = "nz",
            feature = "pf",
            feature = "pg",
            feature = "ph",
            feature = "pk",
            feature = "pn",
            feature = "pw",
            feature = "sb",
            feature = "sg",
            feature = "sh",
            feature = "tc",
            feature = "th",
            feature = "tk",
            feature = "tl",
            feature = "to",
            feature = "tv",
            feature = "tw",
            feature = "vn",
            feature = "vu",
            feature = "wf",
            feature = "ws"
        ))]
        WorldRegion::APAC,
    ];
}
lazy_static! { pub static ref SUPPORTED_ISO_SHORT_NAMES: HashMap<&'static str, Alpha2> = HashMap::from([
    #[cfg(feature = "ad")] // The Principality of Andorra (Europe)
    ("andorra", Alpha2::AD),
    #[cfg(feature = "ae")] // The United Arab Emirates (Asia)
    ("united arab emirates", Alpha2::AE),
    #[cfg(feature = "af")] // The Islamic Republic of Afghanistan (Asia)
    ("afghanistan", Alpha2::AF),
    #[cfg(feature = "ag")] // Antigua and Barbuda (Americas)
    ("antigua and barbuda", Alpha2::AG),
    #[cfg(feature = "ai")] // Anguilla (Americas)
    ("anguilla", Alpha2::AI),
    #[cfg(feature = "al")] // The Republic of Albania (Europe)
    ("albania", Alpha2::AL),
    #[cfg(feature = "am")] // The Republic of Armenia (Asia)
    ("armenia", Alpha2::AM),
    #[cfg(feature = "ao")] // The Republic of Angola (Africa)
    ("angola", Alpha2::AO),
    #[cfg(feature = "aq")] // Antarctica
    ("antarctica", Alpha2::AQ),
    #[cfg(feature = "ar")] // The Argentine Republic (Americas)
    ("argentina", Alpha2::AR),
    #[cfg(feature = "as")] // The Territory of American Samoa (Oceania)
    ("american samoa", Alpha2::AS),
    #[cfg(feature = "at")] // The Republic of Austria (Europe)
    ("austria", Alpha2::AT),
    #[cfg(feature = "au")] // The Commonwealth of Australia (Oceania)
    ("australia", Alpha2::AU),
    #[cfg(feature = "aw")] // Aruba (Americas)
    ("aruba", Alpha2::AW),
    #[cfg(feature = "ax")] // Åland (Europe)
    ("åland islands", Alpha2::AX),
    #[cfg(feature = "az")] // The Republic of Azerbaijan (Asia)
    ("azerbaijan", Alpha2::AZ),
    #[cfg(feature = "ba")] // Bosnia and Herzegovina (Europe)
    ("bosnia and herzegovina", Alpha2::BA),
    #[cfg(feature = "bb")] // Barbados (Americas)
    ("barbados", Alpha2::BB),
    #[cfg(feature = "bd")] // The People's Republic of Bangladesh (Asia)
    ("bangladesh", Alpha2::BD),
    #[cfg(feature = "be")] // The Kingdom of Belgium (Europe)
    ("belgium", Alpha2::BE),
    #[cfg(feature = "bf")] // Burkina Faso (Africa)
    ("burkina faso", Alpha2::BF),
    #[cfg(feature = "bg")] // The Republic of Bulgaria (Europe)
    ("bulgaria", Alpha2::BG),
    #[cfg(feature = "bh")] // The Kingdom of Bahrain (Asia)
    ("bahrain", Alpha2::BH),
    #[cfg(feature = "bi")] // The Republic of Burundi (Africa)
    ("burundi", Alpha2::BI),
    #[cfg(feature = "bj")] // The Republic of Benin (Africa)
    ("benin", Alpha2::BJ),
    #[cfg(feature = "bl")] // The Collectivity of Saint-Barthélemy (Americas)
    ("saint barthélemy", Alpha2::BL),
    #[cfg(feature = "bm")] // Bermuda (Americas)
    ("bermuda", Alpha2::BM),
    #[cfg(feature = "bn")] // The Nation of Brunei, the Abode of Peace (Asia)
    ("brunei darussalam", Alpha2::BN),
    #[cfg(feature = "bo")] // The Plurinational State of Bolivia (Americas)
    ("bolivia (plurinational state of)", Alpha2::BO),
    #[cfg(feature = "bq")] // Bonaire, Sint Eustatius and Saba (Americas)
    ("bonaire, sint eustatius and saba", Alpha2::BQ),
    #[cfg(feature = "br")] // The Federative Republic of Brazil (Americas)
    ("brazil", Alpha2::BR),
    #[cfg(feature = "bs")] // The Commonwealth of The Bahamas (Americas)
    ("bahamas", Alpha2::BS),
    #[cfg(feature = "bt")] // The Kingdom of Bhutan (Asia)
    ("bhutan", Alpha2::BT),
    #[cfg(feature = "bv")] // Bouvet Island
    ("bouvet island", Alpha2::BV),
    #[cfg(feature = "bw")] // The Republic of Botswana (Africa)
    ("botswana", Alpha2::BW),
    #[cfg(feature = "by")] // The Republic of Belarus (Europe)
    ("belarus", Alpha2::BY),
    #[cfg(feature = "bz")] // Belize (Americas)
    ("belize", Alpha2::BZ),
    #[cfg(feature = "ca")] // Canada (Americas)
    ("canada", Alpha2::CA),
    #[cfg(feature = "cc")] // The Territory of Cocos (Keeling) Islands (Oceania)
    ("cocos (keeling) islands", Alpha2::CC),
    #[cfg(feature = "cd")] // The Democratic Republic of the Congo (Africa)
    ("congo (democratic republic of the)", Alpha2::CD),
    #[cfg(feature = "cf")] // The Central African Republic (Africa)
    ("central african republic", Alpha2::CF),
    #[cfg(feature = "cg")] // The Republic of the Congo (Africa)
    ("congo", Alpha2::CG),
    #[cfg(feature = "ch")] // The Swiss Confederation (Europe)
    ("switzerland", Alpha2::CH),
    #[cfg(feature = "ci")] // The Republic of Côte d'Ivoire (Africa)
    ("côte d'ivoire", Alpha2::CI),
    #[cfg(feature = "ck")] // The Cook Islands (Oceania)
    ("cook islands", Alpha2::CK),
    #[cfg(feature = "cl")] // The Republic of Chile (Americas)
    ("chile", Alpha2::CL),
    #[cfg(feature = "cm")] // The Republic of Cameroon (Africa)
    ("cameroon", Alpha2::CM),
    #[cfg(feature = "cn")] // The People's Republic of China (Asia)
    ("china", Alpha2::CN),
    #[cfg(feature = "co")] // The Republic of Colombia (Americas)
    ("colombia", Alpha2::CO),
    #[cfg(feature = "cr")] // The Republic of Costa Rica (Americas)
    ("costa rica", Alpha2::CR),
    #[cfg(feature = "cu")] // The Republic of Cuba (Americas)
    ("cuba", Alpha2::CU),
    #[cfg(feature = "cv")] // The Republic of Cabo Verde (Africa)
    ("cabo verde", Alpha2::CV),
    #[cfg(feature = "cw")] // The Country of Curaçao (Americas)
    ("curaçao", Alpha2::CW),
    #[cfg(feature = "cx")] // The Territory of Christmas Island (Oceania)
    ("christmas island", Alpha2::CX),
    #[cfg(feature = "cy")] // The Republic of Cyprus (Asia)
    ("cyprus", Alpha2::CY),
    #[cfg(feature = "cz")] // The Czech Republic (Europe)
    ("czechia", Alpha2::CZ),
    #[cfg(feature = "de")] // The Federal Republic of Germany (Europe)
    ("germany", Alpha2::DE),
    #[cfg(feature = "dj")] // The Republic of Djibouti (Africa)
    ("djibouti", Alpha2::DJ),
    #[cfg(feature = "dk")] // The Kingdom of Denmark (Europe)
    ("denmark", Alpha2::DK),
    #[cfg(feature = "dm")] // The Commonwealth of Dominica (Americas)
    ("dominica", Alpha2::DM),
    #[cfg(feature = "do")] // The Dominican Republic (Americas)
    ("dominican republic", Alpha2::DO),
    #[cfg(feature = "dz")] // The People's Democratic Republic of Algeria (Africa)
    ("algeria", Alpha2::DZ),
    #[cfg(feature = "ec")] // The Republic of Ecuador (Americas)
    ("ecuador", Alpha2::EC),
    #[cfg(feature = "ee")] // The Republic of Estonia (Europe)
    ("estonia", Alpha2::EE),
    #[cfg(feature = "eg")] // The Arab Republic of Egypt (Africa)
    ("egypt", Alpha2::EG),
    #[cfg(feature = "eh")] // The Sahrawi Arab Democratic Republic (Africa)
    ("western sahara", Alpha2::EH),
    #[cfg(feature = "er")] // The State of Eritrea (Africa)
    ("eritrea", Alpha2::ER),
    #[cfg(feature = "es")] // The Kingdom of Spain (Europe)
    ("spain", Alpha2::ES),
    #[cfg(feature = "et")] // The Federal Democratic Republic of Ethiopia (Africa)
    ("ethiopia", Alpha2::ET),
    #[cfg(feature = "fi")] // The Republic of Finland (Europe)
    ("finland", Alpha2::FI),
    #[cfg(feature = "fj")] // The Republic of Fiji (Oceania)
    ("fiji", Alpha2::FJ),
    #[cfg(feature = "fk")] // The Falkland Islands (Americas)
    ("falkland islands (malvinas)", Alpha2::FK),
    #[cfg(feature = "fm")] // The Federated States of Micronesia (Oceania)
    ("micronesia (federated states of)", Alpha2::FM),
    #[cfg(feature = "fo")] // The Faroe Islands (Europe)
    ("faroe islands", Alpha2::FO),
    #[cfg(feature = "fr")] // The French Republic (Europe)
    ("france", Alpha2::FR),
    #[cfg(feature = "ga")] // The Gabonese Republic (Africa)
    ("gabon", Alpha2::GA),
    #[cfg(feature = "gb")] // The United Kingdom of Great Britain and Northern Ireland (Europe)
    ("united kingdom of great britain and northern ireland", Alpha2::GB),
    #[cfg(feature = "gd")] // Grenada (Americas)
    ("grenada", Alpha2::GD),
    #[cfg(feature = "ge")] // Georgia (Asia)
    ("georgia", Alpha2::GE),
    #[cfg(feature = "gf")] // Guyane (Americas)
    ("french guiana", Alpha2::GF),
    #[cfg(feature = "gg")] // The Bailiwick of Guernsey (Europe)
    ("guernsey", Alpha2::GG),
    #[cfg(feature = "gh")] // The Republic of Ghana (Africa)
    ("ghana", Alpha2::GH),
    #[cfg(feature = "gi")] // Gibraltar (Europe)
    ("gibraltar", Alpha2::GI),
    #[cfg(feature = "gl")] // Kalaallit Nunaat (Americas)
    ("greenland", Alpha2::GL),
    #[cfg(feature = "gm")] // The Republic of The Gambia (Africa)
    ("gambia", Alpha2::GM),
    #[cfg(feature = "gn")] // The Republic of Guinea (Africa)
    ("guinea", Alpha2::GN),
    #[cfg(feature = "gp")] // Guadeloupe (Americas)
    ("guadeloupe", Alpha2::GP),
    #[cfg(feature = "gq")] // The Republic of Equatorial Guinea (Africa)
    ("equatorial guinea", Alpha2::GQ),
    #[cfg(feature = "gr")] // The Hellenic Republic (Europe)
    ("greece", Alpha2::GR),
    #[cfg(feature = "gs")] // South Georgia and the South Sandwich Islands (Americas)
    ("south georgia and the south sandwich islands", Alpha2::GS),
    #[cfg(feature = "gt")] // The Republic of Guatemala (Americas)
    ("guatemala", Alpha2::GT),
    #[cfg(feature = "gu")] // The Territory of Guam (Oceania)
    ("guam", Alpha2::GU),
    #[cfg(feature = "gw")] // The Republic of Guinea-Bissau (Africa)
    ("guinea-bissau", Alpha2::GW),
    #[cfg(feature = "gy")] // The Co-operative Republic of Guyana (Americas)
    ("guyana", Alpha2::GY),
    #[cfg(feature = "hk")] // The Hong Kong Special Administrative Region of China (Asia)
    ("hong kong", Alpha2::HK),
    #[cfg(feature = "hm")] // The Territory of Heard Island and McDonald Islands
    ("heard island and mcdonald islands", Alpha2::HM),
    #[cfg(feature = "hn")] // The Republic of Honduras (Americas)
    ("honduras", Alpha2::HN),
    #[cfg(feature = "hr")] // The Republic of Croatia (Europe)
    ("croatia", Alpha2::HR),
    #[cfg(feature = "ht")] // The Republic of Haiti (Americas)
    ("haiti", Alpha2::HT),
    #[cfg(feature = "hu")] // Hungary (Europe)
    ("hungary", Alpha2::HU),
    #[cfg(feature = "id")] // The Republic of Indonesia (Asia)
    ("indonesia", Alpha2::ID),
    #[cfg(feature = "ie")] // Ireland (Europe)
    ("ireland", Alpha2::IE),
    #[cfg(feature = "il")] // The State of Israel (Asia)
    ("israel", Alpha2::IL),
    #[cfg(feature = "im")] // The Isle of Man (Europe)
    ("isle of man", Alpha2::IM),
    #[cfg(feature = "in")] // The Republic of India (Asia)
    ("india", Alpha2::IN),
    #[cfg(feature = "io")] // The British Indian Ocean Territory (Africa)
    ("british indian ocean territory", Alpha2::IO),
    #[cfg(feature = "iq")] // The Republic of Iraq (Asia)
    ("iraq", Alpha2::IQ),
    #[cfg(feature = "ir")] // The Islamic Republic of Iran (Asia)
    ("iran (islamic republic of)", Alpha2::IR),
    #[cfg(feature = "is")] // Iceland (Europe)
    ("iceland", Alpha2::IS),
    #[cfg(feature = "it")] // The Italian Republic (Europe)
    ("italy", Alpha2::IT),
    #[cfg(feature = "je")] // The Bailiwick of Jersey (Europe)
    ("jersey", Alpha2::JE),
    #[cfg(feature = "jm")] // Jamaica (Americas)
    ("jamaica", Alpha2::JM),
    #[cfg(feature = "jo")] // The Hashemite Kingdom of Jordan (Asia)
    ("jordan", Alpha2::JO),
    #[cfg(feature = "jp")] // Japan (Asia)
    ("japan", Alpha2::JP),
    #[cfg(feature = "ke")] // The Republic of Kenya (Africa)
    ("kenya", Alpha2::KE),
    #[cfg(feature = "kg")] // The Kyrgyz Republic (Asia)
    ("kyrgyzstan", Alpha2::KG),
    #[cfg(feature = "kh")] // The Kingdom of Cambodia (Asia)
    ("cambodia", Alpha2::KH),
    #[cfg(feature = "ki")] // The Republic of Kiribati (Oceania)
    ("kiribati", Alpha2::KI),
    #[cfg(feature = "km")] // The Union of the Comoros (Africa)
    ("comoros", Alpha2::KM),
    #[cfg(feature = "kn")] // Saint Kitts and Nevis (Americas)
    ("saint kitts and nevis", Alpha2::KN),
    #[cfg(feature = "kp")] // The Democratic People's Republic of Korea (Asia)
    ("korea (democratic people's republic of)", Alpha2::KP),
    #[cfg(feature = "kr")] // The Republic of Korea (Asia)
    ("korea (republic of)", Alpha2::KR),
    #[cfg(feature = "kw")] // The State of Kuwait (Asia)
    ("kuwait", Alpha2::KW),
    #[cfg(feature = "ky")] // The Cayman Islands (Americas)
    ("cayman islands", Alpha2::KY),
    #[cfg(feature = "kz")] // The Republic of Kazakhstan (Asia)
    ("kazakhstan", Alpha2::KZ),
    #[cfg(feature = "la")] // The Lao People's Democratic Republic (Asia)
    ("lao people's democratic republic", Alpha2::LA),
    #[cfg(feature = "lb")] // The Lebanese Republic (Asia)
    ("lebanon", Alpha2::LB),
    #[cfg(feature = "lc")] // Saint Lucia (Americas)
    ("saint lucia", Alpha2::LC),
    #[cfg(feature = "li")] // The Principality of Liechtenstein (Europe)
    ("liechtenstein", Alpha2::LI),
    #[cfg(feature = "lk")] // The Democratic Socialist Republic of Sri Lanka (Asia)
    ("sri lanka", Alpha2::LK),
    #[cfg(feature = "lr")] // The Republic of Liberia (Africa)
    ("liberia", Alpha2::LR),
    #[cfg(feature = "ls")] // The Kingdom of Lesotho (Africa)
    ("lesotho", Alpha2::LS),
    #[cfg(feature = "lt")] // The Republic of Lithuania (Europe)
    ("lithuania", Alpha2::LT),
    #[cfg(feature = "lu")] // The Grand Duchy of Luxembourg (Europe)
    ("luxembourg", Alpha2::LU),
    #[cfg(feature = "lv")] // The Republic of Latvia (Europe)
    ("latvia", Alpha2::LV),
    #[cfg(feature = "ly")] // The State of Libya (Africa)
    ("libya", Alpha2::LY),
    #[cfg(feature = "ma")] // The Kingdom of Morocco (Africa)
    ("morocco", Alpha2::MA),
    #[cfg(feature = "mc")] // The Principality of Monaco (Europe)
    ("monaco", Alpha2::MC),
    #[cfg(feature = "md")] // The Republic of Moldova (Europe)
    ("moldova (republic of)", Alpha2::MD),
    #[cfg(feature = "me")] // Montenegro (Europe)
    ("montenegro", Alpha2::ME),
    #[cfg(feature = "mf")] // The Collectivity of Saint-Martin (Americas)
    ("saint martin (french part)", Alpha2::MF),
    #[cfg(feature = "mg")] // The Republic of Madagascar (Africa)
    ("madagascar", Alpha2::MG),
    #[cfg(feature = "mh")] // The Republic of the Marshall Islands (Oceania)
    ("marshall islands", Alpha2::MH),
    #[cfg(feature = "mk")] // The Republic of North Macedonia (Europe)
    ("north macedonia", Alpha2::MK),
    #[cfg(feature = "ml")] // The Republic of Mali (Africa)
    ("mali", Alpha2::ML),
    #[cfg(feature = "mm")] // The Republic of the Union of Myanmar (Asia)
    ("myanmar", Alpha2::MM),
    #[cfg(feature = "mn")] // Mongolia (Asia)
    ("mongolia", Alpha2::MN),
    #[cfg(feature = "mo")] // The Macao Special Administrative Region of China (Asia)
    ("macao", Alpha2::MO),
    #[cfg(feature = "mp")] // The Commonwealth of the Northern Mariana Islands (Oceania)
    ("northern mariana islands", Alpha2::MP),
    #[cfg(feature = "mq")] // Martinique (Americas)
    ("martinique", Alpha2::MQ),
    #[cfg(feature = "mr")] // The Islamic Republic of Mauritania (Africa)
    ("mauritania", Alpha2::MR),
    #[cfg(feature = "ms")] // Montserrat (Americas)
    ("montserrat", Alpha2::MS),
    #[cfg(feature = "mt")] // The Republic of Malta (Europe)
    ("malta", Alpha2::MT),
    #[cfg(feature = "mu")] // The Republic of Mauritius (Africa)
    ("mauritius", Alpha2::MU),
    #[cfg(feature = "mv")] // The Republic of Maldives (Asia)
    ("maldives", Alpha2::MV),
    #[cfg(feature = "mw")] // The Republic of Malawi (Africa)
    ("malawi", Alpha2::MW),
    #[cfg(feature = "mx")] // The United Mexican States (Americas)
    ("mexico", Alpha2::MX),
    #[cfg(feature = "my")] // Malaysia (Asia)
    ("malaysia", Alpha2::MY),
    #[cfg(feature = "mz")] // The Republic of Mozambique (Africa)
    ("mozambique", Alpha2::MZ),
    #[cfg(feature = "na")] // The Republic of Namibia (Africa)
    ("namibia", Alpha2::NA),
    #[cfg(feature = "nc")] // New Caledonia (Oceania)
    ("new caledonia", Alpha2::NC),
    #[cfg(feature = "ne")] // The Republic of the Niger (Africa)
    ("niger", Alpha2::NE),
    #[cfg(feature = "nf")] // The Territory of Norfolk Island (Oceania)
    ("norfolk island", Alpha2::NF),
    #[cfg(feature = "ng")] // The Federal Republic of Nigeria (Africa)
    ("nigeria", Alpha2::NG),
    #[cfg(feature = "ni")] // The Republic of Nicaragua (Americas)
    ("nicaragua", Alpha2::NI),
    #[cfg(feature = "nl")] // The Kingdom of the Netherlands (Europe)
    ("netherlands", Alpha2::NL),
    #[cfg(feature = "no")] // The Kingdom of Norway (Europe)
    ("norway", Alpha2::NO),
    #[cfg(feature = "np")] // The Federal Democratic Republic of Nepal (Asia)
    ("nepal", Alpha2::NP),
    #[cfg(feature = "nr")] // The Republic of Nauru (Oceania)
    ("nauru", Alpha2::NR),
    #[cfg(feature = "nu")] // Niue (Oceania)
    ("niue", Alpha2::NU),
    #[cfg(feature = "nz")] // New Zealand (Oceania)
    ("new zealand", Alpha2::NZ),
    #[cfg(feature = "om")] // The Sultanate of Oman (Asia)
    ("oman", Alpha2::OM),
    #[cfg(feature = "pa")] // The Republic of Panamá (Americas)
    ("panama", Alpha2::PA),
    #[cfg(feature = "pe")] // The Republic of Perú (Americas)
    ("peru", Alpha2::PE),
    #[cfg(feature = "pf")] // French Polynesia (Oceania)
    ("french polynesia", Alpha2::PF),
    #[cfg(feature = "pg")] // The Independent State of Papua New Guinea (Oceania)
    ("papua new guinea", Alpha2::PG),
    #[cfg(feature = "ph")] // The Republic of the Philippines (Asia)
    ("philippines", Alpha2::PH),
    #[cfg(feature = "pk")] // The Islamic Republic of Pakistan (Asia)
    ("pakistan", Alpha2::PK),
    #[cfg(feature = "pl")] // The Republic of Poland (Europe)
    ("poland", Alpha2::PL),
    #[cfg(feature = "pm")] // The Overseas Collectivity of Saint-Pierre and Miquelon (Americas)
    ("saint pierre and miquelon", Alpha2::PM),
    #[cfg(feature = "pn")] // The Pitcairn, Henderson, Ducie and Oeno Islands (Oceania)
    ("pitcairn", Alpha2::PN),
    #[cfg(feature = "pr")] // The Commonwealth of Puerto Rico (Americas)
    ("puerto rico", Alpha2::PR),
    #[cfg(feature = "ps")] // The State of Palestine (Asia)
    ("palestine, state of", Alpha2::PS),
    #[cfg(feature = "pt")] // The Portuguese Republic (Europe)
    ("portugal", Alpha2::PT),
    #[cfg(feature = "pw")] // The Republic of Palau (Oceania)
    ("palau", Alpha2::PW),
    #[cfg(feature = "py")] // The Republic of Paraguay (Americas)
    ("paraguay", Alpha2::PY),
    #[cfg(feature = "qa")] // The State of Qatar (Asia)
    ("qatar", Alpha2::QA),
    #[cfg(feature = "re")] // Réunion (Africa)
    ("réunion", Alpha2::RE),
    #[cfg(feature = "ro")] // Romania (Europe)
    ("romania", Alpha2::RO),
    #[cfg(feature = "rs")] // The Republic of Serbia (Europe)
    ("serbia", Alpha2::RS),
    #[cfg(feature = "ru")] // The Russian Federation (Europe)
    ("russian federation", Alpha2::RU),
    #[cfg(feature = "rw")] // The Republic of Rwanda (Africa)
    ("rwanda", Alpha2::RW),
    #[cfg(feature = "sa")] // The Kingdom of Saudi Arabia (Asia)
    ("saudi arabia", Alpha2::SA),
    #[cfg(feature = "sb")] // The Solomon Islands (Oceania)
    ("solomon islands", Alpha2::SB),
    #[cfg(feature = "sc")] // The Republic of Seychelles (Africa)
    ("seychelles", Alpha2::SC),
    #[cfg(feature = "sd")] // The Republic of the Sudan (Africa)
    ("sudan", Alpha2::SD),
    #[cfg(feature = "se")] // The Kingdom of Sweden (Europe)
    ("sweden", Alpha2::SE),
    #[cfg(feature = "sg")] // The Republic of Singapore (Asia)
    ("singapore", Alpha2::SG),
    #[cfg(feature = "sh")] // Saint Helena, Ascension and Tristan da Cunha (Africa)
    ("saint helena, ascension and tristan da cunha", Alpha2::SH),
    #[cfg(feature = "si")] // The Republic of Slovenia (Europe)
    ("slovenia", Alpha2::SI),
    #[cfg(feature = "sj")] // Svalbard and Jan Mayen (Europe)
    ("svalbard and jan mayen", Alpha2::SJ),
    #[cfg(feature = "sk")] // The Slovak Republic (Europe)
    ("slovakia", Alpha2::SK),
    #[cfg(feature = "sl")] // The Republic of Sierra Leone (Africa)
    ("sierra leone", Alpha2::SL),
    #[cfg(feature = "sm")] // The Republic of San Marino (Europe)
    ("san marino", Alpha2::SM),
    #[cfg(feature = "sn")] // The Republic of Senegal (Africa)
    ("senegal", Alpha2::SN),
    #[cfg(feature = "so")] // The Federal Republic of Somalia (Africa)
    ("somalia", Alpha2::SO),
    #[cfg(feature = "sr")] // The Republic of Suriname (Americas)
    ("suriname", Alpha2::SR),
    #[cfg(feature = "ss")] // The Republic of South Sudan (Africa)
    ("south sudan", Alpha2::SS),
    #[cfg(feature = "st")] // The Democratic Republic of São Tomé and Príncipe (Africa)
    ("sao tome and principe", Alpha2::ST),
    #[cfg(feature = "sv")] // The Republic of El Salvador (Americas)
    ("el salvador", Alpha2::SV),
    #[cfg(feature = "sx")] // Sint Maarten (Americas)
    ("sint maarten (dutch part)", Alpha2::SX),
    #[cfg(feature = "sy")] // The Syrian Arab Republic (Asia)
    ("syrian arab republic", Alpha2::SY),
    #[cfg(feature = "sz")] // The Kingdom of Eswatini (Africa)
    ("eswatini", Alpha2::SZ),
    #[cfg(feature = "tc")] // The Turks and Caicos Islands (Americas)
    ("turks and caicos islands", Alpha2::TC),
    #[cfg(feature = "td")] // The Republic of Chad (Africa)
    ("chad", Alpha2::TD),
    #[cfg(feature = "tf")] // The French Southern and Antarctic Lands (Africa)
    ("french southern territories", Alpha2::TF),
    #[cfg(feature = "tg")] // The Togolese Republic (Africa)
    ("togo", Alpha2::TG),
    #[cfg(feature = "th")] // The Kingdom of Thailand (Asia)
    ("thailand", Alpha2::TH),
    #[cfg(feature = "tj")] // The Republic of Tajikistan (Asia)
    ("tajikistan", Alpha2::TJ),
    #[cfg(feature = "tk")] // Tokelau (Oceania)
    ("tokelau", Alpha2::TK),
    #[cfg(feature = "tl")] // The Democratic Republic of Timor-Leste (Asia)
    ("timor-leste", Alpha2::TL),
    #[cfg(feature = "tm")] // Turkmenistan (Asia)
    ("turkmenistan", Alpha2::TM),
    #[cfg(feature = "tn")] // The Republic of Tunisia (Africa)
    ("tunisia", Alpha2::TN),
    #[cfg(feature = "to")] // The Kingdom of Tonga (Oceania)
    ("tonga", Alpha2::TO),
    #[cfg(feature = "tr")] // The Republic of Turkey (Asia)
    ("turkey", Alpha2::TR),
    #[cfg(feature = "tt")] // The Republic of Trinidad and Tobago (Americas)
    ("trinidad and tobago", Alpha2::TT),
    #[cfg(feature = "tv")] // Tuvalu (Oceania)
    ("tuvalu", Alpha2::TV),
    #[cfg(feature = "tw")] // The Republic of China (Asia)
    ("taiwan, province of china", Alpha2::TW),
    #[cfg(feature = "tz")] // The United Republic of Tanzania (Africa)
    ("tanzania, united republic of", Alpha2::TZ),
    #[cfg(feature = "ua")] // Ukraine (Europe)
    ("ukraine", Alpha2::UA),
    #[cfg(feature = "ug")] // The Republic of Uganda (Africa)
    ("uganda", Alpha2::UG),
    #[cfg(feature = "um")] // United States Minor Outlying Islands (Americas)
    ("united states minor outlying islands", Alpha2::UM),
    #[cfg(feature = "us")] // The United States of America (Americas)
    ("united states of america", Alpha2::US),
    #[cfg(feature = "uy")] // The Oriental Republic of Uruguay (Americas)
    ("uruguay", Alpha2::UY),
    #[cfg(feature = "uz")] // The Republic of Uzbekistan (Asia)
    ("uzbekistan", Alpha2::UZ),
    #[cfg(feature = "va")] // The Holy See (Europe)
    ("holy see", Alpha2::VA),
    #[cfg(feature = "vc")] // Saint Vincent and the Grenadines (Americas)
    ("saint vincent and the grenadines", Alpha2::VC),
    #[cfg(feature = "ve")] // The Bolivarian Republic of Venezuela (Americas)
    ("venezuela (bolivarian republic of)", Alpha2::VE),
    #[cfg(feature = "vg")] // The Virgin Islands (Americas)
    ("virgin islands (british)", Alpha2::VG),
    #[cfg(feature = "vi")] // The Virgin Islands of the United States (Americas)
    ("virgin islands (u.s.)", Alpha2::VI),
    #[cfg(feature = "vn")] // The Socialist Republic of Viet Nam (Asia)
    ("viet nam", Alpha2::VN),
    #[cfg(feature = "vu")] // The Republic of Vanuatu (Oceania)
    ("vanuatu", Alpha2::VU),
    #[cfg(feature = "wf")] // The Territory of the Wallis and Futuna Islands (Oceania)
    ("wallis and futuna", Alpha2::WF),
    #[cfg(feature = "ws")] // The Independent State of Samoa (Oceania)
    ("samoa", Alpha2::WS),
    #[cfg(feature = "ye")] // The Republic of Yemen (Asia)
    ("yemen", Alpha2::YE),
    #[cfg(feature = "yt")] // The Department of Mayotte (Africa)
    ("mayotte", Alpha2::YT),
    #[cfg(feature = "za")] // The Republic of South Africa (Africa)
    ("south africa", Alpha2::ZA),
    #[cfg(feature = "zm")] // The Republic of Zambia (Africa)
    ("zambia", Alpha2::ZM),
    #[cfg(feature = "zw")] // The Republic of Zimbabwe (Africa)
    ("zimbabwe", Alpha2::ZW),
]);}
lazy_static! { pub static ref SUPPORTED_ISO_LONG_NAMES: HashMap<&'static str, Alpha2> = HashMap::from([
    #[cfg(feature = "ad")] // The Principality of Andorra (Europe)
    ("the principality of andorra", Alpha2::AD),
    #[cfg(feature = "ae")] // The United Arab Emirates (Asia)
    ("the united arab emirates", Alpha2::AE),
    #[cfg(feature = "af")] // The Islamic Republic of Afghanistan (Asia)
    ("the islamic republic of afghanistan", Alpha2::AF),
    #[cfg(feature = "ag")] // Antigua and Barbuda (Americas)
    ("antigua and barbuda", Alpha2::AG),
    #[cfg(feature = "ai")] // Anguilla (Americas)
    ("anguilla", Alpha2::AI),
    #[cfg(feature = "al")] // The Republic of Albania (Europe)
    ("the republic of albania", Alpha2::AL),
    #[cfg(feature = "am")] // The Republic of Armenia (Asia)
    ("the republic of armenia", Alpha2::AM),
    #[cfg(feature = "ao")] // The Republic of Angola (Africa)
    ("the republic of angola", Alpha2::AO),
    #[cfg(feature = "aq")] // Antarctica
    ("antarctica", Alpha2::AQ),
    #[cfg(feature = "ar")] // The Argentine Republic (Americas)
    ("the argentine republic", Alpha2::AR),
    #[cfg(feature = "as")] // The Territory of American Samoa (Oceania)
    ("the territory of american samoa", Alpha2::AS),
    #[cfg(feature = "at")] // The Republic of Austria (Europe)
    ("the republic of austria", Alpha2::AT),
    #[cfg(feature = "au")] // The Commonwealth of Australia (Oceania)
    ("the commonwealth of australia", Alpha2::AU),
    #[cfg(feature = "aw")] // Aruba (Americas)
    ("aruba", Alpha2::AW),
    #[cfg(feature = "ax")] // Åland (Europe)
    ("åland", Alpha2::AX),
    #[cfg(feature = "az")] // The Republic of Azerbaijan (Asia)
    ("the republic of azerbaijan", Alpha2::AZ),
    #[cfg(feature = "ba")] // Bosnia and Herzegovina (Europe)
    ("bosnia and herzegovina", Alpha2::BA),
    #[cfg(feature = "bb")] // Barbados (Americas)
    ("barbados", Alpha2::BB),
    #[cfg(feature = "bd")] // The People's Republic of Bangladesh (Asia)
    ("the people's republic of bangladesh", Alpha2::BD),
    #[cfg(feature = "be")] // The Kingdom of Belgium (Europe)
    ("the kingdom of belgium", Alpha2::BE),
    #[cfg(feature = "bf")] // Burkina Faso (Africa)
    ("burkina faso", Alpha2::BF),
    #[cfg(feature = "bg")] // The Republic of Bulgaria (Europe)
    ("the republic of bulgaria", Alpha2::BG),
    #[cfg(feature = "bh")] // The Kingdom of Bahrain (Asia)
    ("the kingdom of bahrain", Alpha2::BH),
    #[cfg(feature = "bi")] // The Republic of Burundi (Africa)
    ("the republic of burundi", Alpha2::BI),
    #[cfg(feature = "bj")] // The Republic of Benin (Africa)
    ("the republic of benin", Alpha2::BJ),
    #[cfg(feature = "bl")] // The Collectivity of Saint-Barthélemy (Americas)
    ("the collectivity of saint-barthélemy", Alpha2::BL),
    #[cfg(feature = "bm")] // Bermuda (Americas)
    ("bermuda", Alpha2::BM),
    #[cfg(feature = "bn")] // The Nation of Brunei, the Abode of Peace (Asia)
    ("the nation of brunei, the abode of peace", Alpha2::BN),
    #[cfg(feature = "bo")] // The Plurinational State of Bolivia (Americas)
    ("the plurinational state of bolivia", Alpha2::BO),
    #[cfg(feature = "bq")] // Bonaire, Sint Eustatius and Saba (Americas)
    ("bonaire, sint eustatius and saba", Alpha2::BQ),
    #[cfg(feature = "br")] // The Federative Republic of Brazil (Americas)
    ("the federative republic of brazil", Alpha2::BR),
    #[cfg(feature = "bs")] // The Commonwealth of The Bahamas (Americas)
    ("the commonwealth of the bahamas", Alpha2::BS),
    #[cfg(feature = "bt")] // The Kingdom of Bhutan (Asia)
    ("the kingdom of bhutan", Alpha2::BT),
    #[cfg(feature = "bv")] // Bouvet Island
    ("bouvet island", Alpha2::BV),
    #[cfg(feature = "bw")] // The Republic of Botswana (Africa)
    ("the republic of botswana", Alpha2::BW),
    #[cfg(feature = "by")] // The Republic of Belarus (Europe)
    ("the republic of belarus", Alpha2::BY),
    #[cfg(feature = "bz")] // Belize (Americas)
    ("belize", Alpha2::BZ),
    #[cfg(feature = "ca")] // Canada (Americas)
    ("canada", Alpha2::CA),
    #[cfg(feature = "cc")] // The Territory of Cocos (Keeling) Islands (Oceania)
    ("the territory of cocos (keeling) islands", Alpha2::CC),
    #[cfg(feature = "cd")] // The Democratic Republic of the Congo (Africa)
    ("the democratic republic of the congo", Alpha2::CD),
    #[cfg(feature = "cf")] // The Central African Republic (Africa)
    ("the central african republic", Alpha2::CF),
    #[cfg(feature = "cg")] // The Republic of the Congo (Africa)
    ("the republic of the congo", Alpha2::CG),
    #[cfg(feature = "ch")] // The Swiss Confederation (Europe)
    ("the swiss confederation", Alpha2::CH),
    #[cfg(feature = "ci")] // The Republic of Côte d'Ivoire (Africa)
    ("the republic of côte d'ivoire", Alpha2::CI),
    #[cfg(feature = "ck")] // The Cook Islands (Oceania)
    ("the cook islands", Alpha2::CK),
    #[cfg(feature = "cl")] // The Republic of Chile (Americas)
    ("the republic of chile", Alpha2::CL),
    #[cfg(feature = "cm")] // The Republic of Cameroon (Africa)
    ("the republic of cameroon", Alpha2::CM),
    #[cfg(feature = "cn")] // The People's Republic of China (Asia)
    ("the people's republic of china", Alpha2::CN),
    #[cfg(feature = "co")] // The Republic of Colombia (Americas)
    ("the republic of colombia", Alpha2::CO),
    #[cfg(feature = "cr")] // The Republic of Costa Rica (Americas)
    ("the republic of costa rica", Alpha2::CR),
    #[cfg(feature = "cu")] // The Republic of Cuba (Americas)
    ("the republic of cuba", Alpha2::CU),
    #[cfg(feature = "cv")] // The Republic of Cabo Verde (Africa)
    ("the republic of cabo verde", Alpha2::CV),
    #[cfg(feature = "cw")] // The Country of Curaçao (Americas)
    ("the country of curaçao", Alpha2::CW),
    #[cfg(feature = "cx")] // The Territory of Christmas Island (Oceania)
    ("the territory of christmas island", Alpha2::CX),
    #[cfg(feature = "cy")] // The Republic of Cyprus (Asia)
    ("the republic of cyprus", Alpha2::CY),
    #[cfg(feature = "cz")] // The Czech Republic (Europe)
    ("the czech republic", Alpha2::CZ),
    #[cfg(feature = "de")] // The Federal Republic of Germany (Europe)
    ("the federal republic of germany", Alpha2::DE),
    #[cfg(feature = "dj")] // The Republic of Djibouti (Africa)
    ("the republic of djibouti", Alpha2::DJ),
    #[cfg(feature = "dk")] // The Kingdom of Denmark (Europe)
    ("the kingdom of denmark", Alpha2::DK),
    #[cfg(feature = "dm")] // The Commonwealth of Dominica (Americas)
    ("the commonwealth of dominica", Alpha2::DM),
    #[cfg(feature = "do")] // The Dominican Republic (Americas)
    ("the dominican republic", Alpha2::DO),
    #[cfg(feature = "dz")] // The People's Democratic Republic of Algeria (Africa)
    ("the people's democratic republic of algeria", Alpha2::DZ),
    #[cfg(feature = "ec")] // The Republic of Ecuador (Americas)
    ("the republic of ecuador", Alpha2::EC),
    #[cfg(feature = "ee")] // The Republic of Estonia (Europe)
    ("the republic of estonia", Alpha2::EE),
    #[cfg(feature = "eg")] // The Arab Republic of Egypt (Africa)
    ("the arab republic of egypt", Alpha2::EG),
    #[cfg(feature = "eh")] // The Sahrawi Arab Democratic Republic (Africa)
    ("the sahrawi arab democratic republic", Alpha2::EH),
    #[cfg(feature = "er")] // The State of Eritrea (Africa)
    ("the state of eritrea", Alpha2::ER),
    #[cfg(feature = "es")] // The Kingdom of Spain (Europe)
    ("the kingdom of spain", Alpha2::ES),
    #[cfg(feature = "et")] // The Federal Democratic Republic of Ethiopia (Africa)
    ("the federal democratic republic of ethiopia", Alpha2::ET),
    #[cfg(feature = "fi")] // The Republic of Finland (Europe)
    ("the republic of finland", Alpha2::FI),
    #[cfg(feature = "fj")] // The Republic of Fiji (Oceania)
    ("the republic of fiji", Alpha2::FJ),
    #[cfg(feature = "fk")] // The Falkland Islands (Americas)
    ("the falkland islands", Alpha2::FK),
    #[cfg(feature = "fm")] // The Federated States of Micronesia (Oceania)
    ("the federated states of micronesia", Alpha2::FM),
    #[cfg(feature = "fo")] // The Faroe Islands (Europe)
    ("the faroe islands", Alpha2::FO),
    #[cfg(feature = "fr")] // The French Republic (Europe)
    ("the french republic", Alpha2::FR),
    #[cfg(feature = "ga")] // The Gabonese Republic (Africa)
    ("the gabonese republic", Alpha2::GA),
    #[cfg(feature = "gb")] // The United Kingdom of Great Britain and Northern Ireland (Europe)
    ("the united kingdom of great britain and northern ireland", Alpha2::GB),
    #[cfg(feature = "gd")] // Grenada (Americas)
    ("grenada", Alpha2::GD),
    #[cfg(feature = "ge")] // Georgia (Asia)
    ("georgia", Alpha2::GE),
    #[cfg(feature = "gf")] // Guyane (Americas)
    ("guyane", Alpha2::GF),
    #[cfg(feature = "gg")] // The Bailiwick of Guernsey (Europe)
    ("the bailiwick of guernsey", Alpha2::GG),
    #[cfg(feature = "gh")] // The Republic of Ghana (Africa)
    ("the republic of ghana", Alpha2::GH),
    #[cfg(feature = "gi")] // Gibraltar (Europe)
    ("gibraltar", Alpha2::GI),
    #[cfg(feature = "gl")] // Kalaallit Nunaat (Americas)
    ("kalaallit nunaat", Alpha2::GL),
    #[cfg(feature = "gm")] // The Republic of The Gambia (Africa)
    ("the republic of the gambia", Alpha2::GM),
    #[cfg(feature = "gn")] // The Republic of Guinea (Africa)
    ("the republic of guinea", Alpha2::GN),
    #[cfg(feature = "gp")] // Guadeloupe (Americas)
    ("guadeloupe", Alpha2::GP),
    #[cfg(feature = "gq")] // The Republic of Equatorial Guinea (Africa)
    ("the republic of equatorial guinea", Alpha2::GQ),
    #[cfg(feature = "gr")] // The Hellenic Republic (Europe)
    ("the hellenic republic", Alpha2::GR),
    #[cfg(feature = "gs")] // South Georgia and the South Sandwich Islands (Americas)
    ("south georgia and the south sandwich islands", Alpha2::GS),
    #[cfg(feature = "gt")] // The Republic of Guatemala (Americas)
    ("the republic of guatemala", Alpha2::GT),
    #[cfg(feature = "gu")] // The Territory of Guam (Oceania)
    ("the territory of guam", Alpha2::GU),
    #[cfg(feature = "gw")] // The Republic of Guinea-Bissau (Africa)
    ("the republic of guinea-bissau", Alpha2::GW),
    #[cfg(feature = "gy")] // The Co-operative Republic of Guyana (Americas)
    ("the co-operative republic of guyana", Alpha2::GY),
    #[cfg(feature = "hk")] // The Hong Kong Special Administrative Region of China (Asia)
    ("the hong kong special administrative region of china", Alpha2::HK),
    #[cfg(feature = "hm")] // The Territory of Heard Island and McDonald Islands
    ("the territory of heard island and mcdonald islands", Alpha2::HM),
    #[cfg(feature = "hn")] // The Republic of Honduras (Americas)
    ("the republic of honduras", Alpha2::HN),
    #[cfg(feature = "hr")] // The Republic of Croatia (Europe)
    ("the republic of croatia", Alpha2::HR),
    #[cfg(feature = "ht")] // The Republic of Haiti (Americas)
    ("the republic of haiti", Alpha2::HT),
    #[cfg(feature = "hu")] // Hungary (Europe)
    ("hungary", Alpha2::HU),
    #[cfg(feature = "id")] // The Republic of Indonesia (Asia)
    ("the republic of indonesia", Alpha2::ID),
    #[cfg(feature = "ie")] // Ireland (Europe)
    ("ireland", Alpha2::IE),
    #[cfg(feature = "il")] // The State of Israel (Asia)
    ("the state of israel", Alpha2::IL),
    #[cfg(feature = "im")] // The Isle of Man (Europe)
    ("the isle of man", Alpha2::IM),
    #[cfg(feature = "in")] // The Republic of India (Asia)
    ("the republic of india", Alpha2::IN),
    #[cfg(feature = "io")] // The British Indian Ocean Territory (Africa)
    ("the british indian ocean territory", Alpha2::IO),
    #[cfg(feature = "iq")] // The Republic of Iraq (Asia)
    ("the republic of iraq", Alpha2::IQ),
    #[cfg(feature = "ir")] // The Islamic Republic of Iran (Asia)
    ("the islamic republic of iran", Alpha2::IR),
    #[cfg(feature = "is")] // Iceland (Europe)
    ("iceland", Alpha2::IS),
    #[cfg(feature = "it")] // The Italian Republic (Europe)
    ("the italian republic", Alpha2::IT),
    #[cfg(feature = "je")] // The Bailiwick of Jersey (Europe)
    ("the bailiwick of jersey", Alpha2::JE),
    #[cfg(feature = "jm")] // Jamaica (Americas)
    ("jamaica", Alpha2::JM),
    #[cfg(feature = "jo")] // The Hashemite Kingdom of Jordan (Asia)
    ("the hashemite kingdom of jordan", Alpha2::JO),
    #[cfg(feature = "jp")] // Japan (Asia)
    ("japan", Alpha2::JP),
    #[cfg(feature = "ke")] // The Republic of Kenya (Africa)
    ("the republic of kenya", Alpha2::KE),
    #[cfg(feature = "kg")] // The Kyrgyz Republic (Asia)
    ("the kyrgyz republic", Alpha2::KG),
    #[cfg(feature = "kh")] // The Kingdom of Cambodia (Asia)
    ("the kingdom of cambodia", Alpha2::KH),
    #[cfg(feature = "ki")] // The Republic of Kiribati (Oceania)
    ("the republic of kiribati", Alpha2::KI),
    #[cfg(feature = "km")] // The Union of the Comoros (Africa)
    ("the union of the comoros", Alpha2::KM),
    #[cfg(feature = "kn")] // Saint Kitts and Nevis (Americas)
    ("saint kitts and nevis", Alpha2::KN),
    #[cfg(feature = "kp")] // The Democratic People's Republic of Korea (Asia)
    ("the democratic people's republic of korea", Alpha2::KP),
    #[cfg(feature = "kr")] // The Republic of Korea (Asia)
    ("the republic of korea", Alpha2::KR),
    #[cfg(feature = "kw")] // The State of Kuwait (Asia)
    ("the state of kuwait", Alpha2::KW),
    #[cfg(feature = "ky")] // The Cayman Islands (Americas)
    ("the cayman islands", Alpha2::KY),
    #[cfg(feature = "kz")] // The Republic of Kazakhstan (Asia)
    ("the republic of kazakhstan", Alpha2::KZ),
    #[cfg(feature = "la")] // The Lao People's Democratic Republic (Asia)
    ("the lao people's democratic republic", Alpha2::LA),
    #[cfg(feature = "lb")] // The Lebanese Republic (Asia)
    ("the lebanese republic", Alpha2::LB),
    #[cfg(feature = "lc")] // Saint Lucia (Americas)
    ("saint lucia", Alpha2::LC),
    #[cfg(feature = "li")] // The Principality of Liechtenstein (Europe)
    ("the principality of liechtenstein", Alpha2::LI),
    #[cfg(feature = "lk")] // The Democratic Socialist Republic of Sri Lanka (Asia)
    ("the democratic socialist republic of sri lanka", Alpha2::LK),
    #[cfg(feature = "lr")] // The Republic of Liberia (Africa)
    ("the republic of liberia", Alpha2::LR),
    #[cfg(feature = "ls")] // The Kingdom of Lesotho (Africa)
    ("the kingdom of lesotho", Alpha2::LS),
    #[cfg(feature = "lt")] // The Republic of Lithuania (Europe)
    ("the republic of lithuania", Alpha2::LT),
    #[cfg(feature = "lu")] // The Grand Duchy of Luxembourg (Europe)
    ("the grand duchy of luxembourg", Alpha2::LU),
    #[cfg(feature = "lv")] // The Republic of Latvia (Europe)
    ("the republic of latvia", Alpha2::LV),
    #[cfg(feature = "ly")] // The State of Libya (Africa)
    ("the state of libya", Alpha2::LY),
    #[cfg(feature = "ma")] // The Kingdom of Morocco (Africa)
    ("the kingdom of morocco", Alpha2::MA),
    #[cfg(feature = "mc")] // The Principality of Monaco (Europe)
    ("the principality of monaco", Alpha2::MC),
    #[cfg(feature = "md")] // The Republic of Moldova (Europe)
    ("the republic of moldova", Alpha2::MD),
    #[cfg(feature = "me")] // Montenegro (Europe)
    ("montenegro", Alpha2::ME),
    #[cfg(feature = "mf")] // The Collectivity of Saint-Martin (Americas)
    ("the collectivity of saint-martin", Alpha2::MF),
    #[cfg(feature = "mg")] // The Republic of Madagascar (Africa)
    ("the republic of madagascar", Alpha2::MG),
    #[cfg(feature = "mh")] // The Republic of the Marshall Islands (Oceania)
    ("the republic of the marshall islands", Alpha2::MH),
    #[cfg(feature = "mk")] // The Republic of North Macedonia (Europe)
    ("the republic of north macedonia", Alpha2::MK),
    #[cfg(feature = "ml")] // The Republic of Mali (Africa)
    ("the republic of mali", Alpha2::ML),
    #[cfg(feature = "mm")] // The Republic of the Union of Myanmar (Asia)
    ("the republic of the union of myanmar", Alpha2::MM),
    #[cfg(feature = "mn")] // Mongolia (Asia)
    ("mongolia", Alpha2::MN),
    #[cfg(feature = "mo")] // The Macao Special Administrative Region of China (Asia)
    ("the macao special administrative region of china", Alpha2::MO),
    #[cfg(feature = "mp")] // The Commonwealth of the Northern Mariana Islands (Oceania)
    ("the commonwealth of the northern mariana islands", Alpha2::MP),
    #[cfg(feature = "mq")] // Martinique (Americas)
    ("martinique", Alpha2::MQ),
    #[cfg(feature = "mr")] // The Islamic Republic of Mauritania (Africa)
    ("the islamic republic of mauritania", Alpha2::MR),
    #[cfg(feature = "ms")] // Montserrat (Americas)
    ("montserrat", Alpha2::MS),
    #[cfg(feature = "mt")] // The Republic of Malta (Europe)
    ("the republic of malta", Alpha2::MT),
    #[cfg(feature = "mu")] // The Republic of Mauritius (Africa)
    ("the republic of mauritius", Alpha2::MU),
    #[cfg(feature = "mv")] // The Republic of Maldives (Asia)
    ("the republic of maldives", Alpha2::MV),
    #[cfg(feature = "mw")] // The Republic of Malawi (Africa)
    ("the republic of malawi", Alpha2::MW),
    #[cfg(feature = "mx")] // The United Mexican States (Americas)
    ("the united mexican states", Alpha2::MX),
    #[cfg(feature = "my")] // Malaysia (Asia)
    ("malaysia", Alpha2::MY),
    #[cfg(feature = "mz")] // The Republic of Mozambique (Africa)
    ("the republic of mozambique", Alpha2::MZ),
    #[cfg(feature = "na")] // The Republic of Namibia (Africa)
    ("the republic of namibia", Alpha2::NA),
    #[cfg(feature = "nc")] // New Caledonia (Oceania)
    ("new caledonia", Alpha2::NC),
    #[cfg(feature = "ne")] // The Republic of the Niger (Africa)
    ("the republic of the niger", Alpha2::NE),
    #[cfg(feature = "nf")] // The Territory of Norfolk Island (Oceania)
    ("the territory of norfolk island", Alpha2::NF),
    #[cfg(feature = "ng")] // The Federal Republic of Nigeria (Africa)
    ("the federal republic of nigeria", Alpha2::NG),
    #[cfg(feature = "ni")] // The Republic of Nicaragua (Americas)
    ("the republic of nicaragua", Alpha2::NI),
    #[cfg(feature = "nl")] // The Kingdom of the Netherlands (Europe)
    ("the kingdom of the netherlands", Alpha2::NL),
    #[cfg(feature = "no")] // The Kingdom of Norway (Europe)
    ("the kingdom of norway", Alpha2::NO),
    #[cfg(feature = "np")] // The Federal Democratic Republic of Nepal (Asia)
    ("the federal democratic republic of nepal", Alpha2::NP),
    #[cfg(feature = "nr")] // The Republic of Nauru (Oceania)
    ("the republic of nauru", Alpha2::NR),
    #[cfg(feature = "nu")] // Niue (Oceania)
    ("niue", Alpha2::NU),
    #[cfg(feature = "nz")] // New Zealand (Oceania)
    ("new zealand", Alpha2::NZ),
    #[cfg(feature = "om")] // The Sultanate of Oman (Asia)
    ("the sultanate of oman", Alpha2::OM),
    #[cfg(feature = "pa")] // The Republic of Panamá (Americas)
    ("the republic of panamá", Alpha2::PA),
    #[cfg(feature = "pe")] // The Republic of Perú (Americas)
    ("the republic of perú", Alpha2::PE),
    #[cfg(feature = "pf")] // French Polynesia (Oceania)
    ("french polynesia", Alpha2::PF),
    #[cfg(feature = "pg")] // The Independent State of Papua New Guinea (Oceania)
    ("the independent state of papua new guinea", Alpha2::PG),
    #[cfg(feature = "ph")] // The Republic of the Philippines (Asia)
    ("the republic of the philippines", Alpha2::PH),
    #[cfg(feature = "pk")] // The Islamic Republic of Pakistan (Asia)
    ("the islamic republic of pakistan", Alpha2::PK),
    #[cfg(feature = "pl")] // The Republic of Poland (Europe)
    ("the republic of poland", Alpha2::PL),
    #[cfg(feature = "pm")] // The Overseas Collectivity of Saint-Pierre and Miquelon (Americas)
    ("the overseas collectivity of saint-pierre and miquelon", Alpha2::PM),
    #[cfg(feature = "pn")] // The Pitcairn, Henderson, Ducie and Oeno Islands (Oceania)
    ("the pitcairn, henderson, ducie and oeno islands", Alpha2::PN),
    #[cfg(feature = "pr")] // The Commonwealth of Puerto Rico (Americas)
    ("the commonwealth of puerto rico", Alpha2::PR),
    #[cfg(feature = "ps")] // The State of Palestine (Asia)
    ("the state of palestine", Alpha2::PS),
    #[cfg(feature = "pt")] // The Portuguese Republic (Europe)
    ("the portuguese republic", Alpha2::PT),
    #[cfg(feature = "pw")] // The Republic of Palau (Oceania)
    ("the republic of palau", Alpha2::PW),
    #[cfg(feature = "py")] // The Republic of Paraguay (Americas)
    ("the republic of paraguay", Alpha2::PY),
    #[cfg(feature = "qa")] // The State of Qatar (Asia)
    ("the state of qatar", Alpha2::QA),
    #[cfg(feature = "re")] // Réunion (Africa)
    ("réunion", Alpha2::RE),
    #[cfg(feature = "ro")] // Romania (Europe)
    ("romania", Alpha2::RO),
    #[cfg(feature = "rs")] // The Republic of Serbia (Europe)
    ("the republic of serbia", Alpha2::RS),
    #[cfg(feature = "ru")] // The Russian Federation (Europe)
    ("the russian federation", Alpha2::RU),
    #[cfg(feature = "rw")] // The Republic of Rwanda (Africa)
    ("the republic of rwanda", Alpha2::RW),
    #[cfg(feature = "sa")] // The Kingdom of Saudi Arabia (Asia)
    ("the kingdom of saudi arabia", Alpha2::SA),
    #[cfg(feature = "sb")] // The Solomon Islands (Oceania)
    ("the solomon islands", Alpha2::SB),
    #[cfg(feature = "sc")] // The Republic of Seychelles (Africa)
    ("the republic of seychelles", Alpha2::SC),
    #[cfg(feature = "sd")] // The Republic of the Sudan (Africa)
    ("the republic of the sudan", Alpha2::SD),
    #[cfg(feature = "se")] // The Kingdom of Sweden (Europe)
    ("the kingdom of sweden", Alpha2::SE),
    #[cfg(feature = "sg")] // The Republic of Singapore (Asia)
    ("the republic of singapore", Alpha2::SG),
    #[cfg(feature = "sh")] // Saint Helena, Ascension and Tristan da Cunha (Africa)
    ("saint helena, ascension and tristan da cunha", Alpha2::SH),
    #[cfg(feature = "si")] // The Republic of Slovenia (Europe)
    ("the republic of slovenia", Alpha2::SI),
    #[cfg(feature = "sj")] // Svalbard and Jan Mayen (Europe)
    ("svalbard and jan mayen", Alpha2::SJ),
    #[cfg(feature = "sk")] // The Slovak Republic (Europe)
    ("the slovak republic", Alpha2::SK),
    #[cfg(feature = "sl")] // The Republic of Sierra Leone (Africa)
    ("the republic of sierra leone", Alpha2::SL),
    #[cfg(feature = "sm")] // The Republic of San Marino (Europe)
    ("the republic of san marino", Alpha2::SM),
    #[cfg(feature = "sn")] // The Republic of Senegal (Africa)
    ("the republic of senegal", Alpha2::SN),
    #[cfg(feature = "so")] // The Federal Republic of Somalia (Africa)
    ("the federal republic of somalia", Alpha2::SO),
    #[cfg(feature = "sr")] // The Republic of Suriname (Americas)
    ("the republic of suriname", Alpha2::SR),
    #[cfg(feature = "ss")] // The Republic of South Sudan (Africa)
    ("the republic of south sudan", Alpha2::SS),
    #[cfg(feature = "st")] // The Democratic Republic of São Tomé and Príncipe (Africa)
    ("the democratic republic of são tomé and príncipe", Alpha2::ST),
    #[cfg(feature = "sv")] // The Republic of El Salvador (Americas)
    ("the republic of el salvador", Alpha2::SV),
    #[cfg(feature = "sx")] // Sint Maarten (Americas)
    ("sint maarten", Alpha2::SX),
    #[cfg(feature = "sy")] // The Syrian Arab Republic (Asia)
    ("the syrian arab republic", Alpha2::SY),
    #[cfg(feature = "sz")] // The Kingdom of Eswatini (Africa)
    ("the kingdom of eswatini", Alpha2::SZ),
    #[cfg(feature = "tc")] // The Turks and Caicos Islands (Americas)
    ("the turks and caicos islands", Alpha2::TC),
    #[cfg(feature = "td")] // The Republic of Chad (Africa)
    ("the republic of chad", Alpha2::TD),
    #[cfg(feature = "tf")] // The French Southern and Antarctic Lands (Africa)
    ("the french southern and antarctic lands", Alpha2::TF),
    #[cfg(feature = "tg")] // The Togolese Republic (Africa)
    ("the togolese republic", Alpha2::TG),
    #[cfg(feature = "th")] // The Kingdom of Thailand (Asia)
    ("the kingdom of thailand", Alpha2::TH),
    #[cfg(feature = "tj")] // The Republic of Tajikistan (Asia)
    ("the republic of tajikistan", Alpha2::TJ),
    #[cfg(feature = "tk")] // Tokelau (Oceania)
    ("tokelau", Alpha2::TK),
    #[cfg(feature = "tl")] // The Democratic Republic of Timor-Leste (Asia)
    ("the democratic republic of timor-leste", Alpha2::TL),
    #[cfg(feature = "tm")] // Turkmenistan (Asia)
    ("turkmenistan", Alpha2::TM),
    #[cfg(feature = "tn")] // The Republic of Tunisia (Africa)
    ("the republic of tunisia", Alpha2::TN),
    #[cfg(feature = "to")] // The Kingdom of Tonga (Oceania)
    ("the kingdom of tonga", Alpha2::TO),
    #[cfg(feature = "tr")] // The Republic of Turkey (Asia)
    ("the republic of turkey", Alpha2::TR),
    #[cfg(feature = "tt")] // The Republic of Trinidad and Tobago (Americas)
    ("the republic of trinidad and tobago", Alpha2::TT),
    #[cfg(feature = "tv")] // Tuvalu (Oceania)
    ("tuvalu", Alpha2::TV),
    #[cfg(feature = "tw")] // The Republic of China (Asia)
    ("the republic of china", Alpha2::TW),
    #[cfg(feature = "tz")] // The United Republic of Tanzania (Africa)
    ("the united republic of tanzania", Alpha2::TZ),
    #[cfg(feature = "ua")] // Ukraine (Europe)
    ("ukraine", Alpha2::UA),
    #[cfg(feature = "ug")] // The Republic of Uganda (Africa)
    ("the republic of uganda", Alpha2::UG),
    #[cfg(feature = "um")] // United States Minor Outlying Islands (Americas)
    ("united states minor outlying islands", Alpha2::UM),
    #[cfg(feature = "us")] // The United States of America (Americas)
    ("the united states of america", Alpha2::US),
    #[cfg(feature = "uy")] // The Oriental Republic of Uruguay (Americas)
    ("the oriental republic of uruguay", Alpha2::UY),
    #[cfg(feature = "uz")] // The Republic of Uzbekistan (Asia)
    ("the republic of uzbekistan", Alpha2::UZ),
    #[cfg(feature = "va")] // The Holy See (Europe)
    ("the holy see", Alpha2::VA),
    #[cfg(feature = "vc")] // Saint Vincent and the Grenadines (Americas)
    ("saint vincent and the grenadines", Alpha2::VC),
    #[cfg(feature = "ve")] // The Bolivarian Republic of Venezuela (Americas)
    ("the bolivarian republic of venezuela", Alpha2::VE),
    #[cfg(feature = "vg")] // The Virgin Islands (Americas)
    ("the virgin islands", Alpha2::VG),
    #[cfg(feature = "vi")] // The Virgin Islands of the United States (Americas)
    ("the virgin islands of the united states", Alpha2::VI),
    #[cfg(feature = "vn")] // The Socialist Republic of Viet Nam (Asia)
    ("the socialist republic of viet nam", Alpha2::VN),
    #[cfg(feature = "vu")] // The Republic of Vanuatu (Oceania)
    ("the republic of vanuatu", Alpha2::VU),
    #[cfg(feature = "wf")] // The Territory of the Wallis and Futuna Islands (Oceania)
    ("the territory of the wallis and futuna islands", Alpha2::WF),
    #[cfg(feature = "ws")] // The Independent State of Samoa (Oceania)
    ("the independent state of samoa", Alpha2::WS),
    #[cfg(feature = "ye")] // The Republic of Yemen (Asia)
    ("the republic of yemen", Alpha2::YE),
    #[cfg(feature = "yt")] // The Department of Mayotte (Africa)
    ("the department of mayotte", Alpha2::YT),
    #[cfg(feature = "za")] // The Republic of South Africa (Africa)
    ("the republic of south africa", Alpha2::ZA),
    #[cfg(feature = "zm")] // The Republic of Zambia (Africa)
    ("the republic of zambia", Alpha2::ZM),
    #[cfg(feature = "zw")] // The Republic of Zimbabwe (Africa)
    ("the republic of zimbabwe", Alpha2::ZW),
]);}
lazy_static! { pub static ref SUPPORTED_COUNTRY_CODE: HashMap<usize, Alpha2> = HashMap::from([
    #[cfg(feature = "ad")] // The Principality of Andorra (Europe)
    (376, Alpha2::AD),
    #[cfg(feature = "ae")] // The United Arab Emirates (Asia)
    (971, Alpha2::AE),
    #[cfg(feature = "af")] // The Islamic Republic of Afghanistan (Asia)
    (93, Alpha2::AF),
    #[cfg(feature = "ag")] // Antigua and Barbuda (Americas)
    (1, Alpha2::AG),
    #[cfg(feature = "ai")] // Anguilla (Americas)
    (1, Alpha2::AI),
    #[cfg(feature = "al")] // The Republic of Albania (Europe)
    (355, Alpha2::AL),
    #[cfg(feature = "am")] // The Republic of Armenia (Asia)
    (374, Alpha2::AM),
    #[cfg(feature = "ao")] // The Republic of Angola (Africa)
    (244, Alpha2::AO),
    #[cfg(feature = "aq")] // Antarctica
    (672, Alpha2::AQ),
    #[cfg(feature = "ar")] // The Argentine Republic (Americas)
    (54, Alpha2::AR),
    #[cfg(feature = "as")] // The Territory of American Samoa (Oceania)
    (1, Alpha2::AS),
    #[cfg(feature = "at")] // The Republic of Austria (Europe)
    (43, Alpha2::AT),
    #[cfg(feature = "au")] // The Commonwealth of Australia (Oceania)
    (61, Alpha2::AU),
    #[cfg(feature = "aw")] // Aruba (Americas)
    (297, Alpha2::AW),
    #[cfg(feature = "ax")] // Åland (Europe)
    (358, Alpha2::AX),
    #[cfg(feature = "az")] // The Republic of Azerbaijan (Asia)
    (994, Alpha2::AZ),
    #[cfg(feature = "ba")] // Bosnia and Herzegovina (Europe)
    (387, Alpha2::BA),
    #[cfg(feature = "bb")] // Barbados (Americas)
    (1, Alpha2::BB),
    #[cfg(feature = "bd")] // The People's Republic of Bangladesh (Asia)
    (880, Alpha2::BD),
    #[cfg(feature = "be")] // The Kingdom of Belgium (Europe)
    (32, Alpha2::BE),
    #[cfg(feature = "bf")] // Burkina Faso (Africa)
    (226, Alpha2::BF),
    #[cfg(feature = "bg")] // The Republic of Bulgaria (Europe)
    (359, Alpha2::BG),
    #[cfg(feature = "bh")] // The Kingdom of Bahrain (Asia)
    (973, Alpha2::BH),
    #[cfg(feature = "bi")] // The Republic of Burundi (Africa)
    (257, Alpha2::BI),
    #[cfg(feature = "bj")] // The Republic of Benin (Africa)
    (229, Alpha2::BJ),
    #[cfg(feature = "bl")] // The Collectivity of Saint-Barthélemy (Americas)
    (590, Alpha2::BL),
    #[cfg(feature = "bm")] // Bermuda (Americas)
    (1, Alpha2::BM),
    #[cfg(feature = "bn")] // The Nation of Brunei, the Abode of Peace (Asia)
    (673, Alpha2::BN),
    #[cfg(feature = "bo")] // The Plurinational State of Bolivia (Americas)
    (591, Alpha2::BO),
    #[cfg(feature = "bq")] // Bonaire, Sint Eustatius and Saba (Americas)
    (599, Alpha2::BQ),
    #[cfg(feature = "br")] // The Federative Republic of Brazil (Americas)
    (55, Alpha2::BR),
    #[cfg(feature = "bs")] // The Commonwealth of The Bahamas (Americas)
    (1, Alpha2::BS),
    #[cfg(feature = "bt")] // The Kingdom of Bhutan (Asia)
    (975, Alpha2::BT),
    #[cfg(feature = "bv")] // Bouvet Island
    (47, Alpha2::BV),
    #[cfg(feature = "bw")] // The Republic of Botswana (Africa)
    (267, Alpha2::BW),
    #[cfg(feature = "by")] // The Republic of Belarus (Europe)
    (375, Alpha2::BY),
    #[cfg(feature = "bz")] // Belize (Americas)
    (501, Alpha2::BZ),
    #[cfg(feature = "ca")] // Canada (Americas)
    (1, Alpha2::CA),
    #[cfg(feature = "cc")] // The Territory of Cocos (Keeling) Islands (Oceania)
    (61, Alpha2::CC),
    #[cfg(feature = "cd")] // The Democratic Republic of the Congo (Africa)
    (243, Alpha2::CD),
    #[cfg(feature = "cf")] // The Central African Republic (Africa)
    (236, Alpha2::CF),
    #[cfg(feature = "cg")] // The Republic of the Congo (Africa)
    (242, Alpha2::CG),
    #[cfg(feature = "ch")] // The Swiss Confederation (Europe)
    (41, Alpha2::CH),
    #[cfg(feature = "ci")] // The Republic of Côte d'Ivoire (Africa)
    (225, Alpha2::CI),
    #[cfg(feature = "ck")] // The Cook Islands (Oceania)
    (682, Alpha2::CK),
    #[cfg(feature = "cl")] // The Republic of Chile (Americas)
    (56, Alpha2::CL),
    #[cfg(feature = "cm")] // The Republic of Cameroon (Africa)
    (237, Alpha2::CM),
    #[cfg(feature = "cn")] // The People's Republic of China (Asia)
    (86, Alpha2::CN),
    #[cfg(feature = "co")] // The Republic of Colombia (Americas)
    (57, Alpha2::CO),
    #[cfg(feature = "cr")] // The Republic of Costa Rica (Americas)
    (506, Alpha2::CR),
    #[cfg(feature = "cu")] // The Republic of Cuba (Americas)
    (53, Alpha2::CU),
    #[cfg(feature = "cv")] // The Republic of Cabo Verde (Africa)
    (238, Alpha2::CV),
    #[cfg(feature = "cw")] // The Country of Curaçao (Americas)
    (599, Alpha2::CW),
    #[cfg(feature = "cx")] // The Territory of Christmas Island (Oceania)
    (61, Alpha2::CX),
    #[cfg(feature = "cy")] // The Republic of Cyprus (Asia)
    (357, Alpha2::CY),
    #[cfg(feature = "cz")] // The Czech Republic (Europe)
    (420, Alpha2::CZ),
    #[cfg(feature = "de")] // The Federal Republic of Germany (Europe)
    (49, Alpha2::DE),
    #[cfg(feature = "dj")] // The Republic of Djibouti (Africa)
    (253, Alpha2::DJ),
    #[cfg(feature = "dk")] // The Kingdom of Denmark (Europe)
    (45, Alpha2::DK),
    #[cfg(feature = "dm")] // The Commonwealth of Dominica (Americas)
    (1, Alpha2::DM),
    #[cfg(feature = "do")] // The Dominican Republic (Americas)
    (1, Alpha2::DO),
    #[cfg(feature = "dz")] // The People's Democratic Republic of Algeria (Africa)
    (213, Alpha2::DZ),
    #[cfg(feature = "ec")] // The Republic of Ecuador (Americas)
    (593, Alpha2::EC),
    #[cfg(feature = "ee")] // The Republic of Estonia (Europe)
    (372, Alpha2::EE),
    #[cfg(feature = "eg")] // The Arab Republic of Egypt (Africa)
    (20, Alpha2::EG),
    #[cfg(feature = "eh")] // The Sahrawi Arab Democratic Republic (Africa)
    (212, Alpha2::EH),
    #[cfg(feature = "er")] // The State of Eritrea (Africa)
    (291, Alpha2::ER),
    #[cfg(feature = "es")] // The Kingdom of Spain (Europe)
    (34, Alpha2::ES),
    #[cfg(feature = "et")] // The Federal Democratic Republic of Ethiopia (Africa)
    (251, Alpha2::ET),
    #[cfg(feature = "fi")] // The Republic of Finland (Europe)
    (358, Alpha2::FI),
    #[cfg(feature = "fj")] // The Republic of Fiji (Oceania)
    (679, Alpha2::FJ),
    #[cfg(feature = "fk")] // The Falkland Islands (Americas)
    (500, Alpha2::FK),
    #[cfg(feature = "fm")] // The Federated States of Micronesia (Oceania)
    (691, Alpha2::FM),
    #[cfg(feature = "fo")] // The Faroe Islands (Europe)
    (298, Alpha2::FO),
    #[cfg(feature = "fr")] // The French Republic (Europe)
    (33, Alpha2::FR),
    #[cfg(feature = "ga")] // The Gabonese Republic (Africa)
    (241, Alpha2::GA),
    #[cfg(feature = "gb")] // The United Kingdom of Great Britain and Northern Ireland (Europe)
    (44, Alpha2::GB),
    #[cfg(feature = "gd")] // Grenada (Americas)
    (1, Alpha2::GD),
    #[cfg(feature = "ge")] // Georgia (Asia)
    (995, Alpha2::GE),
    #[cfg(feature = "gf")] // Guyane (Americas)
    (594, Alpha2::GF),
    #[cfg(feature = "gg")] // The Bailiwick of Guernsey (Europe)
    (44, Alpha2::GG),
    #[cfg(feature = "gh")] // The Republic of Ghana (Africa)
    (233, Alpha2::GH),
    #[cfg(feature = "gi")] // Gibraltar (Europe)
    (350, Alpha2::GI),
    #[cfg(feature = "gl")] // Kalaallit Nunaat (Americas)
    (299, Alpha2::GL),
    #[cfg(feature = "gm")] // The Republic of The Gambia (Africa)
    (220, Alpha2::GM),
    #[cfg(feature = "gn")] // The Republic of Guinea (Africa)
    (224, Alpha2::GN),
    #[cfg(feature = "gp")] // Guadeloupe (Americas)
    (590, Alpha2::GP),
    #[cfg(feature = "gq")] // The Republic of Equatorial Guinea (Africa)
    (240, Alpha2::GQ),
    #[cfg(feature = "gr")] // The Hellenic Republic (Europe)
    (30, Alpha2::GR),
    #[cfg(feature = "gs")] // South Georgia and the South Sandwich Islands (Americas)
    (500, Alpha2::GS),
    #[cfg(feature = "gt")] // The Republic of Guatemala (Americas)
    (502, Alpha2::GT),
    #[cfg(feature = "gu")] // The Territory of Guam (Oceania)
    (1, Alpha2::GU),
    #[cfg(feature = "gw")] // The Republic of Guinea-Bissau (Africa)
    (245, Alpha2::GW),
    #[cfg(feature = "gy")] // The Co-operative Republic of Guyana (Americas)
    (592, Alpha2::GY),
    #[cfg(feature = "hk")] // The Hong Kong Special Administrative Region of China (Asia)
    (852, Alpha2::HK),
    #[cfg(feature = "hm")] // The Territory of Heard Island and McDonald Islands
    (672, Alpha2::HM),
    #[cfg(feature = "hn")] // The Republic of Honduras (Americas)
    (504, Alpha2::HN),
    #[cfg(feature = "hr")] // The Republic of Croatia (Europe)
    (385, Alpha2::HR),
    #[cfg(feature = "ht")] // The Republic of Haiti (Americas)
    (509, Alpha2::HT),
    #[cfg(feature = "hu")] // Hungary (Europe)
    (36, Alpha2::HU),
    #[cfg(feature = "id")] // The Republic of Indonesia (Asia)
    (62, Alpha2::ID),
    #[cfg(feature = "ie")] // Ireland (Europe)
    (353, Alpha2::IE),
    #[cfg(feature = "il")] // The State of Israel (Asia)
    (972, Alpha2::IL),
    #[cfg(feature = "im")] // The Isle of Man (Europe)
    (44, Alpha2::IM),
    #[cfg(feature = "in")] // The Republic of India (Asia)
    (91, Alpha2::IN),
    #[cfg(feature = "io")] // The British Indian Ocean Territory (Africa)
    (246, Alpha2::IO),
    #[cfg(feature = "iq")] // The Republic of Iraq (Asia)
    (964, Alpha2::IQ),
    #[cfg(feature = "ir")] // The Islamic Republic of Iran (Asia)
    (98, Alpha2::IR),
    #[cfg(feature = "is")] // Iceland (Europe)
    (354, Alpha2::IS),
    #[cfg(feature = "it")] // The Italian Republic (Europe)
    (39, Alpha2::IT),
    #[cfg(feature = "je")] // The Bailiwick of Jersey (Europe)
    (44, Alpha2::JE),
    #[cfg(feature = "jm")] // Jamaica (Americas)
    (1, Alpha2::JM),
    #[cfg(feature = "jo")] // The Hashemite Kingdom of Jordan (Asia)
    (962, Alpha2::JO),
    #[cfg(feature = "jp")] // Japan (Asia)
    (81, Alpha2::JP),
    #[cfg(feature = "ke")] // The Republic of Kenya (Africa)
    (254, Alpha2::KE),
    #[cfg(feature = "kg")] // The Kyrgyz Republic (Asia)
    (996, Alpha2::KG),
    #[cfg(feature = "kh")] // The Kingdom of Cambodia (Asia)
    (855, Alpha2::KH),
    #[cfg(feature = "ki")] // The Republic of Kiribati (Oceania)
    (686, Alpha2::KI),
    #[cfg(feature = "km")] // The Union of the Comoros (Africa)
    (269, Alpha2::KM),
    #[cfg(feature = "kn")] // Saint Kitts and Nevis (Americas)
    (1, Alpha2::KN),
    #[cfg(feature = "kp")] // The Democratic People's Republic of Korea (Asia)
    (850, Alpha2::KP),
    #[cfg(feature = "kr")] // The Republic of Korea (Asia)
    (82, Alpha2::KR),
    #[cfg(feature = "kw")] // The State of Kuwait (Asia)
    (965, Alpha2::KW),
    #[cfg(feature = "ky")] // The Cayman Islands (Americas)
    (1, Alpha2::KY),
    #[cfg(feature = "kz")] // The Republic of Kazakhstan (Asia)
    (7, Alpha2::KZ),
    #[cfg(feature = "la")] // The Lao People's Democratic Republic (Asia)
    (856, Alpha2::LA),
    #[cfg(feature = "lb")] // The Lebanese Republic (Asia)
    (961, Alpha2::LB),
    #[cfg(feature = "lc")] // Saint Lucia (Americas)
    (1, Alpha2::LC),
    #[cfg(feature = "li")] // The Principality of Liechtenstein (Europe)
    (423, Alpha2::LI),
    #[cfg(feature = "lk")] // The Democratic Socialist Republic of Sri Lanka (Asia)
    (94, Alpha2::LK),
    #[cfg(feature = "lr")] // The Republic of Liberia (Africa)
    (231, Alpha2::LR),
    #[cfg(feature = "ls")] // The Kingdom of Lesotho (Africa)
    (266, Alpha2::LS),
    #[cfg(feature = "lt")] // The Republic of Lithuania (Europe)
    (370, Alpha2::LT),
    #[cfg(feature = "lu")] // The Grand Duchy of Luxembourg (Europe)
    (352, Alpha2::LU),
    #[cfg(feature = "lv")] // The Republic of Latvia (Europe)
    (371, Alpha2::LV),
    #[cfg(feature = "ly")] // The State of Libya (Africa)
    (218, Alpha2::LY),
    #[cfg(feature = "ma")] // The Kingdom of Morocco (Africa)
    (212, Alpha2::MA),
    #[cfg(feature = "mc")] // The Principality of Monaco (Europe)
    (377, Alpha2::MC),
    #[cfg(feature = "md")] // The Republic of Moldova (Europe)
    (373, Alpha2::MD),
    #[cfg(feature = "me")] // Montenegro (Europe)
    (382, Alpha2::ME),
    #[cfg(feature = "mf")] // The Collectivity of Saint-Martin (Americas)
    (590, Alpha2::MF),
    #[cfg(feature = "mg")] // The Republic of Madagascar (Africa)
    (261, Alpha2::MG),
    #[cfg(feature = "mh")] // The Republic of the Marshall Islands (Oceania)
    (692, Alpha2::MH),
    #[cfg(feature = "mk")] // The Republic of North Macedonia (Europe)
    (389, Alpha2::MK),
    #[cfg(feature = "ml")] // The Republic of Mali (Africa)
    (223, Alpha2::ML),
    #[cfg(feature = "mm")] // The Republic of the Union of Myanmar (Asia)
    (95, Alpha2::MM),
    #[cfg(feature = "mn")] // Mongolia (Asia)
    (976, Alpha2::MN),
    #[cfg(feature = "mo")] // The Macao Special Administrative Region of China (Asia)
    (853, Alpha2::MO),
    #[cfg(feature = "mp")] // The Commonwealth of the Northern Mariana Islands (Oceania)
    (1, Alpha2::MP),
    #[cfg(feature = "mq")] // Martinique (Americas)
    (596, Alpha2::MQ),
    #[cfg(feature = "mr")] // The Islamic Republic of Mauritania (Africa)
    (222, Alpha2::MR),
    #[cfg(feature = "ms")] // Montserrat (Americas)
    (1, Alpha2::MS),
    #[cfg(feature = "mt")] // The Republic of Malta (Europe)
    (356, Alpha2::MT),
    #[cfg(feature = "mu")] // The Republic of Mauritius (Africa)
    (230, Alpha2::MU),
    #[cfg(feature = "mv")] // The Republic of Maldives (Asia)
    (960, Alpha2::MV),
    #[cfg(feature = "mw")] // The Republic of Malawi (Africa)
    (265, Alpha2::MW),
    #[cfg(feature = "mx")] // The United Mexican States (Americas)
    (52, Alpha2::MX),
    #[cfg(feature = "my")] // Malaysia (Asia)
    (60, Alpha2::MY),
    #[cfg(feature = "mz")] // The Republic of Mozambique (Africa)
    (258, Alpha2::MZ),
    #[cfg(feature = "na")] // The Republic of Namibia (Africa)
    (264, Alpha2::NA),
    #[cfg(feature = "nc")] // New Caledonia (Oceania)
    (687, Alpha2::NC),
    #[cfg(feature = "ne")] // The Republic of the Niger (Africa)
    (227, Alpha2::NE),
    #[cfg(feature = "nf")] // The Territory of Norfolk Island (Oceania)
    (672, Alpha2::NF),
    #[cfg(feature = "ng")] // The Federal Republic of Nigeria (Africa)
    (234, Alpha2::NG),
    #[cfg(feature = "ni")] // The Republic of Nicaragua (Americas)
    (505, Alpha2::NI),
    #[cfg(feature = "nl")] // The Kingdom of the Netherlands (Europe)
    (31, Alpha2::NL),
    #[cfg(feature = "no")] // The Kingdom of Norway (Europe)
    (47, Alpha2::NO),
    #[cfg(feature = "np")] // The Federal Democratic Republic of Nepal (Asia)
    (977, Alpha2::NP),
    #[cfg(feature = "nr")] // The Republic of Nauru (Oceania)
    (674, Alpha2::NR),
    #[cfg(feature = "nu")] // Niue (Oceania)
    (683, Alpha2::NU),
    #[cfg(feature = "nz")] // New Zealand (Oceania)
    (64, Alpha2::NZ),
    #[cfg(feature = "om")] // The Sultanate of Oman (Asia)
    (968, Alpha2::OM),
    #[cfg(feature = "pa")] // The Republic of Panamá (Americas)
    (507, Alpha2::PA),
    #[cfg(feature = "pe")] // The Republic of Perú (Americas)
    (51, Alpha2::PE),
    #[cfg(feature = "pf")] // French Polynesia (Oceania)
    (689, Alpha2::PF),
    #[cfg(feature = "pg")] // The Independent State of Papua New Guinea (Oceania)
    (675, Alpha2::PG),
    #[cfg(feature = "ph")] // The Republic of the Philippines (Asia)
    (63, Alpha2::PH),
    #[cfg(feature = "pk")] // The Islamic Republic of Pakistan (Asia)
    (92, Alpha2::PK),
    #[cfg(feature = "pl")] // The Republic of Poland (Europe)
    (48, Alpha2::PL),
    #[cfg(feature = "pm")] // The Overseas Collectivity of Saint-Pierre and Miquelon (Americas)
    (508, Alpha2::PM),
    #[cfg(feature = "pn")] // The Pitcairn, Henderson, Ducie and Oeno Islands (Oceania)
    (64, Alpha2::PN),
    #[cfg(feature = "pr")] // The Commonwealth of Puerto Rico (Americas)
    (1, Alpha2::PR),
    #[cfg(feature = "ps")] // The State of Palestine (Asia)
    (970, Alpha2::PS),
    #[cfg(feature = "pt")] // The Portuguese Republic (Europe)
    (351, Alpha2::PT),
    #[cfg(feature = "pw")] // The Republic of Palau (Oceania)
    (680, Alpha2::PW),
    #[cfg(feature = "py")] // The Republic of Paraguay (Americas)
    (595, Alpha2::PY),
    #[cfg(feature = "qa")] // The State of Qatar (Asia)
    (974, Alpha2::QA),
    #[cfg(feature = "re")] // Réunion (Africa)
    (262, Alpha2::RE),
    #[cfg(feature = "ro")] // Romania (Europe)
    (40, Alpha2::RO),
    #[cfg(feature = "rs")] // The Republic of Serbia (Europe)
    (381, Alpha2::RS),
    #[cfg(feature = "ru")] // The Russian Federation (Europe)
    (7, Alpha2::RU),
    #[cfg(feature = "rw")] // The Republic of Rwanda (Africa)
    (250, Alpha2::RW),
    #[cfg(feature = "sa")] // The Kingdom of Saudi Arabia (Asia)
    (966, Alpha2::SA),
    #[cfg(feature = "sb")] // The Solomon Islands (Oceania)
    (677, Alpha2::SB),
    #[cfg(feature = "sc")] // The Republic of Seychelles (Africa)
    (248, Alpha2::SC),
    #[cfg(feature = "sd")] // The Republic of the Sudan (Africa)
    (249, Alpha2::SD),
    #[cfg(feature = "se")] // The Kingdom of Sweden (Europe)
    (46, Alpha2::SE),
    #[cfg(feature = "sg")] // The Republic of Singapore (Asia)
    (65, Alpha2::SG),
    #[cfg(feature = "sh")] // Saint Helena, Ascension and Tristan da Cunha (Africa)
    (290, Alpha2::SH),
    #[cfg(feature = "si")] // The Republic of Slovenia (Europe)
    (386, Alpha2::SI),
    #[cfg(feature = "sj")] // Svalbard and Jan Mayen (Europe)
    (47, Alpha2::SJ),
    #[cfg(feature = "sk")] // The Slovak Republic (Europe)
    (421, Alpha2::SK),
    #[cfg(feature = "sl")] // The Republic of Sierra Leone (Africa)
    (232, Alpha2::SL),
    #[cfg(feature = "sm")] // The Republic of San Marino (Europe)
    (378, Alpha2::SM),
    #[cfg(feature = "sn")] // The Republic of Senegal (Africa)
    (221, Alpha2::SN),
    #[cfg(feature = "so")] // The Federal Republic of Somalia (Africa)
    (252, Alpha2::SO),
    #[cfg(feature = "sr")] // The Republic of Suriname (Americas)
    (597, Alpha2::SR),
    #[cfg(feature = "ss")] // The Republic of South Sudan (Africa)
    (211, Alpha2::SS),
    #[cfg(feature = "st")] // The Democratic Republic of São Tomé and Príncipe (Africa)
    (239, Alpha2::ST),
    #[cfg(feature = "sv")] // The Republic of El Salvador (Americas)
    (503, Alpha2::SV),
    #[cfg(feature = "sx")] // Sint Maarten (Americas)
    (1, Alpha2::SX),
    #[cfg(feature = "sy")] // The Syrian Arab Republic (Asia)
    (963, Alpha2::SY),
    #[cfg(feature = "sz")] // The Kingdom of Eswatini (Africa)
    (268, Alpha2::SZ),
    #[cfg(feature = "tc")] // The Turks and Caicos Islands (Americas)
    (1, Alpha2::TC),
    #[cfg(feature = "td")] // The Republic of Chad (Africa)
    (235, Alpha2::TD),
    #[cfg(feature = "tf")] // The French Southern and Antarctic Lands (Africa)
    (262, Alpha2::TF),
    #[cfg(feature = "tg")] // The Togolese Republic (Africa)
    (228, Alpha2::TG),
    #[cfg(feature = "th")] // The Kingdom of Thailand (Asia)
    (66, Alpha2::TH),
    #[cfg(feature = "tj")] // The Republic of Tajikistan (Asia)
    (992, Alpha2::TJ),
    #[cfg(feature = "tk")] // Tokelau (Oceania)
    (690, Alpha2::TK),
    #[cfg(feature = "tl")] // The Democratic Republic of Timor-Leste (Asia)
    (670, Alpha2::TL),
    #[cfg(feature = "tm")] // Turkmenistan (Asia)
    (993, Alpha2::TM),
    #[cfg(feature = "tn")] // The Republic of Tunisia (Africa)
    (216, Alpha2::TN),
    #[cfg(feature = "to")] // The Kingdom of Tonga (Oceania)
    (676, Alpha2::TO),
    #[cfg(feature = "tr")] // The Republic of Turkey (Asia)
    (90, Alpha2::TR),
    #[cfg(feature = "tt")] // The Republic of Trinidad and Tobago (Americas)
    (1, Alpha2::TT),
    #[cfg(feature = "tv")] // Tuvalu (Oceania)
    (688, Alpha2::TV),
    #[cfg(feature = "tw")] // The Republic of China (Asia)
    (886, Alpha2::TW),
    #[cfg(feature = "tz")] // The United Republic of Tanzania (Africa)
    (255, Alpha2::TZ),
    #[cfg(feature = "ua")] // Ukraine (Europe)
    (380, Alpha2::UA),
    #[cfg(feature = "ug")] // The Republic of Uganda (Africa)
    (256, Alpha2::UG),
    #[cfg(feature = "um")] // United States Minor Outlying Islands (Americas)
    (1, Alpha2::UM),
    #[cfg(feature = "us")] // The United States of America (Americas)
    (1, Alpha2::US),
    #[cfg(feature = "uy")] // The Oriental Republic of Uruguay (Americas)
    (598, Alpha2::UY),
    #[cfg(feature = "uz")] // The Republic of Uzbekistan (Asia)
    (998, Alpha2::UZ),
    #[cfg(feature = "va")] // The Holy See (Europe)
    (39, Alpha2::VA),
    #[cfg(feature = "vc")] // Saint Vincent and the Grenadines (Americas)
    (1, Alpha2::VC),
    #[cfg(feature = "ve")] // The Bolivarian Republic of Venezuela (Americas)
    (58, Alpha2::VE),
    #[cfg(feature = "vg")] // The Virgin Islands (Americas)
    (1, Alpha2::VG),
    #[cfg(feature = "vi")] // The Virgin Islands of the United States (Americas)
    (1, Alpha2::VI),
    #[cfg(feature = "vn")] // The Socialist Republic of Viet Nam (Asia)
    (84, Alpha2::VN),
    #[cfg(feature = "vu")] // The Republic of Vanuatu (Oceania)
    (678, Alpha2::VU),
    #[cfg(feature = "wf")] // The Territory of the Wallis and Futuna Islands (Oceania)
    (681, Alpha2::WF),
    #[cfg(feature = "ws")] // The Independent State of Samoa (Oceania)
    (685, Alpha2::WS),
    #[cfg(feature = "ye")] // The Republic of Yemen (Asia)
    (967, Alpha2::YE),
    #[cfg(feature = "yt")] // The Department of Mayotte (Africa)
    (262, Alpha2::YT),
    #[cfg(feature = "za")] // The Republic of South Africa (Africa)
    (27, Alpha2::ZA),
    #[cfg(feature = "zm")] // The Republic of Zambia (Africa)
    (260, Alpha2::ZM),
    #[cfg(feature = "zw")] // The Republic of Zimbabwe (Africa)
    (263, Alpha2::ZW),
]);}
