// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The People's Republic of China

#[cfg(all(feature = "cn", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}} {{region_short}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::CN;
    pub const ALPHA3: Alpha3 = Alpha3::CHN;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 86;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::CNY;
    pub const GEC: Option<GEC> = Some(GEC::CH);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::CHN);
    pub const ISO_SHORT_NAME: &str = "China";
    pub const ISO_LONG_NAME: &str = "The People's Republic of China";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["zh"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["zh"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7, 8, 9, 10, 11];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Chinese");
    pub const NUMBER: &str = "156";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{6}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternAsia);
    pub const UN_LOCODE: &str = "CN";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["China", "Chine", "中国"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "China"),
        ("af", "Sjina"),
        ("ak", "China"),
        ("am", "ኂ፤ና"),
        ("an", "China"),
        ("ar", "الص\u{651}ين"),
        ("as", "চীন"),
        ("ay", "China"),
        ("az", "Çin"),
        ("ba", "China"),
        ("be", "Кітай"),
        ("bg", "Китай"),
        ("bi", "China"),
        ("bn", "চীন"),
        ("bn_IN", "চীন"),
        ("br", "Sina"),
        ("bs", "Kina"),
        ("ca", "Xina"),
        ("ce", "China"),
        ("ch", "China"),
        ("cs", "Čína"),
        ("cv", "China"),
        ("cy", "Tsieina"),
        ("da", "Kina"),
        ("de", "China"),
        ("dv", "China"),
        ("dz", "ཅ་ཡ\u{f7a}་ན།(ར\u{f92}\u{fb1}་ནག)"),
        ("ee", "China"),
        ("el", "Κίνα"),
        ("en", "China"),
        ("eo", "Ĉinio"),
        ("es", "China"),
        ("et", "Hiina"),
        ("eu", "Txina"),
        ("fa", "چین"),
        ("ff", "China"),
        ("fi", "Kiina"),
        ("fo", "Kina"),
        ("fr", "Chine"),
        ("fy", "China"),
        ("ga", "An tSín"),
        ("gl", "China"),
        ("gn", "China"),
        ("gu", "ચીન"),
        ("gv", "China"),
        ("ha", "China"),
        ("he", "סין"),
        ("hi", "चीन"),
        ("hr", "Kina"),
        ("ht", "Chin"),
        ("hu", "Kína"),
        ("hy", "Չինաստան"),
        ("ia", "China"),
        ("id", "Cina"),
        ("io", "China"),
        ("is", "Kína"),
        ("it", "Cina"),
        ("iu", "China"),
        ("ja", "中国"),
        ("ka", "ჩინეთი"),
        ("ki", "China"),
        ("kk", "Қытай"),
        ("kl", "China"),
        ("km", "ច\u{17b7}ន"),
        ("kn", "ಚೈನಾ"),
        ("ko", "중국"),
        ("ku", "Çîn"),
        ("kv", "China"),
        ("kw", "China"),
        ("ky", "Кытай"),
        ("lo", "China"),
        ("lt", "Kinija"),
        ("lv", "Ķīna"),
        ("mi", "Haina"),
        ("mk", "Кина"),
        ("ml", "ചൈന"),
        ("mn", "Хятад"),
        ("mr", "चीन"),
        ("ms", "China"),
        ("mt", "Ċina"),
        ("my", "China"),
        ("na", "China"),
        ("nb", "Kina"),
        ("ne", "चीन"),
        ("nl", "China"),
        ("nn", "Kina"),
        ("nv", "China"),
        ("oc", "China"),
        ("or", "ଚୀନ"),
        ("pa", "ਚੀਨ"),
        ("pi", "China"),
        ("pl", "Chiny"),
        ("ps", "چین"),
        ("pt", "China"),
        ("pt_BR", "China"),
        ("ro", "China"),
        ("ru", "Китай"),
        ("rw", "Ubushinwa"),
        ("sc", "Tzina"),
        ("sd", "China"),
        ("si", "ච\u{dd3}නය"),
        ("sk", "Čína"),
        ("sl", "Kitajska"),
        ("so", "Shiinaha"),
        ("sq", "Kinë"),
        ("sr", "Кина"),
        ("sv", "Kina"),
        ("sw", "China"),
        ("ta", "ச\u{bc0}ன\u{bbe}"),
        ("te", "చ\u{c48}న\u{c3e}"),
        ("tg", "Чин"),
        ("th", "จ\u{e35}น"),
        ("ti", "ቻይና"),
        ("tk", "China"),
        ("tl", "Tsina"),
        ("tr", "Çin"),
        ("tt", "Чин"),
        ("ug", "جۇڭگو"),
        ("uk", "Китай"),
        ("ur", "China"),
        ("uz", "China"),
        ("ve", "China"),
        ("vi", "Trung Quốc"),
        ("wa", "Chine"),
        ("wo", "Ciin"),
        ("xh", "Tshayina"),
        ("yo", "China"),
        ("zh_CN", "中国"),
        ("zh_HK", "中國"),
        ("zh_TW", "中國"),
        ("zu", "Isi-Shayina"),
    ];
    #[cfg(all(feature = "cn", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 35.86166;
        pub const LONGITUDE: f64 = 104.195397;
        pub const MAX_LATITUDE: f64 = 53.5609739;
        pub const MAX_LONGITUDE: f64 = 134.7754563;
        pub const MIN_LATITUDE: f64 = 17.9996;
        pub const MIN_LONGITUDE: f64 = 73.4994136;
        pub const NORTHEAST_LATITUDE: f64 = 53.5609739;
        pub const NORTHEAST_LONGITUDE: f64 = 134.7754563;
        pub const SOUTHWEST_LATITUDE: f64 = 17.9996;
        pub const SOUTHWEST_LONGITUDE: f64 = 73.4994136;
    }
}
#[cfg(all(feature = "cn", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 35.86166,
            longitude: 104.195397,
            max_latitude: 53.5609739,
            max_longitude: 134.7754563,
            min_latitude: 17.9996,
            min_longitude: 73.4994136,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 53.5609739,
                    longitude: 134.7754563,
                },
                southwest: CountryGeoBound {
                    latitude: 17.9996,
                    longitude: 73.4994136,
                },
            },
        }
    }
}

#[cfg(all(feature = "cn", feature = "subdivisions"))]
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
                    "AH",
                    Subdivision{
                        name: "AH",
                        country_alpha2: Alpha2::CN,
                        code: "AH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.861184), longitude: Some(117.284923), max_latitude: Some(34.6542338), min_latitude: Some(29.3930379), max_longitude: Some(119.6491442), min_longitude: Some(114.8827517)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Anhui"), ("ar", "آنهوي"), ("az", "Anhoy"), ("be", "Правінцыя Аньхой"), ("bg", "Анхуей"), ("bn", "আনহ\u{9c1}য\u{9bc}েই"), ("bs", "Anhui"), ("ca", "Anhui"), ("ceb", "Anhui Sheng"), ("cs", "An-chuej"), ("cy", "Anhui"), ("da", "Anhui"), ("de", "Anhui"), ("el", "Ανχουί"), ("en", "Anhui"), ("es", "Anhui"), ("et", "Anhui"), ("eu", "Anhui"), ("fa", "آن\u{200c}هوئی"), ("fi", "Anhui"), ("fr", "Anhui"), ("ga", "Anhui"), ("gl", "Anhui"), ("gu", "અનહ\u{ac1}ઇ"), ("he", "אנחווי"), ("hi", "अनह\u{941}इ"), ("hr", "Anhui"), ("hu", "Anhuj"), ("hy", "Անհոյ"), ("id", "Anhui"), ("it", "Anhui"), ("ja", "安徽省"), ("ka", "ანხოი"), ("kk", "Аньхой"), ("kn", "ಅನ\u{ccd}ಹುಯ\u{cbf}"), ("ko", "안후이 성"), ("lt", "Anhui"), ("lv", "Aņhui"), ("mk", "Анхуеј"), ("ml", "ആൻഹ\u{d41}യി"), ("mn", "Аньхуй муж"), ("mr", "आ\u{902}ह\u{94d}वी"), ("ms", "Anhui"), ("my", "အန\u{103a}းဟ\u{103d}ေးပြည\u{103a}နယ\u{103a}"), ("nb", "Anhui"), ("nl", "Anhui"), ("no", "Anhui"), ("pa", "ਅਨਹ\u{a41}ਈ"), ("pl", "Anhui"), ("pt", "Anhui"), ("ro", "Anhui"), ("ru", "Аньхой"), ("si", "අන\u{dca}හ\u{dd4}ය\u{dd2}"), ("sk", "An-chuej"), ("sr", "Анхуеј"), ("sr_Latn", "Anhuej"), ("sv", "Anhui"), ("sw", "Anhui"), ("ta", "அன\u{bcd}ஹுயி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అన\u{c4d}హూయ\u{c3f}"), ("th", "มณฑลอานฮ\u{e38}ย"), ("tr", "Anhui"), ("uk", "Аньхой"), ("ur", "انہوئی"), ("uz", "Anxoy"), ("vi", "An Huy"), ("yue", "安徽"), ("yue_Hans", "安徽"), ("zh", "安徽省")]),
                        unofficial_name_list: ["Anhui"].to_vec(),
                    }
                ),
                (
                    "BJ",
                    Subdivision{
                        name: "BJ",
                        country_alpha2: Alpha2::CN,
                        code: "BJ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.904211), longitude: Some(116.407395), max_latitude: Some(41.0608158), min_latitude: Some(39.4427581), max_longitude: Some(117.5146251), min_longitude: Some(115.4234115)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Beijing"), ("am", "ቤዪጂንግ"), ("ar", "بكين"), ("as", "বেইজিং"), ("az", "Pekin"), ("be", "Пекін"), ("bg", "Пекин"), ("bn", "বেইজিং"), ("bs", "Peking"), ("ca", "Pequín"), ("ceb", "Beijing"), ("cs", "Peking"), ("cy", "Beijing"), ("da", "Beijing"), ("de", "Peking"), ("el", "Πεκίνο"), ("en", "Beijing"), ("es", "Pekín"), ("et", "Peking"), ("eu", "Pekin"), ("fa", "پکن"), ("fi", "Peking"), ("fr", "Pékin"), ("ga", "Béising"), ("gl", "Pequín"), ("gu", "બ\u{ac7}ઇજિ\u{a82}ગ"), ("ha", "Beijing"), ("ha_NE", "Beijing"), ("he", "בייג׳ינג"), ("hi", "बीजि\u{902}ग"), ("hr", "Peking"), ("hu", "Peking"), ("hy", "Պեկին"), ("id", "Beijing"), ("is", "Beijing"), ("it", "Pechino"), ("ja", "北京市"), ("jv", "Beijing"), ("ka", "პეკინი"), ("kk", "Бейжің"), ("km", "ប\u{17c9}េកា\u{17c6}ង"), ("kn", "ಬೀಜ\u{cbf}ಂಗ\u{ccd}"), ("ko", "베이징 시"), ("ky", "Бейжин"), ("lo", "ປ\u{eb1}ກກ\u{eb4}\u{ec8}ງ"), ("lt", "Pekinas"), ("lv", "Pekina"), ("mk", "Пекинг"), ("ml", "ബെയ\u{d4d}\u{200c}ജിങ\u{d4d}ങ\u{d4d}\u{200c}"), ("mn", "Бээжин"), ("mr", "बीजि\u{902}ग"), ("ms", "Beijing"), ("my", "ပေကျင\u{103a}းမြ\u{102d}\u{102f}\u{1037}"), ("nb", "Beijing"), ("ne", "ब\u{947}इजिङ"), ("nl", "Peking"), ("no", "Beijing"), ("or", "ବେଜ\u{b3f}ଂ"), ("pa", "ਬੀਜਿ\u{a70}ਗ"), ("pl", "Pekin"), ("ps", "بېجنګ"), ("pt", "Pequim"), ("ro", "Beijing"), ("ru", "Пекин"), ("si", "බෙය\u{dd2}ජ\u{dd2}ං"), ("sk", "Peking"), ("sl", "Peking"), ("so", "Beijing"), ("sq", "Pekini"), ("sr", "Пекинг"), ("sr_Latn", "Peking"), ("sv", "Peking"), ("sw", "Beijing"), ("ta", "பெய\u{bcd}ஜிங\u{bcd}"), ("te", "బ\u{c40}జ\u{c3f}ంగ\u{c4d}"), ("th", "ป\u{e31}กก\u{e34}\u{e48}ง"), ("tk", "Pekin"), ("tr", "Pekin"), ("uk", "Пекін"), ("ur", "بیجنگ"), ("uz", "Pekin"), ("vi", "Bắc Kinh"), ("yo", "Beijing"), ("yo_BJ", "Beijing"), ("yue", "北京"), ("yue_Hans", "北京"), ("zh", "北京市"), ("zu", "Beijing")]),
                        unofficial_name_list: ["Beijing", "Pekín"].to_vec(),
                    }
                ),
                (
                    "CQ",
                    Subdivision{
                        name: "CQ",
                        country_alpha2: Alpha2::CN,
                        code: "CQ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.56301), longitude: Some(106.551557), max_latitude: Some(32.2011871), min_latitude: Some(28.1602253), max_longitude: Some(110.1998582), min_longitude: Some(105.2897606)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tsjongtsjing"), ("am", "ቾንግጪንግ"), ("ar", "تشونغتشينغ"), ("az", "Çuntsin"), ("be", "Горад Чунцын"), ("bg", "Чунцин"), ("bn", "ছোংছিং"), ("bs", "Chongqing"), ("ca", "Txungking"), ("ceb", "Chongqing Shi"), ("cs", "Čchung-čching"), ("cy", "Chongqing"), ("da", "Chongqing"), ("de", "Chongqing"), ("el", "Τσονγκίνγκ"), ("en", "Chongqing"), ("es", "Chongqing"), ("et", "Chongqing Shi"), ("eu", "Chongqing"), ("fa", "چونگ\u{200c}کینگ"), ("fi", "Chongqing"), ("fr", "Chongqing"), ("ga", "Chongqing"), ("gl", "Chongqing"), ("gu", "ચૉ\u{a82}ગકિ\u{a82}ગ"), ("ha", "Chongqing"), ("ha_NE", "Chongqing"), ("he", "צ׳ונגצ׳ינג"), ("hi", "चो\u{902}ग\u{94d}कि\u{902}ग"), ("hr", "Chongqing"), ("hu", "Csungking"), ("id", "Chongqing"), ("is", "Chongqing"), ("it", "Chongqing"), ("ja", "重慶市"), ("ka", "ჩუნცინი"), ("kn", "ಚಾಂಗ\u{ccd}\u{200d}ಕ\u{cbf}ಂಗ\u{ccd}"), ("ko", "충칭 시"), ("lt", "Čongčingas"), ("lv", "Čuncjina"), ("mk", "Чунгќинг"), ("ml", "ചോങ\u{d4d}ചിങ\u{d4d}"), ("mn", "Чунцин"), ("mr", "चो\u{902}गछि\u{902}ग"), ("ms", "Chongqing"), ("my", "ချ\u{102f}\u{1036}ချင\u{1037}\u{103a}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Chongqing"), ("nl", "Chongqing"), ("no", "Chongqing"), ("pa", "ਚ\u{a4c}\u{a02}ਗਕਿ\u{a70}ਗ"), ("pl", "Chongqing"), ("pt", "Chongqing"), ("ro", "Chongqing"), ("ru", "Чунцин"), ("si", "චොන\u{dca}ක\u{dd2}න\u{dca}"), ("sk", "Čchung-čching"), ("sl", "Chongqing"), ("sq", "Çongqing"), ("sr", "Чунгкинг"), ("sr_Latn", "Čungking"), ("sv", "Chongqing"), ("ta", "சோங\u{bcd}கிங\u{bcd}"), ("te", "చ\u{c3e}ంగ\u{c4d} క\u{c3f}ంగ\u{c4d}"), ("th", "ฉงช\u{e34}\u{e48}ง"), ("tk", "Çunszin"), ("tr", "Çongçing"), ("uk", "Чунцін"), ("ur", "چونگ کینگ"), ("uz", "Chunsin"), ("vi", "Trùng Khánh"), ("yue", "重慶"), ("yue_Hans", "重庆"), ("zh", "重庆市")]),
                        unofficial_name_list: ["Chongqing"].to_vec(),
                    }
                ),
                (
                    "FJ",
                    Subdivision{
                        name: "FJ",
                        country_alpha2: Alpha2::CN,
                        code: "FJ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(26.10078), longitude: Some(119.295144), max_latitude: Some(28.3129013), min_latitude: Some(23.528783), max_longitude: Some(120.7270943), min_longitude: Some(115.8522901)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Foekien"), ("ar", "فوجيان"), ("az", "Fucyen"), ("be", "Правінцыя Фуцзянь"), ("bg", "Фудзиен"), ("bn", "ফ\u{9c1}চিয\u{9bc}েন"), ("bs", "Fujian"), ("ca", "Fujian"), ("ceb", "Fujian Sheng"), ("cs", "Fu-ťien"), ("cy", "Fujian"), ("da", "Fujian"), ("de", "Fujian"), ("el", "Φουτσιάν"), ("en", "Fujian"), ("es", "Fujian"), ("et", "Fujian"), ("eu", "Fujian"), ("fa", "فوجیان"), ("fi", "Fujian"), ("fr", "Fujian"), ("ga", "Fujian"), ("he", "פוג׳יין"), ("hi", "फ\u{93c}\u{942}ज\u{94d}यान"), ("hr", "Fujian"), ("hu", "Fucsien"), ("hy", "Ֆուցզյան"), ("id", "Fujian"), ("it", "Fujian"), ("ja", "福建省"), ("jv", "Fujian"), ("ka", "ფუძიანი"), ("ko", "푸젠 성"), ("lt", "Fudzianas"), ("lv", "Fudzjaņa"), ("mk", "Фуѓен"), ("mn", "Фужянь муж"), ("mr", "फ\u{942}च\u{94d}यान"), ("ms", "Fujian"), ("my", "ဖ\u{1030}ကျန\u{1037}\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "Fujian"), ("nl", "Fujian"), ("no", "Fujian"), ("pa", "ਫ\u{a42}ਜਾਨ"), ("pl", "Fujian"), ("pt", "Fujian"), ("ro", "Fujian"), ("ru", "Фуцзянь"), ("sr", "Фуђен"), ("sr_Latn", "Fuđen"), ("sv", "Fujian"), ("sw", "Fujian"), ("ta", "புஜிய\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("th", "มณฑลฝ\u{e39}เจ\u{e35}\u{e49}ยน"), ("tr", "Fujian"), ("uk", "Фуцзянь"), ("ur", "فوجیان"), ("uz", "Futszyan"), ("vi", "Phúc Kiến"), ("yue", "福建"), ("yue_Hans", "福建"), ("zh", "福建省")]),
                        unofficial_name_list: ["Fujian"].to_vec(),
                    }
                ),
                (
                    "GD",
                    Subdivision{
                        name: "GD",
                        country_alpha2: Alpha2::CN,
                        code: "GD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(23.132191), longitude: Some(113.266531), max_latitude: Some(25.5167714), min_latitude: Some(20.2209735), max_longitude: Some(117.3185073), min_longitude: Some(109.668994)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kwangdong"), ("ar", "غوانغدونغ"), ("be", "Правінцыя Гуандун"), ("bg", "Гуандун"), ("bn", "ক\u{9c1}য\u{9bc}\u{9be}ংত\u{9c1}ঙ"), ("bs", "Guangdong"), ("ca", "Guangdong"), ("ceb", "Guangdong Sheng"), ("cs", "Kuang-tung"), ("cy", "Guangdong"), ("da", "Guangdong"), ("de", "Guangdong"), ("el", "Κουανγκτόνγκ"), ("en", "Guangdong"), ("es", "Provincia de Cantón"), ("et", "Guangdong"), ("eu", "Guangdong"), ("fa", "گوانگ\u{200c}دونگ"), ("fi", "Guangdong"), ("fr", "Guangdong"), ("ga", "Guangdong"), ("gu", "ગ\u{ac1}આ\u{a82}ગડો\u{a82}ગ"), ("he", "גואנגדונג"), ("hi", "ग\u{941}आ\u{902}गदो\u{902}ग"), ("hr", "Guangdong"), ("hu", "Kuangtung"), ("hy", "Գուանդուն"), ("id", "Guangdong"), ("is", "Guangdong"), ("it", "Guangdong"), ("ja", "広東省"), ("ka", "გუანდუნი"), ("kk", "Гуандун"), ("kn", "ಗುವಾಂಗ\u{ccd}ಡಾಂಗ\u{ccd}"), ("ko", "광둥 성"), ("lt", "Guangdongas"), ("lv", "Guanduna"), ("mk", "Гуангдунг"), ("ml", "ഗ\u{d4d}വ\u{d3e}ങ\u{d4d}\u{200c}ഡോങ\u{d4d}"), ("mn", "Гуандун муж"), ("mr", "क\u{94d}वा\u{902}गतो\u{902}ग"), ("ms", "Guangdong"), ("my", "က\u{103d}မ\u{103a}တ\u{102f}န\u{103a}းပြည\u{103a}နယ\u{103a}"), ("nb", "Guangdong"), ("nl", "Guangdong"), ("no", "Guangdong"), ("pa", "ਗ\u{a41}ਆ\u{a02}ਗਦ\u{a4b}\u{a02}ਗ"), ("pl", "Guangdong"), ("pt", "Guangdong"), ("ro", "Guangdong"), ("ru", "Гуандун"), ("si", "ග\u{dd4}වන\u{dca}ග\u{dca}ඩොන\u{dca}ග\u{dca}"), ("sl", "Guangdong"), ("sr", "Гуангдунг"), ("sr_Latn", "Guangdung"), ("sv", "Guangdong"), ("sw", "Guangdong"), ("ta", "குவ\u{bbe}ங\u{bcd}ட\u{bbe}ங\u{bcd}"), ("te", "గ\u{c4d}వ\u{c3e}ంగ\u{c4d}\u{200c}డ\u{c3e}ంగ\u{c4d}"), ("th", "มณฑลกวางต\u{e38}\u{e49}ง"), ("tr", "Guangdong"), ("uk", "Гуандун"), ("ur", "گوانگڈونگ"), ("uz", "Guangun"), ("vi", "Quảng Đông"), ("yue", "廣東"), ("yue_Hans", "广东"), ("zh", "广东省")]),
                        unofficial_name_list: ["Guangdong"].to_vec(),
                    }
                ),
                (
                    "GS",
                    Subdivision{
                        name: "GS",
                        country_alpha2: Alpha2::CN,
                        code: "GS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.059561), longitude: Some(103.826447), max_latitude: Some(42.7951629), min_latitude: Some(32.5941106), max_longitude: Some(108.7123363), min_longitude: Some(92.33914659999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Gansu"), ("ar", "قانسو"), ("az", "Qansu"), ("be", "Правінцыя Ганьсу"), ("bg", "Гансу"), ("bn", "ক\u{9be}নস\u{9c1}"), ("bs", "Gansu"), ("ca", "Gansu"), ("ceb", "Gansu Sheng"), ("cs", "Kan-su"), ("cy", "Gansu"), ("da", "Gansu"), ("de", "Gansu"), ("el", "Γκανσού"), ("en", "Gansu"), ("es", "Gansu"), ("et", "Gansu"), ("eu", "Gansu"), ("fa", "گانسو"), ("fi", "Gansu"), ("fr", "Gansu"), ("ga", "Gansu"), ("gu", "ગન\u{acd}સ\u{ac1}"), ("he", "גאנסו"), ("hi", "गा\u{902}स\u{942}"), ("hr", "Gansu"), ("hu", "Kanszu"), ("id", "Gansu"), ("it", "Gansu"), ("ja", "甘粛省"), ("ka", "განსუ"), ("kk", "Ганьсу"), ("kn", "ಗ\u{ccd}ಯಾನ\u{ccd}ಸು"), ("ko", "간쑤 성"), ("lt", "Gansu"), ("lv", "Gaņsu"), ("mk", "Гансу"), ("mn", "Ганьсү"), ("mr", "कान\u{94d}स\u{942}"), ("ms", "Gansu"), ("my", "ကန\u{103a}းစ\u{102f}ပြည\u{103a}နယ\u{103a}"), ("nb", "Gansu"), ("nl", "Gansu"), ("no", "Gansu"), ("pl", "Gansu"), ("pt", "Gansu"), ("ro", "Gansu"), ("ru", "Ганьсу"), ("si", "ගන\u{dca}ස\u{dd6}"), ("sl", "Gansu"), ("sr", "Гансу"), ("sr_Latn", "Gansu"), ("sv", "Gansu"), ("sw", "Gansu"), ("ta", "க\u{bbe}ன\u{bcd}சு"), ("te", "గ\u{c3e}న\u{c4d}సు"), ("th", "มณฑลกานซ\u{e39}\u{e48}"), ("tr", "Kansu"), ("uk", "Ґаньсу"), ("ur", "گانسو"), ("uz", "Gansu"), ("vi", "Cam Túc"), ("yue", "甘肅"), ("yue_Hans", "甘肃"), ("zh", "甘肃省")]),
                        unofficial_name_list: ["Gansu"].to_vec(),
                    }
                ),
                (
                    "GX",
                    Subdivision{
                        name: "GX",
                        country_alpha2: Alpha2::CN,
                        code: "GX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.815478), longitude: Some(108.327546), max_latitude: Some(26.3855658), min_latitude: Some(20.8998362), max_longitude: Some(112.0618507), min_longitude: Some(104.4501321)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Guangxi"), ("ar", "قوانغشي"), ("az", "Quansi-Çjuan muxtar rayonu"), ("be", "Гуансі-Чжуанскі аўтаномны раён"), ("bg", "Гуанси-джуански автономен регион"), ("bn", "ক\u{9c1}য\u{9bc}\u{9be}ংশি"), ("bs", "Guangxi"), ("ca", "Guangxi"), ("ceb", "Guangxi Zhuangzu Zizhiqu"), ("cs", "Kuang-si"), ("cy", "Guangxi"), ("da", "Guangxi"), ("de", "Guangxi"), ("el", "Κουανγκσί"), ("en", "Guangxi"), ("es", "Guangxi"), ("et", "Guangxi"), ("eu", "Guangxi"), ("fa", "گوانگشی"), ("fi", "Guangxi"), ("fr", "Guangxi"), ("ga", "Guangxi"), ("gl", "Guangxi"), ("gu", "ગ\u{ac1}આન\u{acd}ક\u{acd}સી ઝ\u{ac1}આ\u{a82}ગ ઓટોનોમસ પ\u{acd}રદ\u{ac7}શ"), ("he", "גואנגשי"), ("hi", "ग\u{941}आ\u{902}गशी"), ("hr", "Guangxi"), ("hu", "Kuanghszi-Csuang Autonóm Terület"), ("hy", "Գուանսի-Չժոունի ինքնավար շրջան"), ("id", "Guangxi"), ("it", "Guangxi"), ("ja", "広西チワン族自治区"), ("ka", "გუანსი-ჯუანი"), ("kn", "ಗುವಾಂಗ\u{ccd}ಕ\u{ccd}ಸ\u{cbf} ಝುವಾಂಗ\u{ccd} ಸ\u{ccd}ವಾಯತ\u{ccd}ತ ಪ\u{ccd}ರದೇಶ"), ("ko", "광시 좡족 자치구"), ("lt", "Guangsi"), ("lv", "Guansji Džuanu autonomais reģions"), ("mk", "Гуангси"), ("ml", "ഗ\u{d41}വ\u{d3e}ങ\u{d4d}ക\u{d4d}സി"), ("mr", "क\u{94d}वा\u{902}ग\u{94d}शी"), ("ms", "Guangxi"), ("nb", "Guangxi"), ("nl", "Guangxi"), ("no", "Guangxi"), ("pl", "Kuangsi"), ("pt", "Guangxi"), ("ro", "Guangxi"), ("ru", "Гуанси-Чжуанский автономный район"), ("si", "ග\u{dd4}අන\u{dca}ක\u{dca}ස\u{dd2} ස\u{dd4}ආන\u{dca}ග\u{dca} ස\u{dca}ව\u{dcf}ධ\u{dd3}න කල\u{dcf}පය"), ("sr", "Гуангси"), ("sr_Latn", "Guangsi"), ("sv", "Guangxi"), ("sw", "Guangxi"), ("ta", "குவ\u{bbe}ங\u{bcd}ஷி"), ("te", "గువ\u{c3e}ంగ\u{c4d}జ\u{c40} జువ\u{c3e}ంగ\u{c4d} అట\u{c3e}నమస\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตปกครองตนเองกว\u{e48}างซ\u{e35}จ\u{e49}วง"), ("tr", "Guangxi Zhuang Özerk Bölgesi"), ("uk", "Гуансі-Чжуанський автономний район"), ("ur", "گوانگشی"), ("uz", "Guansi-chjuan muxtor rayoni"), ("vi", "Quảng Tây"), ("yue", "廣西"), ("yue_Hans", "广西"), ("zh", "广西壮族自治区")]),
                        unofficial_name_list: ["Guangxi Zhuang"].to_vec(),
                    }
                ),
                (
                    "GZ",
                    Subdivision{
                        name: "GZ",
                        country_alpha2: Alpha2::CN,
                        code: "GZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(26.598194), longitude: Some(106.70741), max_latitude: Some(29.2212757), min_latitude: Some(24.6199195), max_longitude: Some(109.5977559), min_longitude: Some(103.6014388)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Guizhou"), ("ar", "قويتشو"), ("be", "Правінцыя Гуйчжоу"), ("bg", "Гуейджоу"), ("bn", "ক\u{9c1}য\u{9bc}েইচৌ"), ("ca", "Guizhou"), ("ceb", "Guizhou Sheng"), ("cs", "Kuej-čou"), ("cy", "Guizhou"), ("da", "Guizhou"), ("de", "Guizhou"), ("el", "Κουιτσού"), ("en", "Guizhou"), ("es", "Guizhou"), ("et", "Guizhou"), ("eu", "Guizhou"), ("fa", "گوئیژو"), ("fi", "Guizhou"), ("fr", "Guizhou"), ("ga", "Guizhou"), ("gu", "ગ\u{ac1}ઇઝોઉ"), ("he", "גוויג׳ואו"), ("hi", "ग\u{941}इझोऊ"), ("hr", "Guizhou"), ("hu", "Kujcsou"), ("hy", "Գույչժոու"), ("id", "Guizhou"), ("it", "Guizhou"), ("ja", "貴州省"), ("ka", "გუიჯოუ"), ("kn", "ಗುಯ\u{cbf}ಝ\u{ccc}"), ("ko", "구이저우 성"), ("lt", "Guidžou"), ("lv", "Guidžou"), ("mk", "Гуејџоу"), ("mn", "Гуйжоу муж"), ("mr", "क\u{94d}वीचौ"), ("ms", "Guizhou"), ("nb", "Guizhou"), ("nl", "Guizhou"), ("no", "Guizhou"), ("pl", "Kuejczou"), ("pt", "Guizhou"), ("ro", "Guizhou"), ("ru", "Гуйчжоу"), ("si", "ග\u{dd4}ය\u{dd2}ෂ\u{dd4}"), ("sr", "Гуејџоу"), ("sr_Latn", "Guejdžou"), ("sv", "Guizhou"), ("sw", "Guizhou"), ("ta", "குயிசூ"), ("te", "గుయ\u{c3f}జ\u{c4b}"), ("th", "มณฑลก\u{e38}\u{e49}ยโจว"), ("tr", "Guizhou"), ("uk", "Ґуйчжоу"), ("ur", "گوئیژو"), ("uz", "Guychjou"), ("vi", "Quý Châu"), ("yue", "貴州"), ("yue_Hans", "贵州"), ("zh", "贵州省")]),
                        unofficial_name_list: ["Guizhou"].to_vec(),
                    }
                ),
                (
                    "HA",
                    Subdivision{
                        name: "HA",
                        country_alpha2: Alpha2::CN,
                        code: "HA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.765515), longitude: Some(113.753602), max_latitude: Some(36.3665602), min_latitude: Some(31.382371), max_longitude: Some(116.6522322), min_longitude: Some(110.3604761)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Henan"), ("ar", "خنان"), ("be", "Правінцыя Хэнань"), ("bg", "Хънан"), ("bn", "হ\u{9cd}যন\u{9be}ন"), ("ca", "Henan"), ("ceb", "Henan Sheng"), ("cs", "Che-nan"), ("cy", "Henan"), ("da", "Henan"), ("de", "Henan"), ("el", "Χενάν"), ("en", "Henan"), ("es", "Henan"), ("et", "Henan"), ("eu", "Henan"), ("fa", "استان هنان"), ("fi", "He’nan"), ("fr", "Henan"), ("ga", "Henan"), ("gl", "Henan"), ("gu", "હ\u{ac7}નન"), ("he", "חנאן"), ("hi", "ह\u{947}नान"), ("hr", "Henan"), ("hu", "Honan"), ("hy", "Հենան"), ("id", "Henan"), ("it", "Henan"), ("ja", "河南省"), ("ka", "ხენანი"), ("kk", "Хэнань"), ("kn", "ಹ\u{cc6}ನಾನ\u{ccd}"), ("ko", "허난 성"), ("lt", "Henanas"), ("lv", "Henaņa"), ("mk", "Хенан"), ("ml", "ഹെന\u{d3e}ൻ"), ("mn", "Хэнань"), ("mr", "हनान"), ("ms", "Henan"), ("nb", "Henan"), ("nl", "Henan"), ("no", "Henan"), ("pa", "ਹ\u{a47}ਨਾਨ"), ("pl", "Henan"), ("pt", "Henan"), ("ro", "Henan"), ("ru", "Хэнань"), ("si", "හෙනන\u{dca}"), ("sl", "Henan"), ("sr", "Хенан"), ("sr_Latn", "Henan"), ("sv", "Henan"), ("sw", "Henan"), ("ta", "ஹெய\u{bcd}ந\u{bbe}ன\u{bcd}"), ("te", "హ\u{c46}న\u{c3e}న\u{c4d}"), ("th", "มณฑลเหอหนาน"), ("tr", "Henan"), ("uk", "Хенань"), ("ur", "ہینان"), ("uz", "Xenan"), ("vi", "Hà Nam"), ("yue", "河南"), ("yue_Hans", "河南"), ("zh", "河南省")]),
                        unofficial_name_list: ["Henan"].to_vec(),
                    }
                ),
                (
                    "HB",
                    Subdivision{
                        name: "HB",
                        country_alpha2: Alpha2::CN,
                        code: "HB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.546498), longitude: Some(114.341862), max_latitude: Some(33.2756161), min_latitude: Some(29.0294884), max_longitude: Some(116.1345773), min_longitude: Some(108.3669646)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hubei"), ("ar", "خوبي"), ("be", "Правінцыя Хубэй"), ("bg", "Хубей"), ("bn", "হ\u{9c1}পেই"), ("ca", "Hubei"), ("ceb", "Hubei Sheng"), ("cs", "Chu-pej"), ("cy", "Hubei"), ("da", "Hubei"), ("de", "Hubei"), ("el", "Χουπέι"), ("en", "Hubei"), ("es", "Hubei"), ("et", "Hubei"), ("eu", "Hubei"), ("fa", "هوبئی"), ("fi", "Hubei"), ("fr", "Hubei"), ("ga", "Hubei"), ("gu", "હ\u{ac1}બ\u{ac7}ઇ"), ("he", "חוביי"), ("hi", "ह\u{942}ब\u{947}ई"), ("hr", "Hubei"), ("hu", "Hupej"), ("id", "Hubei"), ("is", "Hubei"), ("it", "Hubei"), ("ja", "湖北省"), ("ka", "ხუბეი"), ("kk", "Хубэй"), ("kn", "ಹುಬೈ"), ("ko", "후베이 성"), ("lt", "Hubėjus"), ("lv", "Hubei"), ("mk", "Хубеј"), ("mr", "ह\u{942}प\u{948}"), ("ms", "Hubei"), ("nb", "Hubei"), ("nl", "Hubei"), ("no", "Hubei"), ("pa", "ਹ\u{a42}ਬ\u{a47}ਈ"), ("pl", "Hubei"), ("pt", "Hubei"), ("ro", "Hubei"), ("ru", "Хубэй"), ("si", "හ\u{dd4}බෙය\u{dd2}"), ("sl", "Hubei"), ("sr", "Хубеј"), ("sr_Latn", "Hubej"), ("sv", "Hubei"), ("sw", "Hubei"), ("ta", "ஹுபேய\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "హుబ\u{c47}"), ("th", "มณฑลห\u{e39}เป\u{e48}ย\u{e4c}"), ("tr", "Hubei"), ("uk", "Хубей"), ("ur", "ہوبئی"), ("uz", "Xubey"), ("vi", "Hồ Bắc"), ("yue", "湖北"), ("yue_Hans", "湖北"), ("zh", "湖北省")]),
                        unofficial_name_list: ["Hubei"].to_vec(),
                    }
                ),
                (
                    "HE",
                    Subdivision{
                        name: "HE",
                        country_alpha2: Alpha2::CN,
                        code: "HE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.037057), longitude: Some(114.468665), max_latitude: Some(42.6197178), min_latitude: Some(36.0482067), max_longitude: Some(119.8710782), min_longitude: Some(113.4653687)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hebei"), ("ar", "خبي"), ("az", "Hebei"), ("be", "Правінцыя Хэбэй"), ("bg", "Хъбей"), ("bn", "হ\u{9cd}যপেই"), ("ca", "Hebei"), ("ceb", "Hebei Sheng"), ("cs", "Che-pej"), ("cy", "Hebei"), ("da", "Hebei"), ("de", "Hebei"), ("el", "Χεπέι"), ("en", "Hebei"), ("es", "Hebei"), ("et", "Hebei"), ("eu", "Hebei"), ("fa", "هبئی"), ("fi", "Hebei"), ("fr", "Hebei"), ("ga", "Hebei"), ("gu", "હ\u{ac7}બી"), ("he", "חביי"), ("hi", "ह\u{947}ब\u{947}ई"), ("hr", "Hebei"), ("hu", "Hopej"), ("hy", "Հեբեյ"), ("id", "Hebei"), ("it", "Hebei"), ("ja", "河北省"), ("ka", "ხებეი"), ("kn", "ಹ\u{cc6}ಬ\u{cbf}"), ("ko", "허베이 성"), ("lt", "Hebei"), ("lv", "Hebei"), ("mk", "Хебеј"), ("ml", "ഹെബെയ\u{d4d}"), ("mn", "Хэбэй"), ("mr", "हप\u{948}"), ("ms", "Hebei"), ("nb", "Hebei"), ("nl", "Hebei"), ("no", "Hebei"), ("pl", "Hebei"), ("pt", "Hebei"), ("ro", "Hebei"), ("ru", "Хэбэй"), ("si", "හෙබෙය\u{dd2}"), ("sl", "Hebei"), ("sr", "Хебеј"), ("sr_Latn", "Hebej"), ("sv", "Hebei"), ("sw", "Hebei"), ("ta", "ஏபெய\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "హ\u{c46}బ\u{c40}"), ("th", "มณฑลเหอเป\u{e48}ย\u{e4c}"), ("tr", "Hebei"), ("uk", "Хебей"), ("ur", "ہیبئی"), ("uz", "Xebey"), ("vi", "Hà Bắc"), ("yue", "河北"), ("yue_Hans", "河北"), ("zh", "河北省")]),
                        unofficial_name_list: ["Hebei"].to_vec(),
                    }
                ),
                (
                    "HI",
                    Subdivision{
                        name: "HI",
                        country_alpha2: Alpha2::CN,
                        code: "HI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.017378), longitude: Some(110.349229), max_latitude: Some(20.1589349), min_latitude: Some(18.1576157), max_longitude: Some(111.2785748), min_longitude: Some(108.6164681)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hainan"), ("am", "ሃይናን"), ("ar", "هاينان"), ("az", "Haynan"), ("be", "Правінцыя Хайнань"), ("bg", "Хайнан"), ("bn", "হ\u{9be}ইন\u{9be}ন"), ("ca", "Hainan"), ("ceb", "Hainan Sheng"), ("cs", "Chaj-nan"), ("cy", "Hainan"), ("da", "Hainan"), ("de", "Hainan"), ("el", "Χαϊνάν"), ("en", "Hainan"), ("es", "Hainan"), ("et", "Hainan"), ("eu", "Hainan"), ("fa", "هاینان"), ("fi", "Hainan"), ("fr", "Hainan"), ("ga", "Hainan"), ("gl", "Hainan"), ("gu", "હ\u{ac8}નન"), ("he", "האינאן"), ("hi", "हाइनान"), ("hr", "Hainan"), ("hu", "Hajnan"), ("id", "Hainan"), ("is", "Hainan"), ("it", "Hainan"), ("ja", "海南省"), ("ka", "ხაინანი"), ("kk", "Хайнань аралы"), ("kn", "ಹೈನಾನ\u{ccd}"), ("ko", "하이난 성"), ("ky", "Хайнань"), ("lt", "Hainanas"), ("lv", "Hainaņa"), ("mk", "Хајнан"), ("mn", "Хайнань"), ("mr", "हाइनान"), ("ms", "Hainan"), ("nb", "Hainan"), ("nl", "Hainan"), ("no", "Hainan"), ("pa", "ਹਾਈਨਾਨ"), ("pl", "Hajnan"), ("pt", "Hainan"), ("ro", "Hainan"), ("ru", "Хайнань"), ("si", "හය\u{dd2}නන\u{dca}"), ("sk", "Chaj-nan"), ("sr", "Хајнан"), ("sr_Latn", "Hajnan"), ("sv", "Hainan"), ("sw", "Hainan"), ("ta", "ஆய\u{bcd}ன\u{bbe}ன\u{bcd}"), ("te", "హ\u{c48}నన\u{c4d}"), ("th", "มณฑลไหหลำ"), ("tr", "Hainan"), ("uk", "Хайнань"), ("ur", "ہائنان"), ("uz", "Xaynan"), ("vi", "Hải Nam"), ("yue", "海南"), ("yue_Hans", "海南"), ("zh", "海南省")]),
                        unofficial_name_list: ["Hainan"].to_vec(),
                    }
                ),
                (
                    "HK",
                    Subdivision{
                        name: "HK",
                        country_alpha2: Alpha2::CN,
                        code: "HK",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::SpecialAdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hongkong"), ("am", "ሆንግ ኮንግ"), ("ar", "هونغ كونغ"), ("as", "হংকং"), ("az", "Honkonq"), ("be", "Ганконг"), ("bg", "Хонконг"), ("bn", "হংকং"), ("bs", "Hong Kong"), ("ca", "Hong Kong"), ("ceb", "Hong Kong"), ("cs", "Hongkong"), ("cy", "Hong Cong"), ("da", "Hongkong"), ("de", "Hongkong"), ("el", "Χονγκ Κονγκ"), ("en", "Hong Kong"), ("es", "Hong Kong"), ("et", "Hongkong"), ("eu", "Hong Kong"), ("fa", "هنگ کنگ"), ("fi", "Hongkong"), ("fr", "Hong Kong"), ("ga", "Hong Cong"), ("gl", "Hong Kong"), ("gu", "હો\u{a82}ગકો\u{a82}ગ"), ("ha", "Hong Kong"), ("ha_NE", "Hong Kong"), ("he", "הונג קונג"), ("hi", "हा\u{902}गका\u{902}ग"), ("hr", "Hong Kong"), ("hu", "Hongkong"), ("hy", "Հոնկոնգ"), ("id", "Hong Kong"), ("ig", "Hong Kong"), ("is", "Hong Kong"), ("it", "Hong Kong"), ("ja", "香港"), ("jv", "Hong Kong"), ("ka", "ჰონგ-კონგი"), ("kk", "Гонконг"), ("km", "ហ\u{17bb}ងក\u{17bb}ង"), ("kn", "ಹಾಂಗ\u{ccd} ಕಾಂಗ\u{ccd}"), ("ko", "홍콩"), ("ky", "Гонконг"), ("lo", "ຮ\u{ebb}ງກ\u{ebb}ງ"), ("lt", "Honkongas"), ("lv", "Honkonga"), ("mk", "Хонгконг"), ("ml", "ഹോങ\u{d4d}കോങ\u{d4d}"), ("mn", "Хонг Конг"), ("mr", "हा\u{901}ग का\u{901}ग"), ("ms", "Hong Kong"), ("my", "ဟောင\u{103a}ကောင\u{103a}"), ("ne", "हङकङ"), ("nl", "Hongkong"), ("no", "Hongkong"), ("or", "ହଂକଂ"), ("pa", "ਹਾ\u{a02}ਗ ਕਾ\u{a02}ਗ"), ("pl", "Hongkong"), ("ps", "هانګ کانګ"), ("pt", "Hong Kong"), ("ro", "Hong Kong"), ("ru", "Гонконг"), ("si", "හොංකොං"), ("sk", "Hongkong"), ("sl", "Hong Kong"), ("so", "Hong Kong"), ("sq", "Hong Kong"), ("sr", "Хонгконг"), ("sr_Latn", "Hongkong"), ("sv", "Hongkong"), ("sw", "Hong Kong"), ("ta", "ஃகொங\u{bcd}கொங\u{bcd}"), ("te", "హ\u{c3e}ంగ\u{c4d}\u{200c}క\u{c3e}ంగ\u{c4d}"), ("th", "เขตบร\u{e34}หารพ\u{e34}เศษฮ\u{e48}องกง"), ("tk", "Gonkong"), ("tr", "Hong Kong"), ("uk", "Гонконг"), ("ur", "ہانگ کانگ"), ("uz", "Gonkong"), ("vi", "Hồng Kông"), ("yo", "Họ\u{301}ng Kọng"), ("yo_BJ", "Hɔ\u{301}ng Kɔng"), ("yue", "香港"), ("yue_Hans", "香港"), ("zh", "香港")]),
                        unofficial_name_list: ["Hongkong", "Xianggang"].to_vec(),
                    }
                ),
                (
                    "HL",
                    Subdivision{
                        name: "HL",
                        country_alpha2: Alpha2::CN,
                        code: "HL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.74236699999999), longitude: Some(126.661665), max_latitude: Some(53.5636239), min_latitude: Some(43.4257985), max_longitude: Some(135.0956699), min_longitude: Some(121.1817531)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Heilongjiang"), ("ar", "هيلونغجيانغ"), ("be", "Правінцыя Хэйлунцзян"), ("bg", "Хъйлундзян"), ("bn", "হেইলোংচিয\u{9bc}\u{9be}ং"), ("ca", "Heilongjiang"), ("ceb", "Heilongjiang Sheng"), ("cs", "Chej-lung-ťiang"), ("cy", "Heilongjiang"), ("da", "Heilongjiang"), ("de", "Heilongjiang"), ("el", "Χεϊλονγκτσιάνγκ"), ("en", "Heilongjiang"), ("es", "Heilongjiang"), ("et", "Heilongjiang"), ("eu", "Heilongjiang"), ("fa", "هیلونگ\u{200c}جیانگ"), ("fi", "Heilongjiang"), ("fr", "Heilongjiang"), ("ga", "Heilongjiang"), ("gl", "Heilongjiang"), ("gu", "હ\u{ac7}યલો\u{a82}ગજિયા\u{a82}ગ"), ("he", "חיילונגג׳יאנג"), ("hi", "ह\u{947}इलो\u{902}गजिया\u{902}ग"), ("hr", "Heilongjiang"), ("hu", "Hejlungcsiang"), ("hy", "Հեյլունցզյան"), ("id", "Heilongjiang"), ("it", "Heilongjiang"), ("ja", "黒竜江省"), ("ka", "ხეილუნძიანი"), ("kn", "ಹೀಲೋಂಗ\u{ccd}ಜ\u{cbf}ಯಾಂಗ\u{ccd}"), ("ko", "헤이룽장 성"), ("lt", "Heilongdziangas"), ("lv", "Heilundzjana"), ("mk", "Хејлунгѓанг"), ("mn", "Хармөрөн"), ("mr", "ह\u{948}लो\u{902}गच\u{94d}या\u{902}ग"), ("ms", "Heilongjiang"), ("nb", "Heilongjiang"), ("nl", "Heilongjiang"), ("no", "Heilongjiang"), ("pl", "Heilongjiang"), ("pt", "Heilongjiang"), ("ro", "Heilongjiang"), ("ru", "Хэйлунцзян"), ("si", "හෙල\u{dd2}ඔන\u{dca}ග\u{dca}ජ\u{dd2}ය\u{dcf}න\u{dca}ග\u{dca}"), ("sr", "Хејлунгђанг"), ("sr_Latn", "Hejlungđang"), ("sv", "Heilongjiang"), ("sw", "Heilongjiang"), ("ta", "கெய\u{bcd}லோங\u{bcd}சிய\u{bbe}ங\u{bcd}"), ("te", "హ\u{c40}ల\u{c3e}ంగ\u{c4d}\u{200c}జ\u{c3f}య\u{c3e}ంగ\u{c4d}"), ("th", "มณฑลเฮย\u{e4c}หลงเจ\u{e35}ยง"), ("tr", "Heilongjiang"), ("uk", "Провінція Хейлунцзян"), ("ur", "ہیلونگجیانگ"), ("uz", "Xeylunszyan"), ("vi", "Hắc Long Giang"), ("yue", "黑龍江"), ("yue_Hans", "黑龙江"), ("zh", "黑龙江省")]),
                        unofficial_name_list: ["Heilongjiang"].to_vec(),
                    }
                ),
                (
                    "HN",
                    Subdivision{
                        name: "HN",
                        country_alpha2: Alpha2::CN,
                        code: "HN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.112449), longitude: Some(112.98381), max_latitude: Some(30.126363), min_latitude: Some(24.6363234), max_longitude: Some(114.2612647), min_longitude: Some(108.7908413)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hunan"), ("ar", "خونان"), ("az", "Xunan"), ("be", "Правінцыя Хунань"), ("bg", "Хунан"), ("bn", "হ\u{9c1}ন\u{9be}ন"), ("ca", "Hunan"), ("ceb", "Hunan Sheng"), ("cs", "Chu-nan"), ("cy", "Hunan"), ("da", "Hunan"), ("de", "Hunan"), ("el", "Χουνάν"), ("en", "Hunan"), ("es", "Hunan"), ("et", "Hunan"), ("eu", "Hunan"), ("fa", "هونان"), ("fi", "Hunan"), ("fr", "Hunan"), ("ga", "Hunan"), ("gu", "હ\u{ac1}નાન"), ("he", "חונאן"), ("hi", "ह\u{942}नान"), ("hr", "Hunan"), ("hu", "Hunan"), ("id", "Hunan"), ("it", "Hunan"), ("ja", "湖南省"), ("ka", "ხუნანი"), ("kn", "ಹುನಾನ\u{ccd}"), ("ko", "후난 성"), ("lt", "Hunanas"), ("lv", "Hunaņa"), ("mk", "Хунан"), ("ml", "ഹ\u{d41}ന\u{d3e}ൻ"), ("mn", "Хүнань"), ("mr", "ह\u{942}नान"), ("ms", "Hunan"), ("nb", "Hunan"), ("nl", "Hunan"), ("no", "Hunan"), ("pl", "Hunan"), ("pt", "Hunan"), ("ro", "Hunan"), ("ru", "Хунань"), ("si", "හ\u{dd4}න\u{dcf}න\u{dca}"), ("sl", "Hunan"), ("sr", "Хунан"), ("sr_Latn", "Hunan"), ("sv", "Hunan"), ("sw", "Hunan"), ("ta", "ஹுன\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "హున\u{c3e}న\u{c4d}"), ("th", "มณฑลห\u{e39}หนาน"), ("tr", "Hunan"), ("uk", "Хунань"), ("ur", "ہونان"), ("uz", "Xunan"), ("vi", "Hồ Nam"), ("yue", "湖南"), ("yue_Hans", "湖南"), ("zh", "湖南省")]),
                        unofficial_name_list: ["Hunan"].to_vec(),
                    }
                ),
                (
                    "JL",
                    Subdivision{
                        name: "JL",
                        country_alpha2: Alpha2::CN,
                        code: "JL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.837883), longitude: Some(126.549572), max_latitude: Some(44.6481581), min_latitude: Some(42.5614763), max_longitude: Some(127.9247254), min_longitude: Some(125.6457386)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Jilin"), ("ar", "جيلين"), ("be", "Правінцыя Гірын"), ("bg", "Дзилин"), ("bn", "চিলিন"), ("ca", "Jilin"), ("ceb", "Jilin Sheng"), ("cs", "Ťi-lin"), ("cy", "Jilin"), ("da", "Jilin"), ("de", "Jilin"), ("el", "Τζιλίν"), ("en", "Jilin"), ("es", "Jilin"), ("et", "Jilini provints"), ("eu", "Jilin"), ("fa", "جی\u{200c}لین"), ("fi", "Jilin"), ("fr", "Jilin"), ("ga", "Jilin"), ("gl", "Jilin"), ("gu", "જિલિન"), ("he", "ג׳ילין"), ("hi", "जीलिन"), ("hr", "Jilin"), ("hu", "Csilin"), ("hy", "Գիրին"), ("id", "Jilin"), ("it", "Jilin"), ("ja", "吉林省"), ("jv", "Jilin"), ("ka", "გირინი"), ("kn", "ಜ\u{cbf}ಲ\u{cbf}ನ\u{ccd}"), ("ko", "지린 성"), ("lt", "Dzilinas"), ("lv", "Dzjiliņa"), ("mk", "Ѓилин"), ("mn", "Гирин"), ("mr", "चीलिन"), ("ms", "Jilin"), ("nb", "Jilin"), ("nl", "Jilin"), ("no", "Jilin"), ("pa", "ਜੀਲਿਨ"), ("pl", "Jilin"), ("pt", "Jilin"), ("ro", "Jilin"), ("ru", "Гирин"), ("si", "ජ\u{dd2}ල\u{dd2}න\u{dca}"), ("sr", "Ђилин"), ("sr_Latn", "Đilin"), ("sv", "Jilin"), ("sw", "Jilin"), ("ta", "சிலின\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "జ\u{c3f}ల\u{c3f}న\u{c4d}"), ("th", "มณฑลจ\u{e35}\u{e4b}หล\u{e34}น"), ("tr", "Jilin"), ("uk", "Цзілінь"), ("ur", "جیلن"), ("uz", "Girin"), ("vi", "Cát Lâm"), ("yue", "吉林"), ("yue_Hans", "吉林"), ("zh", "吉林省")]),
                        unofficial_name_list: ["Jilin"].to_vec(),
                    }
                ),
                (
                    "JS",
                    Subdivision{
                        name: "JS",
                        country_alpha2: Alpha2::CN,
                        code: "JS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.061707), longitude: Some(118.763232), max_latitude: Some(35.1245136), min_latitude: Some(30.7578406), max_longitude: Some(121.9627784), min_longitude: Some(116.3619604)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Jiangsu"), ("ar", "جيانغسو"), ("az", "Cianqsu"), ("be", "Правінцыя Цзянсу"), ("bg", "Дзянсу"), ("bn", "চিয\u{9bc}\u{9be}ংস\u{9c1}"), ("ca", "Jiangsu"), ("ceb", "Jiangsu Sheng"), ("cs", "Ťiang-su"), ("cy", "Jiangsu"), ("da", "Jiangsu"), ("de", "Jiangsu"), ("el", "Τσιανγκσού"), ("en", "Jiangsu"), ("es", "Jiangsu"), ("et", "Jiangsu"), ("eu", "Jiangsu"), ("fa", "جیانگسو"), ("fi", "Jiangsu"), ("fr", "Jiangsu"), ("ga", "Jiangsu"), ("gu", "જિઆ\u{a82}ગ\u{acd}સ\u{ac1}"), ("he", "ג׳יאנגסו"), ("hi", "जिआ\u{902}गस\u{942}"), ("hr", "Jiangsu"), ("hu", "Csiangszu"), ("hy", "Ցզյանսու"), ("id", "Jiangsu"), ("it", "Jiangsu"), ("ja", "江蘇省"), ("ka", "ძიანსუ"), ("kn", "ಜ\u{cbf}ಯಾಂಗ\u{ccd}ಸು"), ("ko", "장쑤 성"), ("lt", "Dziangsu"), ("lv", "Dzjansu"), ("mk", "Ѓангсу"), ("ml", "ജ\u{d3e}ങ\u{d4d}സ\u{d42}"), ("mn", "Жянсү"), ("mr", "च\u{94d}या\u{902}ग\u{94d}स\u{942}"), ("ms", "Jiangsu"), ("nb", "Jiangsu"), ("nl", "Jiangsu"), ("no", "Jiangsu"), ("pl", "Jiangsu"), ("pt", "Jiangsu"), ("ro", "Jiangsu"), ("ru", "Цзянсу"), ("si", "ජ\u{dd2}යන\u{dca}ග\u{dca}ස\u{dd4}"), ("sl", "Džiangsu"), ("sr", "Ђангсу"), ("sr_Latn", "Đangsu"), ("sv", "Jiangsu"), ("sw", "Jiangsu"), ("ta", "சிய\u{bbe}ங\u{bcd}சு"), ("te", "జ\u{c3f}య\u{c3e}ంగ\u{c4d}సు"), ("th", "มณฑลเจ\u{e35}ยงซ\u{e39}"), ("tr", "Jiangsu"), ("uk", "Цзянсу"), ("ur", "جیانگسو"), ("uz", "Szyansu"), ("vi", "Giang Tô"), ("yue", "江蘇"), ("yue_Hans", "江苏"), ("zh", "江苏省")]),
                        unofficial_name_list: ["Jiangsu"].to_vec(),
                    }
                ),
                (
                    "JX",
                    Subdivision{
                        name: "JX",
                        country_alpha2: Alpha2::CN,
                        code: "JX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.675697), longitude: Some(115.909228), max_latitude: Some(30.0749464), min_latitude: Some(24.4862522), max_longitude: Some(118.4871059), min_longitude: Some(113.5781862)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Jiangsji"), ("ar", "جيانغشي"), ("az", "Jianqxi"), ("be", "Правінцыя Цзянсі"), ("bg", "Дзянси"), ("bn", "চিয\u{9bc}\u{9be}ংশি"), ("ca", "Jiangxi"), ("ceb", "Jiangxi Sheng"), ("cs", "Ťiang-si"), ("cy", "Jiangxi"), ("da", "Jiangxi"), ("de", "Jiangxi"), ("el", "Τσιανγκσί"), ("en", "Jiangxi"), ("es", "Jiangxi"), ("et", "Jiangxi"), ("eu", "Jiangxi"), ("fa", "جیانگشی"), ("fi", "Jiangxi"), ("fr", "Jiangxi"), ("ga", "Jiangxi"), ("gu", "જીએન\u{acd}ક\u{acd}સી"), ("he", "ג׳יאנגשי"), ("hi", "जिआ\u{902}गशी"), ("hr", "Jiangxi"), ("hu", "Csianghszi"), ("hy", "Ցզյանսի"), ("id", "Jiangxi"), ("it", "Jiangxi"), ("ja", "江西省"), ("ka", "ძიანსი"), ("kn", "ಜ\u{cbf}ಯಾಂಗ\u{ccd}ಕ\u{ccd}ಸ\u{cbf}"), ("ko", "장시 성"), ("lt", "Dziangsi"), ("lv", "Dzjansji"), ("mk", "Ѓангси"), ("mn", "Жянши муж"), ("mr", "च\u{94d}या\u{902}ग\u{94d}शी"), ("ms", "Jiangxi"), ("nb", "Jiangxi"), ("nl", "Jiangxi"), ("no", "Jiangxi"), ("pl", "Jiangxi"), ("pt", "Jiangxi"), ("ro", "Jiangxi"), ("ru", "Цзянси"), ("si", "ජ\u{dd2}යන\u{dca}ග\u{dca}ක\u{dca}ස\u{dd2}"), ("sr", "Ђангси"), ("sr_Latn", "Đangsi"), ("sv", "Jiangxi"), ("sw", "Jiangxi"), ("ta", "ஜிய\u{bbe}ங\u{bcd}சி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "జ\u{c3f}య\u{c3e}ంగ\u{c4d}స\u{c40}"), ("th", "มณฑลเจ\u{e35}ยงซ\u{e35}"), ("tr", "Jiangxi"), ("uk", "Цзянсі"), ("ur", "جیانگشی"), ("uz", "Szyansi"), ("vi", "Giang Tây"), ("yue", "江西"), ("yue_Hans", "江西"), ("zh", "江西省")]),
                        unofficial_name_list: ["Jiangxi"].to_vec(),
                    }
                ),
                (
                    "LN",
                    Subdivision{
                        name: "LN",
                        country_alpha2: Alpha2::CN,
                        code: "LN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.836175), longitude: Some(123.431383), max_latitude: Some(43.4909841), min_latitude: Some(38.7224309), max_longitude: Some(125.7914594), min_longitude: Some(118.845467)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Liaoning"), ("ar", "لياونينغ"), ("be", "Правінцыя Ляанін"), ("bg", "Ляонин"), ("bn", "লিয\u{9bc}\u{9be}ওনিং"), ("ca", "Liaoning"), ("ceb", "Liaoning Sheng"), ("cs", "Liao-ning"), ("cy", "Liaoning"), ("da", "Liaoning"), ("de", "Liaoning"), ("el", "Λιαόνινγκ"), ("en", "Liaoning"), ("es", "Liaoning"), ("et", "Liaoning"), ("eu", "Liaoning"), ("fa", "لیائونینگ"), ("fi", "Liaoning"), ("fr", "Liaoning"), ("ga", "Liaoning"), ("gl", "Liaoning"), ("gu", "લિયોનિ\u{a82}ગ"), ("he", "ליאונינג"), ("hi", "लियाओनि\u{902}ग"), ("hr", "Liaoning"), ("hu", "Liaoning"), ("hy", "Լյաոնին"), ("id", "Liaoning"), ("it", "Liaoning"), ("ja", "遼寧省"), ("ka", "ლიაონინი"), ("kn", "ಲ\u{cbf}ಯಾನ\u{cbf}ಂಗ\u{ccd}"), ("ko", "랴오닝 성"), ("lt", "Liaoningas"), ("lv", "Liaonina"), ("mk", "Љаонинг"), ("mn", "Ляонин"), ("mr", "ल\u{94d}याओनि\u{902}ग"), ("ms", "Liaoning"), ("my", "လျောင\u{103a}းနင\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "Liaoning"), ("nl", "Liaoning"), ("no", "Liaoning"), ("pa", "ਲੀਆਓਨਿ\u{a70}ਗ"), ("pl", "Liaoning"), ("pt", "Liaoning"), ("ro", "Liaoning"), ("ru", "Ляонин"), ("si", "ල\u{dd2}යෝන\u{dd2}න\u{dca}ග\u{dca}"), ("sl", "Liaoning"), ("sr", "Љаонинг"), ("sr_Latn", "Ljaoning"), ("sv", "Liaoning"), ("sw", "Liaoning"), ("ta", "லிய\u{bbe}வோனிங\u{bcd}"), ("te", "ల\u{c3f}య\u{c4b}వ\u{c3e}న\u{c3f}ంగ\u{c4d}"), ("th", "มณฑลเหล\u{e35}ยวหน\u{e34}ง"), ("tr", "Liaoning"), ("uk", "Ляонін"), ("ur", "لیاؤننگ"), ("uz", "Lyaonin"), ("vi", "Liêu Ninh"), ("yue", "遼寧"), ("yue_Hans", "辽宁"), ("zh", "辽宁省")]),
                        unofficial_name_list: ["Liaoning"].to_vec(),
                    }
                ),
                (
                    "MO",
                    Subdivision{
                        name: "MO",
                        country_alpha2: Alpha2::CN,
                        code: "MO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.07663429999999), longitude: Some(4.313052), max_latitude: Some(52.0766395), min_latitude: Some(52.07662879999999), max_longitude: Some(4.313068599999999), min_longitude: Some(4.3130354)}),
                        comments: None,
                        subdivision_type: SubdivisionType::SpecialAdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Macau"), ("am", "ማካው"), ("ar", "ماكاو"), ("as", "ম\u{9be}ক\u{9be}ও"), ("az", "Makao"), ("be", "Макаа"), ("bg", "Макао"), ("bn", "ম\u{9be}ক\u{9be}ও"), ("bs", "Makao"), ("ca", "Macau"), ("ceb", "Macau"), ("cs", "Macao"), ("cy", "Macau"), ("da", "Macao"), ("de", "Macau"), ("el", "Μακάου"), ("en", "Macau"), ("es", "Macao"), ("et", "Macau"), ("eu", "Macao"), ("fa", "ماکائو"), ("fi", "Macao"), ("fr", "Macao"), ("ga", "Macau"), ("gu", "મકાઉ"), ("ha", "Macau"), ("ha_NE", "Macau"), ("he", "מקאו"), ("hi", "मकाउ"), ("hr", "Makao"), ("hu", "Makaó"), ("hy", "Մակաու"), ("id", "Makau"), ("is", "Makaó"), ("it", "Macao"), ("ja", "マカオ"), ("jv", "Makau"), ("ka", "მაკაო"), ("kk", "Аумын"), ("km", "ម\u{17c9}ាកាវ"), ("kn", "ಮಕಾವು"), ("ko", "마카오"), ("ky", "Макао"), ("lt", "Makao"), ("lv", "Makao"), ("mk", "Макао"), ("ml", "മക\u{d57}"), ("mn", "Макао"), ("mr", "मकाओ"), ("ms", "Makau"), ("my", "မကာအ\u{102d}\u{102f}"), ("ne", "मकाउ"), ("nl", "Macau"), ("no", "Macao"), ("or", "ମକ\u{b3e}ଉ"), ("pa", "ਮਕਾਉ"), ("pl", "Makau"), ("pt", "Macau"), ("ro", "Macao"), ("ru", "Макао"), ("si", "මක\u{dcf}වෝ"), ("sk", "Macao"), ("sl", "Macao"), ("so", "Makaw"), ("sq", "Makao"), ("sr", "Макао"), ("sr_Latn", "Makao"), ("sv", "Macao"), ("sw", "Macau"), ("ta", "மக\u{bcd}க\u{bbe}வு"), ("te", "మక\u{c3e}వు"), ("th", "มาเก\u{e4a}า"), ("tk", "Makao"), ("tr", "Makao"), ("uk", "Аоминь"), ("ur", "مکاؤ"), ("uz", "Makao"), ("vi", "Ma Cao"), ("yo", "Màkáù"), ("yo_BJ", "Màkáù"), ("yue", "澳門"), ("yue_Hans", "澳门"), ("zh", "澳門")]),
                        unofficial_name_list: ["Aomen (zh) ***"].to_vec(),
                    }
                ),
                (
                    "NM",
                    Subdivision{
                        name: "NM",
                        country_alpha2: Alpha2::CN,
                        code: "NM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.81739), longitude: Some(111.76629), max_latitude: Some(53.33717799999999), min_latitude: Some(37.4067797), max_longitude: Some(126.0755856), min_longitude: Some(97.17276269999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Binne-Mongolië"), ("ar", "منغوليا الداخلية"), ("az", "Daxili Monqolustan Muxtar Rayonu"), ("be", "Унутраная Манголія"), ("bg", "Вътрешна Монголия"), ("bn", "অন\u{9cd}তর\u{9cd}দেশীয\u{9bc} মঙ\u{9cd}গোলিয\u{9bc}\u{9be}"), ("bs", "Unutrašnja Mongolija"), ("ca", "Mongòlia Interior"), ("ceb", "Inner Mongolia Autonomous Region"), ("cs", "Vnitřní Mongolsko"), ("cy", "Mongolia Fewnol"), ("da", "Indre Mongoliet"), ("de", "Innere Mongolei"), ("el", "Εσωτερική Μογγολία"), ("en", "Inner Mongolia"), ("es", "Mongolia Interior"), ("et", "Sise-Mongoolia"), ("eu", "Barne Mongolia"), ("fa", "مغولستان داخلی"), ("fi", "Sisä-Mongolia"), ("fr", "Mongolie-Intérieure"), ("ga", "An Ion-Mhongóil"), ("gl", "Mongolia Interior"), ("gu", "ઇનર મ\u{a82}ગોલિયા"), ("he", "מונגוליה הפנימית"), ("hi", "भीतरी म\u{902}गोलिया"), ("hr", "Unutarnja Mongolija"), ("hu", "Belső-Mongólia Autonóm Terület"), ("id", "Mongolia Dalam"), ("it", "Mongolia Interna"), ("ja", "内モンゴル自治区"), ("ka", "შიგა მონღოლეთი"), ("kk", "Ішкі Моңғолия"), ("kn", "ಇನ\u{ccd}ನರ\u{ccd} ಮಂಗೋಲ\u{cbf}ಯಾ"), ("ko", "내몽골 자치구"), ("lt", "Vidinė Mongolija"), ("lv", "Iekšējā Mongolija"), ("mk", "Внатрешна Монголија"), ("ml", "ഇന\u{d4d}നർ മംഗോളിയ"), ("mn", "Өвөр Монголын Өөртөө Засах Орон"), ("mr", "आ\u{902}तरिक म\u{902}गोलिया"), ("ms", "Mongolia Dalam"), ("nb", "Indre Mongolia"), ("ne", "भित\u{94d}री मङ\u{94d}गोलिया"), ("nl", "Binnen-Mongolië"), ("no", "Indre Mongolia"), ("pa", "ਅ\u{a70}ਦਰ\u{a42}ਨੀ ਮ\u{a70}ਗ\u{a4b}ਲੀਆ"), ("pl", "Mongolia Wewnętrzna"), ("pt", "Mongólia Interior"), ("ro", "Mongolia Interioară"), ("ru", "Внутренняя Монголия"), ("si", "ඇත\u{dd4}ල\u{dca} මොන\u{dca}ගෝල\u{dd2}ය\u{dcf}ව"), ("sk", "Vnútorné Mongolsko"), ("sr", "Унутрашња Монголија"), ("sr_Latn", "Unutrašnja Mongolija"), ("sv", "Inre Mongoliet"), ("sw", "Mongolia ya Kichina"), ("ta", "உள\u{bcd} மங\u{bcd}கோலிய\u{bbe}"), ("te", "ఇన\u{c4d}నర\u{c4d} మంగ\u{c4b}ల\u{c3f}య\u{c3e}"), ("th", "เขตปกครองตนเองมองโกเล\u{e35}ยใน"), ("tr", "İç Moğolistan"), ("uk", "Внутрішня Монголія"), ("ur", "اندرونی منگولیا"), ("uz", "Ichki Mongoliya"), ("vi", "Nội Mông"), ("yue", "內蒙古"), ("yue_Hans", "内蒙古"), ("zh", "内蒙古自治区")]),
                        unofficial_name_list: ["Inner Mongolia", "Nei Monggol"].to_vec(),
                    }
                ),
                (
                    "NX",
                    Subdivision{
                        name: "NX",
                        country_alpha2: Alpha2::CN,
                        code: "NX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.471318), longitude: Some(106.258754), max_latitude: Some(39.3815692), min_latitude: Some(35.2383034), max_longitude: Some(107.6592744), min_longitude: Some(104.2870392)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ningxia"), ("ar", "نينغشيا"), ("az", "Ninsya-Huey Muxtar Rayonu"), ("be", "Нінся-Хуэйскі аўтаномны раён"), ("bg", "Нинся-хуейски автономен регион"), ("bn", "নিংশিয\u{9bc}\u{9be}"), ("ca", "Ningxia"), ("ceb", "Ningxia Huizu Zizhiqu"), ("cs", "Ning-sia"), ("cy", "Ningxia"), ("da", "Ningxia"), ("de", "Ningxia"), ("el", "Νινγκσιά"), ("en", "Ningxia"), ("es", "Ningxia"), ("et", "Ningxia"), ("eu", "Ningxia"), ("fa", "نینگ\u{200c}شیا"), ("fi", "Ningxia"), ("fr", "Níngxià"), ("ga", "Ningxia"), ("gl", "Ningxia"), ("gu", "નિ\u{a82}ગઝિયા હ\u{ac1}ઈ ઓટોનોમસ પ\u{acd}રદ\u{ac7}શ"), ("he", "נינגשיה"), ("hi", "नि\u{902}गशिया"), ("hr", "Ningxia"), ("hu", "Ninghszia-Huj Autonóm Terület"), ("hy", "Նինսյա-Խուեյ"), ("id", "Ningxia"), ("it", "Ningsia"), ("ja", "寧夏回族自治区"), ("jv", "Ningxia"), ("ka", "ნინსია-ხუეი"), ("kk", "Нинся"), ("kn", "ನ\u{cbf}ಂಗ\u{ccd}ಕ\u{ccd}ಸ\u{cbf}ಯಾ ಹುಯ\u{cbf} ಸ\u{ccd}ವಾಯತ\u{ccd}ತ ಪ\u{ccd}ರದೇಶ"), ("ko", "닝샤 후이족 자치구"), ("lt", "Ningsia"), ("lv", "Ninsjas Hueju autonomais reģions"), ("mk", "Нингсја"), ("ml", "നിൻഗ\u{d4d}സിയ"), ("mn", "Ниншя - Хотонгийн өөртөө засах орон"), ("mr", "नि\u{902}ग\u{94d}स\u{94d}या"), ("ms", "Ningxia"), ("nb", "Ningxia"), ("nl", "Ningxia"), ("no", "Ningxia"), ("pl", "Ningxia"), ("pt", "Ningxia"), ("ro", "Ningxia"), ("ru", "Нинся-Хуэйский автономный район"), ("si", "න\u{dd2}න\u{dca}ග\u{dca}ක\u{dca}ස\u{dd2}ය\u{dcf} හ\u{dd4}ය\u{dd2} ස\u{dca}\u{200c}ව\u{dcf}ධ\u{dd3}න කල\u{dcf}පය"), ("sr", "Нингсја"), ("sr_Latn", "Ningsja"), ("sv", "Ningxia"), ("sw", "Ningxia"), ("ta", "நின\u{bcd}ஷிய\u{bbe} தன\u{bcd}ன\u{bbe}ட\u{bcd}சிப\u{bcd} பகுதி"), ("te", "న\u{c3f}ంజ\u{c40}\u{c3f}య\u{c3e} హూయ\u{c3f} అట\u{c3e}నమస\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตปกครองตนเองหน\u{e34}งเซ\u{e35}\u{e48}ยห\u{e38}ย"), ("tr", "Ningxia Huizu Özerk Bölgesi"), ("uk", "Нінся-Хуейський автономний район"), ("ur", "نینگشیا"), ("uz", "Ninsya-Xuey muxtor rayoni"), ("vi", "Ninh Hạ"), ("yue", "寧夏"), ("yue_Hans", "宁夏"), ("zh", "宁夏回族自治区")]),
                        unofficial_name_list: ["Ningxia Hui"].to_vec(),
                    }
                ),
                (
                    "QH",
                    Subdivision{
                        name: "QH",
                        country_alpha2: Alpha2::CN,
                        code: "QH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.620938), longitude: Some(101.780251), max_latitude: Some(39.2083444), min_latitude: Some(31.5986623), max_longitude: Some(103.0709708), min_longitude: Some(89.40442469999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Qinghai"), ("ar", "تشينغهاي"), ("be", "Правінцыя Цынхай"), ("bg", "Цинхай"), ("bn", "ছিংহ\u{9be}ই"), ("ca", "Qinghai"), ("ceb", "Qinghai Sheng"), ("cs", "Čching-chaj"), ("cy", "Qinghai"), ("da", "Qinghai"), ("de", "Qinghai"), ("el", "Κινγκχάι"), ("en", "Qinghai"), ("es", "Qinghai"), ("et", "Qinghai"), ("eu", "Qinghai"), ("fa", "چینگهای"), ("fi", "Qinghai"), ("fr", "Qinghai"), ("ga", "Qinghai"), ("gl", "Qinghai"), ("gu", "ક\u{acd}વિઘાઈ"), ("he", "צ׳ינגהאי"), ("hi", "चि\u{902}गहई"), ("hr", "Qinghai"), ("hu", "Csinghaj"), ("hy", "Ցինհայ"), ("id", "Qinghai"), ("it", "Qinghai"), ("ja", "青海省"), ("ka", "ცინხაი"), ("kn", "ಕ\u{ccd}ವ\u{cbf}ಂಗ\u{ccd}ಹೈ"), ("ko", "칭하이 성"), ("lt", "Činghai"), ("lv", "Cjinhai"), ("mk", "Ќингхај"), ("mn", "Хөхнуур"), ("mr", "छि\u{902}गहाय"), ("ms", "Qinghai"), ("nb", "Qinghai"), ("nl", "Qinghai"), ("no", "Qinghai"), ("pl", "Qinghai"), ("pt", "Qinghai"), ("ro", "Qinghai"), ("ru", "Цинхай"), ("si", "ක\u{dd2}න\u{dca}ග\u{dca}හය\u{dd2}"), ("sr", "Ћингхај"), ("sr_Latn", "Ćinghaj"), ("sv", "Qinghai"), ("sw", "Qinghai"), ("ta", "கிங\u{bcd}ஹ\u{bbe}ய\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఖ\u{c3f}ంగ\u{c3e}య\u{c4d}"), ("th", "มณฑลช\u{e34}งไห\u{e48}"), ("tr", "Çinghay"), ("uk", "Цінхай"), ("ur", "چنگھائی"), ("uz", "Sinxay"), ("vi", "Thanh Hải"), ("yue", "青海"), ("yue_Hans", "青海"), ("zh", "青海省")]),
                        unofficial_name_list: ["Qinghai"].to_vec(),
                    }
                ),
                (
                    "SC",
                    Subdivision{
                        name: "SC",
                        country_alpha2: Alpha2::CN,
                        code: "SC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.651652), longitude: Some(104.075931), max_latitude: Some(34.313), min_latitude: Some(26.0458656), max_longitude: Some(108.5467123), min_longitude: Some(97.34808160000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sitsjuan"), ("ar", "سيتشوان"), ("az", "Sıçuan"), ("be", "Правінцыя Сычуань"), ("bg", "Съчуан"), ("bn", "সিছ\u{9c1}য\u{9bc}\u{9be}ন"), ("bs", "Sečuan"), ("ca", "Sichuan"), ("ceb", "Sichuan Sheng"), ("cs", "S’-čchuan"), ("cy", "Sichuan"), ("da", "Sichuan"), ("de", "Sichuan"), ("el", "Σιτσουάν"), ("en", "Sichuan"), ("es", "Sichuan"), ("et", "Sichuan"), ("eu", "Sichuan"), ("fa", "سیچوآن"), ("fi", "Sichuan"), ("fr", "Sichuan"), ("ga", "Sichuan"), ("gu", "સિચ\u{ac1}આન"), ("he", "סצ׳ואן"), ("hi", "सिच\u{941}आन"), ("hr", "Sečuan"), ("hu", "Szecsuan"), ("hy", "Սիչուան"), ("id", "Sichuan"), ("is", "Sesúan"), ("it", "Sichuan"), ("ja", "四川省"), ("ka", "სიჩუანი"), ("kk", "Сычуань"), ("kn", "ಸ\u{cbf}ಚುವಾನ\u{ccd}"), ("ko", "쓰촨 성"), ("lt", "Sičuanas"), ("lv", "Sičuaņa"), ("mk", "Сечуан"), ("ml", "സിഷ\u{d4d}വ\u{d3e}ൻ"), ("mn", "Сычуань муж"), ("mr", "स-च\u{94d}वान"), ("ms", "Sichuan"), ("nb", "Sichuan"), ("nl", "Sichuan"), ("no", "Sichuan"), ("pl", "Syczuan"), ("pt", "Sichuan"), ("ro", "Sichuan"), ("ru", "Сычуань"), ("si", "ස\u{dd2}ච\u{dd4}ව\u{dcf}න\u{dca}"), ("sk", "S’-čchuan"), ("sl", "Sečuan"), ("sr", "Сичуан"), ("sr_Latn", "Sičuan"), ("sv", "Sichuan"), ("sw", "Sichuan"), ("ta", "சிச\u{bcd}சுவ\u{bbe}ன\u{bcd}"), ("te", "స\u{c3f}చువ\u{c3e}న\u{c4d}"), ("th", "มณฑลเสฉวน"), ("tr", "Siçuan"), ("uk", "Сичуань"), ("ur", "سیچوان"), ("uz", "Sichuan"), ("vi", "Tứ Xuyên"), ("yue", "四川"), ("yue_Hans", "四川"), ("zh", "四川省")]),
                        unofficial_name_list: ["Sichuan"].to_vec(),
                    }
                ),
                (
                    "SD",
                    Subdivision{
                        name: "SD",
                        country_alpha2: Alpha2::CN,
                        code: "SD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.66853), longitude: Some(117.020359), max_latitude: Some(38.4011447), min_latitude: Some(34.3773522), max_longitude: Some(122.7100401), min_longitude: Some(114.8246302)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sjandong"), ("ar", "شاندونغ"), ("be", "Правінцыя Шаньдун"), ("bg", "Шандун"), ("bn", "শ\u{9be}নত\u{9c1}ং"), ("ca", "Shandong"), ("ceb", "Shandong Sheng"), ("cs", "Šan-tung"), ("cy", "Shandong"), ("da", "Shandong"), ("de", "Shandong"), ("el", "Σαν-τούνγκ"), ("en", "Shandong"), ("es", "Shandong"), ("et", "Shandong"), ("eu", "Shandong"), ("fa", "شاندونگ"), ("fi", "Shandong"), ("fr", "Shandong"), ("ga", "Shandong"), ("gu", "શાનડો\u{a82}ગ"), ("he", "שאנדונג"), ("hi", "शानदो\u{902}ग"), ("hr", "Shandong"), ("hu", "Santung"), ("hy", "Շանդուն"), ("id", "Shandong"), ("it", "Shandong"), ("ja", "山東省"), ("ka", "შანდუნი"), ("kn", "ಷಾಂಡಾಂಗ\u{ccd}"), ("ko", "산둥 성"), ("lt", "Šandongas"), ("lv", "Šaņduna"), ("mk", "Шандунг"), ("ml", "ശ\u{d3e}ൻഡോങ\u{d4d}"), ("mn", "Шаньдун"), ("mr", "षा\u{902}तो\u{902}ग"), ("ms", "Shandong"), ("nb", "Shandong"), ("nl", "Shandong"), ("no", "Shandong"), ("pl", "Szantung"), ("pt", "Shandong"), ("ro", "Shandong"), ("ru", "Шаньдун"), ("si", "ශැන\u{dca}ඩොන\u{dca}ග\u{dca}"), ("sl", "Šandong"), ("sr", "Шандунг"), ("sr_Latn", "Šandung"), ("sv", "Shandong"), ("sw", "Shandong"), ("ta", "ச\u{bbe}ண\u{bcd}டோங\u{bcd}"), ("te", "ష\u{c3e}ండ\u{c3e}ంగ\u{c4d}"), ("th", "มณฑลซานตง"), ("tr", "Şantung"), ("uk", "Шаньдун"), ("ur", "شانڈونگ"), ("uz", "Shandun"), ("vi", "Sơn Đông"), ("yue", "山東"), ("yue_Hans", "山东"), ("zh", "山东省")]),
                        unofficial_name_list: ["Shandong"].to_vec(),
                    }
                ),
                (
                    "SH",
                    Subdivision{
                        name: "SH",
                        country_alpha2: Alpha2::CN,
                        code: "SH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.230416), longitude: Some(121.473701), max_latitude: Some(31.868217), min_latitude: Some(30.68027), max_longitude: Some(122.2470663), min_longitude: Some(120.8582174)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sjanghai"), ("am", "ሻንግሃይ"), ("ar", "شانغهاي"), ("as", "ছ\u{9be}ংহ\u{9be}ই"), ("az", "Şanxay"), ("be", "Шанхай"), ("bg", "Шанхай"), ("bn", "স\u{9be}ংহ\u{9be}ই"), ("bs", "Šangaj"), ("ca", "Xangai"), ("ceb", "Shanghai"), ("cs", "Šanghaj"), ("cy", "Shanghai"), ("da", "Shanghai"), ("de", "Shanghai"), ("el", "Σαγκάη"), ("en", "Shanghai"), ("es", "Shanghái"), ("et", "Shanghai"), ("eu", "Shanghai"), ("fa", "شانگهای"), ("fi", "Shanghai"), ("fr", "Shanghai"), ("ga", "Shang-hai"), ("gl", "Shanghai"), ("gu", "શ\u{a82}ઘાઈ"), ("ha", "Shanghai"), ("ha_NE", "Shanghai"), ("he", "שאנגחאי"), ("hi", "श\u{902}घाई"), ("hr", "Šangaj"), ("hu", "Sanghaj"), ("hy", "Շանհայ"), ("id", "Shanghai"), ("is", "Sjanghæ"), ("it", "Shanghai"), ("ja", "上海市"), ("jv", "Shanghai"), ("ka", "შანხაი"), ("kk", "Шанхай"), ("kn", "ಶಾಂಘೈ"), ("ko", "상하이 시"), ("ky", "Шанхай"), ("lo", "ຊຽງໄຮ\u{ec9}"), ("lt", "Šanchajus"), ("lv", "Šanhaja"), ("mk", "Шангај"), ("ml", "ഷ\u{d3e}ങ\u{d4d}ഹ\u{d3e}യ\u{d4d}"), ("mn", "Шанхай"), ("mr", "शा\u{902}घाय"), ("ms", "Shanghai"), ("my", "ရ\u{103e}န\u{103a}ဟ\u{102d}\u{102f}င\u{103a}းမြ\u{102d}\u{102f}\u{1037}"), ("nb", "Shanghai"), ("ne", "शाङ\u{94d}घाई"), ("nl", "Shanghai"), ("no", "Shanghai"), ("pa", "ਸ\u{a3c}\u{a70}ਘਾਈ"), ("pl", "Szanghaj"), ("ps", "شانګهای"), ("pt", "Xangai"), ("ro", "Shanghai"), ("ru", "Шанхай"), ("si", "ෂැංහය\u{dd2}"), ("sk", "Šanghaj"), ("sl", "Šanghaj"), ("sq", "Shangai"), ("sr", "Шангај"), ("sr_Latn", "Šangaj"), ("sv", "Shanghai"), ("sw", "Shanghai"), ("ta", "ச\u{bbe}ங\u{bcd}க\u{bbe}ய\u{bcd}"), ("te", "ష\u{c3e}ంఘ\u{c48}"), ("th", "เซ\u{e35}\u{e48}ยงไฮ\u{e49}"), ("tk", "Şanhaý"), ("tr", "Şanghay"), ("uk", "Шанхай"), ("ur", "شنگھائی"), ("uz", "Shanxay"), ("vi", "Thượng Hải"), ("yue", "上海"), ("yue_Hans", "上海"), ("zh", "上海市")]),
                        unofficial_name_list: ["Schanghai"].to_vec(),
                    }
                ),
                (
                    "SN",
                    Subdivision{
                        name: "SN",
                        country_alpha2: Alpha2::CN,
                        code: "SN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.265472), longitude: Some(108.954239), max_latitude: Some(39.5870105), min_latitude: Some(31.7050598), max_longitude: Some(111.2480544), min_longitude: Some(105.491949)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sjaanxi"), ("ar", "شنشي"), ("be", "Правінцыя Шэньсі"), ("bg", "Шънси"), ("bn", "শ\u{9cd}য\u{9be}নসি"), ("bs", "Shaanxi"), ("ca", "Shaanxi"), ("ceb", "Shaanxi"), ("cs", "Šen-si"), ("cy", "Shaanxi"), ("da", "Shaanxi"), ("de", "Shaanxi"), ("el", "Σαάνξι"), ("en", "Shaanxi"), ("es", "Shaanxi"), ("et", "Shaanxi"), ("eu", "Shaanxi"), ("fa", "شاآنشی"), ("fi", "Shaanxi"), ("fr", "Shaanxi"), ("ga", "Shaanxi"), ("gu", "શા\u{a82}ક\u{acd}ષી"), ("he", "שאאנשי"), ("hi", "शान\u{94d}शी"), ("hr", "Shaanxi"), ("hu", "Senhszi"), ("hy", "Շենսի"), ("id", "Shaanxi"), ("it", "Shaanxi"), ("ja", "陝西省"), ("ka", "შენსი"), ("kn", "ಶಾಂಕ\u{ccd}ಸ\u{cbf}²"), ("ko", "산시 성²"), ("lt", "Šaansi"), ("lv", "Šaaņsji"), ("mk", "Шенси"), ("ml", "ഷ\u{d3e}ങ\u{d4d}സി"), ("mn", "Шэньси муж"), ("mr", "षा‘न\u{94d}शी"), ("ms", "Shaanxi"), ("nb", "Shaanxi"), ("nl", "Shaanxi"), ("no", "Shaanxi"), ("pa", "ਸ\u{a3c}\u{a48}ਨਸ\u{a3c}ੀ"), ("pl", "Shaanxi"), ("pt", "Shaanxi"), ("ro", "Shaanxi"), ("ru", "Шэньси"), ("si", "ශ\u{dcf}න\u{dca}ක\u{dca}ස\u{dd2}"), ("sr", "Шенси"), ("sr_Latn", "Šensi"), ("sv", "Shaanxi"), ("sw", "Shaanxi"), ("ta", "சென\u{bcd}சி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ష\u{c3e}ంక\u{c4d}స\u{c3f}²"), ("th", "มณฑลส\u{e48}านซ\u{e35}"), ("tr", "Şensi"), ("uk", "Шеньсі"), ("ur", "شانسی"), ("uz", "Shensi"), ("vi", "Thiểm Tây"), ("yue", "陝西"), ("yue_Hans", "陕西"), ("zh", "陕西省")]),
                        unofficial_name_list: ["Shaanxi"].to_vec(),
                    }
                ),
                (
                    "SX",
                    Subdivision{
                        name: "SX",
                        country_alpha2: Alpha2::CN,
                        code: "SX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.873532), longitude: Some(112.562398), max_latitude: Some(40.7373522), min_latitude: Some(34.5819351), max_longitude: Some(114.5689224), min_longitude: Some(110.2302481)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Shanxi"), ("ar", "شانسي"), ("be", "Правінцыя Шаньсі"), ("bg", "Шанси"), ("bn", "শ\u{9be}নসি"), ("ca", "Shanxi"), ("ceb", "Shanxi Sheng"), ("cs", "Šan-si"), ("cy", "Shanxi"), ("da", "Shanxi"), ("de", "Shanxi"), ("el", "Σανσί"), ("en", "Shanxi"), ("es", "Shanxi"), ("et", "Shanxi"), ("eu", "Shanxi"), ("fa", "شانشی"), ("fi", "Shanxi"), ("fr", "Shanxi"), ("ga", "Shanxi"), ("gu", "શાન\u{acd}ક\u{acd}સી"), ("he", "שאנשי"), ("hi", "शन\u{94d}शी"), ("hr", "Shanxi"), ("hu", "Sanhszi"), ("hy", "Շանսի"), ("id", "Shanxi"), ("it", "Shanxi"), ("ja", "山西省"), ("ka", "შანსი"), ("kn", "ಶಾಂಕ\u{ccd}ಸ\u{cbf}"), ("ko", "산시 성"), ("lt", "Šansi"), ("lv", "Šaņsji"), ("mk", "Шанси"), ("mn", "Шаньси муж"), ("mr", "षान\u{94d}शी"), ("ms", "Shanxi"), ("nb", "Shanxi"), ("nl", "Shanxi"), ("no", "Shanxi"), ("pa", "ਸ\u{a3c}ਾਨਸ\u{a3c}ੀ"), ("pl", "Shanxi"), ("pt", "Shanxi"), ("ro", "Shanxi"), ("ru", "Шаньси"), ("si", "ශන\u{dca}ක\u{dca}ස\u{dd2}"), ("sl", "Shanxi"), ("sr", "Шанси"), ("sr_Latn", "Šansi"), ("sv", "Shanxi"), ("sw", "Shanxi"), ("ta", "ச\u{bbe}ன\u{bcd}சி"), ("te", "ష\u{c3e}ంక\u{c4d}స\u{c3f}"), ("th", "มณฑลซานซ\u{e35}"), ("tr", "Şansi"), ("uk", "Шаньсі"), ("ur", "شنسی"), ("uz", "Shansi"), ("vi", "Sơn Tây"), ("yue", "山西"), ("yue_Hans", "山西"), ("zh", "山西省")]),
                        unofficial_name_list: ["Shanxi"].to_vec(),
                    }
                ),
                (
                    "TJ",
                    Subdivision{
                        name: "TJ",
                        country_alpha2: Alpha2::CN,
                        code: "TJ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.084158), longitude: Some(117.200983), max_latitude: Some(40.2532141), min_latitude: Some(38.5555781), max_longitude: Some(118.0656116), min_longitude: Some(116.7080286)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tianjin"), ("am", "ትየንጂን"), ("ar", "تيانجين"), ("az", "Tyantszin"), ("bg", "Тиендзин"), ("bn", "থিয\u{9bc}েনচিন"), ("bs", "Tianjin"), ("ca", "Tientsin"), ("ceb", "Tianjin Shi"), ("cs", "Tchien-ťin"), ("cy", "Tianjin"), ("da", "Tianjin"), ("de", "Tianjin"), ("el", "Τιαντσίν"), ("en", "Tianjin"), ("es", "Tianjin"), ("et", "Tianjin Shi"), ("eu", "Tianjin"), ("fa", "تیانجین"), ("fi", "Tianjin"), ("fr", "Tianjin"), ("ga", "Tianjin"), ("gl", "Tianjin"), ("gu", "તિયાન\u{acd}જીન"), ("ha", "Tianjin"), ("ha_NE", "Tianjin"), ("he", "טיינג׳ין"), ("hi", "तिआ\u{902}जिन"), ("hr", "Tianjin"), ("hu", "Tiencsin"), ("hy", "Տյանցզին"), ("id", "Tianjin"), ("is", "Tianjin"), ("it", "Tientsin"), ("ja", "天津市"), ("ka", "ტიანძინი"), ("kk", "Тяньцзинь"), ("kn", "ಟ\u{cbf}ಯಾಂಜ\u{cbf}ನ\u{ccd}"), ("ko", "톈진 시"), ("lt", "Tiandzinas"), ("lv", "Tjaņdziņa"), ("mk", "Тјенѓин"), ("ml", "ടിയ\u{d3e}ൻജിൻ"), ("mn", "Тяньжинь"), ("mr", "त\u{94d}या\u{902}जिन"), ("ms", "Tianjin"), ("my", "ထျန\u{103a}းကျင\u{103a}းမြ\u{102d}\u{102f}\u{1037}"), ("nb", "Tianjin"), ("nl", "Tianjin"), ("no", "Tianjin"), ("pa", "ਥਿਆਨਚਿਨ"), ("pl", "Tiencin"), ("pt", "Tianjin"), ("ro", "Tianjin"), ("ru", "Тяньцзинь"), ("si", "ට\u{dd2}යන\u{dca}ජ\u{dd2}න\u{dca}"), ("sk", "Tiencin"), ("sl", "Tjandžin"), ("sq", "Tianxhin"), ("sr", "Тјенцин"), ("sr_Latn", "Tjencin"), ("sv", "Tianjin"), ("ta", "திய\u{bbe}ன\u{bcd}ஜின\u{bcd}"), ("te", "ట\u{c3f}య\u{c3e}ంజ\u{c3f}న\u{c4d}"), ("th", "เท\u{e35}ยนจ\u{e34}น"), ("tk", "Týanszin"), ("tr", "Tientsin"), ("uk", "Тяньцзінь"), ("ur", "تیانجین"), ("uz", "Tyantszin"), ("vi", "Thiên Tân"), ("yue", "天津"), ("yue_Hans", "天津"), ("zh", "天津市")]),
                        unofficial_name_list: ["Tianjin"].to_vec(),
                    }
                ),
                (
                    "TW",
                    Subdivision{
                        name: "TW",
                        country_alpha2: Alpha2::CN,
                        code: "TW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(23.69781), longitude: Some(120.960515), max_latitude: Some(26.3836884), min_latitude: Some(20.5862202), max_longitude: Some(123.4934282), min_longitude: Some(116.7118602)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تايوان، جمهورية الصين الشعبية"), ("bn", "ত\u{9be}ইওয\u{9bc}\u{9be}ন প\u{9cd}রদেশ"), ("ca", "Taiwan"), ("cs", "Tchaj-wan (Čínská lidová republika)"), ("de", "Provinz Taiwan"), ("el", "Επαρχία Ταϊβάν (Λαϊκή Δημοκρατία της Κίνας)"), ("en", "Taiwan"), ("es", "Provincia de Taiwán"), ("fa", "استان تایوان"), ("fr", "province de Taiwan"), ("id", "Provinsi Taiwan, Republik Rakyat Tiongkok"), ("it", "provincia di Taiwan (Repubblica Popolare Cinese)"), ("ja", "台湾省"), ("ko", "타이완성"), ("mk", "Тајван (покрајина)"), ("nl", "Taiwan Province (Volksrepubliek China)"), ("pl", "Tajwan (Chińska Republika Ludowa)"), ("pt", "Província de Taiwan"), ("ru", "Тайвань"), ("sr", "Тајван (покрајина)"), ("sr_Latn", "Tajvan (pokrajina)"), ("sv", "Taiwan-provinsen i Folkrepubliken Kina"), ("th", "มณฑลไต\u{e49}หว\u{e31}น"), ("tr", "Tayvan"), ("uk", "Тайвань (провінція Китайської Народної Республіки)"), ("ur", "صوبہ تائیوان، عوامی جمہوریہ چین"), ("vi", "Đài Loan"), ("yue", "臺灣省 (中華人民共和國)"), ("yue_Hans", "台湾省 (中华人民共和国)"), ("zh", "台湾省 (中华人民共和国)")]),
                        unofficial_name_list: ["Taiwan *"].to_vec(),
                    }
                ),
                (
                    "XJ",
                    Subdivision{
                        name: "XJ",
                        country_alpha2: Alpha2::CN,
                        code: "XJ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.793026), longitude: Some(87.627704), max_latitude: Some(49.1823419), min_latitude: Some(34.334503), max_longitude: Some(96.38619419999999), min_longitude: Some(73.502355)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Xinjiang"), ("am", "ሢንጅያንግ"), ("ar", "سنجان"), ("be", "Сіньцзян-Уйгурскі аўтаномны раён"), ("bg", "Синдзян-уйгурски автономен регион"), ("bn", "শিনচিয\u{9bc}\u{9be}ং"), ("ca", "Xinjiang"), ("ceb", "Xinjiang Uygur Zizhiqu"), ("cs", "Sin-ťiang"), ("cy", "Xinjiang"), ("da", "Xinjiang"), ("de", "Xinjiang"), ("el", "Σιντζιάνγκ"), ("en", "Xinjiang"), ("es", "Sinkiang"), ("et", "Xinjiang"), ("eu", "Xinjiang"), ("fa", "سین\u{200c}کیانگ"), ("fi", "Xinjiang"), ("fr", "Xinjiang"), ("ga", "Xinjiang"), ("gl", "Xinjiang"), ("gu", "ઝિ\u{a82}જિયા\u{a82}ગ"), ("he", "שינג׳יאנג"), ("hi", "शि\u{902}जिया\u{902}ग"), ("hr", "Xinjiang"), ("hu", "Hszincsiang-Ujgur Autonóm Terület"), ("id", "Xinjiang"), ("is", "Xinjiang"), ("it", "Sinkiang"), ("ja", "新疆ウイグル自治区"), ("jv", "Xinjiang"), ("ka", "სინძიანი"), ("kk", "Шыңжаң Ұйғыр аутономиялық ауданы"), ("kn", "ಕ\u{ccd}ಸ\u{cbf}ನ\u{ccd}ಜ\u{cbf}ಯಾಂಗ\u{ccd}"), ("ko", "신장 위구르 자치구"), ("ky", "Шинжаң-Уйгур автоном району"), ("lt", "Sindziangas"), ("lv", "Siņdzjana"), ("mk", "Синѓанг"), ("ml", "സിൻജിയ\u{d3e}ങ\u{d4d}"), ("mn", "Шиньжян - Уйгурын өөртөө засах орон"), ("mr", "शि\u{902}च\u{94d}या\u{902}ग"), ("ms", "Xinjiang"), ("nb", "Xinjiang"), ("nl", "Sinkiang"), ("no", "Xinjiang"), ("pa", "ਸ\u{a3c}ਿਨਚਿਆ\u{a02}ਙ"), ("pl", "Sinciang"), ("pt", "Xinjiang"), ("ro", "Xinjiang"), ("ru", "Синьцзян-Уйгурский автономный район"), ("si", "ක\u{dca}ස\u{dd2}න\u{dca}ග\u{dca}ජ\u{dd2}යන\u{dca}ග\u{dca}"), ("sk", "Sin-ťiang"), ("sq", "Sinkiang"), ("sr", "Сикјанг"), ("sr_Latn", "Sikjang"), ("sv", "Xinjiang"), ("ta", "சிஞ\u{bcd}சிய\u{bbe}ங\u{bcd}"), ("te", "జ\u{c3f}ంజ\u{c3f}య\u{c3e}ంగ\u{c4d}"), ("th", "เขตปกครองตนเองซ\u{e34}นเจ\u{e35}ยงอ\u{e38}ยก\u{e39}ร\u{e4c}"), ("tk", "Hinjiýan uýgur awtonom etraby"), ("tr", "Sincan Uygur Özerk Bölgesi"), ("uk", "Сіньцзян-Уйгурський автономний район"), ("ur", "سنکیانگ"), ("uz", "Sinszyan-Uygʻur muxtor rayoni"), ("vi", "Tân Cương"), ("yue", "新疆"), ("yue_Hans", "新疆"), ("zh", "新疆维吾尔自治区")]),
                        unofficial_name_list: ["Uighur", "Uygur"].to_vec(),
                    }
                ),
                (
                    "XZ",
                    Subdivision{
                        name: "XZ",
                        country_alpha2: Alpha2::CN,
                        code: "XZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.646923), longitude: Some(91.117212), max_latitude: Some(36.4833345), min_latitude: Some(26.8548157), max_longitude: Some(99.116241), min_longitude: Some(78.3955448)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tibet"), ("ar", "منطقة التبت ذاتية الحكم"), ("as", "তিব\u{9cd}বত স\u{9cd}ব\u{9be}য\u{9bc}ত\u{9cd}তশ\u{9be}সিত অঞ\u{9cd}চল"), ("az", "Tibet muxtar rayonu"), ("be", "Тыбецкі аўтаномны раён"), ("bg", "Тибетски автономен регион"), ("bn", "তিব\u{9cd}বত স\u{9cd}ব\u{9be}য\u{9bc}ত\u{9cd}তশ\u{9be}সিত অঞ\u{9cd}চল"), ("bs", "Tibet"), ("ca", "Regió Autònoma del Tibet"), ("ceb", "Tibet Autonomous Region"), ("cs", "Tibetská autonomní oblast"), ("cy", "Ardal hunanlywodraethol Tibet"), ("da", "Autonom Region Tibet"), ("de", "Autonomes Gebiet Tibet"), ("el", "Θιβέτ"), ("en", "Tibet"), ("es", "Tíbet"), ("et", "Tiibeti autonoomne piirkond"), ("eu", "Tibeteko eskualde autonomoa"), ("fa", "منطقه خودمختار تبت"), ("fi", "Tiibetin autonominen alue"), ("fr", "Région autonome du Tibet"), ("gl", "Rexión Autónoma do Tíbet"), ("he", "המחוז האוטונומי הטיבטי"), ("hi", "बोड स\u{94d}वायत\u{94d}त क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Tibet"), ("hu", "Tibeti Autonóm Terület"), ("hy", "Տիբեթի ինքնավար շրջան"), ("id", "Tibet"), ("is", "Tíbet"), ("it", "regione autonoma del Tibet"), ("ja", "チベット自治区"), ("ka", "ტიბეტის ავტონომიური რეგიონი"), ("ko", "티베트 자치구"), ("lt", "Tibeto autonominis regionas"), ("lv", "Tibetas autonomais reģions"), ("mk", "Тибет"), ("ml", "തിബെത\u{d4d}ത\u{d4d} സ\u{d4d}വയംഭരണപ\u{d4d}രദേശം"), ("mn", "Төвөдийн өөртөө засах орон"), ("mr", "तिब\u{947}ट स\u{94d}वायत\u{94d}त प\u{94d}रद\u{947}श"), ("ms", "Kawasan Autonomi Tibet"), ("nb", "Tibet"), ("nl", "Tibetaanse Autonome Regio"), ("no", "Tibet"), ("or", "ତ\u{b3f}ବ\u{b4d}ବତ"), ("pa", "ਤਿ\u{a71}ਬਤ ਖ\u{a3c}\u{a41}ਦਮ\u{a41}ਖ\u{a3c}ਤਿਆਰ ਸ\u{a42}ਬਾ"), ("pl", "Tybetański Region Autonomiczny"), ("pt", "Região Autônoma do Tibete"), ("ro", "Regiunea autonomă Tibet"), ("ru", "Тибетский автономный район"), ("sk", "Tibet"), ("sq", "Tibet"), ("sr", "Тибет"), ("sr_Latn", "Tibet"), ("sv", "Autonoma regionen Tibet"), ("ta", "திபெத\u{bcd} தன\u{bcd}ன\u{bbe}ட\u{bcd}சிப\u{bcd} பகுதி"), ("te", "ట\u{c3f}బ\u{c46}ట\u{c4d} స\u{c4d}వ\u{c3e}ధ\u{c3f}క\u{c3e}ర ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตปกครองตนเองท\u{e34}เบต"), ("tr", "Tibet Özerk Bölgesi"), ("uk", "Тибетський автономний район"), ("ur", "تبت خود مختار علاقہ"), ("uz", "Tibet muxtor rayoni"), ("vi", "Khu tự trị Tây Tạng"), ("yo", "Agbègbè Aladawa Tibet"), ("yo_BJ", "Agbègbè Aladawa Tibet"), ("yue", "西藏"), ("yue_Hans", "西藏"), ("zh", "西藏自治区")]),
                        unofficial_name_list: ["Tibet"].to_vec(),
                    }
                ),
                (
                    "YN",
                    Subdivision{
                        name: "YN",
                        country_alpha2: Alpha2::CN,
                        code: "YN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(25.045806), longitude: Some(102.710002), max_latitude: Some(29.2233271), min_latitude: Some(21.1417698), max_longitude: Some(106.1977227), min_longitude: Some(97.5279786)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Yunnan"), ("ar", "يونان"), ("be", "Правінцыя Юньнань"), ("bg", "Юнан"), ("bn", "ইউন\u{9be}ন"), ("ca", "Yunnan"), ("ceb", "Yunnan Sheng"), ("cs", "Jün-nan"), ("cy", "Yunnan"), ("da", "Yunnan"), ("de", "Yunnan"), ("el", "Γιουνάν"), ("en", "Yunnan"), ("es", "Yunnan"), ("et", "Yunnan"), ("eu", "Yunnan"), ("fa", "یون\u{200c}نان"), ("fi", "Yunnan"), ("fr", "Yunnan"), ("ga", "Yunnan"), ("gu", "ય\u{ac1}ન\u{acd}નન"), ("he", "יונאן"), ("hi", "य\u{941}न\u{94d}नान"), ("hr", "Yunnan"), ("hu", "Jünnan"), ("hy", "Յունան"), ("id", "Yunnan"), ("is", "Yunnan"), ("it", "Yunnan"), ("ja", "雲南省"), ("ka", "იუნანი"), ("kn", "ಯುನ\u{ccd}ನಾನ\u{ccd}"), ("ko", "윈난 성"), ("lt", "Junanas"), ("lv", "Juņnaņa"), ("mk", "Јунан"), ("ml", "യ\u{d41}ന\u{d4d}ന\u{d3e}ൻ"), ("mn", "Юньнань муж"), ("mr", "य\u{941}इन\u{94d}नान"), ("ms", "Yunan"), ("my", "ယ\u{1030}နန\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "Yunnan"), ("nl", "Yunnan"), ("no", "Yunnan"), ("pl", "Junnan"), ("pt", "Yunnan"), ("ro", "Yunnan"), ("ru", "Юньнань"), ("si", "ය\u{dd4}න\u{dcf}න\u{dca}"), ("sk", "Jün-nan"), ("sl", "Yunnan"), ("sr", "Јунан"), ("sr_Latn", "Junan"), ("sv", "Yunnan"), ("sw", "Yunnan"), ("ta", "யுன\u{bcd}ன\u{bbe}ன\u{bcd}"), ("te", "యున\u{c3e}న\u{c4d}"), ("th", "มณฑลย\u{e39}นนาน"), ("tr", "Yünnan"), ("uk", "Юньнань"), ("ur", "یوننان"), ("uz", "Yunnan"), ("vi", "Vân Nam"), ("yue", "雲南"), ("yue_Hans", "云南"), ("zh", "云南省")]),
                        unofficial_name_list: ["Yunnan"].to_vec(),
                    }
                ),
                (
                    "ZJ",
                    Subdivision{
                        name: "ZJ",
                        country_alpha2: Alpha2::CN,
                        code: "ZJ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.267443), longitude: Some(120.152792), max_latitude: Some(31.1787821), min_latitude: Some(27.041518), max_longitude: Some(122.9494378), min_longitude: Some(118.028279)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Zhejiang"), ("ar", "تشيجيانغ"), ("az", "Çjetszyan"), ("be", "Правінцыя Чжэцзян"), ("bg", "Джъдзян"), ("bn", "চেচিয\u{9bc}\u{9be}ং"), ("ca", "Zhejiang"), ("ceb", "Zhejiang Sheng"), ("cs", "Če-ťiang"), ("cy", "Zhejiang"), ("da", "Zhejiang"), ("de", "Zhejiang"), ("el", "Τσετσιάνγκ"), ("en", "Zhejiang"), ("es", "Zhejiang"), ("et", "Zhejiang"), ("eu", "Zhejiang"), ("fa", "چجیانگ"), ("fi", "Zhejiang"), ("fr", "Zhejiang"), ("ga", "Zhejiang"), ("gu", "ઝ\u{ac7}જીઆ\u{a82}ગ"), ("he", "ג׳ג׳יאנג"), ("hi", "झ\u{947}जिया\u{902}ग"), ("hr", "Zhejiang"), ("hu", "Csöcsiang"), ("hy", "Չժեցզյան"), ("id", "Zhejiang"), ("it", "Zhejiang"), ("ja", "浙江省"), ("ka", "ჯეძიანი"), ("kn", "ಝ\u{cc6}ಜ\u{cbf}ಯಾಂಗ\u{ccd}"), ("ko", "저장 성"), ("lt", "Džedziangas"), ("lv", "Džedzjana"), ("mk", "Џеѓанг"), ("mn", "Жөжян"), ("mr", "च-च\u{94d}या\u{902}ग"), ("ms", "Zhejiang"), ("nb", "Zhejiang"), ("ne", "झ\u{947}जिआङ"), ("nl", "Zhejiang"), ("no", "Zhejiang"), ("pl", "Zhejiang"), ("pt", "Zhejiang"), ("ro", "Zhejiang"), ("ru", "Чжэцзян"), ("si", "සෙජ\u{dd2}ය\u{dcf}න\u{dca}ග\u{dca}"), ("sk", "Če-ťiang"), ("sl", "Žeijang"), ("sr", "Џеђанг"), ("sr_Latn", "Džeđang"), ("sv", "Zhejiang"), ("sw", "Zhejiang"), ("ta", "செஜிய\u{bbe}ங\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "జ\u{c40}జంగ\u{c4d}"), ("th", "มณฑลเจ\u{e49}อเจ\u{e35}ยง"), ("tr", "Zhejiang"), ("uk", "Чжецзян"), ("ur", "ژجیانگ"), ("uz", "Chjetszyan"), ("vi", "Chiết Giang"), ("yue", "浙江"), ("yue_Hans", "浙江"), ("zh", "浙江省")]),
                        unofficial_name_list: ["Zhejiang"].to_vec(),
                    }
                ),
            ]

        )
    }
}
#[allow(unused_imports)]
use crate::{
    Alpha2, Alpha3, Continent, Country, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC,
    IOC,
};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "cn")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::CN,
        alpha3: Alpha3::CHN,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{postalcode}} {{city}} {{region_short}}\n{{country}}",
        ),
        continent: Continent::Asia,
        country_code: 86,
        currency_code: CurrencyCode::CNY,
        gec: Some(GEC::CH),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::CHN),
        iso_long_name: "The People's Republic of China",
        iso_short_name: "China",
        official_language_list: ["zh"].to_vec(),
        spoken_language_list: ["zh"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7, 8, 9, 10, 11].to_vec(),
        national_prefix: "0",
        nationality: Some("Chinese"),
        number: "156",
        postal_code: true,
        postal_code_format: Some("\\d{6}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternAsia),
        un_locode: "CN",
        unofficial_name_list: ["China", "Chine", "中国"].to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "China"),
            ("af", "Sjina"),
            ("ak", "China"),
            ("am", "ኂ፤ና"),
            ("an", "China"),
            ("ar", "الص\u{651}ين"),
            ("as", "চীন"),
            ("ay", "China"),
            ("az", "Çin"),
            ("ba", "China"),
            ("be", "Кітай"),
            ("bg", "Китай"),
            ("bi", "China"),
            ("bn", "চীন"),
            ("bn_IN", "চীন"),
            ("br", "Sina"),
            ("bs", "Kina"),
            ("ca", "Xina"),
            ("ce", "China"),
            ("ch", "China"),
            ("cs", "Čína"),
            ("cv", "China"),
            ("cy", "Tsieina"),
            ("da", "Kina"),
            ("de", "China"),
            ("dv", "China"),
            ("dz", "ཅ་ཡ\u{f7a}་ན།(ར\u{f92}\u{fb1}་ནག)"),
            ("ee", "China"),
            ("el", "Κίνα"),
            ("en", "China"),
            ("eo", "Ĉinio"),
            ("es", "China"),
            ("et", "Hiina"),
            ("eu", "Txina"),
            ("fa", "چین"),
            ("ff", "China"),
            ("fi", "Kiina"),
            ("fo", "Kina"),
            ("fr", "Chine"),
            ("fy", "China"),
            ("ga", "An tSín"),
            ("gl", "China"),
            ("gn", "China"),
            ("gu", "ચીન"),
            ("gv", "China"),
            ("ha", "China"),
            ("he", "סין"),
            ("hi", "चीन"),
            ("hr", "Kina"),
            ("ht", "Chin"),
            ("hu", "Kína"),
            ("hy", "Չինաստան"),
            ("ia", "China"),
            ("id", "Cina"),
            ("io", "China"),
            ("is", "Kína"),
            ("it", "Cina"),
            ("iu", "China"),
            ("ja", "中国"),
            ("ka", "ჩინეთი"),
            ("ki", "China"),
            ("kk", "Қытай"),
            ("kl", "China"),
            ("km", "ច\u{17b7}ន"),
            ("kn", "ಚೈನಾ"),
            ("ko", "중국"),
            ("ku", "Çîn"),
            ("kv", "China"),
            ("kw", "China"),
            ("ky", "Кытай"),
            ("lo", "China"),
            ("lt", "Kinija"),
            ("lv", "Ķīna"),
            ("mi", "Haina"),
            ("mk", "Кина"),
            ("ml", "ചൈന"),
            ("mn", "Хятад"),
            ("mr", "चीन"),
            ("ms", "China"),
            ("mt", "Ċina"),
            ("my", "China"),
            ("na", "China"),
            ("nb", "Kina"),
            ("ne", "चीन"),
            ("nl", "China"),
            ("nn", "Kina"),
            ("nv", "China"),
            ("oc", "China"),
            ("or", "ଚୀନ"),
            ("pa", "ਚੀਨ"),
            ("pi", "China"),
            ("pl", "Chiny"),
            ("ps", "چین"),
            ("pt", "China"),
            ("pt_BR", "China"),
            ("ro", "China"),
            ("ru", "Китай"),
            ("rw", "Ubushinwa"),
            ("sc", "Tzina"),
            ("sd", "China"),
            ("si", "ච\u{dd3}නය"),
            ("sk", "Čína"),
            ("sl", "Kitajska"),
            ("so", "Shiinaha"),
            ("sq", "Kinë"),
            ("sr", "Кина"),
            ("sv", "Kina"),
            ("sw", "China"),
            ("ta", "ச\u{bc0}ன\u{bbe}"),
            ("te", "చ\u{c48}న\u{c3e}"),
            ("tg", "Чин"),
            ("th", "จ\u{e35}น"),
            ("ti", "ቻይና"),
            ("tk", "China"),
            ("tl", "Tsina"),
            ("tr", "Çin"),
            ("tt", "Чин"),
            ("ug", "جۇڭگو"),
            ("uk", "Китай"),
            ("ur", "China"),
            ("uz", "China"),
            ("ve", "China"),
            ("vi", "Trung Quốc"),
            ("wa", "Chine"),
            ("wo", "Ciin"),
            ("xh", "Tshayina"),
            ("yo", "China"),
            ("zh_CN", "中国"),
            ("zh_HK", "中國"),
            ("zh_TW", "中國"),
            ("zu", "Isi-Shayina"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
