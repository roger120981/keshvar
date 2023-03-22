// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Mauritius

#[cfg(all(feature = "mu", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::MU;
    pub const ALPHA3: Alpha3 = Alpha3::MUS;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 230;
    pub const CURRENCY_CODE: &str = "MUR";
    pub const GEC: Option<GEC> = Some(GEC::MP);
    pub const INTERNATIONAL_PREFIX: &str = "020";
    pub const IOC: Option<&str> = Some("MRI");
    pub const ISO_SHORT_NAME: &str = "Mauritius";
    pub const ISO_LONG_NAME: &str = "The Republic of Mauritius";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Mauritian");
    pub const NUMBER: &str = "480";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{3}(?:\\d{2}|[A-Z]{2}\\d{3})");
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternAfrica);
    pub const UN_LOCODE: &str = "MU";
    pub const UNOFFICIAL_NAME_LIST: &[&str] =
        &["Mauritius", "Île Maurice", "Mauricio", "モーリシャス"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Mauritius"),
        ("af", "Mauritius"),
        ("ak", "Mauritius"),
        ("am", "ሢሩሸስ"),
        ("an", "Mauritius"),
        ("ar", "موريشيوس"),
        ("as", "মৰিছ\u{9be}ছ"),
        ("ay", "Mauritius"),
        ("az", "Mauritius"),
        ("ba", "Mauritius"),
        ("be", "Маўрыкій"),
        ("bg", "Мавриций"),
        ("bi", "Mauritius"),
        ("bn", "মরিশ\u{9be}স"),
        ("bn_IN", "মরিশ\u{9be}স"),
        ("br", "Maoris"),
        ("bs", "Mauricijus"),
        ("ca", "Maurici"),
        ("ce", "Маврики"),
        ("ch", "Mauritius"),
        ("cs", "Mauricius"),
        ("cv", "Маврики"),
        ("cy", "Mauritius"),
        ("da", "Mauritius"),
        ("de", "Mauritius"),
        ("dv", "މ\u{7ae}ރ\u{7a8}ޝ\u{7a6}ސ\u{7b0}"),
        ("dz", "མའ\u{f74}་ར\u{f72}་ཤ\u{f72}་ཡ\u{f71}ས\u{f72}།"),
        ("ee", "Mauritius"),
        ("el", "Μαυρίκιος"),
        ("en", "Mauritius"),
        ("eo", "Maŭricio"),
        ("es", "Mauricio"),
        ("et", "Mauritius"),
        ("eu", "Maurizio"),
        ("fa", "موریتیوس"),
        ("ff", "Mauritius"),
        ("fi", "Mauritius"),
        ("fo", "Móritius"),
        ("fr", "Maurice"),
        ("fy", "Mauritsius"),
        ("ga", "Oileán Mhuirís"),
        ("gl", "Mauricio"),
        ("gn", "Mauritius"),
        ("gu", "મોરિશિયસ"),
        ("gv", "Ellan Wirrish"),
        ("ha", "Mauritius"),
        ("he", "מאוריציוס"),
        ("hi", "मॉरिशस"),
        ("hr", "Mauricijus"),
        ("ht", "Moris"),
        ("hu", "Mauritius"),
        ("hy", "Մավրիտոս"),
        ("ia", "Mauritio"),
        ("id", "Mauritius"),
        ("io", "Maurico"),
        ("is", "Máritíus"),
        ("it", "Maurizio"),
        ("iu", "Mauritius"),
        ("ja", "モーリシャス"),
        ("ka", "მავრიკია"),
        ("ki", "Mauritius"),
        ("kk", "Маврикий"),
        ("kl", "Mauritius"),
        ("km", "ម\u{17c9}\u{17bc}រ\u{17b8}ទ\u{17bb}ស"),
        ("kn", "ಮಾರ\u{cbf}ಶಸ\u{ccd}"),
        ("ko", "모리셔스"),
        ("ku", "Morîtiyus"),
        ("kv", "Mauritius"),
        ("kw", "Ynys Morrys"),
        ("ky", "Маврикий"),
        ("lo", "Mauritius"),
        ("lt", "Mauricijus"),
        ("lv", "Maurīcija"),
        ("mi", "Maurituhi"),
        ("mk", "Маурициус"),
        ("ml", "മൌറീഷ\u{d4d}യസ\u{d4d}"),
        ("mn", "Mauritius"),
        ("mr", "मॉरिशस"),
        ("ms", "Mauritius"),
        ("mt", "Mawrizji"),
        (
            "my",
            "မောရစ\u{103a}ရ\u{103e}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Mauritius"),
        ("nb", "Mauritius"),
        ("ne", "माउरिसस"),
        ("nl", "Mauritius"),
        ("nn", "Mauritius"),
        ("nv", "Mauritius"),
        ("oc", "Maurici"),
        ("or", "ମରୀଶସ"),
        ("pa", "ਮਾਓਟੀਸ"),
        ("pi", "मारिशस"),
        ("pl", "Mauritius"),
        ("ps", "ماوریتوس"),
        ("pt", "Maurícia"),
        ("pt_BR", "Maurício"),
        ("ro", "Maurițius"),
        ("ru", "Маврикий"),
        ("rw", "Morise"),
        ("sc", "Maurìtzius"),
        ("sd", "Mauritius"),
        ("si", "මව\u{dd4}ර\u{dd2}ට\u{dd2}යස\u{dca}"),
        ("sk", "Maurícius"),
        ("sl", "Mavricij"),
        ("so", "Mauritius"),
        ("sq", "Mauricius"),
        ("sr", "Маурицијус"),
        ("sv", "Mauritius"),
        ("sw", "Mauritius"),
        ("ta", "மொர\u{bc0}ஷியஸ\u{bcd}"),
        ("te", "మ\u{c3e}ర\u{c3f}షస\u{c4d}"),
        ("tg", "Маврикий"),
        ("th", "มอร\u{e34}เช\u{e35}ยส"),
        ("ti", "ማሩሸስ"),
        ("tk", "Mauritius"),
        ("tl", "Mauritius"),
        ("tr", "Mauritius"),
        ("tt", "Мауритиус"),
        ("ug", "ماۋرىتىئۇس"),
        ("uk", "Маврикій"),
        ("ur", "موریشس"),
        ("uz", "Mavritsiya"),
        ("ve", "Mauritius"),
        ("vi", "Mô-ri-sơ-xợ"),
        ("wa", "Iye Môrice"),
        ("wo", "Móoris"),
        ("xh", "Mauritius"),
        ("yo", "Mọ\u{301}rísì"),
        ("zh_CN", "毛里求斯"),
        ("zh_HK", "毛里裘斯"),
        ("zh_TW", "模里西斯"),
        ("zu", "IMorishisi"),
    ];
    #[cfg(all(feature = "mu", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -20.348404;
        pub const LONGITUDE: f64 = 57.55215200000001;
        pub const MAX_LATITUDE: f64 = -10.0878538;
        pub const MAX_LONGITUDE: f64 = 63.80859390000001;
        pub const MIN_LATITUDE: f64 = -20.7458403;
        pub const MIN_LONGITUDE: f64 = 56.3159179;
        pub const NORTHEAST_LATITUDE: f64 = -10.0878538;
        pub const NORTHEAST_LONGITUDE: f64 = 63.80859390000001;
        pub const SOUTHWEST_LATITUDE: f64 = -20.7458403;
        pub const SOUTHWEST_LONGITUDE: f64 = 56.3159179;
    }
}
#[cfg(all(feature = "mu", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -20.348404,
            longitude: 57.55215200000001,
            max_latitude: -10.0878538,
            max_longitude: 63.80859390000001,
            min_latitude: -20.7458403,
            min_longitude: 56.3159179,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: -10.0878538,
                    longitude: 63.80859390000001,
                },
                southwest: CountryGeoBound {
                    latitude: -20.7458403,
                    longitude: 56.3159179,
                },
            },
        }
    }
}

#[cfg(all(feature = "mu", feature = "subdivisions"))]
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
                    "AG",
                    Subdivision{
                        name: "AG",
                        country_alpha2: Alpha2::MU,
                        code: "AG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-10.4681988), longitude: Some(56.690672), max_latitude: Some(-10.343819), min_latitude: Some(-10.4908943), max_longitude: Some(56.7038727), min_longitude: Some(56.58613399999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Dependency,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أغاليغا"), ("be", "Архіпелаг Агалега"), ("bn", "আগ\u{9be}লেগ\u{9be}"), ("ca", "Agalega"), ("ccp", "𑄃𑄉𑄣𑄬𑄉"), ("ceb", "Agalega Islands"), ("cs", "Agalegské ostrovy"), ("da", "Agaléga"), ("de", "Agalega-Inseln"), ("el", "Αγκαλέγκα"), ("en", "Agaléga"), ("es", "Islas Agalega"), ("et", "Agalega"), ("fa", "جزایر آگالگا"), ("fi", "Agaléga"), ("fr", "Agaléga"), ("gl", "Agalega"), ("gu", "અગાલ\u{ac7}ગા"), ("hi", "अपाल\u{947}गा"), ("id", "Agaléga"), ("it", "Agalega"), ("ja", "アガレガ諸島"), ("ka", "აგალეგა"), ("kn", "ಅಗಾಲ\u{cc6}ಗಾ"), ("ko", "아갈레가 제도"), ("lt", "Agalega"), ("lv", "Agalega"), ("mr", "अगाल\u{947}गा"), ("ms", "Agalega"), ("nb", "Agalegaøyene"), ("nl", "Agalega-eilanden"), ("no", "Agalegaøyene"), ("pl", "Wyspy Agalega"), ("pt", "Agalega"), ("ro", "Insulele Agalega"), ("ru", "Агалега"), ("si", "අගලේග\u{dcf}"), ("sr", "Агалега острва"), ("sr_Latn", "Agalega ostrva"), ("sv", "Agalega"), ("sw", "Agalega"), ("ta", "அகலேக\u{bbe}"), ("te", "అగ\u{c3e}ల\u{c46}గ\u{c3e}"), ("th", "อกาเลก\u{e49}า"), ("tr", "Agaléga Adaları"), ("uk", "Агалега"), ("ur", "آگالگا"), ("vi", "Agaléga"), ("yo", "Àwọn Erékùṣù Agalega"), ("yo_BJ", "Àwɔn Erékùshù Agalega"), ("yue", "阿加萊加群島"), ("yue_Hans", "阿加莱加群岛"), ("zh", "阿加萊加群島")]),
                        unofficial_name_list: ["Agalega Islands"].to_vec(),
                    }
                ),
                (
                    "BL",
                    Subdivision{
                        name: "BL",
                        country_alpha2: Alpha2::MU,
                        code: "BL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-20.34756), longitude: Some(57.3651898), max_latitude: Some(-20.3339857), min_latitude: Some(-20.3596977), max_longitude: Some(57.3702022), min_longitude: Some(57.3608296)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "النهر الأسود"), ("bn", "রিভেরি নয\u{9bc}রে জেল\u{9be}"), ("ca", "Rivière Noire"), ("ccp", "𑄢\u{11128}𑄞\u{11128}𑄠𑄢\u{11134} 𑄚\u{11130}𑄠𑄢\u{11134}"), ("ceb", "Black River District"), ("da", "Riviere Noire District"), ("de", "Black River"), ("el", "Μπλακ Ρίβερ"), ("en", "Rivière Noire"), ("es", "Distrito de Black River"), ("fi", "Rivière Noiren kaupunginosa"), ("fr", "Rivière Noire"), ("gu", "રિવિએર નોઇર જિલ\u{acd}લો"), ("hi", "रिव\u{947}यर नोयर जिला"), ("id", "Distrik Rivière Noire"), ("it", "Distretto di Black River"), ("ja", "ブラックリバー県"), ("jv", "Black River"), ("ka", "რივერ-ნუარი"), ("kn", "ರ\u{cbf}ವ\u{cbf}ಯರ\u{ccd} ನ\u{cc2}ರ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "블랙리버 구"), ("lt", "Blek Riverio rajonas"), ("lv", "Blekriveras distrikts"), ("mr", "रिव\u{94d}हिएर नोयर जिल\u{94d}हा"), ("ms", "Riviere Noire District"), ("nb", "Black River"), ("nl", "Black River"), ("no", "Black River"), ("pl", "Black River"), ("pt", "Black River"), ("ro", "Districtul Black River"), ("ru", "Ривьер-Нуар"), ("si", "ර\u{dd2}වය\u{dd2}රේ නොය\u{dd2}රේ ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Блек Ривер Маурицијус"), ("sr_Latn", "Blek River Mauricijus"), ("sv", "Black River"), ("ta", "றிவைரே நோயிரே ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ర\u{c3f}వ\u{c3f}యర\u{c4d} న\u{c4b}యర\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตร\u{e34}เว\u{e35}ยเรน\u{e31}วร\u{e4c}"), ("tr", "Black River Bölgesi"), ("uk", "Район Рівʼер-Нуар"), ("ur", "ریویر نوار ضلع"), ("vi", "Quận Rivière Noire"), ("yue", "黑河區"), ("yue_Hans", "黑河区"), ("zh", "黑河區")]),
                        unofficial_name_list: ["Black River"].to_vec(),
                    }
                ),
                (
                    "CC",
                    Subdivision{
                        name: "CC",
                        country_alpha2: Alpha2::MU,
                        code: "CC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-16.6333333), longitude: Some(59.6333333), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Dependency,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كارغادوس كارايوس"), ("be", "Каргадас-Карахас"), ("bg", "Каргадос-Карахос"), ("bn", "ক\u{9be}র\u{9cd}গ\u{9be}ডোস ক\u{9be}র\u{9be}জ\u{9c1}স"), ("ca", "Cargados Carajos"), ("ccp", "𑄇𑄢\u{11134}𑄉𑄓\u{1112e}𑄌\u{11134} 𑄇𑄢\u{11134}𑄎\u{1112e}𑄌\u{11134}"), ("cs", "Cargados Carajos"), ("da", "Cargados Carajos"), ("de", "Cargados-Carajos-Inseln"), ("el", "Καργκάντος Καράτζος (Σαίντ Μπράντον)"), ("en", "Cargados Carajos"), ("es", "Cargados Carajos"), ("et", "Cargados Carajos"), ("fi", "Cargados Carajos"), ("fr", "Saint-Brandon"), ("gl", "Cargados Carajos"), ("gu", "કાર\u{acd}ગાડોસ કારાજોસ"), ("hi", "कार\u{94d}गाडोस काराओस"), ("id", "Cargados Carajos"), ("it", "Cargados Carajos"), ("ja", "カルガドス・カラホス諸島"), ("ka", "კარგადოს-კარახოსი"), ("kn", "ಕಾರ\u{ccd}ಗಡೋಸ\u{ccd} ಕ\u{ccd}ಯಾರಾಜೊಸ\u{ccd}"), ("ko", "카르가도스 카라호스 제도"), ("lt", "Kargados Karachos"), ("lv", "Sentbrendona salas"), ("mr", "कार\u{94d}गाडो क\u{945}रजोस"), ("ms", "Cargados Carajos"), ("nb", "Cargados Carajos"), ("nl", "Cargados Carajos"), ("no", "Cargados Carajos"), ("pl", "Cargados Carajos"), ("pt", "Cargados Carajos"), ("ro", "Cargados Carajos"), ("ru", "Каргадос-Карахос"), ("si", "ක\u{dcf}ර\u{dca}ගඩොස\u{dca} කරජෝස\u{dca}"), ("sr", "Каргадос Карахос"), ("sr_Latn", "Kargados Karahos"), ("sv", "Cargados Carajos"), ("sw", "Cargados Carajos"), ("ta", "க\u{bbe}ர\u{bcd}கடோஸ\u{bcd} கர\u{bbe}ஜொஸ\u{bcd}"), ("te", "క\u{c3e}ర\u{c4d}గడ\u{c4b}స\u{c4d} క\u{c3e}ర\u{c3e}జ\u{c4b}స\u{c4d}"), ("th", "คาร\u{e4c}กาดอส คาราจอส"), ("tr", "Cargados-Carajos Adaları"), ("uk", "Каргадос-Карахос"), ("ur", "سینٹ برینڈن"), ("vi", "Cargados Carajos"), ("yo", "Cargados Carajos"), ("yo_BJ", "Cargados Carajos"), ("zh", "卡加多斯-卡拉若斯群岛")]),
                        unofficial_name_list: ["Cargados Carajos Shoals [Saint Brandon Islands]"].to_vec(),
                    }
                ),
                (
                    "FL",
                    Subdivision{
                        name: "FL",
                        country_alpha2: Alpha2::MU,
                        code: "FL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-20.2257836), longitude: Some(57.7119274), max_latitude: Some(-20.107765), min_latitude: Some(-20.3324041), max_longitude: Some(57.800854), min_longitude: Some(57.61632109999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فلاسك"), ("bn", "ফ\u{9cd}ল\u{9be}ক\u{9be} জেল\u{9be}"), ("ca", "Flacq"), ("ccp", "𑄜\u{11133}𑄣𑄇\u{11134}"), ("ceb", "Flacq District"), ("da", "Flacq District"), ("de", "Flacq"), ("el", "Φλακ"), ("en", "Flacq"), ("es", "Distrito de Flacq"), ("fa", "ناحیه فلاک"), ("fi", "Flacqin kaupunginosa"), ("fr", "Flacq"), ("gu", "ફ\u{acd}લ\u{ac7}ક જિલ\u{acd}લો"), ("he", "מחוז פלאק"), ("hi", "फ\u{94d}ल\u{948}क जिला"), ("id", "Distrik Flacq"), ("it", "Flacq"), ("ja", "フラック県"), ("ka", "ფლაკი"), ("kn", "ಫ\u{ccd}ಲಾಕ\u{ccd} ಡ\u{cbf}ಸ\u{ccd}ಟ\u{ccd}ರ\u{cbf}ಕ\u{ccd}ಟ\u{ccd}"), ("ko", "플라크 구"), ("lt", "Flako rajonas"), ("lv", "Flakas distrikts"), ("mr", "फ\u{94d}ल\u{945}क\u{94d}स जिल\u{94d}हा"), ("ms", "Flacq District"), ("nb", "Flacq"), ("nl", "Flacq"), ("no", "Flacq"), ("pl", "Flacq"), ("pt", "Flacq"), ("ro", "Districtul Flacq"), ("ru", "Флак"), ("si", "ෆ\u{dca}ල\u{dcf}ක\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Флак"), ("sr_Latn", "Flak"), ("sv", "Flacq"), ("ta", "பிளக\u{bcd}கு ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఫ\u{c4d}ల\u{c3e}క\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตเฟลค"), ("tr", "Flacq Bölgesi"), ("uk", "Район Флак"), ("ur", "فلاق ضلع"), ("vi", "Quận Flacq"), ("yue", "弗拉克區"), ("yue_Hans", "弗拉克区"), ("zh", "弗拉克區")]),
                        unofficial_name_list: ["Flacq"].to_vec(),
                    }
                ),
                (
                    "GP",
                    Subdivision{
                        name: "GP",
                        country_alpha2: Alpha2::MU,
                        code: "GP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-20.3851546), longitude: Some(57.6665742), max_latitude: Some(-20.313135), min_latitude: Some(-20.4977424), max_longitude: Some(57.77633979999999), min_longitude: Some(57.520995)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غراند بورت"), ("bn", "গ\u{9cd}র\u{9cd}য\u{9be}ন\u{9cd}ড পোর\u{9cd}ট জেল\u{9be}"), ("ccp", "𑄉\u{11133}𑄢𑄚\u{11133}𑄓\u{11134} 𑄛\u{1112e}𑄢\u{11134}𑄑\u{11134}"), ("ceb", "Grand Port District"), ("da", "Grand Port District"), ("de", "Grand Port"), ("el", "Γκραντ Πορτ"), ("en", "Grand Port"), ("es", "Distrito de Grand Port"), ("fa", "ناحیه گرند پورت"), ("fi", "Grand Portin kaupunginosa"), ("fr", "Grand Port"), ("gu", "ગ\u{acd}રાન\u{acd}ડ પોર\u{acd}ટ જિલ\u{acd}લો"), ("hi", "ग\u{94d}र\u{948}\u{902}ड पोर\u{94d}ट जिला"), ("id", "Distrik Grand Port"), ("it", "Grand Port"), ("ja", "グラン・ポール県"), ("ka", "გრანდ-პორტი"), ("kn", "ಗ\u{ccd}ರ\u{ccd}ಯಾಂಡ\u{ccd} ಪೋರ\u{ccd}ಟ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "그랑포르 구"), ("lt", "Gran Porto rajonas"), ("lv", "Granporas distrikts"), ("ml", "ഗ\u{d4d}ര\u{d3e}ൻഡ\u{d4d} പോർട\u{d4d}ട\u{d4d} ജില\u{d4d}ല"), ("mr", "ग\u{94d}र\u{901}ड पोर\u{94d}ट जिल\u{94d}हा"), ("ms", "Grand Port District"), ("nb", "Grand Port"), ("nl", "Grand Port"), ("no", "Grand Port"), ("pl", "Grand Port"), ("pt", "Grand Port"), ("ro", "Districtul Grand Port"), ("ru", "Гранд-Порт"), ("si", "ග\u{dca}\u{200d}රෑන\u{dca}ඩ\u{dca} පොර\u{dca}ට\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Гранд Порт"), ("sr_Latn", "Grand Port"), ("sv", "Grand Port"), ("ta", "கிர\u{bbe}ண\u{bcd}ட\u{bcd} போர\u{bcd}ட\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "గ\u{c4d}ర\u{c3e}ండ\u{c4d} ప\u{c4b}ర\u{c4d}ట\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "แกรนด\u{e4c} พอร\u{e4c}ท"), ("tr", "Grand Port Bölgesi"), ("uk", "Район Гранд-Порт"), ("ur", "گرینڈ پورٹ ضلع"), ("vi", "Quận Grand Port"), ("yue", "大港區"), ("yue_Hans", "大港区"), ("zh", "大港區")]),
                        unofficial_name_list: ["Grand Port"].to_vec(),
                    }
                ),
                (
                    "MO",
                    Subdivision{
                        name: "MO",
                        country_alpha2: Alpha2::MU,
                        code: "MO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-20.219), longitude: Some(57.496), max_latitude: Some(-20.2121464), min_latitude: Some(-20.2391669), max_longitude: Some(57.53708469999999), min_longitude: Some(57.48081200000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة موكا"), ("bn", "মক\u{9be} জেল\u{9be}"), ("ca", "Moka"), ("ccp", "𑄟\u{1112e}𑄇"), ("ceb", "Moka District"), ("da", "Moka"), ("de", "Moka"), ("el", "Μόκα"), ("en", "Moka"), ("es", "Distrito de Moka"), ("fa", "ناحیه موکا"), ("fi", "Moka"), ("fr", "Moka"), ("gu", "મોકા જિલ\u{acd}લો"), ("hi", "मोका जिला"), ("id", "Distrik Moka"), ("it", "Distretto di Moka"), ("ja", "モカ県"), ("ka", "მოკის ოლქი"), ("kn", "ಮೊಕಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "모카 구"), ("lt", "Mokos rajonas"), ("lv", "Mokas distrikts"), ("mr", "मोका जिल\u{94d}हा"), ("ms", "Moka District"), ("nb", "Moka"), ("nl", "Moka"), ("no", "Moka"), ("pl", "Moka"), ("pt", "Moka"), ("ro", "Districtul Moka"), ("ru", "Мока"), ("si", "මොක\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Мока Маурицијус"), ("sr_Latn", "Moka Mauricijus"), ("sv", "Moka"), ("ta", "மோக\u{bcd}க\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "మ\u{c4b}క\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "โมค\u{e48}า"), ("tr", "Moka Bölgesi"), ("uk", "Мока"), ("ur", "موکا ضلع"), ("vi", "Quận Moka"), ("yue", "莫卡區"), ("yue_Hans", "莫卡区"), ("zh", "莫卡區")]),
                        unofficial_name_list: ["Moka"].to_vec(),
                    }
                ),
                (
                    "PA",
                    Subdivision{
                        name: "PA",
                        country_alpha2: Alpha2::MU,
                        code: "PA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-20.116667), longitude: Some(57.58333299999999), max_latitude: Some(-20.0948687), min_latitude: Some(-20.1210344), max_longitude: Some(57.59561029999999), min_longitude: Some(57.5670416)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة بامبلموسز"), ("bn", "প\u{9be}মপ\u{9cd}লেমোসেস জেল\u{9be}"), ("ca", "Pamplemousses"), ("ccp", "𑄛𑄟\u{11134}𑄛\u{11133}𑄣\u{11128}𑄟\u{1112f}𑄥𑄬𑄌\u{11134}"), ("ceb", "Pamplemousses District"), ("da", "Pamplemousses"), ("de", "Pamplemousses"), ("el", "Παμπλεμούς"), ("en", "Pamplemousses"), ("es", "Distrito de Pamplemousses"), ("fa", "ناحیه پامپلموسس"), ("fi", "Pamplemoussesin kaupunginosa"), ("fr", "Pamplemousses"), ("gu", "પ\u{ac7}મ\u{acd}પલમાઉસ\u{ac7}સ જિલ\u{acd}લો"), ("hi", "प\u{948}म\u{94d}पलम\u{942}स\u{947}स जिला"), ("hr", "Pamplemousses"), ("id", "Distrik Pamplemousses"), ("it", "Pamplemousses"), ("ja", "パンプルムース県"), ("ka", "პამპლემუსი"), ("kn", "ಪಂಪ\u{ccd}ಮ\u{ccc}ಸಸ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "팜플레무스 구"), ("lt", "Pamplemuso rajonas"), ("lv", "Pamplmusas distrikts"), ("mr", "प\u{902}पमामाउस जिल\u{94d}हा"), ("ms", "Pamplemousses District"), ("nb", "Pamplemousses"), ("nl", "Pamplemousses"), ("no", "Pamplemousses"), ("pl", "Pamplemousses"), ("pt", "Pamplemousses"), ("ro", "Districtul Pamplemousses"), ("ru", "Памплемус"), ("si", "පැම\u{dca}ප\u{dca}ලේම\u{dd4}සෙස\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Distrikt Pamplemousses"), ("ta", "ப\u{bbe}ம\u{bcd}பிளேமோவ\u{bcd}ஸ\u{bcd}செஸ\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ప\u{c3e}ంప\u{c4d}ల\u{c3f}మ\u{c4b}స\u{c46}స\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "แพมเพ\u{e34}ลม\u{e39}สเซส"), ("tr", "Pamplemousses Bölgesi"), ("uk", "Район Памплемус"), ("ur", "پامپلیموس ضلع"), ("vi", "Quận Pamplemousses"), ("yue", "龐波慕斯區"), ("yue_Hans", "庞波慕斯区"), ("zh", "龐波慕斯區")]),
                        unofficial_name_list: ["Pamplemousses"].to_vec(),
                    }
                ),
                (
                    "PL",
                    Subdivision{
                        name: "PL",
                        country_alpha2: Alpha2::MU,
                        code: "PL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-20.166667), longitude: Some(57.516667), max_latitude: Some(-20.124287), min_latitude: Some(-20.2016751), max_longitude: Some(57.5659989), min_longitude: Some(57.42768760000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄛\u{1112e}𑄢\u{11134}𑄑\u{11134} 𑄣\u{1112a}𑄃\u{11128}𑄌\u{11134} 𑄎𑄬𑄣"), ("da", "Port Louis District"), ("de", "Port Louis"), ("en", "Port Louis District"), ("es", "Distrito de Port Louis"), ("fa", "ناحیه پورت لوئیس"), ("fr", "Port Louis"), ("gl", "Port Louis"), ("it", "Distretto di Port Louis"), ("ja", "ポートルイス県"), ("ka", "პორტ-ლუის ოლქი"), ("ko", "포트루이스 구"), ("lt", "Port Luiso rajonas"), ("nb", "Port Louis"), ("nl", "Port Louis"), ("no", "Port Louis"), ("pl", "Port Louis"), ("pt", "Port Louis"), ("ro", "Districtul Port Louis"), ("ru", "Порт-Луи"), ("sv", "Port Louis"), ("ta", "போர\u{bcd}ட\u{bcd} லூயிஸ\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("tr", "Port Louis ilçesi"), ("uk", "Порт-Луї²"), ("ur", "پورٹ لوئس ضلع"), ("yue", "路易港區"), ("yue_Hans", "路易港区"), ("zh", "路易港區")]),
                        unofficial_name_list: ["Port Louis City"].to_vec(),
                    }
                ),
                (
                    "PW",
                    Subdivision{
                        name: "PW",
                        country_alpha2: Alpha2::MU,
                        code: "PW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-20.3054872), longitude: Some(57.48535609999999), max_latitude: Some(-20.187828), min_latitude: Some(-20.4237189), max_longitude: Some(57.5981641), min_longitude: Some(57.43777499999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بلينز ويلهمز"), ("bn", "প\u{9cd}লেইন\u{9cd}স উইলহেম জেল\u{9be}"), ("ca", "Plaines Wilhems"), ("ccp", "𑄛\u{11133}𑄣\u{1112d}𑄚𑄬𑄌\u{11134} 𑄅\u{1112a}𑄃\u{11128}𑄣\u{11134}𑄦𑄬𑄟\u{11134}"), ("ceb", "Plaines Wilhems District"), ("da", "Plaines Wilhems District"), ("de", "Plaines Wilhems"), ("el", "Πλέινς Γουίλχεμς"), ("en", "Plaines Wilhems"), ("es", "Distrito de Plaines Wilhems"), ("fa", "ناحیه پلن ویلهم"), ("fi", "Plaines Wilhemsin kaupunginosa"), ("fr", "Plaines Wilhems"), ("gu", "પ\u{acd}લ\u{ac7}ઇન\u{acd}સ વિલહ\u{ac7}મ\u{acd}સ જિલ\u{acd}લો"), ("hi", "प\u{94d}ल\u{947}न\u{94d}स विल\u{94d}ह\u{947}म\u{94d}स जिला"), ("id", "Distrik Plaines Wilhems"), ("it", "Plaines Wilhems"), ("ja", "プレーン・ウィルヘルム県"), ("ka", "პლენ-ვილემი"), ("kn", "ಪ\u{ccd}ಲೈನ\u{ccd}ಸ\u{ccd} ವ\u{cbf}ಲ\u{ccd}ಹ\u{cc6}ಮ\u{ccd}ಸ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "플랭윌렐름 구"), ("lt", "Plen Vilemo rajonas"), ("lv", "Plēnvilemsas distrikts"), ("mr", "प\u{94d}ल\u{947}न\u{94d}स विल\u{94d}ह\u{947}म\u{94d}स जिल\u{94d}हा"), ("ms", "Plaines Wilhems District"), ("nb", "Plaines Wilhems"), ("nl", "Plaines Wilhems"), ("no", "Plaines Wilhems"), ("pl", "Plaines Wilhems"), ("pt", "Plaines Wilhems"), ("ro", "Districtul Plaines Wilhems"), ("ru", "Плен-Вилем"), ("si", "ප\u{dca}ලේන\u{dca}ස\u{dca} ව\u{dd2}ල\u{dca}හෙම\u{dca}ස\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Plaines Wilhems"), ("ta", "பிலெய\u{bcd}ன\u{bcd}சு வில\u{bcd}ஹெம\u{bcd}சு ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ప\u{c4d}ల\u{c46}య\u{c3f}న\u{c4d}స\u{c4d} వ\u{c3f}ల\u{c4d}హ\u{c46}మ\u{c4d}స\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตเพลนส\u{e4c} ว\u{e34}ลเฮล\u{e4c}ม"), ("tr", "Plaines Wilhems Bölgesi"), ("uk", "Район Плен-Вілем"), ("ur", "پلیئن ویلیمز ضلع"), ("vi", "Quận Plaines Wilhems"), ("yue", "威廉平原區"), ("yue_Hans", "威廉平原区"), ("zh", "威廉平原區")]),
                        unofficial_name_list: ["Plaines Wilhems"].to_vec(),
                    }
                ),
                (
                    "RO",
                    Subdivision{
                        name: "RO",
                        country_alpha2: Alpha2::MU,
                        code: "RO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-19.7245385), longitude: Some(63.4272185), max_latitude: Some(-19.6657049), min_latitude: Some(-19.7743364), max_longitude: Some(63.5035945), min_longitude: Some(63.328886)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Dependency,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Rodrigues"), ("ar", "رودريغوس"), ("be", "Востраў Радрыгес"), ("bn", "রডরিগ\u{9c1}য\u{9bc}েজ"), ("ca", "Illa de Rodrigues"), ("ccp", "𑄢\u{1112e}𑄓\u{11133}𑄢\u{11128}𑄉\u{1112a}𑄠𑄬𑄌\u{11134}"), ("ceb", "Rodrigues"), ("cs", "Rodrigues"), ("da", "Rodrigues"), ("de", "Rodrigues"), ("el", "Ροντρίγκες"), ("en", "Rodrigues"), ("es", "Rodrigues"), ("eu", "Rodrigues"), ("fa", "جزیره رودریگز"), ("fi", "Rodrigues"), ("fr", "Rodrigues"), ("gl", "Rodrigues"), ("gu", "રોડ\u{acd}રીગ\u{acd}ય\u{ac1}એસ"), ("he", "רודריגז"), ("hi", "रोड\u{94d}रिग\u{94d}ज\u{93c}"), ("id", "Rodrigues"), ("it", "Rodrigues"), ("ja", "ロドリゲス島"), ("ka", "როდრიგესი"), ("kn", "ರೊಡ\u{ccd}ರ\u{cbf}ಗಸ\u{ccd}"), ("ko", "로드리게스 섬"), ("lt", "Rodrigesas"), ("lv", "Rodrigesa"), ("mr", "रॉड\u{94d}रिग\u{94d}ज"), ("ms", "Rodrigues"), ("nb", "Rodrigues"), ("nl", "Rodrigues"), ("no", "Rodrigues"), ("pl", "Rodrigues"), ("pt", "Rodrigues"), ("ro", "Insula Rodrigues"), ("ru", "Родригес"), ("si", "රොඩ\u{dca}ර\u{dd2}ග\u{dd4}ස\u{dca}"), ("sr", "Родригез"), ("sr_Latn", "Rodrigez"), ("sv", "Rodrigues"), ("sw", "Rodrigues"), ("ta", "ரொட\u{bcd}ரிக\u{bcd}கஸ\u{bcd}"), ("te", "ర\u{c4b}డ\u{c4d}ర\u{c3f}గ\u{c4d}స\u{c4d}"), ("th", "เกาะโรดร\u{e35}ก"), ("tr", "Rodrigues"), ("uk", "Родрігес"), ("ur", "روڈریگس"), ("vi", "Rodrigues"), ("yo", "Rodrigues"), ("yo_BJ", "Rodrigues"), ("yue", "羅德里格斯島"), ("yue_Hans", "罗德里格斯岛"), ("zh", "罗德里格斯岛")]),
                        unofficial_name_list: ["Rodrigues Island"].to_vec(),
                    }
                ),
                (
                    "RR",
                    Subdivision{
                        name: "RR",
                        country_alpha2: Alpha2::MU,
                        code: "RR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-20.0905351), longitude: Some(57.6949177), max_latitude: Some(-20.0736276), min_latitude: Some(-20.1159055), max_longitude: Some(57.7115289), min_longitude: Some(57.6779652)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ريفير دو ريمبارت"), ("bn", "রিভেরি ডো রেম\u{9cd}প জেল\u{9be}"), ("ccp", "𑄢\u{11128}𑄞\u{11128}𑄠𑄢\u{11134} 𑄓\u{1112a} 𑄢\u{11128}𑄟\u{11134}𑄛\u{11127}𑄢\u{11134}𑄑\u{11134}"), ("ceb", "Rivière du Rempart District"), ("da", "Riviere du Rempart District"), ("de", "Rivière du Rempart"), ("el", "Ριβιέρ ντου Ρεμπάρτ"), ("en", "Rivière du Rempart"), ("es", "Distrito de Rivière du Rempart"), ("fa", "ناحیه ریویر دو رمپار"), ("fi", "Rivière du Rempartin kaupunginosa"), ("fr", "Rivière du Rempart"), ("gu", "રિવિએર\u{ac7} ડ\u{ac1} ર\u{ac7}મ\u{acd}પર\u{acd}ટ જિલ\u{acd}લો"), ("hi", "रिव\u{947}यर ड\u{942} र\u{947}म\u{94d}पार\u{94d}ट जिला"), ("id", "Distrik Rivière du Rempart"), ("it", "Rivière du Rempart"), ("ja", "リヴィエール・デュ・ランパール県"), ("ka", "რივიერ-დუ-რემპარი"), ("kn", "ರ\u{cbf}ವ\u{cbf}ಯ\u{cc6}ರ\u{cc6} ಡು ರ\u{cc6}ಮ\u{ccd}ಪಾಟ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "리비에르뒤랑파르 구"), ("lt", "Rivjer diu Ramparo rajonas"), ("lv", "Rivjēras di Ramparas distrikts"), ("mr", "रिव\u{94d}हिएर\u{947} ड\u{941} र\u{947}मर\u{94d}ट जिल\u{94d}हा"), ("ms", "Riviere du Rempart District"), ("nb", "Rivière du Rempart"), ("nl", "Rivière du Rempart"), ("no", "Rivière du Rempart"), ("pl", "Rivière du Rempart"), ("pt", "Rivière du Rempart"), ("ro", "Districtul Rivière du Rempart"), ("ru", "Ривьер-дю-Рампар"), ("si", "ර\u{dd2}වය\u{dd2}රේ ඩ\u{dd4} රෙම\u{dca}පර\u{dca}ට\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Rivière du Rempart"), ("ta", "றிவைரே டு ரெம\u{bcd}பர\u{bcd}ட\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ర\u{c3f}వ\u{c3f}యర\u{c4d} డూ ర\u{c46}ంప\u{c3e}ర\u{c4d}ట\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตร\u{e34}เว\u{e35}ยเร ด\u{e39} เรมพาร\u{e4c}ท"), ("tr", "Riviere du Rempart Bölgesi"), ("uk", "Район Рівʼер-дю-Рампар"), ("ur", "ریویر دو ریمپار ضلع"), ("vi", "Quận Rivière du Rempart"), ("yue", "朗帕河區"), ("yue_Hans", "朗帕河区"), ("zh", "朗帕河區")]),
                        unofficial_name_list: ["Rivière du Rempart"].to_vec(),
                    }
                ),
                (
                    "SA",
                    Subdivision{
                        name: "SA",
                        country_alpha2: Alpha2::MU,
                        code: "SA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-20.473953), longitude: Some(57.48535609999999), max_latitude: Some(-20.396972), min_latitude: Some(-20.5254986), max_longitude: Some(57.6193767), min_longitude: Some(57.37086850000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سافاني"), ("bn", "স\u{9be}ব\u{9be}নে জেল\u{9be}"), ("ca", "Savanne"), ("ccp", "𑄥𑄞𑄚\u{11134}"), ("ceb", "Savanne District"), ("da", "Savanne District"), ("de", "Savanne"), ("el", "Σαβάν"), ("en", "Savanne"), ("es", "Distrito de Savanne"), ("fa", "ناحیه ساوانه"), ("fi", "Savannen kaupunginosa"), ("fr", "Savanne"), ("gu", "સવાન\u{ac7} જિલ\u{acd}લો"), ("hi", "सवान\u{947} जिला"), ("id", "Distrik Savanne"), ("it", "Savanne"), ("ja", "サバンナ県"), ("ka", "სავანის ოლქი"), ("kn", "ಸವನ\u{ccd}ನ\u{cc6} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "사반 구"), ("lt", "Savano rajonas"), ("lv", "Savannas distrikts"), ("mr", "सवाना जिल\u{94d}हा"), ("ms", "Savanne District"), ("nb", "Savanne"), ("nl", "Savanne"), ("no", "Savanne"), ("pl", "Savanne"), ("pt", "Savanne"), ("ro", "Districtul Savanne"), ("ru", "Саван"), ("si", "සවන\u{dca}නේ ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Savanne"), ("ta", "சவன\u{bcd}னே ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "స\u{c3e}వ\u{c46}న\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตซาวาน"), ("tr", "Savanne Bölgesi"), ("uk", "Район Саван"), ("ur", "ساوانے ضلع"), ("vi", "Quận Savanne"), ("yue", "薩凡納區"), ("yue_Hans", "萨凡纳区"), ("zh", "薩凡納區")]),
                        unofficial_name_list: ["Savanne"].to_vec(),
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
#[cfg(feature = "mu")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::MU,
        alpha3: Alpha3::MUS,
        address_format: None,
        continent: Continent::Africa,
        country_code: 230,
        currency_code: "MUR",
        gec: Some(GEC::MP),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "020",
        ioc: Some("MRI"),
        iso_long_name: "The Republic of Mauritius",
        iso_short_name: "Mauritius",
        official_language_list: ["en"].to_vec(),
        spoken_language_list: ["en"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7].to_vec(),
        national_prefix: "None",
        nationality: Some("Mauritian"),
        number: "480",
        postal_code: true,
        postal_code_format: Some("\\d{3}(?:\\d{2}|[A-Z]{2}\\d{3})"),
        region: Some(Region::Africa),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternAfrica),
        un_locode: "MU",
        unofficial_name_list: ["Mauritius", "Île Maurice", "Mauricio", "モーリシャス"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Mauritius"),
            ("af", "Mauritius"),
            ("ak", "Mauritius"),
            ("am", "ሢሩሸስ"),
            ("an", "Mauritius"),
            ("ar", "موريشيوس"),
            ("as", "মৰিছ\u{9be}ছ"),
            ("ay", "Mauritius"),
            ("az", "Mauritius"),
            ("ba", "Mauritius"),
            ("be", "Маўрыкій"),
            ("bg", "Мавриций"),
            ("bi", "Mauritius"),
            ("bn", "মরিশ\u{9be}স"),
            ("bn_IN", "মরিশ\u{9be}স"),
            ("br", "Maoris"),
            ("bs", "Mauricijus"),
            ("ca", "Maurici"),
            ("ce", "Маврики"),
            ("ch", "Mauritius"),
            ("cs", "Mauricius"),
            ("cv", "Маврики"),
            ("cy", "Mauritius"),
            ("da", "Mauritius"),
            ("de", "Mauritius"),
            ("dv", "މ\u{7ae}ރ\u{7a8}ޝ\u{7a6}ސ\u{7b0}"),
            ("dz", "མའ\u{f74}་ར\u{f72}་ཤ\u{f72}་ཡ\u{f71}ས\u{f72}།"),
            ("ee", "Mauritius"),
            ("el", "Μαυρίκιος"),
            ("en", "Mauritius"),
            ("eo", "Maŭricio"),
            ("es", "Mauricio"),
            ("et", "Mauritius"),
            ("eu", "Maurizio"),
            ("fa", "موریتیوس"),
            ("ff", "Mauritius"),
            ("fi", "Mauritius"),
            ("fo", "Móritius"),
            ("fr", "Maurice"),
            ("fy", "Mauritsius"),
            ("ga", "Oileán Mhuirís"),
            ("gl", "Mauricio"),
            ("gn", "Mauritius"),
            ("gu", "મોરિશિયસ"),
            ("gv", "Ellan Wirrish"),
            ("ha", "Mauritius"),
            ("he", "מאוריציוס"),
            ("hi", "मॉरिशस"),
            ("hr", "Mauricijus"),
            ("ht", "Moris"),
            ("hu", "Mauritius"),
            ("hy", "Մավրիտոս"),
            ("ia", "Mauritio"),
            ("id", "Mauritius"),
            ("io", "Maurico"),
            ("is", "Máritíus"),
            ("it", "Maurizio"),
            ("iu", "Mauritius"),
            ("ja", "モーリシャス"),
            ("ka", "მავრიკია"),
            ("ki", "Mauritius"),
            ("kk", "Маврикий"),
            ("kl", "Mauritius"),
            ("km", "ម\u{17c9}\u{17bc}រ\u{17b8}ទ\u{17bb}ស"),
            ("kn", "ಮಾರ\u{cbf}ಶಸ\u{ccd}"),
            ("ko", "모리셔스"),
            ("ku", "Morîtiyus"),
            ("kv", "Mauritius"),
            ("kw", "Ynys Morrys"),
            ("ky", "Маврикий"),
            ("lo", "Mauritius"),
            ("lt", "Mauricijus"),
            ("lv", "Maurīcija"),
            ("mi", "Maurituhi"),
            ("mk", "Маурициус"),
            ("ml", "മൌറീഷ\u{d4d}യസ\u{d4d}"),
            ("mn", "Mauritius"),
            ("mr", "मॉरिशस"),
            ("ms", "Mauritius"),
            ("mt", "Mawrizji"),
            (
                "my",
                "မောရစ\u{103a}ရ\u{103e}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Mauritius"),
            ("nb", "Mauritius"),
            ("ne", "माउरिसस"),
            ("nl", "Mauritius"),
            ("nn", "Mauritius"),
            ("nv", "Mauritius"),
            ("oc", "Maurici"),
            ("or", "ମରୀଶସ"),
            ("pa", "ਮਾਓਟੀਸ"),
            ("pi", "मारिशस"),
            ("pl", "Mauritius"),
            ("ps", "ماوریتوس"),
            ("pt", "Maurícia"),
            ("pt_BR", "Maurício"),
            ("ro", "Maurițius"),
            ("ru", "Маврикий"),
            ("rw", "Morise"),
            ("sc", "Maurìtzius"),
            ("sd", "Mauritius"),
            ("si", "මව\u{dd4}ර\u{dd2}ට\u{dd2}යස\u{dca}"),
            ("sk", "Maurícius"),
            ("sl", "Mavricij"),
            ("so", "Mauritius"),
            ("sq", "Mauricius"),
            ("sr", "Маурицијус"),
            ("sv", "Mauritius"),
            ("sw", "Mauritius"),
            ("ta", "மொர\u{bc0}ஷியஸ\u{bcd}"),
            ("te", "మ\u{c3e}ర\u{c3f}షస\u{c4d}"),
            ("tg", "Маврикий"),
            ("th", "มอร\u{e34}เช\u{e35}ยส"),
            ("ti", "ማሩሸስ"),
            ("tk", "Mauritius"),
            ("tl", "Mauritius"),
            ("tr", "Mauritius"),
            ("tt", "Мауритиус"),
            ("ug", "ماۋرىتىئۇس"),
            ("uk", "Маврикій"),
            ("ur", "موریشس"),
            ("uz", "Mavritsiya"),
            ("ve", "Mauritius"),
            ("vi", "Mô-ri-sơ-xợ"),
            ("wa", "Iye Môrice"),
            ("wo", "Móoris"),
            ("xh", "Mauritius"),
            ("yo", "Mọ\u{301}rísì"),
            ("zh_CN", "毛里求斯"),
            ("zh_HK", "毛里裘斯"),
            ("zh_TW", "模里西斯"),
            ("zu", "IMorishisi"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}