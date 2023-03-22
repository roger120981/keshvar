// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/ioc.rs`)

#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
/// An enum containing `International Olympic Committee` (IOC) codes.
///
/// # Example
/// ```
/// use keshvar::{IOC, Alpha2};
///
/// assert_eq!(Ok(IOC::BER), IOC::try_from("ber"));
/// assert_eq!(IOC::BER.to_alpha2(), Alpha2::BM); // Bermuda (Americas)
/// ```
/// We usually need to convert [`Alpha2`](crate::Alpha2) to [`Country`](crate::Country) and use that object instead.
pub enum IOC {
    #[cfg(feature = "ad")]
    /// The Principality of Andorra (Europe)
    AND,
    #[cfg(feature = "ae")]
    /// The United Arab Emirates (Asia)
    UAE,
    #[cfg(feature = "af")]
    /// The Islamic Republic of Afghanistan (Asia)
    AFG,
    #[cfg(feature = "ag")]
    /// Antigua and Barbuda (Americas)
    ANT,
    #[cfg(feature = "al")]
    /// The Republic of Albania (Europe)
    ALB,
    #[cfg(feature = "am")]
    /// The Republic of Armenia (Asia)
    ARM,
    #[cfg(feature = "ao")]
    /// The Republic of Angola (Africa)
    ANG,
    #[cfg(feature = "ar")]
    /// The Argentine Republic (Americas)
    ARG,
    #[cfg(feature = "as")]
    /// The Territory of American Samoa (Oceania)
    ASA,
    #[cfg(feature = "at")]
    /// The Republic of Austria (Europe)
    AUT,
    #[cfg(feature = "au")]
    /// The Commonwealth of Australia (Oceania)
    AUS,
    #[cfg(feature = "aw")]
    /// Aruba (Americas)
    ARU,
    #[cfg(feature = "az")]
    /// The Republic of Azerbaijan (Asia)
    AZE,
    #[cfg(feature = "ba")]
    /// Bosnia and Herzegovina (Europe)
    BIH,
    #[cfg(feature = "bb")]
    /// Barbados (Americas)
    BAR,
    #[cfg(feature = "bd")]
    /// The People's Republic of Bangladesh (Asia)
    BAN,
    #[cfg(feature = "be")]
    /// The Kingdom of Belgium (Europe)
    BEL,
    #[cfg(feature = "bf")]
    /// Burkina Faso (Africa)
    BUR,
    #[cfg(feature = "bg")]
    /// The Republic of Bulgaria (Europe)
    BUL,
    #[cfg(feature = "bh")]
    /// The Kingdom of Bahrain (Asia)
    BRN,
    #[cfg(feature = "bi")]
    /// The Republic of Burundi (Africa)
    BDI,
    #[cfg(feature = "bj")]
    /// The Republic of Benin (Africa)
    BEN,
    #[cfg(feature = "bm")]
    /// Bermuda (Americas)
    BER,
    #[cfg(feature = "bn")]
    /// The Nation of Brunei, the Abode of Peace (Asia)
    BRU,
    #[cfg(feature = "bo")]
    /// The Plurinational State of Bolivia (Americas)
    BOL,
    #[cfg(feature = "br")]
    /// The Federative Republic of Brazil (Americas)
    BRA,
    #[cfg(feature = "bs")]
    /// The Commonwealth of The Bahamas (Americas)
    BAH,
    #[cfg(feature = "bt")]
    /// The Kingdom of Bhutan (Asia)
    BHU,
    #[cfg(feature = "bw")]
    /// The Republic of Botswana (Africa)
    BOT,
    #[cfg(feature = "by")]
    /// The Republic of Belarus (Europe)
    BLR,
    #[cfg(feature = "bz")]
    /// Belize (Americas)
    BIZ,
    #[cfg(feature = "ca")]
    /// Canada (Americas)
    CAN,
    #[cfg(feature = "cd")]
    /// The Democratic Republic of the Congo (Africa)
    COD,
    #[cfg(feature = "cf")]
    /// The Central African Republic (Africa)
    CAF,
    #[cfg(feature = "cg")]
    /// The Republic of the Congo (Africa)
    CGO,
    #[cfg(feature = "ch")]
    /// The Swiss Confederation (Europe)
    SUI,
    #[cfg(feature = "ci")]
    /// The Republic of Côte d'Ivoire (Africa)
    CIV,
    #[cfg(feature = "ck")]
    /// The Cook Islands (Oceania)
    COK,
    #[cfg(feature = "cl")]
    /// The Republic of Chile (Americas)
    CHI,
    #[cfg(feature = "cm")]
    /// The Republic of Cameroon (Africa)
    CMR,
    #[cfg(feature = "cn")]
    /// The People's Republic of China (Asia)
    CHN,
    #[cfg(feature = "co")]
    /// The Republic of Colombia (Americas)
    COL,
    #[cfg(feature = "cr")]
    /// The Republic of Costa Rica (Americas)
    CRC,
    #[cfg(feature = "cu")]
    /// The Republic of Cuba (Americas)
    CUB,
    #[cfg(feature = "cv")]
    /// The Republic of Cabo Verde (Africa)
    CPV,
    #[cfg(feature = "cy")]
    /// The Republic of Cyprus (Asia)
    CYP,
    #[cfg(feature = "cz")]
    /// The Czech Republic (Europe)
    CZE,
    #[cfg(feature = "de")]
    /// The Federal Republic of Germany (Europe)
    GER,
    #[cfg(feature = "dj")]
    /// The Republic of Djibouti (Africa)
    DJI,
    #[cfg(feature = "dk")]
    /// The Kingdom of Denmark (Europe)
    DEN,
    #[cfg(feature = "dm")]
    /// The Commonwealth of Dominica (Americas)
    DMA,
    #[cfg(feature = "do")]
    /// The Dominican Republic (Americas)
    DOM,
    #[cfg(feature = "dz")]
    /// The People's Democratic Republic of Algeria (Africa)
    ALG,
    #[cfg(feature = "ec")]
    /// The Republic of Ecuador (Americas)
    ECU,
    #[cfg(feature = "ee")]
    /// The Republic of Estonia (Europe)
    EST,
    #[cfg(feature = "eg")]
    /// The Arab Republic of Egypt (Africa)
    EGY,
    #[cfg(feature = "er")]
    /// The State of Eritrea (Africa)
    ERI,
    #[cfg(feature = "es")]
    /// The Kingdom of Spain (Europe)
    ESP,
    #[cfg(feature = "et")]
    /// The Federal Democratic Republic of Ethiopia (Africa)
    ETH,
    #[cfg(feature = "fi")]
    /// The Republic of Finland (Europe)
    FIN,
    #[cfg(feature = "fj")]
    /// The Republic of Fiji (Oceania)
    FIJ,
    #[cfg(feature = "fm")]
    /// The Federated States of Micronesia (Oceania)
    FSM,
    #[cfg(feature = "fo")]
    /// The Faroe Islands (Europe)
    FRO,
    #[cfg(feature = "fr")]
    /// The French Republic (Europe)
    FRA,
    #[cfg(feature = "ga")]
    /// The Gabonese Republic (Africa)
    GAB,
    #[cfg(feature = "gb")]
    /// The United Kingdom of Great Britain and Northern Ireland (Europe)
    GBR,
    #[cfg(feature = "gd")]
    /// Grenada (Americas)
    GRN,
    #[cfg(feature = "ge")]
    /// Georgia (Asia)
    GEO,
    #[cfg(feature = "gh")]
    /// The Republic of Ghana (Africa)
    GHA,
    #[cfg(feature = "gm")]
    /// The Republic of The Gambia (Africa)
    GAM,
    #[cfg(feature = "gn")]
    /// The Republic of Guinea (Africa)
    GUI,
    #[cfg(feature = "gq")]
    /// The Republic of Equatorial Guinea (Africa)
    GEQ,
    #[cfg(feature = "gr")]
    /// The Hellenic Republic (Europe)
    GRE,
    #[cfg(feature = "gt")]
    /// The Republic of Guatemala (Americas)
    GUA,
    #[cfg(feature = "gu")]
    /// The Territory of Guam (Oceania)
    GUM,
    #[cfg(feature = "gw")]
    /// The Republic of Guinea-Bissau (Africa)
    GBS,
    #[cfg(feature = "gy")]
    /// The Co-operative Republic of Guyana (Americas)
    GUY,
    #[cfg(feature = "hk")]
    /// The Hong Kong Special Administrative Region of China (Asia)
    HKG,
    #[cfg(feature = "hn")]
    /// The Republic of Honduras (Americas)
    HON,
    #[cfg(feature = "hr")]
    /// The Republic of Croatia (Europe)
    CRO,
    #[cfg(feature = "ht")]
    /// The Republic of Haiti (Americas)
    HAI,
    #[cfg(feature = "hu")]
    /// Hungary (Europe)
    HUN,
    #[cfg(feature = "id")]
    /// The Republic of Indonesia (Asia)
    INA,
    #[cfg(feature = "ie")]
    /// Ireland (Europe)
    IRL,
    #[cfg(feature = "il")]
    /// The State of Israel (Asia)
    ISR,
    #[cfg(feature = "in")]
    /// The Republic of India (Asia)
    IND,
    #[cfg(feature = "iq")]
    /// The Republic of Iraq (Asia)
    IRQ,
    #[cfg(feature = "ir")]
    /// The Islamic Republic of Iran (Asia)
    IRI,
    #[cfg(feature = "is")]
    /// Iceland (Europe)
    ISL,
    #[cfg(feature = "it")]
    /// The Italian Republic (Europe)
    ITA,
    #[cfg(feature = "jm")]
    /// Jamaica (Americas)
    JAM,
    #[cfg(feature = "jo")]
    /// The Hashemite Kingdom of Jordan (Asia)
    JOR,
    #[cfg(feature = "jp")]
    /// Japan (Asia)
    JPN,
    #[cfg(feature = "ke")]
    /// The Republic of Kenya (Africa)
    KEN,
    #[cfg(feature = "kg")]
    /// The Kyrgyz Republic (Asia)
    KGZ,
    #[cfg(feature = "kh")]
    /// The Kingdom of Cambodia (Asia)
    CAM,
    #[cfg(feature = "ki")]
    /// The Republic of Kiribati (Oceania)
    KIR,
    #[cfg(feature = "km")]
    /// The Union of the Comoros (Africa)
    COM,
    #[cfg(feature = "kn")]
    /// Saint Kitts and Nevis (Americas)
    SKN,
    #[cfg(feature = "kp")]
    /// The Democratic People's Republic of Korea (Asia)
    PRK,
    #[cfg(feature = "kr")]
    /// The Republic of Korea (Asia)
    KOR,
    #[cfg(feature = "kw")]
    /// The State of Kuwait (Asia)
    KUW,
    #[cfg(feature = "ky")]
    /// The Cayman Islands (Americas)
    CAY,
    #[cfg(feature = "kz")]
    /// The Republic of Kazakhstan (Asia)
    KAZ,
    #[cfg(feature = "la")]
    /// The Lao People's Democratic Republic (Asia)
    LAO,
    #[cfg(feature = "lb")]
    /// The Lebanese Republic (Asia)
    LIB,
    #[cfg(feature = "lc")]
    /// Saint Lucia (Americas)
    LCA,
    #[cfg(feature = "li")]
    /// The Principality of Liechtenstein (Europe)
    LIE,
    #[cfg(feature = "lk")]
    /// The Democratic Socialist Republic of Sri Lanka (Asia)
    SRI,
    #[cfg(feature = "lr")]
    /// The Republic of Liberia (Africa)
    LBR,
    #[cfg(feature = "ls")]
    /// The Kingdom of Lesotho (Africa)
    LES,
    #[cfg(feature = "lt")]
    /// The Republic of Lithuania (Europe)
    LTU,
    #[cfg(feature = "lu")]
    /// The Grand Duchy of Luxembourg (Europe)
    LUX,
    #[cfg(feature = "lv")]
    /// The Republic of Latvia (Europe)
    LAT,
    #[cfg(feature = "ly")]
    /// The State of Libya (Africa)
    LBA,
    #[cfg(feature = "ma")]
    /// The Kingdom of Morocco (Africa)
    MAR,
    #[cfg(feature = "mc")]
    /// The Principality of Monaco (Europe)
    MON,
    #[cfg(feature = "md")]
    /// The Republic of Moldova (Europe)
    MDA,
    #[cfg(feature = "me")]
    /// Montenegro (Europe)
    MNE,
    #[cfg(feature = "mg")]
    /// The Republic of Madagascar (Africa)
    MAD,
    #[cfg(feature = "mh")]
    /// The Republic of the Marshall Islands (Oceania)
    MHL,
    #[cfg(feature = "mk")]
    /// The Republic of North Macedonia (Europe)
    MKD,
    #[cfg(feature = "ml")]
    /// The Republic of Mali (Africa)
    MLI,
    #[cfg(feature = "mm")]
    /// The Republic of the Union of Myanmar (Asia)
    MYA,
    #[cfg(feature = "mn")]
    /// Mongolia (Asia)
    MGL,
    #[cfg(feature = "mr")]
    /// The Islamic Republic of Mauritania (Africa)
    MTN,
    #[cfg(feature = "mt")]
    /// The Republic of Malta (Europe)
    MLT,
    #[cfg(feature = "mu")]
    /// The Republic of Mauritius (Africa)
    MRI,
    #[cfg(feature = "mv")]
    /// The Republic of Maldives (Asia)
    MDV,
    #[cfg(feature = "mw")]
    /// The Republic of Malawi (Africa)
    MAW,
    #[cfg(feature = "mx")]
    /// The United Mexican States (Americas)
    MEX,
    #[cfg(feature = "my")]
    /// Malaysia (Asia)
    MAS,
    #[cfg(feature = "mz")]
    /// The Republic of Mozambique (Africa)
    MOZ,
    #[cfg(feature = "na")]
    /// The Republic of Namibia (Africa)
    NAM,
    #[cfg(feature = "ne")]
    /// The Republic of the Niger (Africa)
    NIG,
    #[cfg(feature = "ng")]
    /// The Federal Republic of Nigeria (Africa)
    NGR,
    #[cfg(feature = "ni")]
    /// The Republic of Nicaragua (Americas)
    NCA,
    #[cfg(feature = "nl")]
    /// The Kingdom of the Netherlands (Europe)
    NED,
    #[cfg(feature = "no")]
    /// The Kingdom of Norway (Europe)
    NOR,
    #[cfg(feature = "np")]
    /// The Federal Democratic Republic of Nepal (Asia)
    NEP,
    #[cfg(feature = "nr")]
    /// The Republic of Nauru (Oceania)
    NRU,
    #[cfg(feature = "nz")]
    /// New Zealand (Oceania)
    NZL,
    #[cfg(feature = "om")]
    /// The Sultanate of Oman (Asia)
    OMA,
    #[cfg(feature = "pa")]
    /// The Republic of Panamá (Americas)
    PAN,
    #[cfg(feature = "pe")]
    /// The Republic of Perú (Americas)
    PER,
    #[cfg(feature = "pg")]
    /// The Independent State of Papua New Guinea (Oceania)
    PNG,
    #[cfg(feature = "ph")]
    /// The Republic of the Philippines (Asia)
    PHI,
    #[cfg(feature = "pk")]
    /// The Islamic Republic of Pakistan (Asia)
    PAK,
    #[cfg(feature = "pl")]
    /// The Republic of Poland (Europe)
    POL,
    #[cfg(feature = "pr")]
    /// The Commonwealth of Puerto Rico (Americas)
    PUR,
    #[cfg(feature = "ps")]
    /// The State of Palestine (Asia)
    PLE,
    #[cfg(feature = "pt")]
    /// The Portuguese Republic (Europe)
    POR,
    #[cfg(feature = "pw")]
    /// The Republic of Palau (Oceania)
    PLW,
    #[cfg(feature = "py")]
    /// The Republic of Paraguay (Americas)
    PAR,
    #[cfg(feature = "qa")]
    /// The State of Qatar (Asia)
    QAT,
    #[cfg(feature = "ro")]
    /// Romania (Europe)
    ROU,
    #[cfg(feature = "rs")]
    /// The Republic of Serbia (Europe)
    SRB,
    #[cfg(feature = "ru")]
    /// The Russian Federation (Europe)
    RUS,
    #[cfg(feature = "rw")]
    /// The Republic of Rwanda (Africa)
    RWA,
    #[cfg(feature = "sa")]
    /// The Kingdom of Saudi Arabia (Asia)
    KSA,
    #[cfg(feature = "sb")]
    /// The Solomon Islands (Oceania)
    SOL,
    #[cfg(feature = "sc")]
    /// The Republic of Seychelles (Africa)
    SEY,
    #[cfg(feature = "sd")]
    /// The Republic of the Sudan (Africa)
    SUD,
    #[cfg(feature = "se")]
    /// The Kingdom of Sweden (Europe)
    SWE,
    #[cfg(feature = "sg")]
    /// The Republic of Singapore (Asia)
    SGP,
    #[cfg(feature = "si")]
    /// The Republic of Slovenia (Europe)
    SLO,
    #[cfg(feature = "sk")]
    /// The Slovak Republic (Europe)
    SVK,
    #[cfg(feature = "sl")]
    /// The Republic of Sierra Leone (Africa)
    SLE,
    #[cfg(feature = "sm")]
    /// The Republic of San Marino (Europe)
    SMR,
    #[cfg(feature = "sn")]
    /// The Republic of Senegal (Africa)
    SEN,
    #[cfg(feature = "so")]
    /// The Federal Republic of Somalia (Africa)
    SOM,
    #[cfg(feature = "sr")]
    /// The Republic of Suriname (Americas)
    SUR,
    #[cfg(feature = "st")]
    /// The Democratic Republic of São Tomé and Príncipe (Africa)
    STP,
    #[cfg(feature = "sv")]
    /// The Republic of El Salvador (Americas)
    ESA,
    #[cfg(feature = "sy")]
    /// The Syrian Arab Republic (Asia)
    SYR,
    #[cfg(feature = "sz")]
    /// The Kingdom of Eswatini (Africa)
    SWZ,
    #[cfg(feature = "td")]
    /// The Republic of Chad (Africa)
    CHA,
    #[cfg(feature = "tg")]
    /// The Togolese Republic (Africa)
    TOG,
    #[cfg(feature = "th")]
    /// The Kingdom of Thailand (Asia)
    THA,
    #[cfg(feature = "tj")]
    /// The Republic of Tajikistan (Asia)
    TJK,
    #[cfg(feature = "tl")]
    /// The Democratic Republic of Timor-Leste (Asia)
    TLS,
    #[cfg(feature = "tm")]
    /// Turkmenistan (Asia)
    TKM,
    #[cfg(feature = "tn")]
    /// The Republic of Tunisia (Africa)
    TUN,
    #[cfg(feature = "to")]
    /// The Kingdom of Tonga (Oceania)
    TGA,
    #[cfg(feature = "tr")]
    /// The Republic of Turkey (Asia)
    TUR,
    #[cfg(feature = "tt")]
    /// The Republic of Trinidad and Tobago (Americas)
    TRI,
    #[cfg(feature = "tv")]
    /// Tuvalu (Oceania)
    TUV,
    #[cfg(feature = "tw")]
    /// The Republic of China (Asia)
    TPE,
    #[cfg(feature = "tz")]
    /// The United Republic of Tanzania (Africa)
    TAN,
    #[cfg(feature = "ua")]
    /// Ukraine (Europe)
    UKR,
    #[cfg(feature = "ug")]
    /// The Republic of Uganda (Africa)
    UGA,
    #[cfg(feature = "us")]
    /// The United States of America (Americas)
    USA,
    #[cfg(feature = "uy")]
    /// The Oriental Republic of Uruguay (Americas)
    URU,
    #[cfg(feature = "uz")]
    /// The Republic of Uzbekistan (Asia)
    UZB,
    #[cfg(feature = "vc")]
    /// Saint Vincent and the Grenadines (Americas)
    VIN,
    #[cfg(feature = "ve")]
    /// The Bolivarian Republic of Venezuela (Americas)
    VEN,
    #[cfg(feature = "vg")]
    /// The Virgin Islands (Americas)
    IVB,
    #[cfg(feature = "vi")]
    /// The Virgin Islands of the United States (Americas)
    ISV,
    #[cfg(feature = "vn")]
    /// The Socialist Republic of Viet Nam (Asia)
    VIE,
    #[cfg(feature = "vu")]
    /// The Republic of Vanuatu (Oceania)
    VAN,
    #[cfg(feature = "ws")]
    /// The Independent State of Samoa (Oceania)
    SAM,
    #[cfg(feature = "ye")]
    /// The Republic of Yemen (Asia)
    YEM,
    #[cfg(feature = "za")]
    /// The Republic of South Africa (Africa)
    RSA,
    #[cfg(feature = "zm")]
    /// The Republic of Zambia (Africa)
    ZAM,
    #[cfg(feature = "zw")]
    /// The Republic of Zimbabwe (Africa)
    ZIM,
}
#[cfg(any(
    feature = "ad",
    feature = "ae",
    feature = "af",
    feature = "ag",
    feature = "al",
    feature = "am",
    feature = "ao",
    feature = "ar",
    feature = "as",
    feature = "at",
    feature = "au",
    feature = "aw",
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
    feature = "bm",
    feature = "bn",
    feature = "bo",
    feature = "br",
    feature = "bs",
    feature = "bt",
    feature = "bw",
    feature = "by",
    feature = "bz",
    feature = "ca",
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
    feature = "er",
    feature = "es",
    feature = "et",
    feature = "fi",
    feature = "fj",
    feature = "fm",
    feature = "fo",
    feature = "fr",
    feature = "ga",
    feature = "gb",
    feature = "gd",
    feature = "ge",
    feature = "gh",
    feature = "gm",
    feature = "gn",
    feature = "gq",
    feature = "gr",
    feature = "gt",
    feature = "gu",
    feature = "gw",
    feature = "gy",
    feature = "hk",
    feature = "hn",
    feature = "hr",
    feature = "ht",
    feature = "hu",
    feature = "id",
    feature = "ie",
    feature = "il",
    feature = "in",
    feature = "iq",
    feature = "ir",
    feature = "is",
    feature = "it",
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
    feature = "mg",
    feature = "mh",
    feature = "mk",
    feature = "ml",
    feature = "mm",
    feature = "mn",
    feature = "mr",
    feature = "mt",
    feature = "mu",
    feature = "mv",
    feature = "mw",
    feature = "mx",
    feature = "my",
    feature = "mz",
    feature = "na",
    feature = "ne",
    feature = "ng",
    feature = "ni",
    feature = "nl",
    feature = "no",
    feature = "np",
    feature = "nr",
    feature = "nz",
    feature = "om",
    feature = "pa",
    feature = "pe",
    feature = "pg",
    feature = "ph",
    feature = "pk",
    feature = "pl",
    feature = "pr",
    feature = "ps",
    feature = "pt",
    feature = "pw",
    feature = "py",
    feature = "qa",
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
    feature = "si",
    feature = "sk",
    feature = "sl",
    feature = "sm",
    feature = "sn",
    feature = "so",
    feature = "sr",
    feature = "st",
    feature = "sv",
    feature = "sy",
    feature = "sz",
    feature = "td",
    feature = "tg",
    feature = "th",
    feature = "tj",
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
    feature = "us",
    feature = "uy",
    feature = "uz",
    feature = "vc",
    feature = "ve",
    feature = "vg",
    feature = "vi",
    feature = "vn",
    feature = "vu",
    feature = "ws",
    feature = "ye",
    feature = "za",
    feature = "zm",
    feature = "zw",
))]
mod impls {
    use super::IOC;
    use crate::{Alpha2, Country, SearchError, SearchedItems};

    impl IOC {
        pub fn to_alpha2(&self) -> Alpha2 {
            match self {
                #[cfg(feature = "ad")] // The Principality of Andorra (Europe)
                IOC::AND => Alpha2::AD,
                #[cfg(feature = "ae")] // The United Arab Emirates (Asia)
                IOC::UAE => Alpha2::AE,
                #[cfg(feature = "af")] // The Islamic Republic of Afghanistan (Asia)
                IOC::AFG => Alpha2::AF,
                #[cfg(feature = "ag")] // Antigua and Barbuda (Americas)
                IOC::ANT => Alpha2::AG,
                #[cfg(feature = "al")] // The Republic of Albania (Europe)
                IOC::ALB => Alpha2::AL,
                #[cfg(feature = "am")] // The Republic of Armenia (Asia)
                IOC::ARM => Alpha2::AM,
                #[cfg(feature = "ao")] // The Republic of Angola (Africa)
                IOC::ANG => Alpha2::AO,
                #[cfg(feature = "ar")] // The Argentine Republic (Americas)
                IOC::ARG => Alpha2::AR,
                #[cfg(feature = "as")] // The Territory of American Samoa (Oceania)
                IOC::ASA => Alpha2::AS,
                #[cfg(feature = "at")] // The Republic of Austria (Europe)
                IOC::AUT => Alpha2::AT,
                #[cfg(feature = "au")] // The Commonwealth of Australia (Oceania)
                IOC::AUS => Alpha2::AU,
                #[cfg(feature = "aw")] // Aruba (Americas)
                IOC::ARU => Alpha2::AW,
                #[cfg(feature = "az")] // The Republic of Azerbaijan (Asia)
                IOC::AZE => Alpha2::AZ,
                #[cfg(feature = "ba")] // Bosnia and Herzegovina (Europe)
                IOC::BIH => Alpha2::BA,
                #[cfg(feature = "bb")] // Barbados (Americas)
                IOC::BAR => Alpha2::BB,
                #[cfg(feature = "bd")] // The People's Republic of Bangladesh (Asia)
                IOC::BAN => Alpha2::BD,
                #[cfg(feature = "be")] // The Kingdom of Belgium (Europe)
                IOC::BEL => Alpha2::BE,
                #[cfg(feature = "bf")] // Burkina Faso (Africa)
                IOC::BUR => Alpha2::BF,
                #[cfg(feature = "bg")] // The Republic of Bulgaria (Europe)
                IOC::BUL => Alpha2::BG,
                #[cfg(feature = "bh")] // The Kingdom of Bahrain (Asia)
                IOC::BRN => Alpha2::BH,
                #[cfg(feature = "bi")] // The Republic of Burundi (Africa)
                IOC::BDI => Alpha2::BI,
                #[cfg(feature = "bj")] // The Republic of Benin (Africa)
                IOC::BEN => Alpha2::BJ,
                #[cfg(feature = "bm")] // Bermuda (Americas)
                IOC::BER => Alpha2::BM,
                #[cfg(feature = "bn")] // The Nation of Brunei, the Abode of Peace (Asia)
                IOC::BRU => Alpha2::BN,
                #[cfg(feature = "bo")] // The Plurinational State of Bolivia (Americas)
                IOC::BOL => Alpha2::BO,
                #[cfg(feature = "br")] // The Federative Republic of Brazil (Americas)
                IOC::BRA => Alpha2::BR,
                #[cfg(feature = "bs")] // The Commonwealth of The Bahamas (Americas)
                IOC::BAH => Alpha2::BS,
                #[cfg(feature = "bt")] // The Kingdom of Bhutan (Asia)
                IOC::BHU => Alpha2::BT,
                #[cfg(feature = "bw")] // The Republic of Botswana (Africa)
                IOC::BOT => Alpha2::BW,
                #[cfg(feature = "by")] // The Republic of Belarus (Europe)
                IOC::BLR => Alpha2::BY,
                #[cfg(feature = "bz")] // Belize (Americas)
                IOC::BIZ => Alpha2::BZ,
                #[cfg(feature = "ca")] // Canada (Americas)
                IOC::CAN => Alpha2::CA,
                #[cfg(feature = "cd")] // The Democratic Republic of the Congo (Africa)
                IOC::COD => Alpha2::CD,
                #[cfg(feature = "cf")] // The Central African Republic (Africa)
                IOC::CAF => Alpha2::CF,
                #[cfg(feature = "cg")] // The Republic of the Congo (Africa)
                IOC::CGO => Alpha2::CG,
                #[cfg(feature = "ch")] // The Swiss Confederation (Europe)
                IOC::SUI => Alpha2::CH,
                #[cfg(feature = "ci")] // The Republic of Côte d'Ivoire (Africa)
                IOC::CIV => Alpha2::CI,
                #[cfg(feature = "ck")] // The Cook Islands (Oceania)
                IOC::COK => Alpha2::CK,
                #[cfg(feature = "cl")] // The Republic of Chile (Americas)
                IOC::CHI => Alpha2::CL,
                #[cfg(feature = "cm")] // The Republic of Cameroon (Africa)
                IOC::CMR => Alpha2::CM,
                #[cfg(feature = "cn")] // The People's Republic of China (Asia)
                IOC::CHN => Alpha2::CN,
                #[cfg(feature = "co")] // The Republic of Colombia (Americas)
                IOC::COL => Alpha2::CO,
                #[cfg(feature = "cr")] // The Republic of Costa Rica (Americas)
                IOC::CRC => Alpha2::CR,
                #[cfg(feature = "cu")] // The Republic of Cuba (Americas)
                IOC::CUB => Alpha2::CU,
                #[cfg(feature = "cv")] // The Republic of Cabo Verde (Africa)
                IOC::CPV => Alpha2::CV,
                #[cfg(feature = "cy")] // The Republic of Cyprus (Asia)
                IOC::CYP => Alpha2::CY,
                #[cfg(feature = "cz")] // The Czech Republic (Europe)
                IOC::CZE => Alpha2::CZ,
                #[cfg(feature = "de")] // The Federal Republic of Germany (Europe)
                IOC::GER => Alpha2::DE,
                #[cfg(feature = "dj")] // The Republic of Djibouti (Africa)
                IOC::DJI => Alpha2::DJ,
                #[cfg(feature = "dk")] // The Kingdom of Denmark (Europe)
                IOC::DEN => Alpha2::DK,
                #[cfg(feature = "dm")] // The Commonwealth of Dominica (Americas)
                IOC::DMA => Alpha2::DM,
                #[cfg(feature = "do")] // The Dominican Republic (Americas)
                IOC::DOM => Alpha2::DO,
                #[cfg(feature = "dz")] // The People's Democratic Republic of Algeria (Africa)
                IOC::ALG => Alpha2::DZ,
                #[cfg(feature = "ec")] // The Republic of Ecuador (Americas)
                IOC::ECU => Alpha2::EC,
                #[cfg(feature = "ee")] // The Republic of Estonia (Europe)
                IOC::EST => Alpha2::EE,
                #[cfg(feature = "eg")] // The Arab Republic of Egypt (Africa)
                IOC::EGY => Alpha2::EG,
                #[cfg(feature = "er")] // The State of Eritrea (Africa)
                IOC::ERI => Alpha2::ER,
                #[cfg(feature = "es")] // The Kingdom of Spain (Europe)
                IOC::ESP => Alpha2::ES,
                #[cfg(feature = "et")] // The Federal Democratic Republic of Ethiopia (Africa)
                IOC::ETH => Alpha2::ET,
                #[cfg(feature = "fi")] // The Republic of Finland (Europe)
                IOC::FIN => Alpha2::FI,
                #[cfg(feature = "fj")] // The Republic of Fiji (Oceania)
                IOC::FIJ => Alpha2::FJ,
                #[cfg(feature = "fm")] // The Federated States of Micronesia (Oceania)
                IOC::FSM => Alpha2::FM,
                #[cfg(feature = "fo")] // The Faroe Islands (Europe)
                IOC::FRO => Alpha2::FO,
                #[cfg(feature = "fr")] // The French Republic (Europe)
                IOC::FRA => Alpha2::FR,
                #[cfg(feature = "ga")] // The Gabonese Republic (Africa)
                IOC::GAB => Alpha2::GA,
                #[cfg(feature = "gb")]
                // The United Kingdom of Great Britain and Northern Ireland (Europe)
                IOC::GBR => Alpha2::GB,
                #[cfg(feature = "gd")] // Grenada (Americas)
                IOC::GRN => Alpha2::GD,
                #[cfg(feature = "ge")] // Georgia (Asia)
                IOC::GEO => Alpha2::GE,
                #[cfg(feature = "gh")] // The Republic of Ghana (Africa)
                IOC::GHA => Alpha2::GH,
                #[cfg(feature = "gm")] // The Republic of The Gambia (Africa)
                IOC::GAM => Alpha2::GM,
                #[cfg(feature = "gn")] // The Republic of Guinea (Africa)
                IOC::GUI => Alpha2::GN,
                #[cfg(feature = "gq")] // The Republic of Equatorial Guinea (Africa)
                IOC::GEQ => Alpha2::GQ,
                #[cfg(feature = "gr")] // The Hellenic Republic (Europe)
                IOC::GRE => Alpha2::GR,
                #[cfg(feature = "gt")] // The Republic of Guatemala (Americas)
                IOC::GUA => Alpha2::GT,
                #[cfg(feature = "gu")] // The Territory of Guam (Oceania)
                IOC::GUM => Alpha2::GU,
                #[cfg(feature = "gw")] // The Republic of Guinea-Bissau (Africa)
                IOC::GBS => Alpha2::GW,
                #[cfg(feature = "gy")] // The Co-operative Republic of Guyana (Americas)
                IOC::GUY => Alpha2::GY,
                #[cfg(feature = "hk")]
                // The Hong Kong Special Administrative Region of China (Asia)
                IOC::HKG => Alpha2::HK,
                #[cfg(feature = "hn")] // The Republic of Honduras (Americas)
                IOC::HON => Alpha2::HN,
                #[cfg(feature = "hr")] // The Republic of Croatia (Europe)
                IOC::CRO => Alpha2::HR,
                #[cfg(feature = "ht")] // The Republic of Haiti (Americas)
                IOC::HAI => Alpha2::HT,
                #[cfg(feature = "hu")] // Hungary (Europe)
                IOC::HUN => Alpha2::HU,
                #[cfg(feature = "id")] // The Republic of Indonesia (Asia)
                IOC::INA => Alpha2::ID,
                #[cfg(feature = "ie")] // Ireland (Europe)
                IOC::IRL => Alpha2::IE,
                #[cfg(feature = "il")] // The State of Israel (Asia)
                IOC::ISR => Alpha2::IL,
                #[cfg(feature = "in")] // The Republic of India (Asia)
                IOC::IND => Alpha2::IN,
                #[cfg(feature = "iq")] // The Republic of Iraq (Asia)
                IOC::IRQ => Alpha2::IQ,
                #[cfg(feature = "ir")] // The Islamic Republic of Iran (Asia)
                IOC::IRI => Alpha2::IR,
                #[cfg(feature = "is")] // Iceland (Europe)
                IOC::ISL => Alpha2::IS,
                #[cfg(feature = "it")] // The Italian Republic (Europe)
                IOC::ITA => Alpha2::IT,
                #[cfg(feature = "jm")] // Jamaica (Americas)
                IOC::JAM => Alpha2::JM,
                #[cfg(feature = "jo")] // The Hashemite Kingdom of Jordan (Asia)
                IOC::JOR => Alpha2::JO,
                #[cfg(feature = "jp")] // Japan (Asia)
                IOC::JPN => Alpha2::JP,
                #[cfg(feature = "ke")] // The Republic of Kenya (Africa)
                IOC::KEN => Alpha2::KE,
                #[cfg(feature = "kg")] // The Kyrgyz Republic (Asia)
                IOC::KGZ => Alpha2::KG,
                #[cfg(feature = "kh")] // The Kingdom of Cambodia (Asia)
                IOC::CAM => Alpha2::KH,
                #[cfg(feature = "ki")] // The Republic of Kiribati (Oceania)
                IOC::KIR => Alpha2::KI,
                #[cfg(feature = "km")] // The Union of the Comoros (Africa)
                IOC::COM => Alpha2::KM,
                #[cfg(feature = "kn")] // Saint Kitts and Nevis (Americas)
                IOC::SKN => Alpha2::KN,
                #[cfg(feature = "kp")] // The Democratic People's Republic of Korea (Asia)
                IOC::PRK => Alpha2::KP,
                #[cfg(feature = "kr")] // The Republic of Korea (Asia)
                IOC::KOR => Alpha2::KR,
                #[cfg(feature = "kw")] // The State of Kuwait (Asia)
                IOC::KUW => Alpha2::KW,
                #[cfg(feature = "ky")] // The Cayman Islands (Americas)
                IOC::CAY => Alpha2::KY,
                #[cfg(feature = "kz")] // The Republic of Kazakhstan (Asia)
                IOC::KAZ => Alpha2::KZ,
                #[cfg(feature = "la")] // The Lao People's Democratic Republic (Asia)
                IOC::LAO => Alpha2::LA,
                #[cfg(feature = "lb")] // The Lebanese Republic (Asia)
                IOC::LIB => Alpha2::LB,
                #[cfg(feature = "lc")] // Saint Lucia (Americas)
                IOC::LCA => Alpha2::LC,
                #[cfg(feature = "li")] // The Principality of Liechtenstein (Europe)
                IOC::LIE => Alpha2::LI,
                #[cfg(feature = "lk")] // The Democratic Socialist Republic of Sri Lanka (Asia)
                IOC::SRI => Alpha2::LK,
                #[cfg(feature = "lr")] // The Republic of Liberia (Africa)
                IOC::LBR => Alpha2::LR,
                #[cfg(feature = "ls")] // The Kingdom of Lesotho (Africa)
                IOC::LES => Alpha2::LS,
                #[cfg(feature = "lt")] // The Republic of Lithuania (Europe)
                IOC::LTU => Alpha2::LT,
                #[cfg(feature = "lu")] // The Grand Duchy of Luxembourg (Europe)
                IOC::LUX => Alpha2::LU,
                #[cfg(feature = "lv")] // The Republic of Latvia (Europe)
                IOC::LAT => Alpha2::LV,
                #[cfg(feature = "ly")] // The State of Libya (Africa)
                IOC::LBA => Alpha2::LY,
                #[cfg(feature = "ma")] // The Kingdom of Morocco (Africa)
                IOC::MAR => Alpha2::MA,
                #[cfg(feature = "mc")] // The Principality of Monaco (Europe)
                IOC::MON => Alpha2::MC,
                #[cfg(feature = "md")] // The Republic of Moldova (Europe)
                IOC::MDA => Alpha2::MD,
                #[cfg(feature = "me")] // Montenegro (Europe)
                IOC::MNE => Alpha2::ME,
                #[cfg(feature = "mg")] // The Republic of Madagascar (Africa)
                IOC::MAD => Alpha2::MG,
                #[cfg(feature = "mh")] // The Republic of the Marshall Islands (Oceania)
                IOC::MHL => Alpha2::MH,
                #[cfg(feature = "mk")] // The Republic of North Macedonia (Europe)
                IOC::MKD => Alpha2::MK,
                #[cfg(feature = "ml")] // The Republic of Mali (Africa)
                IOC::MLI => Alpha2::ML,
                #[cfg(feature = "mm")] // The Republic of the Union of Myanmar (Asia)
                IOC::MYA => Alpha2::MM,
                #[cfg(feature = "mn")] // Mongolia (Asia)
                IOC::MGL => Alpha2::MN,
                #[cfg(feature = "mr")] // The Islamic Republic of Mauritania (Africa)
                IOC::MTN => Alpha2::MR,
                #[cfg(feature = "mt")] // The Republic of Malta (Europe)
                IOC::MLT => Alpha2::MT,
                #[cfg(feature = "mu")] // The Republic of Mauritius (Africa)
                IOC::MRI => Alpha2::MU,
                #[cfg(feature = "mv")] // The Republic of Maldives (Asia)
                IOC::MDV => Alpha2::MV,
                #[cfg(feature = "mw")] // The Republic of Malawi (Africa)
                IOC::MAW => Alpha2::MW,
                #[cfg(feature = "mx")] // The United Mexican States (Americas)
                IOC::MEX => Alpha2::MX,
                #[cfg(feature = "my")] // Malaysia (Asia)
                IOC::MAS => Alpha2::MY,
                #[cfg(feature = "mz")] // The Republic of Mozambique (Africa)
                IOC::MOZ => Alpha2::MZ,
                #[cfg(feature = "na")] // The Republic of Namibia (Africa)
                IOC::NAM => Alpha2::NA,
                #[cfg(feature = "ne")] // The Republic of the Niger (Africa)
                IOC::NIG => Alpha2::NE,
                #[cfg(feature = "ng")] // The Federal Republic of Nigeria (Africa)
                IOC::NGR => Alpha2::NG,
                #[cfg(feature = "ni")] // The Republic of Nicaragua (Americas)
                IOC::NCA => Alpha2::NI,
                #[cfg(feature = "nl")] // The Kingdom of the Netherlands (Europe)
                IOC::NED => Alpha2::NL,
                #[cfg(feature = "no")] // The Kingdom of Norway (Europe)
                IOC::NOR => Alpha2::NO,
                #[cfg(feature = "np")] // The Federal Democratic Republic of Nepal (Asia)
                IOC::NEP => Alpha2::NP,
                #[cfg(feature = "nr")] // The Republic of Nauru (Oceania)
                IOC::NRU => Alpha2::NR,
                #[cfg(feature = "nz")] // New Zealand (Oceania)
                IOC::NZL => Alpha2::NZ,
                #[cfg(feature = "om")] // The Sultanate of Oman (Asia)
                IOC::OMA => Alpha2::OM,
                #[cfg(feature = "pa")] // The Republic of Panamá (Americas)
                IOC::PAN => Alpha2::PA,
                #[cfg(feature = "pe")] // The Republic of Perú (Americas)
                IOC::PER => Alpha2::PE,
                #[cfg(feature = "pg")] // The Independent State of Papua New Guinea (Oceania)
                IOC::PNG => Alpha2::PG,
                #[cfg(feature = "ph")] // The Republic of the Philippines (Asia)
                IOC::PHI => Alpha2::PH,
                #[cfg(feature = "pk")] // The Islamic Republic of Pakistan (Asia)
                IOC::PAK => Alpha2::PK,
                #[cfg(feature = "pl")] // The Republic of Poland (Europe)
                IOC::POL => Alpha2::PL,
                #[cfg(feature = "pr")] // The Commonwealth of Puerto Rico (Americas)
                IOC::PUR => Alpha2::PR,
                #[cfg(feature = "ps")] // The State of Palestine (Asia)
                IOC::PLE => Alpha2::PS,
                #[cfg(feature = "pt")] // The Portuguese Republic (Europe)
                IOC::POR => Alpha2::PT,
                #[cfg(feature = "pw")] // The Republic of Palau (Oceania)
                IOC::PLW => Alpha2::PW,
                #[cfg(feature = "py")] // The Republic of Paraguay (Americas)
                IOC::PAR => Alpha2::PY,
                #[cfg(feature = "qa")] // The State of Qatar (Asia)
                IOC::QAT => Alpha2::QA,
                #[cfg(feature = "ro")] // Romania (Europe)
                IOC::ROU => Alpha2::RO,
                #[cfg(feature = "rs")] // The Republic of Serbia (Europe)
                IOC::SRB => Alpha2::RS,
                #[cfg(feature = "ru")] // The Russian Federation (Europe)
                IOC::RUS => Alpha2::RU,
                #[cfg(feature = "rw")] // The Republic of Rwanda (Africa)
                IOC::RWA => Alpha2::RW,
                #[cfg(feature = "sa")] // The Kingdom of Saudi Arabia (Asia)
                IOC::KSA => Alpha2::SA,
                #[cfg(feature = "sb")] // The Solomon Islands (Oceania)
                IOC::SOL => Alpha2::SB,
                #[cfg(feature = "sc")] // The Republic of Seychelles (Africa)
                IOC::SEY => Alpha2::SC,
                #[cfg(feature = "sd")] // The Republic of the Sudan (Africa)
                IOC::SUD => Alpha2::SD,
                #[cfg(feature = "se")] // The Kingdom of Sweden (Europe)
                IOC::SWE => Alpha2::SE,
                #[cfg(feature = "sg")] // The Republic of Singapore (Asia)
                IOC::SGP => Alpha2::SG,
                #[cfg(feature = "si")] // The Republic of Slovenia (Europe)
                IOC::SLO => Alpha2::SI,
                #[cfg(feature = "sk")] // The Slovak Republic (Europe)
                IOC::SVK => Alpha2::SK,
                #[cfg(feature = "sl")] // The Republic of Sierra Leone (Africa)
                IOC::SLE => Alpha2::SL,
                #[cfg(feature = "sm")] // The Republic of San Marino (Europe)
                IOC::SMR => Alpha2::SM,
                #[cfg(feature = "sn")] // The Republic of Senegal (Africa)
                IOC::SEN => Alpha2::SN,
                #[cfg(feature = "so")] // The Federal Republic of Somalia (Africa)
                IOC::SOM => Alpha2::SO,
                #[cfg(feature = "sr")] // The Republic of Suriname (Americas)
                IOC::SUR => Alpha2::SR,
                #[cfg(feature = "st")] // The Democratic Republic of São Tomé and Príncipe (Africa)
                IOC::STP => Alpha2::ST,
                #[cfg(feature = "sv")] // The Republic of El Salvador (Americas)
                IOC::ESA => Alpha2::SV,
                #[cfg(feature = "sy")] // The Syrian Arab Republic (Asia)
                IOC::SYR => Alpha2::SY,
                #[cfg(feature = "sz")] // The Kingdom of Eswatini (Africa)
                IOC::SWZ => Alpha2::SZ,
                #[cfg(feature = "td")] // The Republic of Chad (Africa)
                IOC::CHA => Alpha2::TD,
                #[cfg(feature = "tg")] // The Togolese Republic (Africa)
                IOC::TOG => Alpha2::TG,
                #[cfg(feature = "th")] // The Kingdom of Thailand (Asia)
                IOC::THA => Alpha2::TH,
                #[cfg(feature = "tj")] // The Republic of Tajikistan (Asia)
                IOC::TJK => Alpha2::TJ,
                #[cfg(feature = "tl")] // The Democratic Republic of Timor-Leste (Asia)
                IOC::TLS => Alpha2::TL,
                #[cfg(feature = "tm")] // Turkmenistan (Asia)
                IOC::TKM => Alpha2::TM,
                #[cfg(feature = "tn")] // The Republic of Tunisia (Africa)
                IOC::TUN => Alpha2::TN,
                #[cfg(feature = "to")] // The Kingdom of Tonga (Oceania)
                IOC::TGA => Alpha2::TO,
                #[cfg(feature = "tr")] // The Republic of Turkey (Asia)
                IOC::TUR => Alpha2::TR,
                #[cfg(feature = "tt")] // The Republic of Trinidad and Tobago (Americas)
                IOC::TRI => Alpha2::TT,
                #[cfg(feature = "tv")] // Tuvalu (Oceania)
                IOC::TUV => Alpha2::TV,
                #[cfg(feature = "tw")] // The Republic of China (Asia)
                IOC::TPE => Alpha2::TW,
                #[cfg(feature = "tz")] // The United Republic of Tanzania (Africa)
                IOC::TAN => Alpha2::TZ,
                #[cfg(feature = "ua")] // Ukraine (Europe)
                IOC::UKR => Alpha2::UA,
                #[cfg(feature = "ug")] // The Republic of Uganda (Africa)
                IOC::UGA => Alpha2::UG,
                #[cfg(feature = "us")] // The United States of America (Americas)
                IOC::USA => Alpha2::US,
                #[cfg(feature = "uy")] // The Oriental Republic of Uruguay (Americas)
                IOC::URU => Alpha2::UY,
                #[cfg(feature = "uz")] // The Republic of Uzbekistan (Asia)
                IOC::UZB => Alpha2::UZ,
                #[cfg(feature = "vc")] // Saint Vincent and the Grenadines (Americas)
                IOC::VIN => Alpha2::VC,
                #[cfg(feature = "ve")] // The Bolivarian Republic of Venezuela (Americas)
                IOC::VEN => Alpha2::VE,
                #[cfg(feature = "vg")] // The Virgin Islands (Americas)
                IOC::IVB => Alpha2::VG,
                #[cfg(feature = "vi")] // The Virgin Islands of the United States (Americas)
                IOC::ISV => Alpha2::VI,
                #[cfg(feature = "vn")] // The Socialist Republic of Viet Nam (Asia)
                IOC::VIE => Alpha2::VN,
                #[cfg(feature = "vu")] // The Republic of Vanuatu (Oceania)
                IOC::VAN => Alpha2::VU,
                #[cfg(feature = "ws")] // The Independent State of Samoa (Oceania)
                IOC::SAM => Alpha2::WS,
                #[cfg(feature = "ye")] // The Republic of Yemen (Asia)
                IOC::YEM => Alpha2::YE,
                #[cfg(feature = "za")] // The Republic of South Africa (Africa)
                IOC::RSA => Alpha2::ZA,
                #[cfg(feature = "zm")] // The Republic of Zambia (Africa)
                IOC::ZAM => Alpha2::ZM,
                #[cfg(feature = "zw")] // The Republic of Zimbabwe (Africa)
                IOC::ZIM => Alpha2::ZW,
            }
        }

        pub fn to_country(&self) -> Country {
            self.to_alpha2().to_country()
        }
    }

    impl ToString for IOC {
        fn to_string(&self) -> String {
            match self {
                #[cfg(feature = "ad")] // The Principality of Andorra (Europe)
                IOC::AND => "AND",
                #[cfg(feature = "ae")] // The United Arab Emirates (Asia)
                IOC::UAE => "UAE",
                #[cfg(feature = "af")] // The Islamic Republic of Afghanistan (Asia)
                IOC::AFG => "AFG",
                #[cfg(feature = "ag")] // Antigua and Barbuda (Americas)
                IOC::ANT => "ANT",
                #[cfg(feature = "al")] // The Republic of Albania (Europe)
                IOC::ALB => "ALB",
                #[cfg(feature = "am")] // The Republic of Armenia (Asia)
                IOC::ARM => "ARM",
                #[cfg(feature = "ao")] // The Republic of Angola (Africa)
                IOC::ANG => "ANG",
                #[cfg(feature = "ar")] // The Argentine Republic (Americas)
                IOC::ARG => "ARG",
                #[cfg(feature = "as")] // The Territory of American Samoa (Oceania)
                IOC::ASA => "ASA",
                #[cfg(feature = "at")] // The Republic of Austria (Europe)
                IOC::AUT => "AUT",
                #[cfg(feature = "au")] // The Commonwealth of Australia (Oceania)
                IOC::AUS => "AUS",
                #[cfg(feature = "aw")] // Aruba (Americas)
                IOC::ARU => "ARU",
                #[cfg(feature = "az")] // The Republic of Azerbaijan (Asia)
                IOC::AZE => "AZE",
                #[cfg(feature = "ba")] // Bosnia and Herzegovina (Europe)
                IOC::BIH => "BIH",
                #[cfg(feature = "bb")] // Barbados (Americas)
                IOC::BAR => "BAR",
                #[cfg(feature = "bd")] // The People's Republic of Bangladesh (Asia)
                IOC::BAN => "BAN",
                #[cfg(feature = "be")] // The Kingdom of Belgium (Europe)
                IOC::BEL => "BEL",
                #[cfg(feature = "bf")] // Burkina Faso (Africa)
                IOC::BUR => "BUR",
                #[cfg(feature = "bg")] // The Republic of Bulgaria (Europe)
                IOC::BUL => "BUL",
                #[cfg(feature = "bh")] // The Kingdom of Bahrain (Asia)
                IOC::BRN => "BRN",
                #[cfg(feature = "bi")] // The Republic of Burundi (Africa)
                IOC::BDI => "BDI",
                #[cfg(feature = "bj")] // The Republic of Benin (Africa)
                IOC::BEN => "BEN",
                #[cfg(feature = "bm")] // Bermuda (Americas)
                IOC::BER => "BER",
                #[cfg(feature = "bn")] // The Nation of Brunei, the Abode of Peace (Asia)
                IOC::BRU => "BRU",
                #[cfg(feature = "bo")] // The Plurinational State of Bolivia (Americas)
                IOC::BOL => "BOL",
                #[cfg(feature = "br")] // The Federative Republic of Brazil (Americas)
                IOC::BRA => "BRA",
                #[cfg(feature = "bs")] // The Commonwealth of The Bahamas (Americas)
                IOC::BAH => "BAH",
                #[cfg(feature = "bt")] // The Kingdom of Bhutan (Asia)
                IOC::BHU => "BHU",
                #[cfg(feature = "bw")] // The Republic of Botswana (Africa)
                IOC::BOT => "BOT",
                #[cfg(feature = "by")] // The Republic of Belarus (Europe)
                IOC::BLR => "BLR",
                #[cfg(feature = "bz")] // Belize (Americas)
                IOC::BIZ => "BIZ",
                #[cfg(feature = "ca")] // Canada (Americas)
                IOC::CAN => "CAN",
                #[cfg(feature = "cd")] // The Democratic Republic of the Congo (Africa)
                IOC::COD => "COD",
                #[cfg(feature = "cf")] // The Central African Republic (Africa)
                IOC::CAF => "CAF",
                #[cfg(feature = "cg")] // The Republic of the Congo (Africa)
                IOC::CGO => "CGO",
                #[cfg(feature = "ch")] // The Swiss Confederation (Europe)
                IOC::SUI => "SUI",
                #[cfg(feature = "ci")] // The Republic of Côte d'Ivoire (Africa)
                IOC::CIV => "CIV",
                #[cfg(feature = "ck")] // The Cook Islands (Oceania)
                IOC::COK => "COK",
                #[cfg(feature = "cl")] // The Republic of Chile (Americas)
                IOC::CHI => "CHI",
                #[cfg(feature = "cm")] // The Republic of Cameroon (Africa)
                IOC::CMR => "CMR",
                #[cfg(feature = "cn")] // The People's Republic of China (Asia)
                IOC::CHN => "CHN",
                #[cfg(feature = "co")] // The Republic of Colombia (Americas)
                IOC::COL => "COL",
                #[cfg(feature = "cr")] // The Republic of Costa Rica (Americas)
                IOC::CRC => "CRC",
                #[cfg(feature = "cu")] // The Republic of Cuba (Americas)
                IOC::CUB => "CUB",
                #[cfg(feature = "cv")] // The Republic of Cabo Verde (Africa)
                IOC::CPV => "CPV",
                #[cfg(feature = "cy")] // The Republic of Cyprus (Asia)
                IOC::CYP => "CYP",
                #[cfg(feature = "cz")] // The Czech Republic (Europe)
                IOC::CZE => "CZE",
                #[cfg(feature = "de")] // The Federal Republic of Germany (Europe)
                IOC::GER => "GER",
                #[cfg(feature = "dj")] // The Republic of Djibouti (Africa)
                IOC::DJI => "DJI",
                #[cfg(feature = "dk")] // The Kingdom of Denmark (Europe)
                IOC::DEN => "DEN",
                #[cfg(feature = "dm")] // The Commonwealth of Dominica (Americas)
                IOC::DMA => "DMA",
                #[cfg(feature = "do")] // The Dominican Republic (Americas)
                IOC::DOM => "DOM",
                #[cfg(feature = "dz")] // The People's Democratic Republic of Algeria (Africa)
                IOC::ALG => "ALG",
                #[cfg(feature = "ec")] // The Republic of Ecuador (Americas)
                IOC::ECU => "ECU",
                #[cfg(feature = "ee")] // The Republic of Estonia (Europe)
                IOC::EST => "EST",
                #[cfg(feature = "eg")] // The Arab Republic of Egypt (Africa)
                IOC::EGY => "EGY",
                #[cfg(feature = "er")] // The State of Eritrea (Africa)
                IOC::ERI => "ERI",
                #[cfg(feature = "es")] // The Kingdom of Spain (Europe)
                IOC::ESP => "ESP",
                #[cfg(feature = "et")] // The Federal Democratic Republic of Ethiopia (Africa)
                IOC::ETH => "ETH",
                #[cfg(feature = "fi")] // The Republic of Finland (Europe)
                IOC::FIN => "FIN",
                #[cfg(feature = "fj")] // The Republic of Fiji (Oceania)
                IOC::FIJ => "FIJ",
                #[cfg(feature = "fm")] // The Federated States of Micronesia (Oceania)
                IOC::FSM => "FSM",
                #[cfg(feature = "fo")] // The Faroe Islands (Europe)
                IOC::FRO => "FRO",
                #[cfg(feature = "fr")] // The French Republic (Europe)
                IOC::FRA => "FRA",
                #[cfg(feature = "ga")] // The Gabonese Republic (Africa)
                IOC::GAB => "GAB",
                #[cfg(feature = "gb")]
                // The United Kingdom of Great Britain and Northern Ireland (Europe)
                IOC::GBR => "GBR",
                #[cfg(feature = "gd")] // Grenada (Americas)
                IOC::GRN => "GRN",
                #[cfg(feature = "ge")] // Georgia (Asia)
                IOC::GEO => "GEO",
                #[cfg(feature = "gh")] // The Republic of Ghana (Africa)
                IOC::GHA => "GHA",
                #[cfg(feature = "gm")] // The Republic of The Gambia (Africa)
                IOC::GAM => "GAM",
                #[cfg(feature = "gn")] // The Republic of Guinea (Africa)
                IOC::GUI => "GUI",
                #[cfg(feature = "gq")] // The Republic of Equatorial Guinea (Africa)
                IOC::GEQ => "GEQ",
                #[cfg(feature = "gr")] // The Hellenic Republic (Europe)
                IOC::GRE => "GRE",
                #[cfg(feature = "gt")] // The Republic of Guatemala (Americas)
                IOC::GUA => "GUA",
                #[cfg(feature = "gu")] // The Territory of Guam (Oceania)
                IOC::GUM => "GUM",
                #[cfg(feature = "gw")] // The Republic of Guinea-Bissau (Africa)
                IOC::GBS => "GBS",
                #[cfg(feature = "gy")] // The Co-operative Republic of Guyana (Americas)
                IOC::GUY => "GUY",
                #[cfg(feature = "hk")]
                // The Hong Kong Special Administrative Region of China (Asia)
                IOC::HKG => "HKG",
                #[cfg(feature = "hn")] // The Republic of Honduras (Americas)
                IOC::HON => "HON",
                #[cfg(feature = "hr")] // The Republic of Croatia (Europe)
                IOC::CRO => "CRO",
                #[cfg(feature = "ht")] // The Republic of Haiti (Americas)
                IOC::HAI => "HAI",
                #[cfg(feature = "hu")] // Hungary (Europe)
                IOC::HUN => "HUN",
                #[cfg(feature = "id")] // The Republic of Indonesia (Asia)
                IOC::INA => "INA",
                #[cfg(feature = "ie")] // Ireland (Europe)
                IOC::IRL => "IRL",
                #[cfg(feature = "il")] // The State of Israel (Asia)
                IOC::ISR => "ISR",
                #[cfg(feature = "in")] // The Republic of India (Asia)
                IOC::IND => "IND",
                #[cfg(feature = "iq")] // The Republic of Iraq (Asia)
                IOC::IRQ => "IRQ",
                #[cfg(feature = "ir")] // The Islamic Republic of Iran (Asia)
                IOC::IRI => "IRI",
                #[cfg(feature = "is")] // Iceland (Europe)
                IOC::ISL => "ISL",
                #[cfg(feature = "it")] // The Italian Republic (Europe)
                IOC::ITA => "ITA",
                #[cfg(feature = "jm")] // Jamaica (Americas)
                IOC::JAM => "JAM",
                #[cfg(feature = "jo")] // The Hashemite Kingdom of Jordan (Asia)
                IOC::JOR => "JOR",
                #[cfg(feature = "jp")] // Japan (Asia)
                IOC::JPN => "JPN",
                #[cfg(feature = "ke")] // The Republic of Kenya (Africa)
                IOC::KEN => "KEN",
                #[cfg(feature = "kg")] // The Kyrgyz Republic (Asia)
                IOC::KGZ => "KGZ",
                #[cfg(feature = "kh")] // The Kingdom of Cambodia (Asia)
                IOC::CAM => "CAM",
                #[cfg(feature = "ki")] // The Republic of Kiribati (Oceania)
                IOC::KIR => "KIR",
                #[cfg(feature = "km")] // The Union of the Comoros (Africa)
                IOC::COM => "COM",
                #[cfg(feature = "kn")] // Saint Kitts and Nevis (Americas)
                IOC::SKN => "SKN",
                #[cfg(feature = "kp")] // The Democratic People's Republic of Korea (Asia)
                IOC::PRK => "PRK",
                #[cfg(feature = "kr")] // The Republic of Korea (Asia)
                IOC::KOR => "KOR",
                #[cfg(feature = "kw")] // The State of Kuwait (Asia)
                IOC::KUW => "KUW",
                #[cfg(feature = "ky")] // The Cayman Islands (Americas)
                IOC::CAY => "CAY",
                #[cfg(feature = "kz")] // The Republic of Kazakhstan (Asia)
                IOC::KAZ => "KAZ",
                #[cfg(feature = "la")] // The Lao People's Democratic Republic (Asia)
                IOC::LAO => "LAO",
                #[cfg(feature = "lb")] // The Lebanese Republic (Asia)
                IOC::LIB => "LIB",
                #[cfg(feature = "lc")] // Saint Lucia (Americas)
                IOC::LCA => "LCA",
                #[cfg(feature = "li")] // The Principality of Liechtenstein (Europe)
                IOC::LIE => "LIE",
                #[cfg(feature = "lk")] // The Democratic Socialist Republic of Sri Lanka (Asia)
                IOC::SRI => "SRI",
                #[cfg(feature = "lr")] // The Republic of Liberia (Africa)
                IOC::LBR => "LBR",
                #[cfg(feature = "ls")] // The Kingdom of Lesotho (Africa)
                IOC::LES => "LES",
                #[cfg(feature = "lt")] // The Republic of Lithuania (Europe)
                IOC::LTU => "LTU",
                #[cfg(feature = "lu")] // The Grand Duchy of Luxembourg (Europe)
                IOC::LUX => "LUX",
                #[cfg(feature = "lv")] // The Republic of Latvia (Europe)
                IOC::LAT => "LAT",
                #[cfg(feature = "ly")] // The State of Libya (Africa)
                IOC::LBA => "LBA",
                #[cfg(feature = "ma")] // The Kingdom of Morocco (Africa)
                IOC::MAR => "MAR",
                #[cfg(feature = "mc")] // The Principality of Monaco (Europe)
                IOC::MON => "MON",
                #[cfg(feature = "md")] // The Republic of Moldova (Europe)
                IOC::MDA => "MDA",
                #[cfg(feature = "me")] // Montenegro (Europe)
                IOC::MNE => "MNE",
                #[cfg(feature = "mg")] // The Republic of Madagascar (Africa)
                IOC::MAD => "MAD",
                #[cfg(feature = "mh")] // The Republic of the Marshall Islands (Oceania)
                IOC::MHL => "MHL",
                #[cfg(feature = "mk")] // The Republic of North Macedonia (Europe)
                IOC::MKD => "MKD",
                #[cfg(feature = "ml")] // The Republic of Mali (Africa)
                IOC::MLI => "MLI",
                #[cfg(feature = "mm")] // The Republic of the Union of Myanmar (Asia)
                IOC::MYA => "MYA",
                #[cfg(feature = "mn")] // Mongolia (Asia)
                IOC::MGL => "MGL",
                #[cfg(feature = "mr")] // The Islamic Republic of Mauritania (Africa)
                IOC::MTN => "MTN",
                #[cfg(feature = "mt")] // The Republic of Malta (Europe)
                IOC::MLT => "MLT",
                #[cfg(feature = "mu")] // The Republic of Mauritius (Africa)
                IOC::MRI => "MRI",
                #[cfg(feature = "mv")] // The Republic of Maldives (Asia)
                IOC::MDV => "MDV",
                #[cfg(feature = "mw")] // The Republic of Malawi (Africa)
                IOC::MAW => "MAW",
                #[cfg(feature = "mx")] // The United Mexican States (Americas)
                IOC::MEX => "MEX",
                #[cfg(feature = "my")] // Malaysia (Asia)
                IOC::MAS => "MAS",
                #[cfg(feature = "mz")] // The Republic of Mozambique (Africa)
                IOC::MOZ => "MOZ",
                #[cfg(feature = "na")] // The Republic of Namibia (Africa)
                IOC::NAM => "NAM",
                #[cfg(feature = "ne")] // The Republic of the Niger (Africa)
                IOC::NIG => "NIG",
                #[cfg(feature = "ng")] // The Federal Republic of Nigeria (Africa)
                IOC::NGR => "NGR",
                #[cfg(feature = "ni")] // The Republic of Nicaragua (Americas)
                IOC::NCA => "NCA",
                #[cfg(feature = "nl")] // The Kingdom of the Netherlands (Europe)
                IOC::NED => "NED",
                #[cfg(feature = "no")] // The Kingdom of Norway (Europe)
                IOC::NOR => "NOR",
                #[cfg(feature = "np")] // The Federal Democratic Republic of Nepal (Asia)
                IOC::NEP => "NEP",
                #[cfg(feature = "nr")] // The Republic of Nauru (Oceania)
                IOC::NRU => "NRU",
                #[cfg(feature = "nz")] // New Zealand (Oceania)
                IOC::NZL => "NZL",
                #[cfg(feature = "om")] // The Sultanate of Oman (Asia)
                IOC::OMA => "OMA",
                #[cfg(feature = "pa")] // The Republic of Panamá (Americas)
                IOC::PAN => "PAN",
                #[cfg(feature = "pe")] // The Republic of Perú (Americas)
                IOC::PER => "PER",
                #[cfg(feature = "pg")] // The Independent State of Papua New Guinea (Oceania)
                IOC::PNG => "PNG",
                #[cfg(feature = "ph")] // The Republic of the Philippines (Asia)
                IOC::PHI => "PHI",
                #[cfg(feature = "pk")] // The Islamic Republic of Pakistan (Asia)
                IOC::PAK => "PAK",
                #[cfg(feature = "pl")] // The Republic of Poland (Europe)
                IOC::POL => "POL",
                #[cfg(feature = "pr")] // The Commonwealth of Puerto Rico (Americas)
                IOC::PUR => "PUR",
                #[cfg(feature = "ps")] // The State of Palestine (Asia)
                IOC::PLE => "PLE",
                #[cfg(feature = "pt")] // The Portuguese Republic (Europe)
                IOC::POR => "POR",
                #[cfg(feature = "pw")] // The Republic of Palau (Oceania)
                IOC::PLW => "PLW",
                #[cfg(feature = "py")] // The Republic of Paraguay (Americas)
                IOC::PAR => "PAR",
                #[cfg(feature = "qa")] // The State of Qatar (Asia)
                IOC::QAT => "QAT",
                #[cfg(feature = "ro")] // Romania (Europe)
                IOC::ROU => "ROU",
                #[cfg(feature = "rs")] // The Republic of Serbia (Europe)
                IOC::SRB => "SRB",
                #[cfg(feature = "ru")] // The Russian Federation (Europe)
                IOC::RUS => "RUS",
                #[cfg(feature = "rw")] // The Republic of Rwanda (Africa)
                IOC::RWA => "RWA",
                #[cfg(feature = "sa")] // The Kingdom of Saudi Arabia (Asia)
                IOC::KSA => "KSA",
                #[cfg(feature = "sb")] // The Solomon Islands (Oceania)
                IOC::SOL => "SOL",
                #[cfg(feature = "sc")] // The Republic of Seychelles (Africa)
                IOC::SEY => "SEY",
                #[cfg(feature = "sd")] // The Republic of the Sudan (Africa)
                IOC::SUD => "SUD",
                #[cfg(feature = "se")] // The Kingdom of Sweden (Europe)
                IOC::SWE => "SWE",
                #[cfg(feature = "sg")] // The Republic of Singapore (Asia)
                IOC::SGP => "SGP",
                #[cfg(feature = "si")] // The Republic of Slovenia (Europe)
                IOC::SLO => "SLO",
                #[cfg(feature = "sk")] // The Slovak Republic (Europe)
                IOC::SVK => "SVK",
                #[cfg(feature = "sl")] // The Republic of Sierra Leone (Africa)
                IOC::SLE => "SLE",
                #[cfg(feature = "sm")] // The Republic of San Marino (Europe)
                IOC::SMR => "SMR",
                #[cfg(feature = "sn")] // The Republic of Senegal (Africa)
                IOC::SEN => "SEN",
                #[cfg(feature = "so")] // The Federal Republic of Somalia (Africa)
                IOC::SOM => "SOM",
                #[cfg(feature = "sr")] // The Republic of Suriname (Americas)
                IOC::SUR => "SUR",
                #[cfg(feature = "st")] // The Democratic Republic of São Tomé and Príncipe (Africa)
                IOC::STP => "STP",
                #[cfg(feature = "sv")] // The Republic of El Salvador (Americas)
                IOC::ESA => "ESA",
                #[cfg(feature = "sy")] // The Syrian Arab Republic (Asia)
                IOC::SYR => "SYR",
                #[cfg(feature = "sz")] // The Kingdom of Eswatini (Africa)
                IOC::SWZ => "SWZ",
                #[cfg(feature = "td")] // The Republic of Chad (Africa)
                IOC::CHA => "CHA",
                #[cfg(feature = "tg")] // The Togolese Republic (Africa)
                IOC::TOG => "TOG",
                #[cfg(feature = "th")] // The Kingdom of Thailand (Asia)
                IOC::THA => "THA",
                #[cfg(feature = "tj")] // The Republic of Tajikistan (Asia)
                IOC::TJK => "TJK",
                #[cfg(feature = "tl")] // The Democratic Republic of Timor-Leste (Asia)
                IOC::TLS => "TLS",
                #[cfg(feature = "tm")] // Turkmenistan (Asia)
                IOC::TKM => "TKM",
                #[cfg(feature = "tn")] // The Republic of Tunisia (Africa)
                IOC::TUN => "TUN",
                #[cfg(feature = "to")] // The Kingdom of Tonga (Oceania)
                IOC::TGA => "TGA",
                #[cfg(feature = "tr")] // The Republic of Turkey (Asia)
                IOC::TUR => "TUR",
                #[cfg(feature = "tt")] // The Republic of Trinidad and Tobago (Americas)
                IOC::TRI => "TRI",
                #[cfg(feature = "tv")] // Tuvalu (Oceania)
                IOC::TUV => "TUV",
                #[cfg(feature = "tw")] // The Republic of China (Asia)
                IOC::TPE => "TPE",
                #[cfg(feature = "tz")] // The United Republic of Tanzania (Africa)
                IOC::TAN => "TAN",
                #[cfg(feature = "ua")] // Ukraine (Europe)
                IOC::UKR => "UKR",
                #[cfg(feature = "ug")] // The Republic of Uganda (Africa)
                IOC::UGA => "UGA",
                #[cfg(feature = "us")] // The United States of America (Americas)
                IOC::USA => "USA",
                #[cfg(feature = "uy")] // The Oriental Republic of Uruguay (Americas)
                IOC::URU => "URU",
                #[cfg(feature = "uz")] // The Republic of Uzbekistan (Asia)
                IOC::UZB => "UZB",
                #[cfg(feature = "vc")] // Saint Vincent and the Grenadines (Americas)
                IOC::VIN => "VIN",
                #[cfg(feature = "ve")] // The Bolivarian Republic of Venezuela (Americas)
                IOC::VEN => "VEN",
                #[cfg(feature = "vg")] // The Virgin Islands (Americas)
                IOC::IVB => "IVB",
                #[cfg(feature = "vi")] // The Virgin Islands of the United States (Americas)
                IOC::ISV => "ISV",
                #[cfg(feature = "vn")] // The Socialist Republic of Viet Nam (Asia)
                IOC::VIE => "VIE",
                #[cfg(feature = "vu")] // The Republic of Vanuatu (Oceania)
                IOC::VAN => "VAN",
                #[cfg(feature = "ws")] // The Independent State of Samoa (Oceania)
                IOC::SAM => "SAM",
                #[cfg(feature = "ye")] // The Republic of Yemen (Asia)
                IOC::YEM => "YEM",
                #[cfg(feature = "za")] // The Republic of South Africa (Africa)
                IOC::RSA => "RSA",
                #[cfg(feature = "zm")] // The Republic of Zambia (Africa)
                IOC::ZAM => "ZAM",
                #[cfg(feature = "zw")] // The Republic of Zimbabwe (Africa)
                IOC::ZIM => "ZIM",
            }
            .to_string()
        }
    }

    impl TryFrom<&str> for IOC {
        type Error = SearchError;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value.len() != 3 {
                return Err(SearchError::BadInput {
                    expected: "a string with three characters",
                });
            }
            match value.to_uppercase().as_str() {
                #[cfg(feature = "ad")] // The Principality of Andorra (Europe)
                "AND" => Ok(IOC::AND),
                #[cfg(feature = "ae")] // The United Arab Emirates (Asia)
                "UAE" => Ok(IOC::UAE),
                #[cfg(feature = "af")] // The Islamic Republic of Afghanistan (Asia)
                "AFG" => Ok(IOC::AFG),
                #[cfg(feature = "ag")] // Antigua and Barbuda (Americas)
                "ANT" => Ok(IOC::ANT),
                #[cfg(feature = "al")] // The Republic of Albania (Europe)
                "ALB" => Ok(IOC::ALB),
                #[cfg(feature = "am")] // The Republic of Armenia (Asia)
                "ARM" => Ok(IOC::ARM),
                #[cfg(feature = "ao")] // The Republic of Angola (Africa)
                "ANG" => Ok(IOC::ANG),
                #[cfg(feature = "ar")] // The Argentine Republic (Americas)
                "ARG" => Ok(IOC::ARG),
                #[cfg(feature = "as")] // The Territory of American Samoa (Oceania)
                "ASA" => Ok(IOC::ASA),
                #[cfg(feature = "at")] // The Republic of Austria (Europe)
                "AUT" => Ok(IOC::AUT),
                #[cfg(feature = "au")] // The Commonwealth of Australia (Oceania)
                "AUS" => Ok(IOC::AUS),
                #[cfg(feature = "aw")] // Aruba (Americas)
                "ARU" => Ok(IOC::ARU),
                #[cfg(feature = "az")] // The Republic of Azerbaijan (Asia)
                "AZE" => Ok(IOC::AZE),
                #[cfg(feature = "ba")] // Bosnia and Herzegovina (Europe)
                "BIH" => Ok(IOC::BIH),
                #[cfg(feature = "bb")] // Barbados (Americas)
                "BAR" => Ok(IOC::BAR),
                #[cfg(feature = "bd")] // The People's Republic of Bangladesh (Asia)
                "BAN" => Ok(IOC::BAN),
                #[cfg(feature = "be")] // The Kingdom of Belgium (Europe)
                "BEL" => Ok(IOC::BEL),
                #[cfg(feature = "bf")] // Burkina Faso (Africa)
                "BUR" => Ok(IOC::BUR),
                #[cfg(feature = "bg")] // The Republic of Bulgaria (Europe)
                "BUL" => Ok(IOC::BUL),
                #[cfg(feature = "bh")] // The Kingdom of Bahrain (Asia)
                "BRN" => Ok(IOC::BRN),
                #[cfg(feature = "bi")] // The Republic of Burundi (Africa)
                "BDI" => Ok(IOC::BDI),
                #[cfg(feature = "bj")] // The Republic of Benin (Africa)
                "BEN" => Ok(IOC::BEN),
                #[cfg(feature = "bm")] // Bermuda (Americas)
                "BER" => Ok(IOC::BER),
                #[cfg(feature = "bn")] // The Nation of Brunei, the Abode of Peace (Asia)
                "BRU" => Ok(IOC::BRU),
                #[cfg(feature = "bo")] // The Plurinational State of Bolivia (Americas)
                "BOL" => Ok(IOC::BOL),
                #[cfg(feature = "br")] // The Federative Republic of Brazil (Americas)
                "BRA" => Ok(IOC::BRA),
                #[cfg(feature = "bs")] // The Commonwealth of The Bahamas (Americas)
                "BAH" => Ok(IOC::BAH),
                #[cfg(feature = "bt")] // The Kingdom of Bhutan (Asia)
                "BHU" => Ok(IOC::BHU),
                #[cfg(feature = "bw")] // The Republic of Botswana (Africa)
                "BOT" => Ok(IOC::BOT),
                #[cfg(feature = "by")] // The Republic of Belarus (Europe)
                "BLR" => Ok(IOC::BLR),
                #[cfg(feature = "bz")] // Belize (Americas)
                "BIZ" => Ok(IOC::BIZ),
                #[cfg(feature = "ca")] // Canada (Americas)
                "CAN" => Ok(IOC::CAN),
                #[cfg(feature = "cd")] // The Democratic Republic of the Congo (Africa)
                "COD" => Ok(IOC::COD),
                #[cfg(feature = "cf")] // The Central African Republic (Africa)
                "CAF" => Ok(IOC::CAF),
                #[cfg(feature = "cg")] // The Republic of the Congo (Africa)
                "CGO" => Ok(IOC::CGO),
                #[cfg(feature = "ch")] // The Swiss Confederation (Europe)
                "SUI" => Ok(IOC::SUI),
                #[cfg(feature = "ci")] // The Republic of Côte d'Ivoire (Africa)
                "CIV" => Ok(IOC::CIV),
                #[cfg(feature = "ck")] // The Cook Islands (Oceania)
                "COK" => Ok(IOC::COK),
                #[cfg(feature = "cl")] // The Republic of Chile (Americas)
                "CHI" => Ok(IOC::CHI),
                #[cfg(feature = "cm")] // The Republic of Cameroon (Africa)
                "CMR" => Ok(IOC::CMR),
                #[cfg(feature = "cn")] // The People's Republic of China (Asia)
                "CHN" => Ok(IOC::CHN),
                #[cfg(feature = "co")] // The Republic of Colombia (Americas)
                "COL" => Ok(IOC::COL),
                #[cfg(feature = "cr")] // The Republic of Costa Rica (Americas)
                "CRC" => Ok(IOC::CRC),
                #[cfg(feature = "cu")] // The Republic of Cuba (Americas)
                "CUB" => Ok(IOC::CUB),
                #[cfg(feature = "cv")] // The Republic of Cabo Verde (Africa)
                "CPV" => Ok(IOC::CPV),
                #[cfg(feature = "cy")] // The Republic of Cyprus (Asia)
                "CYP" => Ok(IOC::CYP),
                #[cfg(feature = "cz")] // The Czech Republic (Europe)
                "CZE" => Ok(IOC::CZE),
                #[cfg(feature = "de")] // The Federal Republic of Germany (Europe)
                "GER" => Ok(IOC::GER),
                #[cfg(feature = "dj")] // The Republic of Djibouti (Africa)
                "DJI" => Ok(IOC::DJI),
                #[cfg(feature = "dk")] // The Kingdom of Denmark (Europe)
                "DEN" => Ok(IOC::DEN),
                #[cfg(feature = "dm")] // The Commonwealth of Dominica (Americas)
                "DMA" => Ok(IOC::DMA),
                #[cfg(feature = "do")] // The Dominican Republic (Americas)
                "DOM" => Ok(IOC::DOM),
                #[cfg(feature = "dz")] // The People's Democratic Republic of Algeria (Africa)
                "ALG" => Ok(IOC::ALG),
                #[cfg(feature = "ec")] // The Republic of Ecuador (Americas)
                "ECU" => Ok(IOC::ECU),
                #[cfg(feature = "ee")] // The Republic of Estonia (Europe)
                "EST" => Ok(IOC::EST),
                #[cfg(feature = "eg")] // The Arab Republic of Egypt (Africa)
                "EGY" => Ok(IOC::EGY),
                #[cfg(feature = "er")] // The State of Eritrea (Africa)
                "ERI" => Ok(IOC::ERI),
                #[cfg(feature = "es")] // The Kingdom of Spain (Europe)
                "ESP" => Ok(IOC::ESP),
                #[cfg(feature = "et")] // The Federal Democratic Republic of Ethiopia (Africa)
                "ETH" => Ok(IOC::ETH),
                #[cfg(feature = "fi")] // The Republic of Finland (Europe)
                "FIN" => Ok(IOC::FIN),
                #[cfg(feature = "fj")] // The Republic of Fiji (Oceania)
                "FIJ" => Ok(IOC::FIJ),
                #[cfg(feature = "fm")] // The Federated States of Micronesia (Oceania)
                "FSM" => Ok(IOC::FSM),
                #[cfg(feature = "fo")] // The Faroe Islands (Europe)
                "FRO" => Ok(IOC::FRO),
                #[cfg(feature = "fr")] // The French Republic (Europe)
                "FRA" => Ok(IOC::FRA),
                #[cfg(feature = "ga")] // The Gabonese Republic (Africa)
                "GAB" => Ok(IOC::GAB),
                #[cfg(feature = "gb")]
                // The United Kingdom of Great Britain and Northern Ireland (Europe)
                "GBR" => Ok(IOC::GBR),
                #[cfg(feature = "gd")] // Grenada (Americas)
                "GRN" => Ok(IOC::GRN),
                #[cfg(feature = "ge")] // Georgia (Asia)
                "GEO" => Ok(IOC::GEO),
                #[cfg(feature = "gh")] // The Republic of Ghana (Africa)
                "GHA" => Ok(IOC::GHA),
                #[cfg(feature = "gm")] // The Republic of The Gambia (Africa)
                "GAM" => Ok(IOC::GAM),
                #[cfg(feature = "gn")] // The Republic of Guinea (Africa)
                "GUI" => Ok(IOC::GUI),
                #[cfg(feature = "gq")] // The Republic of Equatorial Guinea (Africa)
                "GEQ" => Ok(IOC::GEQ),
                #[cfg(feature = "gr")] // The Hellenic Republic (Europe)
                "GRE" => Ok(IOC::GRE),
                #[cfg(feature = "gt")] // The Republic of Guatemala (Americas)
                "GUA" => Ok(IOC::GUA),
                #[cfg(feature = "gu")] // The Territory of Guam (Oceania)
                "GUM" => Ok(IOC::GUM),
                #[cfg(feature = "gw")] // The Republic of Guinea-Bissau (Africa)
                "GBS" => Ok(IOC::GBS),
                #[cfg(feature = "gy")] // The Co-operative Republic of Guyana (Americas)
                "GUY" => Ok(IOC::GUY),
                #[cfg(feature = "hk")]
                // The Hong Kong Special Administrative Region of China (Asia)
                "HKG" => Ok(IOC::HKG),
                #[cfg(feature = "hn")] // The Republic of Honduras (Americas)
                "HON" => Ok(IOC::HON),
                #[cfg(feature = "hr")] // The Republic of Croatia (Europe)
                "CRO" => Ok(IOC::CRO),
                #[cfg(feature = "ht")] // The Republic of Haiti (Americas)
                "HAI" => Ok(IOC::HAI),
                #[cfg(feature = "hu")] // Hungary (Europe)
                "HUN" => Ok(IOC::HUN),
                #[cfg(feature = "id")] // The Republic of Indonesia (Asia)
                "INA" => Ok(IOC::INA),
                #[cfg(feature = "ie")] // Ireland (Europe)
                "IRL" => Ok(IOC::IRL),
                #[cfg(feature = "il")] // The State of Israel (Asia)
                "ISR" => Ok(IOC::ISR),
                #[cfg(feature = "in")] // The Republic of India (Asia)
                "IND" => Ok(IOC::IND),
                #[cfg(feature = "iq")] // The Republic of Iraq (Asia)
                "IRQ" => Ok(IOC::IRQ),
                #[cfg(feature = "ir")] // The Islamic Republic of Iran (Asia)
                "IRI" => Ok(IOC::IRI),
                #[cfg(feature = "is")] // Iceland (Europe)
                "ISL" => Ok(IOC::ISL),
                #[cfg(feature = "it")] // The Italian Republic (Europe)
                "ITA" => Ok(IOC::ITA),
                #[cfg(feature = "jm")] // Jamaica (Americas)
                "JAM" => Ok(IOC::JAM),
                #[cfg(feature = "jo")] // The Hashemite Kingdom of Jordan (Asia)
                "JOR" => Ok(IOC::JOR),
                #[cfg(feature = "jp")] // Japan (Asia)
                "JPN" => Ok(IOC::JPN),
                #[cfg(feature = "ke")] // The Republic of Kenya (Africa)
                "KEN" => Ok(IOC::KEN),
                #[cfg(feature = "kg")] // The Kyrgyz Republic (Asia)
                "KGZ" => Ok(IOC::KGZ),
                #[cfg(feature = "kh")] // The Kingdom of Cambodia (Asia)
                "CAM" => Ok(IOC::CAM),
                #[cfg(feature = "ki")] // The Republic of Kiribati (Oceania)
                "KIR" => Ok(IOC::KIR),
                #[cfg(feature = "km")] // The Union of the Comoros (Africa)
                "COM" => Ok(IOC::COM),
                #[cfg(feature = "kn")] // Saint Kitts and Nevis (Americas)
                "SKN" => Ok(IOC::SKN),
                #[cfg(feature = "kp")] // The Democratic People's Republic of Korea (Asia)
                "PRK" => Ok(IOC::PRK),
                #[cfg(feature = "kr")] // The Republic of Korea (Asia)
                "KOR" => Ok(IOC::KOR),
                #[cfg(feature = "kw")] // The State of Kuwait (Asia)
                "KUW" => Ok(IOC::KUW),
                #[cfg(feature = "ky")] // The Cayman Islands (Americas)
                "CAY" => Ok(IOC::CAY),
                #[cfg(feature = "kz")] // The Republic of Kazakhstan (Asia)
                "KAZ" => Ok(IOC::KAZ),
                #[cfg(feature = "la")] // The Lao People's Democratic Republic (Asia)
                "LAO" => Ok(IOC::LAO),
                #[cfg(feature = "lb")] // The Lebanese Republic (Asia)
                "LIB" => Ok(IOC::LIB),
                #[cfg(feature = "lc")] // Saint Lucia (Americas)
                "LCA" => Ok(IOC::LCA),
                #[cfg(feature = "li")] // The Principality of Liechtenstein (Europe)
                "LIE" => Ok(IOC::LIE),
                #[cfg(feature = "lk")] // The Democratic Socialist Republic of Sri Lanka (Asia)
                "SRI" => Ok(IOC::SRI),
                #[cfg(feature = "lr")] // The Republic of Liberia (Africa)
                "LBR" => Ok(IOC::LBR),
                #[cfg(feature = "ls")] // The Kingdom of Lesotho (Africa)
                "LES" => Ok(IOC::LES),
                #[cfg(feature = "lt")] // The Republic of Lithuania (Europe)
                "LTU" => Ok(IOC::LTU),
                #[cfg(feature = "lu")] // The Grand Duchy of Luxembourg (Europe)
                "LUX" => Ok(IOC::LUX),
                #[cfg(feature = "lv")] // The Republic of Latvia (Europe)
                "LAT" => Ok(IOC::LAT),
                #[cfg(feature = "ly")] // The State of Libya (Africa)
                "LBA" => Ok(IOC::LBA),
                #[cfg(feature = "ma")] // The Kingdom of Morocco (Africa)
                "MAR" => Ok(IOC::MAR),
                #[cfg(feature = "mc")] // The Principality of Monaco (Europe)
                "MON" => Ok(IOC::MON),
                #[cfg(feature = "md")] // The Republic of Moldova (Europe)
                "MDA" => Ok(IOC::MDA),
                #[cfg(feature = "me")] // Montenegro (Europe)
                "MNE" => Ok(IOC::MNE),
                #[cfg(feature = "mg")] // The Republic of Madagascar (Africa)
                "MAD" => Ok(IOC::MAD),
                #[cfg(feature = "mh")] // The Republic of the Marshall Islands (Oceania)
                "MHL" => Ok(IOC::MHL),
                #[cfg(feature = "mk")] // The Republic of North Macedonia (Europe)
                "MKD" => Ok(IOC::MKD),
                #[cfg(feature = "ml")] // The Republic of Mali (Africa)
                "MLI" => Ok(IOC::MLI),
                #[cfg(feature = "mm")] // The Republic of the Union of Myanmar (Asia)
                "MYA" => Ok(IOC::MYA),
                #[cfg(feature = "mn")] // Mongolia (Asia)
                "MGL" => Ok(IOC::MGL),
                #[cfg(feature = "mr")] // The Islamic Republic of Mauritania (Africa)
                "MTN" => Ok(IOC::MTN),
                #[cfg(feature = "mt")] // The Republic of Malta (Europe)
                "MLT" => Ok(IOC::MLT),
                #[cfg(feature = "mu")] // The Republic of Mauritius (Africa)
                "MRI" => Ok(IOC::MRI),
                #[cfg(feature = "mv")] // The Republic of Maldives (Asia)
                "MDV" => Ok(IOC::MDV),
                #[cfg(feature = "mw")] // The Republic of Malawi (Africa)
                "MAW" => Ok(IOC::MAW),
                #[cfg(feature = "mx")] // The United Mexican States (Americas)
                "MEX" => Ok(IOC::MEX),
                #[cfg(feature = "my")] // Malaysia (Asia)
                "MAS" => Ok(IOC::MAS),
                #[cfg(feature = "mz")] // The Republic of Mozambique (Africa)
                "MOZ" => Ok(IOC::MOZ),
                #[cfg(feature = "na")] // The Republic of Namibia (Africa)
                "NAM" => Ok(IOC::NAM),
                #[cfg(feature = "ne")] // The Republic of the Niger (Africa)
                "NIG" => Ok(IOC::NIG),
                #[cfg(feature = "ng")] // The Federal Republic of Nigeria (Africa)
                "NGR" => Ok(IOC::NGR),
                #[cfg(feature = "ni")] // The Republic of Nicaragua (Americas)
                "NCA" => Ok(IOC::NCA),
                #[cfg(feature = "nl")] // The Kingdom of the Netherlands (Europe)
                "NED" => Ok(IOC::NED),
                #[cfg(feature = "no")] // The Kingdom of Norway (Europe)
                "NOR" => Ok(IOC::NOR),
                #[cfg(feature = "np")] // The Federal Democratic Republic of Nepal (Asia)
                "NEP" => Ok(IOC::NEP),
                #[cfg(feature = "nr")] // The Republic of Nauru (Oceania)
                "NRU" => Ok(IOC::NRU),
                #[cfg(feature = "nz")] // New Zealand (Oceania)
                "NZL" => Ok(IOC::NZL),
                #[cfg(feature = "om")] // The Sultanate of Oman (Asia)
                "OMA" => Ok(IOC::OMA),
                #[cfg(feature = "pa")] // The Republic of Panamá (Americas)
                "PAN" => Ok(IOC::PAN),
                #[cfg(feature = "pe")] // The Republic of Perú (Americas)
                "PER" => Ok(IOC::PER),
                #[cfg(feature = "pg")] // The Independent State of Papua New Guinea (Oceania)
                "PNG" => Ok(IOC::PNG),
                #[cfg(feature = "ph")] // The Republic of the Philippines (Asia)
                "PHI" => Ok(IOC::PHI),
                #[cfg(feature = "pk")] // The Islamic Republic of Pakistan (Asia)
                "PAK" => Ok(IOC::PAK),
                #[cfg(feature = "pl")] // The Republic of Poland (Europe)
                "POL" => Ok(IOC::POL),
                #[cfg(feature = "pr")] // The Commonwealth of Puerto Rico (Americas)
                "PUR" => Ok(IOC::PUR),
                #[cfg(feature = "ps")] // The State of Palestine (Asia)
                "PLE" => Ok(IOC::PLE),
                #[cfg(feature = "pt")] // The Portuguese Republic (Europe)
                "POR" => Ok(IOC::POR),
                #[cfg(feature = "pw")] // The Republic of Palau (Oceania)
                "PLW" => Ok(IOC::PLW),
                #[cfg(feature = "py")] // The Republic of Paraguay (Americas)
                "PAR" => Ok(IOC::PAR),
                #[cfg(feature = "qa")] // The State of Qatar (Asia)
                "QAT" => Ok(IOC::QAT),
                #[cfg(feature = "ro")] // Romania (Europe)
                "ROU" => Ok(IOC::ROU),
                #[cfg(feature = "rs")] // The Republic of Serbia (Europe)
                "SRB" => Ok(IOC::SRB),
                #[cfg(feature = "ru")] // The Russian Federation (Europe)
                "RUS" => Ok(IOC::RUS),
                #[cfg(feature = "rw")] // The Republic of Rwanda (Africa)
                "RWA" => Ok(IOC::RWA),
                #[cfg(feature = "sa")] // The Kingdom of Saudi Arabia (Asia)
                "KSA" => Ok(IOC::KSA),
                #[cfg(feature = "sb")] // The Solomon Islands (Oceania)
                "SOL" => Ok(IOC::SOL),
                #[cfg(feature = "sc")] // The Republic of Seychelles (Africa)
                "SEY" => Ok(IOC::SEY),
                #[cfg(feature = "sd")] // The Republic of the Sudan (Africa)
                "SUD" => Ok(IOC::SUD),
                #[cfg(feature = "se")] // The Kingdom of Sweden (Europe)
                "SWE" => Ok(IOC::SWE),
                #[cfg(feature = "sg")] // The Republic of Singapore (Asia)
                "SGP" => Ok(IOC::SGP),
                #[cfg(feature = "si")] // The Republic of Slovenia (Europe)
                "SLO" => Ok(IOC::SLO),
                #[cfg(feature = "sk")] // The Slovak Republic (Europe)
                "SVK" => Ok(IOC::SVK),
                #[cfg(feature = "sl")] // The Republic of Sierra Leone (Africa)
                "SLE" => Ok(IOC::SLE),
                #[cfg(feature = "sm")] // The Republic of San Marino (Europe)
                "SMR" => Ok(IOC::SMR),
                #[cfg(feature = "sn")] // The Republic of Senegal (Africa)
                "SEN" => Ok(IOC::SEN),
                #[cfg(feature = "so")] // The Federal Republic of Somalia (Africa)
                "SOM" => Ok(IOC::SOM),
                #[cfg(feature = "sr")] // The Republic of Suriname (Americas)
                "SUR" => Ok(IOC::SUR),
                #[cfg(feature = "st")] // The Democratic Republic of São Tomé and Príncipe (Africa)
                "STP" => Ok(IOC::STP),
                #[cfg(feature = "sv")] // The Republic of El Salvador (Americas)
                "ESA" => Ok(IOC::ESA),
                #[cfg(feature = "sy")] // The Syrian Arab Republic (Asia)
                "SYR" => Ok(IOC::SYR),
                #[cfg(feature = "sz")] // The Kingdom of Eswatini (Africa)
                "SWZ" => Ok(IOC::SWZ),
                #[cfg(feature = "td")] // The Republic of Chad (Africa)
                "CHA" => Ok(IOC::CHA),
                #[cfg(feature = "tg")] // The Togolese Republic (Africa)
                "TOG" => Ok(IOC::TOG),
                #[cfg(feature = "th")] // The Kingdom of Thailand (Asia)
                "THA" => Ok(IOC::THA),
                #[cfg(feature = "tj")] // The Republic of Tajikistan (Asia)
                "TJK" => Ok(IOC::TJK),
                #[cfg(feature = "tl")] // The Democratic Republic of Timor-Leste (Asia)
                "TLS" => Ok(IOC::TLS),
                #[cfg(feature = "tm")] // Turkmenistan (Asia)
                "TKM" => Ok(IOC::TKM),
                #[cfg(feature = "tn")] // The Republic of Tunisia (Africa)
                "TUN" => Ok(IOC::TUN),
                #[cfg(feature = "to")] // The Kingdom of Tonga (Oceania)
                "TGA" => Ok(IOC::TGA),
                #[cfg(feature = "tr")] // The Republic of Turkey (Asia)
                "TUR" => Ok(IOC::TUR),
                #[cfg(feature = "tt")] // The Republic of Trinidad and Tobago (Americas)
                "TRI" => Ok(IOC::TRI),
                #[cfg(feature = "tv")] // Tuvalu (Oceania)
                "TUV" => Ok(IOC::TUV),
                #[cfg(feature = "tw")] // The Republic of China (Asia)
                "TPE" => Ok(IOC::TPE),
                #[cfg(feature = "tz")] // The United Republic of Tanzania (Africa)
                "TAN" => Ok(IOC::TAN),
                #[cfg(feature = "ua")] // Ukraine (Europe)
                "UKR" => Ok(IOC::UKR),
                #[cfg(feature = "ug")] // The Republic of Uganda (Africa)
                "UGA" => Ok(IOC::UGA),
                #[cfg(feature = "us")] // The United States of America (Americas)
                "USA" => Ok(IOC::USA),
                #[cfg(feature = "uy")] // The Oriental Republic of Uruguay (Americas)
                "URU" => Ok(IOC::URU),
                #[cfg(feature = "uz")] // The Republic of Uzbekistan (Asia)
                "UZB" => Ok(IOC::UZB),
                #[cfg(feature = "vc")] // Saint Vincent and the Grenadines (Americas)
                "VIN" => Ok(IOC::VIN),
                #[cfg(feature = "ve")] // The Bolivarian Republic of Venezuela (Americas)
                "VEN" => Ok(IOC::VEN),
                #[cfg(feature = "vg")] // The Virgin Islands (Americas)
                "IVB" => Ok(IOC::IVB),
                #[cfg(feature = "vi")] // The Virgin Islands of the United States (Americas)
                "ISV" => Ok(IOC::ISV),
                #[cfg(feature = "vn")] // The Socialist Republic of Viet Nam (Asia)
                "VIE" => Ok(IOC::VIE),
                #[cfg(feature = "vu")] // The Republic of Vanuatu (Oceania)
                "VAN" => Ok(IOC::VAN),
                #[cfg(feature = "ws")] // The Independent State of Samoa (Oceania)
                "SAM" => Ok(IOC::SAM),
                #[cfg(feature = "ye")] // The Republic of Yemen (Asia)
                "YEM" => Ok(IOC::YEM),
                #[cfg(feature = "za")] // The Republic of South Africa (Africa)
                "RSA" => Ok(IOC::RSA),
                #[cfg(feature = "zm")] // The Republic of Zambia (Africa)
                "ZAM" => Ok(IOC::ZAM),
                #[cfg(feature = "zw")] // The Republic of Zimbabwe (Africa)
                "ZIM" => Ok(IOC::ZIM),
                #[cfg(all(
                    feature = "ad",
                    feature = "ae",
                    feature = "af",
                    feature = "ag",
                    feature = "al",
                    feature = "am",
                    feature = "ao",
                    feature = "ar",
                    feature = "as",
                    feature = "at",
                    feature = "au",
                    feature = "aw",
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
                    feature = "bm",
                    feature = "bn",
                    feature = "bo",
                    feature = "br",
                    feature = "bs",
                    feature = "bt",
                    feature = "bw",
                    feature = "by",
                    feature = "bz",
                    feature = "ca",
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
                    feature = "er",
                    feature = "es",
                    feature = "et",
                    feature = "fi",
                    feature = "fj",
                    feature = "fm",
                    feature = "fo",
                    feature = "fr",
                    feature = "ga",
                    feature = "gb",
                    feature = "gd",
                    feature = "ge",
                    feature = "gh",
                    feature = "gm",
                    feature = "gn",
                    feature = "gq",
                    feature = "gr",
                    feature = "gt",
                    feature = "gu",
                    feature = "gw",
                    feature = "gy",
                    feature = "hk",
                    feature = "hn",
                    feature = "hr",
                    feature = "ht",
                    feature = "hu",
                    feature = "id",
                    feature = "ie",
                    feature = "il",
                    feature = "in",
                    feature = "iq",
                    feature = "ir",
                    feature = "is",
                    feature = "it",
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
                    feature = "mg",
                    feature = "mh",
                    feature = "mk",
                    feature = "ml",
                    feature = "mm",
                    feature = "mn",
                    feature = "mr",
                    feature = "mt",
                    feature = "mu",
                    feature = "mv",
                    feature = "mw",
                    feature = "mx",
                    feature = "my",
                    feature = "mz",
                    feature = "na",
                    feature = "ne",
                    feature = "ng",
                    feature = "ni",
                    feature = "nl",
                    feature = "no",
                    feature = "np",
                    feature = "nr",
                    feature = "nz",
                    feature = "om",
                    feature = "pa",
                    feature = "pe",
                    feature = "pg",
                    feature = "ph",
                    feature = "pk",
                    feature = "pl",
                    feature = "pr",
                    feature = "ps",
                    feature = "pt",
                    feature = "pw",
                    feature = "py",
                    feature = "qa",
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
                    feature = "si",
                    feature = "sk",
                    feature = "sl",
                    feature = "sm",
                    feature = "sn",
                    feature = "so",
                    feature = "sr",
                    feature = "st",
                    feature = "sv",
                    feature = "sy",
                    feature = "sz",
                    feature = "td",
                    feature = "tg",
                    feature = "th",
                    feature = "tj",
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
                    feature = "us",
                    feature = "uy",
                    feature = "uz",
                    feature = "vc",
                    feature = "ve",
                    feature = "vg",
                    feature = "vi",
                    feature = "vn",
                    feature = "vu",
                    feature = "ws",
                    feature = "ye",
                    feature = "za",
                    feature = "zm",
                    feature = "zw",
                ))]
                _ => Err(SearchError::NotFound {
                    searched_items: SearchedItems::AllCountries,
                }),
                #[allow(unreachable_patterns)]
                _ => Err(SearchError::NotFound {
                    searched_items: SearchedItems::SupportedCountries(
                        *crate::consts::SUPPORTED_COUNTRIES_COUNT,
                    ),
                }),
            }
        }
    }
}
#[cfg(not(any(
    feature = "ad",
    feature = "ae",
    feature = "af",
    feature = "ag",
    feature = "al",
    feature = "am",
    feature = "ao",
    feature = "ar",
    feature = "as",
    feature = "at",
    feature = "au",
    feature = "aw",
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
    feature = "bm",
    feature = "bn",
    feature = "bo",
    feature = "br",
    feature = "bs",
    feature = "bt",
    feature = "bw",
    feature = "by",
    feature = "bz",
    feature = "ca",
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
    feature = "er",
    feature = "es",
    feature = "et",
    feature = "fi",
    feature = "fj",
    feature = "fm",
    feature = "fo",
    feature = "fr",
    feature = "ga",
    feature = "gb",
    feature = "gd",
    feature = "ge",
    feature = "gh",
    feature = "gm",
    feature = "gn",
    feature = "gq",
    feature = "gr",
    feature = "gt",
    feature = "gu",
    feature = "gw",
    feature = "gy",
    feature = "hk",
    feature = "hn",
    feature = "hr",
    feature = "ht",
    feature = "hu",
    feature = "id",
    feature = "ie",
    feature = "il",
    feature = "in",
    feature = "iq",
    feature = "ir",
    feature = "is",
    feature = "it",
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
    feature = "mg",
    feature = "mh",
    feature = "mk",
    feature = "ml",
    feature = "mm",
    feature = "mn",
    feature = "mr",
    feature = "mt",
    feature = "mu",
    feature = "mv",
    feature = "mw",
    feature = "mx",
    feature = "my",
    feature = "mz",
    feature = "na",
    feature = "ne",
    feature = "ng",
    feature = "ni",
    feature = "nl",
    feature = "no",
    feature = "np",
    feature = "nr",
    feature = "nz",
    feature = "om",
    feature = "pa",
    feature = "pe",
    feature = "pg",
    feature = "ph",
    feature = "pk",
    feature = "pl",
    feature = "pr",
    feature = "ps",
    feature = "pt",
    feature = "pw",
    feature = "py",
    feature = "qa",
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
    feature = "si",
    feature = "sk",
    feature = "sl",
    feature = "sm",
    feature = "sn",
    feature = "so",
    feature = "sr",
    feature = "st",
    feature = "sv",
    feature = "sy",
    feature = "sz",
    feature = "td",
    feature = "tg",
    feature = "th",
    feature = "tj",
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
    feature = "us",
    feature = "uy",
    feature = "uz",
    feature = "vc",
    feature = "ve",
    feature = "vg",
    feature = "vi",
    feature = "vn",
    feature = "vu",
    feature = "ws",
    feature = "ye",
    feature = "za",
    feature = "zm",
    feature = "zw",
)))]
mod impls {
    use super::IOC;
    use crate::{Alpha2, Country, SearchError};

    impl IOC {
        pub fn to_alpha2(&self) -> Alpha2 {
            unimplemented!("No country feature with IOC code is used");
        }

        pub fn to_country(&self) -> Country {
            unimplemented!("No country feature with IOC code is used");
        }
    }

    impl ToString for IOC {
        fn to_string(&self) -> String {
            unimplemented!("No country feature with IOC code is used");
        }
    }

    impl TryFrom<&str> for IOC {
        type Error = SearchError;
        fn try_from(_value: &str) -> Result<Self, Self::Error> {
            unimplemented!("No country feature with IOC code is used");
        }
    }
}