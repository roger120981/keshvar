// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of the Philippines

#[cfg(all(feature = "ph", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}} {{region_short}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::PH;
    pub const ALPHA3: Alpha3 = Alpha3::PHL;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 63;
    pub const CURRENCY_CODE: &str = "PHP";
    pub const GEC: Option<GEC> = Some(GEC::RP);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("PHI");
    pub const ISO_SHORT_NAME: &str = "Philippines";
    pub const ISO_LONG_NAME: &str = "The Republic of the Philippines";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en", "tl"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en", "tl"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9, 10];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Filipino");
    pub const NUMBER: &str = "608";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthEasternAsia);
    pub const UN_LOCODE: &str = "PH";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Philippines",
        "Philippinen",
        "Filipinas",
        "フィリピン",
        "Filipijnen",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Philippines"),
        ("af", "Filippyne"),
        ("ak", "Philippines"),
        ("am", "ፊሊፒንስ"),
        ("an", "Philippines"),
        ("ar", "الفلب\u{651}ين"),
        ("as", "ফিলিপ\u{9be}ইনছ"),
        ("ay", "Philippines"),
        ("az", "Filippin"),
        ("ba", "Philippines"),
        ("be", "Філіпіны"),
        ("bg", "Филипини"),
        ("bi", "Philippines"),
        ("bn", "ফিলিপ\u{9be}ইনস"),
        ("bn_IN", "ফিলিপ\u{9be}ইনস"),
        ("br", "Filipinez"),
        ("bs", "Filipini"),
        ("ca", "Filipines"),
        ("ce", "Филиппин"),
        ("ch", "Filipinas"),
        ("cs", "Filipíny"),
        ("cv", "Филиппин"),
        ("cy", "Pilipinas"),
        ("da", "Filippinerne"),
        ("de", "Philippinen"),
        ("dv", "ފ\u{7a8}ލ\u{7a8}ޕ\u{7a9}ނ\u{7b0}ސ\u{7b0}"),
        ("dz", "ཕ\u{f72}་ལ\u{f72}་པ\u{f72}ནས\u{f72}།"),
        ("ee", "Philippines"),
        ("el", "Φιλιππίνες"),
        ("en", "Philippines"),
        ("eo", "Filipinoj"),
        ("es", "Filipinas"),
        ("et", "Filipiinid"),
        ("eu", "Filipinak"),
        ("fa", "فیلیپین"),
        ("ff", "Philippines"),
        ("fi", "Filippiinit"),
        ("fo", "Filipsoyggjar"),
        ("fr", "Philippines"),
        ("fy", "Filipinen"),
        ("ga", "Na hOileáin Fhilipíneacha"),
        ("gl", "Filipinas"),
        ("gn", "Philippines"),
        ("gu", "ફિલિપાઇન\u{acd}સ"),
        ("gv", "Ny h-Ellanyn Philippeenagh"),
        ("ha", "Filipin"),
        ("he", "הפיליפינים"),
        ("hi", "फ\u{93c}िलीपीन\u{94d}स"),
        ("hr", "Filipini"),
        ("ht", "Filipin"),
        ("hu", "Fülöp-szigetek"),
        ("hy", "Ֆիլիպիններ"),
        ("ia", "Philippinas"),
        ("id", "Filipina"),
        ("io", "Filipini"),
        ("is", "Filippseyjar"),
        ("it", "Filippine"),
        ("iu", "Philippines"),
        ("ja", "フィリピン"),
        ("ka", "ფილიპინები"),
        ("ki", "Philippines"),
        ("kk", "Филиппин"),
        ("kl", "Philippines"),
        ("km", "ហ\u{17d2}វ\u{17b8}ល\u{17b8}ព\u{17b8}ន"),
        ("kn", "ಫ\u{cbf}ಲ\u{cbf}ಪ\u{ccd}ಪೈನ\u{ccd}ಸ\u{ccd}"),
        ("ko", "필리핀"),
        ("ku", "Fîlîpîn"),
        ("kv", "Филиппинъяс"),
        ("kw", "Filipinys"),
        ("ky", "Филиппиндер"),
        ("lo", "ປະເທດຟ\u{eb5}ລ\u{eb4}ບປ\u{eb4}ນ"),
        ("lt", "Filipinai"),
        ("lv", "Filipīnas"),
        ("mi", "Piripīni"),
        ("mk", "Филипини"),
        ("ml", "ഫിലിപ\u{d4d}പൈന\u{d4d}\u{200d}സ\u{d4d}"),
        ("mn", "Флиппен"),
        ("mr", "फिलिपिन\u{94d}स"),
        ("ms", "Filipina"),
        ("mt", "Filippini"),
        (
            "my",
            "ဖ\u{102d}လစ\u{103a}ပ\u{102d}\u{102f}င\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Eben Piripin"),
        ("nb", "Filippinene"),
        ("ne", "फिलिपिन\u{94d}स"),
        ("nl", "Filipijnen"),
        ("nn", "Filippinane"),
        ("nv", "Kéyah Dańlíinii"),
        ("oc", "Filipinas"),
        ("or", "ଫ\u{b3f}ଲ\u{b3f}ପ\u{b3e}ଇନ\u{b4d}ସ"),
        ("pa", "ਫਿਲਿਪੀਨੀਜ਼"),
        ("pi", "फिलिपीन\u{94d}स"),
        ("pl", "Filipiny"),
        ("ps", "فلېپين"),
        ("pt", "Filipinas"),
        ("pt_BR", "Filipinas"),
        ("ro", "Filipine"),
        ("ru", "Филиппины"),
        ("rw", "Filipine"),
        ("sc", "Filipinas"),
        ("sd", "Philippines"),
        ("si", "ප\u{dd2}ල\u{dd2}ප\u{dd3}න"),
        ("sk", "Filipíny"),
        ("sl", "Filipini"),
        ("so", "Filibiin"),
        ("sq", "Filipine"),
        ("sr", "Филипини"),
        ("sv", "Filippinerna"),
        ("sw", "Philippines"),
        ("ta", "பிலிப\u{bcd}பைன\u{bcd}ஸ\u{bcd}"),
        ("te", "ఫ\u{c3f}ల\u{c3f}ప\u{c4d}ప\u{c3f}న\u{c4d}స\u{c4d}"),
        ("tg", "Филиппинҳо"),
        ("th", "ฟ\u{e34}ล\u{e34}ปป\u{e34}นส\u{e4c}"),
        ("ti", "Philippines"),
        ("tk", "Filippinler"),
        ("tl", "Pilipinas"),
        ("tr", "Filipinler"),
        ("tt", "Филиппиннәр"),
        ("ug", "فىلىپپىن"),
        ("uk", "Філіппіни"),
        ("ur", "فلپائن"),
        ("uz", "Filippin"),
        ("ve", "Philippines"),
        ("vi", "Phi-li-pi-nợ"),
        ("wa", "Filipenes"),
        ("wo", "Filipiin"),
        ("xh", "Philippines"),
        ("yo", "Filipínì"),
        ("zh_CN", "菲律宾"),
        ("zh_HK", "菲律賓"),
        ("zh_TW", "菲律賓"),
        ("zu", "Philippines"),
    ];
    #[cfg(all(feature = "ph", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 12.879721;
        pub const LONGITUDE: f64 = 121.774017;
        pub const MAX_LATITUDE: f64 = 21.2412572;
        pub const MAX_LONGITUDE: f64 = 127.6444784;
        pub const MIN_LATITUDE: f64 = 4.2259;
        pub const MIN_LONGITUDE: f64 = 116.1474999;
        pub const NORTHEAST_LATITUDE: f64 = 21.2412572;
        pub const NORTHEAST_LONGITUDE: f64 = 127.6444784;
        pub const SOUTHWEST_LATITUDE: f64 = 4.2259;
        pub const SOUTHWEST_LONGITUDE: f64 = 116.1474999;
    }
}
#[cfg(all(feature = "ph", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 12.879721,
            longitude: 121.774017,
            max_latitude: 21.2412572,
            max_longitude: 127.6444784,
            min_latitude: 4.2259,
            min_longitude: 116.1474999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 21.2412572,
                    longitude: 127.6444784,
                },
                southwest: CountryGeoBound {
                    latitude: 4.2259,
                    longitude: 116.1474999,
                },
            },
        }
    }
}

#[cfg(all(feature = "ph", feature = "subdivisions"))]
pub mod subdivisions {
    use crate::Subdivision;
    use std::collections::HashMap;
    // In this state, We do not know if subdivisions have geo or not!
    #[cfg(feature = "geo")]
    #[allow(unused_imports)]
    use crate::{Alpha2, SubdivisionGeo, SubdivisionType};

    pub fn new() -> HashMap<&'static str, Subdivision> {
        HashMap::from(
            [

                (
                    "00",
                    Subdivision{
                        name: "00",
                        country_alpha2: Alpha2::PH,
                        code: "00",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.6090537), longitude: Some(121.0222565), max_latitude: Some(14.781217), min_latitude: Some(14.3493861), max_longitude: Some(121.132012), min_longitude: Some(120.9172569)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Сталічны Рэгіён"), ("bn", "মেট\u{9cd}রো ম\u{9be}নিল\u{9be}"), ("ca", "Metro Manila"), ("ccp", "𑄟𑄬𑄑\u{11133}𑄢\u{1112e} 𑄟𑄬𑄚\u{11128}𑄣"), ("ceb", "Manila"), ("cs", "Metro Manila"), ("de", "Metro Manila"), ("en", "Metro Manila"), ("es", "Gran Manila"), ("eu", "Manila Handia"), ("fa", "کلانشهر مانیل"), ("fi", "Metro Manila"), ("fr", "Grand Manille"), ("he", "אזור מנילה רבתי"), ("hi", "म\u{947}ट\u{94d}रो मनिला"), ("hr", "Metro Manila"), ("id", "Metro Manila"), ("it", "Regione Capitale Nazionale"), ("ja", "マニラ首都圏"), ("ko", "마닐라 대도시"), ("lt", "Manilos metropolija"), ("mk", "Метро Манила"), ("ml", "മെട\u{d4d}രോ മനില"), ("ms", "Metro Manila"), ("my", "မက\u{103a}ထရ\u{102d}\u{102f} မန\u{102e}လာ"), ("nb", "Metro Manila"), ("nl", "National Capital Region"), ("no", "Metro Manila"), ("pl", "Region Stołeczny"), ("pt", "Grande Manila"), ("ru", "Столичный Регион"), ("sl", "Metro Manila"), ("sr", "Метро Манила"), ("sr_Latn", "Metro Manila"), ("sv", "Metro Manila"), ("ta", "மணில\u{bbe} பெருநகரம\u{bcd}"), ("th", "เมโทรมะน\u{e34}ลา"), ("tr", "Metro Manila"), ("uk", "Столичний регіон (Філіппіни)"), ("ur", "میٹرو منیلا"), ("vi", "Vùng đô thị Manila"), ("yue", "馬尼拉大都會"), ("yue_Hans", "马尼拉大都会"), ("zh", "馬尼拉大都會")]),
                        unofficial_name_list: ["National Capital Region", "Pambansang Punong Rehiyon"].to_vec(),
                    }
                ),
                (
                    "01",
                    Subdivision{
                        name: "01",
                        country_alpha2: Alpha2::PH,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ca", "Regió d’Ilocos"), ("ccp", "𑄃\u{11128}𑄣\u{1112e}𑄇\u{1112e}𑄌\u{11134}"), ("ceb", "Rehiyon sa Ilocos"), ("de", "Ilocos Region"), ("en", "Ilocos"), ("es", "Ilocos"), ("eu", "Ilocos"), ("fi", "Ilocosin alue"), ("fr", "Région d’Ilocos"), ("hi", "इलोकोस क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Ilocos"), ("id", "Ilocos"), ("it", "Ilocos"), ("ja", "イロコス地方"), ("ko", "일로코스 지방"), ("lt", "Ilokoso regionas"), ("mk", "Илокос"), ("nb", "Ilocos Region"), ("nl", "Ilocos Region"), ("no", "Ilocos Region"), ("pl", "Ilocos"), ("pt", "Ilocos"), ("ru", "Илокос"), ("sv", "Ilocosregionen"), ("ta", "இலோகொஸ\u{bcd} பிர\u{bbe}ந\u{bcd}தியம\u{bcd}"), ("th", "เขตอ\u{e35}โลโคส"), ("tr", "Ilocos Bölgesi"), ("uk", "Ілокос"), ("ur", "ایلوکوس علاقہ"), ("vi", "Ilocos"), ("yue", "伊羅戈"), ("yue_Hans", "伊罗戈"), ("zh", "伊羅戈")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::PH,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "وادي كاغيان"), ("ca", "Vall de Cagayan"), ("ccp", "𑄇𑄬𑄉𑄬𑄃\u{11128}𑄠𑄚\u{11134} 𑄞𑄬𑄣\u{11129}"), ("ceb", "Lupot sa Cagayan"), ("de", "Cagayan Valley"), ("en", "Cagayan Valley"), ("es", "Valle del Cagayán"), ("eu", "Cagayan Harana"), ("fi", "Cagayan Valley"), ("fr", "Vallée de Cagayan"), ("hi", "कागायान घाटी"), ("hr", "Cagayan Valley"), ("id", "Lembah Cagayan"), ("it", "Valle di Cagayan"), ("ja", "カガヤン・バレー地方"), ("ko", "카가얀밸리 지방"), ("lt", "Kagajano Slėnis"), ("mk", "Кагајанска Долина"), ("ms", "Lembah Cagayan"), ("nb", "Cagayan Valley"), ("nl", "Cagayan Valley"), ("no", "Cagayan Valley"), ("pl", "Cagayan Valley"), ("ru", "Долина Кагаян"), ("sv", "Cagayandalen"), ("ta", "ககயன\u{bcd} பள\u{bcd}ளத\u{bcd}த\u{bbe}க\u{bcd}கு"), ("th", "เขตล\u{e31}มบ\u{e31}กนางคากาย\u{e31}น"), ("uk", "Долина Кагаян"), ("ur", "کاگایان وادی"), ("vi", "Thung lũng Cagayan"), ("yue", "卡加延谷"), ("yue_Hans", "卡加延谷"), ("zh", "卡加延河谷")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::PH,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ca", "Luzon Central"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11133}𑄢𑄣\u{11134} 𑄣\u{1112a}𑄎\u{11127}𑄚\u{11134}"), ("ceb", "Tunga-tungang Luzon"), ("de", "Central Luzon"), ("en", "Central Luzon"), ("es", "Luzón Central"), ("eu", "Erdialdeko Luzon"), ("fi", "Keski-Luzon"), ("fr", "Luçon centrale"), ("hi", "मध\u{94d}य ल\u{942}ज\u{93c}ोन"), ("hr", "Središnji Luzon"), ("id", "Luzon Tengah"), ("it", "Luzon Centrale"), ("ja", "中部ルソン地方"), ("ko", "중앙루손 지방"), ("lt", "Centrinis Lusonas"), ("mk", "Централен Лузон"), ("nb", "Central Luzon"), ("nl", "Central Luzon"), ("no", "Central Luzon"), ("pl", "Luzon Środkowy"), ("ru", "Центральный Лусон"), ("sv", "Centrala Luzon"), ("ta", "மத\u{bcd}திய லூசோன\u{bcd}"), ("th", "เขตก\u{e34}ตนางล\u{e39}โซน"), ("tr", "Merkez Luzon"), ("uk", "Центральний Лусон"), ("ur", "وسطی لوزون"), ("vi", "Trung Luzon"), ("yue", "中呂宋"), ("yue_Hans", "中吕宋"), ("zh", "中央吕宋")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::PH,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ca", "Regió de Bicol"), ("ccp", "𑄝\u{11128}𑄇\u{1112e}𑄣\u{11134}"), ("ceb", "Bikol"), ("de", "Bicol Region"), ("en", "Bicol"), ("es", "Bicolandia"), ("eu", "Bicolgo eskualdea"), ("fi", "Bicolin alue"), ("fr", "Bicol"), ("hi", "बिकोल क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Bicol"), ("hy", "Բիկոլի շրջան"), ("it", "Bicol"), ("ja", "ビコル地方"), ("ko", "비콜 지방"), ("lt", "Bikolo regionas"), ("mk", "Бикол"), ("nb", "Bicol Region"), ("nl", "Bicol Region"), ("no", "Bicol Region"), ("pl", "Bicol Region"), ("ru", "Бикольский Регион"), ("sv", "Bikolregionen"), ("ta", "பிகோல\u{bcd} பிர\u{bbe}ந\u{bcd}தியம\u{bcd}"), ("th", "เขตบ\u{e35}โคล"), ("uk", "Бікол"), ("ur", "بیکول علاقہ"), ("vi", "Bicol"), ("yue", "比科爾"), ("yue_Hans", "比科尔"), ("zh", "比科爾")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::PH,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بيسايا الغربية"), ("ca", "Visayas Occidentals"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄞\u{11128}𑄥𑄬𑄠𑄌\u{11134}"), ("ceb", "Kasadpang Kabisay-an"), ("de", "Western Visayas"), ("en", "Western Visayas"), ("es", "Bisayas Occidentales"), ("eu", "Mendebaldeko Bisayak"), ("fi", "Länsi-Visayas"), ("fr", "Visayas occidentales"), ("gl", "Visayas Occidental"), ("he", "מערב ויסאיאס"), ("hi", "पश\u{94d}चिमी विसाया"), ("hr", "Zapadni Visayas"), ("it", "Visayas Occidentale"), ("ja", "西ヴィサヤ地方"), ("ko", "서비사야 지방"), ("lt", "Vakarų Visajai"), ("mk", "Западни Висаи"), ("nb", "Western Visayas"), ("nl", "Western Visayas"), ("no", "Western Visayas"), ("pl", "Western Visayas"), ("ru", "Западные Висайи"), ("sv", "Västra Visayas"), ("ta", "மேற\u{bcd}கு விசய\u{bbe}சு"), ("th", "เขตค\u{e31}นล\u{e39}ร\u{e31}งคาบ\u{e35}ซายาอ\u{e31}น"), ("tr", "Batı Visayas"), ("uk", "Західні Вісаї"), ("ur", "مغربی ویسایا"), ("vi", "Tây Visayas"), ("yue", "西維薩亞斯"), ("yue_Hans", "西维萨亚斯"), ("zh", "西米沙鄢")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::PH,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بيسايا الوسطى"), ("ca", "Visayas Centrals"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11133}𑄢𑄣\u{11134} 𑄞\u{11128}𑄥𑄬𑄠𑄌\u{11134}"), ("ceb", "Tunga-tungang Kabisay-an"), ("de", "Central Visayas"), ("en", "Central Visayas"), ("es", "Bisayas Centrales"), ("eu", "Erdialdeko Bisayak"), ("fi", "Keski-Visayas"), ("fr", "Visayas centrales"), ("hi", "मध\u{94d}य विसाया"), ("hr", "Središnji Visayas"), ("hy", "Կենտրոնական Վիսարյա"), ("id", "Bisaya Tengah"), ("it", "Visayas Centrale"), ("ja", "中部ヴィサヤ地方"), ("ko", "중앙비사야 지방"), ("lt", "Centriniai Visajai"), ("mk", "Централни Висаи"), ("nl", "Central Visayas"), ("pl", "Central Visayas"), ("pt", "Visayas Centrais"), ("ru", "Центральные Висайи"), ("sv", "Centrala Visayas"), ("ta", "மத\u{bcd}திய விசய\u{bbe}சு"), ("th", "เขตก\u{e34}ตนางคาบ\u{e35}ซายาอ\u{e31}น"), ("tr", "Merkez Visayas"), ("uk", "Центральні Вісаї"), ("ur", "وسطی ویسایا"), ("vi", "Trung Visayas"), ("yue", "中維薩亞斯"), ("yue_Hans", "中维萨亚斯"), ("zh", "中米沙鄢")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "08",
                        country_alpha2: Alpha2::PH,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Усходнія Вісаі"), ("ca", "Visayas Orientals"), ("ccp", "𑄛\u{1112a}𑄇\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄞\u{11128}𑄥𑄬𑄠𑄌\u{11134}"), ("ceb", "Sidlakang Kabisay-an"), ("de", "Eastern Visayas"), ("en", "Eastern Visayas"), ("es", "Bisayas Orientales"), ("eu", "Ekialdeko Bisayak"), ("fi", "Itä-Visayas"), ("fr", "Visayas orientales"), ("he", "מזרח ויסאיאס"), ("hi", "प\u{942}र\u{94d}वी विसाया"), ("hr", "Istočni Visayas"), ("it", "Visayas Orientale"), ("ja", "東ヴィサヤ地方"), ("ko", "동비사야 지방"), ("lt", "Rytų Visajai"), ("mk", "Источни Висаи"), ("ml", "കിഴക\u{d4d}കൻ വിസ\u{d3e}യസ\u{d4d}"), ("nl", "Eastern Visayas"), ("pl", "Eastern Visayas"), ("ru", "Восточные Висайи"), ("sv", "Östra Visayas"), ("ta", "கிழக\u{bcd}கு விசய\u{bbe}சு"), ("th", "เขตซ\u{e35}ลาง\u{e31}งคาบ\u{e35}ซายาอ\u{e31}น"), ("uk", "Східні Вісаї"), ("ur", "مشرقی ویسایا"), ("vi", "Đông Visayas"), ("yue", "東維薩亞斯"), ("yue_Hans", "东维萨亚斯"), ("zh", "東米沙鄢")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "09",
                        country_alpha2: Alpha2::PH,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شبه جزيرة زامبوانجا"), ("ca", "Península de Zamboanga"), ("ccp", "𑄎𑄟\u{11134}𑄝\u{1112e}𑄠\u{11101}𑄉 𑄛𑄬𑄚\u{11128}𑄚\u{11134}𑄥\u{1112a}𑄣"), ("ceb", "Zamboanga"), ("cs", "Poloostrov Zamboanga"), ("de", "Zamboanga Peninsula"), ("en", "Zamboanga Peninsula"), ("es", "Península de Zamboanga"), ("eu", "Zamboangako penintsula"), ("fi", "Zamboangan niemimaa"), ("fr", "péninsule de Zamboanga"), ("hi", "ज\u{93c}म\u{94d}बोआ\u{902}गा प\u{94d}रायद\u{94d}वीप"), ("hr", "Poluotok Zamboanga"), ("id", "Semenanjung Zamboanga"), ("it", "Penisola di Zamboanga"), ("ja", "サンボアンガ半島地方"), ("ko", "삼보앙가 반도 지방"), ("lt", "Zamboangos pusiasalis"), ("mk", "Полуостров Замбоанга"), ("nb", "Zamboangahalvøya"), ("nl", "Zamboanga Peninsula"), ("no", "Zamboangahalvøya"), ("pl", "Zamboanga Peninsula"), ("pt", "Península de Zamboanga"), ("ru", "Полуостров Замбоанга"), ("sv", "Zamboangahalvön"), ("ta", "சம\u{bcd}பொவ\u{bbe}ங\u{bcd}க\u{bbe} த\u{bc0}பகற\u{bcd}பம\u{bcd}"), ("th", "เขตต\u{e31}งไวนางซ\u{e31}มบวงกา"), ("tr", "Zamboanga Yarımadası"), ("uk", "Півострів Замбоанга"), ("ur", "زامبوانگا جزیرہ نما"), ("vi", "Bán đảo Zamboanga"), ("yue", "三寶顏半島"), ("yue_Hans", "三宝颜半岛"), ("zh", "三宝颜半岛")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::PH,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ca", "Mindanao Septentrional"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄟\u{11128}𑄚\u{11134}𑄓𑄚𑄃\u{1112e}"), ("ceb", "Amihanang Mindanao"), ("de", "Northern Mindanao"), ("en", "Northern Mindanao"), ("es", "Mindanao del Norte"), ("eu", "Iparraldeko Mindanao"), ("fi", "Pohjois-Mindanaon alue"), ("fr", "Mindanao du Nord"), ("hi", "उत\u{94d}तरी मिन\u{94d}दनाओ"), ("hr", "Sjeverni Mindanao"), ("it", "Mindanao Settentrionale"), ("ja", "北ミンダナオ地方"), ("ko", "북민다나오 지방"), ("lt", "Šiaurinis Mindanao"), ("mk", "Северен Минданао"), ("nb", "Northern Mindanao"), ("nl", "Northern Mindanao"), ("no", "Northern Mindanao"), ("pl", "Mindanao Północne"), ("ru", "Северный Минданао"), ("sv", "Norra Mindanao"), ("ta", "வடக\u{bcd}கு மின\u{bcd}டனவு"), ("th", "เขตฮ\u{e35}ลาก\u{e31}งม\u{e34}นดาเนา"), ("uk", "Північне Мінданао"), ("ur", "شمالی مینداناؤ"), ("vi", "Bắc Mindanao"), ("yue", "北棉蘭老"), ("yue_Hans", "北棉兰老"), ("zh", "北棉兰老")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "11",
                    Subdivision{
                        name: "11",
                        country_alpha2: Alpha2::PH,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bn", "দ\u{9be}ভ\u{9be}ও অঞ\u{9cd}চল"), ("ca", "Regió de Davao"), ("ccp", "𑄓𑄞𑄃\u{1112e}"), ("ceb", "Davao"), ("de", "Davao Region"), ("en", "Davao"), ("es", "Región de Davao"), ("eu", "Davaoko eskualdea"), ("fi", "Davaon alue"), ("fr", "région de Davao"), ("hi", "दावाओ क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Davao (regija)"), ("it", "Davao"), ("ja", "ダバオ地方"), ("ko", "다바오 지방"), ("lt", "Davao regionas"), ("mk", "Давао"), ("ms", "Daerah Davao"), ("nl", "Davao Region"), ("pl", "Davao"), ("ru", "Давао (регион)"), ("sv", "Davaoregionen"), ("ta", "டவ\u{bbe}வோ பிர\u{bbe}ந\u{bcd}தியம\u{bcd}"), ("th", "เขตดาเบา"), ("tr", "Davao Bölgesi"), ("uk", "Давао"), ("ur", "داوائو علاقہ"), ("vi", "Vùng Davao"), ("yue", "達沃區"), ("yue_Hans", "达沃区"), ("zh", "达沃区")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "12",
                        country_alpha2: Alpha2::PH,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ca", "SOCCSKSARGEN"), ("ccp", "𑄥\u{11127}𑄌\u{11134}𑄥𑄢\u{11134}𑄉𑄬𑄚\u{11134}"), ("ceb", "SOCCSKSARGEN"), ("cs", "Soccsksargen"), ("de", "SOCCSKSARGEN"), ("en", "Soccsksargen"), ("es", "Región XII"), ("eu", "Soccsksargen"), ("fi", "SOCCSKSARGEN"), ("fr", "SOCCSKSARGEN"), ("hi", "सोकसारज\u{947}न"), ("hr", "SOCCSKSARGEN"), ("it", "SOCCSKSARGEN"), ("ja", "ソクサージェン地方"), ("ko", "소크사르젠 지방"), ("lt", "SOCCSKSARGEN"), ("mk", "Сокксксархен"), ("nl", "SOCCSKSARGEN"), ("pl", "SOCCSKSARGEN"), ("ru", "СОККСКСАРХЕН"), ("sv", "SOCCSKSARGEN"), ("ta", "சொக\u{bcd}ஸ\u{bcd}சர\u{bcd}ஜென\u{bcd}"), ("th", "เขตโซกซาร\u{e4c}เจน"), ("uk", "Сокксксарген"), ("ur", "سوکسارگین"), ("vi", "SOCCSKSARGEN"), ("yue", "中棉蘭老"), ("yue_Hans", "中棉兰老")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "13",
                        country_alpha2: Alpha2::PH,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ca", "Caraga"), ("ccp", "𑄥𑄢\u{11134}𑄉"), ("ceb", "Caraga"), ("de", "Caraga"), ("en", "Caraga"), ("es", "Caraga"), ("eu", "Caraga"), ("fi", "Caraga"), ("fr", "Caraga"), ("hi", "कारागा"), ("hr", "Caraga"), ("it", "Caraga"), ("ja", "カラガ地方"), ("ko", "카라가 지방"), ("lt", "Karaga"), ("mk", "Карага"), ("nb", "Caraga"), ("nl", "Caraga"), ("no", "Caraga"), ("pl", "Caraga"), ("ru", "Карага"), ("sv", "Caraga"), ("ta", "கரக\u{bbe}"), ("th", "เขตคารากา"), ("uk", "Карага"), ("ur", "کاراگا"), ("vi", "Caraga"), ("yue", "卡拉加"), ("yue_Hans", "卡拉加"), ("zh", "卡拉加区")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::PH,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Müsəlman Mindanao muxtar regionu"), ("be", "Аўтаномны рэгіён у Мусульманскім Мінданаа"), ("ca", "Regió Autònoma del Mindanao Musulmà"), ("ccp", "𑄟\u{1112a}𑄌\u{11134}𑄣\u{11128}𑄟\u{11134} 𑄟\u{11128}𑄚\u{11134}𑄓𑄚𑄃\u{1112e}"), ("ceb", "Mindanawng Muslim"), ("cs", "Autonomní region Muslimské Mindanao"), ("de", "Autonomous Region in Muslim Mindanao"), ("en", "Muslim Mindanao"), ("es", "Mindanao Musulmán"), ("eu", "Mindanao Musulmaneko Eskualde Autonomoa"), ("fa", "ناحیه خودمختار مسلمانان میندانائو"), ("fi", "Mindanaon autonominen alue"), ("fr", "région autonome en Mindanao musulmane"), ("gl", "Rexión Autónoma do Mindanao Musulmán"), ("hi", "म\u{941}स\u{94d}लिम मिन\u{94d}दनाओ म\u{947}\u{902} स\u{94d}वशासित क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Autonomna regija Muslimanski Mindanao"), ("id", "Region Otonomi Muslim Mindanao"), ("it", "regione autonoma nel Mindanao Musulmano"), ("ja", "イスラム教徒ミンダナオ自治地域"), ("ko", "무슬림 민다나오 자치구"), ("lt", "Mindanao musulmonų autonominis regionas"), ("mk", "Автономен регион во Муслимански Минданао"), ("ms", "Wilayah Autonomi Islam Mindanao"), ("nb", "Autonomous Region in Muslim Mindanao"), ("nl", "Autonomous Region in Muslim Mindanao"), ("no", "Autonomous Region in Muslim Mindanao"), ("pl", "Muzułmańskie Mindanao"), ("pt", "Região Autónoma do Mindanau Muçulmano"), ("ru", "Автономный регион в Мусульманском Минданао"), ("sr", "Муслимански Минданао"), ("sr_Latn", "Muslimanski Mindanao"), ("sv", "Muslimska Mindanao"), ("ta", "முசுலிம\u{bcd} மிண\u{bcd}டன\u{bbe}வோ தன\u{bcd}ன\u{bbe}ட\u{bcd}சிப\u{bcd} பகுதி"), ("th", "เขตปกครองตนเองในม\u{e34}นดาเนาม\u{e38}สล\u{e34}ม"), ("tr", "Müslüman Mindanao Özerk Bölgesi"), ("uk", "Автономний регіон в Мусульманському Мінданао"), ("ur", "مسلم مینداناؤ کا خود مختار علاقہ"), ("vi", "Khu tự trị Hồi giáo Mindanao"), ("yue", "棉蘭老穆斯林自治區"), ("yue_Hans", "棉兰老穆斯林自治区"), ("zh", "棉兰老穆斯林自治区")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "15",
                    Subdivision{
                        name: "15",
                        country_alpha2: Alpha2::PH,
                        code: "15",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ca", "Regió Administrativa de la Cordillera"), ("ccp", "𑄇\u{11127}𑄢\u{11134}𑄓\u{11128}𑄣𑄬𑄢 𑄃𑄬𑄖\u{11134}𑄟\u{11128}𑄚\u{11128}𑄌\u{11134}𑄑\u{11133}𑄢𑄑\u{11128}𑄛\u{11134}"), ("ceb", "Administratibong Rehiyon sa Cordillera"), ("de", "Cordillera Administrative Region"), ("en", "Cordillera Administrative"), ("es", "La Cordillera"), ("eu", "Cordillerako Eskualde Administratiboa"), ("fi", "Cordilleran hallinnollinen alue"), ("fr", "région administrative de la Cordillère"), ("hi", "कोर\u{94d}दिल\u{94d}य\u{947}रा प\u{94d}रशासनिक क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Administrativna regija Cordillera"), ("it", "Regione amministrativa Cordillera"), ("ja", "コルディリェラ行政地域"), ("ko", "코르디예라 행정구"), ("lt", "Kordiljeros administracinis regionas"), ("mk", "Кордилјера"), ("nb", "Cordillera Administrative Region"), ("nl", "Cordillera Administrative Region"), ("no", "Cordillera Administrative Region"), ("pl", "Cordillera Administrative Region"), ("pt", "Região Administrativa de Cordillera"), ("ru", "Кордильерский административный регион"), ("sv", "Kordiljärernas administrativa region"), ("ta", "கோர\u{bcd}டில\u{bcd}லெர\u{bbe} நிர\u{bcd}வ\u{bbe}கப\u{bcd} பிர\u{bbe}ந\u{bcd}தியம\u{bcd}"), ("th", "เขตบร\u{e34}หารคอร\u{e4c}ด\u{e34}ลเยรา"), ("uk", "Кордильєрський адміністративний регіон"), ("ur", "کوردیلیرا انتظامی علاقہ"), ("vi", "Vùng Hành chính Cordillera"), ("yue", "科迪勒拉"), ("yue_Hans", "科迪勒拉"), ("zh", "科迪勒拉行政區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "40",
                    Subdivision{
                        name: "40",
                        country_alpha2: Alpha2::PH,
                        code: "40",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ca", "CALABARZON"), ("ccp", "𑄇𑄣𑄝𑄢\u{11134}𑄎\u{11127}𑄚\u{11134}"), ("ceb", "CALABARZON"), ("de", "Calabarzon"), ("en", "Calabarzon"), ("es", "Región IV-A"), ("eu", "Calabarzon"), ("fi", "Calabarzon"), ("fr", "Calabarzon"), ("hi", "कालाबारज\u{93c}ोन"), ("hr", "CALABARZON"), ("id", "CALABARZON"), ("it", "CALABARZON"), ("ja", "カラバルソン地方"), ("ko", "칼라바르손 지방"), ("lt", "CALABARZON"), ("mk", "Калабарзон"), ("ms", "CALABARZON"), ("nb", "Calabarzon"), ("nl", "Calabarzon"), ("no", "Calabarzon"), ("pl", "CALABARZON"), ("ru", "КАЛАБАРСОН"), ("sv", "CALABARZON"), ("ta", "கலபர\u{bcd}சொன\u{bcd}"), ("th", "เขตคาลาบาร\u{e4c}โซน"), ("tr", "CALABARZON"), ("uk", "Калабарсон"), ("ur", "کالابارزون"), ("vi", "CALABARZON"), ("yue", "卡拉巴鬆"), ("yue_Hans", "卡拉巴松"), ("zh", "卡拉巴松")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "41",
                    Subdivision{
                        name: "41",
                        country_alpha2: Alpha2::PH,
                        code: "41",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ميماروبا"), ("ca", "MIMAROPA"), ("ccp", "𑄟\u{11128}𑄟𑄢\u{1112e}𑄛"), ("ceb", "MIMAROPA"), ("de", "MIMAROPA"), ("en", "Mimaropa"), ("es", "Región IV-B"), ("eu", "Mimaropa"), ("fi", "Mimaropa"), ("fr", "MIMAROPA"), ("hi", "मिमारोपा"), ("hr", "MIMAROPA"), ("it", "MIMARO"), ("ja", "ミマロパ地方"), ("ko", "미마로파 지방"), ("lt", "MIMAROPA"), ("mk", "Мимаропа"), ("nb", "MIMAROPA"), ("nl", "MIMAROPA"), ("no", "MIMAROPA"), ("pl", "MIMAROPA"), ("ru", "МИМАРОПА"), ("sv", "MIMAROPA"), ("ta", "மிமரோப\u{bbe}"), ("th", "เขตม\u{e35}มาโรปา"), ("uk", "Мімаропа"), ("ur", "میماپورا"), ("vi", "MIMAROPA"), ("yue", "民馬羅巴"), ("yue_Hans", "民马罗巴"), ("zh", "民马罗巴区")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "ABR",
                    Subdivision{
                        name: "ABR",
                        country_alpha2: Alpha2::PH,
                        code: "ABR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.5951122), longitude: Some(120.7982528), max_latitude: Some(17.979682), min_latitude: Some(17.154255), max_longitude: Some(121.124898), min_longitude: Some(120.463118)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبرا"), ("bn", "আব\u{9cd}র\u{9be}"), ("ccp", "𑄃𑄝\u{11133}𑄢"), ("ceb", "Abra"), ("da", "Abra"), ("de", "Abra"), ("el", "Άμπρα"), ("en", "Abra"), ("es", "Abra"), ("fa", "آبرا"), ("fi", "Abra"), ("fr", "Abra"), ("gl", "Abra, Filipinas"), ("gu", "અબ\u{acd}રા"), ("hi", "आब\u{94d}रा प\u{94d}रान\u{94d}त"), ("id", "Abra"), ("it", "Provincia di Abra"), ("ja", "アブラ州"), ("kn", "ಅಬ\u{ccd}ರಾ"), ("ko", "아브라 주"), ("lt", "Abra"), ("lv", "Abra"), ("mk", "Абра"), ("mr", "अब\u{94d}रा"), ("ms", "Abra"), ("nb", "Abra"), ("nl", "Abra"), ("no", "Abra"), ("pl", "Abra"), ("pt", "Abra (província)"), ("ru", "Абра"), ("si", "අබ\u{dca}\u{200d}ර\u{dcf}"), ("sv", "Abra"), ("ta", "அப\u{bcd}ர"), ("te", "అబ\u{c4d}ర\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดอาบรา"), ("tr", "Abra"), ("uk", "Провінція Абра"), ("ur", "ابرا (صوبہ)"), ("vi", "Abra"), ("zh", "阿布拉省")]),
                        unofficial_name_list: ["Abra"].to_vec(),
                    }
                ),
                (
                    "AGN",
                    Subdivision{
                        name: "AGN",
                        country_alpha2: Alpha2::PH,
                        code: "AGN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.9456259), longitude: Some(125.5319234), max_latitude: Some(9.4591), min_latitude: Some(8.6575759), max_longitude: Some(125.7669111), min_longitude: Some(125.2066434)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أغوسان ديل نورت"), ("bn", "আগ\u{9c1}স\u{9be}ন ডেল নর\u{9cd}থে"), ("ca", "Agusan del Norte"), ("ccp", "𑄃𑄉\u{1112a}𑄥𑄚\u{11134} 𑄓𑄬𑄣\u{11134} 𑄚\u{11127}𑄢\u{11134}𑄑𑄬"), ("ceb", "Agusan del Norte"), ("da", "Agusan del Norte"), ("de", "Agusan del Norte"), ("el", "Αγκουσάν ντελ Νόρτε"), ("en", "Agusan del Norte"), ("es", "Agusan del Norte"), ("fa", "آگوسان شمالی"), ("fi", "Agusan del Norte"), ("fr", "Agusan du Nord"), ("gu", "એગ\u{acd}ય\u{ac1}સન ડ\u{ac7}લ નોર\u{acd}ટ"), ("hi", "आग\u{941}सान द\u{947}ल नोर\u{94d}त\u{947}"), ("id", "Agusan del Norte"), ("it", "Provincia di Agusan del Norte"), ("ja", "北アグサン州"), ("kn", "ಅಗುಸನ\u{ccd} ಡ\u{cc6}ರ\u{ccd} ನಾರ\u{ccd}ಟ\u{cc6}"), ("ko", "북아구산 주"), ("lt", "Šiaurės Agusanas"), ("lv", "Agusana del Norte"), ("mk", "Северен Агусан"), ("mr", "अग\u{941}सन ड\u{947}ल नॉर\u{94d}ट"), ("ms", "Agusan del Norte"), ("nb", "Agusan del Norte"), ("nl", "Agusan del Norte"), ("no", "Agusan del Norte"), ("pl", "Agusan del Norte"), ("pt", "Agusão do Norte"), ("ru", "Северный Агусан"), ("si", "අග\u{dd4}ස\u{dcf}න\u{dca} ඩෙල\u{dca} නොර\u{dca}ටේ"), ("sv", "Agusan del Norte"), ("ta", "அங\u{bcd}குசன\u{bcd} டெல\u{bcd} நோர\u{bcd}ட\u{bcd}"), ("te", "అగూసన\u{c4d} డ\u{c46}ల\u{c4d} న\u{c3e}ర\u{c4d}ట\u{c4d}"), ("th", "อก\u{e39}ซาน เดล นอร\u{e4c}เต\u{e49}"), ("tr", "Agusan del Norte"), ("uk", "Північний Агусан"), ("ur", "اگوسان شمالی"), ("vi", "Agusan del Norte"), ("zh", "北阿古桑省")]),
                        unofficial_name_list: ["Agusan del Norte"].to_vec(),
                    }
                ),
                (
                    "AGS",
                    Subdivision{
                        name: "AGS",
                        country_alpha2: Alpha2::PH,
                        code: "AGS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.6051665), longitude: Some(125.916739), max_latitude: Some(9.231869), min_latitude: Some(7.936757), max_longitude: Some(126.361955), min_longitude: Some(125.239547)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أغوسان ديل سور"), ("bn", "আগ\u{9c1}স\u{9be}ন দেল স\u{9c1}র"), ("ccp", "𑄃𑄉\u{1112a}𑄥𑄚\u{11134} 𑄓𑄬𑄣\u{11134} 𑄥𑄢\u{11134}"), ("ceb", "Agusan del Sur"), ("da", "Agusan del Sur"), ("de", "Agusan del Sur"), ("el", "Αγκουσάν ντελ Σουρ"), ("en", "Agusan del Sur"), ("es", "Agusan del Sur"), ("fa", "آگوسان جنوبی"), ("fi", "Agusan del Sur"), ("fr", "Agusan du Sud"), ("gu", "અગ\u{ac1}સન ડ\u{ac7}લ સ\u{ac1}ર"), ("hi", "आग\u{941}सान द\u{947}ल स\u{942}र"), ("id", "Agusan del Sur"), ("it", "Provincia di Agusan del Sur"), ("ja", "南アグサン州"), ("kn", "ಅಗುಸನ\u{ccd} ಡ\u{cc6}ಲ\u{ccd} ಸುರ\u{ccd}"), ("ko", "남아구산 주"), ("lt", "Pietų Agusanas"), ("lv", "Agusana del Sura"), ("mk", "Јужен Агусан"), ("mr", "अग\u{941}सन ड\u{947}ल स\u{941}र"), ("ms", "Agusan del Sur"), ("nb", "Agusan del Sur"), ("nl", "Agusan del Sur"), ("no", "Agusan del Sur"), ("pl", "Agusan del Sur"), ("pt", "Augusan do Sur"), ("ru", "Южный Агусан"), ("si", "අග\u{dd4}ස\u{dcf}න\u{dca} ඩෙල\u{dca} සර\u{dca}"), ("sv", "Agusan del Sur"), ("ta", "அங\u{bcd}குசன\u{bcd} டெல\u{bcd} சூர\u{bcd}"), ("te", "అగూసన\u{c4d} డ\u{c46}ల\u{c4d} సుర\u{c4d}"), ("th", "อก\u{e39}ซาน เดล เซอ"), ("tr", "Agusan del Sur"), ("uk", "Південний Агусан"), ("ur", "اگوسان جنوبی"), ("vi", "Agusan del Sur"), ("zh", "南阿古桑省")]),
                        unofficial_name_list: ["Agusan del Sur"].to_vec(),
                    }
                ),
                (
                    "AKL",
                    Subdivision{
                        name: "AKL",
                        country_alpha2: Alpha2::PH,
                        code: "AKL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.8166109), longitude: Some(122.0941541), max_latitude: Some(11.9994645), min_latitude: Some(11.311094), max_longitude: Some(122.57729), min_longitude: Some(121.848633)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية أكلان"), ("bn", "আক\u{9be}\u{9cd}ল\u{9be}ন"), ("bs", "Aklan"), ("ca", "Aklan"), ("ccp", "𑄃𑄇\u{11134}𑄣𑄚\u{11134}"), ("ceb", "Aklan"), ("da", "Aklan"), ("de", "Aklan"), ("el", "Ακλάν"), ("en", "Aklan"), ("es", "Aklan"), ("eu", "Aklan"), ("fa", "آکلان"), ("fi", "Aklan"), ("fr", "province de Aklan"), ("gu", "અક\u{acd}લાન"), ("hi", "अक\u{94d}लान प\u{94d}रान\u{94d}त"), ("id", "Aklan"), ("it", "Provincia di Aklan"), ("ja", "アクラン州"), ("kn", "ಅಕ\u{ccd}ಲಾನ\u{ccd}"), ("ko", "아클란 주"), ("lt", "Aklanas"), ("lv", "Aklana"), ("mk", "Аклан"), ("mr", "अकालान"), ("ms", "Aklan"), ("nb", "Aklan"), ("nl", "Aklan"), ("no", "Aklan"), ("pl", "Aklan"), ("pt", "Aklan"), ("ru", "Аклан"), ("si", "අක\u{dca}ලන\u{dca}"), ("sv", "Aklan"), ("ta", "அகிலன\u{bcd}"), ("te", "అక\u{c4d}ల\u{c3e}న\u{c4d}"), ("th", "อ\u{e31}คแลน"), ("tr", "Aklan"), ("uk", "Аклан"), ("ur", "اکلان"), ("vi", "Aklan"), ("zh", "阿克兰省")]),
                        unofficial_name_list: ["Aklan"].to_vec(),
                    }
                ),
                (
                    "ALB",
                    Subdivision{
                        name: "ALB",
                        country_alpha2: Alpha2::PH,
                        code: "ALB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.1774827), longitude: Some(123.5280072), max_latitude: Some(13.525534), min_latitude: Some(12.986531), max_longitude: Some(124.220096), min_longitude: Some(123.2739775)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ألباي"), ("bn", "আল\u{9cd}বে"), ("ca", "Albay"), ("ccp", "𑄃𑄣\u{11134}𑄝𑄬"), ("ceb", "Albay"), ("da", "Albay"), ("de", "Albay"), ("el", "Αλμπέι"), ("en", "Albay"), ("es", "Albay"), ("fa", "آلبای"), ("fi", "Albay"), ("fr", "Albay"), ("gu", "આલ\u{acd}બ\u{ac7}"), ("hi", "अल\u{94d}बाय प\u{94d}रान\u{94d}त"), ("hy", "Ալբայ"), ("id", "Albay"), ("it", "Provincia di Albay"), ("ja", "アルバイ州"), ("kn", "ಅಲ\u{ccd}ಬೇ"), ("ko", "알바이 주"), ("lt", "Albajus"), ("lv", "Albaja"), ("mk", "Албај"), ("mr", "अल\u{94d}बाय"), ("ms", "Albay"), ("nb", "Albay"), ("nl", "Albay"), ("no", "Albay"), ("pl", "Albay"), ("pt", "Albay"), ("ru", "Албай"), ("si", "අල\u{dca}බේ"), ("sv", "Albay"), ("ta", "அலிப\u{bbe}ய\u{bcd}"), ("te", "ఆల\u{c4d}బ\u{c47}"), ("th", "อ\u{e31}ลเบย\u{e4c}"), ("tr", "Albay"), ("uk", "Албай"), ("ur", "البائی"), ("vi", "Albay"), ("zh", "阿尔拜省")]),
                        unofficial_name_list: ["Albay"].to_vec(),
                    }
                ),
                (
                    "ANT",
                    Subdivision{
                        name: "ANT",
                        country_alpha2: Alpha2::PH,
                        code: "ANT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.380579), longitude: Some(122.0635005), max_latitude: Some(12.117357), min_latitude: Some(10.4172679), max_longitude: Some(122.325029), min_longitude: Some(121.2828827)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "آنتيك"), ("bn", "এন\u{9cd}টিক"), ("ca", "Antique"), ("ccp", "𑄃𑄚\u{11134}𑄑\u{11128}𑄇\u{11134}"), ("ceb", "Antique"), ("da", "Antique"), ("de", "Antique"), ("el", "Αντίκ"), ("en", "Antique"), ("es", "Antique"), ("fa", "آنتیک"), ("fi", "Antique"), ("fr", "Antique"), ("gu", "એન\u{acd}ટિક"), ("hi", "आन\u{94d}तीक\u{947} प\u{94d}रान\u{94d}त"), ("id", "Antique"), ("it", "Provincia di Antique"), ("ja", "アンティーケ州"), ("kn", "ಆಂಟ\u{cbf}ಕ\u{ccd}"), ("ko", "안티케 주"), ("lt", "Antikė"), ("lv", "Antike"), ("mk", "Антике"), ("mr", "अ\u{901}टिक"), ("ms", "Wilayah Antique"), ("nb", "Antique"), ("nl", "Antique"), ("no", "Antique"), ("pl", "Antique"), ("pt", "Antigo"), ("ru", "Антике"), ("si", "ඇන\u{dca}ට\u{dd2}ක\u{dca}"), ("sv", "Antique"), ("ta", "ஆன\u{bcd}டியூ"), ("te", "య\u{c3e}ంట\u{c3f}క\u{c4d}"), ("th", "แอนต\u{e34}ค"), ("tr", "Antique"), ("uk", "Антік"), ("ur", "اینٹیک (صوبہ)"), ("vi", "Antique"), ("zh", "安蒂克省")]),
                        unofficial_name_list: ["Antique"].to_vec(),
                    }
                ),
                (
                    "APA",
                    Subdivision{
                        name: "APA",
                        country_alpha2: Alpha2::PH,
                        code: "APA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(18.0120304), longitude: Some(121.1710389), max_latitude: Some(18.541674), min_latitude: Some(17.623744), max_longitude: Some(121.4897879), min_longitude: Some(120.9254531)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "آبايو"), ("bn", "আপ\u{9be}য\u{9bc}\u{9be}ও"), ("ca", "Apayao"), ("ccp", "𑄃𑄛𑄬𑄠𑄃\u{1112e}"), ("ceb", "Apayao"), ("da", "Apayao"), ("de", "Apayao"), ("el", "Απαγιάο"), ("en", "Apayao"), ("es", "Apayao"), ("fa", "آپایو"), ("fi", "Apayao"), ("fr", "province de Apayao"), ("gu", "અપાયાઓ"), ("hi", "आपायाओ प\u{94d}रान\u{94d}त"), ("id", "Apayao"), ("it", "Provincia di Apayao"), ("ja", "アパヤオ州"), ("kn", "ಅಪಾಯಾವೊ"), ("ko", "아파야오 주"), ("lt", "Apajao"), ("lv", "Apajao"), ("mk", "Апајао"), ("mr", "अपॉईओ"), ("ms", "Apayao"), ("nb", "Apayao"), ("nl", "Apayao"), ("no", "Apayao"), ("pl", "Apayao"), ("pt", "Apayao"), ("ru", "Апаяо"), ("si", "අපය\u{dcf}වෝ"), ("sv", "Apayao"), ("ta", "அப\u{bbe}யவ\u{bcd}"), ("te", "అప\u{c3e}య\u{c47}వ\u{c4b}"), ("th", "อพาเยา"), ("tr", "Apayao"), ("uk", "Апаяо"), ("ur", "اپایاؤ"), ("vi", "Apayao"), ("zh", "阿巴尧省")]),
                        unofficial_name_list: ["Apayao"].to_vec(),
                    }
                ),
                (
                    "AUR",
                    Subdivision{
                        name: "AUR",
                        country_alpha2: Alpha2::PH,
                        code: "AUR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.991944), longitude: Some(121.636944), max_latitude: Some(17.0173518), min_latitude: Some(16.9357138), max_longitude: Some(121.7060828), min_longitude: Some(121.5742779)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أورورا"), ("bn", "অরোর\u{9be}"), ("ca", "Aurora"), ("ccp", "𑄃\u{11127}𑄅\u{1112a}𑄢\u{1112e}𑄢"), ("ceb", "Aurora"), ("da", "Aurora"), ("de", "Aurora"), ("el", "Ορόρα"), ("en", "Aurora"), ("es", "Aurora"), ("fa", "آرورا"), ("fi", "Aurora"), ("fr", "Aurora"), ("gu", "ઓરોરા"), ("hi", "औरोरा प\u{94d}रान\u{94d}त"), ("id", "Aurora"), ("is", "Aurora"), ("it", "Provincia di Aurora"), ("ja", "アウロラ州"), ("kn", "ಅರೋರಾ"), ("ko", "아우로라 주"), ("lt", "Aurora"), ("lv", "Aurora"), ("mk", "Аурора"), ("mr", "अरोरा"), ("ms", "Aurora"), ("nb", "Aurora"), ("nl", "Aurora"), ("no", "Aurora"), ("pl", "Aurora"), ("pt", "Aurora"), ("ru", "Аурора"), ("si", "අව\u{dd4}රෝර\u{dcf}"), ("sv", "Aurora"), ("ta", "அரோர\u{bbe}"), ("te", "అర\u{c4b}ర\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดออโรรา"), ("tr", "Aurora"), ("uk", "Провінція Аврора"), ("ur", "آرورا (صوبہ)"), ("vi", "Aurora"), ("zh", "奥罗拉省")]),
                        unofficial_name_list: ["Aurora"].to_vec(),
                    }
                ),
                (
                    "BAN",
                    Subdivision{
                        name: "BAN",
                        country_alpha2: Alpha2::PH,
                        code: "BAN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.6416842), longitude: Some(120.4818446), max_latitude: Some(14.9283039), min_latitude: Some(14.3725568), max_longitude: Some(120.6115616), min_longitude: Some(120.2426195)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "باتان"), ("bn", "ব\u{9be}ত\u{9be}ন"), ("ca", "Península de Bataan"), ("ccp", "𑄝𑄑\u{11133}𑄃𑄚\u{11134}"), ("ceb", "Bataan"), ("da", "Bataan"), ("de", "Bataan"), ("el", "Μπαταάν"), ("en", "Bataan"), ("es", "Bataán"), ("fa", "باتاآن"), ("fi", "Bataan"), ("fr", "Bataan"), ("gu", "બટાન"), ("hi", "बाताआन प\u{94d}रान\u{94d}त"), ("id", "Bataan"), ("it", "Provincia di Bataan"), ("ja", "バターン州"), ("kn", "ಬಟಾನ\u{ccd}"), ("ko", "바탄 주"), ("lt", "Bataanas"), ("lv", "Bataana"), ("mk", "Батаан"), ("mr", "बातन"), ("ms", "Bataan"), ("nb", "Bataan"), ("nl", "Bataan"), ("no", "Bataan"), ("pl", "Bataan"), ("pt", "Bataan"), ("ru", "Батаан"), ("si", "බට\u{dcf}න\u{dca}"), ("sv", "Bataan"), ("ta", "பட\u{bcd}ட\u{bbe}ன\u{bcd}"), ("te", "బట\u{c3e}న\u{c4d}"), ("th", "บาตาน"), ("tr", "Bataan"), ("uk", "Батаан"), ("ur", "باتان"), ("vi", "Bataan"), ("zh", "巴丹省")]),
                        unofficial_name_list: ["Bataan"].to_vec(),
                    }
                ),
                (
                    "BAS",
                    Subdivision{
                        name: "BAS",
                        country_alpha2: Alpha2::PH,
                        code: "BAS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.4296349), longitude: Some(121.9870165), max_latitude: Some(6.906267000000001), min_latitude: Some(6.2788594), max_longitude: Some(122.3269363), min_longitude: Some(121.4296532)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "باسيلان"), ("bn", "ব\u{9cd}য\u{9be}সিল\u{9be}ন"), ("ca", "Basilan"), ("ccp", "𑄝𑄥\u{11128}𑄣\u{11127}𑄚\u{11134}"), ("ceb", "Basilan"), ("cs", "Basilan"), ("da", "Basilan"), ("de", "Basilan"), ("el", "Μπασιλάν"), ("en", "Basilan"), ("es", "Basilán"), ("fa", "باسیلان"), ("fi", "Basilan"), ("fr", "Basilan"), ("gl", "Basilán"), ("gu", "બ\u{ac7}સિલન"), ("hi", "बसीलन"), ("id", "Basilan"), ("it", "Provincia di Basilan"), ("ja", "バシラン州"), ("kn", "ಬಸ\u{cbf}ಲಾನ\u{ccd}"), ("ko", "바실란 주"), ("lt", "Basilanas"), ("lv", "Basilana"), ("mk", "Басилан"), ("mr", "ब\u{947}सिल\u{945}न"), ("ms", "Basilan"), ("nb", "Basilan"), ("nl", "Basilan"), ("no", "Basilan"), ("pl", "Basilan"), ("pt", "Província de Basilan"), ("ru", "Басилан"), ("si", "බැස\u{dd2}ලන\u{dca}"), ("sv", "Basilan"), ("ta", "பச\u{bc0}லன\u{bcd}"), ("te", "బ\u{c3e}స\u{c3f}ల\u{c3e}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบาซ\u{e35}ล\u{e31}น"), ("tr", "Basilan"), ("uk", "Басілан"), ("ur", "باسیلان"), ("vi", "Basilan"), ("zh", "巴西兰省")]),
                        unofficial_name_list: ["Basilan"].to_vec(),
                    }
                ),
                (
                    "BEN",
                    Subdivision{
                        name: "BEN",
                        country_alpha2: Alpha2::PH,
                        code: "BEN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.5577257), longitude: Some(120.8039474), max_latitude: Some(16.9382421), min_latitude: Some(16.186448), max_longitude: Some(120.904165), min_longitude: Some(120.468057)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بينغويت"), ("bn", "বেনক\u{9c1}য\u{9bc}েট"), ("ca", "Benguet"), ("ccp", "𑄝𑄬𑄚\u{11134}𑄉\u{1112a}𑄠𑄬𑄖\u{11134}"), ("ceb", "Benguet"), ("da", "Benguet"), ("de", "Benguet"), ("el", "Μπενγκέτ"), ("en", "Benguet"), ("es", "Benguet"), ("fa", "بنگوئه"), ("fi", "Benguet"), ("fr", "province de Benguet"), ("gu", "બ\u{ac7}ન\u{acd}ક\u{acd}વ\u{ac7}ટ"), ("hi", "ब\u{947}\u{902}ग\u{947}त प\u{94d}रान\u{94d}त"), ("id", "Benguet"), ("it", "provincia di Benguet"), ("ja", "ベンゲット州"), ("kn", "ಬ\u{cc6}ಂಗ\u{ccd}ವ\u{cc6}ಟ\u{ccd}"), ("ko", "벵게트 주"), ("lt", "Bengetas"), ("lv", "Bengveta"), ("mk", "Бенгет"), ("mr", "ब\u{947}नक\u{94d}व\u{947}ट"), ("ms", "Benguet"), ("nb", "Benguet"), ("nl", "Benguet"), ("no", "Benguet"), ("pl", "Benguet"), ("pt", "Província de Benguet"), ("ru", "Бенгет"), ("si", "බෙන\u{dca}ග\u{dd4}වට\u{dca}"), ("sv", "Benguet"), ("ta", "பென\u{bcd}குட\u{bcd}"), ("te", "బ\u{c46}ంగ\u{c4d}వ\u{c46}ట\u{c4d}"), ("th", "เบงเกวต"), ("tr", "Benguet"), ("uk", "Бенґет"), ("ur", "بینگیت"), ("vi", "Benguet"), ("zh", "本格特省")]),
                        unofficial_name_list: ["Benguet"].to_vec(),
                    }
                ),
                (
                    "BIL",
                    Subdivision{
                        name: "BIL",
                        country_alpha2: Alpha2::PH,
                        code: "BIL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.466667), longitude: Some(124.483333), max_latitude: Some(11.5656392), min_latitude: Some(11.4556383), max_longitude: Some(124.5410156), min_longitude: Some(124.4320654)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بيليران"), ("az", "Biliran adası"), ("bn", "বিলির\u{9be}ন"), ("ca", "Biliran"), ("ccp", "𑄝\u{11128}𑄣\u{11128}𑄢𑄚\u{11134}"), ("ceb", "Biliran"), ("da", "Biliran"), ("de", "Biliran"), ("el", "Μπιλιράν"), ("en", "Biliran"), ("es", "Bilirán"), ("fa", "بیلیران"), ("fi", "Biliran"), ("fr", "province de Biliran"), ("gl", "Biliran"), ("gu", "બીલીરન"), ("he", "ביליראן"), ("hi", "बिलिरान"), ("id", "Biliran"), ("it", "Provincia di Biliran"), ("ja", "ビリラン州"), ("kn", "ಬ\u{cbf}ಲ\u{cbf}ರಾನ\u{ccd}"), ("ko", "빌리란 주"), ("lt", "Biliranas"), ("lv", "Bilirana"), ("mk", "Билиран"), ("mr", "बिलीरान"), ("ms", "Biliran"), ("nb", "Biliran"), ("nl", "Biliran"), ("no", "Biliran"), ("pl", "Biliran"), ("pt", "Biliran"), ("ru", "Билиран"), ("si", "බ\u{dd2}ල\u{dd2}රන\u{dca}"), ("sv", "Biliran"), ("ta", "பிளிரன\u{bcd}"), ("te", "బ\u{c3f}ల\u{c3f}ర\u{c3e}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e35}ล\u{e35}ร\u{e31}น"), ("tr", "Biliran"), ("uk", "Біліран"), ("ur", "بیلیران"), ("vi", "Biliran"), ("zh", "比利兰省")]),
                        unofficial_name_list: ["Biliran"].to_vec(),
                    }
                ),
                (
                    "BOH",
                    Subdivision{
                        name: "BOH",
                        country_alpha2: Alpha2::PH,
                        code: "BOH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.849991099999999), longitude: Some(124.1435427), max_latitude: Some(10.2562504), min_latitude: Some(9.4856913), max_longitude: Some(124.6197582), min_longitude: Some(123.719351)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بوهول"), ("az", "Bohol"), ("bn", "বোহোল"), ("ca", "Bohol"), ("ccp", "𑄝𑄦\u{1112a}𑄣\u{11134}"), ("ceb", "Bohol"), ("cs", "Bohol"), ("da", "Bohol"), ("de", "Bohol"), ("el", "Μποχόλ"), ("en", "Bohol"), ("es", "Bohol"), ("et", "Bohol"), ("eu", "Bohol"), ("fa", "بهل"), ("fi", "Bohol"), ("fr", "Bohol"), ("gl", "Bohol"), ("gu", "બોહોલ"), ("hi", "बोहोल"), ("hu", "Bohol"), ("id", "Bohol"), ("it", "Provincia di Bohol"), ("ja", "ボホール州"), ("jv", "Bohol"), ("kn", "ಬೊಹೋಲ\u{ccd}"), ("ko", "보홀 주"), ("lt", "Boholis"), ("lv", "Bohola"), ("mk", "Бохол"), ("ml", "ബൊഹോൾ"), ("mr", "बोहोल"), ("ms", "Bohol"), ("nb", "Bohol"), ("ne", "बोहोल"), ("nl", "Bohol"), ("no", "Bohol"), ("pl", "Bohol"), ("pt", "Bohol"), ("ru", "Провинция Бохоль"), ("si", "බොහොල\u{dca}"), ("sr", "Бохол"), ("sr_Latn", "Bohol"), ("sv", "Bohol"), ("ta", "போஹோல\u{bcd}"), ("te", "బ\u{c4b}హ\u{c4b}ల\u{c4d}"), ("th", "โบฮอล"), ("tr", "Bohol"), ("uk", "Бохол, Бохоль"), ("ur", "بوہول"), ("vi", "Bohol"), ("zh", "保和省")]),
                        unofficial_name_list: ["Bohol"].to_vec(),
                    }
                ),
                (
                    "BTG",
                    Subdivision{
                        name: "BTG",
                        country_alpha2: Alpha2::PH,
                        code: "BTG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.7564651), longitude: Some(121.0583076), max_latitude: Some(13.8518358), min_latitude: Some(13.5261516), max_longitude: Some(121.1820692), min_longitude: Some(121.0278529)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "باتانغاس"), ("bn", "ব\u{9be}ট\u{9be}ঙ\u{9cd}গ\u{9be}স"), ("ca", "Batangas"), ("ccp", "𑄝𑄑\u{11101}𑄉𑄌\u{11134}"), ("ceb", "Batangas"), ("da", "Batangas"), ("de", "Batangas"), ("el", "Μπατάνγκας"), ("en", "Batangas"), ("es", "Batangas"), ("fa", "باتانگا"), ("fi", "Batangas"), ("fr", "Batangas"), ("gu", "બ\u{ac7}ટ\u{a82}ગસ"), ("he", "באטאנגס"), ("hi", "बता\u{902}गास प\u{94d}रान\u{94d}त"), ("id", "Batangas"), ("it", "Provincia di Batangas"), ("ja", "バタンガス州"), ("kn", "ಬಟಾಂಗಸ\u{ccd}"), ("ko", "바탕가스 주"), ("lt", "Batangasas"), ("lv", "Batangasa"), ("mk", "Батангас"), ("mr", "बटा\u{902}गस"), ("ms", "Batangas"), ("nb", "Batangas"), ("nl", "Batangas"), ("no", "Batangas"), ("pl", "Batangas"), ("pt", "Batangas"), ("ru", "Батангас"), ("si", "බටනගස\u{dca}"), ("sv", "Batangas"), ("ta", "பட\u{bcd}டன\u{bcd}கஸ\u{bcd}"), ("te", "బటంగ\u{c3e}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบาต\u{e31}งก\u{e31}ส"), ("tr", "Batangas"), ("uk", "Батангас"), ("ur", "باتھانگاس"), ("vi", "Batangas"), ("zh", "八打雁省")]),
                        unofficial_name_list: ["Batangas"].to_vec(),
                    }
                ),
                (
                    "BTN",
                    Subdivision{
                        name: "BTN",
                        country_alpha2: Alpha2::PH,
                        code: "BTN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.4485074), longitude: Some(121.9708129), max_latitude: Some(21.1206112), min_latitude: Some(20.2578335), max_longitude: Some(122.034889), min_longitude: Some(121.778977)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "باتانيس"), ("bn", "ব\u{9be}ত\u{9be}নেস"), ("ca", "Batanes"), ("ccp", "𑄝𑄑𑄚𑄬𑄌\u{11134}"), ("ceb", "Batanes"), ("cy", "Batanes"), ("da", "Batanes"), ("de", "Batanes"), ("el", "Μπατάνες"), ("en", "Batanes"), ("es", "Batanes"), ("fa", "باتانه"), ("fi", "Batanes"), ("fr", "Batanes"), ("gu", "બાટન\u{ac7}સ"), ("he", "בטאנס"), ("hi", "बातान\u{947}स"), ("hr", "Batanes"), ("id", "Batanes"), ("it", "Provincia di Batanes"), ("ja", "バタネス州"), ("kn", "ಬ\u{ccd}ಯಾಟನೀಸ\u{ccd}"), ("ko", "바타네스 주"), ("lt", "Batanesas"), ("lv", "Batanesa"), ("mk", "Батанес"), ("mr", "बाटन\u{947}स"), ("ms", "Batanes"), ("nb", "Batanes"), ("nl", "Batanes"), ("no", "Batanes"), ("pl", "Batanes"), ("pt", "Batanes"), ("ru", "Батанес"), ("si", "බටන\u{dd2}ස\u{dca}"), ("sv", "Batanes"), ("ta", "ப\u{bbe}ட\u{bcd}னஸ\u{bcd}"), ("te", "బ\u{c3e}ట\u{c47}న\u{c4d}స\u{c4d}"), ("th", "บาตาเนส"), ("tr", "Batanes"), ("uk", "Батанес"), ("ur", "باتانیس"), ("vi", "Batanes"), ("zh", "巴丹群島省")]),
                        unofficial_name_list: ["Batanes"].to_vec(),
                    }
                ),
                (
                    "BUK",
                    Subdivision{
                        name: "BUK",
                        country_alpha2: Alpha2::PH,
                        code: "BUK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.0515054), longitude: Some(124.9229946), max_latitude: Some(8.593256), min_latitude: Some(7.3991609), max_longitude: Some(125.444464), min_longitude: Some(124.4961241)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بوكيدنون"), ("bn", "ব\u{9c1}কিদনোন"), ("ca", "Bukidnon"), ("ccp", "𑄝\u{1112a}𑄇\u{11128}𑄖\u{11134}𑄚\u{11127}𑄚\u{11134}"), ("ceb", "Bukidnon"), ("da", "Bukidnon"), ("de", "Bukidnon"), ("el", "Μπούκιντνον"), ("en", "Bukidnon"), ("es", "Bukidnon"), ("fa", "بوکیدنون"), ("fi", "Bukidnon"), ("fr", "province de Bukidnon"), ("gu", "બ\u{ac1}કીડનન"), ("he", "בוקידנון"), ("hi", "ब\u{941}किदनोन प\u{94d}रान\u{94d}त"), ("id", "Bukidnon"), ("it", "Provincia di Bukidnon"), ("ja", "ブキドノン州"), ("kn", "ಬುಕ\u{cbf}ಡ\u{ccd}ನಾನ\u{ccd}"), ("ko", "부키드논 주"), ("lt", "Bukidnonas"), ("lv", "Bukidnona"), ("mk", "Букиднон"), ("mr", "बोकिडॉन"), ("ms", "Bukidnon"), ("nb", "Bukidnon"), ("nl", "Bukidnon"), ("no", "Bukidnon"), ("pl", "Bukidnon"), ("pt", "Bukidnon"), ("ru", "Букиднон"), ("si", "බ\u{dd4}ක\u{dd2}ඩ\u{dca}නොන\u{dca}"), ("sv", "Bukidnon"), ("ta", "பிகிட\u{bcd}ணன\u{bcd}"), ("te", "బుక\u{c3f}డ\u{c4d}న\u{c3e}న\u{c4d}"), ("th", "บ\u{e39}ค\u{e34}ดนอน"), ("tr", "Bukidnon"), ("uk", "Букіднон"), ("ur", "بوکیدنون"), ("vi", "Bukidnon"), ("zh", "布基农省")]),
                        unofficial_name_list: ["Bukidnon"].to_vec(),
                    }
                ),
                (
                    "BUL",
                    Subdivision{
                        name: "BUL",
                        country_alpha2: Alpha2::PH,
                        code: "BUL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.7942735), longitude: Some(120.8799008), max_latitude: Some(14.8234278), min_latitude: Some(14.7098374), max_longitude: Some(120.9149916), min_longitude: Some(120.8245333)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بولاكان"), ("bn", "ব\u{9c1}ল\u{9be}ক\u{9be}ন"), ("ca", "Bulacan"), ("ccp", "𑄝\u{1112a}𑄣𑄇𑄚\u{11134}"), ("ceb", "Bulacan"), ("cs", "Bulacan"), ("da", "Bulacan"), ("de", "Bulacan"), ("el", "Μπουλακάν"), ("en", "Bulacan"), ("es", "Bulacán"), ("fa", "بولاکان"), ("fi", "Bulacan"), ("fr", "Bulacan"), ("gu", "બ\u{ac1}લકન"), ("hi", "ब\u{941}लाकान प\u{94d}रान\u{94d}त"), ("id", "Bulacan"), ("it", "provincia di Bulacan"), ("ja", "ブラカン州"), ("kn", "ಬುಲಕಾನ\u{ccd}"), ("ko", "불라칸 주"), ("lt", "Bulakanas"), ("lv", "Bulakana"), ("mk", "Булакан"), ("mr", "बलकन"), ("ms", "Bulacan"), ("nb", "Bulacan"), ("nl", "Bulacan"), ("no", "Bulacan"), ("pl", "Bulacan"), ("pt", "Bulacão"), ("ru", "Булакан"), ("si", "බ\u{dd4}ලකෑන\u{dca}"), ("sv", "Bulacan"), ("ta", "புலக\u{bcd}க\u{bbe}ன\u{bcd}"), ("te", "బుల\u{c3e}క\u{c3e}న\u{c4d}"), ("th", "แอนซ\u{e35} อ\u{e35}ทอยเร\u{e48}"), ("tr", "Bulacan"), ("uk", "Булакан"), ("ur", "بولاکان"), ("vi", "Bulacan"), ("zh", "布拉干省")]),
                        unofficial_name_list: ["Bulacan"].to_vec(),
                    }
                ),
                (
                    "CAG",
                    Subdivision{
                        name: "CAG",
                        country_alpha2: Alpha2::PH,
                        code: "CAG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(18.2489629), longitude: Some(121.8787833), max_latitude: Some(19.9662652), min_latitude: Some(17.5081239), max_longitude: Some(122.33802), min_longitude: Some(120.9600601)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كاغايان"), ("bn", "ক\u{9be}গ\u{9be}য\u{9bc}\u{9be}ন"), ("ca", "Cagayan"), ("ccp", "𑄇\u{11133}𑄠𑄉𑄬𑄃\u{11128}𑄠𑄚\u{11134}"), ("ceb", "Cagayan"), ("cs", "Cagayan"), ("da", "Cagayan"), ("de", "Cagayan"), ("el", "Καγκαγιάν"), ("en", "Cagayan"), ("es", "Cagayán"), ("fa", "کاگایان"), ("fi", "Cagayan"), ("fr", "Cagayan"), ("gu", "ક\u{ac7}ગયન"), ("hi", "कागायान प\u{94d}रान\u{94d}त"), ("id", "Cagayan"), ("it", "Provincia di Cagayan"), ("ja", "カガヤン州"), ("kn", "ಕ\u{ccd}ಯಾಗಾನ\u{ccd}"), ("ko", "카가얀 주"), ("lt", "Kagajanas"), ("lv", "Kagajana"), ("mk", "Кагајан"), ("mr", "क\u{945}गयन"), ("ms", "Cagayan"), ("nb", "Cagayan"), ("nl", "Cagayan"), ("no", "Cagayan"), ("pl", "Cagayan"), ("pt", "Cagayan"), ("ru", "Кагаян"), ("si", "කගය\u{dcf}න\u{dca}"), ("sv", "Cagayan"), ("ta", "கேகயன\u{bcd}"), ("te", "క\u{c3e}గ\u{c47}య\u{c3e}న\u{c4d}"), ("th", "คากาย\u{e31}น"), ("tr", "Cagayan"), ("uk", "Кагаян"), ("ur", "کاگایان"), ("vi", "Cagayan"), ("zh", "卡加延省")]),
                        unofficial_name_list: ["Cagayan"].to_vec(),
                    }
                ),
                (
                    "CAM",
                    Subdivision{
                        name: "CAM",
                        country_alpha2: Alpha2::PH,
                        code: "CAM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.1732164), longitude: Some(124.7298765), max_latitude: Some(9.2580606), min_latitude: Some(9.0787702), max_longitude: Some(124.806468), min_longitude: Some(124.6333946)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كاميغويين"), ("bn", "ক\u{9be}মিক\u{9c1}ইন"), ("ca", "Camiguin"), ("ccp", "𑄇𑄬𑄟\u{11128}𑄉\u{1112a}𑄃\u{11128}𑄚\u{11134}"), ("ceb", "Camiguin"), ("da", "Camiguin"), ("de", "Camiguin"), ("el", "Καμιγκουίν"), ("en", "Camiguin"), ("es", "Camiguín"), ("fa", "کامیگوئین"), ("fi", "Camiguin"), ("fr", "province de Camiguin"), ("gu", "ક\u{ac7}મિગ\u{acd}ય\u{ac1}ઇન"), ("he", "קמיגין"), ("hi", "कमिगिन"), ("id", "Camiguin"), ("it", "Provincia di Camiguin"), ("ja", "カミギン州"), ("kn", "ಕ\u{ccd}ಯಾಮ\u{cbf}ಗುಯ\u{cbf}ನ\u{ccd}"), ("ko", "카미긴 주"), ("lt", "Kamiginas"), ("lv", "Kamigina"), ("mk", "Камигин"), ("mr", "क\u{945}मग\u{941}ईन"), ("ms", "Camiguin"), ("nb", "Camiguin"), ("nl", "Camiguin"), ("no", "Camiguin"), ("pl", "Camiguin"), ("pt", "Camiguin"), ("ru", "Камигин"), ("si", "කම\u{dd2}ග\u{dd4}ය\u{dd2}න\u{dca}"), ("sv", "Camiguin"), ("ta", "க\u{bbe}மிகின\u{bcd}"), ("te", "క\u{c3e}మ\u{c3f}గ\u{c4d}వ\u{c3f}న\u{c4d}"), ("th", "คาม\u{e34}คว\u{e34}น"), ("tr", "Camiguin"), ("uk", "Камігін"), ("ur", "کامیگوین"), ("vi", "Camiguin"), ("zh", "卡米金省")]),
                        unofficial_name_list: ["Camiguin"].to_vec(),
                    }
                ),
                (
                    "CAN",
                    Subdivision{
                        name: "CAN",
                        country_alpha2: Alpha2::PH,
                        code: "CAN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.1390265), longitude: Some(122.7633036), max_latitude: Some(14.4930237), min_latitude: Some(13.835935), max_longitude: Some(123.1175996), min_longitude: Some(122.301291)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كامارينز نورتي"), ("bn", "ক\u{9cd}য\u{9be}ম\u{9be}রিন নর\u{9cd}টে"), ("ca", "Camarines Norte"), ("ccp", "𑄇𑄬𑄟𑄬𑄢\u{1112d}𑄚\u{11134}𑄥\u{11134} 𑄚\u{11127}𑄢\u{11134}𑄑𑄬"), ("ceb", "Camarines Norte"), ("da", "Camarines Norte"), ("de", "Camarines Norte"), ("el", "Καμαρίνες Νόρτε"), ("en", "Camarines Norte"), ("es", "Camarines Norte"), ("fa", "کامارینه شمالی"), ("fi", "Camarines Norte"), ("fr", "Camarines Norte"), ("gu", "ક\u{ac7}મ\u{ac7}રિન\u{acd}સ નોર\u{acd}ટ"), ("hi", "कामारिन\u{947}स नोर\u{94d}त\u{947} प\u{94d}रान\u{94d}त"), ("id", "Camarines Norte"), ("it", "Provincia di Camarines Norte"), ("ja", "北カマリネス州"), ("kn", "ಕ\u{ccd}ಯಾಮರ\u{cbf}ನ\u{ccd} ನಾರ\u{ccd}ಟ\u{cc6}"), ("ko", "북카마리네스 주"), ("lt", "Kamarines Nortė"), ("lv", "Kamarines Norte"), ("mk", "Северен Камаринес"), ("mr", "क\u{947}मारिन\u{94d}स नॉर\u{94d}ट\u{947}"), ("ms", "Camarines Norte"), ("nb", "Camarines Norte"), ("nl", "Camarines Norte"), ("no", "Camarines Norte"), ("pl", "Camarines Norte"), ("pt", "Camarines Norte"), ("ru", "Северный Камаринес"), ("si", "කමරය\u{dd2}න\u{dca}ස\u{dca} නොර\u{dca}ටේ"), ("sv", "Camarines Norte"), ("ta", "க\u{bbe}ம\u{bbe}ரின\u{bcd}ஸ\u{bcd} நோர\u{bcd}ட\u{bcd}"), ("te", "క\u{c3e}మ\u{c46}ర\u{c48}న\u{c4d}స\u{c4d} న\u{c3e}ర\u{c4d}ట\u{c46}"), ("th", "กามาร\u{e35}เนสเหน\u{e37}อ"), ("tr", "Camarines Norte"), ("uk", "Північний Камарінес"), ("ur", "کامارینز شمالی"), ("vi", "Camarines Norte"), ("zh", "北甘馬粦省")]),
                        unofficial_name_list: ["Camarines Norte"].to_vec(),
                    }
                ),
                (
                    "CAP",
                    Subdivision{
                        name: "CAP",
                        country_alpha2: Alpha2::PH,
                        code: "CAP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.3888799), longitude: Some(122.6277455), max_latitude: Some(11.645162), min_latitude: Some(11.1436509), max_longitude: Some(123.1000324), min_longitude: Some(122.1992869)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كابيز"), ("bn", "ক\u{9be}ফিজ"), ("ccp", "𑄇\u{11133}𑄠𑄛\u{11128}𑄌\u{11134}"), ("ceb", "Capiz"), ("da", "Capiz"), ("de", "Capiz"), ("el", "Καπίζ"), ("en", "Capiz"), ("es", "Cápiz"), ("fa", "کاپیز"), ("fi", "Capiz"), ("fr", "Cápiz"), ("gu", "ક\u{ac7}પિઝ"), ("hi", "कापीज\u{93c} प\u{94d}रान\u{94d}त"), ("id", "Capiz"), ("it", "Provincia di Capiz"), ("ja", "カピス州"), ("kn", "ಕ\u{ccd}ಯಾಪ\u{cbf}ಜ\u{ccd}"), ("ko", "카피스 주"), ("lt", "Kapizas"), ("lv", "Kapiza"), ("mk", "Капиз"), ("mr", "क\u{945}पिझ"), ("ms", "Capiz"), ("nb", "Capiz"), ("nl", "Capiz"), ("no", "Capiz"), ("pl", "Capiz"), ("pt", "Capiz"), ("ru", "Капис"), ("si", "කැප\u{dd2}ස\u{dca}"), ("sv", "Capiz"), ("ta", "க\u{bbe}பிஸ\u{bcd}"), ("te", "క\u{c3e}ప\u{c3f}జ\u{c4d}"), ("th", "คาป\u{e34}ซ"), ("tr", "Capiz"), ("uk", "Капіз"), ("ur", "کاپیز"), ("vi", "Capiz"), ("zh", "卡皮茲省")]),
                        unofficial_name_list: ["Capiz"].to_vec(),
                    }
                ),
                (
                    "CAS",
                    Subdivision{
                        name: "CAS",
                        country_alpha2: Alpha2::PH,
                        code: "CAS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.5250197), longitude: Some(123.3486147), max_latitude: Some(14.1279977), min_latitude: Some(13.2638826), max_longitude: Some(123.97699), min_longitude: Some(122.550947)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كامارينس سور"), ("bn", "ক\u{9be}ম\u{9be}রিনেস সোর"), ("ca", "Camarines Sur"), ("ccp", "𑄇𑄬𑄟𑄬𑄢\u{1112d}𑄚\u{11134} 𑄥𑄢\u{11134}"), ("ceb", "Camarines Sur"), ("da", "Camarines Sur"), ("de", "Camarines Sur"), ("el", "Καμαρίνες Σουρ"), ("en", "Camarines Sur"), ("es", "Camarines Sur"), ("fa", "کامارینه جنوبی"), ("fi", "Camarines Sur"), ("fr", "Camarines Sur"), ("gu", "ક\u{ac7}મ\u{ac7}રિન\u{acd}સ સ\u{ac1}ર"), ("he", "איי קמרינס"), ("hi", "कामारिन\u{947}स स\u{942}र प\u{94d}रान\u{94d}त"), ("hy", "Հարավային Կամարինես"), ("id", "Camarines Sur"), ("it", "Provincia di Camarines Sur"), ("ja", "南カマリネス州"), ("kn", "ಕ\u{ccd}ಯಾಮರೀನ\u{ccd} ಸುರ\u{ccd}"), ("ko", "남카마리네스 주"), ("lt", "Kamarines Suras"), ("lv", "Dienvidkamarinesa"), ("mk", "Јужен Камаринес"), ("mr", "क\u{947}मारिन\u{94d}स स\u{941}र"), ("ms", "Camarines Sur"), ("nb", "Camarines Sur"), ("nl", "Camarines Sur"), ("no", "Camarines Sur"), ("pl", "Camarines Sur"), ("pt", "Camarines Sur"), ("ru", "Южный Камаринес"), ("si", "කමරය\u{dd2}න\u{dca}ස\u{dca} සර\u{dca}"), ("sv", "Camarines Sur"), ("ta", "க\u{bbe}ம\u{bbe}ரின\u{bcd}ஸ\u{bcd} சூர\u{bcd}"), ("te", "క\u{c3e}మ\u{c46}ర\u{c48}న\u{c4d}స\u{c4d} సుర\u{c4d}"), ("th", "คามาไลเนส เซอ"), ("tr", "Camarines Sur"), ("uk", "Південний Камарінес"), ("ur", "کامارینز جنوبی"), ("vi", "Camarines Sur"), ("zh", "南甘馬粦省")]),
                        unofficial_name_list: ["Camarines Sur"].to_vec(),
                    }
                ),
                (
                    "CAT",
                    Subdivision{
                        name: "CAT",
                        country_alpha2: Alpha2::PH,
                        code: "CAT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.7088684), longitude: Some(124.2421597), max_latitude: Some(14.1005278), min_latitude: Some(13.5181658), max_longitude: Some(124.4218897), min_longitude: Some(124.0169811)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كاتاندوانه"), ("bn", "ক\u{9cd}য\u{9be}ট\u{9be}ন\u{9cd}ড\u{9c1}য\u{9bc}\u{9be}ন\u{9cd}স"), ("ca", "Catanduanes"), ("ccp", "𑄇𑄑𑄚\u{11134}𑄓\u{1112a}𑄠𑄬𑄚\u{11134}𑄌\u{11134}"), ("ceb", "Catanduanes"), ("cs", "Catanduanes"), ("da", "Catanduanes"), ("de", "Catanduanes"), ("el", "Καταντουάνες"), ("en", "Catanduanes"), ("es", "Catanduanes"), ("fa", "کاتاندوانه"), ("fi", "Catanduanes"), ("fr", "Catanduanes"), ("gl", "Catanduanes"), ("gu", "ક\u{ac7}ટાન\u{acd}ડ\u{ac1}અન\u{ac7}સ"), ("hi", "कतन\u{94d}द\u{941}आन\u{947}स"), ("id", "Catanduanes"), ("it", "Provincia di Catanduanes"), ("ja", "カタンドゥアネス州"), ("kn", "ಕ\u{ccd}ಯಾಟ\u{ccc}ನ\u{ccd}ವಾನ\u{ccd}ಸ\u{ccd}"), ("ko", "카탄두아네스 주"), ("lt", "Katandvanesas"), ("lv", "Katanduanesa"), ("mk", "Катандуанес"), ("ml", "കട\u{d3e}ൻദ\u{d41}വ\u{d3e}നിസ\u{d4d}"), ("mr", "क\u{945}ट\u{902}ड\u{941}अस"), ("ms", "Catanduanes"), ("nb", "Catanduanes"), ("nl", "Catanduanes"), ("no", "Catanduanes"), ("pl", "Catanduanes"), ("pt", "Catanduanes"), ("ru", "Катандуанес"), ("si", "කටනඩ\u{dd4}වන\u{dd3}ස\u{dca}"), ("sv", "Catanduanes"), ("ta", "கடந\u{bcd}தவனேஸ\u{bcd}"), ("te", "కట\u{c3e}ండుయ\u{c47}న\u{c4d}స\u{c4d}"), ("th", "เม\u{e37}องกาต\u{e31}นด\u{e31}วเนส"), ("tr", "Catanduanes"), ("uk", "Катандуанес"), ("ur", "کاتاندوانیس"), ("vi", "Catanduanes"), ("zh", "卡坦端内斯省")]),
                        unofficial_name_list: ["Catanduanes"].to_vec(),
                    }
                ),
                (
                    "CAV",
                    Subdivision{
                        name: "CAV",
                        country_alpha2: Alpha2::PH,
                        code: "CAV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.4791297), longitude: Some(120.8969634), max_latitude: Some(14.5002355), min_latitude: Some(14.3646447), max_longitude: Some(120.9209909), min_longitude: Some(120.5212855)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كاويته"), ("bn", "ক\u{9cd}য\u{9be}ভিটে"), ("ca", "Cavite"), ("ccp", "𑄇𑄞\u{11128}𑄖\u{11134}"), ("ceb", "Cavite"), ("da", "Provins Cavite"), ("de", "Provinz Cavite"), ("el", "Καβιτέ"), ("en", "Cavite"), ("es", "Cavite"), ("fa", "کاویته"), ("fi", "Cavite"), ("fr", "province de Cavite"), ("gu", "ક\u{ac7}વિટ\u{ac7}"), ("hi", "कावित\u{947} प\u{94d}रान\u{94d}त"), ("hy", "Կավիտե"), ("id", "Cavite"), ("it", "Provincia di Cavite"), ("ja", "カヴィテ州"), ("kn", "ಕ\u{ccd}ಯಾವ\u{cc6}ಟ\u{ccd}"), ("ko", "카비테 주"), ("lt", "Kavitė"), ("lv", "Kavite"), ("mk", "Кавите"), ("mr", "कवाट"), ("ms", "Cavite"), ("nb", "Cavite"), ("nl", "Cavite"), ("no", "Cavite"), ("pl", "Prowincja Cavite"), ("pt", "Província de Cavite"), ("ru", "Кавите"), ("si", "කවය\u{dd2}ට\u{dca}"), ("sv", "Provins Cavite"), ("ta", "க\u{bbe}விட\u{bcd}"), ("te", "క\u{c3e}వ\u{c3f}ట\u{c3f}"), ("th", "คาว\u{e34}ต"), ("tr", "Cavite"), ("uk", "Кавіте"), ("ur", "کاویت"), ("vi", "Cavite"), ("zh", "甲米地省")]),
                        unofficial_name_list: ["Cavite"].to_vec(),
                    }
                ),
                (
                    "CEB",
                    Subdivision{
                        name: "CEB",
                        country_alpha2: Alpha2::PH,
                        code: "CEB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.3156992), longitude: Some(123.8854366), max_latitude: Some(10.498277), min_latitude: Some(10.2594378), max_longitude: Some(123.9326476), min_longitude: Some(123.7633896)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سيبو"), ("bn", "সেব\u{9c1}"), ("ca", "Província de Cebú"), ("ccp", "𑄥𑄬𑄝\u{1112a}"), ("ceb", "Sugbo"), ("cs", "Cebu"), ("da", "Cebu"), ("de", "Provinz Cebu"), ("el", "Κεμπού"), ("en", "Cebu"), ("es", "Cebú"), ("et", "Cebu"), ("eu", "Cebu"), ("fa", "جزیره سبو"), ("fi", "Cebu"), ("fr", "Cebu"), ("gl", "Cebú"), ("gu", "સિબ\u{ac1}"), ("he", "סבו"), ("hi", "सिब\u{941}"), ("hy", "Սեբու"), ("id", "Cebu"), ("it", "Provincia di Cebu"), ("ja", "セブ州"), ("kn", "ಸ\u{cc6}ಬು"), ("ko", "세부 주"), ("lt", "Sebu"), ("lv", "Sebu"), ("mk", "Себу"), ("ml", "സെബ\u{d42}"), ("mr", "सिब\u{942}"), ("ms", "Cebu"), ("nb", "Cebu"), ("nl", "Cebu"), ("no", "Cebu"), ("pl", "Cebu"), ("pt", "Cebu"), ("ru", "Себу"), ("si", "සේබ\u{dd6}"), ("sr", "Себу"), ("sr_Latn", "Sebu"), ("sv", "Cebu"), ("ta", "செப\u{bcd}பு"), ("te", "స\u{c46}బు"), ("th", "เซบ\u{e39}"), ("tr", "Cebu"), ("uk", "Себу"), ("ur", "سیبو"), ("vi", "Cebu"), ("zh", "宿霧省")]),
                        unofficial_name_list: ["Cebu"].to_vec(),
                    }
                ),
                (
                    "COM",
                    Subdivision{
                        name: "COM",
                        country_alpha2: Alpha2::PH,
                        code: "COM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.512514999999999), longitude: Some(126.1762615), max_latitude: Some(7.9994319), min_latitude: Some(7.078257199999999), max_longitude: Some(126.2988501), min_longitude: Some(125.674365)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كامبوستلا ولي"), ("bn", "কম\u{9cd}পোস\u{9cd}টেল\u{9be} ভ\u{9cd}য\u{9be}লি"), ("ccp", "𑄇\u{11127}𑄟\u{11134}𑄛\u{1112a}𑄌𑄑𑄬𑄣 𑄞𑄬𑄣\u{11128}"), ("ceb", "Compostela Valley"), ("da", "Compostela Valley"), ("de", "Compostela Valley"), ("el", "Κομποστέλα Βάλεϊ"), ("en", "Compostela Valley"), ("es", "Valle de Compostela"), ("fa", "کامپوستلا ولی"), ("fi", "Compostela Valley"), ("fr", "Vallée de Compostela"), ("gu", "કોમ\u{acd}પોસ\u{acd}ટ\u{ac7}લા વ\u{ac7}લી"), ("hi", "कोम\u{94d}पोस\u{94d}त\u{947}ला घाटी"), ("id", "Compostela Valley"), ("it", "Provincia di Compostela Valley"), ("ja", "コンポステラ・バレー州"), ("kn", "ಕಾಂಪೊಸ\u{ccd}ಟ\u{cc6}ಲಾ ಕಣ\u{cbf}ವ\u{cc6}"), ("ko", "콤포스텔라밸리 주"), ("lt", "Kompostelos slėnis"), ("lv", "Kompostelas ieleja"), ("mk", "Долина Компостела"), ("mr", "क\u{902}पोस\u{94d}ट\u{947}ला व\u{94d}ह\u{945}ली"), ("ms", "Compostela Valley"), ("nb", "Compostela Valley"), ("nl", "Compostela Valley"), ("no", "Compostela Valley"), ("pl", "Compostela Valley"), ("pt", "Vale de Compostela"), ("ru", "Долина Компостела"), ("si", "කොම\u{dca}පොස\u{dca}ටේල\u{dcf} වැල\u{dd2}"), ("sv", "Composteladalen"), ("ta", "கொம\u{bcd}போஸ\u{bcd}ட\u{bcd}டேள\u{bbe} வ\u{bbe}லே"), ("te", "క\u{c3e}ంప\u{c4b}స\u{c4d}ట\u{c46}ల\u{c3e} వ\u{c4d}య\u{c3e}ల\u{c40}"), ("th", "คอมโพสเตลา"), ("tr", "Compostela Vadisi"), ("uk", "Долина Компостела"), ("ur", "وادی کومپوستیلا"), ("vi", "Compostela Valley"), ("zh", "康波斯特拉谷省")]),
                        unofficial_name_list: ["Compostela Valley"].to_vec(),
                    }
                ),
                (
                    "DAO",
                    Subdivision{
                        name: "DAO",
                        country_alpha2: Alpha2::PH,
                        code: "DAO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.3171585), longitude: Some(126.5419887), max_latitude: Some(8.0013469), min_latitude: Some(5.5511669), max_longitude: Some(126.6015399), min_longitude: Some(125.938116)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "دافاو أورينتال"), ("bn", "দ\u{9be}ভ\u{9be}ও ওরিয\u{9bc}েন\u{9cd}ট\u{9be}ল"), ("ccp", "𑄓𑄞𑄃\u{1112e} 𑄃\u{11127}𑄢\u{11128}𑄠𑄬𑄚\u{11134}𑄑𑄣\u{11134}"), ("ceb", "Davao Oriental"), ("da", "Davao Oriental"), ("de", "Davao Oriental"), ("el", "Νταβάο Οριεντάλ"), ("en", "Davao Oriental"), ("es", "Davao Oriental"), ("fa", "داوائو شرقی"), ("fi", "Davao Oriental"), ("fr", "Davao oriental"), ("gu", "દવાઓ ઓરિએન\u{acd}ટલ"), ("hi", "दावाओ ओरिए\u{902}टल"), ("id", "Davao Oriental"), ("it", "Provincia di Davao Oriental"), ("ja", "東ダバオ州"), ("kn", "ದಾವೊವೊ ಒರ\u{cbf}ಯಂಟಲ\u{ccd}"), ("ko", "동다바오 주"), ("lt", "Rytų Davao"), ("lv", "Austrumdavao"), ("mk", "Источен Давао"), ("mr", "द\u{945}वओ ओरिए\u{902}टल"), ("ms", "Davao Oriental"), ("nb", "Davao Oriental"), ("nl", "Davao Oriental"), ("no", "Davao Oriental"), ("pl", "Davao Oriental"), ("pt", "Davao Oriental"), ("ru", "Восточный Давао"), ("si", "ඩව\u{dcf}ඕ ඔර\u{dd2}යෙන\u{dca}ටල\u{dca}"), ("sv", "Davao Oriental"), ("ta", "ட\u{bbe}வ\u{bbe}வ\u{bcd} ஒரிஎண\u{bcd}டல\u{bcd}"), ("te", "డ\u{c3e}వ\u{c3e}వ\u{c4b} ఓర\u{c3f}యంటల\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดดาเวาโอเร\u{e35}ยนเต\u{e47}ล"), ("tr", "Davao Oriental"), ("uk", "Східний Давао"), ("ur", "داوائو شرقی"), ("vi", "Davao Oriental"), ("zh", "东达沃省")]),
                        unofficial_name_list: ["Davao Oriental"].to_vec(),
                    }
                ),
                (
                    "DAS",
                    Subdivision{
                        name: "DAS",
                        country_alpha2: Alpha2::PH,
                        code: "DAS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.766268699999999), longitude: Some(125.3284269), max_latitude: Some(7.565537000000001), min_latitude: Some(5.367669999999999), max_longitude: Some(125.7164231), min_longitude: Some(125.1010209)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "دافاو ديل سور"), ("bn", "ড\u{9be}ভ\u{9be}ও ডেল স\u{9c1}র"), ("ccp", "𑄓𑄞𑄃\u{1112e} 𑄓𑄬𑄣\u{11134} 𑄥𑄢\u{11134}"), ("ceb", "Davao del Sur"), ("da", "Davao del Sur"), ("de", "Davao del Sur"), ("el", "Νταβάο ντελ Σούρ"), ("en", "Davao del Sur"), ("es", "Davao del Sur"), ("fa", "داوائو جنوبی"), ("fi", "Davao del Sur"), ("fr", "Davao du Sud"), ("gu", "દાવાઓ ડ\u{ac7}લ સ\u{ac1}ર"), ("hi", "दावाओ द\u{947}ल स\u{942}र"), ("id", "Davao del Sur"), ("it", "Provincia di Davao del Sur"), ("ja", "南ダバオ州"), ("kn", "ದಾವೊವೊ ಡ\u{cc6}ಲ\u{ccd} ಸುರ\u{ccd}"), ("ko", "남다바오 주"), ("lt", "Pietų Davao"), ("lv", "Dienviddavao"), ("mk", "Јужен Давао"), ("mr", "डावाओ ड\u{947}ल स\u{941}र"), ("ms", "Davao del Sur"), ("nb", "Davao del Sur"), ("nl", "Davao del Sur"), ("no", "Davao del Sur"), ("pl", "Davao del Sur"), ("pt", "Davao do Sul"), ("ru", "Южный Давао"), ("si", "ඩව\u{dcf}ඕ ඩෙල\u{dca} සර\u{dca}"), ("sv", "Davao del Sur"), ("ta", "டவையோ டேல\u{bcd} சூர\u{bcd}"), ("te", "డ\u{c3e}వ\u{c3e}వ\u{c4b} డ\u{c46}ల\u{c4d} సుర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดดาเวา เดล เซอร\u{e4c}"), ("tr", "Davao del Sur"), ("uk", "Південний Давао"), ("ur", "داوائو جنوبی"), ("vi", "Davao del Sur"), ("zh", "南達沃省")]),
                        unofficial_name_list: ["Davao del Sur"].to_vec(),
                    }
                ),
                (
                    "DAV",
                    Subdivision{
                        name: "DAV",
                        country_alpha2: Alpha2::PH,
                        code: "DAV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.561769899999999), longitude: Some(125.6532848), max_latitude: Some(7.997327), min_latitude: Some(6.894526000000001), max_longitude: Some(125.944498), min_longitude: Some(125.250456)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "دافاو ديل نورت"), ("bn", "দ\u{9be}ভ\u{9be}ও দেল নর\u{9cd}টে"), ("ccp", "𑄓𑄞𑄃\u{1112e} 𑄓𑄬𑄣\u{11134} 𑄚\u{11127}𑄢\u{11134}𑄑𑄬"), ("ceb", "Davao del Norte"), ("da", "Davao del Norte"), ("de", "Davao del Norte"), ("el", "Νταβάο ντελ Νόρτε"), ("en", "Davao del Norte"), ("es", "Davao del Norte"), ("fa", "داوائو شمالی"), ("fi", "Davao del Norte"), ("fr", "Davao du Nord"), ("gu", "દાવાઓ ડ\u{ac7}લ નોર\u{acd}ટ\u{ac7}"), ("hi", "दावाओ द\u{947}ल नोर\u{94d}त\u{947}"), ("id", "Davao del Norte"), ("it", "Provincia di Davao del Norte"), ("ja", "ダバオ州"), ("kn", "ಡೇವೊ ಡ\u{cc6}ಲ\u{ccd} ನಾರ\u{ccd}ಟ\u{cc6}"), ("ko", "북다바오 주"), ("lt", "Šiaurės Davao"), ("lv", "Ziemeļdavao"), ("mk", "Северен Давао"), ("mr", "द\u{945}व\u{94d}हाओ ड\u{947}ल नॉर\u{94d}ट"), ("ms", "Davao del Norte"), ("nb", "Davao del Norte"), ("nl", "Davao del Norte"), ("no", "Davao del Norte"), ("pl", "Davao del Norte"), ("pt", "Davao del Norte"), ("ru", "Северный Давао"), ("si", "ඩව\u{dcf}ඕ ඩෙල\u{dca} නොර\u{dca}ටේ"), ("sv", "Davao del Norte"), ("ta", "ட\u{bbe}வோ டெல\u{bcd} நோர\u{bcd}ட\u{bcd}டே"), ("te", "డ\u{c47}వ\u{c3e} డ\u{c46}ల\u{c4d} న\u{c4b}ర\u{c4d}ట\u{c46}"), ("th", "ดาเวา เดล เหน\u{e37}อ"), ("tr", "Davao del Norte"), ("uk", "Північний Давао"), ("ur", "داوائو شمالی"), ("vi", "Davao del Norte"), ("zh", "北達沃省")]),
                        unofficial_name_list: ["Davao"].to_vec(),
                    }
                ),
                (
                    "DIN",
                    Subdivision{
                        name: "DIN",
                        country_alpha2: Alpha2::PH,
                        code: "DIN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.1281816), longitude: Some(125.6095474), max_latitude: Some(10.4716027), min_latitude: Some(9.8547497), max_longitude: Some(125.7067526), min_longitude: Some(125.465279)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄓\u{11128}𑄚𑄉𑄖\u{11134} 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄥\u{11134}"), ("ceb", "Kapupud-ang Dinagat"), ("de", "Dinagat Islands"), ("en", "Dinagat Islands"), ("es", "Islas Dinagat"), ("fa", "دیناگات آیلندز"), ("fi", "Dinagat Islands"), ("fr", "province des Îles Dinagat"), ("hi", "दिनागत द\u{94d}वीपसम\u{942}ह"), ("id", "Kepulauan Dinagat"), ("it", "Provincia di Dinagat Islands"), ("ja", "ディナガット・アイランズ州"), ("ko", "디나가트 제도 주"), ("mk", "Динагат"), ("nb", "Dinagat Islands"), ("nl", "Dinagat Islands"), ("no", "Dinagat Islands"), ("pl", "Prowincja Dinagat Islands"), ("ru", "Острова Динагат"), ("sv", "Dinagatöarna"), ("th", "จ\u{e31}งหว\u{e31}ดคาป\u{e39}ล\u{e39}อ\u{e31}งด\u{e35}นาก\u{e31}ต"), ("uk", "Острови Дінагат"), ("ur", "جزائر دیناگات"), ("vi", "Quần đảo Dinagat"), ("zh", "迪纳加特群岛")]),
                        unofficial_name_list: ["Dinagat Islands"].to_vec(),
                    }
                ),
                (
                    "DVO",
                    Subdivision{
                        name: "DVO",
                        country_alpha2: Alpha2::PH,
                        code: "DVO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.879721), longitude: Some(121.774017), max_latitude: Some(19.5740241), min_latitude: Some(4.5870339), max_longitude: Some(126.6043837), min_longitude: Some(116.7029193)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄓𑄞𑄃\u{1112e} 𑄃\u{11127}𑄇\u{11134}𑄥\u{11128}𑄓𑄬𑄚\u{11134}𑄑𑄣\u{11134}"), ("ceb", "Davao Occidental"), ("en", "Davao Occidental"), ("es", "Dávao Occidental"), ("fi", "Davao Occidental"), ("fr", "Davao occidental"), ("hi", "दावाओ ओक\u{94d}सीड\u{947}\u{902}टल"), ("id", "Davao Occidental"), ("it", "Davao Occidental"), ("ko", "서다바오 주"), ("mk", "Западен Давао"), ("nl", "Davao Occidental"), ("th", "จ\u{e31}งหว\u{e31}ดค\u{e31}นล\u{e39}ร\u{e31}งดาเบา"), ("uk", "Західне Давао"), ("ur", "داوائو غربی"), ("vi", "Davao Occidental"), ("zh", "西達沃省")]),
                        unofficial_name_list: ["Davao Occidental"].to_vec(),
                    }
                ),
                (
                    "EAS",
                    Subdivision{
                        name: "EAS",
                        country_alpha2: Alpha2::PH,
                        code: "EAS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.5000731), longitude: Some(125.4999908), max_latitude: Some(12.370305), min_latitude: Some(10.690784), max_longitude: Some(125.969378), min_longitude: Some(125.1116951)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سامار الشرقية"), ("bn", "ইস\u{9cd}ট\u{9be}র\u{9cd}ন স\u{9be}ম\u{9be}র"), ("ca", "Samar Oriental"), ("ccp", "𑄛\u{1112a}𑄇\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄥\u{11127}𑄟𑄢\u{11134}"), ("ceb", "Sidlakang Samar"), ("da", "Eastern Samar"), ("de", "Eastern Samar"), ("el", "Ανατολικό Σαμάρ"), ("en", "Eastern Samar"), ("es", "Sámar Oriental"), ("fa", "سامار شرقی"), ("fi", "Eastern Samar"), ("fr", "Samar oriental"), ("gu", "ઇસ\u{acd}ટર\u{acd}ન સમર"), ("hi", "प\u{942}र\u{94d}वी सामार प\u{94d}रान\u{94d}त"), ("id", "Samar Timur"), ("it", "Provincia di Eastern Samar"), ("ja", "東サマル州"), ("kn", "ಪ\u{cc2}ರ\u{ccd}ವ ಸಮರ\u{ccd}"), ("ko", "동사마르 주"), ("lt", "Rytų Samaras"), ("lv", "Austrumu Samara"), ("mk", "Источен Самар"), ("mr", "ईस\u{94d}टर\u{94d}न समर"), ("ms", "Eastern Samar"), ("nb", "Eastern Samar"), ("nl", "Eastern Samar"), ("no", "Eastern Samar"), ("pl", "Eastern Samar"), ("pt", "Samar Oriental"), ("ru", "Восточный Самар"), ("si", "නැගෙනහ\u{dd2}ර සම\u{dcf}ර\u{dca}"), ("sv", "Östra Samar"), ("ta", "கிழக\u{bcd}கு ச\u{bbe}மர\u{bcd}"), ("te", "తూర\u{c4d}పు సమర\u{c4d}"), ("th", "ซามาร\u{e4c}ตะว\u{e31}นออก"), ("tr", "Doğu Samar"), ("uk", "Східний Самар"), ("ur", "مشرقی سامار"), ("vi", "Đông Samar"), ("zh", "東薩馬省")]),
                        unofficial_name_list: ["Eastern Samar"].to_vec(),
                    }
                ),
                (
                    "GUI",
                    Subdivision{
                        name: "GUI",
                        country_alpha2: Alpha2::PH,
                        code: "GUI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.5928661), longitude: Some(122.6325081), max_latitude: Some(10.7544428), min_latitude: Some(10.389255), max_longitude: Some(122.741077), min_longitude: Some(122.4749851)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غيماراس"), ("bn", "গ\u{9c1}ইম\u{9be}র\u{9be}স"), ("ca", "Guimaras"), ("ccp", "𑄉\u{1112d}\u{1112a}𑄟𑄢𑄌\u{11134}"), ("ceb", "Guimaras"), ("da", "Guimaras"), ("de", "Guimaras"), ("el", "Γκουιμάρας"), ("en", "Guimaras"), ("es", "Guimarás"), ("fa", "گیماراس"), ("fi", "Guimaras"), ("fr", "province de Guimaras"), ("gl", "Guimarás"), ("gu", "ગ\u{ac1}મ\u{ac7}રાસ"), ("hi", "गिमारास"), ("id", "Guimaras"), ("it", "Provincia di Guimaras"), ("ja", "ギマラス州"), ("kn", "ಗುಯ\u{cbf}ಮಾರಸ\u{ccd}"), ("ko", "기마라스 주"), ("lt", "Gimarasas"), ("lv", "Gimarasa"), ("mk", "Гимарас"), ("mr", "ग\u{941}इमर\u{94d}स"), ("ms", "Guimaras"), ("nb", "Guimaras"), ("nl", "Guimaras"), ("no", "Guimaras"), ("pl", "Guimaras"), ("pt", "Guimaras"), ("ru", "Гимарас"), ("si", "ග\u{dd4}ය\u{dd2}න\u{dca}මර\u{dcf}ස\u{dca}"), ("sv", "Guimaras"), ("ta", "கும\u{bbe}ரஸ\u{bcd}"), ("te", "గ\u{c3f}మ\u{c3e}రస\u{c4d}"), ("th", "เกาะก\u{e38}ยมาราส"), ("tr", "Guimaras"), ("uk", "Гуймарас"), ("ur", "گوئماراس"), ("vi", "Guimaras"), ("zh", "吉馬拉斯省")]),
                        unofficial_name_list: ["Guimaras"].to_vec(),
                    }
                ),
                (
                    "IFU",
                    Subdivision{
                        name: "IFU",
                        country_alpha2: Alpha2::PH,
                        code: "IFU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.8330792), longitude: Some(121.1710389), max_latitude: Some(17.071433), min_latitude: Some(16.598261), max_longitude: Some(121.5859809), min_longitude: Some(120.881094)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ايفوغايو"), ("bn", "ইফ\u{9c1}গ\u{9be}ও"), ("ca", "Ifugao"), ("ccp", "𑄃\u{1112d}𑄜\u{1112a}𑄉𑄃\u{1112e}"), ("ceb", "Ifugao"), ("da", "Ifugao"), ("de", "Ifugao"), ("el", "Ιφουγκάο"), ("en", "Ifugao"), ("es", "Ifugao"), ("fa", "ایفوگائو"), ("fi", "Ifugao"), ("fr", "Ifugao"), ("gu", "ઇફ\u{ac1}ગાઓ"), ("hi", "इफ\u{93c}\u{941}गाओ प\u{94d}रान\u{94d}त"), ("id", "Ifugao"), ("it", "Provincia di Ifugao"), ("ja", "イフガオ州"), ("kn", "ಇಪುಗಾವೊ"), ("ko", "이푸가오 주"), ("lt", "Ifugao"), ("lv", "Ifugao"), ("mk", "Ифугао"), ("mr", "ईग\u{942}गाओ"), ("ms", "Ifugao"), ("nb", "Ifugao"), ("nl", "Ifugao"), ("no", "Ifugao"), ("pl", "Ifugao"), ("pt", "Ifugão"), ("ru", "Ифугао"), ("si", "ඉෆ\u{dd4}ග\u{dcf}ඕ"), ("sv", "Ifugao"), ("ta", "ஐபியூக\u{bbe}வ\u{bcd}"), ("te", "ఇఫుగ\u{c3e}వ\u{c4b}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e35}ฟ\u{e39}เกา"), ("tr", "Ifugao"), ("uk", "Іфугао"), ("ur", "ایفوگاؤ"), ("vi", "Ifugao"), ("zh", "伊富高省")]),
                        unofficial_name_list: ["Ifugao"].to_vec(),
                    }
                ),
                (
                    "ILI",
                    Subdivision{
                        name: "ILI",
                        country_alpha2: Alpha2::PH,
                        code: "ILI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.7201501), longitude: Some(122.5621063), max_latitude: Some(10.7818569), min_latitude: Some(10.6792844), max_longitude: Some(122.6001559), min_longitude: Some(122.494087)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إلويلو"), ("bn", "ইলোইলো"), ("ca", "Iloilo"), ("ccp", "𑄃\u{11128}𑄣\u{1112e}𑄃\u{11128}𑄣\u{1112e}"), ("ceb", "Iloilo"), ("da", "Iloilo"), ("de", "Iloilo"), ("el", "Ιλόιλο"), ("en", "Iloilo"), ("es", "Iloílo"), ("fa", "ایلوئیلو"), ("fi", "Iloilo"), ("fr", "province de Iloilo"), ("gu", "ઇલોઇલો"), ("hi", "इलोइलो प\u{94d}रान\u{94d}त"), ("hu", "Iloilo"), ("id", "Iloilo"), ("it", "Provincia di Iloilo"), ("ja", "イロイロ州"), ("kn", "ಇಲ\u{ccc}ಲೋ"), ("ko", "일로일로 주"), ("lt", "Iloilas"), ("lv", "Iloilo"), ("mk", "Илоило"), ("mr", "ललॉयलो"), ("ms", "Iloilo"), ("nb", "Iloilo"), ("nl", "Iloilo"), ("no", "Iloilo"), ("pl", "Prowincja Iloilo"), ("pt", "Iloilo"), ("ru", "Илоило"), ("si", "ඉලොය\u{dd2}ලෝ"), ("sv", "Iloilo"), ("ta", "இலோயிலோ"), ("te", "ఇల\u{c3e}య\u{c3f}ల\u{c4b}"), ("th", "อ\u{e34}โลยโล"), ("tr", "Iloilo"), ("uk", "Ілоіло"), ("ur", "الوئیلو"), ("vi", "Iloilo"), ("zh", "伊洛伊洛省")]),
                        unofficial_name_list: ["Iloilo"].to_vec(),
                    }
                ),
                (
                    "ILN",
                    Subdivision{
                        name: "ILN",
                        country_alpha2: Alpha2::PH,
                        code: "ILN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(18.1647281), longitude: Some(120.7115592), max_latitude: Some(18.650944), min_latitude: Some(17.698466), max_longitude: Some(120.980399), min_longitude: Some(120.4290068)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إيلوكوس نورت"), ("bn", "ইলোকোস নর\u{9cd}তে"), ("ca", "Ilocos Norte"), ("ccp", "𑄃\u{11128}𑄣\u{1112e}𑄇\u{1112e}𑄌\u{11134} 𑄟\u{11127}𑄢\u{11134}𑄑𑄬"), ("ceb", "Ilocos Norte"), ("da", "Ilocos Norte"), ("de", "Ilocos Norte"), ("el", "Ιλόκος Νόρτε"), ("en", "Ilocos Norte"), ("es", "Ilocos Norte"), ("fa", "ایلوکوس شمالی"), ("fi", "Ilocos Norte"), ("fr", "Ilocos Norte"), ("gu", "ઇલોકોસ નોર\u{acd}ટ"), ("hi", "इलोकोस नोर\u{94d}त\u{947}"), ("id", "Ilocos Norte"), ("it", "Provincia di Ilocos Norte"), ("ja", "北イロコス州"), ("kn", "ಇಲೋಕೋಸ\u{ccd} ನಾರ\u{ccd}ಟ\u{cc6}"), ("ko", "북일로코스 주"), ("lt", "Šiaurės Ilokų regionas"), ("lv", "Ziemeļilokosa"), ("mk", "Северен Илокос"), ("mr", "इलोकोस नॉर\u{94d}ट\u{947}"), ("ms", "Ilocos Norte"), ("nb", "Ilocos Norte"), ("nl", "Ilocos Norte"), ("no", "Ilocos Norte"), ("pl", "Ilocos Norte"), ("pt", "Ilocos Norte"), ("ro", "Ilocos Norte"), ("ru", "Северный Илокос"), ("si", "ඉලොකොස\u{dca} නොර\u{dca}ටේ"), ("sv", "Norra Ilocos"), ("ta", "இலோக\u{bbe}ஸ\u{bcd} நோர\u{bcd}ட\u{bcd}"), ("te", "ఇల\u{c4b}క\u{c4b}స\u{c4d} న\u{c3e}ర\u{c4d}ట\u{c4d}"), ("th", "โลคอส นอร\u{e4c}ท"), ("tr", "Ilocos Norte"), ("uk", "Північний Ілокос"), ("ur", "ایلوکوس شمالی"), ("vi", "Ilocos Norte"), ("zh", "北伊羅戈省")]),
                        unofficial_name_list: ["Ilocos Norte"].to_vec(),
                    }
                ),
                (
                    "ILS",
                    Subdivision{
                        name: "ILS",
                        country_alpha2: Alpha2::PH,
                        code: "ILS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.2278664), longitude: Some(120.5739579), max_latitude: Some(17.9007002), min_latitude: Some(16.655479), max_longitude: Some(120.860186), min_longitude: Some(120.3427526)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إيلوكوس سور"), ("bn", "ইলোকোস স\u{9c1}র"), ("ca", "Ilocos Sur"), ("ccp", "𑄃\u{11128}𑄣\u{1112e}𑄇\u{1112e}𑄌\u{11134} 𑄥𑄢\u{11134}"), ("ceb", "Ilocos Sur"), ("da", "Ilocos Sur"), ("de", "Ilocos Sur"), ("el", "Ιλόκος Σουρ"), ("en", "Ilocos Sur"), ("es", "Ilocos Sur"), ("fa", "ایلوکوس جنوبی"), ("fi", "Ilocos Sur"), ("fr", "Ilocos Sur"), ("gu", "ઇલોકોસ સ\u{ac1}ર"), ("hi", "इलोकोस स\u{942}र"), ("id", "Ilocos Sur"), ("it", "Provincia di Ilocos Sur"), ("ja", "南イロコス州"), ("kn", "ಇಲೋಕೋಸ\u{ccd} ಸುರ\u{ccd}"), ("ko", "남일로코스 주"), ("lt", "Pietų Ilokų regionas"), ("lv", "Dienvidu Ilokosa"), ("mk", "Јужен Илокос"), ("mr", "इलोकस स\u{941}र"), ("ms", "Ilocos Sur"), ("nb", "Ilocos Sur"), ("nl", "Ilocos Sur"), ("no", "Ilocos Sur"), ("pl", "Ilocos Sur"), ("pt", "Ilocos Sur"), ("ru", "Южный Илокос"), ("si", "ඉලොකොස\u{dca} සර\u{dca}"), ("sv", "Södra Ilocos"), ("ta", "இலோகோஸ\u{bcd} சூர\u{bcd}"), ("te", "ఇల\u{c4b}క\u{c4b}స\u{c4d} సుర\u{c4d}"), ("th", "อ\u{e34}โลคอส ซ\u{e39}ร\u{e4c}"), ("tr", "Ilocos Sur"), ("uk", "Південий Ілокос"), ("ur", "ایلوکوس جنوبی"), ("vi", "Ilocos Sur"), ("zh", "南伊羅戈省")]),
                        unofficial_name_list: ["Ilocos Sur"].to_vec(),
                    }
                ),
                (
                    "ISA",
                    Subdivision{
                        name: "ISA",
                        country_alpha2: Alpha2::PH,
                        code: "ISA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.9753758), longitude: Some(121.8107079), max_latitude: Some(17.5454246), min_latitude: Some(16.3794739), max_longitude: Some(122.5278068), min_longitude: Some(121.3364481)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ايزابلا"), ("bn", "ইস\u{9be}বেল\u{9be}"), ("ca", "Isabela"), ("ccp", "𑄃\u{11128}𑄥𑄝𑄬𑄣"), ("ceb", "Isabela"), ("da", "Isabela"), ("de", "Isabela"), ("el", "Ισαμπέλα"), ("en", "Isabela"), ("es", "Isabela"), ("fa", "ایزابلا"), ("fi", "Isabela"), ("fr", "Isabela"), ("gu", "ઇસાબ\u{ac7}લા"), ("hi", "इसाब\u{947}ला प\u{94d}रान\u{94d}त"), ("id", "Isabela"), ("it", "Provincia di Isabela"), ("ja", "イサベラ州"), ("kn", "ಇಸಾಬ\u{cc6}ಲಾ"), ("ko", "이사벨라 주"), ("lt", "Izabela"), ("lv", "Isabela"), ("mk", "Изабела"), ("mr", "इसाब\u{947}ला"), ("ms", "Isabela"), ("nb", "Isabela"), ("nl", "Isabela"), ("no", "Isabela"), ("pl", "Isabela"), ("pt", "Isabela"), ("ru", "Исабела"), ("si", "ඉසබෙල\u{dcf}"), ("sv", "Isabela"), ("ta", "இசபெல\u{bcd}ல\u{bbe}"), ("te", "ఇసబ\u{c46}ల\u{c4d}ల\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e34}ซาเบลา"), ("tr", "Isabela"), ("ur", "آئزابیلا (صوبہ)"), ("vi", "Isabela"), ("zh", "伊莎貝拉省")]),
                        unofficial_name_list: ["Isabela"].to_vec(),
                    }
                ),
                (
                    "KAL",
                    Subdivision{
                        name: "KAL",
                        country_alpha2: Alpha2::PH,
                        code: "KAL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.4740422), longitude: Some(121.3541631), max_latitude: Some(17.6889911), min_latitude: Some(17.1616239), max_longitude: Some(121.682251), min_longitude: Some(120.909971)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كالينغا"), ("bn", "ক\u{9be}লিংগ\u{9be}"), ("ca", "província de Kalinga"), ("ccp", "𑄇𑄣\u{11128}\u{11101}𑄉"), ("ceb", "Kalinga"), ("da", "Kalinga"), ("de", "Kalinga"), ("el", "Καλίνγκα"), ("en", "Kalinga"), ("es", "Kalinga"), ("fa", "کالینگا"), ("fi", "Kalingan provinssi"), ("fr", "province de Kalinga"), ("gu", "કલિ\u{a82}ગા"), ("hi", "कालि\u{902}गा प\u{94d}रान\u{94d}त"), ("id", "Kalinga"), ("it", "Provincia di Kalinga"), ("ja", "カリンガ州"), ("kn", "ಕಳ\u{cbf}ಂಗ"), ("ko", "칼링가 주"), ("lt", "Kalinga"), ("lv", "Kalinga"), ("mk", "Калинга"), ("mr", "कलि\u{902}गा"), ("ms", "Kalinga"), ("nb", "Kalinga"), ("nl", "Kalinga"), ("no", "Kalinga"), ("or", "କଳ\u{b3f}ଙ\u{b4d}ଗ (ର\u{b3e}ଜ\u{b4d}ୟ), ଫ\u{b3f}ଲ\u{b3f}ପ\u{b3e}ଇନ\u{b4d}ସ"), ("pl", "Kalinga"), ("pt", "Província de Kalinga"), ("ru", "Калинга"), ("si", "ක\u{dcf}ල\u{dd2}ංග"), ("sv", "Kalinga"), ("ta", "கலிங\u{bcd}க\u{bbe}"), ("te", "కళ\u{c3f}ంగ"), ("th", "คาล\u{e34}นก\u{e49}า"), ("tr", "Kalinga"), ("uk", "Провінція Калінга"), ("ur", "کالینگا"), ("vi", "Kalinga"), ("zh", "卡林阿省")]),
                        unofficial_name_list: ["Kalinga"].to_vec(),
                    }
                ),
                (
                    "LAG",
                    Subdivision{
                        name: "LAG",
                        country_alpha2: Alpha2::PH,
                        code: "LAG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.2956294), longitude: Some(121.4961581), max_latitude: Some(14.3487163), min_latitude: Some(14.2647886), max_longitude: Some(121.630674), min_longitude: Some(121.4209908)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لاغونا"), ("bn", "লেগ\u{9c1}ন\u{9be}"), ("ca", "Laguna"), ("ccp", "𑄣\u{11133}𑄠𑄉\u{1112a}𑄚"), ("ceb", "Laguna"), ("da", "Laguna"), ("de", "Laguna"), ("el", "Λαγκούνα"), ("en", "Laguna"), ("es", "La Laguna"), ("eu", "Laguna probintzia"), ("fa", "لاگونا"), ("fi", "Laguna"), ("fr", "Laguna"), ("gu", "લગ\u{ac1}ના"), ("hi", "लग\u{942}ना प\u{94d}रान\u{94d}त"), ("id", "Laguna"), ("it", "Provincia di Laguna"), ("ja", "ラグナ州"), ("ka", "ლაგუნა"), ("kn", "ಲಗುನಾ"), ("ko", "라구나 주"), ("lt", "Laguna"), ("lv", "Laguna"), ("mk", "Лагуна"), ("mr", "लाग\u{941}ना"), ("ms", "Laguna"), ("nb", "Laguna"), ("nl", "Laguna"), ("no", "Laguna"), ("pl", "Laguna"), ("pt", "Laguna"), ("ru", "Лагуна"), ("si", "ලග\u{dd4}න\u{dcf}"), ("sv", "Laguna"), ("ta", "லகுன\u{bbe}"), ("te", "ల\u{c3e}గున\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดลาก\u{e39}น\u{e48}า"), ("tr", "Laguna"), ("uk", "Лагуна"), ("ur", "لاگونا (صوبہ)"), ("vi", "Laguna"), ("zh", "內湖省")]),
                        unofficial_name_list: ["Laguna"].to_vec(),
                    }
                ),
                (
                    "LAN",
                    Subdivision{
                        name: "LAN",
                        country_alpha2: Alpha2::PH,
                        code: "LAN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.8721811), longitude: Some(123.8857747), max_latitude: Some(8.3414729), min_latitude: Some(7.6934621), max_longitude: Some(124.573305), min_longitude: Some(123.621328)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لاناو ديل نورتي"), ("bn", "ল\u{9be}ন\u{9be}ও ডেল নর\u{9cd}টে"), ("ccp", "𑄣𑄚𑄃\u{1112e} 𑄓𑄬𑄣\u{11134} 𑄚\u{11127}𑄢\u{11134}𑄑𑄬"), ("ceb", "Lanao del Norte"), ("da", "Lanao del Norte"), ("de", "Lanao del Norte"), ("el", "Λανάο ντελ Νόρτε"), ("en", "Lanao del Norte"), ("es", "Lanao del Norte"), ("fa", "لانائو شمالی"), ("fi", "Lanao del Norte"), ("fr", "Lanao du Nord"), ("gu", "લાનાઓ ડ\u{ac7}લ નોર\u{acd}ટ"), ("hi", "लानाओ द\u{947}ल नोर\u{94d}त\u{947}"), ("id", "Lanao del Norte"), ("it", "Provincia di Lanao del Norte"), ("ja", "北ラナオ州"), ("kn", "ಲಾನೊ ಡ\u{cc6}ಲ\u{ccd} ನಾರ\u{ccd}ಟ\u{cc6}"), ("ko", "북라나오 주"), ("lt", "Šiaurės Lanao"), ("lv", "Ziemeļlanao"), ("mk", "Северен Ланао"), ("mr", "लानाओ ड\u{947}ल नॉर\u{94d}ट"), ("ms", "Lanao del Norte"), ("nb", "Lanao del Norte"), ("nl", "Lanao del Norte"), ("no", "Lanao del Norte"), ("pl", "Lanao del Norte"), ("pt", "Lanão do Norte"), ("ru", "Северный Ланао"), ("si", "ලන\u{dcf}ඕ ඩෙල\u{dca} නොර\u{dca}ටේ"), ("sv", "Lanao del Norte"), ("ta", "லெனோவோ டெல\u{bcd} நோர\u{bcd}ட\u{bcd}டே"), ("te", "ల\u{c3e}న\u{c3e}వ\u{c4b} డ\u{c46}ల\u{c4d} న\u{c3e}ర\u{c4d}ట\u{c46}"), ("th", "ลานาว เดล เหน\u{e37}อ"), ("tr", "Lanao del Norte"), ("uk", "Північний Ланао"), ("ur", "لاناؤ شمالی"), ("vi", "Lanao del Norte"), ("zh", "北拉瑙省")]),
                        unofficial_name_list: ["Lanao del Norte"].to_vec(),
                    }
                ),
                (
                    "LAS",
                    Subdivision{
                        name: "LAS",
                        country_alpha2: Alpha2::PH,
                        code: "LAS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.823176), longitude: Some(124.4198243), max_latitude: Some(8.2121889), min_latitude: Some(7.3995557), max_longitude: Some(124.807193), min_longitude: Some(123.8264541)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لاناو ديل سور"), ("bn", "ল\u{9be}ন\u{9be}ও দেল স\u{9c1}র"), ("ccp", "𑄣𑄚𑄃\u{1112e} 𑄓𑄬𑄣\u{11134} 𑄥𑄢\u{11134}"), ("ceb", "Lanao del Sur"), ("da", "Lanao del Sur"), ("de", "Lanao del Sur"), ("el", "Λανάο ντελ Σουρ"), ("en", "Lanao del Sur"), ("es", "Lanao del Sur"), ("fa", "لانائو جنوبی"), ("fi", "Lanao del Sur"), ("fr", "Lanao du Sud"), ("gu", "લાનાઓ ડ\u{ac7}લ સ\u{ac1}ર"), ("hi", "लानाओ द\u{947}ल स\u{942}र"), ("id", "Lanao del Sur"), ("it", "Provincia di Lanao del Sur"), ("ja", "南ラナオ州"), ("kn", "ಲಾನೊ ಡ\u{cc6}ಲ\u{ccd} ಸುರ\u{ccd}"), ("ko", "남라나오 주"), ("lt", "Pietų Lanao"), ("lv", "Lanaodelsura"), ("mk", "Јужен Ланао"), ("mr", "लानाओ ड\u{947}ल स\u{941}र"), ("ms", "Lanao del Sur"), ("nb", "Lanao del Sur"), ("nl", "Lanao del Sur"), ("no", "Lanao del Sur"), ("pl", "Lanao del Sur"), ("pt", "Lanão de Sur"), ("ru", "Южный Ланао"), ("si", "ලන\u{dcf}ඕ ඩෙල\u{dca} සර\u{dca}"), ("sv", "Lanao del Sur"), ("ta", "ல\u{bbe}ன\u{bbe}யோ டேல\u{bcd} சூர\u{bcd}"), ("te", "ల\u{c3e}న\u{c3e}వ\u{c4b} డ\u{c46}ల\u{c4d} సుర\u{c4d}"), ("th", "ลาเนา เดล เซอร\u{e4c}"), ("tr", "Lanao del Sur"), ("uk", "Південний Ланао"), ("ur", "لاناؤ دل سور"), ("vi", "Lanao del Sur"), ("zh", "南拉瑙省")]),
                        unofficial_name_list: ["Lanao del Sur"].to_vec(),
                    }
                ),
                (
                    "LEY",
                    Subdivision{
                        name: "LEY",
                        country_alpha2: Alpha2::PH,
                        code: "LEY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.366667), longitude: Some(124.483333), max_latitude: Some(11.4561949), min_latitude: Some(11.2012197), max_longitude: Some(124.5601559), min_longitude: Some(124.3958759)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ليتة"), ("bn", "লেতে"), ("ca", "Província de Leyte"), ("ccp", "𑄣𑄬𑄠𑄖\u{11134}"), ("ceb", "Leyte"), ("da", "Leyte"), ("de", "Leyte"), ("el", "Λέγτε"), ("en", "Leyte"), ("es", "Leyte"), ("fa", "لیته"), ("fi", "Leyte"), ("fr", "Leyte"), ("gu", "લ\u{ac7}ટ\u{ac7}"), ("hi", "ल\u{947}यत\u{947} प\u{94d}रान\u{94d}त"), ("id", "Leyte Utara"), ("it", "Provincia di Leyte"), ("ja", "レイテ州"), ("kn", "ಲೇಯ\u{ccd}ಟ\u{cc6}"), ("ko", "레이테 주"), ("lt", "Leitė"), ("lv", "Leite"), ("mk", "Лејте"), ("mr", "ल\u{947}य\u{947}"), ("ms", "Leyte"), ("nb", "Leyte"), ("nl", "Leyte"), ("no", "Leyte"), ("pl", "Prowincja Leyte"), ("pt", "Leyte"), ("ru", "Лейте"), ("si", "ලේටේ"), ("sv", "Leyte"), ("ta", "லெய\u{bcd}டி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c48}ట\u{c3f}"), ("th", "เกาะเลย\u{e4c}เต"), ("tr", "Leyte"), ("uk", "Провінція Лейте"), ("ur", "لیئتے (صوبہ)"), ("vi", "Leyte"), ("zh", "雷伊泰省")]),
                        unofficial_name_list: ["Leyte"].to_vec(),
                    }
                ),
                (
                    "LUN",
                    Subdivision{
                        name: "LUN",
                        country_alpha2: Alpha2::PH,
                        code: "LUN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.6158906), longitude: Some(120.3209373), max_latitude: Some(16.91881), min_latitude: Some(16.207748), max_longitude: Some(120.5744889), min_longitude: Some(120.2801715)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لا يونيون"), ("bn", "ল\u{9be} ইউনিয\u{9bc}ন"), ("ca", "La Union"), ("ccp", "𑄣 𑄃\u{11128}𑄅\u{1112a}𑄚\u{11128}𑄠\u{11127}𑄚\u{11134}"), ("ceb", "La Union"), ("da", "La Union"), ("de", "La Union"), ("el", "Λα Γιούνιον"), ("en", "La Union"), ("es", "La Unión"), ("fa", "لا یونیون، پرو"), ("fi", "La Union"), ("fr", "La Union"), ("gu", "લા ય\u{ac1}નિયન"), ("hi", "ला य\u{941}नियोन प\u{94d}रान\u{94d}त"), ("id", "La Union"), ("it", "Provincia di La Union"), ("ja", "ラウニオン州"), ("kn", "ಲಾ ಯ\u{cc2}ನ\u{cbf}ಯನ\u{ccd}"), ("ko", "라우니온 주"), ("lt", "La Junion"), ("lv", "Laūniona"), ("mk", "Ла Унион"), ("mr", "ला य\u{941}नियन"), ("ms", "La Union"), ("nb", "La Union"), ("nl", "La Union"), ("no", "La Union"), ("pl", "La Union"), ("pt", "La Unión"), ("ru", "Ла-Унион"), ("si", "ල\u{dcf} ය\u{dd4}න\u{dd2}යන\u{dca}"), ("sv", "La Union"), ("ta", "ல\u{bbe} யூனியன\u{bcd}"), ("te", "ల\u{c3e} యూన\u{c3f}యన\u{c4d}"), ("th", "ลา ย\u{e39}เน\u{e35}ยน"), ("tr", "La Union"), ("uk", "Ла Уніон"), ("ur", "لا یونین"), ("vi", "La Union"), ("zh", "聯合省")]),
                        unofficial_name_list: ["La Union"].to_vec(),
                    }
                ),
                (
                    "MAD",
                    Subdivision{
                        name: "MAD",
                        country_alpha2: Alpha2::PH,
                        code: "MAD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.4767171), longitude: Some(121.9032192), max_latitude: Some(13.5706939), min_latitude: Some(13.1990285), max_longitude: Some(122.1491437), min_longitude: Some(121.80645)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ماريندوك"), ("az", "Marinduke"), ("bn", "ম\u{9be}রিন\u{9cd}দ\u{9c1}ক"), ("ca", "Marinduque"), ("ccp", "𑄟𑄢\u{11128}𑄚\u{11134}𑄓\u{1112a}𑄇\u{1112a}"), ("ceb", "Marinduque"), ("da", "Marinduque"), ("de", "Marinduque"), ("el", "Μαριντούκουε"), ("en", "Marinduque"), ("es", "Marinduque"), ("fa", "ماریندوک"), ("fi", "Marinduque"), ("fr", "province de Marinduque"), ("gl", "Marinduque"), ("gu", "મારિન\u{acd}દ\u{ac1}ક"), ("hi", "मरिनद\u{942}क\u{947}"), ("hr", "Marinduque"), ("id", "Marinduque"), ("it", "Provincia di Marinduque"), ("ja", "マリンドゥケ州"), ("kn", "ಮರ\u{cbf}ಂಡುಕ\u{ccd}ಯ\u{cc2}"), ("ko", "마린두케 주"), ("lt", "Marindukė"), ("lv", "Marinduke"), ("mk", "Мариндуке"), ("ml", "മ\u{d3e}റിൻദ\u{d4d}യ\u{d42}ഖ\u{d4d}"), ("mr", "मरि\u{902}द\u{941}क"), ("ms", "Marinduque"), ("nb", "Marinduque"), ("nl", "Marinduque"), ("no", "Marinduque"), ("pl", "Marinduque"), ("pt", "Marinduque"), ("ru", "Мариндуке"), ("si", "මර\u{dd2}න\u{dca}ඩ\u{dd4}කේ"), ("sv", "Marinduque"), ("ta", "ம\u{bbe}றிண\u{bcd}டுயூ"), ("te", "మ\u{c3e}ర\u{c3f}ండూక\u{c4d}వ\u{c46}"), ("th", "มาร\u{e34}นด\u{e39}ค"), ("tr", "Marinduque"), ("uk", "Маріндук"), ("ur", "ماریندوک"), ("vi", "Marinduque"), ("zh", "馬林杜克省")]),
                        unofficial_name_list: ["Marinduque"].to_vec(),
                    }
                ),
                (
                    "MAG",
                    Subdivision{
                        name: "MAG",
                        country_alpha2: Alpha2::PH,
                        code: "MAG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.9422581), longitude: Some(124.4198243), max_latitude: Some(7.651929099999999), min_latitude: Some(6.649818499999999), max_longitude: Some(125.120551), min_longitude: Some(123.943766)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ماغوييندانايو"), ("bn", "ম\u{9be}গ\u{9c1}ইন\u{9cd}দ\u{9be}ন\u{9be}ও"), ("ca", "Maguindanao"), ("ccp", "𑄟𑄬𑄉\u{1112d}\u{1112a}𑄚\u{11134}𑄓𑄚𑄃\u{1112e}"), ("ceb", "Maguindanao"), ("da", "Maguindanao"), ("de", "Maguindanao"), ("el", "Μαγκουιντανάο"), ("en", "Maguindanao"), ("es", "Maguindanao"), ("fa", "ماگوئیندانائو"), ("fi", "Maguindanao"), ("fr", "Maguindanao"), ("gl", "Provincia de Maguindanao"), ("gu", "માગ\u{ac1}ઈ\u{a82}દાનાઓ"), ("hi", "मगिन\u{94d}दानाओ प\u{94d}रान\u{94d}त"), ("id", "Maguindanao"), ("it", "Provincia di Maguindanao"), ("ja", "マギンダナオ州"), ("kn", "ಮಗುಂದಾನೊ"), ("ko", "마긴다나오 주"), ("lt", "Magindanas"), ("lv", "Magvindanao"), ("mk", "Магинданао"), ("mr", "माग\u{942}इनदानाओ"), ("ms", "Maguindanao"), ("nb", "Maguindanao"), ("nl", "Maguindanao"), ("no", "Maguindanao"), ("pl", "Maguindanao"), ("pt", "Maguindanao"), ("ru", "Магинданао"), ("si", "මග\u{dd4}ඉන\u{dca}ඩන\u{dcf}ඕ"), ("sv", "Maguindanao"), ("ta", "மகின\u{bcd}டன\u{bbe}வ\u{bcd}"), ("te", "మ\u{c3e}గ\u{c4d}వ\u{c3f}ండ\u{c3e}నవ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมาก\u{e39}อ\u{e34}นดาเนา"), ("tr", "Maguindanao"), ("uk", "Магінданао"), ("ur", "ماگوئنداناؤ"), ("vi", "Maguindanao"), ("zh", "馬京達瑙省")]),
                        unofficial_name_list: ["Maguindanao"].to_vec(),
                    }
                ),
                (
                    "MAS",
                    Subdivision{
                        name: "MAS",
                        country_alpha2: Alpha2::PH,
                        code: "MAS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.366667), longitude: Some(123.616667), max_latitude: Some(12.4616029), min_latitude: Some(12.2440624), max_longitude: Some(123.6398035), min_longitude: Some(123.4894181)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ماسبات"), ("bn", "ম\u{9be}সবেট"), ("ca", "Masbate"), ("ccp", "𑄟𑄌\u{11134}𑄝𑄬𑄖\u{11134}"), ("ceb", "Masbate"), ("da", "Masbate"), ("de", "Masbate"), ("el", "Μασμπάτε"), ("en", "Masbate"), ("es", "Masbate"), ("fa", "ماسباته"), ("fi", "Masbate"), ("fr", "Masbate"), ("gu", "માસબ\u{ac7}ટ\u{ac7}"), ("hi", "मस\u{94d}बात\u{947}"), ("hr", "Masbate"), ("hu", "Masbate"), ("id", "Masbate"), ("it", "Provincia di Masbate"), ("ja", "マスバテ州"), ("kn", "ಮಾಸ\u{ccd}ಬೇಟ\u{ccd}"), ("ko", "마스바테 주"), ("lt", "Masbatė"), ("lv", "Masbate"), ("mk", "Масбате"), ("mr", "मासबाट"), ("ms", "Masbate"), ("nb", "Masbate"), ("nl", "Masbate"), ("no", "Masbate"), ("pl", "Masbate"), ("pt", "Masbate"), ("ru", "Масбате"), ("si", "මස\u{dca}බටේ"), ("sv", "Masbate"), ("ta", "ம\u{bbe}சுபட\u{bcd}டே"), ("te", "మ\u{c3e}స\u{c4d}ప\u{c47}ట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดม\u{e31}สบาเต"), ("tr", "Masbate"), ("uk", "Масбате"), ("ur", "ماسبات"), ("vi", "Masbate"), ("yue", "馬士弼"), ("yue_Hans", "马士弼"), ("zh", "馬斯巴特省")]),
                        unofficial_name_list: ["Masbate"].to_vec(),
                    }
                ),
                (
                    "MDC",
                    Subdivision{
                        name: "MDC",
                        country_alpha2: Alpha2::PH,
                        code: "MDC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.1024111), longitude: Some(120.7651284), max_latitude: Some(13.8989589), min_latitude: Some(12.1530185), max_longitude: Some(121.250199), min_longitude: Some(120.0174402)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أوكسيدنتال ميندورو"), ("bn", "অক\u{9cd}সিডেন\u{9cd}ট\u{9be}ল মিন\u{9cd}ডোলো"), ("ca", "Mindoro Occidental"), ("ccp", "𑄃\u{11127}𑄇\u{11134}𑄥\u{11128}𑄓𑄬𑄚\u{11134}𑄑𑄣\u{11134} 𑄟\u{11128}𑄚\u{11134}𑄓\u{1112e}𑄢\u{1112e}"), ("ceb", "Kasadpang Mindoro"), ("da", "Occidental Mindoro"), ("de", "Occidental Mindoro"), ("el", "Οξιντένταλ Μιντόρο"), ("en", "Occidental Mindoro"), ("es", "Mindoro Occidental"), ("fa", "میندورو غربی"), ("fi", "Occidental Mindoro"), ("fr", "Mindoro occidental"), ("gu", "ઓક\u{acd}સિડ\u{ac7}\u{a82}ટલ મિન\u{acd}ડૉરો"), ("hi", "ओक\u{94d}सिड\u{947}न\u{94d}टल मिन\u{94d}दोरो"), ("id", "Occidental Mindoro"), ("it", "Provincia di Occidental Mindoro"), ("ja", "西ミンドロ州"), ("kn", "ಆಕ\u{ccd}ಸ\u{cbf}ಡ\u{cc6}ಂಟಲ\u{ccd} ಮ\u{cbf}ಂಡೋರೋ"), ("ko", "서민도로 주"), ("lt", "Vakarų Mindoras"), ("lv", "Rietumu Mindoro"), ("mk", "Западен Миндоро"), ("mr", "ओक\u{945}सिड\u{947}\u{902}टल मिन\u{94d}डॉरो"), ("ms", "Occidental Mindoro"), ("nb", "Occidental Mindoro"), ("nl", "Occidental Mindoro"), ("no", "Occidental Mindoro"), ("pl", "Occidental Mindoro"), ("pt", "Mindoro Ocidental"), ("ru", "Западный Миндоро"), ("si", "ඔක\u{dca}ස\u{dd2}ඩෙන\u{dca}ටල\u{dca} ම\u{dd2}න\u{dca}ඩොරෝ"), ("sv", "Occidental Mindoro"), ("ta", "அஸிடெண\u{bcd}ட\u{bcd}ட\u{bbe}ள\u{bcd} மிண\u{bcd}டோரோ"), ("te", "ఓక\u{c4d}స\u{c3f}డ\u{c46}ంటల\u{c4d} మ\u{c3f}ండ\u{c4b}ర\u{c4b}"), ("th", "ออคซ\u{e34}เดนทอล ม\u{e34}นโดโร"), ("tr", "Occidental Mindoro"), ("uk", "Західний Міндоро"), ("ur", "غربی میندورو"), ("vi", "Occidental Mindoro"), ("zh", "西民都洛省")]),
                        unofficial_name_list: ["Mindoro Occidental"].to_vec(),
                    }
                ),
                (
                    "MDR",
                    Subdivision{
                        name: "MDR",
                        country_alpha2: Alpha2::PH,
                        code: "MDR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.0564598), longitude: Some(121.4069417), max_latitude: Some(13.5314771), min_latitude: Some(12.1951624), max_longitude: Some(121.5576218), min_longitude: Some(120.8007199)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أورينتال ميندورو"), ("bn", "ওরিয\u{9bc}েন\u{9cd}ট\u{9be}ল মিন\u{9cd}ডোরো"), ("ca", "Mindoro Oriental"), ("ccp", "𑄃\u{11127}𑄢\u{11128}𑄠𑄬𑄚\u{11134}𑄑𑄣\u{11134} 𑄟\u{11128}𑄚\u{11134}𑄓\u{1112e}𑄢\u{1112e}"), ("ceb", "Sidlakang Mindoro"), ("da", "Oriental Mindoro"), ("de", "Oriental Mindoro"), ("el", "Οριεντάλ Μιντόρο"), ("en", "Oriental Mindoro"), ("es", "Mindoro Oriental"), ("fa", "میندورو شرقی"), ("fi", "Oriental Mindoro"), ("fr", "Mindoro oriental"), ("gu", "ઓરિએન\u{acd}ટલ મિન\u{acd}ડોરો"), ("hi", "ओरिय\u{947}न\u{94d}टल मिन\u{94d}दोरो"), ("id", "Oriental Mindoro"), ("it", "Provincia di Oriental Mindoro"), ("ja", "東ミンドロ州"), ("kn", "ಓರ\u{cbf}ಯ\u{cc6}ಂಟಲ\u{ccd} ಮ\u{cbf}ಂಡೋರೊ"), ("ko", "동민도로 주"), ("lt", "Rytų Mindoras"), ("lv", "Austrumu Mindoro"), ("mk", "Источен Миндоро"), ("mr", "ओरिएन\u{94d}टल मि\u{902}डोरो"), ("ms", "Oriental Mindoro"), ("nb", "Oriental Mindoro"), ("nl", "Oriental Mindoro"), ("no", "Oriental Mindoro"), ("pl", "Oriental Mindoro"), ("pt", "Oriental Mindoro"), ("ru", "Восточный Миндоро"), ("si", "ඔර\u{dd2}යන\u{dca}ටල\u{dca} ම\u{dd2}න\u{dca}ඩොරෝ"), ("sv", "Oriental Mindoro"), ("ta", "ஓரியண\u{bcd}டல\u{bcd} மிண\u{bcd}டோரோ"), ("te", "ఓర\u{c3f}యంటల\u{c4d} మ\u{c3f}ండ\u{c4b}ర\u{c4b}"), ("th", "โอรเ\u{e35}ยนทอล ม\u{e34}นโดโร"), ("tr", "Oriental Mindoro"), ("uk", "Східний Міндоро"), ("ur", "شرقی میندورو"), ("vi", "Oriental Mindoro"), ("zh", "東民都洛省")]),
                        unofficial_name_list: ["Mindoro Oriental"].to_vec(),
                    }
                ),
                (
                    "MOU",
                    Subdivision{
                        name: "MOU",
                        country_alpha2: Alpha2::PH,
                        code: "MOU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.0663429), longitude: Some(121.03351), max_latitude: Some(17.306318), min_latitude: Some(16.822498), max_longitude: Some(121.5659461), min_longitude: Some(120.770595)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ماونتين"), ("bn", "ম\u{9be}উন\u{9cd}টেইন প\u{9cd}রদেশ"), ("ca", "Mountain Province"), ("ccp", "𑄟𑄅\u{1112a}𑄚\u{11134}𑄑𑄬𑄚\u{11134}"), ("ceb", "Lalawigang Bulubundukin"), ("da", "Mountain Province"), ("de", "Mountain Province"), ("el", "Μάουντεν"), ("en", "Mountain"), ("es", "La Montaña"), ("fa", "ماونتین"), ("fi", "Mountain Province"), ("fr", "Mountain Province"), ("gu", "માઉન\u{acd}ટ\u{ac7}ન પ\u{acd}રા\u{a82}ત"), ("hi", "माउ\u{902}टन प\u{94d}रान\u{94d}त"), ("id", "Provinsi Pegunungan"), ("it", "Mountain Province"), ("ja", "マウンテン州"), ("kn", "ಪರ\u{ccd}ವತ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "마운틴 주"), ("lt", "Kalnų provincija"), ("lv", "Kalnu province"), ("mk", "Планинска покраина"), ("mr", "माउ\u{902}टन प\u{94d}रा\u{902}त"), ("ms", "Mountain Province"), ("nb", "Bergsprovinsen"), ("nl", "Mountain Province"), ("no", "Bergsprovinsen"), ("pl", "Mountain Province"), ("pt", "Província Mountain"), ("ru", "Горная провинция"), ("si", "කඳ\u{dd4}කර පළ\u{dcf}ත"), ("sv", "Bergsprovinsen"), ("ta", "மௌண\u{bcd}டைன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c4c}ంట\u{c46}య\u{c3f}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมอนเทรน"), ("tr", "Dağ Bölgesi"), ("uk", "Гірська провінція"), ("ur", "ماؤنٹین صوبہ"), ("vi", "Mountain Province"), ("zh", "高山省")]),
                        unofficial_name_list: ["Mountain Province"].to_vec(),
                    }
                ),
                (
                    "MSC",
                    Subdivision{
                        name: "MSC",
                        country_alpha2: Alpha2::PH,
                        code: "MSC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.337490299999999), longitude: Some(123.7070619), max_latitude: Some(8.668944699999999), min_latitude: Some(8.01499), max_longitude: Some(123.8690157), min_longitude: Some(123.5556264)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ميساميس أوتشيدنتال"), ("bn", "মিস\u{9be}মিস ওক\u{9cd}সিডেন\u{9cd}ট\u{9be}ল"), ("ca", "Misamis Occidental"), ("ccp", "𑄟\u{11128}𑄥𑄟\u{11128}𑄌\u{11134} 𑄃\u{11127}𑄇\u{11134}𑄥\u{11128}𑄓𑄬𑄚\u{11134}𑄑𑄣\u{11134}"), ("ceb", "Misamis Occidental"), ("da", "Misamis Occidental"), ("de", "Misamis Occidental"), ("el", "Μισάμις"), ("en", "Misamis Occidental"), ("es", "Misamis Occidental"), ("fa", "میسامی غربی"), ("fi", "Misamis Occidental"), ("fr", "Misamis occidental"), ("gu", "મીસ\u{ac7}મિસ ઓક\u{acd}સીડ\u{ac7}ન\u{acd}ટલ"), ("hi", "मिसामिस ओक\u{94d}सीद\u{947}न\u{94d}ताल प\u{94d}रान\u{94d}त"), ("id", "Misamis Occidental"), ("it", "Provincia di Misamis Occidental"), ("ja", "西ミサミス州"), ("kn", "ಮ\u{cbf}ಸ\u{ccd}ಸಾಮ\u{cbf}ಸ\u{ccd} ಆಕೇಶನಲ\u{ccd}"), ("ko", "서미사미스 주"), ("lt", "Rytų Misamisas"), ("lv", "Rietummisamisa"), ("mk", "Западен Мисамис"), ("mr", "मिसामिस ऑक\u{94d}सिड\u{947}\u{902}टल"), ("ms", "Misamis Occidental"), ("nb", "Misamis Occidental"), ("nl", "Misamis Occidental"), ("no", "Misamis Occidental"), ("pl", "Misamis Occidental"), ("pt", "Misamis Occidental"), ("ru", "Западный Мисамис"), ("si", "ම\u{dd2}ස\u{dcf}ම\u{dd2}ස\u{dca} ඔක\u{dca}ස\u{dd2}ඩේන\u{dca}ටල\u{dca}"), ("sv", "Misamis Occidental"), ("ta", "மிச\u{bcd}சமிஸ\u{bcd} அஸிடெண\u{bcd}ட\u{bcd}ட\u{bbe}ல\u{bcd}"), ("te", "మ\u{c3f}స\u{c3e}మ\u{c3f}స\u{c4d} ఆక\u{c4d}స\u{c3f}డ\u{c46}ంటల\u{c4d}"), ("th", "บาเยลซ\u{e48}า"), ("tr", "Misamis Occidental"), ("uk", "Західний Місаміс"), ("ur", "میسامس غربی"), ("vi", "Misamis Occidental"), ("zh", "西米薩米斯省")]),
                        unofficial_name_list: ["Misamis Occidental"].to_vec(),
                    }
                ),
                (
                    "MSR",
                    Subdivision{
                        name: "MSR",
                        country_alpha2: Alpha2::PH,
                        code: "MSR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.5045558), longitude: Some(124.6219592), max_latitude: Some(9.0931017), min_latitude: Some(8.2358519), max_longitude: Some(125.2555001), min_longitude: Some(124.2491)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ميساميس أوريانتال"), ("bn", "মিস\u{9be}মিস ওরিয\u{9bc}েন\u{9cd}ট\u{9be}ল"), ("ca", "Misamis Oriental"), ("ccp", "𑄟\u{11128}𑄥𑄟\u{11128}𑄌\u{11134} 𑄃\u{11127}𑄢\u{11128}𑄠𑄬𑄚\u{11134}𑄑𑄣\u{11134}"), ("ceb", "Misamis Oriental"), ("da", "Misamis Oriental"), ("de", "Misamis Oriental"), ("el", "Μίσαμις Οριένταλ"), ("en", "Misamis Oriental"), ("es", "Misamis Oriental"), ("fa", "میسامی شرقی"), ("fi", "Misamis Oriental"), ("fr", "Misamis oriental"), ("gu", "મિસામિસ ઓરિએન\u{acd}ટલ"), ("hi", "मिसामिस ओरिएन\u{94d}ताल प\u{94d}रान\u{94d}त"), ("id", "Misamis Oriental"), ("it", "Provincia di Misamis Oriental"), ("ja", "東ミサミス州"), ("kn", "ಮ\u{cbf}ಸಾಮ\u{cbf}ಸ\u{ccd} ಓರ\u{cbf}ಯ\u{cc6}ಂಟಲ\u{ccd}"), ("ko", "동미사미스 주"), ("lt", "Rytų Misamis"), ("lv", "Austrumu Misamisa"), ("mk", "Источен Мисамис"), ("mr", "मिसामिस ओरिए\u{902}टल"), ("ms", "Misamis Oriental"), ("nb", "Misamis Oriental"), ("nl", "Misamis Oriental"), ("no", "Misamis Oriental"), ("pl", "Misamis Oriental"), ("pt", "Misamis Oriental"), ("ru", "Восточный Мисамис"), ("si", "ම\u{dd2}ස\u{dcf}ම\u{dd2}ස\u{dca} ඔර\u{dd2}යන\u{dca}ටල\u{dca}"), ("sv", "Misamis Oriental"), ("ta", "மிஸமிஸ\u{bcd} ஒரிஎண\u{bcd}டல\u{bcd}"), ("te", "మ\u{c3f}స\u{c3e}మ\u{c3f}స\u{c4d} ఓర\u{c3f}యంటల\u{c4d}"), ("th", "ม\u{e34}ซาม\u{e34}สโอเร\u{e35}ยนท\u{e31}ล"), ("tr", "Misamis Oriental"), ("uk", "Східний Місаміс"), ("ur", "میسامس شرقی"), ("vi", "Misamis Oriental"), ("zh", "東米薩米斯省")]),
                        unofficial_name_list: ["Misamis Oriental"].to_vec(),
                    }
                ),
                (
                    "NCO",
                    Subdivision{
                        name: "NCO",
                        country_alpha2: Alpha2::PH,
                        code: "NCO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.1083349), longitude: Some(125.0388164), max_latitude: Some(7.681884999999999), min_latitude: Some(6.7588269), max_longitude: Some(125.31497), min_longitude: Some(124.3334349)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كوتاباتو"), ("bn", "কিোত\u{9be}ব\u{9be}তো"), ("ca", "Cotabato"), ("ccp", "𑄇\u{1112e}𑄑𑄝𑄑\u{1112e}"), ("ceb", "Cotabato"), ("da", "Cotabato"), ("de", "Cotabato"), ("el", "Κοταμπάτο"), ("en", "Cotabato"), ("es", "Cotabato"), ("fa", "کوتاباتو"), ("fi", "Cotabato"), ("fr", "Cotabato"), ("gu", "કોટાબ\u{ac7}ટો"), ("hi", "कोताबातो प\u{94d}रान\u{94d}त"), ("id", "Cotabato"), ("it", "Provincia di Cotabato"), ("ja", "コタバト州"), ("kn", "ಕೋಟಾಬಾಟೊ"), ("ko", "코타바토 주"), ("lt", "Kotabatas"), ("lv", "Kotabato"), ("mk", "Котабато"), ("mr", "कोटाब\u{947}तो"), ("ms", "Cotabato"), ("nb", "Cotabato"), ("nl", "Cotabato"), ("no", "Cotabato"), ("pl", "Cotabato"), ("pt", "Cotabato"), ("ru", "Котабато"), ("si", "කොටබොටෝ"), ("sv", "Cotabato"), ("ta", "க\u{bbe}ட\u{bcd}டப\u{bcd}பட\u{bcd}ட"), ("te", "క\u{c4b}ట\u{c3e}బ\u{c3e}ట\u{c4b}"), ("th", "โคตาบาโต"), ("tr", "Cotabato"), ("uk", "Котабато"), ("ur", "کوتاباتو"), ("vi", "Cotabato"), ("zh", "哥打巴托省")]),
                        unofficial_name_list: ["North Cotabato"].to_vec(),
                    }
                ),
                (
                    "NEC",
                    Subdivision{
                        name: "NEC",
                        country_alpha2: Alpha2::PH,
                        code: "NEC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.2925609), longitude: Some(123.0246518), max_latitude: Some(11.0031714), min_latitude: Some(9.423715099999999), max_longitude: Some(123.5701711), min_longitude: Some(122.3767471)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "نيغروس أوتشيدنتال"), ("bn", "নেগ\u{9cd}রোস অক\u{9cd}সিডেন\u{9cd}ট\u{9be}ল"), ("ca", "Negros Occidental"), ("ccp", "𑄚𑄬𑄉\u{11133}𑄢\u{1112e}𑄌\u{11134} 𑄃\u{11127}𑄇\u{11134}𑄥\u{11128}𑄓𑄬𑄚\u{11134}𑄑𑄣\u{11134}"), ("ceb", "Negros Occidental"), ("da", "Negros Occidental"), ("de", "Negros Occidental"), ("el", "Νέγκρος Οξιντένταλ"), ("en", "Negros Occidental"), ("es", "Negros Occidental"), ("fa", "نگرو غربی"), ("fi", "Negros Occidental"), ("fr", "Negros occidental"), ("gu", "ન\u{ac7}ગ\u{acd}રોસ ઓસીડ\u{ac7}ન\u{acd}ટલ"), ("hi", "न\u{947}ग\u{94d}रोस ओक\u{94d}सीद\u{947}न\u{94d}ताल"), ("id", "Negros Occidental"), ("it", "Provincia di Negros Occidental"), ("ja", "西ネグロス州"), ("kn", "ನ\u{cc6}ಗ\u{ccd}ರೋಸ\u{ccd} ಆಕೇಶನಲ\u{ccd}"), ("ko", "서네그로스 주"), ("lt", "Vakarų Negrosas"), ("lv", "Rietumu Negrosa"), ("mk", "Западен Негрос"), ("mr", "न\u{947}ग\u{94d}रोस ओजीस\u{94d}ट\u{945}नल"), ("ms", "Negros Occidental"), ("nb", "Negros Occidental"), ("nl", "Negros Occidental"), ("no", "Negros Occidental"), ("pl", "Negros Occidental"), ("pt", "Negros Ocidental"), ("ru", "Западный Негрос"), ("si", "නෙග\u{dca}රෝස\u{dca} ඔක\u{dca}ස\u{dd2}ඩෙන\u{dca}ටල\u{dca}"), ("sv", "Negros Occidental"), ("ta", "நெக\u{bcd}ரோஸ\u{bcd} அஸிடெண\u{bcd}ட\u{bcd}ட\u{bbe}ல\u{bcd}"), ("te", "న\u{c46}గ\u{c4d}ర\u{c4b}స\u{c4d} ఓక\u{c4d}స\u{c3f}డ\u{c46}ంటల\u{c4d}"), ("th", "เนกรอส ออคซ\u{e34}เดนท\u{e31}ล"), ("tr", "Negros Occidental"), ("uk", "Західний Негрос"), ("ur", "نیگروس غربی"), ("vi", "Negros Occidental"), ("zh", "西內格羅省")]),
                        unofficial_name_list: ["Negros Occidental"].to_vec(),
                    }
                ),
                (
                    "NER",
                    Subdivision{
                        name: "NER",
                        country_alpha2: Alpha2::PH,
                        code: "NER",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.628208299999999), longitude: Some(122.9888319), max_latitude: Some(10.4163581), min_latitude: Some(9.0385288), max_longitude: Some(123.3400472), min_longitude: Some(122.6101151)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أورينتال نيغروس"), ("bn", "নেগ\u{9cd}রোস ওরিয\u{9bc}েন\u{9cd}ট\u{9be}ল"), ("ca", "Negros Oriental"), ("ccp", "𑄚𑄬𑄉\u{11133}𑄢\u{1112e}𑄌\u{11134} 𑄃\u{11127}𑄢\u{11128}𑄠𑄬𑄚\u{11134}𑄑𑄣\u{11134}"), ("ceb", "Negros Oriental"), ("da", "Negros Oriental"), ("de", "Negros Oriental"), ("el", "Νέγκρος Οριένταλ"), ("en", "Negros Oriental"), ("es", "Negros Oriental"), ("fa", "نگرو شرقی"), ("fi", "Negros Oriental"), ("fr", "Negros oriental"), ("gu", "ન\u{ac7}ગ\u{acd}રોસ ઓરિએન\u{acd}ટલ"), ("he", "נגרוס אוריינטל"), ("hi", "न\u{947}ग\u{94d}रोस ओरिएन\u{94d}ताल"), ("id", "Negros Oriental"), ("it", "Provincia di Negros Oriental"), ("ja", "東ネグロス州"), ("kn", "ನ\u{cc6}ಗ\u{ccd}ರೋಸ\u{ccd} ಒರ\u{cbf}ಯಂಟಲ\u{ccd}"), ("ko", "동네그로스 주"), ("lt", "Rytinis Negrosas"), ("lv", "Austrumu Negrosa"), ("mk", "Источен Негрос"), ("mr", "न\u{947}ग\u{94d}रोस ओरिए\u{902}टल"), ("ms", "Negros Oriental"), ("nb", "Negros Oriental"), ("nl", "Negros Oriental"), ("no", "Negros Oriental"), ("pl", "Negros Oriental"), ("pt", "Negros Oriental"), ("ru", "Восточный Негрос"), ("si", "නෙග\u{dca}රෝස\u{dca} ඔර\u{dd2}යන\u{dca}ටල\u{dca}"), ("sv", "Negros Oriental"), ("ta", "நெகிறோஸ\u{bcd} ஒரிஎண\u{bcd}டல\u{bcd}"), ("te", "న\u{c46}గ\u{c4d}ర\u{c4b}స\u{c4d} ఓర\u{c3f}యంటల\u{c4d}"), ("th", "เนกรอส โอเร\u{e35}ยนท\u{e31}ล"), ("tr", "Negros Oriental"), ("uk", "Східний Негрос"), ("ur", "نیگروس شرقی"), ("vi", "Negros Oriental"), ("zh", "東內格羅省")]),
                        unofficial_name_list: ["Negros Oriental"].to_vec(),
                    }
                ),
                (
                    "NSA",
                    Subdivision{
                        name: "NSA",
                        country_alpha2: Alpha2::PH,
                        code: "NSA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.3613199), longitude: Some(124.7740793), max_latitude: Some(12.6966542), min_latitude: Some(12.1405061), max_longitude: Some(125.3422503), min_longitude: Some(124.0025139)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سامار الشمالية"), ("bn", "নর\u{9cd}দ\u{9be}ন স\u{9be}ম\u{9be}র"), ("ca", "Samar Septentrional"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄥\u{11127}𑄟𑄢\u{11134}"), ("ceb", "Amihanang Samar"), ("da", "Northern Samar"), ("de", "Northern Samar"), ("el", "Βόρειο Σαμάρ"), ("en", "Northern Samar"), ("es", "Sámar del Norte"), ("fa", "سامار شمالی"), ("fi", "Northern Samar"), ("fr", "Samar du Nord"), ("gu", "નધર\u{acd}ન સમર"), ("hi", "उत\u{94d}तरी सामार प\u{94d}रान\u{94d}त"), ("id", "Samar Utara"), ("it", "Provincia di Northern Samar"), ("ja", "北サマル州"), ("kn", "ಉತ\u{ccd}ತರ ಸಮಾರ\u{ccd}"), ("ko", "북사마르 주"), ("lt", "Šiaurės Samaras"), ("lv", "Ziemeļu Samara"), ("mk", "Северен Самар"), ("mr", "नॉर\u{94d}दर\u{94d}न समर"), ("ms", "Northern Samar"), ("nb", "Northern Samar"), ("nl", "Northern Samar"), ("no", "Northern Samar"), ("pl", "Northern Samar"), ("pt", "Northern Samar"), ("ru", "Северный Самар"), ("si", "උත\u{dd4}ර\u{dd4} සමර\u{dca}"), ("sv", "Norra Samar"), ("ta", "நொர\u{bcd}தேர\u{bcd}ந\u{bcd} ச\u{bbe}மர\u{bcd}"), ("te", "ఉత\u{c4d}తర సమర\u{c4d}"), ("th", "เขตนอร\u{e4c}เธ\u{e34}ร\u{e4c}น ซามาร\u{e4c}"), ("tr", "Kuzey Samar"), ("uk", "Північний Самар"), ("ur", "شمالی سامار"), ("vi", "Bắc Samar"), ("zh", "北薩馬省")]),
                        unofficial_name_list: ["Northern Samar"].to_vec(),
                    }
                ),
                (
                    "NUE",
                    Subdivision{
                        name: "NUE",
                        country_alpha2: Alpha2::PH,
                        code: "NUE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.6906831), longitude: Some(120.9876321), max_latitude: Some(16.1302161), min_latitude: Some(15.1651679), max_longitude: Some(121.3782461), min_longitude: Some(120.611833)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "نويفا إيسيا"), ("bn", "ন\u{9cd}য\u{9c1}ভ\u{9be} একিজ\u{9be}"), ("ca", "Nueva Ecija"), ("ccp", "𑄚\u{1112a}𑄠𑄬𑄞 𑄃\u{11128}𑄥\u{11128}𑄎"), ("ceb", "Nueva Ecija"), ("cs", "Nueva Ecija"), ("da", "Nueva Ecija"), ("de", "Nueva Ecija"), ("el", "Νουέβα Εκίτζα"), ("en", "Nueva Ecija"), ("es", "Nueva Écija"), ("fa", "نوئه\u{200c}وا اکیجا"), ("fi", "Nueva Ecija"), ("fr", "Nueva Ecija"), ("gu", "ન\u{ac1}એવા એસિજા"), ("hi", "न\u{941}एवा एसिहा प\u{94d}रान\u{94d}त"), ("id", "Nueva Ecija"), ("it", "Provincia di Nueva Ecija"), ("ja", "ヌエヴァ・エシハ州"), ("kn", "ನುವಾ ಎಜ\u{cbf}ಜಾ"), ("ko", "누에바에시하 주"), ("lt", "Nueva Ecidža"), ("lv", "Nueva Esiha"), ("mk", "Нова Есиха"), ("mr", "न\u{941}ईव एसिज"), ("ms", "Nueva Ecija"), ("nb", "Nueva Ecija"), ("nl", "Nueva Ecija"), ("no", "Nueva Ecija"), ("pl", "Nueva Ecija"), ("pt", "Nueva Ecija"), ("ru", "Нуэва-Эсиха"), ("si", "න\u{dd4}එව\u{dcf} එස\u{dd2}ජ\u{dcf}"), ("sv", "Nueva Ecija"), ("ta", "னுக\u{bcd}கேவ\u{bbe} ஏசிஜ\u{bbe}"), ("te", "నుయ\u{c47}వ\u{c3e} ఎస\u{c3f}జ\u{c3e}"), ("th", "น\u{e39}วา อ\u{e35}ซ\u{e34}จา"), ("tr", "Nueva Ecija"), ("uk", "Нуева-Есіха"), ("ur", "نوئوا اسیہا"), ("vi", "Nueva Ecija"), ("zh", "新怡詩夏省")]),
                        unofficial_name_list: ["Nueva Ecija"].to_vec(),
                    }
                ),
                (
                    "NUV",
                    Subdivision{
                        name: "NUV",
                        country_alpha2: Alpha2::PH,
                        code: "NUV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.3301107), longitude: Some(121.1710389), max_latitude: Some(16.7457349), min_latitude: Some(15.7672), max_longitude: Some(121.465768), min_longitude: Some(120.756631)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "نويفا فيزكايا"), ("bn", "ন\u{9cd}য\u{9c1}ভ\u{9be} ভিজক\u{9be}য\u{9bc}\u{9be}"), ("ca", "Nueva Vizcaya"), ("ccp", "𑄚\u{1112a}𑄠𑄬𑄞 𑄞\u{11128}𑄌𑄇𑄬𑄠"), ("ceb", "Nueva Vizcaya"), ("da", "Nueva Vizcaya"), ("de", "Nueva Vizcaya"), ("el", "Νουέβα Βιζκάγια"), ("en", "Nueva Vizcaya"), ("es", "Nueva Vizcaya"), ("eu", "Bizkai Berria"), ("fa", "نوئه\u{200c}وا ویزکایا"), ("fi", "Nueva Vizcaya"), ("fr", "Nueva Vizcaya"), ("gu", "ન\u{ac1}એવા વિઝકાયા"), ("hi", "न\u{941}एवा विज\u{93c}काया प\u{94d}रान\u{94d}त"), ("id", "Nueva Vizcaya"), ("it", "Provincia di Nueva Vizcaya"), ("ja", "ヌエヴァ・ヴィスカヤ州"), ("kn", "ನುವಾ ವ\u{cbf}ಕಾಯಾಯಾ"), ("ko", "누에바비스카야 주"), ("lt", "Nueva Viskaja"), ("lv", "Nueva Viskaja"), ("mk", "Нова Вискаја"), ("mr", "न\u{942}व\u{94d}ह\u{947} व\u{94d}हिज\u{94d}काय"), ("ms", "Nueva Vizcaya"), ("nb", "Nueva Vizcaya"), ("nl", "Nueva Vizcaya"), ("no", "Nueva Vizcaya"), ("pl", "Nueva Vizcaya"), ("pt", "Nova Vizcaya"), ("ru", "Нуэва-Виская"), ("si", "න\u{dd4}එව\u{dcf} ව\u{dd2}ස\u{dca}කය\u{dcf}"), ("sv", "Nueva Vizcaya"), ("ta", "நியூவ\u{bbe} விஸ\u{bcd}க\u{bcd}க\u{bbe}ய"), ("te", "నుయ\u{c46}వ\u{c3e} వ\u{c3f}జ\u{c4d}క\u{c3e}య\u{c3e}"), ("th", "โนวาว\u{e34}ซคายา"), ("tr", "Nueva Vizcaya"), ("uk", "Нуева-Віская"), ("ur", "نوئوا ویزکایا"), ("vi", "Nueva Vizcaya"), ("zh", "新比斯開省")]),
                        unofficial_name_list: ["Nueva Vizcaya"].to_vec(),
                    }
                ),
                (
                    "PAM",
                    Subdivision{
                        name: "PAM",
                        country_alpha2: Alpha2::PH,
                        code: "PAM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.079409), longitude: Some(120.6199895), max_latitude: Some(15.2718861), min_latitude: Some(14.7673906), max_longitude: Some(120.985538), min_longitude: Some(120.3561269)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بامبانغا"), ("bn", "প\u{9be}ম\u{9cd}প\u{9be}ঙ\u{9cd}গ\u{9be}"), ("ca", "Pampanga"), ("ccp", "𑄛𑄟\u{11134}𑄛𑄚\u{11134}𑄉"), ("ceb", "Pampanga"), ("da", "Pampanga"), ("de", "Provinz Pampanga"), ("el", "Παμπάνγκα"), ("en", "Pampanga"), ("es", "Pampanga"), ("fa", "پامپانگا"), ("fi", "Pampanga"), ("fr", "Pampanga"), ("gu", "પમ\u{acd}પાન\u{acd}ગા"), ("hi", "पाम\u{94d}पा\u{902}गा प\u{94d}रान\u{94d}त"), ("id", "Pampanga"), ("it", "Provincia di Pampanga"), ("ja", "パンパンガ州"), ("kn", "ಪಂಪಾಂಗಾ"), ("ko", "팜팡가 주"), ("lt", "Pampanga"), ("lv", "Pampanga"), ("mk", "Пампанга"), ("mr", "पाम\u{94d}पान\u{94d}गा"), ("ms", "Pampanga"), ("nb", "Pampanga"), ("nl", "Pampanga"), ("no", "Pampanga"), ("pl", "Pampanga"), ("pt", "Pampanga"), ("ru", "Пампанга"), ("si", "පපන\u{dca}ග\u{dcf}"), ("sv", "Pampanga"), ("ta", "பம\u{bcd}பங\u{bcd}க\u{bbe}"), ("te", "పంప\u{c3e}ంగ\u{c3e}"), ("th", "ป\u{e31}มปางา"), ("tr", "Pampanga"), ("uk", "Пампанга"), ("ur", "پامپانگا"), ("vi", "Pampanga"), ("zh", "邦板牙省")]),
                        unofficial_name_list: ["Pampanga"].to_vec(),
                    }
                ),
                (
                    "PAN",
                    Subdivision{
                        name: "PAN",
                        country_alpha2: Alpha2::PH,
                        code: "PAN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.8949055), longitude: Some(120.2863183), max_latitude: Some(16.426502), min_latitude: Some(15.61848), max_longitude: Some(120.9229021), min_longitude: Some(119.7495495)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بانغاسينان"), ("bn", "প\u{9be}ঙ\u{9cd}গ\u{9be}সিন\u{9be}ন"), ("ca", "Pangasinan"), ("ccp", "𑄛𑄚\u{11134}𑄉𑄥\u{11128}𑄚𑄚\u{11134}"), ("ceb", "Pangasinan"), ("da", "Pangasinan"), ("de", "Pangasinán"), ("el", "Πανγκασινάν"), ("en", "Pangasinan"), ("es", "Pangasinán"), ("fa", "پانگاسینان"), ("fi", "Pangasinan"), ("fr", "Pangasinan"), ("gu", "પ\u{a82}ગાસીનન"), ("hi", "पा\u{902}गासिनान प\u{94d}रान\u{94d}त"), ("id", "Pangasinan"), ("it", "Provincia di Pangasinan"), ("ja", "パンガシナン州"), ("jv", "Pangasinan"), ("kn", "ಪಂಗಾಸ\u{cbf}ನಾನ\u{ccd}"), ("ko", "팡가시난 주"), ("lt", "Pangasinanas"), ("lv", "Pangasinana"), ("mk", "Пангасинан"), ("mr", "प\u{902}गासीनन"), ("ms", "Pangasinan"), ("nb", "Pangasinan"), ("nl", "Pangasinan"), ("no", "Pangasinan"), ("pl", "Pangasinan"), ("pt", "Pangasinán"), ("ru", "Пангасинан"), ("si", "පන\u{dca}ගස\u{dd2}න\u{dcf}න\u{dca}"), ("sv", "Pangasinan"), ("ta", "பஞசின\u{bbe}ன\u{bcd}"), ("te", "ప\u{c3e}ంగ\u{c3e}స\u{c3f}నన\u{c4d}"), ("th", "ป\u{e31}งกาส\u{e34}น\u{e31}น"), ("tr", "Pangasinan"), ("uk", "Пангасінан"), ("ur", "پانگاسینان"), ("vi", "Pangasinan"), ("zh", "邦阿西楠省")]),
                        unofficial_name_list: ["Pangasinan"].to_vec(),
                    }
                ),
                (
                    "PLW",
                    Subdivision{
                        name: "PLW",
                        country_alpha2: Alpha2::PH,
                        code: "PLW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.8349493), longitude: Some(118.7383615), max_latitude: Some(12.66335), min_latitude: Some(6.9634569), max_longitude: Some(121.2708889), min_longitude: Some(116.9315639)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بالاوان"), ("be", "Правінцыя Палаван"), ("bg", "Палаван"), ("bn", "প\u{9be}ল\u{9be}ওয\u{9bc}\u{9be}ন"), ("ca", "Palawan"), ("ccp", "𑄛\u{11127}𑄣𑄤𑄚\u{11134}"), ("ceb", "Palawan"), ("cs", "Palawan"), ("cy", "Palawan"), ("da", "Palawan"), ("de", "Palawan"), ("el", "Πάλαβαν"), ("en", "Palawan"), ("es", "Palawan"), ("et", "Palawan"), ("fa", "پالاوان"), ("fi", "Palawan"), ("fr", "province de Palawan"), ("gu", "પાલાવાન"), ("he", "פלאוואן"), ("hi", "पलावन"), ("hr", "Palawan"), ("hu", "Palawan"), ("id", "Palawan"), ("it", "Provincia di Palawan"), ("ja", "パラワン州"), ("kn", "ಪಾಲವಾನ\u{ccd}"), ("ko", "팔라완 주"), ("lt", "Palavanas"), ("lv", "Palavana"), ("mk", "Палаван"), ("ml", "പല\u{d3e}വൻ"), ("mr", "पालावान"), ("ms", "Palawan"), ("nb", "Palawan"), ("nl", "Palawan"), ("no", "Palawan"), ("pl", "Palawan"), ("pt", "Palawan"), ("ru", "Палаван"), ("si", "පලවන\u{dca}"), ("sl", "Palawan"), ("sr", "Палаван"), ("sr_Latn", "Palavan"), ("sv", "Palawan"), ("ta", "பலவ\u{bbe}ன\u{bcd}"), ("te", "ప\u{c3e}ల\u{c3e}వ\u{c3e}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดปาลาว\u{e31}น"), ("tr", "Palawan"), ("uk", "Палаван"), ("ur", "پالاوان"), ("vi", "Palawan"), ("zh", "巴拉望省")]),
                        unofficial_name_list: ["Palawan"].to_vec(),
                    }
                ),
                (
                    "QUE",
                    Subdivision{
                        name: "QUE",
                        country_alpha2: Alpha2::PH,
                        code: "QUE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.0313906), longitude: Some(122.1130909), max_latitude: Some(14.1294959), min_latitude: Some(14.0007015), max_longitude: Some(122.1910019), min_longitude: Some(122.0490502)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كزون"), ("bn", "কৌয\u{9bc}েজন"), ("ca", "Quezon"), ("ccp", "𑄇\u{1112a}𑄠𑄬𑄎\u{11127}𑄚\u{11134}"), ("ceb", "Quezon"), ("da", "Quezon"), ("de", "Quezon"), ("el", "Κουεζόν"), ("en", "Quezon"), ("es", "Quezón"), ("fa", "کزون"), ("fi", "Quezon"), ("fr", "province de Quezon"), ("gu", "ક\u{acd}વ\u{ac7}ઝોન"), ("hi", "क\u{947}ज\u{93c}ोन प\u{94d}रान\u{94d}त"), ("id", "Quezon"), ("it", "Provincia di Quezon"), ("ja", "ケソン州"), ("kn", "ಕ\u{ccd}ವ\u{cc6}ಝೋನ\u{ccd}"), ("ko", "케손 주"), ("lt", "Kezonas"), ("lv", "Kesona"), ("mk", "Кезон"), ("mr", "क\u{94d}व\u{947}झोन"), ("ms", "Quezon"), ("nb", "Quezon"), ("nl", "Quezon"), ("no", "Quezon"), ("pl", "Quezon"), ("pt", "Quezon"), ("ru", "Кесон"), ("si", "ක\u{dd2}ය\u{dd4}සන\u{dca}"), ("sv", "Quezon"), ("ta", "யூஸ\u{bcd}ன\u{bcd}"), ("te", "క\u{c4d}వ\u{c46}జ\u{c3e}న\u{c4d}"), ("th", "เคซอน"), ("tr", "Quezon"), ("uk", "Кесон"), ("ur", "کویزون"), ("vi", "Quezon"), ("zh", "奎松省")]),
                        unofficial_name_list: ["Quezon"].to_vec(),
                    }
                ),
                (
                    "QUI",
                    Subdivision{
                        name: "QUI",
                        country_alpha2: Alpha2::PH,
                        code: "QUI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.133333), longitude: Some(121.7), max_latitude: Some(17.2103775), min_latitude: Some(17.0988256), max_longitude: Some(121.8186592), min_longitude: Some(121.6813735)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كويرينو"), ("bn", "কিরিনো"), ("ca", "Quirino"), ("ccp", "𑄇\u{1112a}𑄃\u{11128}𑄢\u{11128}𑄚\u{1112e}"), ("ceb", "Quirino"), ("da", "Quirino"), ("de", "Quirino"), ("el", "Κουιρίνο"), ("en", "Quirino"), ("es", "Quirino"), ("fa", "کویرینو"), ("fi", "Quirino"), ("fr", "Quirino"), ("gu", "ક\u{acd}વિરિનો"), ("hi", "किरीनो प\u{94d}रान\u{94d}त"), ("id", "Quirino"), ("it", "Provincia di Quirino"), ("ja", "キリノ州"), ("kn", "ಕ\u{ccd}ವ\u{cbf}ರ\u{cbf}ನೊ"), ("ko", "키리노 주"), ("lt", "Kirinas"), ("lv", "Kirino"), ("mk", "Кирино"), ("mr", "क\u{94d}विरिनो"), ("ms", "Quirino"), ("nb", "Quirino"), ("nl", "Quirino"), ("no", "Quirino"), ("pl", "Quirino"), ("pt", "Quirino"), ("ru", "Кирино"), ("si", "ක\u{dd4}ය\u{dd2}ර\u{dd2}නෝ"), ("sv", "Quirino"), ("ta", "குரினொ"), ("te", "క\u{c4d}వ\u{c3f}ర\u{c3f}న\u{c4b}"), ("th", "คว\u{e34}ร\u{e34}โน"), ("tr", "Quirino"), ("uk", "Кіріно"), ("ur", "کوئرینو"), ("vi", "Quirino"), ("zh", "季里諾省")]),
                        unofficial_name_list: ["Angkaki"].to_vec(),
                    }
                ),
                (
                    "RIZ",
                    Subdivision{
                        name: "RIZ",
                        country_alpha2: Alpha2::PH,
                        code: "RIZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.6037446), longitude: Some(121.3084088), max_latitude: Some(14.8888711), min_latitude: Some(14.2896792), max_longitude: Some(121.4698249), min_longitude: Some(121.0935389)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ريزال"), ("bn", "রিজ\u{9be}ল"), ("ca", "Rizal"), ("ccp", "𑄢\u{11128}𑄎𑄣\u{11134}"), ("ceb", "Rizal"), ("da", "Rizal"), ("de", "Rizal"), ("el", "Ριζάλ"), ("en", "Rizal"), ("es", "Rizal"), ("fa", "ریزال"), ("fi", "Rizal"), ("fr", "Rizal"), ("gu", "રિઝાલ"), ("hi", "रिज\u{93c}ाल प\u{94d}रान\u{94d}त"), ("id", "Rizal"), ("it", "Provincia di Rizal"), ("ja", "リサール州"), ("kn", "ರ\u{cbf}ಜಾಲ\u{ccd}"), ("ko", "리살 주"), ("lt", "Rizalis"), ("lv", "Rizala"), ("mk", "Ризал"), ("mr", "रियाझल"), ("ms", "Rizal"), ("nb", "Rizal"), ("nl", "Rizal"), ("no", "Rizal"), ("pl", "Rizal"), ("pt", "Rizal"), ("ru", "Рисаль"), ("si", "ර\u{dd2}ස\u{dcf}ල\u{dca}"), ("sv", "Rizal"), ("ta", "ரிச\u{bbe}ல\u{bcd}"), ("te", "ర\u{c3f}జ\u{c3e}ల\u{c4d}"), ("th", "ร\u{e34}เซล"), ("tr", "Rizal"), ("uk", "Рісаль"), ("ur", "ریزال"), ("vi", "Rizal"), ("zh", "黎剎省")]),
                        unofficial_name_list: ["Rizal"].to_vec(),
                    }
                ),
                (
                    "ROM",
                    Subdivision{
                        name: "ROM",
                        country_alpha2: Alpha2::PH,
                        code: "ROM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.5778554), longitude: Some(122.269129), max_latitude: Some(12.6712134), min_latitude: Some(12.4778353), max_longitude: Some(122.328081), min_longitude: Some(122.2304249)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "رومبلون"), ("bn", "রোম\u{9cd}বলন"), ("ca", "Romblon"), ("ccp", "𑄢\u{1112e}𑄟\u{11134}𑄝\u{11133}𑄣\u{1112e}𑄚\u{11134}"), ("ceb", "Romblon"), ("da", "Romblon"), ("de", "Romblon"), ("el", "Ρόμπλον"), ("en", "Romblon"), ("es", "Romblón"), ("fa", "رومبلون"), ("fi", "Romblon"), ("fr", "Romblon"), ("gu", "રોમ\u{acd}બ\u{acd}લોન"), ("hi", "रोमब\u{94d}लोन"), ("id", "Romblon"), ("it", "Provincia di Romblon"), ("ja", "ロンブロン州"), ("kn", "ರೊಮ\u{ccd}ಬ\u{ccd}ಲೋನ\u{ccd}"), ("ko", "롬블론 주"), ("lt", "Romblonas"), ("lv", "Romblona"), ("mk", "Ромблон"), ("mr", "रोमबलोन"), ("ms", "Romblon"), ("nb", "Romblon"), ("nl", "Romblon"), ("no", "Romblon"), ("pl", "Romblon"), ("pt", "Romblon"), ("ru", "Ромблон"), ("si", "රොම\u{dca}බලෝන\u{dca}"), ("sv", "Romblon"), ("ta", "ரொம\u{bcd}பலோன\u{bcd}"), ("te", "ర\u{c3e}ంబ\u{c4d}ల\u{c3e}న\u{c4d}"), ("th", "เม\u{e37}องรอมบลอน"), ("tr", "Romblon"), ("uk", "Ромблон"), ("ur", "رومبلون"), ("vi", "Romblon"), ("zh", "朗布隆省")]),
                        unofficial_name_list: ["Romblon"].to_vec(),
                    }
                ),
                (
                    "SAR",
                    Subdivision{
                        name: "SAR",
                        country_alpha2: Alpha2::PH,
                        code: "SAR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(5.926717500000001), longitude: Some(124.994751), max_latitude: Some(6.468224999999999), min_latitude: Some(5.566673), max_longitude: Some(125.5387121), min_longitude: Some(124.3531991)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سارانغاني"), ("bn", "স\u{9be}র\u{9be}\u{9be}ঙ\u{9cd}গ\u{9be}নি"), ("ca", "Sarangani"), ("ccp", "𑄥𑄢𑄋\u{11134}𑄉𑄚\u{11128}"), ("ceb", "Sarangani"), ("da", "Sarangani"), ("de", "Sarangani"), ("el", "Σαρανγκάνι"), ("en", "Sarangani"), ("es", "Sarangani"), ("fa", "سارانگانی"), ("fi", "Sarangani"), ("fr", "Sarangani"), ("gu", "સાર\u{a82}ગણી"), ("he", "סרנגני"), ("hi", "सार\u{902}गानी प\u{94d}रान\u{94d}त"), ("id", "Sarangani"), ("it", "Provincia di Sarangani"), ("ja", "サランガニ州"), ("kn", "ಸರಂಗನ\u{cbf}"), ("ko", "사랑가니 주"), ("lt", "Saranganis"), ("lv", "Sarangani"), ("mk", "Сарангани"), ("mr", "सार\u{902}गणी"), ("ms", "Sarangani"), ("nb", "Sarangani"), ("nl", "Sarangani"), ("no", "Sarangani"), ("pl", "Sarangani"), ("pt", "Sarangani"), ("ru", "Сарангани"), ("si", "සරන\u{dca}ගන\u{dd3}"), ("sv", "Sarangani"), ("ta", "சரங\u{bcd}கணி"), ("te", "స\u{c3e}రంగన\u{c3f}"), ("th", "ซารางาน\u{e34}"), ("tr", "Sarangani"), ("uk", "Сарангані"), ("ur", "سارانگانی"), ("vi", "Sarangani"), ("zh", "薩蘭加尼省")]),
                        unofficial_name_list: ["Sarangani"].to_vec(),
                    }
                ),
                (
                    "SCO",
                    Subdivision{
                        name: "SCO",
                        country_alpha2: Alpha2::PH,
                        code: "SCO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.3357565), longitude: Some(124.7740793), max_latitude: Some(6.6679448), min_latitude: Some(5.957796999999999), max_longitude: Some(125.2723331), min_longitude: Some(124.278951)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جنوب كوتاباتو"), ("bn", "দক\u{9cd}ষিণ কোট\u{9be}ব\u{9be}টো"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄇\u{1112e}𑄑𑄝𑄑\u{1112e}"), ("ceb", "Habagatang Cotabato"), ("da", "South Cotabato"), ("de", "South Cotabato"), ("el", "Νότιο Κοταμπάτο"), ("en", "South Cotabato"), ("es", "Cotabato del Sur"), ("fa", "کوتاباتو جنوبی"), ("fi", "South Cotabato"), ("fr", "Cotabato du Sud"), ("gu", "સાઉથ કોટાબ\u{ac7}ટો"), ("hi", "दक\u{94d}षिण कोताबातो प\u{94d}रान\u{94d}त"), ("id", "Cotabato Selatan"), ("it", "Provincia di South Cotabato"), ("ja", "南コタバト州"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಕೊಟಬಾಟೊ"), ("ko", "남코타바토 주"), ("lt", "Pietų Kotabatas"), ("lv", "Dievidkotabato"), ("mk", "Јужен Котабато"), ("mr", "दक\u{94d}षिण कोट\u{947}ब\u{947}टो"), ("ms", "Cotabato Selatan"), ("nb", "South Cotabato"), ("nl", "South Cotabato"), ("no", "South Cotabato"), ("pl", "South Cotabato"), ("pt", "Cotabato do Sul"), ("ru", "Южный Котабато"), ("si", "දක\u{dd4}ණ\u{dd4} කොටබටෝ"), ("sv", "Södra Cotabato"), ("ta", "தெற\u{bcd}கு க\u{bbe}ட\u{bcd}டப\u{bcd}பட\u{bcd}டோ"), ("te", "దక\u{c4d}ష\u{c3f}ణ క\u{c4b}ట\u{c4b}బ\u{c3e}ట\u{c4b}"), ("th", "โคตาบาโตใต\u{e49}"), ("tr", "Güney Cotabato"), ("uk", "Південний Котабато"), ("ur", "جنوبی کوتاباتو"), ("vi", "Nam Cotabato"), ("zh", "南哥打巴托省")]),
                        unofficial_name_list: ["South Cotabato"].to_vec(),
                    }
                ),
                (
                    "SIG",
                    Subdivision{
                        name: "SIG",
                        country_alpha2: Alpha2::PH,
                        code: "SIG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.213263099999999), longitude: Some(123.5157032), max_latitude: Some(9.2403095), min_latitude: Some(9.141587999999999), max_longitude: Some(123.6037446), min_longitude: Some(123.4554852)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سيكيخور"), ("bn", "সিক\u{9c1}ইজোর"), ("ca", "Siquijor"), ("ccp", "𑄥\u{11128}𑄇\u{1112d}\u{1112a}𑄎\u{1112e}𑄢\u{11134}"), ("ceb", "Siquijor"), ("cs", "Siquijor"), ("da", "Siquijor"), ("de", "Siquijor"), ("el", "Σικουιτζόρ"), ("en", "Siquijor"), ("es", "Siquijor"), ("fa", "سیکیخور"), ("fi", "Siquijor"), ("fr", "Siquijor"), ("gu", "સિક\u{acd}વિઝોર"), ("hi", "सिकिहोर"), ("id", "Siquijor"), ("it", "Provincia di Siquijor"), ("ja", "シキホル州"), ("kn", "ಸ\u{cbf}ಕ\u{ccd}ವ\u{cbf}ಜರ\u{ccd}"), ("ko", "시키호르 주"), ("lt", "Sikijoras"), ("lv", "Sikvuijora"), ("mk", "Сикихор"), ("mr", "सीक\u{941}ईजोर"), ("ms", "Siquijor"), ("nb", "Siquijor"), ("nl", "Siquijor"), ("no", "Siquijor"), ("pl", "Siquijor"), ("pt", "Siquijor"), ("ru", "Сикихор"), ("si", "ස\u{dd2}ක\u{dd2}ජෝර\u{dca}"), ("sv", "Siquijor"), ("ta", "சிகுயிஜுர\u{bcd}"), ("te", "స\u{c3f}ఖ\u{c3f}జ\u{c4b}ర\u{c4d}"), ("th", "ซ\u{e34}ก\u{e35}จอร\u{e4c}"), ("tr", "Siquijor"), ("uk", "Сікіхор"), ("ur", "سیقیحور"), ("vi", "Siquijor"), ("zh", "錫基霍爾省")]),
                        unofficial_name_list: ["Siquijor"].to_vec(),
                    }
                ),
                (
                    "SLE",
                    Subdivision{
                        name: "SLE",
                        country_alpha2: Alpha2::PH,
                        code: "SLE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.3346206), longitude: Some(125.1708741), max_latitude: Some(10.6267204), min_latitude: Some(9.895297), max_longitude: Some(125.2990696), min_longitude: Some(124.755684)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لايتي الجنوبية"), ("be", "Паўднёвы Лейтэ"), ("bn", "স\u{9be}উদ\u{9be}র\u{9cd}ন লেটে"), ("ca", "Leyte Meridional"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄣𑄬𑄠𑄖\u{11134}"), ("ceb", "Habagatang Leyte"), ("da", "Southern Leyte"), ("de", "Southern Leyte"), ("el", "Σάουθερν Λέιτε"), ("en", "Southern Leyte"), ("es", "Leyte del Sur"), ("et", "Lõuna-Leyte"), ("fa", "لیته جنوبی"), ("fi", "Southern Leyte"), ("fr", "Leyte du Sud"), ("gu", "સધર\u{acd}ન લ\u{ac7}ટ\u{ac7}"), ("hi", "दक\u{94d}षिणी ल\u{947}यत\u{947} प\u{94d}रान\u{94d}त"), ("id", "Leyte Selatan"), ("it", "Provincia di Southern Leyte"), ("ja", "南レイテ州"), ("kn", "ಸದರ\u{ccd}ನ\u{ccd} ಲೇಯ\u{ccd}ಟ\u{cc6}"), ("ko", "남레이테 주"), ("lt", "Pietryčių Leitė"), ("lv", "Dienvidleite"), ("mk", "Јужен Лејте"), ("mr", "साउदर\u{94d}न ल\u{947}य\u{947}"), ("ms", "Southern Leyte"), ("nb", "Southern Leyte"), ("nl", "Southern Leyte"), ("no", "Southern Leyte"), ("pl", "Southern Leyte"), ("pt", "Southern Leyte"), ("ru", "Южный Лейте"), ("si", "දක\u{dd4}ණ\u{dd4} ලේටේ"), ("sv", "Södra Leyte"), ("ta", "தெற\u{bcd}கு லெய\u{bcd}டி"), ("te", "దక\u{c4d}ష\u{c3f}ణ ల\u{c46}య\u{c3f}ట\u{c47}"), ("th", "เซ\u{e49}าเท\u{e34}ร\u{e4c}น เลย\u{e4c}เต\u{e49}"), ("tr", "Güney Leyte"), ("uk", "Південний Лейте"), ("ur", "جنوبی لیئتے"), ("vi", "Nam Leyte"), ("zh", "南萊特省")]),
                        unofficial_name_list: ["Southern Leyte"].to_vec(),
                    }
                ),
                (
                    "SLU",
                    Subdivision{
                        name: "SLU",
                        country_alpha2: Alpha2::PH,
                        code: "SLU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(5.974901099999999), longitude: Some(121.03351), max_latitude: Some(6.4398352), min_latitude: Some(5.4429449), max_longitude: Some(121.9476239), min_longitude: Some(120.4130955)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سولو"), ("bn", "স\u{9c1}ল\u{9c1}"), ("ca", "Sulu"), ("ccp", "𑄥\u{1112a}𑄣\u{1112a}"), ("ceb", "Sulu"), ("da", "Sulu"), ("de", "Sulu"), ("el", "Σουλού"), ("en", "Sulu"), ("es", "Sulú"), ("fa", "سولو"), ("fi", "Sulu"), ("fr", "Sulu"), ("gu", "સ\u{ac1}લ\u{ac1}"), ("hi", "स\u{941}ल\u{941} प\u{94d}रान\u{94d}त"), ("id", "Sulu"), ("it", "Provincia di Sulu"), ("ja", "スールー州"), ("kn", "ಸುಲು"), ("ko", "술루 주"), ("lt", "Sulu"), ("lv", "Sulu"), ("mk", "Сулу"), ("mr", "सल\u{942}"), ("ms", "Sulu"), ("nb", "Sulu"), ("nl", "Sulu"), ("no", "Sulu"), ("pl", "Archipelag Sulu"), ("pt", "Sulu"), ("ru", "Сулу"), ("si", "ස\u{dd6}ල\u{dd4}"), ("sv", "Sulu"), ("ta", "சுலு"), ("te", "సులు"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e39}ล\u{e39}"), ("tr", "Sulu"), ("uk", "Сулу"), ("ur", "سولو"), ("vi", "Sulu"), ("zh", "蘇祿省")]),
                        unofficial_name_list: ["Sulu"].to_vec(),
                    }
                ),
                (
                    "SOR",
                    Subdivision{
                        name: "SOR",
                        country_alpha2: Alpha2::PH,
                        code: "SOR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.9927095), longitude: Some(124.0147464), max_latitude: Some(13.1196236), min_latitude: Some(12.8177146), max_longitude: Some(124.1477504), min_longitude: Some(123.7906891)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سوروسوغون"), ("bn", "সরসোগন"), ("ca", "Sorsogon"), ("ccp", "𑄥\u{1112e}𑄢\u{11134}𑄥\u{1112e}𑄉𑄚\u{11134}"), ("ceb", "Sorsogon"), ("da", "Sorsogon"), ("de", "Sorsogon"), ("el", "Σορσογκόν"), ("en", "Sorsogon"), ("es", "Sorsogón"), ("fa", "سوروسوگون"), ("fi", "Sorsogon"), ("fr", "Sorsogon"), ("gu", "સર\u{acd}સોગોન"), ("hi", "सोरसोगोन प\u{94d}रान\u{94d}त"), ("id", "Sorsogon"), ("it", "Provincia di Sorsogon"), ("ja", "ソルソゴン州"), ("kn", "ಸೋರ\u{ccd}ಸೋಗಾನ\u{ccd}"), ("ko", "소르소곤 주"), ("lt", "Sorsogonas"), ("lv", "Sorsogona"), ("mk", "Сорсогон"), ("mr", "सर\u{94d}सोगोन"), ("ms", "Sorsogon"), ("nb", "Sorsogon"), ("nl", "Sorsogon"), ("no", "Sorsogon"), ("pl", "Sorsogon"), ("pt", "Sorsogon"), ("ru", "Сорсогон"), ("si", "සොර\u{dca}සොගෝන\u{dca}"), ("sv", "Sorsogon"), ("ta", "சொற\u{bcd}சோகோன\u{bcd}"), ("te", "స\u{c4b}ర\u{c4d}స\u{c4b}గన\u{c4d}"), ("th", "ร\u{e31}ฐทาราบา"), ("tr", "Sorsogon"), ("uk", "Сорсогон"), ("ur", "سورسوگون"), ("vi", "Sorsogon"), ("yue", "索索貢省"), ("yue_Hans", "索索贡省"), ("zh", "索索貢省")]),
                        unofficial_name_list: ["Sorsogon"].to_vec(),
                    }
                ),
                (
                    "SUK",
                    Subdivision{
                        name: "SUK",
                        country_alpha2: Alpha2::PH,
                        code: "SUK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.5069401), longitude: Some(124.4198243), max_latitude: Some(6.883467), min_latitude: Some(6.117249999999999), max_longitude: Some(125.197615), min_longitude: Some(124.0158941)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سلطان قادرات"), ("bn", "স\u{9c1}লত\u{9be}ন ক\u{9c1}দ\u{9cd}র\u{9be}ত"), ("ca", "Sultan Kudarat"), ("ccp", "𑄥\u{1112a}𑄣\u{11134}𑄑𑄚\u{11134} 𑄇\u{1112a}𑄓𑄢𑄖\u{11134}"), ("ceb", "Sultan Kudarat"), ("da", "Sultan Kudarat"), ("de", "Sultan Kudarat"), ("el", "Σουλτάν Κουνταράτ"), ("en", "Sultan Kudarat"), ("es", "Sultán Kudarat"), ("fa", "سلطان کودرت"), ("fi", "Sultan Kudarat"), ("fr", "Sultan Kudarat"), ("gu", "સ\u{ac1}લતાન ક\u{ac1}દારાત"), ("hi", "स\u{941}ल\u{94d}तान क\u{941}दारात प\u{94d}रान\u{94d}त"), ("id", "Sultan Kudarat"), ("it", "Provincia di Sultan Kudarat"), ("ja", "スルタン・クダラット州"), ("kn", "ಸುಲ\u{ccd}ತಾನ\u{ccd} ಕುದಾರತ\u{ccd}"), ("ko", "술탄쿠다라트 주"), ("lt", "Sultan Kudaratas"), ("lv", "Sultana Kudarata"), ("mk", "Султан Кударат"), ("mr", "स\u{941}ल\u{94d}तान क\u{941}दारात"), ("ms", "Sultan Kudarat"), ("nb", "Sultan Kudarat"), ("nl", "Sultan Kudarat"), ("no", "Sultan Kudarat"), ("pl", "Sultan Kudarat"), ("pt", "Sultão Kudarat"), ("ru", "Султан-Кударат"), ("si", "ස\u{dd4}ල\u{dca}ත\u{dcf}න\u{dca} ක\u{dd4}දරට\u{dca}"), ("sv", "Sultan Kudarat"), ("ta", "சுல\u{bcd}த\u{bbe}ன\u{bcd} குட\u{bbe}ரட\u{bcd}"), ("te", "సుల\u{c4d}త\u{c3e}న\u{c4d} కుద\u{c3e}రత\u{c4d}"), ("th", "ซ\u{e38}ลตานก\u{e38}ดาร\u{e31}ต"), ("tr", "Sultan Kudarat"), ("uk", "Султан-Кударат"), ("ur", "سلطان قدرت"), ("vi", "Sultan Kudarat"), ("zh", "蘇丹庫達拉省")]),
                        unofficial_name_list: ["Sultan Kudarat"].to_vec(),
                    }
                ),
                (
                    "SUN",
                    Subdivision{
                        name: "SUN",
                        country_alpha2: Alpha2::PH,
                        code: "SUN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.514828), longitude: Some(125.6969984), max_latitude: Some(10.0626918), min_latitude: Some(9.325377), max_longitude: Some(126.1804676), min_longitude: Some(125.3916907)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سوريجاو ديل نورت"), ("bn", "স\u{9c1}রিগ\u{9be}ল ডেল নর\u{9cd}টে"), ("ccp", "𑄥\u{1112a}𑄢\u{11128}𑄉𑄃\u{1112e} 𑄓𑄬𑄣\u{11134} 𑄚\u{11127}𑄢\u{11134}𑄑𑄬"), ("ceb", "Surigao del Norte"), ("da", "Surigao del Norte"), ("de", "Surigao del Norte"), ("el", "Σουριγκάο ντελ Νόρτε"), ("en", "Surigao del Norte"), ("es", "Surigao del Norte"), ("fa", "سوریگائو شمالی"), ("fi", "Surigao del Norte"), ("fr", "Surigao du Nord"), ("gu", "સ\u{ac1}રિગાઓ ડ\u{ac7}લ નોર\u{acd}ટ\u{ac7}"), ("hi", "स\u{941}रिगाओ द\u{947}ल नोर\u{94d}त\u{947}"), ("id", "Surigao del Norte"), ("it", "Provincia di Surigao del Norte"), ("ja", "北スリガオ州"), ("kn", "ಸುರ\u{cbf}ಗಾವೊ ಡ\u{cc6} ನಾರ\u{ccd}ಟ\u{cc6}"), ("ko", "북수리가오 주"), ("lt", "Šiaurės Surigao"), ("lv", "Ziemeļsurigao"), ("mk", "Северен Суригао"), ("mr", "स\u{941}रिगाओ ड\u{947}ल नॉर\u{94d}ट"), ("ms", "Surigao del Norte"), ("nb", "Surigao del Norte"), ("nl", "Surigao del Norte"), ("no", "Surigao del Norte"), ("pl", "Surigao del Norte"), ("pt", "Surigão do Norte"), ("ru", "Северный Суригао"), ("si", "ස\u{dd4}ර\u{dd2}ග\u{dcf}ඕ ඩෙල\u{dca} නොර\u{dca}ටේ"), ("sv", "Surigao del Norte"), ("ta", "சுரிகையோ டெல\u{bcd} நோர\u{bcd}ட\u{bcd}"), ("te", "సుర\u{c3f}గ\u{c3e}వ\u{c4b} డ\u{c46}ల\u{c4d} న\u{c3e}ర\u{c4d}ట\u{c46}"), ("th", "ซ\u{e38}ร\u{e34}เกา เดล นอร\u{e4c}เต"), ("tr", "Surigao del Norte"), ("uk", "Північний Сурігао"), ("ur", "سوریگاؤ شمالی"), ("vi", "Surigao del Norte"), ("zh", "北苏里高省")]),
                        unofficial_name_list: ["Surigao del Norte"].to_vec(),
                    }
                ),
                (
                    "SUR",
                    Subdivision{
                        name: "SUR",
                        country_alpha2: Alpha2::PH,
                        code: "SUR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.5404906), longitude: Some(126.1144758), max_latitude: Some(9.496260099999999), min_latitude: Some(7.908138999999999), max_longitude: Some(126.4583336), min_longitude: Some(125.7439209)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سوريجاو ديل سور"), ("bn", "স\u{9c1}রিগ\u{9be}ও ডেল সোর"), ("ccp", "𑄥\u{1112a}𑄢\u{11128}𑄉𑄃\u{1112e} 𑄓𑄬𑄣\u{11134} 𑄥𑄢\u{11134}"), ("ceb", "Surigao del Sur"), ("da", "Surigao del Sur"), ("de", "Surigao del Sur"), ("el", "Σουριγκάο ντελ Σούρ"), ("en", "Surigao del Sur"), ("es", "Surigao del Sur"), ("fa", "سوریگائو جنوبی"), ("fi", "Surigao del Sur"), ("fr", "Surigao du Sud"), ("gu", "સ\u{ac1}રિગાઓ ડ\u{ac7}લ સ\u{ac1}ર"), ("hi", "स\u{941}रिगाओ द\u{947}ल स\u{942}र"), ("id", "Surigao del Sur"), ("it", "Provincia di Surigao del Sur"), ("ja", "南スリガオ州"), ("kn", "ಸುರ\u{cbf}ಗಾವೊ ಡ\u{cc6}ಲ\u{ccd} ಸುರ\u{ccd}"), ("ko", "남수리가오 주"), ("lt", "Pietų Surigao"), ("lv", "Dienvidsurigao"), ("mk", "Јужен Суригао"), ("mr", "स\u{941}रिगाओ ड\u{947}ल स\u{941}र"), ("ms", "Surigao del Sur"), ("nb", "Surigao del Sur"), ("nl", "Surigao del Sur"), ("no", "Surigao del Sur"), ("pl", "Surigao del Sur"), ("pt", "Surigão do Sur"), ("ru", "Южный Суригао"), ("si", "ස\u{dd4}ර\u{dd2}ග\u{dcf}ඕ ඩෙල\u{dca} සර\u{dca}"), ("sv", "Surigao del Sur"), ("ta", "சுரிகையோ டெல\u{bcd} சூர\u{bcd}"), ("te", "సుర\u{c3f}గ\u{c3e}వ\u{c4b} డ\u{c46}ల\u{c4d} సుర\u{c4d}"), ("th", "ซ\u{e39}ร\u{e35}เกา เดล เซอ"), ("tr", "Surigao del Sur"), ("uk", "Південий Сурігао"), ("ur", "سوریگاؤ جنوبی"), ("vi", "Surigao del Sur"), ("zh", "南苏里高省")]),
                        unofficial_name_list: ["Surigao del Sur"].to_vec(),
                    }
                ),
                (
                    "TAR",
                    Subdivision{
                        name: "TAR",
                        country_alpha2: Alpha2::PH,
                        code: "TAR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.4754786), longitude: Some(120.5963492), max_latitude: Some(15.543999), min_latitude: Some(15.4011971), max_longitude: Some(120.6978608), min_longitude: Some(120.5281542)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "تارلاك"), ("bn", "ট\u{9be}রল\u{9be}র\u{9cd}ক"), ("ca", "Tarlac"), ("ccp", "𑄑𑄢\u{11134}𑄣𑄇\u{11134}"), ("ceb", "Tarlac"), ("da", "Tarlac"), ("de", "Tarlac"), ("el", "Ταρλάκ"), ("en", "Tarlac"), ("es", "Tarlac"), ("fa", "تارلاک"), ("fi", "Tarlac"), ("fr", "Tarlac"), ("gu", "તારલક"), ("hi", "तरलाक प\u{94d}रान\u{94d}त"), ("hy", "Տարլակ"), ("id", "Tarlac"), ("it", "Provincia di Tarlac"), ("ja", "タルラック州"), ("kn", "ಟಾರ\u{ccd}ಲಾಕ\u{ccd}"), ("ko", "타를라크 주"), ("lt", "Tarlakas"), ("lv", "Tarlaka"), ("mk", "Тарлак"), ("mr", "तारलक"), ("ms", "Tarlac"), ("nb", "Tarlac"), ("nl", "Tarlac"), ("no", "Tarlac"), ("pl", "Tarlac"), ("pt", "Tarlac"), ("ru", "Тарлак"), ("si", "ටර\u{dca}ලක\u{dca}"), ("sv", "Tarlac"), ("ta", "ட\u{bbe}ர\u{bcd}லக\u{bcd}"), ("te", "ట\u{c3e}ర\u{c4d}ల\u{c3e}క\u{c4d}"), ("th", "ทาร\u{e4c}แลค"), ("tr", "Tarlac"), ("uk", "Тарлак"), ("ur", "تارلاک"), ("vi", "Tarlac"), ("zh", "丹轆省")]),
                        unofficial_name_list: ["Tarlac"].to_vec(),
                    }
                ),
                (
                    "TAW",
                    Subdivision{
                        name: "TAW",
                        country_alpha2: Alpha2::PH,
                        code: "TAW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(5.133811), longitude: Some(119.950926), max_latitude: Some(5.9914141), min_latitude: Some(4.616361299999999), max_longitude: Some(120.6829369), min_longitude: Some(119.3835042)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "تاوي تاوي"), ("bn", "ত\u{9be}বি-ত\u{9be}বি"), ("ca", "Tawi-Tawi"), ("ccp", "𑄑\u{11127}𑄠\u{11128}-𑄑\u{11127}𑄠\u{11128}"), ("ceb", "Tawi-Tawi"), ("cs", "Tawi-Tawi"), ("da", "Tawi-Tawi"), ("de", "Tawi-Tawi"), ("el", "Τάουι-Τάουι"), ("en", "Tawi-Tawi"), ("es", "Tawi-Tawi"), ("fa", "تاوی تاوی"), ("fi", "Tawi-Tawi"), ("fr", "Tawi-Tawi"), ("gu", "તાવી-તાવી"), ("hi", "तावी-तावी"), ("id", "Tawi-Tawi"), ("it", "Provincia di Tawi-Tawi"), ("ja", "タウイタウイ州"), ("kn", "ತಾವ\u{cbf}-ತಾವ\u{cbf}"), ("ko", "타위타위 주"), ("lt", "Tavi-Tavis"), ("lv", "Tavi-Tavi"), ("mk", "Тави-Тави"), ("mr", "तावी-तावी"), ("ms", "Tawi-Tawi"), ("nb", "Tawi-Tawi"), ("nl", "Tawi-Tawi"), ("no", "Tawi-Tawi"), ("pl", "Tawi-Tawi"), ("pt", "Tawi-Tawi"), ("ru", "Тави-Тави"), ("si", "ට\u{dcf}ව\u{dd2}-ට\u{dcf}ව\u{dd2}"), ("sk", "Tawi-Tawi"), ("sv", "Tawi-Tawi"), ("ta", "த\u{bbe}வி -த\u{bbe}வி"), ("te", "ట\u{c3e}వ\u{c3f}-ట\u{c3e}వ\u{c3f}"), ("th", "ทะว\u{e35}-ทะว\u{e35}"), ("tr", "Tawi-Tawi"), ("uk", "Таві-Таві"), ("ur", "تاوی تاوی"), ("vi", "Tawi-Tawi"), ("zh", "塔威塔威省")]),
                        unofficial_name_list: ["Tawi-Tawi"].to_vec(),
                    }
                ),
                (
                    "WSA",
                    Subdivision{
                        name: "WSA",
                        country_alpha2: Alpha2::PH,
                        code: "WSA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.5795195), longitude: Some(124.9748219), max_latitude: Some(12.334325), min_latitude: Some(11.0872409), max_longitude: Some(125.307814), min_longitude: Some(124.1477429)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سامار"), ("bg", "Самар"), ("bn", "স\u{9be}ম\u{9be}র"), ("ca", "Província de Samar"), ("ccp", "𑄥\u{11127}𑄟𑄢\u{11134}"), ("ceb", "Samar"), ("cs", "Samar"), ("da", "Samar"), ("de", "Samar"), ("el", "Σαμάρ"), ("en", "Samar"), ("es", "Sámar"), ("fa", "سامار"), ("fi", "Samar"), ("fr", "Samar"), ("gu", "સમર"), ("hi", "सामार प\u{94d}रान\u{94d}त"), ("id", "Samar"), ("it", "Provincia di Samar"), ("ja", "サマル州"), ("kn", "ಸಮರ\u{ccd}"), ("ko", "사마르 주"), ("lt", "Samaras"), ("lv", "Samara"), ("mk", "Самар"), ("mr", "समर"), ("ms", "Samar"), ("nb", "Samar"), ("nl", "Samar"), ("no", "Samar"), ("pl", "Samar"), ("pt", "Samar"), ("ru", "Самар"), ("si", "සම\u{dcf}ර\u{dca}"), ("sv", "Samar"), ("ta", "ச\u{bbe}மர\u{bcd}"), ("te", "సమర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซามาร\u{e4c}"), ("tr", "Samar"), ("uk", "Провінція Самар"), ("ur", "سامار (صوبہ)"), ("vi", "Samar"), ("zh", "薩馬省")]),
                        unofficial_name_list: ["Western Samar"].to_vec(),
                    }
                ),
                (
                    "ZAN",
                    Subdivision{
                        name: "ZAN",
                        country_alpha2: Alpha2::PH,
                        code: "ZAN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.3886282), longitude: Some(123.1688883), max_latitude: Some(8.856403499999999), min_latitude: Some(7.1448079), max_longitude: Some(123.559396), min_longitude: Some(121.9014683)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "زامبوانجا ديل نورت"), ("bn", "জিম\u{9cd}বোয\u{9bc}\u{9be}ঙ\u{9cd}গ\u{9be} ডেল নর\u{9cd}টে"), ("ccp", "𑄎𑄟\u{11134}𑄝\u{1112e}𑄠𑄋\u{11134}𑄉 𑄓𑄬𑄣\u{11134} 𑄚\u{11127}𑄢\u{11134}𑄑𑄬"), ("ceb", "Zamboanga del Norte"), ("da", "Zamboanga del Norte"), ("de", "Provinz Zamboanga del Norte"), ("el", "Ζαμποάνγκα ντελ Νόρτε"), ("en", "Zamboanga del Norte"), ("es", "Zamboanga del Norte"), ("fa", "زامبوانگا شمالی"), ("fi", "Zamboanga del Norte"), ("fr", "Zamboanga du Nord"), ("gu", "ઝામ\u{acd}બોઆ\u{a82}ગા ડ\u{ac7}લ નોર\u{acd}ટ"), ("hi", "ज\u{93c}म\u{94d}बोआ\u{902}गा द\u{947}ल नोर\u{94d}त\u{947}"), ("id", "Zamboanga del Norte"), ("it", "Provincia di Zamboanga del Norte"), ("ja", "北サンボアンガ州"), ("kn", "ಜಂಬೋಂಗಾ ಡ\u{cc6}ಲ\u{ccd} ನಾರ\u{ccd}ಟ\u{cc6}"), ("ko", "북삼보앙가 주"), ("lt", "Šiaurės Zamboanga"), ("lv", "Ziemeļu Zamboanga"), ("mk", "Северна Замбоанга"), ("mr", "झा\u{902}बोअ\u{902}गा ड\u{947}ल नॉर\u{94d}ट"), ("ms", "Zamboanga del Norte"), ("nb", "Zamboanga del Norte"), ("nl", "Zamboanga del Norte"), ("no", "Zamboanga del Norte"), ("pl", "Zamboanga del Norte"), ("pt", "Zamboanga del Norte"), ("ru", "Северная Замбоанга"), ("si", "සම\u{dca}බෝඅන\u{dca}ග\u{dcf} ඩෙල\u{dca} නොර\u{dca}ටේ"), ("sv", "Zamboanga del Norte"), ("ta", "சம\u{bcd}போவ\u{bbe}ங\u{bcd}க டெல\u{bcd} நோர\u{bcd}ட\u{bcd}டே"), ("te", "జంబ\u{c3e}వ\u{c4b}ంగ\u{c3e} డ\u{c46}ల\u{c4d} న\u{c3e}ర\u{c4d}ట\u{c46}"), ("th", "แซมโบอ\u{e31}นก\u{e49}า เดล นอเต\u{e49}"), ("tr", "Zamboanga del Norte"), ("uk", "Північна Замбоанга"), ("ur", "زامبوانگا شمالی"), ("vi", "Zamboanga del Norte"), ("zh", "北三寶顏省")]),
                        unofficial_name_list: ["Zamboanga del Norte"].to_vec(),
                    }
                ),
                (
                    "ZAS",
                    Subdivision{
                        name: "ZAS",
                        country_alpha2: Alpha2::PH,
                        code: "ZAS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.838305399999999), longitude: Some(123.2966657), max_latitude: Some(8.2364218), min_latitude: Some(7.3595234), max_longitude: Some(123.669302), min_longitude: Some(122.916847)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "زامبوانجا ديل سور"), ("bn", "জ\u{9cd}য\u{9be}ম\u{9cd}ব\u{9be}ংগ\u{9be} ডেল স\u{9c1}র"), ("ccp", "𑄎𑄟\u{11134}𑄝\u{1112e}𑄠𑄋\u{11134}𑄉 𑄓𑄬𑄣\u{11134} 𑄥𑄢\u{11134}"), ("ceb", "Zamboanga del Sur"), ("da", "Zamboanga del Sur"), ("de", "Zamboanga del Sur"), ("el", "Ζαμποάνγκα ντελ Σούρ"), ("en", "Zamboanga del Sur"), ("es", "Zamboanga del Sur"), ("fa", "زامبوانگا جنوبی"), ("fi", "Zamboanga del Sur"), ("fr", "Zamboanga du Sud"), ("gu", "ઝામબો\u{a82}ગા ડ\u{ac7}લ સ\u{ac1}ર"), ("hi", "ज\u{93c}म\u{94d}बोआ\u{902}गा द\u{947}ल स\u{942}र"), ("id", "Zamboanga del Sur"), ("it", "Provincia di Zamboanga del Sur"), ("ja", "南サンボアンガ州"), ("kn", "ಜಂಬೊಂಗಾ ಡ\u{cc6}ಲ\u{ccd} ಸುರ\u{ccd}"), ("ko", "남삼보앙가 주"), ("lt", "Pietų Zamboanga"), ("lv", "Dienvidzamboanga"), ("mk", "Јужна Замбоанга"), ("mr", "झा\u{902}बोअ\u{902}गा ड\u{947}ल स\u{941}र"), ("ms", "Zamboanga del Sur"), ("nb", "Zamboanga del Sur"), ("nl", "Zamboanga del Sur"), ("no", "Zamboanga del Sur"), ("pl", "Zamboanga del Sur"), ("pt", "Zamboanga del Sur"), ("ru", "Южная Замбоанга"), ("si", "සැම\u{dca}බෝඅන\u{dca}ග\u{dcf} ඩෙල\u{dca} සර\u{dca}"), ("sv", "Zamboanga del Sur"), ("ta", "ச\u{bbe}ம\u{bcd}போங\u{bcd}க டேல\u{bcd} சூர\u{bcd}"), ("te", "జ\u{c3e}ంబ\u{c4b}ంగ\u{c3e} డ\u{c46}ల\u{c4d} సుర\u{c4d}"), ("th", "แซมโบอ\u{e31}นกา เดล เซอร\u{e4c}"), ("tr", "Zamboanga del Sur"), ("uk", "Південна Замбоанга"), ("ur", "زامبوانگا جنوبی"), ("vi", "Zamboanga del Sur"), ("zh", "南三寶顏省")]),
                        unofficial_name_list: ["Zamboanga del Sur"].to_vec(),
                    }
                ),
                (
                    "ZMB",
                    Subdivision{
                        name: "ZMB",
                        country_alpha2: Alpha2::PH,
                        code: "ZMB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.5081766), longitude: Some(119.9697808), max_latitude: Some(15.845467), min_latitude: Some(14.7397255), max_longitude: Some(120.462163), min_longitude: Some(117.8441242)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "زامباله"), ("az", "Zambales"), ("bn", "জ\u{9be}ম\u{9cd}ব\u{9be}লে"), ("ca", "Zambales"), ("ccp", "𑄎𑄟\u{11134}𑄝𑄣\u{11128}𑄌\u{11134}"), ("ceb", "Zambales"), ("da", "Zambales"), ("de", "Zambales"), ("el", "Ζάμπαλες"), ("en", "Zambales"), ("es", "Zambales"), ("fa", "زامباله"), ("fi", "Zambales"), ("fr", "Zambales"), ("gu", "ઝામ\u{acd}બ\u{ac7}લ\u{acd}સ"), ("he", "סאמבאלס"), ("hi", "ज\u{93c}म\u{94d}बाल\u{947}स प\u{94d}रान\u{94d}त"), ("id", "Zambales"), ("it", "Provincia di Zambales"), ("ja", "サンバレス州"), ("kn", "ಝಂಬಾಲ\u{cc6}ಸ\u{ccd}"), ("ko", "삼발레스 주"), ("lt", "Zambalesas"), ("lv", "Zambale"), ("mk", "Замбалес"), ("mr", "झा\u{902}बल\u{947}स"), ("ms", "Zambales"), ("nb", "Zambales"), ("nl", "Zambales"), ("no", "Zambales"), ("pl", "Zambales"), ("pt", "Zambales"), ("ru", "Самбалес"), ("si", "සැම\u{dca}බල\u{dd3}ස\u{dca}"), ("sv", "Zambales"), ("ta", "ச\u{bbe}ம\u{bcd}பல\u{bcd}ஸ\u{bcd}"), ("te", "జ\u{c3e}ంబ\u{c47}ల\u{c46}స\u{c4d}"), ("th", "แซมบาเลส"), ("tr", "Zambales"), ("uk", "Самбалес"), ("ur", "زامبالیس"), ("vi", "Zambales"), ("zh", "三描礼士省")]),
                        unofficial_name_list: ["Zambales"].to_vec(),
                    }
                ),
                (
                    "ZSI",
                    Subdivision{
                        name: "ZSI",
                        country_alpha2: Alpha2::PH,
                        code: "ZSI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.02427), longitude: Some(122.189), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "زامبوانجا سيبوجاي"), ("bn", "জ\u{9be}ম\u{9cd}ব\u{9c1}য\u{9bc}\u{9be}ঙ\u{9cd}গ\u{9be} সিব\u{9c1}গ\u{9be}ই"), ("ca", "Zamboanga Sibugay"), ("ccp", "𑄎𑄟\u{11134}𑄝\u{1112e}𑄠𑄋\u{11134}𑄉 𑄥\u{11128}𑄝\u{1112a}𑄉𑄬"), ("ceb", "Zamboanga Sibugay"), ("da", "Zamboanga Sibugay"), ("de", "Zamboanga Sibugay"), ("el", "Ζαμποάνγκα Σιμπουγκάϊ"), ("en", "Zamboanga Sibugay"), ("es", "Zamboanga Sibugay"), ("fa", "زامبوانگا سیبوگای"), ("fi", "Zamboanga Sibugay"), ("fr", "Zamboanga Sibugay"), ("gu", "ઝામ\u{acd}બોઆ\u{a82}ગા સિબ\u{ac1}ગ\u{ac7}"), ("hi", "ज\u{93c}म\u{94d}बोआ\u{902}गा सिब\u{941}गय"), ("id", "Zamboanga Sibugay"), ("it", "Provincia di Zamboanga Sibugay"), ("ja", "サンボアンガ・シブガイ州"), ("kn", "ಜಂಬೊಂಗಾ ಸ\u{cbf}ಬುಗ\u{cc6}"), ("ko", "삼보앙가시부가이 주"), ("lt", "Sibugajaus Samboanga"), ("lv", "Zamboanga-Sibugeja"), ("mk", "Замбоанга Сибугај"), ("mr", "झा\u{902}बोआ\u{902}गा सिबगाय\u{947}"), ("ms", "Zamboanga Sibugay"), ("nb", "Zamboanga Sibugay"), ("nl", "Zamboanga Sibugay"), ("no", "Zamboanga Sibugay"), ("pl", "Zamboanga Sibugay"), ("pt", "Zamboanga Sibugay"), ("ru", "Замбоанга-Сибугей"), ("si", "සබෝඅන\u{dca}ග\u{dcf} ස\u{dd2}බ\u{dca}ය\u{dd4}ගේ"), ("sv", "Zamboanga Sibugay"), ("ta", "சம\u{bcd}பொங\u{bcd}க சிப\u{bcd}புக\u{bcd}க\u{bbe}ய\u{bcd}"), ("te", "జ\u{c3e}ంబ\u{c4b}ంగ\u{c3e} స\u{c3f}బుగ\u{c47}"), ("th", "ซ\u{e31}มบวงกา ซ\u{e35}บ\u{e39}เกย\u{e4c}"), ("tr", "Zamboanga Sibugay"), ("uk", "Замбоанга-Сибугай"), ("ur", "زامبوانگا سیبوگائے"), ("vi", "Zamboanga Sibugay"), ("zh", "三寶顏錫布格省")]),
                        unofficial_name_list: ["Zamboanga Sibuguey [Zamboanga Sibugay]"].to_vec(),
                    }
                ),
            ]

        )
    }
}
#[allow(unused_imports)]
use crate::{Alpha2, Alpha3, Continent, Country, Region, SubRegion, WeekDay, WorldRegion, GEC};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "ph")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::PH,
        alpha3: Alpha3::PHL,
        address_format: Some(
            "{{recipient}}\n{{street}} {{region_short}}\n{{postalcode}} {{city}}\n{{country}}",
        ),
        continent: Continent::Asia,
        country_code: 63,
        currency_code: "PHP",
        gec: Some(GEC::RP),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("PHI"),
        iso_long_name: "The Republic of the Philippines",
        iso_short_name: "Philippines",
        official_language_list: ["en", "tl"].to_vec(),
        spoken_language_list: ["en", "tl"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8, 9, 10].to_vec(),
        national_prefix: "0",
        nationality: Some("Filipino"),
        number: "608",
        postal_code: true,
        postal_code_format: Some("\\d{4}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthEasternAsia),
        un_locode: "PH",
        unofficial_name_list: [
            "Philippines",
            "Philippinen",
            "Filipinas",
            "フィリピン",
            "Filipijnen",
        ]
        .to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Philippines"),
            ("af", "Filippyne"),
            ("ak", "Philippines"),
            ("am", "ፊሊፒንስ"),
            ("an", "Philippines"),
            ("ar", "الفلب\u{651}ين"),
            ("as", "ফিলিপ\u{9be}ইনছ"),
            ("ay", "Philippines"),
            ("az", "Filippin"),
            ("ba", "Philippines"),
            ("be", "Філіпіны"),
            ("bg", "Филипини"),
            ("bi", "Philippines"),
            ("bn", "ফিলিপ\u{9be}ইনস"),
            ("bn_IN", "ফিলিপ\u{9be}ইনস"),
            ("br", "Filipinez"),
            ("bs", "Filipini"),
            ("ca", "Filipines"),
            ("ce", "Филиппин"),
            ("ch", "Filipinas"),
            ("cs", "Filipíny"),
            ("cv", "Филиппин"),
            ("cy", "Pilipinas"),
            ("da", "Filippinerne"),
            ("de", "Philippinen"),
            ("dv", "ފ\u{7a8}ލ\u{7a8}ޕ\u{7a9}ނ\u{7b0}ސ\u{7b0}"),
            ("dz", "ཕ\u{f72}་ལ\u{f72}་པ\u{f72}ནས\u{f72}།"),
            ("ee", "Philippines"),
            ("el", "Φιλιππίνες"),
            ("en", "Philippines"),
            ("eo", "Filipinoj"),
            ("es", "Filipinas"),
            ("et", "Filipiinid"),
            ("eu", "Filipinak"),
            ("fa", "فیلیپین"),
            ("ff", "Philippines"),
            ("fi", "Filippiinit"),
            ("fo", "Filipsoyggjar"),
            ("fr", "Philippines"),
            ("fy", "Filipinen"),
            ("ga", "Na hOileáin Fhilipíneacha"),
            ("gl", "Filipinas"),
            ("gn", "Philippines"),
            ("gu", "ફિલિપાઇન\u{acd}સ"),
            ("gv", "Ny h-Ellanyn Philippeenagh"),
            ("ha", "Filipin"),
            ("he", "הפיליפינים"),
            ("hi", "फ\u{93c}िलीपीन\u{94d}स"),
            ("hr", "Filipini"),
            ("ht", "Filipin"),
            ("hu", "Fülöp-szigetek"),
            ("hy", "Ֆիլիպիններ"),
            ("ia", "Philippinas"),
            ("id", "Filipina"),
            ("io", "Filipini"),
            ("is", "Filippseyjar"),
            ("it", "Filippine"),
            ("iu", "Philippines"),
            ("ja", "フィリピン"),
            ("ka", "ფილიპინები"),
            ("ki", "Philippines"),
            ("kk", "Филиппин"),
            ("kl", "Philippines"),
            ("km", "ហ\u{17d2}វ\u{17b8}ល\u{17b8}ព\u{17b8}ន"),
            ("kn", "ಫ\u{cbf}ಲ\u{cbf}ಪ\u{ccd}ಪೈನ\u{ccd}ಸ\u{ccd}"),
            ("ko", "필리핀"),
            ("ku", "Fîlîpîn"),
            ("kv", "Филиппинъяс"),
            ("kw", "Filipinys"),
            ("ky", "Филиппиндер"),
            ("lo", "ປະເທດຟ\u{eb5}ລ\u{eb4}ບປ\u{eb4}ນ"),
            ("lt", "Filipinai"),
            ("lv", "Filipīnas"),
            ("mi", "Piripīni"),
            ("mk", "Филипини"),
            ("ml", "ഫിലിപ\u{d4d}പൈന\u{d4d}\u{200d}സ\u{d4d}"),
            ("mn", "Флиппен"),
            ("mr", "फिलिपिन\u{94d}स"),
            ("ms", "Filipina"),
            ("mt", "Filippini"),
            (
                "my",
                "ဖ\u{102d}လစ\u{103a}ပ\u{102d}\u{102f}င\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Eben Piripin"),
            ("nb", "Filippinene"),
            ("ne", "फिलिपिन\u{94d}स"),
            ("nl", "Filipijnen"),
            ("nn", "Filippinane"),
            ("nv", "Kéyah Dańlíinii"),
            ("oc", "Filipinas"),
            ("or", "ଫ\u{b3f}ଲ\u{b3f}ପ\u{b3e}ଇନ\u{b4d}ସ"),
            ("pa", "ਫਿਲਿਪੀਨੀਜ਼"),
            ("pi", "फिलिपीन\u{94d}स"),
            ("pl", "Filipiny"),
            ("ps", "فلېپين"),
            ("pt", "Filipinas"),
            ("pt_BR", "Filipinas"),
            ("ro", "Filipine"),
            ("ru", "Филиппины"),
            ("rw", "Filipine"),
            ("sc", "Filipinas"),
            ("sd", "Philippines"),
            ("si", "ප\u{dd2}ල\u{dd2}ප\u{dd3}න"),
            ("sk", "Filipíny"),
            ("sl", "Filipini"),
            ("so", "Filibiin"),
            ("sq", "Filipine"),
            ("sr", "Филипини"),
            ("sv", "Filippinerna"),
            ("sw", "Philippines"),
            ("ta", "பிலிப\u{bcd}பைன\u{bcd}ஸ\u{bcd}"),
            ("te", "ఫ\u{c3f}ల\u{c3f}ప\u{c4d}ప\u{c3f}న\u{c4d}స\u{c4d}"),
            ("tg", "Филиппинҳо"),
            ("th", "ฟ\u{e34}ล\u{e34}ปป\u{e34}นส\u{e4c}"),
            ("ti", "Philippines"),
            ("tk", "Filippinler"),
            ("tl", "Pilipinas"),
            ("tr", "Filipinler"),
            ("tt", "Филиппиннәр"),
            ("ug", "فىلىپپىن"),
            ("uk", "Філіппіни"),
            ("ur", "فلپائن"),
            ("uz", "Filippin"),
            ("ve", "Philippines"),
            ("vi", "Phi-li-pi-nợ"),
            ("wa", "Filipenes"),
            ("wo", "Filipiin"),
            ("xh", "Philippines"),
            ("yo", "Filipínì"),
            ("zh_CN", "菲律宾"),
            ("zh_HK", "菲律賓"),
            ("zh_TW", "菲律賓"),
            ("zu", "Philippines"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
