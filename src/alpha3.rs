// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/alpha3.rs`)

use crate::{Alpha2, Country};

#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};

/// An enum containing Alpha3 codes for all countries.
///
/// All countries features are enabled by default. You can disable default features and enabled features for all countries that you need.
///
/// # Examples
/// ```
/// use keshvar::{Alpha2, Alpha3, Country, SearchError, SearchedItems};
///
/// assert_eq!(Ok(Alpha3::USA), Alpha3::try_from("usa")); // not case-sensitive
/// assert_eq!("USA", Alpha3::USA.to_string().as_str());
///
/// // If enabled all countries features:
/// assert_eq!(
///     Err(SearchError::NotFound {
///         searched_items: SearchedItems::AllCountries
///     }),
///     Alpha3::try_from("xxx")
/// );
/// assert_eq!(
///     Err("Could not be found in all countries".to_string()),
///     Alpha3::try_from("xxx").map_err(|error| error.to_string())
/// );
///
/// // If enabled some countries features:
/// // For example we enabled supporting just 10 countries and the US
/// // is not one of them:
/// // assert_eq!(
/// //     Err(SearchError::NotFound {
/// //         searched_items: SearchedItems::SupportedCountries(10)
/// //     }),
/// //     Alpha3::try_from("usa")
/// // );
/// // assert_eq!(
/// //     Err("Could not be found in 10 supported countries".to_string()),
/// //     Alpha3::try_from("usa").map_err(|error| error.to_string())
/// // );
///
/// // Convert to `Alpha2` code:
/// assert_eq!(Alpha2::US, Alpha3::USA.to_alpha2());
/// // Convert to `Country`:
/// let country: Country = Alpha3::USA.to_country();
/// // Get subdivisions of country:
/// let subdivisions = Alpha3::USA.to_subdivisions();
/// ```
/// We usually need to convert it to [`Country`](crate::Country) and use that object instead.
///
/// # Panics
/// Most methods will panic if you do not enable any country feature!
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub enum Alpha3 {
    #[cfg(feature = "ad")]
    /// The Principality of Andorra (Europe)
    AND,
    #[cfg(feature = "ae")]
    /// The United Arab Emirates (Asia)
    ARE,
    #[cfg(feature = "af")]
    /// The Islamic Republic of Afghanistan (Asia)
    AFG,
    #[cfg(feature = "ag")]
    /// Antigua and Barbuda (Americas)
    ATG,
    #[cfg(feature = "ai")]
    /// Anguilla (Americas)
    AIA,
    #[cfg(feature = "al")]
    /// The Republic of Albania (Europe)
    ALB,
    #[cfg(feature = "am")]
    /// The Republic of Armenia (Asia)
    ARM,
    #[cfg(feature = "ao")]
    /// The Republic of Angola (Africa)
    AGO,
    #[cfg(feature = "aq")]
    /// Antarctica
    ATA,
    #[cfg(feature = "ar")]
    /// The Argentine Republic (Americas)
    ARG,
    #[cfg(feature = "as")]
    /// The Territory of American Samoa (Oceania)
    ASM,
    #[cfg(feature = "at")]
    /// The Republic of Austria (Europe)
    AUT,
    #[cfg(feature = "au")]
    /// The Commonwealth of Australia (Oceania)
    AUS,
    #[cfg(feature = "aw")]
    /// Aruba (Americas)
    ABW,
    #[cfg(feature = "ax")]
    /// Åland (Europe)
    ALA,
    #[cfg(feature = "az")]
    /// The Republic of Azerbaijan (Asia)
    AZE,
    #[cfg(feature = "ba")]
    /// Bosnia and Herzegovina (Europe)
    BIH,
    #[cfg(feature = "bb")]
    /// Barbados (Americas)
    BRB,
    #[cfg(feature = "bd")]
    /// The People's Republic of Bangladesh (Asia)
    BGD,
    #[cfg(feature = "be")]
    /// The Kingdom of Belgium (Europe)
    BEL,
    #[cfg(feature = "bf")]
    /// Burkina Faso (Africa)
    BFA,
    #[cfg(feature = "bg")]
    /// The Republic of Bulgaria (Europe)
    BGR,
    #[cfg(feature = "bh")]
    /// The Kingdom of Bahrain (Asia)
    BHR,
    #[cfg(feature = "bi")]
    /// The Republic of Burundi (Africa)
    BDI,
    #[cfg(feature = "bj")]
    /// The Republic of Benin (Africa)
    BEN,
    #[cfg(feature = "bl")]
    /// The Collectivity of Saint-Barthélemy (Americas)
    BLM,
    #[cfg(feature = "bm")]
    /// Bermuda (Americas)
    BMU,
    #[cfg(feature = "bn")]
    /// The Nation of Brunei, the Abode of Peace (Asia)
    BRN,
    #[cfg(feature = "bo")]
    /// The Plurinational State of Bolivia (Americas)
    BOL,
    #[cfg(feature = "bq")]
    /// Bonaire, Sint Eustatius and Saba (Americas)
    BES,
    #[cfg(feature = "br")]
    /// The Federative Republic of Brazil (Americas)
    BRA,
    #[cfg(feature = "bs")]
    /// The Commonwealth of The Bahamas (Americas)
    BHS,
    #[cfg(feature = "bt")]
    /// The Kingdom of Bhutan (Asia)
    BTN,
    #[cfg(feature = "bv")]
    /// Bouvet Island
    BVT,
    #[cfg(feature = "bw")]
    /// The Republic of Botswana (Africa)
    BWA,
    #[cfg(feature = "by")]
    /// The Republic of Belarus (Europe)
    BLR,
    #[cfg(feature = "bz")]
    /// Belize (Americas)
    BLZ,
    #[cfg(feature = "ca")]
    /// Canada (Americas)
    CAN,
    #[cfg(feature = "cc")]
    /// The Territory of Cocos (Keeling) Islands (Oceania)
    CCK,
    #[cfg(feature = "cd")]
    /// The Democratic Republic of the Congo (Africa)
    COD,
    #[cfg(feature = "cf")]
    /// The Central African Republic (Africa)
    CAF,
    #[cfg(feature = "cg")]
    /// The Republic of the Congo (Africa)
    COG,
    #[cfg(feature = "ch")]
    /// The Swiss Confederation (Europe)
    CHE,
    #[cfg(feature = "ci")]
    /// The Republic of Côte d'Ivoire (Africa)
    CIV,
    #[cfg(feature = "ck")]
    /// The Cook Islands (Oceania)
    COK,
    #[cfg(feature = "cl")]
    /// The Republic of Chile (Americas)
    CHL,
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
    CRI,
    #[cfg(feature = "cu")]
    /// The Republic of Cuba (Americas)
    CUB,
    #[cfg(feature = "cv")]
    /// The Republic of Cabo Verde (Africa)
    CPV,
    #[cfg(feature = "cw")]
    /// The Country of Curaçao (Americas)
    CUW,
    #[cfg(feature = "cx")]
    /// The Territory of Christmas Island (Oceania)
    CXR,
    #[cfg(feature = "cy")]
    /// The Republic of Cyprus (Asia)
    CYP,
    #[cfg(feature = "cz")]
    /// The Czech Republic (Europe)
    CZE,
    #[cfg(feature = "de")]
    /// The Federal Republic of Germany (Europe)
    DEU,
    #[cfg(feature = "dj")]
    /// The Republic of Djibouti (Africa)
    DJI,
    #[cfg(feature = "dk")]
    /// The Kingdom of Denmark (Europe)
    DNK,
    #[cfg(feature = "dm")]
    /// The Commonwealth of Dominica (Americas)
    DMA,
    #[cfg(feature = "do")]
    /// The Dominican Republic (Americas)
    DOM,
    #[cfg(feature = "dz")]
    /// The People's Democratic Republic of Algeria (Africa)
    DZA,
    #[cfg(feature = "ec")]
    /// The Republic of Ecuador (Americas)
    ECU,
    #[cfg(feature = "ee")]
    /// The Republic of Estonia (Europe)
    EST,
    #[cfg(feature = "eg")]
    /// The Arab Republic of Egypt (Africa)
    EGY,
    #[cfg(feature = "eh")]
    /// The Sahrawi Arab Democratic Republic (Africa)
    ESH,
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
    FJI,
    #[cfg(feature = "fk")]
    /// The Falkland Islands (Americas)
    FLK,
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
    GRD,
    #[cfg(feature = "ge")]
    /// Georgia (Asia)
    GEO,
    #[cfg(feature = "gf")]
    /// Guyane (Americas)
    GUF,
    #[cfg(feature = "gg")]
    /// The Bailiwick of Guernsey (Europe)
    GGY,
    #[cfg(feature = "gh")]
    /// The Republic of Ghana (Africa)
    GHA,
    #[cfg(feature = "gi")]
    /// Gibraltar (Europe)
    GIB,
    #[cfg(feature = "gl")]
    /// Kalaallit Nunaat (Americas)
    GRL,
    #[cfg(feature = "gm")]
    /// The Republic of The Gambia (Africa)
    GMB,
    #[cfg(feature = "gn")]
    /// The Republic of Guinea (Africa)
    GIN,
    #[cfg(feature = "gp")]
    /// Guadeloupe (Americas)
    GLP,
    #[cfg(feature = "gq")]
    /// The Republic of Equatorial Guinea (Africa)
    GNQ,
    #[cfg(feature = "gr")]
    /// The Hellenic Republic (Europe)
    GRC,
    #[cfg(feature = "gs")]
    /// South Georgia and the South Sandwich Islands (Americas)
    SGS,
    #[cfg(feature = "gt")]
    /// The Republic of Guatemala (Americas)
    GTM,
    #[cfg(feature = "gu")]
    /// The Territory of Guam (Oceania)
    GUM,
    #[cfg(feature = "gw")]
    /// The Republic of Guinea-Bissau (Africa)
    GNB,
    #[cfg(feature = "gy")]
    /// The Co-operative Republic of Guyana (Americas)
    GUY,
    #[cfg(feature = "hk")]
    /// The Hong Kong Special Administrative Region of China (Asia)
    HKG,
    #[cfg(feature = "hm")]
    /// The Territory of Heard Island and McDonald Islands
    HMD,
    #[cfg(feature = "hn")]
    /// The Republic of Honduras (Americas)
    HND,
    #[cfg(feature = "hr")]
    /// The Republic of Croatia (Europe)
    HRV,
    #[cfg(feature = "ht")]
    /// The Republic of Haiti (Americas)
    HTI,
    #[cfg(feature = "hu")]
    /// Hungary (Europe)
    HUN,
    #[cfg(feature = "id")]
    /// The Republic of Indonesia (Asia)
    IDN,
    #[cfg(feature = "ie")]
    /// Ireland (Europe)
    IRL,
    #[cfg(feature = "il")]
    /// The State of Israel (Asia)
    ISR,
    #[cfg(feature = "im")]
    /// The Isle of Man (Europe)
    IMN,
    #[cfg(feature = "in")]
    /// The Republic of India (Asia)
    IND,
    #[cfg(feature = "io")]
    /// The British Indian Ocean Territory (Africa)
    IOT,
    #[cfg(feature = "iq")]
    /// The Republic of Iraq (Asia)
    IRQ,
    #[cfg(feature = "ir")]
    /// The Islamic Republic of Iran (Asia)
    IRN,
    #[cfg(feature = "is")]
    /// Iceland (Europe)
    ISL,
    #[cfg(feature = "it")]
    /// The Italian Republic (Europe)
    ITA,
    #[cfg(feature = "je")]
    /// The Bailiwick of Jersey (Europe)
    JEY,
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
    KHM,
    #[cfg(feature = "ki")]
    /// The Republic of Kiribati (Oceania)
    KIR,
    #[cfg(feature = "km")]
    /// The Union of the Comoros (Africa)
    COM,
    #[cfg(feature = "kn")]
    /// Saint Kitts and Nevis (Americas)
    KNA,
    #[cfg(feature = "kp")]
    /// The Democratic People's Republic of Korea (Asia)
    PRK,
    #[cfg(feature = "kr")]
    /// The Republic of Korea (Asia)
    KOR,
    #[cfg(feature = "kw")]
    /// The State of Kuwait (Asia)
    KWT,
    #[cfg(feature = "ky")]
    /// The Cayman Islands (Americas)
    CYM,
    #[cfg(feature = "kz")]
    /// The Republic of Kazakhstan (Asia)
    KAZ,
    #[cfg(feature = "la")]
    /// The Lao People's Democratic Republic (Asia)
    LAO,
    #[cfg(feature = "lb")]
    /// The Lebanese Republic (Asia)
    LBN,
    #[cfg(feature = "lc")]
    /// Saint Lucia (Americas)
    LCA,
    #[cfg(feature = "li")]
    /// The Principality of Liechtenstein (Europe)
    LIE,
    #[cfg(feature = "lk")]
    /// The Democratic Socialist Republic of Sri Lanka (Asia)
    LKA,
    #[cfg(feature = "lr")]
    /// The Republic of Liberia (Africa)
    LBR,
    #[cfg(feature = "ls")]
    /// The Kingdom of Lesotho (Africa)
    LSO,
    #[cfg(feature = "lt")]
    /// The Republic of Lithuania (Europe)
    LTU,
    #[cfg(feature = "lu")]
    /// The Grand Duchy of Luxembourg (Europe)
    LUX,
    #[cfg(feature = "lv")]
    /// The Republic of Latvia (Europe)
    LVA,
    #[cfg(feature = "ly")]
    /// The State of Libya (Africa)
    LBY,
    #[cfg(feature = "ma")]
    /// The Kingdom of Morocco (Africa)
    MAR,
    #[cfg(feature = "mc")]
    /// The Principality of Monaco (Europe)
    MCO,
    #[cfg(feature = "md")]
    /// The Republic of Moldova (Europe)
    MDA,
    #[cfg(feature = "me")]
    /// Montenegro (Europe)
    MNE,
    #[cfg(feature = "mf")]
    /// The Collectivity of Saint-Martin (Americas)
    MAF,
    #[cfg(feature = "mg")]
    /// The Republic of Madagascar (Africa)
    MDG,
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
    MMR,
    #[cfg(feature = "mn")]
    /// Mongolia (Asia)
    MNG,
    #[cfg(feature = "mo")]
    /// The Macao Special Administrative Region of China (Asia)
    MAC,
    #[cfg(feature = "mp")]
    /// The Commonwealth of the Northern Mariana Islands (Oceania)
    MNP,
    #[cfg(feature = "mq")]
    /// Martinique (Americas)
    MTQ,
    #[cfg(feature = "mr")]
    /// The Islamic Republic of Mauritania (Africa)
    MRT,
    #[cfg(feature = "ms")]
    /// Montserrat (Americas)
    MSR,
    #[cfg(feature = "mt")]
    /// The Republic of Malta (Europe)
    MLT,
    #[cfg(feature = "mu")]
    /// The Republic of Mauritius (Africa)
    MUS,
    #[cfg(feature = "mv")]
    /// The Republic of Maldives (Asia)
    MDV,
    #[cfg(feature = "mw")]
    /// The Republic of Malawi (Africa)
    MWI,
    #[cfg(feature = "mx")]
    /// The United Mexican States (Americas)
    MEX,
    #[cfg(feature = "my")]
    /// Malaysia (Asia)
    MYS,
    #[cfg(feature = "mz")]
    /// The Republic of Mozambique (Africa)
    MOZ,
    #[cfg(feature = "na")]
    /// The Republic of Namibia (Africa)
    NAM,
    #[cfg(feature = "nc")]
    /// New Caledonia (Oceania)
    NCL,
    #[cfg(feature = "ne")]
    /// The Republic of the Niger (Africa)
    NER,
    #[cfg(feature = "nf")]
    /// The Territory of Norfolk Island (Oceania)
    NFK,
    #[cfg(feature = "ng")]
    /// The Federal Republic of Nigeria (Africa)
    NGA,
    #[cfg(feature = "ni")]
    /// The Republic of Nicaragua (Americas)
    NIC,
    #[cfg(feature = "nl")]
    /// The Kingdom of the Netherlands (Europe)
    NLD,
    #[cfg(feature = "no")]
    /// The Kingdom of Norway (Europe)
    NOR,
    #[cfg(feature = "np")]
    /// The Federal Democratic Republic of Nepal (Asia)
    NPL,
    #[cfg(feature = "nr")]
    /// The Republic of Nauru (Oceania)
    NRU,
    #[cfg(feature = "nu")]
    /// Niue (Oceania)
    NIU,
    #[cfg(feature = "nz")]
    /// New Zealand (Oceania)
    NZL,
    #[cfg(feature = "om")]
    /// The Sultanate of Oman (Asia)
    OMN,
    #[cfg(feature = "pa")]
    /// The Republic of Panamá (Americas)
    PAN,
    #[cfg(feature = "pe")]
    /// The Republic of Perú (Americas)
    PER,
    #[cfg(feature = "pf")]
    /// French Polynesia (Oceania)
    PYF,
    #[cfg(feature = "pg")]
    /// The Independent State of Papua New Guinea (Oceania)
    PNG,
    #[cfg(feature = "ph")]
    /// The Republic of the Philippines (Asia)
    PHL,
    #[cfg(feature = "pk")]
    /// The Islamic Republic of Pakistan (Asia)
    PAK,
    #[cfg(feature = "pl")]
    /// The Republic of Poland (Europe)
    POL,
    #[cfg(feature = "pm")]
    /// The Overseas Collectivity of Saint-Pierre and Miquelon (Americas)
    SPM,
    #[cfg(feature = "pn")]
    /// The Pitcairn, Henderson, Ducie and Oeno Islands (Oceania)
    PCN,
    #[cfg(feature = "pr")]
    /// The Commonwealth of Puerto Rico (Americas)
    PRI,
    #[cfg(feature = "ps")]
    /// The State of Palestine (Asia)
    PSE,
    #[cfg(feature = "pt")]
    /// The Portuguese Republic (Europe)
    PRT,
    #[cfg(feature = "pw")]
    /// The Republic of Palau (Oceania)
    PLW,
    #[cfg(feature = "py")]
    /// The Republic of Paraguay (Americas)
    PRY,
    #[cfg(feature = "qa")]
    /// The State of Qatar (Asia)
    QAT,
    #[cfg(feature = "re")]
    /// Réunion (Africa)
    REU,
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
    SAU,
    #[cfg(feature = "sb")]
    /// The Solomon Islands (Oceania)
    SLB,
    #[cfg(feature = "sc")]
    /// The Republic of Seychelles (Africa)
    SYC,
    #[cfg(feature = "sd")]
    /// The Republic of the Sudan (Africa)
    SDN,
    #[cfg(feature = "se")]
    /// The Kingdom of Sweden (Europe)
    SWE,
    #[cfg(feature = "sg")]
    /// The Republic of Singapore (Asia)
    SGP,
    #[cfg(feature = "sh")]
    /// Saint Helena, Ascension and Tristan da Cunha (Africa)
    SHN,
    #[cfg(feature = "si")]
    /// The Republic of Slovenia (Europe)
    SVN,
    #[cfg(feature = "sj")]
    /// Svalbard and Jan Mayen (Europe)
    SJM,
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
    #[cfg(feature = "ss")]
    /// The Republic of South Sudan (Africa)
    SSD,
    #[cfg(feature = "st")]
    /// The Democratic Republic of São Tomé and Príncipe (Africa)
    STP,
    #[cfg(feature = "sv")]
    /// The Republic of El Salvador (Americas)
    SLV,
    #[cfg(feature = "sx")]
    /// Sint Maarten (Americas)
    SXM,
    #[cfg(feature = "sy")]
    /// The Syrian Arab Republic (Asia)
    SYR,
    #[cfg(feature = "sz")]
    /// The Kingdom of Eswatini (Africa)
    SWZ,
    #[cfg(feature = "tc")]
    /// The Turks and Caicos Islands (Americas)
    TCA,
    #[cfg(feature = "td")]
    /// The Republic of Chad (Africa)
    TCD,
    #[cfg(feature = "tf")]
    /// The French Southern and Antarctic Lands (Africa)
    ATF,
    #[cfg(feature = "tg")]
    /// The Togolese Republic (Africa)
    TGO,
    #[cfg(feature = "th")]
    /// The Kingdom of Thailand (Asia)
    THA,
    #[cfg(feature = "tj")]
    /// The Republic of Tajikistan (Asia)
    TJK,
    #[cfg(feature = "tk")]
    /// Tokelau (Oceania)
    TKL,
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
    TON,
    #[cfg(feature = "tr")]
    /// The Republic of Turkey (Asia)
    TUR,
    #[cfg(feature = "tt")]
    /// The Republic of Trinidad and Tobago (Americas)
    TTO,
    #[cfg(feature = "tv")]
    /// Tuvalu (Oceania)
    TUV,
    #[cfg(feature = "tw")]
    /// The Republic of China (Asia)
    TWN,
    #[cfg(feature = "tz")]
    /// The United Republic of Tanzania (Africa)
    TZA,
    #[cfg(feature = "ua")]
    /// Ukraine (Europe)
    UKR,
    #[cfg(feature = "ug")]
    /// The Republic of Uganda (Africa)
    UGA,
    #[cfg(feature = "um")]
    /// United States Minor Outlying Islands (Americas)
    UMI,
    #[cfg(feature = "us")]
    /// The United States of America (Americas)
    USA,
    #[cfg(feature = "uy")]
    /// The Oriental Republic of Uruguay (Americas)
    URY,
    #[cfg(feature = "uz")]
    /// The Republic of Uzbekistan (Asia)
    UZB,
    #[cfg(feature = "va")]
    /// The Holy See (Europe)
    VAT,
    #[cfg(feature = "vc")]
    /// Saint Vincent and the Grenadines (Americas)
    VCT,
    #[cfg(feature = "ve")]
    /// The Bolivarian Republic of Venezuela (Americas)
    VEN,
    #[cfg(feature = "vg")]
    /// The Virgin Islands (Americas)
    VGB,
    #[cfg(feature = "vi")]
    /// The Virgin Islands of the United States (Americas)
    VIR,
    #[cfg(feature = "vn")]
    /// The Socialist Republic of Viet Nam (Asia)
    VNM,
    #[cfg(feature = "vu")]
    /// The Republic of Vanuatu (Oceania)
    VUT,
    #[cfg(feature = "wf")]
    /// The Territory of the Wallis and Futuna Islands (Oceania)
    WLF,
    #[cfg(feature = "ws")]
    /// The Independent State of Samoa (Oceania)
    WSM,
    #[cfg(feature = "ye")]
    /// The Republic of Yemen (Asia)
    YEM,
    #[cfg(feature = "yt")]
    /// The Department of Mayotte (Africa)
    MYT,
    #[cfg(feature = "za")]
    /// The Republic of South Africa (Africa)
    ZAF,
    #[cfg(feature = "zm")]
    /// The Republic of Zambia (Africa)
    ZMB,
    #[cfg(feature = "zw")]
    /// The Republic of Zimbabwe (Africa)
    ZWE,
}

impl From<Alpha3> for Country {
    fn from(alpha3: Alpha3) -> Self {
        Self::from(alpha3.to_alpha2())
    }
}

impl Alpha3 {
    pub fn to_country(&self) -> Country {
        Country::from(*self)
    }
}

impl From<Alpha2> for Alpha3 {
    fn from(alpha2: Alpha2) -> Self {
        alpha2.to_alpha3()
    }
}

#[cfg(feature = "subdivisions")]
use crate::Subdivision;
#[cfg(feature = "subdivisions")]
use std::collections::HashMap;
#[cfg(feature = "subdivisions")]
impl Alpha3 {
    pub fn to_subdivisions(&self) -> HashMap<&'static str, Subdivision> {
        self.to_alpha2().to_subdivisions()
    }
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
mod impls {
    use super::{Alpha2, Alpha3};
    use crate::{SearchError, SearchedItems};

    impl TryFrom<&str> for Alpha3 {
        type Error = SearchError;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value.len() != 3 {
                return Err(SearchError::BadInput {
                    expected: "a string with three characters",
                });
            }
            match value.to_uppercase().as_str() {
                #[cfg(feature = "ad")] // The Principality of Andorra (Europe)
                "AND" => Ok(Self::AND),
                #[cfg(feature = "ae")] // The United Arab Emirates (Asia)
                "ARE" => Ok(Self::ARE),
                #[cfg(feature = "af")] // The Islamic Republic of Afghanistan (Asia)
                "AFG" => Ok(Self::AFG),
                #[cfg(feature = "ag")] // Antigua and Barbuda (Americas)
                "ATG" => Ok(Self::ATG),
                #[cfg(feature = "ai")] // Anguilla (Americas)
                "AIA" => Ok(Self::AIA),
                #[cfg(feature = "al")] // The Republic of Albania (Europe)
                "ALB" => Ok(Self::ALB),
                #[cfg(feature = "am")] // The Republic of Armenia (Asia)
                "ARM" => Ok(Self::ARM),
                #[cfg(feature = "ao")] // The Republic of Angola (Africa)
                "AGO" => Ok(Self::AGO),
                #[cfg(feature = "aq")] // Antarctica
                "ATA" => Ok(Self::ATA),
                #[cfg(feature = "ar")] // The Argentine Republic (Americas)
                "ARG" => Ok(Self::ARG),
                #[cfg(feature = "as")] // The Territory of American Samoa (Oceania)
                "ASM" => Ok(Self::ASM),
                #[cfg(feature = "at")] // The Republic of Austria (Europe)
                "AUT" => Ok(Self::AUT),
                #[cfg(feature = "au")] // The Commonwealth of Australia (Oceania)
                "AUS" => Ok(Self::AUS),
                #[cfg(feature = "aw")] // Aruba (Americas)
                "ABW" => Ok(Self::ABW),
                #[cfg(feature = "ax")] // Åland (Europe)
                "ALA" => Ok(Self::ALA),
                #[cfg(feature = "az")] // The Republic of Azerbaijan (Asia)
                "AZE" => Ok(Self::AZE),
                #[cfg(feature = "ba")] // Bosnia and Herzegovina (Europe)
                "BIH" => Ok(Self::BIH),
                #[cfg(feature = "bb")] // Barbados (Americas)
                "BRB" => Ok(Self::BRB),
                #[cfg(feature = "bd")] // The People's Republic of Bangladesh (Asia)
                "BGD" => Ok(Self::BGD),
                #[cfg(feature = "be")] // The Kingdom of Belgium (Europe)
                "BEL" => Ok(Self::BEL),
                #[cfg(feature = "bf")] // Burkina Faso (Africa)
                "BFA" => Ok(Self::BFA),
                #[cfg(feature = "bg")] // The Republic of Bulgaria (Europe)
                "BGR" => Ok(Self::BGR),
                #[cfg(feature = "bh")] // The Kingdom of Bahrain (Asia)
                "BHR" => Ok(Self::BHR),
                #[cfg(feature = "bi")] // The Republic of Burundi (Africa)
                "BDI" => Ok(Self::BDI),
                #[cfg(feature = "bj")] // The Republic of Benin (Africa)
                "BEN" => Ok(Self::BEN),
                #[cfg(feature = "bl")] // The Collectivity of Saint-Barthélemy (Americas)
                "BLM" => Ok(Self::BLM),
                #[cfg(feature = "bm")] // Bermuda (Americas)
                "BMU" => Ok(Self::BMU),
                #[cfg(feature = "bn")] // The Nation of Brunei, the Abode of Peace (Asia)
                "BRN" => Ok(Self::BRN),
                #[cfg(feature = "bo")] // The Plurinational State of Bolivia (Americas)
                "BOL" => Ok(Self::BOL),
                #[cfg(feature = "bq")] // Bonaire, Sint Eustatius and Saba (Americas)
                "BES" => Ok(Self::BES),
                #[cfg(feature = "br")] // The Federative Republic of Brazil (Americas)
                "BRA" => Ok(Self::BRA),
                #[cfg(feature = "bs")] // The Commonwealth of The Bahamas (Americas)
                "BHS" => Ok(Self::BHS),
                #[cfg(feature = "bt")] // The Kingdom of Bhutan (Asia)
                "BTN" => Ok(Self::BTN),
                #[cfg(feature = "bv")] // Bouvet Island
                "BVT" => Ok(Self::BVT),
                #[cfg(feature = "bw")] // The Republic of Botswana (Africa)
                "BWA" => Ok(Self::BWA),
                #[cfg(feature = "by")] // The Republic of Belarus (Europe)
                "BLR" => Ok(Self::BLR),
                #[cfg(feature = "bz")] // Belize (Americas)
                "BLZ" => Ok(Self::BLZ),
                #[cfg(feature = "ca")] // Canada (Americas)
                "CAN" => Ok(Self::CAN),
                #[cfg(feature = "cc")] // The Territory of Cocos (Keeling) Islands (Oceania)
                "CCK" => Ok(Self::CCK),
                #[cfg(feature = "cd")] // The Democratic Republic of the Congo (Africa)
                "COD" => Ok(Self::COD),
                #[cfg(feature = "cf")] // The Central African Republic (Africa)
                "CAF" => Ok(Self::CAF),
                #[cfg(feature = "cg")] // The Republic of the Congo (Africa)
                "COG" => Ok(Self::COG),
                #[cfg(feature = "ch")] // The Swiss Confederation (Europe)
                "CHE" => Ok(Self::CHE),
                #[cfg(feature = "ci")] // The Republic of Côte d'Ivoire (Africa)
                "CIV" => Ok(Self::CIV),
                #[cfg(feature = "ck")] // The Cook Islands (Oceania)
                "COK" => Ok(Self::COK),
                #[cfg(feature = "cl")] // The Republic of Chile (Americas)
                "CHL" => Ok(Self::CHL),
                #[cfg(feature = "cm")] // The Republic of Cameroon (Africa)
                "CMR" => Ok(Self::CMR),
                #[cfg(feature = "cn")] // The People's Republic of China (Asia)
                "CHN" => Ok(Self::CHN),
                #[cfg(feature = "co")] // The Republic of Colombia (Americas)
                "COL" => Ok(Self::COL),
                #[cfg(feature = "cr")] // The Republic of Costa Rica (Americas)
                "CRI" => Ok(Self::CRI),
                #[cfg(feature = "cu")] // The Republic of Cuba (Americas)
                "CUB" => Ok(Self::CUB),
                #[cfg(feature = "cv")] // The Republic of Cabo Verde (Africa)
                "CPV" => Ok(Self::CPV),
                #[cfg(feature = "cw")] // The Country of Curaçao (Americas)
                "CUW" => Ok(Self::CUW),
                #[cfg(feature = "cx")] // The Territory of Christmas Island (Oceania)
                "CXR" => Ok(Self::CXR),
                #[cfg(feature = "cy")] // The Republic of Cyprus (Asia)
                "CYP" => Ok(Self::CYP),
                #[cfg(feature = "cz")] // The Czech Republic (Europe)
                "CZE" => Ok(Self::CZE),
                #[cfg(feature = "de")] // The Federal Republic of Germany (Europe)
                "DEU" => Ok(Self::DEU),
                #[cfg(feature = "dj")] // The Republic of Djibouti (Africa)
                "DJI" => Ok(Self::DJI),
                #[cfg(feature = "dk")] // The Kingdom of Denmark (Europe)
                "DNK" => Ok(Self::DNK),
                #[cfg(feature = "dm")] // The Commonwealth of Dominica (Americas)
                "DMA" => Ok(Self::DMA),
                #[cfg(feature = "do")] // The Dominican Republic (Americas)
                "DOM" => Ok(Self::DOM),
                #[cfg(feature = "dz")] // The People's Democratic Republic of Algeria (Africa)
                "DZA" => Ok(Self::DZA),
                #[cfg(feature = "ec")] // The Republic of Ecuador (Americas)
                "ECU" => Ok(Self::ECU),
                #[cfg(feature = "ee")] // The Republic of Estonia (Europe)
                "EST" => Ok(Self::EST),
                #[cfg(feature = "eg")] // The Arab Republic of Egypt (Africa)
                "EGY" => Ok(Self::EGY),
                #[cfg(feature = "eh")] // The Sahrawi Arab Democratic Republic (Africa)
                "ESH" => Ok(Self::ESH),
                #[cfg(feature = "er")] // The State of Eritrea (Africa)
                "ERI" => Ok(Self::ERI),
                #[cfg(feature = "es")] // The Kingdom of Spain (Europe)
                "ESP" => Ok(Self::ESP),
                #[cfg(feature = "et")] // The Federal Democratic Republic of Ethiopia (Africa)
                "ETH" => Ok(Self::ETH),
                #[cfg(feature = "fi")] // The Republic of Finland (Europe)
                "FIN" => Ok(Self::FIN),
                #[cfg(feature = "fj")] // The Republic of Fiji (Oceania)
                "FJI" => Ok(Self::FJI),
                #[cfg(feature = "fk")] // The Falkland Islands (Americas)
                "FLK" => Ok(Self::FLK),
                #[cfg(feature = "fm")] // The Federated States of Micronesia (Oceania)
                "FSM" => Ok(Self::FSM),
                #[cfg(feature = "fo")] // The Faroe Islands (Europe)
                "FRO" => Ok(Self::FRO),
                #[cfg(feature = "fr")] // The French Republic (Europe)
                "FRA" => Ok(Self::FRA),
                #[cfg(feature = "ga")] // The Gabonese Republic (Africa)
                "GAB" => Ok(Self::GAB),
                #[cfg(feature = "gb")]
                // The United Kingdom of Great Britain and Northern Ireland (Europe)
                "GBR" => Ok(Self::GBR),
                #[cfg(feature = "gd")] // Grenada (Americas)
                "GRD" => Ok(Self::GRD),
                #[cfg(feature = "ge")] // Georgia (Asia)
                "GEO" => Ok(Self::GEO),
                #[cfg(feature = "gf")] // Guyane (Americas)
                "GUF" => Ok(Self::GUF),
                #[cfg(feature = "gg")] // The Bailiwick of Guernsey (Europe)
                "GGY" => Ok(Self::GGY),
                #[cfg(feature = "gh")] // The Republic of Ghana (Africa)
                "GHA" => Ok(Self::GHA),
                #[cfg(feature = "gi")] // Gibraltar (Europe)
                "GIB" => Ok(Self::GIB),
                #[cfg(feature = "gl")] // Kalaallit Nunaat (Americas)
                "GRL" => Ok(Self::GRL),
                #[cfg(feature = "gm")] // The Republic of The Gambia (Africa)
                "GMB" => Ok(Self::GMB),
                #[cfg(feature = "gn")] // The Republic of Guinea (Africa)
                "GIN" => Ok(Self::GIN),
                #[cfg(feature = "gp")] // Guadeloupe (Americas)
                "GLP" => Ok(Self::GLP),
                #[cfg(feature = "gq")] // The Republic of Equatorial Guinea (Africa)
                "GNQ" => Ok(Self::GNQ),
                #[cfg(feature = "gr")] // The Hellenic Republic (Europe)
                "GRC" => Ok(Self::GRC),
                #[cfg(feature = "gs")] // South Georgia and the South Sandwich Islands (Americas)
                "SGS" => Ok(Self::SGS),
                #[cfg(feature = "gt")] // The Republic of Guatemala (Americas)
                "GTM" => Ok(Self::GTM),
                #[cfg(feature = "gu")] // The Territory of Guam (Oceania)
                "GUM" => Ok(Self::GUM),
                #[cfg(feature = "gw")] // The Republic of Guinea-Bissau (Africa)
                "GNB" => Ok(Self::GNB),
                #[cfg(feature = "gy")] // The Co-operative Republic of Guyana (Americas)
                "GUY" => Ok(Self::GUY),
                #[cfg(feature = "hk")]
                // The Hong Kong Special Administrative Region of China (Asia)
                "HKG" => Ok(Self::HKG),
                #[cfg(feature = "hm")] // The Territory of Heard Island and McDonald Islands
                "HMD" => Ok(Self::HMD),
                #[cfg(feature = "hn")] // The Republic of Honduras (Americas)
                "HND" => Ok(Self::HND),
                #[cfg(feature = "hr")] // The Republic of Croatia (Europe)
                "HRV" => Ok(Self::HRV),
                #[cfg(feature = "ht")] // The Republic of Haiti (Americas)
                "HTI" => Ok(Self::HTI),
                #[cfg(feature = "hu")] // Hungary (Europe)
                "HUN" => Ok(Self::HUN),
                #[cfg(feature = "id")] // The Republic of Indonesia (Asia)
                "IDN" => Ok(Self::IDN),
                #[cfg(feature = "ie")] // Ireland (Europe)
                "IRL" => Ok(Self::IRL),
                #[cfg(feature = "il")] // The State of Israel (Asia)
                "ISR" => Ok(Self::ISR),
                #[cfg(feature = "im")] // The Isle of Man (Europe)
                "IMN" => Ok(Self::IMN),
                #[cfg(feature = "in")] // The Republic of India (Asia)
                "IND" => Ok(Self::IND),
                #[cfg(feature = "io")] // The British Indian Ocean Territory (Africa)
                "IOT" => Ok(Self::IOT),
                #[cfg(feature = "iq")] // The Republic of Iraq (Asia)
                "IRQ" => Ok(Self::IRQ),
                #[cfg(feature = "ir")] // The Islamic Republic of Iran (Asia)
                "IRN" => Ok(Self::IRN),
                #[cfg(feature = "is")] // Iceland (Europe)
                "ISL" => Ok(Self::ISL),
                #[cfg(feature = "it")] // The Italian Republic (Europe)
                "ITA" => Ok(Self::ITA),
                #[cfg(feature = "je")] // The Bailiwick of Jersey (Europe)
                "JEY" => Ok(Self::JEY),
                #[cfg(feature = "jm")] // Jamaica (Americas)
                "JAM" => Ok(Self::JAM),
                #[cfg(feature = "jo")] // The Hashemite Kingdom of Jordan (Asia)
                "JOR" => Ok(Self::JOR),
                #[cfg(feature = "jp")] // Japan (Asia)
                "JPN" => Ok(Self::JPN),
                #[cfg(feature = "ke")] // The Republic of Kenya (Africa)
                "KEN" => Ok(Self::KEN),
                #[cfg(feature = "kg")] // The Kyrgyz Republic (Asia)
                "KGZ" => Ok(Self::KGZ),
                #[cfg(feature = "kh")] // The Kingdom of Cambodia (Asia)
                "KHM" => Ok(Self::KHM),
                #[cfg(feature = "ki")] // The Republic of Kiribati (Oceania)
                "KIR" => Ok(Self::KIR),
                #[cfg(feature = "km")] // The Union of the Comoros (Africa)
                "COM" => Ok(Self::COM),
                #[cfg(feature = "kn")] // Saint Kitts and Nevis (Americas)
                "KNA" => Ok(Self::KNA),
                #[cfg(feature = "kp")] // The Democratic People's Republic of Korea (Asia)
                "PRK" => Ok(Self::PRK),
                #[cfg(feature = "kr")] // The Republic of Korea (Asia)
                "KOR" => Ok(Self::KOR),
                #[cfg(feature = "kw")] // The State of Kuwait (Asia)
                "KWT" => Ok(Self::KWT),
                #[cfg(feature = "ky")] // The Cayman Islands (Americas)
                "CYM" => Ok(Self::CYM),
                #[cfg(feature = "kz")] // The Republic of Kazakhstan (Asia)
                "KAZ" => Ok(Self::KAZ),
                #[cfg(feature = "la")] // The Lao People's Democratic Republic (Asia)
                "LAO" => Ok(Self::LAO),
                #[cfg(feature = "lb")] // The Lebanese Republic (Asia)
                "LBN" => Ok(Self::LBN),
                #[cfg(feature = "lc")] // Saint Lucia (Americas)
                "LCA" => Ok(Self::LCA),
                #[cfg(feature = "li")] // The Principality of Liechtenstein (Europe)
                "LIE" => Ok(Self::LIE),
                #[cfg(feature = "lk")] // The Democratic Socialist Republic of Sri Lanka (Asia)
                "LKA" => Ok(Self::LKA),
                #[cfg(feature = "lr")] // The Republic of Liberia (Africa)
                "LBR" => Ok(Self::LBR),
                #[cfg(feature = "ls")] // The Kingdom of Lesotho (Africa)
                "LSO" => Ok(Self::LSO),
                #[cfg(feature = "lt")] // The Republic of Lithuania (Europe)
                "LTU" => Ok(Self::LTU),
                #[cfg(feature = "lu")] // The Grand Duchy of Luxembourg (Europe)
                "LUX" => Ok(Self::LUX),
                #[cfg(feature = "lv")] // The Republic of Latvia (Europe)
                "LVA" => Ok(Self::LVA),
                #[cfg(feature = "ly")] // The State of Libya (Africa)
                "LBY" => Ok(Self::LBY),
                #[cfg(feature = "ma")] // The Kingdom of Morocco (Africa)
                "MAR" => Ok(Self::MAR),
                #[cfg(feature = "mc")] // The Principality of Monaco (Europe)
                "MCO" => Ok(Self::MCO),
                #[cfg(feature = "md")] // The Republic of Moldova (Europe)
                "MDA" => Ok(Self::MDA),
                #[cfg(feature = "me")] // Montenegro (Europe)
                "MNE" => Ok(Self::MNE),
                #[cfg(feature = "mf")] // The Collectivity of Saint-Martin (Americas)
                "MAF" => Ok(Self::MAF),
                #[cfg(feature = "mg")] // The Republic of Madagascar (Africa)
                "MDG" => Ok(Self::MDG),
                #[cfg(feature = "mh")] // The Republic of the Marshall Islands (Oceania)
                "MHL" => Ok(Self::MHL),
                #[cfg(feature = "mk")] // The Republic of North Macedonia (Europe)
                "MKD" => Ok(Self::MKD),
                #[cfg(feature = "ml")] // The Republic of Mali (Africa)
                "MLI" => Ok(Self::MLI),
                #[cfg(feature = "mm")] // The Republic of the Union of Myanmar (Asia)
                "MMR" => Ok(Self::MMR),
                #[cfg(feature = "mn")] // Mongolia (Asia)
                "MNG" => Ok(Self::MNG),
                #[cfg(feature = "mo")] // The Macao Special Administrative Region of China (Asia)
                "MAC" => Ok(Self::MAC),
                #[cfg(feature = "mp")] // The Commonwealth of the Northern Mariana Islands (Oceania)
                "MNP" => Ok(Self::MNP),
                #[cfg(feature = "mq")] // Martinique (Americas)
                "MTQ" => Ok(Self::MTQ),
                #[cfg(feature = "mr")] // The Islamic Republic of Mauritania (Africa)
                "MRT" => Ok(Self::MRT),
                #[cfg(feature = "ms")] // Montserrat (Americas)
                "MSR" => Ok(Self::MSR),
                #[cfg(feature = "mt")] // The Republic of Malta (Europe)
                "MLT" => Ok(Self::MLT),
                #[cfg(feature = "mu")] // The Republic of Mauritius (Africa)
                "MUS" => Ok(Self::MUS),
                #[cfg(feature = "mv")] // The Republic of Maldives (Asia)
                "MDV" => Ok(Self::MDV),
                #[cfg(feature = "mw")] // The Republic of Malawi (Africa)
                "MWI" => Ok(Self::MWI),
                #[cfg(feature = "mx")] // The United Mexican States (Americas)
                "MEX" => Ok(Self::MEX),
                #[cfg(feature = "my")] // Malaysia (Asia)
                "MYS" => Ok(Self::MYS),
                #[cfg(feature = "mz")] // The Republic of Mozambique (Africa)
                "MOZ" => Ok(Self::MOZ),
                #[cfg(feature = "na")] // The Republic of Namibia (Africa)
                "NAM" => Ok(Self::NAM),
                #[cfg(feature = "nc")] // New Caledonia (Oceania)
                "NCL" => Ok(Self::NCL),
                #[cfg(feature = "ne")] // The Republic of the Niger (Africa)
                "NER" => Ok(Self::NER),
                #[cfg(feature = "nf")] // The Territory of Norfolk Island (Oceania)
                "NFK" => Ok(Self::NFK),
                #[cfg(feature = "ng")] // The Federal Republic of Nigeria (Africa)
                "NGA" => Ok(Self::NGA),
                #[cfg(feature = "ni")] // The Republic of Nicaragua (Americas)
                "NIC" => Ok(Self::NIC),
                #[cfg(feature = "nl")] // The Kingdom of the Netherlands (Europe)
                "NLD" => Ok(Self::NLD),
                #[cfg(feature = "no")] // The Kingdom of Norway (Europe)
                "NOR" => Ok(Self::NOR),
                #[cfg(feature = "np")] // The Federal Democratic Republic of Nepal (Asia)
                "NPL" => Ok(Self::NPL),
                #[cfg(feature = "nr")] // The Republic of Nauru (Oceania)
                "NRU" => Ok(Self::NRU),
                #[cfg(feature = "nu")] // Niue (Oceania)
                "NIU" => Ok(Self::NIU),
                #[cfg(feature = "nz")] // New Zealand (Oceania)
                "NZL" => Ok(Self::NZL),
                #[cfg(feature = "om")] // The Sultanate of Oman (Asia)
                "OMN" => Ok(Self::OMN),
                #[cfg(feature = "pa")] // The Republic of Panamá (Americas)
                "PAN" => Ok(Self::PAN),
                #[cfg(feature = "pe")] // The Republic of Perú (Americas)
                "PER" => Ok(Self::PER),
                #[cfg(feature = "pf")] // French Polynesia (Oceania)
                "PYF" => Ok(Self::PYF),
                #[cfg(feature = "pg")] // The Independent State of Papua New Guinea (Oceania)
                "PNG" => Ok(Self::PNG),
                #[cfg(feature = "ph")] // The Republic of the Philippines (Asia)
                "PHL" => Ok(Self::PHL),
                #[cfg(feature = "pk")] // The Islamic Republic of Pakistan (Asia)
                "PAK" => Ok(Self::PAK),
                #[cfg(feature = "pl")] // The Republic of Poland (Europe)
                "POL" => Ok(Self::POL),
                #[cfg(feature = "pm")]
                // The Overseas Collectivity of Saint-Pierre and Miquelon (Americas)
                "SPM" => Ok(Self::SPM),
                #[cfg(feature = "pn")] // The Pitcairn, Henderson, Ducie and Oeno Islands (Oceania)
                "PCN" => Ok(Self::PCN),
                #[cfg(feature = "pr")] // The Commonwealth of Puerto Rico (Americas)
                "PRI" => Ok(Self::PRI),
                #[cfg(feature = "ps")] // The State of Palestine (Asia)
                "PSE" => Ok(Self::PSE),
                #[cfg(feature = "pt")] // The Portuguese Republic (Europe)
                "PRT" => Ok(Self::PRT),
                #[cfg(feature = "pw")] // The Republic of Palau (Oceania)
                "PLW" => Ok(Self::PLW),
                #[cfg(feature = "py")] // The Republic of Paraguay (Americas)
                "PRY" => Ok(Self::PRY),
                #[cfg(feature = "qa")] // The State of Qatar (Asia)
                "QAT" => Ok(Self::QAT),
                #[cfg(feature = "re")] // Réunion (Africa)
                "REU" => Ok(Self::REU),
                #[cfg(feature = "ro")] // Romania (Europe)
                "ROU" => Ok(Self::ROU),
                #[cfg(feature = "rs")] // The Republic of Serbia (Europe)
                "SRB" => Ok(Self::SRB),
                #[cfg(feature = "ru")] // The Russian Federation (Europe)
                "RUS" => Ok(Self::RUS),
                #[cfg(feature = "rw")] // The Republic of Rwanda (Africa)
                "RWA" => Ok(Self::RWA),
                #[cfg(feature = "sa")] // The Kingdom of Saudi Arabia (Asia)
                "SAU" => Ok(Self::SAU),
                #[cfg(feature = "sb")] // The Solomon Islands (Oceania)
                "SLB" => Ok(Self::SLB),
                #[cfg(feature = "sc")] // The Republic of Seychelles (Africa)
                "SYC" => Ok(Self::SYC),
                #[cfg(feature = "sd")] // The Republic of the Sudan (Africa)
                "SDN" => Ok(Self::SDN),
                #[cfg(feature = "se")] // The Kingdom of Sweden (Europe)
                "SWE" => Ok(Self::SWE),
                #[cfg(feature = "sg")] // The Republic of Singapore (Asia)
                "SGP" => Ok(Self::SGP),
                #[cfg(feature = "sh")] // Saint Helena, Ascension and Tristan da Cunha (Africa)
                "SHN" => Ok(Self::SHN),
                #[cfg(feature = "si")] // The Republic of Slovenia (Europe)
                "SVN" => Ok(Self::SVN),
                #[cfg(feature = "sj")] // Svalbard and Jan Mayen (Europe)
                "SJM" => Ok(Self::SJM),
                #[cfg(feature = "sk")] // The Slovak Republic (Europe)
                "SVK" => Ok(Self::SVK),
                #[cfg(feature = "sl")] // The Republic of Sierra Leone (Africa)
                "SLE" => Ok(Self::SLE),
                #[cfg(feature = "sm")] // The Republic of San Marino (Europe)
                "SMR" => Ok(Self::SMR),
                #[cfg(feature = "sn")] // The Republic of Senegal (Africa)
                "SEN" => Ok(Self::SEN),
                #[cfg(feature = "so")] // The Federal Republic of Somalia (Africa)
                "SOM" => Ok(Self::SOM),
                #[cfg(feature = "sr")] // The Republic of Suriname (Americas)
                "SUR" => Ok(Self::SUR),
                #[cfg(feature = "ss")] // The Republic of South Sudan (Africa)
                "SSD" => Ok(Self::SSD),
                #[cfg(feature = "st")] // The Democratic Republic of São Tomé and Príncipe (Africa)
                "STP" => Ok(Self::STP),
                #[cfg(feature = "sv")] // The Republic of El Salvador (Americas)
                "SLV" => Ok(Self::SLV),
                #[cfg(feature = "sx")] // Sint Maarten (Americas)
                "SXM" => Ok(Self::SXM),
                #[cfg(feature = "sy")] // The Syrian Arab Republic (Asia)
                "SYR" => Ok(Self::SYR),
                #[cfg(feature = "sz")] // The Kingdom of Eswatini (Africa)
                "SWZ" => Ok(Self::SWZ),
                #[cfg(feature = "tc")] // The Turks and Caicos Islands (Americas)
                "TCA" => Ok(Self::TCA),
                #[cfg(feature = "td")] // The Republic of Chad (Africa)
                "TCD" => Ok(Self::TCD),
                #[cfg(feature = "tf")] // The French Southern and Antarctic Lands (Africa)
                "ATF" => Ok(Self::ATF),
                #[cfg(feature = "tg")] // The Togolese Republic (Africa)
                "TGO" => Ok(Self::TGO),
                #[cfg(feature = "th")] // The Kingdom of Thailand (Asia)
                "THA" => Ok(Self::THA),
                #[cfg(feature = "tj")] // The Republic of Tajikistan (Asia)
                "TJK" => Ok(Self::TJK),
                #[cfg(feature = "tk")] // Tokelau (Oceania)
                "TKL" => Ok(Self::TKL),
                #[cfg(feature = "tl")] // The Democratic Republic of Timor-Leste (Asia)
                "TLS" => Ok(Self::TLS),
                #[cfg(feature = "tm")] // Turkmenistan (Asia)
                "TKM" => Ok(Self::TKM),
                #[cfg(feature = "tn")] // The Republic of Tunisia (Africa)
                "TUN" => Ok(Self::TUN),
                #[cfg(feature = "to")] // The Kingdom of Tonga (Oceania)
                "TON" => Ok(Self::TON),
                #[cfg(feature = "tr")] // The Republic of Turkey (Asia)
                "TUR" => Ok(Self::TUR),
                #[cfg(feature = "tt")] // The Republic of Trinidad and Tobago (Americas)
                "TTO" => Ok(Self::TTO),
                #[cfg(feature = "tv")] // Tuvalu (Oceania)
                "TUV" => Ok(Self::TUV),
                #[cfg(feature = "tw")] // The Republic of China (Asia)
                "TWN" => Ok(Self::TWN),
                #[cfg(feature = "tz")] // The United Republic of Tanzania (Africa)
                "TZA" => Ok(Self::TZA),
                #[cfg(feature = "ua")] // Ukraine (Europe)
                "UKR" => Ok(Self::UKR),
                #[cfg(feature = "ug")] // The Republic of Uganda (Africa)
                "UGA" => Ok(Self::UGA),
                #[cfg(feature = "um")] // United States Minor Outlying Islands (Americas)
                "UMI" => Ok(Self::UMI),
                #[cfg(feature = "us")] // The United States of America (Americas)
                "USA" => Ok(Self::USA),
                #[cfg(feature = "uy")] // The Oriental Republic of Uruguay (Americas)
                "URY" => Ok(Self::URY),
                #[cfg(feature = "uz")] // The Republic of Uzbekistan (Asia)
                "UZB" => Ok(Self::UZB),
                #[cfg(feature = "va")] // The Holy See (Europe)
                "VAT" => Ok(Self::VAT),
                #[cfg(feature = "vc")] // Saint Vincent and the Grenadines (Americas)
                "VCT" => Ok(Self::VCT),
                #[cfg(feature = "ve")] // The Bolivarian Republic of Venezuela (Americas)
                "VEN" => Ok(Self::VEN),
                #[cfg(feature = "vg")] // The Virgin Islands (Americas)
                "VGB" => Ok(Self::VGB),
                #[cfg(feature = "vi")] // The Virgin Islands of the United States (Americas)
                "VIR" => Ok(Self::VIR),
                #[cfg(feature = "vn")] // The Socialist Republic of Viet Nam (Asia)
                "VNM" => Ok(Self::VNM),
                #[cfg(feature = "vu")] // The Republic of Vanuatu (Oceania)
                "VUT" => Ok(Self::VUT),
                #[cfg(feature = "wf")] // The Territory of the Wallis and Futuna Islands (Oceania)
                "WLF" => Ok(Self::WLF),
                #[cfg(feature = "ws")] // The Independent State of Samoa (Oceania)
                "WSM" => Ok(Self::WSM),
                #[cfg(feature = "ye")] // The Republic of Yemen (Asia)
                "YEM" => Ok(Self::YEM),
                #[cfg(feature = "yt")] // The Department of Mayotte (Africa)
                "MYT" => Ok(Self::MYT),
                #[cfg(feature = "za")] // The Republic of South Africa (Africa)
                "ZAF" => Ok(Self::ZAF),
                #[cfg(feature = "zm")] // The Republic of Zambia (Africa)
                "ZMB" => Ok(Self::ZMB),
                #[cfg(feature = "zw")] // The Republic of Zimbabwe (Africa)
                "ZWE" => Ok(Self::ZWE),
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

    impl ToString for Alpha3 {
        fn to_string(&self) -> String {
            match self {
                #[cfg(feature = "ad")] // The Principality of Andorra (Europe)
                Alpha3::AND => "AND",
                #[cfg(feature = "ae")] // The United Arab Emirates (Asia)
                Alpha3::ARE => "ARE",
                #[cfg(feature = "af")] // The Islamic Republic of Afghanistan (Asia)
                Alpha3::AFG => "AFG",
                #[cfg(feature = "ag")] // Antigua and Barbuda (Americas)
                Alpha3::ATG => "ATG",
                #[cfg(feature = "ai")] // Anguilla (Americas)
                Alpha3::AIA => "AIA",
                #[cfg(feature = "al")] // The Republic of Albania (Europe)
                Alpha3::ALB => "ALB",
                #[cfg(feature = "am")] // The Republic of Armenia (Asia)
                Alpha3::ARM => "ARM",
                #[cfg(feature = "ao")] // The Republic of Angola (Africa)
                Alpha3::AGO => "AGO",
                #[cfg(feature = "aq")] // Antarctica
                Alpha3::ATA => "ATA",
                #[cfg(feature = "ar")] // The Argentine Republic (Americas)
                Alpha3::ARG => "ARG",
                #[cfg(feature = "as")] // The Territory of American Samoa (Oceania)
                Alpha3::ASM => "ASM",
                #[cfg(feature = "at")] // The Republic of Austria (Europe)
                Alpha3::AUT => "AUT",
                #[cfg(feature = "au")] // The Commonwealth of Australia (Oceania)
                Alpha3::AUS => "AUS",
                #[cfg(feature = "aw")] // Aruba (Americas)
                Alpha3::ABW => "ABW",
                #[cfg(feature = "ax")] // Åland (Europe)
                Alpha3::ALA => "ALA",
                #[cfg(feature = "az")] // The Republic of Azerbaijan (Asia)
                Alpha3::AZE => "AZE",
                #[cfg(feature = "ba")] // Bosnia and Herzegovina (Europe)
                Alpha3::BIH => "BIH",
                #[cfg(feature = "bb")] // Barbados (Americas)
                Alpha3::BRB => "BRB",
                #[cfg(feature = "bd")] // The People's Republic of Bangladesh (Asia)
                Alpha3::BGD => "BGD",
                #[cfg(feature = "be")] // The Kingdom of Belgium (Europe)
                Alpha3::BEL => "BEL",
                #[cfg(feature = "bf")] // Burkina Faso (Africa)
                Alpha3::BFA => "BFA",
                #[cfg(feature = "bg")] // The Republic of Bulgaria (Europe)
                Alpha3::BGR => "BGR",
                #[cfg(feature = "bh")] // The Kingdom of Bahrain (Asia)
                Alpha3::BHR => "BHR",
                #[cfg(feature = "bi")] // The Republic of Burundi (Africa)
                Alpha3::BDI => "BDI",
                #[cfg(feature = "bj")] // The Republic of Benin (Africa)
                Alpha3::BEN => "BEN",
                #[cfg(feature = "bl")] // The Collectivity of Saint-Barthélemy (Americas)
                Alpha3::BLM => "BLM",
                #[cfg(feature = "bm")] // Bermuda (Americas)
                Alpha3::BMU => "BMU",
                #[cfg(feature = "bn")] // The Nation of Brunei, the Abode of Peace (Asia)
                Alpha3::BRN => "BRN",
                #[cfg(feature = "bo")] // The Plurinational State of Bolivia (Americas)
                Alpha3::BOL => "BOL",
                #[cfg(feature = "bq")] // Bonaire, Sint Eustatius and Saba (Americas)
                Alpha3::BES => "BES",
                #[cfg(feature = "br")] // The Federative Republic of Brazil (Americas)
                Alpha3::BRA => "BRA",
                #[cfg(feature = "bs")] // The Commonwealth of The Bahamas (Americas)
                Alpha3::BHS => "BHS",
                #[cfg(feature = "bt")] // The Kingdom of Bhutan (Asia)
                Alpha3::BTN => "BTN",
                #[cfg(feature = "bv")] // Bouvet Island
                Alpha3::BVT => "BVT",
                #[cfg(feature = "bw")] // The Republic of Botswana (Africa)
                Alpha3::BWA => "BWA",
                #[cfg(feature = "by")] // The Republic of Belarus (Europe)
                Alpha3::BLR => "BLR",
                #[cfg(feature = "bz")] // Belize (Americas)
                Alpha3::BLZ => "BLZ",
                #[cfg(feature = "ca")] // Canada (Americas)
                Alpha3::CAN => "CAN",
                #[cfg(feature = "cc")] // The Territory of Cocos (Keeling) Islands (Oceania)
                Alpha3::CCK => "CCK",
                #[cfg(feature = "cd")] // The Democratic Republic of the Congo (Africa)
                Alpha3::COD => "COD",
                #[cfg(feature = "cf")] // The Central African Republic (Africa)
                Alpha3::CAF => "CAF",
                #[cfg(feature = "cg")] // The Republic of the Congo (Africa)
                Alpha3::COG => "COG",
                #[cfg(feature = "ch")] // The Swiss Confederation (Europe)
                Alpha3::CHE => "CHE",
                #[cfg(feature = "ci")] // The Republic of Côte d'Ivoire (Africa)
                Alpha3::CIV => "CIV",
                #[cfg(feature = "ck")] // The Cook Islands (Oceania)
                Alpha3::COK => "COK",
                #[cfg(feature = "cl")] // The Republic of Chile (Americas)
                Alpha3::CHL => "CHL",
                #[cfg(feature = "cm")] // The Republic of Cameroon (Africa)
                Alpha3::CMR => "CMR",
                #[cfg(feature = "cn")] // The People's Republic of China (Asia)
                Alpha3::CHN => "CHN",
                #[cfg(feature = "co")] // The Republic of Colombia (Americas)
                Alpha3::COL => "COL",
                #[cfg(feature = "cr")] // The Republic of Costa Rica (Americas)
                Alpha3::CRI => "CRI",
                #[cfg(feature = "cu")] // The Republic of Cuba (Americas)
                Alpha3::CUB => "CUB",
                #[cfg(feature = "cv")] // The Republic of Cabo Verde (Africa)
                Alpha3::CPV => "CPV",
                #[cfg(feature = "cw")] // The Country of Curaçao (Americas)
                Alpha3::CUW => "CUW",
                #[cfg(feature = "cx")] // The Territory of Christmas Island (Oceania)
                Alpha3::CXR => "CXR",
                #[cfg(feature = "cy")] // The Republic of Cyprus (Asia)
                Alpha3::CYP => "CYP",
                #[cfg(feature = "cz")] // The Czech Republic (Europe)
                Alpha3::CZE => "CZE",
                #[cfg(feature = "de")] // The Federal Republic of Germany (Europe)
                Alpha3::DEU => "DEU",
                #[cfg(feature = "dj")] // The Republic of Djibouti (Africa)
                Alpha3::DJI => "DJI",
                #[cfg(feature = "dk")] // The Kingdom of Denmark (Europe)
                Alpha3::DNK => "DNK",
                #[cfg(feature = "dm")] // The Commonwealth of Dominica (Americas)
                Alpha3::DMA => "DMA",
                #[cfg(feature = "do")] // The Dominican Republic (Americas)
                Alpha3::DOM => "DOM",
                #[cfg(feature = "dz")] // The People's Democratic Republic of Algeria (Africa)
                Alpha3::DZA => "DZA",
                #[cfg(feature = "ec")] // The Republic of Ecuador (Americas)
                Alpha3::ECU => "ECU",
                #[cfg(feature = "ee")] // The Republic of Estonia (Europe)
                Alpha3::EST => "EST",
                #[cfg(feature = "eg")] // The Arab Republic of Egypt (Africa)
                Alpha3::EGY => "EGY",
                #[cfg(feature = "eh")] // The Sahrawi Arab Democratic Republic (Africa)
                Alpha3::ESH => "ESH",
                #[cfg(feature = "er")] // The State of Eritrea (Africa)
                Alpha3::ERI => "ERI",
                #[cfg(feature = "es")] // The Kingdom of Spain (Europe)
                Alpha3::ESP => "ESP",
                #[cfg(feature = "et")] // The Federal Democratic Republic of Ethiopia (Africa)
                Alpha3::ETH => "ETH",
                #[cfg(feature = "fi")] // The Republic of Finland (Europe)
                Alpha3::FIN => "FIN",
                #[cfg(feature = "fj")] // The Republic of Fiji (Oceania)
                Alpha3::FJI => "FJI",
                #[cfg(feature = "fk")] // The Falkland Islands (Americas)
                Alpha3::FLK => "FLK",
                #[cfg(feature = "fm")] // The Federated States of Micronesia (Oceania)
                Alpha3::FSM => "FSM",
                #[cfg(feature = "fo")] // The Faroe Islands (Europe)
                Alpha3::FRO => "FRO",
                #[cfg(feature = "fr")] // The French Republic (Europe)
                Alpha3::FRA => "FRA",
                #[cfg(feature = "ga")] // The Gabonese Republic (Africa)
                Alpha3::GAB => "GAB",
                #[cfg(feature = "gb")]
                // The United Kingdom of Great Britain and Northern Ireland (Europe)
                Alpha3::GBR => "GBR",
                #[cfg(feature = "gd")] // Grenada (Americas)
                Alpha3::GRD => "GRD",
                #[cfg(feature = "ge")] // Georgia (Asia)
                Alpha3::GEO => "GEO",
                #[cfg(feature = "gf")] // Guyane (Americas)
                Alpha3::GUF => "GUF",
                #[cfg(feature = "gg")] // The Bailiwick of Guernsey (Europe)
                Alpha3::GGY => "GGY",
                #[cfg(feature = "gh")] // The Republic of Ghana (Africa)
                Alpha3::GHA => "GHA",
                #[cfg(feature = "gi")] // Gibraltar (Europe)
                Alpha3::GIB => "GIB",
                #[cfg(feature = "gl")] // Kalaallit Nunaat (Americas)
                Alpha3::GRL => "GRL",
                #[cfg(feature = "gm")] // The Republic of The Gambia (Africa)
                Alpha3::GMB => "GMB",
                #[cfg(feature = "gn")] // The Republic of Guinea (Africa)
                Alpha3::GIN => "GIN",
                #[cfg(feature = "gp")] // Guadeloupe (Americas)
                Alpha3::GLP => "GLP",
                #[cfg(feature = "gq")] // The Republic of Equatorial Guinea (Africa)
                Alpha3::GNQ => "GNQ",
                #[cfg(feature = "gr")] // The Hellenic Republic (Europe)
                Alpha3::GRC => "GRC",
                #[cfg(feature = "gs")] // South Georgia and the South Sandwich Islands (Americas)
                Alpha3::SGS => "SGS",
                #[cfg(feature = "gt")] // The Republic of Guatemala (Americas)
                Alpha3::GTM => "GTM",
                #[cfg(feature = "gu")] // The Territory of Guam (Oceania)
                Alpha3::GUM => "GUM",
                #[cfg(feature = "gw")] // The Republic of Guinea-Bissau (Africa)
                Alpha3::GNB => "GNB",
                #[cfg(feature = "gy")] // The Co-operative Republic of Guyana (Americas)
                Alpha3::GUY => "GUY",
                #[cfg(feature = "hk")]
                // The Hong Kong Special Administrative Region of China (Asia)
                Alpha3::HKG => "HKG",
                #[cfg(feature = "hm")] // The Territory of Heard Island and McDonald Islands
                Alpha3::HMD => "HMD",
                #[cfg(feature = "hn")] // The Republic of Honduras (Americas)
                Alpha3::HND => "HND",
                #[cfg(feature = "hr")] // The Republic of Croatia (Europe)
                Alpha3::HRV => "HRV",
                #[cfg(feature = "ht")] // The Republic of Haiti (Americas)
                Alpha3::HTI => "HTI",
                #[cfg(feature = "hu")] // Hungary (Europe)
                Alpha3::HUN => "HUN",
                #[cfg(feature = "id")] // The Republic of Indonesia (Asia)
                Alpha3::IDN => "IDN",
                #[cfg(feature = "ie")] // Ireland (Europe)
                Alpha3::IRL => "IRL",
                #[cfg(feature = "il")] // The State of Israel (Asia)
                Alpha3::ISR => "ISR",
                #[cfg(feature = "im")] // The Isle of Man (Europe)
                Alpha3::IMN => "IMN",
                #[cfg(feature = "in")] // The Republic of India (Asia)
                Alpha3::IND => "IND",
                #[cfg(feature = "io")] // The British Indian Ocean Territory (Africa)
                Alpha3::IOT => "IOT",
                #[cfg(feature = "iq")] // The Republic of Iraq (Asia)
                Alpha3::IRQ => "IRQ",
                #[cfg(feature = "ir")] // The Islamic Republic of Iran (Asia)
                Alpha3::IRN => "IRN",
                #[cfg(feature = "is")] // Iceland (Europe)
                Alpha3::ISL => "ISL",
                #[cfg(feature = "it")] // The Italian Republic (Europe)
                Alpha3::ITA => "ITA",
                #[cfg(feature = "je")] // The Bailiwick of Jersey (Europe)
                Alpha3::JEY => "JEY",
                #[cfg(feature = "jm")] // Jamaica (Americas)
                Alpha3::JAM => "JAM",
                #[cfg(feature = "jo")] // The Hashemite Kingdom of Jordan (Asia)
                Alpha3::JOR => "JOR",
                #[cfg(feature = "jp")] // Japan (Asia)
                Alpha3::JPN => "JPN",
                #[cfg(feature = "ke")] // The Republic of Kenya (Africa)
                Alpha3::KEN => "KEN",
                #[cfg(feature = "kg")] // The Kyrgyz Republic (Asia)
                Alpha3::KGZ => "KGZ",
                #[cfg(feature = "kh")] // The Kingdom of Cambodia (Asia)
                Alpha3::KHM => "KHM",
                #[cfg(feature = "ki")] // The Republic of Kiribati (Oceania)
                Alpha3::KIR => "KIR",
                #[cfg(feature = "km")] // The Union of the Comoros (Africa)
                Alpha3::COM => "COM",
                #[cfg(feature = "kn")] // Saint Kitts and Nevis (Americas)
                Alpha3::KNA => "KNA",
                #[cfg(feature = "kp")] // The Democratic People's Republic of Korea (Asia)
                Alpha3::PRK => "PRK",
                #[cfg(feature = "kr")] // The Republic of Korea (Asia)
                Alpha3::KOR => "KOR",
                #[cfg(feature = "kw")] // The State of Kuwait (Asia)
                Alpha3::KWT => "KWT",
                #[cfg(feature = "ky")] // The Cayman Islands (Americas)
                Alpha3::CYM => "CYM",
                #[cfg(feature = "kz")] // The Republic of Kazakhstan (Asia)
                Alpha3::KAZ => "KAZ",
                #[cfg(feature = "la")] // The Lao People's Democratic Republic (Asia)
                Alpha3::LAO => "LAO",
                #[cfg(feature = "lb")] // The Lebanese Republic (Asia)
                Alpha3::LBN => "LBN",
                #[cfg(feature = "lc")] // Saint Lucia (Americas)
                Alpha3::LCA => "LCA",
                #[cfg(feature = "li")] // The Principality of Liechtenstein (Europe)
                Alpha3::LIE => "LIE",
                #[cfg(feature = "lk")] // The Democratic Socialist Republic of Sri Lanka (Asia)
                Alpha3::LKA => "LKA",
                #[cfg(feature = "lr")] // The Republic of Liberia (Africa)
                Alpha3::LBR => "LBR",
                #[cfg(feature = "ls")] // The Kingdom of Lesotho (Africa)
                Alpha3::LSO => "LSO",
                #[cfg(feature = "lt")] // The Republic of Lithuania (Europe)
                Alpha3::LTU => "LTU",
                #[cfg(feature = "lu")] // The Grand Duchy of Luxembourg (Europe)
                Alpha3::LUX => "LUX",
                #[cfg(feature = "lv")] // The Republic of Latvia (Europe)
                Alpha3::LVA => "LVA",
                #[cfg(feature = "ly")] // The State of Libya (Africa)
                Alpha3::LBY => "LBY",
                #[cfg(feature = "ma")] // The Kingdom of Morocco (Africa)
                Alpha3::MAR => "MAR",
                #[cfg(feature = "mc")] // The Principality of Monaco (Europe)
                Alpha3::MCO => "MCO",
                #[cfg(feature = "md")] // The Republic of Moldova (Europe)
                Alpha3::MDA => "MDA",
                #[cfg(feature = "me")] // Montenegro (Europe)
                Alpha3::MNE => "MNE",
                #[cfg(feature = "mf")] // The Collectivity of Saint-Martin (Americas)
                Alpha3::MAF => "MAF",
                #[cfg(feature = "mg")] // The Republic of Madagascar (Africa)
                Alpha3::MDG => "MDG",
                #[cfg(feature = "mh")] // The Republic of the Marshall Islands (Oceania)
                Alpha3::MHL => "MHL",
                #[cfg(feature = "mk")] // The Republic of North Macedonia (Europe)
                Alpha3::MKD => "MKD",
                #[cfg(feature = "ml")] // The Republic of Mali (Africa)
                Alpha3::MLI => "MLI",
                #[cfg(feature = "mm")] // The Republic of the Union of Myanmar (Asia)
                Alpha3::MMR => "MMR",
                #[cfg(feature = "mn")] // Mongolia (Asia)
                Alpha3::MNG => "MNG",
                #[cfg(feature = "mo")] // The Macao Special Administrative Region of China (Asia)
                Alpha3::MAC => "MAC",
                #[cfg(feature = "mp")] // The Commonwealth of the Northern Mariana Islands (Oceania)
                Alpha3::MNP => "MNP",
                #[cfg(feature = "mq")] // Martinique (Americas)
                Alpha3::MTQ => "MTQ",
                #[cfg(feature = "mr")] // The Islamic Republic of Mauritania (Africa)
                Alpha3::MRT => "MRT",
                #[cfg(feature = "ms")] // Montserrat (Americas)
                Alpha3::MSR => "MSR",
                #[cfg(feature = "mt")] // The Republic of Malta (Europe)
                Alpha3::MLT => "MLT",
                #[cfg(feature = "mu")] // The Republic of Mauritius (Africa)
                Alpha3::MUS => "MUS",
                #[cfg(feature = "mv")] // The Republic of Maldives (Asia)
                Alpha3::MDV => "MDV",
                #[cfg(feature = "mw")] // The Republic of Malawi (Africa)
                Alpha3::MWI => "MWI",
                #[cfg(feature = "mx")] // The United Mexican States (Americas)
                Alpha3::MEX => "MEX",
                #[cfg(feature = "my")] // Malaysia (Asia)
                Alpha3::MYS => "MYS",
                #[cfg(feature = "mz")] // The Republic of Mozambique (Africa)
                Alpha3::MOZ => "MOZ",
                #[cfg(feature = "na")] // The Republic of Namibia (Africa)
                Alpha3::NAM => "NAM",
                #[cfg(feature = "nc")] // New Caledonia (Oceania)
                Alpha3::NCL => "NCL",
                #[cfg(feature = "ne")] // The Republic of the Niger (Africa)
                Alpha3::NER => "NER",
                #[cfg(feature = "nf")] // The Territory of Norfolk Island (Oceania)
                Alpha3::NFK => "NFK",
                #[cfg(feature = "ng")] // The Federal Republic of Nigeria (Africa)
                Alpha3::NGA => "NGA",
                #[cfg(feature = "ni")] // The Republic of Nicaragua (Americas)
                Alpha3::NIC => "NIC",
                #[cfg(feature = "nl")] // The Kingdom of the Netherlands (Europe)
                Alpha3::NLD => "NLD",
                #[cfg(feature = "no")] // The Kingdom of Norway (Europe)
                Alpha3::NOR => "NOR",
                #[cfg(feature = "np")] // The Federal Democratic Republic of Nepal (Asia)
                Alpha3::NPL => "NPL",
                #[cfg(feature = "nr")] // The Republic of Nauru (Oceania)
                Alpha3::NRU => "NRU",
                #[cfg(feature = "nu")] // Niue (Oceania)
                Alpha3::NIU => "NIU",
                #[cfg(feature = "nz")] // New Zealand (Oceania)
                Alpha3::NZL => "NZL",
                #[cfg(feature = "om")] // The Sultanate of Oman (Asia)
                Alpha3::OMN => "OMN",
                #[cfg(feature = "pa")] // The Republic of Panamá (Americas)
                Alpha3::PAN => "PAN",
                #[cfg(feature = "pe")] // The Republic of Perú (Americas)
                Alpha3::PER => "PER",
                #[cfg(feature = "pf")] // French Polynesia (Oceania)
                Alpha3::PYF => "PYF",
                #[cfg(feature = "pg")] // The Independent State of Papua New Guinea (Oceania)
                Alpha3::PNG => "PNG",
                #[cfg(feature = "ph")] // The Republic of the Philippines (Asia)
                Alpha3::PHL => "PHL",
                #[cfg(feature = "pk")] // The Islamic Republic of Pakistan (Asia)
                Alpha3::PAK => "PAK",
                #[cfg(feature = "pl")] // The Republic of Poland (Europe)
                Alpha3::POL => "POL",
                #[cfg(feature = "pm")]
                // The Overseas Collectivity of Saint-Pierre and Miquelon (Americas)
                Alpha3::SPM => "SPM",
                #[cfg(feature = "pn")] // The Pitcairn, Henderson, Ducie and Oeno Islands (Oceania)
                Alpha3::PCN => "PCN",
                #[cfg(feature = "pr")] // The Commonwealth of Puerto Rico (Americas)
                Alpha3::PRI => "PRI",
                #[cfg(feature = "ps")] // The State of Palestine (Asia)
                Alpha3::PSE => "PSE",
                #[cfg(feature = "pt")] // The Portuguese Republic (Europe)
                Alpha3::PRT => "PRT",
                #[cfg(feature = "pw")] // The Republic of Palau (Oceania)
                Alpha3::PLW => "PLW",
                #[cfg(feature = "py")] // The Republic of Paraguay (Americas)
                Alpha3::PRY => "PRY",
                #[cfg(feature = "qa")] // The State of Qatar (Asia)
                Alpha3::QAT => "QAT",
                #[cfg(feature = "re")] // Réunion (Africa)
                Alpha3::REU => "REU",
                #[cfg(feature = "ro")] // Romania (Europe)
                Alpha3::ROU => "ROU",
                #[cfg(feature = "rs")] // The Republic of Serbia (Europe)
                Alpha3::SRB => "SRB",
                #[cfg(feature = "ru")] // The Russian Federation (Europe)
                Alpha3::RUS => "RUS",
                #[cfg(feature = "rw")] // The Republic of Rwanda (Africa)
                Alpha3::RWA => "RWA",
                #[cfg(feature = "sa")] // The Kingdom of Saudi Arabia (Asia)
                Alpha3::SAU => "SAU",
                #[cfg(feature = "sb")] // The Solomon Islands (Oceania)
                Alpha3::SLB => "SLB",
                #[cfg(feature = "sc")] // The Republic of Seychelles (Africa)
                Alpha3::SYC => "SYC",
                #[cfg(feature = "sd")] // The Republic of the Sudan (Africa)
                Alpha3::SDN => "SDN",
                #[cfg(feature = "se")] // The Kingdom of Sweden (Europe)
                Alpha3::SWE => "SWE",
                #[cfg(feature = "sg")] // The Republic of Singapore (Asia)
                Alpha3::SGP => "SGP",
                #[cfg(feature = "sh")] // Saint Helena, Ascension and Tristan da Cunha (Africa)
                Alpha3::SHN => "SHN",
                #[cfg(feature = "si")] // The Republic of Slovenia (Europe)
                Alpha3::SVN => "SVN",
                #[cfg(feature = "sj")] // Svalbard and Jan Mayen (Europe)
                Alpha3::SJM => "SJM",
                #[cfg(feature = "sk")] // The Slovak Republic (Europe)
                Alpha3::SVK => "SVK",
                #[cfg(feature = "sl")] // The Republic of Sierra Leone (Africa)
                Alpha3::SLE => "SLE",
                #[cfg(feature = "sm")] // The Republic of San Marino (Europe)
                Alpha3::SMR => "SMR",
                #[cfg(feature = "sn")] // The Republic of Senegal (Africa)
                Alpha3::SEN => "SEN",
                #[cfg(feature = "so")] // The Federal Republic of Somalia (Africa)
                Alpha3::SOM => "SOM",
                #[cfg(feature = "sr")] // The Republic of Suriname (Americas)
                Alpha3::SUR => "SUR",
                #[cfg(feature = "ss")] // The Republic of South Sudan (Africa)
                Alpha3::SSD => "SSD",
                #[cfg(feature = "st")] // The Democratic Republic of São Tomé and Príncipe (Africa)
                Alpha3::STP => "STP",
                #[cfg(feature = "sv")] // The Republic of El Salvador (Americas)
                Alpha3::SLV => "SLV",
                #[cfg(feature = "sx")] // Sint Maarten (Americas)
                Alpha3::SXM => "SXM",
                #[cfg(feature = "sy")] // The Syrian Arab Republic (Asia)
                Alpha3::SYR => "SYR",
                #[cfg(feature = "sz")] // The Kingdom of Eswatini (Africa)
                Alpha3::SWZ => "SWZ",
                #[cfg(feature = "tc")] // The Turks and Caicos Islands (Americas)
                Alpha3::TCA => "TCA",
                #[cfg(feature = "td")] // The Republic of Chad (Africa)
                Alpha3::TCD => "TCD",
                #[cfg(feature = "tf")] // The French Southern and Antarctic Lands (Africa)
                Alpha3::ATF => "ATF",
                #[cfg(feature = "tg")] // The Togolese Republic (Africa)
                Alpha3::TGO => "TGO",
                #[cfg(feature = "th")] // The Kingdom of Thailand (Asia)
                Alpha3::THA => "THA",
                #[cfg(feature = "tj")] // The Republic of Tajikistan (Asia)
                Alpha3::TJK => "TJK",
                #[cfg(feature = "tk")] // Tokelau (Oceania)
                Alpha3::TKL => "TKL",
                #[cfg(feature = "tl")] // The Democratic Republic of Timor-Leste (Asia)
                Alpha3::TLS => "TLS",
                #[cfg(feature = "tm")] // Turkmenistan (Asia)
                Alpha3::TKM => "TKM",
                #[cfg(feature = "tn")] // The Republic of Tunisia (Africa)
                Alpha3::TUN => "TUN",
                #[cfg(feature = "to")] // The Kingdom of Tonga (Oceania)
                Alpha3::TON => "TON",
                #[cfg(feature = "tr")] // The Republic of Turkey (Asia)
                Alpha3::TUR => "TUR",
                #[cfg(feature = "tt")] // The Republic of Trinidad and Tobago (Americas)
                Alpha3::TTO => "TTO",
                #[cfg(feature = "tv")] // Tuvalu (Oceania)
                Alpha3::TUV => "TUV",
                #[cfg(feature = "tw")] // The Republic of China (Asia)
                Alpha3::TWN => "TWN",
                #[cfg(feature = "tz")] // The United Republic of Tanzania (Africa)
                Alpha3::TZA => "TZA",
                #[cfg(feature = "ua")] // Ukraine (Europe)
                Alpha3::UKR => "UKR",
                #[cfg(feature = "ug")] // The Republic of Uganda (Africa)
                Alpha3::UGA => "UGA",
                #[cfg(feature = "um")] // United States Minor Outlying Islands (Americas)
                Alpha3::UMI => "UMI",
                #[cfg(feature = "us")] // The United States of America (Americas)
                Alpha3::USA => "USA",
                #[cfg(feature = "uy")] // The Oriental Republic of Uruguay (Americas)
                Alpha3::URY => "URY",
                #[cfg(feature = "uz")] // The Republic of Uzbekistan (Asia)
                Alpha3::UZB => "UZB",
                #[cfg(feature = "va")] // The Holy See (Europe)
                Alpha3::VAT => "VAT",
                #[cfg(feature = "vc")] // Saint Vincent and the Grenadines (Americas)
                Alpha3::VCT => "VCT",
                #[cfg(feature = "ve")] // The Bolivarian Republic of Venezuela (Americas)
                Alpha3::VEN => "VEN",
                #[cfg(feature = "vg")] // The Virgin Islands (Americas)
                Alpha3::VGB => "VGB",
                #[cfg(feature = "vi")] // The Virgin Islands of the United States (Americas)
                Alpha3::VIR => "VIR",
                #[cfg(feature = "vn")] // The Socialist Republic of Viet Nam (Asia)
                Alpha3::VNM => "VNM",
                #[cfg(feature = "vu")] // The Republic of Vanuatu (Oceania)
                Alpha3::VUT => "VUT",
                #[cfg(feature = "wf")] // The Territory of the Wallis and Futuna Islands (Oceania)
                Alpha3::WLF => "WLF",
                #[cfg(feature = "ws")] // The Independent State of Samoa (Oceania)
                Alpha3::WSM => "WSM",
                #[cfg(feature = "ye")] // The Republic of Yemen (Asia)
                Alpha3::YEM => "YEM",
                #[cfg(feature = "yt")] // The Department of Mayotte (Africa)
                Alpha3::MYT => "MYT",
                #[cfg(feature = "za")] // The Republic of South Africa (Africa)
                Alpha3::ZAF => "ZAF",
                #[cfg(feature = "zm")] // The Republic of Zambia (Africa)
                Alpha3::ZMB => "ZMB",
                #[cfg(feature = "zw")] // The Republic of Zimbabwe (Africa)
                Alpha3::ZWE => "ZWE",
            }
            .to_string()
        }
    }

    impl Alpha3 {
        pub fn to_alpha2(&self) -> Alpha2 {
            match self {
                #[cfg(feature = "ad")] // The Principality of Andorra (Europe)
                Alpha3::AND => Alpha2::AD,
                #[cfg(feature = "ae")] // The United Arab Emirates (Asia)
                Alpha3::ARE => Alpha2::AE,
                #[cfg(feature = "af")] // The Islamic Republic of Afghanistan (Asia)
                Alpha3::AFG => Alpha2::AF,
                #[cfg(feature = "ag")] // Antigua and Barbuda (Americas)
                Alpha3::ATG => Alpha2::AG,
                #[cfg(feature = "ai")] // Anguilla (Americas)
                Alpha3::AIA => Alpha2::AI,
                #[cfg(feature = "al")] // The Republic of Albania (Europe)
                Alpha3::ALB => Alpha2::AL,
                #[cfg(feature = "am")] // The Republic of Armenia (Asia)
                Alpha3::ARM => Alpha2::AM,
                #[cfg(feature = "ao")] // The Republic of Angola (Africa)
                Alpha3::AGO => Alpha2::AO,
                #[cfg(feature = "aq")] // Antarctica
                Alpha3::ATA => Alpha2::AQ,
                #[cfg(feature = "ar")] // The Argentine Republic (Americas)
                Alpha3::ARG => Alpha2::AR,
                #[cfg(feature = "as")] // The Territory of American Samoa (Oceania)
                Alpha3::ASM => Alpha2::AS,
                #[cfg(feature = "at")] // The Republic of Austria (Europe)
                Alpha3::AUT => Alpha2::AT,
                #[cfg(feature = "au")] // The Commonwealth of Australia (Oceania)
                Alpha3::AUS => Alpha2::AU,
                #[cfg(feature = "aw")] // Aruba (Americas)
                Alpha3::ABW => Alpha2::AW,
                #[cfg(feature = "ax")] // Åland (Europe)
                Alpha3::ALA => Alpha2::AX,
                #[cfg(feature = "az")] // The Republic of Azerbaijan (Asia)
                Alpha3::AZE => Alpha2::AZ,
                #[cfg(feature = "ba")] // Bosnia and Herzegovina (Europe)
                Alpha3::BIH => Alpha2::BA,
                #[cfg(feature = "bb")] // Barbados (Americas)
                Alpha3::BRB => Alpha2::BB,
                #[cfg(feature = "bd")] // The People's Republic of Bangladesh (Asia)
                Alpha3::BGD => Alpha2::BD,
                #[cfg(feature = "be")] // The Kingdom of Belgium (Europe)
                Alpha3::BEL => Alpha2::BE,
                #[cfg(feature = "bf")] // Burkina Faso (Africa)
                Alpha3::BFA => Alpha2::BF,
                #[cfg(feature = "bg")] // The Republic of Bulgaria (Europe)
                Alpha3::BGR => Alpha2::BG,
                #[cfg(feature = "bh")] // The Kingdom of Bahrain (Asia)
                Alpha3::BHR => Alpha2::BH,
                #[cfg(feature = "bi")] // The Republic of Burundi (Africa)
                Alpha3::BDI => Alpha2::BI,
                #[cfg(feature = "bj")] // The Republic of Benin (Africa)
                Alpha3::BEN => Alpha2::BJ,
                #[cfg(feature = "bl")] // The Collectivity of Saint-Barthélemy (Americas)
                Alpha3::BLM => Alpha2::BL,
                #[cfg(feature = "bm")] // Bermuda (Americas)
                Alpha3::BMU => Alpha2::BM,
                #[cfg(feature = "bn")] // The Nation of Brunei, the Abode of Peace (Asia)
                Alpha3::BRN => Alpha2::BN,
                #[cfg(feature = "bo")] // The Plurinational State of Bolivia (Americas)
                Alpha3::BOL => Alpha2::BO,
                #[cfg(feature = "bq")] // Bonaire, Sint Eustatius and Saba (Americas)
                Alpha3::BES => Alpha2::BQ,
                #[cfg(feature = "br")] // The Federative Republic of Brazil (Americas)
                Alpha3::BRA => Alpha2::BR,
                #[cfg(feature = "bs")] // The Commonwealth of The Bahamas (Americas)
                Alpha3::BHS => Alpha2::BS,
                #[cfg(feature = "bt")] // The Kingdom of Bhutan (Asia)
                Alpha3::BTN => Alpha2::BT,
                #[cfg(feature = "bv")] // Bouvet Island
                Alpha3::BVT => Alpha2::BV,
                #[cfg(feature = "bw")] // The Republic of Botswana (Africa)
                Alpha3::BWA => Alpha2::BW,
                #[cfg(feature = "by")] // The Republic of Belarus (Europe)
                Alpha3::BLR => Alpha2::BY,
                #[cfg(feature = "bz")] // Belize (Americas)
                Alpha3::BLZ => Alpha2::BZ,
                #[cfg(feature = "ca")] // Canada (Americas)
                Alpha3::CAN => Alpha2::CA,
                #[cfg(feature = "cc")] // The Territory of Cocos (Keeling) Islands (Oceania)
                Alpha3::CCK => Alpha2::CC,
                #[cfg(feature = "cd")] // The Democratic Republic of the Congo (Africa)
                Alpha3::COD => Alpha2::CD,
                #[cfg(feature = "cf")] // The Central African Republic (Africa)
                Alpha3::CAF => Alpha2::CF,
                #[cfg(feature = "cg")] // The Republic of the Congo (Africa)
                Alpha3::COG => Alpha2::CG,
                #[cfg(feature = "ch")] // The Swiss Confederation (Europe)
                Alpha3::CHE => Alpha2::CH,
                #[cfg(feature = "ci")] // The Republic of Côte d'Ivoire (Africa)
                Alpha3::CIV => Alpha2::CI,
                #[cfg(feature = "ck")] // The Cook Islands (Oceania)
                Alpha3::COK => Alpha2::CK,
                #[cfg(feature = "cl")] // The Republic of Chile (Americas)
                Alpha3::CHL => Alpha2::CL,
                #[cfg(feature = "cm")] // The Republic of Cameroon (Africa)
                Alpha3::CMR => Alpha2::CM,
                #[cfg(feature = "cn")] // The People's Republic of China (Asia)
                Alpha3::CHN => Alpha2::CN,
                #[cfg(feature = "co")] // The Republic of Colombia (Americas)
                Alpha3::COL => Alpha2::CO,
                #[cfg(feature = "cr")] // The Republic of Costa Rica (Americas)
                Alpha3::CRI => Alpha2::CR,
                #[cfg(feature = "cu")] // The Republic of Cuba (Americas)
                Alpha3::CUB => Alpha2::CU,
                #[cfg(feature = "cv")] // The Republic of Cabo Verde (Africa)
                Alpha3::CPV => Alpha2::CV,
                #[cfg(feature = "cw")] // The Country of Curaçao (Americas)
                Alpha3::CUW => Alpha2::CW,
                #[cfg(feature = "cx")] // The Territory of Christmas Island (Oceania)
                Alpha3::CXR => Alpha2::CX,
                #[cfg(feature = "cy")] // The Republic of Cyprus (Asia)
                Alpha3::CYP => Alpha2::CY,
                #[cfg(feature = "cz")] // The Czech Republic (Europe)
                Alpha3::CZE => Alpha2::CZ,
                #[cfg(feature = "de")] // The Federal Republic of Germany (Europe)
                Alpha3::DEU => Alpha2::DE,
                #[cfg(feature = "dj")] // The Republic of Djibouti (Africa)
                Alpha3::DJI => Alpha2::DJ,
                #[cfg(feature = "dk")] // The Kingdom of Denmark (Europe)
                Alpha3::DNK => Alpha2::DK,
                #[cfg(feature = "dm")] // The Commonwealth of Dominica (Americas)
                Alpha3::DMA => Alpha2::DM,
                #[cfg(feature = "do")] // The Dominican Republic (Americas)
                Alpha3::DOM => Alpha2::DO,
                #[cfg(feature = "dz")] // The People's Democratic Republic of Algeria (Africa)
                Alpha3::DZA => Alpha2::DZ,
                #[cfg(feature = "ec")] // The Republic of Ecuador (Americas)
                Alpha3::ECU => Alpha2::EC,
                #[cfg(feature = "ee")] // The Republic of Estonia (Europe)
                Alpha3::EST => Alpha2::EE,
                #[cfg(feature = "eg")] // The Arab Republic of Egypt (Africa)
                Alpha3::EGY => Alpha2::EG,
                #[cfg(feature = "eh")] // The Sahrawi Arab Democratic Republic (Africa)
                Alpha3::ESH => Alpha2::EH,
                #[cfg(feature = "er")] // The State of Eritrea (Africa)
                Alpha3::ERI => Alpha2::ER,
                #[cfg(feature = "es")] // The Kingdom of Spain (Europe)
                Alpha3::ESP => Alpha2::ES,
                #[cfg(feature = "et")] // The Federal Democratic Republic of Ethiopia (Africa)
                Alpha3::ETH => Alpha2::ET,
                #[cfg(feature = "fi")] // The Republic of Finland (Europe)
                Alpha3::FIN => Alpha2::FI,
                #[cfg(feature = "fj")] // The Republic of Fiji (Oceania)
                Alpha3::FJI => Alpha2::FJ,
                #[cfg(feature = "fk")] // The Falkland Islands (Americas)
                Alpha3::FLK => Alpha2::FK,
                #[cfg(feature = "fm")] // The Federated States of Micronesia (Oceania)
                Alpha3::FSM => Alpha2::FM,
                #[cfg(feature = "fo")] // The Faroe Islands (Europe)
                Alpha3::FRO => Alpha2::FO,
                #[cfg(feature = "fr")] // The French Republic (Europe)
                Alpha3::FRA => Alpha2::FR,
                #[cfg(feature = "ga")] // The Gabonese Republic (Africa)
                Alpha3::GAB => Alpha2::GA,
                #[cfg(feature = "gb")]
                // The United Kingdom of Great Britain and Northern Ireland (Europe)
                Alpha3::GBR => Alpha2::GB,
                #[cfg(feature = "gd")] // Grenada (Americas)
                Alpha3::GRD => Alpha2::GD,
                #[cfg(feature = "ge")] // Georgia (Asia)
                Alpha3::GEO => Alpha2::GE,
                #[cfg(feature = "gf")] // Guyane (Americas)
                Alpha3::GUF => Alpha2::GF,
                #[cfg(feature = "gg")] // The Bailiwick of Guernsey (Europe)
                Alpha3::GGY => Alpha2::GG,
                #[cfg(feature = "gh")] // The Republic of Ghana (Africa)
                Alpha3::GHA => Alpha2::GH,
                #[cfg(feature = "gi")] // Gibraltar (Europe)
                Alpha3::GIB => Alpha2::GI,
                #[cfg(feature = "gl")] // Kalaallit Nunaat (Americas)
                Alpha3::GRL => Alpha2::GL,
                #[cfg(feature = "gm")] // The Republic of The Gambia (Africa)
                Alpha3::GMB => Alpha2::GM,
                #[cfg(feature = "gn")] // The Republic of Guinea (Africa)
                Alpha3::GIN => Alpha2::GN,
                #[cfg(feature = "gp")] // Guadeloupe (Americas)
                Alpha3::GLP => Alpha2::GP,
                #[cfg(feature = "gq")] // The Republic of Equatorial Guinea (Africa)
                Alpha3::GNQ => Alpha2::GQ,
                #[cfg(feature = "gr")] // The Hellenic Republic (Europe)
                Alpha3::GRC => Alpha2::GR,
                #[cfg(feature = "gs")] // South Georgia and the South Sandwich Islands (Americas)
                Alpha3::SGS => Alpha2::GS,
                #[cfg(feature = "gt")] // The Republic of Guatemala (Americas)
                Alpha3::GTM => Alpha2::GT,
                #[cfg(feature = "gu")] // The Territory of Guam (Oceania)
                Alpha3::GUM => Alpha2::GU,
                #[cfg(feature = "gw")] // The Republic of Guinea-Bissau (Africa)
                Alpha3::GNB => Alpha2::GW,
                #[cfg(feature = "gy")] // The Co-operative Republic of Guyana (Americas)
                Alpha3::GUY => Alpha2::GY,
                #[cfg(feature = "hk")]
                // The Hong Kong Special Administrative Region of China (Asia)
                Alpha3::HKG => Alpha2::HK,
                #[cfg(feature = "hm")] // The Territory of Heard Island and McDonald Islands
                Alpha3::HMD => Alpha2::HM,
                #[cfg(feature = "hn")] // The Republic of Honduras (Americas)
                Alpha3::HND => Alpha2::HN,
                #[cfg(feature = "hr")] // The Republic of Croatia (Europe)
                Alpha3::HRV => Alpha2::HR,
                #[cfg(feature = "ht")] // The Republic of Haiti (Americas)
                Alpha3::HTI => Alpha2::HT,
                #[cfg(feature = "hu")] // Hungary (Europe)
                Alpha3::HUN => Alpha2::HU,
                #[cfg(feature = "id")] // The Republic of Indonesia (Asia)
                Alpha3::IDN => Alpha2::ID,
                #[cfg(feature = "ie")] // Ireland (Europe)
                Alpha3::IRL => Alpha2::IE,
                #[cfg(feature = "il")] // The State of Israel (Asia)
                Alpha3::ISR => Alpha2::IL,
                #[cfg(feature = "im")] // The Isle of Man (Europe)
                Alpha3::IMN => Alpha2::IM,
                #[cfg(feature = "in")] // The Republic of India (Asia)
                Alpha3::IND => Alpha2::IN,
                #[cfg(feature = "io")] // The British Indian Ocean Territory (Africa)
                Alpha3::IOT => Alpha2::IO,
                #[cfg(feature = "iq")] // The Republic of Iraq (Asia)
                Alpha3::IRQ => Alpha2::IQ,
                #[cfg(feature = "ir")] // The Islamic Republic of Iran (Asia)
                Alpha3::IRN => Alpha2::IR,
                #[cfg(feature = "is")] // Iceland (Europe)
                Alpha3::ISL => Alpha2::IS,
                #[cfg(feature = "it")] // The Italian Republic (Europe)
                Alpha3::ITA => Alpha2::IT,
                #[cfg(feature = "je")] // The Bailiwick of Jersey (Europe)
                Alpha3::JEY => Alpha2::JE,
                #[cfg(feature = "jm")] // Jamaica (Americas)
                Alpha3::JAM => Alpha2::JM,
                #[cfg(feature = "jo")] // The Hashemite Kingdom of Jordan (Asia)
                Alpha3::JOR => Alpha2::JO,
                #[cfg(feature = "jp")] // Japan (Asia)
                Alpha3::JPN => Alpha2::JP,
                #[cfg(feature = "ke")] // The Republic of Kenya (Africa)
                Alpha3::KEN => Alpha2::KE,
                #[cfg(feature = "kg")] // The Kyrgyz Republic (Asia)
                Alpha3::KGZ => Alpha2::KG,
                #[cfg(feature = "kh")] // The Kingdom of Cambodia (Asia)
                Alpha3::KHM => Alpha2::KH,
                #[cfg(feature = "ki")] // The Republic of Kiribati (Oceania)
                Alpha3::KIR => Alpha2::KI,
                #[cfg(feature = "km")] // The Union of the Comoros (Africa)
                Alpha3::COM => Alpha2::KM,
                #[cfg(feature = "kn")] // Saint Kitts and Nevis (Americas)
                Alpha3::KNA => Alpha2::KN,
                #[cfg(feature = "kp")] // The Democratic People's Republic of Korea (Asia)
                Alpha3::PRK => Alpha2::KP,
                #[cfg(feature = "kr")] // The Republic of Korea (Asia)
                Alpha3::KOR => Alpha2::KR,
                #[cfg(feature = "kw")] // The State of Kuwait (Asia)
                Alpha3::KWT => Alpha2::KW,
                #[cfg(feature = "ky")] // The Cayman Islands (Americas)
                Alpha3::CYM => Alpha2::KY,
                #[cfg(feature = "kz")] // The Republic of Kazakhstan (Asia)
                Alpha3::KAZ => Alpha2::KZ,
                #[cfg(feature = "la")] // The Lao People's Democratic Republic (Asia)
                Alpha3::LAO => Alpha2::LA,
                #[cfg(feature = "lb")] // The Lebanese Republic (Asia)
                Alpha3::LBN => Alpha2::LB,
                #[cfg(feature = "lc")] // Saint Lucia (Americas)
                Alpha3::LCA => Alpha2::LC,
                #[cfg(feature = "li")] // The Principality of Liechtenstein (Europe)
                Alpha3::LIE => Alpha2::LI,
                #[cfg(feature = "lk")] // The Democratic Socialist Republic of Sri Lanka (Asia)
                Alpha3::LKA => Alpha2::LK,
                #[cfg(feature = "lr")] // The Republic of Liberia (Africa)
                Alpha3::LBR => Alpha2::LR,
                #[cfg(feature = "ls")] // The Kingdom of Lesotho (Africa)
                Alpha3::LSO => Alpha2::LS,
                #[cfg(feature = "lt")] // The Republic of Lithuania (Europe)
                Alpha3::LTU => Alpha2::LT,
                #[cfg(feature = "lu")] // The Grand Duchy of Luxembourg (Europe)
                Alpha3::LUX => Alpha2::LU,
                #[cfg(feature = "lv")] // The Republic of Latvia (Europe)
                Alpha3::LVA => Alpha2::LV,
                #[cfg(feature = "ly")] // The State of Libya (Africa)
                Alpha3::LBY => Alpha2::LY,
                #[cfg(feature = "ma")] // The Kingdom of Morocco (Africa)
                Alpha3::MAR => Alpha2::MA,
                #[cfg(feature = "mc")] // The Principality of Monaco (Europe)
                Alpha3::MCO => Alpha2::MC,
                #[cfg(feature = "md")] // The Republic of Moldova (Europe)
                Alpha3::MDA => Alpha2::MD,
                #[cfg(feature = "me")] // Montenegro (Europe)
                Alpha3::MNE => Alpha2::ME,
                #[cfg(feature = "mf")] // The Collectivity of Saint-Martin (Americas)
                Alpha3::MAF => Alpha2::MF,
                #[cfg(feature = "mg")] // The Republic of Madagascar (Africa)
                Alpha3::MDG => Alpha2::MG,
                #[cfg(feature = "mh")] // The Republic of the Marshall Islands (Oceania)
                Alpha3::MHL => Alpha2::MH,
                #[cfg(feature = "mk")] // The Republic of North Macedonia (Europe)
                Alpha3::MKD => Alpha2::MK,
                #[cfg(feature = "ml")] // The Republic of Mali (Africa)
                Alpha3::MLI => Alpha2::ML,
                #[cfg(feature = "mm")] // The Republic of the Union of Myanmar (Asia)
                Alpha3::MMR => Alpha2::MM,
                #[cfg(feature = "mn")] // Mongolia (Asia)
                Alpha3::MNG => Alpha2::MN,
                #[cfg(feature = "mo")] // The Macao Special Administrative Region of China (Asia)
                Alpha3::MAC => Alpha2::MO,
                #[cfg(feature = "mp")] // The Commonwealth of the Northern Mariana Islands (Oceania)
                Alpha3::MNP => Alpha2::MP,
                #[cfg(feature = "mq")] // Martinique (Americas)
                Alpha3::MTQ => Alpha2::MQ,
                #[cfg(feature = "mr")] // The Islamic Republic of Mauritania (Africa)
                Alpha3::MRT => Alpha2::MR,
                #[cfg(feature = "ms")] // Montserrat (Americas)
                Alpha3::MSR => Alpha2::MS,
                #[cfg(feature = "mt")] // The Republic of Malta (Europe)
                Alpha3::MLT => Alpha2::MT,
                #[cfg(feature = "mu")] // The Republic of Mauritius (Africa)
                Alpha3::MUS => Alpha2::MU,
                #[cfg(feature = "mv")] // The Republic of Maldives (Asia)
                Alpha3::MDV => Alpha2::MV,
                #[cfg(feature = "mw")] // The Republic of Malawi (Africa)
                Alpha3::MWI => Alpha2::MW,
                #[cfg(feature = "mx")] // The United Mexican States (Americas)
                Alpha3::MEX => Alpha2::MX,
                #[cfg(feature = "my")] // Malaysia (Asia)
                Alpha3::MYS => Alpha2::MY,
                #[cfg(feature = "mz")] // The Republic of Mozambique (Africa)
                Alpha3::MOZ => Alpha2::MZ,
                #[cfg(feature = "na")] // The Republic of Namibia (Africa)
                Alpha3::NAM => Alpha2::NA,
                #[cfg(feature = "nc")] // New Caledonia (Oceania)
                Alpha3::NCL => Alpha2::NC,
                #[cfg(feature = "ne")] // The Republic of the Niger (Africa)
                Alpha3::NER => Alpha2::NE,
                #[cfg(feature = "nf")] // The Territory of Norfolk Island (Oceania)
                Alpha3::NFK => Alpha2::NF,
                #[cfg(feature = "ng")] // The Federal Republic of Nigeria (Africa)
                Alpha3::NGA => Alpha2::NG,
                #[cfg(feature = "ni")] // The Republic of Nicaragua (Americas)
                Alpha3::NIC => Alpha2::NI,
                #[cfg(feature = "nl")] // The Kingdom of the Netherlands (Europe)
                Alpha3::NLD => Alpha2::NL,
                #[cfg(feature = "no")] // The Kingdom of Norway (Europe)
                Alpha3::NOR => Alpha2::NO,
                #[cfg(feature = "np")] // The Federal Democratic Republic of Nepal (Asia)
                Alpha3::NPL => Alpha2::NP,
                #[cfg(feature = "nr")] // The Republic of Nauru (Oceania)
                Alpha3::NRU => Alpha2::NR,
                #[cfg(feature = "nu")] // Niue (Oceania)
                Alpha3::NIU => Alpha2::NU,
                #[cfg(feature = "nz")] // New Zealand (Oceania)
                Alpha3::NZL => Alpha2::NZ,
                #[cfg(feature = "om")] // The Sultanate of Oman (Asia)
                Alpha3::OMN => Alpha2::OM,
                #[cfg(feature = "pa")] // The Republic of Panamá (Americas)
                Alpha3::PAN => Alpha2::PA,
                #[cfg(feature = "pe")] // The Republic of Perú (Americas)
                Alpha3::PER => Alpha2::PE,
                #[cfg(feature = "pf")] // French Polynesia (Oceania)
                Alpha3::PYF => Alpha2::PF,
                #[cfg(feature = "pg")] // The Independent State of Papua New Guinea (Oceania)
                Alpha3::PNG => Alpha2::PG,
                #[cfg(feature = "ph")] // The Republic of the Philippines (Asia)
                Alpha3::PHL => Alpha2::PH,
                #[cfg(feature = "pk")] // The Islamic Republic of Pakistan (Asia)
                Alpha3::PAK => Alpha2::PK,
                #[cfg(feature = "pl")] // The Republic of Poland (Europe)
                Alpha3::POL => Alpha2::PL,
                #[cfg(feature = "pm")]
                // The Overseas Collectivity of Saint-Pierre and Miquelon (Americas)
                Alpha3::SPM => Alpha2::PM,
                #[cfg(feature = "pn")] // The Pitcairn, Henderson, Ducie and Oeno Islands (Oceania)
                Alpha3::PCN => Alpha2::PN,
                #[cfg(feature = "pr")] // The Commonwealth of Puerto Rico (Americas)
                Alpha3::PRI => Alpha2::PR,
                #[cfg(feature = "ps")] // The State of Palestine (Asia)
                Alpha3::PSE => Alpha2::PS,
                #[cfg(feature = "pt")] // The Portuguese Republic (Europe)
                Alpha3::PRT => Alpha2::PT,
                #[cfg(feature = "pw")] // The Republic of Palau (Oceania)
                Alpha3::PLW => Alpha2::PW,
                #[cfg(feature = "py")] // The Republic of Paraguay (Americas)
                Alpha3::PRY => Alpha2::PY,
                #[cfg(feature = "qa")] // The State of Qatar (Asia)
                Alpha3::QAT => Alpha2::QA,
                #[cfg(feature = "re")] // Réunion (Africa)
                Alpha3::REU => Alpha2::RE,
                #[cfg(feature = "ro")] // Romania (Europe)
                Alpha3::ROU => Alpha2::RO,
                #[cfg(feature = "rs")] // The Republic of Serbia (Europe)
                Alpha3::SRB => Alpha2::RS,
                #[cfg(feature = "ru")] // The Russian Federation (Europe)
                Alpha3::RUS => Alpha2::RU,
                #[cfg(feature = "rw")] // The Republic of Rwanda (Africa)
                Alpha3::RWA => Alpha2::RW,
                #[cfg(feature = "sa")] // The Kingdom of Saudi Arabia (Asia)
                Alpha3::SAU => Alpha2::SA,
                #[cfg(feature = "sb")] // The Solomon Islands (Oceania)
                Alpha3::SLB => Alpha2::SB,
                #[cfg(feature = "sc")] // The Republic of Seychelles (Africa)
                Alpha3::SYC => Alpha2::SC,
                #[cfg(feature = "sd")] // The Republic of the Sudan (Africa)
                Alpha3::SDN => Alpha2::SD,
                #[cfg(feature = "se")] // The Kingdom of Sweden (Europe)
                Alpha3::SWE => Alpha2::SE,
                #[cfg(feature = "sg")] // The Republic of Singapore (Asia)
                Alpha3::SGP => Alpha2::SG,
                #[cfg(feature = "sh")] // Saint Helena, Ascension and Tristan da Cunha (Africa)
                Alpha3::SHN => Alpha2::SH,
                #[cfg(feature = "si")] // The Republic of Slovenia (Europe)
                Alpha3::SVN => Alpha2::SI,
                #[cfg(feature = "sj")] // Svalbard and Jan Mayen (Europe)
                Alpha3::SJM => Alpha2::SJ,
                #[cfg(feature = "sk")] // The Slovak Republic (Europe)
                Alpha3::SVK => Alpha2::SK,
                #[cfg(feature = "sl")] // The Republic of Sierra Leone (Africa)
                Alpha3::SLE => Alpha2::SL,
                #[cfg(feature = "sm")] // The Republic of San Marino (Europe)
                Alpha3::SMR => Alpha2::SM,
                #[cfg(feature = "sn")] // The Republic of Senegal (Africa)
                Alpha3::SEN => Alpha2::SN,
                #[cfg(feature = "so")] // The Federal Republic of Somalia (Africa)
                Alpha3::SOM => Alpha2::SO,
                #[cfg(feature = "sr")] // The Republic of Suriname (Americas)
                Alpha3::SUR => Alpha2::SR,
                #[cfg(feature = "ss")] // The Republic of South Sudan (Africa)
                Alpha3::SSD => Alpha2::SS,
                #[cfg(feature = "st")] // The Democratic Republic of São Tomé and Príncipe (Africa)
                Alpha3::STP => Alpha2::ST,
                #[cfg(feature = "sv")] // The Republic of El Salvador (Americas)
                Alpha3::SLV => Alpha2::SV,
                #[cfg(feature = "sx")] // Sint Maarten (Americas)
                Alpha3::SXM => Alpha2::SX,
                #[cfg(feature = "sy")] // The Syrian Arab Republic (Asia)
                Alpha3::SYR => Alpha2::SY,
                #[cfg(feature = "sz")] // The Kingdom of Eswatini (Africa)
                Alpha3::SWZ => Alpha2::SZ,
                #[cfg(feature = "tc")] // The Turks and Caicos Islands (Americas)
                Alpha3::TCA => Alpha2::TC,
                #[cfg(feature = "td")] // The Republic of Chad (Africa)
                Alpha3::TCD => Alpha2::TD,
                #[cfg(feature = "tf")] // The French Southern and Antarctic Lands (Africa)
                Alpha3::ATF => Alpha2::TF,
                #[cfg(feature = "tg")] // The Togolese Republic (Africa)
                Alpha3::TGO => Alpha2::TG,
                #[cfg(feature = "th")] // The Kingdom of Thailand (Asia)
                Alpha3::THA => Alpha2::TH,
                #[cfg(feature = "tj")] // The Republic of Tajikistan (Asia)
                Alpha3::TJK => Alpha2::TJ,
                #[cfg(feature = "tk")] // Tokelau (Oceania)
                Alpha3::TKL => Alpha2::TK,
                #[cfg(feature = "tl")] // The Democratic Republic of Timor-Leste (Asia)
                Alpha3::TLS => Alpha2::TL,
                #[cfg(feature = "tm")] // Turkmenistan (Asia)
                Alpha3::TKM => Alpha2::TM,
                #[cfg(feature = "tn")] // The Republic of Tunisia (Africa)
                Alpha3::TUN => Alpha2::TN,
                #[cfg(feature = "to")] // The Kingdom of Tonga (Oceania)
                Alpha3::TON => Alpha2::TO,
                #[cfg(feature = "tr")] // The Republic of Turkey (Asia)
                Alpha3::TUR => Alpha2::TR,
                #[cfg(feature = "tt")] // The Republic of Trinidad and Tobago (Americas)
                Alpha3::TTO => Alpha2::TT,
                #[cfg(feature = "tv")] // Tuvalu (Oceania)
                Alpha3::TUV => Alpha2::TV,
                #[cfg(feature = "tw")] // The Republic of China (Asia)
                Alpha3::TWN => Alpha2::TW,
                #[cfg(feature = "tz")] // The United Republic of Tanzania (Africa)
                Alpha3::TZA => Alpha2::TZ,
                #[cfg(feature = "ua")] // Ukraine (Europe)
                Alpha3::UKR => Alpha2::UA,
                #[cfg(feature = "ug")] // The Republic of Uganda (Africa)
                Alpha3::UGA => Alpha2::UG,
                #[cfg(feature = "um")] // United States Minor Outlying Islands (Americas)
                Alpha3::UMI => Alpha2::UM,
                #[cfg(feature = "us")] // The United States of America (Americas)
                Alpha3::USA => Alpha2::US,
                #[cfg(feature = "uy")] // The Oriental Republic of Uruguay (Americas)
                Alpha3::URY => Alpha2::UY,
                #[cfg(feature = "uz")] // The Republic of Uzbekistan (Asia)
                Alpha3::UZB => Alpha2::UZ,
                #[cfg(feature = "va")] // The Holy See (Europe)
                Alpha3::VAT => Alpha2::VA,
                #[cfg(feature = "vc")] // Saint Vincent and the Grenadines (Americas)
                Alpha3::VCT => Alpha2::VC,
                #[cfg(feature = "ve")] // The Bolivarian Republic of Venezuela (Americas)
                Alpha3::VEN => Alpha2::VE,
                #[cfg(feature = "vg")] // The Virgin Islands (Americas)
                Alpha3::VGB => Alpha2::VG,
                #[cfg(feature = "vi")] // The Virgin Islands of the United States (Americas)
                Alpha3::VIR => Alpha2::VI,
                #[cfg(feature = "vn")] // The Socialist Republic of Viet Nam (Asia)
                Alpha3::VNM => Alpha2::VN,
                #[cfg(feature = "vu")] // The Republic of Vanuatu (Oceania)
                Alpha3::VUT => Alpha2::VU,
                #[cfg(feature = "wf")] // The Territory of the Wallis and Futuna Islands (Oceania)
                Alpha3::WLF => Alpha2::WF,
                #[cfg(feature = "ws")] // The Independent State of Samoa (Oceania)
                Alpha3::WSM => Alpha2::WS,
                #[cfg(feature = "ye")] // The Republic of Yemen (Asia)
                Alpha3::YEM => Alpha2::YE,
                #[cfg(feature = "yt")] // The Department of Mayotte (Africa)
                Alpha3::MYT => Alpha2::YT,
                #[cfg(feature = "za")] // The Republic of South Africa (Africa)
                Alpha3::ZAF => Alpha2::ZA,
                #[cfg(feature = "zm")] // The Republic of Zambia (Africa)
                Alpha3::ZMB => Alpha2::ZM,
                #[cfg(feature = "zw")] // The Republic of Zimbabwe (Africa)
                Alpha3::ZWE => Alpha2::ZW,
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
mod impls {
    use super::{Alpha2, Alpha3};
    use crate::SearchError;

    impl TryFrom<&str> for Alpha3 {
        type Error = SearchError;

        fn try_from(_value: &str) -> Result<Self, Self::Error> {
            unimplemented!("No country feature is used");
        }
    }

    impl ToString for Alpha3 {
        fn to_string(&self) -> String {
            unimplemented!("No country feature is used");
        }
    }

    impl Alpha3 {
        pub fn to_alpha2(&self) -> Alpha2 {
            unimplemented!("No country feature is used");
        }
    }
}
