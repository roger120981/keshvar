// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Maldives

#[cfg(all(feature = "mv", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::MV;
    pub const ALPHA3: Alpha3 = Alpha3::MDV;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 960;
    pub const CURRENCY_CODE: &str = "MVR";
    pub const GEC: Option<GEC> = Some(GEC::MV);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("MDV");
    pub const ISO_SHORT_NAME: &str = "Maldives";
    pub const ISO_LONG_NAME: &str = "The Republic of Maldives";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["dv"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["dv"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Maldivan");
    pub const NUMBER: &str = "462";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Sunday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthernAsia);
    pub const UN_LOCODE: &str = "MV";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Maldives",
        "Malediven",
        "Maldivas",
        "モルディブ",
        "Maldiven",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Maldives"),
        ("af", "Maledive"),
        ("ak", "Maldives"),
        ("am", "ማልዲቭስ"),
        ("an", "Maldives"),
        ("ar", "جزر المالديف"),
        ("as", "ম\u{9be}লডিভ\u{9cd}\u{200c}চ"),
        ("ay", "Maldives"),
        ("az", "Maldives"),
        ("ba", "Maldives"),
        ("be", "Мальдывы"),
        ("bg", "Малдиви"),
        ("bi", "Maldives"),
        ("bn", "ম\u{9be}লডিভস"),
        ("bn_IN", "ম\u{9be}লডিভস"),
        ("br", "Maldivez"),
        ("bs", "Maldivi"),
        ("ca", "Maldives"),
        ("ce", "Мальдиви"),
        ("ch", "Maldives"),
        ("cs", "Maledivy"),
        ("cv", "Мальдиви"),
        ("cy", "Maldives"),
        ("da", "Maldiverne"),
        ("de", "Malediven"),
        ("dv", "ދ\u{7a8}ވ\u{7ac}ހ\u{7a8}ރ\u{7a7}އ\u{7b0}ޖ\u{7ac}"),
        ("dz", "མ\u{f7a}ལ་ཌ\u{f72}བས\u{f72}།"),
        ("ee", "Maldives"),
        ("el", "Μαλδίβες"),
        ("en", "Maldives"),
        ("eo", "Maldivoj"),
        ("es", "Islas Maldivas"),
        ("et", "Maldiivid"),
        ("eu", "Maldivak"),
        ("fa", "مالدیو"),
        ("ff", "Maldives"),
        ("fi", "Malediivit"),
        ("fo", "Maldivuoyggjarnar"),
        ("fr", "Maldives"),
        ("fy", "Maldiven"),
        ("ga", "Oileáin Mhaildíve"),
        ("gl", "Maldivas"),
        ("gn", "Maldives"),
        ("gu", "માલ\u{acd}દીવ\u{acd}સ"),
        ("gv", "Ny Maldeevaghyn"),
        ("ha", "Maldives"),
        ("he", "האיים המלדיביים"),
        ("hi", "मालदीव"),
        ("hr", "Maldivi"),
        ("ht", "Maldiv"),
        ("hu", "Maldív-szigetek"),
        ("hy", "Մալդիվներ"),
        ("ia", "Maldivas"),
        ("id", "Maladewa"),
        ("io", "Maldivi"),
        ("is", "Maldíveyjar"),
        ("it", "Maldive"),
        ("iu", "Maldives"),
        ("ja", "モルディブ"),
        ("ka", "მალდივის კუნძულები"),
        ("ki", "Maldives"),
        ("kk", "Мальдив аралдары"),
        ("kl", "Maldives"),
        ("km", "ម\u{17c9}ាល\u{17cb}ឌ\u{17b8}វ"),
        ("kn", "ಮಾಲ\u{ccd}ಡೀವ\u{ccd}ಸ\u{ccd}"),
        ("ko", "몰디브"),
        ("ku", "Giravên Maldiv"),
        ("kv", "Мальдивъяс"),
        ("kw", "Maldivys"),
        ("ky", "Мальдивдер"),
        ("lo", "Maldives"),
        ("lt", "Maldyvai"),
        ("lv", "Maldīvija"),
        ("mi", "Maldives"),
        ("mk", "Малдиви"),
        ("ml", "മ\u{d3e}ല\u{d4d}\u{200d}ഡീവ\u{d4d}സ\u{d4d}"),
        ("mn", "Малдивс"),
        ("mr", "मालदिव\u{94d}हज\u{94d}"),
        ("ms", "Maldiv"),
        ("mt", "Maldivi"),
        (
            "my",
            "မော\u{103a}လဒ\u{102d}\u{102f}က\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Mardib"),
        ("nb", "Maldivene"),
        ("ne", "मालदिव\u{94d}स"),
        ("nl", "Maldiven"),
        ("nn", "Maldivane"),
        ("nv", "Naakaii Dootłʼizhí Bikéyah Yázhí"),
        ("oc", "Maldivas"),
        ("or", "ମ\u{b3e}ଲଦୀଭ"),
        ("pa", "ਮਾਲਦੀਵ"),
        ("pi", "मालदीव"),
        ("pl", "Malediwy"),
        ("ps", "مالديپ"),
        ("pt", "Maldivas"),
        ("pt_BR", "Maldivas"),
        ("ro", "Maldive"),
        ("ru", "Мальдивы"),
        ("rw", "Malidivezi"),
        ("sc", "Maldivas"),
        ("sd", "مالديپ"),
        ("si", "ම\u{dcf}ල ද\u{dd2}වය\u{dd2}න"),
        ("sk", "Maldivy"),
        ("sl", "Maldivi"),
        ("so", "Maaldiqeen"),
        ("sq", "Maldive"),
        ("sr", "Малдиви"),
        ("sv", "Maldiverna"),
        ("sw", "Maldives"),
        ("ta", "ம\u{bbe}லத\u{bc0}வுகள\u{bcd}"),
        ("te", "మ\u{c3e}ల\u{c4d}ద\u{c40}వ\u{c4d}స\u{c4d}"),
        ("tg", "Малдив"),
        ("th", "ม\u{e31}ลด\u{e35}ฟส\u{e4c}"),
        ("ti", "Maldives"),
        ("tk", "Maldiwa"),
        ("tl", "Maldives"),
        ("tr", "Maldivler"),
        ("tt", "Малдивлар"),
        ("ug", "مالدىۋې"),
        ("uk", "Мальдіви"),
        ("ur", "مالدیپ"),
        ("uz", "Maldivalar"),
        ("ve", "Maldives"),
        ("vi", "Mal-đi-vợx"),
        ("wa", "Maldives"),
        ("wo", "Maldiif"),
        ("xh", "Maldives"),
        ("yo", "Àwọn Maldive"),
        ("zh_CN", "马尔代夫"),
        ("zh_HK", "馬爾代夫"),
        ("zh_TW", "馬爾地夫"),
        ("zu", "Maldives"),
    ];
    #[cfg(all(feature = "mv", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 3.202778;
        pub const LONGITUDE: f64 = 73.22068;
        pub const MAX_LATITUDE: f64 = 7.5149809;
        pub const MAX_LONGITUDE: f64 = 74.7290038;
        pub const MIN_LATITUDE: f64 = -1.2907844;
        pub const MIN_LONGITUDE: f64 = 71.75170899999999;
        pub const NORTHEAST_LATITUDE: f64 = 7.5149809;
        pub const NORTHEAST_LONGITUDE: f64 = 74.7290038;
        pub const SOUTHWEST_LATITUDE: f64 = -1.2907844;
        pub const SOUTHWEST_LONGITUDE: f64 = 71.75170899999999;
    }
}
#[cfg(all(feature = "mv", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 3.202778,
            longitude: 73.22068,
            max_latitude: 7.5149809,
            max_longitude: 74.7290038,
            min_latitude: -1.2907844,
            min_longitude: 71.75170899999999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 7.5149809,
                    longitude: 74.7290038,
                },
                southwest: CountryGeoBound {
                    latitude: -1.2907844,
                    longitude: 71.75170899999999,
                },
            },
        }
    }
}

#[cfg(all(feature = "mv", feature = "subdivisions"))]
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
                        country_alpha2: Alpha2::MV,
                        code: "00",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeAtoll,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أليف دال أتول"), ("bn", "আলিফ ধ\u{9be}ল আতোল"), ("ccp", "𑄃𑄣\u{11128}𑄛\u{11134} 𑄙𑄣\u{11134}"), ("ceb", "Southern Ari Atoll"), ("cs", "Alif Dál"), ("da", "Alif Dhaal Atoll"), ("de", "Alif Dhaal"), ("dv", "Ariatholhu Dhekunuburi"), ("el", "Αλίφ Ντχάαλ Ατόλ"), ("en", "South Ari Atoll"), ("es", "Atolón Alif Dhaal"), ("fi", "Alif Dhaal Atoll"), ("fr", "Alif Dhaal"), ("gu", "અલિફ ઢાલ એટોલ"), ("hi", "अलिफ ढाल एटोल"), ("hu", "Déli-Ari-atoll"), ("id", "Atol Alif Dhaal"), ("it", "Atollo Alif Dhaal"), ("ja", "アリフ・ダール環礁区"), ("kn", "ಅಲ\u{cbf}ಫ\u{ccd} ಧಾಲ\u{ccd} ಅಟಾಲ\u{ccd}"), ("ko", "알리프달 환초"), ("lt", "Alif Daalo atolas"), ("lv", "Alifu Daalu atols"), ("mr", "अलिफ ढळ एटोल"), ("ms", "Alif Dhaal Atoll"), ("nb", "Alif Dhaal Atoll"), ("nl", "Alif Dhaal-atol"), ("no", "Alif Dhaal Atoll"), ("pl", "Alif Dhaal"), ("pt", "Alif Dhaal Atoll"), ("ru", "Атолл Алифу-Даалу"), ("si", "අල\u{dd2}ෆ\u{dca} ඩ\u{dcf}ල\u{dca} ද\u{dd4}පත\u{dca} සම\u{dd6}හය"), ("sk", "Alif Dál"), ("sv", "Ari Atholhu Dhekunuburi"), ("ta", "அளிப\u{bcd} த\u{bbe}ல\u{bcd} அட\u{bbe}ல\u{bcd}"), ("te", "అల\u{c3f}ఫ\u{c4d} ఢ\u{c3e}ల\u{c4d} అట\u{c4b}ల\u{c4d}"), ("th", "อล\u{e34}ฟ ดาฮ\u{e4c} อโทล"), ("tr", "Alif Dhaal Atoll"), ("uk", "Атол Аліф-Даал"), ("ur", "الف ڈھال اتول"), ("vi", "Alif Dhaal Atoll"), ("zh", "阿利夫達爾環礁")]),
                        unofficial_name_list: ["Alif Dhaal", "Alif Dhaalu"].to_vec(),
                    }
                ),
                (
                    "01",
                    Subdivision{
                        name: "01",
                        country_alpha2: Alpha2::MV,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-0.6413), longitude: Some(73.158), max_latitude: Some(-0.5765364), min_latitude: Some(-0.7036652), max_longitude: Some(73.2395624), min_longitude: Some(73.0761462)}),
                        comments: None,
                        subdivision_type: SubdivisionType::City,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ca", "Atol d’Addu"), ("ccp", "𑄃𑄓\u{11133}𑄦\u{1112a}"), ("ceb", "Seenu Atholhu"), ("cs", "Addu (atol)"), ("de", "Addu-Atoll"), ("dv", "Addu"), ("en", "Addu City"), ("es", "Seenu"), ("eu", "Seenu"), ("fr", "Atoll Addu"), ("hu", "Addu-atoll"), ("id", "Atol Addu"), ("it", "Atollo Seenu"), ("ja", "アッドゥ環礁"), ("ko", "아두 환초"), ("lt", "Adū atolas"), ("mk", "Аду"), ("nl", "Seenu-atol"), ("pl", "Addu"), ("ru", "Сиину"), ("sk", "Sínu"), ("sv", "Seenu Atholhu"), ("ta", "அட\u{bcd}டு பவளத\u{bcd}த\u{bc0}வு"), ("zh", "阿杜環礁")]),
                        unofficial_name_list: ["Addu Atholhu", "Addu Atoll", "Seenu", "Sîn"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::MV,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.8974984), longitude: Some(73.22851190000002), max_latitude: Some(7.1063277), min_latitude: Some(6.8074339), max_longitude: Some(73.23589319999999), min_longitude: Some(72.8088748)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeAtoll,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أليف أليف أتول"), ("bn", "আলিফ আলিফ এটল"), ("ccp", "𑄃𑄣\u{11128}𑄛\u{11134} 𑄃𑄣\u{11128}𑄛\u{11134}"), ("ceb", "Alifu Atholhu"), ("cs", "Alif Alif"), ("da", "Alif Alif Atoll"), ("de", "Alif Alif"), ("dv", "Ariatholhu Uthuruburi"), ("el", "Αλίφ Αλίφ Ατόλ"), ("en", "Alif Alif"), ("es", "Atolón Alif Alif"), ("fi", "Alif Alif Atoll"), ("fr", "Alif Alif"), ("gu", "અલિફ અલિફ એટોલ"), ("hi", "अलिफ अलीफ एटोल"), ("hu", "Északi-Ari-atoll"), ("id", "Atol Alif Alif"), ("it", "Atollo Alif Alif"), ("ja", "アリフ・アリフ・アトル"), ("kn", "ಅಲ\u{cbf}ಫ\u{ccd} ಅಲ\u{cbf}ಫ\u{ccd} ಅಟಾಲ\u{ccd}"), ("ko", "알리프알리프 환초"), ("lt", "Alif Alifo atolas"), ("lv", "Alif Alifas atols"), ("mr", "अलिफ अलीफ एटॉल"), ("ms", "Alif Alif Atoll"), ("nb", "Alif Alif Atoll"), ("nl", "Alif Alif-atol"), ("no", "Alif Alif Atoll"), ("pl", "Alif Alif"), ("pt", "Alif Alif Atoll"), ("ru", "Алиф-Алиф"), ("si", ", අල\u{dd2}ෆ\u{dca} අල\u{dd2}ෆ\u{dca} ද\u{dd4}පත\u{dca} සම\u{dd6}හය"), ("sk", "Alif Alif"), ("sv", "Alifu Atholhu"), ("ta", "அலிப\u{bcd} அளிப\u{bcd} அட\u{bbe}ல\u{bcd}"), ("te", "అల\u{c3f}ఫ\u{c4d} అల\u{c3f}ఫ\u{c4d} అట\u{c4b}ల\u{c4d}"), ("th", "อ\u{e31}ลล\u{e34}ฟ อ\u{e31}ลล\u{e34}ฟ อ\u{e31}ลทอยล\u{e4c}"), ("tr", "Alif Alif Atoll"), ("uk", "Атол Аліф-Аліф"), ("ur", "الف الف اتول"), ("vi", "Alif Alif Atoll"), ("zh", "阿利夫阿利夫環礁")]),
                        unofficial_name_list: ["Alif Atoll Dhekunu", "Alifu Alifu", "North Ari Atoll"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::MV,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(5.4039702), longitude: Some(73.6444762), max_latitude: Some(5.5575245), min_latitude: Some(5.2431393), max_longitude: Some(73.6577511), min_longitude: Some(73.3307469)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeAtoll,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لافياني أتول"), ("bn", "হ\u{9cd}ল\u{9be}ভিয\u{9bc}\u{9be}নি আতোল"), ("ccp", "𑄦\u{11133}𑄣𑄞\u{11128}𑄠𑄚\u{11128}"), ("ceb", "Lhaviyani Atholhu"), ("cs", "Lavijani"), ("da", "Lhaviyani Atoll"), ("de", "Faadhippolhu-Atoll"), ("dv", "Faadhippolhu"), ("el", "Λαβιγιάνι Ατόλ"), ("en", "Faadhippolhu"), ("es", "Atolón Lhaviyani"), ("fi", "Lhaviyani Atoll"), ("fr", "Atoll Faadhippolhu"), ("gu", "લ\u{acd}હાવિયાની એટોલ"), ("he", "אטול לביאני"), ("hi", "लवियानी एटोल"), ("hu", "Fadiffolu-atoll"), ("id", "Atol Lhaviyani"), ("it", "Atollo Lhaviyani"), ("ja", "ラビアニ環礁"), ("kn", "ಲವ\u{cbf}ಯನ\u{cbf} ಅಟಾಲ\u{ccd}"), ("ko", "라비야니 환초"), ("lt", "Lchavijanio atolas"), ("lv", "Lavijani atols"), ("mr", "ल\u{941}वियानी अटॉल"), ("ms", "Lhaviyani Atoll"), ("nb", "Lhaviyani Atoll"), ("nl", "Lhaviyani-atol"), ("no", "Lhaviyani Atoll"), ("pl", "Lhaviyani"), ("pt", "Lhaviyani Atoll"), ("ru", "Лхавийани"), ("si", "ලව\u{dd2}ය\u{dcf}න\u{dd2} ද\u{dd4}පත\u{dca} සම\u{dd6}හය"), ("sk", "Lavijani"), ("sv", "Lhaviyani Atholhu"), ("ta", "ல\u{bcd}ஹவியணி அட\u{bbe}ல\u{bcd}"), ("te", "ల\u{c3e}వ\u{c3f}య\u{c3e}న\u{c3f} అట\u{c4b}ల\u{c4d}"), ("th", "ลยว\u{e34}ยน\u{e34}"), ("tr", "Lhayviyani Atoll"), ("uk", "Атол Лхавійані"), ("ur", "لحاویانی اتول"), ("vi", "Lhaviyani Atoll"), ("zh", "拉黑比亞尼環礁")]),
                        unofficial_name_list: ["Fadiffolu", "Laviyani"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::MV,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(3.6846389), longitude: Some(73.4002915), max_latitude: Some(3.687583399999999), min_latitude: Some(3.3134959), max_longitude: Some(73.7596537), min_longitude: Some(73.36392049999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeAtoll,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "فافو أتول"), ("bn", "ভ\u{9be}ভ\u{9c1} আতোল"), ("ccp", "𑄞𑄞\u{1112a}"), ("ceb", "Vaavu Atholhu"), ("da", "Vaavu Atoll"), ("de", "Vaavu"), ("dv", "Felidheatholhu"), ("el", "Βαάβου Ατόλ"), ("en", "Felidhu Atoll"), ("es", "Atolón Vaavu"), ("fi", "Vaavu Atoll"), ("fr", "Vaavu"), ("gu", "વાવ\u{ac1} એટોલ"), ("hi", "वाव\u{942} एटोल"), ("hu", "Felidu-atoll"), ("id", "Atol Vaavu"), ("it", "Atollo Vaavu"), ("ja", "フェリデ環礁"), ("kn", "ವಾವು ಅಟಾಲ\u{ccd}"), ("ko", "바부 환초"), ("lt", "Vaavu atolas"), ("lv", "Vaavu atols"), ("mr", "व\u{945}व एटोल"), ("ms", "Vaavu Atoll"), ("nb", "Vaavu Atoll"), ("nl", "Vaavu-atol"), ("no", "Vaavu Atoll"), ("pl", "Vaavu"), ("pt", "Vaavu Atoll"), ("ru", "Вааву"), ("si", "ව\u{dcf}ව\u{dd4} ද\u{dd6}පත\u{dca} සම\u{dd6}හය"), ("sk", "Vávu"), ("sv", "Vaavu Atholhu"), ("ta", "வ\u{bbe}வு அட\u{bbe}ல\u{bcd}"), ("te", "వ\u{c3e}వు అట\u{c4b}ల\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซอร\u{e4c}โซกอน"), ("tr", "Vaavu Atoll"), ("uk", "Атол Вааву"), ("ur", "واوو اتول"), ("vi", "Vaavu Atoll"), ("zh", "瓦夫環礁")]),
                        unofficial_name_list: ["Felidhe Atholhu", "Felidhu Atoll", "Felidu", "Vaafu", "Vaavu", "Vâv"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::MV,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(1.9772276), longitude: Some(73.536101), max_latitude: Some(2.1331721), min_latitude: Some(1.7791128), max_longitude: Some(73.58596229999999), min_longitude: Some(73.2375483)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeAtoll,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لامو أتول"), ("bn", "ল\u{9be}ম\u{9c1} আতোল"), ("ccp", "𑄣\u{11133}𑄃𑄟\u{1112a}"), ("ceb", "Laamu Atholhu"), ("da", "Laamu Atoll"), ("de", "Laamu"), ("dv", "Hahdhunmathi"), ("el", "Λαάμου Ατόλ"), ("en", "Hahdhunmathi"), ("es", "Atolón Laamu"), ("fi", "Laamu Atoll"), ("fr", "Atoll Hadhdhunmathi"), ("gu", "લામ\u{ac1} એટોલ"), ("hi", "लाम\u{942} एटोल"), ("hu", "Haddunmati-atoll"), ("id", "Atol Laamu"), ("it", "Atollo Laamu"), ("ja", "ラーム環礁"), ("kn", "ಲಾಮು ಅಟಾಲ\u{ccd}"), ("ko", "라무 환초"), ("lt", "Lamu atolas"), ("lv", "Laamu atols"), ("mr", "लामा एटोल"), ("ms", "Laamu Atoll"), ("nb", "Laamu Atoll"), ("nl", "Laamu-atol"), ("no", "Laamu Atoll"), ("pl", "Laamu"), ("pt", "Atol Laamu"), ("ru", "Лааму"), ("si", "ල\u{dcf}ම\u{dd4} ද\u{dd4}පත\u{dca} සම\u{dd6}හය"), ("sk", "Lámu"), ("sv", "Laamu Atholhu"), ("ta", "ல\u{bbe}மு அட\u{bbe}ல\u{bcd}"), ("te", "ల\u{c3e}ము అట\u{c4b}ల\u{c4d}"), ("th", "ด\u{e34}นแดนคาบารอฟสค\u{e4c}"), ("tr", "Laamu Atoll"), ("uk", "Атол Лааму"), ("ur", "لامو اتول"), ("vi", "Laamu Atoll"), ("zh", "拉穆環礁")]),
                        unofficial_name_list: ["Haddumati", "Hadhdhunmathi", "Hadhunmathi", "Laamu", "Lâm"].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::MV,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.8974984), longitude: Some(73.22851190000002), max_latitude: Some(7.1063277), min_latitude: Some(6.8074339), max_longitude: Some(73.23589319999999), min_longitude: Some(72.8088748)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeAtoll,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "آتول ها ألف"), ("bn", "হ\u{9be} আলিফ এটল"), ("ccp", "𑄦\u{11133}𑄃 𑄃𑄣\u{11128}𑄛\u{11134}"), ("ceb", "Haa Alifu Atholhu"), ("cs", "Há Alif"), ("da", "Haa Alif Atoll"), ("de", "Haa Alif"), ("dv", "Thiladhunmathee Uthuruburi"), ("el", "Χαά Αλίφ Ατόλ"), ("en", "North Thiladhunmathi"), ("es", "Atolón Haa Alif"), ("fi", "Haa Alif Atoll"), ("fr", "Haa Alifu"), ("gu", "હા અલીફ એટોલ"), ("he", "אטול הא אליף"), ("hi", "हा अलिफ एटोल"), ("hu", "Északi-Tiladummati-atoll"), ("id", "Atol Haa Alif"), ("it", "Atollo Haa Alif"), ("ja", "ハーアリフ環礁"), ("kn", "ಹಾ ಅಲ\u{cbf}ಫ\u{ccd} ಅಟಾಲ\u{ccd}"), ("ko", "하알리프 환초"), ("lt", "Ha Atifo atolas"), ("lv", "Haa Alifas atols"), ("mr", "हा अल\u{94d}फ एटॉल"), ("ms", "Haa Alif Atoll"), ("nb", "Haa Alif Atoll"), ("nl", "Haa Alif-atol"), ("no", "Haa Alif Atoll"), ("pl", "Haa Alif"), ("pt", "Haa Alif Atoll"), ("ru", "Хаа-Алиф"), ("si", "හ\u{dcf} අල\u{dd2}ෆ\u{dca} ද\u{dd4}පත\u{dca} සම\u{dd4}හය"), ("sk", "Há Alif"), ("sv", "Haa Alifu Atholhu"), ("ta", "ஹ\u{bbe} அளிப\u{bcd} அட\u{bbe}ல\u{bcd}"), ("te", "హ\u{c3e} ఆల\u{c3f}ఫ\u{c4d} అట\u{c4b}ల\u{c4d}"), ("th", "ฮา อล\u{e34}ฟ อโทล"), ("tr", "Haa Alif Atoll"), ("uk", "Атол Хаа-Аліф"), ("ur", "ہاں الف اتول"), ("vi", "Haa Alif Atoll"), ("zh", "哈阿里夫環礁")]),
                        unofficial_name_list: ["Haa Alifu", "Hâ Alif", "North Thiladhunmathi", "Thiladhunmathi Uthuruburi", "Tiladummati-North"].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "08",
                        country_alpha2: Alpha2::MV,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(2.209209), longitude: Some(73.1494617), max_latitude: Some(2.5602977), min_latitude: Some(2.1634586), max_longitude: Some(73.3696136), min_longitude: Some(72.9074181)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeAtoll,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄗\u{11133}𑄃"), ("cs", "Tá"), ("dv", "Kolhumadulu"), ("en", "Kolhumadulu"), ("es", "Atolón Thaa"), ("fr", "Atoll Kolhumadulu"), ("hu", "Kolumadulu-atoll"), ("it", "Atollo Thaa"), ("ko", "타 환초"), ("nl", "Thaa-atol"), ("pl", "Thaa"), ("ru", "Тхаа"), ("sk", "Tá"), ("zh", "塔亞環礁")]),
                        unofficial_name_list: ["Kolhumadhulu", "Kolumadulu", "Thaa", "Thâ"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "12",
                        country_alpha2: Alpha2::MV,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(2.7854064), longitude: Some(73.42621419999999), max_latitude: Some(3.0929022), min_latitude: Some(2.7635105), max_longitude: Some(73.6417058), min_longitude: Some(73.3577339)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeAtoll,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ميمو أتول"), ("bn", "মেমো আটল"), ("ccp", "𑄟\u{11128}𑄟\u{1112a}"), ("ceb", "Meemu Atholhu"), ("da", "Meemu Atoll"), ("de", "Mulaku-Atoll"), ("dv", "Mulakatholhu"), ("el", "Μέμου Ατόλ"), ("en", "Mulaku Atoll"), ("es", "Atolón Meemu"), ("fi", "Meemu Atoll"), ("fr", "Atoll Mulaku"), ("gu", "અબખાઝિયા"), ("hi", "मीम\u{941} एटोल"), ("hu", "Mulaku-atoll"), ("id", "Atol Meemu"), ("it", "Atollo Meemu"), ("ja", "ミーム環礁"), ("kn", "ಮೀಮು ಅಟಾಲ\u{ccd}"), ("ko", "미무 환초"), ("lt", "Memu atolas"), ("lv", "Meemu atols"), ("mr", "म\u{947}म\u{942} एटोल"), ("ms", "Meemu Atoll"), ("nb", "Meemu Atoll"), ("nl", "Meemu-atol"), ("no", "Meemu Atoll"), ("pl", "Meemu"), ("pt", "Meemu Atoll"), ("ru", "Меему"), ("si", "ම\u{dd3}ම\u{dd4} ද\u{dd4}පත\u{dca} සම\u{dd4}හය"), ("sk", "Mímu"), ("sv", "Meemu Atholhu"), ("ta", "ம\u{bc0}மு அடோல\u{bcd}"), ("te", "మ\u{c40}ము ఆట\u{c4b}ల\u{c4d}"), ("th", "ม\u{e35}ม\u{e38} ม\u{e31}ลด\u{e35}ฟส\u{e4c}"), ("tr", "Meemu Atoll"), ("uk", "Атол Меему"), ("ur", "مییمو اتول"), ("vi", "Meemu Atoll"), ("zh", "美慕環礁")]),
                        unofficial_name_list: ["Meemu", "Mimu", "Mulakatholhu", "Mulaku", "Mulaku Atholhu", "Mîm"].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "13",
                        country_alpha2: Alpha2::MV,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(5.4633655), longitude: Some(73.03472719999999), max_latitude: Some(5.980543099999999), min_latitude: Some(5.3587017), max_longitude: Some(73.0479349), min_longitude: Some(72.8025126)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeAtoll,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "را أتول"), ("bn", "র\u{9be} এটল"), ("ccp", "𑄢\u{11133}𑄃"), ("ceb", "Raa Atholhu"), ("cs", "Rá"), ("da", "Raa Atoll"), ("de", "Raa"), ("dv", "Maalhosmadulu Uthuruburi"), ("el", "Ραά Ατόλ"), ("en", "North Maalhosmadulu"), ("es", "Atolón Raa"), ("fi", "Raa Atoll"), ("fr", "Raa"), ("gu", "રા એટોલ"), ("he", "אטול ראה"), ("hi", "रा एटोल"), ("hu", "Északi-Malosmadulu-atoll"), ("id", "Atol Raa"), ("it", "Atollo Raa"), ("ja", "ラー環礁"), ("kn", "ರಾ ಅಟಾಲ\u{ccd}"), ("ko", "라 환초"), ("lt", "Ra atolas"), ("lv", "Rā atols"), ("mr", "रा एटॉल\u{94d}स"), ("ms", "Raa Atoll"), ("nb", "Raa Atoll"), ("nl", "Raa-atol"), ("no", "Raa Atoll"), ("pl", "Raa"), ("pt", "Raa Atoll"), ("ru", "Раа"), ("si", "ර\u{dcf} ද\u{dd6}පත\u{dca} සම\u{dd6}හය"), ("sk", "Rá"), ("sv", "Raa Atholhu"), ("ta", "ர\u{bbe} அடொல\u{bcd}"), ("te", "ర\u{c3e} అట\u{c4b}ల\u{c4d}"), ("th", "รา อ\u{e38}ท\u{e38}ร\u{e38}"), ("tr", "Raa Atoll"), ("uk", "Атол Раа"), ("ur", "را اتول"), ("vi", "Raa Atoll"), ("zh", "拉環礁")]),
                        unofficial_name_list: ["Maalhosmadhulu Uthuruburi", "Malosmadulu-North", "North Maalhosmadhulu", "Raa", "Râ"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::MV,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(3.0558963), longitude: Some(72.8889301), max_latitude: Some(3.3167972), min_latitude: Some(3.0521718), max_longitude: Some(73.0419406), min_longitude: Some(72.8165155)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeAtoll,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "فافو أتول²"), ("bn", "ফ\u{9be}ফ\u{9c1} আতোল"), ("ccp", "𑄜𑄜\u{1112a}"), ("ceb", "Faafu Atholhu"), ("da", "Faafu Atoll"), ("de", "Nord-Nilandhe-Atoll"), ("dv", "Nilandheatholhu Uthuruburi"), ("el", "Φαάφου Ατόλ"), ("en", "North Nilandhe Atoll"), ("es", "Atolón Faafu"), ("fi", "Faafu Atoll"), ("fr", "Atoll Nilandhe Nord"), ("gu", "ફાફ\u{ac1} એટોલ"), ("hi", "फाफ\u{942} एटोल"), ("id", "Atol Faafu"), ("it", "Atollo Faafu"), ("ja", "北ニランデ環礁区"), ("kn", "ಫಾಫು ಅಟಾಲ\u{ccd}"), ("ko", "파푸 환초"), ("lt", "Faafu Atolas"), ("lv", "Faafu atols"), ("mr", "फ\u{945}फ\u{942} एटोल"), ("ms", "Kepulauan Faafu"), ("nb", "Faafu Atoll"), ("nl", "Faafu-atol"), ("no", "Faafu Atoll"), ("pl", "Faafu"), ("pt", "Faafu Atoll"), ("ru", "Фаафу"), ("si", "ෆ\u{dcf}ෆ\u{dd4} අටොල\u{dca}"), ("sk", "Fáfu"), ("sv", "Faafu Atholhu"), ("ta", "ப\u{bbe}பியூ அடோல\u{bcd}"), ("te", "ఫ\u{c3e}ఫు అట\u{c4b}ల\u{c4d}"), ("th", "ฟาฟ\u{e39} อาทอลล\u{e4c}"), ("tr", "Faafu Atoll"), ("uk", "Атол Фаафу"), ("ur", "فافو اتول"), ("vi", "Faafu Atoll"), ("zh", "法夫環礁")]),
                        unofficial_name_list: ["Faafu", "Faafu Atoll", "Fâf", "Nilandhe Atholhu Uthuruburi", "Nilandu-North", "North Nilandhe", "North Nilandhe Atoll"].to_vec(),
                    }
                ),
                (
                    "17",
                    Subdivision{
                        name: "17",
                        country_alpha2: Alpha2::MV,
                        code: "17",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(2.6965009), longitude: Some(72.8630855), max_latitude: Some(3.0002184), min_latitude: Some(2.6672333), max_longitude: Some(73.0348726), min_longitude: Some(72.8351612)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeAtoll,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "دالو أتول"), ("bn", "দধ\u{9be}ল\u{9c1} আতোল"), ("ccp", "𑄙𑄣\u{1112a}"), ("ceb", "Dhaalu Atholhu"), ("da", "Dhaalu Atoll"), ("de", "Süd-Nilandhe-Atoll"), ("dv", "Nilandheatholhu Dhekunuburi"), ("el", "Ντααλού Ατόλ"), ("en", "South Nilandhe Atoll"), ("es", "Atolón Dhaalu"), ("fi", "Dhaalu Atoll"), ("fr", "Atoll Nilandhe Sud"), ("gu", "દહાલ\u{ac1} એટોલ"), ("hi", "धाल\u{941} एटोल"), ("id", "Atol Dhaalu"), ("it", "Atollo Dhaalu"), ("ja", "ダァール環礁"), ("kn", "ಧಾಲು ಅಟಾಲ\u{ccd}"), ("ko", "달루 환초"), ("lt", "Dchalo atolas"), ("lv", "Dālu atols"), ("mr", "धाल\u{941} एटव\u{94d}हल"), ("ms", "Dhaalu Atoll"), ("nb", "Dhaalu Atoll"), ("nl", "Dhaalu-atol"), ("no", "Dhaalu Atoll"), ("pl", "Dhaalu"), ("pt", "Dhaalu Atoll"), ("ru", "Дхаалу"), ("si", "ඩ\u{dcf}ල\u{dd4} ද\u{dd6}පත\u{dca} සම\u{dd6}හය"), ("sk", "Dálu"), ("sv", "Dhaalu Atholhu"), ("ta", "த\u{bbe}ளு அட\u{bbe}ல\u{bcd}"), ("te", "ఢ\u{c3e}లు అట\u{c4b}ల\u{c4d}"), ("th", "ดาห\u{e4c}ล\u{e39}"), ("tr", "Dhaalu Atoll"), ("uk", "Атол Дхаалу"), ("ur", "دحاالو اتول"), ("vi", "Dhaalu Atoll"), ("zh", "達魯環礁")]),
                        unofficial_name_list: ["Dhaalu", "Dhâl", "Nilandhe Atholhu Dhekunuburi", "Nilandu-South", "South Nilandhe Atoll"].to_vec(),
                    }
                ),
                (
                    "20",
                    Subdivision{
                        name: "20",
                        country_alpha2: Alpha2::MV,
                        code: "20",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(4.8709599), longitude: Some(72.99982109999999), max_latitude: Some(5.3591829), min_latitude: Some(4.861320099999999), max_longitude: Some(73.169557), min_longitude: Some(72.8362762)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeAtoll,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "با اتول"), ("bn", "ব\u{9be}আ আতোল"), ("ccp", "𑄝\u{11133}𑄃"), ("cs", "Bá"), ("da", "Baa Atoll"), ("de", "Baa"), ("dv", "Maalhosmadulu Dhekunuburi"), ("el", "Μπάα Ατόλ"), ("en", "South Maalhosmadulu"), ("es", "Atolón Baa"), ("fi", "Baa Atoll"), ("fr", "Baa"), ("gu", "બા એટોલ"), ("he", "אטול באה"), ("hi", "बा एटोल"), ("hy", "Բաա ատոլ"), ("id", "Atol Baa"), ("it", "Atollo Baa"), ("ja", "バア環礁"), ("kn", "ಬಾ ಅಟಾಲ\u{ccd}"), ("ko", "바 환초"), ("lt", "Ba atolas"), ("lv", "Bā atols"), ("mr", "बा एटॉल"), ("ms", "Kepulauan Baa"), ("nb", "Baa Atoll"), ("nl", "Baa-atol"), ("no", "Baa Atoll"), ("pl", "Baa"), ("pt", "Baa Atoll"), ("ru", "Баа"), ("si", "බ\u{dcf} ද\u{dd4}පත\u{dca} සම\u{dd4}හය"), ("sk", "Bá"), ("sv", "Baa Atoll"), ("ta", "ப\u{bbe} அட\u{bbe}ல\u{bcd}"), ("te", "బ\u{c3e} అట\u{c4b}ల\u{c4d}"), ("th", "บา อะทอล"), ("tr", "Baa Atoll"), ("uk", "Атол Баа"), ("ur", "با اتول"), ("vi", "Baa Atoll"), ("zh", "芭環礁")]),
                        unofficial_name_list: ["Baa", "Bâ", "Maalhosmadhulu Dhekunuburi", "Malosmadulu-South", "South Maalhosmadhulu"].to_vec(),
                    }
                ),
                (
                    "23",
                    Subdivision{
                        name: "23",
                        country_alpha2: Alpha2::MV,
                        code: "23",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.751373699999999), longitude: Some(73.16608339999999), max_latitude: Some(6.7876622), min_latitude: Some(6.3155278), max_longitude: Some(73.1847811), min_longitude: Some(72.6385815)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeAtoll,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ها دالو أتول"), ("bn", "হ\u{9be} ধ\u{9be}ল\u{9c1} আতোল"), ("ccp", "𑄦\u{11133}𑄃 𑄙\u{11133}𑄃𑄣\u{1112a}"), ("ceb", "Haa Dhaalu Atholhu"), ("cs", "Há Dálu"), ("da", "Haa Dhaalu Atoll"), ("de", "Haa Dhaalu"), ("dv", "Thiladhunmathee Dhekunuburi"), ("el", "Χαά Νταάλου Ατόλ"), ("en", "South Thiladhunmathi"), ("es", "Atolón Haa Dhaalu"), ("fi", "Haa Dhaalu Atoll"), ("fr", "Haa Dhaalu"), ("gu", "હા ધાલ\u{ac1} એટોલ"), ("he", "אטול הא דאלו"), ("hi", "हा धाल\u{942} एटोल"), ("hu", "Déli-Tiladummati-atoll"), ("id", "Atol Haa Dhaalu"), ("it", "Atollo Haa Dhaalu"), ("ja", "南ティラドゥンマティー環礁区"), ("kn", "ಹಾ ಧಾಲು ಅಟಾಲ\u{ccd}"), ("ko", "하달루 환초"), ("lt", "Haa Daalu atolas"), ("lv", "Haa Daalu atols"), ("mr", "हा धायाल एटवला"), ("ms", "Haa Dhaalu Atoll"), ("nb", "Haa Dhaalu Atoll"), ("nl", "Haa Dhaalu-atol"), ("no", "Haa Dhaalu Atoll"), ("pl", "Haa Dhaalu"), ("pt", "Haa Dhaalu Atoll"), ("ru", "Хаа-Дхаалу"), ("si", "හ\u{dcf} ඩ\u{dcf}ල\u{dd4} ද\u{dd6}පත\u{dca} සම\u{dd6}හය"), ("sk", "Há Dálu"), ("sv", "Haa Dhaalu-atollen"), ("ta", "ஹ\u{bbe} த\u{bbe}ளு அட\u{bbe}ல\u{bcd}"), ("te", "హ\u{c3e} ఢ\u{c3e}లు అట\u{c4b}ల\u{c4d}"), ("th", "ฮาฮ\u{e4c} ดาฮ\u{e4c}ล\u{e39} อทอลล\u{e4c}"), ("tr", "Haa Dhaalu Atoll"), ("uk", "Атол Хаа-Дхаалу"), ("ur", "ہاں دحاالو اتول"), ("vi", "Haa Dhaalu Atoll"), ("zh", "阿魯環礁")]),
                        unofficial_name_list: ["Haa Dhaalu", "Hâ Dhâl", "South Thiladhunmathi", "Thiladhunmathi Dhekunuburi", "Tiladummati-South"].to_vec(),
                    }
                ),
                (
                    "24",
                    Subdivision{
                        name: "24",
                        country_alpha2: Alpha2::MV,
                        code: "24",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.2869456), longitude: Some(73.2445073), max_latitude: Some(6.452543700000001), min_latitude: Some(5.966799399999999), max_longitude: Some(73.3011246), min_longitude: Some(72.9068712)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeAtoll,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شافياني أتول"), ("bn", "স\u{9cd}য\u{9be}ভিয\u{9bc}\u{9be}নি আতোল"), ("ccp", "𑄥𑄞\u{11128}𑄠𑄚\u{11128}"), ("ceb", "Shaviyani Atholhu"), ("cs", "Šavijani"), ("da", "Shaviyani Atoll"), ("de", "Shaviyani"), ("dv", "Miladhunmadulu Uthuruburi"), ("el", "Σαβιγιάνι Ατόλ"), ("en", "North Miladhunmadulu"), ("es", "Atolón Shaviyani"), ("fi", "Shaviyani Atoll"), ("fr", "Shaviyani"), ("gu", "શાવિયાની એટોલ"), ("he", "אטול שאביאני"), ("hi", "शावियानी एटोल"), ("hu", "Északi-Miladummadulu-atoll"), ("id", "Atol Shaviyani"), ("it", "Atollo Shaviyani"), ("ja", "北ミラドゥンマドゥル環礁区"), ("kn", "ಶವ\u{cbf}ಯನ\u{cbf} ಅಟಾಲ\u{ccd}"), ("ko", "샤비야니 환초"), ("lt", "Šavjano atolas"), ("lv", "Šavijani atols"), ("mr", "श\u{94d}व\u{94d}याणी एटॉल"), ("ms", "Shaviyani Atoll"), ("nb", "Shaviyani Atoll"), ("nl", "Shaviyani-atol"), ("no", "Shaviyani Atoll"), ("pl", "Shaviyani"), ("pt", "Shaviyani Atoll"), ("ru", "Шавийани"), ("si", "ශව\u{dd2}ය\u{dcf}න\u{dd2} ද\u{dd4}පත\u{dca} සම\u{dd4}හය"), ("sk", "Šavijani"), ("sv", "Shaviyani Atholhu"), ("ta", "ஷவியணி அட\u{bbe}ல\u{bcd}"), ("te", "ష\u{c3e}వ\u{c3f}య\u{c3e}న\u{c3f} అట\u{c4b}ల\u{c4d}"), ("th", "เกาะชว\u{e34}ยน\u{e34}"), ("tr", "Shayviyani Atoll"), ("uk", "Атол Шавійані"), ("ur", "شاویانی اتول"), ("vi", "Shaviyani Atoll"), ("zh", "沙維亞尼環礁")]),
                        unofficial_name_list: ["Miladhunmadhulu Uthuruburi", "Miladummadulu-North", "North Miladhunmadhulu", "Shaviyani"].to_vec(),
                    }
                ),
                (
                    "25",
                    Subdivision{
                        name: "25",
                        country_alpha2: Alpha2::MV,
                        code: "25",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(5.9523854), longitude: Some(73.4151346), max_latitude: Some(6.0033134), min_latitude: Some(5.6440025), max_longitude: Some(73.4822166), min_longitude: Some(73.1310586)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeAtoll,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "نونو أتول"), ("bn", "ন\u{9c1}ন\u{9c1} এটল"), ("ccp", "𑄚\u{1112b}𑄚\u{1112a}"), ("ceb", "Noonu Atoll"), ("cs", "Núnu"), ("da", "Noonu Atoll"), ("de", "Noonu"), ("dv", "Miladhunmadulu Dhekunuburi"), ("el", "Νοόνου Ατόλ"), ("en", "South Miladhunmadulu"), ("es", "Atolón Noonu"), ("fi", "Noonu Atoll"), ("fr", "Noonu"), ("gu", "ન\u{ac2}ન\u{ac1} એટોલ"), ("he", "אטול נונו"), ("hi", "न\u{942}न\u{942} एटोल"), ("hu", "Déli-Miladummadulu-atoll"), ("id", "Atol Noonu"), ("it", "Atollo Noonu"), ("ja", "ヌーヌ環礁"), ("kn", "ನ\u{cc2}ನ\u{ccd} ಅಟಾಲ\u{ccd}"), ("ko", "누누 환초"), ("lt", "Nunu atolas"), ("lv", "Noonu atols"), ("mr", "नोओन\u{942} एटॉल"), ("ms", "Noonu Atoll"), ("nb", "Noonu Atoll"), ("nl", "Noonu-atol"), ("no", "Noonu Atoll"), ("pl", "Noonu"), ("pt", "Noonu Atoll"), ("ru", "Ноону"), ("si", "න\u{dd6}න\u{dd4} ද\u{dd4}පත\u{dca} සම\u{dd6}හය"), ("sk", "Núnu"), ("sv", "Noonu Atoll"), ("ta", "நூனு அட\u{bbe}ல\u{bcd}"), ("te", "నూను అట\u{c4b}ల\u{c4d}"), ("th", "น\u{e39}น\u{e38}"), ("tr", "Noonu Atoll"), ("uk", "Атол Ноону"), ("ur", "نونو اتول"), ("vi", "Noonu Atoll"), ("zh", "諾努環礁")]),
                        unofficial_name_list: ["Miladhunmadhulu Dhekunuburi", "Miladummadulu-South", "Mulakatholhu", "Noonu", "Nûn", "South Miladhunmadhulu"].to_vec(),
                    }
                ),
                (
                    "26",
                    Subdivision{
                        name: "26",
                        country_alpha2: Alpha2::MV,
                        code: "26",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(4.9570016), longitude: Some(73.4595787), max_latitude: Some(4.9728929), min_latitude: Some(3.8099492), max_longitude: Some(73.71927020000001), min_longitude: Some(73.3509493)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeAtoll,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كافو أتول"), ("az", "Kaafu atollu"), ("bn", "ক\u{9be}ফ\u{9c1} আটল"), ("ccp", "𑄇𑄜\u{1112a}"), ("ceb", "Kaafu Atoll"), ("cs", "Káfu"), ("da", "Kaafu Atoll"), ("de", "Kaafu"), ("dv", "Maaleatholhu"), ("el", "Καάφου Ατόλ"), ("en", "Male Atoll"), ("es", "Atolón Kaafu"), ("fi", "Kaafu Atoll"), ("fr", "Kaafu"), ("gu", "કાફ\u{ac1} એટોલ"), ("he", "אטול קאפו"), ("hi", "काफ\u{942} एटोल"), ("hu", "Male-atoll"), ("id", "Atol Kaafu"), ("it", "Atollo Kaafu"), ("ja", "カーフ環礁"), ("kn", "ಕಾಫು ಅಟಾಲ\u{ccd}"), ("ko", "카푸 환초"), ("lt", "Kaafo atolas"), ("lv", "Kāfu atols"), ("mr", "काफ\u{93c}\u{942} अटॉल"), ("ms", "Kaafu Atoll"), ("nb", "Kaafu Atoll"), ("nl", "Kaafu-atol"), ("no", "Kaafu Atoll"), ("pl", "Kaafu"), ("pt", "Kaafu Atoll"), ("ru", "Каафу"), ("si", "ක\u{dcf}ෆ\u{dd4} ද\u{dd4}පත\u{dca} සම\u{dd4}හය"), ("sk", "Káfu"), ("sv", "Kaafu Atoll"), ("ta", "க\u{bbe}ஃபு அடோல\u{bcd}"), ("te", "క\u{c3e}ఫు అట\u{c4b}ల\u{c4d}"), ("th", "หม\u{e39}\u{e48}เกาะคาฟ\u{e38}"), ("tr", "Kaafu Atoll"), ("uk", "Атол Каафу"), ("ur", "کافو اتول"), ("vi", "Kaafu Atoll"), ("zh", "卡夫環礁")]),
                        unofficial_name_list: ["Kaafu", "Kâf", "Maleʿ Atholhu", "Malé Atoll"].to_vec(),
                    }
                ),
                (
                    "27",
                    Subdivision{
                        name: "27",
                        country_alpha2: Alpha2::MV,
                        code: "27",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(0.7593605), longitude: Some(73.4330865), max_latitude: Some(0.8975593000000001), min_latitude: Some(0.3470336), max_longitude: Some(73.5735378), min_longitude: Some(73.092454)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeAtoll,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غافو أليف أتول"), ("bn", "গোফ\u{9c1} আলিফ আতোল"), ("ccp", "𑄉\u{11133}𑄃𑄜\u{1112a} 𑄃𑄣\u{11128}𑄛\u{11134}"), ("ceb", "Gaafu Alifu Atholhu"), ("cs", "Gáfu Alif"), ("da", "Gaafu Alif Atoll"), ("de", "Gaafu Alif"), ("dv", "Huvadhuatholhu Uthuruburi"), ("el", "Γκαάφου Αλίφ Ατόλ"), ("en", "North Huvadhu Atoll"), ("es", "Atolón Gaafu Alif"), ("fi", "Gaafu Alif Atoll"), ("fr", "Gaafu Alif"), ("gu", "ગાફ\u{ac1} અલિફ એટોલ"), ("hi", "गाफ\u{941} अलिफ एटोल"), ("id", "Atol Gaafu Alif"), ("it", "Atollo Gaafu Alif"), ("ja", "ガーフ・アリフ環礁"), ("kn", "ಗಾಫು ಅಲ\u{cbf}ಫ\u{ccd} ಅಟಾಲ\u{ccd}"), ("ko", "가푸알리프 환초"), ("lt", "Gaafu Alifo atolas"), ("lv", "Gaafu Alifas atols"), ("mr", "गाफ\u{942} अलिफ अटॉल"), ("ms", "Gaafu Alif Atoll"), ("nb", "Gaafu Alif Atoll"), ("nl", "Gaafu Alif-atol"), ("no", "Gaafu Alif Atoll"), ("pl", "Gaafu Alif"), ("pt", "Gaafu Alif Atoll"), ("ru", "Гаафу-Алиф"), ("si", "ග\u{dcf}ෆ\u{dd4} අල\u{dd2}ෆ\u{dca} ද\u{dd4}පත\u{dca} සම\u{dd6}හය"), ("sk", "Gáfu Alif"), ("sv", "Gaafu Alifu Atholhu"), ("ta", "க\u{bbe}பியூ அளிப\u{bcd} அட\u{bbe}ல\u{bcd}"), ("te", "గ\u{c3e}ఫు అల\u{c40}ఫ\u{c4d} అట\u{c4b}ల\u{c4d}"), ("th", "การ\u{e4c}ฟ\u{e39} อล\u{e34}ฟ อโทล"), ("tr", "Gaafu Alif Atoll"), ("uk", "Атол Гаафу-Аліф"), ("ur", "گافو الف اتول"), ("vi", "Gaafu Alif Atoll"), ("zh", "奧利弗蓋夫環礁")]),
                        unofficial_name_list: ["Gaaf Alif", "Gaafu Alifu", "Gâf Alif", "Huvadhu Atholhu Uthuruburi", "North Huvadhu Atoll", "Suvadiva-Huvadu-North"].to_vec(),
                    }
                ),
                (
                    "28",
                    Subdivision{
                        name: "28",
                        country_alpha2: Alpha2::MV,
                        code: "28",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(0.2751533), longitude: Some(73.435831), max_latitude: Some(0.5438472999999999), min_latitude: Some(0.1880678), max_longitude: Some(73.5213636), min_longitude: Some(72.987068)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeAtoll,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غافو دالو أتول"), ("bn", "গ\u{9be}ফ\u{9c1} ধ\u{9be}ল\u{9c1} আতোল"), ("ccp", "𑄉\u{11133}𑄃𑄜\u{1112a} 𑄙\u{11133}𑄃𑄣\u{1112a}"), ("ceb", "Gaafu Dhaalu Atholhu"), ("cs", "Gáfu Dálu"), ("da", "Gaafu Dhaalu Atoll ( South Huvadhoo)"), ("de", "Gaafu Dhaalu"), ("dv", "Huvadhuatholhu Dhekunuburi"), ("el", "Γκααφού Ντααλού Ατόλ"), ("en", "South Huvadhu Atoll"), ("es", "Atolón Gaafu Dhaalu"), ("fi", "Gaafu Dhaalu Atoll"), ("fr", "Gaafu Dhaalu"), ("gu", "ગાફ\u{ac1} ધાલ\u{ac1} એટોલ"), ("hi", "गाफ\u{942} धाल\u{942} अटोल"), ("id", "Atol Gaafu Dhaalu"), ("it", "Atollo Gaafu Dhaalu"), ("ja", "ガーフ・ダール環礁"), ("kn", "ಗಾಫು ಧಾಲು ಅಟಾಲ\u{ccd}"), ("ko", "가푸달루 환초"), ("lt", "Gaafo Daalo atolas"), ("lv", "Gaafu Dhaalu atols"), ("mr", "ग\u{947}फ\u{942} धाल\u{941} एटोल"), ("ms", "Kepulauan Gaafu Dhaalu"), ("nb", "Gaafu Dhaaky Atoll"), ("nl", "Gaafu Dhaalu-atol"), ("no", "Gaafu Dhaaky Atoll"), ("pl", "Gaafu Dhaalu"), ("pt", "Gaafu Dhaalu Atoll"), ("ru", "Гаафу-Дхаалу"), ("si", "ග\u{dcf}ෆ\u{dd4} ද\u{dcf}ල\u{dd4} ද\u{dd6}පත\u{dca} සම\u{dd6}හය"), ("sk", "Gáfu Dálu"), ("sv", "Gaafu Dhaalu Atholhu"), ("ta", "க\u{bbe}பியூ தய\u{bbe}ளு அடோல\u{bcd}"), ("te", "గ\u{c3e}ఫు ధ\u{c3e}లూ అట\u{c4b}ల\u{c4d}"), ("th", "กาฟ\u{e39} ดาล\u{e39} อตอลล\u{e4c}"), ("tr", "Gaafu Dhaalu Atoll"), ("uk", "Атол Гаафу-Дхаалу"), ("ur", "گافو دحاالو اتول"), ("vi", "Gaafu Dhaalu Atoll"), ("zh", "南蘇瓦迪瓦環礁")]),
                        unofficial_name_list: ["Gaafu Dhaalu", "Gâf Dhâl", "Huvadhu Atholhu Dhekunuburi", "South Huvadhu", "Suvadiva-Huvadu-South"].to_vec(),
                    }
                ),
                (
                    "29",
                    Subdivision{
                        name: "29",
                        country_alpha2: Alpha2::MV,
                        code: "29",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-0.3006425), longitude: Some(73.42391429999999), max_latitude: Some(-0.2771461), min_latitude: Some(-0.3118643), max_longitude: Some(73.44141479999999), min_longitude: Some(73.4100436)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeAtoll,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غنافياني أتول"), ("bn", "গ\u{9cd}ন\u{9be}ভিয\u{9bc}\u{9be}নি আতোল"), ("ccp", "𑄚𑄞\u{11128}𑄠𑄚\u{11128}"), ("ceb", "Gnaviyani Atholhu"), ("cs", "Gnavijani"), ("da", "Gnaviyani Atoll"), ("de", "Gnaviyani"), ("dv", "Fuvammulah"), ("el", "Γκναβιγιάνι Ατόλ"), ("en", "Fuvammulah"), ("es", "Atolón Gnaviyani"), ("fi", "Gnaviyani Atoll"), ("fr", "Atoll de Gnaviyani"), ("gu", "ગ\u{acd}નાવિયાની એટોલ"), ("hi", "नावियानी एटोल"), ("id", "Gnaviyani Atoll"), ("it", "Atollo Gnaviyani"), ("ja", "ニヤヴィヤニ環礁"), ("kn", "ಗ\u{ccd}ವಾನವ\u{cbf}ಣ\u{cbf} ಅಟಾಲ\u{ccd}"), ("ko", "그나비야니 환초"), ("lt", "Gnavijanio atolas"), ("lv", "Gnavijani atols"), ("mr", "ग\u{94d}यानियानी एटोल"), ("ms", "Gnaviyani Atoll"), ("nb", "Gnaviyani Atoll"), ("nl", "Gnaviyani-atol"), ("no", "Gnaviyani Atoll"), ("pl", "Gnaviyani"), ("pt", "Gnaviyani Atoll"), ("ru", "Гнавийани"), ("si", "ග\u{dca}න\u{dcf}ව\u{dd2}ය\u{dcf}න\u{dd4} ද\u{dd4}පත\u{dca} සම\u{dd4}හය"), ("sk", "Gnavijani"), ("sv", "Gnaviyani Atholhu"), ("ta", "குணவியணி அடோல\u{bcd}"), ("te", "గ\u{c4d}న\u{c3e}వ\u{c3f}య\u{c3e}న\u{c3f} అట\u{c4b}ల\u{c4d}"), ("th", "กนาวอยาน\u{e35}อทอลล\u{e4c}"), ("tr", "Gnaviyani Atoll"), ("uk", "Гнавійані атол"), ("ur", "جناویانی اتول"), ("vi", "Gnaviyani Atoll"), ("zh", "加納維雅尼環礁")]),
                        unofficial_name_list: ["Foah Mulah", "Foahmulah", "Foammulah", "Foamulah", "Fua Mulak", "Fuamulaku", "Fuvahmulah", "Gnaviyani", "Gnyaviani"].to_vec(),
                    }
                ),
                (
                    "MLE",
                    Subdivision{
                        name: "MLE",
                        country_alpha2: Alpha2::MV,
                        code: "MLE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(4.1754959), longitude: Some(73.5093474), max_latitude: Some(4.202356), min_latitude: Some(4.1694754), max_longitude: Some(73.5313615), min_longitude: Some(73.4821147)}),
                        comments: None,
                        subdivision_type: SubdivisionType::City,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Malé"), ("am", "ማሌ"), ("ar", "ماليه"), ("az", "Male"), ("be", "Горад Мале"), ("bg", "Мале"), ("bn", "ম\u{9be}লে"), ("bs", "Malé"), ("ca", "Malé"), ("ccp", "𑄟𑄣\u{11128}"), ("ceb", "Male"), ("cs", "Male"), ("cy", "Malé"), ("da", "Malé"), ("de", "Malé"), ("el", "Μαλέ"), ("en", "Malé"), ("es", "Malé"), ("et", "Male"), ("eu", "Malé"), ("fa", "ماله"), ("fi", "Malé"), ("fr", "Malé"), ("ga", "Malé"), ("gl", "Malé"), ("gu", "માલ\u{ac7}"), ("he", "מאלה"), ("hi", "माल\u{947}"), ("hr", "Malé"), ("hu", "Malé"), ("hy", "Մալե"), ("id", "Malé"), ("is", "Malé"), ("it", "Malé"), ("ja", "マレ"), ("ka", "მალე"), ("kk", "Мале"), ("kn", "ಮಾಲ\u{cc6}"), ("ko", "말레"), ("ky", "Мале"), ("lt", "Malė"), ("lv", "Male"), ("mk", "Мале"), ("ml", "മ\u{d3e}ലി"), ("mn", "Мале"), ("mr", "माल\u{947}"), ("ms", "Malé"), ("nb", "Malé"), ("ne", "माल\u{947}"), ("nl", "Malé"), ("no", "Malé"), ("or", "ମ\u{b3e}ଲେ"), ("pa", "ਮਾਲ\u{a47}"), ("pl", "Male"), ("ps", "ماله"), ("pt", "Malé"), ("ro", "Malé"), ("ru", "Мале"), ("si", "ම\u{dcf}ලේ"), ("sk", "Male"), ("sl", "Male"), ("sq", "Malé"), ("sr", "Мале"), ("sr_Latn", "Male"), ("sv", "Malé"), ("sw", "Malé"), ("ta", "ம\u{bbe}லே"), ("te", "మ\u{c3e}ల\u{c47}"), ("th", "มาเล"), ("tk", "Male"), ("tr", "Malé"), ("uk", "Мале"), ("ur", "مالے"), ("uz", "Male"), ("vi", "Malé"), ("yo", "Malé"), ("yo_BJ", "Malé"), ("yue", "馬累"), ("yue_Hans", "马累"), ("zh", "馬累")]),
                        unofficial_name_list: ["Maale", "Mâle"].to_vec(),
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
#[cfg(feature = "mv")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::MV,
        alpha3: Alpha3::MDV,
        address_format: None,
        continent: Continent::Asia,
        country_code: 960,
        currency_code: "MVR",
        gec: Some(GEC::MV),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("MDV"),
        iso_long_name: "The Republic of Maldives",
        iso_short_name: "Maldives",
        official_language_list: ["dv"].to_vec(),
        spoken_language_list: ["dv"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7].to_vec(),
        national_prefix: "None",
        nationality: Some("Maldivan"),
        number: "462",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Sunday,
        subregion: Some(SubRegion::SouthernAsia),
        un_locode: "MV",
        unofficial_name_list: [
            "Maldives",
            "Malediven",
            "Maldivas",
            "モルディブ",
            "Maldiven",
        ]
        .to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Maldives"),
            ("af", "Maledive"),
            ("ak", "Maldives"),
            ("am", "ማልዲቭስ"),
            ("an", "Maldives"),
            ("ar", "جزر المالديف"),
            ("as", "ম\u{9be}লডিভ\u{9cd}\u{200c}চ"),
            ("ay", "Maldives"),
            ("az", "Maldives"),
            ("ba", "Maldives"),
            ("be", "Мальдывы"),
            ("bg", "Малдиви"),
            ("bi", "Maldives"),
            ("bn", "ম\u{9be}লডিভস"),
            ("bn_IN", "ম\u{9be}লডিভস"),
            ("br", "Maldivez"),
            ("bs", "Maldivi"),
            ("ca", "Maldives"),
            ("ce", "Мальдиви"),
            ("ch", "Maldives"),
            ("cs", "Maledivy"),
            ("cv", "Мальдиви"),
            ("cy", "Maldives"),
            ("da", "Maldiverne"),
            ("de", "Malediven"),
            ("dv", "ދ\u{7a8}ވ\u{7ac}ހ\u{7a8}ރ\u{7a7}އ\u{7b0}ޖ\u{7ac}"),
            ("dz", "མ\u{f7a}ལ་ཌ\u{f72}བས\u{f72}།"),
            ("ee", "Maldives"),
            ("el", "Μαλδίβες"),
            ("en", "Maldives"),
            ("eo", "Maldivoj"),
            ("es", "Islas Maldivas"),
            ("et", "Maldiivid"),
            ("eu", "Maldivak"),
            ("fa", "مالدیو"),
            ("ff", "Maldives"),
            ("fi", "Malediivit"),
            ("fo", "Maldivuoyggjarnar"),
            ("fr", "Maldives"),
            ("fy", "Maldiven"),
            ("ga", "Oileáin Mhaildíve"),
            ("gl", "Maldivas"),
            ("gn", "Maldives"),
            ("gu", "માલ\u{acd}દીવ\u{acd}સ"),
            ("gv", "Ny Maldeevaghyn"),
            ("ha", "Maldives"),
            ("he", "האיים המלדיביים"),
            ("hi", "मालदीव"),
            ("hr", "Maldivi"),
            ("ht", "Maldiv"),
            ("hu", "Maldív-szigetek"),
            ("hy", "Մալդիվներ"),
            ("ia", "Maldivas"),
            ("id", "Maladewa"),
            ("io", "Maldivi"),
            ("is", "Maldíveyjar"),
            ("it", "Maldive"),
            ("iu", "Maldives"),
            ("ja", "モルディブ"),
            ("ka", "მალდივის კუნძულები"),
            ("ki", "Maldives"),
            ("kk", "Мальдив аралдары"),
            ("kl", "Maldives"),
            ("km", "ម\u{17c9}ាល\u{17cb}ឌ\u{17b8}វ"),
            ("kn", "ಮಾಲ\u{ccd}ಡೀವ\u{ccd}ಸ\u{ccd}"),
            ("ko", "몰디브"),
            ("ku", "Giravên Maldiv"),
            ("kv", "Мальдивъяс"),
            ("kw", "Maldivys"),
            ("ky", "Мальдивдер"),
            ("lo", "Maldives"),
            ("lt", "Maldyvai"),
            ("lv", "Maldīvija"),
            ("mi", "Maldives"),
            ("mk", "Малдиви"),
            ("ml", "മ\u{d3e}ല\u{d4d}\u{200d}ഡീവ\u{d4d}സ\u{d4d}"),
            ("mn", "Малдивс"),
            ("mr", "मालदिव\u{94d}हज\u{94d}"),
            ("ms", "Maldiv"),
            ("mt", "Maldivi"),
            (
                "my",
                "မော\u{103a}လဒ\u{102d}\u{102f}က\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Mardib"),
            ("nb", "Maldivene"),
            ("ne", "मालदिव\u{94d}स"),
            ("nl", "Maldiven"),
            ("nn", "Maldivane"),
            ("nv", "Naakaii Dootłʼizhí Bikéyah Yázhí"),
            ("oc", "Maldivas"),
            ("or", "ମ\u{b3e}ଲଦୀଭ"),
            ("pa", "ਮਾਲਦੀਵ"),
            ("pi", "मालदीव"),
            ("pl", "Malediwy"),
            ("ps", "مالديپ"),
            ("pt", "Maldivas"),
            ("pt_BR", "Maldivas"),
            ("ro", "Maldive"),
            ("ru", "Мальдивы"),
            ("rw", "Malidivezi"),
            ("sc", "Maldivas"),
            ("sd", "مالديپ"),
            ("si", "ම\u{dcf}ල ද\u{dd2}වය\u{dd2}න"),
            ("sk", "Maldivy"),
            ("sl", "Maldivi"),
            ("so", "Maaldiqeen"),
            ("sq", "Maldive"),
            ("sr", "Малдиви"),
            ("sv", "Maldiverna"),
            ("sw", "Maldives"),
            ("ta", "ம\u{bbe}லத\u{bc0}வுகள\u{bcd}"),
            ("te", "మ\u{c3e}ల\u{c4d}ద\u{c40}వ\u{c4d}స\u{c4d}"),
            ("tg", "Малдив"),
            ("th", "ม\u{e31}ลด\u{e35}ฟส\u{e4c}"),
            ("ti", "Maldives"),
            ("tk", "Maldiwa"),
            ("tl", "Maldives"),
            ("tr", "Maldivler"),
            ("tt", "Малдивлар"),
            ("ug", "مالدىۋې"),
            ("uk", "Мальдіви"),
            ("ur", "مالدیپ"),
            ("uz", "Maldivalar"),
            ("ve", "Maldives"),
            ("vi", "Mal-đi-vợx"),
            ("wa", "Maldives"),
            ("wo", "Maldiif"),
            ("xh", "Maldives"),
            ("yo", "Àwọn Maldive"),
            ("zh_CN", "马尔代夫"),
            ("zh_HK", "馬爾代夫"),
            ("zh_TW", "馬爾地夫"),
            ("zu", "Maldives"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
