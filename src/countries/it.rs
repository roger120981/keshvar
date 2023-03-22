// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Italian Republic

#[cfg(all(feature = "it", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}} {{region_short}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::IT;
    pub const ALPHA3: Alpha3 = Alpha3::ITA;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 39;
    pub const CURRENCY_CODE: &str = "EUR";
    pub const GEC: Option<GEC> = Some(GEC::IT);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("ITA");
    pub const ISO_SHORT_NAME: &str = "Italy";
    pub const ISO_LONG_NAME: &str = "The Italian Republic";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["it"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["it"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[3];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9, 11];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Italian");
    pub const NUMBER: &str = "380";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthernEurope);
    pub const UN_LOCODE: &str = "IT";
    pub const UNOFFICIAL_NAME_LIST: &[&str] =
        &["Italy", "Italien", "Italie", "Italia", "イタリア", "Italië"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Italy"),
        ("af", "Italië"),
        ("ak", "Italy"),
        ("am", "ጣሑ።ን"),
        ("an", "Italy"),
        ("ar", "إيطاليا"),
        ("as", "ইট\u{9be}লি"),
        ("ay", "Italy"),
        ("az", "İtaliya"),
        ("ba", "Italy"),
        ("be", "Італія"),
        ("bg", "Италия"),
        ("bi", "Italy"),
        ("bn", "ইট\u{9be}লি"),
        ("bn_IN", "ইট\u{9be}লি"),
        ("br", "Italia"),
        ("bs", "Italija"),
        ("ca", "Itàlia"),
        ("ce", "Итали"),
        ("ch", "Italia"),
        ("cs", "Itálie"),
        ("cv", "Итали"),
        ("cy", "Yr Eidal"),
        ("da", "Italien"),
        ("de", "Italien"),
        ("dv", "އ\u{7a8}ޓ\u{7a6}ލ\u{7a9}ވ\u{7a8}ލ\u{7a7}ތ\u{7b0}"),
        ("dz", "ཨ\u{f72}་ཊ་ལ\u{f72}།"),
        ("ee", "Italy"),
        ("el", "Ιταλία"),
        ("en", "Italy"),
        ("eo", "Italio"),
        ("es", "Italia"),
        ("et", "Itaalia"),
        ("eu", "Italia"),
        ("fa", "ایتالیا"),
        ("ff", "Italiya"),
        ("fi", "Italia"),
        ("fo", "Italia"),
        ("fr", "Italie"),
        ("fy", "Itaalje"),
        ("ga", "An Iodáil"),
        ("gl", "Italia"),
        ("gn", "Italy"),
        ("gu", "ઇટાલી"),
        ("gv", "Yn Iddaal"),
        ("ha", "Italiya"),
        ("he", "איטליה"),
        ("hi", "इटली"),
        ("hr", "Italija"),
        ("ht", "Itali"),
        ("hu", "Olaszország"),
        ("hy", "Իտալիա"),
        ("ia", "Italia"),
        ("id", "Italia"),
        ("io", "Italia"),
        ("is", "Ítalía"),
        ("it", "Italia"),
        ("iu", "Italy"),
        ("ja", "イタリア"),
        ("ka", "იტალია"),
        ("ki", "Italia"),
        ("kk", "Италия"),
        ("kl", "Italy"),
        ("km", "អ\u{17ca}\u{17b8}តាល\u{17b8}"),
        ("kn", "ಇಟಲ\u{cbf}"),
        ("ko", "이탈리아"),
        ("ku", "Îtalya"),
        ("kv", "Италия"),
        ("kw", "Itali"),
        ("ky", "Италия"),
        ("lo", "ອ\u{eb4}ຕາລ\u{eb5}"),
        ("lt", "Italija"),
        ("lv", "Itālija"),
        ("mi", "Itāria"),
        ("mk", "Италија"),
        ("ml", "ഇറ\u{d4d}റലി"),
        ("mn", "Итали"),
        ("mr", "इटली"),
        ("ms", "Itali"),
        ("mt", "Italja"),
        (
            "my",
            "အ\u{102e}တလ\u{102e}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Itari"),
        ("nb", "Italia"),
        ("ne", "इटाली"),
        ("nl", "Italië"),
        ("nn", "Italia"),
        ("nv", "Doohatsʼíí Yátiʼ Dineʼé Bikéyah"),
        ("oc", "Itàlia"),
        ("or", "ଇଟ\u{b3e}ଲୀ"),
        ("pa", "ਇਟਲੀ"),
        ("pi", "इटली"),
        ("pl", "Włochy"),
        ("ps", "ایټالیه"),
        ("pt", "Itália"),
        ("pt_BR", "Itália"),
        ("ro", "Italia"),
        ("ru", "Италия"),
        ("rw", "Ubutariyani"),
        ("sc", "Itàlia"),
        ("sd", "اٽلي"),
        ("si", "ඉත\u{dcf}ල\u{dd2}ය"),
        ("sk", "Taliansko"),
        ("sl", "Italija"),
        ("so", "Talyaani"),
        ("sq", "Itali"),
        ("sr", "Италија"),
        ("sv", "Italien"),
        ("sw", "Italy"),
        ("ta", "இத\u{bcd}த\u{bbe}லி"),
        ("te", "ఇటల\u{c40}"),
        ("tg", "Италия"),
        ("th", "อ\u{e34}ตาล\u{e35}"),
        ("ti", "ኢጣልያ"),
        ("tk", "Italiýa"),
        ("tl", "Italya"),
        ("tr", "İtalya"),
        ("tt", "İталиа"),
        ("ug", "ئىتالىيە"),
        ("uk", "Італія"),
        ("ur", "اطالیہ"),
        ("uz", "Italiya"),
        ("ve", "Italy"),
        ("vi", "Ý"),
        ("wa", "Itåleye"),
        ("wo", "Itaali"),
        ("xh", "Ithali"),
        ("yo", "Itálíà"),
        ("zh_CN", "意大利"),
        ("zh_HK", "意大利"),
        ("zh_TW", "義大利"),
        ("zu", "ITaliya"),
    ];
    #[cfg(all(feature = "it", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 41.87194;
        pub const LONGITUDE: f64 = 12.56738;
        pub const MAX_LATITUDE: f64 = 47.092;
        pub const MAX_LONGITUDE: f64 = 18.7975999;
        pub const MIN_LATITUDE: f64 = 35.4897;
        pub const MIN_LONGITUDE: f64 = 6.6267201;
        pub const NORTHEAST_LATITUDE: f64 = 47.092;
        pub const NORTHEAST_LONGITUDE: f64 = 18.7975999;
        pub const SOUTHWEST_LATITUDE: f64 = 35.4897;
        pub const SOUTHWEST_LONGITUDE: f64 = 6.6267201;
    }
}
#[cfg(all(feature = "it", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 41.87194,
            longitude: 12.56738,
            max_latitude: 47.092,
            max_longitude: 18.7975999,
            min_latitude: 35.4897,
            min_longitude: 6.6267201,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 47.092,
                    longitude: 18.7975999,
                },
                southwest: CountryGeoBound {
                    latitude: 35.4897,
                    longitude: 6.6267201,
                },
            },
        }
    }
}

#[cfg(all(feature = "it", feature = "subdivisions"))]
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
                    "21",
                    Subdivision{
                        name: "21",
                        country_alpha2: Alpha2::IT,
                        code: "21",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.066667), longitude: Some(7.7), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Piëmont"), ("am", "ፕዬሞንቴ"), ("ar", "بييمونتي"), ("az", "Pyemont"), ("bg", "Пиемонт"), ("bs", "Pijemont"), ("ca", "Piemont"), ("ccp", "𑄛\u{1112d}𑄖\u{11134}𑄟\u{11127}𑄚\u{11133}𑄑\u{11134}"), ("ceb", "Piemonte"), ("cs", "Piemont"), ("cy", "Piemonte"), ("da", "Piemonte"), ("de", "Piemont"), ("el", "Πεδεμόντιο"), ("en", "Piedmont"), ("es", "Piamonte"), ("et", "Piemonte"), ("eu", "Piemonte"), ("fa", "پیمونت"), ("fi", "Piemonte"), ("fr", "Piémont"), ("ga", "Píodmant"), ("gl", "Piemonte"), ("he", "פיימונטה"), ("hi", "पिडमा\u{902}ट"), ("hr", "Pijemont"), ("hu", "Piemont"), ("hy", "Պիեմոնտ"), ("id", "Piemonte"), ("is", "Fjallaland"), ("it", "Piemonte"), ("ja", "ピエモンテ州"), ("jv", "Piemonte"), ("ka", "პიემონტი"), ("kk", "Пьемонт"), ("km", "ខេត\u{17d2}តជើងភ\u{17d2}ន\u{17c6}"), ("ko", "피에몬테 주"), ("lt", "Pjemontas"), ("lv", "Pjemonta"), ("mk", "Пиемонт"), ("mr", "प\u{94d}यिमा\u{901}त"), ("ms", "Piemonte"), ("nb", "Piemonte"), ("ne", "पियडमोन\u{94d}ट"), ("nl", "Piemonte"), ("no", "Piemonte"), ("pa", "ਪੀਏਮ\u{a4b}\u{a02}ਤ\u{a47}"), ("pl", "Piemont"), ("pt", "Piemonte"), ("ro", "Piemont"), ("ru", "Пьемонт"), ("sk", "Piemont"), ("sl", "Piemont"), ("sq", "Piemonti"), ("sr", "Пијемонт"), ("sr_Latn", "Pijemont"), ("sv", "Piemonte"), ("sw", "Piemonte"), ("ta", "பியத\u{bcd}ம\u{bbe}ந\u{bcd}து"), ("th", "แคว\u{e49}นป\u{e35}เยมอนเต"), ("tr", "Piyemonte"), ("uk", "Пʼємонт"), ("ur", "پیعیمونتے"), ("vi", "Piemonte"), ("yue", "皮耶蒙"), ("yue_Hans", "皮耶蒙"), ("zh", "皮埃蒙特")]),
                        unofficial_name_list: ["Piemonte"].to_vec(),
                    }
                ),
                (
                    "23",
                    Subdivision{
                        name: "23",
                        country_alpha2: Alpha2::IT,
                        code: "23",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.746944), longitude: Some(7.439167), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "ቫሌ ዳኦስታ"), ("ar", "وادي أوستا"), ("az", "Aosta Valley"), ("bn", "ভ\u{9be}ল\u{9cd}লে দি অস\u{9cd}ট\u{9be}"), ("bs", "Valle d’Aosta"), ("ca", "Vall d’Aosta"), ("ccp", "𑄃\u{11127}𑄠\u{1112e}𑄌\u{11134}𑄑 𑄞𑄬𑄣\u{11128}"), ("ceb", "Walog sa Aosta"), ("cs", "Valle d’Aosta"), ("cy", "Valle d’Aosta"), ("da", "Valle d’Aosta"), ("de", "Aostatal"), ("el", "Κοιλάδα της Αόστα"), ("en", "Aosta Valley"), ("es", "Valle de Aosta"), ("et", "Valle d’Aosta"), ("eu", "Aostako Harana"), ("fa", "واله دائوستا"), ("fi", "Aostanlaakso"), ("fr", "Vallée d’Aoste"), ("ga", "Valle d’Aosta"), ("gl", "Val de Aosta"), ("gu", "વ\u{ac7}લ દ\u{ac7} ઑઓસ\u{acd}ટા"), ("he", "ואל ד׳אוסטה"), ("hi", "आओस\u{94d}ता घाटी"), ("hr", "Valle d’Aosta"), ("hu", "Valle d’Aosta"), ("id", "Lembah Aosta"), ("is", "Ágústudalur"), ("it", "Valle d’Aosta"), ("ja", "ヴァッレ・ダオスタ州"), ("jv", "Lembah Aosta"), ("kk", "Валле-д’Аоста"), ("kn", "ವ\u{ccd}ಯಾಲ\u{ccd} ಡ\u{cbf} ಅಯೋಸ\u{ccd}ತಾ"), ("ko", "발레다오스타 주"), ("lt", "Aostos slėnis"), ("lv", "Valle d’Aosta"), ("mk", "Аостинска Долина"), ("ml", "അയോസ\u{d4d}റ\u{d4d}റ വ\u{d3e}ലി"), ("mr", "व\u{94d}हाल\u{947} दाओस\u{94d}ता"), ("ms", "Lembah Aosta"), ("nb", "Aostadalen"), ("nl", "Valle d’Aosta"), ("no", "Aostadalen"), ("pa", "ਆਓਸਤਾ ਘਾਟੀ"), ("pl", "Dolina Aosty"), ("pt", "Vale de Aosta"), ("ro", "Valle d’Aosta"), ("si", "වැලේ ඩ\u{dd2} අඔස\u{dca}ට\u{dcf}"), ("sk", "Valle d’Aosta"), ("sl", "Dolina Aoste"), ("sq", "Lugina e Aostës"), ("sr", "Долина Аосте"), ("sr_Latn", "Dolina Aoste"), ("sv", "Aostadalen"), ("sw", "Valle d’Aosta"), ("ta", "அயோஸ\u{bcd}ட\u{bbe} பள\u{bcd}ளத\u{bcd}த\u{bbe}க\u{bcd}கு"), ("te", "వల\u{c4d}ల\u{c47} డ\u{c3f} అవ\u{c4a}స\u{c4d}త\u{c3e}"), ("th", "แคว\u{e49}นปกครองตนเองว\u{e31}ลเลดาออสตา"), ("tr", "Aosta Vadisi"), ("uk", "Валле-дʼАоста"), ("ur", "وادی آوستہ"), ("vi", "Thung lũng Aosta"), ("yue", "歐斯達山谷大區"), ("yue_Hans", "欧斯达山谷大区"), ("zh", "瓦莱达奥斯塔")]),
                        unofficial_name_list: ["Aosta Valley", "Val d'Aosta", "Val d'Aoste", "Val d'Osta", "Val d'Outa"].to_vec(),
                    }
                ),
                (
                    "25",
                    Subdivision{
                        name: "25",
                        country_alpha2: Alpha2::IT,
                        code: "25",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.585556), longitude: Some(9.930278), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Lombardye"), ("am", "ሎምባርዲያ"), ("ar", "لومبارديا"), ("az", "Lombardiya"), ("be", "Ламбардыя"), ("bg", "Ломбардия"), ("bn", "লোম\u{9cd}ব\u{9be}র\u{9cd}ডি"), ("bs", "Lombardija"), ("ca", "Llombardia"), ("ccp", "𑄣\u{1112e}𑄟\u{11134}𑄝𑄢\u{11134}𑄓\u{11128}"), ("ceb", "Lombardia"), ("cs", "Lombardie"), ("cy", "Lombardia"), ("da", "Lombardiet"), ("de", "Lombardei"), ("el", "Λομβαρδία"), ("en", "Lombardy"), ("es", "Lombardía"), ("et", "Lombardia"), ("eu", "Lombardia"), ("fa", "لمباردی"), ("fi", "Lombardia"), ("fr", "Lombardie"), ("ga", "An Lombaird"), ("gl", "Lombardía - Lombardia"), ("he", "לומברדיה"), ("hi", "लोम\u{94d}बार\u{94d}डी"), ("hr", "Lombardija"), ("hu", "Lombardia"), ("hy", "Լոմբարդիա"), ("id", "Lombardia"), ("is", "Langbarðaland"), ("it", "Lombardia"), ("ja", "ロンバルディア州"), ("jv", "Lombardia"), ("ka", "ლომბარდია"), ("kk", "Ломбардия"), ("ko", "롬바르디아 주"), ("lt", "Lombardija"), ("lv", "Lombardija"), ("mk", "Ломбардија"), ("mn", "Ломбард муж"), ("mr", "लो\u{902}बार\u{94d}दिया"), ("ms", "Lombardy"), ("nb", "Lombardia"), ("nl", "Lombardije"), ("no", "Lombardia"), ("pa", "ਲ\u{a4b}\u{a02}ਬਾਰਦੀਆ"), ("pl", "Lombardia"), ("pt", "Lombardia"), ("ro", "Lombardia"), ("ru", "Ломбардия"), ("sk", "Lombardsko"), ("sl", "Lombardija"), ("so", "Lombardia"), ("sq", "Lombardia"), ("sr", "Ломбардија"), ("sr_Latn", "Lombardija"), ("sv", "Lombardiet"), ("sw", "Lombardia"), ("ta", "லோம\u{bcd}ப\u{bbe}ர\u{bcd}டி"), ("th", "แคว\u{e49}นลอมบาร\u{e4c}เด\u{e35}ย"), ("tr", "Lombardiya"), ("uk", "Ломбардія"), ("ur", "لومباردیہ"), ("vi", "Lombardia"), ("yue", "倫巴第大區"), ("yue_Hans", "伦巴第大区")]),
                        unofficial_name_list: ["Lombardy", "Lombardéa"].to_vec(),
                    }
                ),
                (
                    "32",
                    Subdivision{
                        name: "32",
                        country_alpha2: Alpha2::IT,
                        code: "32",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.066667), longitude: Some(11.116667), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Trentino-Suid-Tirool"), ("am", "ትሬንቲኖ-ደቡብ ቲሮል"), ("ar", "ترينتينو ألتو أديجي"), ("az", "Trentino-Alto Adice"), ("be", "Трэнціна-Альта-Адыджэ"), ("bg", "Трентино-Южен Тирол"), ("bs", "Trentino-Južni Tirol"), ("ca", "Trentino - Tirol del Sud"), ("ccp", "𑄑\u{11133}𑄢𑄬𑄚\u{11134}𑄑\u{11128}𑄚\u{1112e}-𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄑\u{1112d}𑄢\u{1112e}𑄣\u{11134}"), ("ceb", "Trentino-Alto Adige"), ("cs", "Tridentsko-Horní Adiže"), ("cy", "Trentino-Alto Adige"), ("da", "Trentino-Sydtyrol"), ("de", "Trentino-Südtirol"), ("el", "Τρεντίνο-Άλτο Άντιτζε"), ("en", "Trentino-South Tyrol"), ("es", "Trentino-Alto Adigio"), ("et", "Trentino-Alto Adige"), ("eu", "Trentino-Adige Garaia"), ("fa", "ترنتینو التو آدیجه"), ("fi", "Trentino-Alto Adige"), ("fr", "Trentin-Haut-Adige"), ("ga", "Trentino-Tirol Theas"), ("gl", "Trentino-Tirol do Sur"), ("he", "טרנטינו - אלטו אדיג׳ה"), ("hr", "Trentino-Južni Tirol"), ("hu", "Trentino-Alto Adige"), ("hy", "Տրենտինո Ալտո Ադիջե"), ("id", "Trentino-Alto Adige"), ("is", "Trentínó-Suður-Týról"), ("it", "Trentino-Alto Adige"), ("jv", "Trentino-Alto Adige"), ("ka", "ტრენტინო-ალტო-ადიჯე"), ("kk", "Трентино — Альто-Адидже"), ("ko", "트렌티노알토아디제 주"), ("lt", "Trentinas-Alto Adidžė"), ("lv", "Trentīno-Alto Adidže"), ("mk", "Трентино-Јужен Тирол"), ("ml", "ട\u{d4d}രെന\u{d4d}റിനോ ആൾട\u{d4d}ടോ അഡിജേ"), ("mr", "त\u{94d}र\u{947}न\u{94d}तिनो-आल\u{94d}तो अदिज\u{947}"), ("ms", "Trentino-Alto Adige"), ("nb", "Trentino-Alto Adige"), ("nl", "Trentino-Zuid-Tirol"), ("no", "Trentino-Alto Adige"), ("pl", "Trydent-Górna Adyga"), ("pt", "Trentino-Alto Ádige"), ("ro", "Trentino-Tirolul de Sud"), ("ru", "Трентино — Альто-Адидже"), ("sk", "Tridentsko-Horná Adiža"), ("sl", "Trentinsko - Zgornje Poadižje"), ("sq", "Trentino-Tiroli Jugor"), ("sr", "Трентино-Јужни Тирол"), ("sr_Latn", "Trentino-Južni Tirol"), ("sv", "Trentino-Alto Adige"), ("sw", "Trentino-Alto Adige"), ("th", "แคว\u{e49}นปกครองตนเองเตรนต\u{e35}โน-อ\u{e31}ลโตอาด\u{e35}เจ"), ("tr", "Trentino-Alto Adige"), ("uk", "Трентіно-Альто-Адідже"), ("ur", "ترینتینو جنوبی ٹائرول"), ("vi", "Trentino-Alto Adige/Südtirol"), ("yue", "圖靈天諾－南鐵羅"), ("yue_Hans", "图灵天诺－南铁罗"), ("zh", "特伦蒂诺-上阿迪杰")]),
                        unofficial_name_list: ["Trentino-South Tyrol"].to_vec(),
                    }
                ),
                (
                    "34",
                    Subdivision{
                        name: "34",
                        country_alpha2: Alpha2::IT,
                        code: "34",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.733333), longitude: Some(11.85), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Veneto"), ("am", "ቨኔቶ"), ("ar", "فينيتو"), ("az", "Veneto"), ("be", "Венета"), ("bg", "Венето"), ("bs", "Veneto"), ("ca", "Vèneto"), ("ccp", "𑄞𑄬𑄚𑄬𑄑\u{1112e}"), ("ceb", "Veneto"), ("cs", "Benátsko"), ("cy", "Veneto"), ("da", "Veneto"), ("de", "Venetien"), ("el", "Βένετο"), ("en", "Veneto"), ("es", "Véneto"), ("et", "Veneto"), ("eu", "Veneto"), ("fa", "ونتو"), ("fi", "Veneto"), ("fr", "Vénétie"), ("ga", "Veneto"), ("gl", "Véneto - Veneto"), ("he", "ונטו"), ("hi", "व\u{948}न\u{947}तो"), ("hr", "Veneto"), ("hu", "Veneto"), ("hy", "Վենետո"), ("id", "Veneto"), ("is", "Venetó"), ("it", "Veneto"), ("ja", "ヴェネト州"), ("jv", "Veneto"), ("ka", "ვენეტო"), ("kk", "Венето"), ("ko", "베네토 주"), ("lt", "Venetas"), ("lv", "Veneto"), ("mk", "Венето"), ("mr", "व\u{94d}ह\u{947}न\u{947}तो"), ("ms", "Veneto"), ("nb", "Veneto"), ("nl", "Veneto"), ("no", "Veneto"), ("pa", "ਵ\u{a48}ਨ\u{a47}ਤ\u{a4b}"), ("pl", "Wenecja Euganejska"), ("pt", "Véneto"), ("ro", "Veneto"), ("ru", "Венеция"), ("sk", "Benátsko"), ("sl", "Benečija"), ("sq", "Veneto"), ("sr", "Венето"), ("sr_Latn", "Veneto"), ("sv", "Veneto"), ("sw", "Veneto"), ("th", "แคว\u{e49}นเวเนโต"), ("tr", "Veneto"), ("uk", "Венето"), ("ur", "وینیتو"), ("vi", "Veneto"), ("yue", "威尼托"), ("yue_Hans", "威尼托")]),
                        unofficial_name_list: ["Venetia", "Vèneto"].to_vec(),
                    }
                ),
                (
                    "36",
                    Subdivision{
                        name: "36",
                        country_alpha2: Alpha2::IT,
                        code: "36",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.636111), longitude: Some(13.804167), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Friuli-Venezia Giulia"), ("am", "ፍሪዩሊ ቬነጽያ ጁልያ"), ("ar", "فريولي فينيتسيا جوليا"), ("az", "Firulli-Venesiya-Culiya"), ("be", "Фрыулі-Венецыя-Джулія"), ("bg", "Фриули - Венеция Джулия"), ("bs", "Furlanija-Julijska Krajina"), ("ca", "Friül - Venècia Júlia"), ("ccp", "𑄜\u{11133}𑄢\u{1112d}𑄅\u{1112a}𑄣\u{11128}𑄠-𑄞𑄬𑄚𑄬𑄎\u{11128}𑄠 𑄉\u{1112d}𑄅\u{1112a}𑄣\u{11128}𑄠"), ("ceb", "Friuli-Venezia Giulia"), ("cs", "Furlansko-Julské Benátsko"), ("cy", "Friuli-Venezia Giulia"), ("da", "Friuli-Venezia Giulia"), ("de", "Friaul-Julisch Venetien"), ("el", "Φρίουλι-Βενέτσια Τζούλια"), ("en", "Friuli–Venezia Giulia"), ("es", "Friuli-Venecia Julia"), ("et", "Friuli-Venezia Giulia"), ("eu", "Friuli-Venezia-Julia"), ("fa", "فریولی ونتزیا جولیا"), ("fi", "Friuli-Venezia Giulia"), ("fr", "Frioul-Vénétie julienne"), ("ga", "Friuli-Venezia Giulia"), ("gl", "Friuli-Venezia Giulia"), ("he", "פריולי-ונציה ג׳וליה"), ("hr", "Furlanija-Julijska krajina"), ("hu", "Friuli-Venezia Giulia"), ("hy", "Ֆրիուլի Վենեցիա Ջուլյա"), ("id", "Friuli–Venezia Giulia"), ("is", "Friúlí"), ("it", "Friuli-Venezia Giulia"), ("jv", "Friuli-Venezia Giulia"), ("ka", "ფრიული-ვენეცია-ჯულია"), ("kk", "Фриули-Венеция-Джулия"), ("ko", "프리울리베네치아줄리아 주"), ("lt", "Friulis-Venecija Džulija"), ("lv", "Friuli-Venēcija Džūlija"), ("mk", "Фурланија-Јулиска краина"), ("ml", "ഫ\u{d4d}രിയ\u{d42}ളി-വെനേസിയ ജിയ\u{d42}ളിയ"), ("mr", "फ\u{94d}र\u{941}ली-व\u{94d}ह\u{947}न\u{947}झिया ज\u{941}लिया"), ("ms", "Friuli-Venezia Giulia"), ("nb", "Friuli-Venezia Giulia"), ("nl", "Friuli-Venezia Giulia"), ("no", "Friuli-Venezia Giulia"), ("pl", "Friuli-Wenecja Julijska"), ("pt", "Friul-Veneza Júlia"), ("ro", "Friuli-Veneția Giulia"), ("ru", "Фриули-Венеция-Джулия"), ("sk", "Furlansko-Júlske Benátky"), ("sl", "Furlanija - Julijska krajina"), ("sq", "Friuli-Venecia Xhulia"), ("sr", "Фурланија-Јулијска крајина"), ("sr_Latn", "Furlanija-Julijska krajina"), ("sv", "Friuli-Venezia Giulia"), ("sw", "Friuli-Venezia Giulia"), ("th", "แคว\u{e49}นปกครองตนเองฟร\u{e35}ย\u{e39}ล\u{e35}-เวเน\u{e47}ตเซ\u{e35}ยจ\u{e39}เล\u{e35}ย"), ("tr", "Friuli-Venezia Giulia"), ("uk", "Фріулі-Венеція-Джулія"), ("ur", "فریولی وینیزیا جولیا"), ("vi", "Friuli–Venezia Giulia"), ("yue", "傅遼利－威尼斯朱利亞"), ("yue_Hans", "傅辽利－威尼斯朱利亚")]),
                        unofficial_name_list: ["Friuli-Venezia Giulia", "Friûl Vignesie Julie", "Furlanija Julijska krajina"].to_vec(),
                    }
                ),
                (
                    "42",
                    Subdivision{
                        name: "42",
                        country_alpha2: Alpha2::IT,
                        code: "42",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.45), longitude: Some(8.766667), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ligurië"), ("am", "ሊጉርያ"), ("ar", "ليغوريا"), ("az", "Liquriya"), ("be", "Лігурыя"), ("bg", "Лигурия"), ("bs", "Ligurija"), ("ca", "Ligúria"), ("ccp", "𑄣\u{11128}𑄉\u{1112a}𑄢\u{11128}𑄠"), ("ceb", "Liguria"), ("cs", "Ligurie"), ("cy", "Liguria"), ("da", "Liguria"), ("de", "Ligurien"), ("el", "Λιγουρία"), ("en", "Liguria"), ("es", "Liguria"), ("et", "Liguuria"), ("eu", "Liguria"), ("fa", "لیگوریا"), ("fi", "Liguria"), ("fr", "Ligurie"), ("ga", "Liguria"), ("gl", "Liguria"), ("he", "ליגוריה"), ("hi", "लिग\u{941}रिया"), ("hr", "Ligurija"), ("hu", "Liguria"), ("hy", "Լիգուրիա"), ("id", "Liguria"), ("is", "Lígúría"), ("it", "Liguria"), ("ja", "リグーリア州"), ("jv", "Liguria"), ("ka", "ლიგურია"), ("kk", "Лигурия"), ("ko", "리구리아 주"), ("lt", "Ligūrija"), ("lv", "Ligūrija"), ("mk", "Лигурија"), ("mr", "लिग\u{941}रिया"), ("ms", "Liguria"), ("nb", "Liguria"), ("nl", "Ligurië"), ("no", "Liguria"), ("pa", "ਲਿਗ\u{a42}ਰੀਆ"), ("pl", "Liguria"), ("pt", "Ligúria"), ("ro", "Liguria"), ("ru", "Лигурия"), ("sk", "Ligúria"), ("sl", "Ligurija"), ("sq", "Liguria"), ("sr", "Лигурија"), ("sr_Latn", "Ligurija"), ("sv", "Ligurien"), ("sw", "Liguria"), ("ta", "இலிகுரிய\u{bbe}"), ("th", "แคว\u{e49}นล\u{e35}ก\u{e39}เร\u{e35}ย"), ("tr", "Ligurya"), ("uk", "Лігурія"), ("ur", "لیگوریا"), ("vi", "Liguria"), ("yue", "列古利亞"), ("yue_Hans", "列古利亚"), ("zh", "利古里亞")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "45",
                    Subdivision{
                        name: "45",
                        country_alpha2: Alpha2::IT,
                        code: "45",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.510556), longitude: Some(10.956944), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Emilia-Romagna"), ("am", "ኤሚልያ-ሮማኛ"), ("ar", "إميليا-رومانيا"), ("az", "Emiliya-Romanya"), ("be", "Эмілія-Раманья"), ("bg", "Емилия-Романя"), ("bs", "Emilia-Romagna"), ("ca", "Emília-Romanya"), ("ccp", "𑄃\u{11128}𑄟\u{11128}𑄣\u{11128}𑄠-𑄢\u{1112e}𑄟𑄇\u{11134}𑄚"), ("ceb", "Emilia-Romagna"), ("cs", "Emilia-Romagna"), ("cy", "Emilia-Romagna"), ("da", "Emilia-Romagna"), ("de", "Emilia-Romagna"), ("el", "Εμίλια-Ρομάνια"), ("en", "Emilia-Romagna"), ("es", "Emilia-Romaña"), ("et", "Emilia Romagna"), ("eu", "Emilia-Romagna"), ("fa", "امیلیا-رومانیا"), ("fi", "Emilia-Romagna"), ("fr", "Émilie-Romagne"), ("ga", "Emilia-Romagna"), ("gl", "Emilia Romaña - Emilia-Romagna"), ("he", "אמיליה-רומאניה"), ("hi", "एमीलिया-रोमाञा"), ("hr", "Emilia-Romagna"), ("hu", "Emilia-Romagna"), ("hy", "Էմիլիա Ռոմանիա"), ("id", "Emilia–Romagna"), ("is", "Emilía-Rómanja"), ("it", "Emilia-Romagna"), ("jv", "Emilia-Romagna"), ("ka", "ემილია-რომანია"), ("kk", "Эмилия-Романья"), ("ko", "에밀리아로마냐 주"), ("lt", "Emilija-Romanija"), ("lv", "Emīlija-Romanja"), ("mk", "Емилија-Ромања"), ("mr", "एमिलिया-रोमान\u{94d}या"), ("ms", "Emilia-Romagna"), ("nb", "Emilia-Romagna"), ("nl", "Emilia-Romagna"), ("no", "Emilia-Romagna"), ("pa", "ਏਮੀਲੀਆ-ਰ\u{a4b}ਮਾਞਾ"), ("pl", "Emilia-Romania"), ("pt", "Emília-Romanha"), ("ro", "Emilia-Romagna"), ("ru", "Эмилия-Романья"), ("sk", "Emilia-Romagna"), ("sl", "Emilija - Romanja"), ("sq", "Emilia-Romanja"), ("sr", "Емилија-Ромања"), ("sr_Latn", "Emilija-Romanja"), ("sv", "Emilia-Romagna"), ("sw", "Emilia-Romagna"), ("ta", "எமிலிய\u{bbe}-ரோம\u{bbe}ஞ\u{bbe}"), ("th", "แคว\u{e49}นเอม\u{e35}เล\u{e35}ย-โรม\u{e31}ญญา"), ("tr", "Emilia-Romagna"), ("uk", "Емілія-Романья"), ("ur", "ایمیلیا رومانیا"), ("vi", "Emilia-Romagna"), ("yue", "愛美利亞－羅曼尼亞"), ("yue_Hans", "爱美利亚－罗曼尼亚"), ("zh", "艾米利亚-罗马涅")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "52",
                    Subdivision{
                        name: "52",
                        country_alpha2: Alpha2::IT,
                        code: "52",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.771389), longitude: Some(11.254167), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Toskane"), ("am", "ቶስካና"), ("ar", "توسكانا"), ("az", "Toskana"), ("be", "Таскана"), ("bg", "Тоскана"), ("bn", "ট\u{9be}স\u{9cd}ক\u{9be}নি"), ("bs", "Toskana"), ("ca", "Toscana"), ("ccp", "𑄑\u{1112a}𑄌\u{11134}𑄇𑄬𑄚\u{11129}"), ("ceb", "Toscana"), ("cs", "Toskánsko"), ("cy", "Toscana"), ("da", "Toscana"), ("de", "Toskana"), ("el", "Τοσκάνη"), ("en", "Tuscany"), ("es", "Toscana"), ("et", "Toscana"), ("eu", "Toskana"), ("fa", "توسکانی"), ("fi", "Toscana"), ("fr", "Toscane"), ("ga", "An Tuscáin"), ("gl", "Toscana"), ("he", "טוסקנה"), ("hi", "टस\u{94d}कनी"), ("hr", "Toskana"), ("hu", "Toszkána"), ("hy", "Տոսկանա"), ("id", "Toscana"), ("is", "Toskana"), ("it", "Toscana"), ("ja", "トスカーナ州"), ("jv", "Toscana"), ("ka", "ტოსკანა"), ("kk", "Тоскана"), ("ko", "토스카나 주"), ("lt", "Toskana"), ("lv", "Toskāna"), ("mk", "Тоскана"), ("ml", "ടസ\u{d4d}കനി"), ("mr", "तोस\u{94d}काना"), ("ms", "Tuscany"), ("nb", "Toscana"), ("nl", "Toscane"), ("no", "Toscana"), ("pa", "ਤ\u{a4b}ਸਕਾਨਾ"), ("pl", "Toskania"), ("pt", "Toscana"), ("ro", "Toscana"), ("ru", "Тоскана"), ("sk", "Toskánsko siena"), ("sl", "Toskana"), ("sq", "toskana"), ("sr", "Тоскана"), ("sr_Latn", "Toskana"), ("sv", "Toscana"), ("sw", "Toscana"), ("ta", "டசுக\u{bcd}கனி"), ("th", "แคว\u{e49}นตอสคานา"), ("tr", "Toskana"), ("uk", "Тоскана"), ("ur", "تسکانہ"), ("vi", "Toscana"), ("yue", "陀斯卡拿"), ("yue_Hans", "陀斯卡拿"), ("zh", "托斯卡纳")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "55",
                    Subdivision{
                        name: "55",
                        country_alpha2: Alpha2::IT,
                        code: "55",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.1121), longitude: Some(12.3888), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Umbrië"), ("am", "ኡምብሪያ"), ("ar", "أومبريا"), ("az", "Umbriya"), ("be", "Умбрыя"), ("bg", "Умбрия"), ("bs", "Umbrija"), ("ca", "Úmbria"), ("ccp", "𑄃𑄟\u{11134}𑄝\u{11133}𑄢\u{11128}𑄠"), ("ceb", "Umbria"), ("cs", "Umbrie"), ("cy", "Umbria"), ("da", "Umbria"), ("de", "Umbrien"), ("el", "Ούμπρια"), ("en", "Umbria"), ("es", "Umbría"), ("et", "Umbria"), ("eu", "Umbria"), ("fa", "اومبریا"), ("fi", "Umbria"), ("fr", "Ombrie"), ("ga", "Umbria"), ("gl", "Umbría - Umbria"), ("he", "אומבריה"), ("hr", "Umbrija"), ("hu", "Umbria"), ("hy", "Ումբրիա"), ("id", "Umbra"), ("is", "Úmbría"), ("it", "Umbria"), ("ja", "ウンブリア州"), ("jv", "Umbria"), ("ka", "უმბრია"), ("kk", "Умбрия"), ("ko", "움브리아 주"), ("lt", "Umbrija"), ("lv", "Umbrija"), ("mk", "Умбрија"), ("mr", "अ\u{902}ब\u{94d}रिया"), ("ms", "Umbria"), ("nb", "Umbria"), ("nl", "Umbrië"), ("no", "Umbria"), ("pa", "ਊ\u{a02}ਬਰੀਆ"), ("pl", "Umbria"), ("pt", "Úmbria"), ("ro", "Umbria"), ("ru", "Умбрия"), ("sk", "Umbria"), ("sl", "Umbrija"), ("sq", "Umbria"), ("sr", "Умбрија"), ("sr_Latn", "Umbrija"), ("sv", "Umbrien"), ("sw", "Umbria"), ("th", "แคว\u{e49}นอ\u{e38}มเบร\u{e35}ย"), ("tr", "Umbria"), ("uk", "Умбрія"), ("ur", "امبریا"), ("vi", "Umbria"), ("yue", "翁布利亞"), ("yue_Hans", "翁布利亚"), ("zh", "翁布里亚")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "57",
                    Subdivision{
                        name: "57",
                        country_alpha2: Alpha2::IT,
                        code: "57",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.616667), longitude: Some(13.516667), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "ማርኬ"), ("ar", "ماركي"), ("be", "Марке"), ("bg", "Марке"), ("bs", "Marche"), ("ca", "Marques"), ("ccp", "𑄟𑄢\u{11134}𑄌𑄬"), ("ceb", "Marche"), ("cs", "Marche"), ("cy", "Marche"), ("da", "Marche"), ("de", "Marken"), ("el", "Μάρκε"), ("en", "Marche"), ("es", "Marcas"), ("et", "Marche"), ("eu", "Markak"), ("fa", "مارکه"), ("fi", "Marche"), ("fr", "Marches"), ("ga", "Marche"), ("gl", "Marche"), ("he", "מארקה"), ("hi", "मार\u{94d}श"), ("hr", "Marke"), ("hu", "Marche"), ("hy", "Մարկե"), ("id", "Marche"), ("is", "Marke"), ("it", "Marche"), ("ja", "マルケ州"), ("jv", "Marche"), ("ka", "მარკე"), ("ko", "마르케 주"), ("lt", "Markė"), ("lv", "Marke"), ("mk", "Марке"), ("mr", "मार\u{94d}क\u{947}"), ("ms", "Marche"), ("nb", "Marche"), ("nl", "Marche"), ("no", "Marche"), ("pa", "ਮਾਰਕ\u{a47}"), ("pl", "Marche"), ("pt", "Marcas"), ("ro", "Marche"), ("ru", "Марке"), ("sk", "Marky"), ("sl", "Marke"), ("sq", "Marke"), ("sr", "Марке"), ("sr_Latn", "Marke"), ("sv", "Marche"), ("sw", "Marche"), ("th", "แคว\u{e49}นมาร\u{e4c}เค"), ("tr", "Marche"), ("uk", "Марке"), ("ur", "مارکے"), ("vi", "Marche"), ("yue", "馬其"), ("yue_Hans", "马其")]),
                        unofficial_name_list: ["Marches"].to_vec(),
                    }
                ),
                (
                    "62",
                    Subdivision{
                        name: "62",
                        country_alpha2: Alpha2::IT,
                        code: "62",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.6552), longitude: Some(12.9896), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Latium"), ("am", "ላጺዮ"), ("ar", "لاتسيو"), ("az", "Latsio"), ("be", "Лацыа"), ("bg", "Лацио"), ("bs", "Lazio"), ("ca", "Laci"), ("ccp", "𑄣𑄎\u{11128}𑄃\u{1112e}"), ("ceb", "Lacio"), ("cs", "Lazio"), ("cy", "Lazio"), ("da", "Lazio"), ("de", "Latium"), ("el", "Λάτιο"), ("en", "Lazio"), ("es", "Lacio"), ("et", "Lazio"), ("eu", "Lazio"), ("fa", "لاتزیو"), ("fi", "Lazio"), ("fr", "Latium"), ("ga", "Lazio"), ("gl", "Lazio"), ("he", "לאציו"), ("hi", "लात\u{94d}सियो"), ("hr", "Lacij"), ("hu", "Lazio"), ("hy", "Լացիո"), ("id", "Latium"), ("is", "Latíum"), ("it", "Lazio"), ("ja", "ラツィオ州"), ("jv", "Latium"), ("ka", "ლაციო"), ("kk", "Лацио"), ("ko", "라치오 주"), ("lt", "Lacijus"), ("lv", "Lacio"), ("mk", "Лацио"), ("mn", "Лацио муж"), ("mr", "लात\u{94d}सियो"), ("ms", "Latium"), ("nb", "Latium"), ("nl", "Lazio"), ("no", "Latium"), ("pa", "ਲਾਤਸੀਓ"), ("pl", "Lacjum"), ("pt", "Lácio"), ("ro", "Lazio"), ("ru", "Лацио"), ("sk", "Lazio"), ("sl", "Lacij"), ("sq", "Lacio"), ("sr", "Лацио"), ("sr_Latn", "Lacio"), ("sv", "Lazio"), ("sw", "Lazio"), ("th", "แคว\u{e49}นล\u{e31}ตซ\u{e35}โย"), ("tr", "Lazio"), ("uk", "Лаціо"), ("ur", "لازیو"), ("vi", "Lazio"), ("yue", "拉素"), ("yue_Hans", "拉素"), ("zh", "拉齐奥")]),
                        unofficial_name_list: ["Latium"].to_vec(),
                    }
                ),
                (
                    "65",
                    Subdivision{
                        name: "65",
                        country_alpha2: Alpha2::IT,
                        code: "65",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Abruzze"), ("am", "አብሩጾ"), ("ar", "أبروتسو"), ("az", "Abruzzo"), ("be", "Абруца"), ("bg", "Абруцо"), ("bs", "Abruzzo"), ("ca", "Abruços"), ("ccp", "𑄃𑄝\u{11133}𑄢\u{1112a}𑄎\u{1112e}"), ("ceb", "Abruzzo"), ("cs", "Abruzzo"), ("cy", "Abruzzo"), ("da", "Abruzzo"), ("de", "Abruzzen"), ("el", "Αμπρούτσο"), ("en", "Abruzzo"), ("es", "Abruzos"), ("et", "Abruzzo"), ("eu", "Abruzzo"), ("fa", "ابروتزو"), ("fi", "Abruzzo"), ("fr", "Abruzzes"), ("ga", "Abruzzo"), ("gl", "Abruzzo"), ("he", "אברוצו"), ("hi", "आब\u{94d}र\u{941}त\u{94d}सो"), ("hr", "Abruzzo"), ("hu", "Abruzzo"), ("hy", "Աբրուցցո"), ("id", "Abruzzo"), ("is", "Abrútsi"), ("it", "Abruzzo"), ("ja", "アブルッツォ州"), ("jv", "Abruzzo"), ("ka", "აბრუცი"), ("kk", "Абруццо"), ("ko", "아브루초 주"), ("lt", "Abrucai"), ("lv", "Abruco"), ("mk", "Абруцо"), ("mr", "आब\u{94d}र\u{941}त\u{94d}सो"), ("ms", "Abruzzo"), ("nb", "Abruzzo"), ("nl", "Abruzzen"), ("no", "Abruzzo"), ("pa", "ਆਬਰ\u{a42}ਤਸ\u{a4b}"), ("pl", "Abruzja"), ("pt", "Abruzos"), ("ro", "Abruzzo"), ("ru", "Абруццо"), ("sk", "Abruzzy"), ("sl", "Abruci"), ("sq", "Abruco"), ("sr", "Абруцо"), ("sr_Latn", "Abruco"), ("sv", "Abruzzo"), ("sw", "Abruzzo"), ("th", "แคว\u{e49}นอาบร\u{e38}ซโซ"), ("tr", "Abruzzo"), ("uk", "Абруццо"), ("ur", "آبروزو"), ("vi", "Abruzzo"), ("yue", "阿布魯佐"), ("yue_Hans", "阿布鲁佐"), ("zh", "阿布鲁佐")]),
                        unofficial_name_list: ["Abbrèzze", "Abbrìzze", "Abbrùzze", "Abbrùzzu", "Abruzzese", "Abruzzi"].to_vec(),
                    }
                ),
                (
                    "67",
                    Subdivision{
                        name: "67",
                        country_alpha2: Alpha2::IT,
                        code: "67",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.6997), longitude: Some(14.6111), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "ሞሊዜ"), ("ar", "موليزي"), ("be", "Малізэ"), ("bg", "Молизе"), ("bs", "Molise"), ("ca", "Molise"), ("ccp", "𑄟\u{11127}𑄣\u{1112d}𑄌\u{11134}"), ("ceb", "Molise"), ("cs", "Molise"), ("cy", "Molise"), ("da", "Molise"), ("de", "Molise"), ("el", "Μολίζε"), ("en", "Molise"), ("es", "Molise"), ("et", "Molise"), ("eu", "Molise"), ("fa", "مولیز"), ("fi", "Molise"), ("fr", "Molise"), ("ga", "Molise"), ("gl", "Molise"), ("he", "מוליזה"), ("hr", "Molise"), ("hu", "Molise"), ("hy", "Մոլիզե"), ("id", "Molise"), ("is", "Mólíse"), ("it", "Molise"), ("ja", "モリーゼ州"), ("jv", "Molise"), ("ka", "მოლიზე"), ("kk", "Молизе"), ("ko", "몰리세 주"), ("lt", "Molizė"), ("lv", "Molize"), ("mk", "Молизе"), ("mr", "मोलीझ"), ("ms", "Molise"), ("nb", "Molise"), ("nl", "Molise"), ("no", "Molise"), ("pl", "Molise"), ("pt", "Molise"), ("ro", "Molise"), ("ru", "Молизе"), ("sk", "Molise"), ("sl", "Molize"), ("sq", "Molize"), ("sr", "Молизе"), ("sr_Latn", "Molize"), ("sv", "Molise"), ("sw", "Molise"), ("th", "แคว\u{e49}นโมล\u{e35}เซ"), ("tr", "Molise"), ("uk", "Молізе"), ("ur", "مولیزے"), ("vi", "Molise"), ("yue", "摩列謝"), ("yue_Hans", "摩列谢")]),
                        unofficial_name_list: ["Mulise"].to_vec(),
                    }
                ),
                (
                    "72",
                    Subdivision{
                        name: "72",
                        country_alpha2: Alpha2::IT,
                        code: "72",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.826111), longitude: Some(14.256389), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kampanië"), ("am", "ካምፓንያ"), ("ar", "كامبانيا"), ("az", "Kampaniya"), ("be", "Кампанія"), ("bg", "Кампания"), ("bn", "ক\u{9be}ম\u{9cd}প\u{9be}নিয\u{9bc}\u{9be}"), ("bs", "Kampanija"), ("ca", "Campània"), ("ccp", "𑄇\u{11133}𑄠𑄟\u{11134}𑄛𑄚\u{11128}𑄠"), ("ceb", "Campania"), ("cs", "Kampánie"), ("cy", "Campania"), ("da", "Campania"), ("de", "Kampanien"), ("el", "Καμπανία Ιταλίας"), ("en", "Campania"), ("es", "Campania"), ("et", "Campania"), ("eu", "Campania"), ("fa", "کامپانیا"), ("fi", "Campania"), ("fr", "Campanie"), ("ga", "Campania"), ("gl", "Campania"), ("he", "קמפניה"), ("hi", "का\u{902}पानिया"), ("hr", "Kampanija"), ("hu", "Campania"), ("hy", "Կամպանիա"), ("id", "Campania"), ("is", "Kampanía"), ("it", "Campania"), ("ja", "カンパニア州"), ("jv", "Campania"), ("ka", "კამპანია"), ("kk", "Кампания"), ("ko", "캄파니아 주"), ("lt", "Kampanija"), ("lv", "Kampānija"), ("mk", "Кампанија"), ("mr", "का\u{902}पानिया"), ("ms", "Campania"), ("nb", "Campania"), ("nl", "Campania"), ("no", "Campania"), ("pa", "ਕਾ\u{a02}ਪਾਨੀਆ"), ("pl", "Kampania"), ("pt", "Campânia"), ("ro", "Campania"), ("ru", "Кампания"), ("sk", "Kampánia"), ("sl", "Kampanija"), ("sq", "Kampania"), ("sr", "Кампанија"), ("sr_Latn", "Kampanija"), ("sv", "Kampanien"), ("sw", "Campania"), ("th", "แคว\u{e49}นค\u{e31}มปาเน\u{e35}ย"), ("tr", "Campania"), ("uk", "Кампанія"), ("ur", "کمپانیہ"), ("vi", "Campania"), ("yue", "甘帕尼亞"), ("yue_Hans", "甘帕尼亚"), ("zh", "坎帕尼亚")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "75",
                    Subdivision{
                        name: "75",
                        country_alpha2: Alpha2::IT,
                        code: "75",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.008611), longitude: Some(16.512778), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Apulië"), ("am", "አፑሊያ"), ("ar", "بوليا"), ("az", "Apuliya"), ("be", "Апулія"), ("bg", "Пулия"), ("bs", "Apulija"), ("ca", "Pulla"), ("ccp", "𑄃𑄬𑄛\u{1112a}𑄣\u{11128}𑄠"), ("ceb", "Apulia"), ("cs", "Apulie"), ("cy", "Puglia"), ("da", "Apulien"), ("de", "Apulien"), ("el", "Απουλία"), ("en", "Apulia"), ("es", "Apulia"), ("et", "Apuulia"), ("eu", "Apulia"), ("fa", "آپولیا"), ("fi", "Apulia"), ("fr", "Pouilles"), ("ga", "Puglia"), ("gl", "Puglia"), ("he", "פוליה"), ("hi", "प\u{941}लिया"), ("hr", "Apulija"), ("hu", "Puglia"), ("hy", "Ապուլիա"), ("id", "Puglia"), ("is", "Apúlía"), ("it", "Puglia"), ("ja", "プッリャ州"), ("jv", "Puglia"), ("ka", "აპულია"), ("kk", "Апулия"), ("ko", "풀리아 주"), ("lt", "Apulija"), ("lv", "Apūlija"), ("mk", "Апулија"), ("mr", "प\u{941}लीया"), ("ms", "Apulia"), ("nb", "Puglia"), ("nl", "Apulië"), ("no", "Puglia"), ("pa", "ਪ\u{a42}ਲੀਆ"), ("pl", "Apulia"), ("pt", "Apúlia"), ("ro", "Apulia"), ("ru", "Апулия"), ("sk", "Apúlia"), ("sl", "Apulija"), ("sq", "Pulia"), ("sr", "Апулија"), ("sr_Latn", "Apulija"), ("sv", "Apulien"), ("sw", "Puglia"), ("th", "แคว\u{e49}นป\u{e38}ลยา"), ("tr", "Puglia"), ("uk", "Апулія"), ("ur", "پلیہ"), ("vi", "Apulia"), ("yue", "蒲利亞"), ("yue_Hans", "蒲利亚")]),
                        unofficial_name_list: ["Apoulía", "Apulia", "Pùglia", "Ἀπουλία"].to_vec(),
                    }
                ),
                (
                    "77",
                    Subdivision{
                        name: "77",
                        country_alpha2: Alpha2::IT,
                        code: "77",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.6431), longitude: Some(15.97), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "ባሲሊካታ"), ("ar", "بازيليكاتا"), ("az", "Bazilikata"), ("be", "Базіліката"), ("bg", "Базиликата"), ("bs", "Basilicata"), ("ca", "Basilicata"), ("ccp", "𑄝𑄥\u{11128}𑄣\u{11128}𑄇𑄑"), ("ceb", "Basilicata"), ("cs", "Basilicata"), ("cy", "Basilicata"), ("da", "Basilicata"), ("de", "Basilikata"), ("el", "Μπαζιλικάτα"), ("en", "Basilicata"), ("es", "Basilicata"), ("et", "Basilicata"), ("eu", "Basilicata"), ("fa", "باسیلیکاتا"), ("fi", "Basilicata"), ("fr", "Basilicate"), ("ga", "Basilicata"), ("gl", "Basilicata"), ("he", "בזיליקטה"), ("hi", "बाज\u{93c}िलीकाता"), ("hr", "Basilicata"), ("hu", "Basilicata"), ("hy", "Բազիլիկատա"), ("id", "Basilicata"), ("is", "Basilíkata"), ("it", "Basilicata"), ("ja", "バジリカータ州"), ("jv", "Basilicata"), ("ka", "ბაზილიკატა"), ("kk", "Базиликата"), ("ko", "바실리카타 주"), ("lt", "Bazilikata"), ("lv", "Bazilikata"), ("mk", "Базиликата"), ("mr", "बाझिलिकाता"), ("ms", "Basilicata"), ("nb", "Basilicata"), ("ne", "बासिलिकाटा"), ("nl", "Basilicata"), ("no", "Basilicata"), ("pa", "ਬਾਜ\u{a3c}ਿਲੀਕਾਤਾ"), ("pl", "Basilicata"), ("pt", "Basilicata"), ("ro", "Basilicata"), ("ru", "Базиликата"), ("sk", "Basilicata"), ("sl", "Bazilikata"), ("sq", "Bazilikata"), ("sr", "Базиликата"), ("sr_Latn", "Bazilikata"), ("sv", "Basilicata"), ("sw", "Basilicata"), ("ta", "பசிளிக\u{bbe}த\u{bbe}"), ("th", "แคว\u{e49}นบาซ\u{e35}ล\u{e35}คาตา"), ("tr", "Basilicata"), ("uk", "Базиліката"), ("ur", "بازیلیکاتا"), ("vi", "Basilicata"), ("yue", "巴斯利卡塔"), ("yue_Hans", "巴斯利卡塔")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "78",
                    Subdivision{
                        name: "78",
                        country_alpha2: Alpha2::IT,
                        code: "78",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kalabrië"), ("am", "ካላብሪያ"), ("ar", "قلورية"), ("az", "Kalabriya"), ("be", "Калабрыя"), ("bg", "Калабрия"), ("bs", "Kalabrija"), ("ca", "Calàbria"), ("ccp", "𑄇𑄣𑄝\u{11133}𑄢\u{11128}𑄠"), ("ceb", "Calabria"), ("cs", "Kalábrie"), ("cy", "Calabria"), ("da", "Calabrien"), ("de", "Kalabrien"), ("el", "Καλαβρία"), ("en", "Calabria"), ("es", "Calabria"), ("et", "Calabria"), ("eu", "Calabria"), ("fa", "کالابریا"), ("fi", "Calabria"), ("fr", "Calabre"), ("ga", "Calabria"), ("gl", "Calabria"), ("he", "קלבריה"), ("hi", "कालाब\u{94d}रिया"), ("hr", "Kalabrija"), ("hu", "Calabria"), ("hy", "Կալաբրիա"), ("id", "Calabria"), ("is", "Kalabría"), ("it", "Calabria"), ("ja", "カラブリア州"), ("jv", "Calabria"), ("ka", "კალაბრია"), ("kk", "Калабрия"), ("ko", "칼라브리아 주"), ("lt", "Kalabrija"), ("lv", "Kalabrija"), ("mk", "Калабрија"), ("mr", "कालाब\u{94d}रिया"), ("ms", "Calabria"), ("ne", "कालाब\u{94d}रिआ"), ("nl", "Calabrië"), ("no", "Calabria"), ("pa", "ਕਾਲਾਬਰੀਆ"), ("pl", "Kalabria"), ("pt", "Calábria"), ("ro", "Calabria"), ("ru", "Калабрия"), ("sk", "Kalábria"), ("sl", "Kalabrija"), ("sq", "Kalabria"), ("sr", "Калабрија"), ("sr_Latn", "Kalabrija"), ("sv", "Kalabrien"), ("sw", "Calabria"), ("ta", "கலபிரிய\u{bbe}"), ("th", "แคว\u{e49}นคาลาเบร\u{e35}ย"), ("tr", "Calabria"), ("uk", "Калабрія"), ("ur", "کلابریا"), ("uz", "Kalabriya"), ("vi", "Calabria"), ("yue", "卡拉布利亞"), ("yue_Hans", "卡拉布利亚"), ("zh", "卡拉布里亚")]),
                        unofficial_name_list: ["Calavría", "Calàbbria", "Kalavrì"].to_vec(),
                    }
                ),
                (
                    "82",
                    Subdivision{
                        name: "82",
                        country_alpha2: Alpha2::IT,
                        code: "82",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.6), longitude: Some(14.0154), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sisilië"), ("am", "ሲኪልያ"), ("ar", "صقلية"), ("az", "Siciliya"), ("be", "Сіцылія"), ("bg", "Сицилия"), ("bn", "সিসিল"), ("bs", "Sicilija"), ("ca", "Sicília"), ("ccp", "𑄥\u{11128}𑄥\u{11128}𑄣\u{11128}"), ("ceb", "Sicilia"), ("cs", "Sicílie"), ("cy", "Sisili"), ("da", "Sicilien"), ("de", "Autonome Region Sizilien"), ("el", "Σικελία"), ("en", "Sicily"), ("es", "Sicilia"), ("et", "Sitsiilia maakond"), ("eu", "Sizilia"), ("fa", "سیسیل"), ("fi", "Sisilia"), ("fr", "Sicile"), ("ga", "An tSicil"), ("gl", "Sicilia"), ("ha", "Sisiliya"), ("ha_NE", "Sisiliya"), ("he", "סיציליה"), ("hi", "सिसिली"), ("hr", "Sicilija"), ("hu", "Szicília"), ("hy", "Սիցիլիա"), ("id", "Sisilia"), ("is", "Sikiley"), ("it", "Regione Siciliana"), ("ja", "シチリア州"), ("jv", "Sisilia"), ("ka", "სიცილია"), ("kk", "Сицилия"), ("ko", "시칠리아"), ("ky", "Сицилия"), ("lt", "Sicilija"), ("lv", "Sicīlija"), ("mk", "Сицилија"), ("ml", "സിസിലി"), ("mr", "सिचिल\u{94d}या"), ("ms", "Sicily"), ("nb", "Sicilia"), ("nl", "Sicilië"), ("no", "Sicilia"), ("pa", "ਸਿਚੀਲੀਆ"), ("pl", "Sycylia"), ("ps", "سيسيلي"), ("pt", "Sicília"), ("ro", "Sicilia"), ("ru", "Сицилия"), ("sk", "Sicília"), ("sl", "Sicilija"), ("so", "Sasiiliya"), ("sq", "Sicilia"), ("sr", "Сицилија"), ("sr_Latn", "Sicilija"), ("sv", "Sicilien"), ("sw", "Sisilia"), ("ta", "சிசிலி"), ("te", "స\u{c3f}స\u{c3f}ల\u{c40}"), ("th", "แคว\u{e49}นปกครองตนเองซ\u{e34}ซ\u{e34}ล\u{e35}"), ("tr", "Sicilya"), ("uk", "Сицилія"), ("ur", "صقلیہ"), ("vi", "Sicilia"), ("yo", "Sicily"), ("yo_BJ", "Sicily"), ("yue", "西西里"), ("yue_Hans", "西西里"), ("zh", "西西里岛")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "88",
                    Subdivision{
                        name: "88",
                        country_alpha2: Alpha2::IT,
                        code: "88",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.1209), longitude: Some(9.0129), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sardinië"), ("am", "ሳርዲኒያ"), ("ar", "سردينيا"), ("az", "Sardiniya"), ("be", "Сардзінія"), ("bg", "Сардиния"), ("bs", "Sardinija"), ("ca", "Sardenya"), ("ccp", "𑄥𑄢\u{11134}𑄓\u{11128}𑄚\u{11128}𑄠"), ("ceb", "Cerdeña"), ("cs", "Sardinie"), ("cy", "Sardinia"), ("da", "Sardinien"), ("de", "Sardinien"), ("el", "Σαρδηνία"), ("en", "Sardinia"), ("es", "Cerdeña"), ("et", "Sardiinia"), ("eu", "Sardinia"), ("fa", "ساردنی"), ("fi", "Sardinia"), ("fr", "Sardaigne"), ("ga", "An tSairdín"), ("gl", "Sardeña"), ("ha", "Sardiniya"), ("ha_NE", "Sardiniya"), ("he", "סרדיניה"), ("hi", "सारडीनिया"), ("hr", "Sardinija"), ("hu", "Szardínia"), ("hy", "Սարդինիա"), ("id", "Sardinia"), ("is", "Sardinía"), ("it", "Sardegna"), ("ja", "サルデーニャ州"), ("jv", "Sardinia"), ("ka", "სარდინია"), ("kk", "Сардиния"), ("ko", "사르데냐"), ("lt", "Sardinija"), ("lv", "Sardīnija"), ("mk", "Сардинија"), ("ml", "സ\u{d3e}ർഡീനിയ"), ("mr", "सार\u{94d}दिनिया"), ("ms", "Sardinia"), ("my", "ဆာဒင\u{103a}းန\u{102e}းယားကျ\u{103d}န\u{103a}း"), ("nb", "Sardinia"), ("nl", "Sardinië"), ("no", "Sardinia"), ("pa", "ਸਾਰਦ\u{a47}ਞਾ"), ("pl", "Sardynia"), ("pt", "Sardenha"), ("ro", "Sardinia"), ("ru", "Сардиния"), ("sc", "Sardìgna"), ("sk", "Sardínia"), ("sl", "Sardinija"), ("so", "Sardiiniya"), ("sq", "Sardenja"), ("sr", "Сардинија"), ("sr_Latn", "Sardinija"), ("sv", "Sardinien"), ("sw", "Sardinia"), ("ta", "ச\u{bbe}ர\u{bcd}த\u{bc0}னிய\u{bbe}"), ("te", "స\u{c3e}ర\u{c4d}డ\u{c40}న\u{c3f}య\u{c3e}"), ("th", "แคว\u{e49}นปกครองตนเองซาร\u{e4c}ด\u{e34}เน\u{e35}ย"), ("tr", "Sardinya"), ("uk", "Сардинія"), ("ur", "ساردینیا"), ("uz", "Sardiniya"), ("vi", "Sardegna"), ("yue", "薩丁島"), ("yue_Hans", "萨丁岛"), ("zh", "撒丁岛")]),
                        unofficial_name_list: ["Saldigna", "Sardegna", "Sardenya", "Sardhigna", "Sardìnnia"].to_vec(),
                    }
                ),
                (
                    "AG",
                    Subdivision{
                        name: "AG",
                        country_alpha2: Alpha2::IT,
                        code: "AG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.3110897), longitude: Some(13.5765475), max_latitude: Some(37.3402724), min_latitude: Some(37.2960119), max_longitude: Some(13.6034021), min_longitude: Some(13.5689481)}),
                        comments: None,
                        subdivision_type: SubdivisionType::FreeMunicipalConsortium,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أغريجنتو"), ("be", "правінцыя Агрыджэнта"), ("bg", "Агридженто"), ("bn", "অ\u{9cd}য\u{9be}গ\u{9cd}রিগেন\u{9cd}টো"), ("ca", "Província d’Agrigent"), ("ccp", "𑄃𑄉\u{11133}𑄢\u{11128}𑄉𑄬𑄚\u{11134}𑄑\u{1112e}"), ("ceb", "Agrigento"), ("cs", "Provincie Agrigento"), ("da", "Agrigento"), ("de", "Provinz Agrigent"), ("el", "Αγκριτζέντο"), ("en", "Agrigento"), ("es", "Agrigento"), ("et", "Agrigento provints"), ("eu", "Agrigentoko probintzia"), ("fa", "استان آگریجنتو"), ("fi", "Agrigenton maakunta"), ("fr", "province d’Agrigente"), ("gl", "Provincia de Agrigento"), ("gu", "એગ\u{acd}રીગ\u{ac7}ન\u{acd}ટો પ\u{acd}રા\u{a82}ત"), ("he", "אגריג׳נטו"), ("hi", "एग\u{94d}रीज\u{947}\u{902}टो प\u{94d}रा\u{902}त"), ("hr", "Agrigento"), ("hu", "Agrigento megye"), ("hy", "Ագրիջենտո"), ("id", "Provinsi Agrigento"), ("it", "provincia di Agrigento"), ("ja", "アグリジェント県"), ("jv", "Provinsi Agrigento"), ("ka", "აგრიჯენტოს პროვინცია"), ("kn", "ಆಗ\u{ccd}ಗ\u{cbf}ಜ\u{cc6}ಂಟೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아그리젠토 현"), ("lt", "Agridžento provincija"), ("lv", "Agridžento province"), ("mr", "ऑग\u{94d}रीग\u{947}\u{902}टो प\u{94d}रा\u{902}त"), ("ms", "Wilayah Agrigento"), ("nb", "Provinsen Agrigento"), ("nl", "Agrigento"), ("no", "Provinsen Agrigento"), ("pl", "Prowincja Agrigento"), ("pt", "Agrigento"), ("ro", "Provincia Agrigento"), ("ru", "Агридженто"), ("si", "අග\u{dca}ර\u{dd2}ගේන\u{dca}ටෝ පළ\u{dcf}ත"), ("sk", "Agrigento"), ("sl", "Agrigento"), ("sq", "Provinca e Agrixhentos"), ("sr", "Агриђенто"), ("sr_Latn", "Agriđento"), ("sv", "Agrigento"), ("ta", "அக\u{bcd}ரிகேண\u{bcd}டோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అగ\u{c4d}రగ\u{c46}ంట\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอากร\u{e34}เจนโต"), ("tr", "Agrigento ili"), ("uk", "Провінція Агрідженто"), ("ur", "صوبہ آگریجنتو"), ("uz", "Agrigento"), ("vi", "Agrigento"), ("zh", "阿格里真托省")]),
                        unofficial_name_list: ["Province of Agrigento"].to_vec(),
                    }
                ),
                (
                    "AL",
                    Subdivision{
                        name: "AL",
                        country_alpha2: Alpha2::IT,
                        code: "AL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.9072727), longitude: Some(8.6116796), max_latitude: Some(44.9397498), min_latitude: Some(44.8773148), max_longitude: Some(8.6404936), min_longitude: Some(8.5540221)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ألساندريا"), ("be", "правінцыя Алесандрыя"), ("bg", "Алесандрия"), ("bn", "আলেস\u{9be}ন\u{9cd}দ\u{9cd}রিয\u{9bc}\u{9be}-র প\u{9cd}রদেশ"), ("ca", "Província d’Alessandria"), ("ccp", "𑄃𑄣𑄬𑄥𑄚\u{11134}𑄓\u{11133}𑄢\u{11128}𑄠"), ("ceb", "Provincia di Alessandria"), ("cs", "Provincie Alessandria"), ("da", "Alessandria"), ("de", "Provinz Alessandria"), ("el", "Επαρχία της Αλεσσάντρια"), ("en", "Alessandria"), ("es", "Alessandria"), ("et", "Alessandria provints"), ("eu", "Alessandriako probintzia"), ("fa", "استان آلساندریا"), ("fi", "Alessandrian maakunta"), ("fr", "Province d’Alexandrie"), ("gl", "Provincia de Alessandria"), ("gu", "એલ\u{ac7}સાન\u{acd}ડ\u{acd}રીઆ પ\u{acd}રા\u{a82}ત"), ("he", "אלסנדריה"), ("hi", "एल\u{947}स\u{948}\u{902}ड\u{94d}रिया प\u{94d}रा\u{902}त"), ("hu", "Alessandria megye"), ("hy", "Ալեսանդրիա"), ("id", "Provinsi Alessandria"), ("is", "Alessandria"), ("it", "provincia di Alessandria"), ("ja", "アレッサンドリア県"), ("jv", "Provinsi Alessandria"), ("ka", "ალესანდრიის პროვინცია"), ("kn", "ಅಲ\u{cc6}ಸ\u{ccd}ಸಾಂಡ\u{ccd}ರ\u{cbf}ಯ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "알레산드리아 현"), ("lt", "Alesandrijos provincija"), ("lv", "Alesandrijas province"), ("mk", "Алесандрија"), ("mr", "अल\u{947}स\u{94d}सा\u{902}द\u{94d}रिया प\u{94d}रा\u{902}त"), ("ms", "Wilayah Alessandria"), ("nb", "Provinsen Alessandria"), ("nl", "Alessandria"), ("no", "Provinsen Alessandria"), ("pl", "Prowincja Alessandria"), ("pt", "Alexandria (província)"), ("ro", "Provincia Alessandria"), ("ru", "Алессандрия"), ("si", "අල\u{dd2}සන\u{dca}ද\u{dca}ර\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sl", "Alessandria"), ("sq", "Provinca e Aleksandrisë"), ("sr", "Алесандрија"), ("sr_Latn", "Alesandrija"), ("sv", "Alessandria"), ("ta", "அலெச\u{bbe}ண\u{bcd}ட\u{bcd}ரிய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అల\u{c46}స\u{c4d}స\u{c3e}ండ\u{c4d}ర\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "อะเลสแซนเดร\u{e35}ย"), ("tr", "Alessandria ili"), ("uk", "Провінція Алессандрія"), ("ur", "صوبہ الیساندریا"), ("uz", "Alessandria"), ("vi", "Alessandria"), ("zh", "亞歷山德里亞省")]),
                        unofficial_name_list: ["Province of Alessandria"].to_vec(),
                    }
                ),
                (
                    "AN",
                    Subdivision{
                        name: "AN",
                        country_alpha2: Alpha2::IT,
                        code: "AN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.6158299), longitude: Some(13.518915), max_latitude: Some(43.6314669), min_latitude: Some(43.532833), max_longitude: Some(13.5498571), min_longitude: Some(13.4637957)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أنكونا"), ("be", "Анкона"), ("bg", "Анкона"), ("bn", "অংকোন\u{9be} প\u{9cd}রদেশ"), ("ca", "Província d’Ancona"), ("ccp", "𑄃𑄚\u{11134}𑄇\u{1112e}𑄚"), ("ceb", "Provincia di Ancona"), ("cs", "Provincie Ancona"), ("da", "Province of Ancona"), ("de", "Provinz Ancona"), ("el", "Ανκόνα"), ("en", "Ancona"), ("es", "Ancona"), ("et", "Ancona provints"), ("eu", "Anconako probintzia"), ("fa", "استان آنکونا"), ("fi", "Ancona"), ("fr", "province d’Ancône"), ("gl", "Provincia de Ancona"), ("gu", "એન\u{acd}કોના પ\u{acd}રા\u{a82}ત"), ("he", "אנקונה"), ("hi", "ए\u{902}कोना प\u{94d}रा\u{902}त"), ("hu", "Ancona megye"), ("hy", "Անկոնա"), ("id", "Provinsi Ancona"), ("it", "provincia di Ancona"), ("ja", "アンコーナ県"), ("jv", "Provinsi Ancona"), ("ka", "ანკონის პროვინცია"), ("kn", "ಆಂಕೋನಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "안코나 현"), ("lt", "Ankonos provincija"), ("lv", "Ankonas province"), ("mr", "अ\u{901}काना प\u{94d}रा\u{902}त"), ("ms", "Wilayah Ancona"), ("nb", "Provinsen Ancona"), ("nl", "Ancona"), ("no", "Provinsen Ancona"), ("pl", "Prowincja Ankona"), ("pt", "Ancona"), ("ro", "Provincia Ancona"), ("ru", "Анкона"), ("si", "ඇන\u{dca}කොන\u{dcf} පළ\u{dcf}ත"), ("sk", "Ancona"), ("sl", "Ancona"), ("sq", "Provinca e Ankonës"), ("sr", "Анкона"), ("sr_Latn", "Ankona"), ("sv", "Ancona"), ("ta", "எண\u{bcd}கோண ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అంక\u{c4b}న\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแอนโคนา"), ("tr", "Ancona ili"), ("uk", "Провінція Анкона"), ("ur", "صوبہ انکونا"), ("uz", "Ancona"), ("vi", "Ancona"), ("zh", "安科納省")]),
                        unofficial_name_list: ["Province of Ancona"].to_vec(),
                    }
                ),
                (
                    "AP",
                    Subdivision{
                        name: "AP",
                        country_alpha2: Alpha2::IT,
                        code: "AP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.8536043), longitude: Some(13.5749442), max_latitude: Some(42.8681651), min_latitude: Some(42.8376024), max_longitude: Some(13.6656375), min_longitude: Some(13.552394)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أسكولي بيتشينو"), ("be", "правінцыя Асколі-Пічэна"), ("bg", "Асколи Пичено"), ("bn", "এসকোলি পিচেনোর প\u{9cd}রদেশ"), ("ca", "Província d’Ascoli Piceno"), ("ccp", "𑄃𑄌\u{11134}𑄇\u{1112e}𑄣\u{11128} 𑄛\u{1112d}𑄌\u{11134}𑄚\u{1112e}"), ("ceb", "Provincia di Ascoli Piceno"), ("cs", "Provincie Ascoli Piceno"), ("da", "Ascoli Piceno"), ("de", "Provinz Ascoli Piceno"), ("el", "Άσκολι Πιτσένο"), ("en", "Ascoli Piceno"), ("es", "Ascoli Piceno"), ("et", "Ascoli Piceno provints"), ("eu", "Ascoli Picenoko probintzia"), ("fa", "استان اسکولی پیچنو"), ("fi", "Ascoli Picenon maakunta"), ("fr", "province d’Ascoli Piceno"), ("gl", "Provincia de Ascoli Piceno"), ("gu", "એસ\u{acd}કોલી પિક\u{ac7}નો પ\u{acd}રા\u{a82}ત"), ("he", "אסקולי פיצ׳נו"), ("hi", "असकोली पिस\u{947}नो प\u{94d}रा\u{902}त"), ("hu", "Ascoli Piceno megye"), ("hy", "Ասկոլի Պիչենո"), ("id", "Provinsi Ascoli Piceno"), ("it", "provincia di Ascoli Piceno"), ("ja", "アスコリ・ピチェーノ県"), ("jv", "Provinsi Ascoli Piceno"), ("ka", "ასკოლი-პიჩენოს პროვინცია"), ("kn", "ಆಸ\u{ccd}ಕೊಲ\u{cbf} ಪ\u{cbf}ಕೊನೋ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아스콜리피체노 현"), ("lt", "Askoli Pičeno provincija"), ("lv", "Askoli Pičēno province"), ("mk", "Асколи-Пичено"), ("mr", "असकोली पिकिओ प\u{94d}रा\u{902}त"), ("ms", "Wilayah Ascoli Piceno"), ("nb", "Provinsen Ascoli Piceno"), ("nl", "Ascoli Piceno"), ("no", "Provinsen Ascoli Piceno"), ("pl", "Prowincja Ascoli Piceno"), ("pt", "Ascoli Piceno"), ("ro", "Provincia Ascoli Piceno"), ("ru", "Асколи-Пичено"), ("si", "ඇස\u{dca}කොල\u{dd2} ප\u{dd2}සෙනෝ පළ\u{dcf}ත"), ("sl", "Ascoli Piceno"), ("sq", "Provinca Ascoli Piceno"), ("sr", "Асколи Пичено"), ("sr_Latn", "Askoli Pičeno"), ("sv", "Ascoli Piceno"), ("ta", "அஸ\u{bcd}கோலி பிசினோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అస\u{c4d}క\u{c4b}ల\u{c3f} ప\u{c3f}స\u{c46}న\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e31}สโกล\u{e35}ป\u{e35}เชโน"), ("tr", "Ascoli Piceno ili"), ("uk", "Провінція Асколі-Пічено"), ("ur", "صوبہ آسکولی پیچینو"), ("uz", "Ascoli Piceno"), ("vi", "Ascoli Piceno"), ("zh", "阿斯科利皮切諾省")]),
                        unofficial_name_list: ["Province of Ascoli Piceno"].to_vec(),
                    }
                ),
                (
                    "AQ",
                    Subdivision{
                        name: "AQ",
                        country_alpha2: Alpha2::IT,
                        code: "AQ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.3498479), longitude: Some(13.3995091), max_latitude: Some(42.388588), min_latitude: Some(42.340926), max_longitude: Some(13.4348911), min_longitude: Some(13.336641)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة لاكويلا"), ("bn", "এল’আক\u{9c1}ইল\u{9be}র প\u{9cd}রদেশ"), ("ca", "Província de L’Aquila"), ("ccp", "𑄣\u{11128}‘𑄃𑄇\u{1112d}\u{1112a}𑄣"), ("ceb", "L’Aquila"), ("cs", "Provincie L’Aquila"), ("da", "L’Aquila"), ("de", "Provinz L’Aquila"), ("el", "Επαρχία της Άκουιλα"), ("en", "L’Aquila"), ("es", "L’Aquila"), ("et", "L’Aquila provints"), ("eu", "L’Aquilako probintzia"), ("fa", "استان لاکویلا"), ("fi", "L’Aquilan maakunta"), ("fr", "Province de L’Aquila"), ("gl", "Provincia de L’Aquila"), ("gu", "લ ‘એક\u{acd}વિલા પ\u{acd}રા\u{a82}ત"), ("he", "ל׳אקווילה"), ("hi", "लकीला प\u{94d}रा\u{902}त"), ("hu", "L’Aquila megye"), ("id", "Provinsi L’Aquila"), ("it", "provincia dell’Aquila"), ("ja", "ラクイラ県"), ("jv", "Provinsi L’Aquila"), ("kn", "ಎಲ\u{ccd} ಅಕ\u{ccd}ವ\u{cbf}ಲಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "라퀼라 현"), ("lt", "Akvilos provincija"), ("lv", "L’Akvilas province"), ("mr", "ल अकव\u{947}लच\u{947} प\u{94d}रा\u{902}त"), ("ms", "Wilayah L’Aquila"), ("nb", "Provinsen L’Aquila"), ("nl", "L’Aquila"), ("no", "Provinsen L’Aquila"), ("pl", "Prowincja L’Aquila"), ("pt", "Áquila"), ("ro", "Provincia L’Aquila"), ("si", "එල\u{dca} අක\u{dd2}ල\u{dcf} පළ\u{dcf}ත"), ("sl", "L’Aquila"), ("sq", "Provinca e Akuilës"), ("sv", "L’Aquila"), ("ta", "ல ‘அகுய\u{bcd}ல ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c3e}అక\u{c40}ల\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแอล อ\u{e31}กคว\u{e34}ลา"), ("tr", "L’Aquila ili"), ("uk", "Провінція ЛʼАкуїла"), ("ur", "صوبہ لاکویلا"), ("uz", "L’Aquila"), ("vi", "L’Aquila"), ("zh", "阿奎拉省")]),
                        unofficial_name_list: ["L'Aquila"].to_vec(),
                    }
                ),
                (
                    "AR",
                    Subdivision{
                        name: "AR",
                        country_alpha2: Alpha2::IT,
                        code: "AR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.46328390000001), longitude: Some(11.8796336), max_latitude: Some(43.4995733), min_latitude: Some(43.3514366), max_longitude: Some(11.9240635), min_longitude: Some(11.8096337)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أريتسو"), ("az", "Arezzo"), ("be", "правінцыя Арэца"), ("bg", "Арецо"), ("bn", "অ\u{9cd}য\u{9be}রেজ\u{9cd}জো-এর প\u{9cd}রদেশ"), ("ca", "Província d’Arezzo"), ("ccp", "𑄃𑄢𑄬𑄎\u{1112e}"), ("ceb", "Province of Arezzo"), ("cs", "Provincie Arezzo"), ("da", "Province of Arezzo"), ("de", "Provinz Arezzo"), ("el", "Αρέτσο"), ("en", "Arezzo"), ("es", "Arezzo"), ("et", "Arezzo provints"), ("eu", "Arezzoko probintzia"), ("fa", "استان آرتزو"), ("fi", "Arezzon maakunta"), ("fr", "province d’Arezzo"), ("gl", "Provincia de Arezzo"), ("gu", "એર\u{ac7}ઝો પ\u{acd}રા\u{a82}ત"), ("he", "ארצו"), ("hi", "अर\u{947}जो प\u{94d}रा\u{902}त"), ("hr", "Arezzo"), ("hu", "Arezzo megye"), ("hy", "Արեցո"), ("id", "Provinsi Arezzo"), ("is", "Arezzo"), ("it", "provincia di Arezzo"), ("ja", "アレッツォ県"), ("jv", "Provinsi Arezzo"), ("ka", "არეცოს პროვინცია"), ("kn", "ಅರ\u{cc6}ಝೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아레초 현"), ("lt", "Areco provincija"), ("lv", "Areco province"), ("mr", "आर\u{947}झझो प\u{94d}रा\u{902}त"), ("ms", "Wilayah Arezzo"), ("nb", "Arezzo"), ("nl", "Arezzo"), ("no", "Arezzo"), ("pl", "Prowincja Arezzo"), ("pt", "Arezzo"), ("ro", "Provincia Arezzo"), ("ru", "Ареццо"), ("si", "අරෙස\u{dca}සෝ පළ\u{dcf}ත"), ("sk", "Arezzo"), ("sl", "Arezzo"), ("sq", "Provinca e Arexos"), ("sr", "Арецо"), ("sr_Latn", "Areco"), ("sv", "Arezzo"), ("ta", "அரேஸ\u{bcd}ஸ\u{bcd}வ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఆర\u{c46}జ\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาเรซโซ"), ("tr", "Arezzo ili"), ("uk", "Провінція Ареццо"), ("ur", "صوبہ آرتزو"), ("uz", "Arezzo"), ("vi", "Arezzo"), ("zh", "阿雷佐省")]),
                        unofficial_name_list: ["Province of Arezzo"].to_vec(),
                    }
                ),
                (
                    "AT",
                    Subdivision{
                        name: "AT",
                        country_alpha2: Alpha2::IT,
                        code: "AT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.90075119999999), longitude: Some(8.2064257), max_latitude: Some(44.9293241), min_latitude: Some(44.8644264), max_longitude: Some(8.2637251), min_longitude: Some(8.1550091)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أستي"), ("be", "правінцыя Асці"), ("bg", "Асти"), ("bn", "আসতি প\u{9cd}রদেশ"), ("ca", "Província d’Asti"), ("ccp", "𑄃𑄌\u{11134}𑄑\u{11128}"), ("ceb", "Provincia di Asti"), ("cs", "Provincie Asti"), ("da", "Asti"), ("de", "Provinz Asti"), ("el", "Άστι"), ("en", "Asti"), ("es", "Asti"), ("et", "Asti provints"), ("eu", "Astiko probintzia"), ("fa", "استان آسته"), ("fi", "Astin maakunta"), ("fr", "Province d’Asti"), ("gl", "Provincia de Asti"), ("gu", "અસ\u{acd}તિ પ\u{acd}રા\u{a82}ત"), ("he", "אסטי"), ("hi", "एस\u{94d}टी प\u{94d}रा\u{902}त"), ("hu", "Asti megye"), ("hy", "Աստի"), ("id", "Provinsi Asti"), ("is", "Asti"), ("it", "provincia di Asti"), ("ja", "アスティ県"), ("jv", "Provinsi Asti"), ("ka", "ასტის პროვინცია"), ("kn", "ಆಸ\u{ccd}ತ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아스티 현"), ("lt", "Asčio provincija"), ("lv", "Asti province"), ("mk", "Асти"), ("mr", "असती प\u{94d}रा\u{902}त"), ("ms", "Wilayah Asti"), ("nb", "Provinsen Asti"), ("nl", "Asti"), ("no", "Provinsen Asti"), ("pl", "Prowincja Asti"), ("pt", "Asti"), ("ro", "Provincia Asti"), ("ru", "Асти"), ("si", "අස\u{dca}ට\u{dd2} පළ\u{dcf}ත"), ("sl", "Asti"), ("sq", "Provinca e Astit"), ("sr", "Асти"), ("sr_Latn", "Asti"), ("sv", "Asti"), ("ta", "அஸ\u{bcd}தி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అస\u{c4d}ట\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e31}สต\u{e35}"), ("tr", "Asti ili"), ("uk", "Провінція Асті"), ("ur", "صوبہ آستی"), ("uz", "Asti"), ("vi", "Asti"), ("zh", "阿斯蒂省")]),
                        unofficial_name_list: ["Province of Asti"].to_vec(),
                    }
                ),
                (
                    "AV",
                    Subdivision{
                        name: "AV",
                        country_alpha2: Alpha2::IT,
                        code: "AV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.914388), longitude: Some(14.7906121), max_latitude: Some(40.9458888), min_latitude: Some(40.8984426), max_longitude: Some(14.8314412), min_longitude: Some(14.7588462)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أفيلينو"), ("be", "правінцыя Авеліна"), ("bg", "Авелино"), ("bn", "অ\u{9cd}য\u{9be}ভেলেনো প\u{9cd}রদেশ"), ("ca", "Província d’Avellino"), ("ccp", "𑄃𑄞𑄬𑄣\u{11128}𑄚\u{1112e}"), ("ceb", "Avellino"), ("cs", "Provincie Avellino"), ("da", "Avellino"), ("de", "Provinz Avellino"), ("el", "Αβελλίνο"), ("en", "Avellino"), ("es", "Avellino"), ("et", "Avellino provints"), ("eu", "Avellinoko probintzia"), ("fa", "استان آولینو"), ("fi", "Avellinon maakunta"), ("fr", "province d’Avellino"), ("gl", "Provincia de Avellino"), ("gu", "અવ\u{ac7}લીનો પ\u{acd}રા\u{a82}ત"), ("he", "אבלינו"), ("hi", "एव\u{947}ल\u{94d}लिनो प\u{94d}रा\u{902}त"), ("hu", "Avellino megye"), ("hy", "Ավելինո"), ("id", "Provinsi Avellino"), ("it", "provincia di Avellino"), ("ja", "アヴェッリーノ県"), ("jv", "Provinsi Avellino"), ("ka", "აველინოს პროვინცია"), ("kn", "ಅವ\u{cc6}ಲ\u{cbf}ನೋ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아벨리노 현"), ("lt", "Avelino provincija"), ("lv", "Avellīno province"), ("mk", "Авелино"), ("mr", "एव\u{947}ल\u{94d}लिनो प\u{94d}रा\u{902}त"), ("ms", "Wilayah Avellino"), ("nb", "Provinsen Avellino"), ("nl", "Avellino"), ("no", "Provinsen Avellino"), ("pl", "Prowincja Avellino"), ("pt", "Avellino"), ("ro", "Provincia Avellino"), ("ru", "Авеллино"), ("si", "අවෙල\u{dd2}නෝ පළ\u{dcf}ත"), ("sl", "Avellino"), ("sq", "Provinca e Avelinos"), ("sr", "Авелино"), ("sr_Latn", "Avelino"), ("sv", "Avellino"), ("ta", "அவெள\u{bcd}ளினோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అవ\u{c46}ల\u{c4d}ల\u{c3f}న\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาเวลล\u{e34}โน"), ("tr", "Avellino ili"), ("uk", "Провінція Авелліно"), ("ur", "صوبہ آویلینو"), ("uz", "Avellino"), ("vi", "Avellino"), ("zh", "阿韋利諾省")]),
                        unofficial_name_list: ["Province of Avellino"].to_vec(),
                    }
                ),
                (
                    "BA",
                    Subdivision{
                        name: "BA",
                        country_alpha2: Alpha2::IT,
                        code: "BA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.1171432), longitude: Some(16.8718715), max_latitude: Some(41.169568), min_latitude: Some(41.0530106), max_longitude: Some(17.0333497), min_longitude: Some(16.7307831)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة باري"), ("be", "правінцыя Бары"), ("bg", "Бари"), ("bn", "মেট\u{9cd}রোপলিটন²"), ("ca", "Província de Bari"), ("ccp", "𑄝𑄢\u{11128}"), ("ceb", "Bari"), ("cs", "Provincie Bari"), ("da", "Metropolitan City of Bari"), ("de", "Provinz Bari"), ("el", "Μπάρι"), ("en", "Bari"), ("es", "Bari"), ("et", "Bari provints"), ("eu", "Bariko probintzia"), ("fa", "استان باری"), ("fi", "Barin maakunta"), ("fr", "province de Bari"), ("gl", "Provincia de Bari"), ("gu", "બારીન\u{ac1}\u{a82} મ\u{ac7}ટ\u{acd}રોપોલિટન શહ\u{ac7}ર"), ("he", "בארי"), ("hi", "बारी प\u{94d}रा\u{902}त"), ("hu", "Bari megye"), ("hy", "Բարի"), ("id", "Provinsi Bari"), ("it", "provincia di Bari"), ("ja", "バーリ県"), ("ka", "ბარის პროვინცია"), ("kn", "ಮ\u{cc6}ರ\u{cbf}ಪಾಲ\u{cbf}ಟನ\u{ccd} ಸ\u{cbf}ಟ\u{cbf} ಆಫ\u{ccd} ಬ\u{ccd}ಯಾರ\u{cbf}"), ("ko", "바리 현"), ("lt", "Bario provincija"), ("lv", "Bari province"), ("mk", "Бари"), ("mr", "बारी महानगरीय शहर"), ("ms", "Wilayah Bari"), ("nb", "Bari"), ("nl", "Bari"), ("no", "Provinsen Bari"), ("pl", "Prowincja Bari"), ("pt", "Bari"), ("ro", "Provincia Bari"), ("ru", "Бари"), ("si", "බ\u{dcf}ර\u{dd2} වල මෙට\u{dca}\u{200d}රොපොල\u{dd2}ටන\u{dca} නගරය"), ("sl", "Bari"), ("sq", "Provinca e Barit"), ("sr", "Бари"), ("sr_Latn", "Bari"), ("sv", "Bari"), ("ta", "மெட\u{bcd}ரோபொலிட\u{bcd}டன\u{bcd} நகரம\u{bcd} ப\u{bbe}ரி"), ("te", "మ\u{c46}ట\u{c4d}ర\u{c4b}ప\u{c3e}ల\u{c3f}టన\u{c4d} స\u{c3f}ట\u{c40} ఆఫ\u{c4d} బ\u{c3e}ర\u{c3f}"), ("th", "เมโทรโพล\u{e34}แทน"), ("tr", "Bari ili"), ("uk", "Провінція Барі"), ("ur", "میٹروپولیٹن شہر باری"), ("uz", "Bari"), ("vi", "Bari"), ("yue", "巴里省"), ("yue_Hans", "巴里省"), ("zh", "巴里省")]),
                        unofficial_name_list: ["Città Metropolitana di Bari", "Metropolitan City of Bari"].to_vec(),
                    }
                ),
                (
                    "BG",
                    Subdivision{
                        name: "BG",
                        country_alpha2: Alpha2::IT,
                        code: "BG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.6982642), longitude: Some(9.6772698), max_latitude: Some(45.7238489), min_latitude: Some(45.6625158), max_longitude: Some(9.7136328), min_longitude: Some(9.620546299999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بيرغامو"), ("be", "правінцыя Бергама"), ("bg", "Бергамо"), ("bn", "প\u{9cd}রভিন\u{9cd}স অব বেরগ\u{9be}মো"), ("ca", "Província de Bèrgam"), ("ccp", "𑄝𑄬𑄢\u{11134}𑄉𑄟\u{1112e}"), ("ceb", "Provincia di Bergamo"), ("cs", "Provincie Bergamo"), ("da", "Bergamo"), ("de", "Provinz Bergamo"), ("el", "Επαρχία του Μπέργκαμο"), ("en", "Bergamo"), ("es", "Bérgamo"), ("et", "Bergamo provints"), ("eu", "Bergamoko probintzia"), ("fa", "استان برگامو"), ("fi", "Bergamon maakunta"), ("fr", "Province de Bergame"), ("gl", "Provincia de Bérgamo"), ("gu", "બર\u{acd}ગ\u{ac7}મો પ\u{acd}રા\u{a82}ત"), ("he", "ברגמו"), ("hi", "बर\u{94d}ग\u{947}मो प\u{94d}रा\u{902}त"), ("hu", "Bergamo megye"), ("hy", "Բերգամո"), ("id", "Provinsi Bergamo"), ("it", "provincia di Bergamo"), ("ja", "ベルガモ県"), ("jv", "Provinsi Bergamo"), ("ka", "ბერგამოს პროვინცია"), ("kn", "ಬರ\u{ccd}ಗಮೋ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "베르가모 현"), ("lt", "Bergamo provincija"), ("lv", "Bergamo province"), ("mk", "Бергамо"), ("mr", "ब\u{947}रगामो प\u{94d}रा\u{902}त"), ("ms", "Wilayah Bergamo"), ("nb", "Provinsen Bergamo"), ("nl", "Bergamo"), ("no", "Provinsen Bergamo"), ("pl", "Prowincja Bergamo"), ("pt", "Bérgamo"), ("ro", "Provincia Bergamo"), ("ru", "Бергамо"), ("si", "බර\u{dca}ග\u{dcf}මෝ පළ\u{dcf}ත"), ("sk", "Bergamo"), ("sl", "Bergamo"), ("sq", "Provinca e Bergamos"), ("sr", "Бергамо"), ("sr_Latn", "Bergamo"), ("sv", "Bergamo"), ("ta", "பெர\u{bcd}கமோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c46}ర\u{c4d}గ\u{c3e}మ\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแบร\u{e4c}กาโม"), ("tr", "Bergamo ili"), ("uk", "Провінція Бергамо"), ("ur", "صوبہ بیرگامو"), ("uz", "Bergamo"), ("vi", "Bergamo"), ("zh", "貝加莫省")]),
                        unofficial_name_list: ["Province of Bergamo"].to_vec(),
                    }
                ),
                (
                    "BI",
                    Subdivision{
                        name: "BI",
                        country_alpha2: Alpha2::IT,
                        code: "BI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.56288420000001), longitude: Some(8.0583397), max_latitude: Some(45.5983864), min_latitude: Some(45.5427224), max_longitude: Some(8.0935542), min_longitude: Some(8.0161269)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بييلا"), ("bg", "Биела"), ("bn", "বেইল\u{9be}-র প\u{9cd}রদেশ"), ("ca", "Província de Biella"), ("ccp", "𑄝\u{11128}𑄠𑄬𑄣"), ("ceb", "Provincia di Biella"), ("cs", "Provincie Biella"), ("da", "Biella"), ("de", "Provinz Biella"), ("el", "Μπιέλλα"), ("en", "Biella"), ("es", "Biella"), ("et", "Biella provints"), ("eu", "Biellako probintzia"), ("fa", "استان بیه\u{200c}لا"), ("fi", "Biellan maakunta"), ("fr", "Province de Biella"), ("gl", "Provincia de Biella"), ("gu", "બિએલા પ\u{acd}રા\u{a82}ત"), ("he", "ביאלה"), ("hi", "बिएला प\u{94d}रा\u{902}त"), ("hu", "Biella megye"), ("hy", "Բիելլա"), ("id", "Provinsi Biella"), ("is", "Biella"), ("it", "provincia di Biella"), ("ja", "ビエッラ県"), ("jv", "Provinsi Biella"), ("ka", "ბიელა"), ("kn", "ಬೈಯಲ\u{ccd}ಲಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "비엘라 현"), ("lt", "Bjelos provincija"), ("lv", "Bjellas province"), ("mr", "ब\u{94d}रीएलला प\u{94d}रा\u{902}त"), ("ms", "Wilayah Biella"), ("nb", "Provinsen Biella"), ("nl", "Biella"), ("no", "Provinsen Biella"), ("pl", "Prowincja Biella"), ("pt", "Biella"), ("ro", "Provincia Biella"), ("ru", "Бьелла"), ("si", "බයෙල\u{dcf} පළ\u{dcf}ත"), ("sl", "Biella"), ("sq", "Provinca e Bielës"), ("sr", "Бијела"), ("sr_Latn", "Bijela"), ("sv", "Biella"), ("ta", "பியெல\u{bcd}ல\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c3f}య\u{c46}ల\u{c4d}ల\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e34}เอลลา"), ("tr", "Biella ili"), ("uk", "Провінція Бʼєлла"), ("ur", "صوبہ بیئلا"), ("uz", "Biella"), ("vi", "Biella"), ("zh", "比耶拉省")]),
                        unofficial_name_list: ["Province of Biella"].to_vec(),
                    }
                ),
                (
                    "BL",
                    Subdivision{
                        name: "BL",
                        country_alpha2: Alpha2::IT,
                        code: "BL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1424635), longitude: Some(12.2167088), max_latitude: Some(46.1715533), min_latitude: Some(46.1237249), max_longitude: Some(12.2513991), min_longitude: Some(12.1569421)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بلونو"), ("be", "Правінцыя Белуна"), ("bg", "Белуно"), ("bn", "বেল\u{9c1}নোর প\u{9cd}রদেশ"), ("ca", "Província de Belluno"), ("ccp", "𑄝𑄬𑄣\u{11128}𑄅\u{1112a}𑄚\u{1112e}"), ("ceb", "Provincia di Belluno"), ("cs", "Provincie Belluno"), ("da", "Belluno"), ("de", "Provinz Belluno"), ("el", "Μπελούνο"), ("en", "Belluno"), ("es", "Belluno"), ("et", "Belluno provints"), ("eu", "Bellunoko probintzia"), ("fa", "استان بلونو"), ("fi", "Bellunon maakunta"), ("fr", "province de Belluno"), ("gl", "Provincia de Belluno"), ("gu", "બ\u{ac7}લ\u{ac2}નો પ\u{acd}રા\u{a82}ત"), ("he", "בלונו"), ("hi", "ब\u{947}ल\u{941}नो प\u{94d}रा\u{902}त"), ("hr", "Belluno (pokrajina)"), ("hu", "Belluno megye"), ("hy", "Բելունո"), ("id", "Provinsi Belluno"), ("it", "provincia di Belluno"), ("ja", "ベッルーノ県"), ("jv", "Provinsi Belluno"), ("ka", "ბელუნოს პროვინცია"), ("kn", "ಬ\u{cc6}ಲ\u{ccd}ಲುನೋ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "벨루노 현"), ("lt", "Beluno provincija"), ("lv", "Belluno province"), ("mk", "Белуно"), ("mr", "ब\u{947}ल\u{941}नो प\u{94d}रा\u{902}त"), ("ms", "Wilayah Belluno"), ("nb", "Provinsen Belluno"), ("nl", "Belluno"), ("no", "Provinsen Belluno"), ("pl", "Prowincja Belluno"), ("pt", "Belluno"), ("ro", "Provincia Belluno"), ("ru", "Беллуно"), ("si", "බෙල\u{dca}ල\u{dd4}නෝ පළ\u{dcf}ත"), ("sl", "Belluno"), ("sq", "Provinca e Belunos"), ("sr", "Белуно (округ)"), ("sr_Latn", "Beluno (okrug)"), ("sv", "Belluno"), ("ta", "பெல\u{bcd}லூனோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c46}లూన\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเบลล\u{e39}โน"), ("tr", "Belluno ili"), ("uk", "Провінція Беллуно"), ("ur", "صوبہ بیلونو"), ("uz", "Belluno"), ("vi", "Belluno"), ("zh", "貝盧諾省")]),
                        unofficial_name_list: ["Province of Belluno"].to_vec(),
                    }
                ),
                (
                    "BN",
                    Subdivision{
                        name: "BN",
                        country_alpha2: Alpha2::IT,
                        code: "BN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.1297613), longitude: Some(14.7826208), max_latitude: Some(41.14756209999999), min_latitude: Some(41.09835899999999), max_longitude: Some(14.8103246), min_longitude: Some(14.7350548)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بينيفنتو"), ("be", "правінцыя Беневента"), ("bg", "Беневенто"), ("bn", "বেনিভেন\u{9cd}ত\u{9c1} প\u{9cd}রদেশ"), ("ca", "Província de Benevent"), ("ccp", "𑄝𑄬𑄚𑄬𑄞𑄬𑄚\u{11134}𑄑\u{1112e}"), ("ceb", "Benevento"), ("cs", "Provincie Benevento"), ("da", "Benevento"), ("de", "Provinz Benevento"), ("el", "Μπενεβέντο"), ("en", "Benevento"), ("es", "Benevento"), ("et", "Benevento provints"), ("eu", "Beneventoko probintzia"), ("fa", "استان بنونتو"), ("fi", "Beneventon maakunta"), ("fr", "province de Bénévent"), ("gl", "Provincia de Benevento"), ("gu", "બ\u{ac7}ન\u{ac7}વ\u{ac7}ન\u{acd}ટો પ\u{acd}રા\u{a82}ત"), ("he", "בנוונטו"), ("hi", "ब\u{947}न\u{947}व\u{947}\u{902}टो प\u{94d}रा\u{902}त"), ("hu", "Benevento megye"), ("hy", "Բենևենտո"), ("id", "Provinsi Benevento"), ("it", "provincia di Benevento"), ("ja", "ベネヴェント県"), ("jv", "Provinsi Benevento"), ("ka", "ბენევენტოს პროვინცია"), ("kn", "ಬ\u{cc6}ನ\u{cc6}ವ\u{cc6}ಂಟೋ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "베네벤토 현"), ("lt", "Benevento provincija"), ("lv", "Benevento province"), ("mk", "Беневенто"), ("mr", "ब\u{947}न\u{947}व\u{947}\u{902}टो प\u{94d}रा\u{902}त"), ("ms", "Wilayah Benevento"), ("nb", "Provinsen Benevento"), ("nl", "Benevento"), ("no", "Provinsen Benevento"), ("pl", "Prowincja Benevento"), ("pt", "Benevento"), ("ro", "Provincia Benevento"), ("ru", "Беневенто"), ("si", "බෙනෙවෙන\u{dca}ටෝ පළ\u{dcf}ත"), ("sl", "Benevento"), ("sq", "Provinca e Beneventos"), ("sr", "Беневенто"), ("sr_Latn", "Benevento"), ("sv", "Benevento"), ("ta", "பெனெவெண\u{bcd}டோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c46}న\u{c46}వ\u{c46}ంట\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ซานตา ล\u{e39}ซ\u{e34}จา"), ("tr", "Benevento ili"), ("uk", "Провінція Беневенто"), ("ur", "صوبہ بینیوینتو"), ("uz", "Benevento"), ("vi", "Benevento"), ("zh", "貝內文托省")]),
                        unofficial_name_list: ["Province of Benevento"].to_vec(),
                    }
                ),
                (
                    "BO",
                    Subdivision{
                        name: "BO",
                        country_alpha2: Alpha2::IT,
                        code: "BO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.494887), longitude: Some(11.3426163), max_latitude: Some(44.5561987), min_latitude: Some(44.4420377), max_longitude: Some(11.4337169), min_longitude: Some(11.2296541)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بولونيا"), ("be", "Правінцыя Балоння"), ("bg", "Болоня"), ("bn", "মেট\u{9cd}রোপলিটন³"), ("ca", "Província de Bolonya"), ("ccp", "𑄝𑄣\u{1112e}𑄇\u{11134}𑄚"), ("ceb", "Bologna"), ("cs", "Provincie Bologna"), ("cy", "Talaith Bologna"), ("da", "Bologna"), ("de", "Provinz Bologna"), ("el", "Μπολόνια"), ("en", "Bologna"), ("es", "Bolonia"), ("et", "Bologna provints"), ("eu", "Boloniako probintzia"), ("fa", "استان بولونیا"), ("fi", "Bolognan maakunta"), ("fr", "Province de Bologne"), ("gl", "Provincia de Boloña"), ("gu", "બોલોગ\u{acd}ના મ\u{ac7}ટ\u{acd}રોપોલિટન શહ\u{ac7}ર"), ("he", "בולוניה"), ("hi", "बोलोना म\u{947}ट\u{94d}रोपोलिटन शहर"), ("hu", "Bologna megye"), ("hy", "Բոլոնյա գավառ"), ("id", "Provinsi Bologna"), ("it", "provincia di Bologna"), ("ja", "ボローニャ県"), ("jv", "Provinsi Bologna"), ("ka", "ბოლონიის პროვინცია"), ("kn", "ಬೊಲೊಗ\u{ccd}ನಾ ಮ\u{cc6}ಟ\u{ccd}ರೋಪಾಲ\u{cbf}ಟನ\u{ccd} ನಗರ"), ("ko", "볼로냐 현"), ("lt", "Bolonijos provincija"), ("lv", "Boloņas province"), ("mk", "Болоња"), ("mr", "बोलोन\u{94d}याच\u{947} महानगरीय शहर"), ("ms", "Wilayah Bologna"), ("nb", "Bologna"), ("nl", "Bologna"), ("no", "Provinsen Bologna"), ("pl", "Prowincja Bolonia"), ("pt", "Bolonha"), ("ro", "Provincia Bologna"), ("ru", "Болонья"), ("si", "බොලොග\u{dca}න\u{dcf} වල මෙට\u{dca}\u{200d}රොපොල\u{dd2}ටන\u{dca} නගරය"), ("sl", "Bologna"), ("sq", "Qyteti Metropolitan i Bolonjës"), ("sr", "Болоња"), ("sr_Latn", "Bolonja"), ("sv", "Bologna"), ("ta", "மெட\u{bcd}ரோபொலிட\u{bcd}டன\u{bcd} நகரம\u{bcd} போலோக\u{bcd}ன\u{bbe}"), ("te", "బ\u{c4b}ల\u{c4b}గ\u{c4d}న\u{c3e} మ\u{c46}ట\u{c4d}ర\u{c4b}ప\u{c3e}ల\u{c3f}టన\u{c4d} స\u{c3f}ట\u{c40}"), ("th", "โบโลญญา"), ("tr", "Bologna ili"), ("uk", "Провінція Болонья"), ("ur", "صوبہ بولونیا"), ("uz", "Bologna"), ("vi", "Bologna"), ("yue", "博洛尼亞省"), ("yue_Hans", "博洛尼亚省"), ("zh", "博洛尼亚省")]),
                        unofficial_name_list: ["Metropolitan City of Bologna"].to_vec(),
                    }
                ),
                (
                    "BR",
                    Subdivision{
                        name: "BR",
                        country_alpha2: Alpha2::IT,
                        code: "BR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.6327278), longitude: Some(17.9417616), max_latitude: Some(40.6814259), min_latitude: Some(40.6030344), max_longitude: Some(17.9624638), min_longitude: Some(17.9094458)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة برينديزي"), ("be", "Правінцыя Брындзізі"), ("bg", "Бриндизи"), ("bn", "ব\u{9cd}রিন\u{9cd}ডিসি-র প\u{9cd}রদেশ"), ("ca", "Província de Bríndisi"), ("ccp", "𑄝\u{11133}𑄢\u{11128}𑄚\u{11134}𑄓\u{1112d}𑄥\u{11128}"), ("ceb", "Brindisi"), ("cs", "Provincie Brindisi"), ("da", "Province of Brindisi"), ("de", "Provinz Brindisi"), ("el", "Μπρίντιζι"), ("en", "Brindisi"), ("es", "Brindisi"), ("et", "Brindisi provints"), ("eu", "Brindisiko probintzia"), ("fa", "استان بریندیسی"), ("fi", "Brindisin maakunta"), ("fr", "Province de Brindisi"), ("gl", "Provincia de Brindisi"), ("gu", "બ\u{acd}રિન\u{acd}ડિસિ પ\u{acd}રા\u{a82}ત"), ("he", "ברינדיזי"), ("hi", "ब\u{94d}रि\u{902}दीसी प\u{94d}रा\u{902}त"), ("hu", "Brindisi megye"), ("hy", "Բրինդիզի"), ("id", "Provinsi Brindisi"), ("it", "provincia di Brindisi"), ("ja", "ブリンディジ県"), ("jv", "Provinsi Brindisi"), ("ka", "ბრინდიზის პროვინცია"), ("kn", "ಬ\u{ccd}ರ\u{cbf}ಂಡ\u{cbf}ಸ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "브린디시 현"), ("lt", "Brindizio provincija"), ("lv", "Brindizi province"), ("mr", "ब\u{94d}रि\u{902}डिसि प\u{94d}रा\u{902}त"), ("ms", "Wilayah Brindisi"), ("nb", "Provinsen Brindisi"), ("nl", "Brindisi"), ("no", "Provinsen Brindisi"), ("pl", "Prowincja Brindisi"), ("pt", "Brindisi"), ("ro", "Provincia Brindisi"), ("ru", "Бриндизи"), ("si", "බ\u{dca}\u{200d}ර\u{dd2}න\u{dca}ඩ\u{dd2}ස\u{dd2} පළ\u{dcf}ත"), ("sl", "Brindisi"), ("sq", "Provinca e Brindizit"), ("sr", "Бриндизи"), ("sr_Latn", "Brindizi"), ("sv", "Brindisi"), ("ta", "பிரிந\u{bcd}திசி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c4d}ర\u{c3f}ండ\u{c3f}స\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบร\u{e34}นด\u{e34}ส\u{e34}"), ("tr", "Brindisi ili"), ("uk", "Провінція Бріндізі"), ("ur", "صوبہ بریندیزی"), ("uz", "Brindisi"), ("vi", "Brindisi"), ("zh", "布林迪西省")]),
                        unofficial_name_list: ["Province of Brindisi"].to_vec(),
                    }
                ),
                (
                    "BS",
                    Subdivision{
                        name: "BS",
                        country_alpha2: Alpha2::IT,
                        code: "BS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.5415526), longitude: Some(10.2118019), max_latitude: Some(45.5900636), min_latitude: Some(45.4971508), max_longitude: Some(10.2999996), min_longitude: Some(10.1473578)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بريشا"), ("be", "правінцыя Брэшыя"), ("bg", "Бреша"), ("bn", "ব\u{9cd}রেসিক\u{9be}র প\u{9cd}রদেশ"), ("ca", "Província de Brescia"), ("ccp", "𑄝\u{11133}𑄢𑄬𑄌\u{11134}𑄥\u{11128}𑄠"), ("ceb", "Provincia di Brescia"), ("cs", "Provincie Brescia"), ("da", "Brescia"), ("de", "Provinz Brescia"), ("el", "Μπρέσια"), ("en", "Brescia"), ("es", "Brescia"), ("et", "Brescia provints"), ("eu", "Bresciako probintzia"), ("fa", "استان برشا"), ("fi", "Brescian maakunta"), ("fr", "Province de Brescia"), ("gl", "Provincia de Brescia"), ("gu", "બ\u{acd}ર\u{acd}ર\u{ac7}સિયા પ\u{acd}રા\u{a82}ત"), ("he", "ברשה"), ("hi", "ब\u{94d}र\u{947}सिया प\u{94d}रा\u{902}त"), ("hr", "Brescia"), ("hu", "Brescia megye"), ("hy", "Բրեշիա"), ("id", "Provinsi Brescia"), ("it", "provincia di Brescia"), ("ja", "ブレシア県"), ("jv", "Provinsi Brescia"), ("ka", "ბრეშის პროვინცია"), ("kk", "Брешиа"), ("kn", "ಬ\u{ccd}ರ\u{cc6}ಸ\u{ccd}ಸ\u{cbf}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "브레시아 현"), ("lt", "Brešos provincija"), ("lv", "Brešas province"), ("mk", "Бреша"), ("mr", "ब\u{94d}र\u{947}शिया चा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Brescia"), ("nb", "Provinsen Brescia"), ("nl", "Brescia"), ("no", "Provinsen Brescia"), ("pl", "Prowincja Brescia"), ("pt", "Bréscia"), ("ro", "Provincia Brescia"), ("ru", "Брешиа"), ("si", "බ\u{dca}රෙස\u{dd2}ක\u{dcf} පළ\u{dcf}ත"), ("sk", "Brescia"), ("sl", "Brescia"), ("sq", "Provinca e Breshias"), ("sr", "Бреша"), ("sr_Latn", "Breša"), ("sv", "Brescia"), ("ta", "பிரேசிய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c4d}ర\u{c46}ష\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเบรเซ\u{e35}ย"), ("tr", "Brescia ili"), ("uk", "Провінція Брешія"), ("ur", "صوبہ بریشا"), ("uz", "Brescia"), ("vi", "Brescia"), ("zh", "布雷西亞省")]),
                        unofficial_name_list: ["Province of Brescia"].to_vec(),
                    }
                ),
                (
                    "BT",
                    Subdivision{
                        name: "BT",
                        country_alpha2: Alpha2::IT,
                        code: "BT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.2004543), longitude: Some(16.2051484), max_latitude: Some(41.4417109), min_latitude: Some(40.897719), max_longitude: Some(16.5422492), min_longitude: Some(15.8702517)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بارليتا أندريا تراني"), ("be", "правінцыя Барлета-Андрыя-Трані"), ("bg", "Барлета-Андрия-Трани"), ("bn", "ব\u{9be}র\u{9cd}লেট\u{9be}-আন\u{9cd}দ\u{9cd}রিয\u{9bc}\u{9be} ত\u{9be}রনি-এর প\u{9cd}রদেশ"), ("ca", "Província de Barleta-Andria-Trani"), ("ccp", "𑄝𑄢\u{11134}𑄣𑄬𑄑-𑄃𑄚\u{11134}𑄓\u{11133}𑄢\u{11128}𑄠-𑄑\u{11133}𑄢𑄚\u{11128}"), ("ceb", "Barletta-Andria-Trani"), ("cs", "Provincie Barletta-Andria-Trani"), ("da", "Province of Barletta-Andria-Trani"), ("de", "Provinz Barletta-Andria-Trani"), ("el", "Μπαρλέττα-Άντρια-Τράνι"), ("en", "Barletta-Andria-Trani"), ("es", "Barletta-Andria-Trani"), ("et", "Barletta-Andria-Trani provints"), ("eu", "Barletta-Andria-Traniko probintzia"), ("fa", "استان بارلتا-آندریا-ترانی"), ("fi", "Barletta-Andria-Tranin maakunta"), ("fr", "province de Barletta-Andria-Trani"), ("gl", "Provincia de Barletta-Andria-Trani"), ("gu", "બ\u{ac7}ર\u{acd}લ\u{ac7}ટા-એન\u{acd}ડ\u{acd}રીયા-ટ\u{acd}રાની પ\u{acd}રા\u{a82}ત"), ("he", "בארלטה-אנדריה-טראני"), ("hi", "बारल\u{948}टा-अ\u{902}द\u{94d}रिया-ट\u{94d}रानी प\u{94d}रा\u{902}त"), ("hu", "Barletta-Andria-Trani megye"), ("hy", "Բարլետա-Անդրիա-Տրանի"), ("id", "Provinsi Barletta-Andria-Trani"), ("it", "provincia di Barletta-Andria-Trani"), ("jv", "Provinsi Barletta-Andria-Trani"), ("ka", "ბარლეტა-ანდრია-ტრანის პროვინცია"), ("kn", "ಬಾರ\u{ccd}ಲ\u{cc6}ಟ\u{ccd}ಟಾ ಆಂಡ\u{ccd}ರ\u{cbf}ಯಾ-ಟ\u{ccd}ರಾನ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "바를레타안드리아트라니 현"), ("lt", "Barletos-Andrijos-Tranio provincija"), ("lv", "Barletas-Andrijas-Trani province"), ("mr", "ब\u{945}ल\u{947}टा-आ\u{902}द\u{94d}रिया-ट\u{94d}रानी प\u{94d}रा\u{902}त"), ("ms", "Wilayah Barletta-Andria-Trani"), ("nb", "Provinsen Barletta-Andria-Trani"), ("nl", "Barletta-Andria-Trani"), ("no", "Provinsen Barletta-Andria-Trani"), ("pl", "Prowincja Barletta-Andria-Trani"), ("pt", "Barletta-Andria-Trani"), ("ro", "Provincia Barletta-Andria-Trani"), ("ru", "Барлетта-Андрия-Трани"), ("si", "බර\u{dca}ලෙට\u{dcf}-අන\u{dca}ද\u{dca}\u{200d}ර\u{dd2}ය\u{dcf} ට\u{dca}\u{200d}ර\u{dcf}න\u{dd2} පළ\u{dcf}ත"), ("sl", "Barletta-Andria-Trani"), ("sq", "Provinca e Barletta-Andria-Tranit"), ("sr", "Барлета-Андрија-Трани"), ("sr_Latn", "Barleta-Andrija-Trani"), ("sv", "Barletta-Andria-Trani"), ("ta", "ப\u{bbe}ர\u{bcd}லேட\u{bcd}ட\u{bbe} -ஆண\u{bcd}ட\u{bcd}ரிய\u{bbe} -த\u{bcd}ர\u{bbe}ணி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c3e}ర\u{c4d}ల\u{c46}ట\u{c3e}-ఆండ\u{c4d}ర\u{c3f}య\u{c3e}-ట\u{c4d}ర\u{c3e}న\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบาร\u{e4c}เลตตา-อ\u{e31}นดร\u{e35}อา-ตราน\u{e35}"), ("tr", "Barletta-Andria-Trani ili"), ("uk", "Провінція Барлетта-Андрія-Трані"), ("ur", "صوبہ بارلیتا-آندریا-ترانی"), ("uz", "Barletta-Andria-Trani"), ("vi", "Barletta-Andria-Trani"), ("zh", "巴爾萊塔-安德里亞-特蘭尼省")]),
                        unofficial_name_list: ["Province of Barletta-Andria-Trani"].to_vec(),
                    }
                ),
                (
                    "BZ",
                    Subdivision{
                        name: "BZ",
                        country_alpha2: Alpha2::IT,
                        code: "BZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4982953), longitude: Some(11.3547582), max_latitude: Some(46.5154347), min_latitude: Some(46.4630385), max_longitude: Some(11.3806479), min_longitude: Some(11.3136675)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousProvince,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Suid-Tirool"), ("ar", "مقاطعة بولسانو"), ("az", "Cənubi Tirol"), ("be", "Паўднёвы Ціроль"), ("bg", "Южен Тирол"), ("ca", "Tirol del Sud"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄑\u{1112d}𑄢\u{1112e}𑄣\u{11134}"), ("ceb", "Bolzano"), ("cs", "Autonomní provincie Bolzano"), ("cy", "Bolzano"), ("da", "Sydtyrol"), ("de", "Südtirol"), ("el", "Αυτόνομη επαρχία του Μπολτσάνο"), ("en", "South Tyrol"), ("es", "Bolzano"), ("et", "Bolzano provints"), ("eu", "Bozen-Hego Tirolgo probintzia autonomoa"), ("fa", "التو آدیجه"), ("fi", "Etelä-Tiroli"), ("fr", "Province autonome de Bolzano"), ("gl", "Provincia autónoma de Bolzano"), ("he", "דרום טירול"), ("hr", "Autonomna pokrajina Bocen"), ("hu", "Bolzano autonóm megye"), ("hy", "Բոլցանո"), ("id", "Provinsi Bolzano-Bozen"), ("it", "provincia autonoma di Bolzano"), ("ja", "ボルツァーノ自治県"), ("jv", "Provinsi otonom Bolzano"), ("ka", "ბოლცანო (პროვინცია)"), ("ko", "볼차노 현"), ("lt", "Pietų Tirolis"), ("lv", "Bolcāno autonomā province"), ("mk", "Јужен Тирол"), ("nb", "Sør-Tirol"), ("nl", "Zuid-Tirol"), ("no", "Sør-Tirol"), ("pl", "Prowincja Bolzano"), ("pt", "Província autónoma de Bolzano"), ("ro", "Provincia Autonomă Bolzano"), ("ru", "Больцано"), ("sk", "Autonómna provincia Bolzano – Horná Adiža"), ("sl", "Južna Tirolska"), ("sq", "Provinca Autonome e Bolzanos"), ("sr", "Болцано"), ("sr_Latn", "Bolcano"), ("sv", "Sydtyrolen"), ("tr", "Güney Tirol"), ("uk", "Провінція Больцано"), ("ur", "جنوبی ٹائرول"), ("vi", "Bolzano-Bozen"), ("zh", "波爾扎諾自治省")]),
                        unofficial_name_list: ["Autonome Provinz Bozen – Südtirol", "Provincia autonoma di Bolzano – Alto Adige", "Provincia autonoma di Bolzano – Alto Adige", "South Tyrol"].to_vec(),
                    }
                ),
                (
                    "CA",
                    Subdivision{
                        name: "CA",
                        country_alpha2: Alpha2::IT,
                        code: "CA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.2238411), longitude: Some(9.1216613), max_latitude: Some(39.26570299999999), min_latitude: Some(39.1851057), max_longitude: Some(9.177219299999999), min_longitude: Some(9.079536700000002)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كالياري"), ("bg", "Каляри"), ("bn", "ক\u{9cd}য\u{9be}গ\u{9cd}লিয\u{9bc}\u{9be}রি প\u{9cd}রদেশ"), ("ca", "Província de Càller"), ("ccp", "𑄇\u{11133}𑄠𑄉\u{11133}𑄣\u{11128}𑄠𑄢\u{11128}"), ("ceb", "Provincia di Cagliari"), ("cs", "Provincie Cagliari"), ("da", "Cagliari"), ("de", "Provinz Cagliari"), ("el", "Κάλιαρι"), ("en", "Cagliari"), ("es", "Cagliari"), ("et", "Cagliari provints"), ("eu", "Cagliariko probintzia"), ("fa", "استان کالیاری"), ("fi", "Cagliarin maakunta"), ("fr", "Province de Cagliari"), ("gl", "Provincia de Cagliari"), ("gu", "ક\u{ac7}ગ\u{acd}લિઆરી પ\u{acd}રા\u{a82}ત"), ("he", "קליארי"), ("hi", "क\u{948}गलिअरी प\u{94d}रा\u{902}त"), ("hu", "Cagliari megye"), ("hy", "Կալիարի"), ("id", "Provinsi Cagliari"), ("it", "provincia di Cagliari"), ("ja", "カリャリ県"), ("jv", "Provinsi Cagliari"), ("ka", "კალიარის პროვინცია"), ("kn", "ಕ\u{ccd}ಯಾಗ\u{ccd}ಲ\u{cbf}ಯಾರ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "칼리아리 현"), ("lt", "Kaljario provincija"), ("lv", "Kaljāri province"), ("mr", "क\u{945}ग\u{94d}लियारी प\u{94d}रा\u{902}त"), ("ms", "Wilayah Cagliari"), ("nb", "Cagliari"), ("nl", "Cagliari"), ("no", "Provinsen Cagliari"), ("pl", "Prowincja Cagliari"), ("pt", "Cagliari"), ("ro", "Provincia Cagliari"), ("ru", "Кальяри"), ("si", "කග\u{dca}ල\u{dd2}ය\u{dcf}ර\u{dd2} පළ\u{dcf}ත"), ("sl", "Cagliari"), ("sr", "Каљари"), ("sr_Latn", "Kaljari"), ("sv", "Cagliari"), ("ta", "க\u{bbe}க\u{bcd}ளியரி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}గ\u{c4d}ల\u{c3f}య\u{c3e}ర\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกาลยาร\u{e35}"), ("tr", "Cagliari ili"), ("uk", "Провінція Кальярі"), ("ur", "صوبہ کالیاری"), ("uz", "Cagliari"), ("vi", "Cagliari"), ("yue", "卡利亞里省"), ("yue_Hans", "卡利亚里省"), ("zh", "卡利亞里省")]),
                        unofficial_name_list: ["Ciudad Metropolitana de Cagliari", "Metropolitan City of Cagliari"].to_vec(),
                    }
                ),
                (
                    "CB",
                    Subdivision{
                        name: "CB",
                        country_alpha2: Alpha2::IT,
                        code: "CB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.5602544), longitude: Some(14.6627161), max_latitude: Some(41.5741618), min_latitude: Some(41.540825), max_longitude: Some(14.6924148), min_longitude: Some(14.643634)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كامبوباسو"), ("be", "правінцыя Кампабаса"), ("bg", "Кампобасо"), ("bn", "ক\u{9cd}য\u{9be}\u{9be}ম\u{9cd}পোব\u{9be}স\u{9cd}যো-এর প\u{9cd}রদেশ"), ("ca", "Província de Campobasso"), ("ccp", "𑄇\u{11133}𑄠𑄟\u{11134}𑄛\u{11127}𑄝𑄥\u{1112e}"), ("ceb", "Provincia di Campobasso"), ("cs", "Provincie Campobasso"), ("da", "Province of Campobasso"), ("de", "Provinz Campobasso"), ("el", "Καμπομπάσο"), ("en", "Campobasso"), ("es", "Campobasso"), ("et", "Campobasso provints"), ("eu", "Campobassoko probintzia"), ("fa", "استان کامپوباسو"), ("fi", "Campobasson maakunta"), ("fr", "Province de Campobasso"), ("gl", "Provincia de Campobasso"), ("gu", "ક\u{ac7}મ\u{acd}પોબાસ\u{acd}સો પ\u{acd}રા\u{a82}ત"), ("he", "קמפובאסו"), ("hi", "क\u{948}\u{902}पोबासो प\u{94d}रा\u{902}त"), ("hr", "Campobasso"), ("hu", "Campobasso megye"), ("hy", "Կամպոբասո"), ("id", "Provinsi Campobasso"), ("it", "provincia di Campobasso"), ("ja", "カンポバッソ県"), ("ka", "კამპობასოს პროვინცია"), ("kn", "ಕ\u{ccd}ಯಾಂಪೊಬಾಸ\u{ccd}ಸೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "캄포바소 현"), ("lt", "Kampobaso provincija"), ("lv", "Kampobaso province"), ("mk", "Кампобасо"), ("mr", "क\u{945}म\u{94d}पबॉस\u{94d}सो प\u{94d}रा\u{902}त"), ("ms", "Wilayah Campobasso"), ("nb", "Provinsen Campobasso"), ("nl", "Campobasso"), ("no", "Provinsen Campobasso"), ("pl", "Prowincja Campobasso"), ("pt", "Campobasso"), ("ro", "Provincia Campobasso"), ("ru", "Кампобассо"), ("si", "කැම\u{dca}පොබස\u{dca}සෝ පළ\u{dcf}ත"), ("sl", "Campobasso"), ("sq", "Provinca e Kampobasos"), ("sr", "Кампобасо"), ("sr_Latn", "Kampobaso"), ("sv", "Campobasso"), ("ta", "க\u{bbe}ம\u{bcd}பொபஸ\u{bcd}ஸோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}ంప\u{c4b}బ\u{c3e}స\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e31}มโปบ\u{e31}สโซ"), ("tr", "Campobasso ili"), ("uk", "Провінція Кампобассо"), ("ur", "صوبہ کامپوباسو"), ("uz", "Campobasso"), ("vi", "Campobasso"), ("zh", "坎波巴索省")]),
                        unofficial_name_list: ["Province of Campobasso"].to_vec(),
                    }
                ),
                (
                    "CE",
                    Subdivision{
                        name: "CE",
                        country_alpha2: Alpha2::IT,
                        code: "CE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.0723484), longitude: Some(14.3311337), max_latitude: Some(41.1125489), min_latitude: Some(41.0487195), max_longitude: Some(14.3785483), min_longitude: Some(14.2981427)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كازيرتا"), ("be", "правінцыя Казерта"), ("bg", "Казерта"), ("bn", "ক\u{9cd}য\u{9be}স\u{9be}র\u{9cd}ট\u{9be}-এর প\u{9cd}রদেশ"), ("ca", "Província de Caserta"), ("ccp", "𑄇\u{11133}𑄠𑄥𑄢\u{11134}𑄑"), ("ceb", "Caserta"), ("cs", "Provincie Caserta"), ("da", "Caserta"), ("de", "Provinz Caserta"), ("el", "Καζέρτα"), ("en", "Caserta"), ("es", "Caserta"), ("et", "Caserta provints"), ("eu", "Casertako probintzia"), ("fa", "استان کسرتا"), ("fi", "Casertan maakunta"), ("fr", "Province de Caserte"), ("gl", "Provincia de Caserta"), ("gu", "કાસ\u{acd}ટ\u{ac7}રા પ\u{acd}રા\u{a82}ત"), ("he", "קזרטה"), ("hi", "कस\u{947}र\u{94d}टा प\u{94d}रा\u{902}त"), ("hu", "Caserta megye"), ("hy", "Կազերտա"), ("id", "Provinsi Caserta"), ("it", "provincia di Caserta"), ("ja", "カゼルタ県"), ("jv", "Provinsi Caserta"), ("ka", "კაზერტის პროვინცია"), ("kn", "ಕ\u{ccd}ಯಾಸ\u{cc6}ರ\u{ccd}ಟಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "카세르타 현"), ("lt", "Kazertos provincija"), ("lv", "Kazertas province"), ("mk", "Казерта"), ("mr", "क\u{945}सर\u{94d}टा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Caserta"), ("nb", "Provinsen Caserta"), ("nl", "Caserta"), ("no", "Provinsen Caserta"), ("pl", "Prowincja Caserta"), ("pt", "Caserta"), ("ro", "Provincia Caserta"), ("ru", "Казерта"), ("si", "කසේර\u{dca}ට\u{dcf} පළ\u{dcf}ත"), ("sl", "Caserta"), ("sq", "Provinca e Kazertës"), ("sr", "Казерта"), ("sr_Latn", "Kazerta"), ("sv", "Caserta"), ("ta", "ஸ\u{bbe}சேர\u{bcd}ட\u{bcd}ட\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}స\u{c46}ర\u{c4d}ట\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดคาเซอร\u{e4c}ทา"), ("tr", "Caserta ili"), ("uk", "Провінція Казерта"), ("ur", "صوبہ کاسیرتا"), ("uz", "Caserta"), ("vi", "Caserta"), ("zh", "卡塞塔省")]),
                        unofficial_name_list: ["Province of Caserta"].to_vec(),
                    }
                ),
                (
                    "CH",
                    Subdivision{
                        name: "CH",
                        country_alpha2: Alpha2::IT,
                        code: "CH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.347886), longitude: Some(14.1635845), max_latitude: Some(42.3962391), min_latitude: Some(42.3265908), max_longitude: Some(14.2106223), min_longitude: Some(14.1011205)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كييتي"), ("bg", "Киети"), ("bn", "চিতি-র প\u{9cd}রদেশ"), ("ca", "Província de Chieti"), ("ccp", "𑄌\u{1112d}𑄠𑄬𑄑\u{11128}"), ("ceb", "Chieti"), ("cs", "Provincie Chieti"), ("da", "Chieti"), ("de", "Provinz Chieti"), ("el", "Επαρχία του Κιέτι"), ("en", "Chieti"), ("es", "Chieti"), ("et", "Chieti provints"), ("eu", "Chietiko probintzia"), ("fa", "استان کییتی"), ("fi", "Chietin maakunta"), ("fr", "Province de Chieti"), ("gl", "Provincia de Chieti"), ("gu", "ચિએટી પ\u{acd}રા\u{a82}ત"), ("he", "קייטי"), ("hi", "क\u{94d}हिती प\u{94d}रा\u{902}त"), ("hr", "Chieti"), ("hu", "Chieti megye"), ("hy", "Կիետի"), ("id", "Provinsi Chieti"), ("it", "provincia di Chieti"), ("ja", "キエーティ県"), ("jv", "Provinsi Chieti"), ("ka", "კიეტის პროვინცია"), ("kn", "ಚ\u{cbf}ಯ\u{cc6}ಟ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "키에티 현"), ("lt", "Kječio provincija"), ("lv", "Kjeti province"), ("mk", "Кјети"), ("mr", "चिएटी प\u{94d}रा\u{902}त"), ("ms", "Wilayah Chieti"), ("nb", "Provinsen Chieti"), ("nl", "Chieti"), ("no", "Provinsen Chieti"), ("pl", "Prowincja Chieti"), ("pt", "Chieti"), ("ro", "Provincia Chieti"), ("ru", "Кьети"), ("si", "චය\u{dd2}ට\u{dd2} පළ\u{dcf}ත"), ("sl", "Chieti"), ("sq", "Provinca e Kietit"), ("sr", "Кјети"), ("sr_Latn", "Kjeti"), ("sv", "Chieti"), ("ta", "செய\u{bcd}டி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "చ\u{c3f}య\u{c47}ట\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดช\u{e34}เอต\u{e34}"), ("tr", "Chieti ili"), ("uk", "Провінція Кʼєті"), ("ur", "صوبہ کئیتی"), ("uz", "Chieti"), ("vi", "Chieti"), ("zh", "基耶蒂省")]),
                        unofficial_name_list: ["Province of Chieti"].to_vec(),
                    }
                ),
                (
                    "CL",
                    Subdivision{
                        name: "CL",
                        country_alpha2: Alpha2::IT,
                        code: "CL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.4901115), longitude: Some(14.0628928), max_latitude: Some(37.5120633), min_latitude: Some(37.459473), max_longitude: Some(14.0996924), min_longitude: Some(14.0140048)}),
                        comments: None,
                        subdivision_type: SubdivisionType::FreeMunicipalConsortium,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كالتانيسيتا"), ("be", "правінцыя Кальтанісета"), ("bg", "Калтанисета"), ("bn", "ক\u{9cd}য\u{9be}ল\u{9cd}ট\u{9be}নিসসেট\u{9be}র প\u{9cd}রদেশ"), ("ca", "Província de Caltanissetta"), ("ccp", "𑄇\u{11133}𑄠𑄣\u{11134}𑄑𑄚\u{11128}𑄥𑄬𑄑\u{11133}𑄦"), ("ceb", "Provincia di Caltanissetta"), ("cs", "Provincie Caltanissetta"), ("da", "Province of Caltanissetta"), ("de", "Provinz Caltanissetta"), ("el", "Καλτανισέττα"), ("en", "Caltanissetta"), ("es", "Caltanissetta"), ("et", "Caltanissetta provints"), ("eu", "Caltanissettako probintzia"), ("fa", "استان کالتانیستا"), ("fi", "Caltanissettan maakunta"), ("fr", "Province de Caltanissetta"), ("gl", "Provincia de Caltanissetta"), ("gu", "ક\u{ac7}લ\u{acd}ટાનિસ\u{ac7}ટા પ\u{acd}રા\u{a82}ત"), ("he", "קלטניסטה"), ("hi", "स\u{947}ल\u{94d}टानिस\u{947}ट\u{94d}टा प\u{94d}रा\u{902}त"), ("hr", "Caltanissetta"), ("hu", "Caltanissetta megye"), ("hy", "Կալտանիսետա"), ("id", "Provinsi Caltanissetta"), ("it", "provincia di Caltanissetta"), ("ja", "カルタニッセッタ県"), ("jv", "Provinsi Caltanissetta"), ("ka", "კალტანისეტის პროვინცია"), ("kn", "ಕ\u{ccd}ಯಾಲ\u{ccd}ಟಾನ\u{cbf}ಸ\u{cc6}ಟ\u{ccd}ಟಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "칼타니세타 현"), ("lt", "Kaltanisetos provincija"), ("lv", "Kaltanisetas province"), ("mr", "क\u{945}ल\u{94d}ट\u{902}सीत प\u{94d}रा\u{902}त"), ("ms", "Wilayah Caltanissetta"), ("nb", "Provinsen Caltanissetta"), ("nl", "Caltanissetta"), ("no", "Provinsen Caltanissetta"), ("pl", "Prowincja Caltanissetta"), ("pt", "Caltanissetta"), ("ro", "Provincia Caltanissetta"), ("ru", "Кальтаниссетта"), ("si", "කල\u{dca}ටන\u{dd2}සේට\u{dcf} පළ\u{dcf}ත"), ("sl", "Caltanissetta"), ("sq", "Provinca e Kaltanisetës"), ("sr", "Калтанисета"), ("sr_Latn", "Kaltaniseta"), ("sv", "Caltanissetta"), ("ta", "க\u{bbe}ல\u{bcd}டனிசிஸ\u{bcd}ட\u{bcd}ட\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}ల\u{c4d}ట\u{c3e}న\u{c3f}స\u{c46}ట\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดค\u{e31}ลทาน\u{e34}สเซ\u{e47}ทต\u{e49}า"), ("tr", "Caltanissetta ili"), ("uk", "Провінція Кальтаніссетта"), ("ur", "صوبہ کالتانیسیتا"), ("uz", "Caltanissetta"), ("vi", "Caltanissetta"), ("zh", "卡爾塔尼塞塔省")]),
                        unofficial_name_list: ["Province of Caltanissetta"].to_vec(),
                    }
                ),
                (
                    "CN",
                    Subdivision{
                        name: "CN",
                        country_alpha2: Alpha2::IT,
                        code: "CN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.3844766), longitude: Some(7.5426711), max_latitude: Some(44.4309618), min_latitude: Some(44.3532038), max_longitude: Some(7.603918799999999), min_longitude: Some(7.508458699999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كونيو"), ("be", "Кунеа"), ("bg", "Кунео"), ("bn", "ক\u{9c1}নেও-এর প\u{9cd}রদেশ"), ("ca", "Província de Cuneo"), ("ccp", "𑄇\u{11128}𑄅\u{1112a}𑄚\u{11128}𑄃\u{1112e}"), ("ceb", "Provincia di Cuneo"), ("cs", "Provincie Cuneo"), ("da", "Cuneo"), ("de", "Provinz Cuneo"), ("el", "Κούνεο"), ("en", "Cuneo"), ("es", "Cuneo"), ("et", "Cuneo provints"), ("eu", "Cuneoko probintzia"), ("fa", "استان کونیو"), ("fi", "Cuneon maakunta"), ("fr", "Province de Coni"), ("gl", "Provincia de Cuneo"), ("gu", "ક\u{ac1}ન\u{ac7}ઓ પ\u{acd}રા\u{a82}ત"), ("he", "קונאו"), ("hi", "क\u{94d}य\u{941}नियो प\u{94d}रा\u{902}त"), ("hu", "Cuneo megye"), ("id", "Provinsi Cuneo"), ("is", "Cuneo"), ("it", "Provincia di Cuneo"), ("ja", "クーネオ県"), ("jv", "Provinsi Cuneo"), ("ka", "კუნეოს პროვინცია"), ("kn", "ಕ\u{ccd}ಯ\u{cc2}ನ\u{cbf}ಯೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "쿠네오 현"), ("lt", "Kunėjo provincija"), ("lv", "Kuneo province"), ("mk", "Кунео"), ("mr", "क\u{941}न\u{947}ओच\u{947} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Cuneo"), ("nb", "Provinsen Cuneo"), ("nl", "Cuneo"), ("no", "Provinsen Cuneo"), ("pl", "Prowincja Cuneo"), ("pt", "Cuneo"), ("ro", "Provincia Cuneo"), ("ru", "Кунео"), ("si", "ක\u{dd4}න\u{dd2}යෝ පළ\u{dcf}ත"), ("sl", "Cuneo"), ("sq", "Provinca e Kuneos"), ("sr", "Кунео"), ("sr_Latn", "Kuneo"), ("sv", "Cuneo"), ("ta", "குனிய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "కున\u{c3f}య\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "โอชานา"), ("tr", "Cuneo ili"), ("uk", "Провінція Кунео"), ("ur", "صوبہ کونیو"), ("uz", "Cuneo"), ("vi", "Cuneo"), ("zh", "庫內奧省")]),
                        unofficial_name_list: ["Province of Cuneo"].to_vec(),
                    }
                ),
                (
                    "CO",
                    Subdivision{
                        name: "CO",
                        country_alpha2: Alpha2::IT,
                        code: "CO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.8080597), longitude: Some(9.0851765), max_latitude: Some(45.8421882), min_latitude: Some(45.75888320000001), max_longitude: Some(9.136950299999999), min_longitude: Some(9.033906100000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كومو"), ("be", "правінцыя Кома"), ("bg", "Комо"), ("bn", "কোমে-এর প\u{9cd}রদেশ"), ("ca", "Província de Como"), ("ccp", "𑄇\u{1112e}𑄟\u{1112e}"), ("ceb", "Provincia di Como"), ("cs", "Provincie Como"), ("da", "Como"), ("de", "Provinz Como"), ("el", "Επαρχία του Κόμο"), ("en", "Como"), ("es", "Como"), ("et", "Como provints"), ("eu", "Comoko probintzia"), ("fa", "استان کومو"), ("fi", "Comon maakunta"), ("fr", "Province de Côme"), ("gl", "Provincia de Como"), ("gu", "કોમો પ\u{acd}રા\u{a82}ત"), ("he", "קומו"), ("hi", "कोमो प\u{94d}रा\u{902}त"), ("hr", "Como"), ("hu", "Como megye"), ("hy", "Կոմո"), ("id", "Provinsi Como"), ("it", "provincia di Como"), ("ja", "コモ県"), ("jv", "Provinsi Como"), ("ka", "კომოს პროვინცია"), ("kn", "ಕೊಮೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "코모 현"), ("lt", "Komo provincija"), ("lv", "Komo province"), ("mk", "Комо"), ("mr", "कोमो प\u{94d}रा\u{902}त"), ("ms", "Wilayah Como"), ("nb", "Provinsen Como"), ("nl", "Como"), ("no", "Provinsen Como"), ("pl", "Prowincja Como"), ("pt", "Como"), ("ro", "Provincia Como"), ("ru", "Комо"), ("si", "කොමෝ පළ\u{dcf}ත"), ("sk", "Como"), ("sl", "Como"), ("sq", "Provinca e Komos"), ("sr", "Комо"), ("sr_Latn", "Komo"), ("sv", "Como"), ("ta", "கோமோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c4b}మ\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโคโม"), ("tr", "Como ili"), ("uk", "Провінція Комо"), ("ur", "صوبہ کومو"), ("uz", "Como"), ("vi", "Como"), ("zh", "科莫省")]),
                        unofficial_name_list: ["Province of Como"].to_vec(),
                    }
                ),
                (
                    "CR",
                    Subdivision{
                        name: "CR",
                        country_alpha2: Alpha2::IT,
                        code: "CR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.133249), longitude: Some(10.0226511), max_latitude: Some(45.1569568), min_latitude: Some(45.1245891), max_longitude: Some(10.0588677), min_longitude: Some(9.9863304)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كريمونا"), ("be", "Крэмона"), ("bg", "Кремона"), ("bn", "ক\u{9cd}রিমোন\u{9be}-র প\u{9cd}রদেশ"), ("ca", "Província de Cremona"), ("ccp", "𑄇\u{11133}𑄢𑄬𑄟\u{11127}𑄚\u{11134}"), ("ceb", "Provincia di Cremona"), ("cs", "Provincie Cremona"), ("da", "Cremona"), ("de", "Provinz Cremona"), ("el", "Επαρχία της Κρεμόνα"), ("en", "Cremona"), ("es", "Cremona"), ("et", "Cremona provints"), ("eu", "Cremonako probintzia"), ("fa", "استان کریمونا"), ("fi", "Cremonan maakunta"), ("fr", "Province de Crémone"), ("gl", "Provincia de Cremona"), ("gu", "ક\u{acd}ર\u{ac7}મોના પ\u{acd}રા\u{a82}ત"), ("he", "קרמונה"), ("hi", "क\u{94d}र\u{947}मोना प\u{94d}रा\u{902}त"), ("hu", "Cremona megye"), ("hy", "Կրեմոնա"), ("id", "Provinsi Cremona"), ("it", "provincia di Cremona"), ("ja", "クレモナ県"), ("jv", "Provinsi Cremona"), ("ka", "კრემონის პროვინცია"), ("kn", "ಕ\u{ccd}ರ\u{cc6}ಮೋನಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "크레모나 현"), ("lt", "Kremonos provincija"), ("lv", "Kremonas province"), ("mr", "क\u{94d}रिमोना प\u{94d}रा\u{902}त"), ("ms", "Wilayah Cremona"), ("nb", "Provinsen Cremona"), ("nl", "Cremona"), ("no", "Provinsen Cremona"), ("pl", "Prowincja Cremona"), ("pt", "Cremona"), ("ro", "Provincia Cremona"), ("ru", "Кремона"), ("si", "ක\u{dca}රෙමෝන\u{dcf} පළ\u{dcf}ත"), ("sk", "Cremona"), ("sl", "Cremona"), ("sq", "Provinca e Kremonës"), ("sr", "Кремона"), ("sr_Latn", "Kremona"), ("sv", "Cremona"), ("ta", "கிரேம\u{bbe}ன ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c4d}ర\u{c46}మ\u{c4b}న\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เครโมนา"), ("tr", "Cremona ili"), ("uk", "Провінція Кремона"), ("ur", "صوبہ کریمونا"), ("uz", "Cremona"), ("vi", "Cremona"), ("zh", "克雷莫納省")]),
                        unofficial_name_list: ["Province of Cremona"].to_vec(),
                    }
                ),
                (
                    "CS",
                    Subdivision{
                        name: "CS",
                        country_alpha2: Alpha2::IT,
                        code: "CS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.2982629), longitude: Some(16.2537357), max_latitude: Some(39.3326104), min_latitude: Some(39.2769898), max_longitude: Some(16.2850572), min_longitude: Some(16.2195601)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Cosenza"), ("ar", "مقاطعة كوزنسا"), ("be", "правінцыя Казенца"), ("bg", "Козенца"), ("bn", "কোসেঞ\u{9cd}জ\u{9be}-এর প\u{9cd}রদেশ"), ("ca", "Província de Cosenza"), ("ccp", "𑄇\u{1112e}𑄥𑄬𑄚\u{11134}𑄎"), ("ceb", "Cosenza"), ("cs", "Provincie Cosenza"), ("da", "Province of Cosenza"), ("de", "Provinz Cosenza"), ("el", "Κοζέντσα"), ("en", "Cosenza"), ("es", "Cosenza"), ("et", "Cosenza provints"), ("eu", "Cosenzako probintzia"), ("fa", "استان کزنتزا"), ("fi", "Cosenzan maakunta"), ("fr", "Province de Cosenza"), ("gl", "Provincia de Cosenza"), ("gu", "કોસ\u{ac7}\u{a82}ઝા પ\u{acd}રા\u{a82}ત"), ("he", "קוזנצה"), ("hi", "कोस\u{947}\u{902}ज\u{93c}ा प\u{94d}रा\u{902}त"), ("hr", "Cosenza"), ("hu", "Cosenza megye"), ("hy", "Կոզենցա"), ("id", "Provinsi Cosenza"), ("it", "provincia di Cosenza"), ("ja", "コゼンツァ県"), ("jv", "Provinsi Cosenza"), ("ka", "კოზენცის პროვინცია"), ("kn", "ಕೋಸ\u{cc6}ನ\u{ccd}ಜ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "코센차 현"), ("lt", "Kozencos provincija"), ("lv", "Kozencas province"), ("mr", "कोस\u{947}\u{902}जा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Cosenza"), ("nb", "Provinsen Cosenza"), ("nl", "Cosenza"), ("no", "Provinsen Cosenza"), ("pl", "Prowincja Cosenza"), ("pt", "Cosenza"), ("ro", "Provincia Cosenza"), ("ru", "Козенца"), ("si", "කොසේන\u{dca}ස\u{dcf} පළ\u{dcf}ත"), ("sl", "Cosenza"), ("sq", "Provinca e Kozencës"), ("sr", "Козенца"), ("sr_Latn", "Kozenca"), ("sv", "Cosenza"), ("ta", "கோசேன\u{bcd}ஜ\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c4b}స\u{c46}ంజ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโคเซนซา"), ("tr", "Cosenza ili"), ("uk", "Провінція Козенца"), ("ur", "صوبہ کوزینتسا"), ("uz", "Cosenza"), ("vi", "Cosenza"), ("zh", "科森札省")]),
                        unofficial_name_list: ["Province of Cosenza"].to_vec(),
                    }
                ),
                (
                    "CT",
                    Subdivision{
                        name: "CT",
                        country_alpha2: Alpha2::IT,
                        code: "CT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.5078772), longitude: Some(15.0830304), max_latitude: Some(37.560876), min_latitude: Some(37.4205463), max_longitude: Some(15.1259294), min_longitude: Some(15.0290963)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كاتانيا"), ("be", "Катанія"), ("bg", "Катания"), ("bn", "ক\u{9cd}য\u{9be}ট\u{9be}নিয\u{9bc}\u{9be}র প\u{9cd}রদেশ"), ("ca", "Província de Catània"), ("ccp", "𑄇\u{11133}𑄠𑄑𑄚\u{11128}𑄠"), ("ceb", "Catania (lalawigan)"), ("cs", "Provincie Catania"), ("da", "Province of Catania"), ("de", "Provinz Catania"), ("el", "Κατάνια"), ("en", "Catania"), ("es", "Catania"), ("et", "Catania provints"), ("eu", "Cataniako probintzia"), ("fa", "استان کاتانیا"), ("fi", "Catanian maakunta"), ("fr", "Province de Catane"), ("gl", "Provincia de Catania"), ("gu", "ક\u{ac7}ટ\u{ac7}નિયાના પ\u{acd}રા\u{a82}ત"), ("he", "קטניה"), ("hi", "कटानिया प\u{94d}रा\u{902}त"), ("hr", "Catania"), ("hu", "Catania megye"), ("hy", "Կատանիա"), ("id", "Provinsi Catania"), ("it", "provincia di Catania"), ("ja", "カターニア県"), ("jv", "Provinsi Catania"), ("ka", "კატანიის პროვინცია"), ("kn", "ಕ\u{cc6}ಟಾನ\u{cbf}ಯ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "카타니아 현"), ("lt", "Katanijos provincija"), ("lv", "Katānijas province"), ("mk", "Катанија"), ("mr", "क\u{947}ट\u{947}नियाचा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Catania"), ("nb", "Catania"), ("nl", "Catania"), ("no", "Provinsen Catania"), ("pl", "Prowincja Katania"), ("pt", "Catânia"), ("ro", "Provincia Catania"), ("ru", "Катания"), ("si", "කටන\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sk", "Catania"), ("sl", "Catania"), ("sq", "Provinca Catania"), ("sr", "Катанија"), ("sr_Latn", "Katanija"), ("sv", "Catania"), ("ta", "கேதனிய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "కట\u{c3e}న\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกาตาเน\u{e35}ย"), ("tr", "Katanya ili"), ("uk", "Провінція Катанія"), ("ur", "صوبہ کاتانیا"), ("uz", "Catania"), ("vi", "Catania"), ("yue", "卡坦尼亞省"), ("yue_Hans", "卡坦尼亚省"), ("zh", "卡塔尼亞省")]),
                        unofficial_name_list: ["Metropolitan City of Catania"].to_vec(),
                    }
                ),
                (
                    "CZ",
                    Subdivision{
                        name: "CZ",
                        country_alpha2: Alpha2::IT,
                        code: "CZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.90979189999999), longitude: Some(16.5876516), max_latitude: Some(38.9497221), min_latitude: Some(38.8153388), max_longitude: Some(16.6484891), min_longitude: Some(16.5536698)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كاتنزارو"), ("be", "правінцыя Катандзара"), ("bg", "Катандзаро"), ("ca", "Província de Catanzaro"), ("ccp", "𑄇\u{11133}𑄠𑄑𑄚\u{11134}𑄎𑄢\u{1112e}"), ("ceb", "Catanzaro"), ("cs", "Provincie Catanzaro"), ("de", "Provinz Catanzaro"), ("el", "Καταντζάρο"), ("en", "Catanzaro"), ("es", "Catanzaro"), ("et", "Catanzaro provints"), ("eu", "Catanzaroko probintzia"), ("fa", "استان کاتانزارو"), ("fi", "Catanzaron maakunta"), ("fr", "Province de Catanzaro"), ("gl", "Provincia de Catanzaro"), ("he", "קטנזארו"), ("hr", "Catanzaro"), ("hu", "Catanzaro megye"), ("hy", "Կատանձարո"), ("id", "Provinsi Catanzaro"), ("it", "provincia di Catanzaro"), ("ja", "カタンザーロ県"), ("jv", "Provinsi Catanzaro"), ("ka", "კატანძაროს პროვინცია"), ("ko", "카탄차로 현"), ("lt", "Katancaro provincija"), ("lv", "Katandzāro province"), ("ms", "Wilayah Catanzaro"), ("nb", "Provinsen Catanzaro"), ("nl", "Catanzaro"), ("no", "Provinsen Catanzaro"), ("pl", "Prowincja Catanzaro"), ("pt", "Catanzaro"), ("ro", "Provincia Catanzaro"), ("ru", "Катандзаро"), ("sl", "Catanzaro"), ("sq", "Provinca e Katanxaros"), ("sr", "Катанцаро"), ("sr_Latn", "Katancaro"), ("sv", "Catanzaro"), ("tr", "Catanzaro ili"), ("uk", "Провінція Катандзаро"), ("ur", "صوبہ کاتاندزارو"), ("uz", "Catanzaro"), ("vi", "Catanzaro"), ("zh", "卡坦札羅省")]),
                        unofficial_name_list: ["Province of Catanzaro"].to_vec(),
                    }
                ),
                (
                    "EN",
                    Subdivision{
                        name: "EN",
                        country_alpha2: Alpha2::IT,
                        code: "EN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.5655551), longitude: Some(14.2751913), max_latitude: Some(37.5705528), min_latitude: Some(37.5456653), max_longitude: Some(14.3051983), min_longitude: Some(14.2629157)}),
                        comments: None,
                        subdivision_type: SubdivisionType::FreeMunicipalConsortium,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة إنا"), ("be", "Энна"), ("bg", "Ена"), ("bn", "এন\u{9be} প\u{9cd}রদেশ"), ("ca", "Província d’Enna"), ("ccp", "𑄃\u{11129}𑄚"), ("ceb", "Enna"), ("cs", "Provincie Enna"), ("da", "Province of Enna"), ("de", "Provinz Enna"), ("el", "Έννα"), ("en", "Enna"), ("es", "Enna"), ("et", "Enna provints"), ("eu", "Ennako probintzia"), ("fa", "استان انا"), ("fi", "Ennan maakunta"), ("fr", "Province d’Enna"), ("gl", "Provincia de Enna"), ("gu", "એન\u{acd}ના પ\u{acd}રા\u{a82}ત"), ("he", "אנה"), ("hi", "एना प\u{94d}रा\u{902}त"), ("hr", "Enna"), ("hu", "Enna megye"), ("hy", "Էննա"), ("id", "Provinsi Enna"), ("it", "provincia di Enna"), ("ja", "エンナ県"), ("jv", "Provinsi Enna"), ("ka", "ენის პროვინცია"), ("kn", "ಎನ\u{ccd}ನ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "엔나 현"), ("lt", "Enos provincija"), ("lv", "Ennas province"), ("mr", "एना प\u{94d}रा\u{902}त"), ("ms", "Wilayah Enna"), ("nb", "Provinsen Enna"), ("nl", "Enna"), ("no", "Provinsen Enna"), ("pl", "Prowincja Enna"), ("pt", "Enna"), ("ro", "Provincia Enna"), ("ru", "Энна"), ("si", "එන\u{dca}න\u{dcf} පළ\u{dcf}ත"), ("sl", "Enna"), ("sr", "Ена"), ("sr_Latn", "Ena"), ("sv", "Enna"), ("ta", "ஏன\u{bcd}ன\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఎన\u{c4d}న\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเอ\u{e47}นนา"), ("tr", "Enna ili"), ("uk", "Провінція Енна"), ("ur", "صوبہ اننا"), ("uz", "Enna"), ("vi", "Enna"), ("zh", "恩納省")]),
                        unofficial_name_list: ["Province of Enna"].to_vec(),
                    }
                ),
                (
                    "FC",
                    Subdivision{
                        name: "FC",
                        country_alpha2: Alpha2::IT,
                        code: "FC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.99476809999999), longitude: Some(11.9804613), max_latitude: Some(44.3307907), min_latitude: Some(43.7405254), max_longitude: Some(12.4574107), min_longitude: Some(11.6455594)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Forlì-Cesena"), ("ar", "مقاطعة فورلي تشيزينا"), ("be", "Правінцыя Фарлі-Чэзена"), ("bg", "Форли-Чезена"), ("bn", "ফ\u{9cd}লোর\u{9cd}লি -সিসেন\u{9be}র প\u{9cd}রদেশ"), ("ca", "Província de Forlì-Cesena"), ("ccp", "𑄜\u{1112e}𑄢\u{11134}𑄣\u{11128}-𑄥𑄬𑄥𑄬𑄚"), ("ceb", "Forlì-Cesena"), ("cs", "Provincie Forlì-Cesena"), ("da", "Forlì-Cesena"), ("de", "Provinz Forlì-Cesena"), ("el", "Φορλί-Τσεζένα"), ("en", "Forlì-Cesena"), ("es", "Forlì-Cesena"), ("et", "Forlì-Cesena provints"), ("eu", "Forlì-Cesenako probintzia"), ("fa", "استان فورلی-چسنا"), ("fi", "Forlì-Cesenan maakunta"), ("fr", "Province de Forlì-Cesena"), ("gl", "Provincia de Forlì-Cesena"), ("gu", "ફોર\u{acd}લિ-ક\u{ac7}સ\u{ac7}ના પ\u{acd}રા\u{a82}ત"), ("he", "פורלי-צ׳זנה"), ("hi", "फोर\u{94d}ली-स\u{947}स\u{947}ना प\u{94d}रा\u{902}त"), ("hu", "Forlì-Cesena megye"), ("hy", "Ֆորլի Չեզանա"), ("id", "Provinsi Forlì-Cesena"), ("it", "provincia di Forlì-Cesena"), ("jv", "Provinsi Forlì-Cesena"), ("ka", "ფორლი-ჩეზენის პროვინცია"), ("kn", "ಫೋರ\u{ccd}ಲ\u{cbf}-ಸ\u{cc6}ಸ\u{cbf}ನಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "포를리체세나 현"), ("lt", "Forli-Čezenos provincija"), ("lv", "Forli-Čezēnas province"), ("mk", "Форли-Чезена"), ("mr", "फोली-स\u{947}स\u{947}ना प\u{94d}रा\u{902}त"), ("ms", "Wilayah Forlì-Cesena"), ("nb", "Provinsen Forlì-Cesena"), ("nl", "Forlì-Cesena"), ("no", "Provinsen Forlì-Cesena"), ("pl", "Prowincja Forlì-Cesena"), ("pt", "Forlì-Cesena"), ("ro", "Provincia Forlì-Cesena"), ("ru", "Форли-Чезена"), ("si", "ෆොර\u{dca}ල\u{dd2} සෙසේන\u{dcf} පළ\u{dcf}ත"), ("sl", "Forlì-Cesena"), ("sr", "Форли-Чезена"), ("sr_Latn", "Forli-Čezena"), ("sv", "Forlì-Cesena"), ("ta", "போர\u{bcd}லி -செசேன\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఫ\u{c4b}ర\u{c4d}ల\u{c3f}-స\u{c46}స\u{c46}న\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฟอร\u{e4c}ล\u{e34}-เชเซน\u{e48}า"), ("tr", "Forli-Cesena ili"), ("uk", "Провінція Форлі-Чезена"), ("ur", "صوبہ فورلی-چیزینا"), ("uz", "Forlì-Cesena"), ("vi", "Forlì-Cesena")]),
                        unofficial_name_list: ["province of Forlì-Cesena"].to_vec(),
                    }
                ),
                (
                    "FE",
                    Subdivision{
                        name: "FE",
                        country_alpha2: Alpha2::IT,
                        code: "FE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.8381237), longitude: Some(11.619787), max_latitude: Some(44.8746839), min_latitude: Some(44.7977949), max_longitude: Some(11.6641086), min_longitude: Some(11.5487829)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ferrara"), ("ar", "مقاطعة فيرارا"), ("be", "Правінцыя Ферара"), ("bg", "Ферара"), ("bn", "ফের\u{200d}\u{9cd}য\u{9be}র\u{9be}র প\u{9cd}রদেশ"), ("ca", "Província de Ferrara"), ("ccp", "𑄜𑄬𑄢𑄬𑄢"), ("ceb", "Ferrara"), ("cs", "Provincie Ferrara"), ("da", "Ferrara"), ("de", "Provinz Ferrara"), ("el", "Φερράρα"), ("en", "Ferrara"), ("es", "Ferrara"), ("et", "Ferrara provints"), ("eu", "Ferrarako probintzia"), ("fa", "استان فرارا"), ("fi", "Ferraran maakunta"), ("fr", "Province de Ferrare"), ("gl", "Provincia de Ferrara"), ("gu", "ફ\u{ac7}રારા પ\u{acd}રા\u{a82}ત"), ("he", "פרארה"), ("hi", "फरारा प\u{94d}रा\u{902}त"), ("hu", "Ferrara megye"), ("hy", "Ֆերերա"), ("id", "Provinsi Ferrara"), ("it", "provincia di Ferrara"), ("ja", "フェラーラ県"), ("jv", "Provinsi Ferrara"), ("ka", "ფერარის პროვინცია"), ("kn", "ಫ\u{cc6}ರಾರಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "페라라 현"), ("lt", "Feraros provincija"), ("lv", "Ferrāras province"), ("mk", "Ферара"), ("mr", "फ\u{947}रारा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Ferrara"), ("nb", "Provinsen Ferrara"), ("nl", "Ferrara"), ("no", "Provinsen Ferrara"), ("pl", "Prowincja Ferrara"), ("pt", "Ferrara"), ("ro", "Provincia Ferrara"), ("ru", "Феррара"), ("si", "ෆෙර\u{dcf}ර\u{dcf} පළ\u{dcf}ත"), ("sl", "Ferrara"), ("sr", "Ферара"), ("sr_Latn", "Ferara"), ("sv", "Ferrara"), ("ta", "பெர\u{bcd}ரர\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఫ\u{c46}ర\u{c4d}ర\u{c3e}ర\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเฟอร\u{e4c}รารา"), ("tr", "Ferrara ili"), ("uk", "Провінція Феррара"), ("ur", "صوبہ فیرارا"), ("uz", "Ferrara"), ("vi", "Ferrara"), ("zh", "費拉拉省")]),
                        unofficial_name_list: ["Ferrara"].to_vec(),
                    }
                ),
                (
                    "FG",
                    Subdivision{
                        name: "FG",
                        country_alpha2: Alpha2::IT,
                        code: "FG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.4621984), longitude: Some(15.5446302), max_latitude: Some(41.4795093), min_latitude: Some(41.4346673), max_longitude: Some(15.5835768), min_longitude: Some(15.5183242)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فودجا"), ("be", "правінцыя Фоджа"), ("bg", "Фоджа"), ("bn", "ফগগিয\u{9bc}\u{9be}র প\u{9cd}রদেশ"), ("ca", "Província de Foggia"), ("ccp", "𑄜\u{1112e}𑄉\u{11128}𑄠"), ("ceb", "Foggia"), ("cs", "Provincie Foggia"), ("da", "Provincia di Foggia"), ("de", "Provinz Foggia"), ("el", "Φότζια"), ("en", "Foggia"), ("es", "Foggia"), ("et", "Foggia provints"), ("eu", "Foggiako probintzia"), ("fa", "استان فوجیا"), ("fi", "Foggian maakunta"), ("fr", "Province de Foggia"), ("gl", "Provincia de Foggia"), ("gu", "ફોગિઆ પ\u{acd}રા\u{a82}ત"), ("he", "פוג׳יה"), ("hi", "फोगिया प\u{94d}रा\u{902}त"), ("hr", "Foggia"), ("hu", "Foggia megye"), ("hy", "Ֆոջա"), ("id", "Provinsi Foggia"), ("it", "provincia di Foggia"), ("ja", "フォッジャ県"), ("jv", "Provinsi Foggia"), ("ka", "ფოჯის პროვინცია"), ("kn", "ಫಾಗ\u{ccd}ಗ\u{cbf}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "포자 현"), ("lt", "Fodžos provincija"), ("lv", "Fodžas province"), ("mk", "Фоџа"), ("mr", "फोगिया प\u{94d}रा\u{902}त"), ("ms", "Wilayah Foggia"), ("nb", "Provinsen Foggia"), ("nl", "Foggia"), ("no", "Provinsen Foggia"), ("pl", "Prowincja Foggia"), ("pt", "Foggia"), ("ro", "Provincia Foggia"), ("ru", "Фоджа"), ("si", "ෆොග\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sk", "Foggia"), ("sl", "Foggia"), ("sq", "Provinca e Foxhias"), ("sr", "Фођа"), ("sr_Latn", "Fođa"), ("sv", "Foggia"), ("ta", "போக\u{bcd}கிய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఫ\u{c4b}గ\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฟอกเก\u{e35}ย"), ("tr", "Foggia ili"), ("uk", "Провінція Фоджа"), ("ur", "صوبہ فوجا"), ("uz", "Foggia"), ("vi", "Foggia"), ("zh", "福賈省")]),
                        unofficial_name_list: ["Province of Foggia"].to_vec(),
                    }
                ),
                (
                    "FI",
                    Subdivision{
                        name: "FI",
                        country_alpha2: Alpha2::IT,
                        code: "FI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.7695604), longitude: Some(11.2558136), max_latitude: Some(43.8329368), min_latitude: Some(43.7269795), max_longitude: Some(11.3278993), min_longitude: Some(11.1540365)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فلورنسا"), ("be", "правінцыя Фларэнцыя"), ("bg", "Флоренция"), ("bn", "ফ\u{9cd}লোরেন\u{9cd}স-র প\u{9cd}রদেশ"), ("ca", "Província de Florència"), ("ccp", "𑄜\u{11133}𑄣\u{1112e}𑄢𑄬𑄚\u{11134}𑄌\u{11134}"), ("ceb", "Province of Florence"), ("cs", "Provincie Firenze"), ("da", "Firenze"), ("de", "Provinz Florenz"), ("el", "Φλωρεντία"), ("en", "Florence"), ("es", "Florencia"), ("et", "Firenze provints"), ("eu", "Florentziako probintzia"), ("fa", "استان فلورانس"), ("fi", "Firenzen maakunta"), ("fr", "Province de Florence"), ("gl", "Provincia de Florencia"), ("gu", "ફ\u{acd}લોર\u{ac7}ન\u{acd}સ પ\u{acd}રા\u{a82}ત"), ("he", "פירנצה"), ("hi", "फ\u{94d}लोर\u{947}\u{902}स प\u{94d}रा\u{902}त"), ("hr", "Firenca"), ("hu", "Firenze megye"), ("hy", "Ֆլորենցիա"), ("id", "Provinsi Firenze"), ("is", "Flórens"), ("it", "provincia di Firenze"), ("ja", "フィレンツェ県"), ("jv", "Provinsi Firenze"), ("ka", "ფლორენციის პროვინცია"), ("kn", "ಫ\u{ccd}ಲಾರ\u{cc6}ನ\u{ccd}ಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "피렌체 현"), ("lt", "Florencijos provincija"), ("lv", "Florences province"), ("mk", "Фиренца"), ("mr", "फ\u{94d}लॉर\u{947}न\u{94d}स प\u{94d}रा\u{902}त"), ("ms", "Wilayah Firenze"), ("nb", "Firenze"), ("nl", "Florence"), ("no", "Firenze"), ("pl", "Prowincja Florencja"), ("pt", "Florença"), ("ro", "Provincia Florența"), ("ru", "Флоренция"), ("si", "ෆ\u{dca}ලොරන\u{dca}ස\u{dca} පළ\u{dcf}ත"), ("sk", "Florencia"), ("sl", "Firenze"), ("sq", "Provinca Firenca"), ("sr", "Фиренца"), ("sr_Latn", "Firenca"), ("sv", "Florens"), ("ta", "பிள\u{bbe}ரென\u{bcd}ஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఫ\u{c4d}ల\u{c4b}ర\u{c46}న\u{c4d}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฟลอเรนซ\u{e4c}"), ("tr", "Firenze ili"), ("uk", "Провінція Флоренція"), ("ur", "صوبہ فلورنس"), ("uz", "Florensiya"), ("vi", "Firenze"), ("zh", "佛羅倫斯省")]),
                        unofficial_name_list: ["Metropolitan City of Florence"].to_vec(),
                    }
                ),
                (
                    "FM",
                    Subdivision{
                        name: "FM",
                        country_alpha2: Alpha2::IT,
                        code: "FM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.1588734), longitude: Some(13.7200884), max_latitude: Some(43.1785854), min_latitude: Some(43.1478961), max_longitude: Some(13.7617123), min_longitude: Some(13.6938605)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فيرمو"), ("be", "Правінцыя Ферма"), ("bg", "Фермо"), ("bn", "ফরমোর প\u{9cd}রদেশ"), ("ca", "Província de Fermo"), ("ccp", "𑄜𑄬𑄢\u{11134}𑄟\u{1112e}"), ("ceb", "Province of Fermo"), ("cs", "Provincie Fermo"), ("da", "Province of Fermo"), ("de", "Provinz Fermo"), ("el", "Φέρμο"), ("en", "Fermo"), ("es", "Fermo"), ("et", "Fermo provints"), ("eu", "Fermoko probintzia"), ("fa", "استان فرمو"), ("fi", "Fermon maakunta"), ("fr", "Province de Fermo"), ("gl", "Provincia de Fermo"), ("gu", "ફ\u{ac7}ર\u{acd}મો પ\u{acd}રા\u{a82}ત"), ("he", "פרמו"), ("hi", "फ\u{93c}\u{947}मो प\u{94d}रा\u{902}त"), ("hr", "Fermo"), ("hu", "Fermo megye"), ("hy", "Ֆերմո"), ("id", "Provinsi Fermo"), ("it", "provincia di Fermo"), ("ja", "フェルモ県"), ("jv", "Provinsi Fermo"), ("ka", "ფერმოს პროვინცია"), ("kn", "ಫ\u{cc6}ರ\u{ccd}ಮೋ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "페르모 현"), ("lt", "Fermo provincija"), ("lv", "Fermo province"), ("mr", "फर\u{94d}लो प\u{94d}रा\u{902}त"), ("ms", "Wilayah Fermo"), ("nb", "Provinsen Fermo"), ("nl", "Fermo"), ("no", "Provinsen Fermo"), ("pl", "Prowincja Fermo"), ("pt", "Fermo"), ("ro", "Provincia Fermo"), ("ru", "Фермо"), ("si", "ෆර\u{dca}මෝ පළ\u{dcf}ත"), ("sl", "Fermo"), ("sq", "Provinca Fermo"), ("sr", "Фермо"), ("sr_Latn", "Fermo"), ("sv", "Fermo"), ("ta", "பெர\u{bcd}மோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఫ\u{c46}ర\u{c4d}మ\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแฟร\u{e4c}โม"), ("tr", "Fermo ili"), ("uk", "Провінція Фермо"), ("ur", "صوبہ فیرمو"), ("uz", "Fermo"), ("vi", "Fermo"), ("zh", "費爾莫省")]),
                        unofficial_name_list: ["Province of Fermo"].to_vec(),
                    }
                ),
                (
                    "FR",
                    Subdivision{
                        name: "FR",
                        country_alpha2: Alpha2::IT,
                        code: "FR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.6396009), longitude: Some(13.3426341), max_latitude: Some(41.66558029999999), min_latitude: Some(41.6035248), max_longitude: Some(13.3785906), min_longitude: Some(13.2967693)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فروزينوني"), ("be", "правінцыя Фразінонэ"), ("bg", "Фрозиноне"), ("bn", "ফ\u{9cd}রোসিনোন-এর প\u{9cd}রদেশ"), ("ca", "Província de Frosinone"), ("ccp", "𑄜\u{11133}𑄢\u{1112e}𑄥\u{11128}𑄚\u{11127}𑄚\u{11134}"), ("ceb", "Frosinone"), ("cs", "Provincie Frosinone"), ("da", "Province of Frosinone"), ("de", "Provinz Frosinone"), ("el", "Επαρχία του Φροζινόνε"), ("en", "Frosinone"), ("es", "Frosinone"), ("et", "Frosinone provints"), ("eu", "Frosinoneko probintzia"), ("fa", "استان فروزینونه"), ("fi", "Frosinonen maakunta"), ("fr", "Province de Frosinone"), ("gl", "Provincia de Frosinone"), ("gu", "ફ\u{acd}રોસિનન પ\u{acd}રા\u{a82}ત"), ("he", "פרוזינונה"), ("hi", "फ\u{94d}रोसिनोन प\u{94d}रा\u{902}त"), ("hr", "Frosinone"), ("hu", "Frosinone megye"), ("hy", "Ֆրոզինոնե"), ("id", "Provinsi Frosinone"), ("it", "provincia di Frosinone"), ("ja", "フロジノーネ県"), ("jv", "Provinsi Frosinone"), ("ka", "ფროზინონეს პროვინცია"), ("kn", "ಫ\u{ccd}ರೊಸ\u{cbf}ನೊನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "프로시노네 현"), ("lt", "Frozinonės provincija"), ("lv", "Frozinones province"), ("mk", "Фрозиноне"), ("mr", "फ\u{94d}रॉसिनोना प\u{94d}रा\u{902}त"), ("ms", "Wilayah Frosinone"), ("nb", "Provinsen Frosinone"), ("nl", "Frosinone"), ("no", "Provinsen Frosinone"), ("pl", "Prowincja Frosinone"), ("pt", "Frosinone"), ("ro", "Provincia Frosinone"), ("ru", "Фрозиноне"), ("si", "ෆ\u{dca}රෝස\u{dd2}නොනේ පළ\u{dcf}ත"), ("sl", "Frosinone"), ("sr", "Фрозиноне"), ("sr_Latn", "Frozinone"), ("sv", "Frosinone"), ("ta", "பிரேசினோன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఫ\u{c4d}ర\u{c4b}స\u{c3f}నన\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฟรอสส\u{e34}น\u{e31}น"), ("tr", "Frosinone ili"), ("uk", "Провінція Фрозіноне"), ("ur", "صوبہ فروزینونے"), ("uz", "Frosinone"), ("vi", "Frosinone"), ("zh", "弗罗西诺内省")]),
                        unofficial_name_list: ["Province of Frosinone"].to_vec(),
                    }
                ),
                (
                    "GE",
                    Subdivision{
                        name: "GE",
                        country_alpha2: Alpha2::IT,
                        code: "GE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.4056499), longitude: Some(8.946256), max_latitude: Some(44.514882), min_latitude: Some(44.3791252), max_longitude: Some(9.065572999999999), min_longitude: Some(8.7160912)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مدينة ميتروبوليتان، جنوة"), ("be", "метрапольны горад Генуя"), ("bn", "মেট\u{9cd}রোপলিটন"), ("ca", "Ciutat metropolitana de Gènova"), ("ccp", "𑄉𑄬𑄚\u{11131}"), ("cs", "Metropolitní město Genova"), ("da", "Metropolitan City of Genoa"), ("de", "Metropolitanstadt Genua"), ("el", "Μετροπόλιταν Σίτι της Γένοβα"), ("en", "Genoa"), ("es", "Ciudad metropolitana de Génova"), ("et", "Città metropolitana di Genova"), ("fi", "Genovan metropolikaupunki"), ("fr", "Ville métropolitaine de Gênes"), ("gu", "જિનોવાન\u{ac1}\u{a82} મ\u{ac7}ટ\u{acd}રોપોલિટન શહ\u{ac7}ર"), ("hi", "ज\u{947}नोआ म\u{947}ट\u{94d}रोपॉलिटन सिटी"), ("id", "Metropolitan City of Genoa"), ("it", "città metropolitana di Genova"), ("ja", "ジェノヴァ"), ("kn", "ಜ\u{cbf}ನೋವಾ ಮ\u{cc6}ಟ\u{ccd}ರೋಪಾಲ\u{cbf}ಟನ\u{ccd} ನಗರ"), ("ko", "제노바 광역시"), ("lt", "Genuja"), ("lv", "Dženova"), ("mr", "ज\u{947}नोवा च\u{947} महानगरीय शहर"), ("ms", "Metropolitan City of Genoa"), ("nb", "Genoa kommune"), ("nl", "Genua"), ("no", "Genoa kommune"), ("pl", "Prowincja Miejska Genoa"), ("pt", "Cidade Metropolitana de Genoa"), ("ru", "метропольный город Генуя"), ("si", "ජ\u{dd2}නෝආ හ\u{dd2} මෙට\u{dca}\u{200d}රොපොල\u{dd2}ටන\u{dca} නගරය"), ("sv", "Genoa kommun"), ("ta", "மெட\u{bcd}ரோபொலிட\u{bcd}டன\u{bcd} நகரம\u{bcd} ஜெனோவ\u{bbe}"), ("te", "మ\u{c46}ట\u{c4d}ర\u{c4b}ప\u{c3e}ల\u{c3f}టన\u{c4d} స\u{c3f}ట\u{c40} ఆఫ\u{c4d} జ\u{c46}న\u{c4b}వ\u{c3e}"), ("th", "เจน\u{e31}ว"), ("tr", "Metropolitan City"), ("uk", "Генуя"), ("ur", "میٹروپولیٹن شہر جینوا"), ("vi", "Thành phố Đô thị của Genoa"), ("zh", "熱那亞廣域市")]),
                        unofficial_name_list: ["Metropolitan City of Genoa"].to_vec(),
                    }
                ),
                (
                    "GO",
                    Subdivision{
                        name: "GO",
                        country_alpha2: Alpha2::IT,
                        code: "GO",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::DecentralizedRegionalEntity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غوريتسيا"), ("be", "правінцыя Гарыцыя"), ("bg", "Гориция"), ("ca", "Província de Gorizia"), ("ccp", "𑄉\u{11127}𑄢\u{11128}𑄎\u{11128}𑄠"), ("ceb", "Gorizia"), ("cs", "Provincie Gorizia"), ("da", "Gorizia"), ("de", "Provinz Görz"), ("el", "Γκορίτσια"), ("en", "Gorizia"), ("es", "Gorizia"), ("et", "Gorizia provints"), ("eu", "Goriziako probintzia"), ("fa", "استان گوریتزیا"), ("fi", "Gorizian maakunta"), ("fr", "Province de Gorizia"), ("gl", "Provincia de Gorizia"), ("he", "גוריציה"), ("hr", "Gorica"), ("hu", "Gorizia megye"), ("hy", "Կաղապար:Գորիցիայի գավառ"), ("id", "Provinsi Gorizia"), ("it", "provincia di Gorizia"), ("ja", "ゴリツィア県"), ("jv", "Provinsi Gorizia"), ("ka", "გორიციის პროვინცია"), ("ko", "고리치아 현"), ("lt", "Goricijos provincija"), ("lv", "Gorīcijas province"), ("mk", "Горица"), ("ms", "Wilayah Gorizia"), ("nl", "Gorizia"), ("no", "Provinsen Gorizia"), ("pl", "Prowincja Gorycja"), ("pt", "Gorizia"), ("ro", "Provincia Gorizia"), ("ru", "Гориция"), ("sl", "Goriška pokrajina"), ("sq", "Provinca e Goricias"), ("sr", "Горица"), ("sr_Latn", "Gorica"), ("sv", "Gorizia"), ("tr", "Gorizia ili"), ("uk", "Провінція Горіція"), ("ur", "صوبہ گوریتسیا"), ("uz", "Gorizia"), ("vi", "Gorizia")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "GR",
                    Subdivision{
                        name: "GR",
                        country_alpha2: Alpha2::IT,
                        code: "GR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.7635254), longitude: Some(11.1123634), max_latitude: Some(42.7959914), min_latitude: Some(42.7397719), max_longitude: Some(11.1709899), min_longitude: Some(11.0839812)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غروسيتو"), ("be", "Грасета"), ("bg", "Гросето"), ("ca", "Província de Grosseto"), ("ccp", "𑄉\u{11133}𑄢\u{1112e}𑄥𑄬𑄑\u{1112e}"), ("ceb", "Provincia di Grosseto"), ("cs", "Provincie Grosseto"), ("da", "Grosseto"), ("de", "Provinz Grosseto"), ("el", "Γκροσσέτο"), ("en", "Grosseto"), ("es", "Grosseto"), ("et", "Grosseto provints"), ("eu", "Grossetoko probintzia"), ("fa", "استان گروستو"), ("fi", "Grosseton maakunta"), ("fr", "Province de Grosseto"), ("gl", "Provincia de Grosseto"), ("he", "גרוסטו"), ("hr", "Grosseto (pokrajina)"), ("hu", "Grosseto megye"), ("hy", "Գրոսետո"), ("id", "Provinsi Grosseto"), ("it", "provincia di Grosseto"), ("ja", "グロッセート県"), ("jv", "Provinsi Grosseto"), ("ka", "გროსეტოს პროვინცია"), ("ko", "그로세토 현"), ("lt", "Groseto provincija"), ("lv", "Groseto province"), ("ms", "Wilayah Grosseto"), ("nb", "Grosseto"), ("nl", "Grosseto"), ("no", "Grosseto"), ("pl", "Prowincja Grosseto"), ("pt", "Grosseto"), ("ro", "Provincia Grosseto"), ("ru", "Гроссето"), ("sl", "Grosseto"), ("sr", "Гросето"), ("sr_Latn", "Groseto"), ("sv", "Grosseto"), ("tr", "Grosseto ili"), ("uk", "Провінція Гроссето"), ("ur", "صوبہ گروسیتو"), ("uz", "Grosseto"), ("vi", "Grosseto"), ("zh", "格羅塞托省")]),
                        unofficial_name_list: ["Province of Grosseto"].to_vec(),
                    }
                ),
                (
                    "IM",
                    Subdivision{
                        name: "IM",
                        country_alpha2: Alpha2::IT,
                        code: "IM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.8905684), longitude: Some(8.0346654), max_latitude: Some(43.9228184), min_latitude: Some(43.8643319), max_longitude: Some(8.0630329), min_longitude: Some(7.9598644)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة إمبيريا"), ("be", "правінцыя Імперыя"), ("bg", "Империя"), ("bn", "ইমপেরিয\u{9bc}\u{9be} প\u{9cd}রদেশ"), ("ca", "Província d’Imperia"), ("ccp", "𑄃\u{11128}𑄟\u{11134}𑄛𑄬𑄢\u{11128}𑄠"), ("ceb", "Provincia di Imperia"), ("cs", "Provincie Imperia"), ("da", "Imperia"), ("de", "Provinz Imperia"), ("el", "Ιμπέρια"), ("en", "Imperia"), ("es", "Imperia"), ("et", "Imperia provints"), ("eu", "Imperiako probintzia"), ("fa", "استان ایمپریا"), ("fi", "Imperian maakunta"), ("fr", "Province d’Imperia"), ("gl", "Provincia de Imperia"), ("gu", "ઇમ\u{acd}પ\u{ac7}રીયા પ\u{acd}રા\u{a82}ત"), ("he", "אימפריה"), ("hi", "इम\u{94d}पीरियाई प\u{94d}रा\u{902}त"), ("hu", "Imperia megye"), ("hy", "Իմպերիա"), ("id", "Provinsi Imperia"), ("it", "provincia di Imperia"), ("ja", "インペリア県"), ("jv", "Provinsi Imperia"), ("ka", "იმპერიის პროვინცია"), ("kn", "ಇಂಪೀರ\u{cbf}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "임페리아 현"), ("lt", "Imperijos provincija"), ("lv", "Impērijas province"), ("mk", "Империја"), ("mr", "इ\u{902}पीरीया प\u{94d}रा\u{902}त"), ("ms", "Wilayah Imperia"), ("nb", "Provinsen Imperia"), ("nl", "Imperia"), ("no", "Provinsen Imperia"), ("pl", "Prowincja Imperia"), ("pt", "Impéria"), ("ro", "Provincia Imperia"), ("ru", "Империя"), ("si", "ඉම\u{dca}පේර\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sl", "Imperia"), ("sq", "Provinca e Imperias"), ("sr", "Империја"), ("sr_Latn", "Imperija"), ("sv", "Imperia"), ("ta", "இம\u{bcd}பேறிய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఇంప\u{c40}ర\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e34}มเพอเร\u{e35}ย"), ("tr", "İmperia"), ("uk", "Провінція Імперія"), ("ur", "صوبہ امپیریا"), ("uz", "Imperia"), ("vi", "Imperia"), ("zh", "因佩里亞省")]),
                        unofficial_name_list: ["Province of Imperia"].to_vec(),
                    }
                ),
                (
                    "IS",
                    Subdivision{
                        name: "IS",
                        country_alpha2: Alpha2::IT,
                        code: "IS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.5960411), longitude: Some(14.2331612), max_latitude: Some(41.6174432), min_latitude: Some(41.578224), max_longitude: Some(14.2538525), min_longitude: Some(14.2135501)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة إيسرنيا"), ("bg", "Изерния"), ("bn", "ইস\u{9cd}ট\u{9be}েরনিয\u{9bc}\u{9be}-এর প\u{9cd}রদেশ"), ("ca", "Província d’Isernia"), ("ccp", "𑄃\u{1112d}𑄥𑄬𑄢\u{11134}𑄚\u{11128}𑄠"), ("ceb", "Provincia di Isernia"), ("cs", "Provincie Isernia"), ("da", "Province of Isernia"), ("de", "Provinz Isernia"), ("el", "Επαρχία της Ισέρνια"), ("en", "Isernia"), ("es", "Isernia"), ("et", "Isernia provints"), ("eu", "Iserniako probintzia"), ("fa", "استان ایسرنیا"), ("fi", "Isernian maakunta"), ("fr", "Province d’Isernia"), ("gl", "Provincia de Isernia"), ("gu", "ઇસર\u{acd}નિયા પ\u{acd}રા\u{a82}ત"), ("he", "איסרניה"), ("hi", "इसर\u{94d}निया प\u{94d}रा\u{902}त"), ("hr", "Isernia"), ("hu", "Isernia megye"), ("hy", "Իզերնիա"), ("id", "Provinsi Isernia"), ("it", "provincia di Isernia"), ("ja", "イゼルニア県"), ("ka", "იზერნიის პროვინცია"), ("kn", "ಇಸ\u{ccd}ರೇನ\u{cbf}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "이세르니아 현"), ("lt", "Izernijos provincija"), ("lv", "Izernijas province"), ("mr", "इस\u{94d}स\u{94d}टरिया प\u{94d}रा\u{902}त"), ("ms", "Wilayah Isernia"), ("nb", "Provinsen Isernia"), ("nl", "Isernia"), ("no", "Provinsen Isernia"), ("pl", "Prowincja Isernia"), ("pt", "Isérnia"), ("ro", "Provincia Isernia"), ("ru", "Изерния"), ("si", "ඉසර\u{dca}න\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sl", "Isernia"), ("sq", "Provinca e Izernias"), ("sr", "Изернија"), ("sr_Latn", "Izernija"), ("sv", "Isernia"), ("ta", "இசேர\u{bcd}நிய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఇసర\u{c4d}న\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e35}แซร\u{e4c}เน\u{e35}ย"), ("tr", "İsernia ili"), ("uk", "Провінція Ізернія"), ("ur", "صوبہ ازیرنیا"), ("uz", "Isernia"), ("vi", "Isernia"), ("zh", "伊塞爾尼亞省")]),
                        unofficial_name_list: ["Province of Isernia"].to_vec(),
                    }
                ),
                (
                    "KR",
                    Subdivision{
                        name: "KR",
                        country_alpha2: Alpha2::IT,
                        code: "KR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.0807932), longitude: Some(17.1271102), max_latitude: Some(39.1192418), min_latitude: Some(39.0379212), max_longitude: Some(17.15866), min_longitude: Some(17.0724863)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كروتوني"), ("az", "Krotone"), ("be", "правінцыя Кратонэ"), ("bg", "Кротоне"), ("bn", "ক\u{9cd}রটোন"), ("bs", "Crotone"), ("ca", "Província de Crotona"), ("ccp", "𑄇\u{11133}𑄢\u{1112e}𑄑\u{1112e}𑄚\u{11134}"), ("ceb", "Crotone"), ("cs", "Provincie Crotone"), ("da", "Province of Crotone"), ("de", "Provinz Crotone"), ("el", "Κροτόνε"), ("en", "Crotone"), ("es", "Crotona"), ("et", "Crotone provints"), ("eu", "Crotonako probintzia"), ("fa", "استان کروتون"), ("fi", "Crotonen maakunta"), ("fr", "Province de Crotone"), ("gl", "Provincia de Crotone"), ("gu", "ક\u{acd}રોટોન"), ("he", "קרוטונה"), ("hi", "क\u{94d}रोटोन"), ("hu", "Crotone megye"), ("hy", "Կրոտոնե"), ("id", "Provinsi Crotone"), ("is", "Crotone"), ("it", "provincia di Crotone"), ("ja", "クロトーネ県"), ("jv", "Provinsi Crotone"), ("ka", "კროტონეს პროვინცია"), ("kn", "ಕ\u{ccd}ರೊಟೊನ\u{cc6}"), ("ko", "크로토네 현"), ("lt", "Krotonės provincija"), ("lv", "Krotones province"), ("mn", "Кротоне"), ("mr", "क\u{94d}रोटोनची प\u{94d}रा\u{902}त"), ("ms", "Wilayah Crotone"), ("nb", "Provinsen Crotone"), ("nl", "Crotone"), ("no", "Provinsen Crotone"), ("pl", "Prowincja Krotona"), ("pt", "Crotone"), ("ro", "Provincia Crotone"), ("ru", "Кротоне"), ("si", "ක\u{dca}\u{200d}රෝටෝන\u{dca}"), ("sl", "Crotone"), ("sq", "Provinca e Krotones"), ("sr", "Кротоне"), ("sr_Latn", "Krotone"), ("sv", "Crotone"), ("ta", "க\u{bcd}ரோடோன\u{bcd}"), ("te", "క\u{c4d}ర\u{c4b}టన\u{c4d}"), ("th", "โกรโตเน"), ("tk", "Kroton"), ("tr", "Crotone ili"), ("uk", "Провінція Кротоне"), ("ur", "صوبہ کروتونے"), ("uz", "Krotone"), ("vi", "Crotone"), ("zh", "克羅托內省")]),
                        unofficial_name_list: ["Province of Crotone"].to_vec(),
                    }
                ),
                (
                    "LC",
                    Subdivision{
                        name: "LC",
                        country_alpha2: Alpha2::IT,
                        code: "LC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.8565698), longitude: Some(9.397670399999999), max_latitude: Some(45.8870552), min_latitude: Some(45.8150829), max_longitude: Some(9.424340899999999), min_longitude: Some(9.3781386)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ليكو"), ("be", "правінцыя Лека"), ("bg", "Леко"), ("bn", "লেক\u{9cd}কো-র প\u{9cd}রদেশ"), ("ca", "Província de Lecco"), ("ccp", "𑄣𑄬𑄇\u{11134}𑄇\u{1112e}"), ("ceb", "Provincia di Lecco"), ("cs", "Provincie Lecco"), ("da", "Lecco"), ("de", "Provinz Lecco"), ("el", "Λέκκο"), ("en", "Lecco"), ("es", "Lecco"), ("et", "Lecco provints"), ("eu", "Leccoko probintzia"), ("fa", "استان لکو"), ("fi", "Leccon maakunta"), ("fr", "Province de Lecco"), ("gl", "Provincia de Lecco"), ("gu", "લ\u{ac7}કો પ\u{acd}રા\u{a82}ત"), ("he", "לקו"), ("hi", "ल\u{947}को प\u{94d}रा\u{902}त"), ("hr", "Pokrajina Lecco"), ("hu", "Lecco megye"), ("hy", "Լեքո"), ("id", "Provinsi Lecco"), ("it", "provincia di Lecco"), ("ja", "レッコ県"), ("jv", "Provinsi Lecco"), ("ka", "ლეკოს პროვინცია"), ("kn", "ಲ\u{cc6}ಕೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "레코 현"), ("lt", "Leko provincija"), ("lv", "Leko province"), ("mk", "Леко"), ("mr", "ल\u{947}क\u{94d}को प\u{94d}रा\u{902}त"), ("ms", "Wilayah Lecco"), ("nb", "Provinsen Lecco"), ("nl", "Lecco"), ("no", "Provinsen Lecco"), ("pl", "Prowincja Lecco"), ("pt", "Lecco"), ("ro", "Provincia Lecco"), ("ru", "Лекко"), ("si", "ලෙක\u{dca}කෝ පළ\u{dcf}ත"), ("sk", "Lecco"), ("sl", "Lecco"), ("sq", "Provinca e Lekos"), ("sr", "Леко"), ("sr_Latn", "Leko"), ("sv", "Lecco"), ("ta", "லெக\u{bcd}கோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c46}క\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เลกโก"), ("tr", "Lecco ili"), ("uk", "Провінція Лекко"), ("ur", "صوبہ لیکو"), ("uz", "Lecco"), ("vi", "Lecco"), ("zh", "萊科省")]),
                        unofficial_name_list: ["Province of Lecco"].to_vec(),
                    }
                ),
                (
                    "LE",
                    Subdivision{
                        name: "LE",
                        country_alpha2: Alpha2::IT,
                        code: "LE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.3515155), longitude: Some(18.1750161), max_latitude: Some(40.3777911), min_latitude: Some(40.3304703), max_longitude: Some(18.2285949), min_longitude: Some(18.1199941)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ليتشي"), ("be", "правінцыя Лечэ"), ("bg", "Лече"), ("bn", "লেসে-র প\u{9cd}রদেশ"), ("ca", "Província de Lecce"), ("ccp", "𑄣𑄬𑄌\u{11134}𑄌𑄬"), ("ceb", "Lecce"), ("cs", "Provincie Lecce"), ("da", "Province of Lecce"), ("de", "Provinz Lecce"), ("el", "Λέτσε"), ("en", "Lecce"), ("es", "Lecce"), ("et", "Lecce provints"), ("eu", "Lecceko probintzia"), ("fa", "استان لچه"), ("fi", "Leccen maakunta"), ("fr", "Province de Lecce"), ("gl", "Provincia de Lecce"), ("gu", "લ\u{ac7}કક\u{ac7} પ\u{acd}રા\u{a82}ત"), ("he", "לצ׳ה"), ("hi", "ल\u{947}सी प\u{94d}रा\u{902}त"), ("hu", "Lecce megye"), ("hy", "Լեչե"), ("id", "Provinsi Lecce"), ("it", "provincia di Lecce"), ("ja", "レッチェ県"), ("jv", "Provinsi Lecce"), ("ka", "ლეჩეს პროვინცია"), ("kn", "ಲ\u{cc6}ಕ\u{ccd}ಸ\u{cc6} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "레체 현"), ("lt", "Lečės provincija"), ("lv", "Lečes province"), ("mr", "ल\u{947}क\u{94d}स\u{947}च\u{947} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Lecce"), ("nb", "Provinsen Lecce"), ("nl", "Lecce"), ("no", "Provinsen Lecce"), ("pl", "Prowincja Lecce"), ("pt", "Lecce"), ("ro", "Provincia Lecce"), ("ru", "Лечче"), ("si", "ලෙසේ පළ\u{dcf}ත"), ("sl", "Lecce"), ("sq", "Provinca e Leçes"), ("sr", "Лече"), ("sr_Latn", "Leče"), ("sv", "Lecce"), ("ta", "லேக\u{bcd}ஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c46}చ\u{c46} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเลกเซ"), ("tr", "Lecce ili"), ("uk", "Провінція Лечче"), ("ur", "صوبہ لیچہ"), ("uz", "Lecce"), ("vi", "Lecce"), ("zh", "萊切省")]),
                        unofficial_name_list: ["Province of Lecce"].to_vec(),
                    }
                ),
                (
                    "LI",
                    Subdivision{
                        name: "LI",
                        country_alpha2: Alpha2::IT,
                        code: "LI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.548473), longitude: Some(10.3105674), max_latitude: Some(43.5958939), min_latitude: Some(43.4832059), max_longitude: Some(10.3601034), min_longitude: Some(10.2905905)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ليفورنو"), ("be", "Ліворна"), ("bg", "Ливорно"), ("bn", "লিভর\u{9cd}নোর প\u{9cd}রদেশ"), ("ca", "Província de Liorna"), ("ccp", "𑄣\u{11128}𑄞\u{1112e}𑄢\u{11134}𑄚\u{1112e}"), ("ceb", "Provincia di Livorno"), ("cs", "Provincie Livorno"), ("da", "Livorno"), ("de", "Provinz Livorno"), ("el", "Λιβόρνο"), ("en", "Livorno"), ("es", "Livorno"), ("et", "Livorno provints"), ("eu", "Livornoko probintzia"), ("fa", "استان لیورنو"), ("fi", "Livornon maakunta"), ("fr", "Province de Livourne"), ("gl", "Provincia de Livorno"), ("gu", "લિવોર\u{acd}નો પ\u{acd}રા\u{a82}ત"), ("he", "ליבורנו"), ("hi", "लिवोर\u{94d}नो प\u{94d}रा\u{902}त"), ("hu", "Livorno megye"), ("hy", "Լիվորնո"), ("id", "Provinsi Livorno"), ("is", "Livorno"), ("it", "provincia di Livorno"), ("ja", "リヴォルノ県"), ("jv", "Provinsi Livorno"), ("ka", "ლივორნოს პროვინცია"), ("kn", "ಲ\u{cbf}ವೊರ\u{ccd}ನೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "리보르노 현"), ("lt", "Livorno provincija"), ("lv", "Livorno province"), ("mk", "Ливорно"), ("mr", "लिवोर\u{94d}नो प\u{94d}रा\u{902}त"), ("ms", "Wilayah Livorno"), ("nb", "Livorno"), ("nl", "Livorno"), ("no", "Livorno"), ("pl", "Prowincja Livorno"), ("pt", "Livorno"), ("ro", "Provincia Livorno"), ("ru", "Ливорно"), ("si", "ල\u{dd2}වොර\u{dca}නෝ පළ\u{dcf}ත"), ("sl", "Livorno"), ("sr", "Ливорно"), ("sr_Latn", "Livorno"), ("sv", "Livorno"), ("ta", "லிவோரனோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c3f}వ\u{c4b}ర\u{c4d}న\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ล\u{e34}วอร\u{e4c}โน"), ("tr", "Livorno ili"), ("uk", "Провінція Ліворно"), ("ur", "صوبہ لیورنو"), ("uz", "Livorno"), ("vi", "Livorno"), ("zh", "利佛諾省")]),
                        unofficial_name_list: ["Province of Livorno"].to_vec(),
                    }
                ),
                (
                    "LO",
                    Subdivision{
                        name: "LO",
                        country_alpha2: Alpha2::IT,
                        code: "LO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.3097228), longitude: Some(9.503715999999999), max_latitude: Some(45.3283109), min_latitude: Some(45.2947817), max_longitude: Some(9.522065099999999), min_longitude: Some(9.4755178)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة لودي"), ("be", "правінцыя Лодзі"), ("bg", "Лоди"), ("bn", "লোডি প\u{9cd}রদেশ"), ("ca", "Província de Lodi"), ("ccp", "𑄣\u{1112e}𑄓\u{11128}"), ("ceb", "Provincia di Lodi"), ("cs", "Provincie Lodi"), ("da", "Lodi"), ("de", "Provinz Lodi"), ("el", "Επαρχία του Λόντι"), ("en", "Lodi"), ("es", "Lodi"), ("et", "Lodi provints"), ("eu", "Lodiko probintzia"), ("fa", "استان لودی"), ("fi", "Lodin maakunta"), ("fr", "Province de Lodi"), ("gl", "Provincia de Lodi"), ("gu", "લોડી પ\u{acd}રા\u{a82}ત"), ("he", "לודי"), ("hi", "लोदी प\u{94d}रा\u{902}त"), ("hu", "Lodi megye"), ("hy", "Լոդի"), ("id", "Provinsi Lodi"), ("it", "provincia di Lodi"), ("ja", "ローディ県"), ("jv", "Provinsi Lodi"), ("ka", "ლოდის პროვინცია"), ("kn", "ಲೋದ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "로디 현"), ("lt", "Lodžio provincija"), ("lv", "Lodi province"), ("mr", "लॉडी प\u{94d}रा\u{902}त"), ("ms", "Wilayah Lodi"), ("nb", "Provinsen Lodi"), ("nl", "Lodi"), ("no", "Provinsen Lodi"), ("pl", "Prowincja Lodi"), ("pt", "Lodi"), ("ro", "Provincia Lodi"), ("ru", "Лоди"), ("si", "ලෝඩ\u{dd2} පළ\u{dcf}ත"), ("sk", "Lodi"), ("sl", "Lodi"), ("sq", "Provinca e Lodit"), ("sr", "Лоди"), ("sr_Latn", "Lodi"), ("sv", "Lodi"), ("ta", "லோடி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c4b}డ\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโลด\u{e35}"), ("tr", "Lodi ili"), ("uk", "Провінція Лоді"), ("ur", "صوبہ لوڈی"), ("uz", "Lodi"), ("vi", "Lodi"), ("zh", "洛迪省")]),
                        unofficial_name_list: ["Province of Lodi"].to_vec(),
                    }
                ),
                (
                    "LT",
                    Subdivision{
                        name: "LT",
                        country_alpha2: Alpha2::IT,
                        code: "LT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.4675671), longitude: Some(12.9035965), max_latitude: Some(41.4864651), min_latitude: Some(41.4379936), max_longitude: Some(12.9582445), min_longitude: Some(12.8445286)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة لاتينا"), ("az", "Latina"), ("be", "правінцыя Лаціна"), ("bg", "Латина"), ("bn", "ল\u{9cd}য\u{9be}টিন\u{9be}-এর প\u{9cd}রদেশ"), ("ca", "Província de Latina"), ("ccp", "𑄣𑄑\u{11128}𑄚"), ("ceb", "Latina"), ("cs", "Provincie Latina"), ("da", "Latina"), ("de", "Provinz Latina"), ("el", "Λατίνα"), ("en", "Latina"), ("es", "Latina"), ("et", "Latina provints"), ("eu", "Latinako probintzia"), ("fa", "استان لاتینا"), ("fi", "Latinan maakunta"), ("fr", "province de Latina"), ("gl", "Provincia de Latina"), ("gu", "લટિના પ\u{acd}રા\u{a82}ત"), ("he", "לאטינה"), ("hi", "ल\u{948}टिना प\u{94d}रा\u{902}त"), ("hr", "Latina"), ("hu", "Latina megye"), ("hy", "Լատինա"), ("id", "Provinsi Latina"), ("it", "provincia di Latina"), ("ja", "ラティーナ県"), ("jv", "Provinsi Latina"), ("ka", "ლატინის პროვინცია"), ("kn", "ಲತೀನಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "라티나 현"), ("lt", "Latinos provincija"), ("lv", "Latīnas province"), ("mk", "Латина"), ("mr", "ल\u{945}टीना प\u{94d}रा\u{902}त"), ("ms", "Wilayah Latina"), ("nb", "Provinsen Latina"), ("nl", "Latina"), ("no", "Provinsen Latina"), ("pl", "Prowincja Latina"), ("pt", "Latina"), ("ro", "Provincia Latina"), ("ru", "Латина"), ("si", "ලට\u{dd2}න\u{dcf} පළ\u{dcf}ත"), ("sl", "Latina"), ("sr", "Латина"), ("sr_Latn", "Latina"), ("sv", "Latina"), ("ta", "லத\u{bcd}த\u{bc0}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c3e}ట\u{c3f}న\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดลาต\u{e34}นา"), ("tr", "Latina ili"), ("uk", "Провінція Латина"), ("ur", "صوبہ لاتینا"), ("uz", "Latina"), ("vi", "Latina"), ("zh", "拉蒂纳省")]),
                        unofficial_name_list: ["Province of Latina"].to_vec(),
                    }
                ),
                (
                    "LU",
                    Subdivision{
                        name: "LU",
                        country_alpha2: Alpha2::IT,
                        code: "LU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.8376211), longitude: Some(10.4950609), max_latitude: Some(43.9307158), min_latitude: Some(43.7687028), max_longitude: Some(10.5595241), min_longitude: Some(10.385155)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة لوكا"), ("be", "правінцыя Лука"), ("bg", "Лука"), ("bn", "ল\u{9c1}ক\u{9cd}ক\u{9be}-র প\u{9cd}রদেশ"), ("ca", "Província de Lucca"), ("ccp", "𑄣\u{11128}𑄅\u{1112a}𑄇\u{11133}𑄦"), ("ceb", "Provincia di Lucca"), ("cs", "Provincie Lucca"), ("da", "Lucca"), ("de", "Provinz Lucca"), ("el", "Λούκα"), ("en", "Lucca"), ("es", "Lucca"), ("et", "Lucca provints"), ("eu", "Luccako probintzia"), ("fa", "استان لوکا"), ("fi", "Luccan maakunta"), ("fr", "Province de Lucques"), ("gl", "Provincia de Lucca"), ("gu", "લ\u{ac1}કા પ\u{acd}રા\u{a82}ત"), ("he", "לוקה"), ("hi", "ल\u{942}का प\u{94d}रा\u{902}त"), ("hu", "Lucca megye"), ("hy", "Կաղապար:Լուկայի գավառ"), ("id", "Provinsi Lucca"), ("it", "provincia di Lucca"), ("ja", "ルッカ県"), ("jv", "Provinsi Lucca"), ("ka", "ლუკას პროვინცია"), ("kn", "ಲ\u{cc2}ಕ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "루카 현"), ("lt", "Lukos provincija"), ("lv", "Lukas province"), ("mr", "ल\u{941}का प\u{94d}रा\u{902}त"), ("ms", "Wilayah Lucca"), ("nb", "Lucca"), ("nl", "Lucca"), ("no", "Lucca"), ("pl", "Prowincja Lukka"), ("pt", "Lucca"), ("ro", "Provincia Lucca"), ("ru", "Лукка"), ("si", "ල\u{dd4}ක\u{dcf} පළ\u{dcf}ත"), ("sl", "Lucca"), ("sr", "Лука"), ("sr_Latn", "Luka"), ("sv", "Lucca"), ("ta", "லூக\u{bcd}க\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "లూక\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดล\u{e38}กกา"), ("tr", "Lucca ili"), ("uk", "Провінція Лукка"), ("ur", "صوبہ لوکا"), ("uz", "Lucca"), ("vi", "Lucca"), ("zh", "盧卡省")]),
                        unofficial_name_list: ["Lucca", "Province of Lucca"].to_vec(),
                    }
                ),
                (
                    "MB",
                    Subdivision{
                        name: "MB",
                        country_alpha2: Alpha2::IT,
                        code: "MB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.623599), longitude: Some(9.258801499999999), max_latitude: Some(45.7427334), min_latitude: Some(45.536646), max_longitude: Some(9.4966723), min_longitude: Some(9.0505239)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة منزا وبريانسا"), ("be", "правінцыя Монца і Брыянца"), ("bg", "Монца и Брианца"), ("bn", "মঞ\u{9cd}জ\u{9be} ও ব\u{9cd}র\u{9be}য\u{9bc}েনজ\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Monza e de la Brianza"), ("ccp", "𑄟\u{11127}𑄚\u{11134}𑄎 𑄃\u{11133}𑄃 𑄝\u{11133}𑄢\u{11128}𑄚\u{11134}𑄎"), ("ceb", "Provincia di Monza e Brianza"), ("cs", "Provincie Monza e Brianza"), ("da", "Monza e Brianza"), ("de", "Provinz Monza und Brianza"), ("el", "Μόντσα-Μπριάντσα"), ("en", "Monza and Brianza"), ("es", "Monza y Brianza"), ("et", "Monza e Brianza provints"), ("eu", "Monza eta Brianzako probintzia"), ("fa", "استان مونتزا و بریانتزا"), ("fi", "Monza e Brianzan maakunta"), ("fr", "Province de Monza et de la Brianza"), ("gl", "Provincia de Monza e Brianza"), ("gu", "મોન\u{acd}ઝા એન\u{acd}ડ બ\u{acd}રિએન\u{acd}ઝા પ\u{acd}રા\u{a82}ત"), ("he", "מונצה-בריאנצה"), ("hi", "मौन\u{94d}ज\u{93c}ा और ब\u{94d}रिएन\u{94d}ज\u{93c}ा प\u{94d}रा\u{902}त"), ("hu", "Monza e Brianza megye"), ("hy", "Մոնցա և Բրիանցա"), ("id", "Provinsi Monza dan Brianza"), ("it", "provincia di Monza e della Brianza"), ("ja", "モンツァ・エ・ブリアンツァ県"), ("ka", "მონცა და ბრიანცის პროვინცია"), ("kn", "ಮೊನ\u{ccd}ಜಾ ಮತ\u{ccd}ತು ಬ\u{ccd}ರ\u{cbf}ಯಾನ\u{ccd}ಜಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "몬차에브리안차 현"), ("lt", "Moncos ir Briancos provincija"), ("lv", "Moncas un Briancas province"), ("mr", "मो\u{902}झा अ\u{901}ड ब\u{94d}रायनझा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Monza dan Brianza"), ("nb", "Provinsen Monza og Brianza"), ("nl", "Monza e Brianza"), ("no", "Provinsen Monza og Brianza"), ("pl", "Prowincja Monza i Brianza"), ("pt", "Província de Monza e Brianza"), ("ro", "Provincia Monza-Brianza"), ("ru", "Монца и Брианца"), ("si", "මොන\u{dca}ස\u{dcf} සහ බ\u{dca}\u{200d}ර\u{dd2}යන\u{dca}ස\u{dcf} පළ\u{dcf}ත"), ("sl", "Monza e Brianza"), ("sq", "Provinca e Monxës e Brianxës"), ("sr", "Монца и Бријанца"), ("sr_Latn", "Monca i Brijanca"), ("sv", "Monza e Brianza"), ("ta", "மொன\u{bcd}ஜ\u{bbe} அண\u{bcd}ட\u{bcd} பிரையன\u{bcd}ஜ\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c3e}ంజ\u{c3e} మర\u{c3f}యు బ\u{c4d}రయ\u{c3e}ంజ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมอนซาและบร\u{e34}อ\u{e31}นซา"), ("tr", "Monza ve Brianza ili"), ("uk", "Провінція Монца і Бріанца"), ("ur", "صوبہ مونتسا اور بریانتزا"), ("vi", "Monza và Brianza"), ("zh", "蒙薩和布里安薩省")]),
                        unofficial_name_list: ["Province of Monza and Brianza"].to_vec(),
                    }
                ),
                (
                    "MC",
                    Subdivision{
                        name: "MC",
                        country_alpha2: Alpha2::IT,
                        code: "MC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.2984268), longitude: Some(13.4534767), max_latitude: Some(43.3133078), min_latitude: Some(43.2789768), max_longitude: Some(13.4876283), min_longitude: Some(13.4161256)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ماشيراتا"), ("be", "Правінцыя Мачэрата"), ("bg", "Мачерата"), ("bn", "ম\u{9be}চের\u{9be}ট\u{9be}-র প\u{9cd}রদেশ"), ("ca", "Província de Macerata"), ("ccp", "𑄟𑄥𑄬𑄢\u{11134}𑄑"), ("ceb", "Provincia di Macerata"), ("cs", "Provincie Macerata"), ("da", "Macerata"), ("de", "Provinz Macerata"), ("el", "Ματσεράτα"), ("en", "Macerata"), ("es", "Macerata"), ("et", "Macerata provints"), ("eu", "Maceratako probintzia"), ("fa", "استان ماچراتا"), ("fi", "Maceratan maakunta"), ("fr", "Province de Macerata"), ("gl", "Provincia de Macerata"), ("gu", "મ\u{ac5}ક\u{ac7}રાટા પ\u{acd}રા\u{a82}ત"), ("he", "מצ׳רטה"), ("hi", "म\u{948}स\u{947}राटा प\u{94d}रा\u{902}त"), ("hu", "Macerata megye"), ("hy", "Մաչերատա"), ("id", "Provinsi Macerata"), ("it", "provincia di Macerata"), ("ja", "マチェラータ県"), ("jv", "Provinsi Macerata"), ("ka", "მაჩერატის პროვინცია"), ("kn", "ಮ\u{ccd}ಯಾಕ\u{cc6}ರಾಟಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "마체라타 현"), ("lt", "Maceratos provincija"), ("lv", "Mačeratas province"), ("mr", "मकार\u{94d}ता प\u{94d}रा\u{902}त"), ("ms", "Wilayah Macerata"), ("nb", "Provinsen Macerata"), ("nl", "Macerata"), ("no", "Provinsen Macerata"), ("pl", "Prowincja Macerata"), ("pt", "Macerata"), ("ro", "Provincia Macerata"), ("ru", "Мачерата"), ("si", "මසේරට\u{dcf} පළ\u{dcf}ත"), ("sl", "Macerata"), ("sq", "Provinca e Maçeratës"), ("sr", "Мачерата"), ("sr_Latn", "Mačerata"), ("sv", "Macerata"), ("ta", "மஸ\u{bcd}ர\u{bbe}ட\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c3e}స\u{c46}ర\u{c3e}ట\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมาเซเรตา"), ("tr", "Macerata ili"), ("uk", "Провінція Мачерата"), ("ur", "صوبہ ماچیراتا"), ("uz", "Macerata"), ("vi", "Macerata"), ("zh", "馬切拉塔省")]),
                        unofficial_name_list: ["Province of Macerata"].to_vec(),
                    }
                ),
                (
                    "ME",
                    Subdivision{
                        name: "ME",
                        country_alpha2: Alpha2::IT,
                        code: "ME",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.1938137), longitude: Some(15.5540152), max_latitude: Some(38.30146269999999), min_latitude: Some(38.0546577), max_longitude: Some(15.6361362), min_longitude: Some(15.4659197)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة مسينة"), ("az", "Messina"), ("be", "правінцыя Месіна"), ("bg", "Месина"), ("bn", "মেজিন\u{9be}"), ("bs", "Mesina"), ("ca", "Província de Messina"), ("ccp", "𑄟𑄬𑄥\u{11128}𑄚"), ("ceb", "Messina"), ("cs", "Provincie Messina"), ("da", "Province of Messina"), ("de", "Metropolitanstadt Messina"), ("el", "Μεσσίνα"), ("en", "Messina"), ("es", "Mesina"), ("et", "Messina provints"), ("eu", "Messinako probintzia"), ("fa", "استان مسینا"), ("fi", "Messinan maakunta"), ("fr", "Province de Messine"), ("gl", "Provincia de Messina"), ("gu", "મ\u{ac7}સિના"), ("he", "מסינה"), ("hi", "म\u{947}सिना"), ("hr", "Messina"), ("hu", "Messina megye"), ("hy", "Մեսսինա"), ("id", "Provinsi Messina"), ("is", "Messina"), ("it", "provincia di Messina"), ("ja", "メッシーナ県"), ("jv", "Provinsi Messina"), ("ka", "მესინის პროვინცია"), ("kn", "ಮ\u{cc6}ಸ\u{ccd}ಸ\u{cbf}ನಾ"), ("ko", "메시나 현"), ("lt", "Mesinos provincija"), ("lv", "Mesīnas province"), ("mk", "Месина"), ("mn", "Мессина"), ("mr", "म\u{947}सिना प\u{94d}रा\u{902}त"), ("ms", "Wilayah Messina"), ("nb", "Provinsen Messina"), ("nl", "Messina"), ("no", "Provinsen Messina"), ("pl", "Prowincja Mesyna"), ("pt", "Messina"), ("ro", "Provincia Messina"), ("ru", "Мессина"), ("si", "මෙස\u{dd2}න\u{dcf}"), ("sl", "Messina"), ("sq", "Mesina"), ("sr", "Месина"), ("sr_Latn", "Mesina"), ("sv", "Messina"), ("ta", "மெஸ\u{bc0}ன\u{bbe}"), ("te", "మ\u{c46}స\u{c40}న\u{c3e}"), ("th", "เมสส\u{e34}นา"), ("tk", "Messina welaýaty"), ("tr", "Messina ili"), ("uk", "Провінція Мессіна"), ("ur", "صوبہ میسینا"), ("uz", "Messina"), ("vi", "Messina"), ("yue", "墨西拿省"), ("yue_Hans", "墨西拿省"), ("zh", "墨西拿省")]),
                        unofficial_name_list: ["Province of Messina"].to_vec(),
                    }
                ),
                (
                    "MI",
                    Subdivision{
                        name: "MI",
                        country_alpha2: Alpha2::IT,
                        code: "MI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.4654219), longitude: Some(9.1859243), max_latitude: Some(45.535689), min_latitude: Some(45.3897787), max_longitude: Some(9.2903463), min_longitude: Some(9.065118199999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ميلانو"), ("be", "правінцыя Мілан"), ("bg", "Милано"), ("bn", "মিল\u{9be}ন প\u{9cd}রদেশ"), ("ca", "província de Milà"), ("ccp", "𑄟\u{11128}𑄣\u{11127}𑄚\u{11134}"), ("ceb", "Città metropolitana di Milano"), ("cs", "Provincie Milano"), ("da", "Milano"), ("de", "Provinz Mailand"), ("el", "Επαρχία του Μιλάνου"), ("en", "Milan"), ("es", "Milán"), ("et", "Milano provints"), ("eu", "Milango probintzia"), ("fa", "استان میلان"), ("fi", "Milanon maakunta"), ("fr", "province de Milan"), ("gl", "Provincia de Milán"), ("gu", "મિલાન પ\u{acd}રા\u{a82}ત"), ("he", "מילאנו"), ("hi", "मिलान प\u{94d}रा\u{902}त"), ("hu", "Milánó megye"), ("hy", "Միլան"), ("id", "Provinsi Milan"), ("it", "provincia di Milano"), ("ja", "ミラノ県"), ("jv", "Provinsi Milan"), ("ka", "მილანის პროვინცია"), ("kn", "ಮ\u{cbf}ಲನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "밀라노 현"), ("lt", "Milano provincija"), ("lv", "Milānas province"), ("mk", "Милано"), ("mn", "Милан аймаг"), ("mr", "मिलान प\u{94d}रा\u{902}त"), ("ms", "Wilayah Milan"), ("nb", "Provinsen Milano"), ("nl", "Milaan"), ("no", "Provinsen Milano"), ("pl", "Prowincja Mediolan"), ("pt", "Milão"), ("ro", "Provincia Milano"), ("ru", "Милан"), ("si", "ම\u{dd2}ල\u{dcf}න\u{dca} පළ\u{dcf}ත"), ("sk", "Miláno"), ("sl", "Milano"), ("sq", "Provinca e Milanos"), ("sr", "Милано (округ)"), ("sr_Latn", "Milano (okrug)"), ("sv", "Milano"), ("ta", "மிலன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c3f}ల\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดม\u{e34}ลาน"), ("tr", "Milano ili"), ("uk", "Провінція Мілан"), ("ur", "صوبہ میلان"), ("uz", "Milan"), ("vi", "Milano"), ("yue", "米蘭省"), ("yue_Hans", "米兰省"), ("zh", "米蘭省")]),
                        unofficial_name_list: ["Milano"].to_vec(),
                    }
                ),
                (
                    "MN",
                    Subdivision{
                        name: "MN",
                        country_alpha2: Alpha2::IT,
                        code: "MN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.1564168), longitude: Some(10.7913751), max_latitude: Some(45.1863062), min_latitude: Some(45.1372793), max_longitude: Some(10.8060815), min_longitude: Some(10.7404198)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة مانتوفا"), ("be", "Мантуя"), ("bg", "Мантуа"), ("bn", "ম\u{9be}ন\u{9cd}ত\u{9c1}য\u{9bc}\u{9be}-এর প\u{9cd}রদেশ"), ("ca", "Província de Màntua"), ("ccp", "𑄟\u{11127}𑄚\u{11134}𑄑\u{1112a}𑄠"), ("ceb", "Provincia di Mantova"), ("cs", "Provincie Mantova"), ("da", "Mantova"), ("de", "Provinz Mantua"), ("el", "Μάντοβα"), ("en", "Mantua"), ("es", "Mantua"), ("et", "Mantova provints"), ("eu", "Mantuako probintzia"), ("fa", "استان منتووا"), ("fi", "Mantovan maakunta"), ("fr", "Province de Mantoue"), ("gl", "Provincia de Mantua"), ("gu", "મન\u{acd}ટ\u{ac1}આ પ\u{acd}રા\u{a82}ત"), ("he", "מנטובה"), ("hi", "मान\u{94d}ट\u{941}आ प\u{94d}रा\u{902}त"), ("hu", "Mantova megye"), ("hy", "Մանտուա"), ("id", "Provinsi Mantova"), ("it", "provincia di Mantova"), ("ja", "マントヴァ県"), ("jv", "Provinsi Mantova"), ("ka", "მანტუის პროვინცია"), ("kn", "ಮಂಟ\u{cc2}ವ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "만토바 현"), ("lt", "Mantujos provincija"), ("lv", "Mantujas province"), ("mr", "म\u{902}ट\u{941}आ प\u{94d}रा\u{902}त"), ("ms", "Wilayah Mantua"), ("nb", "Provinsen Mantova"), ("ne", "मान\u{94d}ट\u{941}आ क\u{94d}ष\u{947}त\u{94d}र"), ("nl", "Mantua"), ("no", "Provinsen Mantova"), ("pl", "Prowincja Mantua"), ("pt", "Mântua (província)"), ("ro", "Provincia Mantova"), ("ru", "Мантуя"), ("si", "මන\u{dca}ට\u{dd4}ආ පළ\u{dcf}ත"), ("sk", "Mantova"), ("sl", "Mantova"), ("sq", "Provinca e Mantovës"), ("sr", "Мантова"), ("sr_Latn", "Mantova"), ("sv", "Mantova"), ("ta", "மேன\u{bcd}டுவ\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c3e}ంటవ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมานต\u{e31}ว"), ("tr", "Mantova ili"), ("uk", "Провінція Мантуя"), ("ur", "صوبہ مانتووا"), ("uz", "Mantuya"), ("vi", "Mantova"), ("zh", "曼托瓦省")]),
                        unofficial_name_list: ["Province of Mantua"].to_vec(),
                    }
                ),
                (
                    "MO",
                    Subdivision{
                        name: "MO",
                        country_alpha2: Alpha2::IT,
                        code: "MO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.6488366), longitude: Some(10.9200867), max_latitude: Some(44.6941981), min_latitude: Some(44.60615620000001), max_longitude: Some(10.990284), min_longitude: Some(10.8531802)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Modena"), ("ar", "مقاطعة مودينا"), ("be", "правінцыя Модэна"), ("bg", "Модена"), ("bn", "মোডেন\u{9be}-র প\u{9cd}রদেশ"), ("ca", "Província de Mòdena"), ("ccp", "𑄟\u{11127}𑄓𑄬𑄚"), ("ceb", "Modena"), ("cs", "Provincie Modena"), ("da", "Modena"), ("de", "Provinz Modena"), ("el", "Μόντενα"), ("en", "Modena"), ("es", "Módena"), ("et", "Modena provints"), ("eu", "Modenako probintzia"), ("fa", "استان مودنا"), ("fi", "Modenan maakunta"), ("fr", "Province de Modène"), ("gl", "Provincia de Módena"), ("gu", "મોડ\u{ac7}ના પ\u{acd}રા\u{a82}ત"), ("he", "מודנה"), ("hi", "मोड\u{947}ना प\u{94d}रा\u{902}त"), ("hu", "Modena megye"), ("hy", "Կաղապար:Մոդենայի գավառ"), ("id", "Provinsi Modena"), ("it", "provincia di Modena"), ("ja", "モデナ県"), ("jv", "Provinsi Modena"), ("ka", "მოდენის პროვინცია"), ("kn", "ಮೊಡ\u{cc6}ನಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "모데나 현"), ("lt", "Modenos provincija"), ("lv", "Modēnas province"), ("mr", "मोड\u{947}ना प\u{94d}रा\u{902}त"), ("ms", "Wilayah Modena"), ("nb", "Provinsen Modena"), ("nl", "Modena"), ("no", "Provinsen Modena"), ("pl", "Prowincja Modena"), ("pt", "Módena"), ("ro", "Provincia Modena"), ("ru", "Модена"), ("si", "මොඩෙන\u{dcf} පළ\u{dcf}ත"), ("sl", "Modena"), ("sr", "Модена"), ("sr_Latn", "Modena"), ("sv", "Modena"), ("ta", "மொடேன\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c3e}డ\u{c46}న\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโมเดนา"), ("tr", "Modena ili"), ("uk", "Провінція Модена"), ("ur", "صوبہ مودینا"), ("uz", "Modena"), ("vi", "Modena"), ("zh", "摩德納省")]),
                        unofficial_name_list: ["Province of Modena"].to_vec(),
                    }
                ),
                (
                    "MS",
                    Subdivision{
                        name: "MS",
                        country_alpha2: Alpha2::IT,
                        code: "MS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.0793245), longitude: Some(10.097677), max_latitude: Some(44.0927356), min_latitude: Some(44.0354134), max_longitude: Some(10.108908), min_longitude: Some(10.0280523)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ماسا كرارا"), ("be", "правінцыя Маса-Карара"), ("bg", "Маса и Карара"), ("bn", "মেস\u{9be} ও ক\u{9be}র\u{9be}র\u{9be}র প\u{9cd}রদেশ"), ("ca", "Província de Massa-Carrara"), ("ccp", "𑄟𑄥 𑄃\u{11133}𑄃 𑄇𑄬𑄢𑄢"), ("ceb", "Provincia di Massa-Carrara"), ("cs", "Provincie Massa-Carrara"), ("da", "Massa-Carrara"), ("de", "Provinz Massa-Carrara"), ("el", "Μάσσα-Καρράρα"), ("en", "Massa and Carrara"), ("es", "Massa y Carrara"), ("et", "Massa ja Carrara provints"), ("eu", "Massa-Carrarako probintzia"), ("fa", "استان ماسا-کارارا"), ("fi", "Massa-Carraran maakunta"), ("fr", "Province de Massa-Carrara"), ("gl", "Provincia de Massa-Carrara"), ("gu", "માસા એન\u{acd}ડ કારારા પ\u{acd}રા\u{a82}ત"), ("he", "מאסה-קרארה"), ("hi", "मासा एव\u{902} करारा प\u{94d}रा\u{902}त"), ("hu", "Massa-Carrara megye"), ("hy", "Մասսա-Կարարա"), ("id", "Provinsi Massa-Carrara"), ("it", "provincia di Massa e Carrara"), ("jv", "Provinsi Massa-Carrara"), ("ka", "მასისა და კარარის პროვინცია"), ("kn", "ಮಾಸಾ ಮತ\u{ccd}ತು ಕರಾರಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "마사에카라라 현"), ("lt", "Masos-Kararos provincija"), ("lv", "Masas-Karrāras province"), ("mk", "Маса-Карара"), ("mr", "मासा अ\u{901}ड क\u{945}राररा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Massa-Carrara"), ("nb", "Massa-Carrara"), ("nl", "Massa-Carrara"), ("no", "Massa-Carrara"), ("pl", "Prowincja Massa-Carrara"), ("pt", "Massa-Carrara"), ("ro", "Provincia Massa-Carrara"), ("ru", "Масса-Каррара"), ("si", "ක\u{dca}\u{200d}ර\u{dd2}ස\u{dca}මස\u{dca} ද\u{dd6}පත\u{dca}"), ("sl", "Massa-Carrara"), ("sr", "Маса-Карара"), ("sr_Latn", "Masa-Karara"), ("sv", "Massa-Carrara"), ("ta", "ம\u{bbe}ஸ\u{bbe} அண\u{bcd}ட\u{bcd} கற\u{bcd}ற\u{bbe}ரே ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c3e}స\u{c3e} మర\u{c3f}యు కర\u{c3e}ర\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมาซซาคาร\u{e4c}ราร\u{e48}า"), ("tr", "Massa ve Carrara ili"), ("uk", "Провінція Масса-Каррара"), ("ur", "صوبہ ماسا اور کارارا"), ("uz", "Massa-Carrara"), ("vi", "Massa-Carrara"), ("zh", "馬薩-卡拉拉省")]),
                        unofficial_name_list: ["Province of Massa-Carrara"].to_vec(),
                    }
                ),
                (
                    "MT",
                    Subdivision{
                        name: "MT",
                        country_alpha2: Alpha2::IT,
                        code: "MT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.666379), longitude: Some(16.6043199), max_latitude: Some(40.6907831), min_latitude: Some(40.6384299), max_longitude: Some(16.6281972), min_longitude: Some(16.5720606)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ماتيرا"), ("be", "правінцыя Матэра"), ("bg", "Матера"), ("ca", "Província de Matera"), ("ccp", "𑄟𑄑𑄬𑄢"), ("ceb", "Matera"), ("cs", "Provincie Matera"), ("de", "Provinz Matera"), ("el", "Ματέρα"), ("en", "Matera"), ("es", "Matera"), ("et", "Matera provints"), ("eu", "Materako probintzia"), ("fa", "استان ماترا"), ("fi", "Materan maakunta"), ("fr", "Province de Matera"), ("gl", "Provincia de Matera"), ("he", "מטרה"), ("hu", "Matera megye"), ("hy", "Մատերա"), ("id", "Provinsi Matera"), ("it", "provincia di Matera"), ("ja", "マテーラ県"), ("jv", "Provinsi Matera"), ("ka", "მატერის პროვინცია"), ("kk", "Матера"), ("ko", "마테라 현"), ("lt", "Materos provincija"), ("lv", "Matēras province"), ("ms", "Wilayah Matera"), ("nb", "Provinsen Matera"), ("nl", "Matera"), ("no", "Provinsen Matera"), ("pl", "Prowincja Matera"), ("pt", "Matera"), ("ro", "Provincia Matera"), ("ru", "Матера"), ("sl", "Matera"), ("sq", "Provinca e Materës"), ("sr", "Матера"), ("sr_Latn", "Matera"), ("sv", "Matera"), ("ta", "ம\u{bbe}தேர\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("tr", "Matera ili"), ("uk", "Провінція Матера"), ("ur", "صوبہ ماتیرا"), ("uz", "Matera"), ("vi", "Matera"), ("zh", "馬泰拉省")]),
                        unofficial_name_list: ["Province of Matera"].to_vec(),
                    }
                ),
                (
                    "NA",
                    Subdivision{
                        name: "NA",
                        country_alpha2: Alpha2::IT,
                        code: "NA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.8517746), longitude: Some(14.2681244), max_latitude: Some(40.9159348), min_latitude: Some(40.7920363), max_longitude: Some(14.3537148), min_longitude: Some(14.1394899)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة نابولي"), ("az", "Neapol əyaləti"), ("be", "Неапаль"), ("bg", "Неапол"), ("bn", "নেপলস প\u{9cd}রদেশ"), ("ca", "Ciutat metropolitana de Nàpols"), ("ccp", "𑄚\u{11133}𑄠𑄛\u{11134}𑄣𑄬𑄌\u{11134}"), ("ceb", "Napoles"), ("cs", "Provincie Napoli"), ("da", "Napoli"), ("de", "Metropolitanstadt Neapel"), ("el", "Νάπολη"), ("en", "Naples"), ("es", "Ciudad metropolitana de Nápoles"), ("et", "Città metropolitana di Napoli"), ("eu", "Napoliko probintzia"), ("fa", "استان ناپل"), ("fi", "Napolin metropolikaupunki"), ("fr", "Ville métropolitaine de Naples"), ("gl", "Provincia de Nápoles"), ("gu", "ન\u{ac7}પલ\u{acd}સ પ\u{acd}રા\u{a82}ત"), ("he", "נאפולי"), ("hi", "न\u{947}पल\u{94d}स प\u{94d}रा\u{902}त"), ("hr", "Napulj (pokrajina)"), ("hu", "Nápoly megye"), ("id", "Provinsi Napoli"), ("it", "città metropolitana di Napoli"), ("ja", "ナポリ県"), ("jv", "Provinsi Napoli"), ("ka", "ნეაპოლის პროვინცია"), ("kn", "ನೇಪಲ\u{ccd}ಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "나폴리 광역시"), ("lt", "Neapolio provincija"), ("lv", "Neapoles province"), ("mk", "Неапол"), ("mr", "न\u{945}पल\u{94d}ज\u{93c} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Naples"), ("nb", "Provinsen Napoli"), ("nl", "Napels"), ("no", "Provinsen Napoli"), ("pl", "Prowincja Neapol"), ("pt", "Nápoles"), ("ro", "Provincia Napoli"), ("ru", "Неаполь"), ("si", "නේපල\u{dca}ස\u{dca} පළ\u{dcf}ත"), ("sl", "Napoli"), ("sq", "Provinca e Napolit"), ("sr", "Напуљ"), ("sr_Latn", "Napulj"), ("sv", "Neapel"), ("ta", "நேபிள\u{bcd}ஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "న\u{c3e}ప\u{c46}ల\u{c4d}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เนเป\u{e34}ลส\u{e4c}"), ("tr", "Napoli ili"), ("uk", "Неаполь"), ("ur", "صوبہ ناپولی"), ("uz", "Neapol"), ("vi", "Tỉnh Napoli"), ("yue", "拿坡利省"), ("yue_Hans", "拿坡利省"), ("zh", "那不勒斯廣域市")]),
                        unofficial_name_list: ["Metropolitan City of Naples"].to_vec(),
                    }
                ),
                (
                    "NO",
                    Subdivision{
                        name: "NO",
                        country_alpha2: Alpha2::IT,
                        code: "NO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.44692999999999), longitude: Some(8.622161199999999), max_latitude: Some(45.4884906), min_latitude: Some(45.4191674), max_longitude: Some(8.6576135), min_longitude: Some(8.573948399999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة نوفارا"), ("be", "правінцыя Навара"), ("bg", "Новара"), ("bn", "নোভ\u{9be}র\u{9be}-র প\u{9cd}রদেশ"), ("ca", "Província de Novara"), ("ccp", "𑄚\u{11127}𑄞𑄢"), ("ceb", "Provincia di Novara"), ("cs", "Provincie Novara"), ("da", "Novara"), ("de", "Provinz Novara"), ("el", "Επαρχία της Νοβάρα"), ("en", "Novara"), ("es", "Novara"), ("et", "Novara provints"), ("eu", "Novarako probintzia"), ("fa", "استان نووارا"), ("fi", "Novaran maakunta"), ("fr", "Province de Novare"), ("gl", "Provincia de Novara"), ("gu", "નોવારા પ\u{acd}રા\u{a82}ત"), ("hi", "नोवारा प\u{94d}रा\u{902}त"), ("hu", "Novara megye"), ("hy", "Նովարա"), ("id", "Provinsi Novara"), ("is", "Novara"), ("it", "provincia di Novara"), ("ja", "ノヴァーラ県"), ("jv", "Provinsi Novara"), ("ka", "ნოვარის პროვინცია"), ("kn", "ನೊವಾರಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "노바라 현"), ("lt", "Novaros provincija"), ("lv", "Novāras province"), ("mr", "नोवारा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Novara"), ("nb", "Provinsen Novara"), ("nl", "Novara"), ("no", "Provinsen Novara"), ("pl", "Prowincja Novara"), ("pt", "Novara"), ("ro", "Provincia Novara"), ("ru", "Новара"), ("si", "නොව\u{dcf}ර\u{dcf} පළ\u{dcf}ත"), ("sl", "Novara"), ("sq", "Provinca e Novarës"), ("sr", "Новара"), ("sr_Latn", "Novara"), ("sv", "Novara"), ("ta", "நொவ\u{bbe}ர\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d} ఆఫ\u{c4d} న\u{c4b}వ\u{c3e}ర\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดโนวารา"), ("tr", "Novara ili"), ("uk", "Провінція Новара"), ("ur", "صوبہ نووارا"), ("uz", "Novara"), ("vi", "Novara"), ("yue", "羅華拉省"), ("yue_Hans", "罗华拉省"), ("zh", "諾瓦拉省")]),
                        unofficial_name_list: ["Novara", "Province of Novara"].to_vec(),
                    }
                ),
                (
                    "NU",
                    Subdivision{
                        name: "NU",
                        country_alpha2: Alpha2::IT,
                        code: "NU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.32024210000001), longitude: Some(9.3264377), max_latitude: Some(40.3334335), min_latitude: Some(40.3053262), max_longitude: Some(9.343346), min_longitude: Some(9.279806599999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة نورو"), ("be", "правінцыя Нуора"), ("bg", "Нуоро"), ("bn", "ন\u{9c1}রোর প\u{9cd}রদেশ"), ("ca", "Província de Nuoro"), ("ccp", "𑄚\u{11127}𑄅\u{1112a}𑄢\u{1112a}"), ("ceb", "Provincia di Nuoro"), ("cs", "Provincie Nuoro"), ("da", "Nuoro"), ("de", "Provinz Nuoro"), ("el", "Νουόρο"), ("en", "Nuoro"), ("es", "Nuoro"), ("et", "Nuoro provints"), ("eu", "Nuoroko probintzia"), ("fa", "استان نیورو"), ("fi", "Nuoron maakunta"), ("fr", "Province de Nuoro"), ("gl", "Provincia de Nuoro"), ("gu", "ન\u{acd}ય\u{ac1}ઓરો પ\u{acd}રા\u{a82}ત"), ("he", "נואורו"), ("hi", "न\u{94d}य\u{941}ओरो प\u{94d}रा\u{902}त"), ("hu", "Nuoro megye"), ("hy", "Նուորո"), ("id", "Provinsi Nuoro"), ("it", "provincia di Nuoro"), ("ja", "ヌーオロ県"), ("jv", "Provinsi Nuoro"), ("ka", "ნუოროს პროვინცია"), ("kn", "ನುವೊರೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "누오로 현"), ("lt", "Nuoro provincija"), ("lv", "Nuoro province"), ("mr", "न\u{941}ऑरोचा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Nuoro"), ("nb", "Provinsen Nuoro"), ("nl", "Nuoro"), ("no", "Provinsen Nuoro"), ("pl", "Prowincja Nuoro"), ("pt", "Nuoro"), ("ro", "Provincia Nuoro"), ("ru", "Нуоро"), ("si", "න\u{dd4}ඔරෝ පළ\u{dcf}ත"), ("sl", "Nuoro"), ("sr", "Нуоро"), ("sr_Latn", "Nuoro"), ("sv", "Nuoro"), ("ta", "நுவரோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "న\u{c4d}యూర\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดน\u{e39}ออโร"), ("tr", "Nuoro ili"), ("uk", "Провінція Нуоро"), ("ur", "صوبہ نوورو"), ("uz", "Nuoro"), ("vi", "Nuoro"), ("zh", "努奧羅省")]),
                        unofficial_name_list: ["Province of Nuoro"].to_vec(),
                    }
                ),
                (
                    "OR",
                    Subdivision{
                        name: "OR",
                        country_alpha2: Alpha2::IT,
                        code: "OR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.906182), longitude: Some(8.5883863), max_latitude: Some(39.9170048), min_latitude: Some(39.8821483), max_longitude: Some(8.6094668), min_longitude: Some(8.5785921)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة أوريستانو"), ("be", "Арыстана"), ("bg", "Ористано"), ("bn", "ওরিস\u{9cd}ট\u{9be}নো প\u{9cd}রদেশ"), ("ca", "província d’Oristany"), ("ccp", "𑄃\u{11127}𑄢\u{11128}𑄌\u{11134}𑄑𑄚\u{1112e}"), ("ceb", "Provincia di Oristano"), ("cs", "Provincie Oristano"), ("da", "Oristano"), ("de", "Provinz Oristano"), ("el", "Οριστάνο"), ("en", "Oristano"), ("es", "Oristán"), ("et", "Oristano provints"), ("eu", "Oristanoko probintzia"), ("fa", "استان اریستانو"), ("fi", "Oristanon maakunta"), ("fr", "Province d’Oristano"), ("gl", "Provincia de Oristano"), ("gu", "ઓરિસટાનો પ\u{acd}રા\u{a82}ત"), ("hi", "ओरिस\u{94d}तानो प\u{94d}रा\u{902}त"), ("hu", "Oristano megye"), ("hy", "Օրիստանո"), ("id", "Provinsi Oristano"), ("it", "provincia di Oristano"), ("ja", "オリスターノ県"), ("jv", "Provinsi Oristano"), ("ka", "ორისტანოს პროვინცია"), ("kn", "ಓರ\u{cbf}ಸ\u{ccd}ಟಾನಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "오리스타노 현"), ("lt", "Oristano provincija"), ("lv", "Oristāno province"), ("mk", "Ористано"), ("mr", "ओरि\u{902}टो प\u{94d}रा\u{902}त"), ("ms", "Wilayah Oristano"), ("nb", "Provinsen Oristano"), ("nl", "Oristano"), ("no", "Provinsen Oristano"), ("pl", "Prowincja Oristano"), ("pt", "Oristano"), ("ro", "Provincia Oristano"), ("ru", "Ористано"), ("si", "ඔර\u{dd2}ස\u{dca}ට\u{dcf} පළ\u{dcf}ත"), ("sl", "Oristano"), ("sr", "Ористано"), ("sr_Latn", "Oristano"), ("sv", "Oristano"), ("ta", "ஒரிஸ\u{bcd}டனோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఓర\u{c3f}స\u{c4d}ట\u{c3e}న\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโอร\u{e34}สตาโน"), ("tr", "Oristano ili"), ("uk", "Провінція Орістано"), ("ur", "صوبہ اوریستانو"), ("uz", "Oristano"), ("vi", "Oristano"), ("zh", "奧里斯塔諾省")]),
                        unofficial_name_list: ["Province of Oristano"].to_vec(),
                    }
                ),
                (
                    "PA",
                    Subdivision{
                        name: "PA",
                        country_alpha2: Alpha2::IT,
                        code: "PA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.1156879), longitude: Some(13.3612671), max_latitude: Some(38.2194654), min_latitude: Some(38.0615392), max_longitude: Some(13.4471566), min_longitude: Some(13.2674205)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة باليرمو"), ("be", "правінцыя Палерма"), ("bg", "Палермо"), ("ca", "Província de Palerm"), ("ccp", "𑄛𑄣𑄬𑄢\u{11134}𑄟\u{1112e}"), ("ceb", "Palermo"), ("cs", "Provincie Palermo"), ("de", "Provinz Palermo"), ("el", "Παλέρμο"), ("en", "Palermo"), ("es", "Palermo"), ("et", "Palermo provints"), ("eu", "Palermoko probintzia"), ("fa", "استان پالرمو"), ("fi", "Palermon maakunta"), ("fr", "Province de Palerme"), ("gl", "Provincia de Palermo"), ("he", "פלרמו"), ("hr", "Palermo"), ("hu", "Palermo megye"), ("hy", "Պալերմո"), ("id", "Provinsi Palermo"), ("it", "provincia di Palermo"), ("ja", "パレルモ県"), ("jv", "Provinsi Palermo"), ("ka", "პალერმოს პროვინცია"), ("ko", "팔레르모 현"), ("lt", "Palermo provincija"), ("lv", "Palermo province"), ("mn", "Палермо аймаг"), ("ms", "Wilayah Palermo"), ("nb", "Provinsen Palermo"), ("nl", "Palermo"), ("no", "Provinsen Palermo"), ("pl", "Prowincja Palermo"), ("pt", "Palermo"), ("ro", "Provincia Palermo"), ("ru", "Палермо"), ("sk", "Palermo"), ("sl", "Palermo"), ("sq", "Provinca e Palermos"), ("sr", "Палермо"), ("sr_Latn", "Palermo"), ("sv", "Palermo"), ("tr", "Palermo ili"), ("uk", "Провінція Палермо"), ("ur", "صوبہ پالیرمو"), ("uz", "Palermo"), ("vi", "Palermo"), ("zh", "巴勒莫省")]),
                        unofficial_name_list: ["Metropolitan City of Palermo"].to_vec(),
                    }
                ),
                (
                    "PC",
                    Subdivision{
                        name: "PC",
                        country_alpha2: Alpha2::IT,
                        code: "PC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.0526206), longitude: Some(9.6929845), max_latitude: Some(45.0691256), min_latitude: Some(45.0154144), max_longitude: Some(9.7612318), min_longitude: Some(9.635065599999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بياتشنزا"), ("bg", "Пиаченца"), ("ca", "Província de Piacenza"), ("ccp", "𑄛\u{11128}𑄠𑄥𑄬𑄚\u{11134}𑄎"), ("ceb", "Piacenza"), ("cs", "Provincie Piacenza"), ("da", "Piacenza"), ("de", "Provinz Piacenza"), ("el", "Επαρχία της Πιατσέντζα"), ("en", "Piacenza"), ("es", "Piacenza"), ("et", "Piacenza provints"), ("eu", "Piacenzako probintzia"), ("fa", "استان پیاچنزا"), ("fi", "Piacenzan maakunta"), ("fr", "Province de Plaisance"), ("gl", "Provincia de Piacenza"), ("he", "פיאצ׳נצה"), ("hu", "Piacenza megye"), ("hy", "Պիաչենցա"), ("id", "Provinsi Piacenza"), ("it", "provincia di Piacenza"), ("ja", "ピアチェンツァ県"), ("jv", "Provinsi Piacenza"), ("ka", "პიაჩენცის პროვინცია"), ("ko", "피아첸차 현"), ("lt", "Pjačencos provincija"), ("lv", "Pjačencas province"), ("ms", "Wilayah Piacenza"), ("nb", "Provinsen Piacenza"), ("nl", "Piacenza"), ("no", "Provinsen Piacenza"), ("pl", "Prowincja Piacenza"), ("pt", "Placência"), ("ro", "Provincia Piacenza"), ("ru", "Пьяченца"), ("sl", "Piacenza"), ("sr", "Пјаченца"), ("sr_Latn", "Pjačenca"), ("sv", "Piacenza"), ("th", "จ\u{e31}งหว\u{e34}ดป\u{e35}อาเชนซา"), ("tr", "Piacenza ili"), ("uk", "Провінція Пʼяченца"), ("ur", "صوبہ پیاچنزا"), ("uz", "Piacenza"), ("vi", "Piacenza"), ("zh", "皮亚琴察省")]),
                        unofficial_name_list: ["Province of Piacenza"].to_vec(),
                    }
                ),
                (
                    "PD",
                    Subdivision{
                        name: "PD",
                        country_alpha2: Alpha2::IT,
                        code: "PD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.4064349), longitude: Some(11.8767611), max_latitude: Some(45.4575002), min_latitude: Some(45.3555073), max_longitude: Some(11.9728649), min_longitude: Some(11.809626)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بادوفا"), ("be", "Падуя"), ("bg", "Падуа"), ("bn", "পোদ\u{9c1}য\u{9bc}\u{9be}র প\u{9cd}রদেশ"), ("ca", "Província de Pàdua"), ("ccp", "𑄛𑄘\u{11131}"), ("ceb", "Provincia di Padova"), ("cs", "Provincie Padova"), ("da", "Padova"), ("de", "Provinz Padua"), ("el", "Πάδοβα"), ("en", "Padua"), ("es", "Padua"), ("et", "Padova provints"), ("eu", "Paduako probintzia"), ("fa", "استان پادووا"), ("fi", "Padovan maakunta"), ("fr", "Province de Padoue"), ("ga", "Padova"), ("gl", "Provincia de Padua"), ("gu", "પડ\u{ac1}આ પ\u{acd}રા\u{a82}ત"), ("he", "פדובה"), ("hi", "पाद\u{941}आ प\u{94d}रा\u{902}त"), ("hr", "Padova (pokrajina)"), ("hu", "Padova megye"), ("hy", "Պադովա"), ("id", "Provinsi Padova"), ("it", "provincia di Padova"), ("ja", "パドヴァ県"), ("jv", "Provinsi Padova"), ("ka", "პადუის პროვინცია"), ("kn", "ಪಡುವಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "파도바 현"), ("lt", "Paduvos provincija"), ("lv", "Padujas province"), ("mk", "Падова"), ("mr", "पड\u{941}आ प\u{94d}रा\u{902}त"), ("ms", "Wilayah Padua"), ("nb", "Provinsen Padova"), ("nl", "Padua"), ("no", "Provinsen Padova"), ("pl", "Prowincja Padwa"), ("pt", "Pádua"), ("ro", "Provincia Padova"), ("ru", "Падуя"), ("si", "පදොව\u{dcf} පළ\u{dcf}ත"), ("sl", "Padova"), ("sq", "Provinca e Padovës"), ("sr", "Падова (округ)"), ("sr_Latn", "Padova (okrug)"), ("sv", "Padova"), ("ta", "படுவ\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c3e}డువ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดปาด\u{e31}ว"), ("tr", "Padova ili"), ("uk", "Провінція Падуя"), ("ur", "صوبہ پادووا"), ("uz", "Padova"), ("vi", "Padova"), ("zh", "帕多瓦省")]),
                        unofficial_name_list: ["Province of Padua"].to_vec(),
                    }
                ),
                (
                    "PE",
                    Subdivision{
                        name: "PE",
                        country_alpha2: Alpha2::IT,
                        code: "PE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.4617902), longitude: Some(14.2160898), max_latitude: Some(42.4938717), min_latitude: Some(42.4171602), max_longitude: Some(14.2543544), min_longitude: Some(14.1528712)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بسكارا"), ("be", "правінцыя Пескара"), ("bg", "Пескара"), ("bn", "প\u{9cd}য\u{9be}স\u{9be}র\u{9be}র প\u{9cd}রদেশ"), ("ca", "Província de Pescara"), ("ccp", "𑄛𑄬𑄌\u{11134}𑄇𑄢"), ("ceb", "Pescara"), ("cs", "Provincie Pescara"), ("da", "Pescara"), ("de", "Provinz Pescara"), ("el", "Επαρχία της Πεσκάρα"), ("en", "Pescara"), ("es", "Pescara"), ("et", "Pescara provints"), ("eu", "Pescarako probintzia"), ("fa", "استان پسکارا"), ("fi", "Pescaran maakunta"), ("fr", "Province de Pescara"), ("gl", "Provincia de Pescara"), ("gu", "પ\u{ac7}સ\u{acd}કારા પ\u{acd}રા\u{a82}ત"), ("he", "פסקארה"), ("hi", "प\u{947}स\u{94d}कारा क\u{947} प\u{94d}रा\u{902}त"), ("hu", "Pescara megye"), ("hy", "Պեսկարա"), ("id", "Provinsi Pescara"), ("it", "provincia di Pescara"), ("ja", "ペスカーラ県"), ("jv", "Provinsi Pescara"), ("ka", "პესკარის პროვინცია"), ("kn", "ಪ\u{cc6}ಸ\u{ccd}ಕಾರಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "페스카라 현"), ("lt", "Peskaros provincija"), ("lv", "Peskāras province"), ("mr", "प\u{947}स\u{94d}कारा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Pescara"), ("nb", "Provinsen Pescara"), ("nl", "Pescara"), ("no", "Provinsen Pescara"), ("pl", "Prowincja Pescara"), ("pt", "Pescara"), ("ro", "Provincia Pescara"), ("ru", "Пескара"), ("si", "පෙස\u{dca}ක\u{dcf}ර\u{dcf} පළ\u{dcf}ත"), ("sl", "Pescara"), ("sq", "Provinca e Peskarës"), ("sr", "Пескара"), ("sr_Latn", "Peskara"), ("sv", "Pescara"), ("ta", "பேசக\u{bcd}க\u{bbe}ர ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c46}స\u{c4d}క\u{c3e}ర\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเพสการา"), ("tr", "Pescara ili"), ("uk", "Провінція Пескара"), ("ur", "صوبہ پسکارا"), ("uz", "Pescara"), ("vi", "Pescara"), ("zh", "佩斯卡拉省")]),
                        unofficial_name_list: ["Province of Pescara"].to_vec(),
                    }
                ),
                (
                    "PG",
                    Subdivision{
                        name: "PG",
                        country_alpha2: Alpha2::IT,
                        code: "PG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.1107168), longitude: Some(12.3908279), max_latitude: Some(43.1499405), min_latitude: Some(43.0400963), max_longitude: Some(12.4521977), min_longitude: Some(12.3084506)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بيرودجا"), ("be", "правінцыя Перуджа"), ("bg", "Перуджа"), ("bn", "পের\u{9c1}গিয\u{9bc}\u{9be}র প\u{9cd}রদেশ"), ("ca", "Província de Perusa"), ("ccp", "𑄛𑄬𑄢\u{1112a}𑄉\u{11128}𑄠"), ("ceb", "Provincia di Perugia"), ("cs", "Provincie Perugia"), ("da", "Perugia"), ("de", "Provinz Perugia"), ("el", "Περούτζια"), ("en", "Perugia"), ("es", "Perugia"), ("et", "Perugia provints"), ("eu", "Perugiako probintzia"), ("fa", "استان پروجا"), ("fi", "Perugian maakunta"), ("fr", "Province de Pérouse"), ("gl", "Provincia de Perugia"), ("gu", "પ\u{ac7}ર\u{ac2}ગિયા પ\u{acd}રા\u{a82}ત"), ("he", "פרוג׳ה"), ("hi", "प\u{947}र\u{941}जिया प\u{94d}रा\u{902}त"), ("hr", "Perugia"), ("hu", "Perugia megye"), ("hy", "Պերուջա"), ("id", "Provinsi Perugia"), ("it", "provincia di Perugia"), ("ja", "ペルージャ県"), ("jv", "Provinsi Perugia"), ("ka", "პერუჯა"), ("kn", "ಪ\u{cc6}ರುಗ\u{cbf}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "페루자 현"), ("lt", "Perudžos provincija"), ("lv", "Perudžas province"), ("mk", "Перуџа"), ("mr", "पर\u{942}गिया प\u{94d}रा\u{902}त"), ("ms", "Wilayah Perugia"), ("nb", "Provinsen Perugia"), ("nl", "Perugia"), ("no", "Provinsen Perugia"), ("pl", "Prowincja Perugia"), ("pt", "Perúgia"), ("ro", "Provincia Perugia"), ("ru", "Перуджа"), ("si", "පෙර\u{dd4}ග\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sl", "Perugia"), ("sq", "Provinca e Peruxhias"), ("sr", "Перуђа"), ("sr_Latn", "Peruđa"), ("sv", "Perugia"), ("ta", "பெருகிய ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c46}రూగ\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเปร\u{e39}เก\u{e35}ย"), ("tr", "Perugia ili"), ("uk", "Провінція Перуджа"), ("ur", "صوبہ پیروجا"), ("uz", "Perugia"), ("vi", "Perugia"), ("zh", "佩魯賈省")]),
                        unofficial_name_list: ["Province of Perugia"].to_vec(),
                    }
                ),
                (
                    "PI",
                    Subdivision{
                        name: "PI",
                        country_alpha2: Alpha2::IT,
                        code: "PI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.7228386), longitude: Some(10.4016888), max_latitude: Some(43.7394813), min_latitude: Some(43.6740201), max_longitude: Some(10.4453807), min_longitude: Some(10.3453756)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بيزا"), ("be", "Піза"), ("bg", "Пиза"), ("bn", "পিস\u{9be}-র প\u{9cd}রদেশ"), ("ca", "Província de Pisa"), ("ccp", "𑄛\u{1112d}𑄥"), ("ceb", "Province of Pisa"), ("cs", "Provincie Pisa"), ("da", "Province of Pisa"), ("de", "Provinz Pisa"), ("el", "Πίζα"), ("en", "Pisa"), ("es", "Pisa"), ("et", "Pisa provints"), ("eu", "Pisako probintzia"), ("fa", "استان پیزا"), ("fi", "Pisan maakunta"), ("fr", "Province de Pise"), ("gl", "Provincia de Pisa"), ("gu", "પીઝા પ\u{acd}રા\u{a82}ત"), ("he", "פיזה"), ("hi", "पीसा प\u{94d}रा\u{902}त"), ("hu", "Pisa megye"), ("hy", "Պիզա"), ("id", "Provinsi Pisa"), ("it", "provincia di Pisa"), ("ja", "ピサ県"), ("jv", "Provinsi Pisa"), ("ka", "პიზის პროვინცია"), ("kn", "ಪ\u{cbf}ಸಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "피사 현"), ("lt", "Pizos provincija"), ("lv", "Pizas province"), ("mr", "पिसा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Pisa"), ("nb", "Pisa"), ("nl", "Pisa"), ("no", "Pisa"), ("pl", "Prowincja Piza"), ("pt", "Pisa"), ("ro", "Provincia Pisa"), ("ru", "Пиза"), ("si", "ප\u{dd2}ස\u{dcf} පළ\u{dcf}ත"), ("sk", "Pisa"), ("sl", "Pisa"), ("sq", "Provinca e Pizës"), ("sr", "Пиза"), ("sr_Latn", "Piza"), ("sv", "Pisa"), ("ta", "பிச\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c40}స\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดป\u{e34}ซา"), ("tr", "Pisa ili"), ("uk", "Провінція Піза"), ("ur", "صوبہ پیسا"), ("uz", "Pisa"), ("vi", "Pisa"), ("zh", "比薩省")]),
                        unofficial_name_list: ["Province of Pisa"].to_vec(),
                    }
                ),
                (
                    "PN",
                    Subdivision{
                        name: "PN",
                        country_alpha2: Alpha2::IT,
                        code: "PN",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::DecentralizedRegionalEntity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بوردينوني"), ("be", "правінцыя Пардэнонэ"), ("bg", "Порденоне"), ("bn", "পোর\u{9cd}ডেনোন-এর প\u{9cd}রদেশ"), ("ca", "Província de Pordenone"), ("ccp", "𑄛\u{11127}𑄢\u{11134}𑄓𑄬𑄚\u{11127}𑄚\u{11134}"), ("ceb", "Pordenone"), ("cs", "Provincie Pordenone"), ("da", "Pordenone"), ("de", "Provinz Pordenone"), ("el", "Πορντενόνε"), ("en", "Pordenone"), ("es", "Pordenone"), ("et", "Pordenone provints"), ("eu", "Pordenoneko probintzia"), ("fa", "استان پوردنونه"), ("fi", "Pordenonen maakunta"), ("fr", "Province de Pordenone"), ("gl", "Provincia de Pordenone"), ("gu", "પોર\u{acd}ડ\u{ac7}નોન પ\u{acd}રા\u{a82}ત"), ("he", "פורדנונה"), ("hi", "पोरद\u{947}नोन प\u{94d}रा\u{902}त"), ("hr", "Pordenone"), ("hu", "Pordenone megye"), ("hy", "Պորդենոնե"), ("id", "Provinsi Pordenone"), ("it", "provincia di Pordenone"), ("ja", "ポルデノーネ県"), ("jv", "Provinsi Pordenone"), ("ka", "პორდენონეს პროვინცია"), ("kn", "ಪೋರ\u{ccd}ಡ\u{cc6}ನೊನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "포르데노네 현"), ("lt", "Pordenonės provincija"), ("lv", "Pordenones province"), ("mk", "Порденоне"), ("mr", "पॉर\u{94d}नड\u{947}नोन प\u{94d}रा\u{902}त"), ("ms", "Wilayah Pordenone"), ("nl", "Pordenone"), ("no", "Provinsen Pordenone"), ("pl", "Prowincja Pordenone"), ("pt", "Pordenone"), ("ro", "Provincia Pordenone"), ("ru", "Порденоне"), ("si", "පොර\u{dca}ඩෙනොනේ පළ\u{dcf}ත"), ("sl", "Pordenone"), ("sq", "Provinca e Pordenones"), ("sr", "Порденоне (округ)"), ("sr_Latn", "Pordenone (okrug)"), ("sv", "Pordenone"), ("ta", "போர\u{bcd}டேன\u{bbe}னே ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d} ఆప\u{c4d} ట\u{c4d}ర\u{c46}వ\u{c3f}స\u{c4b}"), ("th", "จ\u{e31}งหว\u{e31}ดปอร\u{e4c}เดโนน\u{e35}"), ("tr", "Pordenone ili"), ("uk", "Провінція Порденоне"), ("ur", "صوبہ پوردینونے"), ("uz", "Pordenone"), ("vi", "Pordenone")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "PO",
                    Subdivision{
                        name: "PO",
                        country_alpha2: Alpha2::IT,
                        code: "PO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.8777049), longitude: Some(11.102228), max_latitude: Some(43.9252112), min_latitude: Some(43.8314126), max_longitude: Some(11.1474862), min_longitude: Some(11.0202649)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة براتو"), ("be", "правінцыя Прата"), ("bg", "Прато"), ("bn", "প\u{9cd}র\u{9be}টো-এর প\u{9cd}রদেশ"), ("ca", "Província de Prato"), ("ccp", "𑄛\u{11133}𑄢𑄑\u{1112e}"), ("ceb", "Provincia di Prato"), ("cs", "Provincie Prato"), ("da", "Province of Prato"), ("de", "Provinz Prato"), ("el", "Πράτο"), ("en", "Prato"), ("es", "Prato"), ("et", "Prato provints"), ("eu", "Pratoko probintzia"), ("fa", "استان پراتو"), ("fi", "Praton maakunta"), ("fr", "Province de Prato"), ("gl", "Provincia de Prato"), ("gu", "પ\u{acd}રાતો પ\u{acd}રા\u{a82}ત"), ("he", "פראטו"), ("hi", "प\u{94d}राटो प\u{94d}रा\u{902}त"), ("hu", "Prato megye"), ("hy", "Պրատո"), ("id", "Provinsi Prato"), ("it", "provincia di Prato"), ("ja", "プラート県"), ("jv", "Provinsi Prato"), ("ka", "პრატოს პროვინცია"), ("kn", "ಪ\u{ccd}ರಾಟೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "프라토 현"), ("lt", "Prato provincija"), ("lv", "Prato province"), ("mr", "प\u{94d}रतो प\u{94d}रा\u{902}त"), ("ms", "Wilayah Prato"), ("nb", "Prato"), ("nl", "Prato"), ("no", "Prato"), ("pl", "Prowincja Prato"), ("pt", "Prato"), ("ro", "Provincia Prato"), ("ru", "Прато"), ("si", "ප\u{dca}ර\u{dcf}ටෝ පළ\u{dcf}ත"), ("sl", "Prato"), ("sr", "Прато"), ("sr_Latn", "Prato"), ("sv", "Prato"), ("ta", "பிர\u{bbe}டோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c4d}ర\u{c3e}ట\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดปราโต"), ("tr", "Prato ili"), ("uk", "Провінція Прато"), ("ur", "صوبہ پراتو"), ("uz", "Prato"), ("vi", "Prato"), ("zh", "普拉托省")]),
                        unofficial_name_list: ["Province of Prato"].to_vec(),
                    }
                ),
                (
                    "PR",
                    Subdivision{
                        name: "PR",
                        country_alpha2: Alpha2::IT,
                        code: "PR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.801485), longitude: Some(10.3279036), max_latitude: Some(44.8395029), min_latitude: Some(44.7554041), max_longitude: Some(10.3842979), min_longitude: Some(10.2727904)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بارما"), ("be", "Правінцыя Парма"), ("bg", "Парма"), ("ca", "Província de Parma"), ("ccp", "𑄛𑄢\u{11134}𑄟"), ("ceb", "Parma"), ("cs", "Provincie Parma"), ("da", "Parma"), ("de", "Provinz Parma"), ("el", "Πάρμα"), ("en", "Parma"), ("es", "Parma"), ("et", "Parma provints"), ("eu", "Parmako probintzia"), ("fa", "استان پارما"), ("fi", "Parman maakunta"), ("fr", "Province de Parme"), ("gl", "Provincia de Parma"), ("he", "פארמה"), ("hu", "Parma megye"), ("hy", "Պարմա"), ("id", "Provinsi Parma"), ("it", "provincia di Parma"), ("ja", "パルマ県"), ("jv", "Provinsi Parma"), ("ka", "პარმის პროვინცია"), ("ko", "파르마 현"), ("lt", "Parmos provincija"), ("lv", "Parmas province"), ("ms", "Wilayah Parma"), ("nb", "Provinsen Parma"), ("nl", "Parma"), ("no", "Provinsen Parma"), ("pl", "Prowincja Parma"), ("pt", "Parma"), ("ro", "Provincia Parma"), ("ru", "Парма"), ("sl", "Parma"), ("sr", "Парма"), ("sr_Latn", "Parma"), ("sv", "Parma"), ("tr", "Parma ili"), ("uk", "Провінція Парма"), ("ur", "صوبہ پارما"), ("uz", "Parma"), ("vi", "Parma"), ("zh", "帕爾馬省")]),
                        unofficial_name_list: ["Province of Parma"].to_vec(),
                    }
                ),
                (
                    "PT",
                    Subdivision{
                        name: "PT",
                        country_alpha2: Alpha2::IT,
                        code: "PT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.9303475), longitude: Some(10.9078587), max_latitude: Some(44.0101074), min_latitude: Some(43.8804611), max_longitude: Some(10.9946041), min_longitude: Some(10.8661614)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بستويا"), ("be", "Правінцыя Пістоя"), ("bg", "Пистоя"), ("bn", "পিস\u{9cd}টোরিয\u{9bc}\u{9be}-র প\u{9cd}রদেশ"), ("ca", "Província de Pistoia"), ("ccp", "𑄛\u{11128}𑄠𑄌\u{11134}𑄑\u{11130}𑄠"), ("ceb", "Provincia di Pistoia"), ("cs", "Provincie Pistoia"), ("da", "Province of Pistoia"), ("de", "Provinz Pistoia"), ("el", "Πιστόια"), ("en", "Pistoia"), ("es", "Pistoia"), ("et", "Pistoia provints"), ("eu", "Pistoiako probintzia"), ("fa", "استان پیستویا"), ("fi", "Pistoian maakunta"), ("fr", "Province de Pistoia"), ("gl", "Provincia de Pistoia"), ("gu", "પિસ\u{acd}ટોયા પ\u{acd}રા\u{a82}ત"), ("he", "פיסטויה"), ("hi", "पिस\u{94d}टोआ प\u{94d}रा\u{902}त"), ("hu", "Pistoia megye"), ("hy", "Պիստոիա"), ("id", "Provinsi Pistoia"), ("it", "provincia di Pistoia"), ("ja", "ピストイア県"), ("jv", "Provinsi Pistoia"), ("ka", "პისტოიის პროვინცია"), ("kn", "ಪ\u{cbf}ಸ\u{ccd}ಟೊಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "피스토이아 현"), ("lt", "Pistojos provincija"), ("lv", "Pistoijas province"), ("mk", "Пистоја"), ("mr", "पस\u{94d}तोआ प\u{94d}रा\u{902}त"), ("ms", "Wilayah Pistoia"), ("nb", "Pistoia"), ("nl", "Pistoia"), ("no", "Pistoia"), ("pl", "Prowincja Pistoia"), ("pt", "Pistoia"), ("ro", "Provincia Pistoia"), ("ru", "Пистойя"), ("si", "ප\u{dd2}ස\u{dca}ටෝය\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sl", "Pistoia"), ("sr", "Пистоја"), ("sr_Latn", "Pistoja"), ("sv", "Pistoia"), ("ta", "பிஸ\u{bcd}டோஐய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c3f}స\u{c4d}ట\u{c4b}య\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดพ\u{e34}สโตเอ\u{e35}ย"), ("tr", "Pistoia ili"), ("uk", "Провінція Пістоя"), ("ur", "صوبہ پستویا"), ("uz", "Pistoia"), ("vi", "Pistoia")]),
                        unofficial_name_list: ["Province of Pistoia"].to_vec(),
                    }
                ),
                (
                    "PU",
                    Subdivision{
                        name: "PU",
                        country_alpha2: Alpha2::IT,
                        code: "PU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.6130118), longitude: Some(12.7135121), max_latitude: Some(43.9692744), min_latitude: Some(43.4165762), max_longitude: Some(13.1725197), min_longitude: Some(12.1854509)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بيزارو وأوربينو"), ("be", "Правінцыя Пезара-э-Урбіна"), ("bg", "Пезаро и Урбино"), ("bn", "পিস\u{9be}রো এবং উর\u{9cd}বিনো প\u{9cd}রদেশ"), ("ca", "Província de Pesaro i Urbino"), ("ccp", "𑄛\u{11128}𑄥𑄢\u{1112e} 𑄃\u{11133}𑄃 𑄅\u{1112a}𑄢\u{11134}𑄝\u{11128}𑄚\u{1112e}"), ("ceb", "Provincia di Pesaro e Urbino"), ("cs", "Provincie Pesaro e Urbino"), ("da", "Pesaro e Urbino"), ("de", "Provinz Pesaro und Urbino"), ("el", "Πέζαρο-Ουρμπίνο"), ("en", "Pesaro and Urbino"), ("es", "Pesaro y Urbino"), ("et", "Pesaro e Urbino provints"), ("eu", "Pesaro eta Urbinoko probintzia"), ("fa", "استان پزارو و اوربینو"), ("fi", "Pesaro e Urbinon maakunta"), ("fr", "Province de Pesaro et d’Urbino"), ("gl", "Provincia de Pesaro e Urbino"), ("gu", "પ\u{ac7}સારો એન\u{acd}ડ અર\u{acd}બિનો પ\u{acd}રા\u{a82}ત"), ("he", "פזארו אה אורבינו"), ("hi", "प\u{947}सारो और अरबीनो प\u{94d}रा\u{902}त"), ("hu", "Pesaro és Urbino megye"), ("hy", "Պեզարո է Ուրբինո"), ("id", "Provinsi Pesaro dan Urbino"), ("it", "provincia di Pesaro e Urbino"), ("ja", "ペーザロ・エ・ウルビーノ県"), ("jv", "Provinsi Pesaro lan Urbino"), ("ka", "პეზარო და ურბინოს პროვინცია"), ("kn", "ಪ\u{cc6}ಸಾರೊ ಮತ\u{ccd}ತು ಉರ\u{ccd}ಬ\u{cbf}ನೋ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "페사로에우르비노 현"), ("lt", "Pezaro ir Urbino provincija"), ("lv", "Pezāro un Urbīno province"), ("mk", "Пезаро и Урбино"), ("mr", "प\u{947}सारो अ\u{901}ड उरबिनो प\u{94d}रा\u{902}त"), ("ms", "Wilayah Pesaro dan Urbino"), ("nb", "Provinsen Pesaro og Urbino"), ("nl", "Pesaro e Urbino"), ("no", "Provinsen Pesaro og Urbino"), ("pl", "Prowincja Pesaro e Urbino"), ("pt", "Pesaro e Urbino"), ("ro", "Provincia Pesaro și Urbino"), ("ru", "Пезаро-э-Урбино"), ("si", "පෙස\u{dcf}රෝ සහ උර\u{dca}බ\u{dd2}නෝ"), ("sl", "Pesaro e Urbino"), ("sr", "Пезаро и Урбино"), ("sr_Latn", "Pezaro i Urbino"), ("sv", "Pesaro e Urbino"), ("ta", "பேசரோ அண\u{bcd}ட\u{bcd} ஊர\u{bcd}பினோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c46}స\u{c3e}ర\u{c4b} మర\u{c3f}యు ఉర\u{c4d}బ\u{c3f}న\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดป\u{e35}ซาโรและอ\u{e39}ร\u{e4c}บ\u{e35}โน"), ("tr", "Pesaro ve Urbino ili"), ("uk", "Провінція Пезаро і Урбіно"), ("ur", "صوبہ پیزارو اور اوربینو"), ("vi", "Pesaro và Urbino"), ("zh", "佩薩羅-烏爾比諾省")]),
                        unofficial_name_list: ["Province of Pesaro and Urbino"].to_vec(),
                    }
                ),
                (
                    "PV",
                    Subdivision{
                        name: "PV",
                        country_alpha2: Alpha2::IT,
                        code: "PV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.1847248), longitude: Some(9.1582069), max_latitude: Some(45.2099335), min_latitude: Some(45.1655946), max_longitude: Some(9.207372099999999), min_longitude: Some(9.1044182)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بافيا"), ("be", "Павія"), ("bg", "Павия"), ("bn", "প\u{9be}ভিয\u{9bc}\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Pavia"), ("ccp", "𑄛𑄞\u{11128}𑄠"), ("ceb", "Provincia di Pavia"), ("cs", "Provincie Pavia"), ("da", "Pavia"), ("de", "Provinz Pavia"), ("el", "Επαρχία της Παβία"), ("en", "Pavia"), ("es", "Pavía"), ("et", "Pavia provints"), ("eu", "Paviako probintzia"), ("fa", "استان پاویا"), ("fi", "Pavian maakunta"), ("fr", "province de Pavie"), ("gl", "Provincia de Pavía"), ("gu", "પાવીયા પ\u{acd}રા\u{a82}ત"), ("he", "פאביה"), ("hi", "पाविया प\u{94d}रा\u{902}त"), ("hu", "Pavia megye"), ("hy", "Պավիա"), ("id", "Provinsi Pavia"), ("it", "provincia di Pavia"), ("ja", "パヴィーア県"), ("jv", "Provinsi Pavia"), ("ka", "პავიის პროვინცია"), ("kn", "ಪವ\u{cbf}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "파비아 현"), ("lt", "Pavijos provincija"), ("lv", "Pāvijas province"), ("mr", "पाव\u{94d}हिया प\u{94d}रा\u{902}त"), ("ms", "Wilayah Pavia"), ("nb", "Provinsen Pavia"), ("nl", "Pavia"), ("no", "Provinsen Pavia"), ("pl", "Prowincja Pawia"), ("pt", "Pavia"), ("ro", "Provincia Pavia"), ("ru", "Павия"), ("si", "ප\u{dcf}ව\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sk", "Pavia"), ("sl", "Pavia"), ("sq", "Provinca e Pavias"), ("sr", "Павија"), ("sr_Latn", "Pavija"), ("sv", "Pavia"), ("ta", "ப\u{bbe}விய ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c3e}వ\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดปาเว\u{e35}ย"), ("tr", "Pavia ili"), ("uk", "Провінція Павія"), ("ur", "صوبہ پاویا"), ("uz", "Pavia"), ("vi", "Pavia"), ("zh", "帕維亞省")]),
                        unofficial_name_list: ["Province of Pavia"].to_vec(),
                    }
                ),
                (
                    "PZ",
                    Subdivision{
                        name: "PZ",
                        country_alpha2: Alpha2::IT,
                        code: "PZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.6404067), longitude: Some(15.8056041), max_latitude: Some(40.6652423), min_latitude: Some(40.6172146), max_longitude: Some(15.8454938), min_longitude: Some(15.7721653)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بوتنسا"), ("be", "Патэнца"), ("bg", "Потенца"), ("bn", "পতেঙ\u{9cd}গ\u{9be}-র প\u{9cd}রদেশ"), ("ca", "Província de Potenza"), ("ccp", "𑄛\u{11127}𑄑𑄬𑄚\u{11134}𑄎"), ("ceb", "Potenza"), ("cs", "Provincie Potenza"), ("da", "Province of Potenza"), ("de", "Provinz Potenza"), ("el", "Ποτέντσα"), ("en", "Potenza"), ("es", "Potenza"), ("et", "Potenza provints"), ("eu", "Potenzako probintzia"), ("fa", "استان پوتنزا"), ("fi", "Potenzan maakunta"), ("fr", "Province de Potenza"), ("gl", "Provincia de Potenza"), ("gu", "પોટ\u{ac7}ન\u{acd}ઝા પ\u{acd}રા\u{a82}ત"), ("he", "פוטנצה"), ("hi", "पोट\u{947}\u{902}ज\u{93c}ा प\u{94d}रा\u{902}त"), ("hu", "Potenza megye"), ("hy", "Պոտենցա"), ("id", "Provinsi Potenza"), ("it", "provincia di Potenza"), ("ja", "ポテンツァ県"), ("jv", "Provinsi Potenza"), ("ka", "პოტენცის პროვინცია"), ("kn", "ಪೊಟ\u{cc6}ನ\u{ccd}ಜಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "포텐차 현"), ("lt", "Potenzos provincija"), ("lv", "Potencas province"), ("mr", "पोट\u{947}न\u{94d}झा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Potenza"), ("nb", "Provinsen Potenza"), ("nl", "Potenza"), ("no", "Provinsen Potenza"), ("pl", "Prowincja Potenza"), ("pt", "Potenza"), ("ro", "Provincia Potenza"), ("ru", "Потенца"), ("si", "පොටෙන\u{dca}ස\u{dcf} පළ\u{dcf}ත"), ("sl", "Potenza"), ("sq", "Provinca e Potencës"), ("sr", "Потенца"), ("sr_Latn", "Potenca"), ("sv", "Potenza"), ("ta", "போதேன\u{bcd}ச\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c4b}ట\u{c46}ంజ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโปเตนซา"), ("tr", "Potenza ili"), ("uk", "Провінція Потенца"), ("ur", "صوبہ پوتینتسا"), ("uz", "Potenza"), ("vi", "Potenza"), ("zh", "波坦察省")]),
                        unofficial_name_list: ["Province of Potenza"].to_vec(),
                    }
                ),
                (
                    "RA",
                    Subdivision{
                        name: "RA",
                        country_alpha2: Alpha2::IT,
                        code: "RA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.4183598), longitude: Some(12.2035294), max_latitude: Some(44.4405473), min_latitude: Some(44.3841368), max_longitude: Some(12.2342168), min_longitude: Some(12.1668015)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ravenna"), ("ar", "مقاطعة رافينا"), ("be", "Правінцыя Равена"), ("bg", "Равена"), ("bn", "র\u{9be}\u{9be}ভেন\u{9cd}য\u{9be}-র প\u{9cd}রদেশ"), ("ca", "Província de Ravenna"), ("ccp", "𑄢\u{11127}𑄞\u{11128}𑄚"), ("ceb", "Ravenna"), ("cs", "Provincie Ravenna"), ("da", "Ravenna"), ("de", "Provinz Ravenna"), ("el", "Ραβέννα"), ("en", "Ravenna"), ("es", "Rávena"), ("et", "Ravenna provints"), ("eu", "Ravennako probintzia"), ("fa", "استان راونا"), ("fi", "Ravennan maakunta"), ("fr", "Province de Ravenne"), ("gl", "Provincia de Ravenna"), ("gu", "ર\u{ac7}વ\u{ac7}ના પ\u{acd}રા\u{a82}ત"), ("he", "נפת רוונה"), ("hi", "रव\u{947}ना प\u{94d}रा\u{902}त"), ("hr", "Ravenna"), ("hu", "Ravenna megye"), ("hy", "Ռավեննա"), ("id", "Provinsi Ravenna"), ("it", "provincia di Ravenna"), ("ja", "ラヴェンナ県"), ("jv", "Provinsi Ravenna"), ("ka", "რავენის პროვინცია"), ("kn", "ರಾವ\u{cc6}ನ\u{ccd}ನಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "라벤나 현"), ("lt", "Ravenos provincija"), ("lv", "Ravennas province"), ("mk", "Равена"), ("mr", "र\u{947}व\u{947}ना प\u{94d}रा\u{902}त"), ("ms", "Wilayah Ravenna"), ("nb", "Provinsen Ravenna"), ("nl", "Ravenna"), ("no", "Provinsen Ravenna"), ("pl", "Prowincja Rawenna"), ("pt", "Ravena"), ("ro", "Provincia Ravenna"), ("ru", "Равенна"), ("si", "රවේන\u{dca}න\u{dcf} පළ\u{dcf}ත"), ("sl", "Ravenna"), ("sr", "Равена"), ("sr_Latn", "Ravena"), ("sv", "Ravenna"), ("ta", "ர\u{bbe}வென\u{bcd}ன\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ర\u{c3e}వ\u{c46}న\u{c4d}న\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดราเวนนา"), ("tr", "Ravenna ili"), ("uk", "Провінція Равенна"), ("ur", "صوبہ راوینا"), ("uz", "Ravenna"), ("vi", "Ravenna"), ("zh", "拉韋納省")]),
                        unofficial_name_list: ["Province of Ravenna"].to_vec(),
                    }
                ),
                (
                    "RC",
                    Subdivision{
                        name: "RC",
                        country_alpha2: Alpha2::IT,
                        code: "RC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.1113006), longitude: Some(15.6472914), max_latitude: Some(38.204364), min_latitude: Some(37.9914216), max_longitude: Some(15.7133153), min_longitude: Some(15.6302247)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ريدجو كالابريا"), ("be", "Рэджа-Калабрыя"), ("bg", "Реджо Калабрия"), ("bn", "রেজ\u{9cd}জিও ক\u{9cd}য\u{9be}ল\u{9be}ব\u{9cd}রিয\u{9bc}\u{9be}-এর প\u{9cd}রদেশ"), ("ca", "Província de Reggio de Calàbria"), ("ccp", "𑄢𑄬𑄉\u{11128}𑄃\u{1112e} 𑄇\u{11133}𑄠𑄣\u{11134}𑄝\u{11133}𑄢\u{11128}𑄠"), ("ceb", "Reggio Calabria"), ("cs", "Provincie Reggio Calabria"), ("da", "Province of Reggio Calabria"), ("de", "Provinz Reggio Calabria"), ("el", "Ρέτζο-Καλάμπρια"), ("en", "Reggio Calabria"), ("es", "Reggio Calabria"), ("et", "Reggio Calabria provints"), ("eu", "Reggio di Calabriako probintzia"), ("fa", "استان رجیو کالابریا"), ("fi", "Reggio Calabrian maakunta"), ("fr", "Province de Reggio de Calabre"), ("gl", "Provincia de Reggio Calabria"), ("gu", "ર\u{ac7}જિયો ક\u{ac7}લ\u{ac7}બ\u{acd}રિયા પ\u{acd}રા\u{a82}ત"), ("he", "רג׳ו די קלבריה"), ("hi", "र\u{947}जियो क\u{948}लाब\u{94d}रिया प\u{94d}रा\u{902}त"), ("hu", "Reggio Calabria megye"), ("hy", "Ռեջիո Կալաբրիա"), ("id", "Provinsi Reggio Calabria"), ("it", "provincia di Reggio Calabria"), ("ja", "レッジョ・カラブリア県"), ("jv", "Provinsi Reggio Calabria"), ("ka", "რეჯო-კალაბრიის პროვინცია"), ("kn", "ರ\u{cc6}ಗ\u{ccd}ಗ\u{cbf}ಯೋ ಕ\u{ccd}ಯಾಲಬ\u{ccd}ರ\u{cbf}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "레조칼라브리아 현"), ("lt", "Kalabrijos Redžo provincija"), ("lv", "Redžo di Kalabrijas province"), ("mr", "र\u{947}जिओ क\u{945}लब\u{94d}रिया प\u{94d}रा\u{902}त"), ("ms", "Wilayah Reggio Calabria"), ("nb", "Reggio Calabria"), ("nl", "Reggio Calabria"), ("no", "Provinsen Reggio Calabria"), ("pl", "Prowincja Reggio di Calabria"), ("pt", "Reggio Calabria"), ("ro", "Provincia Reggio Calabria"), ("ru", "Реджо-Калабрия"), ("si", "රෙජ\u{dd2}යෝ පළ\u{dcf}ත ,කලබ\u{dca}\u{200d}ර\u{dd2}ය\u{dcf}"), ("sl", "Reggio Calabria"), ("sq", "Provinca e Rexhio Kalabrisë"), ("sr", "Ређо ди Калабрија"), ("sr_Latn", "Ređo di Kalabrija"), ("sv", "Reggio Calabria"), ("ta", "ரெஜிஓ கல\u{bbe}ப\u{bcd}ரிய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ర\u{c46}గ\u{c3f}య\u{c4b} కల\u{c3e}బ\u{c4d}ర\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเรจจ\u{e34}โอ คาราเบร\u{e35}ย"), ("tr", "Reggio Calabria ili"), ("uk", "Провінція Реджо-Калабрія"), ("ur", "صوبہ رجیو کلابریا"), ("uz", "Reggio Calabria"), ("vi", "Reggio Calabria"), ("yue", "雷焦卡拉布里亞省"), ("yue_Hans", "雷焦卡拉布里亚省"), ("zh", "雷焦卡拉布里亞省")]),
                        unofficial_name_list: ["Metropolitan City of Reggio Calabria"].to_vec(),
                    }
                ),
                (
                    "RE",
                    Subdivision{
                        name: "RE",
                        country_alpha2: Alpha2::IT,
                        code: "RE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.6989932), longitude: Some(10.6296859), max_latitude: Some(44.7482256), min_latitude: Some(44.6550334), max_longitude: Some(10.7245189), min_longitude: Some(10.5520175)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ريدجو إميليا"), ("be", "Правінцыя Рэджа-Эмілія"), ("bg", "Реджо Емилия"), ("bn", "রেজিও এমিলিয\u{9bc}\u{9be} এর প\u{9cd}রদেশ"), ("ca", "Província de Reggio de l’Emília"), ("ccp", "𑄢𑄬𑄉\u{11128}𑄃\u{1112e} 𑄃\u{11128}𑄟\u{11128}𑄣\u{1112d}"), ("ceb", "Reggio Emilia"), ("cs", "Provincie Reggio Emilia"), ("da", "Reggio Emilia"), ("de", "Provinz Reggio Emilia"), ("el", "Ρέτζο-Εμίλια"), ("en", "Reggio Emilia"), ("es", "Reggio Emilia"), ("et", "Reggio Emilia provints"), ("eu", "Reggio nell’Emiliako probintzia"), ("fa", "استان رجو امیلیا"), ("fi", "Reggio Emilian maakunta"), ("fr", "Province de Reggio d’Émilie"), ("gl", "Provincia de Reggio Emilia"), ("gu", "ર\u{ac7}જિયો એમિલિઆ પ\u{acd}રા\u{a82}ત"), ("hi", "र\u{947}जिओ एमिलिया प\u{94d}रा\u{902}त"), ("hu", "Reggio Emilia megye"), ("hy", "Ռեջիո Էմիլիա"), ("id", "Provinsi Reggio Emilia"), ("it", "provincia di Reggio nell’Emilia"), ("ja", "レッジョ・エミリア県"), ("jv", "Provinsi Reggio Emilia"), ("ka", "რეჯო-ემილიის პროვინცია"), ("kn", "ರ\u{cc6}ಗ\u{ccd}ಗ\u{cbf}ಯೋ ಎಮ\u{cbf}ಲ\u{cbf}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "레조에밀리아 현"), ("lt", "Emilijos Redžo provincija"), ("lv", "Redžo nell’Emīlijas province"), ("mr", "र\u{947}जियो एमिलिया प\u{94d}रा\u{902}त"), ("ms", "Wilayah Reggio Emilia"), ("nb", "Provinsen Reggio Emilia"), ("nl", "Reggio Emilia"), ("no", "Provinsen Reggio Emilia"), ("pl", "Prowincja Reggio Emilia"), ("pt", "Reggio Emilia"), ("ro", "Provincia Reggio Emilia"), ("ru", "Реджо-Эмилия"), ("si", "රේග\u{dd2}යෝ එම\u{dd2}ල\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sl", "Reggio Emilia"), ("sr", "Ређо Емилија"), ("sr_Latn", "Ređo Emilija"), ("sv", "Reggio Emilia"), ("ta", "ரெக\u{bcd}கியோ எமிலிஆ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ర\u{c46}గ\u{c3f}య\u{c4b} ఎమ\u{c40}ల\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเรจจ\u{e34}โอ เอม\u{e34}เล\u{e35}ย"), ("tr", "Reggio Emilia ili"), ("uk", "Провінція Реджо-Емілія"), ("ur", "صوبہ رجیو امیلیا"), ("uz", "Reggio Emilia"), ("vi", "Reggio Emilia"), ("zh", "雷焦艾米利亞省")]),
                        unofficial_name_list: ["Province of Reggio Emilia", "Reggio Emilia"].to_vec(),
                    }
                ),
                (
                    "RG",
                    Subdivision{
                        name: "RG",
                        country_alpha2: Alpha2::IT,
                        code: "RG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.9269273), longitude: Some(14.7255129), max_latitude: Some(36.9387), min_latitude: Some(36.8986672), max_longitude: Some(14.7500679), min_longitude: Some(14.6705082)}),
                        comments: None,
                        subdivision_type: SubdivisionType::FreeMunicipalConsortium,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة راغوزا"), ("az", "Raquza"), ("be", "правінцыя Рагуза"), ("bg", "Рагуза"), ("bn", "র\u{9be}গ\u{9c1}স\u{9be}-এর প\u{9cd}রদেশ"), ("ca", "Província de Ragusa"), ("ccp", "𑄛𑄉\u{1112a}𑄥"), ("ceb", "Ragusa"), ("cs", "Provincie Ragusa"), ("da", "Province of Ragusa"), ("de", "Provinz Ragusa"), ("el", "Ραγκούζα"), ("en", "Ragusa"), ("es", "Ragusa"), ("et", "Ragusa provints"), ("eu", "Ragusako probintzia"), ("fa", "استان راگوسا"), ("fi", "Ragusan maakunta"), ("fr", "Province de Raguse"), ("gl", "Provincia de Ragusa"), ("gu", "ર\u{ac7}ગ\u{ac1}સા પ\u{acd}રા\u{a82}ત"), ("he", "רגוזה"), ("hi", "राग\u{941}सा क\u{947} प\u{94d}रा\u{902}त"), ("hr", "Ragusa"), ("hu", "Ragusa megye"), ("hy", "Ռագուզա"), ("id", "Provinsi Ragusa"), ("it", "provincia di Ragusa"), ("ja", "ラグーザ県"), ("jv", "Provinsi Ragusa"), ("ka", "რაგუზის პროვინცია"), ("kn", "ರಗುಸಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "라구사 현"), ("lt", "Ragūzos provincija"), ("lv", "Raguzas province"), ("mk", "Рагуза"), ("mr", "राग\u{941}साच\u{947} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Ragusa"), ("nb", "Provinsen Ragusa"), ("nl", "Ragusa"), ("no", "Provinsen Ragusa"), ("pl", "Prowincja Ragusa"), ("pt", "Ragusa"), ("ro", "Provincia Ragusa"), ("ru", "Рагуза"), ("si", "රග\u{dd4}ස\u{dcf} පළ\u{dcf}ත"), ("sl", "Ragusa"), ("sr", "Рагуза"), ("sr_Latn", "Raguza"), ("sv", "Ragusa"), ("ta", "ரகுச\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "రగూస\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดราก\u{e39}ซา"), ("tr", "Ragusa ili"), ("uk", "Провінція Рагуза"), ("ur", "صوبہ راگوزا"), ("uz", "Ragusa"), ("vi", "Ragusa"), ("zh", "拉古薩省")]),
                        unofficial_name_list: ["Province of Ragusa"].to_vec(),
                    }
                ),
                (
                    "RI",
                    Subdivision{
                        name: "RI",
                        country_alpha2: Alpha2::IT,
                        code: "RI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.404509), longitude: Some(12.8567281), max_latitude: Some(42.4392607), min_latitude: Some(42.3928519), max_longitude: Some(12.9040254), min_longitude: Some(12.8270678)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة رييتي"), ("be", "правінцыя Рыеці"), ("bg", "Риети"), ("bn", "রিতির প\u{9cd}রদেশ"), ("ca", "Província de Rieti"), ("ccp", "𑄢\u{1112d}𑄑\u{11128}"), ("ceb", "Rieti"), ("cs", "Provincie Rieti"), ("da", "Province of Rieti"), ("de", "Provinz Rieti"), ("el", "Ριέτι"), ("en", "Rieti"), ("es", "Rieti"), ("et", "Rieti provints"), ("eu", "Rietiko probintzia"), ("fa", "استان ریتی"), ("fi", "Rietin maakunta"), ("fr", "Province de Rieti"), ("gl", "Provincia de Rieti"), ("gu", "રિએટી પ\u{acd}રા\u{a82}ત"), ("he", "ריאטי"), ("hi", "रीती प\u{94d}रा\u{902}त"), ("hr", "Rieti"), ("hu", "Rieti megye"), ("hy", "Ռիետի"), ("id", "Provinsi Rieti"), ("it", "provincia di Rieti"), ("ja", "リエーティ県"), ("jv", "Provinsi Rieti"), ("ka", "რიეტის პროვინცია"), ("kn", "ರೈಟ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "리에티 현"), ("lt", "Riečio provincija"), ("lv", "Rieti province"), ("mr", "रीति प\u{94d}रा\u{902}त"), ("ms", "Wilayah Rieti"), ("nb", "Provinsen Rieti"), ("nl", "Rieti"), ("no", "Provinsen Rieti"), ("pl", "Prowincja Rieti"), ("pt", "Rieti"), ("ro", "Provincia Rieti"), ("ru", "Риети"), ("si", "ර\u{dd3}ට\u{dd2} පළ\u{dcf}ත"), ("sl", "Rieti"), ("sr", "Ријети"), ("sr_Latn", "Rijeti"), ("sv", "Rieti"), ("ta", "ரியேடி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ర\u{c40}ట\u{c40} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เม\u{e37}องร\u{e34}เอต\u{e34}"), ("tr", "Rieti ili"), ("uk", "Провінція Рієті"), ("ur", "صوبہ ریئتی"), ("uz", "Rieti"), ("vi", "Rieti"), ("zh", "列蒂省")]),
                        unofficial_name_list: ["Province of Rieti"].to_vec(),
                    }
                ),
                (
                    "RM",
                    Subdivision{
                        name: "RM",
                        country_alpha2: Alpha2::IT,
                        code: "RM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.9027835), longitude: Some(12.4963655), max_latitude: Some(42.0505462), min_latitude: Some(41.769596), max_longitude: Some(12.7302888), min_longitude: Some(12.341707)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة روما"), ("az", "Roma"), ("be", "правінцыя Рым"), ("bg", "Рим"), ("ca", "Província de Roma"), ("ccp", "𑄢\u{1112e}𑄟\u{11134}"), ("ceb", "Roma"), ("cs", "Provincie Roma"), ("da", "Provinsen Rom"), ("de", "Provinz Rom"), ("el", "Επαρχία της Ρώμης"), ("en", "Rome"), ("es", "Roma"), ("et", "Rooma provints"), ("eu", "Erromako probintzia"), ("fa", "استان رم"), ("fi", "Rooman maakunta"), ("fr", "Province de Rome"), ("gl", "Provincia de Roma"), ("he", "רומא"), ("hi", "रोम प\u{94d}रा\u{902}त"), ("hr", "Rim"), ("hu", "Róma megye"), ("hy", "Հռոմ"), ("id", "Provinsi Roma"), ("it", "provincia di Roma"), ("ja", "ローマ県"), ("jv", "Provinsi Roma"), ("ka", "რომის პროვინცია"), ("ko", "로마 현"), ("lt", "Romos provincija"), ("lv", "Romas province"), ("mk", "Рим"), ("mn", "Ром аймаг"), ("ms", "Wilayah Rom"), ("nb", "Roma"), ("nl", "Rome"), ("no", "Provinsen Roma"), ("pl", "Prowincja Rzym"), ("pt", "Roma"), ("ro", "Provincia Roma"), ("ru", "Рим"), ("sk", "provincia Rím"), ("sl", "Roma"), ("sq", "Provinca e Romës"), ("sr", "Рим"), ("sr_Latn", "Rim"), ("sv", "Rom"), ("tr", "Roma ili"), ("uk", "Провінція Рим"), ("ur", "صوبہ روما"), ("uz", "Rim"), ("vi", "Roma"), ("yue", "羅馬省"), ("yue_Hans", "罗马省"), ("zh", "羅馬省")]),
                        unofficial_name_list: ["Metropolitan City of Rome Capital"].to_vec(),
                    }
                ),
                (
                    "RN",
                    Subdivision{
                        name: "RN",
                        country_alpha2: Alpha2::IT,
                        code: "RN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.0678288), longitude: Some(12.5695158), max_latitude: Some(44.088576), min_latitude: Some(44.0183507), max_longitude: Some(12.6322536), min_longitude: Some(12.5171204)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ريميني"), ("be", "Правінцыя Рыміні"), ("bg", "Римини"), ("bn", "রিমিনির প\u{9cd}রদেশ"), ("ca", "Província de Rimini"), ("ccp", "𑄢\u{11128}𑄟\u{11128}𑄚\u{11128}"), ("ceb", "Rimini"), ("cs", "Provincie Rimini"), ("da", "Province of Rimini"), ("de", "Provinz Rimini"), ("el", "Ρίμινι"), ("en", "Rimini"), ("es", "Rímini"), ("et", "Rimini provints"), ("eu", "Riminiko probintzia"), ("fa", "استان ریمینی"), ("fi", "Riminin maakunta"), ("fr", "Province de Rimini"), ("gl", "Provincia de Rímini"), ("gu", "રિમિની પ\u{acd}રા\u{a82}ત"), ("hi", "रिमिनी प\u{94d}रा\u{902}त"), ("hu", "Rimini megye"), ("hy", "Ռիմինի"), ("id", "Provinsi Rimini"), ("it", "provincia di Rimini"), ("ja", "リミニ県"), ("jv", "Provinsi Rimini"), ("ka", "რიმინის პროვინცია"), ("kn", "ರ\u{cbf}ಮ\u{cbf}ನ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "리미니 현"), ("lt", "Riminio provincija"), ("lv", "Rimini province"), ("mr", "रिमिनी प\u{94d}रा\u{902}त"), ("ms", "Wilayah Rimini"), ("nb", "Provinsen Rimini"), ("nl", "Rimini"), ("no", "Provinsen Rimini"), ("pl", "Prowincja Rimini"), ("pt", "Rimini"), ("ro", "Provincia Rimini"), ("ru", "Римини"), ("si", "ර\u{dd2}ම\u{dd2}න\u{dd2} පළ\u{dcf}ත"), ("sl", "Rimini"), ("sq", "Provinca e Riminit"), ("sr", "Римини"), ("sr_Latn", "Rimini"), ("sv", "Rimini"), ("ta", "ரிமினி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ర\u{c3f}మ\u{c3f}న\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดร\u{e34}ม\u{e34}น\u{e35}"), ("tr", "Rimini ili"), ("uk", "Провінція Ріміні"), ("ur", "صوبہ ریمینی"), ("uz", "Rimini"), ("vi", "Rimini"), ("zh", "里米尼省")]),
                        unofficial_name_list: ["Province of Rimini"].to_vec(),
                    }
                ),
                (
                    "RO",
                    Subdivision{
                        name: "RO",
                        country_alpha2: Alpha2::IT,
                        code: "RO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.0698118), longitude: Some(11.7902158), max_latitude: Some(45.1055056), min_latitude: Some(45.0334583), max_longitude: Some(11.8383162), min_longitude: Some(11.7415282)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة روفيغو"), ("be", "правінцыя Равіга"), ("bg", "Ровиго"), ("bn", "রোভিগ\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Rovigo"), ("ccp", "𑄢\u{1112e}𑄞\u{11128}𑄉\u{1112e}"), ("ceb", "Provincia di Rovigo"), ("cs", "Provincie Rovigo"), ("da", "Rovigo"), ("de", "Provinz Rovigo"), ("el", "Επαρχία του Ροβίγκο"), ("en", "Rovigo"), ("es", "Rovigo"), ("et", "Rovigo provints"), ("eu", "Rovigoko probintzia"), ("fa", "استان روویگو"), ("fi", "Rovigon maakunta"), ("fr", "province de Rovigo"), ("gl", "Provincia de Rovigo"), ("gu", "રોવિગો પ\u{acd}રા\u{a82}ત"), ("he", "רוביגו"), ("hi", "रोविगो प\u{94d}रा\u{902}त"), ("hr", "Rovigo (pokrajina)"), ("hu", "Rovigo megye"), ("hy", "Ռովիգո"), ("id", "Provinsi Rovigo"), ("it", "provincia di Rovigo"), ("ja", "ロヴィーゴ県"), ("jv", "Provinsi Rovigo"), ("ka", "როვიგოს პროვინცია"), ("kn", "ರೋವ\u{cbf}ಗೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "로비고 현"), ("lt", "Rovigo provincija"), ("lv", "Rovigo province"), ("mk", "Ровиго"), ("mr", "रोविगो प\u{94d}रा\u{902}त"), ("ms", "Wilayah Rovigo"), ("nb", "Provinsen Rovigo"), ("nl", "Rovigo"), ("no", "Provinsen Rovigo"), ("pl", "Prowincja Rovigo"), ("pt", "Rovigo"), ("ro", "Provincia Rovigo"), ("ru", "Ровиго"), ("si", "රොව\u{dd2}ගෝ පළ\u{dcf}ත"), ("sl", "Rovigo"), ("sq", "Provinca e Rovigos"), ("sr", "Ровиго"), ("sr_Latn", "Rovigo"), ("sv", "Rovigo"), ("ta", "ரோவிகோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ర\u{c4b}వ\u{c3f}గ\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโรว\u{e34}โก"), ("tr", "Rovigo ili"), ("uk", "Провінція Ровіго"), ("ur", "صوبہ روویگو"), ("uz", "Rovigo"), ("vi", "Rovigo"), ("zh", "羅維戈省")]),
                        unofficial_name_list: ["Province of Rovigo"].to_vec(),
                    }
                ),
                (
                    "SA",
                    Subdivision{
                        name: "SA",
                        country_alpha2: Alpha2::IT,
                        code: "SA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.68244079999999), longitude: Some(14.7680961), max_latitude: Some(40.7121639), min_latitude: Some(40.6373235), max_longitude: Some(14.8513141), min_longitude: Some(14.7339981)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ساليرنو"), ("be", "правінцыя Салерна"), ("bg", "Салерно"), ("bn", "স\u{9cd}য\u{9be}লের\u{9cd}নো-এর প\u{9cd}রদেশ"), ("ca", "Província de Salern"), ("ccp", "𑄥𑄣𑄬𑄢\u{11134}𑄚\u{1112e}"), ("ceb", "Salerno"), ("cs", "Provincie Salerno"), ("da", "Salerno"), ("de", "Provinz Salerno"), ("el", "Σαλέρνο"), ("en", "Salerno"), ("es", "Salerno"), ("et", "Salerno provints"), ("eu", "Salernoko probintzia"), ("fa", "استان سالرنو"), ("fi", "Salernon maakunta"), ("fr", "Province de Salerne"), ("gl", "Provincia de Salerno"), ("gu", "સલ\u{ac7}ર\u{acd}નો પ\u{acd}રા\u{a82}ત"), ("he", "סלרנו"), ("hi", "साल\u{947}र\u{94d}नो प\u{94d}रा\u{902}त"), ("hu", "Salerno megye"), ("hy", "Սալեռնո"), ("id", "Provinsi Salerno"), ("it", "provincia di Salerno"), ("ja", "サレルノ県"), ("jv", "Provinsi Salerno"), ("ka", "სალერნოს პროვინცია"), ("kn", "ಸಲ\u{cc6}ರ\u{ccd}ನೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "살레르노 현"), ("lt", "Salerno provincija"), ("lv", "Salerno province"), ("mk", "Салерно"), ("mr", "साल\u{947}र\u{94d}नो प\u{94d}रा\u{902}त"), ("ms", "Wilayah Salerno"), ("nb", "Provinsen Salerno"), ("nl", "Salerno"), ("no", "Provinsen Salerno"), ("pl", "Prowincja Salerno"), ("pt", "Salerno"), ("ro", "Provincia Salerno"), ("ru", "Салерно"), ("si", "සලේර\u{dca}නෝ පළ\u{dcf}ත"), ("sl", "Salerno"), ("sq", "Provinca e Salernos"), ("sr", "Салерно"), ("sr_Latn", "Salerno"), ("sv", "Salerno"), ("ta", "ச\u{bbe}லிர\u{bcd}னோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3e}ల\u{c46}మ\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซาเลร\u{e4c}โน"), ("tr", "Salerno ili"), ("uk", "Провінція Салерно"), ("ur", "صوبہ سالیرنو"), ("uz", "Salerno"), ("vi", "Salerno"), ("zh", "薩萊諾省")]),
                        unofficial_name_list: ["Province of Salerno"].to_vec(),
                    }
                ),
                (
                    "SI",
                    Subdivision{
                        name: "SI",
                        country_alpha2: Alpha2::IT,
                        code: "SI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.31880899999999), longitude: Some(11.3307574), max_latitude: Some(43.3515692), min_latitude: Some(43.2804476), max_longitude: Some(11.3635397), min_longitude: Some(11.2917587)}),
                        comments: None,
                        subdivision_type: SubdivisionType::FreeMunicipalConsortium,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سيينا"), ("be", "правінцыя Сіена"), ("bg", "Сиена"), ("bn", "সিয\u{9bc}েন\u{9be}-এর প\u{9cd}রদেশ"), ("ca", "Província de Siena"), ("ccp", "𑄥\u{11128}𑄠𑄬𑄚"), ("ceb", "Provincia di Siena"), ("cs", "Provincie Siena"), ("da", "Province of Siena"), ("de", "Provinz Siena"), ("el", "Σιένα"), ("en", "Siena"), ("es", "Siena"), ("et", "Siena provints"), ("eu", "Sienako probintzia"), ("fa", "استان سیه\u{200c}نا"), ("fi", "Sienan maakunta"), ("fr", "province de Sienne"), ("gl", "Provincia de Siena"), ("gu", "સિએના પ\u{acd}રા\u{a82}ત"), ("he", "סיינה"), ("hi", "सिएना प\u{94d}रा\u{902}त"), ("hr", "Provincija Siena"), ("hu", "Siena megye"), ("hy", "Սիենա"), ("id", "Provinsi Siena"), ("it", "provincia di Siena"), ("ja", "シエーナ県"), ("jv", "Provinsi Siena"), ("ka", "სიენის პროვინცია"), ("kn", "ಸ\u{cbf}ಯ\u{cc6}ನಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "시에나 현"), ("lt", "Sienos provincija"), ("lv", "Sjēnas province"), ("mk", "Сиена"), ("mr", "सिएना प\u{94d}रा\u{902}त"), ("ms", "Wilayah Siena"), ("nb", "Siena"), ("nl", "Siena"), ("no", "Siena"), ("pl", "Prowincja Siena"), ("pt", "Siena"), ("ro", "Provincia Siena"), ("ru", "Сиена"), ("si", "ස\u{dd2}යෙන\u{dcf} පළ\u{dcf}ත"), ("sk", "Siena"), ("sl", "Siena"), ("sr", "Сијена"), ("sr_Latn", "Sijena"), ("sv", "Siena"), ("ta", "சியன\u{bcd}ன\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3f}య\u{c47}న\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e35}นา"), ("tr", "Siena ili"), ("uk", "Провінція Сієна"), ("ur", "صوبہ سئینا"), ("uz", "Siena"), ("vi", "Siena"), ("zh", "錫耶納省")]),
                        unofficial_name_list: ["Province of Siena"].to_vec(),
                    }
                ),
                (
                    "SO",
                    Subdivision{
                        name: "SO",
                        country_alpha2: Alpha2::IT,
                        code: "SO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1698583), longitude: Some(9.8787674), max_latitude: Some(46.1773996), min_latitude: Some(46.1588347), max_longitude: Some(9.898327), min_longitude: Some(9.844438799999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سوندريو"), ("be", "правінцыя Сондрыа"), ("bg", "Сондрио"), ("bn", "সন\u{9cd}দ\u{9cd}রিও প\u{9cd}রদেশ"), ("ca", "Província de Sondrio"), ("ccp", "𑄥\u{11127}𑄚\u{11134}𑄓\u{11133}𑄢\u{11128}𑄃\u{1112e}"), ("ceb", "Provincia di Sondrio"), ("cs", "Provincie Sondrio"), ("cy", "Rhanbarth Sondrio"), ("da", "Sondrio"), ("de", "Provinz Sondrio"), ("el", "Επαρχία του Σόντριο"), ("en", "Sondrio"), ("es", "Sondrio"), ("et", "Sondrio provints"), ("eu", "Sondrioko probintzia"), ("fa", "استان سوندریو"), ("fi", "Sondrion maakunta"), ("fr", "province de Sondrio"), ("gl", "Provincia de Sondrio"), ("gu", "સોન\u{acd}ડ\u{acd}રીયો પ\u{acd}રા\u{a82}ત"), ("he", "סונדריו"), ("hi", "सो\u{902}ड\u{94d}रीओ प\u{94d}रा\u{902}त"), ("hu", "Sondrio megye"), ("hy", "Սոնդրիո"), ("id", "Provinsi Sondrio"), ("it", "provincia di Sondrio"), ("ja", "ソンドリオ県"), ("jv", "Provinsi Sondrio"), ("ka", "სონდრიოს პროვინცია"), ("kn", "ಸೊಂಡ\u{ccd}ರ\u{cbf}ಯೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "손드리오 현"), ("lt", "Sondrijo provincija"), ("lv", "Sondrio province"), ("mr", "सो\u{902}ड\u{94d}रिओ प\u{94d}रा\u{902}त"), ("ms", "Wilayah Sondrio"), ("nb", "Provinsen Sondrio"), ("nl", "Sondrio"), ("no", "Provinsen Sondrio"), ("pl", "Prowincja Sondrio"), ("pt", "Sondrio"), ("ro", "Provincia Sondrio"), ("ru", "Сондрио"), ("si", "සොන\u{dca}ඩ\u{dca}ර\u{dd2}යෝ පළ\u{dcf}ත"), ("sk", "Sondrio"), ("sl", "Sondrio"), ("sq", "Provinca e Sondrios"), ("sr", "Сондрио"), ("sr_Latn", "Sondrio"), ("sv", "Sondrio"), ("ta", "சென\u{bcd}றியோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3e}ండ\u{c4d}ర\u{c3f}య\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซานดร\u{e34}โอ"), ("tr", "Sondrio ili"), ("uk", "Провінція Сондріо"), ("ur", "صوبہ سوندریو"), ("uz", "Sondrio"), ("vi", "Sondrio"), ("zh", "松德里奧省")]),
                        unofficial_name_list: ["Province of Sondrio"].to_vec(),
                    }
                ),
                (
                    "SP",
                    Subdivision{
                        name: "SP",
                        country_alpha2: Alpha2::IT,
                        code: "SP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.1024504), longitude: Some(9.824082599999999), max_latitude: Some(44.1367074), min_latitude: Some(44.0777423), max_longitude: Some(9.8885803), min_longitude: Some(9.7850968)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة لا سبيتسيا"), ("az", "Speziya"), ("be", "Спецыя"), ("bg", "Специя"), ("bn", "ল\u{9be} স\u{9cd}পেজিয\u{9bc}\u{9be}-র প\u{9cd}রদেশ"), ("ca", "Província de La Spezia"), ("ccp", "𑄣 𑄥\u{11133}𑄛𑄬𑄎\u{11128}𑄠"), ("ceb", "Provincia di La Spezia"), ("cs", "Provincie La Spezia"), ("da", "La Spezia"), ("de", "Provinz La Spezia"), ("el", "Σπέτσια"), ("en", "La Spezia"), ("es", "La Spezia"), ("et", "La Spezia provints"), ("eu", "La Speziako probintzia"), ("fa", "استان لا اسپتزیا"), ("fi", "La Spezian maakunta"), ("fr", "Province de La Spezia"), ("gl", "Provincia de La Spezia"), ("gu", "લા સ\u{acd}પ\u{ac7}ઝિયા પ\u{acd}રા\u{a82}ત"), ("he", "לה ספציה"), ("hi", "ला स\u{94d}प\u{947}ज\u{93c}िया प\u{94d}रा\u{902}त"), ("hu", "La Spezia megye"), ("hy", "Սպեցիա"), ("id", "Provinsi La Spezia"), ("it", "provincia della Spezia"), ("ja", "ラ・スペツィア県"), ("jv", "Provinsi La Spezia"), ("ka", "სპეციის პროვინცია"), ("kn", "ಲಾ ಸ\u{ccd}ಪೀಜ\u{cbf}ಯ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "라스페치아 현"), ("lt", "La Specijos provincija"), ("lv", "Spēcijas province"), ("mk", "Ла Специја"), ("mr", "ला स\u{94d}प\u{947}झिया प\u{94d}रा\u{902}त"), ("ms", "Wilayah La Spezia"), ("nb", "Provinsen La Spezia"), ("nl", "La Spezia"), ("no", "Provinsen La Spezia"), ("pl", "Prowincja La Spezia"), ("pt", "Spezia"), ("ro", "Provincia La Spezia"), ("ru", "Специя"), ("si", "ල\u{dcf} ස\u{dca}පෙස\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sl", "La Spezia"), ("sr", "Ла Специја"), ("sr_Latn", "La Specija"), ("sv", "La Spezia"), ("ta", "ல\u{bbe} ஸ\u{bcd}பெசிய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c3e} స\u{c4d}ప\u{c47}జ\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดลาสเปเซ\u{e35}ย"), ("tr", "La Spezia ili"), ("uk", "Провінція Ла Спеція"), ("ur", "صوبہ لا سپیتسیا"), ("vi", "La Spezia"), ("zh", "拉斯佩齊亞省")]),
                        unofficial_name_list: ["Province of La Spezia"].to_vec(),
                    }
                ),
                (
                    "SR",
                    Subdivision{
                        name: "SR",
                        country_alpha2: Alpha2::IT,
                        code: "SR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.0754739), longitude: Some(15.2865861), max_latitude: Some(37.1056629), min_latitude: Some(37.0526518), max_longitude: Some(15.3012622), min_longitude: Some(15.2405306)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سرقوسة"), ("az", "Sirakuza"), ("be", "Сіракуза"), ("bg", "Сиракуза"), ("bn", "স\u{9be}য\u{9bc}র\u{9be}ক\u{9c1}স-র প\u{9cd}রদেশ"), ("bs", "Sirakuza"), ("ca", "Província de Siracusa"), ("ccp", "𑄥\u{1112d}𑄢𑄇\u{11128}𑄅\u{1112a}𑄌\u{11134}"), ("ceb", "Provincia di Siracusa"), ("cs", "Provincie Siracusa"), ("da", "provinsen Siracusa"), ("de", "Provinz Syrakus"), ("el", "Συρακούσες"), ("en", "Syracuse"), ("es", "Siracusa"), ("et", "Siracusa provints"), ("eu", "Sirakusako probintzia"), ("fa", "استان سیراکوز"), ("fi", "Syrakusan maakunta"), ("fr", "province de Syracuse"), ("gl", "Provincia de Siracusa"), ("gu", "સિર\u{ac7}ક\u{acd}ય\u{ac1}જ પ\u{acd}રા\u{a82}ત"), ("he", "סירקוזה"), ("hi", "सायराक\u{94d}र\u{942}स प\u{94d}रा\u{902}त"), ("hr", "Sirakuza"), ("hu", "Syracuse megye"), ("hy", "Սիրակուզա"), ("id", "Provinsi Sirakusa"), ("it", "provincia di Siracusa"), ("ja", "シラクーザ県"), ("jv", "Provinsi Siracusa"), ("ka", "სირაკუზის პროვინცია"), ("kn", "ಸ\u{cbf}ರಾಕ\u{ccd}ಯ\u{cc2}ಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "시라쿠사 현"), ("lt", "Sirakūzų provincija"), ("lv", "Sirakūzu province"), ("mk", "Сиракуза"), ("mr", "सिराक\u{94d}य\u{942}ज प\u{94d}रा\u{902}त"), ("ms", "Wilayah Syracuse"), ("nb", "Provinsen Siracusa"), ("nl", "Syracuse"), ("no", "Provinsen Siracusa"), ("pl", "Prowincja Syrakuzy"), ("pt", "Siracusa"), ("ro", "Provincia Siracuza"), ("ru", "Сиракуза"), ("si", "සය\u{dd2}රක\u{dd4}ස\u{dca} පළ\u{dcf}ත"), ("sl", "Siracusa"), ("sr", "Сиракуза"), ("sr_Latn", "Sirakuza"), ("sv", "Syrakusa"), ("ta", "சிரகூஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d} ఆఫ\u{c4d} స\u{c48}ర\u{c3e}క\u{c4d}యూస\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e35}ราค\u{e34}วส\u{e4c}"), ("tr", "Siraküza ili"), ("uk", "Провінція Сіракуза"), ("ur", "صوبہ سرقوسہ"), ("uz", "Siracusa"), ("vi", "Siracusa"), ("zh", "錫拉庫薩省")]),
                        unofficial_name_list: ["Province of Syracuse"].to_vec(),
                    }
                ),
                (
                    "SS",
                    Subdivision{
                        name: "SS",
                        country_alpha2: Alpha2::IT,
                        code: "SS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.7259269), longitude: Some(8.5556826), max_latitude: Some(40.7537418), min_latitude: Some(40.6985242), max_longitude: Some(8.597026099999999), min_longitude: Some(8.4952695)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ساساري"), ("be", "Правінцыя Сасары"), ("bg", "Сасари"), ("bn", "স\u{9be}স\u{9cd}য\u{9be}রি-র প\u{9cd}রদেশ"), ("ca", "Província de Sàsser"), ("ccp", "𑄥𑄥𑄢\u{11128}"), ("ceb", "Provincia di Sassari"), ("cs", "Provincie Sassari"), ("da", "Sassari"), ("de", "Provinz Sassari"), ("el", "Σάσσαρι"), ("en", "Sassari"), ("es", "Sassari"), ("et", "Sassari provints"), ("eu", "Sassariko probintzia"), ("fa", "استان ساساری"), ("fi", "Sassarin maakunta"), ("fr", "Province de Sassari"), ("gl", "Provincia de Sassari"), ("gu", "સાસારી પ\u{acd}રા\u{a82}ત"), ("he", "סאסארי"), ("hi", "ससारी प\u{94d}रा\u{902}त"), ("hu", "Sassari megye"), ("hy", "Սասսարի"), ("id", "Provinsi Sassari"), ("it", "provincia di Sassari"), ("ja", "サッサリ県"), ("jv", "Provinsi Sassari"), ("ka", "სასარის პროვინცია"), ("kn", "ಸಸ\u{ccd}ಸರ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "사사리 현"), ("lt", "Sasario provincija"), ("lv", "Sasāri province"), ("mr", "ससासरी प\u{94d}रा\u{902}त"), ("ms", "Wilayah Sassari"), ("nb", "Provinsen Sassari"), ("nl", "Sassari"), ("no", "Provinsen Sassari"), ("pl", "Prowincja Sassari"), ("pt", "Sassari"), ("ro", "Provincia Sassari"), ("ru", "Сассари"), ("si", "සස\u{dca}ස\u{dcf}ර\u{dd2} පළ\u{dcf}ත"), ("sl", "Sassari"), ("sr", "Сасари"), ("sr_Latn", "Sasari"), ("sv", "Sassari"), ("ta", "ச\u{bbe}ஸ\u{bcd}ஸ\u{bbe}ரி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "సస\u{c3e}ర\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e31}สซาร\u{e35}"), ("tr", "Sassari ili"), ("uk", "Провінція Сассарі"), ("ur", "صوبہ ساساری"), ("uz", "Sassari"), ("vi", "Sassari"), ("zh", "薩薩里省")]),
                        unofficial_name_list: ["Province of Sassari"].to_vec(),
                    }
                ),
                (
                    "SU",
                    Subdivision{
                        name: "SU",
                        country_alpha2: Alpha2::IT,
                        code: "SU",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Sud Sardegna")]),
                        unofficial_name_list: ["Province of Sud Sardegna"].to_vec(),
                    }
                ),
                (
                    "SV",
                    Subdivision{
                        name: "SV",
                        country_alpha2: Alpha2::IT,
                        code: "SV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.2975603), longitude: Some(8.4645), max_latitude: Some(44.32934059999999), min_latitude: Some(44.2811266), max_longitude: Some(8.5042904), min_longitude: Some(8.4347866)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سافونا"), ("be", "правінцыя Савона"), ("bg", "Савона"), ("bn", "স\u{9cd}য\u{9be}ভোন\u{9be}-র প\u{9cd}রদেশ"), ("ca", "Província de Savona"), ("ccp", "𑄥\u{11133}𑄠𑄣\u{1112e}𑄚"), ("ceb", "Provincia di Savona"), ("cs", "Provincie Savona"), ("da", "Province of Savona"), ("de", "Provinz Savona"), ("el", "Σαβόνα"), ("en", "Savona"), ("es", "Savona"), ("et", "Savona provints"), ("eu", "Savonako probintzia"), ("fa", "استان ساونا"), ("fi", "Savonan maakunta"), ("fr", "Province de Savone"), ("gl", "Provincia de Savona"), ("gu", "સવોના પ\u{acd}રા\u{a82}ત"), ("he", "סאבונה"), ("hi", "सवोना प\u{94d}रा\u{902}त"), ("hu", "Savona megye"), ("hy", "Սավոնա"), ("id", "Provinsi Savona"), ("it", "provincia di Savona"), ("ja", "サヴォーナ県"), ("jv", "Provinsi Savona"), ("ka", "სავონის პროვინცია"), ("kn", "ಸವೋನಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "사보나 현"), ("lt", "Savonos provincija"), ("lv", "Savonas province"), ("mk", "Савона"), ("mr", "सावोना प\u{94d}रा\u{902}त"), ("ms", "Wilayah Savona"), ("nb", "Provinsen Savona"), ("nl", "Savona"), ("no", "Provinsen Savona"), ("pl", "Prowincja Savona"), ("pt", "Savona"), ("ro", "Provincia Savona"), ("ru", "Савона"), ("si", "සැවොන\u{dcf} පළ\u{dcf}ත"), ("sl", "Savona"), ("sq", "Provinca e Savonës"), ("sr", "Савона"), ("sr_Latn", "Savona"), ("sv", "Savona"), ("ta", "ச\u{bbe}வொண\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "సవ\u{c4b}న\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซาโวนา"), ("tr", "Savona ili"), ("uk", "Провінція Савона"), ("ur", "صوبہ ساوونا"), ("uz", "Savona"), ("vi", "Savona"), ("zh", "薩沃納省")]),
                        unofficial_name_list: ["Province of Savona"].to_vec(),
                    }
                ),
                (
                    "TA",
                    Subdivision{
                        name: "TA",
                        country_alpha2: Alpha2::IT,
                        code: "TA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.46436060000001), longitude: Some(17.2470303), max_latitude: Some(40.4991197), min_latitude: Some(40.3791815), max_longitude: Some(17.2990258), min_longitude: Some(17.2017386)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تارانتو"), ("be", "правінцыя Таранта"), ("bg", "Таранто"), ("bn", "ট\u{9be}র\u{9be}ন\u{9cd}টো প\u{9cd}রদেশ"), ("ca", "Província de Tàrent"), ("ccp", "𑄑\u{11127}𑄢𑄚\u{11134}𑄑\u{1112e}"), ("ceb", "Taranto"), ("cs", "Provincie Taranto"), ("da", "Tarente"), ("de", "Provinz Tarent"), ("el", "Τάραντας"), ("en", "Taranto"), ("es", "Tarento"), ("et", "Taranto provints"), ("eu", "Tarentoko probintzia"), ("fa", "استان تارانتو"), ("fi", "Taranton maakunta"), ("fr", "province de Tarente"), ("gl", "Provincia de Taranto"), ("gu", "ટારાન\u{acd}ટો પ\u{acd}રા\u{a82}ત"), ("he", "טאראנטו"), ("hi", "टार\u{902}टो प\u{94d}रा\u{902}त"), ("hu", "Taranto megye"), ("hy", "Տարանտո"), ("id", "Provinsi Taranto"), ("it", "provincia di Taranto"), ("ja", "ターラント県"), ("jv", "Provinsi Taranto"), ("ka", "ტარანტოს პროვინცია"), ("kn", "ಟರಾಂಟೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "타란토 현"), ("lt", "Taranto provincija"), ("lv", "Taranto province"), ("mr", "टार\u{902}टो प\u{94d}रा\u{902}त"), ("ms", "Wilayah Taranto"), ("nb", "Provinsen Taranto"), ("nl", "Tarente"), ("no", "Provinsen Taranto"), ("pl", "Prowincja Tarent"), ("pt", "Taranto"), ("ro", "Provincia Taranto"), ("ru", "Таранто"), ("si", "ටරන\u{dca}ටෝ පළ\u{dcf}ත"), ("sl", "Taranto"), ("sq", "Provinca e Tarantos"), ("sr", "Таранто"), ("sr_Latn", "Taranto"), ("sv", "Taranto"), ("ta", "தரண\u{bcd}டோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "టర\u{c3e}ంట\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดทาร\u{e31}นโต"), ("tr", "Taranto ili"), ("uk", "Провінція Таранто"), ("ur", "صوبہ تارانتو"), ("uz", "Taranto"), ("vi", "Taranto"), ("zh", "塔蘭托省")]),
                        unofficial_name_list: ["Province of Taranto"].to_vec(),
                    }
                ),
                (
                    "TE",
                    Subdivision{
                        name: "TE",
                        country_alpha2: Alpha2::IT,
                        code: "TE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.6611431), longitude: Some(13.6986639), max_latitude: Some(42.6811718), min_latitude: Some(42.6428053), max_longitude: Some(13.7452854), min_longitude: Some(13.668671)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تيرامو"), ("be", "правінцыя Тэрама"), ("bg", "Терамо"), ("bn", "টের\u{9be}রনো-র প\u{9cd}রদেশ"), ("ca", "Província de Teramo"), ("ccp", "𑄑𑄬𑄢\u{11134}𑄟\u{1112e}"), ("ceb", "Teramo"), ("cs", "Provincie Teramo"), ("da", "Teramo"), ("de", "Provinz Teramo"), ("el", "Επαρχία του Τέραμο"), ("en", "Teramo"), ("es", "Teramo"), ("et", "Teramo provints"), ("eu", "Teramoko probintzia"), ("fa", "استان تیرامو"), ("fi", "Teramon maakunta"), ("fr", "province de Teramo"), ("gl", "Provincia de Teramo"), ("gu", "ટ\u{ac7}ર\u{ac7}મો પ\u{acd}રા\u{a82}ત"), ("he", "טראמו"), ("hi", "ट\u{947}रामो प\u{94d}रा\u{902}त"), ("hu", "Teramo megye"), ("hy", "Տերամո"), ("id", "Provinsi Teramo"), ("it", "provincia di Teramo"), ("ja", "テーラモ県"), ("jv", "Provinsi Teramo"), ("ka", "ტერამოს პროვინცია"), ("kn", "ಟ\u{cc6}ರಾಮೋ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "테라모 현"), ("lt", "Teramo provincija"), ("lv", "Terāmo province"), ("mr", "ट\u{947}र\u{947}मो प\u{94d}रा\u{902}त"), ("ms", "Wilayah Teramo"), ("nb", "Provinsen Teramo"), ("nl", "Teramo"), ("no", "Provinsen Teramo"), ("pl", "Prowincja Teramo"), ("pt", "Téramo"), ("ro", "Provincia Teramo"), ("ru", "Терамо"), ("si", "ටෙරමෝ පළ\u{dcf}ත"), ("sl", "Teramo"), ("sq", "Provinca e Teramos"), ("sr", "Терамо"), ("sr_Latn", "Teramo"), ("sv", "Teramo"), ("ta", "தெர\u{bcd}மோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ట\u{c46}ర\u{c3e}మ\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเทราโม"), ("tr", "Teramo ili"), ("uk", "Провінція Терамо"), ("ur", "صوبہ تیرامو"), ("uz", "Teramo"), ("vi", "Teramo"), ("zh", "泰拉莫省")]),
                        unofficial_name_list: ["Province of Teramo"].to_vec(),
                    }
                ),
                (
                    "TN",
                    Subdivision{
                        name: "TN",
                        country_alpha2: Alpha2::IT,
                        code: "TN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.0700915), longitude: Some(11.1197626), max_latitude: Some(46.1327915), min_latitude: Some(46.0281314), max_longitude: Some(11.1580189), min_longitude: Some(11.0826925)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousProvince,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Trentino"), ("ar", "مقاطعة ترينتو"), ("be", "правінцыя Трэнта"), ("bg", "Тренто"), ("ca", "Província de Trento"), ("ccp", "𑄑\u{11133}𑄢𑄬𑄚\u{11134}𑄑\u{11128}𑄚\u{1112e}"), ("ceb", "Provincia di Trento"), ("cs", "Autonomní provincie Trento"), ("da", "Trentino"), ("de", "Trentino"), ("el", "Τρέντο"), ("en", "Trentino"), ("es", "Trento"), ("et", "Trento provints"), ("eu", "Trentoko probintzia autonomoa"), ("fa", "ترنتینو"), ("fi", "Trenton maakunta"), ("fr", "province autonome de Trente"), ("gl", "Provincia autónoma de Trento"), ("he", "טרנטו"), ("hr", "Autonomna pokrajina Trident"), ("hu", "Trento megye"), ("hy", "Տրենտո"), ("id", "Provinsi Trento"), ("it", "provincia autonoma di Trento"), ("ja", "トレント自治県"), ("jv", "Provinsi otonom Trento"), ("ka", "ტრენტოს პროვინცია"), ("ko", "트렌토 현"), ("lt", "Trento provincija"), ("lv", "Trento province"), ("mk", "Трентино"), ("ms", "Wilayah Autonomi Trentino"), ("nb", "Trentino"), ("nl", "Trente"), ("no", "Trentino"), ("pl", "Prowincja Trydent"), ("pt", "Província autónoma de Trento"), ("ro", "Provincia Autonomă Trento"), ("ru", "Тренто"), ("sl", "Trento"), ("sq", "Provinca Autonome e Trentos"), ("sr", "Тренто"), ("sr_Latn", "Trento"), ("sv", "Trento"), ("tr", "Trento ili"), ("uk", "Провінція Тренто"), ("vi", "Trento"), ("zh", "特倫托自治省")]),
                        unofficial_name_list: ["Autonomous Province of Trento"].to_vec(),
                    }
                ),
                (
                    "TO",
                    Subdivision{
                        name: "TO",
                        country_alpha2: Alpha2::IT,
                        code: "TO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.070312), longitude: Some(7.686856499999999), max_latitude: Some(45.1335014), min_latitude: Some(45.00677659999999), max_longitude: Some(7.7623282), min_longitude: Some(7.5778502)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تورينو"), ("be", "правінцыя Турын"), ("bg", "Торино"), ("bn", "ত\u{9c1}রিণ-এর প\u{9cd}রদেশ"), ("ca", "província de Torí"), ("ccp", "𑄑\u{1112a}𑄢\u{11128}𑄚\u{11134}"), ("ceb", "Provincia di Torino"), ("cs", "Provincie Torino"), ("cy", "Talaith Torino"), ("da", "Torino"), ("de", "Provinz Turin"), ("el", "Τορίνο"), ("en", "Turin"), ("es", "Turín"), ("et", "Torino provints"), ("eu", "Turingo probintzia"), ("fa", "استان تورین"), ("fi", "Torinon maakunta"), ("fr", "province de Turin"), ("gl", "Provincia de Turín"), ("gu", "ટ\u{acd}ય\u{ac1}રિન પ\u{acd}રા\u{a82}ત"), ("he", "טורינו"), ("hi", "ट\u{94d}य\u{942}रिन प\u{94d}रा\u{902}त"), ("hu", "Torino megye"), ("hy", "Թուրին"), ("id", "Provinsi Torino"), ("is", "Tórínó"), ("it", "provincia di Torino"), ("ja", "トリノ県"), ("jv", "Provinsi Torino"), ("ka", "ტურინის პროვინცია"), ("kn", "ಟುರ\u{cbf}ನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "토리노 현"), ("lt", "Turino provincija"), ("lv", "Turīnas province"), ("mk", "Торино"), ("mr", "ट\u{94d}य\u{942}रिन प\u{94d}रा\u{902}त"), ("ms", "Wilayah Turin"), ("nb", "Provinsen Torino"), ("nl", "Turijn"), ("no", "Provinsen Torino"), ("pl", "Prowincja Turyn"), ("pt", "Turim"), ("ro", "Provincia Torino"), ("ru", "Турин"), ("si", "ට\u{dd2}ය\u{dd4}ර\u{dd2}න\u{dca} පළ\u{dcf}ත"), ("sl", "Torino"), ("sq", "Provinca e Torinos"), ("sr", "Торино"), ("sr_Latn", "Torino"), ("sv", "Torino"), ("ta", "டுரின\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "టుర\u{c3f}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เซมนาน"), ("tr", "Torino ili"), ("uk", "Провінція Турин"), ("ur", "صوبہ تورینو"), ("uz", "Turin"), ("vi", "Torino"), ("yue", "拖連奴省"), ("yue_Hans", "拖连奴省"), ("zh", "都靈省")]),
                        unofficial_name_list: ["Metropolitan City of Turin"].to_vec(),
                    }
                ),
                (
                    "TP",
                    Subdivision{
                        name: "TP",
                        country_alpha2: Alpha2::IT,
                        code: "TP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.0176177), longitude: Some(12.537202), max_latitude: Some(38.0342392), min_latitude: Some(38.0001964), max_longitude: Some(12.5857631), min_longitude: Some(12.4917452)}),
                        comments: None,
                        subdivision_type: SubdivisionType::FreeMunicipalConsortium,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تراباني"), ("be", "Трапані"), ("bg", "Трапани"), ("bn", "ত\u{9cd}র\u{9be}প\u{9be}নি-এর প\u{9cd}রদেশ"), ("ca", "Província de Trapani"), ("ccp", "𑄑\u{11133}𑄢𑄛𑄚\u{11128}"), ("ceb", "Trapani (lalawigan)"), ("cs", "Provincie Trapani"), ("da", "Province of Trapani"), ("de", "Provinz Trapani"), ("el", "Τράπανι"), ("en", "Trapani"), ("es", "Trapani"), ("et", "Trapani provints"), ("eu", "Trapaniko probintzia"), ("fa", "استان تراپانی"), ("fi", "Trapanin maakunta"), ("fr", "province de Trapani"), ("gl", "Provincia de Trapani"), ("gu", "ટ\u{acd}ર\u{ac5}પાની પ\u{acd}રા\u{a82}ત"), ("he", "טרפאני"), ("hi", "ट\u{94d}र\u{948}पनी प\u{94d}रा\u{902}त"), ("hr", "Trapani"), ("hu", "Trapani megye"), ("hy", "Տրապանի"), ("id", "Provinsi Trapani"), ("it", "provincia di Trapani"), ("ja", "トラーパニ県"), ("jv", "Provinsi Trapani"), ("ka", "ტრაპანის პროვინცია"), ("kk", "Трапани (провинция)"), ("kn", "ಟ\u{ccd}ರಾಪನ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "트라파니 현"), ("lt", "Trapanio provincija"), ("lv", "Trapāni province"), ("mr", "ट\u{94d}र\u{945}पनी प\u{94d}रा\u{902}त"), ("ms", "Wilayah Trapani"), ("nb", "Provinsen Trapani"), ("nl", "Trapani"), ("no", "Provinsen Trapani"), ("pl", "Prowincja Trapani"), ("pt", "Trapani"), ("ro", "Provincia Trapani"), ("ru", "Трапани"), ("si", "ට\u{dca}\u{200d}රප\u{dcf}න\u{dd2} පළ\u{dcf}ත"), ("sl", "Trapani"), ("sr", "Трапани"), ("sr_Latn", "Trapani"), ("sv", "Trapani"), ("ta", "ட\u{bcd}ர\u{bbe}ப\u{bcd}பணி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ట\u{c4d}ర\u{c3e}ప\u{c3e}న\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดทราปาน\u{e35}"), ("tr", "Trapani ili"), ("uk", "Провінція Трапані"), ("ur", "صوبہ تراپانی"), ("uz", "Trapani"), ("vi", "Trapani"), ("zh", "特拉帕尼省")]),
                        unofficial_name_list: ["Province of Trapani"].to_vec(),
                    }
                ),
                (
                    "TR",
                    Subdivision{
                        name: "TR",
                        country_alpha2: Alpha2::IT,
                        code: "TR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.5636168), longitude: Some(12.6426604), max_latitude: Some(42.6010524), min_latitude: Some(42.5230136), max_longitude: Some(12.6876014), min_longitude: Some(12.5685648)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تيرني"), ("be", "правінцыя Тэрні"), ("bg", "Терни"), ("bn", "ট\u{9be}র\u{9cd}নি-এর প\u{9cd}রদেশ"), ("ca", "Província de Terni"), ("ccp", "𑄑𑄬𑄢\u{11134}𑄚\u{11128}"), ("ceb", "Provincia di Terni"), ("cs", "Provincie Terni"), ("da", "Province of Terni"), ("de", "Provinz Terni"), ("el", "Τέρνι"), ("en", "Terni"), ("es", "Terni"), ("et", "Terni provints"), ("eu", "Terniko probintzia"), ("fa", "استان ترانی"), ("fi", "Ternin maakunta"), ("fr", "province de Terni"), ("gl", "Provincia de Terni"), ("gu", "ટ\u{ac7}ર\u{acd}નિ પ\u{acd}રા\u{a82}ત"), ("he", "טרני"), ("hi", "टर\u{94d}नि प\u{94d}रा\u{902}त"), ("hr", "Terni"), ("hu", "Terni megye"), ("hy", "Տեռնի"), ("id", "Provinsi Terni"), ("it", "provincia di Terni"), ("ja", "テルニ県"), ("jv", "Provinsi Terni"), ("ka", "ტერნის პროვინცია"), ("kn", "ಟರ\u{ccd}ನ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "테르니 현"), ("lt", "Ternio provincija"), ("lv", "Terni province"), ("mk", "Терни"), ("mr", "त\u{947}रणी प\u{94d}रा\u{902}त"), ("ms", "Wilayah Terni"), ("nb", "Provinsen Terni"), ("nl", "Terni"), ("no", "Provinsen Terni"), ("pl", "Prowincja Terni"), ("pt", "Terni"), ("ro", "Provincia Terni"), ("ru", "Терни"), ("si", "ටෙර\u{dca}න\u{dd2} පළ\u{dcf}ත"), ("sl", "Terni"), ("sq", "Provinca e Ternit"), ("sr", "Терни"), ("sr_Latn", "Terni"), ("sv", "Terni"), ("ta", "டெர\u{bcd}னி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ట\u{c47}ర\u{c4d}న\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเทอร\u{e4c}น\u{e34}"), ("tr", "Terni ili"), ("uk", "Провінція Терні"), ("ur", "صوبہ تیرنی"), ("uz", "Terni"), ("vi", "Terni"), ("zh", "特爾尼省")]),
                        unofficial_name_list: ["Province of Terni"].to_vec(),
                    }
                ),
                (
                    "TS",
                    Subdivision{
                        name: "TS",
                        country_alpha2: Alpha2::IT,
                        code: "TS",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::DecentralizedRegionalEntity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ترييستي"), ("be", "правінцыя Трыест"), ("bg", "Триест"), ("bn", "ট\u{9cd}রীস\u{9cd}টের প\u{9cd}রদেশ"), ("ca", "Província de Trieste"), ("ccp", "𑄑\u{11133}𑄢\u{1112d}𑄃𑄬𑄌\u{11134}𑄑\u{11128}"), ("ceb", "Trieste"), ("cs", "Provincie Trieste"), ("da", "Trieste"), ("de", "Provinz Triest"), ("el", "Τεργέστη"), ("en", "Trieste"), ("es", "Trieste"), ("et", "Trieste provints"), ("eu", "Triesteko probintzia"), ("fa", "استان تریسته"), ("fi", "Triesten maakunta"), ("fr", "Province de Trieste"), ("gl", "Provincia de Trieste"), ("gu", "ટ\u{acd}રીએસ\u{acd}ટ પ\u{acd}રા\u{a82}ત"), ("he", "טריאסטה"), ("hi", "त\u{94d}रिएस\u{94d}त\u{947} प\u{94d}रा\u{902}त"), ("hr", "Trst"), ("hu", "Trieszt megye"), ("id", "Provinsi Trieste"), ("it", "provincia di Trieste"), ("ja", "トリエステ県"), ("jv", "Provinsi Trieste"), ("ka", "ტრიესტის პროვინცია"), ("kn", "ಟ\u{ccd}ರೀಸ\u{ccd}ಟ\u{cc6} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "트리에스테 현"), ("lt", "Triesto provincija"), ("lv", "Triestes province"), ("mk", "Трст"), ("mr", "ट\u{94d}राएस\u{94d}ट\u{947} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Trieste"), ("nl", "Triëst"), ("no", "Provinsen Trieste"), ("pl", "Prowincja Triest"), ("pt", "Trieste"), ("ro", "Provincia Trieste"), ("ru", "Триест"), ("si", "ට\u{dca}\u{200d}ර\u{dd2}යෙස\u{dca}ටේ පළ\u{dcf}ත"), ("sl", "Tržaška pokrajina"), ("sq", "Provinca e Triestes"), ("sr", "Трст"), ("sr_Latn", "Trst"), ("sv", "Trieste"), ("ta", "ட\u{bcd}ரிஸ\u{bcd}ட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ట\u{c4d}ర\u{c40}స\u{c4d}ట\u{c46} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดทร\u{e35}เอสต\u{e4c}"), ("tr", "Trieste ili"), ("uk", "Провінція Трієст"), ("ur", "صوبہ تریستے"), ("uz", "Trieste"), ("vi", "Trieste")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "TV",
                    Subdivision{
                        name: "TV",
                        country_alpha2: Alpha2::IT,
                        code: "TV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.6668893), longitude: Some(12.2430437), max_latitude: Some(45.7030675), min_latitude: Some(45.63740139999999), max_longitude: Some(12.2873065), min_longitude: Some(12.1828564)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Treviso (provinsie)"), ("ar", "مقاطعة تريفيزو"), ("be", "правінцыя Трэвіза"), ("bg", "Тревизо"), ("bn", "ট\u{9cd}রভিসো প\u{9cd}রদেশ"), ("ca", "Província de Treviso"), ("ccp", "𑄑\u{11133}𑄢𑄬𑄞\u{11128}𑄥\u{1112e}"), ("ceb", "Provincia di Treviso"), ("cs", "Provincie Treviso"), ("da", "Treviso"), ("de", "Provinz Treviso"), ("el", "Τρεβίζο"), ("en", "Treviso"), ("es", "Treviso"), ("et", "Treviso provints"), ("eu", "Trevisoko probintzia"), ("fa", "استان ترویزو"), ("fi", "Trevison maakunta"), ("fr", "Province de Trévise"), ("gl", "Provincia de Treviso"), ("gu", "ટ\u{acd}ર\u{ac7}વિસો પ\u{acd}રા\u{a82}ત"), ("he", "טרוויזו"), ("hi", "ट\u{94d}र\u{947}विसो प\u{94d}रा\u{902}त"), ("hr", "Treviso"), ("hu", "Treviso megye"), ("hy", "Տրևիզո"), ("id", "Provinsi Treviso"), ("it", "provincia di Treviso"), ("ja", "トレヴィーゾ県"), ("jv", "Provinsi Treviso"), ("ka", "ტრევიზოს პროვინცია"), ("kn", "ಟ\u{ccd}ರ\u{cc6}ವ\u{cbf}ಸೋ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "트레비소 현"), ("lt", "Trevizo provincija"), ("lv", "Trevizo province"), ("mk", "Тревизо"), ("mr", "ट\u{94d}र\u{947}व\u{94d}हिसो प\u{94d}रा\u{902}त"), ("ms", "Wilayah Treviso"), ("nb", "Provinsen Treviso"), ("nl", "Treviso"), ("no", "Provinsen Treviso"), ("pl", "Prowincja Treviso"), ("pt", "Treviso"), ("ro", "Provincia Treviso"), ("ru", "Тревизо"), ("si", "ට\u{dca}\u{200d}රෙව\u{dd2}ස\u{dca} පළ\u{dcf}ත"), ("sl", "Treviso"), ("sq", "Provinca e Trevizos"), ("sr", "Тревизо (округ)"), ("sr_Latn", "Trevizo (okrug)"), ("sv", "Treviso"), ("ta", "ட\u{bcd}ரேவிசொ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d} ఆప\u{c4d} ట\u{c4d}ర\u{c46}వ\u{c3f}స\u{c4b}²"), ("th", "จ\u{e31}งหว\u{e31}ดเทรว\u{e34}โซ"), ("tr", "Treviso ili"), ("uk", "Провінція Тревізо"), ("ur", "صوبہ تریویزو"), ("uz", "Treviso"), ("vi", "Treviso"), ("zh", "特雷維索省")]),
                        unofficial_name_list: ["Province of Treviso"].to_vec(),
                    }
                ),
                (
                    "UD",
                    Subdivision{
                        name: "UD",
                        country_alpha2: Alpha2::IT,
                        code: "UD",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::DecentralizedRegionalEntity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أوديني"), ("be", "правінцыя Удзінэ"), ("bg", "Удине"), ("bn", "উদিনে প\u{9cd}রদেশ"), ("ca", "Província d’Udine"), ("ccp", "𑄃\u{1112a}𑄓\u{1112d}𑄚\u{11134}"), ("ceb", "Udine"), ("cs", "Provincie Udine"), ("da", "Udine"), ("de", "Provinz Udine"), ("el", "Ούντινε"), ("en", "Udine"), ("es", "Udine"), ("et", "Udine provints"), ("eu", "Udineko probintzia"), ("fa", "استان اودینه"), ("fi", "Udinen maakunta"), ("fr", "Province d’Udine"), ("gl", "Provincia de Udine"), ("gu", "ઉડીન પ\u{acd}રા\u{a82}ત"), ("he", "אודינה"), ("hi", "य\u{942}डाइन प\u{94d}रा\u{902}त"), ("hr", "Udine"), ("hu", "Udine megye"), ("hy", "Ուդինե"), ("id", "Provinsi Udine"), ("it", "provincia di Udine"), ("ja", "ウーディネ県"), ("jv", "Provinsi Udine"), ("ka", "უდინეს პროვინცია"), ("kn", "ಉಡೈನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "우디네 현"), ("lt", "Udinės provincija"), ("lv", "Udīnes province"), ("mk", "Удине"), ("mr", "उडीन प\u{94d}रा\u{902}त"), ("ms", "Wilayah Udine"), ("nl", "Udine"), ("no", "Provinsen Udine"), ("pl", "Prowincja Udine"), ("pt", "Udine"), ("ro", "Provincia Udine"), ("ru", "Удине"), ("si", "උඩ\u{dd2}නේ පළ\u{dcf}ත"), ("sl", "Videmska pokrajina"), ("sq", "Provinca e Udines"), ("sr", "Удине"), ("sr_Latn", "Udine"), ("sv", "Udine"), ("ta", "உடைனே ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఉడ\u{c3f}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เม\u{e37}องอ\u{e39}ด\u{e34}เน"), ("tr", "Udine ili"), ("uk", "Провінція Удіне"), ("ur", "صوبہ اودینے"), ("uz", "Udine"), ("vi", "Udine")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "VA",
                    Subdivision{
                        name: "VA",
                        country_alpha2: Alpha2::IT,
                        code: "VA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.82059890000001), longitude: Some(8.8250576), max_latitude: Some(45.8645247), min_latitude: Some(45.7805121), max_longitude: Some(8.863587299999999), min_longitude: Some(8.7759435)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فاريزي"), ("be", "Правінцыя Варэзэ"), ("bg", "Варезе"), ("bn", "ভ\u{9be}রেস-এর প\u{9cd}রদেশ"), ("ca", "Província de Varese"), ("ccp", "𑄞𑄢\u{11134}𑄥𑄬"), ("ceb", "Provincia di Varese"), ("cs", "Provincie Varese"), ("da", "Varese"), ("de", "Provinz Varese"), ("el", "Επαρχία του Βαρέζε"), ("en", "Varese"), ("es", "Varese"), ("et", "Varese provints"), ("eu", "Vareseko probintzia"), ("fa", "استان ورزه"), ("fi", "Varesen maakunta"), ("fr", "Province de Varèse"), ("gl", "Provincia de Varese"), ("gu", "વર\u{ac7}સ\u{ac7} પ\u{acd}રા\u{a82}ત"), ("he", "וארזה"), ("hi", "वर\u{94d}स\u{947} प\u{94d}रा\u{902}त"), ("hr", "okrug Vareze"), ("hu", "Varese megye"), ("hy", "Վարեզե"), ("id", "Provinsi Varese"), ("it", "provincia di Varese"), ("ja", "ヴァレーゼ県"), ("jv", "Provinsi Varese"), ("ka", "ვარეზეს პროვინცია"), ("kn", "ವರೇಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "바레세 현"), ("lt", "Varezės provincija"), ("lv", "Varēzes province"), ("mr", "वार\u{947}स\u{947} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Varese"), ("nb", "Provinsen Varese"), ("nl", "Varese"), ("no", "Provinsen Varese"), ("pl", "Prowincja Varese"), ("pt", "Varese"), ("ro", "Provincia Varese"), ("ru", "Варезе"), ("si", "වරෙසේ පළ\u{dcf}ත"), ("sk", "Varese"), ("sl", "Varese"), ("sq", "Provinca e Varezes"), ("sr", "Варезе"), ("sr_Latn", "Vareze"), ("sv", "Varese"), ("ta", "வ\u{bbe}ரேஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "వర\u{c40}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดวาเรเซ"), ("tr", "Varese ili"), ("uk", "Провінція Варезе"), ("ur", "صوبہ واریزے"), ("uz", "Varese"), ("vi", "Varese"), ("zh", "瓦雷澤省")]),
                        unofficial_name_list: ["Province of Varese"].to_vec(),
                    }
                ),
                (
                    "VB",
                    Subdivision{
                        name: "VB",
                        country_alpha2: Alpha2::IT,
                        code: "VB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1399688), longitude: Some(8.2724649), max_latitude: Some(46.4644351), min_latitude: Some(45.7669133), max_longitude: Some(8.7110842), min_longitude: Some(7.8684223)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فربانو كوزيو أوسولا"), ("be", "правінцыя Вербана-Кузья-Асола"), ("bg", "Вербано-Кузио-Осола"), ("bn", "ভ\u{9be}রব\u{9be}নো-ক\u{9c1}সিও-অসস\u{9be}ল\u{9be}"), ("ca", "Província de Verbano-Cusio-Ossola"), ("ccp", "𑄞𑄢\u{11134}𑄝𑄚\u{1112e}-𑄇\u{11128}𑄅\u{1112a}𑄥\u{11128}𑄃\u{1112e}-𑄃\u{1112e}𑄥\u{1112e}𑄣"), ("ceb", "Provincia Verbano-Cusio-Ossola"), ("cs", "Provincie Verbano-Cusio-Ossola"), ("da", "Verbano-Cusio-Ossola"), ("de", "Provinz Verbano-Cusio-Ossola"), ("el", "Βερμπάνο-Κούζιο-Όσολα"), ("en", "Verbano-Cusio-Ossola"), ("es", "Verbano-Cusio-Ossola"), ("et", "Verbano-Cusio-Ossola provints"), ("eu", "Verbano-Cusio-Ossolako probintzia"), ("fa", "استان وربانو-کوزیو-اوسولا"), ("fi", "Verbano-Cusio-Ossolan maakunta"), ("fr", "province du Verbano-Cusio-Ossola"), ("gl", "Provincia de Verbano-Cusio-Ossola"), ("gu", "વર\u{acd}બ\u{ac7}નો-ક\u{acd}ય\u{ac1}સિઓ-ઓસ\u{acd}સોલા"), ("hi", "व\u{947}र\u{94d}ब\u{948}नो-क\u{941}सिओ-ओसोला"), ("hu", "Verbano-Cusio-Ossola megye"), ("hy", "Վերբանիո-Կուզիո-Օսոլա"), ("id", "Provinsi Verbano-Cusio-Ossola"), ("is", "Verbania"), ("it", "provincia del Verbano-Cusio-Ossola"), ("ja", "ヴェルバーノ・クジオ・オッソラ県"), ("jv", "Provinsi Verbano-Cusio-Ossola"), ("ka", "ვერბანო-კუზო-ოსოლის პროვინცია"), ("kn", "ವ\u{cc6}ರ\u{ccd}ಬಾನೊ-ಕುಸ\u{cbf}ಯೊ-ಒಸ\u{ccd}ಸೊಲಾ"), ("ko", "베르바노쿠시오오솔라 현"), ("lt", "Verbano-Kuzijo-Osolos provincija"), ("lv", "Verbāno-Kuzio-Osolas province"), ("mr", "व\u{947}र\u{94d}बानो-क\u{94d}य\u{941}सिओ-ओस\u{94d}सोला"), ("ms", "Wilayah Verbano-Cusio-Ossola"), ("nb", "Provinsen Verbano Cusio Ossola"), ("nl", "Verbano-Cusio-Ossola"), ("no", "Provinsen Verbano Cusio Ossola"), ("pl", "Prowincja Cusio Ossola"), ("pt", "Verbano Cusio Ossola"), ("ro", "Provincia Verbano-Cusio-Ossola"), ("ru", "Вербано-Кузьо-Оссола"), ("si", "වර\u{dca}බ\u{dcf}නෝ-ක\u{dd4}ස\u{dd2}ඕ-ඔසොල\u{dcf}"), ("sl", "Verbano-Cusio-Ossola"), ("sq", "Provinca e Verbano-Kuzio-Osolës"), ("sr", "Вербано-Кузио-Осола"), ("sr_Latn", "Verbano-Kuzio-Osola"), ("sv", "Verbano Cusio Ossola"), ("ta", "வேர\u{bcd}பனோ -சுசியோ-ஓஸோல\u{bbe}"), ("te", "వ\u{c46}ర\u{c4d}బ\u{c3e}న\u{c4b}-కూస\u{c3f}య\u{c4b}-ఓస\u{c4b}ల\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดเวอร\u{e4c}บาโน-ค\u{e39}ซ\u{e34}โอ-ออสโซลา"), ("tr", "Verbano-Cusio-Ossola ili"), ("uk", "Провінція Вербано-Кузіо-Оссола"), ("ur", "صوبہ ویربانو-کوزیو-اوسولا"), ("uz", "Verbano-Cusio-Ossola"), ("vi", "Verbano-Cusio-Ossola"), ("zh", "韋爾巴諾-庫西亞-奧索拉省")]),
                        unofficial_name_list: ["Province of Verbano-Cusio-Ossola"].to_vec(),
                    }
                ),
                (
                    "VC",
                    Subdivision{
                        name: "VC",
                        country_alpha2: Alpha2::IT,
                        code: "VC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.32022720000001), longitude: Some(8.418573499999999), max_latitude: Some(45.3419825), min_latitude: Some(45.29851679999999), max_longitude: Some(8.4576716), min_longitude: Some(8.3904657)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فرشيلي"), ("be", "правінцыя Верчэлі"), ("bg", "Верчели"), ("bn", "ভ\u{9be}র\u{9cd}চেলি প\u{9cd}রদেশ"), ("ca", "Província de Vercelli"), ("ccp", "𑄞𑄢\u{11134}𑄥𑄬𑄣\u{11128}"), ("ceb", "Provincia di Vercelli"), ("cs", "Provincie Vercelli"), ("da", "Vercelli"), ("de", "Provinz Vercelli"), ("el", "Επαρχία του Βερτσέλλι"), ("en", "Vercelli"), ("es", "Vercelli"), ("et", "Vercelli provints"), ("eu", "Vercelliko probintzia"), ("fa", "استان ورسلی"), ("fi", "Vercellin maakunta"), ("fr", "Province de Verceil"), ("gl", "Provincia de Vercelli"), ("gu", "વર\u{acd}સ\u{ac7}લી પ\u{acd}રા\u{a82}ત"), ("hi", "व\u{947}र\u{94d}स\u{947}ली प\u{94d}रा\u{902}त"), ("hu", "Vercelli megye"), ("hy", "Վերչելլի"), ("id", "Provinsi Vercelli"), ("is", "Vercelli"), ("it", "provincia di Vercelli"), ("ja", "ヴェルチェッリ県"), ("jv", "Provinsi Vercelli"), ("ka", "ვერჩელის პროვინცია"), ("kn", "ವ\u{cc6}ರ\u{ccd}ಸ\u{cc6}ಲ\u{ccd}ಲ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "베르첼리 현"), ("lt", "Verčelio provincija"), ("lv", "Verčelli province"), ("mr", "वॉर\u{94d}स\u{947}ली प\u{94d}रा\u{902}त"), ("ms", "Wilayah Vercelli"), ("nb", "Provinsen Vercelli"), ("nl", "Vercelli"), ("no", "Provinsen Vercelli"), ("pl", "Prowincja Vercelli"), ("pt", "Vercelli"), ("ro", "Provincia Vercelli"), ("ru", "Верчелли"), ("si", "වර\u{dca}සේල\u{dd2} පළ\u{dcf}ත"), ("sl", "Vercelli"), ("sq", "Provinca e Verçelit"), ("sr", "Верчели"), ("sr_Latn", "Verčeli"), ("sv", "Vercelli"), ("ta", "வெர\u{bcd}ஸள\u{bcd}ளி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "వ\u{c46}ర\u{c4d}స\u{c46}ల\u{c4d}ల\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเวอร\u{e4c}เซลล\u{e35}\u{e48}"), ("tr", "Vercelli ili"), ("uk", "Провінція Верчеллі"), ("ur", "صوبہ ویرچیلی"), ("uz", "Vercelli"), ("vi", "Vercelli"), ("zh", "韋爾切利省")]),
                        unofficial_name_list: ["Province of Vercelli"].to_vec(),
                    }
                ),
                (
                    "VE",
                    Subdivision{
                        name: "VE",
                        country_alpha2: Alpha2::IT,
                        code: "VE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.4408474), longitude: Some(12.3155151), max_latitude: Some(45.5779746), min_latitude: Some(45.233455), max_longitude: Some(12.5966574), min_longitude: Some(12.1668278)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فينيسيا"), ("be", "Венецыя"), ("bg", "Венеция"), ("bn", "ভেনিস-এর প\u{9cd}রদেশ"), ("ca", "Província de Venècia"), ("ccp", "𑄞𑄬𑄚\u{1112d}𑄌\u{11134}"), ("cs", "Provincie Venezia"), ("da", "Venedig"), ("de", "Provinz Venedig"), ("el", "Βενετία"), ("en", "Venice"), ("es", "Venecia"), ("et", "Venezia provints"), ("eu", "Veneziako probintzia"), ("fa", "استان ونیز"), ("fi", "Venetsian maakunta"), ("fr", "province de Venise"), ("gl", "Provincia de Venecia"), ("gu", "વ\u{ac7}નિસ પ\u{acd}રા\u{a82}ત"), ("he", "ונציה"), ("hi", "व\u{947}निस प\u{94d}रा\u{902}त"), ("hr", "Venecija"), ("hu", "Velence megye"), ("hy", "Վենետիկ"), ("id", "Provinsi Venezia"), ("it", "provincia di Venezia"), ("ja", "ヴェネツィア県"), ("jv", "Provinsi Venezia"), ("ka", "ვენეციის პროვინცია"), ("kn", "ವ\u{cc6}ನ\u{cbf}ಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "베네치아 현"), ("lt", "Venecijos provincija"), ("lv", "Venēcijas province"), ("mk", "Венеција"), ("mr", "व\u{94d}ह\u{947}निस प\u{94d}रा\u{902}त"), ("ms", "Wilayah Venice"), ("nb", "Provinsen Venezia"), ("nl", "Venetië"), ("no", "Provinsen Venezia"), ("pl", "Prowincja Wenecja"), ("pt", "Veneza"), ("ro", "Provincia Veneția"), ("ru", "Венеция²"), ("si", "වෙන\u{dd2}ස\u{dca} පළ\u{dcf}ත"), ("sl", "Venezia"), ("sq", "Provinca e Veneciës"), ("sr", "Венеција (округ)"), ("sr_Latn", "Venecija (okrug)"), ("sv", "Venedig"), ("ta", "வெனிஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "వ\u{c46}న\u{c3f}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเวน\u{e34}ส"), ("tr", "Venezia ili"), ("uk", "Провінція Венеція"), ("ur", "صوبہ وینس"), ("uz", "Venetsiya"), ("vi", "Venezia"), ("yue", "威尼斯省"), ("yue_Hans", "威尼斯省"), ("zh", "威尼斯省")]),
                        unofficial_name_list: ["Metropolitan City of Venice"].to_vec(),
                    }
                ),
                (
                    "VI",
                    Subdivision{
                        name: "VI",
                        country_alpha2: Alpha2::IT,
                        code: "VI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.5454787), longitude: Some(11.5354214), max_latitude: Some(45.6039311), min_latitude: Some(45.4965127), max_longitude: Some(11.6108189), min_longitude: Some(11.4910099)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فيشنزا"), ("be", "правінцыя Вічэнца"), ("bg", "Виченца"), ("bn", "ভিচেঞ\u{9cd}জ\u{9be}র প\u{9cd}রদেশ"), ("ca", "Província de Vicenza"), ("ccp", "𑄞\u{1112d}𑄥𑄬𑄚\u{11134}𑄎"), ("ceb", "Provincia di Vicenza"), ("cs", "Provincie Vicenza"), ("da", "Vicenza"), ("de", "Provinz Vicenza"), ("el", "Βιτσέντσα"), ("en", "Vicenza"), ("es", "Vicenza"), ("et", "Vicenza provints"), ("eu", "Vicenzako probintzia"), ("fa", "استان ویچنزا"), ("fi", "Vicenzan maakunta"), ("fr", "province de Vicence"), ("gl", "Provincia de Vicenza"), ("gu", "વિસ\u{ac7}ન\u{acd}ઝા પ\u{acd}રા\u{a82}ત"), ("he", "ויצ׳נצה"), ("hi", "विक\u{947}\u{902}ज\u{93c}ा प\u{94d}रा\u{902}त"), ("hr", "Vicenza (pokrajina)"), ("hu", "Vicenza megye"), ("hy", "Վիչենցա"), ("id", "Provinsi Vicenza"), ("it", "provincia di Vicenza"), ("ja", "ヴィチェンツァ県"), ("jv", "Provinsi Vicenza"), ("ka", "ვიჩენცის პროვინცია"), ("kn", "ವ\u{cbf}ಸ\u{cc6}ಂಜದ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "비첸차 현"), ("lt", "Vičencos provincija"), ("lv", "Vičencas province"), ("mk", "Виченца"), ("mr", "व\u{94d}हिस\u{947}ना प\u{94d}रा\u{902}त"), ("ms", "Wilayah Vicenza"), ("nb", "Provinsen"), ("nl", "Vicenza"), ("no", "Provinsen Vicenza"), ("pl", "Prowincja Vicenza"), ("pt", "Vicenza"), ("ro", "Provincia Vicenza"), ("ru", "Виченца"), ("si", "ව\u{dd2}සේන\u{dca}ස\u{dcf} පළ\u{dcf}ත"), ("sl", "Vicenza"), ("sq", "Provinca e Viçencës"), ("sr", "Виченца (округ)"), ("sr_Latn", "Vičenca (okrug)"), ("sv", "Vicenza"), ("ta", "விசென\u{bcd}சர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "వ\u{c3f}స\u{c46}ంజ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดว\u{e34}เซนซา"), ("tr", "Vicenza ili"), ("uk", "Провінція Віченца"), ("ur", "صوبہ ویچینسا"), ("uz", "Vicenza"), ("vi", "Vicenza"), ("zh", "維琴察省")]),
                        unofficial_name_list: ["Province of Vicenza"].to_vec(),
                    }
                ),
                (
                    "VR",
                    Subdivision{
                        name: "VR",
                        country_alpha2: Alpha2::IT,
                        code: "VR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.43838419999999), longitude: Some(10.9916215), max_latitude: Some(45.4782665), min_latitude: Some(45.3742342), max_longitude: Some(11.0703201), min_longitude: Some(10.9147425)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فيرونا"), ("be", "Верона"), ("bg", "Верона"), ("bn", "ভেরোন\u{9be} এর প\u{9cd}রদেশ"), ("ca", "Província de Verona"), ("ccp", "𑄞𑄬𑄢\u{1112e}𑄚"), ("ceb", "Provincia di Verona"), ("cs", "Provincie Verona"), ("da", "Verona"), ("de", "Provinz Verona"), ("el", "Βερόνα"), ("en", "Verona"), ("es", "Verona"), ("et", "Verona provints"), ("eu", "Veronako probintzia"), ("fa", "استان ورونا"), ("fi", "Veronan maakunta"), ("fr", "province de Vérone"), ("gl", "Provincia de Verona"), ("gu", "વ\u{ac7}રોના પ\u{acd}રા\u{a82}ત"), ("he", "ורונה"), ("hi", "व\u{947}रोना प\u{94d}रा\u{902}त"), ("hr", "Verona (pokrajina)"), ("hu", "Verona megye"), ("hy", "Վերոնա"), ("id", "Provinsi Verona"), ("it", "provincia di Verona"), ("ja", "ヴェローナ県"), ("jv", "Provinsi Verona"), ("ka", "ვერონის პროვინცია"), ("kn", "ವ\u{cc6}ರೋನಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "베로나 현"), ("lt", "Veronos provincija"), ("lv", "Veronas province"), ("mk", "Верона"), ("mr", "व\u{947}रोना प\u{94d}रा\u{902}त"), ("ms", "Wilayah Verona"), ("nb", "Provinsen Verona"), ("nl", "Verona"), ("no", "Provinsen Verona"), ("pl", "Prowincja Werona"), ("pt", "Verona"), ("ro", "Provincia Verona"), ("ru", "Верона"), ("si", "වෙරෝන\u{dcf} පළ\u{dcf}ත"), ("sl", "Verona"), ("sq", "Provinca e Veronës"), ("sr", "Верона"), ("sr_Latn", "Verona"), ("sv", "Verona"), ("ta", "வெரோன\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "వ\u{c46}ర\u{c4b}న\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเวโรนา"), ("tr", "Verona ili"), ("uk", "Провінція Верона"), ("ur", "صوبہ ویرونا"), ("uz", "Verona"), ("vi", "Verona"), ("zh", "維羅納省")]),
                        unofficial_name_list: ["Province of Verona"].to_vec(),
                    }
                ),
                (
                    "VT",
                    Subdivision{
                        name: "VT",
                        country_alpha2: Alpha2::IT,
                        code: "VT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.4206766), longitude: Some(12.107669), max_latitude: Some(42.4405429), min_latitude: Some(42.4044656), max_longitude: Some(12.1411634), min_longitude: Some(12.0796173)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فيتيربو"), ("be", "правінцыя Вітэрба"), ("bg", "Витербо"), ("bn", "ভিট\u{9be}রবো প\u{9cd}রদেশ"), ("ca", "Província de Viterbo"), ("ccp", "𑄞\u{1112d}𑄑𑄬𑄢\u{11134}𑄝\u{1112e}"), ("ceb", "Viterbo"), ("cs", "Provincie Viterbo"), ("da", "Viterbo Province"), ("de", "Provinz Viterbo"), ("el", "Βιτέρμπο"), ("en", "Viterbo"), ("es", "Viterbo"), ("et", "Viterbo provints"), ("eu", "Viterboko probintzia"), ("fa", "استان ویتربو"), ("fi", "Viterbon maakunta"), ("fr", "province de Viterbe"), ("gl", "Provincia de Viterbo"), ("gu", "વિટરબો પ\u{acd}રા\u{a82}ત"), ("he", "ויטרבו"), ("hi", "विट\u{947}र\u{94d}बो प\u{94d}रा\u{902}त"), ("hr", "Viterbo"), ("hu", "Viterbo megye"), ("hy", "Վիտերբո"), ("id", "Provinsi Viterbo"), ("it", "provincia di Viterbo"), ("ja", "ヴィテルボ県"), ("jv", "Provinsi Viterbo"), ("ka", "ვიტერბოს პროვინცია"), ("kn", "ವ\u{cbf}ಟರ\u{ccd}ಬೋ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "비테르보 현"), ("lt", "Viterbo provincija"), ("lv", "Viterbo province"), ("mr", "विटर\u{94d}बो प\u{94d}रा\u{902}त"), ("ms", "Wilayah Viterbo"), ("nb", "Provinsen Viterbo"), ("nl", "Viterbo"), ("no", "Provinsen Viterbo"), ("pl", "Prowincja Viterbo"), ("pt", "Viterbo"), ("ro", "Provincia Viterbo"), ("ru", "Витербо"), ("si", "ව\u{dd2}ටෙර\u{dca}බෝ පළ\u{dcf}ත"), ("sk", "Viterbo"), ("sl", "Viterbo"), ("sr", "Витербо"), ("sr_Latn", "Viterbo"), ("sv", "Viterbo"), ("ta", "விட\u{bcd}டர\u{bcd}போ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "వ\u{c3f}ట\u{c46}ర\u{c4d}బ\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดว\u{e34}เตอร\u{e4c}โบ"), ("tr", "Viterbo ili"), ("uk", "Провінція Вітербо"), ("ur", "صوبہ ویتیربو"), ("uz", "Viterbo"), ("vi", "Viterbo"), ("zh", "维泰博省")]),
                        unofficial_name_list: ["Province of Viterbo"].to_vec(),
                    }
                ),
                (
                    "VV",
                    Subdivision{
                        name: "VV",
                        country_alpha2: Alpha2::IT,
                        code: "VV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.6757774), longitude: Some(16.0983488), max_latitude: Some(38.6929198), min_latitude: Some(38.6645713), max_longitude: Some(16.1277849), min_longitude: Some(16.0756393)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فيبو فالينتيا"), ("be", "правінцыя Віба-Валентыя"), ("bg", "Вибо Валентия"), ("bn", "ভিভো ভ\u{9cd}য\u{9be}লেন\u{9cd}সিয\u{9bc}\u{9be}-এর প\u{9cd}রদেশ"), ("ca", "Província de Vibo Valentia"), ("ccp", "𑄞\u{1112d}𑄝\u{1112e} 𑄞𑄣𑄬𑄚\u{11134}𑄑\u{11128}𑄠"), ("ceb", "Vibo Valentia"), ("cs", "Provincie Vibo Valentia"), ("da", "Province of Vibo Valentia"), ("de", "Provinz Vibo Valentia"), ("el", "Βίμπο Βαλέντια"), ("en", "Vibo Valentia"), ("es", "Vibo Valentia"), ("et", "Vibo Valentia provints"), ("eu", "Vibo Valentiako probintzia"), ("fa", "استان ویبو والنتیا"), ("fi", "Vibo Valentian maakunta"), ("fr", "province de Vibo Valentia"), ("gl", "Provincia de Vibo Valentia"), ("gu", "વિબો વ\u{ac7}લ\u{ac7}ન\u{acd}ટિયા પ\u{acd}રા\u{a82}ત"), ("he", "ויבו ולנטיה"), ("hi", "वीबो व\u{948}ल\u{947}\u{902}शिया प\u{94d}रा\u{902}त"), ("hu", "Vibo Valentia megye"), ("hy", "Վիբո Վալենտիա"), ("id", "Provinsi Vibo Valentia"), ("it", "provincia di Vibo Valentia"), ("ja", "ヴィボ・ヴァレンツィア県"), ("jv", "Provinsi Vibo Valentia"), ("ka", "ვიბო-ვალენტიის პროვინცია"), ("kn", "ವ\u{cbf}ಬೋ ವ\u{ccd}ಯಾಲ\u{cc6}ಂಟ\u{cbf}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "비보발렌티아 현"), ("lt", "Vibo Valentijos provincija"), ("lv", "Vibo Valentijas province"), ("mr", "व\u{94d}हिबो व\u{94d}ह\u{945}ल\u{947}\u{902}टीना प\u{94d}रा\u{902}त"), ("ms", "Wilayah Vibo Valentia"), ("nb", "Provinsen Vibo Valentia"), ("nl", "Vibo Valentia"), ("no", "Provinsen Vibo Valentia"), ("pl", "Prowincja Vibo Walentia"), ("pt", "Vibo Valentia"), ("ro", "Provincia Vibo Valentia"), ("ru", "Вибо-Валентия"), ("si", "ව\u{dd2}බෝ වැලෙන\u{dca}ට\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sl", "Vibo Valentia"), ("sq", "Provinca e Vibo Valentias"), ("sr", "Вибо Валентија"), ("sr_Latn", "Vibo Valentija"), ("sv", "Vibo Valentia"), ("ta", "விபோ வ\u{bbe}லெண\u{bcd}டிய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "వ\u{c3f}బ\u{c4b} వ\u{c3e}ల\u{c46}ంట\u{c3f}న\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดว\u{e35}โบวาเลนเต\u{e35}ย"), ("tr", "Vibo Valentia ili"), ("uk", "Провінція Вібо-Валентія"), ("ur", "صوبہ ویبو والینتسیا"), ("uz", "Vibo Valentia"), ("vi", "Vibo Valentia"), ("zh", "維博瓦倫蒂亞省")]),
                        unofficial_name_list: ["Province of Vibo Valentia"].to_vec(),
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
#[cfg(feature = "it")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::IT,
        alpha3: Alpha3::ITA,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{postalcode}} {{city}} {{region_short}}\n{{country}}",
        ),
        continent: Continent::Europe,
        country_code: 39,
        currency_code: "EUR",
        gec: Some(GEC::IT),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("ITA"),
        iso_long_name: "The Italian Republic",
        iso_short_name: "Italy",
        official_language_list: ["it"].to_vec(),
        spoken_language_list: ["it"].to_vec(),
        national_destination_code_length_list: [3].to_vec(),
        national_number_length_list: [9, 11].to_vec(),
        national_prefix: "None",
        nationality: Some("Italian"),
        number: "380",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthernEurope),
        un_locode: "IT",
        unofficial_name_list: ["Italy", "Italien", "Italie", "Italia", "イタリア", "Italië"]
            .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Italy"),
            ("af", "Italië"),
            ("ak", "Italy"),
            ("am", "ጣሑ።ን"),
            ("an", "Italy"),
            ("ar", "إيطاليا"),
            ("as", "ইট\u{9be}লি"),
            ("ay", "Italy"),
            ("az", "İtaliya"),
            ("ba", "Italy"),
            ("be", "Італія"),
            ("bg", "Италия"),
            ("bi", "Italy"),
            ("bn", "ইট\u{9be}লি"),
            ("bn_IN", "ইট\u{9be}লি"),
            ("br", "Italia"),
            ("bs", "Italija"),
            ("ca", "Itàlia"),
            ("ce", "Итали"),
            ("ch", "Italia"),
            ("cs", "Itálie"),
            ("cv", "Итали"),
            ("cy", "Yr Eidal"),
            ("da", "Italien"),
            ("de", "Italien"),
            ("dv", "އ\u{7a8}ޓ\u{7a6}ލ\u{7a9}ވ\u{7a8}ލ\u{7a7}ތ\u{7b0}"),
            ("dz", "ཨ\u{f72}་ཊ་ལ\u{f72}།"),
            ("ee", "Italy"),
            ("el", "Ιταλία"),
            ("en", "Italy"),
            ("eo", "Italio"),
            ("es", "Italia"),
            ("et", "Itaalia"),
            ("eu", "Italia"),
            ("fa", "ایتالیا"),
            ("ff", "Italiya"),
            ("fi", "Italia"),
            ("fo", "Italia"),
            ("fr", "Italie"),
            ("fy", "Itaalje"),
            ("ga", "An Iodáil"),
            ("gl", "Italia"),
            ("gn", "Italy"),
            ("gu", "ઇટાલી"),
            ("gv", "Yn Iddaal"),
            ("ha", "Italiya"),
            ("he", "איטליה"),
            ("hi", "इटली"),
            ("hr", "Italija"),
            ("ht", "Itali"),
            ("hu", "Olaszország"),
            ("hy", "Իտալիա"),
            ("ia", "Italia"),
            ("id", "Italia"),
            ("io", "Italia"),
            ("is", "Ítalía"),
            ("it", "Italia"),
            ("iu", "Italy"),
            ("ja", "イタリア"),
            ("ka", "იტალია"),
            ("ki", "Italia"),
            ("kk", "Италия"),
            ("kl", "Italy"),
            ("km", "អ\u{17ca}\u{17b8}តាល\u{17b8}"),
            ("kn", "ಇಟಲ\u{cbf}"),
            ("ko", "이탈리아"),
            ("ku", "Îtalya"),
            ("kv", "Италия"),
            ("kw", "Itali"),
            ("ky", "Италия"),
            ("lo", "ອ\u{eb4}ຕາລ\u{eb5}"),
            ("lt", "Italija"),
            ("lv", "Itālija"),
            ("mi", "Itāria"),
            ("mk", "Италија"),
            ("ml", "ഇറ\u{d4d}റലി"),
            ("mn", "Итали"),
            ("mr", "इटली"),
            ("ms", "Itali"),
            ("mt", "Italja"),
            (
                "my",
                "အ\u{102e}တလ\u{102e}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Itari"),
            ("nb", "Italia"),
            ("ne", "इटाली"),
            ("nl", "Italië"),
            ("nn", "Italia"),
            ("nv", "Doohatsʼíí Yátiʼ Dineʼé Bikéyah"),
            ("oc", "Itàlia"),
            ("or", "ଇଟ\u{b3e}ଲୀ"),
            ("pa", "ਇਟਲੀ"),
            ("pi", "इटली"),
            ("pl", "Włochy"),
            ("ps", "ایټالیه"),
            ("pt", "Itália"),
            ("pt_BR", "Itália"),
            ("ro", "Italia"),
            ("ru", "Италия"),
            ("rw", "Ubutariyani"),
            ("sc", "Itàlia"),
            ("sd", "اٽلي"),
            ("si", "ඉත\u{dcf}ල\u{dd2}ය"),
            ("sk", "Taliansko"),
            ("sl", "Italija"),
            ("so", "Talyaani"),
            ("sq", "Itali"),
            ("sr", "Италија"),
            ("sv", "Italien"),
            ("sw", "Italy"),
            ("ta", "இத\u{bcd}த\u{bbe}லி"),
            ("te", "ఇటల\u{c40}"),
            ("tg", "Италия"),
            ("th", "อ\u{e34}ตาล\u{e35}"),
            ("ti", "ኢጣልያ"),
            ("tk", "Italiýa"),
            ("tl", "Italya"),
            ("tr", "İtalya"),
            ("tt", "İталиа"),
            ("ug", "ئىتالىيە"),
            ("uk", "Італія"),
            ("ur", "اطالیہ"),
            ("uz", "Italiya"),
            ("ve", "Italy"),
            ("vi", "Ý"),
            ("wa", "Itåleye"),
            ("wo", "Itaali"),
            ("xh", "Ithali"),
            ("yo", "Itálíà"),
            ("zh_CN", "意大利"),
            ("zh_HK", "意大利"),
            ("zh_TW", "義大利"),
            ("zu", "ITaliya"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
