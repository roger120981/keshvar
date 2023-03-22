// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Chile

#[cfg(all(feature = "cl", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::CL;
    pub const ALPHA3: Alpha3 = Alpha3::CHL;
    pub const CONTINENT: Continent = Continent::SouthAmerica;
    pub const COUNTRY_CODE: usize = 56;
    pub const CURRENCY_CODE: &str = "CLP";
    pub const GEC: Option<GEC> = Some(GEC::CI);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("CHI");
    pub const ISO_SHORT_NAME: &str = "Chile";
    pub const ISO_LONG_NAME: &str = "The Republic of Chile";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["es"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["es"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Chilean");
    pub const NUMBER: &str = "152";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{7}");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthAmerica);
    pub const UN_LOCODE: &str = "CL";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Chile", "チリ", "Chili"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Chile"),
        ("af", "Chili"),
        ("ak", "Chile"),
        ("am", "ኁሑ"),
        ("an", "Chile"),
        ("ar", "تشيلي"),
        ("as", "চিলি"),
        ("ay", "Chile"),
        ("az", "Çili"),
        ("ba", "Chile"),
        ("be", "Чылі"),
        ("bg", "Чили"),
        ("bi", "Chile"),
        ("bn", "চিলি"),
        ("bn_IN", "চিলি"),
        ("br", "Chile"),
        ("bs", "Čile"),
        ("ca", "Xile"),
        ("ce", "Чили"),
        ("ch", "Chile"),
        ("cs", "Chile"),
        ("cv", "Чили"),
        ("cy", "Chile"),
        ("da", "Chile"),
        ("de", "Chile"),
        ("dv", "ޗ\u{7a8}ލ\u{7a9}"),
        ("dz", "ཅ\u{f72}་ལ\u{f72}།"),
        ("ee", "Chile"),
        ("el", "Χιλή"),
        ("en", "Chile"),
        ("eo", "Ĉilio"),
        ("es", "Chile"),
        ("et", "Tšiili"),
        ("eu", "Txile"),
        ("fa", "شیلی"),
        ("ff", "Ciile"),
        ("fi", "Chile"),
        ("fo", "Kili"),
        ("fr", "Chili"),
        ("fy", "Sily"),
        ("ga", "An tSile"),
        ("gl", "Chile"),
        ("gn", "Chile"),
        ("gu", "ચીલી"),
        ("gv", "Yn Çhillee"),
        ("ha", "Chile"),
        ("he", "צ'ילה"),
        ("hi", "चिली"),
        ("hr", "Čile"),
        ("ht", "Chili"),
        ("hu", "Chile"),
        ("hy", "Չիլի"),
        ("ia", "Chile"),
        ("id", "Chili"),
        ("io", "Chili"),
        ("is", "Síle"),
        ("it", "Cile"),
        ("iu", "ᓯᓕ"),
        ("ja", "チリ"),
        ("ka", "ჩილი"),
        ("ki", "Chile"),
        ("kk", "Чили"),
        ("kl", "Chile"),
        ("km", "ឈ\u{17b8}ល\u{17b8}"),
        ("kn", "ಚ\u{cbf}ಲ\u{cbf}"),
        ("ko", "칠레"),
        ("ku", "Sîlî"),
        ("kv", "Чили"),
        ("kw", "Chile"),
        ("ky", "Чили"),
        ("lo", "Chile"),
        ("lt", "Čilė"),
        ("lv", "Čīle"),
        ("mi", "Hiri"),
        ("mk", "Чиле"),
        ("ml", "ചിലി"),
        ("mn", "Чили"),
        ("mr", "चिली"),
        ("ms", "Chile"),
        ("mt", "Ċile"),
        ("my", "ချ\u{102e}လ\u{102e}ပြည\u{103a}သ\u{1030}\u{1037}သမ\u{1039}မတန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
        ("na", "Tsire"),
        ("nb", "Chile"),
        ("ne", "चिली"),
        ("nl", "Chili"),
        ("nn", "Chile"),
        ("nv", "Chile"),
        ("oc", "Chile"),
        ("or", "ଚ\u{b3f}ଲୀ"),
        ("pa", "ਚਿ\u{a71}ਲੀ"),
        ("pi", "चिल\u{947}"),
        ("pl", "Chile"),
        ("ps", "چېلي"),
        ("pt", "Chile"),
        ("pt_BR", "Chile"),
        ("ro", "Chile"),
        ("ru", "Чили"),
        ("rw", "Shili"),
        ("sc", "Tzile"),
        ("sd", "چلي"),
        ("si", "ච\u{dd2}ල\u{dd3}"),
        ("sk", "Čile"),
        ("sl", "Čile"),
        ("so", "Jili"),
        ("sq", "Kili"),
        ("sr", "Чиле"),
        ("sv", "Chile"),
        ("sw", "Chile"),
        ("ta", "சிலி"),
        ("te", "చ\u{c3f}ల\u{c40}"),
        ("tg", "Чили"),
        ("th", "ช\u{e34}ล\u{e35}"),
        ("ti", "ቺሊ"),
        ("tk", "Çili"),
        ("tl", "Tsile"),
        ("tr", "Şili"),
        ("tt", "Чили"),
        ("ug", "چىلى"),
        ("uk", "Чилі"),
        ("ur", "چلی"),
        ("uz", "Chili"),
        ("ve", "Shile"),
        ("vi", "Chi-lê"),
        ("wa", "Tchili"),
        ("wo", "Ciili"),
        ("xh", "Chile"),
        ("yo", "Tsílè"),
        ("zh_CN", "智利"),
        ("zh_HK", "智利"),
        ("zh_TW", "智利"),
        ("zu", "I-Chile"),
];
    #[cfg(all(feature = "cl", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -35.675147;
        pub const LONGITUDE: f64 = -71.542969;
        pub const MAX_LATITUDE: f64 = -17.4983291;
        pub const MAX_LONGITUDE: f64 = -66.3327;
        pub const MIN_LATITUDE: f64 = -56.1455;
        pub const MIN_LONGITUDE: f64 = -110.0281;
        pub const NORTHEAST_LATITUDE: f64 = -17.4983291;
        pub const NORTHEAST_LONGITUDE: f64 = -66.3327;
        pub const SOUTHWEST_LATITUDE: f64 = -56.1455;
        pub const SOUTHWEST_LONGITUDE: f64 = -110.0281;
    }
}
#[cfg(all(feature = "cl", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -35.675147,
            longitude: -71.542969,
            max_latitude: -17.4983291,
            max_longitude: -66.3327,
            min_latitude: -56.1455,
            min_longitude: -110.0281,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: -17.4983291,
                    longitude: -66.3327,
                },
                southwest: CountryGeoBound {
                    latitude: -56.1455,
                    longitude: -110.0281,
                },
            },
        }
    }
}

#[cfg(all(feature = "cl", feature = "subdivisions"))]
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
                    "AI",
                    Subdivision{
                        name: "AI",
                        country_alpha2: Alpha2::CL,
                        code: "AI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-46.378345), longitude: Some(-72.3007623), max_latitude: Some(-43.6399768), min_latitude: Some(-49.3439696), max_longitude: Some(-71.08750119999999), min_longitude: Some(-75.67981809999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم أيسن"), ("bg", "Айсен"), ("bn", "আয\u{9bc}সেন অঞ\u{9cd}চল"), ("ca", "Regió d’Aysén"), ("ccp", "𑄃𑄃\u{11128}𑄥𑄬𑄚\u{11134}"), ("ceb", "Aysén"), ("cs", "Region Aysén"), ("da", "Aisén-regionen"), ("de", "Región de Aysén"), ("el", "Αϊσέν"), ("en", "Aysén"), ("es", "Región Aysén del General Carlos Ibáñez del Campo"), ("eu", "Aisén eskualdea"), ("fa", "منطقه آیسین"), ("fi", "Aisén del General Carlos Ibáñez del Campo"), ("fr", "Région Aisén del General Carlos Ibáñez del Campo"), ("gl", "Rexión de Aisén"), ("gu", "આશ\u{ac7}ન પ\u{acd}રદ\u{ac7}શ"), ("he", "אייסן"), ("hi", "आइसन क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "XI. regija Aisén del General Carlos Ibáñez del Campo"), ("hu", "Aysén régió"), ("hy", "Այսեն"), ("id", "Region Aisén"), ("it", "regione di Aysén"), ("ja", "アイセン・デル・ヘネラル・カルロス・イバニェス・デル・カンポ州"), ("ka", "აისენი"), ("kn", "ಐಸ\u{cc6}ನ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "아이센델헤네랄카를로스이바녜스델캄포 주"), ("lt", "Aiseno regionas"), ("lv", "Aisenas reģions"), ("mr", "अयान प\u{94d}रद\u{947}श"), ("ms", "Aysen Region"), ("nb", "Aisén"), ("nl", "Aysén del General Carlos Ibáñez del Campo"), ("no", "Aisén"), ("pl", "Aisén"), ("pt", "Aisén (região)"), ("ro", "Regiunea Aysén"), ("ru", "Айсен"), ("si", "අය\u{dd2}සෙන\u{dca} කල\u{dcf}පය"), ("sk", "Aysén"), ("sv", "Región de Aisén"), ("ta", "ஐஸேண\u{bcd} பகுதி"), ("te", "ఆయ\u{c46}స\u{c46}న\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ไอเซน"), ("tr", "Aysen bölgesi"), ("ur", "ایسین ریجن"), ("vi", "Khu vực Aysén"), ("yue", "艾森大區"), ("yue_Hans", "艾森大区"), ("zh", "伊瓦涅斯将军艾森大区")]),
                        unofficial_name_list: ["Aisén del General Carlos Ibáñez del Campo", "Aysén"].to_vec(),
                    }
                ),
                (
                    "AN",
                    Subdivision{
                        name: "AN",
                        country_alpha2: Alpha2::CL,
                        code: "AN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-23.65), longitude: Some(-70.39999999999999), max_latitude: Some(-23.0591235), min_latitude: Some(-25.40112), max_longitude: Some(-68.1182127), min_longitude: Some(-70.62911609999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم أنتوفاغاستا"), ("be", "вобласць Антафагаста"), ("bg", "Антофагаста"), ("bn", "আন\u{9cd}ত\u{9c1}ফ\u{9be}গ\u{9be}স\u{9cd}ত\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió d’Antofagasta"), ("ccp", "𑄃𑄚\u{11134}𑄑\u{1112e}𑄜𑄉𑄌\u{11134}𑄑"), ("ceb", "Antofagasta"), ("cs", "Region Antofagasta"), ("da", "Antofagasta-regionen"), ("de", "Región de Antofagasta"), ("el", "Αντοφαγκάστα"), ("en", "Antofagasta"), ("es", "Región de Antofagasta"), ("et", "Antofagasta piirkond"), ("eu", "Antofagasta eskualdea"), ("fa", "منطقه آنتوفاگاستا"), ("fi", "Antofagastan alue"), ("fr", "Région d’Antofagasta"), ("gl", "Rexión de Antofagasta"), ("gu", "એન\u{acd}ટોફગાસ\u{acd}ટા પ\u{acd}રદ\u{ac7}શ"), ("he", "אנטופגסטה"), ("hi", "ए\u{902}टोफगास\u{94d}ता प\u{94d}रद\u{947}श"), ("hr", "II . regija Antofagasta"), ("hy", "Անտոֆագաստա"), ("id", "Wilayah Antofagasta"), ("it", "regione di Antofagasta"), ("ja", "アントファガスタ州"), ("ka", "ანტოფაგასტა"), ("kn", "ಆಂಟೊಫಾಗಸ\u{ccd}ಟಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "안토파가스타 주"), ("lt", "Antofagastos regionas"), ("lv", "Antofagastas reģions"), ("mr", "अ\u{901}टोफागस\u{94d}ता प\u{94d}रद\u{947}श"), ("ms", "Antofagasta Region"), ("nb", "Antofagasta"), ("nl", "Antofagasta"), ("no", "Antofagasta"), ("pl", "Antofagasta"), ("pt", "Região de Antofagasta"), ("ro", "Regiunea Antofagasta"), ("ru", "Антофагаста"), ("si", "අන\u{dca}ටෝෆගස\u{dca}ට\u{dcf} කල\u{dcf}පය"), ("sk", "Antofagasta"), ("sr", "Регион Антофагаста"), ("sr_Latn", "Region Antofagasta"), ("sv", "Región de Antofagasta"), ("ta", "அன\u{bcd}டோபிஹஸ\u{bcd}த\u{bbe} பகுதி"), ("te", "ఆంట\u{c4b}ఫ\u{c3e}గస\u{c4d}త\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เม\u{e37}องอ\u{e31}นโตฟาก\u{e31}สตา"), ("tr", "Antofagasta bölgesi"), ("ur", "انتوفاجاستا ریجن"), ("vi", "Khu vực Antofagasta"), ("zh", "安托法加斯塔大区")]),
                        unofficial_name_list: ["Antofagasta"].to_vec(),
                    }
                ),
                (
                    "AP",
                    Subdivision{
                        name: "AP",
                        country_alpha2: Alpha2::CL,
                        code: "AP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-18.5940485), longitude: Some(-69.4784541), max_latitude: Some(-17.5008571), min_latitude: Some(-19.2302791), max_longitude: Some(-68.9160989), min_longitude: Some(-70.37963309999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم أريكا وبارينكوتا"), ("bg", "Арика и Паринакота"), ("ca", "Regió d’Arica i Parinacota"), ("ccp", "𑄃𑄢\u{11128}𑄇 𑄤𑄠\u{1112d} 𑄛𑄢\u{11128}𑄚𑄇\u{1112e}𑄑"), ("ceb", "Región de Arica y Parinacota"), ("cs", "Region Arica y Parinacota"), ("da", "Arica og Parinacota-regionen"), ("de", "Región de Arica y Parinacota"), ("en", "Arica y Parinacota"), ("es", "Región de Arica y Parinacota"), ("et", "Arica y Parinacota"), ("eu", "Arica eta Parinacota eskualdea"), ("fa", "منطقه آریکا و پاریناکوتا"), ("fi", "Arica-Parinacota"), ("fr", "Région d’Arica et Parinacota"), ("gl", "Rexión de Arica e Parinacota"), ("hr", "XV. regija Arica i Parinacota"), ("hy", "Արիկա և Պարինակոտա"), ("it", "regione di Arica e Parinacota"), ("ja", "アリカ・イ・パリナコータ州"), ("ka", "არიკა და პარინაკოტა"), ("ko", "아리카 이 파리나코타 주"), ("lt", "Arikos ir Parinakotos regionas"), ("nb", "Arica y Parinacota"), ("nl", "Arica y Parinacota"), ("no", "Arica y Parinacota"), ("pl", "Arica y Parinacota"), ("pt", "Região de Arica e Parinacota"), ("ro", "Regiunea Arica și Parinacota"), ("ru", "Арика-и-Паринакота"), ("sk", "Arica y Parinacota"), ("sv", "Región de Arica y Parinacota"), ("sw", "Mkoa wa Arica na Parinacota"), ("tr", "Arica ve Parinacota bölgesi"), ("vi", "Arica và Parinacota"), ("zh", "阿里卡和帕里纳科塔大区")]),
                        unofficial_name_list: ["Arica y Parinacota"].to_vec(),
                    }
                ),
                (
                    "AR",
                    Subdivision{
                        name: "AR",
                        country_alpha2: Alpha2::CL,
                        code: "AR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-38.948921), longitude: Some(-72.331113), max_latitude: Some(-37.5880498), min_latitude: Some(-39.6397245), max_longitude: Some(-70.8301122), min_longitude: Some(-73.5228712)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم أروكانيا"), ("bg", "Араукания"), ("bn", "আর\u{9be}ক\u{9be}নিয\u{9bc}\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de l’Araucània"), ("ccp", "𑄃𑄢𑄅\u{1112a}𑄇𑄚\u{11128}𑄠"), ("ceb", "Región de la Araucanía"), ("cs", "Region Araucanía"), ("da", "Araucanía-regionen"), ("de", "Región de la Araucanía"), ("el", "Αροκανία"), ("en", "Araucanía"), ("es", "Región de la Araucanía"), ("eu", "Araucanía eskualdea"), ("fa", "منطقه آرائوکانیا"), ("fi", "Araucanía"), ("fr", "Région d’Araucanie"), ("gl", "Rexión de Araucanía"), ("gu", "અરાઉક\u{ac7}નિયા પ\u{acd}રદ\u{ac7}શ"), ("he", "אראוקניה"), ("hi", "अराऊक\u{947}निया प\u{94d}रद\u{947}श"), ("hr", "IX. regija Araucanía"), ("hu", "Araucanía régió"), ("hy", "Արուկանիա"), ("id", "Wilayah Araucanía"), ("is", "Araucanía-fylki"), ("it", "regione dell’Araucanía"), ("ja", "ラ・アラウカニア州"), ("ka", "არაუკანია"), ("kn", "ಅರಕುನ\u{cbf}ಯಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "아라우카니아 주"), ("lt", "Araukanijos regionas"), ("lv", "Araukānijas reģions"), ("mr", "अराक\u{94d}रानिया प\u{94d}रद\u{947}श"), ("ms", "Araucania Region"), ("nb", "Araucanía"), ("nl", "Araucanía"), ("no", "Araucanía"), ("pl", "Araukania"), ("pt", "Região da Araucanía"), ("ro", "Regiunea La Araucanía"), ("ru", "Араукания"), ("si", "අරෞ ක\u{dcf}න\u{dd2}ය\u{dcf} කල\u{dcf}පය"), ("sk", "Araucanía"), ("sv", "Región de la Araucanía"), ("sw", "Mkoa wa Araucanía"), ("ta", "அர\u{bcd}துக\u{bbe}னிய\u{bbe} பகுதி"), ("te", "ఆర\u{c3e}క\u{c47}న\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตอเราคาเน\u{e35}ย"), ("tr", "Araucanía bölgesi"), ("ur", "اراوکانیا ریجن"), ("vi", "Khu vực Araucanía"), ("zh", "阿劳卡尼亚大区")]),
                        unofficial_name_list: ["La Araucanía"].to_vec(),
                    }
                ),
                (
                    "AT",
                    Subdivision{
                        name: "AT",
                        country_alpha2: Alpha2::CL,
                        code: "AT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-27.5660558), longitude: Some(-70.050314), max_latitude: Some(-25.2897006), min_latitude: Some(-29.5373752), max_longitude: Some(-68.26686219999999), min_longitude: Some(-71.5936886)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم أتاكاما"), ("bg", "Атакама"), ("bn", "আটক\u{9be}ম\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió d’Atacama"), ("ccp", "𑄃𑄑𑄇𑄟"), ("ceb", "Atacama"), ("cs", "Region Atacama"), ("da", "Atacama-regionen"), ("de", "Región de Atacama"), ("el", "Ατακάμα"), ("en", "Atacama"), ("es", "Región de Atacama"), ("eu", "Atacama eskualdea"), ("fa", "منطقه آتاکاما"), ("fi", "Atacaman alue"), ("fr", "Région d’Atacama"), ("gl", "Rexión de Atacama"), ("gu", "અટાકામા પ\u{acd}રદ\u{ac7}શ"), ("hi", "एटाकामा क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "III. regija Atacama"), ("hy", "Ատակամա"), ("id", "Wilayah Atacama"), ("it", "regione di Atacama"), ("ja", "アタカマ州"), ("ka", "ატაკამა"), ("kn", "ಅಟ\u{ccd}ಕಾಮಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "아타카마 주"), ("lt", "Atakamos regionas"), ("lv", "Atakamas reģions"), ("mr", "अटाकामा प\u{94d}रद\u{947}श"), ("ms", "Atacama Region"), ("nb", "Atacama"), ("nl", "Atacama"), ("no", "Atacama"), ("pl", "Atakama"), ("pt", "Região de Atacama"), ("ro", "Regiunea Atacama"), ("ru", "Атакама"), ("si", "ඇටක\u{dcf}ම\u{dcf} කල\u{dcf}පය"), ("sk", "Atacama"), ("sv", "Región de Atacama"), ("sw", "Mkoa wa Atacama"), ("ta", "அடக\u{bcd}கம\u{bbe} பகுதி"), ("te", "అట\u{c3e}క\u{c3e}మ\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "อาตากามา"), ("tr", "Atacama bölgesi"), ("ur", "اتاکاما ریجن"), ("vi", "Khu vực Atacama"), ("zh", "阿塔卡马大区")]),
                        unofficial_name_list: ["Atacama"].to_vec(),
                    }
                ),
                (
                    "BI",
                    Subdivision{
                        name: "BI",
                        country_alpha2: Alpha2::CL,
                        code: "BI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-36.9777206), longitude: Some(-72.331113), max_latitude: Some(-36.0083148), min_latitude: Some(-38.4918218), max_longitude: Some(-70.98831919999999), min_longitude: Some(-73.9699354)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم بيو بيو"), ("az", "Bio Bio bölgəsi"), ("be", "вобласць Біа-Біа"), ("bg", "Биобио"), ("bn", "অঞ\u{9cd}চল মেট\u{9cd}রোপলিটন"), ("bs", "Regija Bío-Bío"), ("ca", "Regió del Bío-Bío"), ("ccp", "𑄝\u{1112d}𑄃\u{1112e} 𑄝\u{1112d}𑄃\u{1112e}"), ("ceb", "Región del Biobío"), ("cs", "Region Bío-Bío"), ("cy", "Bío Bío Region"), ("da", "Biobío-regionen"), ("de", "Región del Bío-Bío"), ("el", "Περιφέρεια Μπίο Μπίο"), ("en", "Bío Bío"), ("es", "Región del Bío Bío"), ("et", "Bío-Bío piirkond"), ("eu", "Biobío eskualdea"), ("fa", "بیوبیو"), ("fi", "Biobíon alue"), ("fr", "Région du Biobío"), ("gl", "Rexión de Biobío"), ("gu", "રીજન મ\u{ac7}ટ\u{acd}રોપોલિટન"), ("he", "ביוביו"), ("hr", "VIII. regija Biobío"), ("hu", "Biobío régió"), ("hy", "Բիոբիո"), ("id", "Wilayah Bío Bío"), ("is", "Biobío-fylki"), ("it", "regione del Bío Bío"), ("ja", "ビオビオ州"), ("ka", "ბიო-ბიო"), ("kn", "ರ\u{cc6}ಜ\u{cbf}ಯಾನ\u{ccd} ಮಹಾನಗರ"), ("ko", "비오비오 주"), ("lt", "Biobio regionas"), ("lv", "Biobio reģions"), ("mk", "Биобио"), ("mr", "रीजन म\u{947}ट\u{94d}रोपॉलिटन"), ("ms", "Region metropolitana"), ("nb", "Biobío"), ("nl", "Bío-Bío"), ("no", "Biobío"), ("pl", "Biobío"), ("pt", "Região de Bío-Bío"), ("ro", "Regiunea Biobío"), ("ru", "Био-Био"), ("si", "මෙට\u{dca}\u{200d}රොපොල\u{dd2}ටන\u{dcf} කල\u{dcf}පය"), ("sk", "Biobío"), ("sr", "Регион Биобио"), ("sr_Latn", "Region Biobio"), ("sv", "Región del BioBío"), ("ta", "பகுதி மெட\u{bcd}ரோபோலித\u{bbe}ன\u{bbe}"), ("te", "ర\u{c40}జ\u{c3f}యన\u{c4d} మ\u{c46}ట\u{c4d}ర\u{c4b}ప\u{c3e}ల\u{c3f}ట\u{c3e}న\u{c3e}"), ("th", "เรจ\u{e34}โอน\u{e31}ลเมโทรโปล\u{e34}ตาโน"), ("tr", "Bío-Bío bölgesi"), ("uk", "Біобіо"), ("ur", "ریجن میتروپولیتانا"), ("vi", "Vùng Bío Bío"), ("zh", "比奥比奥大区")]),
                        unofficial_name_list: ["Bíobío"].to_vec(),
                    }
                ),
                (
                    "CO",
                    Subdivision{
                        name: "CO",
                        country_alpha2: Alpha2::CL,
                        code: "CO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-29.9533), longitude: Some(-71.3436), max_latitude: Some(-29.9331593), min_latitude: Some(-30.515066), max_longitude: Some(-71.117463), min_longitude: Some(-71.67686309999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم كوكيمبو"), ("be", "Какімба"), ("bg", "Кокимбо"), ("bn", "ক\u{9c1}কিম\u{9cd}ব\u{9c1} অঞ\u{9cd}চল"), ("ca", "Regió de Coquimbo"), ("ccp", "𑄇\u{1112e}𑄇\u{1112a}𑄃\u{11128}𑄟\u{11134}𑄝\u{1112e}"), ("ceb", "Coquimbo"), ("cs", "Region Coquimbo"), ("da", "Coquimbo-regionen"), ("de", "Región de Coquimbo"), ("el", "Κοκίμπο"), ("en", "Coquimbo"), ("es", "Región de Coquimbo"), ("eu", "Coquimbo eskualdea"), ("fa", "منطقه کوکیمبو"), ("fi", "Coquimbo"), ("fr", "Région de Coquimbo"), ("gl", "Rexión de Coquimbo"), ("gu", "કોક\u{ac1}મ\u{acd}બો પ\u{acd}રદ\u{ac7}શ"), ("hi", "कोकिम\u{94d}बो प\u{94d}रद\u{947}श"), ("hr", "IV. regija Coquimbo"), ("hy", "Կոկիմբո"), ("id", "Wilayah Coquimbo"), ("it", "regione di Coquimbo"), ("ja", "コキンボ州"), ("ka", "კოკიმბო"), ("kn", "ಕೊಕ\u{ccd}ವ\u{cbf}ಬೋ ಪ\u{ccd}ರದೇಶ"), ("ko", "코킴보 주"), ("lt", "Kokimbo regionas"), ("lv", "Kokimbo reģions"), ("mk", "Кокимбо"), ("mr", "कोक\u{94d}मिलो प\u{94d}रद\u{947}श"), ("ms", "Coquimbo Region"), ("nb", "Coquimbo"), ("nl", "Coquimbo"), ("no", "Coquimbo"), ("pl", "Coquimbo"), ("pt", "Região de Coquimbo"), ("ro", "Regiunea Coquimbo"), ("ru", "Кокимбо"), ("si", "කොක\u{dd2}ම\u{dca}බෝ කල\u{dcf}පය"), ("sk", "Coquimbo"), ("sv", "Región de Coquimbo"), ("ta", "கோகுய\u{bcd}ம\u{bcd}பொ பகுதி"), ("te", "క\u{c3e}క\u{c4d}వ\u{c3f}ంబ\u{c4b} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แคว\u{e49}นโกก\u{e34}มโบ"), ("tr", "Coquimbo bölgesi"), ("ur", "کوکیمبو علاقہ"), ("vi", "Khu vực Coquimbo"), ("zh", "科金博大区")]),
                        unofficial_name_list: ["Coquimbo"].to_vec(),
                    }
                ),
                (
                    "LI",
                    Subdivision{
                        name: "LI",
                        country_alpha2: Alpha2::CL,
                        code: "LI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-34.5755374), longitude: Some(-71.0022311), max_latitude: Some(-33.8537682), min_latitude: Some(-35.0066722), max_longitude: Some(-70.0121472), min_longitude: Some(-72.0717305)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم ليبيرتادور جينيرال برناردو أوهيجينز"), ("bn", "লিব\u{9be}র\u{9cd}ত\u{9be}দো জেন\u{9be}রেল ব\u{9be}র\u{9cd}ন\u{9be}ডো ও’হিগিন\u{9cd}স অঞ\u{9cd}চল"), ("ca", "Regió d’O’Higgins"), ("ccp", "𑄣\u{1112d}𑄝𑄢\u{11134}𑄑𑄓\u{1112e} 𑄎𑄬𑄚𑄢𑄬𑄣\u{11134} 𑄝𑄢\u{11134}𑄚𑄢\u{11134}𑄓\u{1112e} 𑄃\u{1112e}‘𑄦\u{1112d}𑄎\u{11128}𑄚\u{11134}𑄥\u{11134}"), ("ceb", "Región del Libertador General Bernardo O’Higgins"), ("cs", "Region O’Higgins"), ("da", "O’Higgins-regionen"), ("de", "Región del Libertador General Bernardo O’Higgins"), ("el", "Ο’Χίγκινς"), ("en", "Libertador General Bernardo O’Higgins"), ("es", "Región de O’Higgins"), ("eu", "O’Higgins eskualdea"), ("fa", "منطقه ا هیگینز"), ("fi", "Libertador General Bernardo O’Higginsin maakunta"), ("fr", "Région du Libertador General Bernardo O’Higgins"), ("gl", "Rexión de O’Higgins"), ("gu", "લિબર\u{acd}ટાડોર જનરલ , બર\u{acd}નાર\u{acd}ડો ઓ’હિગિન\u{acd}સ પ\u{acd}રદ\u{ac7}શ"), ("hi", "लिबर\u{94d}टडोर जनरल बर\u{94d}नार\u{94d}डो ओ’हिगिन\u{94d}स क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "VI. regija Libertador General Bernardo O’Higgins"), ("id", "Wilayah Libertador General Bernardo O’Higgins"), ("it", "regione del Libertador General Bernardo O’Higgins"), ("ja", "リベルタドール・ベルナルド・オイギンス州"), ("kn", "ಲ\u{cbf}ಬರ\u{ccd}ಟಡರ\u{ccd} ಜನರಲ\u{ccd} ಬ\u{cc6}ರ\u{ccd}ನಾರ\u{ccd}ಡೊ ಓ ಹ\u{cbf}ಗ\u{ccd}ಗ\u{cbf}ನ\u{ccd}ಸ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "리베르타도르헤네랄베르나르도오이긴스 주"), ("lt", "Išvaduotojo Generolo Bernardo O’Higinso regionas"), ("lv", "Ohiginsa reģions"), ("mr", "लिबर\u{94d}टाडोडर जनरल बर\u{94d}नार\u{94d}डो ओ’हिग\u{94d}गीन\u{94d}स प\u{94d}रद\u{947}श"), ("ms", "Libertador General Bernardo O’Higgins Region"), ("nb", "O’Higgins"), ("nl", "Libertador General Bernardo O’Higgins"), ("no", "O’Higgins"), ("pl", "Libertador"), ("pt", "Região de O’Higgins"), ("ro", "Regiunea Libertador General Bernardo O’Higgins"), ("si", "ල\u{dd2}බර\u{dca}ටෙදෝර\u{dca} ජෙනරල\u{dca} බර\u{dca}න\u{dcf}ඩෝ ඔ හ\u{dd2}ග\u{dd2}න\u{dca}ස\u{dca} කල\u{dcf}පය"), ("sk", "Libertador General Bernardo O’Higgins"), ("sv", "Región de O’Higgins"), ("sw", "Mkoa wa O’Higgins"), ("ta", "லிபெர\u{bcd}ட\u{bcd}டொர\u{bcd} ஜெனரல\u{bcd} பெர\u{bcd}ன\u{bbe}ர\u{bcd}டோ ஓ ‘ஹிக\u{bcd}கின\u{bcd}ஸ\u{bcd} பகுதி"), ("te", "ల\u{c3f}బర\u{c4d}ట\u{c47}డర\u{c4d} జనరల\u{c4d} బ\u{c46}ర\u{c4d}న\u{c3e}ర\u{c4d}డ\u{c4b} ఓ’హ\u{c3f}గ\u{c3f}న\u{c4d}స\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "โอ ฮ\u{e34}กก\u{e34}นส\u{e4c}"), ("tr", "O’Higgins bölgesi"), ("vi", "Khu vực Libertador General Bernardo O’Higgins"), ("zh", "奥伊金斯将军解放者大区")]),
                        unofficial_name_list: ["General Bernardo O'Higgins", "Libertador", "Libertador OʿHiggins"].to_vec(),
                    }
                ),
                (
                    "LL",
                    Subdivision{
                        name: "LL",
                        country_alpha2: Alpha2::CL,
                        code: "LL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-41.9197779), longitude: Some(-72.1416132), max_latitude: Some(-40.2364882), min_latitude: Some(-44.0668098), max_longitude: Some(-71.5841725), min_longitude: Some(-74.8511989)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم لوس لاغوس"), ("bg", "Лос Лагос"), ("bn", "লস ল\u{9be}গোস অঞ\u{9cd}চল"), ("ca", "Regió de Los Lagos"), ("ccp", "𑄣\u{11127}𑄌\u{11134} 𑄣𑄉\u{1112e}𑄌\u{11134}"), ("ceb", "Los Lagos"), ("cs", "Region Los Lagos"), ("da", "Los Lagos-regionen"), ("de", "Región de los Lagos"), ("el", "Λος Λάγκος"), ("en", "Los Lagos"), ("es", "Región de Los Lagos"), ("eu", "Los Lagos eskualdea"), ("fa", "منطقه لوس لاگوس"), ("fi", "Los Lagosin maakunta"), ("fr", "Région des Lacs"), ("gl", "Rexión de Los Lagos"), ("gu", "લોસ લાગોસ પ\u{acd}રા\u{a82}ત"), ("he", "לוס לאגוס"), ("hi", "लॉस लागोस क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "X. regija Los Lagos"), ("hu", "Los Lagos régió"), ("hy", "Լոս Լագոս"), ("id", "Wilayah Los Lagos"), ("is", "Los Lagos-fylki"), ("it", "regione di Los Lagos"), ("ja", "ロス・ラゴス州"), ("ka", "ლოს-ლაგოსი"), ("kn", "ಲಾಸ\u{ccd} ಲಾಗೋಸ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "로스라고스 주"), ("lt", "Los Lagoso regionas"), ("lv", "Loslagosas reģions"), ("mr", "लॉस लागोस प\u{94d}रद\u{947}श"), ("ms", "Los Lagos Region"), ("nb", "Los Lagos"), ("nl", "Los Lagos"), ("no", "Los Lagos"), ("pl", "Los Lagos"), ("pt", "Região de Los Lagos"), ("ro", "Regiunea Los Lagos"), ("ru", "Лос-Лагос"), ("si", "ලොස\u{dca} ල\u{dcf}ගොස\u{dca} කල\u{dcf}පය"), ("sk", "Los Lagos"), ("sv", "Región de Los Lagos"), ("sw", "Mkoa wa Los Lagos"), ("ta", "ல\u{bbe}ஸ\u{bcd} ல\u{bbe}கோஸ\u{bcd} பகுதி"), ("te", "ల\u{c3e}స\u{c4d} ల\u{c3e}గ\u{c4b}స\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ลอสลากอส"), ("tr", "Los Lagos bölgesi"), ("ur", "لوس لاگوس علاقہ"), ("vi", "Khu vực Los Lagos"), ("zh", "湖大区")]),
                        unofficial_name_list: ["Los Lagos"].to_vec(),
                    }
                ),
                (
                    "LR",
                    Subdivision{
                        name: "LR",
                        country_alpha2: Alpha2::CL,
                        code: "LR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-40.2310217), longitude: Some(-72.331113), max_latitude: Some(-39.2871406), min_latitude: Some(-40.6819045), max_longitude: Some(-71.5829256), min_longitude: Some(-73.7275484)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة لوس ريوس"), ("be", "Лос-Рыяс"), ("bg", "Лос Риос"), ("bn", "লস রিওস অঞ\u{9cd}চল"), ("ca", "Regió de Los Ríos"), ("ccp", "𑄣\u{11127}𑄌\u{11134} 𑄢\u{11128}𑄠\u{1112e}𑄌\u{11134}"), ("ceb", "Región de Los Ríos"), ("cs", "Region Los Ríos"), ("da", "Los Ríos-regionen"), ("de", "Región de Los Ríos"), ("el", "Λος Ρίος"), ("en", "Los Ríos"), ("es", "Región de Los Ríos"), ("eu", "Los Ríos eskualdea"), ("fa", "منطقه لوس ریوس"), ("fi", "Los Ríos"), ("fr", "Région des Fleuves"), ("gl", "Rexión de Los Ríos"), ("gu", "લોસ રિયોસ પ\u{acd}રદ\u{ac7}શ"), ("he", "לוס ריוס"), ("hi", "लॉस रिओस क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "XIV. regija Los Ríos"), ("hu", "Los Ríos régió"), ("hy", "Լոս Ռիոս"), ("id", "Wilayah Los Ríos"), ("is", "Los Ríos-fylki"), ("it", "regione di Los Ríos"), ("ja", "ロス・リオス州"), ("ka", "ლოს-რიოსი"), ("kn", "ಲಾಸ\u{ccd} ರ\u{cbf}ಯೋಸ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "로스리오스 주"), ("lt", "Los Rioso regionas"), ("lv", "Losroisas reģions"), ("mr", "लॉस रियोस प\u{94d}रद\u{947}श"), ("ms", "Los Rios Region"), ("nb", "Los Ríos"), ("nl", "Los Ríos"), ("no", "Los Ríos"), ("pl", "Los Ríos"), ("pt", "Região de Los Rios"), ("ro", "Regiunea Los Ríos"), ("ru", "Лос-Риос"), ("si", "ලොස\u{dca} ර\u{dd2}යෝස\u{dca} පළ\u{dcf}ත"), ("sk", "Los Ríos"), ("sv", "Región de Los Ríos"), ("sw", "Mkoa wa Los Ríos"), ("ta", "ல\u{bbe}ஸ\u{bcd} ர\u{bc0}வ\u{bcd}ஸ\u{bcd} பகுதி"), ("te", "ల\u{c3e}స\u{c4d} ర\u{c3f}య\u{c4b}స\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แคว\u{e49}นลอส ร\u{e34}ออส"), ("tr", "Los Ríos bölgesi"), ("ur", "لوس ریوس علاقہ"), ("vi", "Khu vực Los Ríos"), ("zh", "河大区")]),
                        unofficial_name_list: ["Los Ríos"].to_vec(),
                    }
                ),
                (
                    "MA",
                    Subdivision{
                        name: "MA",
                        country_alpha2: Alpha2::CL,
                        code: "MA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-52.20643159999999), longitude: Some(-72.16850010000002), max_latitude: Some(-48.5966006), min_latitude: Some(-55.9799665), max_longitude: Some(-66.4181435), min_longitude: Some(-75.7296587)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ماغالانس"), ("be", "Магальянес і Чылійская Антарктыка"), ("bg", "Магалянес и Чилийска Антарктика"), ("bn", "ম\u{9be}গ\u{9be}ল\u{9be}নেস ল\u{9be} এন\u{9cd}ট\u{9be}র\u{9cd}কটিক\u{9be} চিলিয\u{9bc}েন\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Magallanes i de l’Antàrtica Xilena"), ("ccp", "𑄟\u{11133}𑄠𑄉𑄣\u{11128}𑄚\u{11134} 𑄢𑄬𑄎\u{11128}𑄠\u{11127}𑄚\u{11134}"), ("ceb", "Región de Magallanes y de la Antártica Chilena"), ("cs", "Region Magallanes y la Antártica Chilena"), ("da", "Magallanes og Antártica Chilena-regionen"), ("de", "Región de Magallanes y de la Antártica Chilena"), ("el", "Περιοχή Μαγαγιάνες και η Χιλιανή Ανταρκτική"), ("en", "Magallanes Region"), ("es", "Región de Magallanes y de la Antártica Chilena"), ("eu", "Magallanes y la Antártica Chilena eskualdea"), ("fa", "منطقه ماژلان و قطب جنوب شیلی"), ("fi", "Magallanes y la Antártica Chilenan alue"), ("fr", "Région de Magallanes et de l’Antarctique chilien"), ("gl", "Rexión de Magallanes e da Antártica Chilena"), ("gu", "મ\u{ac7}ગાલ\u{ac7}ન\u{acd}સ ય લા એન\u{acd}ટાર\u{acd}ટિકા ચિલ\u{ac7}ના પ\u{acd}રદ\u{ac7}શ"), ("he", "מגאיאנס ואנטארקטיקה צ׳ילנה"), ("hi", "म\u{948}गल\u{947}न\u{94d}स वाई ला अ\u{902}टार\u{94d}कटिका चिल\u{947}ना क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "XII. regija Magallanes y de la Antártica Chilena"), ("hy", "Մագալանես և Չիլիական Անտարկտիկա"), ("id", "Region Magallanes y la Antártica"), ("it", "regione di Magellano e dell’Antartide Cilena"), ("ja", "マガジャネス・イ・デ・ラ・アンタルティカ・チレーナ州"), ("ka", "მაგალიანესი და ჩილეს ანტარქტიკა"), ("kn", "ಮಗಾಲ\u{cc6}ನ\u{ccd}ಸ\u{ccd} ವೈ ಲಾ ಅಂಟಾರ\u{ccd}ಟ\u{cbf}ಕಾ ಚ\u{cbf}ಲ\u{cc6}ನಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "마가야네스 이 안타르티카칠레나 주"), ("lt", "Magelano ir Čilės Antarktidos regionas"), ("lv", "Magaljanesas un Čīles Antarktikas reģions"), ("ml", "റെജിയോൺ ഡെ മഗല\u{d4d}യ\u{d3e}ൻസ\u{d4d} ഡെല യെ അന\u{d4d}റ\u{d3e}ർട\u{d4d}ടിക\u{d4d}ക ചിലെന"), ("mr", "मागालान\u{94d}स या ला अ\u{902}टार\u{94d}क\u{94d}टिका चिल\u{947}ना प\u{94d}रद\u{947}श"), ("ms", "Daerah Magallanes y la Antartica Chilena"), ("nb", "Magallanes y de la Antártica Chilena"), ("nl", "Magallanes y la Antártica Chilena"), ("no", "Magallanes y de la Antártica Chilena"), ("pl", "Magallanes"), ("pt", "Magalhães e Antártica Chilena"), ("ro", "Regiunea Magallanes și Antartica Chileană"), ("ru", "Магальянес и Чилийская Антарктика"), ("si", "මගලනෙස\u{dca} වය\u{dd2} ල\u{dcf} ඇන\u{dca}ට\u{dcf}ක\u{dca}ට\u{dd2}ක\u{dcf} ච\u{dd2}ලේන\u{dcf} කල\u{dcf}පය"), ("sk", "Magallanes y de la Antártica Chilena"), ("sv", "Región de Magallanes y de la Antártica Chilena"), ("sw", "Mkoa wa Magallanes na Antaktiki ya Chile"), ("ta", "மக\u{bbe}லனேஸி ல\u{bbe} அண\u{bcd}ட\u{bbe}ர\u{bcd}டிக\u{bbe} ச\u{bc0}லேன\u{bbe} பகுதி"), ("te", "మగ\u{c3e}ల\u{c47}నస\u{c4d} య\u{c3e} ల\u{c3e} అంట\u{c3e}ర\u{c4d}క\u{c3f}ట\u{c3f}క\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แคว\u{e49}นมากายาเนสและลาอ\u{e31}นตาร\u{e4c}ต\u{e35}กาช\u{e35}เลนา"), ("tr", "Magallanes y la Antártica Chilena bölgesi"), ("ur", "ماگایانس و لا انتارتیکا چلی علاقہ"), ("vi", "Magellan và Địa Cực Chile"), ("zh", "麦哲伦-智利南极大区")]),
                        unofficial_name_list: ["Magellantes y la Antártica Chilena"].to_vec(),
                    }
                ),
                (
                    "ML",
                    Subdivision{
                        name: "ML",
                        country_alpha2: Alpha2::CL,
                        code: "ML",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-35.5163603), longitude: Some(-71.5723953), max_latitude: Some(-34.7119045), min_latitude: Some(-36.5456538), max_longitude: Some(-70.3260118), min_longitude: Some(-72.78989299999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم مولي"), ("be", "Мауле"), ("bg", "Мауле"), ("bn", "মউলে অঞ\u{9cd}চল"), ("ca", "Regió del Maule"), ("ccp", "𑄟\u{1112f}𑄣𑄬"), ("ceb", "Maule"), ("cs", "Region Maule"), ("da", "Maule-regionen"), ("de", "Región del Maule"), ("el", "Μόλε"), ("en", "Maule"), ("es", "Región del Maule"), ("eu", "Maule eskualdea"), ("fa", "منطقه مائوله"), ("fi", "Maule"), ("fr", "Région du Maule"), ("gl", "Rexión de Maule"), ("gu", "મૌલ પ\u{acd}રદ\u{ac7}શ"), ("he", "מאולה"), ("hi", "मौल क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "VII. regija Maule"), ("hu", "Maule régió"), ("hy", "Մաուլե"), ("id", "Wilayah Maule"), ("it", "regione del Maule"), ("ja", "マウレ州"), ("ka", "მაულე"), ("kn", "ಮಾಲ\u{cc6} ಪ\u{ccd}ರದೇಶ"), ("ko", "마울레 주"), ("lt", "Maulės regionas"), ("lv", "Maules reģions"), ("mr", "मौल प\u{94d}रद\u{947}श"), ("ms", "Maule Region"), ("nb", "Maule"), ("nl", "Maule"), ("no", "Maule"), ("pl", "Maule"), ("pt", "Região de Maule"), ("ro", "Regiunea Maule"), ("ru", "Мауле"), ("si", "මෞලේ කල\u{dcf}පය"), ("sk", "Maule"), ("sv", "Región del Maule"), ("sw", "Mkoa wa Maule"), ("ta", "மௌலி பகுதி"), ("te", "మ\u{c3e}ల\u{c46} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เมาเล"), ("tr", "Maule bölgesi"), ("ur", "ماولے علاقہ"), ("vi", "Khu vực Maule"), ("zh", "马乌莱大区")]),
                        unofficial_name_list: ["Maule"].to_vec(),
                    }
                ),
                (
                    "NB",
                    Subdivision{
                        name: "NB",
                        country_alpha2: Alpha2::CL,
                        code: "NB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-36.599985), longitude: Some(-71.905225), max_latitude: Some(-35.979985), min_latitude: Some(-37.19417), max_longitude: Some(-71.007955), min_longitude: Some(-73.064033)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Ньюбле"), ("ca", "Regió de Ñube"), ("cs", "Ñuble"), ("da", "Ñuble-regionen"), ("de", "Región de Ñuble"), ("en", "Ñuble"), ("es", "Región de Ñuble"), ("eu", "Ñuble eskualdea"), ("fr", "Région de Ñuble"), ("gl", "Rexión de Ñuble"), ("hu", "Ñuble régió"), ("id", "Region Ñuble"), ("it", "Regione di Ñuble"), ("ja", "ニュブレ州"), ("lt", "Ñuble regionas"), ("ms", "Ñuble Region"), ("nb", "Ñuble"), ("nl", "Ñuble"), ("no", "Ñuble"), ("pl", "Ñuble (region)"), ("pt", "Ñuble"), ("ro", "Regiunea Ñuble"), ("ru", "Ньюбле"), ("sk", "Ñuble"), ("sv", "Región de Ñuble"), ("tr", "Ñuble bölgesi")]),
                        unofficial_name_list: ["Ñuble"].to_vec(),
                    }
                ),
                (
                    "RM",
                    Subdivision{
                        name: "RM",
                        country_alpha2: Alpha2::CL,
                        code: "RM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-33.4843354), longitude: Some(-70.6216794), max_latitude: Some(-32.919451), min_latitude: Some(-34.2878805), max_longitude: Some(-69.7689944), min_longitude: Some(-71.7186941)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة سانتياغو متروبوليتان"), ("az", "Santyaqo Metropolitan bölgəsi"), ("bg", "Сантяго"), ("bn", "স\u{9be}ন\u{9cd}তিয\u{9bc}\u{9be}গো মেট\u{9cd}রোপলিটন অঞ\u{9cd}চল"), ("ca", "Regió Metropolitana de Santiago"), ("ccp", "𑄥𑄚\u{11134}𑄑\u{11128}𑄠𑄉\u{1112e} 𑄟𑄬𑄑\u{11133}𑄢\u{1112e}𑄛\u{11127}𑄣\u{11128}𑄑𑄚\u{11134}"), ("ceb", "Región Metropolitana de Santiago"), ("cs", "Metropolitní region Santiago"), ("da", "Santiago Hovedstadsregionen"), ("de", "Región Metropolitana de Santiago"), ("el", "Σαντιάγκο Μετροπόλιταν"), ("en", "Santiago Metropolitan"), ("es", "Región Metropolitana de Santiago"), ("eu", "Santiagoko metropolitar eskualdea"), ("fa", "منطقه شهری سانتیاگو"), ("fi", "Santiago Metropolitan maakunta"), ("fr", "Région métropolitaine de Santiago"), ("gl", "Rexión Metropolitana de Santiago"), ("gu", "મિન\u{acd}સ\u{acd}ક"), ("he", "מטרופולין סנטיאגו"), ("hi", "स\u{947}\u{902}टिआगो म\u{947}ट\u{94d}रोपोलिटन क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Metropolitanska regija Santiaga"), ("hy", "Սանտյագոյի մայրաքաղաքային տարածաշրջան"), ("id", "Wilayah Metropolitan Santiago"), ("it", "regione Metropolitana di Santiago"), ("ja", "首都州"), ("ka", "სანტიაგო"), ("kn", "ಸ\u{ccd}ಯಾಂಟ\u{cbf}ಯಾಗೊ ಮಹಾನಗರ ಪ\u{ccd}ರದೇಶ"), ("ko", "산티아고 수도주"), ("lt", "Santjago metropolinis regionas"), ("lv", "Santjago Metropoles reģions"), ("mr", "सा\u{902}तियागो महानगरीय प\u{94d}रद\u{947}श"), ("ms", "Santiago Metropolitan Region"), ("nb", "Región Metropolitana de Santiago"), ("nl", "Región Metropolitana de Santiago"), ("no", "Región Metropolitana de Santiago"), ("pl", "Region Metropolitalny"), ("pt", "Região Metropolitana de Santiago"), ("ro", "Regiunea Santiago Metropolitan"), ("ru", "Сантьяго"), ("si", "සන\u{dca}ත\u{dd2}ය\u{dcf}ගෝ මෙට\u{dca}\u{200d}රොපොල\u{dd2}ටන\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Región hlavného mesta Santiago"), ("sv", "Región Metropolitana de Santiago"), ("ta", "ச\u{bbe}ண\u{bcd}டிய\u{bbe}கோ மெட\u{bcd}ரோபொலிட\u{bcd}டன\u{bcd} ர\u{bc0}ஜியன\u{bcd}"), ("te", "స\u{c3e}ంట\u{c3f}య\u{c3e}గ\u{c4b} మ\u{c46}ట\u{c4d}ర\u{c4b}ప\u{c3e}ల\u{c3f}టన\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ซ\u{e31}นต\u{e34}อาโก"), ("tr", "Santiago Metropolitan bölgesi"), ("uk", "Столичний Регіон Сантьяго"), ("ur", "سینٹیاگو میٹروپولیٹن علاقہ"), ("vi", "Khu Đô thị Santiago"), ("zh", "圣地亚哥首都大区")]),
                        unofficial_name_list: ["Metropolitana de Santiago"].to_vec(),
                    }
                ),
                (
                    "TA",
                    Subdivision{
                        name: "TA",
                        country_alpha2: Alpha2::CL,
                        code: "TA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-20.2028799), longitude: Some(-69.2877535), max_latitude: Some(-18.9387142), min_latitude: Some(-21.6334004), max_longitude: Some(-68.40684449999999), min_longitude: Some(-70.2885344)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم تاراباكا"), ("bg", "Тарапака"), ("bn", "ত\u{9be}র\u{9be}প\u{9be}ক\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Tarapacá"), ("ccp", "𑄑𑄢𑄛𑄇"), ("ceb", "Región de Tarapacá"), ("cs", "Region Tarapacá"), ("da", "Tarapacá-regionen"), ("de", "Región de Tarapacá"), ("el", "Ταραπακά"), ("en", "Tarapacá"), ("es", "Región de Tarapacá"), ("et", "Tarapacá piirkond"), ("eu", "Tarapacá eskualdea"), ("fa", "منطقه تاراپاکا"), ("fi", "Tarapacá"), ("fr", "Région de Tarapacá"), ("gl", "Rexión de Tarapacá"), ("gu", "તારાપકા પ\u{acd}રદ\u{ac7}શ"), ("he", "טרפקה"), ("hi", "तारापाका क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "I. regija Tarapacá"), ("hy", "Տարապակա"), ("id", "Wilayah Tarapacá"), ("it", "regione di Tarapacá"), ("ja", "タラパカ州"), ("ka", "ტარაპაკა"), ("kn", "ತರಾಪಾಕಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "타라파카 주"), ("lt", "Tarapakos regionas"), ("lv", "Tarapakas reģions"), ("mk", "Тарапака"), ("mr", "तारापाका प\u{94d}रद\u{947}श"), ("ms", "Tarapaca Region"), ("nb", "Tarapacá"), ("nl", "Tarapacá"), ("no", "Tarapacá"), ("pl", "Tarapacá"), ("pt", "Região de Tarapacá"), ("ro", "Regiunea Tarapacá"), ("ru", "Тарапака"), ("si", "ටරපක\u{dcf} කල\u{dcf}පය"), ("sk", "Tarapacá"), ("sv", "Región de Tarapacá"), ("sw", "Mkoa wa Tarapacá"), ("ta", "ட\u{bcd}ர\u{bbe}பிக\u{bcd}க\u{bbe} பகுதி"), ("te", "ట\u{c3e}ర\u{c3e}ప\u{c3e}క\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ทาราปาคา"), ("tr", "Tarapacá bölgesi"), ("ur", "تاراپاکا ریجن"), ("vi", "Khu vực Tarapacá"), ("zh", "塔拉帕卡大区")]),
                        unofficial_name_list: ["Tarapacá"].to_vec(),
                    }
                ),
                (
                    "VS",
                    Subdivision{
                        name: "VS",
                        country_alpha2: Alpha2::CL,
                        code: "VS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-33.045646), longitude: Some(-71.620361), max_latitude: Some(-33.0178165), min_latitude: Some(-33.2149745), max_longitude: Some(-71.38561109999999), min_longitude: Some(-71.7444525)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم فالبارايسو"), ("be", "вобласць Вальпараіса"), ("bg", "Валпараисо"), ("bn", "ব\u{9be}লপ\u{9be}র\u{9be}ইসো অঞ\u{9cd}চল"), ("ca", "Regió de Valparaíso"), ("ccp", "𑄞\u{11127}𑄣\u{11134}𑄛𑄢\u{1112d}𑄥\u{1112e}"), ("ceb", "Región de Valparaíso"), ("cs", "Region Valparaíso"), ("da", "Valparaíso-regionen"), ("de", "Región de Valparaíso"), ("el", "Βαλπαραΐσο"), ("en", "Valparaíso"), ("es", "Región de Valparaíso"), ("eu", "Valparaíso eskualdea"), ("fa", "منطقه والپارایزو"), ("fi", "Valparaíson alue"), ("fr", "Région de Valparaíso"), ("gl", "Rexión de Valparaíso"), ("gu", "વાલ\u{acd}પરાઇઝો પ\u{acd}રદ\u{ac7}શ"), ("he", "ולפראיסו (מחוז)"), ("hi", "व\u{947}लपर\u{948}सो क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "V. regija Valparaíso"), ("hu", "Valparaíso régió"), ("hy", "Վալպարաիսո"), ("id", "Wilayah Valparaíso"), ("it", "regione di Valparaíso"), ("ja", "バルパライソ州"), ("ka", "ვალპარაისო"), ("kn", "ವಾಲ\u{ccd}ಪರೈಸೊ ಪ\u{ccd}ರದೇಶ"), ("ko", "발파라이소 주"), ("lt", "Valparaiso regionas"), ("lv", "Valparaiso reģions"), ("mr", "वालपराइसो प\u{94d}रद\u{947}श"), ("ms", "Valparaiso Region"), ("nb", "Valparaíso"), ("nl", "Valparaíso"), ("no", "Valparaíso"), ("pl", "Valparaíso"), ("pt", "Região de Valparaíso"), ("ro", "Regiunea Valparaíso"), ("ru", "Вальпараисо"), ("si", "වල\u{dca}පරය\u{dd2}සෝ කල\u{dcf}පය"), ("sk", "Valparaíso"), ("sv", "Región de Valparaíso"), ("ta", "வ\u{bbe}ல\u{bcd}பரைஸோ பகுதி"), ("te", "వ\u{c3e}ల\u{c4d}పర\u{c3e}య\u{c3f}స\u{c4b} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ฟาลปาไรโซ"), ("tr", "Valparaíso bölgesi"), ("ur", "والپارایسو علاقہ"), ("vi", "Khu vực Valparaíso"), ("zh", "瓦尔帕莱索大区")]),
                        unofficial_name_list: ["Valparaíso"].to_vec(),
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
#[cfg(feature = "cl")]
pub fn new() -> Country {
    Country{
        alpha2: Alpha2::CL,
        alpha3: Alpha3::CHL,
        address_format: None,
        continent: Continent::SouthAmerica,
        country_code: 56,
        currency_code: "CLP",
        gec: Some(GEC::CI),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("CHI"),
        iso_long_name: "The Republic of Chile",
        iso_short_name: "Chile",
        official_language_list: ["es"].to_vec(),
        spoken_language_list: ["es"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8, 9].to_vec(),
        national_prefix: "0",
        nationality: Some("Chilean"),
        number: "152",
        postal_code: true,
        postal_code_format: Some("\\d{7}"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthAmerica),
        un_locode: "CL",
        unofficial_name_list: ["Chile", "チリ", "Chili"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([("ab", "Chile"), ("af", "Chili"), ("ak", "Chile"), ("am", "ኁሑ"), ("an", "Chile"), ("ar", "تشيلي"), ("as", "চিলি"), ("ay", "Chile"), ("az", "Çili"), ("ba", "Chile"), ("be", "Чылі"), ("bg", "Чили"), ("bi", "Chile"), ("bn", "চিলি"), ("bn_IN", "চিলি"), ("br", "Chile"), ("bs", "Čile"), ("ca", "Xile"), ("ce", "Чили"), ("ch", "Chile"), ("cs", "Chile"), ("cv", "Чили"), ("cy", "Chile"), ("da", "Chile"), ("de", "Chile"), ("dv", "ޗ\u{7a8}ލ\u{7a9}"), ("dz", "ཅ\u{f72}་ལ\u{f72}།"), ("ee", "Chile"), ("el", "Χιλή"), ("en", "Chile"), ("eo", "Ĉilio"), ("es", "Chile"), ("et", "Tšiili"), ("eu", "Txile"), ("fa", "شیلی"), ("ff", "Ciile"), ("fi", "Chile"), ("fo", "Kili"), ("fr", "Chili"), ("fy", "Sily"), ("ga", "An tSile"), ("gl", "Chile"), ("gn", "Chile"), ("gu", "ચીલી"), ("gv", "Yn Çhillee"), ("ha", "Chile"), ("he", "צ'ילה"), ("hi", "चिली"), ("hr", "Čile"), ("ht", "Chili"), ("hu", "Chile"), ("hy", "Չիլի"), ("ia", "Chile"), ("id", "Chili"), ("io", "Chili"), ("is", "Síle"), ("it", "Cile"), ("iu", "ᓯᓕ"), ("ja", "チリ"), ("ka", "ჩილი"), ("ki", "Chile"), ("kk", "Чили"), ("kl", "Chile"), ("km", "ឈ\u{17b8}ល\u{17b8}"), ("kn", "ಚ\u{cbf}ಲ\u{cbf}"), ("ko", "칠레"), ("ku", "Sîlî"), ("kv", "Чили"), ("kw", "Chile"), ("ky", "Чили"), ("lo", "Chile"), ("lt", "Čilė"), ("lv", "Čīle"), ("mi", "Hiri"), ("mk", "Чиле"), ("ml", "ചിലി"), ("mn", "Чили"), ("mr", "चिली"), ("ms", "Chile"), ("mt", "Ċile"), ("my", "ချ\u{102e}လ\u{102e}ပြည\u{103a}သ\u{1030}\u{1037}သမ\u{1039}မတန\u{102d}\u{102f}င\u{103a}င\u{1036}"), ("na", "Tsire"), ("nb", "Chile"), ("ne", "चिली"), ("nl", "Chili"), ("nn", "Chile"), ("nv", "Chile"), ("oc", "Chile"), ("or", "ଚ\u{b3f}ଲୀ"), ("pa", "ਚਿ\u{a71}ਲੀ"), ("pi", "चिल\u{947}"), ("pl", "Chile"), ("ps", "چېلي"), ("pt", "Chile"), ("pt_BR", "Chile"), ("ro", "Chile"), ("ru", "Чили"), ("rw", "Shili"), ("sc", "Tzile"), ("sd", "چلي"), ("si", "ච\u{dd2}ල\u{dd3}"), ("sk", "Čile"), ("sl", "Čile"), ("so", "Jili"), ("sq", "Kili"), ("sr", "Чиле"), ("sv", "Chile"), ("sw", "Chile"), ("ta", "சிலி"), ("te", "చ\u{c3f}ల\u{c40}"), ("tg", "Чили"), ("th", "ช\u{e34}ล\u{e35}"), ("ti", "ቺሊ"), ("tk", "Çili"), ("tl", "Tsile"), ("tr", "Şili"), ("tt", "Чили"), ("ug", "چىلى"), ("uk", "Чилі"), ("ur", "چلی"), ("uz", "Chili"), ("ve", "Shile"), ("vi", "Chi-lê"), ("wa", "Tchili"), ("wo", "Ciili"), ("xh", "Chile"), ("yo", "Tsílè"), ("zh_CN", "智利"), ("zh_HK", "智利"), ("zh_TW", "智利"), ("zu", "I-Chile")]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
