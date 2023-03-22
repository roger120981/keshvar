// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/gec.rs`)

#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
/// An enum containing `Geopolitical Entities and Codes` (GEC).
///
/// # Example
/// ```
/// use keshvar::{GEC, Alpha2};
///
/// assert_eq!(Ok(GEC::FP), GEC::try_from("fp"));
/// assert_eq!(GEC::FP.to_alpha2(), Alpha2::PF); // French Polynesia (Oceania)
/// ```
/// We usually need to convert [`Alpha2`](crate::Alpha2) to [`Country`](crate::Country) and use that object instead.
pub enum GEC {
    #[cfg(feature = "ad")]
    /// The Principality of Andorra (Europe)
    AN,
    #[cfg(feature = "ae")]
    /// The United Arab Emirates (Asia)
    AE,
    #[cfg(feature = "af")]
    /// The Islamic Republic of Afghanistan (Asia)
    AF,
    #[cfg(feature = "ag")]
    /// Antigua and Barbuda (Americas)
    AC,
    #[cfg(feature = "ai")]
    /// Anguilla (Americas)
    AV,
    #[cfg(feature = "al")]
    /// The Republic of Albania (Europe)
    AL,
    #[cfg(feature = "am")]
    /// The Republic of Armenia (Asia)
    AM,
    #[cfg(feature = "ao")]
    /// The Republic of Angola (Africa)
    AO,
    #[cfg(feature = "aq")]
    /// Antarctica
    AY,
    #[cfg(feature = "ar")]
    /// The Argentine Republic (Americas)
    AR,
    #[cfg(feature = "as")]
    /// The Territory of American Samoa (Oceania)
    AQ,
    #[cfg(feature = "at")]
    /// The Republic of Austria (Europe)
    AU,
    #[cfg(feature = "au")]
    /// The Commonwealth of Australia (Oceania)
    AS,
    #[cfg(feature = "aw")]
    /// Aruba (Americas)
    AA,
    #[cfg(feature = "az")]
    /// The Republic of Azerbaijan (Asia)
    AJ,
    #[cfg(feature = "ba")]
    /// Bosnia and Herzegovina (Europe)
    BK,
    #[cfg(feature = "bb")]
    /// Barbados (Americas)
    BB,
    #[cfg(feature = "bd")]
    /// The People's Republic of Bangladesh (Asia)
    BG,
    #[cfg(feature = "be")]
    /// The Kingdom of Belgium (Europe)
    BE,
    #[cfg(feature = "bf")]
    /// Burkina Faso (Africa)
    UV,
    #[cfg(feature = "bg")]
    /// The Republic of Bulgaria (Europe)
    BU,
    #[cfg(feature = "bh")]
    /// The Kingdom of Bahrain (Asia)
    BA,
    #[cfg(feature = "bi")]
    /// The Republic of Burundi (Africa)
    BY,
    #[cfg(feature = "bj")]
    /// The Republic of Benin (Africa)
    BN,
    #[cfg(feature = "bl")]
    /// The Collectivity of Saint-Barthélemy (Americas)
    TB,
    #[cfg(feature = "bm")]
    /// Bermuda (Americas)
    BD,
    #[cfg(feature = "bn")]
    /// The Nation of Brunei, the Abode of Peace (Asia)
    BX,
    #[cfg(feature = "bo")]
    /// The Plurinational State of Bolivia (Americas)
    BL,
    #[cfg(feature = "br")]
    /// The Federative Republic of Brazil (Americas)
    BR,
    #[cfg(feature = "bs")]
    /// The Commonwealth of The Bahamas (Americas)
    BF,
    #[cfg(feature = "bt")]
    /// The Kingdom of Bhutan (Asia)
    BT,
    #[cfg(feature = "bv")]
    /// Bouvet Island
    BV,
    #[cfg(feature = "bw")]
    /// The Republic of Botswana (Africa)
    BC,
    #[cfg(feature = "by")]
    /// The Republic of Belarus (Europe)
    BO,
    #[cfg(feature = "bz")]
    /// Belize (Americas)
    BH,
    #[cfg(feature = "ca")]
    /// Canada (Americas)
    CA,
    #[cfg(feature = "cc")]
    /// The Territory of Cocos (Keeling) Islands (Oceania)
    CK,
    #[cfg(feature = "cd")]
    /// The Democratic Republic of the Congo (Africa)
    CG,
    #[cfg(feature = "cf")]
    /// The Central African Republic (Africa)
    CT,
    #[cfg(feature = "cg")]
    /// The Republic of the Congo (Africa)
    CF,
    #[cfg(feature = "ch")]
    /// The Swiss Confederation (Europe)
    SZ,
    #[cfg(feature = "ci")]
    /// The Republic of Côte d'Ivoire (Africa)
    IV,
    #[cfg(feature = "ck")]
    /// The Cook Islands (Oceania)
    CW,
    #[cfg(feature = "cl")]
    /// The Republic of Chile (Americas)
    CI,
    #[cfg(feature = "cm")]
    /// The Republic of Cameroon (Africa)
    CM,
    #[cfg(feature = "cn")]
    /// The People's Republic of China (Asia)
    CH,
    #[cfg(feature = "co")]
    /// The Republic of Colombia (Americas)
    CO,
    #[cfg(feature = "cr")]
    /// The Republic of Costa Rica (Americas)
    CS,
    #[cfg(feature = "cu")]
    /// The Republic of Cuba (Americas)
    CU,
    #[cfg(feature = "cv")]
    /// The Republic of Cabo Verde (Africa)
    CV,
    #[cfg(feature = "cw")]
    /// The Country of Curaçao (Americas)
    UC,
    #[cfg(feature = "cx")]
    /// The Territory of Christmas Island (Oceania)
    KT,
    #[cfg(feature = "cy")]
    /// The Republic of Cyprus (Asia)
    CY,
    #[cfg(feature = "cz")]
    /// The Czech Republic (Europe)
    EZ,
    #[cfg(feature = "de")]
    /// The Federal Republic of Germany (Europe)
    GM,
    #[cfg(feature = "dj")]
    /// The Republic of Djibouti (Africa)
    DJ,
    #[cfg(feature = "dk")]
    /// The Kingdom of Denmark (Europe)
    DA,
    #[cfg(feature = "dm")]
    /// The Commonwealth of Dominica (Americas)
    DO,
    #[cfg(feature = "do")]
    /// The Dominican Republic (Americas)
    DR,
    #[cfg(feature = "dz")]
    /// The People's Democratic Republic of Algeria (Africa)
    AG,
    #[cfg(feature = "ec")]
    /// The Republic of Ecuador (Americas)
    EC,
    #[cfg(feature = "ee")]
    /// The Republic of Estonia (Europe)
    EN,
    #[cfg(feature = "eg")]
    /// The Arab Republic of Egypt (Africa)
    EG,
    #[cfg(feature = "eh")]
    /// The Sahrawi Arab Democratic Republic (Africa)
    WI,
    #[cfg(feature = "er")]
    /// The State of Eritrea (Africa)
    ER,
    #[cfg(feature = "es")]
    /// The Kingdom of Spain (Europe)
    SP,
    #[cfg(feature = "et")]
    /// The Federal Democratic Republic of Ethiopia (Africa)
    ET,
    #[cfg(feature = "fi")]
    /// The Republic of Finland (Europe)
    FI,
    #[cfg(feature = "fj")]
    /// The Republic of Fiji (Oceania)
    FJ,
    #[cfg(feature = "fk")]
    /// The Falkland Islands (Americas)
    FK,
    #[cfg(feature = "fm")]
    /// The Federated States of Micronesia (Oceania)
    FM,
    #[cfg(feature = "fo")]
    /// The Faroe Islands (Europe)
    FO,
    #[cfg(feature = "fr")]
    /// The French Republic (Europe)
    FR,
    #[cfg(feature = "ga")]
    /// The Gabonese Republic (Africa)
    GB,
    #[cfg(feature = "gb")]
    /// The United Kingdom of Great Britain and Northern Ireland (Europe)
    UK,
    #[cfg(feature = "gd")]
    /// Grenada (Americas)
    GJ,
    #[cfg(feature = "ge")]
    /// Georgia (Asia)
    GG,
    #[cfg(feature = "gf")]
    /// Guyane (Americas)
    FG,
    #[cfg(feature = "gg")]
    /// The Bailiwick of Guernsey (Europe)
    GK,
    #[cfg(feature = "gh")]
    /// The Republic of Ghana (Africa)
    GH,
    #[cfg(feature = "gi")]
    /// Gibraltar (Europe)
    GI,
    #[cfg(feature = "gl")]
    /// Kalaallit Nunaat (Americas)
    GL,
    #[cfg(feature = "gm")]
    /// The Republic of The Gambia (Africa)
    GA,
    #[cfg(feature = "gn")]
    /// The Republic of Guinea (Africa)
    GV,
    #[cfg(feature = "gp")]
    /// Guadeloupe (Americas)
    GP,
    #[cfg(feature = "gq")]
    /// The Republic of Equatorial Guinea (Africa)
    EK,
    #[cfg(feature = "gr")]
    /// The Hellenic Republic (Europe)
    GR,
    #[cfg(feature = "gs")]
    /// South Georgia and the South Sandwich Islands (Americas)
    SX,
    #[cfg(feature = "gt")]
    /// The Republic of Guatemala (Americas)
    GT,
    #[cfg(feature = "gu")]
    /// The Territory of Guam (Oceania)
    GQ,
    #[cfg(feature = "gw")]
    /// The Republic of Guinea-Bissau (Africa)
    PU,
    #[cfg(feature = "gy")]
    /// The Co-operative Republic of Guyana (Americas)
    GY,
    #[cfg(feature = "hk")]
    /// The Hong Kong Special Administrative Region of China (Asia)
    HK,
    #[cfg(feature = "hm")]
    /// The Territory of Heard Island and McDonald Islands
    HM,
    #[cfg(feature = "hn")]
    /// The Republic of Honduras (Americas)
    HO,
    #[cfg(feature = "hr")]
    /// The Republic of Croatia (Europe)
    HR,
    #[cfg(feature = "ht")]
    /// The Republic of Haiti (Americas)
    HA,
    #[cfg(feature = "hu")]
    /// Hungary (Europe)
    HU,
    #[cfg(feature = "id")]
    /// The Republic of Indonesia (Asia)
    ID,
    #[cfg(feature = "ie")]
    /// Ireland (Europe)
    EI,
    #[cfg(feature = "il")]
    /// The State of Israel (Asia)
    IS,
    #[cfg(feature = "im")]
    /// The Isle of Man (Europe)
    IM,
    #[cfg(feature = "in")]
    /// The Republic of India (Asia)
    IN,
    #[cfg(feature = "io")]
    /// The British Indian Ocean Territory (Africa)
    IO,
    #[cfg(feature = "iq")]
    /// The Republic of Iraq (Asia)
    IZ,
    #[cfg(feature = "ir")]
    /// The Islamic Republic of Iran (Asia)
    IR,
    #[cfg(feature = "is")]
    /// Iceland (Europe)
    IC,
    #[cfg(feature = "it")]
    /// The Italian Republic (Europe)
    IT,
    #[cfg(feature = "je")]
    /// The Bailiwick of Jersey (Europe)
    JE,
    #[cfg(feature = "jm")]
    /// Jamaica (Americas)
    JM,
    #[cfg(feature = "jo")]
    /// The Hashemite Kingdom of Jordan (Asia)
    JO,
    #[cfg(feature = "jp")]
    /// Japan (Asia)
    JA,
    #[cfg(feature = "ke")]
    /// The Republic of Kenya (Africa)
    KE,
    #[cfg(feature = "kg")]
    /// The Kyrgyz Republic (Asia)
    KG,
    #[cfg(feature = "kh")]
    /// The Kingdom of Cambodia (Asia)
    CB,
    #[cfg(feature = "ki")]
    /// The Republic of Kiribati (Oceania)
    KR,
    #[cfg(feature = "km")]
    /// The Union of the Comoros (Africa)
    CN,
    #[cfg(feature = "kn")]
    /// Saint Kitts and Nevis (Americas)
    SC,
    #[cfg(feature = "kp")]
    /// The Democratic People's Republic of Korea (Asia)
    KN,
    #[cfg(feature = "kr")]
    /// The Republic of Korea (Asia)
    KS,
    #[cfg(feature = "kw")]
    /// The State of Kuwait (Asia)
    KU,
    #[cfg(feature = "ky")]
    /// The Cayman Islands (Americas)
    CJ,
    #[cfg(feature = "kz")]
    /// The Republic of Kazakhstan (Asia)
    KZ,
    #[cfg(feature = "la")]
    /// The Lao People's Democratic Republic (Asia)
    LA,
    #[cfg(feature = "lb")]
    /// The Lebanese Republic (Asia)
    LE,
    #[cfg(feature = "lc")]
    /// Saint Lucia (Americas)
    ST,
    #[cfg(feature = "li")]
    /// The Principality of Liechtenstein (Europe)
    LS,
    #[cfg(feature = "lk")]
    /// The Democratic Socialist Republic of Sri Lanka (Asia)
    CE,
    #[cfg(feature = "lr")]
    /// The Republic of Liberia (Africa)
    LI,
    #[cfg(feature = "ls")]
    /// The Kingdom of Lesotho (Africa)
    LT,
    #[cfg(feature = "lt")]
    /// The Republic of Lithuania (Europe)
    LH,
    #[cfg(feature = "lu")]
    /// The Grand Duchy of Luxembourg (Europe)
    LU,
    #[cfg(feature = "lv")]
    /// The Republic of Latvia (Europe)
    LG,
    #[cfg(feature = "ly")]
    /// The State of Libya (Africa)
    LY,
    #[cfg(feature = "ma")]
    /// The Kingdom of Morocco (Africa)
    MO,
    #[cfg(feature = "mc")]
    /// The Principality of Monaco (Europe)
    MN,
    #[cfg(feature = "md")]
    /// The Republic of Moldova (Europe)
    MD,
    #[cfg(feature = "me")]
    /// Montenegro (Europe)
    MJ,
    #[cfg(feature = "mf")]
    /// The Collectivity of Saint-Martin (Americas)
    RN,
    #[cfg(feature = "mg")]
    /// The Republic of Madagascar (Africa)
    MA,
    #[cfg(feature = "mh")]
    /// The Republic of the Marshall Islands (Oceania)
    RM,
    #[cfg(feature = "mk")]
    /// The Republic of North Macedonia (Europe)
    MK,
    #[cfg(feature = "ml")]
    /// The Republic of Mali (Africa)
    ML,
    #[cfg(feature = "mm")]
    /// The Republic of the Union of Myanmar (Asia)
    BM,
    #[cfg(feature = "mn")]
    /// Mongolia (Asia)
    MG,
    #[cfg(feature = "mo")]
    /// The Macao Special Administrative Region of China (Asia)
    MC,
    #[cfg(feature = "mp")]
    /// The Commonwealth of the Northern Mariana Islands (Oceania)
    CQ,
    #[cfg(feature = "mq")]
    /// Martinique (Americas)
    MB,
    #[cfg(feature = "mr")]
    /// The Islamic Republic of Mauritania (Africa)
    MR,
    #[cfg(feature = "ms")]
    /// Montserrat (Americas)
    MH,
    #[cfg(feature = "mt")]
    /// The Republic of Malta (Europe)
    MT,
    #[cfg(feature = "mu")]
    /// The Republic of Mauritius (Africa)
    MP,
    #[cfg(feature = "mv")]
    /// The Republic of Maldives (Asia)
    MV,
    #[cfg(feature = "mw")]
    /// The Republic of Malawi (Africa)
    MI,
    #[cfg(feature = "mx")]
    /// The United Mexican States (Americas)
    MX,
    #[cfg(feature = "my")]
    /// Malaysia (Asia)
    MY,
    #[cfg(feature = "mz")]
    /// The Republic of Mozambique (Africa)
    MZ,
    #[cfg(feature = "na")]
    /// The Republic of Namibia (Africa)
    WA,
    #[cfg(feature = "nc")]
    /// New Caledonia (Oceania)
    NC,
    #[cfg(feature = "ne")]
    /// The Republic of the Niger (Africa)
    NG,
    #[cfg(feature = "nf")]
    /// The Territory of Norfolk Island (Oceania)
    NF,
    #[cfg(feature = "ng")]
    /// The Federal Republic of Nigeria (Africa)
    NI,
    #[cfg(feature = "ni")]
    /// The Republic of Nicaragua (Americas)
    NU,
    #[cfg(feature = "nl")]
    /// The Kingdom of the Netherlands (Europe)
    NL,
    #[cfg(feature = "no")]
    /// The Kingdom of Norway (Europe)
    NO,
    #[cfg(feature = "np")]
    /// The Federal Democratic Republic of Nepal (Asia)
    NP,
    #[cfg(feature = "nr")]
    /// The Republic of Nauru (Oceania)
    NR,
    #[cfg(feature = "nu")]
    /// Niue (Oceania)
    NE,
    #[cfg(feature = "nz")]
    /// New Zealand (Oceania)
    NZ,
    #[cfg(feature = "om")]
    /// The Sultanate of Oman (Asia)
    MU,
    #[cfg(feature = "pa")]
    /// The Republic of Panamá (Americas)
    PM,
    #[cfg(feature = "pe")]
    /// The Republic of Perú (Americas)
    PE,
    #[cfg(feature = "pf")]
    /// French Polynesia (Oceania)
    FP,
    #[cfg(feature = "pg")]
    /// The Independent State of Papua New Guinea (Oceania)
    PP,
    #[cfg(feature = "ph")]
    /// The Republic of the Philippines (Asia)
    RP,
    #[cfg(feature = "pk")]
    /// The Islamic Republic of Pakistan (Asia)
    PK,
    #[cfg(feature = "pl")]
    /// The Republic of Poland (Europe)
    PL,
    #[cfg(feature = "pm")]
    /// The Overseas Collectivity of Saint-Pierre and Miquelon (Americas)
    SB,
    #[cfg(feature = "pn")]
    /// The Pitcairn, Henderson, Ducie and Oeno Islands (Oceania)
    PC,
    #[cfg(feature = "pr")]
    /// The Commonwealth of Puerto Rico (Americas)
    RQ,
    #[cfg(feature = "ps")]
    /// The State of Palestine (Asia)
    WE,
    #[cfg(feature = "pt")]
    /// The Portuguese Republic (Europe)
    PO,
    #[cfg(feature = "pw")]
    /// The Republic of Palau (Oceania)
    PS,
    #[cfg(feature = "py")]
    /// The Republic of Paraguay (Americas)
    PA,
    #[cfg(feature = "qa")]
    /// The State of Qatar (Asia)
    QA,
    #[cfg(feature = "re")]
    /// Réunion (Africa)
    RE,
    #[cfg(feature = "ro")]
    /// Romania (Europe)
    RO,
    #[cfg(feature = "rs")]
    /// The Republic of Serbia (Europe)
    RI,
    #[cfg(feature = "ru")]
    /// The Russian Federation (Europe)
    RS,
    #[cfg(feature = "rw")]
    /// The Republic of Rwanda (Africa)
    RW,
    #[cfg(feature = "sa")]
    /// The Kingdom of Saudi Arabia (Asia)
    SA,
    #[cfg(feature = "sb")]
    /// The Solomon Islands (Oceania)
    BP,
    #[cfg(feature = "sc")]
    /// The Republic of Seychelles (Africa)
    SE,
    #[cfg(feature = "sd")]
    /// The Republic of the Sudan (Africa)
    SU,
    #[cfg(feature = "se")]
    /// The Kingdom of Sweden (Europe)
    SW,
    #[cfg(feature = "sg")]
    /// The Republic of Singapore (Asia)
    SN,
    #[cfg(feature = "sh")]
    /// Saint Helena, Ascension and Tristan da Cunha (Africa)
    SH,
    #[cfg(feature = "si")]
    /// The Republic of Slovenia (Europe)
    SI,
    #[cfg(feature = "sj")]
    /// Svalbard and Jan Mayen (Europe)
    SV,
    #[cfg(feature = "sk")]
    /// The Slovak Republic (Europe)
    LO,
    #[cfg(feature = "sl")]
    /// The Republic of Sierra Leone (Africa)
    SL,
    #[cfg(feature = "sm")]
    /// The Republic of San Marino (Europe)
    SM,
    #[cfg(feature = "sn")]
    /// The Republic of Senegal (Africa)
    SG,
    #[cfg(feature = "so")]
    /// The Federal Republic of Somalia (Africa)
    SO,
    #[cfg(feature = "sr")]
    /// The Republic of Suriname (Americas)
    NS,
    #[cfg(feature = "ss")]
    /// The Republic of South Sudan (Africa)
    OD,
    #[cfg(feature = "st")]
    /// The Democratic Republic of São Tomé and Príncipe (Africa)
    TP,
    #[cfg(feature = "sv")]
    /// The Republic of El Salvador (Americas)
    ES,
    #[cfg(feature = "sx")]
    /// Sint Maarten (Americas)
    NN,
    #[cfg(feature = "sy")]
    /// The Syrian Arab Republic (Asia)
    SY,
    #[cfg(feature = "sz")]
    /// The Kingdom of Eswatini (Africa)
    WZ,
    #[cfg(feature = "tc")]
    /// The Turks and Caicos Islands (Americas)
    TK,
    #[cfg(feature = "td")]
    /// The Republic of Chad (Africa)
    CD,
    #[cfg(feature = "tf")]
    /// The French Southern and Antarctic Lands (Africa)
    FS,
    #[cfg(feature = "tg")]
    /// The Togolese Republic (Africa)
    TO,
    #[cfg(feature = "th")]
    /// The Kingdom of Thailand (Asia)
    TH,
    #[cfg(feature = "tj")]
    /// The Republic of Tajikistan (Asia)
    TI,
    #[cfg(feature = "tk")]
    /// Tokelau (Oceania)
    TL,
    #[cfg(feature = "tl")]
    /// The Democratic Republic of Timor-Leste (Asia)
    TT,
    #[cfg(feature = "tm")]
    /// Turkmenistan (Asia)
    TX,
    #[cfg(feature = "tn")]
    /// The Republic of Tunisia (Africa)
    TS,
    #[cfg(feature = "to")]
    /// The Kingdom of Tonga (Oceania)
    TN,
    #[cfg(feature = "tr")]
    /// The Republic of Turkey (Asia)
    TU,
    #[cfg(feature = "tt")]
    /// The Republic of Trinidad and Tobago (Americas)
    TD,
    #[cfg(feature = "tv")]
    /// Tuvalu (Oceania)
    TV,
    #[cfg(feature = "tw")]
    /// The Republic of China (Asia)
    TW,
    #[cfg(feature = "tz")]
    /// The United Republic of Tanzania (Africa)
    TZ,
    #[cfg(feature = "ua")]
    /// Ukraine (Europe)
    UP,
    #[cfg(feature = "ug")]
    /// The Republic of Uganda (Africa)
    UG,
    #[cfg(feature = "us")]
    /// The United States of America (Americas)
    US,
    #[cfg(feature = "uy")]
    /// The Oriental Republic of Uruguay (Americas)
    UY,
    #[cfg(feature = "uz")]
    /// The Republic of Uzbekistan (Asia)
    UZ,
    #[cfg(feature = "va")]
    /// The Holy See (Europe)
    VT,
    #[cfg(feature = "vc")]
    /// Saint Vincent and the Grenadines (Americas)
    VC,
    #[cfg(feature = "ve")]
    /// The Bolivarian Republic of Venezuela (Americas)
    VE,
    #[cfg(feature = "vg")]
    /// The Virgin Islands (Americas)
    VI,
    #[cfg(feature = "vi")]
    /// The Virgin Islands of the United States (Americas)
    VQ,
    #[cfg(feature = "vn")]
    /// The Socialist Republic of Viet Nam (Asia)
    VM,
    #[cfg(feature = "vu")]
    /// The Republic of Vanuatu (Oceania)
    NH,
    #[cfg(feature = "wf")]
    /// The Territory of the Wallis and Futuna Islands (Oceania)
    WF,
    #[cfg(feature = "ws")]
    /// The Independent State of Samoa (Oceania)
    WS,
    #[cfg(feature = "ye")]
    /// The Republic of Yemen (Asia)
    YM,
    #[cfg(feature = "yt")]
    /// The Department of Mayotte (Africa)
    MF,
    #[cfg(feature = "za")]
    /// The Republic of South Africa (Africa)
    SF,
    #[cfg(feature = "zm")]
    /// The Republic of Zambia (Africa)
    ZA,
    #[cfg(feature = "zw")]
    /// The Republic of Zimbabwe (Africa)
    ZI,
}
#[cfg(any(
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
mod impls {
    use super::GEC;
    use crate::{Alpha2, Country, SearchError, SearchedItems};

    impl GEC {
        pub fn to_alpha2(&self) -> Alpha2 {
            match self {
                #[cfg(feature = "ad")] // The Principality of Andorra (Europe)
                GEC::AN => Alpha2::AD,
                #[cfg(feature = "ae")] // The United Arab Emirates (Asia)
                GEC::AE => Alpha2::AE,
                #[cfg(feature = "af")] // The Islamic Republic of Afghanistan (Asia)
                GEC::AF => Alpha2::AF,
                #[cfg(feature = "ag")] // Antigua and Barbuda (Americas)
                GEC::AC => Alpha2::AG,
                #[cfg(feature = "ai")] // Anguilla (Americas)
                GEC::AV => Alpha2::AI,
                #[cfg(feature = "al")] // The Republic of Albania (Europe)
                GEC::AL => Alpha2::AL,
                #[cfg(feature = "am")] // The Republic of Armenia (Asia)
                GEC::AM => Alpha2::AM,
                #[cfg(feature = "ao")] // The Republic of Angola (Africa)
                GEC::AO => Alpha2::AO,
                #[cfg(feature = "aq")] // Antarctica
                GEC::AY => Alpha2::AQ,
                #[cfg(feature = "ar")] // The Argentine Republic (Americas)
                GEC::AR => Alpha2::AR,
                #[cfg(feature = "as")] // The Territory of American Samoa (Oceania)
                GEC::AQ => Alpha2::AS,
                #[cfg(feature = "at")] // The Republic of Austria (Europe)
                GEC::AU => Alpha2::AT,
                #[cfg(feature = "au")] // The Commonwealth of Australia (Oceania)
                GEC::AS => Alpha2::AU,
                #[cfg(feature = "aw")] // Aruba (Americas)
                GEC::AA => Alpha2::AW,
                #[cfg(feature = "az")] // The Republic of Azerbaijan (Asia)
                GEC::AJ => Alpha2::AZ,
                #[cfg(feature = "ba")] // Bosnia and Herzegovina (Europe)
                GEC::BK => Alpha2::BA,
                #[cfg(feature = "bb")] // Barbados (Americas)
                GEC::BB => Alpha2::BB,
                #[cfg(feature = "bd")] // The People's Republic of Bangladesh (Asia)
                GEC::BG => Alpha2::BD,
                #[cfg(feature = "be")] // The Kingdom of Belgium (Europe)
                GEC::BE => Alpha2::BE,
                #[cfg(feature = "bf")] // Burkina Faso (Africa)
                GEC::UV => Alpha2::BF,
                #[cfg(feature = "bg")] // The Republic of Bulgaria (Europe)
                GEC::BU => Alpha2::BG,
                #[cfg(feature = "bh")] // The Kingdom of Bahrain (Asia)
                GEC::BA => Alpha2::BH,
                #[cfg(feature = "bi")] // The Republic of Burundi (Africa)
                GEC::BY => Alpha2::BI,
                #[cfg(feature = "bj")] // The Republic of Benin (Africa)
                GEC::BN => Alpha2::BJ,
                #[cfg(feature = "bl")] // The Collectivity of Saint-Barthélemy (Americas)
                GEC::TB => Alpha2::BL,
                #[cfg(feature = "bm")] // Bermuda (Americas)
                GEC::BD => Alpha2::BM,
                #[cfg(feature = "bn")] // The Nation of Brunei, the Abode of Peace (Asia)
                GEC::BX => Alpha2::BN,
                #[cfg(feature = "bo")] // The Plurinational State of Bolivia (Americas)
                GEC::BL => Alpha2::BO,
                #[cfg(feature = "br")] // The Federative Republic of Brazil (Americas)
                GEC::BR => Alpha2::BR,
                #[cfg(feature = "bs")] // The Commonwealth of The Bahamas (Americas)
                GEC::BF => Alpha2::BS,
                #[cfg(feature = "bt")] // The Kingdom of Bhutan (Asia)
                GEC::BT => Alpha2::BT,
                #[cfg(feature = "bv")] // Bouvet Island
                GEC::BV => Alpha2::BV,
                #[cfg(feature = "bw")] // The Republic of Botswana (Africa)
                GEC::BC => Alpha2::BW,
                #[cfg(feature = "by")] // The Republic of Belarus (Europe)
                GEC::BO => Alpha2::BY,
                #[cfg(feature = "bz")] // Belize (Americas)
                GEC::BH => Alpha2::BZ,
                #[cfg(feature = "ca")] // Canada (Americas)
                GEC::CA => Alpha2::CA,
                #[cfg(feature = "cc")] // The Territory of Cocos (Keeling) Islands (Oceania)
                GEC::CK => Alpha2::CC,
                #[cfg(feature = "cd")] // The Democratic Republic of the Congo (Africa)
                GEC::CG => Alpha2::CD,
                #[cfg(feature = "cf")] // The Central African Republic (Africa)
                GEC::CT => Alpha2::CF,
                #[cfg(feature = "cg")] // The Republic of the Congo (Africa)
                GEC::CF => Alpha2::CG,
                #[cfg(feature = "ch")] // The Swiss Confederation (Europe)
                GEC::SZ => Alpha2::CH,
                #[cfg(feature = "ci")] // The Republic of Côte d'Ivoire (Africa)
                GEC::IV => Alpha2::CI,
                #[cfg(feature = "ck")] // The Cook Islands (Oceania)
                GEC::CW => Alpha2::CK,
                #[cfg(feature = "cl")] // The Republic of Chile (Americas)
                GEC::CI => Alpha2::CL,
                #[cfg(feature = "cm")] // The Republic of Cameroon (Africa)
                GEC::CM => Alpha2::CM,
                #[cfg(feature = "cn")] // The People's Republic of China (Asia)
                GEC::CH => Alpha2::CN,
                #[cfg(feature = "co")] // The Republic of Colombia (Americas)
                GEC::CO => Alpha2::CO,
                #[cfg(feature = "cr")] // The Republic of Costa Rica (Americas)
                GEC::CS => Alpha2::CR,
                #[cfg(feature = "cu")] // The Republic of Cuba (Americas)
                GEC::CU => Alpha2::CU,
                #[cfg(feature = "cv")] // The Republic of Cabo Verde (Africa)
                GEC::CV => Alpha2::CV,
                #[cfg(feature = "cw")] // The Country of Curaçao (Americas)
                GEC::UC => Alpha2::CW,
                #[cfg(feature = "cx")] // The Territory of Christmas Island (Oceania)
                GEC::KT => Alpha2::CX,
                #[cfg(feature = "cy")] // The Republic of Cyprus (Asia)
                GEC::CY => Alpha2::CY,
                #[cfg(feature = "cz")] // The Czech Republic (Europe)
                GEC::EZ => Alpha2::CZ,
                #[cfg(feature = "de")] // The Federal Republic of Germany (Europe)
                GEC::GM => Alpha2::DE,
                #[cfg(feature = "dj")] // The Republic of Djibouti (Africa)
                GEC::DJ => Alpha2::DJ,
                #[cfg(feature = "dk")] // The Kingdom of Denmark (Europe)
                GEC::DA => Alpha2::DK,
                #[cfg(feature = "dm")] // The Commonwealth of Dominica (Americas)
                GEC::DO => Alpha2::DM,
                #[cfg(feature = "do")] // The Dominican Republic (Americas)
                GEC::DR => Alpha2::DO,
                #[cfg(feature = "dz")] // The People's Democratic Republic of Algeria (Africa)
                GEC::AG => Alpha2::DZ,
                #[cfg(feature = "ec")] // The Republic of Ecuador (Americas)
                GEC::EC => Alpha2::EC,
                #[cfg(feature = "ee")] // The Republic of Estonia (Europe)
                GEC::EN => Alpha2::EE,
                #[cfg(feature = "eg")] // The Arab Republic of Egypt (Africa)
                GEC::EG => Alpha2::EG,
                #[cfg(feature = "eh")] // The Sahrawi Arab Democratic Republic (Africa)
                GEC::WI => Alpha2::EH,
                #[cfg(feature = "er")] // The State of Eritrea (Africa)
                GEC::ER => Alpha2::ER,
                #[cfg(feature = "es")] // The Kingdom of Spain (Europe)
                GEC::SP => Alpha2::ES,
                #[cfg(feature = "et")] // The Federal Democratic Republic of Ethiopia (Africa)
                GEC::ET => Alpha2::ET,
                #[cfg(feature = "fi")] // The Republic of Finland (Europe)
                GEC::FI => Alpha2::FI,
                #[cfg(feature = "fj")] // The Republic of Fiji (Oceania)
                GEC::FJ => Alpha2::FJ,
                #[cfg(feature = "fk")] // The Falkland Islands (Americas)
                GEC::FK => Alpha2::FK,
                #[cfg(feature = "fm")] // The Federated States of Micronesia (Oceania)
                GEC::FM => Alpha2::FM,
                #[cfg(feature = "fo")] // The Faroe Islands (Europe)
                GEC::FO => Alpha2::FO,
                #[cfg(feature = "fr")] // The French Republic (Europe)
                GEC::FR => Alpha2::FR,
                #[cfg(feature = "ga")] // The Gabonese Republic (Africa)
                GEC::GB => Alpha2::GA,
                #[cfg(feature = "gb")]
                // The United Kingdom of Great Britain and Northern Ireland (Europe)
                GEC::UK => Alpha2::GB,
                #[cfg(feature = "gd")] // Grenada (Americas)
                GEC::GJ => Alpha2::GD,
                #[cfg(feature = "ge")] // Georgia (Asia)
                GEC::GG => Alpha2::GE,
                #[cfg(feature = "gf")] // Guyane (Americas)
                GEC::FG => Alpha2::GF,
                #[cfg(feature = "gg")] // The Bailiwick of Guernsey (Europe)
                GEC::GK => Alpha2::GG,
                #[cfg(feature = "gh")] // The Republic of Ghana (Africa)
                GEC::GH => Alpha2::GH,
                #[cfg(feature = "gi")] // Gibraltar (Europe)
                GEC::GI => Alpha2::GI,
                #[cfg(feature = "gl")] // Kalaallit Nunaat (Americas)
                GEC::GL => Alpha2::GL,
                #[cfg(feature = "gm")] // The Republic of The Gambia (Africa)
                GEC::GA => Alpha2::GM,
                #[cfg(feature = "gn")] // The Republic of Guinea (Africa)
                GEC::GV => Alpha2::GN,
                #[cfg(feature = "gp")] // Guadeloupe (Americas)
                GEC::GP => Alpha2::GP,
                #[cfg(feature = "gq")] // The Republic of Equatorial Guinea (Africa)
                GEC::EK => Alpha2::GQ,
                #[cfg(feature = "gr")] // The Hellenic Republic (Europe)
                GEC::GR => Alpha2::GR,
                #[cfg(feature = "gs")] // South Georgia and the South Sandwich Islands (Americas)
                GEC::SX => Alpha2::GS,
                #[cfg(feature = "gt")] // The Republic of Guatemala (Americas)
                GEC::GT => Alpha2::GT,
                #[cfg(feature = "gu")] // The Territory of Guam (Oceania)
                GEC::GQ => Alpha2::GU,
                #[cfg(feature = "gw")] // The Republic of Guinea-Bissau (Africa)
                GEC::PU => Alpha2::GW,
                #[cfg(feature = "gy")] // The Co-operative Republic of Guyana (Americas)
                GEC::GY => Alpha2::GY,
                #[cfg(feature = "hk")]
                // The Hong Kong Special Administrative Region of China (Asia)
                GEC::HK => Alpha2::HK,
                #[cfg(feature = "hm")] // The Territory of Heard Island and McDonald Islands
                GEC::HM => Alpha2::HM,
                #[cfg(feature = "hn")] // The Republic of Honduras (Americas)
                GEC::HO => Alpha2::HN,
                #[cfg(feature = "hr")] // The Republic of Croatia (Europe)
                GEC::HR => Alpha2::HR,
                #[cfg(feature = "ht")] // The Republic of Haiti (Americas)
                GEC::HA => Alpha2::HT,
                #[cfg(feature = "hu")] // Hungary (Europe)
                GEC::HU => Alpha2::HU,
                #[cfg(feature = "id")] // The Republic of Indonesia (Asia)
                GEC::ID => Alpha2::ID,
                #[cfg(feature = "ie")] // Ireland (Europe)
                GEC::EI => Alpha2::IE,
                #[cfg(feature = "il")] // The State of Israel (Asia)
                GEC::IS => Alpha2::IL,
                #[cfg(feature = "im")] // The Isle of Man (Europe)
                GEC::IM => Alpha2::IM,
                #[cfg(feature = "in")] // The Republic of India (Asia)
                GEC::IN => Alpha2::IN,
                #[cfg(feature = "io")] // The British Indian Ocean Territory (Africa)
                GEC::IO => Alpha2::IO,
                #[cfg(feature = "iq")] // The Republic of Iraq (Asia)
                GEC::IZ => Alpha2::IQ,
                #[cfg(feature = "ir")] // The Islamic Republic of Iran (Asia)
                GEC::IR => Alpha2::IR,
                #[cfg(feature = "is")] // Iceland (Europe)
                GEC::IC => Alpha2::IS,
                #[cfg(feature = "it")] // The Italian Republic (Europe)
                GEC::IT => Alpha2::IT,
                #[cfg(feature = "je")] // The Bailiwick of Jersey (Europe)
                GEC::JE => Alpha2::JE,
                #[cfg(feature = "jm")] // Jamaica (Americas)
                GEC::JM => Alpha2::JM,
                #[cfg(feature = "jo")] // The Hashemite Kingdom of Jordan (Asia)
                GEC::JO => Alpha2::JO,
                #[cfg(feature = "jp")] // Japan (Asia)
                GEC::JA => Alpha2::JP,
                #[cfg(feature = "ke")] // The Republic of Kenya (Africa)
                GEC::KE => Alpha2::KE,
                #[cfg(feature = "kg")] // The Kyrgyz Republic (Asia)
                GEC::KG => Alpha2::KG,
                #[cfg(feature = "kh")] // The Kingdom of Cambodia (Asia)
                GEC::CB => Alpha2::KH,
                #[cfg(feature = "ki")] // The Republic of Kiribati (Oceania)
                GEC::KR => Alpha2::KI,
                #[cfg(feature = "km")] // The Union of the Comoros (Africa)
                GEC::CN => Alpha2::KM,
                #[cfg(feature = "kn")] // Saint Kitts and Nevis (Americas)
                GEC::SC => Alpha2::KN,
                #[cfg(feature = "kp")] // The Democratic People's Republic of Korea (Asia)
                GEC::KN => Alpha2::KP,
                #[cfg(feature = "kr")] // The Republic of Korea (Asia)
                GEC::KS => Alpha2::KR,
                #[cfg(feature = "kw")] // The State of Kuwait (Asia)
                GEC::KU => Alpha2::KW,
                #[cfg(feature = "ky")] // The Cayman Islands (Americas)
                GEC::CJ => Alpha2::KY,
                #[cfg(feature = "kz")] // The Republic of Kazakhstan (Asia)
                GEC::KZ => Alpha2::KZ,
                #[cfg(feature = "la")] // The Lao People's Democratic Republic (Asia)
                GEC::LA => Alpha2::LA,
                #[cfg(feature = "lb")] // The Lebanese Republic (Asia)
                GEC::LE => Alpha2::LB,
                #[cfg(feature = "lc")] // Saint Lucia (Americas)
                GEC::ST => Alpha2::LC,
                #[cfg(feature = "li")] // The Principality of Liechtenstein (Europe)
                GEC::LS => Alpha2::LI,
                #[cfg(feature = "lk")] // The Democratic Socialist Republic of Sri Lanka (Asia)
                GEC::CE => Alpha2::LK,
                #[cfg(feature = "lr")] // The Republic of Liberia (Africa)
                GEC::LI => Alpha2::LR,
                #[cfg(feature = "ls")] // The Kingdom of Lesotho (Africa)
                GEC::LT => Alpha2::LS,
                #[cfg(feature = "lt")] // The Republic of Lithuania (Europe)
                GEC::LH => Alpha2::LT,
                #[cfg(feature = "lu")] // The Grand Duchy of Luxembourg (Europe)
                GEC::LU => Alpha2::LU,
                #[cfg(feature = "lv")] // The Republic of Latvia (Europe)
                GEC::LG => Alpha2::LV,
                #[cfg(feature = "ly")] // The State of Libya (Africa)
                GEC::LY => Alpha2::LY,
                #[cfg(feature = "ma")] // The Kingdom of Morocco (Africa)
                GEC::MO => Alpha2::MA,
                #[cfg(feature = "mc")] // The Principality of Monaco (Europe)
                GEC::MN => Alpha2::MC,
                #[cfg(feature = "md")] // The Republic of Moldova (Europe)
                GEC::MD => Alpha2::MD,
                #[cfg(feature = "me")] // Montenegro (Europe)
                GEC::MJ => Alpha2::ME,
                #[cfg(feature = "mf")] // The Collectivity of Saint-Martin (Americas)
                GEC::RN => Alpha2::MF,
                #[cfg(feature = "mg")] // The Republic of Madagascar (Africa)
                GEC::MA => Alpha2::MG,
                #[cfg(feature = "mh")] // The Republic of the Marshall Islands (Oceania)
                GEC::RM => Alpha2::MH,
                #[cfg(feature = "mk")] // The Republic of North Macedonia (Europe)
                GEC::MK => Alpha2::MK,
                #[cfg(feature = "ml")] // The Republic of Mali (Africa)
                GEC::ML => Alpha2::ML,
                #[cfg(feature = "mm")] // The Republic of the Union of Myanmar (Asia)
                GEC::BM => Alpha2::MM,
                #[cfg(feature = "mn")] // Mongolia (Asia)
                GEC::MG => Alpha2::MN,
                #[cfg(feature = "mo")] // The Macao Special Administrative Region of China (Asia)
                GEC::MC => Alpha2::MO,
                #[cfg(feature = "mp")] // The Commonwealth of the Northern Mariana Islands (Oceania)
                GEC::CQ => Alpha2::MP,
                #[cfg(feature = "mq")] // Martinique (Americas)
                GEC::MB => Alpha2::MQ,
                #[cfg(feature = "mr")] // The Islamic Republic of Mauritania (Africa)
                GEC::MR => Alpha2::MR,
                #[cfg(feature = "ms")] // Montserrat (Americas)
                GEC::MH => Alpha2::MS,
                #[cfg(feature = "mt")] // The Republic of Malta (Europe)
                GEC::MT => Alpha2::MT,
                #[cfg(feature = "mu")] // The Republic of Mauritius (Africa)
                GEC::MP => Alpha2::MU,
                #[cfg(feature = "mv")] // The Republic of Maldives (Asia)
                GEC::MV => Alpha2::MV,
                #[cfg(feature = "mw")] // The Republic of Malawi (Africa)
                GEC::MI => Alpha2::MW,
                #[cfg(feature = "mx")] // The United Mexican States (Americas)
                GEC::MX => Alpha2::MX,
                #[cfg(feature = "my")] // Malaysia (Asia)
                GEC::MY => Alpha2::MY,
                #[cfg(feature = "mz")] // The Republic of Mozambique (Africa)
                GEC::MZ => Alpha2::MZ,
                #[cfg(feature = "na")] // The Republic of Namibia (Africa)
                GEC::WA => Alpha2::NA,
                #[cfg(feature = "nc")] // New Caledonia (Oceania)
                GEC::NC => Alpha2::NC,
                #[cfg(feature = "ne")] // The Republic of the Niger (Africa)
                GEC::NG => Alpha2::NE,
                #[cfg(feature = "nf")] // The Territory of Norfolk Island (Oceania)
                GEC::NF => Alpha2::NF,
                #[cfg(feature = "ng")] // The Federal Republic of Nigeria (Africa)
                GEC::NI => Alpha2::NG,
                #[cfg(feature = "ni")] // The Republic of Nicaragua (Americas)
                GEC::NU => Alpha2::NI,
                #[cfg(feature = "nl")] // The Kingdom of the Netherlands (Europe)
                GEC::NL => Alpha2::NL,
                #[cfg(feature = "no")] // The Kingdom of Norway (Europe)
                GEC::NO => Alpha2::NO,
                #[cfg(feature = "np")] // The Federal Democratic Republic of Nepal (Asia)
                GEC::NP => Alpha2::NP,
                #[cfg(feature = "nr")] // The Republic of Nauru (Oceania)
                GEC::NR => Alpha2::NR,
                #[cfg(feature = "nu")] // Niue (Oceania)
                GEC::NE => Alpha2::NU,
                #[cfg(feature = "nz")] // New Zealand (Oceania)
                GEC::NZ => Alpha2::NZ,
                #[cfg(feature = "om")] // The Sultanate of Oman (Asia)
                GEC::MU => Alpha2::OM,
                #[cfg(feature = "pa")] // The Republic of Panamá (Americas)
                GEC::PM => Alpha2::PA,
                #[cfg(feature = "pe")] // The Republic of Perú (Americas)
                GEC::PE => Alpha2::PE,
                #[cfg(feature = "pf")] // French Polynesia (Oceania)
                GEC::FP => Alpha2::PF,
                #[cfg(feature = "pg")] // The Independent State of Papua New Guinea (Oceania)
                GEC::PP => Alpha2::PG,
                #[cfg(feature = "ph")] // The Republic of the Philippines (Asia)
                GEC::RP => Alpha2::PH,
                #[cfg(feature = "pk")] // The Islamic Republic of Pakistan (Asia)
                GEC::PK => Alpha2::PK,
                #[cfg(feature = "pl")] // The Republic of Poland (Europe)
                GEC::PL => Alpha2::PL,
                #[cfg(feature = "pm")]
                // The Overseas Collectivity of Saint-Pierre and Miquelon (Americas)
                GEC::SB => Alpha2::PM,
                #[cfg(feature = "pn")] // The Pitcairn, Henderson, Ducie and Oeno Islands (Oceania)
                GEC::PC => Alpha2::PN,
                #[cfg(feature = "pr")] // The Commonwealth of Puerto Rico (Americas)
                GEC::RQ => Alpha2::PR,
                #[cfg(feature = "ps")] // The State of Palestine (Asia)
                GEC::WE => Alpha2::PS,
                #[cfg(feature = "pt")] // The Portuguese Republic (Europe)
                GEC::PO => Alpha2::PT,
                #[cfg(feature = "pw")] // The Republic of Palau (Oceania)
                GEC::PS => Alpha2::PW,
                #[cfg(feature = "py")] // The Republic of Paraguay (Americas)
                GEC::PA => Alpha2::PY,
                #[cfg(feature = "qa")] // The State of Qatar (Asia)
                GEC::QA => Alpha2::QA,
                #[cfg(feature = "re")] // Réunion (Africa)
                GEC::RE => Alpha2::RE,
                #[cfg(feature = "ro")] // Romania (Europe)
                GEC::RO => Alpha2::RO,
                #[cfg(feature = "rs")] // The Republic of Serbia (Europe)
                GEC::RI => Alpha2::RS,
                #[cfg(feature = "ru")] // The Russian Federation (Europe)
                GEC::RS => Alpha2::RU,
                #[cfg(feature = "rw")] // The Republic of Rwanda (Africa)
                GEC::RW => Alpha2::RW,
                #[cfg(feature = "sa")] // The Kingdom of Saudi Arabia (Asia)
                GEC::SA => Alpha2::SA,
                #[cfg(feature = "sb")] // The Solomon Islands (Oceania)
                GEC::BP => Alpha2::SB,
                #[cfg(feature = "sc")] // The Republic of Seychelles (Africa)
                GEC::SE => Alpha2::SC,
                #[cfg(feature = "sd")] // The Republic of the Sudan (Africa)
                GEC::SU => Alpha2::SD,
                #[cfg(feature = "se")] // The Kingdom of Sweden (Europe)
                GEC::SW => Alpha2::SE,
                #[cfg(feature = "sg")] // The Republic of Singapore (Asia)
                GEC::SN => Alpha2::SG,
                #[cfg(feature = "sh")] // Saint Helena, Ascension and Tristan da Cunha (Africa)
                GEC::SH => Alpha2::SH,
                #[cfg(feature = "si")] // The Republic of Slovenia (Europe)
                GEC::SI => Alpha2::SI,
                #[cfg(feature = "sj")] // Svalbard and Jan Mayen (Europe)
                GEC::SV => Alpha2::SJ,
                #[cfg(feature = "sk")] // The Slovak Republic (Europe)
                GEC::LO => Alpha2::SK,
                #[cfg(feature = "sl")] // The Republic of Sierra Leone (Africa)
                GEC::SL => Alpha2::SL,
                #[cfg(feature = "sm")] // The Republic of San Marino (Europe)
                GEC::SM => Alpha2::SM,
                #[cfg(feature = "sn")] // The Republic of Senegal (Africa)
                GEC::SG => Alpha2::SN,
                #[cfg(feature = "so")] // The Federal Republic of Somalia (Africa)
                GEC::SO => Alpha2::SO,
                #[cfg(feature = "sr")] // The Republic of Suriname (Americas)
                GEC::NS => Alpha2::SR,
                #[cfg(feature = "ss")] // The Republic of South Sudan (Africa)
                GEC::OD => Alpha2::SS,
                #[cfg(feature = "st")] // The Democratic Republic of São Tomé and Príncipe (Africa)
                GEC::TP => Alpha2::ST,
                #[cfg(feature = "sv")] // The Republic of El Salvador (Americas)
                GEC::ES => Alpha2::SV,
                #[cfg(feature = "sx")] // Sint Maarten (Americas)
                GEC::NN => Alpha2::SX,
                #[cfg(feature = "sy")] // The Syrian Arab Republic (Asia)
                GEC::SY => Alpha2::SY,
                #[cfg(feature = "sz")] // The Kingdom of Eswatini (Africa)
                GEC::WZ => Alpha2::SZ,
                #[cfg(feature = "tc")] // The Turks and Caicos Islands (Americas)
                GEC::TK => Alpha2::TC,
                #[cfg(feature = "td")] // The Republic of Chad (Africa)
                GEC::CD => Alpha2::TD,
                #[cfg(feature = "tf")] // The French Southern and Antarctic Lands (Africa)
                GEC::FS => Alpha2::TF,
                #[cfg(feature = "tg")] // The Togolese Republic (Africa)
                GEC::TO => Alpha2::TG,
                #[cfg(feature = "th")] // The Kingdom of Thailand (Asia)
                GEC::TH => Alpha2::TH,
                #[cfg(feature = "tj")] // The Republic of Tajikistan (Asia)
                GEC::TI => Alpha2::TJ,
                #[cfg(feature = "tk")] // Tokelau (Oceania)
                GEC::TL => Alpha2::TK,
                #[cfg(feature = "tl")] // The Democratic Republic of Timor-Leste (Asia)
                GEC::TT => Alpha2::TL,
                #[cfg(feature = "tm")] // Turkmenistan (Asia)
                GEC::TX => Alpha2::TM,
                #[cfg(feature = "tn")] // The Republic of Tunisia (Africa)
                GEC::TS => Alpha2::TN,
                #[cfg(feature = "to")] // The Kingdom of Tonga (Oceania)
                GEC::TN => Alpha2::TO,
                #[cfg(feature = "tr")] // The Republic of Turkey (Asia)
                GEC::TU => Alpha2::TR,
                #[cfg(feature = "tt")] // The Republic of Trinidad and Tobago (Americas)
                GEC::TD => Alpha2::TT,
                #[cfg(feature = "tv")] // Tuvalu (Oceania)
                GEC::TV => Alpha2::TV,
                #[cfg(feature = "tw")] // The Republic of China (Asia)
                GEC::TW => Alpha2::TW,
                #[cfg(feature = "tz")] // The United Republic of Tanzania (Africa)
                GEC::TZ => Alpha2::TZ,
                #[cfg(feature = "ua")] // Ukraine (Europe)
                GEC::UP => Alpha2::UA,
                #[cfg(feature = "ug")] // The Republic of Uganda (Africa)
                GEC::UG => Alpha2::UG,
                #[cfg(feature = "us")] // The United States of America (Americas)
                GEC::US => Alpha2::US,
                #[cfg(feature = "uy")] // The Oriental Republic of Uruguay (Americas)
                GEC::UY => Alpha2::UY,
                #[cfg(feature = "uz")] // The Republic of Uzbekistan (Asia)
                GEC::UZ => Alpha2::UZ,
                #[cfg(feature = "va")] // The Holy See (Europe)
                GEC::VT => Alpha2::VA,
                #[cfg(feature = "vc")] // Saint Vincent and the Grenadines (Americas)
                GEC::VC => Alpha2::VC,
                #[cfg(feature = "ve")] // The Bolivarian Republic of Venezuela (Americas)
                GEC::VE => Alpha2::VE,
                #[cfg(feature = "vg")] // The Virgin Islands (Americas)
                GEC::VI => Alpha2::VG,
                #[cfg(feature = "vi")] // The Virgin Islands of the United States (Americas)
                GEC::VQ => Alpha2::VI,
                #[cfg(feature = "vn")] // The Socialist Republic of Viet Nam (Asia)
                GEC::VM => Alpha2::VN,
                #[cfg(feature = "vu")] // The Republic of Vanuatu (Oceania)
                GEC::NH => Alpha2::VU,
                #[cfg(feature = "wf")] // The Territory of the Wallis and Futuna Islands (Oceania)
                GEC::WF => Alpha2::WF,
                #[cfg(feature = "ws")] // The Independent State of Samoa (Oceania)
                GEC::WS => Alpha2::WS,
                #[cfg(feature = "ye")] // The Republic of Yemen (Asia)
                GEC::YM => Alpha2::YE,
                #[cfg(feature = "yt")] // The Department of Mayotte (Africa)
                GEC::MF => Alpha2::YT,
                #[cfg(feature = "za")] // The Republic of South Africa (Africa)
                GEC::SF => Alpha2::ZA,
                #[cfg(feature = "zm")] // The Republic of Zambia (Africa)
                GEC::ZA => Alpha2::ZM,
                #[cfg(feature = "zw")] // The Republic of Zimbabwe (Africa)
                GEC::ZI => Alpha2::ZW,
            }
        }

        pub fn to_country(&self) -> Country {
            self.to_alpha2().to_country()
        }
    }

    impl ToString for GEC {
        fn to_string(&self) -> String {
            match self {
                #[cfg(feature = "ad")] // The Principality of Andorra (Europe)
                GEC::AN => "AN",
                #[cfg(feature = "ae")] // The United Arab Emirates (Asia)
                GEC::AE => "AE",
                #[cfg(feature = "af")] // The Islamic Republic of Afghanistan (Asia)
                GEC::AF => "AF",
                #[cfg(feature = "ag")] // Antigua and Barbuda (Americas)
                GEC::AC => "AC",
                #[cfg(feature = "ai")] // Anguilla (Americas)
                GEC::AV => "AV",
                #[cfg(feature = "al")] // The Republic of Albania (Europe)
                GEC::AL => "AL",
                #[cfg(feature = "am")] // The Republic of Armenia (Asia)
                GEC::AM => "AM",
                #[cfg(feature = "ao")] // The Republic of Angola (Africa)
                GEC::AO => "AO",
                #[cfg(feature = "aq")] // Antarctica
                GEC::AY => "AY",
                #[cfg(feature = "ar")] // The Argentine Republic (Americas)
                GEC::AR => "AR",
                #[cfg(feature = "as")] // The Territory of American Samoa (Oceania)
                GEC::AQ => "AQ",
                #[cfg(feature = "at")] // The Republic of Austria (Europe)
                GEC::AU => "AU",
                #[cfg(feature = "au")] // The Commonwealth of Australia (Oceania)
                GEC::AS => "AS",
                #[cfg(feature = "aw")] // Aruba (Americas)
                GEC::AA => "AA",
                #[cfg(feature = "az")] // The Republic of Azerbaijan (Asia)
                GEC::AJ => "AJ",
                #[cfg(feature = "ba")] // Bosnia and Herzegovina (Europe)
                GEC::BK => "BK",
                #[cfg(feature = "bb")] // Barbados (Americas)
                GEC::BB => "BB",
                #[cfg(feature = "bd")] // The People's Republic of Bangladesh (Asia)
                GEC::BG => "BG",
                #[cfg(feature = "be")] // The Kingdom of Belgium (Europe)
                GEC::BE => "BE",
                #[cfg(feature = "bf")] // Burkina Faso (Africa)
                GEC::UV => "UV",
                #[cfg(feature = "bg")] // The Republic of Bulgaria (Europe)
                GEC::BU => "BU",
                #[cfg(feature = "bh")] // The Kingdom of Bahrain (Asia)
                GEC::BA => "BA",
                #[cfg(feature = "bi")] // The Republic of Burundi (Africa)
                GEC::BY => "BY",
                #[cfg(feature = "bj")] // The Republic of Benin (Africa)
                GEC::BN => "BN",
                #[cfg(feature = "bl")] // The Collectivity of Saint-Barthélemy (Americas)
                GEC::TB => "TB",
                #[cfg(feature = "bm")] // Bermuda (Americas)
                GEC::BD => "BD",
                #[cfg(feature = "bn")] // The Nation of Brunei, the Abode of Peace (Asia)
                GEC::BX => "BX",
                #[cfg(feature = "bo")] // The Plurinational State of Bolivia (Americas)
                GEC::BL => "BL",
                #[cfg(feature = "br")] // The Federative Republic of Brazil (Americas)
                GEC::BR => "BR",
                #[cfg(feature = "bs")] // The Commonwealth of The Bahamas (Americas)
                GEC::BF => "BF",
                #[cfg(feature = "bt")] // The Kingdom of Bhutan (Asia)
                GEC::BT => "BT",
                #[cfg(feature = "bv")] // Bouvet Island
                GEC::BV => "BV",
                #[cfg(feature = "bw")] // The Republic of Botswana (Africa)
                GEC::BC => "BC",
                #[cfg(feature = "by")] // The Republic of Belarus (Europe)
                GEC::BO => "BO",
                #[cfg(feature = "bz")] // Belize (Americas)
                GEC::BH => "BH",
                #[cfg(feature = "ca")] // Canada (Americas)
                GEC::CA => "CA",
                #[cfg(feature = "cc")] // The Territory of Cocos (Keeling) Islands (Oceania)
                GEC::CK => "CK",
                #[cfg(feature = "cd")] // The Democratic Republic of the Congo (Africa)
                GEC::CG => "CG",
                #[cfg(feature = "cf")] // The Central African Republic (Africa)
                GEC::CT => "CT",
                #[cfg(feature = "cg")] // The Republic of the Congo (Africa)
                GEC::CF => "CF",
                #[cfg(feature = "ch")] // The Swiss Confederation (Europe)
                GEC::SZ => "SZ",
                #[cfg(feature = "ci")] // The Republic of Côte d'Ivoire (Africa)
                GEC::IV => "IV",
                #[cfg(feature = "ck")] // The Cook Islands (Oceania)
                GEC::CW => "CW",
                #[cfg(feature = "cl")] // The Republic of Chile (Americas)
                GEC::CI => "CI",
                #[cfg(feature = "cm")] // The Republic of Cameroon (Africa)
                GEC::CM => "CM",
                #[cfg(feature = "cn")] // The People's Republic of China (Asia)
                GEC::CH => "CH",
                #[cfg(feature = "co")] // The Republic of Colombia (Americas)
                GEC::CO => "CO",
                #[cfg(feature = "cr")] // The Republic of Costa Rica (Americas)
                GEC::CS => "CS",
                #[cfg(feature = "cu")] // The Republic of Cuba (Americas)
                GEC::CU => "CU",
                #[cfg(feature = "cv")] // The Republic of Cabo Verde (Africa)
                GEC::CV => "CV",
                #[cfg(feature = "cw")] // The Country of Curaçao (Americas)
                GEC::UC => "UC",
                #[cfg(feature = "cx")] // The Territory of Christmas Island (Oceania)
                GEC::KT => "KT",
                #[cfg(feature = "cy")] // The Republic of Cyprus (Asia)
                GEC::CY => "CY",
                #[cfg(feature = "cz")] // The Czech Republic (Europe)
                GEC::EZ => "EZ",
                #[cfg(feature = "de")] // The Federal Republic of Germany (Europe)
                GEC::GM => "GM",
                #[cfg(feature = "dj")] // The Republic of Djibouti (Africa)
                GEC::DJ => "DJ",
                #[cfg(feature = "dk")] // The Kingdom of Denmark (Europe)
                GEC::DA => "DA",
                #[cfg(feature = "dm")] // The Commonwealth of Dominica (Americas)
                GEC::DO => "DO",
                #[cfg(feature = "do")] // The Dominican Republic (Americas)
                GEC::DR => "DR",
                #[cfg(feature = "dz")] // The People's Democratic Republic of Algeria (Africa)
                GEC::AG => "AG",
                #[cfg(feature = "ec")] // The Republic of Ecuador (Americas)
                GEC::EC => "EC",
                #[cfg(feature = "ee")] // The Republic of Estonia (Europe)
                GEC::EN => "EN",
                #[cfg(feature = "eg")] // The Arab Republic of Egypt (Africa)
                GEC::EG => "EG",
                #[cfg(feature = "eh")] // The Sahrawi Arab Democratic Republic (Africa)
                GEC::WI => "WI",
                #[cfg(feature = "er")] // The State of Eritrea (Africa)
                GEC::ER => "ER",
                #[cfg(feature = "es")] // The Kingdom of Spain (Europe)
                GEC::SP => "SP",
                #[cfg(feature = "et")] // The Federal Democratic Republic of Ethiopia (Africa)
                GEC::ET => "ET",
                #[cfg(feature = "fi")] // The Republic of Finland (Europe)
                GEC::FI => "FI",
                #[cfg(feature = "fj")] // The Republic of Fiji (Oceania)
                GEC::FJ => "FJ",
                #[cfg(feature = "fk")] // The Falkland Islands (Americas)
                GEC::FK => "FK",
                #[cfg(feature = "fm")] // The Federated States of Micronesia (Oceania)
                GEC::FM => "FM",
                #[cfg(feature = "fo")] // The Faroe Islands (Europe)
                GEC::FO => "FO",
                #[cfg(feature = "fr")] // The French Republic (Europe)
                GEC::FR => "FR",
                #[cfg(feature = "ga")] // The Gabonese Republic (Africa)
                GEC::GB => "GB",
                #[cfg(feature = "gb")]
                // The United Kingdom of Great Britain and Northern Ireland (Europe)
                GEC::UK => "UK",
                #[cfg(feature = "gd")] // Grenada (Americas)
                GEC::GJ => "GJ",
                #[cfg(feature = "ge")] // Georgia (Asia)
                GEC::GG => "GG",
                #[cfg(feature = "gf")] // Guyane (Americas)
                GEC::FG => "FG",
                #[cfg(feature = "gg")] // The Bailiwick of Guernsey (Europe)
                GEC::GK => "GK",
                #[cfg(feature = "gh")] // The Republic of Ghana (Africa)
                GEC::GH => "GH",
                #[cfg(feature = "gi")] // Gibraltar (Europe)
                GEC::GI => "GI",
                #[cfg(feature = "gl")] // Kalaallit Nunaat (Americas)
                GEC::GL => "GL",
                #[cfg(feature = "gm")] // The Republic of The Gambia (Africa)
                GEC::GA => "GA",
                #[cfg(feature = "gn")] // The Republic of Guinea (Africa)
                GEC::GV => "GV",
                #[cfg(feature = "gp")] // Guadeloupe (Americas)
                GEC::GP => "GP",
                #[cfg(feature = "gq")] // The Republic of Equatorial Guinea (Africa)
                GEC::EK => "EK",
                #[cfg(feature = "gr")] // The Hellenic Republic (Europe)
                GEC::GR => "GR",
                #[cfg(feature = "gs")] // South Georgia and the South Sandwich Islands (Americas)
                GEC::SX => "SX",
                #[cfg(feature = "gt")] // The Republic of Guatemala (Americas)
                GEC::GT => "GT",
                #[cfg(feature = "gu")] // The Territory of Guam (Oceania)
                GEC::GQ => "GQ",
                #[cfg(feature = "gw")] // The Republic of Guinea-Bissau (Africa)
                GEC::PU => "PU",
                #[cfg(feature = "gy")] // The Co-operative Republic of Guyana (Americas)
                GEC::GY => "GY",
                #[cfg(feature = "hk")]
                // The Hong Kong Special Administrative Region of China (Asia)
                GEC::HK => "HK",
                #[cfg(feature = "hm")] // The Territory of Heard Island and McDonald Islands
                GEC::HM => "HM",
                #[cfg(feature = "hn")] // The Republic of Honduras (Americas)
                GEC::HO => "HO",
                #[cfg(feature = "hr")] // The Republic of Croatia (Europe)
                GEC::HR => "HR",
                #[cfg(feature = "ht")] // The Republic of Haiti (Americas)
                GEC::HA => "HA",
                #[cfg(feature = "hu")] // Hungary (Europe)
                GEC::HU => "HU",
                #[cfg(feature = "id")] // The Republic of Indonesia (Asia)
                GEC::ID => "ID",
                #[cfg(feature = "ie")] // Ireland (Europe)
                GEC::EI => "EI",
                #[cfg(feature = "il")] // The State of Israel (Asia)
                GEC::IS => "IS",
                #[cfg(feature = "im")] // The Isle of Man (Europe)
                GEC::IM => "IM",
                #[cfg(feature = "in")] // The Republic of India (Asia)
                GEC::IN => "IN",
                #[cfg(feature = "io")] // The British Indian Ocean Territory (Africa)
                GEC::IO => "IO",
                #[cfg(feature = "iq")] // The Republic of Iraq (Asia)
                GEC::IZ => "IZ",
                #[cfg(feature = "ir")] // The Islamic Republic of Iran (Asia)
                GEC::IR => "IR",
                #[cfg(feature = "is")] // Iceland (Europe)
                GEC::IC => "IC",
                #[cfg(feature = "it")] // The Italian Republic (Europe)
                GEC::IT => "IT",
                #[cfg(feature = "je")] // The Bailiwick of Jersey (Europe)
                GEC::JE => "JE",
                #[cfg(feature = "jm")] // Jamaica (Americas)
                GEC::JM => "JM",
                #[cfg(feature = "jo")] // The Hashemite Kingdom of Jordan (Asia)
                GEC::JO => "JO",
                #[cfg(feature = "jp")] // Japan (Asia)
                GEC::JA => "JA",
                #[cfg(feature = "ke")] // The Republic of Kenya (Africa)
                GEC::KE => "KE",
                #[cfg(feature = "kg")] // The Kyrgyz Republic (Asia)
                GEC::KG => "KG",
                #[cfg(feature = "kh")] // The Kingdom of Cambodia (Asia)
                GEC::CB => "CB",
                #[cfg(feature = "ki")] // The Republic of Kiribati (Oceania)
                GEC::KR => "KR",
                #[cfg(feature = "km")] // The Union of the Comoros (Africa)
                GEC::CN => "CN",
                #[cfg(feature = "kn")] // Saint Kitts and Nevis (Americas)
                GEC::SC => "SC",
                #[cfg(feature = "kp")] // The Democratic People's Republic of Korea (Asia)
                GEC::KN => "KN",
                #[cfg(feature = "kr")] // The Republic of Korea (Asia)
                GEC::KS => "KS",
                #[cfg(feature = "kw")] // The State of Kuwait (Asia)
                GEC::KU => "KU",
                #[cfg(feature = "ky")] // The Cayman Islands (Americas)
                GEC::CJ => "CJ",
                #[cfg(feature = "kz")] // The Republic of Kazakhstan (Asia)
                GEC::KZ => "KZ",
                #[cfg(feature = "la")] // The Lao People's Democratic Republic (Asia)
                GEC::LA => "LA",
                #[cfg(feature = "lb")] // The Lebanese Republic (Asia)
                GEC::LE => "LE",
                #[cfg(feature = "lc")] // Saint Lucia (Americas)
                GEC::ST => "ST",
                #[cfg(feature = "li")] // The Principality of Liechtenstein (Europe)
                GEC::LS => "LS",
                #[cfg(feature = "lk")] // The Democratic Socialist Republic of Sri Lanka (Asia)
                GEC::CE => "CE",
                #[cfg(feature = "lr")] // The Republic of Liberia (Africa)
                GEC::LI => "LI",
                #[cfg(feature = "ls")] // The Kingdom of Lesotho (Africa)
                GEC::LT => "LT",
                #[cfg(feature = "lt")] // The Republic of Lithuania (Europe)
                GEC::LH => "LH",
                #[cfg(feature = "lu")] // The Grand Duchy of Luxembourg (Europe)
                GEC::LU => "LU",
                #[cfg(feature = "lv")] // The Republic of Latvia (Europe)
                GEC::LG => "LG",
                #[cfg(feature = "ly")] // The State of Libya (Africa)
                GEC::LY => "LY",
                #[cfg(feature = "ma")] // The Kingdom of Morocco (Africa)
                GEC::MO => "MO",
                #[cfg(feature = "mc")] // The Principality of Monaco (Europe)
                GEC::MN => "MN",
                #[cfg(feature = "md")] // The Republic of Moldova (Europe)
                GEC::MD => "MD",
                #[cfg(feature = "me")] // Montenegro (Europe)
                GEC::MJ => "MJ",
                #[cfg(feature = "mf")] // The Collectivity of Saint-Martin (Americas)
                GEC::RN => "RN",
                #[cfg(feature = "mg")] // The Republic of Madagascar (Africa)
                GEC::MA => "MA",
                #[cfg(feature = "mh")] // The Republic of the Marshall Islands (Oceania)
                GEC::RM => "RM",
                #[cfg(feature = "mk")] // The Republic of North Macedonia (Europe)
                GEC::MK => "MK",
                #[cfg(feature = "ml")] // The Republic of Mali (Africa)
                GEC::ML => "ML",
                #[cfg(feature = "mm")] // The Republic of the Union of Myanmar (Asia)
                GEC::BM => "BM",
                #[cfg(feature = "mn")] // Mongolia (Asia)
                GEC::MG => "MG",
                #[cfg(feature = "mo")] // The Macao Special Administrative Region of China (Asia)
                GEC::MC => "MC",
                #[cfg(feature = "mp")] // The Commonwealth of the Northern Mariana Islands (Oceania)
                GEC::CQ => "CQ",
                #[cfg(feature = "mq")] // Martinique (Americas)
                GEC::MB => "MB",
                #[cfg(feature = "mr")] // The Islamic Republic of Mauritania (Africa)
                GEC::MR => "MR",
                #[cfg(feature = "ms")] // Montserrat (Americas)
                GEC::MH => "MH",
                #[cfg(feature = "mt")] // The Republic of Malta (Europe)
                GEC::MT => "MT",
                #[cfg(feature = "mu")] // The Republic of Mauritius (Africa)
                GEC::MP => "MP",
                #[cfg(feature = "mv")] // The Republic of Maldives (Asia)
                GEC::MV => "MV",
                #[cfg(feature = "mw")] // The Republic of Malawi (Africa)
                GEC::MI => "MI",
                #[cfg(feature = "mx")] // The United Mexican States (Americas)
                GEC::MX => "MX",
                #[cfg(feature = "my")] // Malaysia (Asia)
                GEC::MY => "MY",
                #[cfg(feature = "mz")] // The Republic of Mozambique (Africa)
                GEC::MZ => "MZ",
                #[cfg(feature = "na")] // The Republic of Namibia (Africa)
                GEC::WA => "WA",
                #[cfg(feature = "nc")] // New Caledonia (Oceania)
                GEC::NC => "NC",
                #[cfg(feature = "ne")] // The Republic of the Niger (Africa)
                GEC::NG => "NG",
                #[cfg(feature = "nf")] // The Territory of Norfolk Island (Oceania)
                GEC::NF => "NF",
                #[cfg(feature = "ng")] // The Federal Republic of Nigeria (Africa)
                GEC::NI => "NI",
                #[cfg(feature = "ni")] // The Republic of Nicaragua (Americas)
                GEC::NU => "NU",
                #[cfg(feature = "nl")] // The Kingdom of the Netherlands (Europe)
                GEC::NL => "NL",
                #[cfg(feature = "no")] // The Kingdom of Norway (Europe)
                GEC::NO => "NO",
                #[cfg(feature = "np")] // The Federal Democratic Republic of Nepal (Asia)
                GEC::NP => "NP",
                #[cfg(feature = "nr")] // The Republic of Nauru (Oceania)
                GEC::NR => "NR",
                #[cfg(feature = "nu")] // Niue (Oceania)
                GEC::NE => "NE",
                #[cfg(feature = "nz")] // New Zealand (Oceania)
                GEC::NZ => "NZ",
                #[cfg(feature = "om")] // The Sultanate of Oman (Asia)
                GEC::MU => "MU",
                #[cfg(feature = "pa")] // The Republic of Panamá (Americas)
                GEC::PM => "PM",
                #[cfg(feature = "pe")] // The Republic of Perú (Americas)
                GEC::PE => "PE",
                #[cfg(feature = "pf")] // French Polynesia (Oceania)
                GEC::FP => "FP",
                #[cfg(feature = "pg")] // The Independent State of Papua New Guinea (Oceania)
                GEC::PP => "PP",
                #[cfg(feature = "ph")] // The Republic of the Philippines (Asia)
                GEC::RP => "RP",
                #[cfg(feature = "pk")] // The Islamic Republic of Pakistan (Asia)
                GEC::PK => "PK",
                #[cfg(feature = "pl")] // The Republic of Poland (Europe)
                GEC::PL => "PL",
                #[cfg(feature = "pm")]
                // The Overseas Collectivity of Saint-Pierre and Miquelon (Americas)
                GEC::SB => "SB",
                #[cfg(feature = "pn")] // The Pitcairn, Henderson, Ducie and Oeno Islands (Oceania)
                GEC::PC => "PC",
                #[cfg(feature = "pr")] // The Commonwealth of Puerto Rico (Americas)
                GEC::RQ => "RQ",
                #[cfg(feature = "ps")] // The State of Palestine (Asia)
                GEC::WE => "WE",
                #[cfg(feature = "pt")] // The Portuguese Republic (Europe)
                GEC::PO => "PO",
                #[cfg(feature = "pw")] // The Republic of Palau (Oceania)
                GEC::PS => "PS",
                #[cfg(feature = "py")] // The Republic of Paraguay (Americas)
                GEC::PA => "PA",
                #[cfg(feature = "qa")] // The State of Qatar (Asia)
                GEC::QA => "QA",
                #[cfg(feature = "re")] // Réunion (Africa)
                GEC::RE => "RE",
                #[cfg(feature = "ro")] // Romania (Europe)
                GEC::RO => "RO",
                #[cfg(feature = "rs")] // The Republic of Serbia (Europe)
                GEC::RI => "RI",
                #[cfg(feature = "ru")] // The Russian Federation (Europe)
                GEC::RS => "RS",
                #[cfg(feature = "rw")] // The Republic of Rwanda (Africa)
                GEC::RW => "RW",
                #[cfg(feature = "sa")] // The Kingdom of Saudi Arabia (Asia)
                GEC::SA => "SA",
                #[cfg(feature = "sb")] // The Solomon Islands (Oceania)
                GEC::BP => "BP",
                #[cfg(feature = "sc")] // The Republic of Seychelles (Africa)
                GEC::SE => "SE",
                #[cfg(feature = "sd")] // The Republic of the Sudan (Africa)
                GEC::SU => "SU",
                #[cfg(feature = "se")] // The Kingdom of Sweden (Europe)
                GEC::SW => "SW",
                #[cfg(feature = "sg")] // The Republic of Singapore (Asia)
                GEC::SN => "SN",
                #[cfg(feature = "sh")] // Saint Helena, Ascension and Tristan da Cunha (Africa)
                GEC::SH => "SH",
                #[cfg(feature = "si")] // The Republic of Slovenia (Europe)
                GEC::SI => "SI",
                #[cfg(feature = "sj")] // Svalbard and Jan Mayen (Europe)
                GEC::SV => "SV",
                #[cfg(feature = "sk")] // The Slovak Republic (Europe)
                GEC::LO => "LO",
                #[cfg(feature = "sl")] // The Republic of Sierra Leone (Africa)
                GEC::SL => "SL",
                #[cfg(feature = "sm")] // The Republic of San Marino (Europe)
                GEC::SM => "SM",
                #[cfg(feature = "sn")] // The Republic of Senegal (Africa)
                GEC::SG => "SG",
                #[cfg(feature = "so")] // The Federal Republic of Somalia (Africa)
                GEC::SO => "SO",
                #[cfg(feature = "sr")] // The Republic of Suriname (Americas)
                GEC::NS => "NS",
                #[cfg(feature = "ss")] // The Republic of South Sudan (Africa)
                GEC::OD => "OD",
                #[cfg(feature = "st")] // The Democratic Republic of São Tomé and Príncipe (Africa)
                GEC::TP => "TP",
                #[cfg(feature = "sv")] // The Republic of El Salvador (Americas)
                GEC::ES => "ES",
                #[cfg(feature = "sx")] // Sint Maarten (Americas)
                GEC::NN => "NN",
                #[cfg(feature = "sy")] // The Syrian Arab Republic (Asia)
                GEC::SY => "SY",
                #[cfg(feature = "sz")] // The Kingdom of Eswatini (Africa)
                GEC::WZ => "WZ",
                #[cfg(feature = "tc")] // The Turks and Caicos Islands (Americas)
                GEC::TK => "TK",
                #[cfg(feature = "td")] // The Republic of Chad (Africa)
                GEC::CD => "CD",
                #[cfg(feature = "tf")] // The French Southern and Antarctic Lands (Africa)
                GEC::FS => "FS",
                #[cfg(feature = "tg")] // The Togolese Republic (Africa)
                GEC::TO => "TO",
                #[cfg(feature = "th")] // The Kingdom of Thailand (Asia)
                GEC::TH => "TH",
                #[cfg(feature = "tj")] // The Republic of Tajikistan (Asia)
                GEC::TI => "TI",
                #[cfg(feature = "tk")] // Tokelau (Oceania)
                GEC::TL => "TL",
                #[cfg(feature = "tl")] // The Democratic Republic of Timor-Leste (Asia)
                GEC::TT => "TT",
                #[cfg(feature = "tm")] // Turkmenistan (Asia)
                GEC::TX => "TX",
                #[cfg(feature = "tn")] // The Republic of Tunisia (Africa)
                GEC::TS => "TS",
                #[cfg(feature = "to")] // The Kingdom of Tonga (Oceania)
                GEC::TN => "TN",
                #[cfg(feature = "tr")] // The Republic of Turkey (Asia)
                GEC::TU => "TU",
                #[cfg(feature = "tt")] // The Republic of Trinidad and Tobago (Americas)
                GEC::TD => "TD",
                #[cfg(feature = "tv")] // Tuvalu (Oceania)
                GEC::TV => "TV",
                #[cfg(feature = "tw")] // The Republic of China (Asia)
                GEC::TW => "TW",
                #[cfg(feature = "tz")] // The United Republic of Tanzania (Africa)
                GEC::TZ => "TZ",
                #[cfg(feature = "ua")] // Ukraine (Europe)
                GEC::UP => "UP",
                #[cfg(feature = "ug")] // The Republic of Uganda (Africa)
                GEC::UG => "UG",
                #[cfg(feature = "us")] // The United States of America (Americas)
                GEC::US => "US",
                #[cfg(feature = "uy")] // The Oriental Republic of Uruguay (Americas)
                GEC::UY => "UY",
                #[cfg(feature = "uz")] // The Republic of Uzbekistan (Asia)
                GEC::UZ => "UZ",
                #[cfg(feature = "va")] // The Holy See (Europe)
                GEC::VT => "VT",
                #[cfg(feature = "vc")] // Saint Vincent and the Grenadines (Americas)
                GEC::VC => "VC",
                #[cfg(feature = "ve")] // The Bolivarian Republic of Venezuela (Americas)
                GEC::VE => "VE",
                #[cfg(feature = "vg")] // The Virgin Islands (Americas)
                GEC::VI => "VI",
                #[cfg(feature = "vi")] // The Virgin Islands of the United States (Americas)
                GEC::VQ => "VQ",
                #[cfg(feature = "vn")] // The Socialist Republic of Viet Nam (Asia)
                GEC::VM => "VM",
                #[cfg(feature = "vu")] // The Republic of Vanuatu (Oceania)
                GEC::NH => "NH",
                #[cfg(feature = "wf")] // The Territory of the Wallis and Futuna Islands (Oceania)
                GEC::WF => "WF",
                #[cfg(feature = "ws")] // The Independent State of Samoa (Oceania)
                GEC::WS => "WS",
                #[cfg(feature = "ye")] // The Republic of Yemen (Asia)
                GEC::YM => "YM",
                #[cfg(feature = "yt")] // The Department of Mayotte (Africa)
                GEC::MF => "MF",
                #[cfg(feature = "za")] // The Republic of South Africa (Africa)
                GEC::SF => "SF",
                #[cfg(feature = "zm")] // The Republic of Zambia (Africa)
                GEC::ZA => "ZA",
                #[cfg(feature = "zw")] // The Republic of Zimbabwe (Africa)
                GEC::ZI => "ZI",
            }
            .to_string()
        }
    }

    impl TryFrom<&str> for GEC {
        type Error = SearchError;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value.len() != 2 {
                return Err(SearchError::BadInput {
                    expected: "a string with two characters",
                });
            }
            match value.to_uppercase().as_str() {
                #[cfg(feature = "ad")] // The Principality of Andorra (Europe)
                "AN" => Ok(GEC::AN),
                #[cfg(feature = "ae")] // The United Arab Emirates (Asia)
                "AE" => Ok(GEC::AE),
                #[cfg(feature = "af")] // The Islamic Republic of Afghanistan (Asia)
                "AF" => Ok(GEC::AF),
                #[cfg(feature = "ag")] // Antigua and Barbuda (Americas)
                "AC" => Ok(GEC::AC),
                #[cfg(feature = "ai")] // Anguilla (Americas)
                "AV" => Ok(GEC::AV),
                #[cfg(feature = "al")] // The Republic of Albania (Europe)
                "AL" => Ok(GEC::AL),
                #[cfg(feature = "am")] // The Republic of Armenia (Asia)
                "AM" => Ok(GEC::AM),
                #[cfg(feature = "ao")] // The Republic of Angola (Africa)
                "AO" => Ok(GEC::AO),
                #[cfg(feature = "aq")] // Antarctica
                "AY" => Ok(GEC::AY),
                #[cfg(feature = "ar")] // The Argentine Republic (Americas)
                "AR" => Ok(GEC::AR),
                #[cfg(feature = "as")] // The Territory of American Samoa (Oceania)
                "AQ" => Ok(GEC::AQ),
                #[cfg(feature = "at")] // The Republic of Austria (Europe)
                "AU" => Ok(GEC::AU),
                #[cfg(feature = "au")] // The Commonwealth of Australia (Oceania)
                "AS" => Ok(GEC::AS),
                #[cfg(feature = "aw")] // Aruba (Americas)
                "AA" => Ok(GEC::AA),
                #[cfg(feature = "az")] // The Republic of Azerbaijan (Asia)
                "AJ" => Ok(GEC::AJ),
                #[cfg(feature = "ba")] // Bosnia and Herzegovina (Europe)
                "BK" => Ok(GEC::BK),
                #[cfg(feature = "bb")] // Barbados (Americas)
                "BB" => Ok(GEC::BB),
                #[cfg(feature = "bd")] // The People's Republic of Bangladesh (Asia)
                "BG" => Ok(GEC::BG),
                #[cfg(feature = "be")] // The Kingdom of Belgium (Europe)
                "BE" => Ok(GEC::BE),
                #[cfg(feature = "bf")] // Burkina Faso (Africa)
                "UV" => Ok(GEC::UV),
                #[cfg(feature = "bg")] // The Republic of Bulgaria (Europe)
                "BU" => Ok(GEC::BU),
                #[cfg(feature = "bh")] // The Kingdom of Bahrain (Asia)
                "BA" => Ok(GEC::BA),
                #[cfg(feature = "bi")] // The Republic of Burundi (Africa)
                "BY" => Ok(GEC::BY),
                #[cfg(feature = "bj")] // The Republic of Benin (Africa)
                "BN" => Ok(GEC::BN),
                #[cfg(feature = "bl")] // The Collectivity of Saint-Barthélemy (Americas)
                "TB" => Ok(GEC::TB),
                #[cfg(feature = "bm")] // Bermuda (Americas)
                "BD" => Ok(GEC::BD),
                #[cfg(feature = "bn")] // The Nation of Brunei, the Abode of Peace (Asia)
                "BX" => Ok(GEC::BX),
                #[cfg(feature = "bo")] // The Plurinational State of Bolivia (Americas)
                "BL" => Ok(GEC::BL),
                #[cfg(feature = "br")] // The Federative Republic of Brazil (Americas)
                "BR" => Ok(GEC::BR),
                #[cfg(feature = "bs")] // The Commonwealth of The Bahamas (Americas)
                "BF" => Ok(GEC::BF),
                #[cfg(feature = "bt")] // The Kingdom of Bhutan (Asia)
                "BT" => Ok(GEC::BT),
                #[cfg(feature = "bv")] // Bouvet Island
                "BV" => Ok(GEC::BV),
                #[cfg(feature = "bw")] // The Republic of Botswana (Africa)
                "BC" => Ok(GEC::BC),
                #[cfg(feature = "by")] // The Republic of Belarus (Europe)
                "BO" => Ok(GEC::BO),
                #[cfg(feature = "bz")] // Belize (Americas)
                "BH" => Ok(GEC::BH),
                #[cfg(feature = "ca")] // Canada (Americas)
                "CA" => Ok(GEC::CA),
                #[cfg(feature = "cc")] // The Territory of Cocos (Keeling) Islands (Oceania)
                "CK" => Ok(GEC::CK),
                #[cfg(feature = "cd")] // The Democratic Republic of the Congo (Africa)
                "CG" => Ok(GEC::CG),
                #[cfg(feature = "cf")] // The Central African Republic (Africa)
                "CT" => Ok(GEC::CT),
                #[cfg(feature = "cg")] // The Republic of the Congo (Africa)
                "CF" => Ok(GEC::CF),
                #[cfg(feature = "ch")] // The Swiss Confederation (Europe)
                "SZ" => Ok(GEC::SZ),
                #[cfg(feature = "ci")] // The Republic of Côte d'Ivoire (Africa)
                "IV" => Ok(GEC::IV),
                #[cfg(feature = "ck")] // The Cook Islands (Oceania)
                "CW" => Ok(GEC::CW),
                #[cfg(feature = "cl")] // The Republic of Chile (Americas)
                "CI" => Ok(GEC::CI),
                #[cfg(feature = "cm")] // The Republic of Cameroon (Africa)
                "CM" => Ok(GEC::CM),
                #[cfg(feature = "cn")] // The People's Republic of China (Asia)
                "CH" => Ok(GEC::CH),
                #[cfg(feature = "co")] // The Republic of Colombia (Americas)
                "CO" => Ok(GEC::CO),
                #[cfg(feature = "cr")] // The Republic of Costa Rica (Americas)
                "CS" => Ok(GEC::CS),
                #[cfg(feature = "cu")] // The Republic of Cuba (Americas)
                "CU" => Ok(GEC::CU),
                #[cfg(feature = "cv")] // The Republic of Cabo Verde (Africa)
                "CV" => Ok(GEC::CV),
                #[cfg(feature = "cw")] // The Country of Curaçao (Americas)
                "UC" => Ok(GEC::UC),
                #[cfg(feature = "cx")] // The Territory of Christmas Island (Oceania)
                "KT" => Ok(GEC::KT),
                #[cfg(feature = "cy")] // The Republic of Cyprus (Asia)
                "CY" => Ok(GEC::CY),
                #[cfg(feature = "cz")] // The Czech Republic (Europe)
                "EZ" => Ok(GEC::EZ),
                #[cfg(feature = "de")] // The Federal Republic of Germany (Europe)
                "GM" => Ok(GEC::GM),
                #[cfg(feature = "dj")] // The Republic of Djibouti (Africa)
                "DJ" => Ok(GEC::DJ),
                #[cfg(feature = "dk")] // The Kingdom of Denmark (Europe)
                "DA" => Ok(GEC::DA),
                #[cfg(feature = "dm")] // The Commonwealth of Dominica (Americas)
                "DO" => Ok(GEC::DO),
                #[cfg(feature = "do")] // The Dominican Republic (Americas)
                "DR" => Ok(GEC::DR),
                #[cfg(feature = "dz")] // The People's Democratic Republic of Algeria (Africa)
                "AG" => Ok(GEC::AG),
                #[cfg(feature = "ec")] // The Republic of Ecuador (Americas)
                "EC" => Ok(GEC::EC),
                #[cfg(feature = "ee")] // The Republic of Estonia (Europe)
                "EN" => Ok(GEC::EN),
                #[cfg(feature = "eg")] // The Arab Republic of Egypt (Africa)
                "EG" => Ok(GEC::EG),
                #[cfg(feature = "eh")] // The Sahrawi Arab Democratic Republic (Africa)
                "WI" => Ok(GEC::WI),
                #[cfg(feature = "er")] // The State of Eritrea (Africa)
                "ER" => Ok(GEC::ER),
                #[cfg(feature = "es")] // The Kingdom of Spain (Europe)
                "SP" => Ok(GEC::SP),
                #[cfg(feature = "et")] // The Federal Democratic Republic of Ethiopia (Africa)
                "ET" => Ok(GEC::ET),
                #[cfg(feature = "fi")] // The Republic of Finland (Europe)
                "FI" => Ok(GEC::FI),
                #[cfg(feature = "fj")] // The Republic of Fiji (Oceania)
                "FJ" => Ok(GEC::FJ),
                #[cfg(feature = "fk")] // The Falkland Islands (Americas)
                "FK" => Ok(GEC::FK),
                #[cfg(feature = "fm")] // The Federated States of Micronesia (Oceania)
                "FM" => Ok(GEC::FM),
                #[cfg(feature = "fo")] // The Faroe Islands (Europe)
                "FO" => Ok(GEC::FO),
                #[cfg(feature = "fr")] // The French Republic (Europe)
                "FR" => Ok(GEC::FR),
                #[cfg(feature = "ga")] // The Gabonese Republic (Africa)
                "GB" => Ok(GEC::GB),
                #[cfg(feature = "gb")]
                // The United Kingdom of Great Britain and Northern Ireland (Europe)
                "UK" => Ok(GEC::UK),
                #[cfg(feature = "gd")] // Grenada (Americas)
                "GJ" => Ok(GEC::GJ),
                #[cfg(feature = "ge")] // Georgia (Asia)
                "GG" => Ok(GEC::GG),
                #[cfg(feature = "gf")] // Guyane (Americas)
                "FG" => Ok(GEC::FG),
                #[cfg(feature = "gg")] // The Bailiwick of Guernsey (Europe)
                "GK" => Ok(GEC::GK),
                #[cfg(feature = "gh")] // The Republic of Ghana (Africa)
                "GH" => Ok(GEC::GH),
                #[cfg(feature = "gi")] // Gibraltar (Europe)
                "GI" => Ok(GEC::GI),
                #[cfg(feature = "gl")] // Kalaallit Nunaat (Americas)
                "GL" => Ok(GEC::GL),
                #[cfg(feature = "gm")] // The Republic of The Gambia (Africa)
                "GA" => Ok(GEC::GA),
                #[cfg(feature = "gn")] // The Republic of Guinea (Africa)
                "GV" => Ok(GEC::GV),
                #[cfg(feature = "gp")] // Guadeloupe (Americas)
                "GP" => Ok(GEC::GP),
                #[cfg(feature = "gq")] // The Republic of Equatorial Guinea (Africa)
                "EK" => Ok(GEC::EK),
                #[cfg(feature = "gr")] // The Hellenic Republic (Europe)
                "GR" => Ok(GEC::GR),
                #[cfg(feature = "gs")] // South Georgia and the South Sandwich Islands (Americas)
                "SX" => Ok(GEC::SX),
                #[cfg(feature = "gt")] // The Republic of Guatemala (Americas)
                "GT" => Ok(GEC::GT),
                #[cfg(feature = "gu")] // The Territory of Guam (Oceania)
                "GQ" => Ok(GEC::GQ),
                #[cfg(feature = "gw")] // The Republic of Guinea-Bissau (Africa)
                "PU" => Ok(GEC::PU),
                #[cfg(feature = "gy")] // The Co-operative Republic of Guyana (Americas)
                "GY" => Ok(GEC::GY),
                #[cfg(feature = "hk")]
                // The Hong Kong Special Administrative Region of China (Asia)
                "HK" => Ok(GEC::HK),
                #[cfg(feature = "hm")] // The Territory of Heard Island and McDonald Islands
                "HM" => Ok(GEC::HM),
                #[cfg(feature = "hn")] // The Republic of Honduras (Americas)
                "HO" => Ok(GEC::HO),
                #[cfg(feature = "hr")] // The Republic of Croatia (Europe)
                "HR" => Ok(GEC::HR),
                #[cfg(feature = "ht")] // The Republic of Haiti (Americas)
                "HA" => Ok(GEC::HA),
                #[cfg(feature = "hu")] // Hungary (Europe)
                "HU" => Ok(GEC::HU),
                #[cfg(feature = "id")] // The Republic of Indonesia (Asia)
                "ID" => Ok(GEC::ID),
                #[cfg(feature = "ie")] // Ireland (Europe)
                "EI" => Ok(GEC::EI),
                #[cfg(feature = "il")] // The State of Israel (Asia)
                "IS" => Ok(GEC::IS),
                #[cfg(feature = "im")] // The Isle of Man (Europe)
                "IM" => Ok(GEC::IM),
                #[cfg(feature = "in")] // The Republic of India (Asia)
                "IN" => Ok(GEC::IN),
                #[cfg(feature = "io")] // The British Indian Ocean Territory (Africa)
                "IO" => Ok(GEC::IO),
                #[cfg(feature = "iq")] // The Republic of Iraq (Asia)
                "IZ" => Ok(GEC::IZ),
                #[cfg(feature = "ir")] // The Islamic Republic of Iran (Asia)
                "IR" => Ok(GEC::IR),
                #[cfg(feature = "is")] // Iceland (Europe)
                "IC" => Ok(GEC::IC),
                #[cfg(feature = "it")] // The Italian Republic (Europe)
                "IT" => Ok(GEC::IT),
                #[cfg(feature = "je")] // The Bailiwick of Jersey (Europe)
                "JE" => Ok(GEC::JE),
                #[cfg(feature = "jm")] // Jamaica (Americas)
                "JM" => Ok(GEC::JM),
                #[cfg(feature = "jo")] // The Hashemite Kingdom of Jordan (Asia)
                "JO" => Ok(GEC::JO),
                #[cfg(feature = "jp")] // Japan (Asia)
                "JA" => Ok(GEC::JA),
                #[cfg(feature = "ke")] // The Republic of Kenya (Africa)
                "KE" => Ok(GEC::KE),
                #[cfg(feature = "kg")] // The Kyrgyz Republic (Asia)
                "KG" => Ok(GEC::KG),
                #[cfg(feature = "kh")] // The Kingdom of Cambodia (Asia)
                "CB" => Ok(GEC::CB),
                #[cfg(feature = "ki")] // The Republic of Kiribati (Oceania)
                "KR" => Ok(GEC::KR),
                #[cfg(feature = "km")] // The Union of the Comoros (Africa)
                "CN" => Ok(GEC::CN),
                #[cfg(feature = "kn")] // Saint Kitts and Nevis (Americas)
                "SC" => Ok(GEC::SC),
                #[cfg(feature = "kp")] // The Democratic People's Republic of Korea (Asia)
                "KN" => Ok(GEC::KN),
                #[cfg(feature = "kr")] // The Republic of Korea (Asia)
                "KS" => Ok(GEC::KS),
                #[cfg(feature = "kw")] // The State of Kuwait (Asia)
                "KU" => Ok(GEC::KU),
                #[cfg(feature = "ky")] // The Cayman Islands (Americas)
                "CJ" => Ok(GEC::CJ),
                #[cfg(feature = "kz")] // The Republic of Kazakhstan (Asia)
                "KZ" => Ok(GEC::KZ),
                #[cfg(feature = "la")] // The Lao People's Democratic Republic (Asia)
                "LA" => Ok(GEC::LA),
                #[cfg(feature = "lb")] // The Lebanese Republic (Asia)
                "LE" => Ok(GEC::LE),
                #[cfg(feature = "lc")] // Saint Lucia (Americas)
                "ST" => Ok(GEC::ST),
                #[cfg(feature = "li")] // The Principality of Liechtenstein (Europe)
                "LS" => Ok(GEC::LS),
                #[cfg(feature = "lk")] // The Democratic Socialist Republic of Sri Lanka (Asia)
                "CE" => Ok(GEC::CE),
                #[cfg(feature = "lr")] // The Republic of Liberia (Africa)
                "LI" => Ok(GEC::LI),
                #[cfg(feature = "ls")] // The Kingdom of Lesotho (Africa)
                "LT" => Ok(GEC::LT),
                #[cfg(feature = "lt")] // The Republic of Lithuania (Europe)
                "LH" => Ok(GEC::LH),
                #[cfg(feature = "lu")] // The Grand Duchy of Luxembourg (Europe)
                "LU" => Ok(GEC::LU),
                #[cfg(feature = "lv")] // The Republic of Latvia (Europe)
                "LG" => Ok(GEC::LG),
                #[cfg(feature = "ly")] // The State of Libya (Africa)
                "LY" => Ok(GEC::LY),
                #[cfg(feature = "ma")] // The Kingdom of Morocco (Africa)
                "MO" => Ok(GEC::MO),
                #[cfg(feature = "mc")] // The Principality of Monaco (Europe)
                "MN" => Ok(GEC::MN),
                #[cfg(feature = "md")] // The Republic of Moldova (Europe)
                "MD" => Ok(GEC::MD),
                #[cfg(feature = "me")] // Montenegro (Europe)
                "MJ" => Ok(GEC::MJ),
                #[cfg(feature = "mf")] // The Collectivity of Saint-Martin (Americas)
                "RN" => Ok(GEC::RN),
                #[cfg(feature = "mg")] // The Republic of Madagascar (Africa)
                "MA" => Ok(GEC::MA),
                #[cfg(feature = "mh")] // The Republic of the Marshall Islands (Oceania)
                "RM" => Ok(GEC::RM),
                #[cfg(feature = "mk")] // The Republic of North Macedonia (Europe)
                "MK" => Ok(GEC::MK),
                #[cfg(feature = "ml")] // The Republic of Mali (Africa)
                "ML" => Ok(GEC::ML),
                #[cfg(feature = "mm")] // The Republic of the Union of Myanmar (Asia)
                "BM" => Ok(GEC::BM),
                #[cfg(feature = "mn")] // Mongolia (Asia)
                "MG" => Ok(GEC::MG),
                #[cfg(feature = "mo")] // The Macao Special Administrative Region of China (Asia)
                "MC" => Ok(GEC::MC),
                #[cfg(feature = "mp")] // The Commonwealth of the Northern Mariana Islands (Oceania)
                "CQ" => Ok(GEC::CQ),
                #[cfg(feature = "mq")] // Martinique (Americas)
                "MB" => Ok(GEC::MB),
                #[cfg(feature = "mr")] // The Islamic Republic of Mauritania (Africa)
                "MR" => Ok(GEC::MR),
                #[cfg(feature = "ms")] // Montserrat (Americas)
                "MH" => Ok(GEC::MH),
                #[cfg(feature = "mt")] // The Republic of Malta (Europe)
                "MT" => Ok(GEC::MT),
                #[cfg(feature = "mu")] // The Republic of Mauritius (Africa)
                "MP" => Ok(GEC::MP),
                #[cfg(feature = "mv")] // The Republic of Maldives (Asia)
                "MV" => Ok(GEC::MV),
                #[cfg(feature = "mw")] // The Republic of Malawi (Africa)
                "MI" => Ok(GEC::MI),
                #[cfg(feature = "mx")] // The United Mexican States (Americas)
                "MX" => Ok(GEC::MX),
                #[cfg(feature = "my")] // Malaysia (Asia)
                "MY" => Ok(GEC::MY),
                #[cfg(feature = "mz")] // The Republic of Mozambique (Africa)
                "MZ" => Ok(GEC::MZ),
                #[cfg(feature = "na")] // The Republic of Namibia (Africa)
                "WA" => Ok(GEC::WA),
                #[cfg(feature = "nc")] // New Caledonia (Oceania)
                "NC" => Ok(GEC::NC),
                #[cfg(feature = "ne")] // The Republic of the Niger (Africa)
                "NG" => Ok(GEC::NG),
                #[cfg(feature = "nf")] // The Territory of Norfolk Island (Oceania)
                "NF" => Ok(GEC::NF),
                #[cfg(feature = "ng")] // The Federal Republic of Nigeria (Africa)
                "NI" => Ok(GEC::NI),
                #[cfg(feature = "ni")] // The Republic of Nicaragua (Americas)
                "NU" => Ok(GEC::NU),
                #[cfg(feature = "nl")] // The Kingdom of the Netherlands (Europe)
                "NL" => Ok(GEC::NL),
                #[cfg(feature = "no")] // The Kingdom of Norway (Europe)
                "NO" => Ok(GEC::NO),
                #[cfg(feature = "np")] // The Federal Democratic Republic of Nepal (Asia)
                "NP" => Ok(GEC::NP),
                #[cfg(feature = "nr")] // The Republic of Nauru (Oceania)
                "NR" => Ok(GEC::NR),
                #[cfg(feature = "nu")] // Niue (Oceania)
                "NE" => Ok(GEC::NE),
                #[cfg(feature = "nz")] // New Zealand (Oceania)
                "NZ" => Ok(GEC::NZ),
                #[cfg(feature = "om")] // The Sultanate of Oman (Asia)
                "MU" => Ok(GEC::MU),
                #[cfg(feature = "pa")] // The Republic of Panamá (Americas)
                "PM" => Ok(GEC::PM),
                #[cfg(feature = "pe")] // The Republic of Perú (Americas)
                "PE" => Ok(GEC::PE),
                #[cfg(feature = "pf")] // French Polynesia (Oceania)
                "FP" => Ok(GEC::FP),
                #[cfg(feature = "pg")] // The Independent State of Papua New Guinea (Oceania)
                "PP" => Ok(GEC::PP),
                #[cfg(feature = "ph")] // The Republic of the Philippines (Asia)
                "RP" => Ok(GEC::RP),
                #[cfg(feature = "pk")] // The Islamic Republic of Pakistan (Asia)
                "PK" => Ok(GEC::PK),
                #[cfg(feature = "pl")] // The Republic of Poland (Europe)
                "PL" => Ok(GEC::PL),
                #[cfg(feature = "pm")]
                // The Overseas Collectivity of Saint-Pierre and Miquelon (Americas)
                "SB" => Ok(GEC::SB),
                #[cfg(feature = "pn")] // The Pitcairn, Henderson, Ducie and Oeno Islands (Oceania)
                "PC" => Ok(GEC::PC),
                #[cfg(feature = "pr")] // The Commonwealth of Puerto Rico (Americas)
                "RQ" => Ok(GEC::RQ),
                #[cfg(feature = "ps")] // The State of Palestine (Asia)
                "WE" => Ok(GEC::WE),
                #[cfg(feature = "pt")] // The Portuguese Republic (Europe)
                "PO" => Ok(GEC::PO),
                #[cfg(feature = "pw")] // The Republic of Palau (Oceania)
                "PS" => Ok(GEC::PS),
                #[cfg(feature = "py")] // The Republic of Paraguay (Americas)
                "PA" => Ok(GEC::PA),
                #[cfg(feature = "qa")] // The State of Qatar (Asia)
                "QA" => Ok(GEC::QA),
                #[cfg(feature = "re")] // Réunion (Africa)
                "RE" => Ok(GEC::RE),
                #[cfg(feature = "ro")] // Romania (Europe)
                "RO" => Ok(GEC::RO),
                #[cfg(feature = "rs")] // The Republic of Serbia (Europe)
                "RI" => Ok(GEC::RI),
                #[cfg(feature = "ru")] // The Russian Federation (Europe)
                "RS" => Ok(GEC::RS),
                #[cfg(feature = "rw")] // The Republic of Rwanda (Africa)
                "RW" => Ok(GEC::RW),
                #[cfg(feature = "sa")] // The Kingdom of Saudi Arabia (Asia)
                "SA" => Ok(GEC::SA),
                #[cfg(feature = "sb")] // The Solomon Islands (Oceania)
                "BP" => Ok(GEC::BP),
                #[cfg(feature = "sc")] // The Republic of Seychelles (Africa)
                "SE" => Ok(GEC::SE),
                #[cfg(feature = "sd")] // The Republic of the Sudan (Africa)
                "SU" => Ok(GEC::SU),
                #[cfg(feature = "se")] // The Kingdom of Sweden (Europe)
                "SW" => Ok(GEC::SW),
                #[cfg(feature = "sg")] // The Republic of Singapore (Asia)
                "SN" => Ok(GEC::SN),
                #[cfg(feature = "sh")] // Saint Helena, Ascension and Tristan da Cunha (Africa)
                "SH" => Ok(GEC::SH),
                #[cfg(feature = "si")] // The Republic of Slovenia (Europe)
                "SI" => Ok(GEC::SI),
                #[cfg(feature = "sj")] // Svalbard and Jan Mayen (Europe)
                "SV" => Ok(GEC::SV),
                #[cfg(feature = "sk")] // The Slovak Republic (Europe)
                "LO" => Ok(GEC::LO),
                #[cfg(feature = "sl")] // The Republic of Sierra Leone (Africa)
                "SL" => Ok(GEC::SL),
                #[cfg(feature = "sm")] // The Republic of San Marino (Europe)
                "SM" => Ok(GEC::SM),
                #[cfg(feature = "sn")] // The Republic of Senegal (Africa)
                "SG" => Ok(GEC::SG),
                #[cfg(feature = "so")] // The Federal Republic of Somalia (Africa)
                "SO" => Ok(GEC::SO),
                #[cfg(feature = "sr")] // The Republic of Suriname (Americas)
                "NS" => Ok(GEC::NS),
                #[cfg(feature = "ss")] // The Republic of South Sudan (Africa)
                "OD" => Ok(GEC::OD),
                #[cfg(feature = "st")] // The Democratic Republic of São Tomé and Príncipe (Africa)
                "TP" => Ok(GEC::TP),
                #[cfg(feature = "sv")] // The Republic of El Salvador (Americas)
                "ES" => Ok(GEC::ES),
                #[cfg(feature = "sx")] // Sint Maarten (Americas)
                "NN" => Ok(GEC::NN),
                #[cfg(feature = "sy")] // The Syrian Arab Republic (Asia)
                "SY" => Ok(GEC::SY),
                #[cfg(feature = "sz")] // The Kingdom of Eswatini (Africa)
                "WZ" => Ok(GEC::WZ),
                #[cfg(feature = "tc")] // The Turks and Caicos Islands (Americas)
                "TK" => Ok(GEC::TK),
                #[cfg(feature = "td")] // The Republic of Chad (Africa)
                "CD" => Ok(GEC::CD),
                #[cfg(feature = "tf")] // The French Southern and Antarctic Lands (Africa)
                "FS" => Ok(GEC::FS),
                #[cfg(feature = "tg")] // The Togolese Republic (Africa)
                "TO" => Ok(GEC::TO),
                #[cfg(feature = "th")] // The Kingdom of Thailand (Asia)
                "TH" => Ok(GEC::TH),
                #[cfg(feature = "tj")] // The Republic of Tajikistan (Asia)
                "TI" => Ok(GEC::TI),
                #[cfg(feature = "tk")] // Tokelau (Oceania)
                "TL" => Ok(GEC::TL),
                #[cfg(feature = "tl")] // The Democratic Republic of Timor-Leste (Asia)
                "TT" => Ok(GEC::TT),
                #[cfg(feature = "tm")] // Turkmenistan (Asia)
                "TX" => Ok(GEC::TX),
                #[cfg(feature = "tn")] // The Republic of Tunisia (Africa)
                "TS" => Ok(GEC::TS),
                #[cfg(feature = "to")] // The Kingdom of Tonga (Oceania)
                "TN" => Ok(GEC::TN),
                #[cfg(feature = "tr")] // The Republic of Turkey (Asia)
                "TU" => Ok(GEC::TU),
                #[cfg(feature = "tt")] // The Republic of Trinidad and Tobago (Americas)
                "TD" => Ok(GEC::TD),
                #[cfg(feature = "tv")] // Tuvalu (Oceania)
                "TV" => Ok(GEC::TV),
                #[cfg(feature = "tw")] // The Republic of China (Asia)
                "TW" => Ok(GEC::TW),
                #[cfg(feature = "tz")] // The United Republic of Tanzania (Africa)
                "TZ" => Ok(GEC::TZ),
                #[cfg(feature = "ua")] // Ukraine (Europe)
                "UP" => Ok(GEC::UP),
                #[cfg(feature = "ug")] // The Republic of Uganda (Africa)
                "UG" => Ok(GEC::UG),
                #[cfg(feature = "us")] // The United States of America (Americas)
                "US" => Ok(GEC::US),
                #[cfg(feature = "uy")] // The Oriental Republic of Uruguay (Americas)
                "UY" => Ok(GEC::UY),
                #[cfg(feature = "uz")] // The Republic of Uzbekistan (Asia)
                "UZ" => Ok(GEC::UZ),
                #[cfg(feature = "va")] // The Holy See (Europe)
                "VT" => Ok(GEC::VT),
                #[cfg(feature = "vc")] // Saint Vincent and the Grenadines (Americas)
                "VC" => Ok(GEC::VC),
                #[cfg(feature = "ve")] // The Bolivarian Republic of Venezuela (Americas)
                "VE" => Ok(GEC::VE),
                #[cfg(feature = "vg")] // The Virgin Islands (Americas)
                "VI" => Ok(GEC::VI),
                #[cfg(feature = "vi")] // The Virgin Islands of the United States (Americas)
                "VQ" => Ok(GEC::VQ),
                #[cfg(feature = "vn")] // The Socialist Republic of Viet Nam (Asia)
                "VM" => Ok(GEC::VM),
                #[cfg(feature = "vu")] // The Republic of Vanuatu (Oceania)
                "NH" => Ok(GEC::NH),
                #[cfg(feature = "wf")] // The Territory of the Wallis and Futuna Islands (Oceania)
                "WF" => Ok(GEC::WF),
                #[cfg(feature = "ws")] // The Independent State of Samoa (Oceania)
                "WS" => Ok(GEC::WS),
                #[cfg(feature = "ye")] // The Republic of Yemen (Asia)
                "YM" => Ok(GEC::YM),
                #[cfg(feature = "yt")] // The Department of Mayotte (Africa)
                "MF" => Ok(GEC::MF),
                #[cfg(feature = "za")] // The Republic of South Africa (Africa)
                "SF" => Ok(GEC::SF),
                #[cfg(feature = "zm")] // The Republic of Zambia (Africa)
                "ZA" => Ok(GEC::ZA),
                #[cfg(feature = "zw")] // The Republic of Zimbabwe (Africa)
                "ZI" => Ok(GEC::ZI),
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
mod impls {
    use super::GEC;
    use crate::{Alpha2, Country, SearchError};

    impl GEC {
        pub fn to_alpha2(&self) -> Alpha2 {
            unimplemented!("No country feature with GEC code is used");
        }

        pub fn to_country(&self) -> Country {
            unimplemented!("No country feature with GEC code is used");
        }
    }

    impl ToString for GEC {
        fn to_string(&self) -> String {
            unimplemented!("No country feature with GEC code is used");
        }
    }

    impl TryFrom<&str> for GEC {
        type Error = SearchError;
        fn try_from(_value: &str) -> Result<Self, Self::Error> {
            unimplemented!("No country feature with GEC code is used");
        }
    }
}
