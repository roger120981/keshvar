// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Kingdom of Bhutan

#[cfg(all(feature = "bt", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::BT;
    pub const ALPHA3: Alpha3 = Alpha3::BTN;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 975;
    pub const CURRENCY_CODE: &str = "BTN";
    pub const GEC: Option<GEC> = Some(GEC::BT);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("BHU");
    pub const ISO_SHORT_NAME: &str = "Bhutan";
    pub const ISO_LONG_NAME: &str = "The Kingdom of Bhutan";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["dz"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["dz"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7, 8];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Bhutanese");
    pub const NUMBER: &str = "064";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthernAsia);
    pub const UN_LOCODE: &str = "BT";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Bhutan", "Bhoutan", "Bután", "ブータン"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Bhutan"),
        ("af", "Bhoetan"),
        ("ak", "Bhutan"),
        ("am", "ቡህታን"),
        ("an", "Bután"),
        ("ar", "بوتان"),
        ("as", "ভ\u{9c1}ট\u{9be}ন"),
        ("ay", "Bhutan"),
        ("az", "Butan"),
        ("ba", "Bhutan"),
        ("be", "Бутан"),
        ("bg", "Бутан"),
        ("bi", "Bhutan"),
        ("bn", "ভ\u{9c1}ট\u{9be}ন"),
        ("bn_IN", "ভ\u{9c1}ট\u{9be}ন"),
        ("br", "Bhoutan"),
        ("bs", "Butan"),
        ("ca", "Bhutan"),
        ("ce", "Бутан"),
        ("ch", "Bhutan"),
        ("cs", "Bhútán"),
        ("cv", "Бутан"),
        ("cy", "Bhutan"),
        ("da", "Bhutan"),
        ("de", "Bhutan"),
        ("dv", "ބ\u{7ab}ޓ\u{7a7}ނ\u{7b0}"),
        ("dz", "འབ\u{fb2}\u{f74}ག།"),
        ("ee", "Bhutan"),
        ("el", "Μπουτάν"),
        ("en", "Bhutan"),
        ("eo", "Butano"),
        ("es", "Bután"),
        ("et", "Bhutan"),
        ("eu", "Bhutan"),
        ("fa", "بوتان"),
        ("ff", "Bhutan"),
        ("fi", "Bhutan"),
        ("fo", "Butan"),
        ("fr", "Bhoutan"),
        ("fy", "Bûtan"),
        ("ga", "An Bhútáin"),
        ("gl", "Bután"),
        ("gn", "Bhutan"),
        ("gu", "ભ\u{ac1}ટાન"),
        ("gv", "Yn Vutaan"),
        ("ha", "Bhutan"),
        ("he", "בהוטן"),
        ("hi", "भ\u{942}टान"),
        ("hr", "Butan"),
        ("ht", "Boutan"),
        ("hu", "Bhután"),
        ("hy", "Բութան"),
        ("ia", "Bhutan"),
        ("id", "Bhutan"),
        ("io", "Bhutan"),
        ("is", "Bútan"),
        ("it", "Bhutan"),
        ("iu", "Bhutan"),
        ("ja", "ブータン"),
        ("ka", "ბუტანი"),
        ("ki", "Bhutan"),
        ("kk", "Бутан"),
        ("kl", "Bhutan"),
        ("km", "ប\u{17ca}\u{17bc}តាន"),
        ("kn", "ಭ\u{cc2}ತಾನ\u{ccd}"),
        ("ko", "부탄"),
        ("ku", "Bûtan"),
        ("kv", "Бутан"),
        ("kw", "Bhoutan"),
        ("ky", "Бутан"),
        ("lo", "Bhutan"),
        ("lt", "Butanas"),
        ("lv", "Butāna"),
        ("mi", "Bhutan"),
        ("mk", "Бутан"),
        ("ml", "ബ\u{d42}ട\u{d4d}ട\u{d3e}ന\u{d4d}\u{200d}"),
        ("mn", "Бутан"),
        ("mr", "भ\u{942}तान"),
        ("ms", "Bhutan"),
        ("mt", "Butan"),
        (
            "my",
            "ဘ\u{1030}တန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Butan"),
        ("nb", "Bhutan"),
        ("ne", "भ\u{941}टान"),
        ("nl", "Bhutan"),
        ("nn", "Bhutan"),
        ("nv", "Iiʼniʼ Tłʼiishtsoh Bikéyah"),
        ("oc", "Botan"),
        ("or", "ଭ\u{b41}ଟ\u{b3e}ନ"),
        ("pa", "ਭ\u{a41}ਟਾਨ"),
        ("pi", "भ\u{942}टान"),
        ("pl", "Bhutan"),
        ("ps", "بوتان"),
        ("pt", "Butão"),
        ("pt_BR", "Butão"),
        ("ro", "Bhutan"),
        ("ru", "Бутан"),
        ("rw", "Butani"),
        ("sc", "Bhutàn"),
        ("sd", "Bhutan"),
        ("si", "භ\u{dd6}ත\u{dcf}නය"),
        ("sk", "Bhután"),
        ("sl", "Butan"),
        ("so", "Butaan"),
        ("sq", "Butan"),
        ("sr", "Бутан"),
        ("sv", "Bhutan"),
        ("sw", "Bhutan"),
        ("ta", "பூட\u{bcd}ட\u{bbe}ன\u{bcd}"),
        ("te", "భూట\u{c3e}న\u{c4d}"),
        ("tg", "Бутон"),
        ("th", "ภ\u{e39}ฏาน"),
        ("ti", "ቡህታን"),
        ("tk", "Butan"),
        ("tl", "Bhutan"),
        ("tr", "Bhutan"),
        ("tt", "Бутан"),
        ("ug", "بۇتان"),
        ("uk", "Бутан"),
        ("ur", "بھوٹان"),
        ("uz", "Butan"),
        ("ve", "Bhutan"),
        ("vi", "Bu-thănh"),
        ("wa", "Boutan"),
        ("wo", "Butaan"),
        ("xh", "Bhutan"),
        ("yo", "Bhùtán"),
        ("zh_CN", "不丹"),
        ("zh_HK", "不丹"),
        ("zh_TW", "不丹"),
        ("zu", "Bhutan"),
    ];
    #[cfg(all(feature = "bt", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 27.514162;
        pub const LONGITUDE: f64 = 90.433601;
        pub const MAX_LATITUDE: f64 = 28.246987;
        pub const MAX_LONGITUDE: f64 = 92.125232;
        pub const MIN_LATITUDE: f64 = 26.702016;
        pub const MIN_LONGITUDE: f64 = 88.7464739;
        pub const NORTHEAST_LATITUDE: f64 = 28.246987;
        pub const NORTHEAST_LONGITUDE: f64 = 92.125232;
        pub const SOUTHWEST_LATITUDE: f64 = 26.702016;
        pub const SOUTHWEST_LONGITUDE: f64 = 88.7464739;
    }
}
#[cfg(all(feature = "bt", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 27.514162,
            longitude: 90.433601,
            max_latitude: 28.246987,
            max_longitude: 92.125232,
            min_latitude: 26.702016,
            min_longitude: 88.7464739,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 28.246987,
                    longitude: 92.125232,
                },
                southwest: CountryGeoBound {
                    latitude: 26.702016,
                    longitude: 88.7464739,
                },
            },
        }
    }
}

#[cfg(all(feature = "bt", feature = "subdivisions"))]
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
                    "11",
                    Subdivision{
                        name: "11",
                        country_alpha2: Alpha2::BT,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.4285949), longitude: Some(89.4166516), max_latitude: Some(27.7510569), min_latitude: Some(27.1771741), max_longitude: Some(89.563836), min_longitude: Some(89.131271)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بارو"), ("bg", "Паро"), ("bn", "প\u{9be}রো জেল\u{9be}"), ("ca", "Districte de Paro"), ("ccp", "𑄛𑄢\u{1112e}"), ("ceb", "Paro Dzongkhag"), ("da", "Paro District"), ("de", "Paro"), ("el", "Πάρο"), ("en", "Paro"), ("es", "Distrito de Paro"), ("et", "Paro ringkond"), ("fa", "استان پارو"), ("fi", "Paro"), ("fr", "Paro"), ("gu", "પારો જિલ\u{acd}લો"), ("hi", "पारो जिला"), ("hu", "Paro körzet"), ("id", "Distrik Paro"), ("it", "distretto di Paro"), ("ja", "パロ県"), ("kn", "ಪ\u{ccd}ಯಾರೊ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "파로 현"), ("lt", "Paro apskritis"), ("lv", "Paro distrikts"), ("ml", "പ\u{d3e}റോ ജില\u{d4d}ല"), ("mr", "पारो जिल\u{94d}हा"), ("ms", "Paro District"), ("nb", "Paro"), ("nl", "Paro"), ("no", "Paro"), ("pa", "ਪਾਰ\u{a4b} ਜ\u{a3c}ਿਲ\u{a4d}ਹਾ"), ("pl", "Dystrykt Paro"), ("ps", "پارو ولسوالۍ"), ("pt", "Paro"), ("ro", "Paro"), ("ru", "Паро"), ("si", "ප\u{dcf}රෝ ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sl", "Okraj Paro"), ("sr", "Паро"), ("sr_Latn", "Paro"), ("sv", "Paro"), ("ta", "ப\u{bbe}ரோ ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ప\u{c3e}ర\u{c4b} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "มณฑลพาโร"), ("tr", "Paro"), ("uk", "Паро"), ("ur", "پارو ضلع"), ("vi", "Paro"), ("zh", "帕罗宗")]),
                        unofficial_name_list: ["Paro", "Rinpung"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "12",
                        country_alpha2: Alpha2::BT,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.0522919), longitude: Some(89.5756987), max_latitude: Some(27.0611895), min_latitude: Some(27.0483479), max_longitude: Some(89.5806313), min_longitude: Some(89.5604609)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تشوخا"), ("bg", "Чукха"), ("bn", "চ\u{9c1}খ\u{9be} জেল\u{9be}"), ("ca", "Districte de Chukha"), ("ccp", "𑄌\u{1112a}𑄈"), ("ceb", "Chhukha Dzongkhag"), ("cs", "Chhukha"), ("da", "Chukha"), ("de", "Chukha"), ("el", "Τσούκα"), ("en", "Chukha"), ("es", "Distrito de Chukha"), ("et", "Chhukha ringkond"), ("fa", "استان چوخا"), ("fi", "Chukha"), ("fr", "Chukha"), ("gu", "ચ\u{ac1}ખા જિલ\u{acd}લો"), ("hi", "च\u{941}खा जिला"), ("hu", "Chukha körzet"), ("id", "Distrik Chukha"), ("it", "distretto di Chukha"), ("ja", "チュカ県"), ("kn", "ಚುಖಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "추카 현"), ("lt", "Čukos apskritis"), ("lv", "Čhukhas distrikts"), ("mr", "च\u{941}खा जिल\u{94d}हा"), ("ms", "Chukha District"), ("nb", "Chukha"), ("nl", "Chukha"), ("no", "Chukha"), ("pl", "Dystrykt Chukha"), ("ps", "چوکها ولسوالۍ"), ("pt", "Chukha"), ("ro", "Districtul Chukha"), ("ru", "Чукха"), ("si", "ච\u{dd4}න\u{dca}ඛ\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Čhukha"), ("sr", "Чукха"), ("sr_Latn", "Čukha"), ("sv", "Chukha"), ("ta", "சுக\u{bcd}க\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "చుఖ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เจคช\u{e39}คาร\u{e4c}"), ("tr", "Chuka District"), ("uk", "Чукха"), ("ur", "چوکہا ضلع"), ("vi", "Quận Chukha"), ("zh", "楚卡宗")]),
                        unofficial_name_list: ["Chhuka", "Chuka", "Chukha"].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "13",
                        country_alpha2: Alpha2::BT,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.2651669), longitude: Some(89.1705998), max_latitude: Some(27.6211449), min_latitude: Some(27.0723781), max_longitude: Some(89.39598099999999), min_longitude: Some(88.89505199999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ها"), ("bg", "Хаа"), ("bn", "হ\u{9be} জেল\u{9be}"), ("ca", "Districte de Haa"), ("ccp", "𑄦"), ("ceb", "Haa Dzongkhag"), ("da", "Haa District"), ("de", "Haa"), ("el", "Χαά"), ("en", "Haa"), ("es", "Distrito de Haa"), ("et", "Ha ringkond"), ("fa", "استان ها"), ("fi", "Haa"), ("fr", "Haa"), ("gu", "હા જિલ\u{acd}લો"), ("hi", "हा जिला"), ("hu", "Haa körzet"), ("id", "Distrik Haa"), ("it", "distretto di Haa"), ("ja", "ハ県"), ("kn", "ಹಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "하 현"), ("lt", "Haa apskritis"), ("lv", "Ha distrikts"), ("mr", "हा जिल\u{94d}हा"), ("ms", "Haa District"), ("nb", "Haa"), ("nl", "Haa"), ("no", "Haa"), ("pl", "Dystrykt Haa"), ("ps", "ها ولسوالۍ"), ("pt", "Haa"), ("ro", "Haa"), ("ru", "Ха"), ("si", "හ\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Ха"), ("sr_Latn", "Ha"), ("sv", "Haa"), ("ta", "ஹ\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "హ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตฮา"), ("tr", "Haa District"), ("uk", "Хаа"), ("ur", "ہا ضلع"), ("vi", "Quận Haa"), ("zh", "哈阿宗")]),
                        unofficial_name_list: ["Ha", "Haa"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::BT,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(26.9131041), longitude: Some(89.08360189999999), max_latitude: Some(26.9162536), min_latitude: Some(26.9078347), max_longitude: Some(89.090538), min_longitude: Some(89.0766335)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سامتسي"), ("bg", "Самце"), ("bn", "স\u{9be}মটসে জেল\u{9be}"), ("ca", "Districte de Samtse"), ("ccp", "𑄥𑄟\u{11134}𑄥𑄬"), ("ceb", "Samtse Dzongkhag"), ("da", "Samtse District"), ("de", "Samtse"), ("el", "Σάμτσε"), ("en", "Samtse"), ("es", "Distrito de Samtse"), ("et", "Samtse ringkond"), ("fa", "استان سامتس"), ("fi", "Samtse"), ("fr", "Samtse"), ("gu", "સમત\u{acd}સ\u{ac7} જિલ\u{acd}લો"), ("hi", "साम\u{94d}त\u{94d}स\u{947} जिला"), ("hu", "Samtse körzet"), ("id", "Distrik Samtse"), ("it", "distretto di Samtse"), ("ja", "サムツェ県"), ("kn", "ಸ\u{ccd}ಯಾಮ\u{ccd}ಸ\u{cc6} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "삼체 현"), ("lt", "Samtsės apskritis"), ("lv", "Samcī distrikts"), ("mr", "समष\u{94d}टी जिल\u{94d}हा"), ("ms", "Samtse District"), ("nb", "Samtse"), ("nl", "Samtse"), ("no", "Samtse"), ("pl", "Dystrykt Samtse"), ("ps", "سامتسی ولسوالۍ"), ("pt", "Samtse"), ("ro", "Districtul Samtse"), ("ru", "Самце"), ("si", "සම\u{dca}ට\u{dca}සේ ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Самце"), ("sr_Latn", "Samce"), ("sv", "Samtse"), ("ta", "சமட\u{bcd}ஸ\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "స\u{c3e}మ\u{c4d}ట\u{c4d}స\u{c47} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "มณฑลซ\u{e31}มช\u{e34}"), ("tr", "Samtse District"), ("uk", "Самце"), ("ur", "سامتسے ضلع"), ("vi", "Quận Samtse"), ("zh", "萨姆宗")]),
                        unofficial_name_list: ["Samchi", "Samtse"].to_vec(),
                    }
                ),
                (
                    "15",
                    Subdivision{
                        name: "15",
                        country_alpha2: Alpha2::BT,
                        code: "15",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.4727924), longitude: Some(89.6392863), max_latitude: Some(27.5336949), min_latitude: Some(27.4251092), max_longitude: Some(89.6726418), min_longitude: Some(89.60414879999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ثيمفو"), ("bg", "Тхимпху"), ("bn", "থিম\u{9cd}পো জেল\u{9be}"), ("ca", "Districte de Thimphu"), ("ccp", "𑄗\u{11128}𑄟\u{11134}𑄜\u{1112a}"), ("ceb", "Thimphu Dzongkhag"), ("da", "Thimphu District"), ("de", "Thimphu"), ("el", "Τίμφου"), ("en", "Thimphu"), ("es", "Distrito de Timbu"), ("et", "Thimphu ringkond"), ("fa", "استان تیمفو"), ("fi", "Thimphu"), ("fr", "Thimphou"), ("gu", "થિમ\u{acd}ફ\u{ac1} જિલ\u{acd}લો"), ("hi", "थिम\u{94d}फ\u{942} जिला"), ("hu", "Thimphu körzet"), ("id", "Distrik Thimphu"), ("it", "distretto di Thimphu"), ("ja", "ティンプー県"), ("kn", "ತ\u{cbf}ಮ\u{ccd}ಮು ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "팀부 현"), ("lt", "Timfu regionas"), ("lv", "Thimphu distrikts"), ("mn", "Тхимпху зонкаг"), ("mr", "थि\u{902}प\u{942} जिल\u{94d}हा"), ("ms", "Thimphu District"), ("nb", "Thimphu distrikt"), ("ne", "थिम\u{94d}फ\u{942} जिल\u{94d}ला"), ("nl", "Thimphu"), ("no", "Thimphu distrikt"), ("pl", "Thimphu"), ("ps", "تھمپھو ولسوالۍ"), ("pt", "Thimphu"), ("ro", "Districtul Thimphu"), ("ru", "Тхимпху"), ("si", "ත\u{dd2}ම\u{dca}ප\u{dd4} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sl", "Okraj Thimphu"), ("sr", "Тимбу"), ("sr_Latn", "Timbu"), ("sv", "Thimphu"), ("ta", "த\u{bc0}ம\u{bcd}ப\u{bcd}ஹு ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "థ\u{c3f}ంపు జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ท\u{e34}มพ\u{e39}"), ("tr", "Thimpu District"), ("uk", "Тхімпху"), ("ur", "تھمپھو ضلع"), ("vi", "Quận Thimphu"), ("zh", "廷布宗")]),
                        unofficial_name_list: ["Thimbu", "Thimphu", "Thimpu", "Timbhu", "Timbu", "Timphu"].to_vec(),
                    }
                ),
                (
                    "21",
                    Subdivision{
                        name: "21",
                        country_alpha2: Alpha2::BT,
                        code: "21",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.032207), longitude: Some(90.1869644), max_latitude: Some(27.177361), min_latitude: Some(26.8187539), max_longitude: Some(90.357412), min_longitude: Some(90.00392509999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Циранг"), ("ca", "Districte de Tsirang"), ("ccp", "𑄥\u{11128}𑄢\u{11127}\u{11101}"), ("ceb", "Tsirang"), ("de", "Tsirang"), ("en", "Tsirang"), ("es", "Distrito de Tsirang"), ("et", "Tsirangi ringkond"), ("fi", "Tsirang"), ("fr", "Tsirang"), ("hu", "Tsirang körzet"), ("it", "distretto di Tsirang"), ("ja", "チラン県"), ("ko", "치랑 현"), ("nb", "Tsirang"), ("nl", "Tsirang"), ("no", "Tsirang"), ("pl", "Dystrykt Tsirang"), ("ps", "تسيرانگ ولسوالۍ"), ("pt", "Tsirang"), ("ro", "Tsirang"), ("ru", "Циранг"), ("sv", "Tsirang"), ("uk", "Ціранг"), ("ur", "تسیرانگ ضلع"), ("zh", "奇朗宗")]),
                        unofficial_name_list: ["Chirang", "Tsirang"].to_vec(),
                    }
                ),
                (
                    "22",
                    Subdivision{
                        name: "22",
                        country_alpha2: Alpha2::BT,
                        code: "22",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.0322861), longitude: Some(89.8879304), max_latitude: Some(27.26369), min_latitude: Some(26.808901), max_longitude: Some(90.08144100000001), min_longitude: Some(89.648614)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Дагана"), ("ca", "Districte de Dagana"), ("ccp", "𑄓𑄉𑄚"), ("ceb", "Dagana Dzongkhag"), ("de", "Dagana"), ("en", "Dagana"), ("es", "Distrito de Dagana"), ("et", "Dagana ringkond"), ("fa", "استان داگانا"), ("fi", "Dagana"), ("fr", "Dagana"), ("hu", "Dagana körzet"), ("it", "distretto di Dagana"), ("ja", "ダガナ県"), ("ko", "다가나 현"), ("nb", "Dagana"), ("nl", "Dagana"), ("no", "Dagana"), ("or", "ଦ\u{b3e}ଗନ\u{b3e} ଜ\u{b3f}ଲ\u{b4d}ଲ\u{b3e} , ଭ\u{b41}ଟ\u{b3e}ନ"), ("pl", "Dystrykt Dagana"), ("ps", "داگانا ولسوالۍ"), ("pt", "Dagana"), ("ro", "Dagana"), ("ru", "Дагана"), ("sv", "Dagana"), ("uk", "Дагана"), ("ur", "داگانا ضلع"), ("zh", "达加纳宗")]),
                        unofficial_name_list: ["Daga", "Dagana"].to_vec(),
                    }
                ),
                (
                    "23",
                    Subdivision{
                        name: "23",
                        country_alpha2: Alpha2::BT,
                        code: "23",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.5920869), longitude: Some(89.87974589999999), max_latitude: Some(27.5961249), min_latitude: Some(27.5877193), max_longitude: Some(89.88554479999999), min_longitude: Some(89.8721123)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بوناخا"), ("bg", "Пунака"), ("bn", "পোন\u{9be}ক\u{9be} জেল\u{9be}"), ("ca", "Districte de Punakha"), ("ccp", "𑄛\u{1112a}𑄚𑄇\u{11134}"), ("ceb", "Punakha Dzongkhag"), ("da", "Punakha District"), ("de", "Punakha"), ("el", "Πουνάκχα"), ("en", "Punakha"), ("es", "Distrito de Punakha"), ("et", "Punakha ringkond"), ("fi", "Punakha"), ("fr", "Punakha"), ("gu", "પનાખા જિલ\u{acd}લો"), ("hi", "प\u{941}नाखा जिला"), ("hu", "Punakha körzet"), ("id", "Distrik Punakha"), ("it", "distretto di Punakha"), ("ja", "プナカ県"), ("kn", "ಪುನಾಖ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "푸나카 현"), ("lt", "Punakos apskritis"), ("lv", "Punakhas distrikts"), ("mr", "पन\u{94d}हाखा जिल\u{94d}हा"), ("ms", "Punakha District"), ("nb", "Punakha"), ("nl", "Punakha"), ("no", "Punakha"), ("pl", "Dystrykt Punakha"), ("ps", "پوناخا ولسوالۍ"), ("pt", "Punakha"), ("ro", "Punakha"), ("ru", "Пунакха"), ("si", "ප\u{dd6}නක\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sl", "Okraj Punaka"), ("sr", "Пунака"), ("sr_Latn", "Punaka"), ("sv", "Punakha"), ("ta", "புனக\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "పున\u{c3e}ఖ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "มณฑลพ\u{e39}นาคา"), ("tr", "Punakha District"), ("uk", "Пунакха"), ("ur", "پوناخا ضلع"), ("vi", "Quận Punakha"), ("zh", "普那卡宗")]),
                        unofficial_name_list: ["Punakha"].to_vec(),
                    }
                ),
                (
                    "24",
                    Subdivision{
                        name: "24",
                        country_alpha2: Alpha2::BT,
                        code: "24",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.4879018), longitude: Some(89.8996196), max_latitude: Some(27.4991359), min_latitude: Some(27.474542), max_longitude: Some(89.9055432), min_longitude: Some(89.8926686)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة وانغدو فودرانغ"), ("bg", "Вангдуе Пходранг"), ("bn", "ওয\u{9bc}\u{9be}ংড\u{9c1} পোড\u{9cd}র\u{9be}ং জেল\u{9be}"), ("ca", "Districte de Wangdue Phodrang"), ("ccp", "𑄃\u{11127}𑄠\u{11101}𑄓\u{1112a}𑄠𑄬 𑄜\u{11127}𑄖\u{11134}𑄢\u{11127}\u{11101}"), ("ceb", "Wangdue Phodrang Dzongkhag"), ("da", "Wangdue Phodrang District"), ("de", "Wangdue Phodrang"), ("el", "Γουάνγκντουε Φόντρανγκ"), ("en", "Wangdue Phodrang"), ("es", "Distrito de Wangdue Phodrang"), ("et", "Wangdue Phodrangi ringkond"), ("fi", "Wangdue Phodrang"), ("fr", "Wangdue Phodrang"), ("gu", "વા\u{a82}ગડ\u{ac1} ફોડ\u{acd}રા\u{a82}ગ જિલ\u{acd}લો"), ("hi", "वा\u{902}गद\u{94d}य\u{942} फोद\u{94d}रा\u{902}ग जिला"), ("hu", "Wangdue Phodrang körzet"), ("id", "Distrik Wangdue Phodrang"), ("it", "distretto di Wangdue Phodrang"), ("ja", "ワンデュ・ポダン県"), ("kn", "ವಾಂಗ\u{ccd}ಡು ಫೊಡ\u{ccd}ರಾಂಗ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "왕두에포드랑 현"), ("lt", "Vangde Podrango apskritis"), ("lv", "Vangdi-Podrangas distrikts"), ("mr", "वा\u{902}गद\u{941} फोडर\u{902}ग जिल\u{94d}हा"), ("ms", "Wangdue Phodrang District"), ("nb", "Wangdue Phodrang"), ("nl", "Wangdue Phodrang"), ("no", "Wangdue Phodrang"), ("pl", "Dystrykt Wangdue Phodrang"), ("ps", "وانگدو پھودرانگ ولسوالۍ"), ("pt", "Wangdue Phodrang"), ("ro", "Wangdue Phodrang"), ("ru", "Вангди-Пходранг"), ("si", "වන\u{dca}ග\u{dca}ඩ\u{dd4} පොඩ\u{dca}ර\u{dcf}න\u{dca}ග\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sl", "Okraj Vangdi Fodrang"), ("sr", "Вангдуе Пходранг"), ("sr_Latn", "Vangdue Phodrang"), ("sv", "Wangdue Phodrang"), ("ta", "வ\u{bbe}ங\u{bcd}கித\u{bcd}தேயு போட\u{bcd}ரங\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "వ\u{c3e}ంగ\u{c4d}డ\u{c4d}యూ ఫ\u{c4b}డ\u{c4d}ర\u{c3e}ంగ\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตว\u{e31}งด\u{e35}โพดร\u{e31}ง"), ("tr", "Wangdue Phodrang"), ("uk", "Вангді-Пходранг"), ("ur", "وانگدو پھودرانگ ضلع"), ("vi", "Quận Wangdue Phodrang"), ("zh", "旺杜波德朗宗")]),
                        unofficial_name_list: ["Wangdi Phodrang", "Wangdiphodrang", "Wangdue", "Wangdue Phodrang"].to_vec(),
                    }
                ),
                (
                    "31",
                    Subdivision{
                        name: "31",
                        country_alpha2: Alpha2::BT,
                        code: "31",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(26.9373041), longitude: Some(90.4879916), max_latitude: Some(27.209321), min_latitude: Some(26.7076319), max_longitude: Some(90.96380099999999), min_longitude: Some(89.734461)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Сарпанг"), ("ca", "Districte de Sarpang"), ("ccp", "𑄥𑄢\u{11134}𑄛\u{11127}\u{11101}"), ("ceb", "Sarpang Dzongkhag"), ("de", "Sarpang"), ("en", "Sarpang"), ("es", "Distrito de Sarpang"), ("et", "Sarpangi ringkond"), ("fi", "Sarpang"), ("fr", "Sarpang"), ("hu", "Sarpang körzet"), ("it", "distretto di Sarpang"), ("ja", "サルパン県"), ("ko", "사르팡 현"), ("nb", "Sarpang"), ("nl", "Sarpang"), ("no", "Sarpang"), ("pl", "Dystrykt Sarpang"), ("ps", "سارپانگ ولسوالۍ"), ("pt", "Sarpang"), ("ro", "Sarpang"), ("ru", "Сарпанг"), ("sr", "Сарпанг"), ("sr_Latn", "Sarpang"), ("sv", "Sarpang"), ("uk", "Сарпанг"), ("ur", "سارپانگ ضلع"), ("zh", "盖莱普宗")]),
                        unofficial_name_list: ["Gaylegphug", "Geylegphug", "Sarbhang", "Sarpang"].to_vec(),
                    }
                ),
                (
                    "32",
                    Subdivision{
                        name: "32",
                        country_alpha2: Alpha2::BT,
                        code: "32",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.5002269), longitude: Some(90.5080634), max_latitude: Some(27.508043), min_latitude: Some(27.4940347), max_longitude: Some(90.51571369999999), min_longitude: Some(90.5036116)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ترونغزا"), ("bg", "Тронгса"), ("bn", "ট\u{9cd}রোনস\u{9be} জেল\u{9be}"), ("ccp", "𑄑\u{11133}𑄢\u{11127}\u{11101}𑄥"), ("ceb", "Trongsa Dzongkhag"), ("da", "Trongsa District"), ("de", "Trongsa"), ("el", "Τρόνγκσα"), ("en", "Trongsa"), ("es", "Distrito de Trongsa"), ("et", "Trongsa ringkond"), ("fi", "Trongsa"), ("fr", "Trongsa"), ("gu", "ટ\u{acd}રોન\u{acd}ગ\u{acd}સા જિલ\u{acd}લો"), ("hi", "ट\u{94d}रो\u{902}गसा जिला"), ("hu", "Trongsa körzet"), ("id", "Distrik Trongsa"), ("it", "distretto di Trongsa"), ("ja", "トンサ県"), ("kn", "ಟ\u{ccd}ರಾಂಗ\u{ccd}ಸಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "트롱사 현"), ("lt", "Trongsos apskritis"), ("lv", "Tongsā distrikts"), ("mr", "ट\u{94d}रॉन\u{94d}ज जिल\u{94d}हा"), ("ms", "Trongsa District"), ("nb", "Trongsa"), ("nl", "Trongsa"), ("no", "Trongsa"), ("pl", "Dystrykt Trongsa"), ("ps", "ترونگسا ولسوالۍ"), ("pt", "Trongsa"), ("ro", "Trongsa"), ("ru", "Тронгса"), ("si", "ට\u{dca}\u{200d}රොන\u{dca}ග\u{dca}ස\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Trongsa"), ("ta", "ட\u{bcd}ரோங\u{bcd}ஸ\u{bcd}ச\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ట\u{c4d}ర\u{c3e}ంగ\u{c4d}స\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "มณฑลตงซา"), ("tr", "Trongsa District"), ("uk", "Тронгса"), ("ur", "ترونگسا ضلع"), ("vi", "Quận Trongsa"), ("zh", "通萨宗")]),
                        unofficial_name_list: ["Tongsa", "Trongsa"].to_vec(),
                    }
                ),
                (
                    "33",
                    Subdivision{
                        name: "33",
                        country_alpha2: Alpha2::BT,
                        code: "33",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.641839), longitude: Some(90.6773046), max_latitude: Some(28.090851), min_latitude: Some(27.320946), max_longitude: Some(91.0160089), min_longitude: Some(90.48443499999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بامثانغ"), ("be", "Бумтанг"), ("bg", "Бумтанг"), ("bn", "বোমত\u{9be}ং জেল\u{9be}"), ("ca", "Districte de Bumthang"), ("ccp", "𑄝\u{1112a}𑄟\u{11134}𑄗\u{11101}"), ("ceb", "Bumthang Dzongkhag"), ("cs", "Bumthang"), ("da", "Bumthang District"), ("de", "Bumthang"), ("el", "Μπουμθάνγκ"), ("en", "Bumthang"), ("es", "Distrito de Bumthang"), ("et", "Bumthangi ringkond"), ("fa", "استان بومتهنگ"), ("fi", "Bumthang"), ("fr", "Bumthang"), ("gu", "બ\u{a82}થા\u{a82}ગ જિલ\u{acd}લો"), ("hi", "ब\u{941}मथा\u{902}ग जिला"), ("hu", "Bumthang körzet"), ("id", "Distrik Bumthang"), ("it", "distretto di Bumthang"), ("ja", "ブムタン県"), ("kn", "ಬುಮ\u{ccd}ತಂಗ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "붐탕 현"), ("lt", "Bumtango rajonas"), ("lv", "Bumtangas distrikts"), ("mr", "बमथा\u{902}ग जिल\u{94d}हा"), ("ms", "Bumthang District"), ("nb", "Bumthang"), ("nl", "Bumthang"), ("no", "Bumthang"), ("pl", "Dystrykt Bumthang"), ("ps", "بومتھانگ ولسوالۍ"), ("pt", "Bumthang"), ("ro", "Bhumthang"), ("ru", "Бумтанг"), ("si", "බ\u{dd4}ම\u{dca}ත\u{dcf}න\u{dca}ග\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Bumthang"), ("sr", "Бумтанг"), ("sr_Latn", "Bumtang"), ("sv", "Bumthang"), ("ta", "பும\u{bcd}த\u{bbe}ங\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "బుమ\u{c4d}త\u{c3e}ంగ\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "มณฑลบ\u{e38}มท\u{e31}ง"), ("tr", "Bumthang District"), ("uk", "Бумтанг"), ("ur", "بومتھانگ ضلع"), ("vi", "Bumthang"), ("zh", "布姆唐宗")]),
                        unofficial_name_list: ["Bumthang"].to_vec(),
                    }
                ),
                (
                    "34",
                    Subdivision{
                        name: "34",
                        country_alpha2: Alpha2::BT,
                        code: "34",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.1439047), longitude: Some(90.6903529), max_latitude: Some(27.1492833), min_latitude: Some(27.1353061), max_longitude: Some(90.69565779999999), min_longitude: Some(90.6856155)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة زيمغانغ"), ("bg", "Зхемганг"), ("bn", "জেমগ\u{9be}ং জেল\u{9be}"), ("ca", "Districte de Zhemgang"), ("ccp", "𑄏𑄬𑄟\u{11134}𑄉\u{11127}\u{11101}"), ("ceb", "Zhemgang Dzongkhag"), ("da", "Zhemgang District"), ("de", "Zhemgang"), ("el", "Ζεμγκάνγκ"), ("en", "Zhemgang"), ("es", "Distrito de Zhemgang"), ("et", "Zhemgangi ringkond"), ("fi", "Zhemgang"), ("fr", "Zhemgang"), ("gu", "ઝ\u{ac7}મગા\u{a82}ગ જિલ\u{acd}લો"), ("hi", "ज\u{93c}\u{947}मग\u{948}\u{902}ग जिला"), ("hu", "Zhemgang körzet"), ("id", "Distrik Zhemgang"), ("it", "distretto di Zhemgang"), ("ja", "シェムガン県"), ("kn", "ಝ\u{cc6}ಮ\u{cc6}ಗಾಂಗ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "젬강 현"), ("lt", "Džemgchango apskritis"), ("lv", "Žemgas distrikts"), ("mr", "झह\u{947}मगा\u{902}ग जिल\u{94d}हा"), ("ms", "Zhemgang District"), ("nb", "Zhemgang"), ("nl", "Zhemgang"), ("no", "Zhemgang"), ("pl", "Dystrykt Zhemgang"), ("pt", "Zhemgang"), ("ro", "Districtul Zhemgang"), ("ru", "Жемганг"), ("si", "සෙම\u{dca}ග\u{dcf}න\u{dca}ග\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Zhemgang"), ("ta", "ஸிங\u{bcd}கங\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "జ\u{c46}ంగ\u{c3e}ంగ\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "มณฑลเชมก\u{e31}ง"), ("tr", "Zhemgang"), ("uk", "Жемганг (дзонгхаг)"), ("ur", "ژیمگانگ ضلع"), ("vi", "Zhemgang"), ("zh", "谢姆冈宗")]),
                        unofficial_name_list: ["Shemgang", "Zhemgang"].to_vec(),
                    }
                ),
                (
                    "41",
                    Subdivision{
                        name: "41",
                        country_alpha2: Alpha2::BT,
                        code: "41",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.2566795), longitude: Some(91.7538817), max_latitude: Some(27.482902), min_latitude: Some(27.0165199), max_longitude: Some(92.12519789999999), min_longitude: Some(91.378786)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تراشيغانغ"), ("bg", "Трасхиганг"), ("bn", "ট\u{9cd}র\u{9be}শিগ\u{9be}ং জেল\u{9be}"), ("ca", "Districte de Trashigang"), ("ccp", "𑄑\u{11133}𑄢𑄥\u{11128}𑄉\u{11127}\u{11101}"), ("ceb", "Trashigang Dzongkhag"), ("da", "Trashigang District"), ("de", "Trashigang"), ("el", "Τρασιγκάνγκ"), ("en", "Trashigang"), ("es", "Distrito de Trashigang"), ("et", "Trashigangi ringkond"), ("fi", "Trashigang"), ("fr", "Trashigang"), ("gu", "ટ\u{acd}ર\u{ac7}શીગા\u{a82}ગ જિલ\u{acd}લો"), ("hi", "ट\u{94d}र\u{948}शीग\u{948}\u{902}ग जिला"), ("hu", "Trashigang körzet"), ("id", "Distrik Trashigang"), ("it", "distretto di Trashigang"), ("ja", "タシガン県"), ("kn", "ಟ\u{ccd}ರಾಶ\u{cbf}ಗಂಗ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "트라시강 현"), ("lt", "Trašigango apskritis"), ("lv", "Trašigas distrikts"), ("mr", "कचरागड जिल\u{94d}हा"), ("ms", "Trashigang District"), ("nb", "Trashigang"), ("nl", "Trashigang"), ("no", "Trashigang"), ("pl", "Dystrykt Trashigang"), ("pt", "Trashigang"), ("ro", "Districtul Trashigang"), ("ru", "Трашиганг"), ("si", "ට\u{dca}\u{200d}රැෂ\u{dd2}ගැන\u{dca}ග\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Trashigang"), ("ta", "டர\u{bcd}ஷிகங\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ట\u{c4d}ర\u{c3e}ష\u{c3f}గ\u{c3e}ంగ\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตทราช\u{e34}เกง"), ("tr", "Trashigang District"), ("uk", "Трашіганг"), ("ur", "تراشیگانگ ضلع"), ("vi", "Quận Trashigang"), ("zh", "塔希冈宗")]),
                        unofficial_name_list: ["Tashigang", "Trashigang"].to_vec(),
                    }
                ),
                (
                    "42",
                    Subdivision{
                        name: "42",
                        country_alpha2: Alpha2::BT,
                        code: "42",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.275), longitude: Some(91.24), max_latitude: Some(27.280493), min_latitude: Some(27.2708807), max_longitude: Some(91.2454034), min_longitude: Some(91.23415949999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة مونغار"), ("bg", "Монгар"), ("bn", "মনগ\u{9be}র জেল\u{9be}"), ("ca", "Districte de Mongar"), ("ccp", "𑄟\u{11127}\u{11101}𑄉𑄢\u{11134}"), ("ceb", "Mongar"), ("da", "Mongar District"), ("de", "Mongar"), ("el", "Μονγκάρ"), ("en", "Mongar"), ("es", "Distrito de Mongar"), ("et", "Mongari ringkond"), ("fi", "Mongar"), ("fr", "Mongar"), ("gu", "મો\u{a82}ગર જિલ\u{acd}લો"), ("hi", "मो\u{902}गार जिला"), ("hu", "Mongar körzet"), ("id", "Distrik Mongar"), ("it", "distretto di Mongar"), ("ja", "モンガル県"), ("kn", "ಮೊಂಗಾರ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "몽가르 현"), ("lt", "Mongaro apskritis"), ("lv", "Mongā distrikts"), ("mr", "मो\u{902}गार जिल\u{94d}हा"), ("ms", "Mongar District"), ("nb", "Mongar"), ("nl", "Mongar"), ("no", "Mongar"), ("pl", "Dystrykt Mongar"), ("ps", "مونگار ولسوالۍ"), ("pt", "Mongar"), ("ro", "Mongar"), ("ru", "Монгар"), ("si", "මොන\u{dca}ග\u{dcf}ර\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Mongar"), ("ta", "மோன\u{bcd}கர\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "మ\u{c4b}ంగ\u{c3e}ర\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ซาลฟ\u{e34}ท โกเวอโนเรท"), ("tr", "Mongar District"), ("uk", "Монгар"), ("ur", "مونگار ضلع"), ("vi", "Quận Mongar"), ("zh", "蒙加尔宗")]),
                        unofficial_name_list: ["Monggar", "Mongor"].to_vec(),
                    }
                ),
                (
                    "43",
                    Subdivision{
                        name: "43",
                        country_alpha2: Alpha2::BT,
                        code: "43",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.002382), longitude: Some(91.3469247), max_latitude: Some(27.1670959), min_latitude: Some(26.8459449), max_longitude: Some(91.517646), min_longitude: Some(91.17381700000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بيماغاتشيل"), ("bg", "Помагачел"), ("bn", "পেম\u{9be}গ\u{9be}টসেল জেল\u{9be}"), ("ca", "Districte de Pemagatshel"), ("ccp", "𑄛𑄬𑄟𑄉𑄖\u{11134}𑄥𑄬𑄣\u{11134}"), ("ceb", "Pemagatshel Dzongkhag"), ("da", "Pemagatshel District"), ("de", "Pemagatshel"), ("el", "Πεμαγκάτσελ"), ("en", "Pemagatshel"), ("es", "Distrito de Pemagatshel"), ("et", "Pemagatsheli ringkond"), ("fi", "Pemagatshel"), ("fr", "Pemagatshel"), ("gu", "પ\u{ac7}માગાશ\u{ac7}લ જિલ\u{acd}લો"), ("hi", "प\u{947}म\u{947}गात\u{94d}स\u{947}ल जिला"), ("hu", "Pemagatshel körzet"), ("id", "Distrik Pemagatshel"), ("it", "distretto di Pemagatshel"), ("ja", "ペマガツェル県"), ("kn", "ಪ\u{cc6}ಮಗತ\u{ccd}ಶ\u{cc6}ಲ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "페마가첼 현"), ("lt", "Pemagačelo apskritis"), ("lv", "Pemagačeles distrikts"), ("mr", "प\u{947}मगट\u{94d}ह\u{947}ळ जिल\u{94d}हा"), ("ms", "Pemagatshel District"), ("nb", "Pemagatshel"), ("nl", "Pemagatshel"), ("no", "Pemagatshel"), ("pl", "Dystrykt Pemagatshel"), ("ps", "پيماگاتشل ولسوالۍ"), ("pt", "Pemagatshel"), ("ro", "Pemagatshel"), ("ru", "Пемагацел"), ("si", "පෙමගට\u{dca}ෂෙල\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Pemagatshel"), ("ta", "பேமகட\u{bcd}ஷெல\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ప\u{c46}మ\u{c3e}గ\u{c3e}ట\u{c4d}ష\u{c46}ల\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เม\u{e37}องพ\u{e35}มาเกจเชล"), ("tr", "Pemagatshe District"), ("uk", "Пемагацел"), ("ur", "پیماگاتشل ضلع"), ("vi", "Quận Pemagatshel"), ("zh", "佩马加策尔宗")]),
                        unofficial_name_list: ["Pema Gatshel", "Pemagatsel"].to_vec(),
                    }
                ),
                (
                    "44",
                    Subdivision{
                        name: "44",
                        country_alpha2: Alpha2::BT,
                        code: "44",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.6649208), longitude: Some(91.1761004), max_latitude: Some(27.677438), min_latitude: Some(27.6492443), max_longitude: Some(91.1892156), min_longitude: Some(91.1688424)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة لانتس"), ("bg", "Лунтсе"), ("bn", "লোন\u{9cd}সে জেল\u{9be}"), ("ca", "Districte de Lhuntse"), ("ccp", "𑄣\u{1112a}\u{11101}𑄥𑄬"), ("ceb", "Lhuentse Dzongkhag"), ("da", "Lhuntse District"), ("de", "Lhuntse"), ("el", "Λχάντσε"), ("en", "Lhuntse"), ("es", "Distrito de Lhuntse"), ("et", "Lhuentse ringkond"), ("fi", "Lhuntse"), ("fr", "Lhuntse"), ("gu", "લ\u{ac1}ન\u{acd}તસ\u{ac7} જિલ\u{acd}લો"), ("hi", "ल\u{941}न\u{94d}त\u{94d}स\u{947} जिला"), ("hu", "Lhuntse körzet"), ("id", "Distrik Lhuntse"), ("it", "distretto di Lhuntse"), ("ja", "ルンツェ県"), ("kn", "ಲುಂಡ\u{ccd}ಸ\u{cc6} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "룬체 현"), ("lt", "Luncės apskritis"), ("lv", "Lhunci distrikts"), ("mr", "ल\u{941}न\u{941}च\u{947} जिल\u{94d}हा"), ("ms", "Lhuntse District"), ("nb", "Lhuntse"), ("nl", "Lhuntse"), ("no", "Lhuntse"), ("pl", "Dystrykt Lhuntse"), ("ps", "لهنتسی ولسوالۍ"), ("pt", "Lhuntse"), ("ro", "Lhuntse"), ("ru", "Лхунце"), ("si", "ල\u{dd4}න\u{dca}ට\u{dca}සේ ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Lhuntse"), ("ta", "லஹன\u{bcd}ட\u{bcd}ஸ\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "లూంట\u{c4d}స\u{c46} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตลฮ\u{e38}นซ\u{e35}"), ("tr", "Lhuntse District"), ("uk", "Лхунце"), ("ur", "لہنتسے ضلع"), ("vi", "Tỉnh Lhuntse"), ("zh", "伦奇宗")]),
                        unofficial_name_list: ["Lhuentse", "Lhun Tshi", "Lhuntshi", "Lhuntsi"].to_vec(),
                    }
                ),
                (
                    "45",
                    Subdivision{
                        name: "45",
                        country_alpha2: Alpha2::BT,
                        code: "45",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(26.928696), longitude: Some(91.63721500000001), max_latitude: Some(27.2464701), min_latitude: Some(26.777367), max_longitude: Some(92.1221539), min_longitude: Some(90.995525)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سامدروب جونغخار"), ("bg", "Самдруп Джонгхар"), ("bn", "স\u{9be}মড\u{9cd}রোপ জ\u{9be}ংক\u{9be}র জেল\u{9be}"), ("ca", "Districte de Samdrup Jongkhar"), ("ccp", "𑄥\u{11133}𑄠𑄟\u{11134}𑄓\u{11133}𑄢\u{1112a}𑄛\u{11134} 𑄎\u{11127}\u{11101}𑄈𑄢\u{11134}"), ("ceb", "Samdrup Jongkhar Dzongkhag"), ("da", "Samdrup Jongkhar District"), ("de", "Samdrup Jongkhar"), ("el", "Σαμντρούπ Τζόνγκχαρ"), ("en", "Samdrup Jongkhar"), ("es", "Distrito de Samdrup Jongkhar"), ("et", "Samdrup Jongkhari ringkond"), ("fi", "Samdrup Jongkhar"), ("fr", "Samdrup Jongkhar"), ("gu", "સા\u{a82}ડર\u{ac2}પ જો\u{a82}ગખાર જિલ\u{acd}લો"), ("hi", "समद\u{94d}रप जो\u{902}गखार जिला"), ("hu", "Samdrup Jongkhar körzet"), ("id", "Distrik Samdrup Jongkhar"), ("it", "distretto di Samdrup Jongkhar"), ("ja", "サムドゥプ・ジョンカル県"), ("kn", "ಸ\u{ccd}ಯಾಮ\u{ccd}ಡ\u{ccd}ರಪ\u{ccd} ಜೊಂಗ\u{ccd}ಖರ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "삼드룹종카르 현"), ("lt", "Samdrupo Džongcharo apskritis"), ("lv", "Samdzupdzonkhā distrikts"), ("mr", "स\u{945}मद\u{94d}र\u{941}प जो\u{902}गखार जिल\u{94d}हा"), ("ms", "Samdrup Jongkhar District"), ("nb", "Samdrup Jongkhar"), ("nl", "Samdrup Jongkhar"), ("no", "Samdrup Jongkhar"), ("pl", "Dystrykt Samdrup Jongkhar"), ("ps", "سامدروپ جونگخار ولسوالۍ"), ("pt", "Samdrup Jongkhar"), ("ro", "Samdrup Jongkhar"), ("ru", "Самдруп-Джонгхар"), ("si", "සම\u{dca}ඩ\u{dca}රප\u{dca} ජෝන\u{dca}ග\u{dca}ඛ\u{dcf}ර\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Samdrup Jongkhar"), ("ta", "சம\u{bcd}டருப\u{bcd} ஜொங\u{bcd}க\u{bcd}ஹ\u{bcd}ர\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "స\u{c3e}మడ\u{c4d}రప\u{c4d} జ\u{c3e}ంగ\u{c4d}క\u{c3e}ర\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตแซมดร\u{e31}ป จองตาฮ\u{e4c}"), ("tr", "Samdrup Jonkh District"), ("uk", "Самдруп-Джонгхар"), ("ur", "سامدروپ جونگخار ضلع"), ("vi", "Quận Samdrup Jongkhar"), ("zh", "萨姆德鲁琼卡尔宗")]),
                        unofficial_name_list: ["Samdruk Jongkhar", "Samdrup", "Samdrup Jongkha"].to_vec(),
                    }
                ),
                (
                    "GA",
                    Subdivision{
                        name: "GA",
                        country_alpha2: Alpha2::BT,
                        code: "GA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.0185886), longitude: Some(89.9253233), max_latitude: Some(28.323778), min_latitude: Some(27.687483), max_longitude: Some(90.617648), min_longitude: Some(89.43694289999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غاسا"), ("bg", "Гаса"), ("bn", "গ\u{9be}স\u{9be} জেল\u{9be}"), ("ca", "Districte de Gasa"), ("ccp", "𑄉𑄥"), ("ceb", "Gasa"), ("da", "Gasa"), ("de", "Gasa"), ("el", "Γκάσα"), ("en", "Gasa"), ("es", "Distrito de Gasa"), ("et", "Gasa ringkond"), ("fa", "استان گاسا"), ("fi", "Gasa"), ("fr", "Gasa"), ("gu", "ગાસા જિલ\u{acd}લો"), ("hi", "गासा जिला"), ("hu", "Gasa körzet"), ("id", "Distrik Gasa"), ("it", "distretto di Gasa"), ("ja", "ガサ県"), ("kn", "ಗಾಸಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "가사 현"), ("lt", "Gasos rajonas"), ("lv", "Gasas distrikts"), ("mr", "गसा जिल\u{94d}हा"), ("ms", "Gasa District"), ("nb", "Gasa"), ("nl", "Gasa"), ("no", "Gasa"), ("pl", "Dystrykt Gasa"), ("ps", "گاسا ولسوالۍ"), ("pt", "Gasa"), ("ro", "Gasa"), ("ru", "Гаса"), ("si", "ගස\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Gasa"), ("ta", "க\u{bbe}ச\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "గ\u{c3e}స\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตกาซา"), ("tr", "Gasa District"), ("uk", "Гаса"), ("ur", "گاسا ضلع"), ("vi", "Quận Gasa"), ("zh", "加萨宗")]),
                        unofficial_name_list: ["Gaza"].to_vec(),
                    }
                ),
                (
                    "TY",
                    Subdivision{
                        name: "TY",
                        country_alpha2: Alpha2::BT,
                        code: "TY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.583333), longitude: Some(91.466667), max_latitude: Some(28.050773), min_latitude: Some(27.383962), max_longitude: Some(91.73789980000001), min_longitude: Some(91.31767270000002)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تراشيانغتس"), ("bg", "Трасхиянсте"), ("bn", "ট\u{9cd}র\u{9be}শিইয\u{9bc}\u{9be}ংসে জেল\u{9be}"), ("ca", "Districte de Trashiyamgtse"), ("ccp", "𑄑\u{11133}𑄢𑄥\u{11128}𑄠\u{11101}𑄥𑄬"), ("ceb", "Trashi Yangste"), ("da", "Trashiyangtse District"), ("de", "Trashiyangtse"), ("el", "Τρασιγιάνγκτσε"), ("en", "Trashiyangtse"), ("es", "Distrito de Trashiyangste"), ("et", "Trashi Yangtse ringkond"), ("fa", "استان تراشیانگتس"), ("fi", "Trashiyangste"), ("fr", "Trashiyangtse"), ("gu", "ત\u{acd}રાશિયા\u{a82}ગત\u{acd}સ\u{ac7} જિલ\u{acd}લો"), ("hi", "ट\u{94d}र\u{948}शिया\u{902}गत\u{94d}स\u{947} जिला"), ("hu", "Trashiyangtse körzet"), ("id", "Distrik Trashiyangtse"), ("it", "distretto di Trashiyangtse"), ("ja", "タシ・ヤンツェ県"), ("kn", "ಟ\u{ccd}ರಾಶ\u{cbf}ಯಾಂಗ\u{ccd}ಸ\u{cc6} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "트라시양체 현"), ("lt", "Trašijangstės apskritis"), ("lv", "Trašijanci distrikts"), ("mr", "त\u{941}शिया\u{902}गत\u{94d}शी जिल\u{94d}हा"), ("ms", "Trashiyangtse District"), ("nb", "Trashiyangtse"), ("nl", "Trashiyangste"), ("no", "Trashiyangtse"), ("pl", "Dystrykt Trashiyangste"), ("ps", "تراشييانگتسی ولسوالۍ"), ("pt", "Trashiyangste"), ("ro", "Trashiyangtse"), ("ru", "Трашиянгце"), ("si", "ට\u{dca}\u{200d}රශ\u{dd2}යන\u{dca}ග\u{dca}ස\u{dca}ටේ ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sl", "Okraj Trašijangce"), ("sv", "Trashiyangste"), ("ta", "டிரஷிய\u{bbe}ங\u{bcd}ட\u{bcd}ஸே ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ట\u{c4d}ర\u{c3e}ష\u{c3f}య\u{c3e}ంగ\u{c4d}\u{200c}త\u{c4d}స\u{c47} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ทราช\u{e34}ย\u{e31}งเจอ"), ("tr", "Trashiyangste"), ("uk", "Трашіянгце"), ("ur", "تراشییانگتسے ضلع"), ("vi", "Trashiyangste"), ("zh", "塔希央奇宗")]),
                        unofficial_name_list: ["Tashiyangtse"].to_vec(),
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
#[cfg(feature = "bt")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::BT,
        alpha3: Alpha3::BTN,
        address_format: None,
        continent: Continent::Asia,
        country_code: 975,
        currency_code: "BTN",
        gec: Some(GEC::BT),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("BHU"),
        iso_long_name: "The Kingdom of Bhutan",
        iso_short_name: "Bhutan",
        official_language_list: ["dz"].to_vec(),
        spoken_language_list: ["dz"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7, 8].to_vec(),
        national_prefix: "None",
        nationality: Some("Bhutanese"),
        number: "064",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthernAsia),
        un_locode: "BT",
        unofficial_name_list: ["Bhutan", "Bhoutan", "Bután", "ブータン"].to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Bhutan"),
            ("af", "Bhoetan"),
            ("ak", "Bhutan"),
            ("am", "ቡህታን"),
            ("an", "Bután"),
            ("ar", "بوتان"),
            ("as", "ভ\u{9c1}ট\u{9be}ন"),
            ("ay", "Bhutan"),
            ("az", "Butan"),
            ("ba", "Bhutan"),
            ("be", "Бутан"),
            ("bg", "Бутан"),
            ("bi", "Bhutan"),
            ("bn", "ভ\u{9c1}ট\u{9be}ন"),
            ("bn_IN", "ভ\u{9c1}ট\u{9be}ন"),
            ("br", "Bhoutan"),
            ("bs", "Butan"),
            ("ca", "Bhutan"),
            ("ce", "Бутан"),
            ("ch", "Bhutan"),
            ("cs", "Bhútán"),
            ("cv", "Бутан"),
            ("cy", "Bhutan"),
            ("da", "Bhutan"),
            ("de", "Bhutan"),
            ("dv", "ބ\u{7ab}ޓ\u{7a7}ނ\u{7b0}"),
            ("dz", "འབ\u{fb2}\u{f74}ག།"),
            ("ee", "Bhutan"),
            ("el", "Μπουτάν"),
            ("en", "Bhutan"),
            ("eo", "Butano"),
            ("es", "Bután"),
            ("et", "Bhutan"),
            ("eu", "Bhutan"),
            ("fa", "بوتان"),
            ("ff", "Bhutan"),
            ("fi", "Bhutan"),
            ("fo", "Butan"),
            ("fr", "Bhoutan"),
            ("fy", "Bûtan"),
            ("ga", "An Bhútáin"),
            ("gl", "Bután"),
            ("gn", "Bhutan"),
            ("gu", "ભ\u{ac1}ટાન"),
            ("gv", "Yn Vutaan"),
            ("ha", "Bhutan"),
            ("he", "בהוטן"),
            ("hi", "भ\u{942}टान"),
            ("hr", "Butan"),
            ("ht", "Boutan"),
            ("hu", "Bhután"),
            ("hy", "Բութան"),
            ("ia", "Bhutan"),
            ("id", "Bhutan"),
            ("io", "Bhutan"),
            ("is", "Bútan"),
            ("it", "Bhutan"),
            ("iu", "Bhutan"),
            ("ja", "ブータン"),
            ("ka", "ბუტანი"),
            ("ki", "Bhutan"),
            ("kk", "Бутан"),
            ("kl", "Bhutan"),
            ("km", "ប\u{17ca}\u{17bc}តាន"),
            ("kn", "ಭ\u{cc2}ತಾನ\u{ccd}"),
            ("ko", "부탄"),
            ("ku", "Bûtan"),
            ("kv", "Бутан"),
            ("kw", "Bhoutan"),
            ("ky", "Бутан"),
            ("lo", "Bhutan"),
            ("lt", "Butanas"),
            ("lv", "Butāna"),
            ("mi", "Bhutan"),
            ("mk", "Бутан"),
            ("ml", "ബ\u{d42}ട\u{d4d}ട\u{d3e}ന\u{d4d}\u{200d}"),
            ("mn", "Бутан"),
            ("mr", "भ\u{942}तान"),
            ("ms", "Bhutan"),
            ("mt", "Butan"),
            (
                "my",
                "ဘ\u{1030}တန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Butan"),
            ("nb", "Bhutan"),
            ("ne", "भ\u{941}टान"),
            ("nl", "Bhutan"),
            ("nn", "Bhutan"),
            ("nv", "Iiʼniʼ Tłʼiishtsoh Bikéyah"),
            ("oc", "Botan"),
            ("or", "ଭ\u{b41}ଟ\u{b3e}ନ"),
            ("pa", "ਭ\u{a41}ਟਾਨ"),
            ("pi", "भ\u{942}टान"),
            ("pl", "Bhutan"),
            ("ps", "بوتان"),
            ("pt", "Butão"),
            ("pt_BR", "Butão"),
            ("ro", "Bhutan"),
            ("ru", "Бутан"),
            ("rw", "Butani"),
            ("sc", "Bhutàn"),
            ("sd", "Bhutan"),
            ("si", "භ\u{dd6}ත\u{dcf}නය"),
            ("sk", "Bhután"),
            ("sl", "Butan"),
            ("so", "Butaan"),
            ("sq", "Butan"),
            ("sr", "Бутан"),
            ("sv", "Bhutan"),
            ("sw", "Bhutan"),
            ("ta", "பூட\u{bcd}ட\u{bbe}ன\u{bcd}"),
            ("te", "భూట\u{c3e}న\u{c4d}"),
            ("tg", "Бутон"),
            ("th", "ภ\u{e39}ฏาน"),
            ("ti", "ቡህታን"),
            ("tk", "Butan"),
            ("tl", "Bhutan"),
            ("tr", "Bhutan"),
            ("tt", "Бутан"),
            ("ug", "بۇتان"),
            ("uk", "Бутан"),
            ("ur", "بھوٹان"),
            ("uz", "Butan"),
            ("ve", "Bhutan"),
            ("vi", "Bu-thănh"),
            ("wa", "Boutan"),
            ("wo", "Butaan"),
            ("xh", "Bhutan"),
            ("yo", "Bhùtán"),
            ("zh_CN", "不丹"),
            ("zh_HK", "不丹"),
            ("zh_TW", "不丹"),
            ("zu", "Bhutan"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}