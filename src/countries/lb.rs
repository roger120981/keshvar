// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Lebanese Republic

#[cfg(all(feature = "lb", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::LB;
    pub const ALPHA3: Alpha3 = Alpha3::LBN;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 961;
    pub const CURRENCY_CODE: &str = "LBP";
    pub const GEC: Option<GEC> = Some(GEC::LE);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("LIB");
    pub const ISO_SHORT_NAME: &str = "Lebanon";
    pub const ISO_LONG_NAME: &str = "The Lebanese Republic";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ar", "fr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ar", "fr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Lebanese");
    pub const NUMBER: &str = "422";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("(?:\\d{4})(?: ?(?:\\d{4}))?");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternAsia);
    pub const UN_LOCODE: &str = "LB";
    pub const UNOFFICIAL_NAME_LIST: &[&str] =
        &["Lebanon", "لبنان", "Libanon", "Liban", "Líbano", "レバノン"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Lebanon"),
        ("af", "Libanon"),
        ("ak", "Lebanon"),
        ("am", "ሑባኖስ"),
        ("an", "Lebanon"),
        ("ar", "لبنان"),
        ("as", "লেব\u{9be}নন"),
        ("ay", "Lebanon"),
        ("az", "Livan"),
        ("ba", "Lebanon"),
        ("be", "Ліван"),
        ("bg", "Лебанон"),
        ("bi", "Lebanon"),
        ("bn", "লেব\u{9be}নন"),
        ("bn_IN", "লেব\u{9be}নন"),
        ("br", "Liban"),
        ("bs", "Liban"),
        ("ca", "Líban"),
        ("ce", "Ливан"),
        ("ch", "Lebanon"),
        ("cs", "Libanon"),
        ("cv", "Ливан"),
        ("cy", "Lebanon"),
        ("da", "Libanon"),
        ("de", "Libanon"),
        ("dv", "ލ\u{7aa}ބ\u{7aa}ނ\u{7a7}ނ\u{7b0}"),
        ("dz", "ལ\u{f7a}་བ་ན\u{f71}\u{f7c}ན།"),
        ("ee", "Lebanon"),
        ("el", "Λίβανος"),
        ("en", "Lebanon"),
        ("eo", "Libano"),
        ("es", "Líbano"),
        ("et", "Liibanon"),
        ("eu", "Libano"),
        ("fa", "لبنان"),
        ("ff", "Lebanon"),
        ("fi", "Libanon"),
        ("fo", "Libanon"),
        ("fr", "Liban"),
        ("fy", "Libanon"),
        ("ga", "An Liobáin"),
        ("gl", "Líbano"),
        ("gn", "Lebanon"),
        ("gu", "લ\u{ac7}બ\u{ac7}નોન"),
        ("gv", "Yn Livaan"),
        ("ha", "Lebanon"),
        ("he", "לבנון"),
        ("hi", "ल\u{947}बनान"),
        ("hr", "Libanon"),
        ("ht", "Liban"),
        ("hu", "Libanon"),
        ("hy", "Լիբանան"),
        ("ia", "Libano"),
        ("id", "Lebanon"),
        ("io", "Libano"),
        ("is", "Líbanon"),
        ("it", "Libano"),
        ("iu", "Lebanon"),
        ("ja", "レバノン"),
        ("ka", "ლიბანი"),
        ("ki", "Lebanon"),
        ("kk", "Ливан"),
        ("kl", "Lebanon"),
        ("km", "ល\u{17b8}បង\u{17cb}"),
        ("kn", "ಲ\u{cc6}ಬನನ\u{ccd}"),
        ("ko", "레바논"),
        ("ku", "Lubnan"),
        ("kv", "Ливан"),
        ("kw", "Lebnon"),
        ("ky", "Ливан"),
        ("lo", "Lebanon"),
        ("lt", "Libanas"),
        ("lv", "Libāna"),
        ("mi", "Lebanon"),
        ("mk", "Либан"),
        ("ml", "ലെബനോണ\u{d4d}\u{200d}"),
        ("mn", "Либони"),
        ("mr", "ल\u{947}ब\u{947}नॉन"),
        ("ms", "Lubnan"),
        ("mt", "Libanu"),
        (
            "my",
            "လက\u{103a}ဘန\u{103d}န\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Ribanon"),
        ("nb", "Libanon"),
        ("ne", "ल\u{947}बनान"),
        ("nl", "Libanon"),
        ("nn", "Libanon"),
        ("nv", "Lebanon"),
        ("oc", "Liban"),
        ("or", "ଲେବ\u{b3e}ନୋନ"),
        ("pa", "ਲੀਬਨਾਨ"),
        ("pi", "ल\u{947}बनान"),
        ("pl", "Liban"),
        ("ps", "لبنان"),
        ("pt", "Líbano"),
        ("pt_BR", "Líbano"),
        ("ro", "Liban"),
        ("ru", "Ливан"),
        ("rw", "Libani"),
        ("sc", "Lìbanu"),
        ("sd", "Lebanon"),
        ("si", "ලෙබනන"),
        ("sk", "Libanon"),
        ("sl", "Libanon"),
        ("so", "Lubnaan"),
        ("sq", "Liban"),
        ("sr", "Либан"),
        ("sv", "Libanon"),
        ("sw", "Lebanon"),
        ("ta", "லெபன\u{bbe}ன\u{bcd}"),
        ("te", "ల\u{c46}బన\u{c3e}న\u{c4d}"),
        ("tg", "Лубнон"),
        ("th", "เลบานอน"),
        ("ti", "ሊባኖስ"),
        ("tk", "Liwan"),
        ("tl", "Lebanon"),
        ("tr", "Lübnan"),
        ("tt", "Лебанон"),
        ("ug", "لىۋان"),
        ("uk", "Ліван"),
        ("ur", "لبنان"),
        ("uz", "Livan"),
        ("ve", "Lebanon"),
        ("vi", "Le-ba-non"),
        ("wa", "Liban"),
        ("wo", "Libãa"),
        ("xh", "Lebanon"),
        ("yo", "Lẹ\u{301}bánọ\u{301}nì"),
        ("zh_CN", "黎巴嫩"),
        ("zh_HK", "黎巴嫩"),
        ("zh_TW", "黎巴嫩"),
        ("zu", "Lebanon"),
    ];
    #[cfg(all(feature = "lb", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 33.854721;
        pub const LONGITUDE: f64 = 35.862285;
        pub const MAX_LATITUDE: f64 = 34.69209;
        pub const MAX_LONGITUDE: f64 = 36.62372;
        pub const MIN_LATITUDE: f64 = 33.0550256;
        pub const MIN_LONGITUDE: f64 = 35.0711001;
        pub const NORTHEAST_LATITUDE: f64 = 34.69209;
        pub const NORTHEAST_LONGITUDE: f64 = 36.62372;
        pub const SOUTHWEST_LATITUDE: f64 = 33.0550256;
        pub const SOUTHWEST_LONGITUDE: f64 = 35.0711001;
    }
}
#[cfg(all(feature = "lb", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 33.854721,
            longitude: 35.862285,
            max_latitude: 34.69209,
            max_longitude: 36.62372,
            min_latitude: 33.0550256,
            min_longitude: 35.0711001,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 34.69209,
                    longitude: 36.62372,
                },
                southwest: CountryGeoBound {
                    latitude: 33.0550256,
                    longitude: 35.0711001,
                },
            },
        }
    }
}

#[cfg(all(feature = "lb", feature = "subdivisions"))]
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
                    "AK",
                    Subdivision{
                        name: "AK",
                        country_alpha2: Alpha2::LB,
                        code: "AK",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة عكار"), ("ccp", "𑄃\u{11127}𑄇\u{11134}𑄇\u{11127}𑄢\u{11134}"), ("ceb", "Mohafazat Aakkâr"), ("de", "Gouvernement Akkar"), ("en", "Akkar"), ("es", "Gobernación de Akkar"), ("he", "מחוז עכאר"), ("hu", "Akkár kormányzóság"), ("it", "Governatorato di Akkar"), ("ja", "アッカール県"), ("ka", "აკარის მუჰაფაზა"), ("ko", "아카르 주"), ("ru", "Аккар"), ("sv", "Mohafazat Aakkâr"), ("zh", "阿卡省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "AS",
                    Subdivision{
                        name: "AS",
                        country_alpha2: Alpha2::LB,
                        code: "AS",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الشمال"), ("bg", "Северен Ливан"), ("bn", "উত\u{9cd}তর গভর\u{9cd}নোরেট"), ("ca", "Governació del Líban-Nord"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134}"), ("ceb", "Mohafazat Liban-Nord"), ("da", "North Governorate"), ("de", "Gouvernement Nord-Libanon"), ("el", "Νορθ Γκοβερνοράτε"), ("en", "North"), ("es", "Líbano-Norte"), ("et", "Põhja-Liibanon"), ("eu", "Iparraldeko eskualdea"), ("fa", "استان شمالی لبنان"), ("fi", "North kuvernoraatti"), ("fr", "Gouvernorat du Nord"), ("gu", "નોર\u{acd}થ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "צפון לבנון"), ("hi", "उत\u{94d}तर प\u{94d}रान\u{94d}त"), ("hu", "Észak kormányzóság"), ("id", "Kegubernuran Utara"), ("it", "governatorato del Nord Libano"), ("ja", "北レバノン県"), ("ka", "ჩრდილოეთის მუჰაფაზა"), ("kn", "ಉತ\u{ccd}ತರ ರಾಜ\u{ccd}ಯಪಾಲ\u{cbf}ಕ\u{cc6}"), ("ko", "북부 주"), ("lt", "Šiaurės Libano muchafaza"), ("lv", "Ziemeļlibānas muhāfaza"), ("mr", "उत\u{94d}तर गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Pentadbiran Utara"), ("nb", "Nord-Libanon"), ("nl", "Noord"), ("no", "Nord-Libanon"), ("pl", "Dystrykt Północny"), ("pt", "Líbano Setentrional"), ("ro", "Guvernoratul Liban de Nord"), ("ru", "Северный Ливан"), ("si", "උත\u{dd4}ර\u{dd4} පළ\u{dcf}ත"), ("sv", "Mohafazat Liban-Nord"), ("ta", "வடக\u{bcd}கு கோவெர\u{bcd}னோரே"), ("te", "ఉత\u{c4d}తర గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เขตผ\u{e39}\u{e49}ว\u{e48}าเลบานอนเหน\u{e37}อ"), ("tr", "Şimal İli"), ("uk", "Північний Ліван"), ("ur", "محافظہ شمالی"), ("vi", "Tỉnh Bắc"), ("zh", "北部省")]),
                        unofficial_name_list: ["Ash Shamal", "North"].to_vec(),
                    }
                ),
                (
                    "BA",
                    Subdivision{
                        name: "BA",
                        country_alpha2: Alpha2::LB,
                        code: "BA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.8886289), longitude: Some(35.4954794), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بيروت"), ("be", "Правінцыя Бейрут"), ("bg", "Бейрут"), ("bn", "বৈর\u{9c1}ত গভর\u{9cd}নোরেট"), ("ccp", "𑄝\u{11130}𑄢\u{1112a}𑄖\u{11134}"), ("ceb", "Beyrouth"), ("da", "Beirut Governorate"), ("de", "Beirut"), ("el", "Μπεϊρούτ"), ("en", "Beirut"), ("es", "Gobernación de Beirut"), ("eu", "Beiruteko eskualdea"), ("fa", "استان بیروت"), ("fi", "Beiruin kuvernoraatti"), ("fr", "Gouvernorat de Beyrouth"), ("gu", "બ\u{ac7}ર\u{ac1}ત ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז ביירות"), ("hi", "ब\u{947}यर\u{942}त प\u{94d}रान\u{94d}त"), ("hy", "Բեյրութի մարզ"), ("id", "Kegubernuran Beirut"), ("it", "governatorato di Beirut"), ("ja", "ベイルート県"), ("ka", "ბეირუთის მუჰაფაზა"), ("kn", "ಬೈರುತ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "베이루트 주"), ("lt", "Beiruto muchafaza"), ("lv", "Beirūtas muhāfaza"), ("mr", "ब\u{947}र\u{942}त राज\u{94d}यपाल"), ("ms", "Beirut Governorate"), ("nb", "Beirut governementet"), ("nl", "Beiroet"), ("no", "Beirut governementet"), ("pl", "Gubernatorstwo Bejrut"), ("pt", "Beirute"), ("ro", "Guvernoratul Beirut"), ("ru", "Бейрут"), ("si", "බේර\u{dd6}ට\u{dca} පළ\u{dcf}ත"), ("sv", "Guvernementet Beirut"), ("ta", "பெய\u{bcd}ரூட\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "బ\u{c40}రట\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เบร\u{e38}ต"), ("tr", "Beyrut ili"), ("uk", "Бейрут (провінція)"), ("ur", "محافظہ بیروت"), ("vi", "Beirut"), ("zh", "贝鲁特省")]),
                        unofficial_name_list: ["Bayrout", "Bayrut", "Beirut", "Beyrout", "Beyrouth", "Beyrût"].to_vec(),
                    }
                ),
                (
                    "BH",
                    Subdivision{
                        name: "BH",
                        country_alpha2: Alpha2::LB,
                        code: "BH",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بعلبك الهرمل"), ("ccp", "𑄝\u{11127}𑄣\u{11134}𑄝𑄬𑄇\u{11134}-𑄦𑄢\u{11134}𑄟𑄬𑄣\u{11134}"), ("ceb", "Mohafazat Baalbek-Hermel"), ("de", "Gouvernement Baalbek-Hermel"), ("en", "Baalbek-Hermel"), ("es", "Gobernación de Baalbek - Hermel"), ("fr", "gouvernorat de Baalbek-Hermel"), ("he", "מחוז בעלבכ-הרמל"), ("hu", "Baalbek-Hermel kormányzóság"), ("it", "Governatorato di Baalbek-Hermel"), ("ka", "ბაალბექ-ჰერმელის მუჰაფაზა"), ("ko", "바알베크헤르멜 주"), ("ru", "Баальбек-Хирмиль"), ("sv", "Mohafazat Baalbek-Hermel"), ("ur", "محافظہ بعلبک الہرمل"), ("zh", "巴勒貝克-希爾米勒省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "BI",
                    Subdivision{
                        name: "BI",
                        country_alpha2: Alpha2::LB,
                        code: "BI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.7616036), longitude: Some(35.8835437), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "البقاع"), ("be", "Правінцыя Бекаа"), ("bg", "Бекаа"), ("ca", "Governació de la Bekaa"), ("ccp", "𑄝𑄬𑄇"), ("ceb", "Mohafazat Béqaa"), ("de", "Bekaa"), ("en", "Beqaa"), ("es", "Gobernación de Becá"), ("et", "Bekaa kubernerkond"), ("eu", "Bekaa eskualdea"), ("fa", "استان بقاع"), ("fi", "Bekaan laakson hallintoalue"), ("fr", "Gouvernorat de la Bekaa"), ("he", "אל-בקאע"), ("hi", "ब\u{947}क\u{93c}आ प\u{94d}रान\u{94d}त"), ("hu", "Bekaa kormányzóság"), ("hy", "Բեկաայի մարզ"), ("id", "Kegubernuran Beqaa"), ("it", "governatorato della Beqa’"), ("ja", "ベッカー県"), ("ka", "ბექაას მუჰაფაზა"), ("ko", "베카 주"), ("lt", "Bekos muchafaza"), ("nb", "Guvernementet Bekaa"), ("nl", "Beka"), ("no", "Guvernementet Bekaa"), ("pl", "Bekaa"), ("pt", "Beqaa"), ("ro", "Guvernoratul Beqaa"), ("ru", "Бекаа"), ("sl", "Beka"), ("sv", "Guvernementet Bekaa"), ("tr", "Bekaa ili"), ("uk", "Бекаа"), ("ur", "محافظہ بقاع"), ("vi", "Beqaa"), ("zh", "貝卡省")]),
                        unofficial_name_list: ["Bekaa"].to_vec(),
                    }
                ),
                (
                    "JA",
                    Subdivision{
                        name: "JA",
                        country_alpha2: Alpha2::LB,
                        code: "JA",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة الجنوب"), ("be", "Правінцыя Паўднёвы Ліван"), ("bg", "Южен Ливан"), ("ca", "Governació del Líban-Sud"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134}"), ("ceb", "Mohafazat Liban-Sud"), ("de", "Gouvernement Süd-Libanon"), ("en", "South"), ("es", "Líbano-Sur"), ("et", "Lõuna-Liibanoni kubernerkond"), ("eu", "Hegoaldeko eskualdea"), ("fa", "استان جنوبی لبنان"), ("fi", "Etelä-Libanonin hallintoalue"), ("fr", "Gouvernorat du Sud-Liban"), ("he", "מחוז דרום לבנון"), ("hi", "दक\u{94d}षिण प\u{94d}रान\u{94d}त"), ("hu", "Dél kormányzóság"), ("hy", "Հարավային Լիբանան"), ("id", "Kegubernuran Selatan"), ("it", "governatorato del Sud Libano"), ("ja", "南レバノン県"), ("ka", "სამხრეთის მუჰაფაზა"), ("ko", "남부 주"), ("lt", "Pietų Libano muchafaza"), ("ms", "Pentadbiran Selatan"), ("nl", "Zuid"), ("pl", "Dystrykt Południowy"), ("pt", "Líbano Meridional"), ("ro", "Guvernoratul Libanul de Sud"), ("ru", "Южный Ливан"), ("sv", "Mohafazat Liban-Sud"), ("tr", "Cenub ili"), ("uk", "Південний Ліван (провінція)"), ("ur", "محافظہ جنوبی"), ("vi", "Tỉnh Nam, Liban"), ("zh", "南部省")]),
                        unofficial_name_list: ["South"].to_vec(),
                    }
                ),
                (
                    "JL",
                    Subdivision{
                        name: "JL",
                        country_alpha2: Alpha2::LB,
                        code: "JL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.9046635), longitude: Some(35.6969984), max_latitude: Some(34.2113502), min_latitude: Some(33.4916836), max_longitude: Some(36.0150637), min_longitude: Some(35.3858701)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جبل لبنان"), ("be", "Горны Ліван"), ("bg", "Горен Ливан"), ("bn", "ম\u{9be}উন\u{9cd}ট লেব\u{9be}নন গভর\u{9cd}নোরেট"), ("ccp", "𑄟\u{11127}𑄅\u{1112a}𑄚\u{11134}𑄑\u{11134} 𑄣𑄬𑄝𑄚\u{11127}𑄚\u{11134}"), ("da", "Mount Lebanon Governorate"), ("de", "Libanonberg"), ("el", "Μάουντ Λέμπανον"), ("en", "Mount Lebanon"), ("es", "Gobernación del Monte Líbano"), ("et", "Mägi-Liibanon"), ("eu", "Libano Mendiko eskualdea"), ("fa", "استان جبل لبنان"), ("fi", "Mount Lebanonin kuvernoraatti"), ("fr", "Gouvernorat du Mont-Liban"), ("gu", "માઉન\u{acd}ટ લ\u{ac7}બનોન ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז הר הלבנון"), ("hu", "Libanon-hegy kormányzóság"), ("hy", "Լեռնային Լիբանանի պրովինցիա"), ("id", "Kegubernuran Gunung Lebanon"), ("it", "governatorato del Monte Libano"), ("ja", "山岳レバノン県"), ("ka", "მთიანი ლიბანის მუჰაფაზა"), ("kn", "ಮ\u{ccc}ಂಟ\u{ccd} ಲ\u{cc6}ಬನಾನ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "레바논 산 주"), ("lt", "Kalnų Libano muchafaza"), ("lv", "Kalnu Libānas muhāfaza"), ("mr", "माउ\u{902}ट ल\u{947}बनॉन गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Mount Lebanon Governorate"), ("nb", "Libanonfjellene"), ("nl", "Libanongebergte"), ("no", "Libanonfjellene"), ("pl", "Gubernatorstwo Dżabal Lubnan"), ("pt", "Monte Líbano"), ("ro", "Guvernoratul Munții Liban"), ("ru", "Горный Ливан"), ("si", "මව\u{dd4}න\u{dca}ට\u{dca} ලෙබනන\u{dca} පළ\u{dcf}ත"), ("sv", "Libanonberget"), ("ta", "மவுண\u{bcd}ட\u{bcd} லெபன\u{bbe}ன\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "మ\u{c4c}ంట\u{c4d} ల\u{c46}బన\u{c3e}న\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เมาท\u{e4c} เลบานอน โกเวอโนเรท"), ("tr", "Cebel-i Lübnan İli"), ("uk", "Гірський Ліван"), ("ur", "محافظہ جبل لبنان"), ("vi", "Núi Liban"), ("zh", "黎巴嫩山省")]),
                        unofficial_name_list: ["Mount Lebanon"].to_vec(),
                    }
                ),
                (
                    "NA",
                    Subdivision{
                        name: "NA",
                        country_alpha2: Alpha2::LB,
                        code: "NA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.285507), longitude: Some(35.5000999), max_latitude: Some(33.5112517), min_latitude: Some(33.0549366), max_longitude: Some(35.86385569999999), min_longitude: Some(35.2418835)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "النبطية"), ("bg", "Набатия"), ("bn", "ন\u{9be}ব\u{9be}তিয\u{9bc}েহ গভর\u{9cd}নোরেট"), ("ccp", "𑄚\u{11127}𑄝\u{11127}𑄑\u{11128}𑄠𑄦\u{11134}"), ("ceb", "Mohafazat Nabatîyé"), ("cs", "Nabatíja"), ("da", "Nabatieh Governorate"), ("de", "Nabatäa"), ("el", "Ναμπατιέχ"), ("en", "Nabatieh"), ("es", "Gobernación de Nabatiye"), ("et", "An-Nabaţīyah’ kubernerkond"), ("eu", "Nabatiehko eskualdea"), ("fa", "استان نبطیه"), ("fi", "Nabatîyén alue"), ("fr", "Gouvernorat de Nabatieh"), ("gu", "નબ\u{ac7}ટીય\u{ac7}હ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז נבטייה"), ("hi", "नबतीय\u{947} प\u{94d}रान\u{94d}त"), ("hu", "Nabatijja kormányzóság"), ("hy", "Նաբատիայի մարզ"), ("id", "Kegubernuran Nabatiye"), ("it", "governatorato di Nabatiye"), ("ja", "ナバティーエ県"), ("ka", "ნაბათიეს მუჰაფაზა"), ("kn", "ನಬತ\u{cbf}ಹ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "나바티예 주"), ("lt", "Nabatijos muchafaza"), ("lv", "Nabatijas muhāfaza"), ("mr", "नबाटीय\u{947}च\u{947} गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Nabatieh Governorate"), ("nb", "Guvernementet Nabatiye"), ("nl", "Nabatiye"), ("no", "Guvernementet Nabatiye"), ("pl", "Gubernatorstwo An-Nabatija"), ("pt", "Nabatiye"), ("ro", "Guvernoratul Nabatiye"), ("ru", "Набатия"), ("si", "නබට\u{dd2}යෙහ\u{dca} පළ\u{dcf}ත"), ("sl", "Provinca Nabatieh"), ("sv", "Guvernementet Nabatiye"), ("ta", "நபடிஏஹ\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "న\u{c3e}బ\u{c3e}ట\u{c40} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "นาบาต\u{e35}เยะห\u{e4c} โกเวอโนเรท"), ("tr", "Nebatiye ili"), ("uk", "Набатія (провінція)"), ("ur", "محافظہ نبطیہ"), ("vi", "Nabatieh"), ("zh", "納巴泰省")]),
                        unofficial_name_list: ["Nabatiyeh"].to_vec(),
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
#[cfg(feature = "lb")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::LB,
        alpha3: Alpha3::LBN,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Asia,
        country_code: 961,
        currency_code: "LBP",
        gec: Some(GEC::LE),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("LIB"),
        iso_long_name: "The Lebanese Republic",
        iso_short_name: "Lebanon",
        official_language_list: ["ar", "fr"].to_vec(),
        spoken_language_list: ["ar", "fr"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "0",
        nationality: Some("Lebanese"),
        number: "422",
        postal_code: true,
        postal_code_format: Some("(?:\\d{4})(?: ?(?:\\d{4}))?"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::WesternAsia),
        un_locode: "LB",
        unofficial_name_list: ["Lebanon", "لبنان", "Libanon", "Liban", "Líbano", "レバノン"]
            .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Lebanon"),
            ("af", "Libanon"),
            ("ak", "Lebanon"),
            ("am", "ሑባኖስ"),
            ("an", "Lebanon"),
            ("ar", "لبنان"),
            ("as", "লেব\u{9be}নন"),
            ("ay", "Lebanon"),
            ("az", "Livan"),
            ("ba", "Lebanon"),
            ("be", "Ліван"),
            ("bg", "Лебанон"),
            ("bi", "Lebanon"),
            ("bn", "লেব\u{9be}নন"),
            ("bn_IN", "লেব\u{9be}নন"),
            ("br", "Liban"),
            ("bs", "Liban"),
            ("ca", "Líban"),
            ("ce", "Ливан"),
            ("ch", "Lebanon"),
            ("cs", "Libanon"),
            ("cv", "Ливан"),
            ("cy", "Lebanon"),
            ("da", "Libanon"),
            ("de", "Libanon"),
            ("dv", "ލ\u{7aa}ބ\u{7aa}ނ\u{7a7}ނ\u{7b0}"),
            ("dz", "ལ\u{f7a}་བ་ན\u{f71}\u{f7c}ན།"),
            ("ee", "Lebanon"),
            ("el", "Λίβανος"),
            ("en", "Lebanon"),
            ("eo", "Libano"),
            ("es", "Líbano"),
            ("et", "Liibanon"),
            ("eu", "Libano"),
            ("fa", "لبنان"),
            ("ff", "Lebanon"),
            ("fi", "Libanon"),
            ("fo", "Libanon"),
            ("fr", "Liban"),
            ("fy", "Libanon"),
            ("ga", "An Liobáin"),
            ("gl", "Líbano"),
            ("gn", "Lebanon"),
            ("gu", "લ\u{ac7}બ\u{ac7}નોન"),
            ("gv", "Yn Livaan"),
            ("ha", "Lebanon"),
            ("he", "לבנון"),
            ("hi", "ल\u{947}बनान"),
            ("hr", "Libanon"),
            ("ht", "Liban"),
            ("hu", "Libanon"),
            ("hy", "Լիբանան"),
            ("ia", "Libano"),
            ("id", "Lebanon"),
            ("io", "Libano"),
            ("is", "Líbanon"),
            ("it", "Libano"),
            ("iu", "Lebanon"),
            ("ja", "レバノン"),
            ("ka", "ლიბანი"),
            ("ki", "Lebanon"),
            ("kk", "Ливан"),
            ("kl", "Lebanon"),
            ("km", "ល\u{17b8}បង\u{17cb}"),
            ("kn", "ಲ\u{cc6}ಬನನ\u{ccd}"),
            ("ko", "레바논"),
            ("ku", "Lubnan"),
            ("kv", "Ливан"),
            ("kw", "Lebnon"),
            ("ky", "Ливан"),
            ("lo", "Lebanon"),
            ("lt", "Libanas"),
            ("lv", "Libāna"),
            ("mi", "Lebanon"),
            ("mk", "Либан"),
            ("ml", "ലെബനോണ\u{d4d}\u{200d}"),
            ("mn", "Либони"),
            ("mr", "ल\u{947}ब\u{947}नॉन"),
            ("ms", "Lubnan"),
            ("mt", "Libanu"),
            (
                "my",
                "လက\u{103a}ဘန\u{103d}န\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Ribanon"),
            ("nb", "Libanon"),
            ("ne", "ल\u{947}बनान"),
            ("nl", "Libanon"),
            ("nn", "Libanon"),
            ("nv", "Lebanon"),
            ("oc", "Liban"),
            ("or", "ଲେବ\u{b3e}ନୋନ"),
            ("pa", "ਲੀਬਨਾਨ"),
            ("pi", "ल\u{947}बनान"),
            ("pl", "Liban"),
            ("ps", "لبنان"),
            ("pt", "Líbano"),
            ("pt_BR", "Líbano"),
            ("ro", "Liban"),
            ("ru", "Ливан"),
            ("rw", "Libani"),
            ("sc", "Lìbanu"),
            ("sd", "Lebanon"),
            ("si", "ලෙබනන"),
            ("sk", "Libanon"),
            ("sl", "Libanon"),
            ("so", "Lubnaan"),
            ("sq", "Liban"),
            ("sr", "Либан"),
            ("sv", "Libanon"),
            ("sw", "Lebanon"),
            ("ta", "லெபன\u{bbe}ன\u{bcd}"),
            ("te", "ల\u{c46}బన\u{c3e}న\u{c4d}"),
            ("tg", "Лубнон"),
            ("th", "เลบานอน"),
            ("ti", "ሊባኖስ"),
            ("tk", "Liwan"),
            ("tl", "Lebanon"),
            ("tr", "Lübnan"),
            ("tt", "Лебанон"),
            ("ug", "لىۋان"),
            ("uk", "Ліван"),
            ("ur", "لبنان"),
            ("uz", "Livan"),
            ("ve", "Lebanon"),
            ("vi", "Le-ba-non"),
            ("wa", "Liban"),
            ("wo", "Libãa"),
            ("xh", "Lebanon"),
            ("yo", "Lẹ\u{301}bánọ\u{301}nì"),
            ("zh_CN", "黎巴嫩"),
            ("zh_HK", "黎巴嫩"),
            ("zh_TW", "黎巴嫩"),
            ("zu", "Lebanon"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}