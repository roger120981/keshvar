// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Honduras

#[cfg(all(feature = "hn", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::HN;
    pub const ALPHA3: Alpha3 = Alpha3::HND;
    pub const CONTINENT: Continent = Continent::NorthAmerica;
    pub const COUNTRY_CODE: usize = 504;
    pub const CURRENCY_CODE: &str = "HNL";
    pub const GEC: Option<GEC> = Some(GEC::HO);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("HON");
    pub const ISO_SHORT_NAME: &str = "Honduras";
    pub const ISO_LONG_NAME: &str = "The Republic of Honduras";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["es"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["es"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7, 8];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Honduran");
    pub const NUMBER: &str = "340";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::CentralAmerica);
    pub const UN_LOCODE: &str = "HN";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Honduras", "ホンジュラス"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Honduras"),
        ("af", "Honduras"),
        ("ak", "Honduras"),
        ("am", "Honduras"),
        ("an", "Honduras"),
        ("ar", "هندوراس"),
        ("as", "হণ\u{9cd}ড\u{9c1}ৰ\u{9be}ছ"),
        ("ay", "Honduras"),
        ("az", "Honduras"),
        ("ba", "Honduras"),
        ("be", "Гандурас"),
        ("bg", "Хондурас"),
        ("bi", "Honduras"),
        ("bn", "হন\u{9cd}ড\u{9c1}র\u{9be}স"),
        ("bn_IN", "হন\u{9cd}ড\u{9c1}র\u{9be}স"),
        ("br", "Honduras"),
        ("bs", "Honduras"),
        ("ca", "Hondures"),
        ("ce", "Гондурас"),
        ("ch", "Honduras"),
        ("cs", "Honduras"),
        ("cv", "Гондурас"),
        ("cy", "Honduras"),
        ("da", "Honduras"),
        ("de", "Honduras"),
        ("dv", "ހ\u{7ae}ނ\u{7b0}ޑ\u{7a8}އ\u{7aa}ރ\u{7a6}ސ\u{7b0}"),
        ("dz", "ཧ\u{f71}\u{f7c}ན་ཌ\u{f74}་ར\u{f71}ས\u{f72}།"),
        ("ee", "Honduras"),
        ("el", "Ονδούρα"),
        ("en", "Honduras"),
        ("eo", "Honduro"),
        ("es", "Honduras"),
        ("et", "Honduras"),
        ("eu", "Honduras"),
        ("fa", "هندوراس"),
        ("ff", "Honduras"),
        ("fi", "Honduras"),
        ("fo", "Honduras"),
        ("fr", "Honduras"),
        ("fy", "Hondueras"),
        ("ga", "Hondúras"),
        ("gl", "Honduras"),
        ("gn", "Honduras"),
        ("gu", "હોન\u{acd}ડ\u{ac1}રાસ"),
        ("gv", "Ny Hondooraghyn"),
        ("ha", "Honduras"),
        ("he", "הונדורס"),
        ("hi", "हौण\u{94d}ड\u{941}रस"),
        ("hr", "Honduras"),
        ("ht", "Ondiras"),
        ("hu", "Honduras"),
        ("hy", "Հոնդուրաս"),
        ("ia", "Honduras"),
        ("id", "Honduras"),
        ("io", "Honduras"),
        ("is", "Hondúras"),
        ("it", "Honduras"),
        ("iu", "Honduras"),
        ("ja", "ホンジュラス"),
        ("ka", "ჰონდურასი"),
        ("ki", "Honduras"),
        ("kk", "Гондурас"),
        ("kl", "Honduras"),
        ("km", "ហ\u{17bb}ងឌ\u{17bc}រ\u{17c9}ាស\u{17cb}"),
        ("kn", "ಹೊಂಡುರಾಸ\u{ccd}"),
        ("ko", "온두라스"),
        ("ku", "Honduras"),
        ("kv", "Гондурас"),
        ("kw", "Hondouras"),
        ("ky", "Гондурас"),
        ("lo", "Honduras"),
        ("lt", "Hondūras"),
        ("lv", "Hondurasa"),
        ("mi", "Honduras"),
        ("mk", "Хондурас"),
        ("ml", "ഹോണ\u{d4d}ട\u{d41}റ\u{d3e}സ\u{d4d}"),
        ("mn", "Гондурас"),
        ("mr", "हो\u{902}ड\u{941}रास"),
        ("ms", "Honduras"),
        ("mt", "Ħonduras"),
        (
            "my",
            "ဟ\u{103d}န\u{103a}ဒ\u{1030}းရပ\u{103a}စ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Ondurat"),
        ("nb", "Honduras"),
        ("ne", "हन\u{94d}ड\u{941}रस\u{94d}"),
        ("nl", "Honduras"),
        ("nn", "Honduras"),
        ("nv", "Honduras"),
        ("oc", "Honduras"),
        ("or", "ହୋଣ\u{b4d}ଡ\u{b41}ର\u{b3e}ସ"),
        ("pa", "ਹਾਨਡ\u{a42}ਰਸ"),
        ("pi", "हा\u{902}ड\u{942}रस"),
        ("pl", "Honduras"),
        ("ps", "هانډوراس"),
        ("pt", "Honduras"),
        ("pt_BR", "Honduras"),
        ("ro", "Honduras"),
        ("ru", "Гондурас"),
        ("rw", "Hondurasi"),
        ("sc", "Honduras"),
        ("sd", "Honduras"),
        ("si", "හොන\u{dca}ඩ\u{dd4}ර\u{dcf}ස\u{dca}"),
        ("sk", "Honduras"),
        ("sl", "Honduras"),
        ("so", "Honduras"),
        ("sq", "Honduras"),
        ("sr", "Хондурас"),
        ("sv", "Honduras"),
        ("sw", "Honduras"),
        ("ta", "ஹோன\u{bcd}டுர\u{bbe}ஸ\u{bcd}"),
        ("te", "హ\u{c4b}ండురస\u{c4d}"),
        ("tg", "Ҳондурас"),
        ("th", "ฮอนด\u{e39}ร\u{e31}ส"),
        ("ti", "ሆንዱራስ"),
        ("tk", "Gonduras"),
        ("tl", "Honduras"),
        ("tr", "Honduras"),
        ("tt", "Һондурас"),
        ("ug", "ھوندۇراس"),
        ("uk", "Гондурас"),
        ("ur", "ہونڈوراس"),
        ("uz", "Gonduras"),
        ("ve", "Honduras"),
        ("vi", "Hôn-đu-ra-xợ"),
        ("wa", "Honduras"),
        ("wo", "Onduuras"),
        ("xh", "Honduras"),
        ("yo", "Họ\u{300}ndúràs"),
        ("zh_CN", "洪都拉斯"),
        ("zh_HK", "洪都拉斯"),
        ("zh_TW", "宏都拉斯"),
        ("zu", "I-Hondurasi"),
    ];
    #[cfg(all(feature = "hn", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 15.199999;
        pub const LONGITUDE: f64 = -86.241905;
        pub const MAX_LATITUDE: f64 = 17.4677999;
        pub const MAX_LONGITUDE: f64 = -83.0621001;
        pub const MIN_LATITUDE: f64 = 12.9808201;
        pub const MIN_LONGITUDE: f64 = -89.3564822;
        pub const NORTHEAST_LATITUDE: f64 = 17.4677999;
        pub const NORTHEAST_LONGITUDE: f64 = -83.0621001;
        pub const SOUTHWEST_LATITUDE: f64 = 12.9808201;
        pub const SOUTHWEST_LONGITUDE: f64 = -89.3564822;
    }
}
#[cfg(all(feature = "hn", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 15.199999,
            longitude: -86.241905,
            max_latitude: 17.4677999,
            max_longitude: -83.0621001,
            min_latitude: 12.9808201,
            min_longitude: -89.3564822,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 17.4677999,
                    longitude: -83.0621001,
                },
                southwest: CountryGeoBound {
                    latitude: 12.9808201,
                    longitude: -89.3564822,
                },
            },
        }
    }
}

#[cfg(all(feature = "hn", feature = "subdivisions"))]
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
                    "AT",
                    Subdivision{
                        name: "AT",
                        country_alpha2: Alpha2::HN,
                        code: "AT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.6696283), longitude: Some(-87.14228949999999), max_latitude: Some(15.9182919), min_latitude: Some(15.430607), max_longitude: Some(-86.3275431), min_longitude: Some(-87.7970871)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أتلانتيدا"), ("bg", "Атлантида"), ("bn", "আটল\u{9be}ন\u{9cd}টিক\u{9be} বিভ\u{9be}গ"), ("ca", "Departament d’Atlántida"), ("ccp", "𑄃𑄖\u{11134}𑄣𑄥𑄑\u{11128}𑄓"), ("ceb", "Departamento de Atlántida"), ("da", "Atlántida Department"), ("de", "Atlántida"), ("el", "Ατλαντίδα"), ("en", "Atlántida"), ("es", "Departamento de Atlántida"), ("eu", "Atlántida"), ("fa", "استان آتلانتیدا"), ("fi", "Atlántida"), ("fr", "département d’Atlántida"), ("gl", "Departamento de Atlántida"), ("gu", "એટલાન\u{acd}ટિડા વિભાગ"), ("he", "אטלנטידה"), ("hi", "अटला\u{902}टिडा विभाग"), ("hu", "Atlántida megye"), ("id", "Departemen Atlántida"), ("it", "dipartimento di Atlántida"), ("ja", "アトランティダ県"), ("ka", "ატლანტიდის დეპარტამენტი"), ("kn", "ಅಟ\u{ccd}ಲಾಂಟ\u{cbf}ಡಾ ಇಲಾಖ\u{cc6}"), ("ko", "아틀란티다 주"), ("lt", "Atlantidos departamentas"), ("lv", "Atlantīdas departaments"), ("mr", "अटला\u{902}टिडा विभाग"), ("ms", "Atlantida Department"), ("nb", "Atlantida department"), ("nl", "Atlántida"), ("no", "Atlantida department"), ("pl", "Atlántida"), ("pt", "Atlántida"), ("ro", "Atlántida"), ("ru", "Атлантида"), ("si", "අට\u{dca}ලන\u{dca}ට\u{dd2}ඩ\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Atlántida"), ("ta", "அட\u{bcd}ல\u{bbe}ன\u{bcd}டிட\u{bbe} துறை"), ("te", "అట\u{c4d}ల\u{c3e}ంట\u{c3f}డ\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e31}ตล\u{e31}นต\u{e35}ดา"), ("tr", "Atlántida Departmanı"), ("uk", "Атлантида"), ("ur", "آتلانتیدا محکمہ"), ("vi", "Atlántida"), ("zh", "阿特蘭蒂達省")]),
                        unofficial_name_list: ["Atlántida"].to_vec(),
                    }
                ),
                (
                    "CH",
                    Subdivision{
                        name: "CH",
                        country_alpha2: Alpha2::HN,
                        code: "CH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.3198572), longitude: Some(-87.1622344), max_latitude: Some(13.3577214), min_latitude: Some(13.2443735), max_longitude: Some(-87.11497299999999), min_longitude: Some(-87.2275829)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة تشولوتيكا"), ("bg", "Чолутека"), ("bn", "ক\u{9c1}ল\u{9c1}তেক\u{9be} বিভ\u{9be}গ"), ("ca", "Departament de Choluteca"), ("ccp", "𑄌\u{1112e}𑄣\u{11128}𑄅\u{1112a}𑄑𑄬𑄇"), ("ceb", "Departamento de Choluteca"), ("da", "Choluteca Department"), ("de", "Departamento Choluteca"), ("el", "Τσολουτέκα"), ("en", "Choluteca"), ("es", "Departamento de Choluteca"), ("eu", "Choluteca"), ("fa", "استان چولوتکا"), ("fi", "Choluteca"), ("fr", "département de Choluteca"), ("gl", "Departamento de Choluteca"), ("gu", "ચોલ\u{ac1}ટ\u{ac7}કા વિભાગ"), ("hi", "चोल\u{941}ट\u{947}का विभाग"), ("hu", "Choluteca megye"), ("id", "Departemen Choluteca"), ("it", "dipartimento di Choluteca"), ("ja", "チョルテカ県"), ("ka", "ჩოლუტეკის დეპარტამენტი"), ("kn", "ಕೊಲುಟ\u{cc6}ಕಾ ಇಲಾಖ\u{cc6}"), ("ko", "촐루테카 주"), ("lt", "Čolutekos departamentas"), ("lv", "Čolutekas departaments"), ("mr", "चोल\u{941}ट\u{947}का विभाग"), ("ms", "Choluteca Department"), ("nb", "Choluteca"), ("nl", "Choluteca"), ("no", "Choluteca"), ("pl", "Choluteca"), ("pt", "Choluteca"), ("ro", "Departamentul Choluteca"), ("ru", "Чолутека"), ("si", "චොල\u{dd4}ටෙක\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Choluteca"), ("sv", "Choluteca"), ("ta", "சொல\u{bcd}லுட\u{bc0}க\u{bcd}க\u{bbe} துறை"), ("te", "చ\u{c4b}లుట\u{c46}క\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "โชล\u{e39}เตกา"), ("tr", "Choluteca Departmanı"), ("uk", "Чолутека"), ("ur", "چولوتیکا ڈیپارٹمنٹ"), ("vi", "Choluteca"), ("zh", "喬盧特卡省")]),
                        unofficial_name_list: ["Choluteca"].to_vec(),
                    }
                ),
                (
                    "CL",
                    Subdivision{
                        name: "CL",
                        country_alpha2: Alpha2::HN,
                        code: "CL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.6391768), longitude: Some(-85.35496499999999), max_latitude: Some(16.0269222), min_latitude: Some(15.0795581), max_longitude: Some(-84.9903293), min_longitude: Some(-86.4258461)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة كولون"), ("bg", "Колон"), ("bn", "কোলন বিভ\u{9be}গ"), ("ca", "Departament de Colón"), ("ccp", "𑄇\u{1112e}𑄣\u{1112e}𑄚\u{11134}"), ("ceb", "Departamento de Colón"), ("da", "Colón Department"), ("de", "Departamento Colón"), ("el", "Κολόν"), ("en", "Colón"), ("es", "Departamento de Colón"), ("eu", "Colón"), ("fa", "استان کولون"), ("fi", "Colón"), ("fr", "département de Colón"), ("gl", "Departamento de Colón"), ("gu", "કોલોન વિભાગ"), ("hi", "कोलन विभाग (हो\u{902}ड\u{941}रास)"), ("hu", "Colón megye"), ("id", "Departemen Colón"), ("it", "dipartimento di Colón"), ("ja", "コロン県"), ("ka", "კოლონის დეპარტამენტი"), ("kn", "ಕೊಲೊನ\u{ccd} ಡ\u{cbf}ಪಾರ\u{ccd}ಟ\u{ccd}ಮ\u{cc6}ಂಟ\u{ccd}"), ("ko", "콜론 주"), ("lt", "Kolono departamentas"), ("lv", "Kolonas departaments"), ("mr", "कॉलोन विभाग"), ("ms", "Colon Department"), ("nb", "Colon department"), ("nl", "Colón"), ("no", "Colon department"), ("pl", "Colón"), ("pt", "Colón"), ("ro", "Colón"), ("ru", "Колон"), ("si", "කොලෝන\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Departamento de Colón (departement)"), ("ta", "கோலோன\u{bcd} துறை"), ("te", "క\u{c4b}ల\u{c4b}న\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "โคลอนด\u{e34}พาทเมนท\u{e4c}"), ("tr", "Colon Departmanı"), ("uk", "Колон"), ("ur", "کولون محکمہ"), ("vi", "Colón"), ("zh", "科隆省")]),
                        unofficial_name_list: ["Colón"].to_vec(),
                    }
                ),
                (
                    "CM",
                    Subdivision{
                        name: "CM",
                        country_alpha2: Alpha2::HN,
                        code: "CM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.5534828), longitude: Some(-87.6186379), max_latitude: Some(15.0569), min_latitude: Some(14.0668049), max_longitude: Some(-87.261583), min_longitude: Some(-88.08321099999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Комаягуа"), ("ca", "Comayagua"), ("ccp", "𑄇\u{1112e}𑄟𑄬𑄠𑄋\u{11134}𑄉\u{1112a}𑄠"), ("ceb", "Departamento de Comayagua"), ("cs", "Comayagua"), ("de", "Departamento Comayagua"), ("en", "Comayagua"), ("es", "Comayagua"), ("eu", "Comayagua"), ("fa", "استان کومایاگوآ"), ("fi", "Comayagua"), ("fr", "département de Comayagua"), ("gl", "Departamento de Comayagua"), ("hu", "Comayagua megye"), ("id", "Departemen Comayagua"), ("it", "dipartimento di Comayagua"), ("ja", "コマヤグア県"), ("ka", "კომაიაგუის დეპარტამენტი"), ("ko", "코마야과 주"), ("lt", "Komajagvos departamentas"), ("mr", "कोमायाग\u{941}आ प\u{94d}रा\u{902}त"), ("nl", "Comayagua"), ("pl", "Comayagua"), ("pt", "Comayagua"), ("ro", "Comayagua"), ("ru", "Комаягуа"), ("sk", "Comayagua"), ("sv", "Departamento de Comayagua"), ("uk", "Комаягуа"), ("ur", "کومایاغوا محکمہ"), ("vi", "Comayagua"), ("zh", "科馬亞瓜省")]),
                        unofficial_name_list: ["Comayagua"].to_vec(),
                    }
                ),
                (
                    "CP",
                    Subdivision{
                        name: "CP",
                        country_alpha2: Alpha2::HN,
                        code: "CP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.9360838), longitude: Some(-88.86469799999999), max_latitude: Some(15.294539), min_latitude: Some(14.5260461), max_longitude: Some(-88.63253), min_longitude: Some(-89.22522)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة كوبان"), ("bg", "Копан"), ("bn", "কোপ\u{9be}ন বিভ\u{9be}গ"), ("ca", "Copán"), ("ccp", "𑄇\u{1112e}𑄛𑄚\u{11134}"), ("ceb", "Departamento de Copán"), ("da", "Copán Department"), ("de", "Departamento Copán"), ("el", "Κοπάν"), ("en", "Copán"), ("es", "Departamento de Copán"), ("eu", "Copán"), ("fa", "استان کوپان"), ("fi", "Copán"), ("fr", "département de Copán"), ("gl", "Departamento de Copán"), ("gu", "કોપ\u{ac5}ન વિભાગ"), ("hi", "कोपन विभाग"), ("hu", "Copán megye"), ("id", "Departemen Copán"), ("it", "dipartimento di Copán"), ("ja", "コパン県"), ("ka", "კოპანის დეპარტამენტი"), ("kn", "ಕೊಪಾನ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "코판 주"), ("lt", "Kopano departamentas"), ("lv", "Kopanas departaments"), ("mr", "कोपन विभाग"), ("ms", "Copan Department"), ("nb", "Copan department"), ("nl", "Copán"), ("no", "Copan department"), ("pl", "Copán"), ("pt", "Copán"), ("ru", "Копан"), ("si", "කොප\u{dcf}න\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Departamento de Copán"), ("ta", "கோபன\u{bcd} துறை"), ("te", "క\u{c4b}పన\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโกป\u{e31}น"), ("tr", "Copan Departmanı"), ("uk", "Копан"), ("ur", "کوپان محکمہ"), ("vi", "Copán"), ("zh", "科潘省")]),
                        unofficial_name_list: ["Copán"].to_vec(),
                    }
                ),
                (
                    "CR",
                    Subdivision{
                        name: "CR",
                        country_alpha2: Alpha2::HN,
                        code: "CR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.4909515), longitude: Some(-87.9334803), max_latitude: Some(15.9198125), min_latitude: Some(14.8104159), max_longitude: Some(-87.7257844), min_longitude: Some(-88.44252600000002)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة كورتيس"), ("bg", "Кортес"), ("bn", "কর\u{9cd}টেস বিভ\u{9be}গ"), ("ca", "Departament de Cortés"), ("ccp", "𑄇\u{11127}𑄢\u{11134}𑄑𑄬𑄌\u{11134}"), ("ceb", "Departamento de Cortés"), ("cs", "Cortés"), ("da", "Cortés Department"), ("de", "Departamento Cortés"), ("el", "Κορτές"), ("en", "Cortés"), ("es", "Cortés"), ("eu", "Cortés"), ("fa", "استان کورتس"), ("fi", "Cortés"), ("fr", "département de Cortés"), ("gl", "Departamento de Cortés"), ("gu", "કોર\u{acd}ટિસ વિભાગ"), ("hi", "कोर\u{94d}ट\u{947}ज विभाग"), ("hu", "Cortés megye"), ("id", "Departemen Cortés"), ("it", "dipartimento di Cortés"), ("ja", "コルテス県"), ("ka", "კორტესის დეპარტამენტი"), ("kn", "ಕಾರ\u{ccd}ಟ\u{cc6}ಸ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "코르테스 주"), ("lt", "Korteso departamentas"), ("lv", "Kortesa departaments"), ("mr", "कोर\u{94d}त\u{947}स प\u{94d}रा\u{902}त"), ("ms", "Cortes Department"), ("nb", "Cortes"), ("nl", "Cortés"), ("no", "Cortes"), ("pl", "Cortés"), ("pt", "Cortés"), ("ru", "Кортес"), ("si", "කොටේස\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Cortés"), ("sv", "Departamento de Cortés"), ("ta", "க\u{bbe}ர\u{bcd}ட\u{bcd}ஸ\u{bcd} டெப\u{bbe}ர\u{bcd}ட\u{bcd}மென\u{bcd}ட"), ("te", "క\u{c4b}ర\u{c4d}ట\u{c46}స\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "คอร\u{e4c}เตส"), ("tr", "Cortés Departmanı"), ("uk", "Кортес"), ("ur", "کورتیس محکمہ"), ("vi", "Cortés"), ("zh", "科爾特斯省")]),
                        unofficial_name_list: ["Cortés"].to_vec(),
                    }
                ),
                (
                    "EP",
                    Subdivision{
                        name: "EP",
                        country_alpha2: Alpha2::HN,
                        code: "EP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.0736932), longitude: Some(-86.41873079999999), max_latitude: Some(14.3801481), min_latitude: Some(13.5013601), max_longitude: Some(-85.57647709999999), min_longitude: Some(-87.23560499999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة بارايسو"), ("bg", "Ел Параисо"), ("bn", "এল প\u{9be}র\u{9be}ইসো বিভ\u{9be}গ"), ("ca", "Departament d’El Paraíso"), ("ccp", "𑄃𑄬𑄣\u{11134} 𑄛𑄢\u{1112d}𑄥\u{1112e}"), ("ceb", "Departamento de El Paraíso"), ("da", "El Paraíso Department"), ("de", "Departamento El Paraíso"), ("el", "Ελ Παραΐσο"), ("en", "El Paraíso"), ("es", "El Paraíso"), ("eu", "El Paraíso"), ("fa", "استان ال پارائیسو"), ("fi", "El Paraiso"), ("fr", "département d’El Paraíso"), ("gl", "Departamento de El Paraíso"), ("gu", "અલ પારાઈસો વિભાગ"), ("he", "אל פאראיסו"), ("hi", "अल प\u{947}राइसो डिपार\u{94d}टम\u{947}\u{902}ट"), ("hu", "El Paraíso megye"), ("id", "Departemen El Paraíso"), ("it", "dipartimento di El Paraíso"), ("ja", "エル・パライソ県"), ("ka", "ელ-პარაისოს დეპარტამენტი"), ("kn", "ಎಲ\u{ccd} ಪಾರೈಸೊ ಇಲಾಖ\u{cc6}"), ("ko", "엘파라이소 주"), ("lt", "El Paraiso departamentas"), ("lv", "Elparaiso departaments"), ("mr", "अल पाराईओ विभाग"), ("ms", "El Paraiso Department"), ("nb", "El Paraiso"), ("nl", "El Paraíso"), ("no", "El Paraiso"), ("pl", "El Paraíso"), ("pt", "El Paraíso"), ("ru", "Эль-Параисо"), ("si", "එල\u{dca} පරය\u{dd2}සෝ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "El Paraiso"), ("ta", "எல\u{bcd} பரிசோ துறை"), ("te", "ఎల\u{c4d} ప\u{c3e}ర\u{c3e}స\u{c3f}య\u{c4b} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เอลปาไรโซ"), ("tr", "El Paraiso Departmanı"), ("uk", "Ель-Параїсо"), ("ur", "ال پارایسو ڈیپارٹمنٹ"), ("vi", "El Paraíso"), ("zh", "埃爾帕拉伊索省")]),
                        unofficial_name_list: ["El Paraíso"].to_vec(),
                    }
                ),
                (
                    "FM",
                    Subdivision{
                        name: "FM",
                        country_alpha2: Alpha2::HN,
                        code: "FM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.45411), longitude: Some(-87.0624261), max_latitude: Some(15.013803), min_latitude: Some(13.662749), max_longitude: Some(-86.70461999999999), min_longitude: Some(-87.648878)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة فرانسيسكو مورازان"), ("be", "Франсіска Марасан"), ("bg", "Франсиско Морасан"), ("bn", "ফ\u{9cd}র\u{9be}ন\u{9cd}সিসকো মর\u{9be}জ\u{9be}ন বিভ\u{9be}গ"), ("ca", "Departament de Francisco Morazán"), ("ccp", "𑄜\u{11133}𑄢𑄚\u{11134}𑄥\u{11128}𑄌\u{11134}𑄇\u{1112e} 𑄟\u{1112e}𑄢𑄎𑄚\u{11134}"), ("ceb", "Departamento de Francisco Morazán"), ("da", "Francisco Morazán Department"), ("de", "Departamento Francisco Morazán"), ("el", "Φρανκίσκο Μοραζάν"), ("en", "Francisco Morazán"), ("es", "Francisco Morazán"), ("eu", "Francisco Morazán"), ("fa", "استان فرانسیسکو مورازان"), ("fi", "Francisco Morazán"), ("fr", "département de Francisco Morazán"), ("gl", "Departamento de Francisco Morazán"), ("gu", "ફ\u{acd}રાન\u{acd}સિસ\u{acd}કો મોરાઝાન વિભાગ"), ("hi", "फ\u{94d}र\u{947}\u{902}सिस\u{94d}को मोराज\u{93c}न विभाग"), ("hu", "Francisco Morazán megye"), ("id", "Departemen Francisco Morazán"), ("it", "dipartimento di Francisco Morazán"), ("ja", "フランシスコ・モラサン県"), ("ka", "ფრანსისკო-მორასანის დეპარტამენტი"), ("kn", "ಫ\u{ccd}ರಾನ\u{ccd}ಸ\u{cbf}ಸ\u{ccd}ಕೊ ಮೊರಾಜನ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "프란시스코모라산 주"), ("lt", "Fransisko Morasano departamentas"), ("lv", "Fransiskomorasanas departaments"), ("mr", "फ\u{94d}रा\u{902}सिस\u{94d}को मोराझान प\u{94d}रा\u{902}त"), ("ms", "Francisco Morazan Department"), ("nb", "Franciso Morazan department"), ("nl", "Francisco Morazán"), ("no", "Franciso Morazan department"), ("pl", "Francisco Morazán (departament)"), ("pt", "Francisco Morazán"), ("ru", "Франсиско Морасан"), ("si", "ෆ\u{dca}රැන\u{dca}ස\u{dd2}ස\u{dca}කෝ මොරස\u{dcf}න\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Departamento de Francisco Morazán"), ("ta", "பிர\u{bbe}ன\u{bcd}சிஸ\u{bcd}கோ மோர\u{bbe}சன\u{bcd} துறை"), ("te", "ఫ\u{c4d}ర\u{c3e}న\u{c4d}స\u{c3f}స\u{c4d}క\u{c4b} మ\u{c4b}ర\u{c3e}జన\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ฟรานซ\u{e34}สโกมอราซ\u{e31}น"), ("tr", "Francisco Moraz"), ("uk", "Франсіско Морасан"), ("ur", "فرانسسکو مورازان ڈیپارٹمنٹ"), ("vi", "Francisco Morazán"), ("zh", "弗朗西斯科-莫拉桑省")]),
                        unofficial_name_list: ["Francisco Morazán"].to_vec(),
                    }
                ),
                (
                    "GD",
                    Subdivision{
                        name: "GD",
                        country_alpha2: Alpha2::HN,
                        code: "GD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.341806), longitude: Some(-84.6060449), max_latitude: Some(15.9782862), min_latitude: Some(14.614529), max_longitude: Some(-83.15540299999999), min_longitude: Some(-84.99412)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة غراسياس ديوس"), ("bg", "Грасиас а Диос"), ("bn", "গ\u{9be}র\u{9cd}সিয\u{9bc}\u{9be}স অ\u{9cd}য\u{9be} ড\u{9be}য\u{9bc}োস বিভ\u{9be}গ"), ("ca", "Departament de Gracias a Dios"), ("ccp", "𑄉\u{11133}𑄢𑄥\u{11128}𑄠𑄌\u{11134} 𑄃 𑄓\u{11128}𑄠\u{1112e}𑄌\u{11134}"), ("ceb", "Departamento de Gracias a Dios"), ("cs", "Gracias a Dios"), ("da", "Gracias a Dios Department"), ("de", "Gracias a Dios"), ("el", "Γκρασίας α Ντίος"), ("en", "Gracias a Dios"), ("es", "Gracias a Dios"), ("eu", "Gracias a Dios"), ("fa", "استان گراسیاس آدیوس"), ("fi", "Gracias a Dios"), ("fr", "département de Gracias a Dios"), ("gl", "Departamento de Gracias a Dios"), ("gu", "ગ\u{acd}ર\u{ac7}સિયસ અ ડાયોસ વિભાગ"), ("hi", "ग\u{94d}रासियास आ दिओस विभाग"), ("hu", "Gracias a Dios megye"), ("id", "Departemen Gracias a Dios"), ("it", "dipartimento di Gracias a Dios"), ("ja", "グラシアス・ア・ディオス県"), ("ka", "გრასიას-ა-დიოსის დეპარტამენტი"), ("kn", "ಗ\u{ccd}ರೇಸ\u{ccd}ಯಾಸ\u{ccd} ಡ\u{cbf}ವೊಸ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "그라시아스아디오스 주"), ("lt", "Grasjas a Dioso departamentas"), ("lv", "Grasjasadjosas departaments"), ("mr", "ग\u{94d}र\u{945}सिस ए ड\u{94d}य\u{941}ओस डिपार\u{94d}टम\u{947}\u{902}ट"), ("ms", "Gracias a Dios Department"), ("nb", "Gracias a Dios"), ("nl", "Gracias a Dios"), ("no", "Gracias a Dios"), ("pl", "Gracias a Dios"), ("pt", "Gracias a Dios"), ("ru", "Грасьяс-а-Дьос"), ("si", "ග\u{dca}\u{200d}ර\u{dcf}ස\u{dd2}යස\u{dca} අ ඩයොස\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Gracias a Dios"), ("sv", "Departamento de Gracias a Dios"), ("ta", "க\u{bcd}ர\u{bbe}சியஸ\u{bcd} அ டியோஸ\u{bcd} துறை"), ("te", "గ\u{c4d}ర\u{c3e}ంస\u{c3f}య\u{c3e}స\u{c4d} ఎ డ\u{c3f}య\u{c3e}స\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เม\u{e37}องซาบล\u{e4c}จ\u{e31}ค"), ("tr", "Gracias a Dios"), ("uk", "Грасіас-а-Діос"), ("ur", "گریشیئس آ دیوس ڈیپارٹمنٹ"), ("vi", "Gracias a Dios"), ("zh", "格拉西亚斯-阿迪奥斯省")]),
                        unofficial_name_list: ["Gracias a Dios"].to_vec(),
                    }
                ),
                (
                    "IB",
                    Subdivision{
                        name: "IB",
                        country_alpha2: Alpha2::HN,
                        code: "IB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.3526078), longitude: Some(-86.4895463), max_latitude: Some(17.415835), min_latitude: Some(16.069366), max_longitude: Some(-83.9287665), min_longitude: Some(-86.98986409999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة جزر الخليج"), ("be", "Астравы Іслас-дэ-ла-Баія"), ("bg", "Ислас де ла Баия"), ("bn", "বে আইল\u{9cd}য\u{9be}ন\u{9cd}ড বিভ\u{9be}গ"), ("ca", "Illes de la Badia"), ("ccp", "𑄝𑄬 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄌\u{11134}"), ("ceb", "Departamento de Islas de la Bahía"), ("cs", "Islas de la Bahía"), ("da", "Bay Islands Department"), ("de", "Islas de la Bahía"), ("el", "Νησιά Μπέι"), ("en", "Bay Islands"), ("es", "Islas de la Bahía"), ("et", "Bahía saared"), ("eu", "Islas de la Bahía"), ("fa", "استان ایسلاس دلا باهیا"), ("fi", "Islas de la Bahía"), ("fr", "département des Islas de la Bahía"), ("gl", "Departamento de Islas de la Bahía"), ("gu", "બ\u{ac7} આયલ\u{ac7}ન\u{acd}ડ\u{acd}સ વિભાગ"), ("he", "איי באייה"), ("hi", "ब\u{947} द\u{94d}वीप सम\u{942}ह विभाग"), ("hu", "Islas de la Bahía megye"), ("id", "Islas de la Bahía"), ("it", "dipartimento di Islas de la Bahía"), ("ja", "イスラス・デ・ラ・バイア県"), ("ka", "ისლას-დე-ლა-ბაიის დეპარტამენტი"), ("kn", "ಬೇ ದ\u{ccd}ವೀಪಗಳು ಇಲಾಖ\u{cc6}"), ("ko", "이슬라스데라바이아 주"), ("lt", "Bahijos salos"), ("lv", "Baijas Salas departaments"), ("mr", "इस\u{94d}लास द\u{947} ला बाहिया"), ("ms", "Bay Islands Department"), ("nb", "Bay Islands department"), ("nl", "Islas de la Bahía"), ("no", "Bay Islands department"), ("pl", "Wyspy Bahia"), ("pt", "Ilhas da Baía"), ("ru", "Ислас-де-ла-Баия"), ("si", "කලප\u{dd4} ද\u{dd6}පත\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Islas de la Bahía"), ("sv", "Bahíaöarna"), ("ta", "பே இஸ\u{bcd}ல\u{bbe}ண\u{bcd}ட\u{bcd}ஸ\u{bcd} துறை"), ("te", "బ\u{c47} ఐల\u{c3e}ండ\u{c4d}స\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เบย\u{e4c} ไอซ\u{e4c}แลนด\u{e4c}"), ("tr", "Bay Adaları"), ("uk", "Іслас-де-ла-Байя"), ("ur", "بے آیلینڈز محکمہ"), ("vi", "Islas de la Bahía"), ("yue", "海灣群島省"), ("yue_Hans", "海湾群岛省"), ("zh", "海灣群島省")]),
                        unofficial_name_list: ["Islas de la Bahía"].to_vec(),
                    }
                ),
                (
                    "IN",
                    Subdivision{
                        name: "IN",
                        country_alpha2: Alpha2::HN,
                        code: "IN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.316667), longitude: Some(-88.16666699999999), max_latitude: Some(14.3376566), min_latitude: Some(14.309215), max_longitude: Some(-88.1368732), min_longitude: Some(-88.1977272)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة إنتبوكا"), ("bg", "Интибука"), ("bn", "ইন\u{9cd}টিব\u{9c1}ক\u{9be} বিভ\u{9be}গ"), ("ca", "Intibucá"), ("ccp", "𑄃\u{11128}𑄚\u{11134}𑄑\u{11128}𑄝\u{11128}𑄅\u{1112a}𑄇"), ("ceb", "Departamento de Intibucá"), ("da", "Intibucá Department"), ("de", "Intibucá"), ("el", "Ιντιμπουκά"), ("en", "Intibucá"), ("es", "Intibucá"), ("eu", "Intibucá"), ("fa", "استان اینتیبوکا"), ("fi", "Intibucán departmentti"), ("fr", "département d’Intibucá"), ("gl", "Departamento de Intibucá"), ("gu", "ઇન\u{acd}ટિબ\u{acd}ય\u{ac1}કા વિભાગ"), ("hi", "इ\u{902}टीब\u{941}का विभाग"), ("hu", "Intibucá megye"), ("id", "Departemen Intibucá"), ("it", "dipartimento di Intibucá"), ("ja", "インティブカ県"), ("ka", "ინტიბუკის დეპარტამენტი"), ("kn", "ಇಂಟ\u{cbf}ಬುಕಾ ಇಲಾಖ\u{cc6}"), ("ko", "인티부카 주"), ("lt", "Intibukos departamentas"), ("lv", "Intibukas departaments"), ("mr", "इ\u{902}टीब\u{942}च विभाग"), ("ms", "Intibuca Department"), ("nb", "Intibuca department"), ("nl", "Intibucá"), ("no", "Intibuca department"), ("pl", "Intibucá"), ("pt", "Intibucá"), ("ru", "Интибука"), ("si", "ඉන\u{dca}ට\u{dd2}බ\u{dd4}ක\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Departamento de Intibucá"), ("ta", "இன\u{bcd}டிபுக\u{bcd}க\u{bbe} துறை"), ("te", "ఇంట\u{c3f}బూక\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เม\u{e37}องอ\u{e34}นท\u{e34}บ\u{e39}คา"), ("tr", "Intibuca Departmanı"), ("uk", "Інтібука"), ("ur", "انتیپوکا محکمہ"), ("vi", "Intibucá"), ("zh", "因蒂布卡省")]),
                        unofficial_name_list: ["Intibucá"].to_vec(),
                    }
                ),
                (
                    "LE",
                    Subdivision{
                        name: "LE",
                        country_alpha2: Alpha2::HN,
                        code: "LE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.1887698), longitude: Some(-88.55653099999999), max_latitude: Some(14.955633), min_latitude: Some(13.978775), max_longitude: Some(-88.3192471), min_longitude: Some(-88.99023400000002)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة ليمبيرا"), ("bg", "Лемпира"), ("bn", "লেম\u{9cd}পির\u{9be} বিভ\u{9be}গ"), ("ca", "Departament de Lempira"), ("ccp", "𑄣𑄬𑄟\u{11134}𑄛\u{11128}𑄢"), ("ceb", "Departamento de Lempira"), ("da", "Lempira Department"), ("de", "Departamento Lempira"), ("el", "Λεμπίραν"), ("en", "Lempira"), ("es", "Lempira"), ("eu", "Lempira"), ("fa", "استان لمپیرا"), ("fi", "Lempiran departmentti"), ("fr", "département de Lempira"), ("gl", "Departamento de Lempira"), ("gu", "લ\u{ac7}મ\u{acd}પિરા વિભાગ"), ("hi", "ल\u{947}म\u{94d}पिरा विभाग"), ("hu", "Lempira megye"), ("id", "Departemen Lempira"), ("it", "dipartimento di Lempira"), ("ja", "レンピーラ県"), ("ka", "ლემპირას დეპარტამენტი"), ("kn", "ಲ\u{cc6}ಂಪ\u{ccd}ಪ\u{cbf} ಇಲಾಖ\u{cc6}"), ("ko", "렘피라 주"), ("lt", "Lempiros departamentas"), ("lv", "Lempiras departaments"), ("mr", "लिम\u{94d}पिरा विभाग"), ("ms", "Jabatan Lempira"), ("nb", "Lampira department"), ("nl", "Lempira"), ("no", "Lampira department"), ("pl", "Lempira"), ("pt", "Lempira"), ("ru", "Лемпира"), ("si", "ලේම\u{dca}ප\u{dd2}ර\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Departamento de Lempira"), ("ta", "லெம\u{bcd}பிர\u{bbe} துறை"), ("te", "ల\u{c46}ంప\u{c3f}ర\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เม\u{e37}องเลมพ\u{e34}รา"), ("tr", "Lempira Departmanı"), ("uk", "Лемпіра (департамент)"), ("ur", "لیمپیرا ڈیپارٹمنٹ"), ("vi", "Lempira"), ("zh", "倫皮拉省")]),
                        unofficial_name_list: ["Lempira"].to_vec(),
                    }
                ),
                (
                    "LP",
                    Subdivision{
                        name: "LP",
                        country_alpha2: Alpha2::HN,
                        code: "LP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.9984833), longitude: Some(-87.9334803), max_latitude: Some(14.417439), min_latitude: Some(13.822687), max_longitude: Some(-87.6088089), min_longitude: Some(-88.23758199999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة لا باز"), ("bg", "Ла Пас"), ("bn", "ল\u{9be} প\u{9be}জ বিভ\u{9be}গ"), ("ca", "Departament de La Paz"), ("ccp", "𑄣 𑄛𑄌\u{11134}"), ("ceb", "Departamento de La Paz (departamento sa Honduras)"), ("da", "La Paz Department"), ("de", "La Paz"), ("el", "Λα Παζ"), ("en", "La Paz"), ("es", "La Paz"), ("eu", "La Paz"), ("fa", "استان لاپاز"), ("fi", "La Pazin departementti"), ("fr", "département de La Paz"), ("gl", "Departamento de La Paz"), ("gu", "લા પાઝ વિભાગ"), ("he", "לה פאס"), ("hi", "ला-पाज\u{93c} विभाग"), ("hu", "La Paz megye"), ("id", "Departemen La Paz"), ("it", "dipartimento di La Paz"), ("ja", "ラパス県"), ("ka", "ლა-პასის დეპარტამენტი (ჰონდურასი)"), ("kn", "ಲಾ ಪಾಜ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "라파스 주"), ("lt", "La Paso departamentas"), ("lv", "Lapasas departaments"), ("mr", "ला पाझ विभाग"), ("ms", "La Paz Department"), ("nb", "La Paz department"), ("nl", "La Paz"), ("no", "La Paz department"), ("pl", "La Paz (departament Hondurasu)"), ("pt", "La Paz"), ("ru", "Ла-Пас"), ("si", "ල\u{dcf} ප\u{dcf}ස\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Departamento de La Paz (departement i Honduras)"), ("ta", "ல\u{bbe} பஸ\u{bcd} துறை"), ("te", "ల\u{c3e} ప\u{c3e}జ\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดลาปาซ"), ("tr", "La paz"), ("uk", "Ла-Пас (департамент Гондурасу)"), ("ur", "لا پاز ڈیپارٹمنٹ"), ("vi", "La Paz"), ("zh", "拉巴斯省 (洪都拉斯)")]),
                        unofficial_name_list: ["La Paz"].to_vec(),
                    }
                ),
                (
                    "OC",
                    Subdivision{
                        name: "OC",
                        country_alpha2: Alpha2::HN,
                        code: "OC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.5170347), longitude: Some(-89.0561532), max_latitude: Some(14.747355), min_latitude: Some(14.249759), max_longitude: Some(-88.716308), min_longitude: Some(-89.350792)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة أوكوتيبيك"), ("bg", "Окотепеке"), ("bn", "অকোটেপেক বিভ\u{9be}গ"), ("ca", "Departament d’Ocotepeque"), ("ccp", "𑄃\u{1112e}𑄇\u{1112e}𑄑𑄬𑄛𑄬𑄇\u{11134}"), ("ceb", "Departamento de Ocotepeque"), ("da", "Ocotepeque Department"), ("de", "Departamento Ocotepeque"), ("el", "Οκοτεπέκε"), ("en", "Ocotepeque"), ("es", "Ocotepeque"), ("eu", "Ocotepeque"), ("fa", "استان اوکوتپکوئه"), ("fi", "Ocotepeque"), ("fr", "département d’Ocotepeque"), ("gl", "Departamento de Ocotepeque"), ("gu", "ઓકોટ\u{ac7}પ\u{ac7}ક\u{acd}ય\u{ac1} વિભાગ"), ("hi", "ओकोट\u{947}प\u{947}क\u{94d}य\u{942} विभाग"), ("hu", "Ocotepeque megye"), ("id", "Departemen Ocotepeque"), ("it", "dipartimento di Ocotepeque"), ("ja", "オコテペケ県"), ("ka", "ოკოტეპეკეს დეპარტამენტი"), ("kn", "ಒಕೊಟ\u{cc6}ಪ\u{cc6}ಕ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "오코테페케 주"), ("lt", "Okotepekės departamentas"), ("lv", "Okotepekes departaments"), ("mr", "ओकोट\u{947}प\u{947}क\u{94d}य\u{942} विभाग"), ("ms", "Ocotepeque Department"), ("nb", "Ocotepeque department"), ("nl", "Ocotepeque"), ("no", "Ocotepeque department"), ("pl", "Department Ocotepeque"), ("pt", "Ocotepeque"), ("ru", "Окотепеке"), ("si", "ඔකොටෙපෙක\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Departamento de Ocotepeque"), ("ta", "ஒகோடிப\u{bcd}பேயூ துறை"), ("te", "ఆక\u{c4d}ట\u{c4b}ప\u{c46}ఖ\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "โอโคเพค\u{e34}ว ด\u{e35}พาร\u{e4c}ทเม\u{e49}น"), ("tr", "Ocotepeque Departmanı"), ("uk", "Окотепеке"), ("ur", "اوکوتیپیقوی ڈیپارٹمنٹ"), ("vi", "Ocotepeque"), ("zh", "奥科特佩克省")]),
                        unofficial_name_list: ["Ocotepeque"].to_vec(),
                    }
                ),
                (
                    "OL",
                    Subdivision{
                        name: "OL",
                        country_alpha2: Alpha2::HN,
                        code: "OL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.8067406), longitude: Some(-85.76666449999999), max_latitude: Some(15.61391), min_latitude: Some(14.046359), max_longitude: Some(-84.98785400000001), min_longitude: Some(-86.9544921)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة أولانشو"), ("bg", "Оланчо"), ("bn", "ওল\u{9be}নকো বিভ\u{9be}গ"), ("ca", "Departament d’Olancho"), ("ccp", "𑄃\u{1112e}𑄣𑄚\u{11134}𑄇\u{1112e}"), ("ceb", "Departamento de Olancho"), ("da", "Olancho"), ("de", "Departamento Olancho"), ("el", "Ολάντσο"), ("en", "Olancho"), ("es", "Olancho"), ("eu", "Olancho"), ("fa", "استان اولانچو"), ("fi", "Olancho"), ("fr", "département d’Olancho"), ("gl", "Departamento de Olancho"), ("gu", "ઓલા\u{a82}કો વિભાગ"), ("hi", "ओलान\u{94d}को विभाग"), ("hu", "Olancho megye"), ("id", "Departemen Olancho"), ("it", "dipartimento di Olancho"), ("ja", "オランチョ県"), ("ka", "ოლანჩოს დეპარტამენტი"), ("kn", "ಒಲಾಂಚೊ ಇಲಾಖ\u{cc6}"), ("ko", "올란초 주"), ("lt", "Olančo departamentas"), ("lv", "Olančo departaments"), ("mr", "ओला\u{902}चो विभाग"), ("ms", "Olancho Department"), ("nb", "Olancho department"), ("nl", "Olancho"), ("no", "Olancho department"), ("pl", "Olancho"), ("pt", "Olancho"), ("ru", "Оланчо"), ("si", "ඕලන\u{dca}චෝ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Departamento de Olancho"), ("ta", "ஒல\u{bbe}ன\u{bcd}சோ துறை"), ("te", "ఓల\u{c3e}ంచ\u{c4b} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโอล\u{e31}นโช"), ("tr", "Olancho Department"), ("uk", "Оланчо"), ("ur", "اولانچو ڈیپارٹمنٹ"), ("vi", "Olancho"), ("zh", "奧蘭喬省")]),
                        unofficial_name_list: ["Olancho"].to_vec(),
                    }
                ),
                (
                    "SB",
                    Subdivision{
                        name: "SB",
                        country_alpha2: Alpha2::HN,
                        code: "SB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.1202795), longitude: Some(-88.4016041), max_latitude: Some(15.53789), min_latitude: Some(14.5849759), max_longitude: Some(-87.9959605), min_longitude: Some(-88.751592)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة سانتا باربرا"), ("bg", "Санта Барбара"), ("bn", "স\u{9be}ন\u{9cd}ত\u{9be} ব\u{9be}রব\u{9be}র\u{9be} বিভ\u{9be}গ"), ("ca", "Departament de Santa Bárbara"), ("ccp", "𑄥𑄚\u{11134}𑄑 𑄝𑄢\u{11134}𑄝𑄢"), ("ceb", "Departamento de Santa Bárbara (departamento)"), ("da", "Santa Bárbara Department"), ("de", "Departamento Santa Bárbara"), ("el", "Σάντα Μπάρμπαρα"), ("en", "Santa Bárbara"), ("es", "Santa Bárbara"), ("eu", "Santa Bárbara"), ("fa", "استان سانتا باربارا"), ("fi", "Santa Bárbaran departmentti"), ("fr", "département de Santa Bárbara"), ("gl", "Departamento de Santa Bárbara"), ("gu", "સાન\u{acd}ટા બાર\u{acd}બરા વિભાગ"), ("hi", "स\u{948}\u{902}टा बारबरा विभाग"), ("hu", "Santa Bárbara megye"), ("id", "Departemen Santa Bárbara"), ("it", "dipartimento di Santa Bárbara"), ("ja", "サンタ・バルバラ県"), ("ka", "სანტა-ბარბარას დეპარტამენტი"), ("kn", "ಸಾಂಟಾ ಬರ\u{ccd}ಬರಾ ಇಲಾಖ\u{cc6}"), ("ko", "산타바르바라 주"), ("lt", "Santa Barbaros departamentas"), ("lv", "Santabarbaras departaments"), ("mr", "सा\u{902}ता बारबरा विभाग"), ("ms", "Santa Barbara Department"), ("nb", "Santa Barbara Department"), ("nl", "Santa Bárbara"), ("no", "Santa Barbara Department"), ("pl", "Santa Bárbara"), ("pt", "Santa Bárbara"), ("ru", "Санта-Барбара"), ("si", "සැන\u{dca}ට\u{dcf} බ\u{dcf}ර\u{dca}බර\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Santa Bárbara"), ("sv", "Departamento de Santa Bárbara (departement)"), ("ta", "ச\u{bbe}ண\u{bcd}ட\u{bbe} ப\u{bbe}ர\u{bcd}பர\u{bbe} துறை"), ("te", "స\u{c3e}ంట\u{c3e} బ\u{c3e}ర\u{c4d}బర\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เขตซานตาบาร\u{e4c}บารา"), ("tr", "Santa Bárbara"), ("uk", "Санта-Барбара"), ("ur", "سنتا بڑبڑا ڈیپارٹمنٹ"), ("vi", "Santa Bárbara"), ("zh", "聖巴巴拉省")]),
                        unofficial_name_list: ["Santa Bárbara"].to_vec(),
                    }
                ),
                (
                    "VA",
                    Subdivision{
                        name: "VA",
                        country_alpha2: Alpha2::HN,
                        code: "VA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.5782936), longitude: Some(-87.5791287), max_latitude: Some(13.853783), min_latitude: Some(13.2487044), max_longitude: Some(-87.361198), min_longitude: Some(-87.8682931)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة فالي"), ("be", "Валье"), ("bg", "Вале"), ("bn", "ভ\u{9be}ল বিভ\u{9be}গ"), ("ca", "Departament de Valle"), ("ccp", "𑄞𑄣𑄬"), ("ceb", "Departamento de Valle"), ("da", "Valle Department"), ("de", "Departamento Valle"), ("el", "Βάλλε"), ("en", "Valle"), ("es", "Valle"), ("eu", "Valle"), ("fa", "استان واله"), ("fi", "Valle"), ("fr", "département de Valle"), ("gl", "Departamento de Valle"), ("gu", "વાલ\u{ac7} વિભાગ"), ("hi", "व\u{948}ल\u{947} विभाग"), ("hu", "Valle megye"), ("id", "Departemen Valle"), ("it", "dipartimento di Valle"), ("ja", "バジェ県"), ("ka", "ვალიეს დეპარტამენტი"), ("kn", "ವ\u{ccd}ಯಾಲ\u{cc6} ಇಲಾಖ\u{cc6}"), ("ko", "바예 주"), ("lt", "Valjės departamentas"), ("lv", "Valjes departaments"), ("mr", "व\u{945}ल\u{947} प\u{94d}रद\u{947}श"), ("ms", "Valle Department"), ("nb", "Valle"), ("nl", "Valle"), ("no", "Valle"), ("pl", "Valle"), ("pt", "Valle"), ("ru", "Валье"), ("si", "වල\u{dca}ලේ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Valle"), ("sv", "Departamento de Valle"), ("ta", "வல\u{bcd}லே துறை"), ("te", "వల\u{c4d}ల\u{c47} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เวลล\u{e35} ด\u{e35}พาทเม\u{e49}น"), ("tr", "Valle Department"), ("uk", "Вальє"), ("ur", "والی ڈیپارٹمنٹ"), ("vi", "Valle"), ("zh", "山谷省")]),
                        unofficial_name_list: ["Valle"].to_vec(),
                    }
                ),
                (
                    "YO",
                    Subdivision{
                        name: "YO",
                        country_alpha2: Alpha2::HN,
                        code: "YO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.2949679), longitude: Some(-87.14228949999999), max_latitude: Some(15.729222), min_latitude: Some(14.7905061), max_longitude: Some(-86.2196529), min_longitude: Some(-87.9417579)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "يورو"), ("bg", "Йоро"), ("bn", "ইউরো বিভ\u{9be}গ"), ("ca", "Departament de Yoro"), ("ccp", "𑄃\u{11128}𑄠\u{1112e}𑄢\u{1112e}"), ("ceb", "Departamento de Yoro"), ("da", "Yoro Department"), ("de", "Departamento Yoro"), ("el", "Γιόρο"), ("en", "Yoro"), ("es", "Yoro"), ("eu", "Yoro"), ("fa", "استان یورو"), ("fi", "Yoron departmentti"), ("fr", "département de Yoro"), ("gl", "Departamento de Yoro"), ("gu", "યોરો વિભાગ"), ("hi", "योरो विभाग"), ("hu", "Yoro megye"), ("id", "Departemen Yoro"), ("it", "dipartimento di Yoro"), ("ja", "ヨロ県"), ("ka", "იოროს დეპარტამენტი"), ("kn", "ಯಾರೊ ಡ\u{cbf}ಪಾರ\u{ccd}ಟ\u{ccd}ಮ\u{cc6}ಂಟ\u{ccd}"), ("ko", "요로 주"), ("lt", "Joro departamentas"), ("lv", "Joro departaments"), ("mr", "योरॉ विभाग"), ("ms", "Yoro Department"), ("nb", "Yoro department"), ("nl", "Yoro"), ("no", "Yoro department"), ("pl", "Yoro"), ("pt", "Yoro"), ("ru", "Йоро"), ("si", "යොරෝ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Departamento de Yoro"), ("ta", "ய\u{bbe}ரோ துறை"), ("te", "య\u{c4b}ర\u{c4b} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "โยโร"), ("tr", "Yoro Departmanı"), ("uk", "Йоро"), ("ur", "یورو ڈیپارٹمنٹ"), ("vi", "Yoro"), ("zh", "約羅省")]),
                        unofficial_name_list: ["Yoro"].to_vec(),
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
#[cfg(feature = "hn")]
pub fn new() -> Country {
    Country{
        alpha2: Alpha2::HN,
        alpha3: Alpha3::HND,
        address_format: None,
        continent: Continent::NorthAmerica,
        country_code: 504,
        currency_code: "HNL",
        gec: Some(GEC::HO),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("HON"),
        iso_long_name: "The Republic of Honduras",
        iso_short_name: "Honduras",
        official_language_list: ["es"].to_vec(),
        spoken_language_list: ["es"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7, 8].to_vec(),
        national_prefix: "None",
        nationality: Some("Honduran"),
        number: "340",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::CentralAmerica),
        un_locode: "HN",
        unofficial_name_list: ["Honduras", "ホンジュラス"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([("ab", "Honduras"), ("af", "Honduras"), ("ak", "Honduras"), ("am", "Honduras"), ("an", "Honduras"), ("ar", "هندوراس"), ("as", "হণ\u{9cd}ড\u{9c1}ৰ\u{9be}ছ"), ("ay", "Honduras"), ("az", "Honduras"), ("ba", "Honduras"), ("be", "Гандурас"), ("bg", "Хондурас"), ("bi", "Honduras"), ("bn", "হন\u{9cd}ড\u{9c1}র\u{9be}স"), ("bn_IN", "হন\u{9cd}ড\u{9c1}র\u{9be}স"), ("br", "Honduras"), ("bs", "Honduras"), ("ca", "Hondures"), ("ce", "Гондурас"), ("ch", "Honduras"), ("cs", "Honduras"), ("cv", "Гондурас"), ("cy", "Honduras"), ("da", "Honduras"), ("de", "Honduras"), ("dv", "ހ\u{7ae}ނ\u{7b0}ޑ\u{7a8}އ\u{7aa}ރ\u{7a6}ސ\u{7b0}"), ("dz", "ཧ\u{f71}\u{f7c}ན་ཌ\u{f74}་ར\u{f71}ས\u{f72}།"), ("ee", "Honduras"), ("el", "Ονδούρα"), ("en", "Honduras"), ("eo", "Honduro"), ("es", "Honduras"), ("et", "Honduras"), ("eu", "Honduras"), ("fa", "هندوراس"), ("ff", "Honduras"), ("fi", "Honduras"), ("fo", "Honduras"), ("fr", "Honduras"), ("fy", "Hondueras"), ("ga", "Hondúras"), ("gl", "Honduras"), ("gn", "Honduras"), ("gu", "હોન\u{acd}ડ\u{ac1}રાસ"), ("gv", "Ny Hondooraghyn"), ("ha", "Honduras"), ("he", "הונדורס"), ("hi", "हौण\u{94d}ड\u{941}रस"), ("hr", "Honduras"), ("ht", "Ondiras"), ("hu", "Honduras"), ("hy", "Հոնդուրաս"), ("ia", "Honduras"), ("id", "Honduras"), ("io", "Honduras"), ("is", "Hondúras"), ("it", "Honduras"), ("iu", "Honduras"), ("ja", "ホンジュラス"), ("ka", "ჰონდურასი"), ("ki", "Honduras"), ("kk", "Гондурас"), ("kl", "Honduras"), ("km", "ហ\u{17bb}ងឌ\u{17bc}រ\u{17c9}ាស\u{17cb}"), ("kn", "ಹೊಂಡುರಾಸ\u{ccd}"), ("ko", "온두라스"), ("ku", "Honduras"), ("kv", "Гондурас"), ("kw", "Hondouras"), ("ky", "Гондурас"), ("lo", "Honduras"), ("lt", "Hondūras"), ("lv", "Hondurasa"), ("mi", "Honduras"), ("mk", "Хондурас"), ("ml", "ഹോണ\u{d4d}ട\u{d41}റ\u{d3e}സ\u{d4d}"), ("mn", "Гондурас"), ("mr", "हो\u{902}ड\u{941}रास"), ("ms", "Honduras"), ("mt", "Ħonduras"), ("my", "ဟ\u{103d}န\u{103a}ဒ\u{1030}းရပ\u{103a}စ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}"), ("na", "Ondurat"), ("nb", "Honduras"), ("ne", "हन\u{94d}ड\u{941}रस\u{94d}"), ("nl", "Honduras"), ("nn", "Honduras"), ("nv", "Honduras"), ("oc", "Honduras"), ("or", "ହୋଣ\u{b4d}ଡ\u{b41}ର\u{b3e}ସ"), ("pa", "ਹਾਨਡ\u{a42}ਰਸ"), ("pi", "हा\u{902}ड\u{942}रस"), ("pl", "Honduras"), ("ps", "هانډوراس"), ("pt", "Honduras"), ("pt_BR", "Honduras"), ("ro", "Honduras"), ("ru", "Гондурас"), ("rw", "Hondurasi"), ("sc", "Honduras"), ("sd", "Honduras"), ("si", "හොන\u{dca}ඩ\u{dd4}ර\u{dcf}ස\u{dca}"), ("sk", "Honduras"), ("sl", "Honduras"), ("so", "Honduras"), ("sq", "Honduras"), ("sr", "Хондурас"), ("sv", "Honduras"), ("sw", "Honduras"), ("ta", "ஹோன\u{bcd}டுர\u{bbe}ஸ\u{bcd}"), ("te", "హ\u{c4b}ండురస\u{c4d}"), ("tg", "Ҳондурас"), ("th", "ฮอนด\u{e39}ร\u{e31}ส"), ("ti", "ሆንዱራስ"), ("tk", "Gonduras"), ("tl", "Honduras"), ("tr", "Honduras"), ("tt", "Һондурас"), ("ug", "ھوندۇراس"), ("uk", "Гондурас"), ("ur", "ہونڈوراس"), ("uz", "Gonduras"), ("ve", "Honduras"), ("vi", "Hôn-đu-ra-xợ"), ("wa", "Honduras"), ("wo", "Onduuras"), ("xh", "Honduras"), ("yo", "Họ\u{300}ndúràs"), ("zh_CN", "洪都拉斯"), ("zh_HK", "洪都拉斯"), ("zh_TW", "宏都拉斯"), ("zu", "I-Hondurasi")]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
