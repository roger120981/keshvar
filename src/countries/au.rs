// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Commonwealth of Australia

#[cfg(all(feature = "au", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{city}} {{region_short}} {{postalcode}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::AU;
    pub const ALPHA3: Alpha3 = Alpha3::AUS;
    pub const CONTINENT: Continent = Continent::Australia;
    pub const COUNTRY_CODE: usize = 61;
    pub const CURRENCY_CODE: &str = "AUD";
    pub const GEC: Option<GEC> = Some(GEC::AS);
    pub const INTERNATIONAL_PREFIX: &str = "0011";
    pub const IOC: Option<&str> = Some("AUS");
    pub const ISO_SHORT_NAME: &str = "Australia";
    pub const ISO_LONG_NAME: &str = "The Commonwealth of Australia";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Australian");
    pub const NUMBER: &str = "036";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Oceania);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::AustraliaAndNewZealand);
    pub const UN_LOCODE: &str = "AU";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Australien",
        "Australie",
        "オーストラリア",
        "Australië",
        "澳洲",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Australia"),
        ("af", "Australië"),
        ("ak", "Australia"),
        ("am", "ጐፄስትሳሑ።"),
        ("an", "Australia"),
        ("ar", "أستراليا"),
        ("as", "অস\u{9cd}ট\u{9cd}ৰেলিয়\u{9be}"),
        ("ay", "Australia"),
        ("az", "Avstraliya"),
        ("ba", "Australia"),
        ("be", "Аўстралія"),
        ("bg", "Австралия"),
        ("bi", "Australia"),
        ("bn", "অস\u{9cd}ট\u{9cd}রেলিয়\u{9be}"),
        ("bn_IN", "অস\u{9cd}ট\u{9cd}রেলিয়\u{9be}"),
        ("br", "Aostralia"),
        ("bs", "Australija"),
        ("ca", "Austràlia"),
        ("ce", "Австрали"),
        ("ch", "Australia"),
        ("cs", "Austrálie"),
        ("cv", "Австрали"),
        ("cy", "Awstralia"),
        ("da", "Australien"),
        ("de", "Australien"),
        ("dv", "އ\u{7ae}ސ\u{7b0}ޓ\u{7a6}ރ\u{7aa}ލ\u{7a8}ޔ\u{7a7}"),
        ("dz", "ཨ\u{f71}ས\u{f72}་ཊ་ལ\u{f72}་ཡ།"),
        ("ee", "Australia"),
        ("el", "Αυστραλία"),
        ("en", "Australia"),
        ("eo", "Aŭstralio"),
        ("es", "Australia"),
        ("et", "Austraalia"),
        ("eu", "Australia"),
        ("fa", "استرالیا"),
        ("ff", "Australia"),
        ("fi", "Australia"),
        ("fo", "Avstralia"),
        ("fr", "Australie"),
        ("fy", "Austraalje"),
        ("ga", "An Astráil"),
        ("gl", "Australia"),
        ("gn", "Australia"),
        ("gu", "ઓસ\u{acd}ટ\u{acd}ર\u{ac7}લિયા"),
        ("gv", "Yn Austrail"),
        ("ha", "Asturaliya"),
        ("he", "אוסטרליה"),
        ("hi", "ऑस\u{94d}ट\u{94d}र\u{947}लिया"),
        ("hr", "Australija"),
        ("ht", "Ostrali"),
        ("hu", "Ausztrália"),
        ("hy", "Ավստրալիա"),
        ("ia", "Australia"),
        ("id", "Australia"),
        ("io", "Australia"),
        ("is", "Ástralía"),
        ("it", "Australia"),
        ("iu", "ᐊᔅᑦᕌᓕᐊ"),
        ("ja", "オーストラリア連邦"),
        ("ka", "ავსტრალია"),
        ("ki", "Australia"),
        ("kk", "Австралия"),
        ("kl", "Australia"),
        ("km", "អ\u{17bc}ស\u{17d2}ត\u{17d2}រាល\u{17b8}"),
        ("kn", "ಆಸ\u{ccd}ಟ\u{ccd}ರೇಲ\u{cbf}ಯ"),
        ("ko", "오스트레일리아"),
        ("ku", "Awustralya"),
        ("kv", "Австралия"),
        ("kw", "Ostrali"),
        ("ky", "Австралия"),
        ("lo", "ປະເທດອ\u{ebb}ດສະຕາລ\u{eb5}"),
        ("lt", "Australija"),
        ("lv", "Austrālija"),
        ("mi", "Ahitereiria"),
        ("mk", "Австралија"),
        ("ml", "ഓസ\u{d4d}ട\u{d4d}രേലിയ"),
        ("mn", "Австрали"),
        ("mr", "ऑस\u{94d}ट\u{94d}र\u{947}लिया"),
        ("ms", "Australia"),
        ("mt", "Awstralja"),
        ("my", "ဩစတြေးလျန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
        ("na", "Otereiriya"),
        ("nb", "Australia"),
        ("ne", "अष\u{94d}ट\u{94d}र\u{947}लिया"),
        ("nl", "Australië"),
        ("nn", "Australia"),
        ("nv", "Nahatʼeʼiitsoh Bikéyah"),
        ("oc", "Australia"),
        ("or", "ଅଷ\u{b4d}ଟ\u{b4d}ରେଲ\u{b3f}ୟ\u{b3e}"),
        ("pa", "ਅਸਟਰ\u{a47}ਲੀਆ"),
        ("pi", "आस\u{94d}ट\u{94d}र\u{947}लिया"),
        ("pl", "Australia"),
        ("ps", "آسټراليا"),
        ("pt", "Austrália"),
        ("pt_BR", "Austrália"),
        ("ro", "Australia"),
        ("ru", "Австралия"),
        ("rw", "Ositaraliya"),
        ("sc", "Austràlia"),
        ("sd", "آسٽريليا"),
        ("si", "ඔස\u{dca}ට\u{dca}\u{200d}රේල\u{dd2}ය\u{dcf}ව"),
        ("sk", "Austrália"),
        ("sl", "Avstralija"),
        ("so", "Awstraaliya"),
        ("sq", "Australi"),
        ("sr", "Аустралија"),
        ("sv", "Australien"),
        ("sw", "Australia"),
        ("ta", "ஆஸ\u{bcd}திரேலிய\u{bbe}"),
        ("te", "ఓస\u{c4d}ట\u{c4d}ర\u{c47}ల\u{c3f}య\u{c3e}"),
        ("tg", "Австралия"),
        ("th", "ออสเตรเล\u{e35}ย"),
        ("ti", "ኣውስትራልያ"),
        ("tk", "Awstraliýa"),
        ("tl", "Australya"),
        ("tr", "Avustralya"),
        ("tt", "Аустралиа"),
        ("ug", "ئاۋسترالىيە"),
        ("uk", "Австралія"),
        ("ur", "آسٹریلیا"),
        ("uz", "Avstraliya"),
        ("ve", "Australia"),
        ("vi", "Úc"),
        ("wa", "Ostraleye"),
        ("wo", "Óstraali"),
        ("xh", "Australiya"),
        ("yo", "Austrálíà"),
        ("zh_CN", "澳大利亚"),
        ("zh_HK", "澳大利亞"),
        ("zh_TW", "澳大利亞"),
        ("zu", "I-Ostreliya"),
    ];
    #[cfg(all(feature = "au", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -25.274398;
        pub const LONGITUDE: f64 = 133.775136;
        pub const MAX_LATITUDE: f64 = -9.187026399999999;
        pub const MAX_LONGITUDE: f64 = 159.2872223;
        pub const MIN_LATITUDE: f64 = -54.83376579999999;
        pub const MIN_LONGITUDE: f64 = 110.9510339;
        pub const NORTHEAST_LATITUDE: f64 = -9.187026399999999;
        pub const NORTHEAST_LONGITUDE: f64 = 159.2872223;
        pub const SOUTHWEST_LATITUDE: f64 = -54.83376579999999;
        pub const SOUTHWEST_LONGITUDE: f64 = 110.9510339;
    }
}
#[cfg(all(feature = "au", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -25.274398,
            longitude: 133.775136,
            max_latitude: -9.187026399999999,
            max_longitude: 159.2872223,
            min_latitude: -54.83376579999999,
            min_longitude: 110.9510339,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: -9.187026399999999,
                    longitude: 159.2872223,
                },
                southwest: CountryGeoBound {
                    latitude: -54.83376579999999,
                    longitude: 110.9510339,
                },
            },
        }
    }
}

#[cfg(all(feature = "au", feature = "subdivisions"))]
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
                    "ACT",
                    Subdivision{
                        name: "ACT",
                        country_alpha2: Alpha2::AU,
                        code: "ACT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-35.4734679), longitude: Some(149.0123679), max_latitude: Some(-35.1245128), min_latitude: Some(-35.9205307), max_longitude: Some(149.3992848), min_longitude: Some(148.7640971)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Territory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Australiese Hoofstadgebied"), ("ar", "مقاطعة العاصمة الأسترالية"), ("az", "Avstraliya Paytaxt Ərazisi"), ("be", "Аўстралійская сталічная тэрыторыя"), ("bg", "Австралийска столична територия"), ("bn", "অস\u{9cd}ট\u{9cd}রেলিয\u{9bc}\u{9be}ন ক\u{9cd}য\u{9be}পিট\u{9be}ল টেরিটরি"), ("bs", "Teritorija australijskog glavnog grada"), ("ca", "Territori de la Capital Australiana"), ("ccp", "𑄃\u{11127}𑄌\u{11134}𑄑\u{11133}𑄢𑄬𑄣\u{11128}𑄠𑄚\u{11134} 𑄇\u{11133}𑄠𑄛\u{11128}𑄑𑄣\u{11134} 𑄑𑄬𑄢\u{11128}𑄑\u{1112e}𑄢\u{11128}"), ("ceb", "Australian Capital Territory"), ("cs", "Teritorium hlavního města Austrálie"), ("cy", "Tiriogaeth Prifddinas Awstralia"), ("da", "Australian Capital Territory"), ("de", "Australian Capital Territory"), ("el", "Επικράτεια Αυστραλιανής Πρωτεύουσας"), ("en", "Australian Capital Territory"), ("es", "Territorio de la Capital Australiana"), ("et", "Austraalia pealinna ala"), ("eu", "Australiako Hiriburuaren Lurraldea"), ("fa", "قلمرو پایتختی استرالیا"), ("fi", "Australian pääkaupunkiterritorio"), ("fr", "Territoire de la capitale australienne"), ("ga", "Críoch Phríomhchathair na hAstráile"), ("gl", "Territorio da Capital Australiana"), ("gu", "ઓસ\u{acd}ટ\u{acd}ર\u{ac7}લિયન ક\u{ac7}પિટલ ટ\u{ac7}રિટરી"), ("he", "טריטוריית הבירה האוסטרלית"), ("hi", "ऑस\u{94d}ट\u{94d}र\u{947}लियाई राजधानी क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Teritorij australskog glavnog grada"), ("hu", "Ausztráliai fővárosi terület"), ("hy", "Ավստրալիական մայրաքաղաքային տարածք"), ("id", "Wilayah Ibu Kota Australia"), ("is", "Höfuðborgarsvæði Ástralíu"), ("it", "Territorio della Capitale Australiana"), ("ja", "オーストラリア首都特別地域"), ("ka", "ფედერალური ტერიტორია"), ("kn", "ಆಸ\u{ccd}ಟ\u{ccd}ರೇಲ\u{cbf}ಯನ\u{ccd} ಕ\u{ccd}ಯಾಪ\u{cbf}ಟಲ\u{ccd} ಟ\u{cc6}ರ\u{cbf}ಟರ\u{cbf}"), ("ko", "오스트레일리아 수도 준주"), ("lt", "Australijos sostinės teritorija"), ("lv", "Austrālijas galvaspilsētas teritorija"), ("mk", "Австралиска престолнинска територија"), ("mn", "Австралийн Нийслэлийн Нутаг Дэвсгэр"), ("mr", "ऑस\u{94d}ट\u{94d}र\u{947}लियन क\u{945}पिटल ट\u{947}रिटोरी"), ("ms", "Wilayah Ibu Negara Australia"), ("nb", "Australian Capital Territory"), ("ne", "अस\u{94d}ट\u{94d}र\u{947}लियन राजधानी क\u{94d}ष\u{947}त\u{94d}र"), ("nl", "Australian Capital Territory"), ("no", "Australian Capital Territory"), ("pa", "ਆਸਟਰ\u{a47}ਲੀਆਈ ਰਾਜਧਾਨੀ ਰਾਜਖ\u{a47}ਤਰ"), ("pl", "Australijskie Terytorium Stołeczne"), ("pt", "Território da Capital Australiana"), ("ro", "Australian Capital Territory"), ("ru", "Австралийская столичная территория"), ("si", "ඕස\u{dca}ට\u{dca}\u{200d}රේල\u{dd2}ය\u{dcf}න\u{dd4} ප\u{dca}\u{200d}රධ\u{dcf}න කල\u{dcf}පය"), ("sk", "Teritórium austrálskeho hlavného mesta"), ("sr", "Аустралијска престоничка територија"), ("sr_Latn", "Australijska prestonička teritorija"), ("sv", "Australian Capital Territory"), ("ta", "ஆத\u{bcd}திரேலியத\u{bcd} தலைநகர ஆட\u{bcd}புலம\u{bcd}"), ("te", "ఆస\u{c4d}ట\u{c4d}ర\u{c47}ల\u{c3f}యన\u{c4d} క\u{c4d}య\u{c3e}ప\u{c3f}టల\u{c4d} ట\u{c46}ర\u{c3f}టర\u{c40}"), ("th", "ออสเตรเล\u{e35}ยนแคพ\u{e34}ทอลเทร\u{e4c}ร\u{e34}ทอร\u{e35}"), ("tr", "Avustralya Başkent Bölgesi"), ("uk", "Австралійська столична територія"), ("ur", "آسٹریلوی دارالحکومت علاقہ"), ("vi", "Lãnh thổ Thủ đô Úc"), ("yue", "澳洲首都地區"), ("yue_Hans", "澳洲首都地区"), ("zh", "澳大利亞首都特區")]),
                        unofficial_name_list: ["Australian Capital Territory"].to_vec(),
                    }
                ),
                (
                    "NSW",
                    Subdivision{
                        name: "NSW",
                        country_alpha2: Alpha2::AU,
                        code: "NSW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-33.864174), longitude: Some(151.2052868), max_latitude: Some(-28.156192), min_latitude: Some(-37.5052772), max_longitude: Some(153.6535617), min_longitude: Some(140.9992123)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nieu-Suid-Wallis"), ("am", "ኑ ሳውስ ዌልስ"), ("ar", "نيوساوث ويلز"), ("az", "Yeni Cənubi Uels"), ("be", "штат Новы Паўднёвы Уэльс"), ("bg", "Нови Южен Уелс"), ("bn", "নিউ স\u{9be}উথ ওয\u{9bc}েল\u{9cd}স"), ("bs", "Novi Južni Wales"), ("ca", "Nova Gal·les del Sud"), ("ccp", "𑄚\u{11131} 𑄘\u{11127}𑄉\u{11128}𑄚\u{11134} 𑄃\u{1112e}𑄠𑄬𑄣\u{11134}"), ("ceb", "State of New South Wales"), ("cs", "Nový Jižní Wales"), ("cy", "De Cymru Newydd"), ("da", "New South Wales"), ("de", "New South Wales"), ("el", "Νέα Νότια Ουαλία"), ("en", "New South Wales"), ("es", "Nueva Gales del Sur"), ("et", "Uus-Lõuna-Wales"), ("eu", "Hegoaldeko Gales Berria"), ("fa", "نیو ساوت ولز"), ("fi", "Uusi Etelä-Wales"), ("fr", "Nouvelle-Galles du Sud"), ("ga", "New South Wales"), ("gl", "Nova Gales do Sur"), ("gu", "ન\u{acd}ય\u{ac2} સાઉથ વ\u{ac7}લ\u{acd}સ"), ("he", "ניו סאות׳ ויילס"), ("hi", "न\u{94d}य\u{942} साउथ व\u{947}ल\u{94d}स"), ("hr", "Novi Južni Wales"), ("hu", "Új-Dél-Wales"), ("hy", "Նոր Հարավային Ուելս"), ("id", "New South Wales"), ("is", "Nýja Suður-Wales"), ("it", "Nuovo Galles del Sud"), ("ja", "ニューサウスウェールズ州"), ("jv", "New South Wales"), ("ka", "ახალი სამხრეთი უელსი"), ("kn", "ನ\u{ccd}ಯ\u{cc2} ಸ\u{ccc}ತ\u{ccd} ವೇಲ\u{ccd}ಸ\u{ccd}"), ("ko", "뉴사우스웨일스 주"), ("lt", "Naujasis Pietų Velsas"), ("lv", "Jaundienvidvelsa"), ("mk", "Нов Јужен Велс"), ("ml", "ന\u{d4d}യ\u{d42} സ\u{d57}ത\u{d4d}ത\u{d4d} വെയ\u{d4d}ൽസ\u{d4d}"), ("mn", "Шинэ Өмнөд Вэльс"), ("mr", "न\u{94d}य\u{942} साउथ व\u{947}ल\u{94d}स"), ("ms", "New South Wales"), ("nb", "New South Wales"), ("ne", "न\u{94d}य\u{942} साउथ व\u{947}ल\u{94d}स"), ("nl", "Nieuw-Zuid-Wales"), ("no", "New South Wales"), ("pa", "ਨਿਊ ਸਾਊਥ ਵ\u{a47}ਲਜ\u{a3c}"), ("pl", "Nowa Południowa Walia"), ("pt", "Nova Gales do Sul"), ("ro", "Noul Wales de Sud"), ("ru", "Новый Южный Уэльс"), ("si", "න\u{dd2}ව\u{dca} සව\u{dd4}ත\u{dca} වේල\u{dca}ස\u{dca}"), ("sk", "Nový Južný Wales"), ("sl", "Novi Južni Wales"), ("so", "New South Wales"), ("sq", "Uellsi i Ri Jugor"), ("sr", "Нови Јужни Велс"), ("sr_Latn", "Novi Južni Vels"), ("sv", "New South Wales"), ("sw", "New South Wales"), ("ta", "நியூ சவுத\u{bcd} வேல\u{bcd}ஸ\u{bcd}"), ("te", "న\u{c4d}యూ స\u{c4c}త\u{c4d} వ\u{c47}ల\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐน\u{e34}วเซาท\u{e4c}เวลส\u{e4c}"), ("tr", "Yeni Güney Galler"), ("uk", "Новий Південний Уельс"), ("ur", "نیو ساؤتھ ویلز"), ("uz", "Yangi janubiy uels"), ("vi", "New South Wales"), ("yue", "新南威爾士"), ("yue_Hans", "新南威尔士"), ("zh", "新南威爾士州")]),
                        unofficial_name_list: ["New South Wales"].to_vec(),
                    }
                ),
                (
                    "NT",
                    Subdivision{
                        name: "NT",
                        country_alpha2: Alpha2::AU,
                        code: "NT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-19.4914108), longitude: Some(132.5509603), max_latitude: Some(-10.9055196), min_latitude: Some(-26.0168698), max_longitude: Some(137.9990092), min_longitude: Some(129.0004244)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Territory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Noordelike Gebied"), ("ar", "إقليم شمالي"), ("az", "Şimal ərazisi"), ("be", "Паўночная тэрыторыя, Аўстралія"), ("bg", "Северна територия"), ("bn", "নর\u{9cd}দ\u{9be}ন টেরিটরি"), ("bs", "Sjeverna teritorija"), ("ca", "Territori del Nord"), ("ccp", "𑄚\u{11127}𑄢\u{11134}𑄘𑄢\u{11134}𑄚\u{11134} 𑄑𑄬𑄢\u{11128}𑄑\u{1112e}𑄢\u{11128}"), ("ceb", "Northern Territory"), ("cs", "Severní teritorium"), ("cy", "Tiriogaeth y Gogledd"), ("da", "Northern Territory"), ("de", "Northern Territory"), ("el", "Βόρεια Επικράτεια"), ("en", "Northern Territory"), ("es", "Territorio del Norte"), ("et", "Põhjaterritoorium"), ("eu", "Iparraldeko Lurraldea"), ("fa", "قلمرو شمالی"), ("fi", "Pohjoisterritorio"), ("fr", "Territoire du Nord"), ("ga", "Críoch an Tuaiscirt"), ("gl", "Territorio do Norte"), ("gu", "નધર\u{acd}ન રીજન"), ("he", "הטריטוריה הצפונית"), ("hi", "नॉर\u{94d}थर\u{94d}न ट\u{947}रिटरी"), ("hr", "Sjeverni teritorij"), ("hu", "Északi terület"), ("hy", "Հյուսիսային տարածք"), ("id", "Wilayah Utara"), ("is", "Norður-svæðið"), ("it", "Territorio del Nord"), ("ja", "ノーザンテリトリー"), ("ka", "ჩრდილოეთი ტერიტორია"), ("kn", "ಉತ\u{ccd}ತರ ಪ\u{ccd}ರದೇಶ"), ("ko", "노던 준주"), ("lt", "Šiaurinė Teritorija"), ("lv", "Ziemeļu teritorija"), ("mk", "Северна Територија"), ("mn", "Хойд Нутаг Дэвсгэр"), ("mr", "नॉर\u{94d}दर\u{94d}न ट\u{947}रिटोरी"), ("ms", "Wilayah Utara"), ("my", "မြောက\u{103a}ဩစတြေးလျပြည\u{103a}နယ\u{103a}"), ("nb", "Nordterritoriet"), ("nl", "Noordelijk Territorium"), ("no", "Nordterritoriet"), ("pa", "ਉ\u{a71}ਤਰੀ ਰਾਜਖ\u{a47}ਤਰ"), ("pl", "Terytorium Północne"), ("pt", "Território do Norte"), ("ro", "Teritoriul de Nord"), ("ru", "Северная территория"), ("si", "උත\u{dd4}ර\u{dd4} භ\u{dd6}ම\u{dd2}ය"), ("sk", "Severné teritórium"), ("sq", "Northern Territory"), ("sr", "Северна територија"), ("sr_Latn", "Severna teritorija"), ("sv", "Northern Territory"), ("sw", "Northern Territory"), ("ta", "வட ஆட\u{bcd}புலம\u{bcd}"), ("te", "ఉత\u{c4d}తర ట\u{c46}ర\u{c3f}టర\u{c40}"), ("th", "นอร\u{e4c}เท\u{e34}ร\u{e4c}นเทร\u{e4c}ร\u{e34}ทอร\u{e35}"), ("tr", "Kuzey Toprakları"), ("uk", "Північна територія"), ("ur", "شمالی علاقہ"), ("vi", "Lãnh thổ Bắc Úc"), ("yue", "北領地"), ("yue_Hans", "北领地"), ("zh", "北領地")]),
                        unofficial_name_list: ["Northern Territory"].to_vec(),
                    }
                ),
                (
                    "QLD",
                    Subdivision{
                        name: "QLD",
                        country_alpha2: Alpha2::AU,
                        code: "QLD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-20.9175738), longitude: Some(142.7027956), max_latitude: Some(-9.92973), min_latitude: Some(-29.1785876), max_longitude: Some(153.5529199), min_longitude: Some(137.9945748)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Queensland"), ("am", "ኲንዝላንድ"), ("ar", "كوينزلاند"), ("az", "Kvinslend"), ("be", "Штат Квінслэнд"), ("bg", "Куинсланд"), ("bn", "ক\u{9c1}ইন\u{9cd}সল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("bs", "Queensland"), ("ca", "Queensland"), ("ccp", "𑄇\u{1112a}𑄃\u{11128}𑄚\u{11134}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "State of Queensland"), ("cs", "Queensland"), ("cy", "Queensland"), ("da", "Queensland"), ("de", "Queensland"), ("el", "Κουΐνσλαντ"), ("en", "Queensland"), ("es", "Queensland"), ("et", "Queensland"), ("eu", "Queensland"), ("fa", "کوئینزلند"), ("fi", "Queensland"), ("fr", "Queensland"), ("ga", "Queensland"), ("gl", "Queensland"), ("gu", "ક\u{acd}વીન\u{acd}સલ\u{ac7}ન\u{acd}ડ"), ("he", "קווינסלנד"), ("hi", "क\u{94d}वीन\u{94d}सल\u{948}ण\u{94d}ड"), ("hr", "Queensland"), ("hu", "Queensland"), ("hy", "Քվինսլենդ"), ("id", "Queensland"), ("is", "Queensland"), ("it", "Queensland"), ("ja", "クイーンズランド州"), ("jv", "Queensland"), ("ka", "კუინზლენდი"), ("kk", "Квинсленд"), ("kn", "ಕ\u{ccd}ವೀನ\u{ccd}ಸ\u{ccd}\u{200c}ಲ\u{ccd}ಯಾಂಡ\u{ccd}\u{200c}"), ("ko", "퀸즐랜드 주"), ("lt", "Kvinslandas"), ("lv", "Kvīnslenda"), ("mk", "Квинсленд"), ("ml", "ക\u{d4d}വീൻസ\u{d4d}\u{200c}ല\u{d3e}ൻഡ\u{d4d}"), ("mn", "Квийнслэнд"), ("mr", "क\u{94d}वीन\u{94d}सल\u{902}ड"), ("ms", "Queensland"), ("my", "က\u{103d}င\u{103a}းစလန\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "Queensland"), ("ne", "क\u{94d}विन\u{94d}सल\u{94d}याण\u{94d}ड"), ("nl", "Queensland"), ("no", "Queensland"), ("pa", "ਕਵੀਨਜ\u{a3c}ਲ\u{a48}\u{a02}ਡ"), ("pl", "Queensland"), ("pt", "Queensland"), ("ro", "Queensland"), ("ru", "Квинсленд"), ("si", "ක\u{dca}ව\u{dd2}න\u{dca}ස\u{dca}ලන\u{dca}තය"), ("sk", "Queensland"), ("sl", "Queensland"), ("so", "Queensland"), ("sq", "Queensland"), ("sr", "Квинсленд"), ("sr_Latn", "Kvinslend"), ("sv", "Queensland"), ("sw", "Queensland"), ("ta", "குயின\u{bcd}ஸ\u{bcd}ல\u{bbe}ந\u{bcd}து"), ("te", "క\u{c4d}వ\u{c40}న\u{c4d}స\u{c4d}\u{200c}ల\u{c3e}ండ\u{c4d}"), ("th", "ร\u{e31}ฐคว\u{e35}นส\u{e4c}แลนด\u{e4c}"), ("tr", "Queensland"), ("uk", "Квінсленд"), ("ur", "کوئنزلینڈ"), ("uz", "Kvinslend"), ("vi", "Queensland"), ("yo", "Queensland"), ("yo_BJ", "Queensland"), ("yue", "昆士蘭"), ("yue_Hans", "昆士兰"), ("zh", "昆士蘭州")]),
                        unofficial_name_list: ["Queensland"].to_vec(),
                    }
                ),
                (
                    "SA",
                    Subdivision{
                        name: "SA",
                        country_alpha2: Alpha2::AU,
                        code: "SA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-30.0002315), longitude: Some(136.2091547), max_latitude: Some(-25.996392), min_latitude: Some(-38.06121), max_longitude: Some(141.0028804), min_longitude: Some(129.0005162)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Suid-Australië"), ("am", "ደቡብ አውስትራሊያ"), ("ar", "جنوب أستراليا"), ("az", "Cənubi Avstraliya"), ("be", "Штат Паўднёвая Аўстралія"), ("bg", "Южна Австралия"), ("bn", "স\u{9be}উথ অস\u{9cd}ট\u{9cd}রেলিয\u{9bc}\u{9be}"), ("bs", "Južna Australija"), ("ca", "Austràlia Meridional"), ("ccp", "𑄘\u{11127}𑄉\u{11128}𑄚\u{11134} 𑄃\u{11127}𑄌\u{11134}𑄑\u{11133}𑄢𑄬𑄣\u{11128}𑄠"), ("ceb", "State of South Australia"), ("cs", "Jižní Austrálie"), ("cy", "De Awstralia"), ("da", "South Australia"), ("de", "South Australia"), ("el", "Νότια Αυστραλία"), ("en", "South Australia"), ("es", "Australia Meridional"), ("et", "Lõuna-Austraalia"), ("eu", "Hegoaldeko Australia"), ("fa", "استرالیای جنوبی"), ("fi", "Etelä-Australia"), ("fr", "Australie-Méridionale"), ("ga", "Deisceart na hAstráile"), ("gl", "Australia Meridional"), ("gu", "સાઉથ ઓસ\u{acd}ટ\u{acd}ર\u{ac7}લિયા"), ("he", "אוסטרליה הדרומית"), ("hi", "दक\u{94d}षिण ऑस\u{94d}ट\u{94d}र\u{947}लिया"), ("hr", "Južna Australija"), ("hu", "Dél-Ausztrália"), ("hy", "Հարավային Ավստրալիա"), ("id", "Australia Selatan"), ("is", "Suður-Ástralía"), ("it", "Australia Meridionale"), ("ja", "南オーストラリア州"), ("ka", "სამხრეთი ავსტრალია"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಆಸ\u{ccd}ಟ\u{ccd}ರೇಲ\u{cbf}ಯಾ"), ("ko", "사우스오스트레일리아 주"), ("lt", "Pietų Australija"), ("lv", "Dienvidaustrālija"), ("mk", "Јужна Австралија"), ("mn", "Өмнөд Австрали"), ("mr", "साउथ ऑस\u{94d}ट\u{94d}र\u{947}लिया"), ("ms", "Australia Selatan"), ("nb", "Sør-Australia"), ("ne", "दक\u{94d}षिण अस\u{94d}ट\u{94d}र\u{947}लिया"), ("nl", "Zuid-Australië"), ("no", "Sør-Australia"), ("pa", "ਸਾਊਥ ਆਸਟਰ\u{a47}ਲੀਆ"), ("pl", "Australia Południowa"), ("pt", "Austrália Meridional"), ("ro", "Australia de Sud"), ("ru", "Южная Австралия"), ("si", "දක\u{dd4}ණ\u{dd4} ඔස\u{dca}ට\u{dca}\u{200d}රේල\u{dd2}ය\u{dcf}ව"), ("sk", "Južná Austrália"), ("sl", "Južna Avstralija"), ("so", "Koonfur Australia"), ("sq", "Australina Jugore"), ("sr", "Јужна Аустралија"), ("sr_Latn", "Južna Australija"), ("sv", "South Australia"), ("sw", "Australia Kusini"), ("ta", "தெற\u{bcd}கு ஆஸ\u{bcd}திரேலிய\u{bbe}"), ("te", "దక\u{c4d}ష\u{c3f}ణ ఆస\u{c4d}ట\u{c4d}ర\u{c47}ల\u{c3f}య\u{c3e}"), ("th", "ร\u{e31}ฐเซาท\u{e4c}ออสเตรเล\u{e35}ย"), ("tr", "Güney Avustralya"), ("uk", "Південна Австралія"), ("ur", "جنوبی آسٹریلیا"), ("vi", "Nam Úc"), ("yue", "南澳洲"), ("yue_Hans", "南澳洲"), ("zh", "南澳大利亚州")]),
                        unofficial_name_list: ["South Australia"].to_vec(),
                    }
                ),
                (
                    "TAS",
                    Subdivision{
                        name: "TAS",
                        country_alpha2: Alpha2::AU,
                        code: "TAS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-41.3650419), longitude: Some(146.6284905), max_latitude: Some(-39.4380355), min_latitude: Some(-43.8077261), max_longitude: Some(148.7283943), min_longitude: Some(143.8182852)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tasmanië"), ("am", "ታዝሜኒያ"), ("ar", "تسمانيا"), ("az", "Tasmaniya"), ("be", "Тасманія"), ("bg", "Тасмания"), ("bn", "ত\u{9be}সম\u{9be}নিয\u{9bc}\u{9be}"), ("bs", "Tasmanija"), ("ca", "Tasmània"), ("ccp", "𑄑𑄌\u{11134}𑄟𑄚\u{11128}𑄠"), ("ceb", "State of Tasmania"), ("cs", "Tasmánie"), ("cy", "Tasmania"), ("da", "Tasmanien"), ("de", "Tasmanien"), ("el", "Τασμανία"), ("en", "Tasmania"), ("es", "Tasmania"), ("et", "Tasmaania"), ("eu", "Tasmania"), ("fa", "تاسمانی"), ("fi", "Tasmania"), ("fr", "Tasmanie"), ("ga", "An Tasmáin"), ("gl", "Tasmania"), ("gu", "તાસ\u{acd}માનિયા"), ("he", "טסמניה"), ("hi", "टासमानिया"), ("hr", "Tasmanija"), ("hu", "Tasmania"), ("hy", "Թասմանիա"), ("id", "Tasmania"), ("is", "Tasmanía"), ("it", "Tasmania"), ("ja", "タスマニア州"), ("ka", "ტასმანია"), ("kk", "Тасман аралы"), ("kn", "ಟ\u{ccd}ಯಾಸ\u{ccd}ಮ\u{cc6}ನ\u{cbf}ಯಾ"), ("ko", "태즈메이니아 주"), ("ky", "Тасмания"), ("lt", "Tasmanija"), ("lv", "Tasmanija"), ("mk", "Тасманија"), ("ml", "ട\u{d3e}സ\u{d4d}മേനിയ"), ("mn", "Тасмани"), ("mr", "टास\u{94d}मानिया"), ("ms", "Tasmania"), ("nb", "Tasmania"), ("ne", "ट\u{94d}याज\u{94d}मानिया"), ("nl", "Tasmanië"), ("no", "Tasmania"), ("or", "ଟ\u{b3e}ସମ\u{b3e}ନ\u{b3f}ଆ"), ("pa", "ਤਸਮਾਨੀਆ"), ("pl", "Tasmania"), ("pt", "Tasmânia"), ("ro", "Tasmania"), ("ru", "Тасмания"), ("si", "ටස\u{dca}මෙන\u{dd2}ය\u{dcf}"), ("sk", "Tasmánia"), ("sl", "Tasmanija"), ("so", "Tasmaniya"), ("sq", "Tasmania"), ("sr", "Тасманија"), ("sr_Latn", "Tasmanija"), ("sv", "Tasmanien"), ("sw", "Tasmania"), ("ta", "த\u{bbe}சுமேனிய\u{bbe}"), ("te", "ట\u{c3e}స\u{c4d}మ\u{c3e}న\u{c3f}య\u{c3e}"), ("th", "ร\u{e31}ฐแทสเมเน\u{e35}ย"), ("tr", "Tasmanya"), ("uk", "Тасманія"), ("ur", "تسمانیا"), ("uz", "Tasmaniya"), ("vi", "Tasmania"), ("yue", "塔斯曼尼亞省"), ("yue_Hans", "塔斯曼尼亚省"), ("zh", "塔斯馬尼亞州")]),
                        unofficial_name_list: ["Tasmania"].to_vec(),
                    }
                ),
                (
                    "VIC",
                    Subdivision{
                        name: "VIC",
                        country_alpha2: Alpha2::AU,
                        code: "VIC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-37.4713077), longitude: Some(144.7851531), max_latitude: Some(-33.9810507), min_latitude: Some(-39.2247306), max_longitude: Some(149.9764882), min_longitude: Some(140.9624773)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Victoria"), ("ar", "ولاية فيكتوريا"), ("az", "Viktoriya"), ("be", "штат Вікторыя"), ("bg", "Виктория"), ("bn", "ভিক\u{9cd}টোরিয\u{9bc}\u{9be}"), ("bs", "Victoria"), ("ca", "Victoria"), ("ccp", "𑄞\u{11128}𑄇\u{11134}𑄑\u{1112e}𑄢\u{11128}𑄠"), ("ceb", "State of Victoria"), ("cs", "Victoria"), ("cy", "Victoria"), ("da", "Victoria"), ("de", "Victoria"), ("el", "Βικτώρια"), ("en", "Victoria"), ("es", "Victoria"), ("et", "Victoria"), ("eu", "Victoria"), ("fa", "ویکتوریا"), ("fi", "Victoria"), ("fr", "Victoria"), ("ga", "Victoria"), ("gl", "Victoria"), ("gu", "વિક\u{acd}ટોરિયા"), ("he", "ויקטוריה"), ("hi", "विक\u{94d}टोरिया"), ("hr", "Victoria"), ("hu", "Victoria"), ("hy", "Վիկտորիա"), ("id", "Victoria"), ("is", "Victoria"), ("it", "Victoria"), ("ja", "ビクトリア州"), ("jv", "Victoria"), ("ka", "ვიქტორია"), ("kn", "ವ\u{cbf}ಕ\u{ccd}ಟೋರ\u{cbf}ಯಾ"), ("ko", "빅토리아 주"), ("lt", "Viktorija"), ("lv", "Viktorija"), ("mk", "Викторија"), ("mn", "Викториа"), ("mr", "व\u{94d}हिक\u{94d}टोरिया"), ("ms", "Victoria"), ("my", "ဝ\u{102d}တ\u{102d}\u{102f}ရ\u{102d}ယ ပြည\u{103a}နယ\u{103a}"), ("nb", "Victoria"), ("ne", "भिक\u{94d}टोरिया"), ("nl", "Victoria"), ("no", "Victoria"), ("pa", "ਵਿਕਟ\u{a4b}ਰੀਆ"), ("pl", "Wiktoria"), ("pt", "Vitória"), ("ro", "Victoria"), ("ru", "Виктория"), ("si", "ව\u{dd2}ක\u{dca}ටෝර\u{dd2}ය\u{dcf}"), ("sk", "Viktória"), ("sl", "Viktorija"), ("sr", "Викторија"), ("sr_Latn", "Viktorija"), ("sv", "Victoria"), ("sw", "Victoria"), ("ta", "விக\u{bcd}டோரிய\u{bbe}"), ("te", "వ\u{c3f}క\u{c4d}ట\u{c4b}ర\u{c3f}య\u{c3e}"), ("th", "ร\u{e31}ฐว\u{e34}กตอเร\u{e35}ย"), ("tr", "Victoria"), ("uk", "Вікторія"), ("ur", "وکٹوریہ (آسٹریلیا)"), ("vi", "Victoria"), ("yue", "維多利亞州"), ("yue_Hans", "维多利亚州"), ("zh", "維多利亞州")]),
                        unofficial_name_list: ["Victoria"].to_vec(),
                    }
                ),
                (
                    "WA",
                    Subdivision{
                        name: "WA",
                        country_alpha2: Alpha2::AU,
                        code: "WA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-27.6728168), longitude: Some(121.6283098), max_latitude: Some(-13.6894901), min_latitude: Some(-35.1939944), max_longitude: Some(129.0025979), min_longitude: Some(112.92145)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Wes-Australië"), ("ar", "أستراليا الغربية"), ("az", "Qərbi Avstraliya"), ("be", "Штат Заходняя Аўстралія"), ("bg", "Западна Австралия"), ("bn", "ওয\u{9bc}েস\u{9cd}ট\u{9be}র\u{9cd}ন অস\u{9cd}ট\u{9cd}রেলিয\u{9bc}\u{9be}"), ("bs", "Zapadna Australija"), ("ca", "Austràlia Occidental"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄃\u{11127}𑄌\u{11134}𑄑\u{11133}𑄢𑄬𑄣\u{11128}𑄠"), ("ceb", "State of Western Australia"), ("cs", "Západní Austrálie"), ("cy", "Gorllewin Awstralia"), ("da", "Western Australia"), ("de", "Western Australia"), ("el", "Δυτική Αυστραλία"), ("en", "Western Australia"), ("es", "Australia Occidental"), ("et", "Lääne-Austraalia"), ("eu", "Mendebaldeko Australia"), ("fa", "استرالیای غربی"), ("fi", "Länsi-Australia"), ("fr", "Australie-Occidentale"), ("ga", "Iarthar na hAstráile"), ("gl", "Australia Occidental"), ("gu", "વ\u{ac7}સ\u{acd}ટર\u{acd}ન ઑસ\u{acd}ટ\u{acd}ર\u{ac7}લિયા"), ("he", "אוסטרליה המערבית"), ("hi", "पश\u{94d}चिमी ऑस\u{94d}ट\u{94d}र\u{947}लिया"), ("hr", "Zapadna Australija"), ("hu", "Nyugat-Ausztrália"), ("hy", "Արևմտյան Ավստրալիա"), ("id", "Australia Barat"), ("is", "Vestur-Ástralía"), ("it", "Australia Occidentale"), ("ja", "西オーストラリア州"), ("ka", "დასავლეთი ავსტრალია"), ("kn", "ಪಶ\u{ccd}ಚ\u{cbf}ಮ ಆಸ\u{ccd}ಟ\u{ccd}ರೇಲ\u{cbf}ಯಾ"), ("ko", "웨스턴오스트레일리아 주"), ("lt", "Vakarų Australija"), ("lv", "Rietumaustrālija"), ("mk", "Западна Австралија"), ("mn", "Баруун Австрали"), ("mr", "व\u{947}स\u{94d}टर\u{94d}न ऑस\u{94d}ट\u{94d}र\u{947}लिया"), ("ms", "Australia Barat"), ("my", "အနောက\u{103a}ဩစတြေးလျပြည\u{103a}နယ\u{103a}"), ("nb", "Vest-Australia"), ("ne", "पश\u{94d}चिमी अस\u{94d}ट\u{94d}र\u{947}लिया"), ("nl", "West-Australië"), ("no", "Vest-Australia"), ("pa", "ਪ\u{a71}ਛਮੀ ਆਸਟਰ\u{a47}ਲੀਆ"), ("pl", "Australia Zachodnia"), ("pt", "Austrália Ocidental"), ("ro", "Australia de Vest"), ("ru", "Западная Австралия"), ("si", "දක\u{dd4}ණ\u{dd4} ඔස\u{dca}ට\u{dca}\u{200d}රේල\u{dd2}ය\u{dcf}ව²"), ("sk", "Západná Austrália"), ("sl", "Zahodna Avstralija"), ("so", "Galbeed Australia"), ("sr", "Западна Аустралија"), ("sr_Latn", "Zapadna Australija"), ("sv", "Western Australia"), ("sw", "Australia ya Magharibi"), ("ta", "மேற\u{bcd}கு ஆஸ\u{bcd}திரேலிய\u{bbe}"), ("te", "పశ\u{c4d}చ\u{c3f}మ ఆస\u{c4d}ట\u{c4d}ర\u{c47}ల\u{c3f}య\u{c3e}"), ("th", "ร\u{e31}ฐเวสเท\u{e34}ร\u{e4c}นออสเตรเล\u{e35}ย"), ("tr", "Batı Avustralya"), ("uk", "Західна Австралія"), ("ur", "مغربی آسٹریلیا"), ("vi", "Tây Úc"), ("yue", "西澳洲"), ("yue_Hans", "西澳洲"), ("zh", "西澳大利亚州")]),
                        unofficial_name_list: ["Western Australia"].to_vec(),
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
#[cfg(feature = "au")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::AU,
        alpha3: Alpha3::AUS,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{city}} {{region_short}} {{postalcode}}\n{{country}}",
        ),
        continent: Continent::Australia,
        country_code: 61,
        currency_code: "AUD",
        gec: Some(GEC::AS),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "0011",
        ioc: Some("AUS"),
        iso_long_name: "The Commonwealth of Australia",
        iso_short_name: "Australia",
        official_language_list: ["en"].to_vec(),
        spoken_language_list: ["en"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "0",
        nationality: Some("Australian"),
        number: "036",
        postal_code: true,
        postal_code_format: Some("\\d{4}"),
        region: Some(Region::Oceania),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::AustraliaAndNewZealand),
        un_locode: "AU",
        unofficial_name_list: [
            "Australien",
            "Australie",
            "オーストラリア",
            "Australië",
            "澳洲",
        ]
        .to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Australia"),
            ("af", "Australië"),
            ("ak", "Australia"),
            ("am", "ጐፄስትሳሑ።"),
            ("an", "Australia"),
            ("ar", "أستراليا"),
            ("as", "অস\u{9cd}ট\u{9cd}ৰেলিয়\u{9be}"),
            ("ay", "Australia"),
            ("az", "Avstraliya"),
            ("ba", "Australia"),
            ("be", "Аўстралія"),
            ("bg", "Австралия"),
            ("bi", "Australia"),
            ("bn", "অস\u{9cd}ট\u{9cd}রেলিয়\u{9be}"),
            ("bn_IN", "অস\u{9cd}ট\u{9cd}রেলিয়\u{9be}"),
            ("br", "Aostralia"),
            ("bs", "Australija"),
            ("ca", "Austràlia"),
            ("ce", "Австрали"),
            ("ch", "Australia"),
            ("cs", "Austrálie"),
            ("cv", "Австрали"),
            ("cy", "Awstralia"),
            ("da", "Australien"),
            ("de", "Australien"),
            ("dv", "އ\u{7ae}ސ\u{7b0}ޓ\u{7a6}ރ\u{7aa}ލ\u{7a8}ޔ\u{7a7}"),
            ("dz", "ཨ\u{f71}ས\u{f72}་ཊ་ལ\u{f72}་ཡ།"),
            ("ee", "Australia"),
            ("el", "Αυστραλία"),
            ("en", "Australia"),
            ("eo", "Aŭstralio"),
            ("es", "Australia"),
            ("et", "Austraalia"),
            ("eu", "Australia"),
            ("fa", "استرالیا"),
            ("ff", "Australia"),
            ("fi", "Australia"),
            ("fo", "Avstralia"),
            ("fr", "Australie"),
            ("fy", "Austraalje"),
            ("ga", "An Astráil"),
            ("gl", "Australia"),
            ("gn", "Australia"),
            ("gu", "ઓસ\u{acd}ટ\u{acd}ર\u{ac7}લિયા"),
            ("gv", "Yn Austrail"),
            ("ha", "Asturaliya"),
            ("he", "אוסטרליה"),
            ("hi", "ऑस\u{94d}ट\u{94d}र\u{947}लिया"),
            ("hr", "Australija"),
            ("ht", "Ostrali"),
            ("hu", "Ausztrália"),
            ("hy", "Ավստրալիա"),
            ("ia", "Australia"),
            ("id", "Australia"),
            ("io", "Australia"),
            ("is", "Ástralía"),
            ("it", "Australia"),
            ("iu", "ᐊᔅᑦᕌᓕᐊ"),
            ("ja", "オーストラリア連邦"),
            ("ka", "ავსტრალია"),
            ("ki", "Australia"),
            ("kk", "Австралия"),
            ("kl", "Australia"),
            ("km", "អ\u{17bc}ស\u{17d2}ត\u{17d2}រាល\u{17b8}"),
            ("kn", "ಆಸ\u{ccd}ಟ\u{ccd}ರೇಲ\u{cbf}ಯ"),
            ("ko", "오스트레일리아"),
            ("ku", "Awustralya"),
            ("kv", "Австралия"),
            ("kw", "Ostrali"),
            ("ky", "Австралия"),
            ("lo", "ປະເທດອ\u{ebb}ດສະຕາລ\u{eb5}"),
            ("lt", "Australija"),
            ("lv", "Austrālija"),
            ("mi", "Ahitereiria"),
            ("mk", "Австралија"),
            ("ml", "ഓസ\u{d4d}ട\u{d4d}രേലിയ"),
            ("mn", "Австрали"),
            ("mr", "ऑस\u{94d}ट\u{94d}र\u{947}लिया"),
            ("ms", "Australia"),
            ("mt", "Awstralja"),
            ("my", "ဩစတြေးလျန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
            ("na", "Otereiriya"),
            ("nb", "Australia"),
            ("ne", "अष\u{94d}ट\u{94d}र\u{947}लिया"),
            ("nl", "Australië"),
            ("nn", "Australia"),
            ("nv", "Nahatʼeʼiitsoh Bikéyah"),
            ("oc", "Australia"),
            ("or", "ଅଷ\u{b4d}ଟ\u{b4d}ରେଲ\u{b3f}ୟ\u{b3e}"),
            ("pa", "ਅਸਟਰ\u{a47}ਲੀਆ"),
            ("pi", "आस\u{94d}ट\u{94d}र\u{947}लिया"),
            ("pl", "Australia"),
            ("ps", "آسټراليا"),
            ("pt", "Austrália"),
            ("pt_BR", "Austrália"),
            ("ro", "Australia"),
            ("ru", "Австралия"),
            ("rw", "Ositaraliya"),
            ("sc", "Austràlia"),
            ("sd", "آسٽريليا"),
            ("si", "ඔස\u{dca}ට\u{dca}\u{200d}රේල\u{dd2}ය\u{dcf}ව"),
            ("sk", "Austrália"),
            ("sl", "Avstralija"),
            ("so", "Awstraaliya"),
            ("sq", "Australi"),
            ("sr", "Аустралија"),
            ("sv", "Australien"),
            ("sw", "Australia"),
            ("ta", "ஆஸ\u{bcd}திரேலிய\u{bbe}"),
            ("te", "ఓస\u{c4d}ట\u{c4d}ర\u{c47}ల\u{c3f}య\u{c3e}"),
            ("tg", "Австралия"),
            ("th", "ออสเตรเล\u{e35}ย"),
            ("ti", "ኣውስትራልያ"),
            ("tk", "Awstraliýa"),
            ("tl", "Australya"),
            ("tr", "Avustralya"),
            ("tt", "Аустралиа"),
            ("ug", "ئاۋسترالىيە"),
            ("uk", "Австралія"),
            ("ur", "آسٹریلیا"),
            ("uz", "Avstraliya"),
            ("ve", "Australia"),
            ("vi", "Úc"),
            ("wa", "Ostraleye"),
            ("wo", "Óstraali"),
            ("xh", "Australiya"),
            ("yo", "Austrálíà"),
            ("zh_CN", "澳大利亚"),
            ("zh_HK", "澳大利亞"),
            ("zh_TW", "澳大利亞"),
            ("zu", "I-Ostreliya"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}