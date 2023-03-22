// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Armenia

#[cfg(all(feature = "am", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::AM;
    pub const ALPHA3: Alpha3 = Alpha3::ARM;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 374;
    pub const CURRENCY_CODE: &str = "AMD";
    pub const GEC: Option<GEC> = Some(GEC::AM);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("ARM");
    pub const ISO_SHORT_NAME: &str = "Armenia";
    pub const ISO_LONG_NAME: &str = "The Republic of Armenia";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["hy", "ru"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["hy", "ru"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "8";
    pub const NATIONALITY: Option<&str> = Some("Armenian");
    pub const NUMBER: &str = "051";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("(?:37)?\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternAsia);
    pub const UN_LOCODE: &str = "AM";
    pub const UNOFFICIAL_NAME_LIST: &[&str] =
        &["Armenia", "Armenien", "Arménie", "アルメニア", "Armenië"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Armenia"),
        ("af", "Armenië"),
        ("ak", "Armenia"),
        ("am", "ጐሴሣኒ።"),
        ("an", "Armenia"),
        ("ar", "أرمينيا"),
        ("as", "আৰ\u{9cd}মেনিয়\u{9be}"),
        ("ay", "Armenia"),
        ("az", "Ermənistan"),
        ("ba", "Armenia"),
        ("be", "Арменія"),
        ("bg", "Армения"),
        ("bi", "Armenia"),
        ("bn", "আর\u{9cd}মেনিয়\u{9be}"),
        ("bn_IN", "আর\u{9cd}মেনিয়\u{9be}"),
        ("br", "Armenia"),
        ("bs", "Armenija"),
        ("ca", "Armènia"),
        ("ce", "Аремалойн"),
        ("ch", "Armenia"),
        ("cs", "Arménie"),
        ("cv", "Аремалойн"),
        ("cy", "Armenia"),
        ("da", "Armenien"),
        ("de", "Armenien"),
        ("dv", "އ\u{7a6}ރ\u{7aa}މ\u{7a9}ނ\u{7a8}އ\u{7a7}"),
        ("dz", "ཨར་མ\u{f72}་ན\u{f72}་ཡ།"),
        ("ee", "Armenia"),
        ("el", "Αρμενία"),
        ("en", "Armenia"),
        ("eo", "Armenio"),
        ("es", "Armenia"),
        ("et", "Armeenia"),
        ("eu", "Armenia"),
        ("fa", "ارمنستان"),
        ("ff", "Armaaniya"),
        ("fi", "Armenia"),
        ("fo", "Armenia"),
        ("fr", "Arménie"),
        ("fy", "Armeenje"),
        ("ga", "An Airméin"),
        ("gl", "Armenia"),
        ("gn", "Armenia"),
        ("gu", "આર\u{acd}મ\u{ac7}નિયા"),
        ("gv", "Yn Armeain"),
        ("ha", "Armeniya"),
        ("he", "ארמניה"),
        ("hi", "आर\u{94d}मीनिया"),
        ("hr", "Armenija"),
        ("ht", "Ameni"),
        ("hu", "Örményország"),
        ("hy", "Հայաստանի Հանրապետութիւն"),
        ("ia", "Armenia"),
        ("id", "Armenia"),
        ("io", "Armenia"),
        ("is", "Armenía"),
        ("it", "Armenia"),
        ("iu", "Armenia"),
        ("ja", "アルメニア"),
        ("ka", "სასომხეთი"),
        ("ki", "Armenia"),
        ("kk", "Армения"),
        ("kl", "Armenia"),
        ("km", "អាមេន\u{17b8}"),
        ("kn", "ಆರ\u{ccd}ಮೇನ\u{cbf}ಯಾ"),
        ("ko", "아르메니아"),
        ("ku", "Ermenistan"),
        ("kv", "Армения"),
        ("kw", "Armeni"),
        ("ky", "Армения"),
        ("lo", "ປະເທດອາກເມນ\u{eb5}"),
        ("lt", "Armėnija"),
        ("lv", "Armēnija"),
        ("mi", "Āmenia"),
        ("mk", "Ерменија"),
        ("ml", "അര\u{d4d}\u{200d}മീനിയ"),
        ("mn", "Армен"),
        ("mr", "अम\u{947}रिका"),
        ("ms", "Armenia"),
        ("mt", "Armenja"),
        (
            "my",
            "အာမေးန\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Arminiya"),
        ("nb", "Armenia"),
        ("ne", "अरम\u{947}निया"),
        ("nl", "Armenië"),
        ("nn", "Armenia"),
        ("nv", "Aooméénii Bikéyah"),
        ("oc", "Armenia"),
        ("or", "ଆର\u{b4d}ମେନ\u{b3f}ୟ\u{b3e}"),
        ("pa", "ਅਰਮੀਨਾ"),
        ("pi", "आर\u{94d}मीनिया"),
        ("pl", "Armenia"),
        ("ps", "ارمنستان"),
        ("pt", "Arménia"),
        ("pt_BR", "Armênia"),
        ("ro", "Armenia"),
        ("ru", "Армения"),
        ("rw", "Arumeniya"),
        ("sc", "Armènia"),
        ("sd", "آرمينيا"),
        ("si", "ආමේන\u{dd2}ය\u{dcf}"),
        ("sk", "Arménsko"),
        ("sl", "Armenija"),
        ("so", "Armeeniya"),
        ("sq", "Armeni"),
        ("sr", "Јерменија"),
        ("sv", "Armenien"),
        ("sw", "Armenia"),
        ("ta", "அர\u{bcd}மெனிய\u{bbe}"),
        ("te", "అమ\u{c47}ర\u{c3f}క\u{c3e}"),
        ("tg", "Арманистон"),
        ("th", "อาร\u{e4c}เมเน\u{e35}ย"),
        ("ti", "ኣርሜንያ"),
        ("tk", "Ermenistan"),
        ("tl", "Armenya"),
        ("tr", "Ermenistan"),
        ("tt", "Әрмәнстан"),
        ("ug", "ئەرمېنىيە"),
        ("uk", "Вірменія"),
        ("ur", "آرمینیا"),
        ("uz", "Armaniston"),
        ("ve", "Armenia"),
        ("vi", "Ac-mê-ni"),
        ("wa", "Årmeneye"),
        ("wo", "Armeeni"),
        ("xh", "Armenia"),
        ("yo", "Arméníà"),
        ("zh_CN", "亚美尼亚"),
        ("zh_HK", "亞美尼亞"),
        ("zh_TW", "亞美尼亞"),
        ("zu", "Armenia"),
    ];
    #[cfg(all(feature = "am", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 40.069099;
        pub const LONGITUDE: f64 = 45.038189;
        pub const MAX_LATITUDE: f64 = 41.300993;
        pub const MAX_LONGITUDE: f64 = 46.6342219;
        pub const MIN_LATITUDE: f64 = 38.840244;
        pub const MIN_LONGITUDE: f64 = 43.4472601;
        pub const NORTHEAST_LATITUDE: f64 = 41.300993;
        pub const NORTHEAST_LONGITUDE: f64 = 46.6342219;
        pub const SOUTHWEST_LATITUDE: f64 = 38.840244;
        pub const SOUTHWEST_LONGITUDE: f64 = 43.4472601;
    }
}
#[cfg(all(feature = "am", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 40.069099,
            longitude: 45.038189,
            max_latitude: 41.300993,
            max_longitude: 46.6342219,
            min_latitude: 38.840244,
            min_longitude: 43.4472601,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 41.300993,
                    longitude: 46.6342219,
                },
                southwest: CountryGeoBound {
                    latitude: 38.840244,
                    longitude: 43.4472601,
                },
            },
        }
    }
}

#[cfg(all(feature = "am", feature = "subdivisions"))]
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
                        country_alpha2: Alpha2::AM,
                        code: "AG",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة آراغاتسوتن"), ("az", "Ağin"), ("be", "Марз Арагацотн"), ("bg", "Арагацотн"), ("bn", "আর\u{9be}\u{9be}গ\u{9be}টসন অঞ\u{9cd}চল"), ("bs", "Aragacotn"), ("ca", "Aragadzotn"), ("ccp", "𑄃𑄢𑄉𑄖\u{11134}𑄥\u{11127}𑄖\u{11133}𑄚\u{11134}"), ("ceb", "Aragatsotni Marz"), ("cs", "Aragacotn"), ("da", "Aragatsotn Region"), ("de", "Aragazotn"), ("el", "Επαρχία Αραγκατσότν"), ("en", "Aragatsotn"), ("es", "Aragatsotn"), ("et", "Aragatsotni maakond"), ("eu", "Aragatsotn"), ("fa", "استان آراگاتسوتن"), ("fi", "Aragatsotn"), ("fr", "Aragatsotn"), ("gl", "Provincia de Aragatsotn"), ("gu", "એરાગાટ\u{acd}સોટન પ\u{acd}રદ\u{ac7}શ"), ("hi", "अरागत\u{94d}सोत\u{94d}न"), ("hr", "Aragacotn"), ("hu", "Aragacotn tartomány"), ("hy", "Արագածոտնի մարզ"), ("id", "Aragatsotn"), ("it", "Aragatsotn"), ("ja", "アラガツォトゥン地方"), ("ka", "არაგაწოტნის პროვინცია"), ("kn", "ಅರಗತ\u{ccd}ಸೊಟ\u{ccd}ನ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "아라가초튼 주"), ("lt", "Aragacotno sritis"), ("lv", "Aragatsotnas province"), ("mk", "Арагацотн"), ("mn", "Арагацотн аймаг"), ("mr", "अ\u{945}रामासॉटन प\u{94d}रद\u{947}श"), ("ms", "Wilayah Aragatsotn"), ("nb", "Aragatsotn"), ("ne", "अरागत\u{94d}सोत\u{94d}न प\u{94d}रान\u{94d}त"), ("nl", "Aragatsotn"), ("no", "Aragatsotn"), ("pa", "ਅਰਾਗਤਸ\u{a4b}ਤਨ"), ("pl", "Prowincja Aragacotn"), ("pt", "Aragatsotn"), ("ro", "Provincia Aragatsotn"), ("ru", "Арагацотнская область"), ("si", "අරගට\u{dca}සොට\u{dca}න\u{dca} කල\u{dcf}පය"), ("sr", "Арагацотн"), ("sr_Latn", "Aragacotn"), ("sv", "Aragatsotn"), ("ta", "அரைக\u{bcd}க\u{bbe}ட\u{bcd}சோடன\u{bcd} பகுதி"), ("te", "ఆర\u{c3e}గ\u{c3e}ట\u{c4d}స\u{c4b}ట\u{c3f}న\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตอาร\u{e4c}แกทซอท\u{e4c}น"), ("tr", "Aragatsotn"), ("uk", "Араґацотн"), ("ur", "آراگاتسوتن صوبہ"), ("vi", "Aragatsotn"), ("yue", "阿拉加措特恩省"), ("yue_Hans", "阿拉加措特恩省"), ("zh", "阿拉加措特恩省")]),
                        unofficial_name_list: ["Aragacotn"].to_vec(),
                    }
                ),
                (
                    "AR",
                    Subdivision{
                        name: "AR",
                        country_alpha2: Alpha2::AM,
                        code: "AR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.9753273), longitude: Some(44.8338058), max_latitude: Some(40.189931), min_latitude: Some(39.713722), max_longitude: Some(45.11704899999999), min_longitude: Some(44.3149281)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة أرارات"), ("az", "Ararat mərzi"), ("be", "Марз Арарат"), ("bg", "Арарат"), ("bn", "আর\u{9be}র\u{9be}ত প\u{9cd}রদেশ"), ("bs", "Ararat (pokrajina)"), ("ca", "Ararat"), ("ccp", "𑄃𑄢𑄢𑄖\u{11134}"), ("ceb", "Ararati Marz"), ("cs", "Ararat"), ("da", "Ararat Province"), ("de", "Region Ararat"), ("el", "Επαρχία Αραράτ"), ("en", "Ararat"), ("es", "Ararat"), ("et", "Ararati maakond"), ("eu", "Ararat"), ("fa", "استان آرارات"), ("fi", "Ararat"), ("fr", "Ararat"), ("gu", "અરરાટ પ\u{acd}રા\u{a82}ત"), ("hi", "अरारत प\u{94d}रा\u{902}त"), ("hr", "Ararat"), ("hu", "Ararat tartomány"), ("hy", "Արարատի մարզ"), ("id", "Provinsi Ararat"), ("it", "provincia di Ararat"), ("ja", "アララト地方"), ("ka", "არარატის პროვინცია"), ("kn", "ಅರರಾತ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아라라트 주"), ("lt", "Ararato sritis"), ("lv", "Ararata reģions"), ("mk", "Арарат"), ("mn", "Арарат аймаг"), ("mr", "अरारत प\u{94d}रा\u{902}त"), ("ms", "Wilayah Ararat"), ("nb", "Ararat"), ("ne", "अरारत प\u{94d}रान\u{94d}त"), ("nl", "Ararat"), ("no", "Ararat"), ("pl", "Prowincja Ararat"), ("pt", "Ararate"), ("ro", "Provincia Ararat"), ("ru", "Араратская область"), ("si", "අරරට\u{dca} පළ\u{dcf}ත"), ("sr", "Арарат"), ("sr_Latn", "Ararat"), ("sv", "Ararat"), ("ta", "அர\u{bbe}ரட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఆర\u{c3e}ర\u{c3e}ట\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอราแรต"), ("tr", "Ararat İli"), ("uk", "Арарат"), ("ur", "ارارات صوبہ"), ("vi", "Ararat"), ("zh", "亞拉拉特省")]),
                        unofficial_name_list: ["Ararat"].to_vec(),
                    }
                ),
                (
                    "AV",
                    Subdivision{
                        name: "AV",
                        country_alpha2: Alpha2::AM,
                        code: "AV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.1315615), longitude: Some(43.8325355), max_latitude: Some(40.283289), min_latitude: Some(40.01846700000001), max_longitude: Some(44.4403099), min_longitude: Some(43.6464811)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة آرماوير"), ("az", "Armavir mərzi"), ("be", "Марз Армавір"), ("bg", "Армавир"), ("bn", "আরম\u{9be}ভির অঞ\u{9cd}চল"), ("bs", "Armavir (pokrajina)"), ("ca", "Armavir"), ("ccp", "𑄃𑄢\u{11134}𑄟𑄞\u{11128}𑄢\u{11134}"), ("ceb", "Armaviri Marz"), ("cs", "Kategorie:Armavir (provincie)"), ("da", "Armavir Region"), ("de", "Armawir"), ("el", "Αρμαβίρ"), ("en", "Armavir"), ("es", "Armavir"), ("et", "Armaviri maakond"), ("eu", "Armavir"), ("fa", "استان آرماویر"), ("fi", "Armavir"), ("fr", "Armavir"), ("gu", "અર\u{acd}માવિર પ\u{acd}રદ\u{ac7}શ"), ("hi", "अर\u{94d}मावीर प\u{94d}रा\u{902}त"), ("hr", "Armavir"), ("hu", "Armavir tartomány"), ("hy", "Արմավիրի մարզ"), ("id", "Armavir"), ("it", "provincia di Armavir"), ("ja", "アルマヴィル地方"), ("ka", "არმავირის პროვინცია"), ("kn", "ಆರ\u{ccd}ಮವೀ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "아르마비르 주"), ("lt", "Armaviro sritis"), ("lv", "Armaviras reģions"), ("mk", "Армавир"), ("mn", "Армавир аймаг"), ("mr", "अर\u{94d}मावीर प\u{94d}रद\u{947}श"), ("ms", "Wilayah Armavir"), ("nb", "Armavir"), ("ne", "अर\u{94d}मावीर प\u{94d}रान\u{94d}त"), ("nl", "Armavir"), ("no", "Armavir"), ("pa", "ਅਰਮਾਵੀਰ ਸ\u{a42}ਬਾ"), ("pl", "Prowincja Armawir"), ("pt", "Armavir"), ("ro", "Provincia Armavir"), ("ru", "Армавирская область"), ("si", "අර\u{dca}මව\u{dd2}ර\u{dca} කල\u{dcf}පය"), ("sr", "Армавир"), ("sr_Latn", "Armavir"), ("sv", "Armavir"), ("ta", "அர\u{bcd}மவ\u{bc0}ர\u{bcd} பகுதி"), ("te", "ఆర\u{c4d}మవ\u{c40}ర\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "จ\u{e31}งหว\u{e31}ดอาร\u{e4c}มาเว\u{e35}ยร\u{e4c}"), ("tr", "Armavir İdari Bölgesi"), ("uk", "Армавір"), ("ur", "آرامویر صوبہ"), ("vi", "Armavir"), ("zh", "亞馬維爾省")]),
                        unofficial_name_list: ["Armavir"].to_vec(),
                    }
                ),
                (
                    "ER",
                    Subdivision{
                        name: "ER",
                        country_alpha2: Alpha2::AM,
                        code: "ER",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.183333), longitude: Some(44.516667), max_latitude: Some(40.2426667), min_latitude: Some(40.0641141), max_longitude: Some(44.6150493), min_longitude: Some(44.3620849)}),
                        comments: None,
                        subdivision_type: SubdivisionType::City,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Jerewan"), ("am", "ዬሬቫን"), ("ar", "يريفان"), ("az", "İrəvan"), ("be", "Ерэван"), ("bg", "Ереван"), ("bn", "ইয\u{9bc}েরেভ\u{9be}ন"), ("bs", "Erevan"), ("ca", "Erevan"), ("ccp", "𑄃\u{11128}𑄠𑄬𑄢𑄬𑄞\u{11127}𑄚\u{11134}"), ("ceb", "Yerevan (lalawigan)"), ("cs", "Jerevan"), ("cy", "Yerevan"), ("da", "Jerevan"), ("de", "Jerewan"), ("el", "Γερεβάν"), ("en", "Yerevan"), ("es", "Ereván"), ("et", "Jerevan"), ("eu", "Erevan"), ("fa", "ایروان"), ("fi", "Jerevan"), ("fr", "Erevan"), ("ga", "Eireaván"), ("gl", "Iereván"), ("gu", "ય\u{ac7}ર\u{ac7}વન"), ("he", "ירוואן"), ("hi", "य\u{947}रवान"), ("hr", "Erevan"), ("hu", "Jereván"), ("hy", "Երևան"), ("id", "Yerevan"), ("is", "Jerevan"), ("it", "Erevan"), ("ja", "エレバン"), ("jv", "Yerevan"), ("ka", "ერევანი"), ("kk", "Ереван"), ("kn", "ಯ\u{cc6}ರ\u{cc6}ವಾನ\u{ccd}"), ("ko", "예레반"), ("ky", "Ереван"), ("lt", "Jerevanas"), ("lv", "Erevāna"), ("mk", "Ереван"), ("ml", "യെറിവ\u{d3e}ൻ"), ("mn", "Ереван"), ("mr", "य\u{947}र\u{947}व\u{94d}हान"), ("ms", "Yerevan"), ("nb", "Jerevan"), ("ne", "य\u{947}रवान प\u{94d}रान\u{94d}त"), ("nl", "Jerevan"), ("no", "Jerevan"), ("pa", "ਯ\u{a47}ਰਵਾਨ"), ("pl", "Erywań"), ("pt", "Erevan"), ("ro", "Erevan"), ("ru", "Ереван"), ("si", "යේරව\u{dcf}න\u{dca}"), ("sk", "Jerevan"), ("sl", "Erevan"), ("sq", "Jerevani"), ("sr", "Јереван"), ("sr_Latn", "Jerevan"), ("sv", "Jerevan"), ("sw", "Yerevan"), ("ta", "யெரெவ\u{bbe}ன\u{bcd}"), ("te", "య\u{c46}రవ\u{c3e}న\u{c4d}"), ("th", "เยเรวาน"), ("tk", "Ýerewan"), ("tr", "Erivan"), ("uk", "Єреван"), ("ur", "یریوان"), ("uz", "Yerevan"), ("vi", "Yerevan"), ("yo", "Yerevan"), ("yo_BJ", "Yerevan"), ("yue", "埃里溫"), ("yue_Hans", "埃里温"), ("zh", "葉里溫")]),
                        unofficial_name_list: ["Erevan"].to_vec(),
                    }
                ),
                (
                    "GR",
                    Subdivision{
                        name: "GR",
                        country_alpha2: Alpha2::AM,
                        code: "GR",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة غغاركونيك"), ("az", "Geğarkunik"), ("be", "Марз Гегаркунік"), ("bg", "Гегаркуник"), ("bn", "গেঘ\u{9be}রক\u{9c1}নিক প\u{9cd}রদেশ"), ("bs", "Geharkunik"), ("ca", "Província de Gegharkunik"), ("ccp", "𑄎\u{11128}𑄊\u{11127}𑄢\u{11134}𑄇\u{1112a}𑄚\u{11128}𑄇\u{11134}"), ("ceb", "Geghark’unik’i Marz"), ("cs", "Gegharkunik"), ("cy", "Gegharkunik"), ("da", "Gegharkunik Province"), ("de", "Gegharkunik"), ("el", "Γκεγκαρκουνίκ"), ("en", "Gegharkunik"), ("es", "Geghark’unik’"), ("et", "Gegharkhunikhi maakond"), ("eu", "Gegharkunik"), ("fa", "منطقه دریاچه سوان"), ("fi", "Geghark’unik’"), ("fr", "Gegharkunik"), ("gu", "ગ\u{ac7}ઘરક\u{acd}ય\u{ac1}નિક પ\u{acd}રા\u{a82}ત"), ("hi", "ग\u{947}घार\u{94d}क\u{941}निक प\u{94d}रा\u{902}त"), ("hr", "Gegharkunik"), ("hu", "Gegarkounik tartomány"), ("hy", "Գեղարքունիքի մարզ"), ("id", "Provinsi Gegharkunik"), ("it", "Gegharkunik"), ("ja", "ゲガルクニク地方"), ("ka", "გეღარქუნიქის პროვინცია"), ("kn", "ಜ\u{cbf}ಘಾರ\u{ccd}ಕುನ\u{cbf}ಕ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "게가르쿠니크 주"), ("lt", "Gegarkuniko sritis"), ("lv", "Gegarkunikas reģions"), ("mk", "Гехаркуник"), ("mn", "Гехаркуник аймаг"), ("mr", "ग\u{947}घरक\u{94d}य\u{941}निक प\u{94d}रा\u{902}त"), ("ms", "Gegharkunik"), ("nb", "Gegharkunik"), ("ne", "ग\u{947}घार\u{94d}क\u{941}निक प\u{94d}रान\u{94d}त"), ("nl", "Gegharkoenik"), ("no", "Gegharkunik"), ("pl", "Prowincja Gegharkunik"), ("pt", "Gegharkunik"), ("ro", "Provincia Gegharkunik"), ("ru", "Гехаркуникская область"), ("si", "ගෙග\u{dca}හ\u{dcf}ක\u{dd4}න\u{dd2}ක\u{dca} පළ\u{dcf}ත"), ("sr", "Гехаркуник"), ("sr_Latn", "Geharkunik"), ("sv", "Gegharkunik"), ("ta", "கேக\u{bbe}ர\u{bcd}குனிக\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గ\u{c46}గ\u{c3e}ర\u{c4d}కున\u{c3f}క\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเกการ\u{e4c}ค\u{e39}น\u{e34}ค"), ("tr", "Geğarkunik"), ("uk", "Ґегаркунік"), ("ur", "گیغارکونیک صوبہ"), ("vi", "Gegharkunik"), ("zh", "格加爾庫尼克省")]),
                        unofficial_name_list: ["Gegharkunick"].to_vec(),
                    }
                ),
                (
                    "KT",
                    Subdivision{
                        name: "KT",
                        country_alpha2: Alpha2::AM,
                        code: "KT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.4277896), longitude: Some(44.6641741), max_latitude: Some(40.71982999999999), min_latitude: Some(40.0972899), max_longitude: Some(45.043105), min_longitude: Some(44.398311)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كوتايك"), ("az", "Kotayk mərzi"), ("be", "Марз Катайк"), ("bg", "Котайк"), ("bn", "কোত\u{9be}য\u{9bc}েক অঞ\u{9cd}চল"), ("bs", "Kotajk"), ("ca", "Regió de Kotayk"), ("ccp", "𑄇\u{11127}𑄑\u{1112d}𑄇\u{11134}"), ("ceb", "Kotayk’i Marz"), ("cs", "Kotajk"), ("da", "Kotayk Region"), ("de", "Region Kotajk"), ("el", "Κόταγικ"), ("en", "Kotayk"), ("es", "Kotayk’"), ("et", "Kotajkhi maakond"), ("eu", "Kotaik"), ("fa", "استان کوتایک"), ("fi", "Kotajk’"), ("fr", "Kotayk"), ("gu", "કોટાક પ\u{acd}રા\u{a82}ત"), ("hi", "कोटायक प\u{94d}रा\u{902}त"), ("hr", "Kotajk"), ("hu", "Kotajk tartomány"), ("hy", "Կոտայքի մարզ"), ("id", "Kotayk"), ("it", "provincia di Kotayk’"), ("ja", "コタイク地方"), ("ka", "კოტაიქის პროვინცია"), ("kn", "ಕೋಟ\u{ccd}ಯಾಕ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "코타이크 주"), ("lt", "Kotaiko sritis"), ("lv", "Kotaikas reģions"), ("mk", "Котајк"), ("mn", "Котайк аймаг"), ("mr", "कोतक प\u{94d}रद\u{947}श"), ("ms", "Kotayk"), ("nb", "Kotajk"), ("ne", "कोटायक प\u{94d}रान\u{94d}त"), ("nl", "Kotayk"), ("no", "Kotajk"), ("pa", "ਕ\u{a4b}ਟਾਇਕ"), ("pl", "Prowincja Kotajk"), ("pt", "Kotayk"), ("ro", "Provincia Kotayk"), ("ru", "Котайкская область"), ("si", "කොටය\u{dd2}ක\u{dca} කල\u{dcf}පය"), ("sr", "Котајк"), ("sr_Latn", "Kotajk"), ("sv", "Kotajk"), ("ta", "கொடய\u{bcd}க\u{bcd} பகுதி"), ("te", "క\u{c4b}ట\u{c47}క\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโคเทก"), ("tr", "Kotayk"), ("uk", "Котайк"), ("ur", "کوتایک صوبہ"), ("vi", "Kotayk"), ("zh", "科泰克省")]),
                        unofficial_name_list: ["Kotaik"].to_vec(),
                    }
                ),
                (
                    "LO",
                    Subdivision{
                        name: "LO",
                        country_alpha2: Alpha2::AM,
                        code: "LO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.969845), longitude: Some(44.490014), max_latitude: Some(41.299259), min_latitude: Some(40.668043), max_longitude: Some(44.960212), min_longitude: Some(44.016761)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة لوري"), ("az", "Loru mahalı"), ("be", "Марз Лары"), ("bg", "Лори"), ("bn", "লরি অঞ\u{9cd}চল"), ("bs", "Lori"), ("ca", "Lori"), ("ccp", "𑄣\u{11127}𑄢\u{11128}"), ("ceb", "Lorru Marz"), ("cs", "Lorri"), ("da", "Lori Region"), ("de", "Lori"), ("el", "Λόρι"), ("en", "Lori"), ("es", "Lorri"), ("et", "Lori maakond"), ("eu", "Lori"), ("fa", "استان لوری"), ("fi", "Lorri"), ("fr", "Lorri"), ("gu", "લોરી પ\u{acd}રદ\u{ac7}શ"), ("hi", "लोरी प\u{94d}रा\u{902}त"), ("hr", "Lori"), ("hu", "Lorj tartomány"), ("hy", "Լոռու մարզ"), ("id", "Lori"), ("it", "Lori"), ("ja", "ロリ地方"), ("ka", "ლორეს პროვინცია"), ("kn", "ಲೋರ\u{cbf} ಪ\u{ccd}ರದೇಶ"), ("ko", "로리 주"), ("lt", "Lorio sritis"), ("lv", "Lori reģions"), ("mk", "Лори"), ("mn", "Лори аймаг"), ("mr", "लोरी प\u{94d}रद\u{947}श"), ("ms", "Lorri"), ("nb", "Lori"), ("ne", "लोरी प\u{94d}रान\u{94d}त"), ("nl", "Lori"), ("no", "Lori"), ("pa", "ਲ\u{a4b}ਰੀ ਪ\u{a4d}ਰਾ\u{a02}ਤ"), ("pl", "Prowincja Lori"), ("pt", "Lorri"), ("ro", "Provincia Lori"), ("ru", "Лорийская область"), ("si", "ලොර\u{dd2} කල\u{dcf}පය"), ("sr", "Лори"), ("sr_Latn", "Lori"), ("sv", "Lorri"), ("ta", "லோரி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c4b}ర\u{c3f} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "จ\u{e31}งหว\u{e31}ดโลร\u{e35}"), ("tr", "Lori"), ("uk", "Лорі"), ("ur", "لوری صوبہ"), ("vi", "Lori"), ("zh", "洛里省")]),
                        unofficial_name_list: ["Lorri"].to_vec(),
                    }
                ),
                (
                    "SH",
                    Subdivision{
                        name: "SH",
                        country_alpha2: Alpha2::AM,
                        code: "SH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.9630814), longitude: Some(43.8102461), max_latitude: Some(41.1808931), min_latitude: Some(40.439465), max_longitude: Some(44.2037191), min_longitude: Some(43.4497797)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة شيراك"), ("az", "Amasiya"), ("be", "Марз Шырак"), ("bg", "Ширак"), ("bn", "শির\u{9be}ক অঞ\u{9cd}চল"), ("bs", "Širak"), ("ca", "Xirak"), ("ccp", "𑄥\u{11128}𑄢𑄇\u{11134}"), ("ceb", "Shiraki Marz"), ("cs", "Širak"), ("da", "Shirak Region"), ("de", "Schirak"), ("el", "Σιράκ"), ("en", "Shirak"), ("es", "Shirak"), ("et", "Širaki maakond"), ("eu", "Xirak"), ("fa", "استان شیراک"), ("fi", "Širak"), ("fr", "Shirak"), ("gu", "શિરક પ\u{acd}રદ\u{ac7}શ"), ("hi", "शिराक प\u{94d}रा\u{902}त"), ("hr", "Širak"), ("hu", "Sirak tartomány"), ("hy", "Շիրակի մարզ"), ("id", "Shirak"), ("it", "provincia di Shirak"), ("ja", "シラク地方"), ("ka", "შირაკის პროვინცია"), ("kn", "ಶ\u{cbf}ರಾಕ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "시라크 주"), ("lt", "Širako sritis"), ("lv", "Širakas reģions"), ("mk", "Ширак"), ("mn", "Ширак аймаг"), ("mr", "शिरक प\u{94d}रद\u{947}श"), ("ms", "Shirak"), ("nb", "Sjirak"), ("ne", "शिराक प\u{94d}रान\u{94d}त"), ("nl", "Sjirak"), ("no", "Sjirak"), ("pa", "ਸ\u{a3c}ਿਰਾਕ"), ("pl", "Prowincja Szirak"), ("pt", "Shirak"), ("ro", "Provincia Shirak"), ("ru", "Ширакская область"), ("si", "ශ\u{dd2}රක\u{dca} කල\u{dcf}පය"), ("sr", "Ширак"), ("sr_Latn", "Širak"), ("sv", "Sjirak"), ("ta", "ஷிர\u{bcd}க\u{bcd} பகுதி"), ("te", "ష\u{c3f}ర\u{c3e}క\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตเฟอร\u{e4c}กานา"), ("tr", "Şirak İdari Bölgesi"), ("uk", "Ширак"), ("ur", "شیراک صوبہ"), ("vi", "Shirak"), ("zh", "希拉克省")]),
                        unofficial_name_list: ["Širak"].to_vec(),
                    }
                ),
                (
                    "SU",
                    Subdivision{
                        name: "SU",
                        country_alpha2: Alpha2::AM,
                        code: "SU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.3194392), longitude: Some(46.14609189999999), max_latitude: Some(39.854328), min_latitude: Some(38.830521), max_longitude: Some(46.630035), min_longitude: Some(45.70893909999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة سيونيك"), ("az", "Sünik"), ("be", "Марз Сюнік"), ("bg", "Сюник"), ("bn", "স\u{9be}য\u{9bc}\u{9c1}নিক প\u{9cd}রদেশ"), ("bs", "Sjunik"), ("ca", "Siunia"), ("ccp", "𑄥\u{1112d}𑄠\u{1112a}𑄚\u{11128}𑄇\u{11134}"), ("ceb", "Syunik’i Marz"), ("cs", "Sjunik"), ("cy", "Syunik"), ("da", "Syunik Province"), ("de", "Sjunik"), ("el", "Σγιούνικ"), ("en", "Syunik"), ("es", "Syunik’"), ("et", "Sjunikhi maakond"), ("eu", "Siunik"), ("fa", "استان سیونیک"), ("fi", "Sjunik’"), ("fr", "Syunik"), ("gu", "સ\u{acd}ય\u{ac1}નિક પ\u{acd}રા\u{a82}ત"), ("hi", "स\u{94d}य\u{941}निक प\u{94d}रा\u{902}त"), ("hr", "Sjunik"), ("hu", "Szjounik tartomány"), ("hy", "Սյունիքի մարզ"), ("id", "Syunik"), ("it", "Syunik"), ("ja", "シュニク地方"), ("ka", "სიუნიქის პროვინცია"), ("kn", "ಸೈನ\u{cbf}ಕ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "슈니크 주"), ("lt", "Siuniko sritis"), ("lv", "Sjunikas reģions"), ("mk", "Сјуник"), ("mn", "Сюник аймаг"), ("mr", "स\u{94d}य\u{941}निक प\u{94d}रा\u{902}त"), ("ms", "Wilayah Syunik"), ("nb", "Syunik"), ("ne", "स\u{94d}य\u{941}निक प\u{94d}रान\u{94d}त"), ("nl", "Sjoenik"), ("no", "Syunik"), ("pl", "Prowincja Sjunik"), ("pt", "Syunik"), ("ro", "Provincia Syunik"), ("ru", "Сюникская область"), ("si", "සය\u{dd4}න\u{dd2}ක\u{dca} පළ\u{dcf}ත"), ("sl", "Sjunik"), ("sr", "Сјуник"), ("sr_Latn", "Sjunik"), ("sv", "Siunik"), ("ta", "சுயூனிக\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c48}యూన\u{c3f}క\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e39}ย\u{e4c}น\u{e34}ค"), ("tr", "Syunik ili"), ("uk", "Сюнік"), ("ur", "سیونیک صوبہ"), ("vi", "Syunik"), ("zh", "休尼克省")]),
                        unofficial_name_list: ["Syunik'"].to_vec(),
                    }
                ),
                (
                    "TV",
                    Subdivision{
                        name: "TV",
                        country_alpha2: Alpha2::AM,
                        code: "TV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.8866296), longitude: Some(45.339349), max_latitude: Some(41.3018379), min_latitude: Some(40.648954), max_longitude: Some(45.5956799), min_longitude: Some(44.766082)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة تاووش"), ("az", "Tavuş"), ("be", "Марз Тавуш"), ("bg", "Тавуш"), ("bn", "ত\u{9be}ব\u{9c1}শ অঞ\u{9cd}চল"), ("bs", "Tavuš"), ("ca", "Tavush"), ("ccp", "𑄑𑄞\u{1112a}𑄌\u{11134}"), ("ceb", "Tavushi Marz"), ("cs", "Tavuš"), ("da", "Tavush Region"), ("de", "Tawusch"), ("el", "Ταβούς"), ("en", "Tavush"), ("es", "Tavush"), ("et", "Tavuši maakond"), ("eu", "Tavux"), ("fa", "استان تاووش"), ("fi", "Tavuš"), ("fr", "Tavush"), ("gu", "તાવ\u{ac1}શ પ\u{acd}રદ\u{ac7}શ"), ("hi", "ताव\u{942}श"), ("hr", "Tavuš"), ("hu", "Tavous tartomány"), ("hy", "Տավուշի մարզ"), ("id", "Provinsi Tavush"), ("it", "provincia di Tavowš"), ("ja", "タヴシュ地方"), ("ka", "ტავუშის პროვინცია"), ("kn", "ತವಷ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "타부시 주"), ("lt", "Tavušo sritis"), ("lv", "Tavušas reģions"), ("mk", "Тавуш"), ("mn", "Тавуш аймаг"), ("mr", "तावश प\u{94d}रद\u{947}श"), ("ms", "Wilayah Tavush"), ("nb", "Tavusj"), ("ne", "ताव\u{942}श प\u{94d}रान\u{94d}त"), ("nl", "Tavoesj"), ("no", "Tavusj"), ("pa", "ਤਵ\u{a42}ਸ\u{a3c}"), ("pl", "Prowincja Tawusz"), ("pt", "Tavush"), ("ro", "Provincia Tavuș"), ("ru", "Тавушская область"), ("si", "ටව\u{dd4}ෂ\u{dca} කල\u{dcf}පය"), ("sr", "Тавуш"), ("sr_Latn", "Tavuš"), ("sv", "Tavusj"), ("ta", "த\u{bbe}வுஸ\u{bcd} பகுதி"), ("te", "ట\u{c3e}వుష\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ทาว\u{e38}ช"), ("tr", "Tavuş"), ("uk", "Тавуш"), ("ur", "تاووش صوبہ"), ("vi", "Tavush"), ("zh", "塔武什省")]),
                        unofficial_name_list: ["Tavoush"].to_vec(),
                    }
                ),
                (
                    "VD",
                    Subdivision{
                        name: "VD",
                        country_alpha2: Alpha2::AM,
                        code: "VD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.8107912), longitude: Some(45.4967174), max_latitude: Some(40.01380899999999), min_latitude: Some(39.502243), max_longitude: Some(45.825905), min_longitude: Some(45.066502)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة وايوتس\u{200c}جور"), ("az", "Dərələyəz mahalı"), ("be", "Марз Ваёц-Дзор"), ("bg", "Вайоц Дзор"), ("bn", "ভ\u{9be}য\u{9bc}োত জ\u{9c1}র অঞ\u{9cd}চল"), ("bs", "Vajots Dzor"), ("ca", "Vaiots Tzor"), ("ccp", "𑄞\u{11127}𑄠\u{1112e}𑄖\u{11134}𑄥\u{11134} 𑄘\u{1112e}𑄢\u{11134}"), ("ceb", "Vayots’ Dzori Marz"), ("cs", "Vajoc Dzor"), ("cy", "Vayots Dzor"), ("da", "Vayots Dzor Region"), ("de", "Wajoz Dsor"), ("el", "Βάγιοτς Ντζορ"), ("en", "Vayots Dzor"), ("es", "Vayots’ Dzor"), ("et", "Vajotsh Dzori maakond"), ("eu", "Vajots Dzor"), ("fa", "استان وایوتس\u{200c}جور"), ("fi", "Vajots’ Dzor"), ("fr", "Vayots Dzor"), ("gu", "વ\u{ac7}યોઝ ડીઝોર પ\u{acd}રદ\u{ac7}શ"), ("hi", "वायोत\u{94d}स द\u{94d}ज\u{93c}ोर"), ("hr", "Vajots Dzor"), ("hu", "Vajoc Dzor tartomány"), ("hy", "Վայոց ձորի մարզ"), ("id", "Provinsi Vayots Dzor"), ("it", "Vayots Dzor"), ("ja", "ヴァヨツ・ゾル地方"), ("ka", "ვაიოცძორის პროვინცია"), ("kn", "ವಾಯೋಟ\u{ccd}ಸ\u{ccd} ಡ\u{cbf}ಜೋರ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "바요츠조르 주"), ("lt", "Vajocdzoro sritis"), ("lv", "Vajotdzoras reģions"), ("mk", "Вајоц Ѕор"), ("mn", "Вайоцзор аймаг"), ("mr", "वायोट\u{94d}स डोजोर प\u{94d}रद\u{947}श"), ("ms", "Wilayah Vayots Dzor"), ("nb", "Vajots Dzor"), ("ne", "वायोत\u{94d}स द\u{94d}ज\u{93c}ोर प\u{94d}रान\u{94d}त"), ("nl", "Vayots Dzor"), ("no", "Vajots Dzor"), ("pa", "ਵਾਇਓਤ ਜ\u{a4b}ਰ ਸ\u{a42}ਬਾ"), ("pl", "Prowincja Wajoc Dzor"), ("pt", "Vayots Dzor"), ("ro", "Provincia Vayots Dzor"), ("ru", "Вайоцдзорская область"), ("si", "වයෝට\u{dca}ස\u{dca} ඩ\u{dd2}සොර\u{dca} කල\u{dcf}පය"), ("sr", "Вајоц Џор"), ("sr_Latn", "Vajoc Džor"), ("sv", "Vajots Dzor"), ("ta", "வயோட\u{bcd}ஸ\u{bcd} டஸ\u{bcd}வ\u{bcd}ர\u{bcd} பகுதி"), ("te", "వ\u{c3e}య\u{c4b}ట\u{c4d}స\u{c4d} డ\u{c3f}జ\u{c4b}ర\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "วายท ดซอส"), ("tr", "Vayots Dzor"), ("uk", "Вайоц-Дзор"), ("ur", "وایوتس جور صوبہ"), ("vi", "Vayots Dzor"), ("zh", "瓦約茨佐爾省")]),
                        unofficial_name_list: ["Vayoc Jor"].to_vec(),
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
#[cfg(feature = "am")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::AM,
        alpha3: Alpha3::ARM,
        address_format: None,
        continent: Continent::Asia,
        country_code: 374,
        currency_code: "AMD",
        gec: Some(GEC::AM),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("ARM"),
        iso_long_name: "The Republic of Armenia",
        iso_short_name: "Armenia",
        official_language_list: ["hy", "ru"].to_vec(),
        spoken_language_list: ["hy", "ru"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "8",
        nationality: Some("Armenian"),
        number: "051",
        postal_code: true,
        postal_code_format: Some("(?:37)?\\d{4}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::WesternAsia),
        un_locode: "AM",
        unofficial_name_list: ["Armenia", "Armenien", "Arménie", "アルメニア", "Armenië"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Armenia"),
            ("af", "Armenië"),
            ("ak", "Armenia"),
            ("am", "ጐሴሣኒ።"),
            ("an", "Armenia"),
            ("ar", "أرمينيا"),
            ("as", "আৰ\u{9cd}মেনিয়\u{9be}"),
            ("ay", "Armenia"),
            ("az", "Ermənistan"),
            ("ba", "Armenia"),
            ("be", "Арменія"),
            ("bg", "Армения"),
            ("bi", "Armenia"),
            ("bn", "আর\u{9cd}মেনিয়\u{9be}"),
            ("bn_IN", "আর\u{9cd}মেনিয়\u{9be}"),
            ("br", "Armenia"),
            ("bs", "Armenija"),
            ("ca", "Armènia"),
            ("ce", "Аремалойн"),
            ("ch", "Armenia"),
            ("cs", "Arménie"),
            ("cv", "Аремалойн"),
            ("cy", "Armenia"),
            ("da", "Armenien"),
            ("de", "Armenien"),
            ("dv", "އ\u{7a6}ރ\u{7aa}މ\u{7a9}ނ\u{7a8}އ\u{7a7}"),
            ("dz", "ཨར་མ\u{f72}་ན\u{f72}་ཡ།"),
            ("ee", "Armenia"),
            ("el", "Αρμενία"),
            ("en", "Armenia"),
            ("eo", "Armenio"),
            ("es", "Armenia"),
            ("et", "Armeenia"),
            ("eu", "Armenia"),
            ("fa", "ارمنستان"),
            ("ff", "Armaaniya"),
            ("fi", "Armenia"),
            ("fo", "Armenia"),
            ("fr", "Arménie"),
            ("fy", "Armeenje"),
            ("ga", "An Airméin"),
            ("gl", "Armenia"),
            ("gn", "Armenia"),
            ("gu", "આર\u{acd}મ\u{ac7}નિયા"),
            ("gv", "Yn Armeain"),
            ("ha", "Armeniya"),
            ("he", "ארמניה"),
            ("hi", "आर\u{94d}मीनिया"),
            ("hr", "Armenija"),
            ("ht", "Ameni"),
            ("hu", "Örményország"),
            ("hy", "Հայաստանի Հանրապետութիւն"),
            ("ia", "Armenia"),
            ("id", "Armenia"),
            ("io", "Armenia"),
            ("is", "Armenía"),
            ("it", "Armenia"),
            ("iu", "Armenia"),
            ("ja", "アルメニア"),
            ("ka", "სასომხეთი"),
            ("ki", "Armenia"),
            ("kk", "Армения"),
            ("kl", "Armenia"),
            ("km", "អាមេន\u{17b8}"),
            ("kn", "ಆರ\u{ccd}ಮೇನ\u{cbf}ಯಾ"),
            ("ko", "아르메니아"),
            ("ku", "Ermenistan"),
            ("kv", "Армения"),
            ("kw", "Armeni"),
            ("ky", "Армения"),
            ("lo", "ປະເທດອາກເມນ\u{eb5}"),
            ("lt", "Armėnija"),
            ("lv", "Armēnija"),
            ("mi", "Āmenia"),
            ("mk", "Ерменија"),
            ("ml", "അര\u{d4d}\u{200d}മീനിയ"),
            ("mn", "Армен"),
            ("mr", "अम\u{947}रिका"),
            ("ms", "Armenia"),
            ("mt", "Armenja"),
            (
                "my",
                "အာမေးန\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Arminiya"),
            ("nb", "Armenia"),
            ("ne", "अरम\u{947}निया"),
            ("nl", "Armenië"),
            ("nn", "Armenia"),
            ("nv", "Aooméénii Bikéyah"),
            ("oc", "Armenia"),
            ("or", "ଆର\u{b4d}ମେନ\u{b3f}ୟ\u{b3e}"),
            ("pa", "ਅਰਮੀਨਾ"),
            ("pi", "आर\u{94d}मीनिया"),
            ("pl", "Armenia"),
            ("ps", "ارمنستان"),
            ("pt", "Arménia"),
            ("pt_BR", "Armênia"),
            ("ro", "Armenia"),
            ("ru", "Армения"),
            ("rw", "Arumeniya"),
            ("sc", "Armènia"),
            ("sd", "آرمينيا"),
            ("si", "ආමේන\u{dd2}ය\u{dcf}"),
            ("sk", "Arménsko"),
            ("sl", "Armenija"),
            ("so", "Armeeniya"),
            ("sq", "Armeni"),
            ("sr", "Јерменија"),
            ("sv", "Armenien"),
            ("sw", "Armenia"),
            ("ta", "அர\u{bcd}மெனிய\u{bbe}"),
            ("te", "అమ\u{c47}ర\u{c3f}క\u{c3e}"),
            ("tg", "Арманистон"),
            ("th", "อาร\u{e4c}เมเน\u{e35}ย"),
            ("ti", "ኣርሜንያ"),
            ("tk", "Ermenistan"),
            ("tl", "Armenya"),
            ("tr", "Ermenistan"),
            ("tt", "Әрмәнстан"),
            ("ug", "ئەرمېنىيە"),
            ("uk", "Вірменія"),
            ("ur", "آرمینیا"),
            ("uz", "Armaniston"),
            ("ve", "Armenia"),
            ("vi", "Ac-mê-ni"),
            ("wa", "Årmeneye"),
            ("wo", "Armeeni"),
            ("xh", "Armenia"),
            ("yo", "Arméníà"),
            ("zh_CN", "亚美尼亚"),
            ("zh_HK", "亞美尼亞"),
            ("zh_TW", "亞美尼亞"),
            ("zu", "Armenia"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}