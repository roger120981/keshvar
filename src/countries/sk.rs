// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Slovak Republic

#[cfg(all(feature = "sk", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::SK;
    pub const ALPHA3: Alpha3 = Alpha3::SVK;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 421;
    pub const CURRENCY_CODE: &str = "EUR";
    pub const GEC: Option<GEC> = Some(GEC::LO);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("SVK");
    pub const ISO_SHORT_NAME: &str = "Slovakia";
    pub const ISO_LONG_NAME: &str = "The Slovak Republic";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["sk"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["sk"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Slovak");
    pub const NUMBER: &str = "703";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{3} ?\\d{2}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternEurope);
    pub const UN_LOCODE: &str = "SK";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Slovakia",
        "Slowakei",
        "Slovaquie",
        "República Eslovaca",
        "スロバキア",
        "Slowakije",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Slovakia"),
        ("af", "Slowakye"),
        ("ak", "Slovakia"),
        ("am", "ስሕቲጡ።"),
        ("an", "Slovakia"),
        ("ar", "سلوفاكيا"),
        ("as", "স\u{9cd}লোভ\u{9be}কিয়\u{9be}"),
        ("ay", "Slovakia"),
        ("az", "Slovakiya"),
        ("ba", "Slovakia"),
        ("be", "Славакія"),
        ("bg", "Словакия"),
        ("bi", "Slovakia"),
        ("bn", "স\u{9cd}লোভ\u{9be}কিয়\u{9be}"),
        ("bn_IN", "স\u{9cd}লোভ\u{9be}কিয়\u{9be}"),
        ("br", "Slovakia"),
        ("bs", "Slovačka"),
        ("ca", "Eslovàquia"),
        ("ce", "Словаки"),
        ("ch", "Slovakia"),
        ("cs", "Slovensko"),
        ("cv", "Словаки"),
        ("cy", "Slofacia"),
        ("da", "Slovakiet"),
        ("de", "Slowakei"),
        ("dv", "ސ\u{7aa}ލ\u{7ae}ވ\u{7a7}ކ\u{7a8}އ\u{7a7}"),
        ("dz", "ས\u{f7c}ལ\u{f7c}་བ་ཀ\u{f72}་ཡ།"),
        ("ee", "Slovakia"),
        ("el", "Σλοβακία"),
        ("en", "Slovakia"),
        ("eo", "Slovakio"),
        ("es", "Eslovaquia"),
        ("et", "Slovakkia"),
        ("eu", "Eslovakia"),
        ("fa", "اسلواکی"),
        ("ff", "Sulowakiya"),
        ("fi", "Slovakia"),
        ("fo", "Slovakia"),
        ("fr", "Slovaquie"),
        ("fy", "Slowakije"),
        ("ga", "An tSlóvaic"),
        ("gl", "Eslovaquia"),
        ("gn", "Slovakia"),
        ("gu", "સ\u{acd}લોવ\u{ac7}કિઆ"),
        ("gv", "Yn Clovack"),
        ("ha", "Slofakiya"),
        ("he", "סלובקיה"),
        ("hi", "स\u{94d}लोवाकिया"),
        ("hr", "Slovačka"),
        ("ht", "Slovaki"),
        ("hu", "Szlovákia"),
        ("hy", "Սլովակիա"),
        ("ia", "Slovachia"),
        ("id", "Slowakia"),
        ("io", "Slovakia"),
        ("is", "Slóvakía"),
        ("it", "Slovacchia"),
        ("iu", "Slovakia"),
        ("ja", "スロバキア"),
        ("ka", "სლოვაკეთი"),
        ("ki", "Slovakia"),
        ("kk", "Словакия"),
        ("kl", "Slovakia"),
        ("km", "ស\u{17d2}ល\u{17bc}វ\u{17c9}ាគ\u{17b8}"),
        ("kn", "ಸ\u{ccd}ಲೋವಾಕ\u{cbf}ಯಾ"),
        ("ko", "슬로바키아"),
        ("ku", "Slovakya"),
        ("kv", "Словакия"),
        ("kw", "Slovaki"),
        ("ky", "Словакия"),
        ("lo", "ປະເທດສະໂລວາກ\u{eb5}"),
        ("lt", "Slovakija"),
        ("lv", "Slovākija"),
        ("mi", "Horowākia"),
        ("mk", "Словачка"),
        ("ml", "സ\u{d4d}ലോവ\u{d3e}ക\u{d4d}യ"),
        ("mn", "Словак"),
        ("mr", "स\u{94d}लोव\u{94d}हाकिया"),
        ("ms", "Slovakia"),
        ("mt", "Slovakia"),
        (
            "my",
            "ဆလ\u{102d}\u{102f}ဗားက\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Slowakia"),
        ("nb", "Slovakia"),
        ("ne", "स\u{94d}लोभाकिया"),
        ("nl", "Slowakije"),
        ("nn", "Slovakia"),
        ("nv", "Słóbaʼ Bikéyah"),
        ("oc", "Eslovaquia"),
        ("or", "ସ\u{b4d}ଲୋଭ\u{b3e}କ\u{b3f}ୟ\u{b3e}"),
        ("pa", "ਸਲ\u{a4b}ਵਾਕੀਆ"),
        ("pi", "स\u{94d}लोवाकिया"),
        ("pl", "Słowacja"),
        ("ps", "سلواکيا"),
        ("pt", "Eslováquia"),
        ("pt_BR", "Eslováquia"),
        ("ro", "Slovacia"),
        ("ru", "Словакия"),
        ("rw", "Silovakiya"),
        ("sc", "Islovàchia"),
        ("sd", "Slovakia"),
        ("si", "ස\u{dca}ලෝවැක\u{dd2}ය\u{dcf}ව"),
        ("sk", "Slovensko"),
        ("sl", "Slovaška"),
        ("so", "Slovakia"),
        ("sq", "Sllovaki"),
        ("sr", "Словачка"),
        ("sv", "Slovakien"),
        ("sw", "Slovakia"),
        ("ta", "சுலோவேக\u{bcd}கிய\u{bbe}"),
        ("te", "స\u{c4d}ల\u{c4b}వ\u{c3e}క\u{c3f}య\u{c3e}"),
        ("tg", "Словакия"),
        ("th", "สโลวะเก\u{e35}ย"),
        ("ti", "ስሎቫኪያ"),
        ("tk", "Slowak"),
        ("tl", "Slovakia"),
        ("tr", "Slovakya"),
        ("tt", "Словакиа"),
        ("ug", "سىلوۋاكىيە"),
        ("uk", "Словаччина"),
        ("ur", "سلوواکیہ"),
        ("uz", "Slovakiya"),
        ("ve", "Slovakia"),
        ("vi", "Xlô-vác"),
        ("wa", "Eslovakeye"),
        ("wo", "Eslowaaki"),
        ("xh", "Slovakia"),
        ("yo", "Slofákíà"),
        ("zh_CN", "斯洛伐克"),
        ("zh_HK", "斯洛伐克"),
        ("zh_TW", "斯洛伐克"),
        ("zu", "ISlovaki"),
    ];
    #[cfg(all(feature = "sk", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 48.669026;
        pub const LONGITUDE: f64 = 19.699024;
        pub const MAX_LATITUDE: f64 = 49.613805;
        pub const MAX_LONGITUDE: f64 = 22.5658602;
        pub const MIN_LATITUDE: f64 = 47.731159;
        pub const MIN_LONGITUDE: f64 = 16.8331821;
        pub const NORTHEAST_LATITUDE: f64 = 49.613805;
        pub const NORTHEAST_LONGITUDE: f64 = 22.5658602;
        pub const SOUTHWEST_LATITUDE: f64 = 47.731159;
        pub const SOUTHWEST_LONGITUDE: f64 = 16.8331821;
    }
}
#[cfg(all(feature = "sk", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 48.669026,
            longitude: 19.699024,
            max_latitude: 49.613805,
            max_longitude: 22.5658602,
            min_latitude: 47.731159,
            min_longitude: 16.8331821,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 49.613805,
                    longitude: 22.5658602,
                },
                southwest: CountryGeoBound {
                    latitude: 47.731159,
                    longitude: 16.8331821,
                },
            },
        }
    }
}

#[cfg(all(feature = "sk", feature = "subdivisions"))]
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
                    "BC",
                    Subdivision{
                        name: "BC",
                        country_alpha2: Alpha2::SK,
                        code: "BC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.5312499), longitude: Some(19.382874), max_latitude: Some(48.946956), min_latitude: Some(48.0536532), max_longitude: Some(20.469993), min_longitude: Some(18.4786941)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم بانسكا بيستريتسا"), ("az", "Banska Bistritsa bölgəsi"), ("be", "Банскабістрыцкі край"), ("bg", "Банскобистришки край"), ("bn", "ব\u{9be}ন\u{9cd}স\u{9be}ক\u{9be} ব\u{9be}ইস\u{9cd}ট\u{9cd}রিক\u{9be} অঞ\u{9cd}চল"), ("bs", "Banska Bistrica"), ("ca", "Regió de Banská Bystrica"), ("ccp", "𑄝𑄚\u{11134}𑄇 𑄝𑄠𑄑\u{11133}𑄢\u{11128}𑄇"), ("ceb", "Banskobystrický kraj"), ("cs", "Banskobystrický kraj"), ("da", "Banská Bystrica"), ("de", "Banskobystrický kraj"), ("el", "Μπανσκά Μπιστρίκα"), ("en", "Banská Bystrica"), ("es", "Región de Banská Bystrica"), ("et", "Banská Bystrica maakond"), ("eu", "Banská Bystrica eskualdea"), ("fa", "منطقه بانسکا بیستریتسا"), ("fi", "Banská Bystrican alue"), ("fr", "Région de Banská Bystrica"), ("gu", "બાન\u{acd}સ\u{acd}કા બાયસ\u{acd}ટ\u{acd}રીકા પ\u{acd}રદ\u{ac7}શ"), ("he", "באנסקה ביסטריצה"), ("hi", "ब\u{948}\u{902}स\u{94d}का बिस\u{94d}त\u{94d}रिका क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Banskobystrický kraj"), ("hu", "Besztercebánya megye"), ("id", "Region Banská Bystrica"), ("it", "regione di Banská Bystrica"), ("ja", "バンスカー・ビストリツァ県"), ("ka", "ბანსკა-ბისტრიცის მხარე"), ("kn", "ಬನ\u{ccd}ಸ\u{ccd}ಕಾ ಬೈಸ\u{ccd}ಟ\u{ccd}ರ\u{cbf}ಕಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "반스카비스트리차 주"), ("lt", "Banska Bistricos kraštas"), ("lv", "Banska Bistricas apgabals"), ("mr", "बा\u{902}स\u{94d}का बायाट\u{94d}रिक प\u{94d}रद\u{947}श"), ("ms", "Daerah Banská Bystrica"), ("nb", "Banská Bystrica"), ("nl", "Banská Bystrica"), ("no", "Banská Bystrica"), ("pl", "Kraj bańskobystrzycki"), ("pt", "Banská Bystrica"), ("ro", "Regiunea Banská Bystrica"), ("ru", "Банскобистрицкий край"), ("si", "බන\u{dca}ස\u{dca}ක\u{dcf} බ\u{dd2}ස\u{dca}ට\u{dca}\u{200d}ර\u{dd2}ක\u{dcf} කල\u{dcf}පය"), ("sk", "Banskobystrický kraj"), ("sr", "Банскобистрички крај"), ("sr_Latn", "Banskobistrički kraj"), ("sv", "Banská Bystrica"), ("ta", "பன\u{bcd}ஸ\u{bcd}க\u{bbe} பிஸ\u{bcd}ட\u{bcd}ரிக\u{bbe} ர\u{bc0}ஜியன\u{bcd}"), ("te", "బ\u{c3e}ంస\u{c4d}క\u{c3e} బ\u{c3f}స\u{c4d}ట\u{c4d}ర\u{c3f}క\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "บ\u{e31}นสกาบ\u{e34}สตร\u{e34}กา"), ("tr", "Banská Bystrica Bölgesi"), ("uk", "Банськобистрицький край"), ("ur", "بانسکا بسٹریکا ریجن"), ("vi", "Khu vực Banská Bystrica"), ("zh", "班斯卡·比斯特理察州")]),
                        unofficial_name_list: ["Banskobystrický kraj"].to_vec(),
                    }
                ),
                (
                    "BL",
                    Subdivision{
                        name: "BL",
                        country_alpha2: Alpha2::SK,
                        code: "BL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.3118304), longitude: Some(17.1973299), max_latitude: Some(48.6543947), min_latitude: Some(48.0066865), max_longitude: Some(17.522104), min_longitude: Some(16.833182)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم براتيسلافا"), ("az", "Bratislava bölgəsi"), ("be", "Браціслаўскі край"), ("bg", "Братиславски край"), ("bn", "ব\u{9cd}র\u{9be}তিস\u{9be}ব\u{9be} অঞ\u{9cd}চল"), ("bs", "Bratislava"), ("ca", "Regió de Bratislava"), ("ccp", "𑄝\u{11133}𑄢𑄑\u{11128}𑄌\u{11134}𑄣𑄞"), ("ceb", "Bratislavský kraj"), ("cs", "Bratislavský kraj"), ("da", "Bratislava"), ("de", "Bratislavský kraj"), ("el", "Μπρατισλάβα"), ("en", "Bratislava"), ("es", "Región de Bratislava"), ("et", "Bratislava maakond"), ("eu", "Bratislava eskualdea"), ("fa", "منطقه براتیسلاوا"), ("fi", "Bratislavan alue"), ("fr", "Région de Bratislava"), ("gu", "બ\u{acd}ર\u{ac7}ટિસ\u{acd}લાવા પ\u{acd}રદ\u{ac7}શ"), ("hi", "ब\u{94d}रातिस\u{94d}लावा प\u{94d}रद\u{947}श"), ("hr", "Bratislavský kraj"), ("hu", "Pozsony megye"), ("hy", "Բրատիսլավայի երկրամաս"), ("id", "Region Bratislava"), ("it", "regione di Bratislava"), ("ja", "ブラチスラヴァ県"), ("ka", "ბრატისლავის მხარე"), ("kn", "ಬ\u{ccd}ರಾಟ\u{cbf}ಸ\u{ccd}ಲಾವಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "브라티슬라바 주"), ("lt", "Bratislavos kraštas"), ("lv", "Bratislavas apgabals"), ("mk", "Братиславски крај"), ("mn", "Братислав аймаг"), ("mr", "ब\u{94d}र\u{945}टिस\u{94d}लावा प\u{94d}रद\u{947}श"), ("ms", "Daerah Bratislava"), ("nb", "Bratislava"), ("nl", "Bratislava"), ("no", "Bratislava"), ("pl", "Kraj bratysławski"), ("pt", "Bratislava"), ("ro", "Regiunea Bratislava"), ("ru", "Братиславский край"), ("si", "බ\u{dca}\u{200d}රට\u{dd2}ස\u{dca}ලව\u{dcf} කල\u{dcf}පය"), ("sk", "Bratislavský kraj"), ("sr", "Братиславски крај"), ("sr_Latn", "Bratislavski kraj"), ("sv", "Bratislava"), ("ta", "ப\u{bcd}ரட\u{bc0}ஸ\u{bcd}லவ\u{bbe} பகுதி"), ("te", "బ\u{c4d}ర\u{c3e}ట\u{c3f}స\u{c4d}ల\u{c3e}వ\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตบราต\u{e34}สลาวา"), ("tr", "Bratislava Bölgesi"), ("uk", "Братиславський край"), ("ur", "براتیسلاوا ریجن"), ("vi", "Bratislava"), ("zh", "布拉迪斯拉發州")]),
                        unofficial_name_list: ["Bratislavský kraj"].to_vec(),
                    }
                ),
                (
                    "KI",
                    Subdivision{
                        name: "KI",
                        country_alpha2: Alpha2::SK,
                        code: "KI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.6375737), longitude: Some(21.0834225), max_latitude: Some(49.0197507), min_latitude: Some(48.33251449999999), max_longitude: Some(22.3877253), min_longitude: Some(20.1811385)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم كوشيتسه"), ("az", "Koşitse bölgəsi"), ("be", "Кошыцкі край"), ("bg", "Кошицки край"), ("bn", "কোসিক অঞ\u{9cd}চল"), ("bs", "Košice"), ("ca", "Regió de Košice"), ("ccp", "𑄇\u{1112e}𑄥\u{1112d}𑄌\u{11134}"), ("ceb", "Košický kraj"), ("cs", "Košický kraj"), ("da", "Košice"), ("de", "Košický kraj"), ("el", "Κοσίκε"), ("en", "Košice"), ("es", "Región de Košice"), ("et", "Košice maakond"), ("eu", "Košice eskualdea"), ("fa", "منطقه کوشیتسه"), ("fi", "Košicen alue"), ("fr", "Région de Košice"), ("gu", "કોસિસ પ\u{acd}રદ\u{ac7}શ"), ("hi", "कोसिस क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Košický kraj"), ("hu", "Kassa megye"), ("id", "Region Košice"), ("it", "regione di Košice"), ("ja", "コシツェ県"), ("ka", "კოშიცეს მხარე"), ("kn", "ಕೋಸೀಸ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "코시체 주"), ("lt", "Košicės kraštas"), ("lv", "Košices apgabals"), ("mr", "कॉसिस प\u{94d}रद\u{947}श"), ("ms", "Daerah Košice"), ("nb", "Košice"), ("nl", "Košice"), ("no", "Košice"), ("pl", "Kraj koszycki"), ("pt", "Košice"), ("ro", "Regiunea Košice"), ("ru", "Кошицкий край"), ("si", "කොසය\u{dd2}ස\u{dca} කල\u{dcf}පය"), ("sk", "Košický kraj"), ("sr", "Кошички крај"), ("sr_Latn", "Košički kraj"), ("sv", "Košice"), ("ta", "கோசிஸ\u{bcd} பகுதி"), ("te", "క\u{c4b}య\u{c3f}స\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "โคไซซ\u{e35}"), ("tr", "Košice Bölgesi"), ("uk", "Кошицький край"), ("ur", "کوشیسہ علاقہ"), ("vi", "Khu vực Kosice"), ("zh", "科希策州")]),
                        unofficial_name_list: ["Košický kraj"].to_vec(),
                    }
                ),
                (
                    "NI",
                    Subdivision{
                        name: "NI",
                        country_alpha2: Alpha2::SK,
                        code: "NI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.0143765), longitude: Some(18.5416505), max_latitude: Some(48.7145252), min_latitude: Some(47.7313783), max_longitude: Some(19.0724064), min_longitude: Some(17.7072734)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم نيترا"), ("az", "Nitra bölgəsi"), ("be", "Нітранскі край"), ("bg", "Нитрански край"), ("bn", "নিট\u{9cd}র\u{9be} অঞ\u{9cd}চল"), ("bs", "Nitra"), ("ca", "Regió de Nitra"), ("ccp", "𑄚\u{11128}𑄑\u{11133}𑄢"), ("ceb", "Nitriansky kraj"), ("cs", "Nitranský kraj"), ("da", "Nitra"), ("de", "Nitriansky kraj"), ("el", "Νίτρα"), ("en", "Nitra"), ("es", "Región de Nitra"), ("et", "Nitra maakond"), ("eu", "Nitra eskualdea"), ("fa", "منطقه نیترا"), ("fi", "Nitran alue"), ("fr", "Région de Nitra"), ("gu", "નિટ\u{acd}રા પ\u{acd}રદ\u{ac7}શ"), ("hi", "निट\u{94d}रा क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Nitriansky kraj"), ("hu", "Nyitra megye"), ("id", "Region Nitra"), ("it", "regione di Nitra"), ("ja", "ニトラ県"), ("ka", "ნიტრის მხარე"), ("kn", "ನೈಟ\u{ccd}ರಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "니트라 주"), ("lt", "Nitros kraštas"), ("lv", "Nitras apgabals"), ("mr", "निट\u{94d}रा प\u{94d}रद\u{947}श"), ("ms", "Daerah Nitra"), ("nb", "Nitra"), ("nl", "Nitra"), ("no", "Nitra"), ("pl", "Kraj nitrzański"), ("pt", "Nitra"), ("ro", "Regiunea Nitra"), ("ru", "Нитранский край"), ("si", "න\u{dd2}ට\u{dca}\u{200d}ර\u{dcf} කල\u{dcf}පය"), ("sk", "Nitriansky kraj"), ("sr", "Њитрански крај"), ("sr_Latn", "Njitranski kraj"), ("sv", "Nitra"), ("ta", "நித\u{bcd}ர\u{bbe} பகுதி"), ("te", "న\u{c3f}ట\u{c4d}ర\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตน\u{e34}ตรา"), ("tr", "Nitra Bölgesi"), ("uk", "Нітранський край"), ("ur", "نیترا علاقہ"), ("vi", "Nitra"), ("zh", "尼特拉州")]),
                        unofficial_name_list: ["Nitriansky kraj"].to_vec(),
                    }
                ),
                (
                    "PV",
                    Subdivision{
                        name: "PV",
                        country_alpha2: Alpha2::SK,
                        code: "PV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.1716773), longitude: Some(21.3742001), max_latitude: Some(49.4608248), min_latitude: Some(48.762623), max_longitude: Some(22.5658602), min_longitude: Some(19.8690332)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم بريشوف"), ("az", "Preşov bölgəsi"), ("be", "Прэшаўскі край"), ("bg", "Прешовски край"), ("bn", "প\u{9cd}রেসো অঞ\u{9cd}চল"), ("bs", "Prešov"), ("ca", "Regió de Prešov"), ("ccp", "𑄛\u{11133}𑄢𑄬𑄥\u{1112e}𑄛\u{11134}"), ("ceb", "Prešovský kraj"), ("cs", "Prešovský kraj"), ("da", "Prešov"), ("de", "Prešovský kraj"), ("el", "Πρεσόβ"), ("en", "Prešov"), ("es", "Región de Prešov"), ("et", "Prešovi maakond"), ("eu", "Prešov eskualdea"), ("fa", "منطقه پرشوف"), ("fi", "Prešovin alue"), ("fr", "Région de Prešov"), ("gu", "પ\u{acd}ર\u{ac7}સોવ પ\u{acd}રદ\u{ac7}શ"), ("he", "מחוז פרשוב"), ("hi", "प\u{94d}रीसोव क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Prešovský kraj"), ("hu", "Eperjes megye"), ("id", "Region Prešov"), ("it", "regione di Prešov"), ("ja", "プレショウ県"), ("ka", "პრეშოვის მხარე"), ("kn", "ಪ\u{ccd}ರ\u{cc6}ಸೋವ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "프레쇼프 주"), ("lt", "Prešovo kraštas"), ("lv", "Prešovas apgabals"), ("mr", "प\u{94d}रीसोव प\u{94d}रद\u{947}श"), ("ms", "Wilayah Prešov"), ("nb", "Prešov"), ("nl", "Prešov"), ("no", "Prešov"), ("pl", "Kraj preszowski"), ("pt", "Prešov"), ("ro", "Regiunea Prešov"), ("ru", "Прешовский край"), ("si", "ප\u{dca}රෙසොව\u{dca} කල\u{dcf}පය"), ("sk", "Prešovský kraj"), ("sr", "Прешовски крај"), ("sr_Latn", "Prešovski kraj"), ("sv", "Prešov"), ("ta", "ப\u{bcd}ரேஸோவ\u{bcd} பகுதி"), ("te", "ప\u{c4d}ర\u{c46}స\u{c4b}వ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แคว\u{e49}นเพรชอฟ"), ("tr", "Prešov Bölgesi"), ("uk", "Пряшівський край"), ("ur", "پریسوو ریجن"), ("vi", "Vùng Prešov"), ("zh", "普列索夫州")]),
                        unofficial_name_list: ["Prešovský kraj"].to_vec(),
                    }
                ),
                (
                    "TA",
                    Subdivision{
                        name: "TA",
                        country_alpha2: Alpha2::SK,
                        code: "TA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.3943898), longitude: Some(17.7216205), max_latitude: Some(48.8782091), min_latitude: Some(47.7574508), max_longitude: Some(17.9854498), min_longitude: Some(16.933916)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم ترنافا"), ("az", "Trnava bölgəsi"), ("be", "Трнаўскі край"), ("bg", "Търнавски край"), ("bn", "ত\u{9be}ন\u{9be}ভ\u{9be} অঞ\u{9cd}চল"), ("bs", "Trnava"), ("ca", "Regió de Trnava"), ("ccp", "𑄑\u{11133}𑄢𑄚𑄞"), ("ceb", "Trnavský kraj"), ("cs", "Trnavský kraj"), ("da", "Trnava"), ("de", "Trnavský kraj"), ("el", "Τρνάβα"), ("en", "Trnava"), ("es", "Región de Trnava"), ("et", "Trnava maakond"), ("eu", "Trnava eskualdea"), ("fa", "منطقه ترناوا"), ("fi", "Trnavan alue"), ("fr", "Région de Trnava"), ("gu", "ટર\u{acd}નાવા પ\u{acd}રદ\u{ac7}શ"), ("he", "מחוז טרנבה"), ("hi", "ट\u{94d}रनवा क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Trnavský kraj"), ("hu", "Nagyszombat megye"), ("hy", "Տրնավայի երկրամաս"), ("id", "Region Trnava"), ("it", "regione di Trnava"), ("ja", "トルナヴァ県"), ("ka", "ტრნავის მხარე"), ("kn", "ಟ\u{ccd}ರಾನ\u{ccd}ವಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "트르나바 주"), ("lt", "Trnavos kraštas"), ("lv", "Trnavas apgabals"), ("mn", "Трнав аймаг"), ("mr", "ट\u{94d}र\u{94d}नव\u{94d}हा प\u{94d}रद\u{947}श"), ("ms", "Daerah Trnava"), ("nb", "Trnava"), ("nl", "Trnava"), ("no", "Trnava"), ("pl", "Kraj trnawski"), ("pt", "Trnava"), ("ro", "Regiunea Trnava"), ("ru", "Трнавский край"), ("si", "ට\u{dca}\u{200d}ර\u{dca}නව\u{dcf} කල\u{dcf}පය"), ("sk", "Trnavský kraj"), ("sr", "Трнавски крај"), ("sr_Latn", "Trnavski kraj"), ("sv", "Trnava"), ("ta", "ட\u{bcd}றனவ\u{bbe} பகுதி"), ("te", "ట\u{c4d}రన\u{c3e}వ\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เตอร\u{e4c}นาวา"), ("tr", "Trnava Bölgesi"), ("uk", "Трнавський край"), ("ur", "ترناوا ریجن"), ("vi", "Trnava"), ("zh", "特爾納瓦州")]),
                        unofficial_name_list: ["Trnavský kraj"].to_vec(),
                    }
                ),
                (
                    "TC",
                    Subdivision{
                        name: "TC",
                        country_alpha2: Alpha2::SK,
                        code: "TC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.8086758), longitude: Some(18.2324027), max_latitude: Some(49.31953009999999), min_latitude: Some(48.4855226), max_longitude: Some(18.8268433), min_longitude: Some(17.3541989)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم ترنتشين"), ("az", "Trençin bölgəsi"), ("be", "Трэнчынскі край"), ("bg", "Тренчински край"), ("bn", "ট\u{9cd}রেঙ\u{9cd}কিন অঞ\u{9cd}চল"), ("bs", "Trenčin"), ("ca", "Regió de Trenčín"), ("ccp", "𑄑\u{11133}𑄢𑄬𑄚\u{11134}𑄥\u{11128}𑄚\u{11134}"), ("ceb", "Trenčiansky kraj"), ("cs", "Trenčínský kraj"), ("da", "Trenčín"), ("de", "Trenčiansky kraj"), ("el", "Τρένκιν"), ("en", "Trenčín"), ("es", "Región de Trenčín"), ("et", "Trenčíni maakond"), ("eu", "Trenčín eskualdea"), ("fa", "منطقه ترنچین"), ("fi", "Trenčínin alue"), ("fr", "Région de Trenčín"), ("gu", "ટ\u{acd}ર\u{ac7}નસિન પ\u{acd}રદ\u{ac7}શ"), ("he", "מחוז טרנצ׳ין"), ("hi", "ट\u{94d}र\u{947}न\u{94d}सिन क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Trenčiansky kraj"), ("hu", "Trencsén megye"), ("hy", "Տենչինի երկրամաս"), ("id", "Region Trenčín"), ("it", "regione di Trenčín"), ("ja", "トレンチーン県"), ("ka", "ტრენჩინის მხარე"), ("kn", "ಟ\u{ccd}ರ\u{cc6}ನ\u{ccd}ಸ\u{cbf}ನ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "트렌친 주"), ("lt", "Trenčyno kraštas"), ("lv", "Trenčīnas apgabals"), ("mr", "ट\u{94d}र\u{947}\u{902}टिन प\u{94d}रद\u{947}श"), ("ms", "Daerah Trenčín"), ("nb", "Trenčín"), ("nl", "Trenčín"), ("no", "Trenčín"), ("pl", "Kraj trenczyński"), ("pt", "Trenčín"), ("ro", "Regiunea Trenčín"), ("ru", "Тренчинский край"), ("si", "ට\u{dca}\u{200d}රන\u{dca}ස\u{dd2}න\u{dca} කල\u{dcf}පය"), ("sk", "Trenčiansky kraj"), ("sr", "Тренчински крај"), ("sr_Latn", "Trenčinski kraj"), ("sv", "Trenčín"), ("ta", "ட\u{bcd}ரெஸின\u{bcd} பகுதி"), ("te", "ట\u{c4d}ర\u{c46}న\u{c4d}స\u{c3f}న\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตเทรนซ\u{e34}น"), ("tr", "Trenčín Bölgesi"), ("uk", "Тренчинський край"), ("ur", "ترینکین ریجن"), ("vi", "Trenčín"), ("zh", "特倫欽州")]),
                        unofficial_name_list: ["Trenciansky kraj"].to_vec(),
                    }
                ),
                (
                    "ZI",
                    Subdivision{
                        name: "ZI",
                        country_alpha2: Alpha2::SK,
                        code: "ZI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.2031435), longitude: Some(19.3645733), max_latitude: Some(49.6138171), min_latitude: Some(48.7412625), max_longitude: Some(20.0600755), min_longitude: Some(18.3226616)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم جيلينا"), ("az", "Jilina bölgəsi"), ("be", "Жылінскі край"), ("bg", "Жилински край"), ("bn", "জিলিন\u{9be} অঞ\u{9cd}চল"), ("bs", "Žilina"), ("ca", "Regió de Žilina"), ("ccp", "𑄎\u{11128}𑄣\u{11128}𑄚"), ("ceb", "Žilinský kraj"), ("cs", "Žilinský kraj"), ("da", "Žilina"), ("de", "Žilinský kraj"), ("el", "Επαρχία Ζιλίνα"), ("en", "Žilina"), ("es", "Región de Žilina"), ("et", "Žilina maakond"), ("eu", "Žilina eskualdea"), ("fa", "منطقه ژیلینا"), ("fi", "Žilinan alue"), ("fr", "Région de Žilina"), ("gu", "ઝીલીના પ\u{acd}રદ\u{ac7}શ"), ("he", "מחוז ז׳ילינה"), ("hi", "ज\u{93c}िलिना प\u{94d}रद\u{947}श"), ("hr", "Žilinský kraj"), ("hu", "Zsolna megye"), ("id", "Region Žilina"), ("it", "regione di Žilina"), ("ja", "ジリナ県"), ("ka", "ჟილინის მხარე"), ("kn", "ಝ\u{cbf}ಲ\u{cbf}ನಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "질리나 주"), ("lt", "Žilinos kraštas"), ("lv", "Žilinas apgabals"), ("mn", "Жилин аймаг"), ("mr", "जिल\u{947}टिना प\u{94d}रद\u{947}श"), ("ms", "Daerah Žilina"), ("nb", "Žilina"), ("nl", "Žilina"), ("no", "Žilina"), ("pl", "Kraj żyliński"), ("pt", "Žilina"), ("ro", "Regiunea Žilina"), ("ru", "Жилинский край"), ("si", "ස\u{dd2}ල\u{dd2}න\u{dcf} කල\u{dcf}පය"), ("sk", "Žilinský kraj"), ("sr", "Жилински крај"), ("sr_Latn", "Žilinski kraj"), ("sv", "Žilina"), ("ta", "ஜில\u{bc0}ன\u{bbe} பகுதி"), ("te", "జ\u{c3f}ల\u{c3f}న\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตซ\u{e34}ล\u{e34}นา"), ("tr", "Žilina Bölgesi"), ("uk", "Жилінський край"), ("ur", "ژیلینا علاقہ"), ("vi", "Khu vực Zilina"), ("zh", "日利納州")]),
                        unofficial_name_list: ["Žilinský kraj"].to_vec(),
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
#[cfg(feature = "sk")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::SK,
        alpha3: Alpha3::SVK,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 421,
        currency_code: "EUR",
        gec: Some(GEC::LO),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("SVK"),
        iso_long_name: "The Slovak Republic",
        iso_short_name: "Slovakia",
        official_language_list: ["sk"].to_vec(),
        spoken_language_list: ["sk"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "0",
        nationality: Some("Slovak"),
        number: "703",
        postal_code: true,
        postal_code_format: Some("\\d{3} ?\\d{2}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternEurope),
        un_locode: "SK",
        unofficial_name_list: [
            "Slovakia",
            "Slowakei",
            "Slovaquie",
            "República Eslovaca",
            "スロバキア",
            "Slowakije",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Slovakia"),
            ("af", "Slowakye"),
            ("ak", "Slovakia"),
            ("am", "ስሕቲጡ።"),
            ("an", "Slovakia"),
            ("ar", "سلوفاكيا"),
            ("as", "স\u{9cd}লোভ\u{9be}কিয়\u{9be}"),
            ("ay", "Slovakia"),
            ("az", "Slovakiya"),
            ("ba", "Slovakia"),
            ("be", "Славакія"),
            ("bg", "Словакия"),
            ("bi", "Slovakia"),
            ("bn", "স\u{9cd}লোভ\u{9be}কিয়\u{9be}"),
            ("bn_IN", "স\u{9cd}লোভ\u{9be}কিয়\u{9be}"),
            ("br", "Slovakia"),
            ("bs", "Slovačka"),
            ("ca", "Eslovàquia"),
            ("ce", "Словаки"),
            ("ch", "Slovakia"),
            ("cs", "Slovensko"),
            ("cv", "Словаки"),
            ("cy", "Slofacia"),
            ("da", "Slovakiet"),
            ("de", "Slowakei"),
            ("dv", "ސ\u{7aa}ލ\u{7ae}ވ\u{7a7}ކ\u{7a8}އ\u{7a7}"),
            ("dz", "ས\u{f7c}ལ\u{f7c}་བ་ཀ\u{f72}་ཡ།"),
            ("ee", "Slovakia"),
            ("el", "Σλοβακία"),
            ("en", "Slovakia"),
            ("eo", "Slovakio"),
            ("es", "Eslovaquia"),
            ("et", "Slovakkia"),
            ("eu", "Eslovakia"),
            ("fa", "اسلواکی"),
            ("ff", "Sulowakiya"),
            ("fi", "Slovakia"),
            ("fo", "Slovakia"),
            ("fr", "Slovaquie"),
            ("fy", "Slowakije"),
            ("ga", "An tSlóvaic"),
            ("gl", "Eslovaquia"),
            ("gn", "Slovakia"),
            ("gu", "સ\u{acd}લોવ\u{ac7}કિઆ"),
            ("gv", "Yn Clovack"),
            ("ha", "Slofakiya"),
            ("he", "סלובקיה"),
            ("hi", "स\u{94d}लोवाकिया"),
            ("hr", "Slovačka"),
            ("ht", "Slovaki"),
            ("hu", "Szlovákia"),
            ("hy", "Սլովակիա"),
            ("ia", "Slovachia"),
            ("id", "Slowakia"),
            ("io", "Slovakia"),
            ("is", "Slóvakía"),
            ("it", "Slovacchia"),
            ("iu", "Slovakia"),
            ("ja", "スロバキア"),
            ("ka", "სლოვაკეთი"),
            ("ki", "Slovakia"),
            ("kk", "Словакия"),
            ("kl", "Slovakia"),
            ("km", "ស\u{17d2}ល\u{17bc}វ\u{17c9}ាគ\u{17b8}"),
            ("kn", "ಸ\u{ccd}ಲೋವಾಕ\u{cbf}ಯಾ"),
            ("ko", "슬로바키아"),
            ("ku", "Slovakya"),
            ("kv", "Словакия"),
            ("kw", "Slovaki"),
            ("ky", "Словакия"),
            ("lo", "ປະເທດສະໂລວາກ\u{eb5}"),
            ("lt", "Slovakija"),
            ("lv", "Slovākija"),
            ("mi", "Horowākia"),
            ("mk", "Словачка"),
            ("ml", "സ\u{d4d}ലോവ\u{d3e}ക\u{d4d}യ"),
            ("mn", "Словак"),
            ("mr", "स\u{94d}लोव\u{94d}हाकिया"),
            ("ms", "Slovakia"),
            ("mt", "Slovakia"),
            (
                "my",
                "ဆလ\u{102d}\u{102f}ဗားက\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Slowakia"),
            ("nb", "Slovakia"),
            ("ne", "स\u{94d}लोभाकिया"),
            ("nl", "Slowakije"),
            ("nn", "Slovakia"),
            ("nv", "Słóbaʼ Bikéyah"),
            ("oc", "Eslovaquia"),
            ("or", "ସ\u{b4d}ଲୋଭ\u{b3e}କ\u{b3f}ୟ\u{b3e}"),
            ("pa", "ਸਲ\u{a4b}ਵਾਕੀਆ"),
            ("pi", "स\u{94d}लोवाकिया"),
            ("pl", "Słowacja"),
            ("ps", "سلواکيا"),
            ("pt", "Eslováquia"),
            ("pt_BR", "Eslováquia"),
            ("ro", "Slovacia"),
            ("ru", "Словакия"),
            ("rw", "Silovakiya"),
            ("sc", "Islovàchia"),
            ("sd", "Slovakia"),
            ("si", "ස\u{dca}ලෝවැක\u{dd2}ය\u{dcf}ව"),
            ("sk", "Slovensko"),
            ("sl", "Slovaška"),
            ("so", "Slovakia"),
            ("sq", "Sllovaki"),
            ("sr", "Словачка"),
            ("sv", "Slovakien"),
            ("sw", "Slovakia"),
            ("ta", "சுலோவேக\u{bcd}கிய\u{bbe}"),
            ("te", "స\u{c4d}ల\u{c4b}వ\u{c3e}క\u{c3f}య\u{c3e}"),
            ("tg", "Словакия"),
            ("th", "สโลวะเก\u{e35}ย"),
            ("ti", "ስሎቫኪያ"),
            ("tk", "Slowak"),
            ("tl", "Slovakia"),
            ("tr", "Slovakya"),
            ("tt", "Словакиа"),
            ("ug", "سىلوۋاكىيە"),
            ("uk", "Словаччина"),
            ("ur", "سلوواکیہ"),
            ("uz", "Slovakiya"),
            ("ve", "Slovakia"),
            ("vi", "Xlô-vác"),
            ("wa", "Eslovakeye"),
            ("wo", "Eslowaaki"),
            ("xh", "Slovakia"),
            ("yo", "Slofákíà"),
            ("zh_CN", "斯洛伐克"),
            ("zh_HK", "斯洛伐克"),
            ("zh_TW", "斯洛伐克"),
            ("zu", "ISlovaki"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
