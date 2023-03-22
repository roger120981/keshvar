// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// New Zealand

#[cfg(all(feature = "nz", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{region}}\n{{city}} {{postalcode}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::NZ;
    pub const ALPHA3: Alpha3 = Alpha3::NZL;
    pub const CONTINENT: Continent = Continent::Australia;
    pub const COUNTRY_CODE: usize = 64;
    pub const CURRENCY_CODE: &str = "NZD";
    pub const GEC: Option<GEC> = Some(GEC::NZ);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("NZL");
    pub const ISO_SHORT_NAME: &str = "New Zealand";
    pub const ISO_LONG_NAME: &str = "New Zealand";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[1];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("New Zealander");
    pub const NUMBER: &str = "554";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Oceania);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::AustraliaAndNewZealand);
    pub const UN_LOCODE: &str = "NZ";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "New Zealand",
        "Neuseeland",
        "Nouvelle Zélande",
        "Nueva Zelanda",
        "ニュージーランド",
        "Nieuw-Zeeland",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "New Zealand"),
        ("af", "Nieu-Seeland"),
        ("ak", "New Zealand"),
        ("am", "ኒፄ ፑሒን፥"),
        ("an", "New Zealand"),
        ("ar", "نيوزيلاندا"),
        ("as", "নিউজিলেণ\u{9cd}ড"),
        ("ay", "New Zealand"),
        ("az", "Yeni Zellandiya"),
        ("ba", "New Zealand"),
        ("be", "Новая Зеландыя"),
        ("bg", "Нова Зеландия"),
        ("bi", "New Zealand"),
        ("bn", "নিউজিল\u{9cd}য\u{9be}ন\u{9cd}ড"),
        ("bn_IN", "নিউজিল\u{9cd}য\u{9be}ন\u{9cd}ড"),
        ("br", "Zeland nevez"),
        ("bs", "Novi Zeland"),
        ("ca", "Nova Zelanda"),
        ("ce", "Керла Зеланди"),
        ("ch", "New Zealand"),
        ("cs", "Nový Zéland"),
        ("cv", "Керла Зеланди"),
        ("cy", "Seland Newydd"),
        ("da", "New Zealand"),
        ("de", "Neuseeland"),
        ("dv", "ނ\u{7a8}އ\u{7aa}ޒ\u{7a8}ލ\u{7ad}ނ\u{7b0}ޑ\u{7aa}"),
        ("dz", "ན\u{f72}འ\u{f74}་ཛ\u{f72}་ལ\u{f7a}ནཌ\u{f72}།"),
        ("ee", "New Zealand"),
        ("el", "Νέα Ζηλανδία"),
        ("en", "New Zealand"),
        ("eo", "Nov-Zelando"),
        ("es", "Nueva Zelanda"),
        ("et", "Uus-Meremaa"),
        ("eu", "Zeelanda Berria"),
        ("fa", "نیوزیلند"),
        ("ff", "New Zealand"),
        ("fi", "Uusi-Seelanti"),
        ("fo", "Ný Sæland"),
        ("fr", "Nouvelle-Zélande"),
        ("fy", "Nij-Seelân"),
        ("ga", "An Nua-Shéalainn"),
        ("gl", "Nova Celandia"),
        ("gn", "New Zealand"),
        ("gu", "ન\u{acd}ય\u{ac1} ઝીલ\u{ac7}ન\u{acd}ડ"),
        ("gv", "Yn Teelynn Noa"),
        ("ha", "New Zealand"),
        ("he", "ניו זילנד"),
        ("hi", "न\u{94d}य\u{942}ज\u{93c}ील\u{948}ण\u{94d}ड"),
        ("hr", "Novi Zeland"),
        ("ht", "Nouvèl Zelann"),
        ("hu", "Új-Zéland"),
        ("hy", "Նոր Զելանդիա"),
        ("ia", "Nove Zelanda"),
        ("id", "Selandia Baru"),
        ("io", "Nova-Zelando"),
        ("is", "Nýja-Sjáland"),
        ("it", "Nuova Zelanda"),
        ("iu", "New Zealand"),
        ("ja", "ニュージーランド"),
        ("ka", "ახალი ზელანდია"),
        ("ki", "New Zealand"),
        ("kk", "Жаңа Зеландия"),
        ("kl", "New Zealand"),
        ("km", "ន\u{17bc}វែលហ\u{17d2}សេឡង\u{17cb}"),
        ("kn", "ನ\u{ccd}ಯ\u{cc2} ಜ\u{cbf}ಲ\u{ccd}ಯಂಡ\u{ccd}"),
        ("ko", "뉴질랜드"),
        ("ku", "Zelandaya Nû"),
        ("kv", "Выль Зеландия"),
        ("kw", "Mordir Nowydh"),
        ("ky", "Жаңы Зеландия"),
        ("lo", "ປະເທດນ\u{eb9}ແວນ ເຊລ\u{eb1}ງ"),
        ("lt", "Naujoji Zelandija"),
        ("lv", "Jaunzēlande"),
        ("mi", "Aotearoa"),
        ("mk", "Нов Зеланд"),
        ("ml", "ന\u{d4d}യ\u{d42}സില\u{d3e}ന\u{d4d}\u{200d}ഡ\u{d4d}"),
        ("mn", "Шинэ зеланд"),
        ("mr", "न\u{94d}य\u{942}झिल\u{902}ड"),
        ("ms", "New Zealand"),
        ("mt", "New Zealand"),
        (
            "my",
            "နယ\u{1030}းဇ\u{102e}လန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Niu Djiran"),
        ("nb", "New Zealand"),
        ("ne", "न\u{94d}य\u{941}जिल\u{94d}याण\u{94d}ड"),
        ("nl", "Nieuw-Zeeland"),
        ("nn", "New Zealand"),
        ("nv", "New Zealand"),
        ("oc", "Novèla Zelanda"),
        ("or", "ନ\u{b4d}ଯ\u{b41}ଜୀଲ\u{b4d}ଯ\u{b3e}ଣ\u{b4d}ଡ"),
        ("pa", "ਨਿਊਜ਼ੀਲ\u{a48}\u{a02}ਡ"),
        ("pi", "न\u{94d}य\u{942}-जील\u{948}\u{902}ड"),
        ("pl", "Nowa Zelandia"),
        ("ps", "نیوزیلنډ"),
        ("pt", "Nova Zelândia"),
        ("pt_BR", "Nova Zelândia"),
        ("ro", "Noua Zeelandă"),
        ("ru", "Новая Зеландия"),
        ("rw", "Nuveli Zelande"),
        ("sc", "Zelanda Noa"),
        ("sd", "New Zealand"),
        ("si", "නවස\u{dd3}ලන\u{dca}තය"),
        ("sk", "Nový Zéland"),
        ("sl", "Nova Zelandija"),
        ("so", "Neyuusilaand"),
        ("sq", "Zelandë e Re"),
        ("sr", "Нови Зеланд"),
        ("sv", "Nya Zeeland"),
        ("sw", "New Zealand"),
        ("ta", "நியூசில\u{bbe}ந\u{bcd}து"),
        ("te", "న\u{c4d}యూజ\u{c3f}ల\u{c3e}ండ\u{c4d}"),
        ("tg", "Зеландияи Нав"),
        ("th", "น\u{e34}วซ\u{e35}แลนด\u{e4c}"),
        ("ti", "ኒው ዚላንድ"),
        ("tk", "Täze Zelandiýa"),
        ("tl", "New Zealand"),
        ("tr", "Yeni Zelanda"),
        ("tt", "Яңа Зеаланд"),
        ("ug", "يېڭى زېلاندىيە"),
        ("uk", "Нова Зеландія"),
        ("ur", "نیوزی لینڈ"),
        ("uz", "Yangi Zelandiya"),
        ("ve", "New Zealand"),
        ("vi", "Niu Xi-lân"),
        ("wa", "Nouve Zelande"),
        ("wo", "Nuweel Selaand"),
        ("xh", "New Zealand (izealand entsha)"),
        ("yo", "New Zealand"),
        ("zh_CN", "新西兰"),
        ("zh_HK", "新西蘭"),
        ("zh_TW", "紐西蘭"),
        ("zu", "INyuzilandi"),
    ];
    #[cfg(all(feature = "nz", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -40.900557;
        pub const LONGITUDE: f64 = 174.885971;
        pub const MAX_LATITUDE: f64 = -28.8773225;
        pub const MAX_LONGITUDE: f64 = -175.1235077;
        pub const MIN_LATITUDE: f64 = -52.7224663;
        pub const MIN_LONGITUDE: f64 = 165.7437641;
        pub const NORTHEAST_LATITUDE: f64 = -28.8773225;
        pub const NORTHEAST_LONGITUDE: f64 = -175.1235077;
        pub const SOUTHWEST_LATITUDE: f64 = -52.7224663;
        pub const SOUTHWEST_LONGITUDE: f64 = 165.7437641;
    }
}
#[cfg(all(feature = "nz", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -40.900557,
            longitude: 174.885971,
            max_latitude: -28.8773225,
            max_longitude: -175.1235077,
            min_latitude: -52.7224663,
            min_longitude: 165.7437641,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: -28.8773225,
                    longitude: -175.1235077,
                },
                southwest: CountryGeoBound {
                    latitude: -52.7224663,
                    longitude: 165.7437641,
                },
            },
        }
    }
}

#[cfg(all(feature = "nz", feature = "subdivisions"))]
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
                    "AUK",
                    Subdivision{
                        name: "AUK",
                        country_alpha2: Alpha2::NZ,
                        code: "AUK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-36.8484597), longitude: Some(174.7633315), max_latitude: Some(-36.660571), min_latitude: Some(-37.0654751), max_longitude: Some(175.2871371), min_longitude: Some(174.4438016)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم أوكلاند"), ("be", "Окленд"), ("bg", "Окланд"), ("bn", "অকল\u{9cd}য\u{9be}ন\u{9cd}ড অঞ\u{9cd}চল"), ("ca", "Regió d’Auckland"), ("ccp", "𑄃\u{11127}𑄇\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Auckland"), ("da", "Auckland Region"), ("de", "Region Auckland"), ("el", "Περιοχή του Ώκλαντ"), ("en", "Auckland"), ("es", "Región de Auckland"), ("eu", "Auckland eskualdea"), ("fi", "Aucklandin hallintoalue"), ("fr", "Auckland"), ("gl", "Rexión de Auckland"), ("gu", "ઓકલ\u{ac7}ન\u{acd}ડ પ\u{acd}રદ\u{ac7}શ"), ("he", "אוקלנד"), ("hi", "ऑकल\u{948}\u{902}ड क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Auckland"), ("hu", "Auckland régió"), ("hy", "Օքլենդ"), ("id", "Region Auckland"), ("it", "Auckland"), ("ja", "オークランド地方"), ("ka", "ოკლენდის რეგიონი"), ("kn", "ಆಕ\u{ccd}ಲ\u{cc6}ಂಡ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "오클랜드 지방"), ("lt", "Oklendo regionas"), ("lv", "Oklendas reģions"), ("mk", "Окленд"), ("mr", "ऑकल\u{902}ड प\u{94d}रद\u{947}श"), ("ms", "Wilayah Auckland"), ("nb", "Auckland"), ("nl", "Auckland"), ("no", "Auckland"), ("pl", "Auckland"), ("pt", "Auckland"), ("ru", "Окленд"), ("si", "ඕක\u{dca}ලන\u{dca}ඩ\u{dca} කල\u{dcf}පය"), ("sv", "Auckland"), ("ta", "ஆக\u{bcd}கலன\u{bcd}ட\u{bcd} பகுதி"), ("te", "అక\u{c4d}ల\u{c3e}ండ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ออคแลนด\u{e4c}"), ("tr", "Auckland Bölgesi"), ("uk", "Окленд"), ("ur", "آکلینڈ علاقہ"), ("vi", "Vùng Auckland"), ("zh", "奥克兰大区")]),
                        unofficial_name_list: ["Auckland"].to_vec(),
                    }
                ),
                (
                    "BOP",
                    Subdivision{
                        name: "BOP",
                        country_alpha2: Alpha2::NZ,
                        code: "BOP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-37.6825027), longitude: Some(176.1880232), max_latitude: Some(-37.2644254), min_latitude: Some(-38.9363223), max_longitude: Some(178.1075597), min_longitude: Some(175.8517874)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم خليج بانتي"), ("bg", "Бей ъф Пленти"), ("bn", "প\u{9cd}লেন\u{9cd}টি উপস\u{9be}গর অঞ\u{9cd}চল"), ("ca", "Bay of Plenty"), ("ccp", "𑄝𑄬 𑄃\u{11127}𑄛\u{11134} 𑄛\u{11133}𑄣𑄬𑄚\u{11134}𑄑\u{11128}"), ("cs", "Bay of Plenty"), ("da", "Bay of Plenty Region"), ("de", "Bay of Plenty"), ("el", "Μπέι οφ Πλέντι"), ("en", "Bay of Plenty"), ("es", "Bay of Plenty"), ("eu", "Bay of Plenty eskualdea"), ("fi", "Bay of Plenty"), ("fr", "baie de l’Abondance"), ("gu", "બ\u{ac7} ઓફ પ\u{acd}લ\u{ac7}ન\u{acd}ટી પ\u{acd}રદ\u{ac7}શ"), ("he", "ביי אוף פלנטי"), ("hi", "ब\u{947} ऑफ प\u{94d}ल\u{947}\u{902}टी क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Bay of Plenty"), ("hy", "Բեյ օֆ Փլենթի"), ("id", "Wilayah Bay of Plenty"), ("it", "Bay of Plenty"), ("ja", "ベイ・オブ・プレンティ地方"), ("ka", "ბეი-ოვ-პლენტის რეგიონი"), ("kn", "ಬೇ ಆಫ\u{ccd} ಪ\u{ccd}ಲ\u{cc6}ಂಟ\u{cbf} ಪ\u{ccd}ರದೇಶ"), ("ko", "베이오브플렌티 지방"), ("lt", "Plenčio įlankos regionas"), ("lv", "Bejofplenti reģions"), ("mk", "Залив Пленти"), ("mr", "ब\u{947} ऑफ प\u{94d}ल\u{947}नटी प\u{94d}रद\u{947}श"), ("ms", "Wilayah Bay of Plenty"), ("nb", "Bay of Plenty"), ("nl", "Bay of Plenty"), ("no", "Bay of Plenty"), ("pl", "Bay of Plenty"), ("pt", "Região da Baia de Plenty"), ("ru", "Бей-оф-Пленти"), ("si", "ප\u{dca}ලේන\u{dca}ට\u{dd2} කලප\u{dd4} කල\u{dcf}පය"), ("sv", "Bay of Plenty"), ("ta", "ப\u{bcd}ளெண\u{bcd}ட\u{bcd}டி பகுதி பே"), ("te", "బ\u{c47} ఆఫ\u{c4d} ప\u{c4d}ల\u{c46}ంట\u{c40} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เบย\u{e4c} ออฟ เพลนต\u{e35}\u{e49}"), ("tr", "Bay of Plenty Bölgesi"), ("uk", "Бей оф Пленті"), ("ur", "بے آف پلینٹی علاقہ"), ("vi", "Khu vực Bay Of Plenty"), ("zh", "普伦蒂湾大区")]),
                        unofficial_name_list: ["Bay of Plenty"].to_vec(),
                    }
                ),
                (
                    "CAN",
                    Subdivision{
                        name: "CAN",
                        country_alpha2: Alpha2::NZ,
                        code: "CAN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-43.7542275), longitude: Some(171.1637245), max_latitude: Some(-41.9073951), min_latitude: Some(-44.9402681), max_longitude: Some(174.0956747), min_longitude: Some(169.8520438)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتربيري - نيوزلندا"), ("bg", "Кентърбъри"), ("bn", "ক\u{9cd}য\u{9be}ন\u{9cd}ট\u{9be}বেরি অঞ\u{9cd}চল"), ("ca", "Canterbury"), ("ccp", "𑄇𑄚\u{11134}𑄑𑄢\u{11134}𑄝\u{1112a}𑄢\u{11128}"), ("cs", "Canterbury"), ("da", "Canterbury"), ("de", "Canterbury"), ("el", "Καντέρμπερι"), ("en", "Canterbury"), ("es", "Canterbury"), ("et", "Canterbury ringkond"), ("eu", "Canterbury eskualdea"), ("fa", "ناحیه کانتربوری"), ("fi", "Canterbury"), ("fr", "Canterbury"), ("gu", "ક\u{ac7}ન\u{acd}ટરબરી પ\u{acd}રદ\u{ac7}શ"), ("he", "קנטרברי"), ("hi", "क\u{948}\u{902}टरबरी, न\u{94d}य\u{942}जील\u{948}\u{902}ड"), ("hu", "Canterbury régió"), ("id", "Canterbury"), ("is", "Canterbury"), ("it", "Canterbury"), ("ja", "カンタベリー地方"), ("ka", "კენტერბერის რეგიონი"), ("kn", "ಕ\u{ccd}ಯಾಂಟರ\u{ccd}ಬರ\u{cbf} ಪ\u{ccd}ರದೇಶ"), ("ko", "캔터베리 지방"), ("lt", "Kanterburio regionas"), ("lv", "Kenterberijas reģions"), ("mk", "Кантербери"), ("mr", "क\u{901}टरबरी प\u{94d}रद\u{947}श"), ("ms", "Wilayah Canterbury"), ("nb", "Canterbury"), ("nl", "Canterbury"), ("no", "Canterbury"), ("pl", "Canterbury"), ("pt", "Canterbury"), ("ru", "Кентербери"), ("si", "කැන\u{dca}ටබර\u{dd2} කල\u{dcf}පය"), ("sv", "Canterbury, Nya Zeeland"), ("ta", "க\u{bbe}ன\u{bcd}டெர\u{bcd}புரி ர\u{bc0}ஜியன\u{bcd}"), ("te", "స\u{c46}ంట\u{c4d}రర\u{c4d}బర\u{c40} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แคนเทอเบอร\u{e35}\u{e48}"), ("tr", "Canterbury Region"), ("uk", "Кантербері"), ("ur", "کینٹربری، نیوزی لینڈ"), ("vi", "Canterbury"), ("zh", "坎特伯雷")]),
                        unofficial_name_list: ["Canterbury"].to_vec(),
                    }
                ),
                (
                    "CIT",
                    Subdivision{
                        name: "CIT",
                        country_alpha2: Alpha2::NZ,
                        code: "CIT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-43.9120964), longitude: Some(-176.5433025), max_latitude: Some(-43.5631344), min_latitude: Some(-44.4343162), max_longitude: Some(-175.8314073), min_longitude: Some(-176.8940878)}),
                        comments: None,
                        subdivision_type: SubdivisionType::SpecialIslandAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جزر تشاتام"), ("az", "Çatem"), ("be", "Астравы Чатэм"), ("bg", "Чатъм"), ("bn", "চ\u{9cd}য\u{9be}থ\u{9be}ম আইল\u{9cd}য\u{9be}ন\u{9cd}ডস"), ("ca", "Illes Chatham"), ("ccp", "𑄇𑄗𑄟\u{11134} 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄥\u{11134}"), ("ceb", "Chatham Islands (rehiyon)"), ("cs", "Chathamské ostrovy"), ("da", "Chatham Øerne"), ("de", "Chathaminseln"), ("el", "Νησιά Τσάταμ"), ("en", "Chatham Islands"), ("es", "Islas Chatham"), ("et", "Chathami saared"), ("eu", "Chatham Islands"), ("fa", "جزایر چاتام"), ("fi", "Chathamsaaret"), ("fr", "Îles Chatham"), ("gl", "Illas Chatham"), ("gu", "ચ\u{ac5}થમ આઇલ\u{ac7}ન\u{acd}ડ\u{acd}સ"), ("he", "איי צ׳טהאם"), ("hi", "चाथम द\u{94d}वीपसम\u{942}ह"), ("hr", "Chathamski otoci"), ("hu", "Chatham-szigetek"), ("hy", "Չաթեմ կղզիներ"), ("id", "Kepulauan Chatham"), ("it", "Isole Chatham"), ("ja", "チャタム諸島"), ("ka", "ჩატემის კუნძულები"), ("kn", "ಚಾಥಮ\u{ccd} ದ\u{ccd}ವೀಪಗಳು"), ("ko", "채텀 제도"), ("lt", "Čatamo salos"), ("lv", "Četema salas"), ("mr", "च\u{945}थम आईसल\u{901}डस"), ("ms", "Kepulauan Chatham"), ("nb", "Chathamøyene"), ("nl", "Chathameilanden"), ("no", "Chathamøyene"), ("pl", "Wyspy Chatham"), ("pt", "Ilhas Chatham"), ("ru", "Чатем"), ("si", "චැතැම\u{dca} ද\u{dd6}පත\u{dca}"), ("sr", "Четем острва"), ("sr_Latn", "Četem ostrva"), ("sv", "Chathamöarna"), ("ta", "சத\u{bcd}த\u{bbe}ம\u{bcd} த\u{bc0}வு"), ("te", "చ\u{c3e}తమ\u{c4d} ద\u{c40}వులు"), ("th", "หม\u{e39}\u{e48}เกาะแชท\u{e31}ม"), ("tr", "Chatham Adaları"), ("uk", "Чатем"), ("ur", "چاتھم آئی لینڈ"), ("vi", "Quần đảo Chatham"), ("zh", "查塔姆群岛")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "GIS",
                    Subdivision{
                        name: "GIS",
                        country_alpha2: Alpha2::NZ,
                        code: "GIS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-38.662334), longitude: Some(178.017649), max_latitude: Some(-38.6219639), min_latitude: Some(-38.710757), max_longitude: Some(178.115354), min_longitude: Some(177.9373069)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم جيسبورن"), ("bg", "Гисбърн"), ("bn", "গিসবন অঞ\u{9cd}চল"), ("ca", "Regió de Gisborne"), ("ccp", "𑄉\u{11128}𑄌\u{11134}𑄝\u{11127}𑄢\u{11134}𑄚\u{11134}"), ("ceb", "Gisborne"), ("da", "Gisborne region"), ("de", "Gisborne Region"), ("el", "Γκίσμπερν"), ("en", "Gisborne"), ("es", "Gisborne"), ("fi", "Gisbornen maakunta"), ("fr", "Gisborne"), ("gu", "ગિસબોર\u{acd}ન પ\u{acd}રદ\u{ac7}શ"), ("hi", "जिस\u{94d}बॉर\u{94d}न क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Gisborne"), ("hy", "Գիսբորն"), ("id", "Wilayah Gisborne"), ("it", "Gisborne"), ("ja", "ギズボーン地方"), ("ka", "გიზბორნის რეგიონი"), ("kn", "ಗ\u{cbf}ಸ\u{ccd}ಬೋರ\u{ccd}ನ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "기즈번 지방"), ("lt", "Gisborno apskritis"), ("lv", "Gisbornas reģions"), ("mk", "Гизборн"), ("mr", "जिस\u{94d}बर\u{94d}न प\u{94d}रद\u{947}श"), ("ms", "Wilayah Gisborne"), ("nb", "Gisborne"), ("nl", "Gisborne"), ("no", "Gisborne"), ("pl", "Gisborne"), ("pt", "Gisborne"), ("ru", "Гисборн"), ("si", "ග\u{dd2}ස\u{dca}බෝර\u{dca}නේ කල\u{dcf}පය"), ("sv", "Gisborne"), ("ta", "கிசுபர\u{bcd}ன\u{bcd} பகுதி"), ("te", "గ\u{c3f}స\u{c4d}బ\u{c3e}ర\u{c4d}న\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เนเรทา"), ("tr", "Gisborne Bölgesi"), ("uk", "Гісборн"), ("ur", "ضلع گسبورن"), ("vi", "Khu vực Gisborne"), ("zh", "吉斯伯恩大区")]),
                        unofficial_name_list: ["Gisborne"].to_vec(),
                    }
                ),
                (
                    "HKB",
                    Subdivision{
                        name: "HKB",
                        country_alpha2: Alpha2::NZ,
                        code: "HKB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-39.7711616), longitude: Some(176.7416374), max_latitude: Some(-38.1752186), min_latitude: Some(-40.440783), max_longitude: Some(178.002017), min_longitude: Some(175.8300119)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم هاوكس باي"), ("bg", "Хоукс Бей"), ("bn", "হ\u{9be}কেস বে অঞ\u{9cd}চল"), ("ca", "Hawke’s Bay"), ("ccp", "𑄦\u{11127}𑄇𑄬𑄌\u{11134} 𑄝𑄬"), ("ceb", "Hawke’s Bay"), ("cs", "Hawke’s Bay"), ("da", "Hawke’s Bay Region"), ("de", "Hawke’s Bay"), ("en", "Hawke’s Bay"), ("es", "Hawke’s Bay"), ("eu", "Hawke’s Bay eskualdea"), ("fi", "Hawke’s Bayn maakunta"), ("fr", "Hawke’s Bay"), ("gu", "હૉક\u{acd}સ બ\u{ac7} પ\u{acd}રા\u{a82}ત"), ("he", "הוקס ביי"), ("hi", "हॉक\u{94d}स ब\u{947} क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Hawke’s Bay"), ("id", "Wilayah Hawke’s Bay"), ("it", "Hawke’s Bay"), ("ja", "ホークス・ベイ地方"), ("ka", "ჰოკს-ბეის რეგიონი"), ("kn", "ಹಾಕ\u{ccd}ಸ\u{ccd} ಬೇ ಪ\u{ccd}ರದೇಶ"), ("ko", "호크스베이 지방"), ("lt", "Houkis Bėjaus regionas"), ("lv", "Hoksbejas reģions"), ("mk", "Хоков Залив"), ("mr", "हॉकस ब\u{947} प\u{94d}रद\u{947}श"), ("ms", "Wilayah Hawke’s Bay"), ("nb", "Hawke’s Bay"), ("nl", "Hawke’s Bay"), ("no", "Hawke’s Bay"), ("pl", "Hawke’s Bay"), ("pt", "Hawke’s Bay"), ("ru", "Хокс-Бей"), ("si", "හොකෙස\u{dca} කලප\u{dd4} කල\u{dcf}පය"), ("sv", "Hawke’s Bay"), ("ta", "ஹவ\u{bcd}க\u{bcd}கி ‘ஸ\u{bcd} பே பகுதி"), ("te", "హ\u{c3e}క\u{c4d}స\u{c4d} బ\u{c47} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ฮอว\u{e4c}ค เบย\u{e4c}"), ("tr", "Hawke Limano Bölgesi"), ("uk", "Хоукіс Бей"), ("ur", "ہاکس بے علاقہ"), ("vi", "Khu vực Hawke’s Bay"), ("zh", "霍克湾大区")]),
                        unofficial_name_list: ["Hawke's Bay"].to_vec(),
                    }
                ),
                (
                    "MBH",
                    Subdivision{
                        name: "MBH",
                        country_alpha2: Alpha2::NZ,
                        code: "MBH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-41.57269), longitude: Some(173.4216613), max_latitude: Some(-40.66335100000001), min_latitude: Some(-42.3366886), max_longitude: Some(174.3922804), min_longitude: Some(172.7185518)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم مارلبورو"), ("bg", "Марлборо"), ("bn", "ম\u{9be}র\u{9cd}লবোরো অঞ\u{9cd}চল"), ("ca", "Marlborough"), ("ccp", "𑄟𑄢\u{11134}𑄣\u{11134}"), ("ceb", "Marlborough"), ("cy", "Marlborough"), ("da", "Marlborough Region"), ("de", "Marlborough"), ("en", "Marlborough"), ("es", "Marlborough"), ("fi", "Marlborough"), ("fr", "Marlborough"), ("gu", "માર\u{acd}લબોરો પ\u{acd}રદ\u{ac7}શ"), ("hi", "मार\u{94d}लबोरो क\u{94d}ष\u{947}त\u{94d}र"), ("id", "Wilayah Marlborough"), ("it", "Marlborough"), ("ja", "マールボロ地方"), ("ka", "მარლბოროს რეგიონი"), ("kn", "ಮಾರ\u{ccd}ಲ\u{ccd}ಬರೋ ಪ\u{ccd}ರದೇಶ"), ("ko", "말버러 지방"), ("lt", "Mariburo regionas"), ("lv", "Mārlboro reģions"), ("mk", "Марлборо"), ("mr", "मार\u{94d}लबोरो प\u{94d}रद\u{947}श"), ("ms", "Wilayah Marlborough"), ("nb", "Marlborough"), ("nl", "Marlborough"), ("no", "Marlborough"), ("pl", "Marlborough"), ("pt", "Região de Marborough"), ("ru", "Марлборо"), ("si", "ම\u{dcf}ල\u{dca}බරෝ කල\u{dcf}පය"), ("sv", "Marlborough, Nya Zeeland"), ("ta", "ம\u{bbe}ர\u{bcd}ல\u{bcd}போர\u{bbe}கஹ\u{bcd} பகுதி"), ("te", "మ\u{c3e}ల\u{c4d}\u{200c}బ\u{c4a}ర\u{c4b} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "มาโบเราท\u{e4c}"), ("tr", "Marlborough Bölgesi"), ("uk", "Мальбороу"), ("ur", "مارلبورو علاقہ"), ("vi", "Khu vực Marlborough"), ("zh", "马尔堡")]),
                        unofficial_name_list: ["Marlborough"].to_vec(),
                    }
                ),
                (
                    "MWT",
                    Subdivision{
                        name: "MWT",
                        country_alpha2: Alpha2::NZ,
                        code: "MWT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-39.7273356), longitude: Some(175.4375574), max_latitude: Some(-38.4877777), min_latitude: Some(-40.7824838), max_longitude: Some(176.632784), min_longitude: Some(174.7620864)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ماناواتو - وانجانوي"), ("bg", "Манавату-Уонгануи"), ("bn", "ম\u{9be}ন\u{9be}ওয\u{9bc}\u{9be}ত\u{9c1} ওয\u{9bc}\u{9be}ংগ\u{9be}ন\u{9be}ই অঞ\u{9cd}চল"), ("ca", "Manawatu-Wanganui region"), ("ccp", "𑄟𑄚𑄤𑄑\u{1112a}-𑄤𑄋\u{11134}𑄉𑄚\u{1112d}\u{1112a}"), ("ceb", "Manawatu-Wanganui"), ("da", "Manawatu-Wanganui Region"), ("de", "Manawatu-Wanganui"), ("el", "Μαναουάτου-Γουανγκανούι"), ("en", "Manawatu-Wanganui"), ("es", "Manawatu-Wanganui"), ("eu", "Manawatu-Wanganui eskualdea"), ("fi", "Manawatu-Wanganuin"), ("fr", "Manawatu-Wanganui"), ("gu", "માનાવાત\u{ac1}-વા\u{a82}ગન\u{ac1}ઇ પ\u{acd}રદ\u{ac7}શ"), ("he", "מנוואטו-ונגנאוי"), ("hi", "मनावात\u{941}-वा\u{902}गन\u{941}ई प\u{94d}रद\u{947}श"), ("hr", "Manawatu-Wanganui"), ("id", "Wilayah Manawatu-Wanganui"), ("it", "Manawatu-Wanganui"), ("ja", "マナワツ・ワンガヌイ地方"), ("ka", "მანავატუ-უანგანუის რეგიონი"), ("kn", "ಮನಾವುಟು-ವಂಗನ\u{ccd}ಯು ಪ\u{ccd}ರದೇಶ"), ("ko", "마너와투왕거누이 지방"), ("lt", "Manavatu-Vanganujis"), ("lv", "Manavatu-Vanganuji"), ("mk", "Манавату-Вангануи"), ("mr", "म\u{945}नवाट\u{942}-वा\u{902}गन\u{941}इ प\u{94d}रद\u{947}श"), ("ms", "Wilayah Manawatu-Wanganui"), ("nb", "Manawatu-Wanganui"), ("nl", "Manawatu-Wanganui"), ("no", "Manawatu-Wanganui"), ("pl", "Manawatu-Wanganui"), ("pt", "Manawatu-Wanganui"), ("ru", "Манавату-Уангануи"), ("si", "මනවට\u{dd4} වන\u{dca}ගන\u{dd4}ය\u{dd2} කල\u{dcf}පය"), ("sv", "Manawatu-Wanganui"), ("ta", "மனவ\u{bbe}டு-வ\u{bbe}ங\u{bcd}கணுய\u{bcd} பகுதி"), ("te", "మన\u{c3e}వటు-వ\u{c3e}ంగనూయ\u{c3f} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตมานาวาท\u{e39}แวนกาน\u{e39}"), ("tr", "Manawatu-Wanguni Bölgesi"), ("uk", "Манавату-Вангануї"), ("ur", "ماناواتو-وانگانوی"), ("vi", "Khu vực Manawatu-Wanganui"), ("zh", "马纳瓦图－旺加努伊")]),
                        unofficial_name_list: ["Wanganui-Manawatu"].to_vec(),
                    }
                ),
                (
                    "NSN",
                    Subdivision{
                        name: "NSN",
                        country_alpha2: Alpha2::NZ,
                        code: "NSN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-41.2706319), longitude: Some(173.2839653), max_latitude: Some(-41.22283849999999), min_latitude: Some(-41.347096), max_longitude: Some(173.342576), min_longitude: Some(173.189612)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Нельсан"), ("ccp", "𑄚𑄬𑄣\u{11134}𑄥\u{11127}𑄚\u{11134}"), ("ceb", "Nelson"), ("de", "Nelson"), ("en", "Nelson"), ("fr", "Nelson"), ("hi", "न\u{947}ल\u{94d}सन क\u{94d}ष\u{947}त\u{94d}र"), ("ja", "ネルソン地方"), ("kn", "ನ\u{cc6}ಲ\u{ccd}ಸನ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "넬슨 지방"), ("mk", "Нелсон"), ("nl", "Nelson"), ("pl", "Nelson"), ("ru", "Нельсон"), ("sv", "Nelson")]),
                        unofficial_name_list: ["Nelson"].to_vec(),
                    }
                ),
                (
                    "NTL",
                    Subdivision{
                        name: "NTL",
                        country_alpha2: Alpha2::NZ,
                        code: "NTL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-35.5795461), longitude: Some(173.7624053), max_latitude: Some(-34.1295578), min_latitude: Some(-36.396875), max_longitude: Some(174.7832097), min_longitude: Some(172.0297516)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Northland"), ("ar", "مقاطعة نورثلاند"), ("bg", "Нортланд"), ("bn", "নর\u{9cd}থল\u{9cd}য\u{9be}ন\u{9cd}ড অঞ\u{9cd}চল"), ("ca", "Northland"), ("ccp", "𑄚\u{11127}𑄢\u{11134}𑄒\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Northland"), ("da", "Northland Region"), ("de", "Northland"), ("en", "Northland"), ("es", "Northland"), ("eu", "Northland eskualdea"), ("fa", "منطقه نورثلند"), ("fi", "Northland"), ("fr", "Northland"), ("gu", "નોર\u{acd}થલ\u{ac7}\u{a82}ડ પ\u{acd}રદ\u{ac7}શ"), ("he", "נורת׳לנד"), ("hi", "नॉर\u{94d}थल\u{948}\u{902}ड प\u{94d}रद\u{947}श"), ("hr", "Northland"), ("hu", "Northland"), ("hy", "Նորթլենդ"), ("id", "Region Northland"), ("it", "Northland"), ("ja", "ノースランド地方"), ("ka", "ნორთლენდის რეგიონი"), ("kn", "ಉತ\u{ccd}ತರ ಪ\u{ccd}ರದೇಶ"), ("ko", "노스랜드 지방"), ("lt", "Nortlendo regionas"), ("lv", "Nortlendas reģions"), ("mk", "Нортленд"), ("mr", "नॉर\u{94d}थल\u{901}ड प\u{94d}रद\u{947}श"), ("ms", "Wilayah Northland"), ("nb", "Northland"), ("nl", "Northland"), ("no", "Northland"), ("pl", "Northland"), ("pt", "Northland"), ("ro", "Northland"), ("ru", "Нортленд"), ("si", "නොර\u{dca}ට\u{dca}ලන\u{dca}ඩ\u{dca} කල\u{dcf}පය"), ("sv", "Northland"), ("ta", "வடக\u{bcd}கு லன\u{bcd}ட\u{bcd} பகுதி"), ("te", "న\u{c3e}ర\u{c4d}త\u{c4d} ల\u{c3e}ండ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "จ\u{e31}งหว\u{e31}ดนอร\u{e4c}ธเลนด\u{e4c}"), ("tr", "Northland Bölgesi"), ("uk", "Нортленд"), ("ur", "نارتھ لینڈ علاقہ"), ("vi", "Northland"), ("zh", "北地大区")]),
                        unofficial_name_list: ["Northland"].to_vec(),
                    }
                ),
                (
                    "OTA",
                    Subdivision{
                        name: "OTA",
                        country_alpha2: Alpha2::NZ,
                        code: "OTA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-44.8280041), longitude: Some(169.6345253), max_latitude: Some(-43.7888995), min_latitude: Some(-46.638692), max_longitude: Some(171.1554182), min_longitude: Some(168.1163886)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم أوتاغو"), ("bg", "Отаго"), ("bn", "ওট\u{9be}গো অঞ\u{9cd}চল"), ("ca", "Otago"), ("ccp", "𑄃\u{11127}𑄑𑄉\u{1112e}"), ("ceb", "Otago"), ("cs", "Otago"), ("cy", "Otago"), ("da", "Otago Region"), ("de", "Otago"), ("en", "Otago"), ("es", "Otago"), ("eu", "Otago eskualdea"), ("fa", "اوتاگو"), ("fi", "Otago"), ("fr", "Otago"), ("gu", "ઑટાગો પ\u{acd}રદ\u{ac7}શ"), ("he", "אוטגו"), ("hi", "ओटागो क\u{94d}ष\u{947}त\u{94d}र"), ("hu", "Otago régió"), ("id", "Wilayah Otago"), ("it", "Otago"), ("ja", "オタゴ地方"), ("ka", "ოტაგოს რეგიონი"), ("kn", "ಒಟಾಗೋ ಪ\u{ccd}ರದೇಶ"), ("ko", "오타고 지방"), ("lt", "Otagas"), ("lv", "Otago reģions"), ("mk", "Отаго"), ("mr", "ओट\u{945}गो प\u{94d}रद\u{947}श"), ("ms", "Wilayah Otago"), ("nb", "Otago"), ("nl", "Otago"), ("no", "Otago"), ("pl", "Otago"), ("pt", "Otago"), ("ro", "Otago"), ("ru", "Отаго"), ("si", "ඔට\u{dcf}ගෝ කල\u{dcf}පය"), ("sv", "Otago"), ("ta", "ஒட\u{bbe}கோ"), ("te", "ఓట\u{c3e}గ\u{c4b} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตโอทาโก"), ("tr", "Otago Bölgesi"), ("uk", "Отаго"), ("ur", "اوٹاگو"), ("vi", "Khu vực Otago"), ("zh", "奥塔哥大区")]),
                        unofficial_name_list: ["Otago"].to_vec(),
                    }
                ),
                (
                    "STL",
                    Subdivision{
                        name: "STL",
                        country_alpha2: Alpha2::NZ,
                        code: "STL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-45.84891589999999), longitude: Some(167.6755387), max_latitude: Some(-44.290561), min_latitude: Some(-47.2899505), max_longitude: Some(169.4939393), min_longitude: Some(166.426128)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم سوثلاند"), ("bg", "Саутланд"), ("bn", "স\u{9be}উথল\u{9cd}য\u{9be}ন\u{9cd}ড অঞ\u{9cd}চল"), ("ca", "Southland"), ("ccp", "𑄥𑄅\u{1112a}𑄖\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Southland"), ("da", "Southland Region"), ("de", "Southland (Region)"), ("el", "Σάουθλαντ"), ("en", "Southland"), ("es", "Southland"), ("eu", "Southland eskualdea"), ("fa", "سرزمین جنوبی، نیوزیلند"), ("fi", "Southland"), ("fr", "Southland"), ("gu", "દક\u{acd}ષિણલ\u{ac7}ન\u{acd}ડ પ\u{acd}રદ\u{ac7}શ"), ("he", "סאות׳לנד"), ("hi", "साउथल\u{948}\u{902}ड क\u{94d}ष\u{947}त\u{94d}र"), ("id", "Wilayah Southland"), ("it", "Southland"), ("ja", "サウスランド地方"), ("ka", "საუთლანდის რეგიონი"), ("kn", "ಸ\u{ccc}ತ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "사우스랜드 지방"), ("lt", "Soutlando regionas"), ("lv", "Sautlendas reģions"), ("mk", "Саутленд"), ("mr", "साउथल\u{901}ड प\u{94d}रद\u{947}श"), ("ms", "Wilayah Southland"), ("nb", "Southland"), ("nl", "Southland"), ("no", "Southland"), ("pl", "Southland"), ("pt", "Southland"), ("ru", "Саутленд"), ("si", "සව\u{dd4}ත\u{dca}ලන\u{dca}ඩ\u{dca} කල\u{dcf}පය"), ("sv", "Southland"), ("ta", "தெற\u{bcd}கு ல\u{bbe}ந\u{bcd}து பகுதி"), ("te", "స\u{c4c}త\u{c4d}\u{200c}ల\u{c3e}ండ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เซาท\u{e4c}แลนด\u{e4c}"), ("tr", "Southland Bölgesi"), ("uk", "Саутленд"), ("ur", "ساؤتھ لینڈ، نیوزی لینڈ"), ("vi", "Khu vực Southland"), ("zh", "南地大区")]),
                        unofficial_name_list: ["Southland"].to_vec(),
                    }
                ),
                (
                    "TAS",
                    Subdivision{
                        name: "TAS",
                        country_alpha2: Alpha2::NZ,
                        code: "TAS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-41.2122123), longitude: Some(172.7347142), max_latitude: Some(-40.4980299), min_latitude: Some(-42.3055977), max_longitude: Some(173.2318855), min_longitude: Some(172.093419)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تاسمان"), ("bg", "Тасман"), ("bn", "ত\u{9be}সম\u{9be}ন জেল\u{9be}"), ("ca", "Tasman"), ("ccp", "𑄑𑄌\u{11134}𑄟\u{11133}𑄠𑄚\u{11134}"), ("ceb", "Tasman District"), ("da", "Tasman District"), ("de", "Tasman"), ("el", "Τασμάν"), ("en", "Tasman"), ("es", "Tasman"), ("eu", "Tasman eskualdea"), ("fi", "Tasman"), ("fr", "Tasman"), ("gu", "તાસ\u{acd}માન જિલ\u{acd}લો"), ("hi", "तस\u{94d}मान जिला"), ("id", "Distrik Tasman"), ("it", "Tasman"), ("ja", "タスマン地方"), ("ka", "ტასმანის რეგიონი"), ("kn", "ತಾಸ\u{ccd}ಮನ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "태즈먼 지방"), ("lt", "Tasmano apskritis"), ("lv", "Tasmanas reģions"), ("mk", "Тасман"), ("mr", "तस\u{94d}मान जिल\u{94d}हा"), ("ms", "Wilayah Tasman"), ("nb", "Tasman"), ("nl", "Tasman"), ("no", "Tasman"), ("pl", "Tasman"), ("pt", "Tasman"), ("ru", "Тасман"), ("si", "ටස\u{dca}මන\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Tasman"), ("ta", "ட\u{bbe}ஸ\u{bcd}ம\u{bbe}ன\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ట\u{c3e}స\u{c4d}మ\u{c3e}న\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "แทสม\u{e31}น"), ("tr", "Tasman Dstrict"), ("uk", "Тасман"), ("ur", "ضلع تسمان"), ("vi", "Quận Tasman"), ("zh", "塔斯曼")]),
                        unofficial_name_list: ["Tasman"].to_vec(),
                    }
                ),
                (
                    "TKI",
                    Subdivision{
                        name: "TKI",
                        country_alpha2: Alpha2::NZ,
                        code: "TKI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-39.3538149), longitude: Some(174.4382721), max_latitude: Some(-38.706394), min_latitude: Some(-39.869374), max_longitude: Some(174.9761954), min_longitude: Some(173.751323)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم تاراناكي"), ("bg", "Таранаки"), ("bn", "ত\u{9be}র\u{9be}ন\u{9be}কি অঞ\u{9cd}চল"), ("ca", "Taranaki"), ("ccp", "𑄑𑄢𑄚𑄇\u{11128}"), ("ceb", "Taranaki"), ("da", "Taranaki Region"), ("de", "Taranaki"), ("el", "Ταρανάκι"), ("en", "Taranaki"), ("es", "Taranaki"), ("eu", "Taranaki eskualdea"), ("fi", "Taranakin alue"), ("fr", "Taranaki"), ("gu", "તારાનાકી પ\u{acd}રદ\u{ac7}શ"), ("he", "טראנאקי"), ("hi", "तारानाकी"), ("hr", "Taranaki"), ("id", "Taranaki"), ("it", "Taranaki"), ("ja", "タラナキ地方"), ("ka", "ტარანაკის რეგიონი"), ("kn", "ತರಣಕ\u{cbf} ಪ\u{ccd}ರದೇಶ"), ("ko", "타라나키 지방"), ("lt", "Taranakio regionas"), ("lv", "Taranaki reģions"), ("mk", "Таранаки"), ("mr", "तारानाकी प\u{94d}रद\u{947}श"), ("ms", "Wilayah Taranaki"), ("nb", "Taranaki"), ("nl", "Taranaki"), ("no", "Taranaki"), ("pl", "Taranaki"), ("pt", "Taranaki"), ("ru", "Таранаки"), ("si", "ටරනක\u{dd2} කල\u{dcf}පය"), ("sv", "Taranaki"), ("ta", "தர\u{bbe}ன\u{bbe}கி பகுதி"), ("te", "త\u{c3e}ర\u{c3e}న\u{c3e}క\u{c3f} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ทารานาก\u{e34}"), ("tr", "Taranaki Bölgesi"), ("uk", "Таранакі"), ("ur", "تاراناکی"), ("vi", "Khu vực Taranaki"), ("zh", "塔拉纳基大区")]),
                        unofficial_name_list: ["Taranaki"].to_vec(),
                    }
                ),
                (
                    "WGN",
                    Subdivision{
                        name: "WGN",
                        country_alpha2: Alpha2::NZ,
                        code: "WGN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-41.2864603), longitude: Some(174.776236), max_latitude: Some(-41.126285), min_latitude: Some(-41.3624551), max_longitude: Some(174.9106252), min_longitude: Some(174.613084)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم ويلينغتون"), ("bg", "Уелингтън"), ("bn", "ওয\u{9bc}েলিংটন অঞ\u{9cd}চল"), ("ca", "Regió de Wellington"), ("ccp", "𑄃\u{1112e}𑄠𑄬𑄣\u{11128}\u{11101}𑄑\u{11127}𑄚\u{11134}"), ("ceb", "Wellington"), ("da", "Wellington Region"), ("de", "Wellington"), ("el", "Περιοχή του Γουέλινγκτον"), ("en", "Wellington"), ("es", "Región de Wellington"), ("eu", "Wellington eskualdea"), ("fi", "Wellingtonin hallintoalue"), ("fr", "Wellington"), ("gu", "વ\u{ac7}લિ\u{a82}ગ\u{acd}ટન પ\u{acd}રદ\u{ac7}શ"), ("he", "ולינגטון"), ("hi", "व\u{947}लि\u{902}ग\u{94d}टन क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Wellington"), ("id", "Wilayah Wellington"), ("it", "Wellington"), ("ja", "ウェリントン地方"), ("ka", "ველინგტონის რეგიონი"), ("kn", "ವ\u{cc6}ಲ\u{ccd}ಲ\u{cbf}ಂಗ\u{ccd}ಟನ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "웰링턴 지방"), ("lt", "Velingtono regionas"), ("lv", "Velingtonas reģions"), ("mk", "Велингтон"), ("mr", "व\u{947}लि\u{902}ग\u{94d}टन प\u{94d}रद\u{947}श"), ("ms", "Wilayah Wellington"), ("nb", "Wellington"), ("nl", "Wellington"), ("no", "Wellington"), ("pl", "Wellington"), ("pt", "Wellington"), ("ru", "Веллингтон"), ("si", "වෙල\u{dd2}න\u{dca}ටන\u{dca} කල\u{dcf}පය"), ("sv", "Wellington"), ("ta", "வெலிங\u{bcd}டன\u{bcd} பகுதி"), ("te", "వ\u{c46}ల\u{c4d}ల\u{c3f}ంగ\u{c4d}టన\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เวลล\u{e34}งต\u{e31}น"), ("tr", "Wellington Bölgesi"), ("uk", "Веллінгтон"), ("ur", "ویلنگٹن علاقہ"), ("vi", "Khu vực Wellington"), ("zh", "惠灵顿大区")]),
                        unofficial_name_list: ["Wellington"].to_vec(),
                    }
                ),
                (
                    "WKO",
                    Subdivision{
                        name: "WKO",
                        country_alpha2: Alpha2::NZ,
                        code: "WKO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-38.0594263), longitude: Some(175.4375574), max_latitude: Some(-36.42871969999999), min_latitude: Some(-39.300639), max_longitude: Some(176.6632954), min_longitude: Some(174.6134234)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم وايكاتو"), ("be", "Рэгіён Вайката"), ("bg", "Уайкато"), ("bn", "ওয\u{9bc}\u{9be}ইক\u{9be}ত\u{9c1} অঞ\u{9cd}চল"), ("ca", "Waikato"), ("ccp", "𑄃\u{1112e}𑄠𑄇𑄑\u{1112e}"), ("ceb", "Waikato"), ("da", "Waikato Region"), ("de", "Waikato"), ("el", "Γουαϊκάτο"), ("en", "Waikato"), ("es", "Waikato"), ("eu", "Waikato eskualdea"), ("fa", "وایکاتو"), ("fi", "Waikaton maakunta"), ("fr", "Waikato"), ("gu", "વાઇકાટો પ\u{acd}રદ\u{ac7}શ"), ("he", "ואיקטו"), ("hi", "वाइकाटो क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Waikato"), ("hu", "Waikato"), ("hy", "Ուակիթո"), ("id", "Waikato"), ("it", "Waikato"), ("ja", "ワイカト地方"), ("ka", "უაიკატოს რეგიონი"), ("kn", "ವೈಕಾಟೊ ಪ\u{ccd}ರದೇಶ"), ("ko", "와이카토 지방"), ("lt", "Vaikato regionas"), ("lv", "Vaitako reģions"), ("mk", "Ваикато"), ("mr", "वाइकाटो प\u{94d}रद\u{947}श"), ("ms", "Wilayah Waikato"), ("nb", "Waikato"), ("nl", "Waikato"), ("no", "Waikato"), ("pl", "Waikato"), ("pt", "Waikato"), ("ru", "Уаикато"), ("si", "වය\u{dd2}කටෝ කල\u{dcf}පය"), ("sv", "Waikato"), ("ta", "வைக\u{bbe}டோ பகுதி"), ("te", "వ\u{c48}క\u{c3e}ట\u{c4b} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ไวกาโต"), ("tr", "Waikato Bölgesi"), ("uk", "Ваікато"), ("ur", "وائکاٹو"), ("vi", "Khu vực Waikato"), ("zh", "懷卡托")]),
                        unofficial_name_list: ["Waikato"].to_vec(),
                    }
                ),
                (
                    "WTC",
                    Subdivision{
                        name: "WTC",
                        country_alpha2: Alpha2::NZ,
                        code: "WTC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-42.4064185), longitude: Some(171.6911559), max_latitude: Some(-41.16730769999999), min_latitude: Some(-44.7743752), max_longitude: Some(172.4800189), min_longitude: Some(168.0529071)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم الساحل الغربي"), ("bg", "Уест Коуст"), ("bn", "ওয\u{9bc}েস\u{9cd}ট কোস\u{9cd}ট অঞ\u{9cd}চল"), ("ca", "West Coast"), ("ccp", "𑄃\u{1112e}𑄠𑄬𑄌\u{11134} 𑄇\u{1112e}𑄌\u{11134}𑄑\u{11134}"), ("ceb", "West Coast"), ("da", "West Coast Region"), ("de", "West Coast"), ("el", "Δυτική Ακτή"), ("en", "West Coast"), ("es", "West Coast"), ("eu", "West Coast eskualdea"), ("fi", "West Coast"), ("fr", "West Coast"), ("gu", "પશ\u{acd}ચિમ કોસ\u{acd}ટ પ\u{acd}રદ\u{ac7}શ"), ("he", "החוף המערבי"), ("hi", "व\u{947}स\u{94d}ट कोस\u{94d}ट क\u{94d}ष\u{947}त\u{94d}र"), ("id", "West Coast, Selandia Baru"), ("it", "West Coast"), ("ja", "ウェスト・コースト地方"), ("ka", "უესტ-კოსტის რეგიონი"), ("kn", "ಪಶ\u{ccd}ಚ\u{cbf}ಮ ಕರಾವಳ\u{cbf} ಪ\u{ccd}ರದೇಶ"), ("ko", "웨스트코스트 지방"), ("lt", "Vakarinės pakrantės regionas"), ("lv", "Vestkostas reģions"), ("mk", "Западно Крајбрежје"), ("mr", "पश\u{94d}चिम कोस\u{94d}ट प\u{94d}रद\u{947}श"), ("ms", "Wilayah Pantai Barat"), ("nb", "West Coast"), ("nl", "West Coast"), ("no", "West Coast"), ("pl", "West Coast"), ("pt", "Costa Oeste"), ("ru", "Уэст-Кост"), ("si", "වෙස\u{dca}ට\u{dca} කෝස\u{dca}ට\u{dca} කල\u{dcf}පය"), ("sv", "West Coast"), ("ta", "மேற\u{bcd}கு கோஸ\u{bcd}ட\u{bcd} பகுதி"), ("te", "పశ\u{c4d}చ\u{c3f}మ త\u{c40}ర ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เวส คอส"), ("tr", "Batı Sahili Bölgesi"), ("uk", "Вест Коаст"), ("ur", "ویسٹ کوسٹ، نیوزی لینڈ"), ("vi", "Khu vực West Coast"), ("zh", "西岸大区")]),
                        unofficial_name_list: ["West Coast"].to_vec(),
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
#[cfg(feature = "nz")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::NZ,
        alpha3: Alpha3::NZL,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{region}}\n{{city}} {{postalcode}}\n{{country}}",
        ),
        continent: Continent::Australia,
        country_code: 64,
        currency_code: "NZD",
        gec: Some(GEC::NZ),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("NZL"),
        iso_long_name: "New Zealand",
        iso_short_name: "New Zealand",
        official_language_list: ["en"].to_vec(),
        spoken_language_list: ["en"].to_vec(),
        national_destination_code_length_list: [1].to_vec(),
        national_number_length_list: [8, 9].to_vec(),
        national_prefix: "0",
        nationality: Some("New Zealander"),
        number: "554",
        postal_code: true,
        postal_code_format: Some("\\d{4}"),
        region: Some(Region::Oceania),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::AustraliaAndNewZealand),
        un_locode: "NZ",
        unofficial_name_list: [
            "New Zealand",
            "Neuseeland",
            "Nouvelle Zélande",
            "Nueva Zelanda",
            "ニュージーランド",
            "Nieuw-Zeeland",
        ]
        .to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "New Zealand"),
            ("af", "Nieu-Seeland"),
            ("ak", "New Zealand"),
            ("am", "ኒፄ ፑሒን፥"),
            ("an", "New Zealand"),
            ("ar", "نيوزيلاندا"),
            ("as", "নিউজিলেণ\u{9cd}ড"),
            ("ay", "New Zealand"),
            ("az", "Yeni Zellandiya"),
            ("ba", "New Zealand"),
            ("be", "Новая Зеландыя"),
            ("bg", "Нова Зеландия"),
            ("bi", "New Zealand"),
            ("bn", "নিউজিল\u{9cd}য\u{9be}ন\u{9cd}ড"),
            ("bn_IN", "নিউজিল\u{9cd}য\u{9be}ন\u{9cd}ড"),
            ("br", "Zeland nevez"),
            ("bs", "Novi Zeland"),
            ("ca", "Nova Zelanda"),
            ("ce", "Керла Зеланди"),
            ("ch", "New Zealand"),
            ("cs", "Nový Zéland"),
            ("cv", "Керла Зеланди"),
            ("cy", "Seland Newydd"),
            ("da", "New Zealand"),
            ("de", "Neuseeland"),
            ("dv", "ނ\u{7a8}އ\u{7aa}ޒ\u{7a8}ލ\u{7ad}ނ\u{7b0}ޑ\u{7aa}"),
            ("dz", "ན\u{f72}འ\u{f74}་ཛ\u{f72}་ལ\u{f7a}ནཌ\u{f72}།"),
            ("ee", "New Zealand"),
            ("el", "Νέα Ζηλανδία"),
            ("en", "New Zealand"),
            ("eo", "Nov-Zelando"),
            ("es", "Nueva Zelanda"),
            ("et", "Uus-Meremaa"),
            ("eu", "Zeelanda Berria"),
            ("fa", "نیوزیلند"),
            ("ff", "New Zealand"),
            ("fi", "Uusi-Seelanti"),
            ("fo", "Ný Sæland"),
            ("fr", "Nouvelle-Zélande"),
            ("fy", "Nij-Seelân"),
            ("ga", "An Nua-Shéalainn"),
            ("gl", "Nova Celandia"),
            ("gn", "New Zealand"),
            ("gu", "ન\u{acd}ય\u{ac1} ઝીલ\u{ac7}ન\u{acd}ડ"),
            ("gv", "Yn Teelynn Noa"),
            ("ha", "New Zealand"),
            ("he", "ניו זילנד"),
            ("hi", "न\u{94d}य\u{942}ज\u{93c}ील\u{948}ण\u{94d}ड"),
            ("hr", "Novi Zeland"),
            ("ht", "Nouvèl Zelann"),
            ("hu", "Új-Zéland"),
            ("hy", "Նոր Զելանդիա"),
            ("ia", "Nove Zelanda"),
            ("id", "Selandia Baru"),
            ("io", "Nova-Zelando"),
            ("is", "Nýja-Sjáland"),
            ("it", "Nuova Zelanda"),
            ("iu", "New Zealand"),
            ("ja", "ニュージーランド"),
            ("ka", "ახალი ზელანდია"),
            ("ki", "New Zealand"),
            ("kk", "Жаңа Зеландия"),
            ("kl", "New Zealand"),
            ("km", "ន\u{17bc}វែលហ\u{17d2}សេឡង\u{17cb}"),
            ("kn", "ನ\u{ccd}ಯ\u{cc2} ಜ\u{cbf}ಲ\u{ccd}ಯಂಡ\u{ccd}"),
            ("ko", "뉴질랜드"),
            ("ku", "Zelandaya Nû"),
            ("kv", "Выль Зеландия"),
            ("kw", "Mordir Nowydh"),
            ("ky", "Жаңы Зеландия"),
            ("lo", "ປະເທດນ\u{eb9}ແວນ ເຊລ\u{eb1}ງ"),
            ("lt", "Naujoji Zelandija"),
            ("lv", "Jaunzēlande"),
            ("mi", "Aotearoa"),
            ("mk", "Нов Зеланд"),
            ("ml", "ന\u{d4d}യ\u{d42}സില\u{d3e}ന\u{d4d}\u{200d}ഡ\u{d4d}"),
            ("mn", "Шинэ зеланд"),
            ("mr", "न\u{94d}य\u{942}झिल\u{902}ड"),
            ("ms", "New Zealand"),
            ("mt", "New Zealand"),
            (
                "my",
                "နယ\u{1030}းဇ\u{102e}လန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Niu Djiran"),
            ("nb", "New Zealand"),
            ("ne", "न\u{94d}य\u{941}जिल\u{94d}याण\u{94d}ड"),
            ("nl", "Nieuw-Zeeland"),
            ("nn", "New Zealand"),
            ("nv", "New Zealand"),
            ("oc", "Novèla Zelanda"),
            ("or", "ନ\u{b4d}ଯ\u{b41}ଜୀଲ\u{b4d}ଯ\u{b3e}ଣ\u{b4d}ଡ"),
            ("pa", "ਨਿਊਜ਼ੀਲ\u{a48}\u{a02}ਡ"),
            ("pi", "न\u{94d}य\u{942}-जील\u{948}\u{902}ड"),
            ("pl", "Nowa Zelandia"),
            ("ps", "نیوزیلنډ"),
            ("pt", "Nova Zelândia"),
            ("pt_BR", "Nova Zelândia"),
            ("ro", "Noua Zeelandă"),
            ("ru", "Новая Зеландия"),
            ("rw", "Nuveli Zelande"),
            ("sc", "Zelanda Noa"),
            ("sd", "New Zealand"),
            ("si", "නවස\u{dd3}ලන\u{dca}තය"),
            ("sk", "Nový Zéland"),
            ("sl", "Nova Zelandija"),
            ("so", "Neyuusilaand"),
            ("sq", "Zelandë e Re"),
            ("sr", "Нови Зеланд"),
            ("sv", "Nya Zeeland"),
            ("sw", "New Zealand"),
            ("ta", "நியூசில\u{bbe}ந\u{bcd}து"),
            ("te", "న\u{c4d}యూజ\u{c3f}ల\u{c3e}ండ\u{c4d}"),
            ("tg", "Зеландияи Нав"),
            ("th", "น\u{e34}วซ\u{e35}แลนด\u{e4c}"),
            ("ti", "ኒው ዚላንድ"),
            ("tk", "Täze Zelandiýa"),
            ("tl", "New Zealand"),
            ("tr", "Yeni Zelanda"),
            ("tt", "Яңа Зеаланд"),
            ("ug", "يېڭى زېلاندىيە"),
            ("uk", "Нова Зеландія"),
            ("ur", "نیوزی لینڈ"),
            ("uz", "Yangi Zelandiya"),
            ("ve", "New Zealand"),
            ("vi", "Niu Xi-lân"),
            ("wa", "Nouve Zelande"),
            ("wo", "Nuweel Selaand"),
            ("xh", "New Zealand (izealand entsha)"),
            ("yo", "New Zealand"),
            ("zh_CN", "新西兰"),
            ("zh_HK", "新西蘭"),
            ("zh_TW", "紐西蘭"),
            ("zu", "INyuzilandi"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}