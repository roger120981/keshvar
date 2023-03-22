// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The French Republic

#[cfg(all(feature = "fr", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::FR;
    pub const ALPHA3: Alpha3 = Alpha3::FRA;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 33;
    pub const CURRENCY_CODE: &str = "EUR";
    pub const GEC: Option<GEC> = Some(GEC::FR);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("FRA");
    pub const ISO_SHORT_NAME: &str = "France";
    pub const ISO_LONG_NAME: &str = "The French Republic";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["fr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["fr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[1];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9, 10];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("French");
    pub const NUMBER: &str = "250";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{2} ?\\d{3}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternEurope);
    pub const UN_LOCODE: &str = "FR";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "France",
        "Frankreich",
        "the French Republic",
        "フランス",
        "Frankrijk",
        "Francia",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "France"),
        ("af", "Frankryk"),
        ("ak", "France"),
        ("am", "France"),
        ("an", "France"),
        ("ar", "فرنسا"),
        ("as", "ফ\u{9cd}ইচ\u{9cd}\u{200c}ল\u{9be}মিক\u{9be}ন\u{9cd}স"),
        ("ay", "France"),
        ("az", "Fransa"),
        ("ba", "France"),
        ("be", "Францыя"),
        ("bg", "Франция"),
        ("bi", "France"),
        ("bn", "ফ\u{9cd}র\u{9be}ন\u{9cd}স"),
        ("bn_IN", "ফ\u{9cd}র\u{9be}ন\u{9cd}স"),
        ("br", "Frañs"),
        ("bs", "Francuska"),
        ("ca", "França"),
        ("ce", "Франци"),
        ("ch", "Francia"),
        ("cs", "Francie"),
        ("cv", "Франци"),
        ("cy", "Ffrainc"),
        ("da", "Frankrig"),
        ("de", "Frankreich"),
        (
            "dv",
            "ފ\u{7a6}ރ\u{7a6}ނ\u{7b0}ސ\u{7ad}ސ\u{7a8}ވ\u{7a8}ލ\u{7a7}ތ\u{7b0}",
        ),
        ("dz", "ཕར\u{f71}ནས\u{f72}།"),
        ("ee", "France"),
        ("el", "Γαλλία"),
        ("en", "France"),
        ("eo", "Francio"),
        ("es", "Francia"),
        ("et", "Prantsusmaa"),
        ("eu", "Frantzia"),
        ("fa", "فرانسه"),
        ("ff", "Faransi"),
        ("fi", "Ranska"),
        ("fo", "Frakland"),
        ("fr", "France"),
        ("fy", "Frankryk"),
        ("ga", "An Fhrainc"),
        ("gl", "Francia"),
        ("gn", "France"),
        ("gu", "ફ\u{acd}રાન\u{acd}સ"),
        ("gv", "Yn Rank"),
        ("ha", "Faransa"),
        ("he", "צרפת"),
        ("hi", "फ\u{93c}\u{94d}रान\u{94d}स"),
        ("hr", "Francuska"),
        ("ht", "Frans"),
        ("hu", "Franciaország"),
        ("hy", "Ֆրանսիա"),
        ("ia", "Francia"),
        ("id", "Perancis"),
        ("io", "Francia"),
        ("is", "Frakkland"),
        ("it", "Francia"),
        ("iu", "France"),
        ("ja", "フランス"),
        ("ka", "საფრანგეთი"),
        ("ki", "Baranja"),
        ("kk", "Франция"),
        ("kl", "France"),
        ("km", "បារា\u{17c6}ង"),
        ("kn", "ಫ\u{ccd}ರಾನ\u{ccd}ಸ\u{ccd}"),
        ("ko", "프랑스"),
        ("ku", "Fransa"),
        ("kv", "Франция"),
        ("kw", "Pow Frynk"),
        ("ky", "Франция"),
        ("lo", "ປະເທດຝະລ\u{eb1}\u{ec8}ງ"),
        ("lt", "Prancūzija"),
        ("lv", "Francija"),
        ("mi", "Wīwī"),
        ("mk", "Франција"),
        ("ml", "ഫ\u{d4d}ര\u{d3e}ന\u{d4d}\u{200d}സ\u{d4d}"),
        ("mn", "Франц"),
        ("mr", "फ\u{94d}रान\u{94d}स"),
        ("ms", "Perancis"),
        ("mt", "Franza"),
        (
            "my",
            "ပြင\u{103a}သစ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Prant"),
        ("nb", "Frankrike"),
        ("ne", "फ\u{94d}रान\u{94d}स"),
        ("nl", "Frankrijk"),
        ("nn", "Frankrike"),
        ("nv", "Dáághahii Dinéʼiʼ Bikéyah"),
        ("oc", "França"),
        ("or", "ଫ\u{b4d}ର\u{b3e}ନ\u{b4d}ସ"),
        ("pa", "ਫਰਾ\u{a02}ਸ"),
        ("pi", "फ\u{94d}रा\u{902}स"),
        ("pl", "Francja"),
        ("ps", "فرانسه"),
        ("pt", "França"),
        ("pt_BR", "França"),
        ("ro", "Franța"),
        ("ru", "Франция"),
        ("rw", "Ubufaransa"),
        ("sc", "Frantza"),
        ("sd", "فرانس"),
        ("si", "ප\u{dca}\u{200d}රංශය"),
        ("sk", "Francúzsko"),
        ("sl", "Francija"),
        ("so", "Faransiis"),
        ("sq", "Francë"),
        ("sr", "Француска"),
        ("sv", "Frankrike"),
        ("sw", "Ufaransa"),
        ("ta", "ஃப\u{bcd}ர\u{bbe}ன\u{bcd}ஸ\u{bcd}"),
        ("te", "ఫ\u{c4d}ర\u{c3e}న\u{c4d}స\u{c4d}"),
        ("tg", "Фаронса"),
        ("th", "ฝร\u{e31}\u{e48}งเศส"),
        ("ti", "ፈረንሳ"),
        ("tk", "Fransiýa"),
        ("tl", "Pransya"),
        ("tr", "Fransa"),
        ("tt", "Франсиа"),
        ("ug", "فىرانسىيە"),
        ("uk", "Франція"),
        ("ur", "فرانس"),
        ("uz", "Fransiya"),
        ("ve", "Fura"),
        ("vi", "Pháp"),
        ("wa", "France"),
        ("wo", "Faraas"),
        ("xh", "Franisi"),
        ("yo", "Fránsì"),
        ("zh_CN", "法国"),
        ("zh_HK", "法國"),
        ("zh_TW", "法國"),
        ("zu", "IFulansi"),
    ];
    #[cfg(all(feature = "fr", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 46.227638;
        pub const LONGITUDE: f64 = 2.213749;
        pub const MAX_LATITUDE: f64 = 51.1241999;
        pub const MAX_LONGITUDE: f64 = 9.6624999;
        pub const MIN_LATITUDE: f64 = 41.31433;
        pub const MIN_LONGITUDE: f64 = -5.5591;
        pub const NORTHEAST_LATITUDE: f64 = 51.1241999;
        pub const NORTHEAST_LONGITUDE: f64 = 9.6624999;
        pub const SOUTHWEST_LATITUDE: f64 = 41.31433;
        pub const SOUTHWEST_LONGITUDE: f64 = -5.5591;
    }
}
#[cfg(all(feature = "fr", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 46.227638,
            longitude: 2.213749,
            max_latitude: 51.1241999,
            max_longitude: 9.6624999,
            min_latitude: 41.31433,
            min_longitude: -5.5591,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 51.1241999,
                    longitude: 9.6624999,
                },
                southwest: CountryGeoBound {
                    latitude: 41.31433,
                    longitude: -5.5591,
                },
            },
        }
    }
}

#[cfg(all(feature = "fr", feature = "subdivisions"))]
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
                    "01",
                    Subdivision{
                        name: "01",
                        country_alpha2: Alpha2::FR,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.2475706), longitude: Some(5.1307681), max_latitude: Some(46.519953), min_latitude: Some(45.611093), max_longitude: Some(6.170198099999999), min_longitude: Some(4.728066999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ain"), ("ar", "أين"), ("az", "En (departament)"), ("be", "Дэпартамент Эн"), ("bg", "Ен"), ("bn", "আইন"), ("ca", "Ain"), ("ccp", "𑄃\u{1112d}𑄚\u{11134}"), ("ceb", "Ain"), ("cs", "Ain"), ("cy", "Ain"), ("da", "Ain"), ("de", "Département Ain"), ("el", "Αν"), ("en", "Ain"), ("es", "Ain"), ("et", "Aini departemang"), ("eu", "Ain"), ("fa", "ان"), ("fi", "Ain"), ("fr", "Ain"), ("gl", "Ain"), ("gu", "ઐન"), ("he", "אן"), ("hi", "एन"), ("hu", "Ain"), ("hy", "Էն"), ("id", "Ain"), ("it", "Ain"), ("ja", "アン県"), ("jv", "Ain"), ("ka", "ენი"), ("kk", "Эн"), ("kn", "ಐನ\u{ccd}"), ("ko", "앵 주"), ("ky", "Эн"), ("lt", "Enas"), ("lv", "Ēna"), ("mr", "एन, फ\u{94d}रान\u{94d}स"), ("ms", "Ain"), ("nb", "Ain"), ("nl", "Ain"), ("no", "Ain"), ("pl", "Ain"), ("pt", "Ain"), ("ro", "Ain"), ("ru", "Эн"), ("si", "අය\u{dd2}න\u{dca}"), ("sk", "Ain"), ("sl", "Ain"), ("sq", "Ain"), ("sr", "Ен"), ("sr_Latn", "En"), ("sv", "Ain"), ("sw", "Ain"), ("ta", "ஆயின\u{bcd}"), ("te", "ఎయ\u{c3f}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแอ\u{e47}ง"), ("tr", "Ain"), ("uk", "Ен"), ("ur", "ان، فرانس"), ("vi", "Ain"), ("yue", "安"), ("yue_Hans", "安"), ("zh", "安省")]),
                        unofficial_name_list: ["Ain"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::FR,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.4769199), longitude: Some(3.4417368), max_latitude: Some(50.0694951), min_latitude: Some(48.837212), max_longitude: Some(4.255678899999999), min_longitude: Some(2.9582769)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Aisne"), ("ar", "أيسن"), ("az", "Ena departamenti"), ("be", "Дэпартамент Эна"), ("bg", "Ен²"), ("bn", "আইস\u{9cd}নে"), ("ca", "Aisne"), ("ccp", "𑄃\u{1112d}𑄌\u{11134}𑄚𑄬"), ("ceb", "Aisne"), ("cs", "Aisne"), ("cy", "Aisne"), ("da", "Aisne"), ("de", "Département Aisne"), ("el", "Αιν"), ("en", "Aisne"), ("es", "Aisne"), ("et", "Aisne’i departemang"), ("eu", "Aisne"), ("fa", "انه"), ("fi", "Aisne"), ("fr", "Aisne"), ("gl", "Aisne"), ("gu", "એશન\u{ac7}"), ("he", "אן²"), ("hi", "एय\u{947}न"), ("hu", "Aisne"), ("hy", "Էնա"), ("id", "Aisne"), ("it", "Aisne"), ("ja", "エーヌ県"), ("jv", "Aisne"), ("ka", "ენა"), ("kk", "Эна"), ("kn", "ಐಸ\u{ccd}ನ\u{cc6}"), ("ko", "엔 주"), ("lt", "Ena"), ("lv", "Ēna²"), ("mk", "Ена"), ("mr", "अएन"), ("ms", "Aisne"), ("nb", "Aisne"), ("nl", "Aisne"), ("no", "Aisne"), ("pl", "Aisne"), ("pt", "Aisne"), ("ro", "Aisne"), ("ru", "Эна"), ("si", "අය\u{dd2}ස\u{dca}නේ"), ("sk", "Aisne"), ("sl", "Aisne"), ("sq", "Aisne"), ("sr", "Ен²"), ("sr_Latn", "En²"), ("sv", "Aisne"), ("sw", "Aisne"), ("ta", "ஐஸின\u{bcd}"), ("te", "ఏయ\u{c3f}స\u{c4d}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแอน"), ("tr", "Aisne"), ("uk", "Ена"), ("ur", "اینہ"), ("vi", "Aisne"), ("yue", "埃納"), ("yue_Hans", "埃纳"), ("zh", "埃纳省")]),
                        unofficial_name_list: ["Aisne"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::FR,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3115552), longitude: Some(3.4167655), max_latitude: Some(46.804293), min_latitude: Some(45.9307329), max_longitude: Some(4.0057391), min_longitude: Some(2.2767951)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Allier"), ("ar", "أليي"), ("az", "Alye"), ("be", "Дэпартамент Алье"), ("bg", "Алие"), ("bn", "আলিয\u{9bc}ের"), ("ca", "Alier"), ("ccp", "𑄃𑄣\u{11133}𑄦\u{11128}𑄠𑄢\u{11134}"), ("ceb", "Allier"), ("cs", "Allier"), ("cy", "Allier"), ("da", "Allier"), ("de", "Département Allier"), ("el", "Αλλιέ"), ("en", "Allier"), ("es", "Allier"), ("et", "Allier’ departemang"), ("eu", "Allier"), ("fa", "آلیه"), ("fi", "Allier"), ("fr", "Allier"), ("gl", "Allier"), ("gu", "એલિયર"), ("he", "אלייה"), ("hi", "एलियर"), ("hu", "Allier"), ("hy", "Ալյե"), ("id", "Allier"), ("it", "Allier"), ("ja", "アリエ県"), ("jv", "Allier"), ("ka", "ალე"), ("kk", "Алье"), ("kn", "ಅಲ\u{cbf}ಯರ\u{ccd}"), ("ko", "알리에 주"), ("lt", "Aljė"), ("lv", "Aljē"), ("mr", "आल\u{94d}य\u{947}"), ("ms", "Allier"), ("nb", "Allier"), ("nl", "Allier"), ("no", "Allier"), ("os", "Alèir"), ("pl", "Allier"), ("pt", "Allier"), ("ro", "Allier"), ("ru", "Алье"), ("si", "අල\u{dca}ල\u{dd3}ර\u{dca}"), ("sk", "Allier"), ("sl", "Allier"), ("sq", "Allier"), ("sr", "Алије"), ("sr_Latn", "Alije"), ("sv", "Allier"), ("sw", "Allier"), ("ta", "அள\u{bcd}ளியர\u{bcd}"), ("te", "ఆల\u{c3f}యర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาล\u{e35}เย"), ("tr", "Allier"), ("uk", "Альє"), ("ur", "الیے"), ("vi", "Allier"), ("yue", "阿列"), ("yue_Hans", "阿列"), ("zh", "阿列省")]),
                        unofficial_name_list: ["Allier"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::FR,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.07787159999999), longitude: Some(6.2375947), max_latitude: Some(44.6599989), min_latitude: Some(43.6683251), max_longitude: Some(6.969039), min_longitude: Some(5.496792999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Alpes-de-Haute-Provence"), ("ar", "ألب البروفنس العليا"), ("az", "Yuxarı Provans Alpları (departament)"), ("be", "Альпы Верхняга Праванса"), ("bg", "Алп дьо От Прованс"), ("bn", "অ\u{9cd}য\u{9be}সপলস দি হ\u{9be}উতে প\u{9cd}রদেশ"), ("ca", "Alps de l’Alta Provença"), ("ccp", "𑄃\u{11127}𑄣\u{11134}𑄛𑄬𑄌\u{11134}-𑄓𑄬-𑄦𑄅\u{1112a}𑄖\u{11134}-𑄛\u{11133}𑄢\u{11127}𑄞𑄬𑄚\u{11134}𑄌\u{11134}"), ("ceb", "Alpes-de-Haute-Provence"), ("cs", "Alpes-de-Haute-Provence"), ("cy", "Alpes-de-Haute-Provence"), ("da", "Alpes-de-Haute-Provence"), ("de", "Alpes-de-Haute-Provence"), ("el", "Αλπ-ντε-Ωτ-Προβάνς"), ("en", "Alpes-de-Haute-Provence"), ("es", "Alpes de Alta Provenza"), ("et", "Alpes-de-Haute-Provence"), ("eu", "Alpes-de-Haute-Provence"), ("fa", "آلپ-دو-اوت-پرووانس"), ("fi", "Alpes-de-Haute-Provence"), ("fr", "Alpes-de-Haute-Provence"), ("gl", "Alpes da Alta Provenza"), ("gu", "આલ\u{acd}પ\u{acd}સ-દ\u{ac7}-હૌટ-પ\u{acd}રોવ\u{ac7}ન\u{acd}સ"), ("he", "האלפים של פרובאנס עילית"), ("hi", "एल\u{94d}प\u{947}स-डी-ओट-प\u{94d}रोव\u{947}\u{902}स"), ("hu", "Alpes-de-Haute-Provence"), ("hy", "Վերին Ալպերի Պրովանս"), ("id", "Alpes-de-Haute-Provence"), ("it", "Alpi dell’Alta Provenza"), ("ka", "ზემო პროვანსის ალპები"), ("kk", "Жоғарғы Прованс Альпілері"), ("kn", "ಆಲ\u{ccd}ಪ\u{cc6}ಸ\u{ccd}-ಡ\u{cbf}-ಹಾಟ\u{cc6}-ಪ\u{ccd}ರೊವ\u{cc6}ನ\u{ccd}ಸ\u{ccd}"), ("ko", "알프드오트프로방스 주"), ("lt", "Aukštutinio Provanso Alpės"), ("lv", "Augšprovansas Alpi"), ("mk", "Горнопровансалски Алпи"), ("mr", "आल\u{94d}प-दा-ऑत-प\u{94d}रोव\u{94d}हा\u{901}स"), ("ms", "Alpes-de-Haute-Provence"), ("nb", "Alpes-de-Haute-Provence"), ("nl", "Alpes-de-Haute-Provence"), ("no", "Alpes-de-Haute-Provence"), ("pl", "Alpy Górnej Prowansji"), ("pt", "Alpes da Alta Provença"), ("ro", "Alpes-de-Haute-Provence"), ("ru", "Альпы Верхнего Прованса"), ("si", "ඇල\u{dca}ප\u{dca}ස\u{dca}-ඩ\u{dd2}-හව\u{dd4}ටේ පළ\u{dcf}ත"), ("sk", "Alpes-de-Haute-Provence"), ("sl", "Alpes-de-Haute-Provence"), ("sq", "Alpes-de-Haute-Provence"), ("sr", "Горњопровансалски Алпи"), ("sr_Latn", "Gornjoprovansalski Alpi"), ("sv", "Alpes-de-Haute-Provence"), ("sw", "Alpes-de-Haute-Provence"), ("ta", "ஆல\u{bcd}ப\u{bcd}ஸ\u{bcd} -டே -அவுட\u{bcd} -ப\u{bcd}ரொவென\u{bcd}ஸ\u{bcd}"), ("te", "ఆల\u{c4d}ప\u{c46}స\u{c4d}-డ\u{c3f}-హ\u{c3e}ట\u{c3f}-ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาลป\u{e4c}-เดอ-โอต-พรอว\u{e47}องส\u{e4c}"), ("tr", "Alpes-de-Haute-Provence"), ("uk", "Альпи Верхнього Провансу"), ("ur", "آلپ-دو-بالائی-پروانس"), ("vi", "Alpes-de-Haute-Provence"), ("yue", "上普羅旺斯阿爾卑斯"), ("yue_Hans", "上普罗旺斯阿尔卑斯"), ("zh", "上普罗旺斯阿尔卑斯省")]),
                        unofficial_name_list: ["Alpes-de-Haute-Provence"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::FR,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.6008723), longitude: Some(6.322607199999999), max_latitude: Some(45.12685099999999), min_latitude: Some(44.186442), max_longitude: Some(7.077154999999999), min_longitude: Some(5.4183639)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hautes-Alpes"), ("ar", "الألب العليا"), ("az", "Yuxarı Alplar"), ("be", "Верхнія Альпы"), ("bg", "Отз Алп"), ("bn", "হ\u{9be}উতেস আল\u{9cd}পিস"), ("ca", "Alts Alps"), ("ccp", "𑄦𑄅\u{1112a}𑄖\u{11134}-𑄃𑄣\u{11134}𑄛𑄬𑄌\u{11134}"), ("ceb", "Hautes-Alpes"), ("cs", "Hautes-Alpes"), ("cy", "Hautes-Alpes"), ("da", "Hautes-Alpes"), ("de", "Département Hautes-Alpes"), ("el", "Ωτ-Αλπ"), ("en", "Hautes-Alpes"), ("es", "Altos Alpes"), ("et", "Hautes-Alpes"), ("eu", "Alpe Garaiak"), ("fa", "اوت-آلپ"), ("fi", "Hautes-Alpes"), ("fr", "Hautes-Alpes"), ("gl", "Alpes Altos"), ("gu", "હોટસ-આલ\u{acd}પ\u{acd}સ"), ("he", "האלפים העליונים"), ("hi", "ओट\u{947}स-एल\u{94d}प\u{947}स"), ("hu", "Hautes-Alpes"), ("hy", "Վերին Ալպեր"), ("id", "Hautes-Alpes"), ("it", "Alte Alpi"), ("ka", "ზემო ალპები"), ("kk", "Жоғарғы Альпілер"), ("kn", "ಹ\u{ccc}ಟ\u{cc6}ಸ\u{ccd}-ಆಲ\u{ccd}ಪ\u{ccd}ಸ\u{ccd}"), ("ko", "오트잘프 주"), ("lt", "Aukštutinės Alpės"), ("lv", "Augšalpi"), ("mr", "ऑत-आल\u{94d}प"), ("ms", "Hautes-Alpes"), ("nb", "Hautes-Alpes"), ("nl", "Hautes-Alpes"), ("no", "Hautes-Alpes"), ("pl", "Alpy Wysokie"), ("pt", "Altos Alpes"), ("ro", "Hautes-Alpes"), ("ru", "Верхние Альпы"), ("si", "හව\u{dd4}ටෙස\u{dca}-ඇල\u{dca}ප\u{dca}ස\u{dca}"), ("sk", "Hautes-Alpes"), ("sl", "Hautes-Alpes"), ("sq", "Hautes-Alpes"), ("sr", "Горњи Алпи"), ("sr_Latn", "Gornji Alpi"), ("sv", "Hautes-Alpes"), ("sw", "Hautes-Alpes"), ("ta", "ஹஉட\u{bcd}ஸ\u{bcd}-ஆல\u{bcd}ப\u{bcd}ஸ\u{bcd}"), ("te", "హ\u{c3e}ట\u{c4d}స\u{c4d}-ఆల\u{c4d}ప\u{c46}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโอตซาลป\u{e4c}"), ("tr", "Hautes-Alpes"), ("uk", "Верхні Альпи"), ("ur", "بالائی-آلپ"), ("vi", "Hautes-Alpes"), ("yue", "上阿爾卑斯"), ("yue_Hans", "上阿尔卑斯"), ("zh", "上阿尔卑斯省")]),
                        unofficial_name_list: ["Hautes-Alpes"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::FR,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.9466791), longitude: Some(7.179025999999999), max_latitude: Some(44.3611549), min_latitude: Some(43.48030199999999), max_longitude: Some(7.718992999999998), min_longitude: Some(6.635411899999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Alpes-Maritimes"), ("ar", "الألب البحرية"), ("az", "Dənizkənarı Alplar"), ("be", "Дэпартамент Альпы Прыморскія"), ("bg", "Алп Маритим"), ("bn", "অ\u{9cd}য\u{9be}পলস মেরিত\u{9be}ইম"), ("ca", "Alps Marítims"), ("ccp", "𑄃𑄣\u{11134}𑄛𑄬𑄌\u{11134}-𑄟𑄬𑄢\u{11128}𑄑\u{1112d}𑄟\u{11134}𑄌\u{11134}"), ("ceb", "Alpes-Maritimes"), ("cs", "Alpes-Maritimes"), ("cy", "Alpes-Maritimes"), ("da", "Alpes-Maritimes"), ("de", "Alpes-Maritimes"), ("el", "Αλπ-Μαριτίμ"), ("en", "Alpes-Maritimes"), ("es", "Alpes Marítimos"), ("et", "Alpes-Maritimes"), ("eu", "Itsas Alpeak"), ("fa", "آلپ-ماریتیم"), ("fi", "Alpes-Maritimes"), ("fr", "Alpes-Maritimes"), ("gl", "Alpes Marítimos"), ("gu", "આલ\u{acd}પ\u{acd}સ-મ\u{ac7}રીટાઇમ\u{acd}સ"), ("he", "האלפים הימיים"), ("hi", "एल\u{94d}प\u{94d}स-म\u{948}रिटाइम\u{94d}स"), ("hr", "Alpes-Maritimes"), ("hu", "Alpes-Maritimes"), ("hy", "Ծովափնյա Ալպեր"), ("id", "Alpes-Maritimes"), ("it", "Alpi Marittime"), ("ka", "ზღვისპირა ალპები"), ("kk", "Альпі-Маритим"), ("kn", "ಆಲ\u{ccd}ಪ\u{cc6}ಸ\u{ccd}-ಮಾರ\u{cbf}ಟೈಮ\u{ccd}ಸ\u{ccd}"), ("ko", "알프마리팀 주"), ("lt", "Pajūrio Alpės"), ("lv", "Piejūras Alpi"), ("mk", "Приморски Алпи"), ("mr", "आल\u{94d}प-मरितीम"), ("ms", "Alpes-Maritimes"), ("nb", "Alpes-Maritimes"), ("nl", "Alpes-Maritimes"), ("no", "Alpes-Maritimes"), ("pl", "Alpy Nadmorskie"), ("pt", "Alpes Marítimos"), ("ro", "Alpes-Maritimes"), ("ru", "Приморские Альпы"), ("si", "ඇල\u{dca}ප\u{dca}ස\u{dca}-මැර\u{dd2}ටය\u{dd2}ම\u{dca}ස\u{dca}"), ("sk", "Alpes-Maritimes"), ("sl", "Alpes-Maritimes"), ("sq", "Alpes-Maritimes"), ("sr", "Приморски Алпи"), ("sr_Latn", "Primorski Alpi"), ("sv", "Alpes-Maritimes"), ("sw", "Alpes-Maritimes"), ("ta", "ஆல\u{bcd}ப\u{bcd}ஸ\u{bcd} -ம\u{bbe}ரிடிம\u{bcd}ஸ\u{bcd}"), ("te", "ఆల\u{c4d}ప\u{c46}స\u{c4d}-మ\u{c3e}ర\u{c3f}ట\u{c48}మ\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาลป\u{e4c}-มาร\u{e35}ต\u{e35}ม"), ("tr", "Alpes-Maritimes"), ("uk", "Приморські Альпи"), ("ur", "آلپ-ماریتیم"), ("vi", "Alpes-Maritimes"), ("yue", "濱海阿爾卑斯"), ("yue_Hans", "滨海阿尔卑斯"), ("zh", "滨海阿尔卑斯省")]),
                        unofficial_name_list: ["Alpes-Maritimes"].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::FR,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.759629), longitude: Some(4.5624426), max_latitude: Some(45.3662), min_latitude: Some(44.26434099999999), max_longitude: Some(4.8864709), min_longitude: Some(3.8611)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ardèche"), ("ar", "الأرديش"), ("az", "Ardeş (departament)"), ("be", "Ардэш"), ("bg", "Ардеш"), ("bn", "আর\u{9cd}দেচে"), ("ca", "Ardecha"), ("ccp", "𑄃𑄢\u{11134}𑄓𑄬𑄌\u{11134}"), ("ceb", "Ardèche"), ("cs", "Ardèche"), ("cy", "Ardèche"), ("da", "Ardèche"), ("de", "Ardèche"), ("el", "Αρντές"), ("en", "Ardèche"), ("es", "Ardéche"), ("et", "Ardèche’i departemang"), ("eu", "Ardèche"), ("fa", "آردش"), ("fi", "Ardèche"), ("fr", "Ardèche"), ("gl", "Ardèche"), ("gu", "અર\u{acd}ડ\u{ac7}ચ\u{ac7}"), ("he", "ארדש"), ("hi", "आरड\u{947}श"), ("hu", "Ardèche"), ("hy", "Արդեշ"), ("id", "Ardèche"), ("it", "Ardèche"), ("ja", "アルデシュ県"), ("ka", "არდეში"), ("kk", "Ардеш"), ("kn", "ಆರ\u{ccd}ಡ\u{cc6}ಚ\u{cc6}"), ("ko", "아르데슈 주"), ("lt", "Ardešas"), ("lv", "Ardēša"), ("mk", "Ардеш"), ("mr", "आर\u{94d}द\u{947}श"), ("ms", "Ardèche"), ("nb", "Ardèche"), ("nl", "Ardèche"), ("no", "Ardèche"), ("pl", "Ardèche"), ("pt", "Ardèche"), ("ro", "Ardèche"), ("ru", "Ардеш"), ("si", "අර\u{dca}ඩෙචේ"), ("sk", "Ardèche"), ("sl", "Ardèche"), ("sq", "Ardèche"), ("sr", "Ардеш"), ("sr_Latn", "Ardeš"), ("sv", "Ardèche"), ("sw", "Ardèche"), ("ta", "அர\u{bcd}டெச\u{bcd}சே"), ("te", "ఆర\u{c4d}డ\u{c46}చ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาร\u{e4c}แด\u{e47}ช"), ("tr", "Ardèche"), ("uk", "Ардеш"), ("ur", "آردیش"), ("vi", "Ardèche"), ("yue", "阿爾代什"), ("yue_Hans", "阿尔代什"), ("zh", "阿尔代什省")]),
                        unofficial_name_list: ["Ardèche"].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "08",
                        country_alpha2: Alpha2::FR,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.7624642), longitude: Some(4.6285053), max_latitude: Some(50.169162), min_latitude: Some(49.22697489999999), max_longitude: Some(5.394246), min_longitude: Some(4.0244631)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ardennes"), ("ar", "الأردين"), ("az", "Ardenlər (departament)"), ("be", "дэпартамент Ардэны"), ("bg", "Арден"), ("bn", "আরদেনেস"), ("ca", "Ardenes"), ("ccp", "𑄃𑄢\u{11134}𑄓𑄬𑄚\u{11134}𑄥𑄬𑄌\u{11134}"), ("ceb", "Ardennes"), ("cs", "Ardensko"), ("cy", "Ardennes"), ("da", "Ardennes"), ("de", "Département Ardennes"), ("el", "Αρδέννες"), ("en", "Ardennes"), ("es", "Ardenas"), ("et", "Ardennesi departemang"), ("eu", "Ardenak"), ("fa", "آردن"), ("fi", "Ardennes"), ("fr", "Ardennes"), ("gl", "Ardenas - Ardennes"), ("gu", "અર\u{acd}ડ\u{ac7}ન\u{acd}સ"), ("he", "ארדן"), ("hi", "आर\u{94d}द\u{947}न\u{947}स"), ("hr", "Ardennes"), ("hu", "Ardennes"), ("hy", "Արդեններ"), ("id", "Ardennes"), ("it", "Ardenne"), ("ja", "アルデンヌ県"), ("ka", "არდენი"), ("kk", "Арденне"), ("kn", "ಆರ\u{ccd}ಡ\u{cc6}ನ\u{ccd}ನ\u{cc6}ಸ\u{ccd}"), ("ko", "아르덴 주"), ("lt", "Ardėnai"), ("lv", "Ardēni"), ("mk", "Ардени"), ("mr", "अ\u{200d}\u{945}र\u{94d}द\u{947}न"), ("ms", "Ardennes"), ("nb", "Ardennes"), ("nl", "Ardennes"), ("no", "Ardennes"), ("pl", "Ardeny"), ("pt", "Ardenas"), ("ro", "Ardennes"), ("ru", "Арденны"), ("si", "අර\u{dca}ඩෙන\u{dca}නෙස\u{dca}"), ("sk", "Ardennes"), ("sl", "Ardennes"), ("sq", "Ardennes"), ("sr", "Ардени"), ("sr_Latn", "Ardeni"), ("sv", "Ardennes"), ("sw", "Ardennes"), ("ta", "அர\u{bcd}டேன\u{bcd}னெஸ\u{bcd}"), ("te", "అర\u{c4d}డ\u{c46}న\u{c4d}న\u{c46}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาร\u{e4c}แดน"), ("tr", "Ardennes"), ("uk", "Арденни"), ("ur", "اردن"), ("vi", "Ardennes"), ("yue", "亞丁"), ("yue_Hans", "亚丁"), ("zh", "阿登省")]),
                        unofficial_name_list: ["Ardennes"].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "09",
                        country_alpha2: Alpha2::FR,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.9326292), longitude: Some(1.443469), max_latitude: Some(43.3162221), min_latitude: Some(42.571489), max_longitude: Some(2.1758469), min_longitude: Some(0.825994)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ariège"), ("ar", "أرييج"), ("az", "Aryej"), ("bg", "Ариеж"), ("bn", "অ\u{9cd}য\u{9be}রিগে"), ("ca", "Arieja"), ("ccp", "𑄃\u{11127}𑄢\u{11128}𑄠𑄬𑄌\u{11134}"), ("ceb", "Ariège"), ("cs", "Ariège"), ("cy", "Ariège"), ("da", "Ariège"), ("de", "Département Ariège"), ("el", "Αριέζ"), ("en", "Ariège"), ("es", "Ariège"), ("et", "Ariège’i departemang"), ("eu", "Ariège"), ("fa", "آرییژ"), ("fi", "Ariège"), ("fr", "Ariège"), ("gl", "Ariège"), ("gu", "એરિ\u{a82}ગ\u{ac7}"), ("he", "ארייז׳"), ("hi", "एरीज"), ("hu", "Ariège"), ("hy", "Արյեժ"), ("id", "Ariège"), ("it", "Ariège"), ("ja", "アリエージュ県"), ("ka", "არეჟი"), ("kk", "Арьеж"), ("kn", "ಅರ\u{cbf}ಯ\u{cc6}ಜ\u{ccd}"), ("ko", "아리에주 주"), ("lt", "Arježas"), ("lv", "Arjēža"), ("mr", "आर\u{94d}य\u{947}ज"), ("ms", "Ariège"), ("nb", "Ariège"), ("nl", "Ariège"), ("no", "Ariège"), ("pl", "Ariège"), ("pt", "Ariège"), ("ro", "Ariège"), ("ru", "Арьеж"), ("si", "අර\u{dd3}ගේ"), ("sk", "Ariège"), ("sl", "Ariège"), ("sq", "Ariège"), ("sr", "Арјеж"), ("sr_Latn", "Arjež"), ("sv", "Ariège"), ("sw", "Ariège"), ("ta", "அரிஏஜே"), ("te", "ఆర\u{c3f}య\u{c47}జ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาเร\u{e35}ยฌ"), ("tr", "Ariège"), ("uk", "Арʼєж"), ("ur", "اریج"), ("vi", "Ariège"), ("yue", "阿列日"), ("yue_Hans", "阿列日"), ("zh", "阿列日省")]),
                        unofficial_name_list: ["Ariège"].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::FR,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.1563418), longitude: Some(4.3732462), max_latitude: Some(48.716736), min_latitude: Some(47.923696), max_longitude: Some(4.864605099999999), min_longitude: Some(3.3836479)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Aube"), ("ar", "أوب"), ("az", "Ob (departament)"), ("be", "Дэпартамент Об"), ("bg", "Об"), ("bn", "অবে"), ("ca", "Aube"), ("ccp", "𑄃\u{11127}𑄅\u{1112a}𑄝𑄬"), ("ceb", "Aube"), ("cs", "Aube"), ("cy", "Aube"), ("da", "Aube"), ("de", "Département Aube"), ("el", "Ωμπ"), ("en", "Aube"), ("es", "Aube"), ("et", "Aube’i departemang"), ("eu", "Aube"), ("fa", "اوب"), ("fi", "Aube"), ("fr", "Aube"), ("gl", "Aube"), ("gu", "ઔબ\u{ac7}"), ("he", "אוב"), ("hi", "ऑब\u{947}"), ("hr", "Aube"), ("hu", "Aube"), ("hy", "Օբ"), ("id", "Aube"), ("it", "Aube"), ("ja", "オーブ県"), ("ka", "ობი"), ("kk", "Об"), ("kn", "ಆಯುಬ\u{ccd}"), ("ko", "오브 주"), ("lt", "Obas"), ("lv", "Oba"), ("mk", "Об"), ("mr", "ऑब"), ("ms", "Aube"), ("nb", "Aube"), ("nl", "Aube"), ("no", "Aube"), ("pl", "Aube"), ("pt", "Aube"), ("ro", "Aube"), ("ru", "Об"), ("si", "ඖබේ"), ("sk", "Aube"), ("sl", "Aube"), ("sq", "Aube"), ("sr", "Об"), ("sr_Latn", "Ob"), ("sv", "Aube"), ("sw", "Aube"), ("ta", "ஆபே"), ("te", "ఆబ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโอบ"), ("tr", "Aube"), ("uk", "Об"), ("ur", "اوب"), ("vi", "Aube"), ("yue", "奧布"), ("yue_Hans", "奥布"), ("zh", "奥布省")]),
                        unofficial_name_list: ["Aube"].to_vec(),
                    }
                ),
                (
                    "11",
                    Subdivision{
                        name: "11",
                        country_alpha2: Alpha2::FR,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.0724667), longitude: Some(2.3813621), max_latitude: Some(43.460066), min_latitude: Some(42.648912), max_longitude: Some(3.240139), min_longitude: Some(1.688426)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Aude"), ("ar", "أود"), ("az", "Od (departament)"), ("be", "Дэпартамент Од"), ("bg", "Од"), ("bn", "ওদ"), ("ca", "Aude"), ("ccp", "𑄃\u{11127}𑄅\u{1112a}𑄓𑄬"), ("ceb", "Aude"), ("cs", "Aude"), ("cy", "Aude"), ("da", "Aude"), ("de", "Département Aude"), ("el", "Ωντ"), ("en", "Aude"), ("es", "Aude"), ("et", "Aude’i departemang"), ("eu", "Aude"), ("fa", "اود"), ("fi", "Aude"), ("fr", "Aude"), ("gl", "Aude"), ("gu", "ઔડ"), ("he", "אוד"), ("hi", "ओड\u{947}"), ("hu", "Aude"), ("hy", "Օդ"), ("id", "Aude"), ("it", "Aude"), ("ja", "オード県"), ("ka", "ოდი"), ("kk", "Од"), ("kn", "ಔಡ\u{cc6}"), ("ko", "오드 주"), ("lt", "Odas"), ("lv", "Oda"), ("mn", "Од аймаг"), ("mr", "ऑद"), ("ms", "Aude"), ("nb", "Aude"), ("nl", "Aude"), ("no", "Aude"), ("pl", "Aude"), ("pt", "Aude"), ("ro", "Aude"), ("ru", "Од"), ("si", "ඖඩේ"), ("sk", "Aude"), ("sl", "Aude"), ("sq", "Aude"), ("sr", "Од"), ("sr_Latn", "Od"), ("sv", "Aude"), ("sw", "Aude"), ("ta", "ஆடி"), ("te", "ఆడ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโอด"), ("tr", "Aude"), ("uk", "Од"), ("ur", "اود"), ("vi", "Aude"), ("yue", "奧德"), ("yue_Hans", "奥德"), ("zh", "奥德省")]),
                        unofficial_name_list: ["Aude"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "12",
                        country_alpha2: Alpha2::FR,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.2179747), longitude: Some(2.6189273), max_latitude: Some(44.941441), min_latitude: Some(43.690619), max_longitude: Some(3.4517541), min_longitude: Some(1.839313)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Aveyron"), ("ar", "أفيرون"), ("az", "Averon (departament)"), ("be", "Аверон"), ("bg", "Аверон"), ("bn", "এভেরন"), ("ca", "Avairon"), ("ccp", "𑄃\u{11127}𑄞𑄬𑄢\u{11127}𑄚\u{11134}"), ("ceb", "Aveyron"), ("cs", "Aveyron"), ("cy", "Aveyron"), ("da", "Aveyron"), ("de", "Département Aveyron"), ("el", "Αβεϊρόν"), ("en", "Aveyron"), ("es", "Aveyron"), ("et", "Aveyroni departemang"), ("eu", "Aveyron"), ("fa", "آویرون"), ("fi", "Aveyron"), ("fr", "Aveyron"), ("gl", "Aveyron"), ("gu", "એવ\u{ac7}રોન"), ("he", "אוורון"), ("hi", "एव\u{947}रौन"), ("hu", "Aveyron"), ("hy", "Ավերոն"), ("id", "Aveyron"), ("it", "Aveyron"), ("ja", "アヴェロン県"), ("ka", "ავერონი"), ("kk", "Аверон"), ("kn", "ಅವೇಯ\u{ccd}ರ\u{ccd}ಓನ\u{ccd}"), ("ko", "아베롱 주"), ("lt", "Averonas"), ("lv", "Aveirona"), ("mk", "Аверон"), ("mr", "अ\u{200d}\u{945}व\u{94d}ह\u{947}रो\u{902}"), ("ms", "Aveyron"), ("nb", "Aveyron"), ("nl", "Aveyron"), ("no", "Aveyron"), ("pl", "Aveyron"), ("pt", "Aveyron"), ("ro", "Aveyron"), ("ru", "Аверон"), ("si", "අවෙය\u{dd2}රොන\u{dca}"), ("sk", "Aveyron"), ("sl", "Aveyron"), ("sq", "Aveyron"), ("sr", "Аверон"), ("sr_Latn", "Averon"), ("sv", "Aveyron"), ("sw", "Aveyron"), ("ta", "அவேய\u{bcd}றோன\u{bcd}"), ("te", "అవ\u{c47}ర\u{c3e}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาแวรง"), ("tr", "Aveyron"), ("uk", "Аверон"), ("ur", "اویرون"), ("vi", "Aveyron"), ("yue", "阿韋龍"), ("yue_Hans", "阿韦龙"), ("zh", "阿韦龙省")]),
                        unofficial_name_list: ["Aveyron"].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "13",
                        country_alpha2: Alpha2::FR,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.59116789999999), longitude: Some(5.3102505), max_latitude: Some(43.924136), min_latitude: Some(43.1573841), max_longitude: Some(5.8134309), min_longitude: Some(4.230207)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Bouches-du-Rhône"), ("ar", "بوش دي رون"), ("az", "Buş-dü-Ron"), ("be", "Буш-дзю-Рон"), ("bg", "Буш дю Рон"), ("bn", "ব\u{9c1}চেস-ড\u{9c1}-রোন"), ("ca", "Boques del Roine"), ("ccp", "𑄝𑄅\u{1112a}𑄚\u{11134}𑄌\u{11134}-𑄓\u{1112a}-𑄢\u{1112e}𑄚\u{11134}"), ("ceb", "Bouches-du-Rhône"), ("cs", "Bouches-du-Rhône"), ("cy", "Bouches-du-Rhône"), ("da", "Bouches-du-Rhône"), ("de", "Département Bouches-du-Rhône"), ("el", "Μπους-ντυ-Ρον"), ("en", "Bouches-du-Rhône"), ("es", "Bocas del Ródano"), ("et", "Bouches-du-Rhône"), ("eu", "Bouches-du-Rhône"), ("fa", "بوش دو رون"), ("fi", "Bouches-du-Rhône"), ("fr", "Bouches-du-Rhône"), ("gl", "Bocas de Ródano"), ("gu", "બોચ\u{ac7}સ-ડ\u{ac1}-રૉન"), ("he", "שפך הרון"), ("hi", "बोच\u{947}स-ड\u{941}-रोन"), ("hu", "Bouches-du-Rhône"), ("hy", "Բուշ-դյու-Ռոն"), ("id", "Bouches-du-Rhône"), ("it", "Bocche del Rodano"), ("ka", "ბუშ-დიუ-რონი"), ("kk", "Буш-дю-Рон"), ("kn", "ಬ\u{ccc}ಚ\u{cc6}ಸ\u{ccd}-ಡು-ರೋನ\u{ccd}"), ("ko", "부슈뒤론 주"), ("lt", "Ronos delta"), ("lv", "Bušdirona"), ("mr", "ब\u{941}श-द\u{94d}य\u{941}-रोन"), ("ms", "Bouches-du-Rhône"), ("nb", "Bouches-du-Rhône"), ("nl", "Bouches-du-Rhône"), ("no", "Bouches-du-Rhône"), ("pl", "Delta Rodanu"), ("pt", "Bocas do Ródano"), ("ro", "Bouches-du-Rhône"), ("ru", "Буш-дю-Рон"), ("si", "බෞචෙස\u{dca}-ඩ\u{dd4}-රෝනේ"), ("sk", "Bouches-du-Rhône"), ("sl", "Bouches-du-Rhône"), ("sq", "Bouches-du-Rhône"), ("sr", "Ушће Роне"), ("sr_Latn", "Ušće Rone"), ("sv", "Bouches-du-Rhône"), ("sw", "Bouches-du-Rhône"), ("ta", "பௌச\u{bcd}ஸ\u{bcd} -டு -ரஹ\u{bbe}னே"), ("te", "బ\u{c4c}చ\u{c46}స\u{c4d}-డు-ర\u{c4b}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e38}ช-ด\u{e39}ว\u{e4c}-โรน"), ("tr", "Bouches-du-Rhône"), ("uk", "Буш-дю-Рон"), ("ur", "بوش-دو-رون"), ("vi", "Bouches-du-Rhône"), ("yue", "隆河河口"), ("yue_Hans", "隆河河口"), ("zh", "罗讷河口省")]),
                        unofficial_name_list: ["Bouches-du-Rhône"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::FR,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.1213315), longitude: Some(-0.4330578), max_latitude: Some(49.4299189), min_latitude: Some(48.751681), max_longitude: Some(0.4464769), min_longitude: Some(-1.159777)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Calvados"), ("ar", "كالفادوس"), ("az", "Kalvados"), ("be", "Дэпартамент Кальвадос"), ("bg", "Калвадос"), ("bn", "ক\u{9cd}য\u{9be}\u{9be}লভ\u{9be}ডোস"), ("ca", "Calvados"), ("ccp", "𑄇\u{11133}𑄠𑄢\u{11134}𑄞𑄓\u{1112e}𑄌\u{11134}"), ("ceb", "Calvados"), ("cs", "Calvados"), ("cy", "Calvados"), ("da", "Calvados"), ("de", "Département Calvados"), ("el", "Καλβαντός"), ("en", "Calvados"), ("es", "Calvados"), ("et", "Calvadosi departemang"), ("eu", "Calvados"), ("fa", "کالوادوس"), ("fi", "Calvados"), ("fr", "Calvados"), ("gl", "Calvados"), ("gu", "ક\u{ac7}લ\u{acd}વાડોસ"), ("he", "קלבדוס"), ("hi", "स\u{947}ल\u{94d}वाडोस"), ("hu", "Calvados"), ("hy", "Կալվադոս"), ("id", "Calvados"), ("it", "Calvados"), ("ja", "カルヴァドス県"), ("jv", "Calvados"), ("ka", "კალვადოსი"), ("kk", "Кальвадос"), ("kn", "ಕ\u{ccd}ಯಾಲ\u{ccd}ವಾಡೋಸ\u{ccd}"), ("ko", "칼바도스 주"), ("lt", "Kalvadosas"), ("lv", "Kalvadosa"), ("mr", "काल\u{94d}व\u{94d}हादोस"), ("ms", "Calvados"), ("nb", "Calvados"), ("nl", "Calvados"), ("no", "Calvados"), ("pl", "Calvados"), ("pt", "Calvados"), ("ro", "Calvados"), ("ru", "Кальвадос"), ("si", "කල\u{dca}වඩෝස\u{dca}"), ("sk", "Calvados"), ("sl", "Calvados"), ("sq", "Calvados"), ("sr", "Калвадос"), ("sr_Latn", "Kalvados"), ("sv", "Calvados"), ("sw", "Calvados"), ("ta", "ச\u{bbe}ல\u{bcd}வடோஸ\u{bcd}"), ("te", "క\u{c3e}ల\u{c4d}వ\u{c3e}డ\u{c4b}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกาลวาโดส"), ("tr", "Calvados"), ("uk", "Кальвадос"), ("ur", "کالوادوس"), ("vi", "Calvados"), ("yue", "卡爾華多斯"), ("yue_Hans", "卡尔华多斯"), ("zh", "卡爾瓦多斯省")]),
                        unofficial_name_list: ["Calvados"].to_vec(),
                    }
                ),
                (
                    "15",
                    Subdivision{
                        name: "15",
                        country_alpha2: Alpha2::FR,
                        code: "15",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.1191997), longitude: Some(2.6326062), max_latitude: Some(45.4834729), min_latitude: Some(44.61577579999999), max_longitude: Some(3.371465), min_longitude: Some(2.062882)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Cantal"), ("ar", "كانتال"), ("az", "Kantal"), ("be", "Дэпартамент Канталь"), ("bg", "Кантал"), ("bn", "ক\u{9cd}য\u{9be}ন\u{9cd}ট\u{9be}ল"), ("ca", "Cantal"), ("ccp", "𑄇\u{11133}𑄠𑄚\u{11134}𑄑𑄣\u{11134}"), ("ceb", "Cantal"), ("cs", "Cantal"), ("cy", "Cantal"), ("da", "Cantal"), ("de", "Département Cantal"), ("el", "Καντάλ"), ("en", "Cantal"), ("es", "Cantal"), ("et", "Cantali departemang"), ("eu", "Cantal"), ("fa", "کانتال"), ("fi", "Cantal"), ("fr", "Cantal"), ("gl", "Cantal"), ("gu", "ક\u{ac7}ન\u{acd}ટલ"), ("he", "קנטל"), ("hi", "क\u{948}\u{902}टाल"), ("hu", "Cantal"), ("hy", "Կանտալ"), ("id", "Cantal"), ("it", "Cantal"), ("ja", "カンタル県"), ("ka", "კანტალი"), ("kk", "Канталь"), ("kn", "ಕ\u{ccd}ಯಾಂಟಲ\u{ccd}"), ("ko", "캉탈 주"), ("lt", "Kantalis"), ("lv", "Kantāla"), ("mr", "का\u{902}त\u{945}ल"), ("ms", "Cantal"), ("nb", "Cantal"), ("nl", "Cantal"), ("no", "Cantal"), ("pl", "Cantal"), ("pt", "Cantal"), ("ro", "Cantal"), ("ru", "Канталь"), ("si", "කැන\u{dca}ටල\u{dca}"), ("sk", "Cantal"), ("sl", "Cantal"), ("sq", "Cantal"), ("sr", "Кантал"), ("sr_Latn", "Kantal"), ("sv", "Cantal"), ("sw", "Cantal"), ("ta", "க\u{bbe}ண\u{bcd}டல\u{bcd}"), ("te", "క\u{c3e}ంట\u{c3e}ల\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e47}องตาล"), ("tr", "Cantal"), ("uk", "Канталь"), ("ur", "کانتال"), ("vi", "Cantal"), ("yue", "康塔爾"), ("yue_Hans", "康塔尔"), ("zh", "康塔爾省")]),
                        unofficial_name_list: ["Cantal"].to_vec(),
                    }
                ),
                (
                    "16",
                    Subdivision{
                        name: "16",
                        country_alpha2: Alpha2::FR,
                        code: "16",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.7519958), longitude: Some(0.1534761), max_latitude: Some(46.140851), min_latitude: Some(45.1916819), max_longitude: Some(0.9471240999999999), min_longitude: Some(-0.463103)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Charente"), ("ar", "شارنت"), ("az", "Şaranta (departament)"), ("be", "Шаранта"), ("bg", "Шарант"), ("bn", "চ\u{9be}রেন\u{9cd}ট"), ("ca", "Charente"), ("ccp", "𑄇𑄢𑄬𑄚\u{11134}𑄑\u{11128}"), ("ceb", "Charente"), ("cs", "Charente"), ("cy", "Charente"), ("da", "Charente"), ("de", "Département Charente"), ("el", "Σαράντ"), ("en", "Charente"), ("es", "Charente"), ("et", "Charente’i departemang"), ("eu", "Charente"), ("fa", "شارانت"), ("fi", "Charente"), ("fr", "Charente"), ("gl", "Charente"), ("gu", "શ\u{ac7}ર\u{ac7}ન\u{acd}ટ\u{ac7}"), ("he", "שראנט"), ("hi", "च\u{947}र\u{902}टी"), ("hu", "Charente"), ("hy", "Շարանտ"), ("id", "Charente"), ("it", "Charente"), ("ja", "シャラント県"), ("ka", "შარანტა"), ("kk", "Шаранта"), ("kn", "ಚರ\u{cc6}ನ\u{ccd}"), ("ko", "샤랑트 주"), ("lt", "Šaranta"), ("lv", "Šaranta"), ("mr", "शारा\u{902}त"), ("ms", "Charente"), ("nb", "Charente"), ("nl", "Charente"), ("no", "Charente"), ("pl", "Charente"), ("pt", "Charente"), ("ro", "Charente"), ("ru", "Шаранта"), ("si", "චරෙන\u{dca}ටේ"), ("sk", "Charente"), ("sl", "Charente"), ("sq", "Charente"), ("sr", "Шарант"), ("sr_Latn", "Šarant"), ("sv", "Charente"), ("sw", "Charente"), ("ta", "ச\u{bbe}ரெண\u{bcd}டே"), ("te", "చ\u{c3e}ర\u{c46}ంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดชาร\u{e47}องต\u{e4c}"), ("tr", "Charente"), ("uk", "Шаранта"), ("ur", "شارنت"), ("vi", "Charente"), ("yue", "沙藍特"), ("yue_Hans", "沙蓝特"), ("zh", "夏朗德省")]),
                        unofficial_name_list: ["Charente"].to_vec(),
                    }
                ),
                (
                    "17",
                    Subdivision{
                        name: "17",
                        country_alpha2: Alpha2::FR,
                        code: "17",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.74948999999999), longitude: Some(-0.7733188), max_latitude: Some(46.371485), min_latitude: Some(45.0887499), max_longitude: Some(0.005972), min_longitude: Some(-1.5626689)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Charente-Maritime"), ("ar", "شارنت البحرية"), ("az", "Dənizkənarı Şaranta"), ("be", "Дэпартамент Шаранта Прыморская"), ("bg", "Шарант Маритим"), ("bn", "শ\u{9be}রন\u{9cd}ত\u{9cd}\u{200c}-ম\u{9cd}য\u{9be}রিট\u{9be}ইম"), ("ca", "Charente Marítim"), ("ccp", "𑄇𑄢𑄬𑄚\u{11134}𑄑\u{11128}-𑄟𑄬𑄢\u{11128}𑄑\u{1112d}𑄟\u{11134}"), ("ceb", "Charente-Maritime"), ("cs", "Charente-Maritime"), ("cy", "Charente-Maritime"), ("da", "Charente-Maritime"), ("de", "Département Charente-Maritime"), ("el", "Σαράντ-Μαριτίμ"), ("en", "Charente-Maritime"), ("es", "Charente Marítimo"), ("et", "Charente-Maritime"), ("eu", "Charente-Maritime"), ("fa", "شرانت-ماریتیم"), ("fi", "Charente-Maritime"), ("fr", "Charente-Maritime"), ("gl", "Charente Marítimo"), ("gu", "બામાકો"), ("he", "השראנט הימי"), ("hi", "च\u{947}र\u{902}टी-म\u{947}रीटाइम"), ("hu", "Charente-Maritime"), ("hy", "Ծովափնյա Շարանտ"), ("id", "Charente-Maritime"), ("it", "Charente Marittima"), ("ka", "ზღვისპირა შარანტა"), ("kk", "Шаранта-Маритим"), ("kn", "ಚರ\u{cc6}ನ\u{ccd}ಟ\u{cc6}-ಮಾರ\u{cbf}ಟೈಮ\u{ccd}"), ("ko", "샤랑트마리팀 주"), ("lt", "Pajūrio Šaranta"), ("lv", "Piejūras Šaranta"), ("mk", "Приморска Шаранта"), ("mr", "शारा\u{902}त-मरितीम"), ("ms", "Charente-Maritime"), ("nb", "Charente-Maritime"), ("nl", "Charente-Maritime"), ("no", "Charente-Maritime"), ("pl", "Charente-Maritime"), ("pt", "Charente-Maritime"), ("ro", "Charente-Maritime"), ("ru", "Приморская Шаранта"), ("si", "චරෙන\u{dca}ටේ-මර\u{dd2}ටය\u{dd2}ම\u{dca}"), ("sk", "Charente-Maritime"), ("sl", "Charente-Maritime"), ("sq", "Charente-Maritime"), ("sr", "Приморски Шарант"), ("sr_Latn", "Primorski Šarant"), ("sv", "Charente-Maritime"), ("sw", "Charente-Maritime"), ("ta", "ச\u{bbe}ரெண\u{bcd}டே -ம\u{bbe}ரிடைம\u{bcd}"), ("te", "చ\u{c3e}ర\u{c46}ంట\u{c4d}-మ\u{c3e}ర\u{c3f}ట\u{c48}మ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดชาร\u{e47}องต\u{e4c}-มาร\u{e35}ต\u{e35}ม"), ("tr", "Charente-Maritime"), ("uk", "Приморська Шаранта"), ("ur", "شارنت-ماریتیم"), ("vi", "Charente-Maritime"), ("yue", "濱海沙藍特"), ("yue_Hans", "滨海沙蓝特"), ("zh", "滨海夏朗德省")]),
                        unofficial_name_list: ["Charente-Maritime"].to_vec(),
                    }
                ),
                (
                    "18",
                    Subdivision{
                        name: "18",
                        country_alpha2: Alpha2::FR,
                        code: "18",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.954005), longitude: Some(2.4671908), max_latitude: Some(47.629116), min_latitude: Some(46.4204769), max_longitude: Some(3.0797451), min_longitude: Some(1.773677)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Cher (département)"), ("ar", "شير"), ("az", "Şer (departament)"), ("be", "Шэр, дэпартамент"), ("bg", "Шер"), ("bn", "চের"), ("ca", "Cher"), ("ccp", "𑄌𑄬𑄢\u{11134}"), ("ceb", "Cher"), ("cs", "Cher"), ("cy", "Cher"), ("da", "Cher"), ("de", "Département Cher"), ("el", "Σερ"), ("en", "Cher"), ("es", "Cher"), ("et", "Cheri departemang"), ("eu", "Cher"), ("fa", "شر"), ("fi", "Cher"), ("fr", "Cher"), ("gl", "Cher"), ("gu", "ચ\u{ac7}ર"), ("hi", "च\u{947}र"), ("hu", "Cher"), ("hy", "Շեր"), ("id", "Cher"), ("is", "Cher"), ("it", "Cher"), ("ja", "シェール県"), ("ka", "შერი"), ("kk", "Шер"), ("kn", "ಚ\u{cc6}ರ\u{ccd}"), ("ko", "셰르 주"), ("lt", "Šeras"), ("lv", "Šēra"), ("mr", "श\u{947}र"), ("ms", "Cher"), ("nb", "Cher"), ("nl", "Cher"), ("no", "Cher"), ("pl", "Cher"), ("pt", "Cher"), ("ro", "Cher"), ("ru", "Шер"), ("si", "චෙර\u{dca}"), ("sk", "Cher"), ("sl", "Cher"), ("sq", "Cher"), ("sr", "Шер"), ("sr_Latn", "Šer"), ("sv", "Cher"), ("sw", "Cher"), ("ta", "செர\u{bcd}"), ("te", "చ\u{c46}ర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแชร\u{e4c}"), ("tr", "Cher"), ("uk", "Шер"), ("ur", "شر"), ("vi", "Cher, Centre"), ("yue", "些爾"), ("yue_Hans", "些尔"), ("zh", "谢尔省")]),
                        unofficial_name_list: ["Cher"].to_vec(),
                    }
                ),
                (
                    "19",
                    Subdivision{
                        name: "19",
                        country_alpha2: Alpha2::FR,
                        code: "19",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.372114), longitude: Some(1.873739), max_latitude: Some(45.40995909999999), min_latitude: Some(45.32584199999999), max_longitude: Some(1.921707), min_longitude: Some(1.822327)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Corrèze"), ("ar", "كوريز"), ("az", "Korrez"), ("be", "Карэз"), ("bg", "Корез"), ("bn", "ক\u{9c1}রেজে"), ("ca", "Corresa"), ("ccp", "𑄇\u{1112e}𑄢𑄬𑄌\u{11134}"), ("ceb", "Corrèze"), ("cs", "Corrèze"), ("cy", "Corrèze"), ("da", "Corrèze"), ("de", "Département Corrèze"), ("el", "Κορέζ"), ("en", "Corrèze"), ("es", "Corréze"), ("et", "Corrèze’i departemang"), ("eu", "Corrèze"), ("fa", "کورز"), ("fi", "Corrèze"), ("fr", "Corrèze"), ("gl", "Corrèze"), ("gu", "કોર\u{ac7}ઝ"), ("he", "קורז"), ("hi", "कॉरीज\u{93c}"), ("hu", "Corrèze"), ("hy", "Կորեզ"), ("id", "Corrèze"), ("it", "Corrèze"), ("ja", "コレーズ県"), ("ka", "კორეზი"), ("kk", "Коррез"), ("kn", "ಕಾರ\u{ccd}ರೀಜ\u{ccd}"), ("ko", "코레즈 주"), ("lt", "Korezas"), ("lv", "Korēza"), ("mr", "कोर\u{947}झ"), ("ms", "Corrèze"), ("nb", "Corrèze"), ("nl", "Corrèze"), ("no", "Corrèze"), ("pl", "Corrèze"), ("pt", "Corrèze"), ("ro", "Corrèze"), ("ru", "Коррез"), ("si", "කොර\u{dca}රෙසේ"), ("sk", "Corrèze"), ("sl", "Corrèze"), ("sq", "Corrèze"), ("sr", "Корез"), ("sr_Latn", "Korez"), ("sv", "Corrèze"), ("sw", "Corrèze"), ("ta", "க\u{bbe}ர\u{bcd}ப\u{bcd}ரேஸி"), ("te", "క\u{c4b}ర\u{c46}జ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกอแรซ"), ("tr", "Corrèze"), ("uk", "Коррез"), ("ur", "کوریز"), ("vi", "Corrèze"), ("yue", "科雷茲"), ("yue_Hans", "科雷兹"), ("zh", "科雷兹省")]),
                        unofficial_name_list: ["Corrèze"].to_vec(),
                    }
                ),
                (
                    "20R",
                    Subdivision{
                        name: "20R",
                        country_alpha2: Alpha2::FR,
                        code: "20R",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.180721), longitude: Some(9.04704), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCollectivityWithSpecialStatus,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Korsika"), ("am", "ኮርሲካ"), ("ar", "كورسيكا"), ("az", "Korsika"), ("be", "Корсіка"), ("bg", "Корсика"), ("bn", "কর\u{9cd}স"), ("bs", "Korzika"), ("ca", "Còrsega"), ("ccp", "𑄇\u{1112e}𑄢\u{11134}𑄌\u{11134}"), ("ceb", "Corse"), ("cs", "Korsika"), ("cy", "Corsica"), ("da", "Korsika"), ("de", "Korsika"), ("el", "Κορσική"), ("en", "Corsica"), ("es", "Córcega"), ("et", "Korsika"), ("eu", "Korsika"), ("fa", "جزیره کرس"), ("fi", "Korsika"), ("fr", "Corse"), ("ga", "An Chorsaic"), ("gl", "Córsega"), ("ha", "Korsika"), ("ha_NE", "Korsika"), ("he", "קורסיקה"), ("hi", "कोर\u{94d}सिका"), ("hr", "Korzika"), ("hu", "Korzika"), ("hy", "Կորսիկա"), ("id", "Korsika"), ("is", "Korsíka"), ("it", "Corsica"), ("ja", "コルシカ島"), ("jv", "Korsika"), ("ka", "კორსიკა"), ("kk", "Корсика аралы"), ("kn", "ಕಾರ\u{ccd}ಸ\u{cbf}ಕ"), ("ko", "코르시카"), ("ky", "Корсика"), ("lt", "Korsika"), ("lv", "Korsika"), ("mk", "Корзика"), ("mr", "कॉर\u{94d}स"), ("ms", "Corse"), ("nb", "Korsika"), ("nl", "Corsica"), ("no", "Korsika"), ("pa", "ਕਾਰਸਿਕਾ"), ("pl", "Korsyka"), ("pt", "Córsega"), ("ro", "Corsica"), ("ru", "Корсика"), ("sk", "Korzika"), ("sl", "Korzika"), ("so", "Coorsica"), ("sq", "Korsika"), ("sr", "Корзика"), ("sr_Latn", "Korzika"), ("sv", "Korsika"), ("sw", "Korsika"), ("ta", "கோர\u{bcd}சிக\u{bbe}"), ("th", "คอร\u{e4c}ซ\u{e34}กา"), ("tr", "Korsika"), ("uk", "Корсика"), ("ur", "کورسیکا"), ("uz", "Korsika"), ("vi", "Corse"), ("yue", "科西嘉"), ("yue_Hans", "科西嘉"), ("zh", "科西嘉岛")]),
                        unofficial_name_list: ["Corse", "Corsica"].to_vec(),
                    }
                ),
                (
                    "21",
                    Subdivision{
                        name: "21",
                        country_alpha2: Alpha2::FR,
                        code: "21",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.5126795), longitude: Some(4.635412), max_latitude: Some(48.0313109), min_latitude: Some(46.899847), max_longitude: Some(5.518767), min_longitude: Some(4.065189999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Côte-d’Or"), ("ar", "كوت دور"), ("az", "Kot-d’Or"), ("bn", "কোট-ডী‘অর"), ("ca", "Costa d’Or"), ("ccp", "𑄇\u{1112e}𑄖\u{11134}-𑄓\u{11128}‘𑄃\u{11127}𑄢\u{11134}"), ("ceb", "Côte-d’Or"), ("cs", "Côte-d’Or"), ("cy", "Côte-d’Or"), ("da", "Côte-d’Or"), ("de", "Département Côte-d’Or"), ("el", "Κοτ-ντ’Ορ"), ("en", "Côte-d’Or"), ("es", "Côte-d’Or"), ("et", "Côte-d’Ori departemang"), ("eu", "Côte-d’Or"), ("fa", "کوت دور"), ("fi", "Côte-d’Or"), ("fr", "Côte-d’Or"), ("gl", "Côte-d’Or"), ("gu", "કોટ-ડી‘ઓર"), ("he", "קוט ד׳ור"), ("hi", "कोट\u{947}-डीओर"), ("hu", "Côte-d’Or"), ("id", "Côte-d’Or"), ("is", "Côte-d’Or"), ("it", "Côte-d’Or"), ("jv", "Côte-d’Or"), ("kk", "Кот-д’Ор"), ("kn", "ಕೋಟ\u{ccd}-ಡ\u{cbf}‘ಒರ\u{ccd}"), ("ko", "코트도르 주"), ("lt", "Kot d’Oras"), ("lv", "Kotdora"), ("mk", "Златен Брег"), ("mr", "कोत-द’ओर"), ("ms", "Côte-d’Or"), ("nb", "Côte-d’Or"), ("nl", "Côte-d’Or"), ("no", "Côte-d’Or"), ("pl", "Côte-d’Or"), ("pt", "Côte-d’Or"), ("ro", "Côte-d’Or"), ("si", "කෝටේ -ඩ\u{dd2}ඕර\u{dca}"), ("sk", "Côte-d’Or"), ("sl", "Côte-d’Or"), ("sq", "Côte-d’Or"), ("sr", "Златна обала"), ("sr_Latn", "Zlatna obala"), ("sv", "Côte-d’Or"), ("sw", "Côte-d’Or"), ("ta", "கோட\u{bcd}-டி ‘ஓர\u{bcd}"), ("te", "క\u{c4b}ట\u{c46}-డ\u{c3f}ఓర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโกต-ดอร\u{e4c}"), ("tr", "Côte-d’Or"), ("uk", "Кот-дʼОр"), ("ur", "کوت دور"), ("vi", "Côte-d’Or"), ("yue", "黃金丘"), ("yue_Hans", "黄金丘"), ("zh", "科多尔省")]),
                        unofficial_name_list: ["Côte-d'Or"].to_vec(),
                    }
                ),
                (
                    "22",
                    Subdivision{
                        name: "22",
                        country_alpha2: Alpha2::FR,
                        code: "22",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.5108101), longitude: Some(-3.3263676), max_latitude: Some(48.90093599999999), min_latitude: Some(48.0325681), max_longitude: Some(-1.909064), min_longitude: Some(-3.665906)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Côtes-d’Armor"), ("ar", "كوت درمور"), ("az", "Kot-d’Armor"), ("bn", "কোট-ডি‘আমর"), ("ca", "Costes del Nord"), ("ccp", "𑄇\u{1112e}𑄖\u{11134}-𑄓\u{11128}‘𑄃𑄢\u{11134}𑄟\u{1112e}𑄢\u{11134}"), ("ceb", "Côtes-d’Armor"), ("cs", "Côtes-d’Armor"), ("cy", "Aodoù-an-Arvor"), ("da", "Côtes-d’Armor"), ("de", "Département Côtes-d’Armor"), ("el", "Κοτ-ντ’Αρμόρ"), ("en", "Côtes-d’Armor"), ("es", "Côtes-d’Armor"), ("et", "Côtes-d’Armori departemang"), ("eu", "Côtes-d’Armor"), ("fa", "کوت-دارمور"), ("fi", "Côtes-d’Armor"), ("fr", "Côtes-d’Armor"), ("gl", "Côtes-d’Armor"), ("gu", "કોટ\u{acd}સ-ડી‘આર\u{acd}મર"), ("he", "קוט-ד׳ארמור"), ("hi", "कोट\u{947}स-डी‘आर\u{94d}मोर"), ("hu", "Côtes-d’Armor"), ("id", "Côtes-d’Armor"), ("is", "Côtes-d’Armor"), ("it", "Côtes-d’Armor"), ("kk", "Кот-д’Армор"), ("kn", "ಕೋಟ\u{ccd}ಸ\u{ccd}-ಡ\u{cbf}ಆರ\u{ccd}ಮರ\u{ccd}"), ("ko", "코트다르모르 주"), ("lt", "Kot d’Armoras"), ("lv", "Kotdarmora"), ("mr", "कोत-द’आर\u{94d}मोर"), ("ms", "Côtes-d’Armor"), ("nb", "Côtes-d’Armor"), ("nl", "Côtes-d’Armor"), ("no", "Côtes-d’Armor"), ("pl", "Côtes-d’Armor"), ("pt", "Côtes-d’Armor"), ("ro", "Côtes-d’Armor"), ("si", "කෝටෙස\u{dca} -ඩ\u{dd2} අර\u{dca}මර\u{dca}"), ("sk", "Côtes-d’Armor"), ("sl", "Côtes-d’Armor"), ("sr", "Обале Армора"), ("sr_Latn", "Obale Armora"), ("sv", "Côtes-d’Armor"), ("sw", "Côtes-d’Armor"), ("ta", "க\u{bbe}ட\u{bcd}ஸ\u{bcd} -ட’அர\u{bcd}மோர\u{bcd}"), ("te", "క\u{c4b}ట\u{c46}స\u{c4d}-డ\u{c3f} ఆర\u{c4d}మర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโกต-ดาร\u{e4c}มอร\u{e4c}"), ("tr", "Côtes-d’Armor"), ("uk", "Кот-дʼАрмор"), ("ur", "کوت درمور"), ("vi", "Côtes-d’Armor"), ("yue", "阿摩爾濱海"), ("yue_Hans", "阿摩尔滨海"), ("zh", "阿摩尔滨海省")]),
                        unofficial_name_list: ["Côtes-du-Nord"].to_vec(),
                    }
                ),
                (
                    "23",
                    Subdivision{
                        name: "23",
                        country_alpha2: Alpha2::FR,
                        code: "23",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.03776329999999), longitude: Some(2.0627832), max_latitude: Some(46.45536999999999), min_latitude: Some(45.663551), max_longitude: Some(2.6112931), min_longitude: Some(1.3726041)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Creuse"), ("ar", "كروز"), ("az", "Kröz"), ("be", "Кроз, дэпартамент"), ("bg", "Крьоз"), ("bn", "ক\u{9cd}র\u{9c1}জ"), ("ca", "Cruesa"), ("ccp", "𑄇\u{11133}𑄢𑄬𑄅\u{1112a}𑄌\u{11134}"), ("ceb", "Creuse"), ("cs", "Creuse"), ("cy", "Creuse"), ("da", "Creuse"), ("de", "Département Creuse"), ("el", "Κρεζ"), ("en", "Creuse"), ("es", "Creuse"), ("et", "Creuse’i departemang"), ("eu", "Creuse"), ("fa", "کروز"), ("fi", "Creuse"), ("fr", "Creuse"), ("gl", "Creuse"), ("gu", "ક\u{acd}ર\u{ac7}ય\u{ac1}ઝ"), ("he", "קרז"), ("hi", "क\u{94d}र\u{942}स\u{947}"), ("hu", "Creuse"), ("hy", "Կրյոզ"), ("id", "Creuse"), ("it", "Creuse"), ("ja", "クルーズ県"), ("ka", "კრეზი"), ("kk", "Крёз"), ("kn", "ಕ\u{ccd}ರ\u{cc2}ಸ\u{ccd}"), ("ko", "크뢰즈 주"), ("lt", "Krezas"), ("lv", "Krēza"), ("mr", "क\u{94d}र\u{942}झ"), ("ms", "Creuse"), ("nb", "Creuse"), ("nl", "Creuse"), ("no", "Creuse"), ("pl", "Creuse"), ("pt", "Creuse"), ("ro", "Creuse"), ("ru", "Крёз"), ("si", "ක\u{dca}\u{200d}රෙය\u{dd6}ස\u{dca}"), ("sk", "Creuse"), ("sl", "Creuse"), ("sq", "Creuse"), ("sr", "Крез"), ("sr_Latn", "Krez"), ("sv", "Creuse"), ("sw", "Creuse"), ("ta", "கிரெஸெ"), ("te", "క\u{c4d}ర\u{c3f}యూస\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเคร\u{e34}ซ"), ("tr", "Creuse"), ("uk", "Крез"), ("ur", "کروز"), ("vi", "Creuse"), ("yue", "克勒茲"), ("yue_Hans", "克勒兹"), ("zh", "克勒兹省")]),
                        unofficial_name_list: ["Creuse"].to_vec(),
                    }
                ),
                (
                    "24",
                    Subdivision{
                        name: "24",
                        country_alpha2: Alpha2::FR,
                        code: "24",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.14694859999999), longitude: Some(0.7572205), max_latitude: Some(45.7147689), min_latitude: Some(44.570736), max_longitude: Some(1.448245), min_longitude: Some(-0.041877)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Dordogne"), ("ar", "دوردونيي"), ("az", "Dordon"), ("be", "Дардонь"), ("bg", "Дордон"), ("bn", "দোর\u{9cd}দোগ\u{9cd}নে"), ("ca", "Dordonya"), ("ccp", "𑄓\u{11127}𑄢\u{11134}𑄓\u{11127}𑄇\u{11134}"), ("ceb", "Dordogne"), ("cs", "Dordogne"), ("cy", "Dordogne"), ("da", "Dordogne"), ("de", "Département Dordogne"), ("el", "Ντορντόνι"), ("en", "Dordogne"), ("es", "Dordoña"), ("et", "Dordogne’i departemang"), ("eu", "Dordoina"), ("fa", "دوردون"), ("fi", "Dordogne"), ("fr", "Dordogne"), ("gl", "Dordoña"), ("gu", "દોર\u{acd}દોગ\u{acd}ન\u{ac7}"), ("he", "דורדון"), ("hi", "दोर\u{94d}दो\u{902}ग"), ("hu", "Dordogne"), ("hy", "Դորդոն"), ("id", "Dordogne"), ("it", "Dordogna"), ("ja", "ドルドーニュ県"), ("ka", "დორდონი"), ("kk", "Дордонь"), ("kn", "ಡೋರ\u{ccd}ಡೋಗ\u{ccd}ನ\u{cc6}"), ("ko", "도르도뉴 주"), ("lt", "Dordonė"), ("lv", "Dordoņa"), ("mk", "Дордоња"), ("mr", "दोर\u{94d}गोन\u{94d}य"), ("ms", "Dordogne"), ("nb", "Dordogne"), ("nl", "Dordogne"), ("no", "Dordogne"), ("pl", "Dordogne"), ("pt", "Dordonha"), ("ro", "Dordogne"), ("ru", "Дордонь"), ("si", "ඩොර\u{dca}ඩොග\u{dca}නේ"), ("sk", "Dordogne"), ("sl", "Dordogne"), ("sq", "Dordogne"), ("sr", "Дордоња"), ("sr_Latn", "Dordonja"), ("sv", "Dordogne"), ("sw", "Dordogne"), ("ta", "டோர\u{bcd}டோக\u{bcd}னே"), ("te", "డ\u{c3e}ర\u{c4d}డ\u{c4b}న\u{c3f}"), ("th", "จ\u{e31}งหว\u{e31}ดดอร\u{e4c}ดอญ"), ("tr", "Dordogne"), ("uk", "Дордонь"), ("ur", "ڈورڈوین"), ("vi", "Dordogne"), ("yue", "多爾多涅"), ("yue_Hans", "多尔多涅"), ("zh", "多爾多涅省")]),
                        unofficial_name_list: ["Dordogne"].to_vec(),
                    }
                ),
                (
                    "25",
                    Subdivision{
                        name: "25",
                        country_alpha2: Alpha2::FR,
                        code: "25",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.92760000000001), longitude: Some(6.349000999999999), max_latitude: Some(46.9485301), min_latitude: Some(46.9157861), max_longitude: Some(6.3995831), min_longitude: Some(6.330692)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Doubs (département)"), ("ar", "دوبس"), ("az", "Du (departament)"), ("be", "Дэпартамент Ду"), ("bg", "Ду"), ("bn", "ড\u{9c1}বস"), ("ca", "Doubs"), ("ccp", "𑄓\u{1112f}𑄛\u{11134}𑄌\u{11134}"), ("ceb", "Doubs"), ("cs", "Doubs"), ("cy", "Doubs"), ("da", "Doubs"), ("de", "Département Doubs"), ("el", "Ντουμπ"), ("en", "Doubs"), ("es", "Doubs"), ("et", "Doubs’ departemang"), ("eu", "Doubs"), ("fa", "دو"), ("fi", "Doubs"), ("fr", "Doubs"), ("gl", "Doubs"), ("gu", "ડોબ\u{acd}સ"), ("he", "דו"), ("hi", "ड\u{942}ब\u{94d}स"), ("hu", "Doubs"), ("hy", "Դու"), ("id", "Doubs"), ("it", "Doubs"), ("ja", "ドゥー県"), ("ka", "დუ"), ("kk", "Ду"), ("kn", "ಡ\u{ccc}ಬ\u{ccd}ಸ\u{ccd}"), ("ko", "두 주"), ("lt", "Du"), ("lv", "Dū"), ("mr", "द\u{942}ब"), ("ms", "Doubs"), ("nb", "Doubs"), ("nl", "Doubs"), ("no", "Doubs"), ("pl", "Doubs"), ("pt", "Doubs"), ("ro", "Doubs"), ("ru", "Ду"), ("si", "ඩඋබ\u{dca}ස\u{dca}"), ("sk", "Doubs"), ("sl", "Doubs"), ("sq", "Doubs"), ("sr", "Ду"), ("sr_Latn", "Du"), ("sv", "Doubs"), ("sw", "Doubs"), ("ta", "டௌப\u{bcd}ஸ\u{bcd}"), ("te", "డ\u{c4c}బ\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดด\u{e39}"), ("tr", "Doubs"), ("uk", "Ду"), ("ur", "ڈو"), ("vi", "Doubs"), ("yue", "杜"), ("yue_Hans", "杜"), ("zh", "杜省")]),
                        unofficial_name_list: ["Doubs"].to_vec(),
                    }
                ),
                (
                    "26",
                    Subdivision{
                        name: "26",
                        country_alpha2: Alpha2::FR,
                        code: "26",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.73118960000001), longitude: Some(5.2266675), max_latitude: Some(45.343976), min_latitude: Some(44.115494), max_longitude: Some(5.830446), min_longitude: Some(4.6468618)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Drôme"), ("ar", "دروم"), ("az", "Drom (departament)"), ("be", "Дэпартамент Дром"), ("bg", "Дром"), ("bn", "দ\u{9cd}রোম"), ("ca", "Droma"), ("ccp", "𑄓\u{11133}𑄢\u{1112e}𑄟\u{11134}"), ("ceb", "Drôme"), ("cs", "Drôme"), ("cy", "Drôme"), ("da", "Drôme"), ("de", "Département Drôme"), ("el", "Ντρομ"), ("en", "Drôme"), ("es", "Drôme"), ("et", "Drôme’i departemang"), ("eu", "Drôme"), ("fa", "دروم"), ("fi", "Drôme"), ("fr", "Drôme"), ("gl", "Drôme"), ("gu", "ડ\u{acd}રોમ"), ("he", "דרום"), ("hi", "ड\u{94d}रोम\u{947}"), ("hu", "Drôme"), ("hy", "Դրոմ"), ("id", "Drôme"), ("it", "Drôme"), ("ja", "ドローム県"), ("ka", "დრომი"), ("kk", "Дром"), ("kn", "ಡ\u{ccd}ರೊಮ\u{ccd}"), ("ko", "드롬 주"), ("lt", "Dromas"), ("lv", "Droma"), ("mr", "द\u{94d}रोम"), ("ms", "Drôme"), ("nb", "Drôme"), ("nl", "Drôme"), ("no", "Drôme"), ("pl", "Drôme"), ("pt", "Drôme"), ("ro", "Drôme"), ("ru", "Дром"), ("si", "ඩ\u{dca}රෝමේ"), ("sk", "Drôme"), ("sl", "Drôme"), ("sq", "Drôme"), ("sr", "Дром"), ("sr_Latn", "Drom"), ("sv", "Drôme"), ("sw", "Drôme"), ("ta", "ட\u{bcd}ர\u{bbe}ம\u{bcd}"), ("te", "డ\u{c4d}ర\u{c4b}మ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโดรม"), ("tr", "Drôme"), ("uk", "Дром"), ("ur", "ڈروم"), ("vi", "Drôme"), ("yue", "德龍"), ("yue_Hans", "德龙"), ("zh", "德龙省")]),
                        unofficial_name_list: ["Drôme"].to_vec(),
                    }
                ),
                (
                    "27",
                    Subdivision{
                        name: "27",
                        country_alpha2: Alpha2::FR,
                        code: "27",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.11817629999999), longitude: Some(0.9582113999999999), max_latitude: Some(49.485283), min_latitude: Some(48.666427), max_longitude: Some(1.8031109), min_longitude: Some(0.2967251)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Eure"), ("ar", "أور"), ("az", "Er (departament)"), ("be", "Дэпартамент Эр"), ("bg", "Йор"), ("bn", "ইউর"), ("ca", "Eure"), ("ccp", "𑄃\u{11128}𑄅\u{1112a}𑄢\u{11128}"), ("ceb", "Eure"), ("cs", "Eure"), ("cy", "Eure"), ("da", "Eure"), ("de", "Département Eure"), ("el", "Ερ"), ("en", "Eure"), ("es", "Eure"), ("et", "Eure’i departemang"), ("eu", "Eure"), ("fa", "شهرستان اور"), ("fi", "Eure"), ("fr", "Eure"), ("gl", "Eure"), ("gu", "ય\u{ac1}ર\u{ac7}"), ("he", "אר"), ("hi", "य\u{942}र\u{947}"), ("hu", "Eure"), ("hy", "Էր"), ("id", "Eure"), ("it", "Eure"), ("ja", "ウール県"), ("ka", "ერი"), ("kk", "Эр"), ("kn", "ಯುಯ\u{cc2}ರ\u{ccd}"), ("ko", "외르 주"), ("lt", "Eras"), ("lv", "Ēra"), ("mk", "Ер"), ("mr", "य\u{941}र"), ("ms", "Eure"), ("nb", "Eure"), ("nl", "Eure"), ("no", "Eure"), ("pl", "Eure"), ("pt", "Eure"), ("ro", "Eure"), ("ru", "Эр"), ("si", "ඉය\u{dd4}රේ"), ("sk", "Eure"), ("sl", "Eure"), ("sr", "Ер"), ("sr_Latn", "Er"), ("sv", "Eure"), ("sw", "Eure"), ("ta", "ஐரே"), ("te", "యుూర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเออร\u{e4c}"), ("tr", "Eure"), ("uk", "Ер"), ("ur", "اور"), ("vi", "Eure"), ("yue", "厄爾"), ("yue_Hans", "厄尔"), ("zh", "厄尔省")]),
                        unofficial_name_list: ["Eure"].to_vec(),
                    }
                ),
                (
                    "28",
                    Subdivision{
                        name: "28",
                        country_alpha2: Alpha2::FR,
                        code: "28",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.5525242), longitude: Some(1.1989814), max_latitude: Some(48.941029), min_latitude: Some(47.95381800000001), max_longitude: Some(1.99456), min_longitude: Some(0.755676)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Eure-et-Loir"), ("ar", "أور ولوار"), ("az", "Er və Luar"), ("be", "Эр і Луар"), ("bg", "Йор е Лоар"), ("bn", "ইউরে-এট-লয\u{9bc}ের"), ("ca", "Eure i Loir"), ("ccp", "𑄃\u{11128}𑄅\u{1112a}𑄢𑄬-𑄃𑄬𑄖\u{11134}-𑄣\u{11130}𑄢\u{11134}"), ("ceb", "Eure-et-Loir"), ("cs", "Eure-et-Loir"), ("cy", "Eure-et-Loir"), ("da", "Eure-et-Loir"), ("de", "Département Eure-et-Loir"), ("el", "Ερ-ε-Λουάρ"), ("en", "Eure-et-Loir"), ("es", "Eure y Loir"), ("et", "Eure-et-Loiri departemang"), ("eu", "Eure-et-Loir"), ("fa", "اور-ا-لوآر"), ("fi", "Eure-et-Loir"), ("fr", "Eure-et-Loir"), ("gl", "Eure e Loir"), ("gu", "ય\u{ac1}ર\u{ac7}-એટ-લોઈર"), ("he", "אר ולואר"), ("hi", "य\u{942}र\u{947}-एट-लोइर"), ("hu", "Eure-et-Loir"), ("hy", "Էր և Լուար"), ("id", "Eure-et-Loir"), ("is", "Eure-et-Loir"), ("it", "Eure-et-Loir"), ("ka", "ერი და ლუარი"), ("kk", "Эр және Луар"), ("kn", "ಯುರ\u{cc6}-ಎಟ\u{ccd}-ಲೋಯ\u{cbf}ರ\u{ccd}"), ("ko", "외르에루아르 주"), ("lt", "Eras ir Luara"), ("lv", "Ēra un Luāra"), ("mk", "Ер и Лоар"), ("mr", "य\u{941}र-ए-ल\u{941}आर"), ("ms", "Eure-et-Loir"), ("nb", "Eure-et-Loir"), ("nl", "Eure-et-Loir"), ("no", "Eure-et-Loir"), ("pl", "Eure-et-Loir"), ("pt", "Eure-et-Loir"), ("ro", "Eure-et-Loir"), ("ru", "Эр и Луар"), ("si", "ඉය\u{dd4}රේ-එට\u{dca}\u{200c}-ලොය\u{dd2}ර\u{dca}"), ("sk", "Eure-et-Loir"), ("sl", "Eure-et-Loir"), ("sr", "Ер и Лоар"), ("sr_Latn", "Er i Loar"), ("sv", "Eure-et-Loir"), ("sw", "Eure-et-Loir"), ("ta", "யுயிர\u{bcd} -எட\u{bcd} -லோயர\u{bcd}"), ("te", "యూర\u{c4d}-ఎట\u{c4d}-ల\u{c4b}యర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเออเรล\u{e31}วร\u{e4c}"), ("tr", "Eure-et-Loir"), ("uk", "Ер і Луар"), ("ur", "اور-اے-لوار"), ("vi", "Eure-et-Loir"), ("yue", "厄爾－盧華"), ("yue_Hans", "厄尔－卢华"), ("zh", "厄尔-卢瓦尔省")]),
                        unofficial_name_list: ["Eure-et-Loir"].to_vec(),
                    }
                ),
                (
                    "29",
                    Subdivision{
                        name: "29",
                        country_alpha2: Alpha2::FR,
                        code: "29",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.2520249), longitude: Some(-3.9300525), max_latitude: Some(48.7535), min_latitude: Some(47.701242), max_longitude: Some(-3.3866189), min_longitude: Some(-5.141292099999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Finistère"), ("ar", "فنستير"), ("az", "Finister"), ("be", "Фіністэр"), ("bg", "Финистер"), ("bn", "ফিনিস\u{9cd}তে"), ("ca", "Finisterre"), ("ccp", "𑄜\u{11128}𑄚\u{11128}𑄌\u{11134}𑄑𑄢\u{11134}"), ("ceb", "Finistère"), ("cs", "Finistère"), ("cy", "Penn-ar-Bed"), ("da", "Finistère"), ("de", "Département Finistère"), ("el", "Φινιστέρ"), ("en", "Finistère"), ("es", "Finisterre"), ("et", "Finistère"), ("eu", "Finistère"), ("fa", "فینیستر"), ("fi", "Finistère"), ("fr", "Finistère"), ("ga", "Penn-ar-Bed"), ("gl", "Finistère"), ("gu", "ફિનિસ\u{acd}તર"), ("he", "פיניסטר"), ("hi", "फिनिस\u{94d}त\u{947}र"), ("hu", "Finistère"), ("hy", "Ֆինիստեր"), ("id", "Finistère"), ("is", "Finistère"), ("it", "Finistère"), ("ja", "フィニステール県"), ("ka", "ფინისტერი"), ("kk", "Финистер"), ("kn", "ಫ\u{cbf}ನ\u{cbf}ಸ\u{ccd}ಟ\u{ccd}ರ\u{cc6}"), ("ko", "피니스테르 주"), ("lt", "Finisteras"), ("lv", "Finistēra"), ("mk", "Финистер"), ("mr", "फिनिस\u{94d}तर"), ("ms", "Finistère"), ("nb", "Finistère"), ("nl", "Finistère"), ("no", "Finistère"), ("pl", "Finistère"), ("pt", "Finistère"), ("ro", "Finistère"), ("ru", "Финистер"), ("si", "ෆ\u{dd2}න\u{dd2}ස\u{dca}ටේරේ"), ("sk", "Finistère"), ("sl", "Finistère"), ("sq", "Finistère"), ("sr", "Финистер"), ("sr_Latn", "Finister"), ("sv", "Finistère"), ("sw", "Finistère"), ("ta", "பினிஸ\u{bcd}ட\u{bcd}டர\u{bcd}"), ("te", "ఫ\u{c3f}న\u{c3f}స\u{c4d}ట\u{c3f}యర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฟ\u{e35}น\u{e34}สแตร\u{e4c}"), ("tr", "Finistère"), ("uk", "Фіністер"), ("ur", "فینیستیر"), ("vi", "Finistère"), ("yue", "菲尼斯泰爾"), ("yue_Hans", "菲尼斯泰尔"), ("zh", "非尼斯泰尔省")]),
                        unofficial_name_list: ["Finistère"].to_vec(),
                    }
                ),
                (
                    "2A",
                    Subdivision{
                        name: "2A",
                        country_alpha2: Alpha2::FR,
                        code: "2A",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.8102633), longitude: Some(8.9245343), max_latitude: Some(42.381519), min_latitude: Some(41.333543), max_longitude: Some(9.408043000000001), min_longitude: Some(8.539899199999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Corse-du-Sud"), ("ar", "كورسيكا الجنوبية"), ("az", "Cənubi Korsika"), ("be", "Корсіка Паўднёвая"), ("bg", "Южна Корсика"), ("bn", "কোর\u{9cd}স ড\u{9c1} সৌদ"), ("ca", "Còrsega del Sud"), ("ccp", "𑄇\u{1112e}𑄢\u{11134}-𑄓\u{1112a}-𑄥𑄖\u{11134}"), ("ceb", "Corse-du-Sud"), ("cs", "Corse-du-Sud"), ("cy", "Corse-du-Sud"), ("da", "Corse-du-Sud"), ("de", "Département Corse-du-Sud"), ("el", "Νότια Κορσική"), ("en", "Corse-du-Sud"), ("es", "Córcega del Sur"), ("et", "Corse du Sud"), ("eu", "Hego Korsika"), ("fa", "کرس جنوبی"), ("fi", "Corse-du-Sud"), ("fr", "Corse-du-Sud"), ("gl", "Córsega do Sur"), ("gu", "કૉર\u{acd}સ-ડ\u{ac1}-સ\u{ac1}દ"), ("he", "קורסיקה הדרומית"), ("hi", "कोरस\u{947}-ड\u{941}-सड"), ("hr", "Corse-du-Sud"), ("hu", "Corse-du-Sud"), ("hy", "Հարավային Կորսիկա"), ("id", "Corse-du-Sud"), ("it", "Corsica del Sud"), ("ka", "სამხრეთი კორსიკა"), ("kk", "Оңтүстік Корсика"), ("kn", "ಕೋರ\u{ccd}ಸ\u{ccd}-ಡು-ಸುಡ\u{ccd}"), ("ko", "코르스뒤쉬드 주"), ("lt", "Pietų Korsika"), ("lv", "Dienvidkorsika"), ("mr", "कॉर\u{94d}स-द\u{94d}य\u{941}-स\u{941}द"), ("ms", "Corse-du-Sud"), ("nb", "Corse-du-Sud"), ("nl", "Corse-du-Sud"), ("no", "Corse-du-Sud"), ("pl", "Korsyka Południowa"), ("pt", "Córsega do Sul"), ("ro", "Corse-du-Sud"), ("ru", "Южная Корсика"), ("si", "කොර\u{dca}ස\u{dca}-ඩ\u{dd4}-ස\u{dd4}ඩ\u{dca}"), ("sk", "Corse-du-Sud"), ("sl", "Corse-du-Sud"), ("sq", "Corse-du-Sud"), ("sr", "Јужна Корзика"), ("sr_Latn", "Južna Korzika"), ("sv", "Corse-du-Sud"), ("sw", "Corse-du-Sud"), ("ta", "கோர\u{bcd}ஸ\u{bcd} -டு -சுட\u{bcd}"), ("te", "క\u{c4b}ర\u{c4d}స\u{c4d}-డ\u{c4d}యూ-సద\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกอร\u{e4c}ส-ด\u{e39}ว\u{e4c}-ซ\u{e39}ด"), ("tr", "Corse-du-Sud"), ("uk", "Південна Корсика"), ("ur", "کورس-جنوبی"), ("vi", "Corse-du-Sud"), ("yue", "南歌斯嘉"), ("yue_Hans", "南歌斯嘉"), ("zh", "南科西嘉")]),
                        unofficial_name_list: ["Corse-du-Sud"].to_vec(),
                    }
                ),
                (
                    "2B",
                    Subdivision{
                        name: "2B",
                        country_alpha2: Alpha2::FR,
                        code: "2B",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.4097877), longitude: Some(9.2785583), max_latitude: Some(43.0276781), min_latitude: Some(41.832168), max_longitude: Some(9.560067799999999), min_longitude: Some(8.5733069)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Haute-Corse"), ("ar", "كورسيكا العليا"), ("az", "Yuxarı Korsika"), ("be", "Корсіка Верхняя"), ("bg", "От Корс"), ("bn", "হোতে-কোর\u{9cd}স"), ("ca", "Alta Còrsega"), ("ccp", "𑄦𑄅\u{1112a}𑄖\u{11134}-𑄇\u{1112e}𑄢\u{11134}𑄌\u{11134}"), ("ceb", "Haute-Corse"), ("cs", "Haute-Corse"), ("cy", "Haute-Corse"), ("da", "Haute-Corse"), ("de", "Département Haute-Corse"), ("el", "Άνω Κορσική"), ("en", "Haute-Corse"), ("es", "Alta Córcega"), ("et", "Haute-Corse’i departemang"), ("eu", "Korsika Garaia"), ("fa", "کرس شمالی"), ("fi", "Haute-Corse"), ("fr", "Haute-Corse"), ("gl", "Alta Córsega"), ("gu", "હૌટ-કૉર\u{acd}સ"), ("he", "קורסיקה עילית"), ("hi", "ओट-कोरस\u{947}"), ("hr", "Haute-Corse"), ("hu", "Haute-Corse"), ("hy", "Վերին Կորսիկա"), ("id", "Haute-Corse"), ("it", "Alta Corsica"), ("ka", "ჩრდილოეთი კორსიკა"), ("kk", "Жоғарғы Корсика"), ("kn", "ಹಾಟ\u{ccd}-ಕೋರ\u{ccd}ಸ\u{ccd}"), ("ko", "오트코르스 주"), ("lt", "Aukštutinė Korsika"), ("lv", "Augškorsika"), ("mr", "ऑत-कॉर\u{94d}स"), ("ms", "Haute-Corse"), ("nb", "Haute-Corse"), ("nl", "Haute-Corse"), ("no", "Haute-Corse"), ("pl", "Górna Korsyka"), ("pt", "Alta Córsega"), ("ro", "Haute-Corse"), ("ru", "Верхняя Корсика"), ("si", "හෞටේ-කොර\u{dca}ස\u{dca}"), ("sk", "Haute-Corse"), ("sl", "Haute-Corse"), ("sq", "Haute-Corse"), ("sr", "Горња Корзика"), ("sr_Latn", "Gornja Korzika"), ("sv", "Haute-Corse"), ("sw", "Haute-Corse"), ("ta", "ஹூட\u{bcd} -கோர\u{bcd}ஸ\u{bcd}"), ("te", "హ\u{c3e}ట\u{c4d}-క\u{c4b}ర\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโอต-กอร\u{e4c}ส"), ("tr", "Haute-Corse"), ("uk", "Верхня Корсика"), ("ur", "بالائی-کورس"), ("vi", "Haute-Corse"), ("yue", "上歌斯嘉"), ("yue_Hans", "上歌斯嘉"), ("zh", "上科西嘉")]),
                        unofficial_name_list: ["Haute-Corse"].to_vec(),
                    }
                ),
                (
                    "30",
                    Subdivision{
                        name: "30",
                        country_alpha2: Alpha2::FR,
                        code: "30",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.9446996), longitude: Some(4.1513764), max_latitude: Some(44.4596639), min_latitude: Some(43.460159), max_longitude: Some(4.845564), min_longitude: Some(3.261869)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Gard"), ("ar", "غارد"), ("az", "Qar (departament)"), ("be", "Дэпартамент Гар"), ("bg", "Гар"), ("bn", "গ\u{9be}র\u{9cd}ড"), ("ca", "Gard"), ("ccp", "𑄉𑄢\u{11134}𑄓\u{11134}"), ("ceb", "Gard"), ("cs", "Gard"), ("cy", "Gard"), ("da", "Gard"), ("de", "Gard"), ("el", "Γκαρ"), ("en", "Gard"), ("es", "Gard"), ("et", "Gard’i departemang"), ("eu", "Gard"), ("fa", "گار"), ("fi", "Gard"), ("fr", "Gard"), ("gl", "Gard"), ("gu", "ગાર\u{acd}ડ"), ("he", "גאר"), ("hi", "गर\u{94d}ड"), ("hu", "Gard"), ("hy", "Գար"), ("id", "Gard"), ("it", "Gard"), ("ja", "ガール県"), ("ka", "გარი"), ("kk", "Гар"), ("kn", "ಗಾರ\u{ccd}ಡ\u{ccd}"), ("ko", "가르 주"), ("lt", "Garas"), ("lv", "Gāra"), ("mn", "Гар аймаг"), ("mr", "गार\u{94d}द"), ("ms", "Gard"), ("nb", "Gard"), ("nl", "Gard"), ("no", "Gard"), ("pl", "Gard"), ("pt", "Gard"), ("ro", "Gard"), ("ru", "Гар"), ("si", "ග\u{dcf}ර\u{dca}ඩ\u{dca}"), ("sk", "Gard"), ("sl", "Gard"), ("sr", "Гар"), ("sr_Latn", "Gar"), ("sv", "Gard"), ("sw", "Gard"), ("ta", "க\u{bbe}ர\u{bcd}ட\u{bcd}"), ("te", "గ\u{c3e}ర\u{c4d}డ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดการ\u{e4c}"), ("tr", "Gard"), ("uk", "Гар"), ("ur", "گار"), ("vi", "Gard"), ("yue", "加爾"), ("yue_Hans", "加尔"), ("zh", "加尔省")]),
                        unofficial_name_list: ["Gard"].to_vec(),
                    }
                ),
                (
                    "31",
                    Subdivision{
                        name: "31",
                        country_alpha2: Alpha2::FR,
                        code: "31",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.4010462), longitude: Some(1.135302), max_latitude: Some(43.92153099999999), min_latitude: Some(42.6893299), max_longitude: Some(2.048299), min_longitude: Some(0.4416861)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Haute-Garonne"), ("ar", "غارون العليا"), ("be", "Гарона Верхняя"), ("bg", "От Гарон"), ("bn", "হ\u{9be}উটে-গ\u{9be}রনে"), ("ca", "Alta Garona"), ("ccp", "𑄦𑄅\u{1112a}𑄖\u{11134}-𑄉\u{11133}𑄠𑄢\u{1112e}𑄚\u{11133}𑄦\u{11128}"), ("ceb", "Haute-Garonne"), ("cs", "Haute-Garonne"), ("cy", "Haute-Garonne"), ("da", "Haute-Garonne"), ("de", "Département Haute-Garonne"), ("el", "Ωτ-Γκαρόν"), ("en", "Haute-Garonne"), ("es", "Alto Garona"), ("et", "Haute-Garonne"), ("eu", "Garona Garaia"), ("fa", "اوت-گارون"), ("fi", "Haute-Garonne"), ("fr", "Haute-Garonne"), ("gl", "Alta Garona"), ("gu", "હૌટ-ગરોન"), ("he", "גארון עילית"), ("hi", "ओट-गरोन"), ("hu", "Haute-Garonne"), ("hy", "Վերին Գարոն"), ("id", "Haute-Garonne"), ("it", "Alta Garonna"), ("ka", "ზემო გარონა"), ("kk", "Жоғарғы Гаронна"), ("kn", "ಹ\u{ccc}ಟ\u{cc6}-ಗ\u{ccd}ಯಾರೋನ\u{cc6}"), ("ko", "오트가론 주"), ("lt", "Aukštutinė Garona"), ("lv", "Augšgaronna"), ("mr", "ऑत-गारोन"), ("ms", "Haute-Garonne"), ("nb", "Haute-Garonne"), ("nl", "Haute-Garonne"), ("no", "Haute-Garonne"), ("pl", "Górna Garonna"), ("pt", "Alta Garona"), ("ro", "Haute-Garonne"), ("ru", "Верхняя Гаронна"), ("si", "හෞටේ -ගරොන\u{dca}නේ"), ("sk", "Haute-Garonne"), ("sl", "Haute-Garonne"), ("sq", "Haute-Garonne"), ("sr", "Горња Гарона"), ("sr_Latn", "Gornja Garona"), ("sv", "Haute-Garonne"), ("sw", "Haute-Garonne"), ("ta", "ஹூட\u{bcd} -கரோன\u{bcd}"), ("te", "హూట\u{c4d} గ\u{c3e}ర\u{c4b}న\u{c40}"), ("th", "จ\u{e31}งหว\u{e31}ดโอต-การอน"), ("tr", "Haute-Garonne"), ("uk", "Верхня Гаронна"), ("ur", "بالائی-گارون"), ("vi", "Haute-Garonne"), ("yue", "上加龍"), ("yue_Hans", "上加龙"), ("zh", "上加龙省")]),
                        unofficial_name_list: ["Haute-Garonne"].to_vec(),
                    }
                ),
                (
                    "32",
                    Subdivision{
                        name: "32",
                        country_alpha2: Alpha2::FR,
                        code: "32",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.6366479), longitude: Some(0.4502368), max_latitude: Some(44.0800249), min_latitude: Some(43.310868), max_longitude: Some(1.203249), min_longitude: Some(-0.282299)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Gers"), ("ar", "جارس"), ("be", "Дэпартамент Жэр"), ("bg", "Жерс"), ("bn", "গেরস"), ("ca", "Gers"), ("ccp", "𑄉𑄢\u{11134}𑄌\u{11134}"), ("ceb", "Gers"), ("cs", "Gers"), ("cy", "Gers"), ("da", "Gers"), ("de", "Département Gers"), ("el", "Ζερς"), ("en", "Gers"), ("es", "Gers"), ("et", "Gers’i departemang"), ("eu", "Gers"), ("fa", "ژر"), ("fi", "Gers"), ("fr", "Gers"), ("gl", "Gers"), ("gu", "ગ\u{ac7}ર\u{acd}સ"), ("he", "ז׳ר"), ("hi", "ग\u{947}र\u{94d}स"), ("hu", "Gers"), ("hy", "Ժեր"), ("id", "Gers"), ("it", "Gers"), ("ja", "ジェール県"), ("ka", "ჟერი"), ("kk", "Жер"), ("kn", "ಗರ\u{ccd}ಸ\u{ccd}"), ("ko", "제르 주"), ("lt", "Žeras"), ("lv", "Žēra"), ("mr", "ज\u{947}र"), ("ms", "Gers"), ("nb", "Gers"), ("nl", "Gers"), ("no", "Gers"), ("pl", "Gers"), ("pt", "Gers"), ("ro", "Gers"), ("ru", "Жер"), ("si", "ජෙර\u{dca}ස\u{dca}"), ("sk", "Gers"), ("sl", "Gers"), ("sq", "Gers"), ("sr", "Жерс"), ("sr_Latn", "Žers"), ("sv", "Gers"), ("sw", "Gers"), ("ta", "ஜெர\u{bcd}ஸ\u{bcd}"), ("te", "జ\u{c46}ర\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแฌร\u{e4c}"), ("tr", "Gers"), ("uk", "Жер"), ("ur", "جرس"), ("vi", "Gers"), ("yue", "熱爾"), ("yue_Hans", "热尔"), ("zh", "热尔省")]),
                        unofficial_name_list: ["Gers"].to_vec(),
                    }
                ),
                (
                    "33",
                    Subdivision{
                        name: "33",
                        country_alpha2: Alpha2::FR,
                        code: "33",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.84966499999999), longitude: Some(-0.4502368), max_latitude: Some(45.573636), min_latitude: Some(44.1939019), max_longitude: Some(0.315137), min_longitude: Some(-1.2614241)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Gironde"), ("ar", "جيروند"), ("az", "Jirond"), ("be", "Жыронда"), ("bg", "Жиронд"), ("bn", "গিরন\u{9cd}ড"), ("ca", "Gironda"), ("ccp", "𑄉\u{1112d}𑄢\u{1112e}𑄚\u{11134}𑄓𑄬"), ("ceb", "Gironde"), ("cs", "Gironde"), ("cy", "Gironde"), ("da", "Gironde"), ("de", "Département Gironde"), ("el", "Ζιρόντ"), ("en", "Gironde"), ("es", "Gironda"), ("et", "Gironde’i departemang"), ("eu", "Gironda"), ("fa", "ژیروند"), ("fi", "Gironde"), ("fr", "Gironde"), ("gl", "Xironda"), ("gu", "ગિરોન\u{acd}દ"), ("he", "ז׳ירונד"), ("hi", "गिरो\u{902}द\u{947}"), ("hu", "Gironde"), ("hy", "Ժիրոնդ"), ("id", "Gironde"), ("it", "Gironda"), ("ja", "ジロンド県"), ("jv", "Dhépartemèn Gironde"), ("ka", "ჟირონდა"), ("kk", "Жиронда"), ("kn", "ಗ\u{cbf}ರೊಂಡ\u{cc6}"), ("ko", "지롱드 주"), ("lt", "Žironda"), ("lv", "Žironda"), ("mk", "Жиронда (департман)"), ("mr", "जिरो\u{902}द"), ("ms", "Gironde"), ("nb", "Gironde"), ("nl", "Gironde"), ("no", "Gironde"), ("pl", "Żyronda"), ("pt", "Gironda"), ("ro", "Gironde"), ("ru", "Жиронда"), ("si", "ග\u{dd2}රෝන\u{dca}ඩේ"), ("sk", "Gironde"), ("sl", "Gironde"), ("sq", "Gironda"), ("sr", "Жиронда"), ("sr_Latn", "Žironda"), ("sv", "Gironde"), ("sw", "Gironde"), ("ta", "கிர\u{bbe}ண\u{bcd}டே"), ("te", "గ\u{c3f}ర\u{c4b}ండ\u{c46}"), ("th", "จ\u{e31}งหว\u{e31}ดฌ\u{e35}รงด\u{e4c}"), ("tr", "Gironde"), ("uk", "Жиронда"), ("ur", "جیروند"), ("vi", "Gironde"), ("yue", "吉倫特"), ("yue_Hans", "吉伦特"), ("zh", "吉倫特省")]),
                        unofficial_name_list: ["Gironde"].to_vec(),
                    }
                ),
                (
                    "34",
                    Subdivision{
                        name: "34",
                        country_alpha2: Alpha2::FR,
                        code: "34",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.5912356), longitude: Some(3.2583626), max_latitude: Some(43.9727599), min_latitude: Some(43.2102099), max_longitude: Some(4.1945401), min_longitude: Some(2.539549)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hérault"), ("ar", "هيرولت"), ("be", "Дэпартамент Эро"), ("bg", "Еро"), ("bn", "হের\u{9be}ল\u{9cd}ট"), ("bs", "Hérault"), ("ca", "Erau"), ("ccp", "𑄦𑄬𑄢𑄅\u{1112a}𑄣\u{11133}𑄑\u{11134}"), ("ceb", "Hérault"), ("cs", "Hérault"), ("cy", "Hérault"), ("da", "Hérault"), ("de", "Département Hérault"), ("el", "Ερώ"), ("en", "Hérault"), ("es", "Hérault"), ("et", "Hérault’ departemang"), ("eu", "Hérault"), ("fa", "شهرستان ارو، فرانسه"), ("fi", "Hérault"), ("fr", "Hérault"), ("gl", "Hérault"), ("gu", "હ\u{ac7}રોલ\u{acd}ટ"), ("he", "ארו"), ("hi", "ह\u{947}रौल\u{94d}ट"), ("hr", "Hérault"), ("hu", "Hérault"), ("hy", "Էրո"), ("id", "Hérault"), ("it", "Hérault"), ("ja", "エロー県"), ("ka", "ერო"), ("kk", "Эро"), ("kn", "ಹ\u{cc6}ರಾಲ\u{ccd}ಟ\u{ccd}"), ("ko", "에로 주"), ("lt", "Hero"), ("lv", "Ēro"), ("mk", "Еро"), ("mn", "Эро аймаг"), ("mr", "एरॉ"), ("ms", "Hérault"), ("nb", "Hérault"), ("nl", "Hérault"), ("no", "Hérault"), ("pl", "Hérault"), ("pt", "Hérault"), ("ro", "Hérault"), ("ru", "Эро"), ("si", ", හේර\u{dcf}උල\u{dca}ට\u{dca}"), ("sk", "Hérault"), ("sl", "Hérault"), ("sr", "Еро"), ("sr_Latn", "Ero"), ("sv", "Hérault"), ("sw", "Hérault"), ("ta", "ஹெர\u{bbe}ல\u{bcd}"), ("te", "హ\u{c46}ర\u{c3e}ల\u{c4d}ట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเอโร"), ("tr", "Hérault"), ("uk", "Еро"), ("ur", "ایرو"), ("vi", "Hérault"), ("yue", "埃羅"), ("yue_Hans", "埃罗"), ("zh", "埃罗省")]),
                        unofficial_name_list: ["Hérault"].to_vec(),
                    }
                ),
                (
                    "35",
                    Subdivision{
                        name: "35",
                        country_alpha2: Alpha2::FR,
                        code: "35",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.2292016), longitude: Some(-1.5300695), max_latitude: Some(48.721737), min_latitude: Some(47.631614), max_longitude: Some(-1.0156211), min_longitude: Some(-2.289611)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ille-et-Vilaine"), ("ar", "إيل وفيلان"), ("be", "Іль і Вілен"), ("bg", "Ил е Вилен"), ("bn", "ইল\u{9cd}লে-এট-ভিল\u{9be}ইন"), ("ca", "Ille i Vilaine"), ("ccp", "𑄃\u{11128}𑄣\u{11133}𑄦\u{11128}-𑄃𑄬𑄖\u{11134}-𑄞\u{11128}𑄣\u{1112d}𑄚𑄬"), ("ceb", "Ille-et-Vilaine"), ("cs", "Ille-et-Vilaine"), ("cy", "Îl-ha-Gwilun"), ("da", "Ille-et-Vilaine"), ("de", "Département Ille-et-Vilaine"), ("el", "Ιλ-ε-Βιλαίν"), ("en", "Ille-et-Vilaine"), ("es", "Ille y Vilaine"), ("et", "Ille-et-Vilaine’i departemang"), ("eu", "Ille-et-Vilaine"), ("fa", "ایل-ا-ویلن"), ("fi", "Ille-et-Vilaine"), ("fr", "Ille-et-Vilaine"), ("gl", "Ille e Vilaine"), ("gu", "ઈલ\u{ac7}-એટ-વિલ\u{ac8}ન"), ("he", "איל ווילן"), ("hi", "इल-एट-वील\u{947}न"), ("hu", "Ille-et-Vilaine"), ("hy", "Իլ և Վիլեն"), ("id", "Ille-et-Vilaine"), ("is", "Ille-et-Vilaine"), ("it", "Ille-et-Vilaine"), ("ka", "ილი და ვილენი"), ("kk", "Иль және Вилен"), ("kn", "ಇಲ\u{cc6}-ಎಟ\u{ccd}-ವ\u{cbf}ಲೇನ\u{ccd}"), ("ko", "일에빌렌 주"), ("lt", "Ilis ir Vilenas"), ("lv", "Ila un Vilēna"), ("mr", "इल-ए-व\u{94d}हिल\u{947}न"), ("ms", "Ille-et-Vilaine"), ("nb", "Ille-et-Vilaine"), ("nl", "Ille-et-Vilaine"), ("no", "Ille-et-Vilaine"), ("pl", "Ille-et-Vilaine"), ("pt", "Ille-et-Vilaine"), ("ro", "Ille-et-Vilaine"), ("ru", "Иль и Вилен"), ("si", "ඉල\u{dca}ලෙ-එට\u{dca}-ව\u{dd2}ලය\u{dd2}නේ"), ("sk", "Ille-et-Vilaine"), ("sl", "Ille-et-Vilaine"), ("sq", "Ille-et-Vilaine"), ("sr", "Ил и Вилен"), ("sr_Latn", "Il i Vilen"), ("sv", "Ille-et-Vilaine"), ("sw", "Ille-et-Vilaine"), ("ta", "இல\u{bcd}லே -எட\u{bcd} -விலைனே"), ("te", "ఇల\u{c4d}ల\u{c46}-ఎట\u{c4d}-వ\u{c3f}ల\u{c47}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e35}เลว\u{e35}แลน"), ("tr", "Ille-et-Vilaine"), ("uk", "Іль і Вілен"), ("ur", "ایل-اے-ویلن"), ("vi", "Ille-et-Vilaine"), ("yue", "伊勒-維萊訥"), ("yue_Hans", "伊勒-维莱讷"), ("zh", "伊勒-维莱讷省")]),
                        unofficial_name_list: ["Ille-et-Vilaine"].to_vec(),
                    }
                ),
                (
                    "36",
                    Subdivision{
                        name: "36",
                        country_alpha2: Alpha2::FR,
                        code: "36",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.6613966), longitude: Some(1.4482662), max_latitude: Some(47.277465), min_latitude: Some(46.3469059), max_longitude: Some(2.204572), min_longitude: Some(0.8674139)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Indre (département)"), ("ar", "أندر"), ("az", "Endr"), ("be", "Дэпартамент Эндр"), ("bg", "Ендър"), ("bn", "ইনড\u{9cd}রে"), ("ca", "Indre"), ("ccp", "𑄃\u{11128}𑄚\u{11134}𑄓\u{11133}𑄢𑄬"), ("ceb", "Indre"), ("cs", "Indre"), ("cy", "Indre"), ("da", "Indre"), ("de", "Département Indre"), ("el", "Αντρ"), ("en", "Indre"), ("es", "Indre"), ("et", "Indre’i departemang"), ("eu", "Indre"), ("fa", "اندر"), ("fi", "Indre"), ("fr", "Indre"), ("gl", "Indre"), ("gu", "ઈન\u{acd}ડ\u{acd}ર\u{ac7}"), ("he", "אנדר"), ("hi", "आइन\u{94d}द\u{94d}र\u{947}"), ("hu", "Indre"), ("hy", "Էնդր"), ("id", "Indre"), ("is", "Indre"), ("it", "Indre"), ("ja", "アンドル県"), ("ka", "ენდრი"), ("kk", "Эндр"), ("kn", "ಇಂದರ\u{ccd}"), ("ko", "앵드르 주"), ("lt", "Endras"), ("lv", "Endra"), ("mr", "ए\u{902}द\u{94d}र"), ("ms", "Indre"), ("nb", "Indre"), ("nl", "Indre"), ("no", "Indre"), ("pl", "Indre"), ("pt", "Indre"), ("ro", "Indre"), ("ru", "Эндр"), ("si", "ඉන\u{dca}ඩ\u{dca}රේ"), ("sk", "Indre"), ("sl", "Indre"), ("sq", "Indre"), ("sr", "Ендр"), ("sr_Latn", "Endr"), ("sv", "Indre"), ("sw", "Indre"), ("ta", "இன\u{bcd}றே"), ("te", "ఇండ\u{c4d}ర\u{c46}"), ("th", "จ\u{e31}งหว\u{e31}ดแอ\u{e47}งดร\u{e4c}"), ("tr", "Indre"), ("uk", "Ендр"), ("ur", "آندر"), ("vi", "Indre"), ("yue", "安德爾"), ("yue_Hans", "安德尔"), ("zh", "安德尔省")]),
                        unofficial_name_list: ["Indre"].to_vec(),
                    }
                ),
                (
                    "37",
                    Subdivision{
                        name: "37",
                        country_alpha2: Alpha2::FR,
                        code: "37",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.28949249999999), longitude: Some(0.816097), max_latitude: Some(47.7098679), min_latitude: Some(46.736714), max_longitude: Some(1.366049), min_longitude: Some(0.0527369)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Indre-et-Loire"), ("ar", "أندر ولوار"), ("be", "Эндр і Луара"), ("bg", "Ендър е Лоар"), ("bn", "ইন\u{9cd}দ\u{9cd}রে-এট-লয\u{9bc}ের"), ("ca", "Indre i Loira"), ("ccp", "𑄃\u{11128}𑄚\u{11134}𑄓\u{11133}𑄢𑄬-𑄃𑄬𑄖\u{11134}-𑄣\u{11130}𑄢\u{11134}"), ("ceb", "Indre-et-Loire"), ("cs", "Indre-et-Loire"), ("cy", "Indre-et-Loire"), ("da", "Indre-et-Loire"), ("de", "Département Indre-et-Loire"), ("el", "Αντρ-ε-Λουάρ"), ("en", "Indre-et-Loire"), ("es", "Indre y Loira"), ("et", "Indre-et-Loire’i departemang"), ("eu", "Indre-et-Loire"), ("fa", "اندر الوآر"), ("fi", "Indre-et-Loire"), ("fr", "Indre-et-Loire"), ("gl", "Indre e Loira"), ("gu", "એન\u{acd}ડ\u{acd}ર\u{ac7}-એટ-લોઈર"), ("he", "אנדר ולואר"), ("hi", "आ\u{902}द\u{94d}र\u{947}-एट-लोइर"), ("hu", "Indre-et-Loire"), ("hy", "Էնդր և Լուար"), ("id", "Indre-et-Loire"), ("is", "Indre-et-Loire"), ("it", "Indre e Loira"), ("ka", "ენდრი და ლუარა"), ("kk", "Эндр және Луара"), ("kn", "ಇಂಡ\u{ccd}ರ\u{cc6}-ಎಟ\u{ccd}-ಲೋರ\u{cc6}"), ("ko", "앵드르에루아르 주"), ("lt", "Endras ir Luara"), ("lv", "Endra un Luāra"), ("mk", "Ендр и Лоара"), ("mr", "ए\u{902}द\u{94d}र-ए-लावार"), ("ms", "Indre-et-Loire"), ("nb", "Indre-et-Loire"), ("nl", "Indre-et-Loire"), ("no", "Indre-et-Loire"), ("pl", "Indre-et-Loire"), ("pt", "Indre-et-Loire"), ("ro", "Indre-et-Loire"), ("ru", "Эндр и Луара"), ("si", "ඉන\u{dca}ද\u{dca}\u{200d}රේ-එට\u{dca}-ලොය\u{dd2}රේ"), ("sk", "Indre-et-Loire"), ("sl", "Indre-et-Loire"), ("sq", "Indre-et-Loire"), ("sr", "Ендр и Лоара"), ("sr_Latn", "Endr i Loara"), ("sv", "Indre-et-Loire"), ("sw", "Indre-et-Loire"), ("ta", "இன\u{bcd}றே -எட\u{bcd} -லோஇரே"), ("te", "ఇండ\u{c4d}ర\u{c46}-ఎట\u{c4d}-ల\u{c4b}య\u{c3f}ర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแอ\u{e47}งเดรล\u{e31}วร\u{e4c}"), ("tr", "Indre-et-Loire"), ("uk", "Ендр і Луара"), ("ur", "آندر-اے-لوار"), ("vi", "Indre-et-Loire"), ("yue", "安德爾-盧華爾"), ("yue_Hans", "安德尔-卢华尔"), ("zh", "安德尔-卢瓦尔省")]),
                        unofficial_name_list: ["Indre-et-Loire"].to_vec(),
                    }
                ),
                (
                    "38",
                    Subdivision{
                        name: "38",
                        country_alpha2: Alpha2::FR,
                        code: "38",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.9957745), longitude: Some(5.9293476), max_latitude: Some(45.883397), min_latitude: Some(44.695873), max_longitude: Some(6.359309), min_longitude: Some(4.7425939)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Isère"), ("ar", "إزار"), ("be", "Дэпартамент Ізер"), ("bg", "Изер"), ("bn", "ইসে"), ("ca", "Isèra"), ("ccp", "𑄃\u{1112d}𑄥𑄢\u{11134}"), ("ceb", "Isère"), ("cs", "Isère"), ("cy", "Isère"), ("da", "Isère"), ("de", "Département Isère"), ("el", "Ιζέρ"), ("en", "Isère"), ("es", "Isère"), ("et", "Isère’i departemang"), ("eu", "Isère"), ("fa", "ایزر"), ("fi", "Isère"), ("fr", "Isère"), ("gl", "Isère"), ("gu", "ઇસ\u{ac7}ર"), ("he", "איזר"), ("hi", "इस\u{947}र\u{947}"), ("hu", "Isère"), ("hy", "Իզեր"), ("id", "Isère"), ("it", "Isère"), ("ja", "イゼール県"), ("ka", "იზერი"), ("kk", "Изер"), ("kn", "ಇಶ\u{cc6}ರ\u{ccd}"), ("ko", "이제르 주"), ("lt", "Izeras"), ("lv", "Izēra"), ("mr", "इझ\u{947}र"), ("ms", "Isère"), ("nb", "Isère"), ("nl", "Isère"), ("no", "Isère"), ("pl", "Isère"), ("pt", "Isère"), ("ro", "Isère"), ("ru", "Изер"), ("si", "ඉසේරේ"), ("sk", "Isère"), ("sl", "Isère"), ("sq", "Isère"), ("sr", "Изер"), ("sr_Latn", "Izer"), ("sv", "Isère"), ("sw", "Isère"), ("ta", "இசேரே"), ("te", "ఇస\u{c3f}యర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e35}แซร\u{e4c}"), ("tr", "Isère"), ("uk", "Ізер"), ("ur", "ایزار"), ("vi", "Isère"), ("yue", "伊澤爾"), ("yue_Hans", "伊泽尔"), ("zh", "伊泽尔省")]),
                        unofficial_name_list: ["Isère"].to_vec(),
                    }
                ),
                (
                    "39",
                    Subdivision{
                        name: "39",
                        country_alpha2: Alpha2::FR,
                        code: "39",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.76247499999999), longitude: Some(5.6729159), max_latitude: Some(47.30594379999999), min_latitude: Some(46.260695), max_longitude: Some(6.207189), min_longitude: Some(5.251316)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Jura (département)"), ("ar", "جورا"), ("be", "Дэпартамент Юра"), ("bg", "Жура"), ("bn", "জ\u{9c1}র\u{9be}"), ("ca", "Jura"), ("ccp", "𑄎\u{1112a}𑄢"), ("ceb", "Jura"), ("cs", "Jura"), ("cy", "Jura"), ("da", "Jura"), ("de", "Département Jura"), ("el", "Ζυρά"), ("en", "Jura"), ("es", "Jura"), ("et", "Jura departemang"), ("eu", "Jura"), ("fa", "ژورا"), ("fi", "Jura"), ("fr", "Jura"), ("gl", "Jura"), ("gu", "જ\u{ac1}રા"), ("he", "ז׳ורה"), ("hi", "ज\u{93c}\u{941}रा"), ("hu", "Jura"), ("hy", "Յուրա"), ("id", "Jura"), ("it", "Giura"), ("ja", "ジュラ県"), ("ka", "იურა"), ("kk", "Юра"), ("kn", "ಜ\u{cc2}ರಾ"), ("ko", "쥐라 주"), ("lt", "Jura"), ("lv", "Jura"), ("mr", "श\u{94d}य\u{941}र\u{945}"), ("ms", "Jura"), ("nb", "Jura"), ("nl", "Jura"), ("no", "Jura"), ("pl", "Jura"), ("pt", "Jura"), ("ro", "Jura"), ("ru", "Юра"), ("si", "ජ\u{dd4}ර\u{dcf}"), ("sk", "Jura"), ("sl", "Jura"), ("sq", "Jura"), ("sr", "Јура"), ("sr_Latn", "Jura"), ("sv", "Jura"), ("sw", "Jura"), ("ta", "ஜூர\u{bbe}"), ("te", "జూర\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดฌ\u{e39}ว\u{e4c}รา"), ("tr", "Jura"), ("uk", "Жура"), ("ur", "جورا"), ("vi", "Jura, Franche-Comté"), ("yue", "侏羅省"), ("yue_Hans", "侏罗省"), ("zh", "汝拉省")]),
                        unofficial_name_list: ["Jura"].to_vec(),
                    }
                ),
                (
                    "40",
                    Subdivision{
                        name: "40",
                        country_alpha2: Alpha2::FR,
                        code: "40",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.9412045), longitude: Some(-0.7532808999999999), max_latitude: Some(44.532381), min_latitude: Some(43.48743), max_longitude: Some(0.1366911), min_longitude: Some(-1.5235748)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Landes"), ("ar", "لاند"), ("be", "Дэпартамент Ланды"), ("bg", "Ланд"), ("bn", "ল\u{9be}ন\u{9cd}ডেশ"), ("ca", "Landes"), ("ccp", "𑄣\u{11133}𑄠𑄚\u{11134}𑄓𑄬𑄌\u{11134}"), ("ceb", "Landes"), ("cs", "Landes"), ("cy", "Landes"), ("da", "Landes"), ("de", "Département Landes"), ("el", "Λαντ"), ("en", "Landes"), ("es", "Landas"), ("et", "Landes’i departemang"), ("eu", "Landak"), ("fa", "لاند"), ("fi", "Landes"), ("fr", "Landes"), ("gl", "Landes"), ("gu", "લા\u{a82}દ\u{ac7}સ"), ("he", "לונד"), ("hi", "ल\u{948}\u{902}ड\u{947}स"), ("hu", "Landes"), ("hy", "Լանդեր"), ("id", "Landes"), ("it", "Landes"), ("ja", "ランド県"), ("ka", "ლანდი"), ("kk", "Ландтер"), ("kn", "ಲ\u{ccd}ಯಾಂಡ\u{cc6}ಸ\u{ccd}"), ("ko", "랑드 주"), ("lt", "Landai"), ("lv", "Landa"), ("mk", "Ланд (департман)"), ("mr", "ला\u{902}द\u{947}स"), ("ms", "Landes"), ("nb", "Landes"), ("nl", "Landes"), ("no", "Landes"), ("pl", "Landy"), ("pt", "Landes"), ("ro", "Landes"), ("ru", "Ланды"), ("si", "ලන\u{dca}ඩ\u{dd2}ස\u{dca}"), ("sk", "Landes"), ("sl", "Landes"), ("sq", "Landes"), ("sr", "Ланд"), ("sr_Latn", "Land"), ("sv", "Landes"), ("sw", "Landes"), ("ta", "லேண\u{bcd}ட\u{bcd}ஸ\u{bcd}"), ("te", "ల\u{c3e}ండ\u{c46}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดล\u{e47}องด\u{e4c}"), ("tr", "Landes"), ("uk", "Ланди"), ("ur", "لانڈیس"), ("vi", "Landes"), ("yue", "朗德省"), ("yue_Hans", "朗德省"), ("zh", "朗德省")]),
                        unofficial_name_list: ["Landes"].to_vec(),
                    }
                ),
                (
                    "41",
                    Subdivision{
                        name: "41",
                        country_alpha2: Alpha2::FR,
                        code: "41",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.6761905), longitude: Some(1.4159072), max_latitude: Some(48.133235), min_latitude: Some(47.186391), max_longitude: Some(2.247891), min_longitude: Some(0.5804869)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Loir-et-Cher"), ("ar", "لوار وشير"), ("be", "Луар і Шэр"), ("bg", "Лоар е Шер"), ("bn", "লয\u{9bc}ের-এত-চের"), ("ca", "Loir i Cher"), ("ccp", "𑄣\u{11130}𑄢\u{11134}-𑄃𑄬𑄖\u{11134}-𑄌𑄬𑄢\u{11134}"), ("ceb", "Loir-et-Cher"), ("cs", "Loir-et-Cher"), ("cy", "Loir-et-Cher"), ("da", "Loir-et-Cher"), ("de", "Département Loir-et-Cher"), ("el", "Λουάρ-ε-Σερ"), ("en", "Loir-et-Cher"), ("es", "Loir y Cher"), ("et", "Loir-et-Cheri departemang"), ("eu", "Loir-et-Cher"), ("fa", "لوآر-ا-شر"), ("fi", "Loir-et-Cher"), ("fr", "Loir-et-Cher"), ("gl", "Loir e Cher"), ("gu", "લોઈર-એટ-ચ\u{ac7}ર"), ("he", "לואר ושר"), ("hi", "लोइर-एट-च\u{947}र"), ("hu", "Loir-et-Cher"), ("hy", "Լուար և Շեր"), ("id", "Loir-et-Cher"), ("is", "Loir-et-Cher"), ("it", "Loir-et-Cher"), ("ka", "ლუარი და შერი"), ("kk", "Луар және Шер"), ("kn", "ಲೋಯ\u{cbf}ರ\u{ccd}-ಎಟ\u{ccd}-ಚ\u{cc6}ರ\u{ccd}"), ("ko", "루아르에셰르 주"), ("lt", "Luaras ir Šeras"), ("lv", "Luāra un Šēra"), ("mk", "Лоар и Шер"), ("mr", "ल\u{941}आर-ए-श\u{947}र"), ("ms", "Loir-et-Cher"), ("nb", "Loir-et-Cher"), ("nl", "Loir-et-Cher"), ("no", "Loir-et-Cher"), ("pl", "Loir-et-Cher"), ("pt", "Loir-et-Cher"), ("ro", "Loir-et-Cher"), ("ru", "Луар и Шер"), ("si", "ලයර\u{dca}-එට\u{dca}-චෙර\u{dca}"), ("sk", "Loir-et-Cher"), ("sl", "Loir-et-Cher"), ("sr", "Лоар и Шер"), ("sr_Latn", "Loar i Šer"), ("sv", "Loir-et-Cher"), ("sw", "Loir-et-Cher"), ("ta", "லோயர\u{bcd}-எட\u{bcd} -செர\u{bcd}"), ("te", "ల\u{c4b}య\u{c3f}ర\u{c4d}-ఎట\u{c4d}-చ\u{c46}ర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดล\u{e31}วเรแชร\u{e4c}"), ("tr", "Loir-et-Cher"), ("uk", "Луар і Шер"), ("ur", "لوار-اے-شر"), ("vi", "Loir-et-Cher"), ("yue", "盧華－些爾"), ("yue_Hans", "卢华－些尔"), ("zh", "卢瓦尔-谢尔省")]),
                        unofficial_name_list: ["Loir-et-Cher"].to_vec(),
                    }
                ),
                (
                    "42",
                    Subdivision{
                        name: "42",
                        country_alpha2: Alpha2::FR,
                        code: "42",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.9846475), longitude: Some(4.052544999999999), max_latitude: Some(46.276589), min_latitude: Some(45.23104), max_longitude: Some(4.760375), min_longitude: Some(3.688893)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Loire"), ("ar", "لوار"), ("az", "Luara (departament)"), ("be", "Дэпартамент Луара"), ("bg", "Лоар"), ("bn", "লইর\u{9be}"), ("ca", "Loira"), ("ccp", "𑄣\u{11130}𑄢\u{11134}"), ("ceb", "Loire"), ("cs", "Loire"), ("cy", "Loire"), ("da", "Loire"), ("de", "Département Loire"), ("el", "Λουάρ"), ("en", "Loire"), ("es", "Loira"), ("et", "Loire’i departemang"), ("eu", "Loire"), ("fa", "لوآر"), ("fi", "Loire"), ("fr", "Loire"), ("gl", "Loira, Francia"), ("gu", "લોઈર\u{ac7}"), ("he", "לואר"), ("hi", "लोयर"), ("hr", "Loire"), ("hu", "Loire"), ("hy", "Լուար"), ("id", "Loire"), ("it", "Loira"), ("ja", "ロワール県"), ("ka", "ლუარა"), ("kk", "Луара"), ("kn", "ಲೋಯರ\u{ccd}"), ("ko", "루아르 주"), ("lt", "Luara"), ("lv", "Luāra"), ("mr", "लावार"), ("ms", "Loire"), ("nb", "Loire"), ("nl", "Loire"), ("no", "Loire"), ("pl", "Loara"), ("pt", "Loire"), ("ro", "Loire"), ("ru", "Луара"), ("si", "ලොය\u{dd2}රේ"), ("sk", "Loire"), ("sl", "Loire"), ("sr", "Лоара"), ("sr_Latn", "Loara"), ("sv", "Loire"), ("sw", "Loire"), ("ta", "லோயர\u{bcd}"), ("te", "ల\u{c4b}య\u{c3f}ర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดล\u{e31}วร\u{e4c}"), ("tr", "Loire"), ("uk", "Луара"), ("ur", "لویری"), ("vi", "Loire"), ("yue", "盧華爾"), ("yue_Hans", "卢华尔"), ("zh", "卢瓦尔省")]),
                        unofficial_name_list: ["Loire"].to_vec(),
                    }
                ),
                (
                    "43",
                    Subdivision{
                        name: "43",
                        country_alpha2: Alpha2::FR,
                        code: "43",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.0821226), longitude: Some(3.9266366), max_latitude: Some(45.42760879999999), min_latitude: Some(44.743961), max_longitude: Some(4.490819), min_longitude: Some(3.082197)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Haute-Loire"), ("ar", "لوار العليا"), ("az", "Ot-Luağ"), ("be", "Дэпартамент Луара Верхняя"), ("bg", "От Лоар"), ("bn", "হ\u{9be}উততে ল\u{9c1}ইর"), ("ca", "Alt Loira"), ("ccp", "𑄦𑄅\u{1112a}𑄖\u{11134}-𑄣\u{11130}𑄢\u{11134}"), ("ceb", "Haute-Loire"), ("cs", "Haute-Loire"), ("cy", "Haute-Loire"), ("da", "Haute-Loire"), ("de", "Département Haute-Loire"), ("el", "Ωτ-Λουάρ"), ("en", "Haute-Loire"), ("es", "Alto Loira"), ("et", "Haute-Loire’i departemang"), ("eu", "Haute-Loire"), ("fa", "اوت-لوار"), ("fi", "Haute-Loire"), ("fr", "Haute-Loire"), ("gl", "Alto Loira"), ("gu", "હૌટ-લોઈર"), ("he", "לואר עילי"), ("hi", "ओट-लोइर"), ("hr", "Haute-Loire"), ("hu", "Haute-Loire"), ("hy", "Վերին Լուար"), ("id", "Haute-Loire"), ("it", "Alta Loira"), ("ka", "ზემო ლუარა"), ("kk", "Жоғарғы Луара"), ("kn", "ಹಾಟ\u{cc6}-ಲೋಯರ\u{ccd}"), ("ko", "오트루아르 주"), ("lt", "Aukštutinė Luara"), ("lv", "Augšluāra"), ("mr", "ऑत-लावार"), ("ms", "Haute-Loire"), ("nb", "Haute-Loire"), ("nl", "Haute-Loire"), ("no", "Haute-Loire"), ("pl", "Górna Loara"), ("pt", "Haute-Loire"), ("ro", "Haute-Loire"), ("ru", "Верхняя Луара"), ("si", "හෞටේ-ලොය\u{dd2}රේ"), ("sk", "Haute-Loire"), ("sl", "Haute-Loire"), ("sq", "Haute-Loire"), ("sr", "Горња Лоара"), ("sr_Latn", "Gornja Loara"), ("sv", "Haute-Loire"), ("sw", "Haute-Loire"), ("ta", "ஹூட\u{bcd} -லோயிரே"), ("te", "హ\u{c3e}ట\u{c4d}-ల\u{c4b}య\u{c3f}ర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโอต-ล\u{e31}วร\u{e4c}"), ("tr", "Haute-Loire"), ("uk", "Верхня Луара"), ("ur", "بالائی-لوار"), ("vi", "Haute-Loire"), ("yue", "上盧華爾"), ("yue_Hans", "上卢华尔"), ("zh", "上盧瓦爾省")]),
                        unofficial_name_list: ["Haute-Loire"].to_vec(),
                    }
                ),
                (
                    "44",
                    Subdivision{
                        name: "44",
                        country_alpha2: Alpha2::FR,
                        code: "44",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.27804680000001), longitude: Some(-1.8157647), max_latitude: Some(47.835927), min_latitude: Some(46.860073), max_longitude: Some(-0.9243831), min_longitude: Some(-2.559609)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Loire-Atlantique"), ("ar", "لوار الأطلسية"), ("az", "Luar-Atlantik"), ("be", "Луара Атлантычная"), ("bg", "Лоар Атлантик"), ("bn", "লইরে-আটল\u{9be}ন\u{9cd}তিক"), ("ca", "Loira Atlàntic"), ("ccp", "𑄣\u{11130}𑄢\u{11134}-𑄃𑄖\u{11134}𑄣\u{11133}𑄠𑄚\u{11134}𑄑\u{11128}𑄇\u{11134}"), ("ceb", "Loire-Atlantique"), ("cs", "Loire-Atlantique"), ("cy", "Loire-Atlantique"), ("da", "Loire-Atlantique"), ("de", "Département Loire-Atlantique"), ("el", "Λουάρ-Ατλαντίκ"), ("en", "Loire-Atlantique"), ("es", "Loira Atlántico"), ("et", "Loire-Atlantique"), ("eu", "Loire-Atlantique"), ("fa", "لوآر-آتلانتیک"), ("fi", "Loire-Atlantique"), ("fr", "Loire-Atlantique"), ("gl", "Loira Atlántico"), ("gu", "લોઅર-એટલાન\u{acd}ટિક"), ("he", "הלואר האטלנטי"), ("hi", "लोइर-अटला\u{902}टिक"), ("hu", "Loire-Atlantique"), ("hy", "Ատլանտյան Լուար"), ("id", "Loire-Atlantique"), ("it", "Loira Atlantica"), ("ka", "ატლანტიკის ლუარა"), ("kk", "Атлантикалық Луара"), ("kn", "ಲೊಯ\u{cbf}ರ\u{ccd}-ಅಟ\u{ccd}ಲಾಂಟ\u{cbf}ಕ\u{ccd}"), ("ko", "루아르아틀랑티크 주"), ("lt", "Atlanto Luara"), ("lv", "Atlantijas Luāra"), ("mr", "लावार-अतला\u{902}तिक"), ("ms", "Loire-Atlantique"), ("nb", "Loire-Atlantique"), ("nl", "Loire-Atlantique"), ("no", "Loire-Atlantique"), ("pl", "Loara Atlantycka"), ("pt", "Loire-Atlantique"), ("ro", "Loire-Atlantique"), ("ru", "Атлантическая Луара"), ("si", "ලොය\u{dd2}රේ-අත\u{dca}ල\u{dcf}න\u{dca}ත\u{dd2}ක\u{dca}"), ("sk", "Loire-Atlantique"), ("sl", "Loire-Atlantique"), ("sq", "Loire-Atlantique"), ("sr", "Атлантска Лоара"), ("sr_Latn", "Atlantska Loara"), ("sv", "Loire-Atlantique"), ("sw", "Loire-Atlantique"), ("te", "ల\u{c4b}యర\u{c4d}-అట\u{c4d}ల\u{c3e}ంట\u{c3f}క\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดล\u{e31}วร\u{e31}ตล\u{e47}องต\u{e34}ก"), ("tr", "Loire-Atlantique"), ("uk", "Атлантична Луара"), ("ur", "لوار-اتلانتیک"), ("vi", "Loire-Atlantique"), ("yue", "盧華爾-大西洋"), ("yue_Hans", "卢华尔-大西洋"), ("zh", "大西洋卢瓦尔省")]),
                        unofficial_name_list: ["Loire-Atlantique"].to_vec(),
                    }
                ),
                (
                    "45",
                    Subdivision{
                        name: "45",
                        country_alpha2: Alpha2::FR,
                        code: "45",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.900771), longitude: Some(2.2018172), max_latitude: Some(48.344954), min_latitude: Some(47.483027), max_longitude: Some(3.1284099), min_longitude: Some(1.51145)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Loiret"), ("ar", "لواريت"), ("az", "Luare (departament)"), ("be", "Луарэ"), ("bg", "Лоаре"), ("bn", "লইরেত"), ("ca", "Loiret"), ("ccp", "𑄣\u{11130}𑄢𑄬𑄖\u{11134}"), ("ceb", "Loiret"), ("cs", "Loiret"), ("cy", "Loiret"), ("da", "Loiret"), ("de", "Département Loiret"), ("el", "Λουαρέ"), ("en", "Loiret"), ("es", "Loiret"), ("et", "Loiret’ departemang"), ("eu", "Loiret"), ("fa", "لوآره"), ("fi", "Loiret"), ("fr", "Loiret"), ("gl", "Loiret"), ("gu", "લોઈર\u{ac7}ટ"), ("he", "לוארה"), ("hi", "लोइर\u{947}ट"), ("hu", "Loiret"), ("hy", "Լուարե"), ("id", "Loiret"), ("is", "Loiret"), ("it", "Loiret"), ("ja", "ロワレ県"), ("ka", "ლუარე"), ("kk", "Луаре"), ("kn", "ಲೊರ\u{cc6}ಟ\u{ccd}"), ("ko", "루아레 주"), ("lt", "Luarė"), ("lv", "Luarē"), ("mr", "ल\u{941}आर\u{947}"), ("ms", "Loiret"), ("nb", "Loiret"), ("nl", "Loiret"), ("no", "Loiret"), ("pl", "Loiret"), ("pt", "Loiret"), ("ro", "Loiret"), ("ru", "Луаре"), ("si", "ලොය\u{dd2}රෙට\u{dca}"), ("sk", "Loiret"), ("sl", "Loiret"), ("sq", "Loiret"), ("sr", "Лоаре"), ("sr_Latn", "Loare"), ("sv", "Loiret"), ("sw", "Loiret"), ("ta", "லோஇரேட\u{bcd}"), ("te", "ల\u{c4b}య\u{c3f}ర\u{c46}ట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดล\u{e31}วแร"), ("tr", "Loiret"), ("uk", "Луаре"), ("ur", "لوارے"), ("vi", "Loiret"), ("yue", "盧華雷"), ("yue_Hans", "卢华雷"), ("zh", "卢瓦雷省")]),
                        unofficial_name_list: ["Loiret"].to_vec(),
                    }
                ),
                (
                    "46",
                    Subdivision{
                        name: "46",
                        country_alpha2: Alpha2::FR,
                        code: "46",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.5379358), longitude: Some(1.6760691), max_latitude: Some(45.046684), min_latitude: Some(44.20334810000001), max_longitude: Some(2.210891), min_longitude: Some(0.9815220999999998)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Lot"), ("ar", "لوت"), ("az", "Lo (departament)"), ("be", "Дэпартамент Ло"), ("bg", "Лот"), ("bn", "লট"), ("ca", "Òlt"), ("ccp", "𑄢\u{11127}𑄖\u{11134}"), ("ceb", "Lot"), ("cs", "Lot"), ("cy", "Lot"), ("da", "Lot"), ("de", "Département Lot"), ("el", "Λο"), ("en", "Lot"), ("es", "Lot"), ("et", "Loti departemang"), ("eu", "Lot"), ("fa", "لو"), ("fi", "Lot"), ("fr", "Lot"), ("gl", "Lot"), ("gu", "લોટ"), ("he", "לוט"), ("hi", "लॉट"), ("hu", "Lot"), ("hy", "Լո"), ("id", "Lot"), ("it", "Lot"), ("ja", "ロット県"), ("ka", "ლო"), ("kk", "Ло"), ("kn", "ಲಾಟ\u{ccd}"), ("ko", "로트 주"), ("lt", "Lo"), ("lv", "Lo"), ("mk", "Лот"), ("mr", "लॉत"), ("ms", "Lot"), ("nb", "Lot"), ("nl", "Lot"), ("no", "Lot"), ("pl", "Lot"), ("pt", "Lot"), ("ro", "Lot"), ("ru", "Ло"), ("si", "ලොට\u{dca}"), ("sk", "Lot"), ("sl", "Lot"), ("sr", "Лот"), ("sr_Latn", "Lot"), ("sv", "Lot"), ("sw", "Lot"), ("ta", "லோட\u{bcd}"), ("te", "ల\u{c4b}ట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดล\u{e47}อต"), ("tr", "Lot"), ("uk", "Лот"), ("ur", "لوت"), ("vi", "Lot, Midi-Pyrénées"), ("yue", "洛特"), ("yue_Hans", "洛特"), ("zh", "洛特省")]),
                        unofficial_name_list: ["Lot"].to_vec(),
                    }
                ),
                (
                    "47",
                    Subdivision{
                        name: "47",
                        country_alpha2: Alpha2::FR,
                        code: "47",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.2470173), longitude: Some(0.4502368), max_latitude: Some(44.765678), min_latitude: Some(43.9725879), max_longitude: Some(1.078341), min_longitude: Some(-0.140673)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Lot-et-Garonne"), ("ar", "لوت وغارون"), ("az", "Lo və Qaronna"), ("be", "Дэпартамент Ло і Гарона"), ("bg", "Лот е Гарон"), ("bn", "লট-এট-গ\u{9cd}য\u{9be}রোন"), ("ca", "Òlt i Garona"), ("ccp", "𑄣\u{11127}𑄖\u{11134}-𑄃𑄬𑄖\u{11134}-𑄉𑄢\u{1112e}𑄚\u{11133}𑄦\u{11128}"), ("ceb", "Lot-et-Garonne"), ("cs", "Lot-et-Garonne"), ("cy", "Lot-et-Garonne"), ("da", "Lot-et-Garonne"), ("de", "Département Lot-et-Garonne"), ("el", "Λοτ-ε-Γκαρόν"), ("en", "Lot-et-Garonne"), ("es", "Lot y Garona"), ("et", "Lot-et-Garonne’i departemang"), ("eu", "Lot-et-Garona"), ("fa", "لو-ا-گرون"), ("fi", "Lot-et-Garonne"), ("fr", "Lot-et-Garonne"), ("gl", "Lot e Garona"), ("gu", "લોટ-એટ-ગ\u{ac7}રોન"), ("he", "לוט וגארון"), ("hi", "ल\u{942}त-एट-गोरोन"), ("hu", "Lot-et-Garonne"), ("hy", "Լո և Գարոն"), ("id", "Lot-et-Garonne"), ("it", "Lot e Garonna"), ("ka", "ლო და გარონა"), ("kk", "Ло және Гаронна"), ("kn", "ಲಾಟ\u{ccd} ಎಟ\u{ccd}-ಗ\u{ccd}ಯಾರೋನ\u{cc6}"), ("ko", "로트에가론 주"), ("lt", "Lo ir Garona"), ("lv", "Lo un Garonna"), ("mr", "लोत-एत-गारोन"), ("ms", "Lot-et-Garonne"), ("nb", "Lot-et-Garonne"), ("nl", "Lot-et-Garonne"), ("no", "Lot-et-Garonne"), ("pl", "Lot-et-Garonne"), ("pt", "Lot e Garona"), ("ro", "Lot-et-Garonne"), ("ru", "Ло и Гаронна"), ("si", "ලොට\u{dca}-එට\u{dca}-ගරෝන\u{dca}නේ"), ("sk", "Lot-et-Garonne"), ("sl", "Lot-et-Garonne"), ("sq", "Lot-et-Garonne"), ("sr", "Лот и Гарона"), ("sr_Latn", "Lot i Garona"), ("sv", "Lot-et-Garonne"), ("sw", "Lot-et-Garonne"), ("ta", "லோட\u{bcd} -எட\u{bcd} -க\u{bbe}ரோணனே"), ("te", "ల\u{c3e}ట\u{c46}\u{c4d}-ఎట\u{c4d}-గ\u{c4d}య\u{c3e}రన\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดลอเตการอน"), ("tr", "Lot-et-Garonne"), ("uk", "Лот і Гаронна"), ("ur", "لوت-اے-گارون"), ("vi", "Lot-et-Garonne"), ("yue", "洛特-加龍"), ("yue_Hans", "洛特-加龙"), ("zh", "洛特-加龍省")]),
                        unofficial_name_list: ["Lot-et-Garonne"].to_vec(),
                    }
                ),
                (
                    "48",
                    Subdivision{
                        name: "48",
                        country_alpha2: Alpha2::FR,
                        code: "48",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.494203), longitude: Some(3.5812692), max_latitude: Some(44.975761), min_latitude: Some(44.1095909), max_longitude: Some(3.998366), min_longitude: Some(2.981197)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Lozère"), ("ar", "لوزار"), ("be", "Дэпартамент Лазер"), ("bg", "Лозер"), ("bn", "লোজের"), ("ca", "Losera"), ("ccp", "𑄣\u{1112e}𑄡𑄠𑄬𑄢\u{11134}"), ("ceb", "Lozère"), ("cs", "Lozère"), ("cy", "Lozère"), ("da", "Lozère"), ("de", "Département Lozère"), ("el", "Λοζέρ"), ("en", "Lozère"), ("es", "Lozère"), ("et", "Lozère’i departemang"), ("eu", "Lozère"), ("fa", "لوزر"), ("fi", "Lozère"), ("fr", "Lozère"), ("gl", "Lozère"), ("gu", "લોઝ\u{ac7}ર\u{ac7}"), ("he", "לוזר"), ("hi", "लोज\u{93c}\u{947}र\u{947}"), ("hu", "Lozère"), ("hy", "Լոզեր"), ("id", "Lozère"), ("it", "Lozère"), ("ja", "ロゼール県"), ("ka", "ლოზერი"), ("kk", "Лозер"), ("kn", "ಲೊಝ\u{cc6}ರ\u{cc6}"), ("ko", "로제르 주"), ("lt", "Lozeras"), ("lv", "Lozēra"), ("mr", "लोझ\u{947}र"), ("ms", "Lozère"), ("nb", "Lozère"), ("nl", "Lozère"), ("no", "Lozère"), ("pl", "Lozère"), ("pt", "Lozère"), ("ro", "Lozère"), ("ru", "Лозер"), ("si", "ලොසෙරේ"), ("sk", "Lozère"), ("sl", "Lozère"), ("sq", "Lozère"), ("sr", "Лозер"), ("sr_Latn", "Lozer"), ("sv", "Lozère"), ("sw", "Lozère"), ("ta", "லோஸிர\u{bcd}"), ("te", "ల\u{c4b}జ\u{c3f}యర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดลอแซร\u{e4c}"), ("tr", "Lozère"), ("uk", "Лозер"), ("ur", "لوزیر"), ("vi", "Lozère"), ("yue", "洛澤爾"), ("yue_Hans", "洛泽尔"), ("zh", "洛泽尔省")]),
                        unofficial_name_list: ["Lozère"].to_vec(),
                    }
                ),
                (
                    "49",
                    Subdivision{
                        name: "49",
                        country_alpha2: Alpha2::FR,
                        code: "49",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.2913545), longitude: Some(-0.4877852), max_latitude: Some(47.809978), min_latitude: Some(46.9688829), max_longitude: Some(0.234549), min_longitude: Some(-1.3505026)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Maine-et-Loire"), ("ar", "ماين ولوار"), ("az", "Men və Luara"), ("be", "Мен і Луара"), ("bg", "Мен е Лоар"), ("bn", "মেইন ইট লরি"), ("ca", "Maine i Loira"), ("ccp", "𑄟\u{1112d}𑄚\u{11134}-𑄃𑄬𑄖\u{11134}-𑄣\u{11130}𑄢\u{11134}"), ("ceb", "Maine-et-Loire"), ("cs", "Maine-et-Loire"), ("cy", "Maine-et-Loire"), ("da", "Maine-et-Loire"), ("de", "Département Maine-et-Loire"), ("el", "Μαιν-ε-Λουάρ"), ("en", "Maine-et-Loire"), ("es", "Maine y Loira"), ("et", "Maine-et-Loire"), ("eu", "Maine-et-Loire"), ("fa", "من الوآر"), ("fi", "Maine-et-Loire"), ("fr", "Maine-et-Loire"), ("gl", "Maine e Loira"), ("gu", "મ\u{ac8}ન-એટ-લોઈર"), ("he", "מן ולואר"), ("hi", "म\u{947}न-एट-लोइर"), ("hu", "Maine-et-Loire"), ("hy", "Մեն և Լուար"), ("id", "Maine-et-Loire"), ("it", "Maine e Loira"), ("ka", "მენი და ლუარა"), ("kk", "Мен және Луара"), ("kn", "ಮೈನ\u{cc6}-ಎಟ\u{ccd}-ಲೋರ\u{cc6}"), ("ko", "멘에루아르 주"), ("lt", "Menas ir Luara"), ("lv", "Mēna un Luāra"), ("mr", "म\u{947}न-एत-लावार"), ("ms", "Maine-et-Loire"), ("nb", "Maine-et-Loire"), ("nl", "Maine-et-Loire"), ("no", "Maine-et-Loire"), ("pl", "Maine-et-Loire"), ("pt", "Maine-et-Loire"), ("ro", "Maine-et-Loire"), ("ru", "Мен и Луара"), ("si", "මය\u{dd2}නේ-එට\u{dca}-ලොය\u{dd2}රේ"), ("sk", "Maine-et-Loire"), ("sl", "Maine-et-Loire"), ("sq", "Maine-et-Loire"), ("sr", "Мен и Лоара"), ("sr_Latn", "Men i Loara"), ("sv", "Maine-et-Loire"), ("sw", "Maine-et-Loire"), ("ta", "மைனி-எட\u{bcd} -லோயிரே"), ("te", "మ\u{c46}య\u{c3f}న\u{c46}-ఎట\u{c4d}-ల\u{c4b}యర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแมเนล\u{e31}วร\u{e4c}"), ("tr", "Maine-et-Loire"), ("uk", "Мен і Луара"), ("ur", "مین-اے-لوار"), ("vi", "Maine-et-Loire"), ("yue", "曼恩-盧華爾"), ("yue_Hans", "曼恩-卢华尔"), ("zh", "曼恩-卢瓦尔省")]),
                        unofficial_name_list: ["Maine-et-Loire"].to_vec(),
                    }
                ),
                (
                    "50",
                    Subdivision{
                        name: "50",
                        country_alpha2: Alpha2::FR,
                        code: "50",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.114712), longitude: Some(-1.3115949), max_latitude: Some(49.727762), min_latitude: Some(48.45579679999999), max_longitude: Some(-0.7348169000000001), min_longitude: Some(-1.954995)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Manche"), ("ar", "المانش"), ("az", "Manş (departament)"), ("be", "Дэпартамент Манш"), ("bg", "Манш"), ("bn", "ম\u{9be}নছে"), ("ca", "Manche"), ("ccp", "𑄟𑄚\u{11134}𑄌𑄬"), ("ceb", "Manche"), ("cs", "Manche"), ("cy", "Manche"), ("da", "Manche"), ("de", "Département Manche"), ("el", "Μανς"), ("en", "Manche"), ("es", "Mancha"), ("et", "Manche’i departemang"), ("eu", "Manche"), ("fa", "مانش"), ("fi", "Manche"), ("fr", "Manche"), ("gl", "Mancha"), ("gu", "મા\u{a82}ચ\u{ac7}"), ("he", "מאנש"), ("hi", "मान\u{94d}श\u{947}"), ("hu", "Manche"), ("hy", "Մանշ"), ("id", "Manche"), ("it", "Manica"), ("ja", "マンシュ県"), ("ka", "მანში"), ("kk", "Манш"), ("kn", "ಮಂಚ\u{cc6}"), ("ko", "망슈 주"), ("lt", "Manšas"), ("lv", "Manša"), ("mr", "मा\u{902}च"), ("ms", "Manche"), ("nb", "Manche"), ("nl", "Manche"), ("no", "Manche"), ("pl", "Manche"), ("pt", "Mancha"), ("ro", "Manche"), ("ru", "Манш"), ("si", "මන\u{dca}චේ"), ("sk", "Manche"), ("sl", "Manche"), ("sq", "Manche"), ("sr", "Манш"), ("sr_Latn", "Manš"), ("sv", "Manche"), ("sw", "Manche"), ("ta", "ம\u{bbe}ஞ\u{bcd}சே"), ("te", "మ\u{c3e}ంచ\u{c3f}"), ("th", "จ\u{e31}งหว\u{e31}ดม\u{e47}องช\u{e4c}"), ("tr", "Manche"), ("uk", "Манш"), ("ur", "مانش"), ("vi", "Manche"), ("yue", "芒什"), ("yue_Hans", "芒什"), ("zh", "芒什省")]),
                        unofficial_name_list: ["Manche"].to_vec(),
                    }
                ),
                (
                    "51",
                    Subdivision{
                        name: "51",
                        country_alpha2: Alpha2::FR,
                        code: "51",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.128754), longitude: Some(4.1475445), max_latitude: Some(49.40782799999999), min_latitude: Some(48.515251), max_longitude: Some(5.039668), min_longitude: Some(3.395865)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Marne"), ("ar", "المارن"), ("az", "Marna (departament)"), ("be", "Дэпартамент Марна"), ("bg", "Марн"), ("bn", "ম\u{9be}র\u{9cd}নে"), ("ca", "Marne"), ("ccp", "𑄟𑄢\u{11134}𑄚𑄬"), ("ceb", "Marne"), ("cs", "Marne"), ("cy", "Marne"), ("da", "Marne"), ("de", "Département Marne"), ("el", "Μαρν"), ("en", "Marne"), ("es", "Marne"), ("et", "Marne’i departemang"), ("eu", "Marne"), ("fa", "مرن"), ("fi", "Marne"), ("fr", "Marne"), ("gl", "Marne"), ("gu", "માર\u{acd}ન\u{ac7}"), ("he", "מארן (מחוז)"), ("hi", "मार\u{94d}न\u{947}"), ("hr", "Marne"), ("hu", "Marne"), ("hy", "Մարն"), ("id", "Marne"), ("it", "Marna"), ("ja", "マルヌ県"), ("ka", "მარნა"), ("kk", "Марна"), ("kn", "ಮರ\u{ccd}ನ\u{cc6}"), ("ko", "마른 주"), ("lt", "Marna"), ("lv", "Marna"), ("mr", "मार\u{94d}न"), ("ms", "Marne"), ("nb", "Marne"), ("nl", "Marne"), ("no", "Marne"), ("pl", "Marna"), ("pt", "Marne"), ("ro", "Marne"), ("ru", "Марна"), ("si", "මර\u{dca}නේ"), ("sk", "Marne"), ("sl", "Marne"), ("sq", "Marne"), ("sr", "Марна"), ("sr_Latn", "Marna"), ("sv", "Marne"), ("sw", "Marne"), ("ta", "ம\u{bbe}றனே"), ("te", "మ\u{c3e}ర\u{c4d}న\u{c46}"), ("th", "จ\u{e31}งหว\u{e31}ดมาร\u{e4c}น"), ("tr", "Marne"), ("uk", "Марна"), ("ur", "مارن، فرانس"), ("vi", "Marne"), ("yue", "馬恩"), ("yue_Hans", "马恩"), ("zh", "马恩省")]),
                        unofficial_name_list: ["Marne"].to_vec(),
                    }
                ),
                (
                    "52",
                    Subdivision{
                        name: "52",
                        country_alpha2: Alpha2::FR,
                        code: "52",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.1260968), longitude: Some(5.1071322), max_latitude: Some(48.689322), min_latitude: Some(47.576566), max_longitude: Some(5.891043), min_longitude: Some(4.6266029)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Haute-Marne"), ("ar", "المارن العليا"), ("az", "Yuxarı Marna"), ("be", "Дэпартамент Марна Верхняя"), ("bg", "От Марн"), ("bn", "হোতে-ম\u{9be}র\u{9cd}নে"), ("ca", "Alt Marne"), ("ccp", "𑄦𑄅\u{1112a}𑄖\u{11134}-𑄟𑄢\u{11134}𑄚𑄬"), ("ceb", "Haute-Marne"), ("cs", "Haute-Marne"), ("cy", "Haute-Marne"), ("da", "Haute-Marne"), ("de", "Département Haute-Marne"), ("el", "Ωτ-Μαρν"), ("en", "Haute-Marne"), ("es", "Alto Marne"), ("et", "Haute-Marne’i departemang"), ("eu", "Haute-Marne"), ("fa", "اوت-مارن"), ("fi", "Haute-Marne"), ("fr", "Haute-Marne"), ("gl", "Alto Marne"), ("gu", "હૌટ-માર\u{acd}ન\u{ac7}"), ("he", "הוט-מארן"), ("hi", "ओट-मार\u{94d}न"), ("hr", "Haute-Marne"), ("hu", "Haute-Marne"), ("hy", "Վերին Մարն"), ("id", "Haute-Marne"), ("it", "Alta Marna"), ("ka", "ზემო მარნა"), ("kk", "Жоғарғы Марна"), ("kn", "ಹ\u{ccc}ಟ\u{cc6}-ಮರ\u{ccd}ನ\u{cc6}"), ("ko", "오트마른 주"), ("lt", "Aukštutinė Marna"), ("lv", "Augšmarna"), ("mr", "ऑत-मार\u{94d}न"), ("ms", "Haute-Marne"), ("nb", "Haute-Marne"), ("nl", "Haute-Marne"), ("no", "Haute-Marne"), ("pl", "Górna Marna"), ("pt", "Haute-Marne"), ("ro", "Haute-Marne"), ("ru", "Верхняя Марна"), ("si", "හෞටේ මර\u{dca}නේ"), ("sk", "Haute-Marne"), ("sl", "Haute-Marne"), ("sq", "Haute-Marne"), ("sr", "Горња Марна"), ("sr_Latn", "Gornja Marna"), ("sv", "Haute-Marne"), ("sw", "Haute-Marne"), ("ta", "ஹூட\u{bcd} -ம\u{bbe}ர\u{bcd}னே"), ("te", "హ\u{c3e}ట\u{c4d}-మ\u{c3e}ర\u{c4d}న\u{c46}"), ("th", "จ\u{e31}งหว\u{e31}ดโอต-มาร\u{e4c}น"), ("tr", "Haute-Marne"), ("uk", "Верхня Марна"), ("ur", "بالائی-مارن"), ("vi", "Haute-Marne"), ("yue", "上馬恩"), ("yue_Hans", "上马恩"), ("zh", "上马恩省")]),
                        unofficial_name_list: ["Haute-Marne"].to_vec(),
                    }
                ),
                (
                    "53",
                    Subdivision{
                        name: "53",
                        country_alpha2: Alpha2::FR,
                        code: "53",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.3061239), longitude: Some(-0.620935), max_latitude: Some(48.33539690000001), min_latitude: Some(48.2778439), max_longitude: Some(-0.5777470000000001), min_longitude: Some(-0.6502759)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Mayenne"), ("ar", "ماين"), ("az", "Mayen (departament)"), ("be", "Дэпартамент Маен"), ("bg", "Майен"), ("bn", "ম\u{9be}য\u{9bc}েন"), ("ca", "Mayenne"), ("ccp", "𑄟𑄠𑄬𑄚\u{11133}𑄦\u{11128}"), ("ceb", "Mayenne"), ("cs", "Mayenne"), ("cy", "Mayenne"), ("da", "Mayenne"), ("de", "Mayenne"), ("el", "Μαγιέν"), ("en", "Mayenne"), ("es", "Mayenne"), ("et", "Mayenne’i departemang"), ("eu", "Mayenne"), ("fa", "ماین"), ("fi", "Mayenne"), ("fr", "Mayenne"), ("gl", "Mayenne"), ("gu", "માય\u{ac7}ન"), ("he", "מאיין"), ("hi", "म\u{947}य\u{947}न"), ("hu", "Mayenne"), ("hy", "Մայեն"), ("id", "Mayenne"), ("it", "Mayenne"), ("ja", "マイエンヌ県"), ("ka", "მაიენი"), ("kk", "Майен"), ("kn", "ಮಾಯ\u{cc6}ನ\u{ccd}ನ\u{cc6}"), ("ko", "마옌 주"), ("lt", "Majenas"), ("lv", "Majenna"), ("mr", "माय\u{947}न"), ("ms", "Mayenne"), ("nb", "Mayenne"), ("nl", "Mayenne"), ("no", "Mayenne"), ("pl", "Mayenne"), ("pt", "Mayenne"), ("ro", "Mayenne"), ("ru", "Майен"), ("si", "මෙයෙන\u{dca}නේ"), ("sk", "Mayenne"), ("sl", "Mayenne"), ("sq", "Mayenne"), ("sr", "Мајен"), ("sr_Latn", "Majen"), ("sv", "Mayenne"), ("sw", "Mayenne"), ("ta", "ம\u{bbe}யென\u{bcd}னே"), ("te", "మ\u{c47}య\u{c46}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมาแยน"), ("tr", "Mayenne"), ("uk", "Майєнн"), ("ur", "ماین"), ("vi", "Mayenne"), ("yue", "馬耶訥"), ("yue_Hans", "马耶讷"), ("zh", "马耶讷省")]),
                        unofficial_name_list: ["Mayenne"].to_vec(),
                    }
                ),
                (
                    "54",
                    Subdivision{
                        name: "54",
                        country_alpha2: Alpha2::FR,
                        code: "54",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.7997007), longitude: Some(6.094701400000001), max_latitude: Some(49.56326800000001), min_latitude: Some(48.348987), max_longitude: Some(7.123213100000001), min_longitude: Some(5.426108)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Meurthe-et-Moselle"), ("ar", "مورت وموزيل"), ("az", "Mört və Mözel"), ("be", "Дэпартамент Мёрт і Мозель"), ("bg", "Мьорт е Мозел"), ("bn", "ম\u{9c1}র\u{9cd}থ -এট-মোসেল\u{9cd}লে"), ("ca", "Meurthe i Mosel·la"), ("ccp", "𑄟𑄬𑄅\u{1112a}𑄢\u{11134}𑄗\u{11128}-𑄃𑄬𑄖\u{11134}-𑄟\u{1112e}𑄥𑄬𑄣\u{11133}𑄦\u{11128}"), ("ceb", "Meurthe-et-Moselle"), ("cs", "Meurthe-et-Moselle"), ("cy", "Meurthe-et-Moselle"), ("da", "Meurthe-et-Moselle"), ("de", "Département Meurthe-et-Moselle"), ("el", "Μερτ-ε-Μοζέλ"), ("en", "Meurthe-et-Moselle"), ("es", "Meurthe y Mosela"), ("et", "Meurthe-et-Moselle’i departemang"), ("eu", "Meurthe-et-Moselle"), ("fa", "مورت موزل"), ("fi", "Meurthe-et-Moselle"), ("fr", "Meurthe-et-Moselle"), ("gl", "Meurthe e Mosela"), ("gu", "મ\u{acd}ય\u{ac1}ર\u{acd}થ\u{ac7}-એટ-મોસ\u{acd}લ\u{ac7}"), ("he", "מרת ומוזל"), ("hi", "माउर\u{94d}थ\u{947}-एट-मोस\u{947}ल"), ("hu", "Meurthe-et-Moselle"), ("hy", "Մյորթ և Մոզել"), ("id", "Meurthe-et-Moselle"), ("it", "Meurthe e Mosella"), ("ka", "მერთი და მოზელი"), ("kk", "Мёрт және Мозель"), ("kn", "ಮ\u{cc2}ರ\u{ccd}ಥ\u{cc6}-ಎಟ\u{ccd}-ಮೊಸ\u{cc6}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "뫼르트에모젤 주"), ("lt", "Mertas ir Mozelis"), ("lv", "Merta un Mozele"), ("mk", "Мерт и Мозел"), ("mr", "म\u{94d}य\u{941}र\u{94d}त\u{947}-ए-मोझ\u{947}ल"), ("ms", "Meurthe-et-Moselle"), ("nb", "Meurthe-et-Moselle"), ("nl", "Meurthe-et-Moselle"), ("no", "Meurthe-et-Moselle"), ("pl", "Meurthe-et-Moselle"), ("pt", "Meurthe-et-Moselle"), ("ro", "Meurthe-et-Moselle"), ("ru", "Мёрт и Мозель"), ("si", "මෙඋර\u{dca}තේ එට\u{dca} මොසේල\u{dca}ලේ"), ("sk", "Meurthe-et-Moselle"), ("sl", "Meurthe-et-Moselle"), ("sq", "Meurthe-et-Moselle"), ("sr", "Мерт и Мозел"), ("sr_Latn", "Mert i Mozel"), ("sv", "Meurthe-et-Moselle"), ("sw", "Meurthe-et-Moselle"), ("ta", "மேயுர\u{bcd}தே-எட\u{bcd} -மொசெல\u{bcd}லே"), ("te", "మ\u{c46}యుర\u{c4d}త\u{c46}-ఎట\u{c4d}-మ\u{c4b}స\u{c46}ల\u{c4d}ల\u{c46}"), ("th", "จ\u{e31}งหว\u{e31}ดเมอร\u{e4c}เตมอแซล"), ("tr", "Meurthe-et-Moselle"), ("uk", "Мерт і Мозель"), ("ur", "مرتے-اے-موزیل"), ("vi", "Meurthe-et-Moselle"), ("yue", "默爾特-摩澤爾"), ("yue_Hans", "默尔特-摩泽尔"), ("zh", "默尔特-摩泽尔省")]),
                        unofficial_name_list: ["Meurthe-et-Moselle"].to_vec(),
                    }
                ),
                (
                    "55",
                    Subdivision{
                        name: "55",
                        country_alpha2: Alpha2::FR,
                        code: "55",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.0824319), longitude: Some(5.282399700000001), max_latitude: Some(49.616864), min_latitude: Some(48.4089919), max_longitude: Some(5.8542058), min_longitude: Some(4.8883769)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Meuse"), ("ar", "موز"), ("az", "Möz (departament)"), ("be", "Дэпартамент Мёз"), ("bg", "Мьоз"), ("bn", "মিউস"), ("ca", "Mosa"), ("ccp", "𑄟𑄬𑄃\u{11128}𑄃\u{1112a}𑄌\u{11134}"), ("ceb", "Meuse"), ("cs", "Meuse"), ("cy", "Meuse"), ("da", "Meuse"), ("de", "Département Meuse"), ("el", "Μεζ"), ("en", "Meuse"), ("es", "Mosa"), ("et", "Meuse’i departemang"), ("eu", "Meuse"), ("fa", "موز"), ("fi", "Meuse"), ("fr", "Meuse"), ("gl", "Mosa"), ("gu", "મીય\u{ac1}ઝ"), ("he", "מז"), ("hi", "म\u{94d}य\u{942}स\u{947}"), ("hu", "Meuse"), ("hy", "Մյոզ"), ("id", "Meuse"), ("it", "Mosa"), ("ja", "ムーズ県"), ("ka", "მეზის დეპარტამენტი"), ("kk", "Мез"), ("kn", "ಮೇಸ\u{ccd}"), ("ko", "뫼즈 주"), ("lt", "Mezas"), ("lv", "Mēza"), ("mk", "Меза"), ("mr", "म\u{94d}य\u{941}झ"), ("ms", "Meuse"), ("nb", "Meuse"), ("nl", "Meuse"), ("no", "Meuse"), ("pl", "Moza"), ("pt", "Meuse"), ("ro", "Meuse"), ("ru", "Мёз"), ("si", "මෙය\u{dd4}සේ"), ("sk", "Meuse"), ("sl", "Meuse"), ("sr", "Меза"), ("sr_Latn", "Meza"), ("sv", "Meuse"), ("sw", "Meuse"), ("ta", "ம\u{bc0}உஸ\u{bcd}"), ("te", "మ\u{c3f}యూజ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเม\u{e34}ซ"), ("tr", "Meuse"), ("uk", "Мез"), ("ur", "موز"), ("vi", "Meuse"), ("yue", "默茲"), ("yue_Hans", "默兹"), ("zh", "默兹省")]),
                        unofficial_name_list: ["Meuse"].to_vec(),
                    }
                ),
                (
                    "56",
                    Subdivision{
                        name: "56",
                        country_alpha2: Alpha2::FR,
                        code: "56",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.8852929), longitude: Some(-2.9001865), max_latitude: Some(48.21088899999999), min_latitude: Some(47.27785290000001), max_longitude: Some(-2.0353381), min_longitude: Some(-3.7349149)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Morbihan"), ("ar", "موربيهان"), ("az", "Morbian"), ("be", "Марбіян"), ("bg", "Морбиан"), ("bn", "মরবিহ\u{9be}ন"), ("ca", "Ar Mor-Bihan"), ("ccp", "𑄟\u{11127}𑄢\u{11134}𑄝\u{11128}𑄦𑄚\u{11134}"), ("ceb", "Morbihan"), ("cs", "Morbihan"), ("cy", "Mor-Bihan"), ("da", "Morbihan"), ("de", "Département Morbihan"), ("el", "Μορμπιχάν"), ("en", "Morbihan"), ("es", "Morbihan"), ("et", "Morbihani departemang"), ("eu", "Morbihan"), ("fa", "موربیان"), ("fi", "Morbihan"), ("fr", "Morbihan"), ("gl", "Morbihan"), ("gu", "મોરબીહાન"), ("he", "מורביאן"), ("hi", "मोरबिहन"), ("hu", "Morbihan"), ("hy", "Մորբիան"), ("id", "Morbihan"), ("is", "Morbihan"), ("it", "Morbihan"), ("ja", "モルビアン県"), ("ka", "მორბიანი"), ("kk", "Морбиан"), ("kn", "ಮೊರ\u{ccd}ಬ\u{cbf}ಹಾನ\u{ccd}"), ("ko", "모르비앙 주"), ("lt", "Morbihanas"), ("lv", "Morbiāna"), ("mk", "Морбијан"), ("mr", "मॉर\u{94d}बिया\u{902}"), ("ms", "Morbihan"), ("nb", "Morbihan"), ("nl", "Morbihan"), ("no", "Morbihan"), ("pl", "Morbihan"), ("pt", "Morbihan"), ("ro", "Morbihan"), ("ru", "Морбиан"), ("si", "මොර\u{dca}බ\u{dd2}හ\u{dcf}න\u{dca}"), ("sk", "Morbihan"), ("sl", "Morbihan"), ("sq", "Morbihan"), ("sr", "Морбијан"), ("sr_Latn", "Morbijan"), ("sv", "Morbihan"), ("sw", "Morbihan"), ("ta", "மோர\u{bcd}பிஹன\u{bcd}"), ("te", "మ\u{c4b}ర\u{c4d}బ\u{c3f}హ\u{c3e}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมอร\u{e4c}บ\u{e35}อ\u{e47}อง"), ("tr", "Morbihan"), ("uk", "Морбіан"), ("ur", "موربیاں"), ("vi", "Morbihan"), ("yue", "莫爾比昂"), ("yue_Hans", "莫尔比昂"), ("zh", "莫尔比昂省")]),
                        unofficial_name_list: ["Morbihan"].to_vec(),
                    }
                ),
                (
                    "57",
                    Subdivision{
                        name: "57",
                        country_alpha2: Alpha2::FR,
                        code: "57",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.0983839), longitude: Some(6.5527641), max_latitude: Some(49.515124), min_latitude: Some(48.5267229), max_longitude: Some(7.640046900000001), min_longitude: Some(5.891857099999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Moselle"), ("ar", "موزيل"), ("az", "Mozel"), ("be", "Дэпартамент Мозель"), ("bg", "Мозел"), ("bn", "মোসেল\u{9cd}যে"), ("ca", "Mosel·la"), ("ccp", "𑄟\u{1112e}𑄥𑄬𑄣\u{11133}𑄣\u{11128}"), ("ceb", "Moselle"), ("cs", "Moselle"), ("cy", "Moselle"), ("da", "Moselle"), ("de", "Département Moselle"), ("el", "Μοζέλ"), ("en", "Moselle"), ("es", "Mosela"), ("et", "Moselle’i departemang"), ("eu", "Moselle"), ("fa", "موزل"), ("fi", "Moselle"), ("fr", "Moselle"), ("gl", "Mosela"), ("gu", "મોસ\u{ac7}લ\u{ac7}"), ("he", "מוזל"), ("hi", "मॉस\u{947}ल"), ("hr", "Moselle"), ("hu", "Moselle"), ("hy", "Մոզել"), ("id", "Moselle"), ("it", "Mosella"), ("ja", "モゼル県"), ("ka", "მოზელი"), ("kk", "Мозель"), ("kn", "ಮೋಸ\u{cc6}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "모젤 주"), ("lt", "Mozelis"), ("lv", "Mozele"), ("mk", "Мозел"), ("mr", "मोझ\u{947}ल"), ("ms", "Moselle"), ("nb", "Moselle"), ("nl", "Moselle"), ("no", "Moselle"), ("pl", "Mozela"), ("pt", "Mosela"), ("ro", "Moselle"), ("ru", "Мозель"), ("si", "මොසෙලේ"), ("sk", "Moselle"), ("sl", "Moselle"), ("sq", "Moselle"), ("sr", "Мозел"), ("sr_Latn", "Mozel"), ("sv", "Moselle"), ("sw", "Moselle"), ("ta", "மோசெல\u{bcd}லே"), ("te", "మ\u{c4b}స\u{c46}ల\u{c4d}ల\u{c46}"), ("th", "จ\u{e31}งหว\u{e31}ดมอแซล"), ("tr", "Moselle"), ("uk", "Мозель"), ("ur", "موسیلی"), ("vi", "Moselle"), ("yue", "摩澤爾"), ("yue_Hans", "摩泽尔"), ("zh", "摩泽尔省")]),
                        unofficial_name_list: ["Moselle"].to_vec(),
                    }
                ),
                (
                    "58",
                    Subdivision{
                        name: "58",
                        country_alpha2: Alpha2::FR,
                        code: "58",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.2381708), longitude: Some(3.5294522), max_latitude: Some(47.5882881), min_latitude: Some(46.651024), max_longitude: Some(4.23191), min_longitude: Some(2.8459898)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nièvre"), ("ar", "نيافر"), ("az", "Nyevr"), ("be", "Дэпартамент Ньеўр"), ("bg", "Ниевър"), ("bn", "নিভ\u{9cd}রে"), ("ca", "Nièvre"), ("ccp", "𑄚\u{1112d}𑄛\u{11134}𑄢\u{11128}"), ("ceb", "Nièvre"), ("cs", "Nièvre"), ("cy", "Nièvre"), ("da", "Nièvre"), ("de", "Département Nièvre"), ("el", "Νιέβρ"), ("en", "Nièvre"), ("es", "Nièvre"), ("et", "Nièvre’i departemang"), ("eu", "Nièvre"), ("fa", "نی\u{200c}یور"), ("fi", "Nièvre"), ("fr", "Nièvre"), ("gl", "Nièvre"), ("gu", "નીએવ\u{acd}ર\u{ac7}"), ("he", "נייוור"), ("hi", "निएवर\u{947}"), ("hu", "Nièvre"), ("hy", "Նյևր"), ("id", "Nièvre"), ("it", "Nièvre"), ("ja", "ニエーヴル県"), ("ka", "ნიევრი"), ("kk", "Ньевр"), ("kn", "ನೀವ\u{cc6}ವ\u{ccd}ರ\u{cc6}"), ("ko", "니에브르 주"), ("lt", "Njevras"), ("lv", "Njevra"), ("mr", "न\u{94d}य\u{947}व\u{94d}र"), ("ms", "Nièvre"), ("nb", "Nièvre"), ("nl", "Nièvre"), ("no", "Nièvre"), ("pl", "Nièvre"), ("pt", "Nièvre"), ("ro", "Nièvre"), ("ru", "Ньевр"), ("si", "නය\u{dd2}ව\u{dca}රේ"), ("sk", "Nièvre"), ("sl", "Nièvre"), ("sr", "Нијевр"), ("sr_Latn", "Nijevr"), ("sv", "Nièvre"), ("sw", "Nièvre"), ("ta", "நிஐவரே"), ("te", "న\u{c40}వర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเน\u{e35}ยฟวร\u{e4c}"), ("tr", "Nièvre"), ("uk", "Ньєвр"), ("ur", "نیاور"), ("vi", "Nièvre"), ("yue", "涅夫勒"), ("yue_Hans", "涅夫勒"), ("zh", "涅夫勒省")]),
                        unofficial_name_list: ["Nièvre"].to_vec(),
                    }
                ),
                (
                    "59",
                    Subdivision{
                        name: "59",
                        country_alpha2: Alpha2::FR,
                        code: "59",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.3851246), longitude: Some(3.2642436), max_latitude: Some(51.08899), min_latitude: Some(49.9690609), max_longitude: Some(4.2316779), min_longitude: Some(2.089297)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nord"), ("ar", "نور"), ("az", "Nor"), ("be", "Дэпартамент Нор"), ("bg", "Нор"), ("bn", "নর\u{9cd}ড"), ("ca", "Nord"), ("ccp", "𑄚\u{11127}𑄢\u{11134}𑄓\u{11134}"), ("ceb", "Nord"), ("cs", "Nord"), ("cy", "Nord"), ("da", "Nord"), ("de", "Département Nord"), ("el", "Νορ"), ("en", "Nord"), ("es", "Norte"), ("et", "Nord’i departemang"), ("eu", "Nord"), ("fa", "نور"), ("fi", "Nord"), ("fr", "Nord"), ("gl", "Norte, Francia"), ("gu", "નોર\u{acd}ડ"), ("he", "נור"), ("hi", "नोर\u{94d}ड"), ("hu", "Nord"), ("hy", "Նոր"), ("id", "Nord, Perancis"), ("it", "Nord"), ("ja", "ノール県"), ("ka", "ნორი"), ("kk", "Нор"), ("kn", "ನಾರ\u{ccd}ಡ\u{ccd}"), ("ko", "노르 주"), ("lt", "Šiaurė"), ("lv", "Nora"), ("mk", "Север"), ("mr", "नोर"), ("ms", "Nord"), ("nb", "Nord"), ("nl", "Noorderdepartement"), ("no", "Nord"), ("pl", "Nord"), ("pt", "Norte (departamento)"), ("ro", "Nord"), ("ru", "Нор"), ("si", "නොර\u{dca}ඩ\u{dca}"), ("sk", "Nord"), ("sl", "Nord"), ("sr", "Нор"), ("sr_Latn", "Nor"), ("sv", "Nord"), ("sw", "Nord"), ("ta", "நொர\u{bcd}ட\u{bcd}"), ("te", "న\u{c3e}ర\u{c4d}డ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดนอร\u{e4c}"), ("tr", "Nord"), ("uk", "Нор"), ("ur", "نور"), ("vi", "Nord, Nord-Pas-de-Calais"), ("yue", "諾爾"), ("yue_Hans", "诺尔"), ("zh", "诺尔省")]),
                        unofficial_name_list: ["Nord"].to_vec(),
                    }
                ),
                (
                    "60",
                    Subdivision{
                        name: "60",
                        country_alpha2: Alpha2::FR,
                        code: "60",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.42145679999999), longitude: Some(2.4146396), max_latitude: Some(49.76392209999999), min_latitude: Some(49.060525), max_longitude: Some(3.166125), min_longitude: Some(1.6888659)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Oise"), ("ar", "واز"), ("az", "Uaza (departament)"), ("be", "Дэпартамент Уаза"), ("bg", "Оаз"), ("bn", "ওয\u{9bc}িজ"), ("ca", "Oise"), ("ccp", "𑄃\u{1112e}𑄃\u{11128}𑄌\u{11134}"), ("ceb", "Oise"), ("cs", "Oise"), ("cy", "Oise"), ("da", "Oise"), ("de", "Département Oise"), ("el", "Ουάζ"), ("en", "Oise"), ("es", "Oise"), ("et", "Oise’i departemang"), ("eu", "Oise"), ("fa", "اوآز"), ("fi", "Oise"), ("fr", "Oise"), ("gl", "Oise"), ("gu", "ઓઇસ"), ("he", "אואז"), ("hi", "ओइस"), ("hu", "Oise"), ("hy", "Ուազ"), ("id", "Oise"), ("it", "Oise"), ("ja", "オワーズ県"), ("ka", "უაზა"), ("kk", "Уаза"), ("kn", "ಒಯ\u{ccd}ಸ\u{ccd}"), ("ko", "우아즈 주"), ("lt", "Uaza"), ("lv", "Uāza"), ("mk", "Оаза"), ("mr", "वाझ"), ("ms", "Oise"), ("nb", "Oise"), ("nl", "Oise"), ("no", "Oise"), ("pl", "Oise"), ("pt", "Oise"), ("ro", "Oise"), ("ru", "Уаза"), ("si", "ඔය\u{dd2}සේ"), ("sk", "Oise"), ("sl", "Oise"), ("sq", "Oise"), ("sr", "Оаза"), ("sr_Latn", "Oaza"), ("sv", "Oise"), ("sw", "Oise"), ("ta", "ச\u{bbe}ய\u{bcd}ஸ\u{bcd}"), ("te", "ఓయ\u{c3f}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอวซ"), ("tr", "Oise"), ("uk", "Уаза"), ("ur", "واز"), ("vi", "Oise"), ("yue", "華茲"), ("yue_Hans", "华兹"), ("zh", "瓦兹省")]),
                        unofficial_name_list: ["Oise"].to_vec(),
                    }
                ),
                (
                    "61",
                    Subdivision{
                        name: "61",
                        country_alpha2: Alpha2::FR,
                        code: "61",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.6388567), longitude: Some(0.0848201), max_latitude: Some(48.972535), min_latitude: Some(48.17988500000001), max_longitude: Some(0.976576), min_longitude: Some(-0.860575)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Orne"), ("ar", "أورن"), ("az", "Orn"), ("be", "Дэпартамент Орн"), ("bg", "Орн"), ("bn", "ওর\u{9cd}নে"), ("ca", "Orne"), ("ccp", "𑄃\u{11127}𑄢\u{11134}𑄚𑄬"), ("ceb", "Orne"), ("cs", "Orne"), ("cy", "Orne"), ("da", "Orne"), ("de", "Département Orne"), ("el", "Ορν"), ("en", "Orne"), ("es", "Orne"), ("et", "Orne’i departemang"), ("eu", "Orne"), ("fa", "اورن"), ("fi", "Orne"), ("fr", "Orne"), ("gl", "Orne"), ("gu", "ઓરન\u{ac7}"), ("he", "אורן"), ("hi", "ओर\u{94d}न"), ("hu", "Orne"), ("hy", "Օրն"), ("id", "Orne"), ("it", "Orne"), ("ja", "オルヌ県"), ("ka", "ორნი"), ("kk", "Орн"), ("kn", "ಒರ\u{ccd}ನ\u{cc6}"), ("ko", "오른 주"), ("lt", "Ornas"), ("lv", "Orna"), ("mr", "ऑर\u{94d}न"), ("ms", "Orne"), ("nb", "Orne"), ("nl", "Orne"), ("no", "Orne"), ("pl", "Orne"), ("pt", "Orne"), ("ro", "Orne"), ("ru", "Орн"), ("si", "ඔර\u{dca}නේ"), ("sk", "Orne"), ("sl", "Orne"), ("sq", "Orne"), ("sr", "Орн"), ("sr_Latn", "Orn"), ("sv", "Orne"), ("sw", "Orne"), ("ta", "ஓரனே"), ("te", "ఆర\u{c4d}న\u{c46}"), ("th", "จ\u{e31}งหว\u{e31}ดออร\u{e4c}น"), ("tr", "Orne"), ("uk", "Орн"), ("ur", "اورن"), ("vi", "Orne"), ("yue", "奧恩"), ("yue_Hans", "奥恩"), ("zh", "奧恩省")]),
                        unofficial_name_list: ["Orne"].to_vec(),
                    }
                ),
                (
                    "62",
                    Subdivision{
                        name: "62",
                        country_alpha2: Alpha2::FR,
                        code: "62",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.5732769), longitude: Some(2.3244679), max_latitude: Some(51.0067741), min_latitude: Some(50.01976), max_longitude: Some(3.1881861), min_longitude: Some(1.555598)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Pas-de-Calais"), ("ar", "باد كاليه"), ("be", "Дэпартамент Па-дэ-Кале"), ("bg", "Па дьо Кале"), ("bn", "প\u{9be}স-দে-ক\u{9cd}য\u{9be}ল\u{9be}ইস"), ("ca", "Pas-de-Calais"), ("ccp", "𑄛𑄌\u{11134}-𑄓𑄬-𑄇\u{11133}𑄠𑄣\u{1112d}𑄌\u{11134}"), ("ceb", "Pas-de-Calais"), ("cs", "Pas-de-Calais"), ("cy", "Pas-de-Calais"), ("da", "Pas-de-Calais"), ("de", "Département Pas-de-Calais"), ("el", "Πα-ντε-Καλαί"), ("en", "Pas-de-Calais"), ("es", "Paso de Calais"), ("et", "Pas-de-Calais’ departemang"), ("eu", "Pas-de-Calais"), ("fa", "پا-دو-کاله"), ("fi", "Pas-de-Calais"), ("fr", "Pas-de-Calais"), ("gl", "Pas-de-Calais"), ("gu", "પાસ-દ\u{ac7}-કાલાઈસ"), ("he", "פה-דה-קאלה"), ("hi", "पास-डी-कलाइस"), ("hu", "Pas-de-Calais"), ("hy", "Պա-դը-Կալե"), ("id", "Pas-de-Calais"), ("it", "Passo di Calais"), ("ka", "პა-დე-კალე"), ("kk", "Па-де-Кале"), ("kn", "ಪಾಸ\u{ccd}-ಡ\u{cc6}-ಕ\u{ccd}ಯಾಲೈಸ\u{ccd}"), ("ko", "파드칼레 주"), ("lt", "Pa de Kalė"), ("lv", "Padekalē"), ("mk", "Па де Кале"), ("mr", "पा-द-क\u{945}ल\u{947}"), ("ms", "Pas-de-Calais"), ("nb", "Pas-de-Calais"), ("nl", "Pas-de-Calais"), ("no", "Pas-de-Calais"), ("pl", "Pas-de-Calais"), ("pt", "Pas-de-Calais"), ("ro", "Pas-de-Calais"), ("ru", "Па-де-Кале"), ("si", "පස\u{dca}-ඩේ-කලය\u{dd2}ස\u{dca}"), ("sk", "Pas-de-Calais"), ("sl", "Pas-de-Calais"), ("sq", "Pas-de-Calais"), ("sr", "Па де Кале"), ("sr_Latn", "Pa de Kale"), ("sv", "Pas-de-Calais"), ("sw", "Pas-de-Calais"), ("ta", "ப\u{bbe}ஸ\u{bcd}-டி -கல\u{bbe}ய\u{bcd}ஸ\u{bcd}"), ("te", "ప\u{c3e}స\u{c4d}-ద-క\u{c3e}ల\u{c46}య\u{c3f}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดปาดกาแล"), ("tr", "Pas-de-Calais"), ("uk", "Па-де-Кале"), ("ur", "پا دے کالے"), ("vi", "Pas-de-Calais"), ("yue", "加來海峽"), ("yue_Hans", "加来海峡"), ("zh", "加来海峡省")]),
                        unofficial_name_list: ["Pas-de-Calais"].to_vec(),
                    }
                ),
                (
                    "63",
                    Subdivision{
                        name: "63",
                        country_alpha2: Alpha2::FR,
                        code: "63",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.7725738), longitude: Some(2.9644431), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Puy-de-Dôme"), ("ar", "بوي دي دوم"), ("be", "Дэпартамент Пюі-дэ-Дом"), ("bg", "Пюи дьо Дом"), ("bn", "প\u{9c1}য\u{9bc}-দে-ডোম"), ("ca", "Puèi Domat"), ("ccp", "𑄜\u{1112d}-𑄓𑄬-𑄓\u{1112e}𑄟\u{11134}"), ("ceb", "Puy-de-Dôme"), ("cs", "Puy-de-Dôme"), ("cy", "Puy-de-Dôme"), ("da", "Puy-de-Dôme"), ("de", "Département Puy-de-Dôme"), ("el", "Πουί-ντε-Ντομ"), ("en", "Puy-de-Dôme"), ("es", "Puy-de-Dôme"), ("et", "Puy-de-Dôme’i departemang"), ("eu", "Puy-de-Dôme"), ("fa", "پوی ددوم"), ("fi", "Puy-de-Dôme"), ("fr", "Puy-de-Dôme"), ("gl", "Puy-de-Dôme"), ("gu", "પ\u{acd}ય\u{ac1}-દ\u{ac7}-ડોમ"), ("he", "פויי-דה-דום"), ("hi", "प\u{941}ए-ड\u{947}-डोम"), ("hu", "Puy-de-Dôme"), ("hy", "Պյուի-դը-Դոմ"), ("id", "Puy-de-Dôme"), ("it", "Puy-de-Dôme"), ("ka", "პუი-დე-დომი"), ("kk", "Пюи-де-Дом"), ("kn", "ಪ\u{cc2}-ಡ\u{cbf}-ಡೋಮ\u{ccd}"), ("ko", "퓌드돔 주"), ("lt", "Piui de Domas"), ("lv", "Pijdedoma"), ("mr", "प\u{941}य-द\u{947}-दोम"), ("ms", "Puy-de-Dôme"), ("nb", "Puy-de-Dôme"), ("nl", "Puy-de-Dôme"), ("no", "Puy-de-Dôme"), ("pl", "Puy-de-Dôme"), ("pt", "Puy-de-Dôme"), ("ro", "Puy-de-Dôme"), ("ru", "Пюи-де-Дом"), ("si", "ප\u{dd4}ය\u{dd2}-ඩෙ-ඩෝමේ"), ("sk", "Puy-de-Dôme"), ("sl", "Puy-de-Dôme"), ("sq", "Puy-de-Dôme"), ("sr", "Пиј де Дом"), ("sr_Latn", "Pij de Dom"), ("sv", "Puy-de-Dôme"), ("sw", "Puy-de-Dôme"), ("ta", "பூய\u{bcd}-டி -டோமே"), ("te", "పుయ\u{c4d}-డ\u{c46}-డ\u{c4b}మ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดป\u{e38}ย-เดอ-โดม"), ("tr", "Puy-de-Dôme"), ("uk", "Пюї-де-Дом"), ("ur", "پوی-دو-دوم"), ("vi", "Puy-de-Dôme"), ("yue", "多姆山"), ("yue_Hans", "多姆山"), ("zh", "多姆山省")]),
                        unofficial_name_list: ["Puy-de-Dôme"].to_vec(),
                    }
                ),
                (
                    "64",
                    Subdivision{
                        name: "64",
                        country_alpha2: Alpha2::FR,
                        code: "64",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.3269942), longitude: Some(-0.7532808999999999), max_latitude: Some(43.59672490000001), min_latitude: Some(42.777531), max_longitude: Some(0.029807), min_longitude: Some(-1.7923251)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Pyrénées-Atlantiques"), ("ar", "البرانيس الأطلسية"), ("az", "Atlantik Pireney"), ("be", "Пірэнеі Атлантычныя"), ("bg", "Пирене Атлантик"), ("bn", "পিরেনিস-আটল\u{9be}ন\u{9cd}টিস"), ("ca", "Pirineus Atlàntics"), ("ccp", "𑄛\u{1112d}𑄢𑄬𑄚\u{11128}𑄌\u{11134}-𑄃𑄖\u{11134}𑄣𑄚\u{11134}𑄑\u{11128}𑄇\u{11134}𑄌\u{11134}"), ("ceb", "Pyrénées-Atlantiques"), ("cs", "Pyrénées-Atlantiques"), ("cy", "Pyrénées-Atlantiques"), ("da", "Pyrénées-Atlantiques"), ("de", "Département Pyrénées-Atlantiques"), ("el", "Ατλαντικά Πυρηναία"), ("en", "Pyrénées-Atlantiques"), ("es", "Pirineos Atlánticos"), ("et", "Pyrénées-Atlantiques"), ("eu", "Pirinio Atlantikoak"), ("fa", "پیرنه-آتلانتیک"), ("fi", "Pyrénées-Atlantiques"), ("fr", "Pyrénées-Atlantiques"), ("gl", "Pireneos Atlánticos"), ("gu", "પીર\u{ac7}ન\u{ac7}સ-એટલાન\u{acd}ટીક"), ("he", "הפירנאים האטלנטיים"), ("hi", "पायर\u{947}नीज\u{93c}-अटला\u{902}टिक\u{947}स"), ("hr", "Atlantski Pireneji"), ("hu", "Pyrénées-Atlantiques"), ("hy", "Ատլանտյան Պիրենեյներ"), ("id", "Pyrénées-Atlantiques"), ("it", "Pirenei Atlantici"), ("ka", "ატლანტური პირენეები"), ("kk", "Атлантикалық Пиреней"), ("kn", "ಪೈರ\u{cbf}ನೀಸ\u{ccd}-ಅಟ\u{ccd}ಲಾಂಟ\u{cbf}ಕ\u{ccd}ಸ\u{ccd}"), ("ko", "피레네자틀랑티크 주"), ("lt", "Atlanto Pirėnai"), ("lv", "Atlantijas Pireneji"), ("mr", "पिर\u{947}न\u{947}-अतला\u{902}तिक"), ("ms", "Pyrénées-Atlantiques"), ("nb", "Pyrénées-Atlantiques"), ("nl", "Pyrénées-Atlantiques"), ("no", "Pyrénées-Atlantiques"), ("pl", "Pireneje Atlantyckie"), ("pt", "Pirineus Atlânticos"), ("ro", "Pyrénées-Atlantiques"), ("ru", "Атлантические Пиренеи"), ("si", "ප\u{dd2}රෙන\u{dd3}ස\u{dca} අට\u{dca}ල\u{dcf}න\u{dca}ට\u{dd2}ක\u{dca}ස\u{dca}"), ("sk", "Pyrénées-Atlantiques"), ("sl", "Pyrénées-Atlantiques"), ("sq", "Pyrénées-Atlantiques"), ("sr", "Атлантски Пиринеји"), ("sr_Latn", "Atlantski Pirineji"), ("sv", "Pyrénées-Atlantiques"), ("sw", "Pyrénées-Atlantiques"), ("ta", "பெயரென\u{bc0}ஸ\u{bcd} -அட\u{bcd}ல\u{bbe}ன\u{bcd}டியூஸ\u{bcd}"), ("te", "ప\u{c48}ర\u{c46}న\u{c40}స\u{c4d}-అట\u{c4d}ల\u{c3e}ంట\u{c3f}క\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดป\u{e35}เรเน-อ\u{e31}ตล\u{e47}องต\u{e34}ก"), ("tr", "Pyrénées-Atlantiques"), ("uk", "Атлантичні Піренеї"), ("ur", "پیرینے-اتلانتیک"), ("vi", "Pyrénées-Atlantiques"), ("yue", "比利牛斯－大西洋"), ("yue_Hans", "比利牛斯－大西洋"), ("zh", "比利牛斯-大西洋省")]),
                        unofficial_name_list: ["Pyrénées-Atlantiques"].to_vec(),
                    }
                ),
                (
                    "65",
                    Subdivision{
                        name: "65",
                        country_alpha2: Alpha2::FR,
                        code: "65",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.0193924), longitude: Some(0.1494988), max_latitude: Some(43.61333399999999), min_latitude: Some(42.6733059), max_longitude: Some(0.6461191), min_longitude: Some(-0.32716)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hautes-Pyrénées"), ("ar", "البرانيس العليا"), ("az", "Yuxarı Pireney"), ("be", "Дэпартамент Пірэнеі Верхнія"), ("bg", "От Пирене"), ("bn", "হ\u{9be}উতেস প\u{9be}য\u{9bc}রেন\u{9cd}স"), ("ca", "Alts Pirineus"), ("ccp", "𑄦𑄅\u{1112a}𑄖\u{11134}-𑄛\u{1112d}𑄢𑄬𑄚\u{11128}𑄌\u{11134}"), ("ceb", "Hautes-Pyrénées"), ("cs", "Hautes-Pyrénées"), ("cy", "Hautes-Pyrénées"), ("da", "Hautes-Pyrénées"), ("de", "Département Hautes-Pyrénées"), ("el", "Άνω Πυρηναία"), ("en", "Hautes-Pyrénées"), ("es", "Altos Pirineos"), ("et", "Hautes-Pyrénées"), ("eu", "Pirinio Garaiak"), ("fa", "اوپیرنه"), ("fi", "Hautes-Pyrénées"), ("fr", "Hautes-Pyrénées"), ("gl", "Altos Pireneos"), ("gu", "હોટ\u{acd}સ-પિર\u{ac7}ન\u{ac7}સ"), ("he", "הפירנאים העליונים"), ("hi", "ओट\u{94d}स-पायर\u{947}नीज\u{93c}"), ("hu", "Hautes-Pyrénées"), ("hy", "Վերին Պիրենեյներ"), ("id", "Hautes-Pyrénées"), ("it", "Alti Pirenei"), ("ka", "ზემო პირენეები"), ("kk", "Жоғарғы Пиреней"), ("kn", "ಹ\u{ccc}ಟ\u{cc6}ಸ\u{ccd}-ಪೈರ\u{cbf}ನೀಸ\u{ccd}"), ("ko", "오트피레네 주"), ("lt", "Aukštutiniai Pirėnai"), ("lv", "Augšpireneji"), ("mk", "Горни Пиринеи"), ("mr", "ऑत-पिर\u{947}न\u{947}"), ("ms", "Hautes-Pyrénées"), ("nb", "Hautes-Pyrénées"), ("nl", "Hautes-Pyrénées"), ("no", "Hautes-Pyrénées"), ("pl", "Pireneje Wysokie"), ("pt", "Altos Pirenéus"), ("ro", "Hautes-Pyrénées"), ("ru", "Верхние Пиренеи"), ("si", "හෞට\u{dd2}ස\u{dca}-පය\u{dd2}රෙන\u{dd2}ස\u{dca}"), ("sk", "Hautes-Pyrénées"), ("sl", "Hautes-Pyrénées"), ("sq", "Hautes-Pyrénées"), ("sr", "Високи Пиринеји"), ("sr_Latn", "Visoki Pirineji"), ("sv", "Hautes-Pyrénées"), ("sw", "Hautes-Pyrénées"), ("ta", "ஹூட\u{bcd}ஸ\u{bcd} -பெயரென\u{bc0}ஸ\u{bcd}"), ("te", "హ\u{c3e}ట\u{c4d}స\u{c4d} ప\u{c48}ర\u{c46}న\u{c40}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโอต-ป\u{e35}เรเน"), ("tr", "Hautes-Pyrénées"), ("uk", "Верхні Піренеї"), ("ur", "بالائی-پیرینے"), ("vi", "Hautes-Pyrénées"), ("yue", "上比利牛斯"), ("yue_Hans", "上比利牛斯"), ("zh", "上比利牛斯省")]),
                        unofficial_name_list: ["Hautes-Pyrénées"].to_vec(),
                    }
                ),
                (
                    "66",
                    Subdivision{
                        name: "66",
                        country_alpha2: Alpha2::FR,
                        code: "66",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.6012912), longitude: Some(2.539603), max_latitude: Some(42.9185399), min_latitude: Some(42.333014), max_longitude: Some(3.177833), min_longitude: Some(1.7216351)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Pyrénées-Orientales"), ("ar", "البرانيس الشرقية"), ("az", "Şərqi Pireney"), ("be", "Дэпартамент Пірэнеі Усходнія"), ("bg", "Източни Пиренеи"), ("bn", "প\u{9be}ইরিনিস-ওরিয\u{9bc}েন\u{9cd}ট\u{9be}লিস"), ("ca", "Pirineus Orientals"), ("ccp", "𑄛\u{1112d}𑄢𑄬𑄚\u{11128}𑄌\u{11134}-𑄃\u{11127}𑄢\u{11128}𑄃𑄬𑄚\u{11134}𑄑𑄣\u{11134}𑄌\u{11134}"), ("ceb", "Pyrénées-Orientales"), ("cs", "Pyrénées-Orientales"), ("cy", "Pyrénées-Orientales"), ("da", "Pyrénées-Orientales"), ("de", "Département Pyrénées-Orientales"), ("el", "Ανατολικά Πυρηναία"), ("en", "Pyrénées-Orientales"), ("es", "Pirineos Orientales"), ("et", "Pyrénées-Orientales"), ("eu", "Ekialdeko Pirinioak"), ("fa", "پیرنه اوریانتل"), ("fi", "Pyrénées-Orientales"), ("fr", "Pyrénées-Orientales"), ("gl", "Pireneos Orientais"), ("gu", "પિર\u{ac7}ન\u{ac7}સ-ઓરિએ\u{a82}ટાલ\u{ac7}સ"), ("he", "הפירנאים המזרחיים"), ("hi", "पायर\u{947}नीस-ओरिय\u{902}टल\u{94d}स"), ("hu", "Pyrénées-Orientales"), ("hy", "Արևելյան Պիրենեյներ"), ("id", "Pyrénées-Orientales"), ("it", "Pirenei Orientali"), ("ka", "აღმოსავლეთ პირენეის დეპარტამენტი"), ("kk", "Шығыс Пиреней"), ("kn", "ಪೈರ\u{cbf}ನೀಸ\u{ccd}-ಓರ\u{cbf}ಯಂಟ\u{cc6}ಸ\u{ccd}"), ("ko", "피레네조리앙탈 주"), ("lt", "Rytų Pirėnai"), ("lv", "Austrumpireneji"), ("mn", "Дорно Пиреней"), ("mr", "पिर\u{947}न\u{947}-ओरिए\u{902}ताल"), ("ms", "Pyrénées-Orientales"), ("nb", "Pyrénées-Orientales"), ("nl", "Pyrénées-Orientales"), ("no", "Pyrénées-Orientales"), ("pl", "Pireneje Wschodnie"), ("pt", "Pirineus Orientais"), ("ro", "Pyrénées-Orientales"), ("ru", "Восточные Пиренеи"), ("si", "පය\u{dca}රෙන\u{dd2}ස\u{dca}-ඔර\u{dd2}යෙන\u{dca}ටල\u{dd2}ස\u{dca}"), ("sk", "Pyrénées-Orientales"), ("sl", "Pyrénées-Orientales"), ("sq", "Pyrénées-Orientales"), ("sr", "Источни Пиринеји"), ("sr_Latn", "Istočni Pirineji"), ("sv", "Pyrénées-Orientales"), ("sw", "Pyrénées-Orientales"), ("ta", "ப\u{bcd}ய\u{bcd}ரந\u{bc0}ஸ\u{bcd} -ஒர\u{bc0}எண\u{bcd}ட\u{bcd}லெஸ\u{bcd}"), ("te", "ప\u{c48}ర\u{c46}న\u{c40}స\u{c4d}-ఓర\u{c3f}య\u{c46}ంట\u{c47}ల\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดป\u{e35}เรเน-ออร\u{e35}ย\u{e47}องตาล"), ("tr", "Pyrénées-Orientales"), ("uk", "Східні Піренеї"), ("ur", "پیرینے-اورینتال"), ("vi", "Pyrénées-Orientales"), ("yue", "東比利牛斯省"), ("yue_Hans", "东比利牛斯省"), ("zh", "东比利牛斯省")]),
                        unofficial_name_list: ["Pyrénées-Orientales"].to_vec(),
                    }
                ),
                (
                    "67",
                    Subdivision{
                        name: "67",
                        country_alpha2: Alpha2::FR,
                        code: "67",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.6343172), longitude: Some(7.525293800000001), max_latitude: Some(49.0778581), min_latitude: Some(48.120387), max_longitude: Some(8.2335491), min_longitude: Some(6.9406139)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Bas-Rhin"), ("ar", "الراين الأسفل"), ("az", "Aşağı Reyn"), ("be", "Рэйн Ніжні"), ("bg", "Ба Рен"), ("bn", "ব\u{9be}স-হ\u{9cd}রিন"), ("ca", "Baix Rin"), ("ccp", "𑄝𑄌\u{11134}-𑄢\u{1112d}𑄚\u{11134}"), ("ceb", "Bas-Rhin"), ("cs", "Bas-Rhin"), ("cy", "Bas-Rhin"), ("da", "Bas-Rhin"), ("de", "Département Bas-Rhin"), ("el", "Κάτω Ρήνος"), ("en", "Bas-Rhin"), ("es", "Bajo Rin"), ("et", "Bas-Rhini departemang"), ("eu", "Bas-Rhin"), ("fa", "با-رن"), ("fi", "Bas-Rhin"), ("fr", "Bas-Rhin"), ("gl", "Baixo Rin"), ("gu", "બાસ-રાઈન"), ("he", "ריין תחתון"), ("hi", "बास-राइन"), ("hr", "Bas-Rhin"), ("hu", "Bas-Rhin"), ("hy", "Ներքին Ռեն"), ("id", "Bas-Rhin"), ("it", "Basso Reno"), ("ka", "ქვემო რეინი"), ("kk", "Төменгі Рейн"), ("kn", "ಬಾಸ\u{ccd}-ರೈನ\u{ccd}"), ("ko", "바랭 주"), ("lt", "Žemutinis Reinas"), ("lv", "Lejasreina"), ("mk", "Долна Рајна"), ("mr", "बास-ऱ\u{94d}हिन"), ("ms", "Bas-Rhin"), ("nb", "Bas-Rhin"), ("nl", "Bas-Rhin"), ("no", "Bas-Rhin"), ("pl", "Dolny Ren"), ("pt", "Baixo Reno"), ("ro", "Bas-Rhin"), ("ru", "Нижний Рейн"), ("si", "බ\u{dcf}ස\u{dca}-ර\u{dd2}න\u{dca}"), ("sk", "Bas-Rhin"), ("sl", "Bas-Rhin"), ("sq", "Bas-Rhin"), ("sr", "Доња Рајна"), ("sr_Latn", "Donja Rajna"), ("sv", "Bas-Rhin"), ("sw", "Bas-Rhin"), ("ta", "பஸ\u{bcd} -ரஹ\u{bc0}ன\u{bcd}"), ("te", "బ\u{c3e}స\u{c4d}-ర\u{c48}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบา-แร\u{e47}ง"), ("tr", "Bas-Rhin"), ("uk", "Нижній Рейн"), ("ur", "زیریں-رن"), ("vi", "Bas-Rhin"), ("yue", "下萊茵"), ("yue_Hans", "下莱茵"), ("zh", "下莱茵省")]),
                        unofficial_name_list: ["Bas-Rhin"].to_vec(),
                    }
                ),
                (
                    "68",
                    Subdivision{
                        name: "68",
                        country_alpha2: Alpha2::FR,
                        code: "68",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.9315041), longitude: Some(7.2441099), max_latitude: Some(48.311198), min_latitude: Some(47.4202619), max_longitude: Some(7.622121099999998), min_longitude: Some(6.841025999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Haut-Rhin"), ("ar", "الراين الأعلى"), ("az", "Yuxarı Reyn"), ("be", "Рэйн Верхні"), ("bg", "О Рен"), ("bn", "হন\u{9cd}ত-রিন"), ("ca", "Alt Rin"), ("ccp", "𑄦𑄅\u{1112a}𑄖\u{11134}-𑄢\u{1112d}𑄚\u{11134}"), ("ceb", "Haut-Rhin"), ("cs", "Haut-Rhin"), ("cy", "Haut-Rhin"), ("da", "Haut-Rhin"), ("de", "Département Haut-Rhin"), ("el", "Άνω Ρήνος"), ("en", "Haut-Rhin"), ("es", "Alto Rin"), ("et", "Haut-Rhini departemang"), ("eu", "Haut-Rhin"), ("fa", "اوت-رن"), ("fi", "Haut-Rhin"), ("fr", "Haut-Rhin"), ("gl", "Alto Rin"), ("gu", "હોટ-રીન"), ("he", "או-רן"), ("hi", "हौत-रिन"), ("hr", "Haut-Rhin"), ("hu", "Haut-Rhin"), ("hy", "Վերին Հռենոս"), ("id", "Haut-Rhin"), ("it", "Alto Reno"), ("ka", "ზემო რეინი"), ("kk", "Жоғарғы Рейн"), ("kn", "ಹ\u{ccc}ಟ\u{ccd}-ರೈನ\u{ccd}"), ("ko", "오랭 주"), ("ky", "Жогорку Рейн ойдуңу"), ("lt", "Aukštutinis Reinas"), ("lv", "Augšreina"), ("mk", "Горна Рајна"), ("mr", "ऑत-ऱ\u{94d}हिन"), ("ms", "Haut-Rhin"), ("nb", "Haut-Rhin"), ("nl", "Haut-Rhin"), ("no", "Haut-Rhin"), ("pl", "Górny Ren"), ("pt", "Alto Reno"), ("ro", "Haut-Rhin"), ("ru", "Верхний Рейн"), ("si", "හව\u{dd4}ට\u{dca} -ර\u{dd2}න\u{dca}"), ("sk", "Haut-Rhin"), ("sl", "Haut-Rhin"), ("sq", "Alto Rin"), ("sr", "Горња Рајна"), ("sr_Latn", "Gornja Rajna"), ("sv", "Haut-Rhin"), ("sw", "Haut-Rhin"), ("ta", "ஹூட\u{bcd} -ரஹ\u{bc0}ன\u{bcd}"), ("te", "హ\u{c3e}ట\u{c4d}-ర\u{c48}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโอ-แร\u{e47}ง"), ("tr", "Haut-Rhin"), ("uk", "Верхній Рейн"), ("ur", "بلند-رن"), ("vi", "Haut-Rhin"), ("yue", "上萊茵"), ("yue_Hans", "上莱茵"), ("zh", "上莱茵省")]),
                        unofficial_name_list: ["Haut-Rhin"].to_vec(),
                    }
                ),
                (
                    "69",
                    Subdivision{
                        name: "69",
                        country_alpha2: Alpha2::FR,
                        code: "69",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.7351456), longitude: Some(4.6108043), max_latitude: Some(46.30650199999999), min_latitude: Some(45.45413), max_longitude: Some(5.1601089), min_longitude: Some(4.243647)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Rhône (département)"), ("ar", "رون"), ("az", "Rona (departament)"), ("be", "Рона"), ("bg", "Рон"), ("bn", "রোন\u{9cd}"), ("ca", "Roine"), ("ccp", "𑄢\u{1112e}𑄚\u{11134}"), ("ceb", "Rhône"), ("cs", "Rhône"), ("cy", "‘department’ y Rhône"), ("da", "Rhône"), ("de", "Département Rhône"), ("el", "Ρον"), ("en", "Rhône"), ("es", "Ródano"), ("et", "Rhône’i departemang"), ("eu", "Rodano"), ("fa", "رون"), ("fi", "Rhône"), ("fr", "Rhône"), ("gl", "Ródano"), ("gu", "રૉન"), ("he", "רון"), ("hi", "रोन (विभाग)"), ("hu", "Rhône"), ("hy", "Ռոն"), ("id", "Rhône"), ("it", "Rodano"), ("ja", "ローヌ県"), ("ka", "რონის დეპარტამენტი"), ("kk", "Рона"), ("kn", "ರೋನ\u{ccd}"), ("ko", "론 주"), ("lt", "Rona"), ("lv", "Rona"), ("mk", "Рона"), ("mr", "रोन"), ("ms", "Rhône"), ("nb", "Rhône"), ("nl", "Rhône"), ("no", "Rhône"), ("pl", "Rodan"), ("pt", "Ródano"), ("ro", "Rhône"), ("ru", "Рона"), ("si", "රෝනේ"), ("sk", "Rhône"), ("sl", "Rhône"), ("sq", "Rhône"), ("sr", "Рона"), ("sr_Latn", "Rona"), ("sv", "Rhône"), ("sw", "Rhône"), ("ta", "ரஹ\u{bbe}னே"), ("te", "ర\u{c4b}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโรน"), ("tr", "Rhône"), ("uk", "Рона"), ("ur", "رون"), ("vi", "Rhône"), ("yue", "隆河省"), ("yue_Hans", "隆河省"), ("zh", "罗讷省")]),
                        unofficial_name_list: ["Rhône"].to_vec(),
                    }
                ),
                (
                    "69M",
                    Subdivision{
                        name: "69M",
                        country_alpha2: Alpha2::FR,
                        code: "69M",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.7351456), longitude: Some(4.6108043), max_latitude: Some(46.30650199999999), min_latitude: Some(45.45413), max_longitude: Some(5.1601089), min_longitude: Some(4.243647)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCollectivityWithSpecialStatus,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Lyon Metropolis"), ("fr", "Métropole de Lyon")]),
                        unofficial_name_list: ["Métropole de Lyon"].to_vec(),
                    }
                ),
                (
                    "6AE",
                    Subdivision{
                        name: "6AE",
                        country_alpha2: Alpha2::FR,
                        code: "6AE",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::EuropeanCollectivity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Alsace"), ("fr", "Alsace")]),
                        unofficial_name_list: ["Alsace"].to_vec(),
                    }
                ),
                (
                    "70",
                    Subdivision{
                        name: "70",
                        country_alpha2: Alpha2::FR,
                        code: "70",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.7569806), longitude: Some(6.1556282), max_latitude: Some(48.024154), min_latitude: Some(47.2525531), max_longitude: Some(6.8249469), min_longitude: Some(5.366937099999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Haute-Saône"), ("ar", "سون العليا"), ("az", "Yuxarı Sona"), ("be", "Дэпартамент Сона Верхняя"), ("bg", "От Сон"), ("bn", "হোতে-স\u{9be}ওন"), ("ca", "Alt Saona"), ("ccp", "𑄦𑄅\u{1112a}𑄖\u{11134}-𑄥𑄃\u{1112e}𑄚\u{11128}"), ("ceb", "Haute-Saône"), ("cs", "Haute-Saône"), ("cy", "Haute-Saône"), ("da", "Haute-Saône"), ("de", "Département Haute-Saône"), ("el", "Ωτ-Σον"), ("en", "Haute-Saône"), ("es", "Alto Saona"), ("et", "Haute-Saône’i departemang"), ("eu", "Haute-Saône"), ("fa", "اوت-سئون"), ("fi", "Haute-Saône"), ("fr", "Haute-Saône"), ("gl", "Alto Saona"), ("gu", "હૌટ-સઓન"), ("he", "סון עילית"), ("hi", "ओट-साओन"), ("hr", "Gornja Saôna"), ("hu", "Haute-Saône"), ("hy", "Վերին Սոն"), ("id", "Haute-Saône"), ("it", "Alta Saona"), ("ka", "ზემო სონა"), ("kk", "Жоғарғы Сона"), ("kn", "ಹಾಟ\u{cc6}-ಸೊನ\u{cc6}"), ("ko", "오트손 주"), ("lt", "Aukštutinė Sona"), ("lv", "Augšsona"), ("mk", "Горна Сона"), ("mr", "ऑत-सॉन"), ("ms", "Haute-Saône"), ("nb", "Haute-Saône"), ("nl", "Haute-Saône"), ("no", "Haute-Saône"), ("pl", "Górna Saona"), ("pt", "Haute-Saône"), ("ro", "Haute-Saône"), ("ru", "Верхняя Сона"), ("si", "හෞටේ -සඕනේ"), ("sk", "Haute-Saône"), ("sl", "Haute-Saône"), ("sq", "Haute-Saône"), ("sr", "Горња Саона"), ("sr_Latn", "Gornja Saona"), ("sv", "Haute-Saône"), ("sw", "Haute-Saône"), ("ta", "ஹூட\u{bcd}-ஷொனே"), ("te", "హ\u{c3e}ట\u{c4d}-సయ\u{c4b}న\u{c3f}"), ("th", "จ\u{e31}งหว\u{e31}ดโอต-โซน"), ("tr", "Haute-Saône"), ("uk", "Верхня Сона"), ("ur", "بالائی-سون"), ("vi", "Haute-Saône"), ("yue", "上索恩"), ("yue_Hans", "上索恩"), ("zh", "上索恩省")]),
                        unofficial_name_list: ["Haute-Saône"].to_vec(),
                    }
                ),
                (
                    "71",
                    Subdivision{
                        name: "71",
                        country_alpha2: Alpha2::FR,
                        code: "71",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5827512), longitude: Some(4.486670999999999), max_latitude: Some(47.155772), min_latitude: Some(46.15607), max_longitude: Some(5.465288999999999), min_longitude: Some(3.622593)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Saône-et-Loire"), ("ar", "سون ولوار"), ("az", "Sona və Luara"), ("be", "Дэпартамент Сона і Луара"), ("bg", "Сон е Лоар"), ("bn", "সোনে ইট লরি"), ("ca", "Saona i Loira"), ("ccp", "𑄥𑄃\u{1112e}𑄚\u{11128}-𑄃𑄬𑄖\u{11134}-𑄣\u{11130}𑄢\u{11134}"), ("ceb", "Saône-et-Loire"), ("cs", "Saône-et-Loire"), ("cy", "Saône-et-Loire"), ("da", "Saône-et-Loire"), ("de", "Département Saône-et-Loire"), ("el", "Σον-ε-Λουάρ"), ("en", "Saône-et-Loire"), ("es", "Saona y Loira"), ("et", "Saône-et-Loire’i departemang"), ("eu", "Saône-et-Loire"), ("fa", "سائون-ا-لوآر"), ("fi", "Saône-et-Loire"), ("fr", "Saône-et-Loire"), ("gl", "Saona e Loira"), ("gu", "સાઓન-એટ-લોઈર"), ("he", "סון ולואר"), ("hi", "साओन-एट-लोइर"), ("hu", "Saône-et-Loire"), ("hy", "Սոն-է-Լուար"), ("id", "Saône-et-Loire"), ("it", "Saona e Loira"), ("ka", "სონა და ლუარა"), ("kk", "Сона және Луара"), ("kn", "ಸ\u{ccc}ನ\u{cc6}-ಎಟ\u{ccd}-ಲೋರ\u{cc6}"), ("ko", "손에루아르 주"), ("lt", "Sona ir Luara"), ("lv", "Sona un Luāra"), ("mr", "सॉन-ए-लावार"), ("ms", "Saône-et-Loire"), ("nb", "Saône-et-Loire"), ("nl", "Saône-et-Loire"), ("no", "Saône-et-Loire"), ("pl", "Saona i Loara"), ("pt", "Saône-et-Loire"), ("ro", "Saône-et-Loire"), ("ru", "Сона и Луара"), ("si", "සොනේ-එට\u{dca}\u{200c}-ලොය\u{dd2}ර\u{dca}"), ("sk", "Saône-et-Loire"), ("sl", "Saône-et-Loire"), ("sq", "Saône-et-Loire"), ("sr", "Саона и Лоара"), ("sr_Latn", "Saona i Loara"), ("sv", "Saône-et-Loire"), ("sw", "Saône-et-Loire"), ("ta", "சேயோன\u{bcd}-எட\u{bcd} -லோயிரே"), ("te", "స\u{c3e}వన\u{c4d}-ఎట\u{c4d}-ల\u{c4b}యర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโซเนล\u{e31}วร\u{e4c}"), ("tr", "Saône-et-Loire"), ("uk", "Сона і Луара"), ("ur", "سون-اے-لوآر"), ("vi", "Saône-et-Loire"), ("yue", "索恩盧瓦"), ("yue_Hans", "索恩卢瓦"), ("zh", "索恩-卢瓦尔省")]),
                        unofficial_name_list: ["Saône-et-Loire"].to_vec(),
                    }
                ),
                (
                    "72",
                    Subdivision{
                        name: "72",
                        country_alpha2: Alpha2::FR,
                        code: "72",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.9217014), longitude: Some(0.1655803), max_latitude: Some(48.48502), min_latitude: Some(47.56840099999999), max_longitude: Some(0.9166388999999999), min_longitude: Some(-0.4480631)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sarthe"), ("ar", "سارت"), ("az", "Sarta"), ("be", "Дэпартамент Сарта"), ("bg", "Сарт"), ("bn", "স\u{9be}\u{981}র\u{9cd}তে"), ("ca", "Sarthe"), ("ccp", "𑄥𑄢\u{11134}𑄗𑄬"), ("ceb", "Sarthe"), ("cs", "Sarthe"), ("cy", "Sarthe"), ("da", "Sarthe"), ("de", "Département Sarthe"), ("el", "Σαρτ"), ("en", "Sarthe"), ("es", "Sarthe"), ("et", "Sarthe’i departemang"), ("eu", "Sarthe"), ("fa", "سرت"), ("fi", "Sarthe"), ("fr", "Sarthe"), ("gl", "Sarthe"), ("gu", "સર\u{acd}થ\u{ac7}"), ("he", "סארת"), ("hi", "सार\u{94d}थ\u{947}"), ("hu", "Sarthe"), ("hy", "Սարթ"), ("id", "Sarthe"), ("it", "Sarthe"), ("ja", "サルト県"), ("ka", "სარტა"), ("kk", "Сарта"), ("kn", "ಸಾರ\u{ccd}ಥ\u{cc6}"), ("ko", "사르트 주"), ("lt", "Sartas"), ("lv", "Sarta"), ("mr", "सार\u{94d}त"), ("ms", "Sarthe"), ("nb", "Sarthe"), ("nl", "Sarthe"), ("no", "Sarthe"), ("pl", "Sarthe"), ("pt", "Sarthe"), ("ro", "Sarthe"), ("ru", "Сарта"), ("si", "සර\u{dca}තේ"), ("sk", "Sarthe"), ("sl", "Sarthe"), ("sq", "Sarthe"), ("sr", "Сарт"), ("sr_Latn", "Sart"), ("sv", "Sarthe"), ("sw", "Sarthe"), ("ta", "சேர\u{bcd}த\u{bcd}தே"), ("te", "స\u{c3e}ర\u{c4d}థ\u{c46}"), ("th", "จ\u{e31}งหว\u{e31}ดซาร\u{e4c}ต"), ("tr", "Sarthe"), ("uk", "Сарта"), ("ur", "سارت"), ("vi", "Sarthe"), ("yue", "薩爾特"), ("yue_Hans", "萨尔特"), ("zh", "萨尔特省")]),
                        unofficial_name_list: ["Sarthe"].to_vec(),
                    }
                ),
                (
                    "73",
                    Subdivision{
                        name: "73",
                        country_alpha2: Alpha2::FR,
                        code: "73",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.771079), longitude: Some(5.742806000000001), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Savoie"), ("ar", "سافوا"), ("az", "Savoyya"), ("be", "Дэпартамент Савоя"), ("bg", "Савоа"), ("bn", "স\u{9be}ভ\u{9c1}য\u{9bc}ে"), ("ca", "Savoia"), ("ccp", "𑄥\u{11127}𑄞\u{11130}"), ("ceb", "Savoie"), ("cs", "Savojsko"), ("cy", "Savoie"), ("da", "Savoie"), ("de", "Département Savoie"), ("el", "Σαβοΐα"), ("en", "Savoie"), ("es", "Saboya"), ("et", "Savoia departemang"), ("eu", "Savoia"), ("fa", "سووآ"), ("fi", "Savoie"), ("fr", "Savoie"), ("gl", "Savoia"), ("gu", "સાવોઈ"), ("he", "סבואה"), ("hi", "सवोई"), ("hu", "Savoie"), ("hy", "Սավոյ"), ("id", "Savoie"), ("it", "Savoia"), ("ja", "サヴォワ県"), ("ka", "სავოია"), ("kk", "Савойя"), ("kn", "ಸವೊಯ\u{cbf}"), ("ko", "사부아 주"), ("lt", "Savoja"), ("lv", "Savoja"), ("mk", "Савоја"), ("mr", "साव\u{94d}वा"), ("ms", "Savoie"), ("nb", "Savoie"), ("ne", "साभोए"), ("nl", "Savoie"), ("no", "Savoie"), ("pl", "Sabaudia"), ("pt", "Saboia"), ("ro", "Savoie"), ("ru", "Савойя"), ("si", "සැවෝය\u{dd2}"), ("sk", "Savoie"), ("sl", "Savoie"), ("sq", "Savoia"), ("sr", "Савоја"), ("sr_Latn", "Savoja"), ("sv", "Savoie"), ("sw", "Savoie"), ("ta", "ச\u{bbe}வோய\u{bcd}"), ("te", "సవ\u{c4b}య\u{c40}"), ("th", "จ\u{e31}งหว\u{e31}ดซาว\u{e31}ว"), ("tr", "Savoie"), ("uk", "Савойя"), ("ur", "ساووا"), ("vi", "Savoie"), ("yue", "薩華"), ("yue_Hans", "萨华"), ("zh", "萨瓦省")]),
                        unofficial_name_list: ["Savoie"].to_vec(),
                    }
                ),
                (
                    "74",
                    Subdivision{
                        name: "74",
                        country_alpha2: Alpha2::FR,
                        code: "74",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1756788), longitude: Some(6.5389621), max_latitude: Some(46.408243), min_latitude: Some(45.681659), max_longitude: Some(7.0450649), min_longitude: Some(5.80502)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Haute-Savoie"), ("ar", "سافوا العليا"), ("az", "Yuxarı Savoyya"), ("be", "Дэпартамент Верхняя Савоя"), ("bg", "От Савоа"), ("bn", "হ\u{9be}ঊত সেভ\u{9c1}য\u{9bc}ে"), ("ca", "Alta Savoia"), ("ccp", "𑄦𑄅\u{1112a}𑄖\u{11134}-𑄥\u{11127}𑄞\u{11130}"), ("ceb", "Haute-Savoie"), ("cs", "Horní Savojsko"), ("cy", "Haute-Savoie"), ("da", "Haute-Savoie"), ("de", "Département Haute-Savoie"), ("el", "Άνω Σαβοΐα"), ("en", "Haute-Savoie"), ("es", "Alta Saboya"), ("et", "Haute-Savoie departemang"), ("eu", "Savoia Garaia"), ("fa", "اوت سووآ"), ("fi", "Haute-Savoie"), ("fr", "Haute-Savoie"), ("gl", "Alta Savoia"), ("gu", "હૌટ-સ\u{ac7}વોઇ"), ("he", "סבואה עילית"), ("hi", "ओट-स\u{947}वॉइ"), ("hu", "Haute-Savoie"), ("hy", "Վերին Սավոյ"), ("id", "Haute-Savoie"), ("it", "Alta Savoia"), ("ka", "ზემო სავოია"), ("kk", "Жоғарғы Савойя"), ("kn", "ಹ\u{ccc}ಟ\u{cc6}-ಸವೊಯ\u{cbf}"), ("ko", "오트사부아 주"), ("lt", "Aukštutinė Savoja"), ("lv", "Augšsavoja"), ("mr", "ऑत-साव\u{94d}वा"), ("ms", "Haute-Savoie"), ("nb", "Haute-Savoie"), ("nl", "Haute-Savoie"), ("no", "Haute-Savoie"), ("pl", "Górna Sabaudia"), ("pt", "Alta Saboia"), ("ro", "Haute-Savoie"), ("ru", "Верхняя Савойя"), ("si", "හෞටේ -සැවෝය\u{dd2}"), ("sk", "Haute-Savoie"), ("sl", "Haute-Savoie"), ("sq", "Haute-Savoie"), ("sr", "Горња Савоја"), ("sr_Latn", "Gornja Savoja"), ("sv", "Haute-Savoie"), ("sw", "Haute-Savoie"), ("ta", "ஹூட\u{bcd}-ச\u{bbe}வையே"), ("te", "హ\u{c3e}ట\u{c4d}-స\u{c3e}వ\u{c4b}య\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโอต-ซาว\u{e31}ว"), ("tr", "Haute-Savoie"), ("uk", "Верхня Савойя"), ("ur", "بالائی-ساووا"), ("vi", "Haute-Savoie"), ("yo", "Haute-Savoie"), ("yo_BJ", "Haute-Savoie"), ("yue", "上薩華"), ("yue_Hans", "上萨华"), ("zh", "上萨瓦省")]),
                        unofficial_name_list: ["Haute-Savoie"].to_vec(),
                    }
                ),
                (
                    "75C",
                    Subdivision{
                        name: "75C",
                        country_alpha2: Alpha2::FR,
                        code: "75C",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.856614), longitude: Some(2.3522219), max_latitude: Some(48.9021449), min_latitude: Some(48.815573), max_longitude: Some(2.4699208), min_longitude: Some(2.224199)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCollectivityWithSpecialStatus,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Parys"), ("am", "ፓሪስ"), ("ar", "باريس"), ("az", "Paris"), ("be", "Парыж"), ("bg", "Париж"), ("bn", "প\u{9cd}য\u{9be}রিস"), ("bs", "Pariz"), ("ca", "París"), ("ccp", "𑄛𑄬𑄢\u{11128}𑄌\u{11134}"), ("ceb", "Paris"), ("cs", "Paříž"), ("cy", "Paris"), ("da", "Paris"), ("de", "Paris"), ("el", "Παρίσι"), ("en", "Paris"), ("es", "París"), ("et", "Pariis"), ("eu", "Paris"), ("fa", "پاریس"), ("fi", "Pariisi"), ("fr", "Paris"), ("ga", "Páras"), ("gl", "París"), ("gu", "પ\u{ac5}રિસ"), ("ha", "Pariis"), ("ha_NE", "Pariis"), ("he", "פריז"), ("hi", "प\u{948}रिस"), ("hr", "Pariz"), ("hu", "Párizs"), ("hy", "Փարիզ"), ("id", "Paris"), ("is", "París"), ("it", "Parigi"), ("ja", "パリ"), ("jv", "Paris"), ("ka", "პარიზი"), ("kk", "Париж"), ("km", "ប\u{17c9}ារ\u{17b8}ស"), ("kn", "ಪ\u{ccd}ಯಾರ\u{cbf}ಸ\u{ccd}"), ("ko", "파리"), ("ky", "Париж"), ("lo", "ປາລ\u{eb5}"), ("lt", "Paryžius"), ("lv", "Parīze"), ("mk", "Париз"), ("ml", "പ\u{d3e}രിസ\u{d4d}"), ("mn", "Парис"), ("mr", "प\u{945}रिस"), ("ms", "Paris"), ("my", "ပါရ\u{102e}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Paris"), ("ne", "प\u{947}रिस"), ("nl", "Parijs"), ("no", "Paris"), ("or", "ପ\u{b4d}ୟ\u{b3e}ର\u{b3f}ସ"), ("pa", "ਪ\u{a48}ਰਿਸ"), ("pl", "Paryż"), ("ps", "پاريس"), ("pt", "Paris"), ("ro", "Paris"), ("ru", "Париж"), ("sd", "پيرس"), ("si", "පැර\u{dd2}ස\u{dca}"), ("sk", "Paríž"), ("sl", "Pariz"), ("so", "Baariis"), ("sq", "Parisi"), ("sr", "Париз"), ("sr_Latn", "Pariz"), ("sv", "Paris"), ("sw", "Paris"), ("ta", "ப\u{bbe}ரிஸ\u{bcd}"), ("te", "ప\u{c3e}ర\u{c3f}స\u{c4d}"), ("th", "ปาร\u{e35}ส"), ("tk", "Pariž"), ("tr", "Paris"), ("uk", "Париж"), ("ur", "پیرس"), ("uz", "Parij"), ("vi", "Paris"), ("yo", "Parisi"), ("yo_BJ", "Parisi"), ("yue", "巴黎"), ("yue_Hans", "巴黎"), ("zh", "巴黎"), ("zu", "IParisi")]),
                        unofficial_name_list: ["Paris"].to_vec(),
                    }
                ),
                (
                    "76",
                    Subdivision{
                        name: "76",
                        country_alpha2: Alpha2::FR,
                        code: "76",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.922992), longitude: Some(1.077483), max_latitude: Some(49.9408489), min_latitude: Some(49.899252), max_longitude: Some(1.1269969), min_longitude: Some(1.0399821)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Seine-Maritime"), ("ar", "السان البحرية"), ("az", "Dənizkənarı Sena"), ("be", "Сена Прыморская"), ("bg", "Сен Маритим"), ("bn", "সেইন-মেরিট\u{9be}ইম"), ("bs", "Seine-Maritime"), ("ca", "Sena Marítim"), ("ccp", "𑄥\u{1112d}𑄚\u{11134}-𑄟𑄬𑄢\u{11128}𑄑\u{1112d}𑄟\u{11134}"), ("ceb", "Seine-Maritime"), ("cs", "Seine-Maritime"), ("cy", "Seine-Maritime"), ("da", "Seine-Maritime"), ("de", "Département Seine-Maritime"), ("el", "Σεν-Μαριτίμ"), ("en", "Seine-Maritime"), ("es", "Sena Marítimo"), ("et", "Seine-Maritime’i departemang"), ("eu", "Seine-Maritime"), ("fa", "سن ماریتیم"), ("fi", "Seine-Maritime"), ("fr", "Seine-Maritime"), ("gl", "Seine-Maritime"), ("gu", "સ\u{ac7}ઇન-મ\u{ac7}રીટાઇમ"), ("he", "סן-מריטים"), ("hi", "सीन-म\u{947}रीटाइम"), ("hu", "Seine-Maritime"), ("hy", "Ծովամերձ Սեն"), ("id", "Seine-Maritime"), ("it", "Senna Marittima"), ("ka", "ზღვისპირა სენა"), ("kk", "Сен-Маритим"), ("kn", "ಸೀನ\u{ccd}-ಮಾರ\u{cbf}ಟೈಮ\u{ccd}"), ("ko", "센마리팀 주"), ("lt", "Pajūrio Sena"), ("lv", "Piejūras Sēna"), ("mr", "सीन-मरितीम"), ("ms", "Seine-Maritime"), ("nb", "Seine-Maritime"), ("nl", "Seine-Maritime"), ("no", "Seine-Maritime"), ("pl", "Seine-Maritime"), ("pt", "Sena Marítimo"), ("ro", "Seine-Maritime"), ("ru", "Приморская Сена"), ("si", "ස\u{dd3}නේ-ම\u{dcf}ර\u{dd2}ටය\u{dd2}ම\u{dca}"), ("sk", "Seine-Maritime"), ("sl", "Seine-Maritime"), ("sr", "Приморска Сена"), ("sr_Latn", "Primorska Sena"), ("sv", "Seine-Maritime"), ("sw", "Seine-Maritime"), ("ta", "சேனே-மரிடைம\u{bcd}"), ("te", "స\u{c46}య\u{c3f}న\u{c3f}-మ\u{c3e}ర\u{c3f}ట\u{c48}మ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแซน-มาร\u{e35}ต\u{e35}ม"), ("tr", "Seine-Maritime"), ("uk", "Приморська Сена"), ("ur", "سین-ماریتیم"), ("vi", "Seine-Maritime"), ("yue", "濱海塞納"), ("yue_Hans", "滨海塞纳"), ("zh", "滨海塞纳省")]),
                        unofficial_name_list: ["Seine-Maritime"].to_vec(),
                    }
                ),
                (
                    "77",
                    Subdivision{
                        name: "77",
                        country_alpha2: Alpha2::FR,
                        code: "77",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.841082), longitude: Some(2.999366), max_latitude: Some(49.1178979), min_latitude: Some(48.1200811), max_longitude: Some(3.5590069), min_longitude: Some(2.3923261)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Seine-et-Marne"), ("ar", "السان ومارن"), ("az", "Sena və Marna"), ("be", "Дэпартамент Сена і Марна"), ("bg", "Сен е Марн"), ("bn", "সেইন-এট-ম\u{9be}র\u{9cd}নে"), ("ca", "Sena i Marne"), ("ccp", "𑄥\u{1112d}𑄚\u{11134}-𑄃𑄬𑄖\u{11134}-𑄟𑄢\u{11134}𑄚𑄬"), ("ceb", "Seine-et-Marne"), ("cs", "Seine-et-Marne"), ("cy", "Seine-et-Marne"), ("da", "Seine-et-Marne"), ("de", "Département Seine-et-Marne"), ("el", "Σεν-ε-Μαρν"), ("en", "Seine-et-Marne"), ("es", "Sena y Marne"), ("et", "Seine-et-Marne"), ("eu", "Seine-et-Marne"), ("fa", "سن-ا-مارن"), ("fi", "Seine-et-Marne"), ("fr", "Seine-et-Marne"), ("gl", "Sena e Marne"), ("gu", "સ\u{ac7}ઇન-એટ-માર\u{acd}ન\u{ac7}"), ("he", "סן ומארן"), ("hi", "सीन-एट-मार\u{94d}न"), ("hu", "Seine-et-Marne"), ("hy", "Սեն-է-Մարն"), ("id", "Seine-et-Marne"), ("it", "Senna e Marna"), ("ka", "სენა და მარნა"), ("kk", "Сена және Марна"), ("kn", "ಸೀನ\u{ccd}-ಎಟ\u{ccd}-ಮರ\u{ccd}ನ\u{cc6}"), ("ko", "센에마른 주"), ("lt", "Sena ir Marna"), ("lv", "Sēna un Marna"), ("mr", "सीन-एत-मार\u{94d}न"), ("ms", "Seine-et-Marne"), ("nb", "Seine-et-Marne"), ("nl", "Seine-et-Marne"), ("no", "Seine-et-Marne"), ("pl", "Sekwana i Marna"), ("pt", "Sena e Marne"), ("ro", "Seine-et-Marne"), ("ru", "Сена и Марна"), ("si", "සෙය\u{dd2}නේ-එට\u{dca}-මර\u{dca}නේ"), ("sk", "Seine-et-Marne"), ("sl", "Seine-et-Marne"), ("sq", "Seine-et-Marne"), ("sr", "Сена и Марна"), ("sr_Latn", "Sena i Marna"), ("sv", "Seine-et-Marne"), ("sw", "Seine-et-Marne"), ("ta", "செய\u{bcd}ன\u{bcd}-எட\u{bcd}-ம\u{bbe}ர\u{bcd}னே"), ("te", "స\u{c3f}య\u{c47}న\u{c4d}-ఎట\u{c4d}-మ\u{c3e}ర\u{c4d}న\u{c46}"), ("th", "จ\u{e31}งหว\u{e31}ดแซเนมาร\u{e4c}น"), ("tr", "Seine-et-Marne"), ("uk", "Сена і Марна"), ("ur", "سین-اے-مارن"), ("vi", "Seine-et-Marne"), ("yue", "塞納-馬恩"), ("yue_Hans", "塞纳-马恩"), ("zh", "塞纳-马恩省")]),
                        unofficial_name_list: ["Seine-et-Marne"].to_vec(),
                    }
                ),
                (
                    "78",
                    Subdivision{
                        name: "78",
                        country_alpha2: Alpha2::FR,
                        code: "78",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.7850939), longitude: Some(1.8256572), max_latitude: Some(49.0854481), min_latitude: Some(48.43855689999999), max_longitude: Some(2.2291269), min_longitude: Some(1.44617)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Yvelines"), ("ar", "الإيفلين"), ("az", "İvelin"), ("be", "Івелін"), ("bg", "Ивлин"), ("bn", "ইয\u{9bc}েভলিন\u{9cd}স"), ("ca", "Yvelines"), ("ccp", "𑄞𑄬𑄣\u{1112d}𑄚𑄬𑄌\u{11134}"), ("ceb", "Yvelines"), ("cs", "Yvelines"), ("cy", "Yvelines"), ("da", "Yvelines"), ("de", "Département Yvelines"), ("el", "Υβελίν"), ("en", "Yvelines"), ("es", "Yvelines"), ("et", "Yvelines"), ("eu", "Yvelines"), ("fa", "ایولین"), ("fi", "Yvelines"), ("fr", "Yvelines"), ("gl", "Yvelines"), ("gu", "યવ\u{ac7}લાઈન\u{acd}સ"), ("he", "איוולין"), ("hi", "य\u{942}व\u{947}लीन\u{94d}स"), ("hu", "Yvelines"), ("hy", "Իվելին"), ("id", "Yvelines"), ("it", "Yvelines"), ("ja", "イヴリーヌ県"), ("ka", "იველინი"), ("kk", "Ивелин"), ("kn", "ಯವ\u{cc6}ಲ\u{cc6}ನ\u{ccd}ಸ\u{ccd}"), ("ko", "이블린 주"), ("lt", "Ivlinas"), ("lv", "Ivelīna"), ("mk", "Ивлин"), ("mr", "इव\u{94d}हलिन"), ("ms", "Yvelines"), ("nb", "Yvelines"), ("nl", "Yvelines"), ("no", "Yvelines"), ("pl", "Yvelines"), ("pt", "Yvelines"), ("ro", "Yvelines"), ("ru", "Ивелин"), ("si", "ය\u{dd4}වෙලය\u{dd2}න\u{dca}ස\u{dca}"), ("sk", "Yvelines"), ("sl", "Yvelines"), ("sq", "Yvelines"), ("sr", "Ивлин"), ("sr_Latn", "Ivlin"), ("sv", "Yvelines"), ("sw", "Yvelines"), ("ta", "யவேலின\u{bcd}ஸ\u{bcd}"), ("te", "య\u{c46}వ\u{c46}ల\u{c48}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e35}ฟว\u{e4c}ล\u{e35}น"), ("tr", "Yvelines"), ("uk", "Івлін"), ("ur", "یولنس"), ("vi", "Yvelines"), ("yue", "伊夫林"), ("yue_Hans", "伊夫林"), ("zh", "伊夫林省")]),
                        unofficial_name_list: ["Yvelines"].to_vec(),
                    }
                ),
                (
                    "79",
                    Subdivision{
                        name: "79",
                        country_alpha2: Alpha2::FR,
                        code: "79",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5926541), longitude: Some(-0.3962844), max_latitude: Some(47.1085489), min_latitude: Some(45.969669), max_longitude: Some(0.220405), min_longitude: Some(-0.9036799999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Deux-Sèvres"), ("ar", "دو سيفر"), ("az", "Dö-Sevr"), ("be", "Дзё-Сеўр"), ("bg", "Дьо Севър"), ("bn", "ড\u{9c1}য\u{9bc}েক\u{9cd}স-সেভ\u{9cd}রে"), ("ca", "Deux-Sèvres"), ("ccp", "𑄓\u{1112a}𑄠𑄬𑄇\u{11134}-𑄥𑄬𑄛\u{11134}𑄢𑄬𑄌\u{11134}"), ("ceb", "Deux-Sèvres"), ("cs", "Deux-Sèvres"), ("cy", "Deux-Sèvres"), ("da", "Deux-Sèvres"), ("de", "Département Deux-Sèvres"), ("el", "Ντε-Σεβρ"), ("en", "Deux-Sèvres"), ("es", "Deux-Sèvres"), ("et", "Deux-Sèvres"), ("eu", "Deux-Sèvres"), ("fa", "دو-سور"), ("fi", "Deux-Sèvres"), ("fr", "Deux-Sèvres"), ("gl", "Deux-Sèvres"), ("gu", "ડ\u{acd}ય\u{ac1}ક\u{acd}સ-સ\u{ac7}વ\u{acd}ર\u{ac7}સ"), ("he", "דה-סוור"), ("hi", "ड\u{94d}य\u{942}-स\u{947}वर\u{947}स"), ("hu", "Deux-Sèvres"), ("hy", "Դյու-Սևր"), ("id", "Deux-Sèvres"), ("it", "Deux-Sèvres"), ("ka", "დე-სევრი"), ("kk", "Де-Севр"), ("kn", "ಡ\u{cbf}ಯಕ\u{ccd}ಸ\u{ccd}-ಸ\u{cc6}ವ\u{cc6}ರ\u{cc6}ಸ\u{ccd}"), ("ko", "되세브르 주"), ("lt", "De Sevras"), ("lv", "Desevra"), ("mr", "द\u{94d}य\u{942}-स\u{947}व\u{94d}र"), ("ms", "Deux-Sèvres"), ("nb", "Deux-Sèvres"), ("nl", "Deux-Sèvres"), ("no", "Deux-Sèvres"), ("pl", "Deux-Sèvres"), ("pt", "Deux-Sèvres"), ("ro", "Deux-Sèvres"), ("ru", "Дё-Севр"), ("si", "ඩ\u{dd2}ය\u{dd4}ක\u{dca}ස\u{dca}- සෙව\u{dca}රෙස\u{dca}"), ("sk", "Deux-Sèvres"), ("sl", "Deux-Sèvres"), ("sq", "Deux-Sèvres"), ("sr", "Де Севр"), ("sr_Latn", "De Sevr"), ("sv", "Deux-Sèvres"), ("sw", "Deux-Sèvres"), ("ta", "டெஸ\u{bcd}-செவரெஸ\u{bcd}"), ("te", "డ\u{c4d}యూక\u{c4d}స\u{c4d}-స\u{c46}వ\u{c4d}ర\u{c46}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเดอ-แซฟวร\u{e4c}"), ("tr", "Deux-Sèvres"), ("uk", "Де-Севр"), ("ur", "ڈو-سیور"), ("vi", "Deux-Sèvres"), ("yue", "德塞夫勒"), ("yue_Hans", "德塞夫勒"), ("zh", "德塞夫勒省")]),
                        unofficial_name_list: ["Deux-Sèvres"].to_vec(),
                    }
                ),
                (
                    "80",
                    Subdivision{
                        name: "80",
                        country_alpha2: Alpha2::FR,
                        code: "80",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.914518), longitude: Some(2.2707095), max_latitude: Some(50.3663219), min_latitude: Some(49.5716231), max_longitude: Some(3.203045), min_longitude: Some(1.379663)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Somme (département)"), ("ar", "سوم"), ("az", "Somma"), ("be", "Дэпартамент Сома"), ("bg", "Сом"), ("bn", "সোম\u{9cd}যে"), ("ca", "Somme"), ("ccp", "𑄥\u{11127}𑄟\u{11133}𑄦𑄬"), ("ceb", "Somme"), ("cs", "Somme"), ("cy", "Somme"), ("da", "Somme"), ("de", "Département Somme"), ("el", "Σομ"), ("en", "Somme"), ("es", "Somme"), ("et", "Somme’i departemang"), ("eu", "Somme"), ("fa", "سم (فرانسه)"), ("fi", "Somme"), ("fr", "Somme"), ("gl", "Somme"), ("gu", "સોમ\u{ac7}"), ("he", "סום"), ("hi", "सोम\u{947}"), ("hu", "Somme"), ("hy", "Սոմ"), ("id", "Somme"), ("it", "Somme"), ("ja", "ソンム県"), ("ka", "სომა"), ("kk", "Сомма"), ("kn", "ಸೊಮ\u{ccd}ಮ\u{cc6}"), ("ko", "솜 주"), ("lt", "Soma"), ("lv", "Somma"), ("mk", "Сома"), ("mr", "सोम, फ\u{94d}रान\u{94d}स"), ("ms", "Somme"), ("nb", "Somme"), ("nl", "Somme"), ("no", "Somme"), ("pl", "Somma"), ("pt", "Somme"), ("ro", "Somme"), ("ru", "Сомма"), ("si", "සොම\u{dca}මේ"), ("sk", "Somme"), ("sl", "Somme"), ("sq", "Somme"), ("sr", "Сома"), ("sr_Latn", "Soma"), ("sv", "Somme"), ("sw", "Somme"), ("ta", "செம\u{bcd}மே"), ("te", "స\u{c4b}మ\u{c46}"), ("th", "จ\u{e31}งหว\u{e31}ดซอม"), ("tr", "Somme"), ("uk", "Сомма"), ("ur", "سوم"), ("vi", "Somme"), ("yue", "索姆"), ("yue_Hans", "索姆"), ("zh", "索姆省")]),
                        unofficial_name_list: ["Somme"].to_vec(),
                    }
                ),
                (
                    "81",
                    Subdivision{
                        name: "81",
                        country_alpha2: Alpha2::FR,
                        code: "81",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.9264401), longitude: Some(1.9881527), max_latitude: Some(44.201493), min_latitude: Some(43.3822819), max_longitude: Some(2.937474), min_longitude: Some(1.535198)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tarn"), ("ar", "تارن"), ("az", "Tarn (departament)"), ("be", "Дэпартамент Тарн"), ("bg", "Тарн"), ("bn", "ট\u{9be}রন"), ("ca", "Tarn"), ("ccp", "𑄑𑄢\u{11134}𑄚\u{11134}"), ("ceb", "Tarn"), ("cs", "Tarn"), ("cy", "Tarn"), ("da", "Tarn"), ("de", "Département Tarn"), ("el", "Ταρν"), ("en", "Tarn"), ("es", "Tarn"), ("et", "Tarni departemang"), ("eu", "Tarn"), ("fa", "تارن"), ("fi", "Tarn"), ("fr", "Tarn"), ("gl", "Tarn"), ("gu", "ટાર\u{acd}ન"), ("he", "טארן"), ("hi", "टार\u{94d}न"), ("hr", "Tarn"), ("hu", "Tarn"), ("hy", "Թարն"), ("id", "Tarn"), ("it", "Tarn"), ("ja", "タルヌ県"), ("ka", "ტარნი"), ("kk", "Тарн"), ("kn", "ಟಾರ\u{ccd}ನ\u{ccd}"), ("ko", "타른 주"), ("lt", "Tarnas"), ("lv", "Tarna"), ("mr", "तार\u{94d}न"), ("ms", "Tarn"), ("nb", "Tarn"), ("nl", "Tarn"), ("no", "Tarn"), ("pl", "Tarn"), ("pt", "Tarn"), ("ro", "Tarn"), ("ru", "Тарн"), ("si", "ට\u{dcf}ර\u{dca}න\u{dca}"), ("sk", "Tarn"), ("sl", "Tarn"), ("sq", "Tarn"), ("sr", "Тарн"), ("sr_Latn", "Tarn"), ("sv", "Tarn"), ("sw", "Tarn"), ("ta", "டர\u{bcd}ன\u{bcd}"), ("te", "ట\u{c3e}ర\u{c4d}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดตาร\u{e4c}น"), ("tr", "Tarn"), ("uk", "Тарн"), ("ur", "تارن"), ("vi", "Tarn"), ("yue", "塔恩"), ("yue_Hans", "塔恩"), ("zh", "塔恩省")]),
                        unofficial_name_list: ["Tarn"].to_vec(),
                    }
                ),
                (
                    "82",
                    Subdivision{
                        name: "82",
                        country_alpha2: Alpha2::FR,
                        code: "82",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.0126679), longitude: Some(1.2891036), max_latitude: Some(44.39392489999999), min_latitude: Some(43.7675901), max_longitude: Some(2.000898), min_longitude: Some(0.7378110999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tarn-et-Garonne"), ("ar", "تارن وغارون"), ("az", "Tarn və Qaronna"), ("be", "Тарн і Гарона"), ("bg", "Тарн е Гарон"), ("bn", "ট\u{9be}র\u{9cd}ন-এট-গ\u{9cd}য\u{9be}রোন"), ("ca", "Tarn i Garona"), ("ccp", "𑄑𑄢\u{11134}𑄚\u{11134}-𑄃𑄬𑄖\u{11134}-𑄉\u{11133}𑄠𑄢\u{1112e}𑄚\u{11133}𑄦\u{11128}"), ("ceb", "Tarn-et-Garonne"), ("cs", "Tarn-et-Garonne"), ("cy", "Tarn-et-Garonne"), ("da", "Tarn-et-Garonne"), ("de", "Département Tarn-et-Garonne"), ("el", "Ταρν-ε-Γκαρόν"), ("en", "Tarn-et-Garonne"), ("es", "Tarn y Garona"), ("et", "Tarn-et-Garonne"), ("eu", "Tarn eta Garona"), ("fa", "تارن-ا-گارون"), ("fi", "Tarn-et-Garonne"), ("fr", "Tarn-et-Garonne"), ("gl", "Tarn-et-Garonne"), ("gu", "તાર\u{acd}ન-એટ-ગરોન"), ("he", "טארן וגארון"), ("hi", "टार\u{94d}न-एट-ग\u{948}र\u{947}न"), ("hu", "Tarn-et-Garonne"), ("hy", "Թարն-է-Գարոն"), ("id", "Tarn-et-Garonne"), ("it", "Tarn e Garonna"), ("ka", "ტარნი და გარონა"), ("kk", "Тарн және Гаронна"), ("kn", "ಟಾರ\u{ccd}ನ\u{ccd}-ಇಟ\u{ccd}-ಗ\u{ccd}ಯಾರೋನ\u{cc6}"), ("ko", "타른에가론 주"), ("lt", "Tarnas ir Garona"), ("lv", "Tarna un Garonna"), ("mr", "तार\u{94d}न-एत-गारोन"), ("ms", "Tarn-et-Garonne"), ("nb", "Tarn-et-Garonne"), ("nl", "Tarn-et-Garonne"), ("no", "Tarn-et-Garonne"), ("pl", "Tarn i Garonna"), ("pt", "Tarn-et-Garonne"), ("ro", "Tarn-et-Garonne"), ("ru", "Тарн и Гаронна"), ("si", "ට\u{dcf}ර\u{dca}න\u{dca}-එට\u{dca} -ගරෝනේ"), ("sk", "Tarn-et-Garonne"), ("sl", "Tarn-et-Garonne"), ("sq", "Tarn-et-Garonne"), ("sr", "Тарн и Гарона"), ("sr_Latn", "Tarn i Garona"), ("sv", "Tarn-et-Garonne"), ("sw", "Tarn-et-Garonne"), ("ta", "டர\u{bcd}ன\u{bcd} -எட\u{bcd} -க\u{bbe}ரோணனே"), ("te", "ట\u{c3e}ర\u{c4d}న\u{c4d}-ఎట\u{c4d}-గ\u{c3e}ర\u{c4b}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดตาร\u{e4c}เนการอน"), ("tr", "Tarn-et-Garonne"), ("uk", "Тарн і Гаронна"), ("ur", "تارن-اے-گارون"), ("vi", "Tarn-et-Garonne"), ("yue", "塔恩-加龍"), ("yue_Hans", "塔恩-加龙"), ("zh", "塔恩-加龙省")]),
                        unofficial_name_list: ["Tarn-et-Garonne"].to_vec(),
                    }
                ),
                (
                    "83",
                    Subdivision{
                        name: "83",
                        country_alpha2: Alpha2::FR,
                        code: "83",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.46764599999999), longitude: Some(6.2375947), max_latitude: Some(43.808881), min_latitude: Some(42.98199810000001), max_longitude: Some(6.933446), min_longitude: Some(5.6559)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Var"), ("ar", "فار"), ("az", "Var (departament)"), ("be", "Дэпартамент Вар"), ("bg", "Вар"), ("bn", "ভ\u{9be}র"), ("ca", "Var"), ("ccp", "𑄞𑄢\u{11134}"), ("ceb", "Var"), ("cs", "Var"), ("cy", "Var"), ("da", "Var"), ("de", "Département Var"), ("el", "Βαρ"), ("en", "Var"), ("es", "Var"), ("et", "Var’i departemang"), ("eu", "Var"), ("fa", "ور"), ("fi", "Var"), ("fr", "Var"), ("gl", "Var"), ("gu", "વાર"), ("hi", "वार"), ("hu", "Var"), ("hy", "Վար"), ("id", "Var"), ("it", "Varo"), ("ja", "ヴァール県"), ("ka", "ვარი"), ("kk", "Вар"), ("kn", "ವಾರ\u{ccd}"), ("ko", "바르 주"), ("lt", "Varas"), ("lv", "Vāra"), ("mr", "व\u{94d}हार"), ("ms", "Var"), ("nb", "Var"), ("nl", "Var"), ("no", "Var"), ("pl", "Var"), ("pt", "Var"), ("ro", "Var"), ("ru", "Вар"), ("si", "ව\u{dcf}ර\u{dca}"), ("sk", "Var"), ("sl", "Var"), ("sq", "Var"), ("sr", "Вар"), ("sr_Latn", "Var"), ("sv", "Var"), ("sw", "Var"), ("ta", "வ\u{bbe}ர\u{bcd}"), ("te", "వ\u{c3e}ర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดวาร\u{e4c}"), ("tr", "Var"), ("uk", "Вар"), ("ur", "وار"), ("vi", "Var"), ("yue", "華爾"), ("yue_Hans", "华尔"), ("zh", "瓦尔省")]),
                        unofficial_name_list: ["Var"].to_vec(),
                    }
                ),
                (
                    "84",
                    Subdivision{
                        name: "84",
                        country_alpha2: Alpha2::FR,
                        code: "84",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.0565054), longitude: Some(5.1432068), max_latitude: Some(44.4315659), min_latitude: Some(43.658718), max_longitude: Some(5.757334999999999), min_longitude: Some(4.649082)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Vaucluse"), ("ar", "فوكلوز"), ("az", "Voklüz (departament)"), ("be", "Ваклюз"), ("bg", "Воклюз"), ("bn", "ভক\u{9cd}ল\u{9c1}স"), ("ca", "Valclusa"), ("ccp", "𑄞𑄅\u{1112a}𑄇\u{11133}𑄣\u{1112a}𑄌\u{11134}"), ("ceb", "Vaucluse"), ("cs", "Vaucluse"), ("cy", "Vaucluse"), ("da", "Vaucluse"), ("de", "Département Vaucluse"), ("el", "Βωκλύζ"), ("en", "Vaucluse"), ("es", "Vaucluse"), ("et", "Vaucluse’i departemang"), ("eu", "Vaucluse"), ("fa", "وکلوز"), ("fi", "Vaucluse"), ("fr", "Vaucluse"), ("gl", "Vaucluse"), ("gu", "વૌક\u{acd}લોઝ"), ("he", "ווקלוז"), ("hi", "वॉक\u{94d}ल\u{942}स"), ("hu", "Vaucluse"), ("hy", "Վոքլյուզ"), ("id", "Vaucluse"), ("it", "Vaucluse"), ("ja", "ヴォクリューズ県"), ("ka", "ვოკლიუზი"), ("kk", "Воклюз"), ("kn", "ವಾಕ\u{ccd}ಲುಸ\u{ccd}"), ("ko", "보클뤼즈 주"), ("lt", "Vokliūzas"), ("lv", "Voklīza"), ("mr", "व\u{94d}हॉक\u{94d}ल\u{94d}य\u{941}झ"), ("ms", "Vaucluse"), ("nb", "Vaucluse"), ("nl", "Vaucluse"), ("no", "Vaucluse"), ("pl", "Vaucluse"), ("pt", "Vaucluse"), ("ro", "Vaucluse"), ("ru", "Воклюз"), ("si", "වව\u{dd4}ක\u{dca}ල\u{dd4}සේ"), ("sk", "Vaucluse"), ("sl", "Vaucluse"), ("sq", "Vaucluse"), ("sr", "Воклиз"), ("sr_Latn", "Vokliz"), ("sv", "Vaucluse"), ("sw", "Vaucluse"), ("ta", "வ\u{bbe}க\u{bcd}கிலுசே"), ("te", "వ\u{c3e}క\u{c4d}లూజ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโวกล\u{e39}ซ"), ("tr", "Vaucluse"), ("uk", "Воклюз"), ("ur", "وکلوز"), ("vi", "Vaucluse"), ("yue", "禾克呂茲"), ("yue_Hans", "禾克吕兹"), ("zh", "沃克吕兹省")]),
                        unofficial_name_list: ["Vaucluse"].to_vec(),
                    }
                ),
                (
                    "85",
                    Subdivision{
                        name: "85",
                        country_alpha2: Alpha2::FR,
                        code: "85",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.6613966), longitude: Some(-1.4482662), max_latitude: Some(47.0850081), min_latitude: Some(46.26653899999999), max_longitude: Some(-0.538134), min_longitude: Some(-2.3998896)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Vendée"), ("ar", "فونديه"), ("az", "Vandeya"), ("be", "Вандэя"), ("bg", "Вандея"), ("bn", "ভেন\u{9cd}ডি"), ("ca", "Vendée"), ("ccp", "𑄞𑄬𑄚\u{11134}𑄘\u{11133}𑄦\u{11128}"), ("ceb", "Vendée"), ("cs", "Vendée"), ("cy", "Vendée"), ("da", "Vendée"), ("de", "Département Vendée"), ("el", "Βαντέ"), ("en", "Vendée"), ("es", "Vandea"), ("et", "Vendée"), ("eu", "Vendée"), ("fa", "وانده"), ("fi", "Vendée"), ("fr", "Vendée"), ("gl", "Vendée"), ("gu", "વ\u{ac7}ન\u{acd}ડી"), ("he", "ונדה"), ("hi", "व\u{947}\u{902}डी"), ("hr", "Vendée"), ("hu", "Vendée"), ("hy", "Վանդե"), ("id", "Vendée"), ("is", "Vendée"), ("it", "Vandea"), ("ja", "ヴァンデ県"), ("ka", "ვანდეია"), ("kk", "Вандея"), ("kn", "ವ\u{cc6}ಂಡ\u{cbf}"), ("ko", "방데 주"), ("lt", "Vandėja"), ("lv", "Vandeja"), ("mk", "Вандеја (департман)"), ("mr", "वा\u{902}द\u{947}"), ("ms", "Vendée"), ("nb", "Vendée"), ("nl", "Vendée"), ("no", "Vendée"), ("pl", "Wandea"), ("pt", "Vendeia"), ("ro", "Vendée"), ("ru", "Вандея"), ("si", "වෙන\u{dca}ඩ\u{dd2}"), ("sk", "Vendée"), ("sl", "Vendée"), ("sq", "Vendée"), ("sr", "Вандеја"), ("sr_Latn", "Vandeja"), ("sv", "Vendée"), ("sw", "Vendée"), ("ta", "வேண\u{bcd}ட\u{bc0}"), ("te", "వ\u{c46}ండ\u{c40}"), ("th", "จ\u{e31}งหว\u{e31}ดว\u{e47}องเด"), ("tr", "Vendée"), ("uk", "Вандея"), ("ur", "واندے"), ("vi", "Vendée"), ("yue", "旺代"), ("yue_Hans", "旺代"), ("zh", "旺代省")]),
                        unofficial_name_list: ["Vendée"].to_vec(),
                    }
                ),
                (
                    "86",
                    Subdivision{
                        name: "86",
                        country_alpha2: Alpha2::FR,
                        code: "86",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.525587), longitude: Some(4.874339), max_latitude: Some(45.557463), min_latitude: Some(45.4867009), max_longitude: Some(4.923409899999999), min_longitude: Some(4.837251999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Vienne"), ("ar", "فيين"), ("az", "Vyenna"), ("bg", "Виен"), ("bn", "ভিএন\u{9be}"), ("ca", "Viena"), ("ccp", "𑄞\u{1112d}𑄠𑄬𑄚\u{11133}𑄦\u{11128}"), ("ceb", "Vienne"), ("cs", "Vienne"), ("cy", "Vienne"), ("da", "Vienne"), ("de", "Département Vienne"), ("el", "Βιέν"), ("en", "Vienne"), ("es", "Vienne"), ("et", "Vienne’i departemang"), ("eu", "Vienne"), ("fa", "وین"), ("fi", "Vienne"), ("fr", "Vienne"), ("gl", "Vienne"), ("gu", "વિય\u{ac7}ન"), ("he", "ויין"), ("hi", "वियन\u{947}"), ("hr", "Vienne"), ("hu", "Vienne"), ("hy", "Վյեն"), ("id", "Vienne"), ("it", "Vienne"), ("ja", "ヴィエンヌ県"), ("jv", "Vienne"), ("ka", "ვიენა"), ("kk", "Вьенна"), ("kn", "ವೈನ\u{ccd}ನ\u{cc6}"), ("ko", "비엔 주"), ("lt", "Vjenas"), ("lv", "Vjenna"), ("mk", "Вјена"), ("mr", "व\u{94d}हिय\u{947}न"), ("ms", "Vienne"), ("nb", "Vienne"), ("nl", "Vienne"), ("no", "Vienne"), ("pl", "Vienne"), ("pt", "Vienne"), ("ro", "Vienne"), ("ru", "Вьенна"), ("si", "ව\u{dd2}යෙන\u{dca}නේ"), ("sk", "Vienne"), ("sl", "Vienne"), ("sq", "Vienne"), ("sr", "Вијен"), ("sr_Latn", "Vijen"), ("sv", "Vienne"), ("sw", "Vienne"), ("ta", "வியென\u{bcd}னே"), ("te", "వ\u{c3f}య\u{c46}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเว\u{e35}ยน"), ("tr", "Vienne"), ("uk", "Вʼєнна"), ("ur", "ویئن"), ("vi", "Vienne"), ("yue", "維埃納"), ("yue_Hans", "维埃纳"), ("zh", "维埃纳省")]),
                        unofficial_name_list: ["Vienne"].to_vec(),
                    }
                ),
                (
                    "87",
                    Subdivision{
                        name: "87",
                        country_alpha2: Alpha2::FR,
                        code: "87",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.7435173), longitude: Some(1.4025484), max_latitude: Some(46.40158599999999), min_latitude: Some(45.43663), max_longitude: Some(1.9110789), min_longitude: Some(0.62925)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Haute-Vienne"), ("ar", "فيين العليا"), ("az", "Yuxarı Vyenna"), ("bg", "От Виен"), ("bn", "হ\u{9be}উতে-ভিয\u{9bc}েনে"), ("ca", "Alta Viena"), ("ccp", "𑄦𑄅\u{1112a}𑄖\u{11134}-𑄞\u{1112d}𑄠𑄬𑄚\u{11133}𑄦\u{11128}"), ("ceb", "Haute-Vienne"), ("cs", "Haute-Vienne"), ("cy", "Haute-Vienne"), ("da", "Haute-Vienne"), ("de", "Département Haute-Vienne"), ("el", "Ωτ-Βιέν"), ("en", "Haute-Vienne"), ("es", "Alto Vienne"), ("et", "Haute-Vienne"), ("eu", "Haute-Vienne"), ("fa", "اوت-وین"), ("fi", "Haute-Vienne"), ("fr", "Haute-Vienne"), ("gl", "Alto Vienne"), ("gu", "હૌટ-વિય\u{ac7}ન"), ("he", "ויין עילית"), ("hi", "ओट-वियन\u{947}"), ("hu", "Haute-Vienne"), ("hy", "Օտ-Վյեն"), ("id", "Haute-Vienne"), ("it", "Alta Vienne"), ("ka", "ზემო ვიენა"), ("kk", "Жоғарғы Вьенна"), ("kn", "ಹಾಟ\u{cc6}-ವ\u{cbf}ಯ\u{cc6}ನ\u{ccd}"), ("ko", "오트비엔 주"), ("lt", "Aukštutinis Vjenas"), ("lv", "Augšvjenna"), ("mk", "Горна Вјена"), ("mr", "ऑत-व\u{94d}हिय\u{947}न"), ("ms", "Haute-Vienne"), ("nb", "Haute-Vienne"), ("nl", "Haute-Vienne"), ("no", "Haute-Vienne"), ("pl", "Haute-Vienne"), ("pt", "Haute-Vienne"), ("ro", "Haute-Vienne"), ("ru", "Верхняя Вьенна"), ("si", "හෞටේ -ව\u{dd2}එනේ"), ("sk", "Haute-Vienne"), ("sl", "Haute-Vienne"), ("sq", "Haute-Vienne"), ("sr", "Горњи Вијен"), ("sr_Latn", "Gornji Vijen"), ("sv", "Haute-Vienne"), ("sw", "Haute-Vienne"), ("ta", "ஹூட\u{bcd} -விஏன\u{bcd}னே"), ("te", "హ\u{c3e}ట\u{c4d}-వ\u{c3f}య\u{c46}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโอต-เว\u{e35}ยน"), ("tr", "Haute-Vienne"), ("uk", "Верхня Вʼєнна"), ("ur", "بالائی-ویئن"), ("vi", "Haute-Vienne"), ("yue", "上維埃納"), ("yue_Hans", "上维埃纳"), ("zh", "上维埃纳省")]),
                        unofficial_name_list: ["Haute-Vienne"].to_vec(),
                    }
                ),
                (
                    "88",
                    Subdivision{
                        name: "88",
                        country_alpha2: Alpha2::FR,
                        code: "88",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.1446427), longitude: Some(6.3355935), max_latitude: Some(48.513663), min_latitude: Some(47.8132981), max_longitude: Some(7.198364), min_longitude: Some(5.3936269)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Vosges"), ("ar", "فوج"), ("az", "Vogezlər"), ("be", "Дэпартамент Вагезы"), ("bg", "Вож"), ("bn", "ভজগেস"), ("ca", "Vosges"), ("ccp", "𑄞\u{1112e}𑄌\u{11134}𑄉𑄬𑄌\u{11134}"), ("ceb", "Vosges"), ("cs", "Vosges"), ("cy", "Vosges"), ("da", "Vosges"), ("de", "Département Vosges"), ("el", "Βοζ"), ("en", "Vosges"), ("es", "Vosgos"), ("et", "Vosges’i departemang"), ("eu", "Vosges"), ("fa", "ووژ"), ("fi", "Vosges"), ("fr", "Vosges"), ("gl", "Vosgos"), ("gu", "વોસ\u{acd}ગ\u{ac7}સ"), ("he", "ווז׳"), ("hi", "वोशस"), ("hu", "Vosges"), ("hy", "Վոգեզներ"), ("id", "Vosges"), ("it", "Vosgi"), ("ja", "ヴォージュ県"), ("ka", "ვოგეზი"), ("kk", "Вогез"), ("kn", "ವೊಸ\u{ccd}ಜ\u{cc6}ಸ\u{ccd}"), ("ko", "보주 주"), ("lt", "Vogėzai"), ("lv", "Vogēzi"), ("mk", "Вогези"), ("mr", "व\u{94d}हॉझ"), ("ms", "Vosges"), ("nb", "Vosges"), ("nl", "Vosges"), ("no", "Vosges"), ("pl", "Wogezy"), ("pt", "Vosges"), ("ro", "Vosges"), ("ru", "Вогезы"), ("si", ", වොස\u{dca}ගෙස\u{dca}"), ("sk", "Vosges"), ("sl", "Vosges"), ("sq", "Vosgos"), ("sr", "Вогези"), ("sr_Latn", "Vogezi"), ("sv", "Vosges"), ("sw", "Vosges"), ("ta", "வோஸ\u{bcd}ஜ\u{bcd}ஸ\u{bcd}"), ("te", "వ\u{c4b}స\u{c4d}గ\u{c46}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโวฌ"), ("tr", "Vosges"), ("uk", "Вогези"), ("ur", "ووژ"), ("vi", "Vosges"), ("yue", "孚日"), ("yue_Hans", "孚日"), ("zh", "孚日省")]),
                        unofficial_name_list: ["Vosges"].to_vec(),
                    }
                ),
                (
                    "89",
                    Subdivision{
                        name: "89",
                        country_alpha2: Alpha2::FR,
                        code: "89",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.8652728), longitude: Some(3.6079823), max_latitude: Some(48.400061), min_latitude: Some(47.310363), max_longitude: Some(4.340295999999999), min_longitude: Some(2.848432)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Yonne"), ("ar", "يون"), ("az", "Yonna"), ("be", "Дэпартамент Іона"), ("bg", "Йон"), ("bn", "ইয\u{9bc}োন\u{9cd}যে"), ("ca", "Yonne"), ("ccp", "𑄃\u{11128}𑄠\u{1112e}𑄚\u{11133}𑄦\u{11128}"), ("ceb", "Yonne"), ("cs", "Yonne"), ("cy", "Yonne"), ("da", "Yonne"), ("de", "Département Yonne"), ("el", "Ιόν"), ("en", "Yonne"), ("es", "Yonne"), ("et", "Yonne’i departemang"), ("eu", "Yonne"), ("fa", "یون"), ("fi", "Yonne"), ("fr", "Yonne"), ("gl", "Yonne, Francia"), ("gu", "યોન"), ("he", "יון"), ("hi", "योन\u{947}"), ("hu", "Yonne"), ("hy", "Յոն"), ("id", "Yonne"), ("it", "Yonne"), ("ja", "ヨンヌ県"), ("ka", "იონა"), ("kk", "Йонна"), ("kn", "ಯೋನ\u{cc6}"), ("ko", "욘 주"), ("lt", "Jonas"), ("lv", "Jona"), ("mr", "योन"), ("ms", "Yonne"), ("nb", "Yonne"), ("nl", "Yonne"), ("no", "Yonne"), ("pl", "Yonne"), ("pt", "Yonne"), ("ro", "Yonne"), ("ru", "Йонна"), ("si", "යෝන\u{dca}නේ"), ("sk", "Yonne"), ("sl", "Yonne"), ("sq", "Yonne"), ("sr", "Јон"), ("sr_Latn", "Jon"), ("sv", "Yonne"), ("sw", "Yonne"), ("ta", "யொன\u{bcd}னே"), ("te", "య\u{c4b}న\u{c4d}న\u{c46}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e35}ยอน"), ("tr", "Yonne"), ("uk", "Йонна"), ("ur", "یوننے"), ("vi", "Yonne"), ("yue", "約訥"), ("yue_Hans", "约讷"), ("zh", "约讷省")]),
                        unofficial_name_list: ["Yonne"].to_vec(),
                    }
                ),
                (
                    "90",
                    Subdivision{
                        name: "90",
                        country_alpha2: Alpha2::FR,
                        code: "90",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.59465729999999), longitude: Some(6.920771599999999), max_latitude: Some(47.82511299999999), min_latitude: Some(47.433383), max_longitude: Some(7.143381), min_longitude: Some(6.756256)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Territoire de Belfort"), ("ar", "إقليم بلفور"), ("az", "Belfor ərazisi"), ("be", "Тэрыторыя Бельфор"), ("bg", "Теритоар дьо Белфор"), ("bn", "টেরিটোরি ডি বেলফোর\u{9cd}ট"), ("ca", "Territori de Belfort"), ("ccp", "𑄑𑄬𑄢\u{11133}𑄦\u{11128}𑄑\u{11130}𑄢\u{11134} 𑄓𑄬 𑄝𑄬𑄣\u{11134}𑄜\u{1112e}𑄢\u{11134}𑄑\u{11134}"), ("ceb", "Territoire de Belfort"), ("cs", "Territoire de Belfort"), ("cy", "Territoire de Belfort"), ("da", "Territoire de Belfort"), ("de", "Territoire de Belfort"), ("el", "Έδαφος του Μπελφόρ"), ("en", "Territoire de Belfort"), ("es", "Territorio de Belfort"), ("et", "Belfort’i departemang"), ("eu", "Belfort Herrialdea"), ("fa", "تریتوآر دو بلفور"), ("fi", "Territoire de Belfort"), ("fr", "Territoire de Belfort"), ("gl", "Territorio de Belfort"), ("gu", "ટ\u{ac7}રિટોઇરા દ\u{ac7} બ\u{ac7}લ\u{acd}ફોર\u{acd}ટ"), ("he", "טריטוריית בלפור"), ("hi", "ट\u{947}र\u{947}टोइर डी ब\u{947}ल\u{94d}फोर\u{94d}ट"), ("hu", "Territoire de Belfort"), ("hy", "Բելֆորի տարածք"), ("id", "Territoire de Belfort"), ("it", "Territorio di Belfort"), ("ja", "テリトワール・ド・ベルフォール県"), ("ka", "ბელფორის ტერიტორია"), ("kk", "Территория-де-Бельфор"), ("kn", "ಟ\u{cc6}ರ\u{cbf}ಟೊಯ\u{cbf}ರ\u{ccd} ಡ\u{cbf} ಬ\u{cc6}ಲ\u{ccd}ಫೋರ\u{ccd}ಟ\u{ccd}"), ("ko", "테리투아르드벨포르 주"), ("lt", "Belforo teritorija"), ("lv", "Belfora"), ("mk", "Територија Белфор"), ("mr", "त\u{947}रितॉर द\u{947} ब\u{947}ल\u{94d}फॉर"), ("ms", "Territoire de Belfort"), ("nb", "Territoire de Belfort"), ("nl", "Territoire de Belfort"), ("no", "Territoire de Belfort"), ("pl", "Territoire-de-Belfort"), ("pt", "Território de Belfort"), ("ro", "Territoire de Belfort"), ("ru", "Территория Бельфор"), ("si", "බෙල\u{dca}ෆෝර\u{dca}ට\u{dca} පළ\u{dcf}ත"), ("sk", "Territoire de Belfort"), ("sl", "Territoire de Belfort"), ("sq", "Territoire de Belfort"), ("sr", "Територија Белфор"), ("sr_Latn", "Teritorija Belfor"), ("sv", "Territoire de Belfort"), ("sw", "Territoire de Belfort"), ("ta", "டெரரிடோனிரே டி பெலஃபோர\u{bcd}ட\u{bcd}"), ("te", "ట\u{c46}ర\u{c3f}ట\u{c4b}య\u{c3f}ర\u{c4d} డ\u{c3f} బ\u{c46}ల\u{c4d}ఫ\u{c4b}ర\u{c4d}ట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแตร\u{e35}ต\u{e31}วร\u{e4c}เดอแบลฟอร\u{e4c}"), ("tr", "Territoire de Belfort"), ("uk", "Територія Бельфор"), ("ur", "تیریتوار دو بیلفور"), ("vi", "Lãnh thổ Belfort"), ("yue", "貝爾福地區"), ("yue_Hans", "贝尔福地区"), ("zh", "贝尔福地区")]),
                        unofficial_name_list: ["Territoire de Belfort"].to_vec(),
                    }
                ),
                (
                    "91",
                    Subdivision{
                        name: "91",
                        country_alpha2: Alpha2::FR,
                        code: "91",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.4585698), longitude: Some(2.1569416), max_latitude: Some(48.7761319), min_latitude: Some(48.28455599999999), max_longitude: Some(2.5856331), min_longitude: Some(1.9145131)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Essonne"), ("ar", "إيسون"), ("az", "Eson"), ("be", "Дэпартамент Эсон"), ("bg", "Есон"), ("bn", "এসোনে"), ("ca", "Essonne"), ("ccp", "𑄃\u{11128}𑄥\u{11133}𑄦\u{1112e}𑄚\u{11133}𑄦\u{11128}"), ("ceb", "Essonne"), ("cs", "Essonne"), ("cy", "Essonne"), ("da", "Essonne"), ("de", "Département Essonne"), ("el", "Εσσόν"), ("en", "Essonne"), ("es", "Essonne"), ("et", "Essonne’i departemang"), ("eu", "Essonne"), ("fa", "اسون"), ("fi", "Essonne"), ("fr", "Essonne"), ("gl", "Essonne"), ("gu", "એસોન"), ("he", "אסון"), ("hi", "एसोन\u{947}"), ("hu", "Essonne"), ("hy", "Էսոն"), ("id", "Essonne"), ("it", "Essonne"), ("ja", "エソンヌ県"), ("ka", "ესონა"), ("kk", "Эсон"), ("kn", "ಎಸ\u{ccd}ಸೊನ\u{cc6}"), ("ko", "에손 주"), ("lt", "Esonas"), ("lv", "Esona"), ("mk", "Есона"), ("mr", "एसोन"), ("ms", "Essonne"), ("nb", "Essonne"), ("ne", "एस\u{94d}सोन\u{94d}म\u{947}"), ("nl", "Essonne"), ("no", "Essonne"), ("pl", "Essonne"), ("pt", "Essonne"), ("ro", "Essonne"), ("ru", "Эсон"), ("si", "එස\u{dca}සොන\u{dca}නේ"), ("sk", "Essonne"), ("sl", "Essonne"), ("sq", "Essonne"), ("sr", "Есон"), ("sr_Latn", "Eson"), ("sv", "Essonne"), ("sw", "Essonne"), ("ta", "எஸ\u{bcd}ஸோன\u{bcd}னே"), ("te", "ఎస\u{c4d}సన\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเอซอน"), ("tr", "Essonne"), ("uk", "Ессонн"), ("ur", "ایسون"), ("vi", "Essonne"), ("yue", "埃松"), ("yue_Hans", "埃松"), ("zh", "埃松省")]),
                        unofficial_name_list: ["Essonne"].to_vec(),
                    }
                ),
                (
                    "92",
                    Subdivision{
                        name: "92",
                        country_alpha2: Alpha2::FR,
                        code: "92",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.828508), longitude: Some(2.2188068), max_latitude: Some(48.9509619), min_latitude: Some(48.729351), max_longitude: Some(2.336941), min_longitude: Some(2.145702)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hauts-de-Seine"), ("ar", "هوت دو سين"), ("az", "O-de-Sen"), ("be", "О-дэ-Сен"), ("bg", "О дьо Сен"), ("bn", "হোতস-দে-সেইন"), ("ca", "Alts del Sena"), ("ccp", "𑄦𑄅\u{1112a}𑄖\u{11134}-𑄓𑄬-𑄥\u{1112d}𑄚\u{11134}"), ("ceb", "Hauts-de-Seine"), ("cs", "Hauts-de-Seine"), ("cy", "Hauts-de-Seine"), ("da", "Hauts-de-Seine"), ("de", "Département Hauts-de-Seine"), ("el", "Ω-ντε-Σεν"), ("en", "Hauts-de-Seine"), ("es", "Altos del Sena"), ("et", "Hauts-de-Seine"), ("eu", "Hauts-de-Seine"), ("fa", "او-دو-سن"), ("fi", "Hauts-de-Seine"), ("fr", "Hauts-de-Seine"), ("gl", "Hauts-de-Seine"), ("gu", "હોટ\u{acd}સ-દ\u{ac7}-સ\u{ac7}ઇન"), ("he", "רמות הסן"), ("hi", "ओट\u{94d}स-डी-सीन"), ("hu", "Hauts-de-Seine"), ("hy", "Օ-դը-Սեն"), ("id", "Hauts-de-Seine"), ("it", "Hauts-de-Seine"), ("jv", "Hauts-de-Seine"), ("ka", "ო-დე-სენა"), ("kk", "О-де-Сен"), ("kn", "ಹ\u{ccc}ಟ\u{ccd}ಸ\u{ccd}-ಡ\u{cc6}-ಸೀನ\u{ccd}"), ("ko", "오드센 주"), ("lt", "Aukštutinė Sena"), ("lv", "Odesēna"), ("mk", "Сенски Висови"), ("mr", "ऑत-द\u{947}-सीन"), ("ms", "Hauts-de-Seine"), ("nb", "Hauts-de-Seine"), ("nl", "Hauts-de-Seine"), ("no", "Hauts-de-Seine"), ("pl", "Hauts-de-Seine"), ("pt", "Altos do Sena"), ("ro", "Hauts-de-Seine"), ("ru", "О-де-Сен"), ("si", "හෞට\u{dca}ස\u{dca}-ඩ\u{dd2} සෙය\u{dd2}නේ"), ("sk", "Hauts-de-Seine"), ("sl", "Hauts-de-Seine"), ("sq", "Hauts-de-Seine"), ("sr", "Сенски висови"), ("sr_Latn", "Senski visovi"), ("sv", "Hauts-de-Seine"), ("sw", "Hauts-de-Seine"), ("ta", "ஹூட\u{bcd}ஸ\u{bcd} -டி -ஷைனி"), ("te", "హ\u{c3e}ట\u{c4d}స\u{c4d}-డ\u{c3f}-స\u{c3f}య\u{c46}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโอดแซน"), ("tr", "Hauts-de-Seine"), ("uk", "О-де-Сен"), ("ur", "بلند-دو-سین"), ("vi", "Hauts-de-Seine"), ("yue", "上塞納省"), ("yue_Hans", "上塞纳省"), ("zh", "上塞纳省")]),
                        unofficial_name_list: ["Hauts-de-Seine"].to_vec(),
                    }
                ),
                (
                    "93",
                    Subdivision{
                        name: "93",
                        country_alpha2: Alpha2::FR,
                        code: "93",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.9137455), longitude: Some(2.4845729), max_latitude: Some(49.012329), min_latitude: Some(48.807248), max_longitude: Some(2.6032919), min_longitude: Some(2.2883109)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Seine-Saint-Denis"), ("ar", "سين سان دوني"), ("az", "Sen-Sen-Deni"), ("be", "Сен-Сен-Дэні"), ("bg", "Сен Сен Дьони"), ("bn", "সেইন-সেন\u{9cd}ট-ডেনিস"), ("ca", "Sena Saint-Denis"), ("ccp", "𑄥\u{1112d}𑄚\u{11134}-𑄥𑄬𑄚\u{11134}-𑄓𑄬𑄚\u{11128}𑄌\u{11134}"), ("ceb", "Seine-Saint-Denis"), ("cs", "Seine-Saint-Denis"), ("cy", "Seine-Saint-Denis"), ("da", "Seine-Saint-Denis"), ("de", "Département Seine-Saint-Denis"), ("el", "Σεν-Σαιν-Ντενί"), ("en", "Seine-Saint-Denis"), ("es", "Sena-Saint Denis"), ("et", "Seine-Saint-Denis"), ("eu", "Seine-Saint-Denis"), ("fa", "سن-سن-دونی"), ("fi", "Seine-Saint-Denis"), ("fr", "Seine-Saint-Denis"), ("gl", "Sena-San Denis"), ("gu", "સ\u{ac7}ઇન-સ\u{ac7}ન\u{acd}ટ-ડ\u{ac7}નિસ"), ("he", "סן-סן-דני"), ("hi", "सीन-स\u{947}\u{902}ट-ड\u{947}निस"), ("hu", "Seine-Saint-Denis"), ("hy", "Սեն-Սեն-Դենի"), ("id", "Seine-Saint-Denis"), ("it", "Senna-Saint-Denis"), ("ka", "სენა-სენ-დენი"), ("kk", "Сена-Сен-Дени"), ("kn", "ಸೈನ\u{ccd}-ಸೇಂಟ\u{ccd}-ಡ\u{cc6}ನ\u{cbf}ಸ\u{ccd}"), ("ko", "센생드니 주"), ("lt", "Sena-Sen Deni"), ("lv", "Sēna-Sendenī"), ("mk", "Сена-Сен Дени"), ("mr", "सीन-स\u{947}\u{902}त-द\u{947}निस"), ("ms", "Seine-Saint-Denis"), ("nb", "Seine-Saint-Denis"), ("nl", "Seine-Saint-Denis"), ("no", "Seine-Saint-Denis"), ("pl", "Seine-Saint-Denis"), ("pt", "Seine-Saint-Denis"), ("ro", "Seine-Saint-Denis"), ("ru", "Сен-Сен-Дени"), ("si", "සෙය\u{dd2}නේ-ශ\u{dcf}න\u{dca}ත-ඩෙන\u{dd2}ස\u{dca}"), ("sk", "Seine-Saint-Denis"), ("sl", "Seine-Saint-Denis"), ("sq", "Seine-Saint-Denis"), ("sr", "Сена-Сен Дени"), ("sr_Latn", "Sena-Sen Deni"), ("sv", "Seine-Saint-Denis"), ("sw", "Seine-Saint-Denis"), ("ta", "ஷைனி -செயின\u{bcd}ட\u{bcd} -டெனிஸ\u{bcd}"), ("te", "స\u{c40}న\u{c4d}-స\u{c46}య\u{c3f}ంట\u{c4d}-డ\u{c46}న\u{c3f}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแซน-แซ\u{e47}ง-เดอน\u{e35}"), ("tr", "Seine-Saint-Denis"), ("uk", "Сена-Сен-Дені"), ("ur", "سین-سینٹ-دونی"), ("vi", "Seine-Saint-Denis"), ("yue", "塞納－聖但尼"), ("yue_Hans", "塞纳－圣但尼"), ("zh", "塞纳-圣但尼省")]),
                        unofficial_name_list: ["Seine-Saint-Denis"].to_vec(),
                    }
                ),
                (
                    "94",
                    Subdivision{
                        name: "94",
                        country_alpha2: Alpha2::FR,
                        code: "94",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.7931426), longitude: Some(2.4740337), max_latitude: Some(48.861484), min_latitude: Some(48.68764300000001), max_longitude: Some(2.6156419), min_longitude: Some(2.3086759)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Val-de-Marne"), ("ar", "فال دو مارن"), ("az", "Val-de-Marn"), ("be", "Валь-дэ-Марн"), ("bg", "Вал дьо Марн"), ("bn", "ভ\u{9be}ল-ডে-ম\u{9be}র\u{9cd}ন"), ("ca", "Val-de-Marne"), ("ccp", "𑄞\u{11127}𑄣\u{11134}-𑄓𑄬-𑄟𑄢\u{11134}𑄚𑄬"), ("ceb", "Val-de-Marne"), ("cs", "Val-de-Marne"), ("cy", "Val-de-Marne"), ("da", "Val-de-Marne"), ("de", "Département Val-de-Marne"), ("el", "Βαλ-ντε-Μαρν"), ("en", "Val-de-Marne"), ("es", "Valle del Marne"), ("et", "Val-de-Marne"), ("eu", "Val-de-Marne"), ("fa", "ول دو مرن"), ("fi", "Val-de-Marne"), ("fr", "Val-de-Marne"), ("gl", "Val do Marne"), ("gu", "વાલ-દ\u{ac7}-માર\u{acd}ન\u{ac7}"), ("he", "עמק המארן"), ("hi", "व\u{948}ल-डी-मार\u{94d}न\u{947}"), ("hu", "Val-de-Marne"), ("hy", "Վալ-դը-Մարն"), ("id", "Val-de-Marne"), ("it", "Valle della Marna"), ("ka", "ვალ-დე-მარნი"), ("kk", "Валь-де-Марн"), ("kn", "ವ\u{ccd}ಯಾಲ\u{ccd}-ಡ\u{cbf}-ಮರ\u{ccd}ನ\u{cc6}"), ("ko", "발드마른 주"), ("lt", "Marnos slėnis"), ("lv", "Valdemarna"), ("mr", "व\u{94d}हाल-द\u{947}-मार\u{94d}न"), ("ms", "Val-de-Marne"), ("nb", "Val-de-Marne"), ("nl", "Val-de-Marne"), ("no", "Val-de-Marne"), ("pl", "Dolina Marny"), ("pt", "Val-de-Marne"), ("ro", "Val-de-Marne"), ("ru", "Валь-де-Марн"), ("si", "වල\u{dca}-ඩේ-මර\u{dca}නේ"), ("sk", "Val-de-Marne"), ("sl", "Val-de-Marne"), ("sq", "Val-de-Marne"), ("sr", "Долина Марне"), ("sr_Latn", "Dolina Marne"), ("sv", "Val-de-Marne"), ("sw", "Val-de-Marne"), ("ta", "வ\u{bbe}ல\u{bcd} -டி -மரனே"), ("te", "వ\u{c3e}ల\u{c4d}క\u{c4d}-డ\u{c3f}-మ\u{c3e}ర\u{c4d}న\u{c46}"), ("th", "จ\u{e31}งหว\u{e31}ดวาล-เดอ-มาร\u{e4c}น"), ("tr", "Val-de-Marne"), ("uk", "Валь-де-Марн"), ("ur", "ول-دو-مارن"), ("vi", "Val-de-Marne"), ("yue", "馬恩河谷"), ("yue_Hans", "马恩河谷"), ("zh", "马恩河谷省")]),
                        unofficial_name_list: ["Val-de-Marne"].to_vec(),
                    }
                ),
                (
                    "95",
                    Subdivision{
                        name: "95",
                        country_alpha2: Alpha2::FR,
                        code: "95",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.0615901), longitude: Some(2.1581351), max_latitude: Some(49.241504), min_latitude: Some(48.9086749), max_longitude: Some(2.5949791), min_longitude: Some(1.6087331)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDepartment,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Val-d’Oise"), ("ar", "فال دواز"), ("az", "Val-d’Uaz"), ("bn", "ভ\u{9be}ল-ডি‘অইস"), ("ca", "Val-d’Oise"), ("ccp", "𑄞\u{11127}𑄣\u{11134}-𑄓\u{11128}‘𑄃\u{1112e}𑄃\u{11128}𑄌\u{11134}"), ("ceb", "Val-d’Oise"), ("cs", "Val-d’Oise"), ("cy", "Val-d’Oise"), ("da", "Val-d’Oise"), ("de", "Département Val-d’Oise"), ("el", "Βαλ-ντ’Ουάζ"), ("en", "Val-d’Oise"), ("es", "Valle del Oise"), ("et", "Val-d’Oise"), ("eu", "Val-d’Oise"), ("fa", "ول-دوآز"), ("fi", "Val-d’Oise"), ("fr", "Val-d’Oise"), ("gl", "Val do Oise"), ("gu", "વ\u{ac7}લ-ડી‘ઓઇસ"), ("he", "עמק האואז"), ("hi", "व\u{947}ल-डी‘ओइस"), ("hu", "Val-d’Oise"), ("hy", "Վալ-դը-Ուազ"), ("id", "Val-d’Oise"), ("it", "Val-d’Oise"), ("kk", "Валь-д’Уаз"), ("kn", "ವ\u{ccd}ಯಾಲ\u{ccd}-ಡೈಸ\u{ccd}"), ("ko", "발두아즈 주"), ("lt", "Uazos slėnis"), ("lv", "Valduāza"), ("mr", "व\u{94d}हाल-द\u{94d}वाज"), ("ms", "Val-d’Oise"), ("nb", "Val-d’Oise"), ("nl", "Val-d’Oise"), ("no", "Val-d’Oise"), ("pl", "Val-d’Oise"), ("pt", "Val-d’Oise"), ("ro", "Val-d’Oise"), ("si", "වල\u{dca} ඩ\u{dd2} ඔය\u{dd2}ස\u{dca}"), ("sk", "Val-d’Oise"), ("sl", "Val-d’Oise"), ("sq", "Val-d’Oise"), ("sr", "Долина Оазе"), ("sr_Latn", "Dolina Oaze"), ("sv", "Val-d’Oise"), ("sw", "Val-d’Oise"), ("ta", "வ\u{bbe}ல\u{bcd} -டி‘ஒய\u{bcd}ஸ\u{bcd}"), ("te", "వ\u{c3e}ల\u{c4d}-ద\u{c3f}ఓయ\u{c3f}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดวาล-ดวซ"), ("tr", "Val-d’Oise"), ("uk", "Валь-дʼУаз"), ("ur", "ول-دواز"), ("vi", "Val-d’Oise"), ("yue", "華茲河谷"), ("yue_Hans", "华兹河谷"), ("zh", "瓦勒德瓦兹省")]),
                        unofficial_name_list: ["Val-d'Oise"].to_vec(),
                    }
                ),
                (
                    "971",
                    Subdivision{
                        name: "971",
                        country_alpha2: Alpha2::FR,
                        code: "971",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.1730949), longitude: Some(-61.4054001), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::OverseasDepartmentalCollectivity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄉\u{1112a}𑄠𑄓𑄬𑄣\u{1112f}𑄛\u{11134}"), ("en", "Guadeloupe"), ("fr", "Guadeloupe")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "972",
                    Subdivision{
                        name: "972",
                        country_alpha2: Alpha2::FR,
                        code: "972",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::OverseasUniqueTerritorialCollectivity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄟𑄢\u{11134}𑄑\u{11128}𑄚\u{11128}𑄇\u{11134}"), ("en", "Martinique")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "973",
                    Subdivision{
                        name: "973",
                        country_alpha2: Alpha2::FR,
                        code: "973",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::OverseasUniqueTerritorialCollectivity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄜\u{11133}𑄢𑄬𑄚\u{11134}𑄌\u{11134}𑄉\u{1112d}\u{1112a}𑄠𑄚"), ("en", "French Guiana"), ("fr", "Guyane")]),
                        unofficial_name_list: ["La Guyane"].to_vec(),
                    }
                ),
                (
                    "974",
                    Subdivision{
                        name: "974",
                        country_alpha2: Alpha2::FR,
                        code: "974",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.8499), longitude: Some(2.637), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::OverseasDepartmentalCollectivity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄣 𑄢𑄬𑄅\u{1112a}𑄚\u{11128}𑄠\u{11127}𑄚\u{11134}"), ("en", "La Réunion")]),
                        unofficial_name_list: ["Île Bourbon"].to_vec(),
                    }
                ),
                (
                    "976",
                    Subdivision{
                        name: "976",
                        country_alpha2: Alpha2::FR,
                        code: "976",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-12.809645), longitude: Some(45.130741), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::OverseasDepartmentalCollectivity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄟𑄠\u{1112e}𑄖\u{11133}𑄠𑄬"), ("en", "Mayotte"), ("fr", "Mayotte")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "ARA",
                    Subdivision{
                        name: "ARA",
                        country_alpha2: Alpha2::FR,
                        code: "ARA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.46035), longitude: Some(4.62416), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Auvergne-Rhône-Alpes"), ("ar", "أوفرن-رون ألب"), ("az", "Overn-Rona-Alp"), ("be", "Авернь — Рона-Альпы"), ("bg", "Оверн-Рона-Алпи"), ("ca", "Alvèrnia - Roine-Alps"), ("ccp", "𑄃\u{11127}𑄅\u{1112a}𑄞𑄢\u{11134}-𑄢\u{1112e}𑄚\u{11134}-𑄃𑄣\u{11134}𑄛𑄬𑄌\u{11134}"), ("ceb", "Auvergne-Rhône-Alpes"), ("cs", "Auvergne-Rhône-Alpes"), ("cy", "Auvergne-Rhône-Alpes"), ("da", "Auvergne-Rhône-Alpes"), ("de", "Auvergne-Rhône-Alpes"), ("el", "Ωβέρνη-Ρον-Αλπ"), ("en", "Auvergne-Rhône-Alpes"), ("es", "Auvernia-Ródano-Alpes"), ("et", "Auvergne-Rhône-Alpes"), ("eu", "Auvernia-Rhône-Alpeak"), ("fa", "اوورنی-رون-آلپ"), ("fi", "Auvergne-Rhône-Alpes"), ("fr", "Auvergne-Rhône-Alpes"), ("gl", "Auvernia-Ródano-Alpes"), ("he", "אוברן-רון-אלפ"), ("hi", "ऑव\u{947}रगन-रोन-एल\u{94d}प\u{94d}स"), ("hu", "Auvergne-Rhône-Alpes"), ("id", "Auvergne-Rhône-Alpes"), ("is", "Auvergne-Rhône-Alpes"), ("it", "Alvernia-Rodano-Alpi"), ("ko", "오베르뉴론알프"), ("lt", "Overnė-Rona-Alpės"), ("lv", "Overņa-Rona-Alpi"), ("mk", "Оверњ-Рона-Алпи"), ("nb", "Auvergne-Rhône-Alpes"), ("nl", "Auvergne-Rhône-Alpes"), ("no", "Auvergne-Rhône-Alpes"), ("oc", "Auvèrnhe Ròse Aups"), ("pl", "Owernia-Rodan-Alpy"), ("pt", "Auvérnia-Ródano-Alpes"), ("ro", "Auverge-Ron-Alpi"), ("ru", "Овернь — Рона — Альпы"), ("sk", "Auvergne-Rhône-Alpes"), ("sq", "Auvernia-Rhône-Alpes"), ("sr", "Оверња-Рона-Алпи"), ("sr_Latn", "Overnja-Rona-Alpi"), ("sv", "Auvergne-Rhône-Alpes"), ("sw", "Auvergne-Rhône-Alpes"), ("th", "แคว\u{e49}นโอแวร\u{e4c}ญ-โรนาลป\u{e4c}"), ("tr", "Auvergne-Rhône-Alpes"), ("uk", "Овернь-Рона-Альпи"), ("ur", "اوویغنئے-غون-آلپ"), ("vi", "Auvergne-Rhône-Alpes"), ("yue", "奧文尼－隆河－阿爾卑斯"), ("yue_Hans", "奥文尼－隆河－阿尔卑斯"), ("zh", "奧文尼-隆-阿爾卑斯")]),
                        unofficial_name_list: ["Auvergne-Rhône-Alpes", "Auvèrnhe-Ròse-Aups", "Ôvèrgne-Rôno-Arpes"].to_vec(),
                    }
                ),
                (
                    "BFC",
                    Subdivision{
                        name: "BFC",
                        country_alpha2: Alpha2::FR,
                        code: "BFC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.278111), longitude: Some(4.99421), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Boergondië-Franche-Comté"), ("ar", "تصنيف:بورغوني- فرانش كومتيه"), ("az", "Burqundiya—Franş—Konte"), ("bg", "Бургундия-Франш Конте"), ("ca", "Borgonya - Franc Comtat"), ("ccp", "𑄝\u{1112f}𑄢\u{11134}𑄉\u{11127}𑄇\u{11134}-𑄜\u{11133}𑄢𑄚\u{11134}𑄌\u{11134}-𑄇\u{11127}𑄟\u{11134}𑄑\u{11128}"), ("ceb", "Bourgogne-Franche-Comté"), ("cs", "Burgundsko-Franche-Comté"), ("cy", "Bourgogne-Franche-Comté"), ("da", "Bourgogne-Franche-Comté"), ("de", "Bourgogne-Franche-Comté"), ("el", "Βουργουνδία-Φρανς-Κοντέ"), ("en", "Burgundy-Franche-Comté"), ("es", "Borgoña-Franco Condado"), ("eu", "Borgoina-Franche-Comté"), ("fa", "بورگوین-فرانش-کنته"), ("fi", "Bourgogne-Franche-Comté"), ("fr", "Bourgogne-Franche-Comté"), ("gl", "Borgoña-Franco Condado"), ("he", "בורגון-פראנש-קונטה"), ("hi", "बोरगोग\u{94d}न-फ\u{93c}\u{94d}र\u{947}न\u{94d}च-कोम\u{94d}ट\u{947}"), ("id", "Bourgogne-Franche-Comté"), ("it", "Borgogna-Franca Contea"), ("ko", "부르고뉴프랑슈콩테"), ("lt", "Burgundija–Franš Kontė"), ("mn", "Бургунд муж"), ("nb", "Bourgogne-Franche-Comté"), ("nl", "Bourgogne-Franche-Comté"), ("no", "Bourgogne-Franche-Comté"), ("pl", "Burgundia-Franche-Comté"), ("pt", "Borgonha-Franco-Condado"), ("ru", "Бургундия — Франш-Конте"), ("sk", "Burgundsko-Franche-Comté"), ("sq", "Burgonja-Franche-Comté"), ("sr", "Бургундија-Франш-Конте"), ("sr_Latn", "Burgundija-Franš-Konte"), ("sv", "Bourgogne-Franche-Comté"), ("th", "แคว\u{e49}นบ\u{e39}ร\u{e4c}กอญ-ฟร\u{e47}องช\u{e4c}-กงเต"), ("uk", "Бургундія-Франш-Конте"), ("ur", "بورغونئے-فغانش-کومتے"), ("vi", "Bourgogne-Franche-Comté"), ("yue", "布爾岡-法朗殊-康堤"), ("yue_Hans", "布尔冈-法朗殊-康堤"), ("zh", "勃艮第-弗朗什-孔泰大區")]),
                        unofficial_name_list: ["Borgogne-Franche-Comtât", "Bourgogne-Franche-Comté"].to_vec(),
                    }
                ),
                (
                    "BL",
                    Subdivision{
                        name: "BL",
                        country_alpha2: Alpha2::FR,
                        code: "BL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.9139222), longitude: Some(-62.8338521), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄥𑄬𑄚\u{11134} 𑄝𑄢\u{11134}𑄗𑄬𑄣𑄬𑄟\u{11128}"), ("en", "Saint Barthélemy"), ("fr", "Saint Barthélemy")]),
                        unofficial_name_list: ["Saint Barthélemy", "St. Barthélemy"].to_vec(),
                    }
                ),
                (
                    "BRE",
                    Subdivision{
                        name: "BRE",
                        country_alpha2: Alpha2::FR,
                        code: "BRE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.002789), longitude: Some(1.68221), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Bretagne"), ("am", "ብረታኝ"), ("ar", "بريتاني"), ("az", "Bretan"), ("be", "Брэтань"), ("bg", "Бретан"), ("bn", "ব\u{9cd}রত\u{9be}ইন"), ("br", "Breizh"), ("bs", "Bretanja"), ("ca", "Bretanya"), ("ccp", "𑄝\u{11133}𑄢𑄑𑄬𑄇\u{11134}"), ("ceb", "Bretagne"), ("cs", "Bretaň"), ("cy", "Bretagne"), ("da", "Bretagne"), ("de", "Bretagne"), ("el", "Βρετάνη"), ("en", "Brittany"), ("es", "Bretaña"), ("et", "Bretagne"), ("eu", "Bretainia"), ("fa", "بریتانی (فرانسه)"), ("fi", "Bretagne"), ("fr", "Bretagne"), ("gl", "Bretaña - Breizh"), ("he", "ברטאן"), ("hr", "Bretanja"), ("hu", "Bretagne"), ("hy", "Բրետան"), ("id", "Bretagne"), ("is", "Bretanía"), ("it", "Bretagna"), ("ja", "ブルターニュ地域圏"), ("jv", "Bretagne"), ("ka", "ბრეტანი"), ("ko", "브르타뉴"), ("lt", "Bretanė"), ("lv", "Bretaņa"), ("mk", "Бретања"), ("mn", "Бретань муж"), ("mr", "ब\u{94d}रत\u{94d}तान\u{94d}य"), ("ms", "Bretagne"), ("nb", "Bretagne"), ("nl", "Bretagne"), ("no", "Bretagne"), ("pl", "Bretania"), ("pt", "Bretanha"), ("ro", "Bretania"), ("ru", "Бретань"), ("sk", "Bretónsko"), ("sl", "Bretanja"), ("sq", "Bretagne"), ("sr", "Бретања"), ("sr_Latn", "Bretanja"), ("sv", "Bretagne"), ("sw", "Bretagne"), ("th", "แคว\u{e49}นเบรอตาญ"), ("tr", "Bretonya"), ("uk", "Бретань"), ("ur", "بریتانیہ"), ("uz", "Bretan"), ("vi", "Bretagne"), ("yue", "布禮斯大區"), ("yue_Hans", "布礼斯大区"), ("zh", "布列塔尼")]),
                        unofficial_name_list: ["Breizh", "Bretagne"].to_vec(),
                    }
                ),
                (
                    "CP",
                    Subdivision{
                        name: "CP",
                        country_alpha2: Alpha2::FR,
                        code: "CP",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄇\u{11133}𑄣\u{11128}𑄛𑄢\u{11134}𑄑\u{11127}𑄚\u{11134} 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("en", "Clipperton Island")]),
                        unofficial_name_list: ["Clipperton Island"].to_vec(),
                    }
                ),
                (
                    "CVL",
                    Subdivision{
                        name: "CVL",
                        country_alpha2: Alpha2::FR,
                        code: "CVL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.64405), longitude: Some(1.59046), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Centre-Val de Loire"), ("ar", "سانتر-فال دو لوار"), ("az", "Sentr"), ("be", "Рэгіён Цэнтр"), ("bg", "Център-Вал дьо Лоар"), ("bn", "স\u{981}ত\u{9cd}র\u{9cd}\u{200c}"), ("bs", "Centre"), ("ca", "Centre"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑𑄢\u{11134}-𑄞\u{11127}𑄣\u{11134} 𑄓𑄬 𑄣\u{11130}𑄢𑄬"), ("ceb", "Centre"), ("cs", "Centre"), ("cy", "Centre"), ("da", "Centre"), ("de", "Centre-Val de Loire"), ("el", "Κεντρική Περιοχή"), ("en", "Centre-Val de Loire"), ("es", "Centro-Valle de Loira"), ("et", "Keskpiirkond"), ("eu", "Centre"), ("fa", "سانتر"), ("fi", "Centre"), ("fr", "Centre-Val de Loire"), ("gl", "Centro"), ("he", "סאנטר-עמק הלואר"), ("hi", "स\u{947}\u{902}टर-व\u{948}ल द\u{947} लोयर"), ("hr", "Centre"), ("hu", "Centre-Val de Loire"), ("id", "Centre, Perancis"), ("is", "Centre"), ("it", "Centro-Valle della Loira"), ("ja", "サントル地域圏"), ("jv", "Centre"), ("ka", "ცენტრი"), ("ko", "상트르"), ("lt", "Centras"), ("lv", "Centrs"), ("mk", "Центар-Долина на Лоара"), ("mr", "सा\u{901}त\u{94d}र"), ("ms", "Centre"), ("nb", "Centre"), ("nl", "Centre-Val de Loire"), ("no", "Centre"), ("pa", "ਸ\u{a4c}\u{a02}ਤਰ"), ("pl", "Region Centralny-Dolina Loary"), ("pt", "Centro-Vale do Loire"), ("ro", "Centru"), ("ru", "Центральная Долина Луары"), ("sk", "Centre"), ("sl", "Center, Francija"), ("sq", "Centre"), ("sv", "Centre"), ("sw", "Mkoa wa Centre"), ("th", "แคว\u{e49}นซ\u{e47}องทร\u{e4c}-วาลเดอล\u{e31}วร\u{e4c}"), ("tr", "Merkez"), ("uk", "Центр"), ("ur", "سانتر-وال دو لوار"), ("vi", "Centre"), ("yue", "中央－盧華爾山谷大區"), ("yue_Hans", "中央－卢华尔山谷大区"), ("zh", "中央")]),
                        unofficial_name_list: ["Centre Region", "Centre-Loire Valley", "Centre-Val de Loire"].to_vec(),
                    }
                ),
                (
                    "GES",
                    Subdivision{
                        name: "GES",
                        country_alpha2: Alpha2::FR,
                        code: "GES",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.92086), longitude: Some(4.41123), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Grand Est"), ("ar", "غراند إيست"), ("az", "Qrand Est"), ("be", "Гранд Эст"), ("bg", "Гранд Ест"), ("ca", "Gran Est"), ("ccp", "𑄉\u{11133}𑄢𑄚\u{11133}𑄓\u{11134}-𑄃\u{11128}𑄌\u{11134}𑄑\u{11134}"), ("ceb", "Grand Est"), ("cs", "Grand Est"), ("cy", "Dwyrain Mawr"), ("da", "Alsace-Champagne-Ardenne-Lorraine"), ("de", "der Große Osten"), ("el", "Αλσατία-Καμπανία-Αρδέννες-Λωρραίνη"), ("en", "Grand-Est"), ("es", "Gran Este"), ("eu", "Grand Est"), ("fa", "آلزاس-شامپاین-آردن-لورن"), ("fi", "Alsace-Champagne-Ardenne-Lorraine"), ("fr", "Grand Est"), ("ga", "Alsace-Champagne-Ardenne-Lorraine"), ("gl", "Alsacia-Champaña-Ardenas-Lorena"), ("he", "גראנד אסט"), ("hi", "ग\u{94d}रा\u{902}ड एस\u{94d}ट"), ("hy", "Գրանդ Էստ"), ("id", "Timur Raya"), ("it", "Grand Est"), ("ja", "グラン・テスト地域圏"), ("ko", "그랑테스트"), ("lb", "de Grouss Osten"), ("mk", "Голем Исток"), ("nb", "Grand Est"), ("nl", "Grand Est"), ("no", "Grand Est"), ("pl", "Grand Est"), ("pt", "Alsácia-Champanha-Ardenas-Lorena"), ("ru", "Гранд-Эст"), ("sk", "Alsasko-Champagne-Ardenne-Lotrinsko"), ("sq", "Lindja e madhe"), ("sr", "Гран ест"), ("sr_Latn", "Gran est"), ("sv", "Alsace-Champagne-Ardenne-Lorraine"), ("th", "แคว\u{e49}นกร\u{e47}องแต\u{e47}สต\u{e4c}"), ("tr", "Grand Est"), ("uk", "Гранд-Ест"), ("ur", "گرایت است"), ("vi", "Grand Est"), ("yue", "大東部"), ("yue_Hans", "大东部"), ("zh", "阿爾薩斯-香檳-阿登-洛林")]),
                        unofficial_name_list: ["Grand Est"].to_vec(),
                    }
                ),
                (
                    "HDF",
                    Subdivision{
                        name: "HDF",
                        country_alpha2: Alpha2::FR,
                        code: "HDF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: None, longitude: Some(2.81877), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hauts-de-France"), ("az", "O-de-Frans"), ("be", "рэгіён О-дэ-Франс"), ("bg", "О дьо Франс"), ("ca", "Alts de França"), ("ccp", "𑄦𑄅\u{1112a}𑄖\u{11134}-𑄓𑄬-𑄜\u{11133}𑄢𑄚\u{11134}𑄌\u{11134}"), ("ceb", "Hauts-de-France"), ("cs", "Hauts-de-France"), ("cy", "Nord-Pas-de-Calais-Picardie"), ("da", "Nord-Pas-de-Calais-Picardie"), ("de", "Hauts-de-France"), ("el", "Ω-ντε-Φρανς"), ("en", "Hauts-de-France"), ("es", "Hauts-de-France"), ("eu", "Hauts-de-France"), ("fa", "نور-پا-دو-کاله-پیکاردی"), ("fi", "Nord-Pas-de-Calais-Picardie"), ("fr", "Hauts-de-France"), ("gl", "Altos de Francia"), ("he", "או-דה-פראנס"), ("hi", "हाउत\u{94d}स-द\u{947}-फ\u{94d}रा\u{902}स"), ("hu", "Felső-Franciaország"), ("hy", "Օ-դը-Ֆրանս"), ("id", "Nord-Pas-de-Calais-Picardie"), ("it", "Alta Francia"), ("ko", "오드프랑스"), ("lt", "Aukštutinė Prancūzija"), ("mk", "Горна Франција"), ("nb", "Hauts-de-France"), ("nl", "Opper-Frankrijk"), ("no", "Hauts-de-France"), ("pl", "Hauts-de-France"), ("pt", "Norte-Passo de Calais-Picardia"), ("ru", "О-де-Франс"), ("sk", "Nord-Pas-de-Calais-Pikardia"), ("sq", "Epërmet e Francës"), ("sr", "О де Франс"), ("sr_Latn", "O de Frans"), ("sv", "Nord-Pas-de-Calais-Picardie"), ("th", "แคว\u{e49}นโอดฟร\u{e47}องส\u{e4c}"), ("tr", "Hauts-de-France"), ("uk", "О-де-Франс"), ("ur", "او دے فرانس"), ("vi", "Hauts-de-France"), ("yue", "上法蘭西"), ("yue_Hans", "上法兰西"), ("zh", "北-加來-皮卡第")]),
                        unofficial_name_list: ["Hauts-de-France"].to_vec(),
                    }
                ),
                (
                    "IDF",
                    Subdivision{
                        name: "IDF",
                        country_alpha2: Alpha2::FR,
                        code: "IDF",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Île-de-France"), ("am", "ኢል-ደ-ፍራንስ"), ("ar", "إيل دو فرانس"), ("az", "İl de Frans"), ("be", "Іль-дэ-Франс"), ("bg", "Ил дьо Франс"), ("bn", "ইল\u{9cd}\u{200c}-দ\u{9cd}য-ফ\u{9cd}র\u{981}স"), ("bs", "Île-de-France"), ("ca", "Illa de França"), ("ccp", "𑄃\u{1112d}𑄣𑄬-𑄓𑄬-𑄜\u{11133}𑄢𑄚\u{11134}𑄌\u{11134}"), ("ceb", "Île-de-France"), ("cs", "Île-de-France"), ("cy", "Île-de-France"), ("da", "Île-de-France"), ("de", "Île-de-France"), ("el", "Ιλ-ντε-Φρανς"), ("en", "Île-de-France"), ("es", "Isla de Francia"), ("et", "Île-de-France"), ("eu", "Île-de-France"), ("fa", "ایل-دو-فرانس"), ("fi", "Île-de-France"), ("fr", "Île-de-France"), ("ga", "Île-de-France"), ("gl", "Illa de Francia"), ("he", "איל-דה-פראנס"), ("hi", "इल-दा-फ\u{93c}\u{94d}रान\u{94d}स"), ("hr", "Île-de-France"), ("hu", "Île-de-France"), ("hy", "Իլ-դը-Ֆրանս"), ("id", "Île-de-France"), ("is", "Île-de-France"), ("it", "Île-de-France"), ("jv", "Île-de-France"), ("ka", "ილ-დე-ფრანსი"), ("km", "អ\u{17ca}\u{17b8}លដ\u{17ba}ហ\u{17d2}រ\u{17d2}វែន"), ("ko", "일드프랑스"), ("lt", "Il de Fransas"), ("lv", "Ildefransa"), ("mk", "Ил де Франс"), ("mn", "Иль-де-Франс"), ("mr", "इल-दा-फ\u{94d}रान\u{94d}स"), ("ms", "Île-de-France"), ("nb", "Île-de-France"), ("nl", "Île-de-France"), ("no", "Île-de-France"), ("pa", "ਫ\u{a3c}ਰਾ\u{a02}ਸ ਦਾ ਟਾਪ\u{a42}"), ("pl", "Île-de-France"), ("pt", "Ilha de França"), ("ro", "Île-de-France"), ("ru", "Иль-де-Франс"), ("sk", "Île-de-France"), ("sl", "Île-de-France"), ("sq", "Île-de-France"), ("sv", "Île-de-France"), ("sw", "Île-de-France"), ("ta", "இல\u{bcd} ட பிர\u{bbe}ன\u{bcd}சு"), ("th", "แคว\u{e49}นอ\u{e35}ล-เดอ-ฟร\u{e47}องส\u{e4c}"), ("tr", "Île-de-France"), ("uk", "Іль-де-Франс"), ("ur", "ایل-دو-فرانس"), ("uz", "Il-de-frans"), ("vi", "Île-de-France"), ("yue", "法蘭西島大區"), ("yue_Hans", "法兰西岛大区"), ("zh", "法兰西岛")]),
                        unofficial_name_list: ["Région Parisienne", "Île-de-France"].to_vec(),
                    }
                ),
                (
                    "MF",
                    Subdivision{
                        name: "MF",
                        country_alpha2: Alpha2::FR,
                        code: "MF",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄥𑄬𑄚\u{11134} 𑄟𑄢\u{11134}𑄑\u{11128}𑄚\u{11134}"), ("en", "St. Martin"), ("fr", "Saint-Martin")]),
                        unofficial_name_list: ["Saint Martin", "St. Martin"].to_vec(),
                    }
                ),
                (
                    "NAQ",
                    Subdivision{
                        name: "NAQ",
                        country_alpha2: Alpha2::FR,
                        code: "NAQ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.7087), longitude: Some(0.6269), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nieu-Akwitanië"), ("ar", "آكيتن جديد"), ("az", "Akvitaniya-Limuzen-Puatu-Şaranta"), ("be", "Новая Аквітанія"), ("bg", "Нова Аквитания"), ("ca", "Nova Aquitània"), ("ccp", "𑄚\u{1112f}𑄞𑄬𑄣𑄬-𑄃\u{11127}𑄇\u{1112d}\u{1112a}𑄑𑄬\u{1112d}𑄚\u{11134}"), ("ceb", "Nouvelle-Aquitaine"), ("cs", "Nová Akvitánie"), ("cy", "Nouvelle-Aquitaine"), ("da", "Aquitaine-Limousin-Poitou-Charentes"), ("de", "Nouvelle-Aquitaine"), ("el", "Νέα Ακουιτανία"), ("en", "Nouvelle-Aquitaine"), ("es", "Nueva Aquitania"), ("eu", "Akitania Berria"), ("fa", "آکیتن-لیموزن-پواتو-شرانت"), ("fi", "Aquitaine-Limousin-Poitou-Charentes"), ("fr", "Nouvelle-Aquitaine"), ("gl", "Aquitania-Lemosín-Poitou-Charentes"), ("he", "אקיטן החדשה"), ("hi", "नॉव\u{947}ल\u{947} एक\u{94d}विटाइन"), ("hr", "Nova Akvitanija"), ("hu", "Új-Aquitania"), ("hy", "Նոր Ակվիտանիա"), ("id", "Aquitaine-Baru"), ("is", "Nýja-Akvitanía"), ("it", "Nuova Aquitania"), ("ko", "누벨아키텐"), ("lv", "Jaunakvitānija"), ("mk", "Нова Аквитанија"), ("nb", "Nouvelle-Aquitaine"), ("nl", "Nouvelle-Aquitaine"), ("no", "Nouvelle-Aquitaine"), ("oc", "Nòva Aquitània"), ("pl", "Nowa Akwitania"), ("pt", "Nova-Aquitânia"), ("ru", "Новая Аквитания"), ("sk", "Nové Akvitánsko"), ("sq", "Akuitania e Re"), ("sr", "Нова Аквитанија"), ("sr_Latn", "Nova Akvitanija"), ("sv", "Aquitaine-Limousin-Poitou-Charentes"), ("th", "แคว\u{e49}นน\u{e39}แวลาก\u{e35}แตน"), ("tr", "Nouvelle-Aquitaine"), ("uk", "Нова Аквітанія"), ("ur", "نوویل-ایکیتین"), ("uz", "Yangi Akvitaniya"), ("vi", "Nouvelle-Aquitaine"), ("yue", "新阿傑坦"), ("yue_Hans", "新阿杰坦"), ("zh", "阿基坦-利木森-普瓦圖-夏朗特")]),
                        unofficial_name_list: ["Nouvelle-Aquitaine"].to_vec(),
                    }
                ),
                (
                    "NC",
                    Subdivision{
                        name: "NC",
                        country_alpha2: Alpha2::FR,
                        code: "NC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.7595731), longitude: Some(-79.01929969999999), max_latitude: Some(36.5881568), min_latitude: Some(33.8409689), max_longitude: Some(-75.4599515), min_longitude: Some(-84.32186899999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄚\u{11131} 𑄇\u{11127}𑄣𑄬𑄓\u{1112e}𑄚\u{11128}𑄠"), ("en", "New Caledonia"), ("fr", "Nouvelle-Calédonie")]),
                        unofficial_name_list: ["Nouvelle-Calédonie"].to_vec(),
                    }
                ),
                (
                    "NOR",
                    Subdivision{
                        name: "NOR",
                        country_alpha2: Alpha2::FR,
                        code: "NOR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.8499), longitude: Some(2.637), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Normandië"), ("ar", "نورماندي"), ("az", "Normandiya"), ("be", "Рэгіён Нармандыя"), ("bg", "Нормандия"), ("ca", "Normandia"), ("ccp", "𑄚\u{11127}𑄢\u{11134}𑄟𑄚\u{11134}𑄓\u{1112d}"), ("cs", "Normandie"), ("cy", "Normandi"), ("da", "Normandie"), ("de", "Normandie"), ("el", "Νορμανδία"), ("en", "Normandie"), ("es", "Normandía"), ("eu", "Normandia"), ("fa", "نرماندی"), ("fi", "Normandia"), ("fr", "Normandie"), ("gl", "Normandía"), ("he", "נורמנדי"), ("hi", "नोर\u{94d}म\u{902}डी"), ("it", "Normandia"), ("ja", "ノルマンディー地域圏"), ("ka", "ნორმანდია"), ("ko", "노르망디"), ("mk", "Нормандија"), ("nl", "Normandië"), ("pl", "Normandia"), ("pt", "Normandia"), ("ru", "Нормандия"), ("sk", "Normandia"), ("sq", "Normandia"), ("sv", "Normandie (region)"), ("th", "แคว\u{e49}นนอร\u{e4c}ม\u{e47}องด\u{e35}"), ("tr", "Normandiya"), ("uk", "Нормандія"), ("ur", "نورمینڈی"), ("vi", "Normandie"), ("yue", "諾曼第"), ("yue_Hans", "诺曼第"), ("zh", "诺曼底大区")]),
                        unofficial_name_list: ["Normandie", "Normaundie"].to_vec(),
                    }
                ),
                (
                    "OCC",
                    Subdivision{
                        name: "OCC",
                        country_alpha2: Alpha2::FR,
                        code: "OCC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.8927), longitude: Some(3.2828), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Oksitanië"), ("ar", "الأوكيتانية"), ("az", "Lanqedok-Russillyon-Midi-Pirenei"), ("be", "рэгіён Аксітанія"), ("bg", "Окситания"), ("ca", "Occitània"), ("ccp", "𑄃\u{11127}𑄇\u{11134}𑄥\u{11128}𑄑𑄚\u{1112d}"), ("ceb", "Occitanie"), ("cs", "Okcitánie"), ("cy", "Ocsitania"), ("da", "Languedoc-Roussillon-Midi-Pyrénées"), ("de", "Okzitanien"), ("el", "Οξιτανία"), ("en", "Occitanie"), ("es", "Occitania"), ("eu", "Okzitania"), ("fa", "لانگداک-روسیون-پیرنه میانه"), ("fi", "Languedoc-Roussillon-Midi-Pyrénées"), ("fr", "Occitanie"), ("gl", "Occitania"), ("he", "אוקסיטניה"), ("hi", "ओसीटानिया"), ("hy", "Օկսիտանիա"), ("id", "Ositania"), ("it", "Occitania"), ("ja", "オクシタニー地域圏"), ("ko", "옥시타니"), ("nb", "Occitanie"), ("nl", "Occitanie"), ("no", "Occitanie"), ("oc", "Occitània"), ("pl", "Oksytania"), ("pt", "Occitânia"), ("ru", "Окситания"), ("sk", "Occitánia"), ("sq", "Oksitania"), ("sr", "Окситанија"), ("sr_Latn", "Oksitanija"), ("sv", "Languedoc-Roussillon-Midi-Pyrénées"), ("th", "แคว\u{e49}นอ\u{e47}อกซ\u{e35}ตาน\u{e35}"), ("tr", "Occitanie"), ("uk", "Окситанія"), ("ur", "اوکیتانی"), ("vi", "Occitanie"), ("yue", "奧斯坦尼"), ("yue_Hans", "奥斯坦尼"), ("zh", "朗格多克-魯西永-南部-庇里牛斯")]),
                        unofficial_name_list: ["Occitania", "Occitanie", "Occitània"].to_vec(),
                    }
                ),
                (
                    "PAC",
                    Subdivision{
                        name: "PAC",
                        country_alpha2: Alpha2::FR,
                        code: "PAC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.9352), longitude: Some(6.0679), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Provence-Alpes-Côte d’Azur"), ("ar", "بروفنس ألب كوت دازور"), ("az", "Provans-Alp-Kot d’Azur"), ("be", "Праванс-Альпы-Лазурны бераг"), ("bg", "Прованс-Алпи-Лазурен бряг"), ("bn", "প\u{9cd}রোভ\u{981}স-আল\u{9cd}প-কোত দ\u{9be}জ\u{9cd}য\u{9c1}র"), ("ca", "Provença – Alps – Costa Blava"), ("ccp", "𑄛\u{11133}𑄢\u{11127}𑄞𑄬𑄚\u{11134}𑄌\u{11134}-𑄃\u{11127}𑄘\u{11134}𑄛𑄬𑄌\u{11134}-𑄇\u{1112e}𑄑𑄬-𑄓\u{11128}‘𑄃\u{11127}𑄏\u{1112a}𑄢\u{11134}"), ("ceb", "Provence-Alpes-Côte d’Azur"), ("cs", "Provence-Alpes-Côte d’Azur"), ("cy", "Provence-Alpes-Côte d’Azur"), ("da", "Provence-Alpes-Côte d’Azur"), ("de", "Provence-Alpes-Côte d’Azur"), ("el", "Προβηγκία-Άλπεις-Κυανή Ακτή"), ("en", "Provence-Alpes-Côte-d’Azur"), ("es", "Provenza-Alpes-Costa Azul"), ("et", "Provence-Alpes-Côte d’Azur"), ("eu", "Provence-Alpes-Côte d’Azur"), ("fa", "پروانس آلپ\u{200c}-کوت دازور"), ("fi", "Provence-Alpes-Côte d’Azur"), ("fr", "Provence-Alpes-Côte d’Azur"), ("ga", "Provence-Alpes-Côte d’Azur"), ("gl", "Provenza-Alpes-Costa Azul"), ("he", "פרובאנס-אלפ-קוט ד׳אזור"), ("hi", "प\u{94d}रोव\u{947}\u{902}स-एल\u{94d}पस-कोट डी‘आज\u{93c}\u{941}र"), ("hr", "Provansa-Alpe-Azurna obala"), ("hu", "Provence-Alpes-Côte d’Azur"), ("hy", "Պրովանս-Ալպեր-Լազուր ափ"), ("id", "Provence-Alpes-Côte d’Azur"), ("it", "Provenza-Alpi-Costa Azzurra"), ("jv", "Provence-Alpes-Côte d’Azur"), ("ka", "პროვანსი-ალპები-ლაჟვარდოვანი ნაპირი"), ("ko", "프로방스알프코트다쥐르"), ("lt", "Provansas-Alpės-Žydrasis Krantas"), ("lv", "Provansa-Alpi-Azūra Krasts"), ("mk", "Прованса-Алпи-Азурен Брег"), ("mr", "प\u{94d}रोव\u{94d}हा\u{901}स-आल\u{94d}प-कोत द\u{947}झ\u{94d}य\u{941}र"), ("ms", "Provence-Alpes-Côte d’Azur"), ("nb", "Provence-Alpes-Côte d’Azur"), ("nl", "Provence-Alpes-Côte d’Azur"), ("no", "Provence-Alpes-Côte d’Azur"), ("oc", "Provença-Aups-Còsta d'Azur"), ("pa", "ਪ\u{a4d}ਰ\u{a4b}ਵਾ\u{a02}ਸ-ਆਲਪ-ਅਸਮਾਨੀ ਤਟ"), ("pl", "Prowansja-Alpy-Lazurowe Wybrzeże"), ("pt", "Provença-Alpes-Costa Azul"), ("ro", "Provența-Alpi-Coasta de Azur"), ("ru", "Прованс — Альпы — Лазурный Берег"), ("sk", "Provence-Alpes-Côte d’Azur"), ("sl", "Provansa-Alpe-Azurna obala"), ("sq", "Provence-Alpes-Côte d’Azur"), ("sr", "Прованса-Алпи-Азурна обала"), ("sr_Latn", "Provansa-Alpi-Azurna obala"), ("sv", "Provence-Alpes-Côte d’Azur"), ("sw", "Provence-Alpes-Côte d’Azur"), ("th", "แคว\u{e49}นพรอว\u{e47}องซาลป\u{e4c}-โกตดาซ\u{e39}ร\u{e4c}"), ("tr", "Provence-Alpes-Côte d’Azur"), ("uk", "Прованс-Альпи-Лазурний берег"), ("ur", "پروانس-آلپ-کوت دازور"), ("vi", "Provence-Alpes-Côte d’Azur"), ("yue", "普羅旺斯-阿爾卑斯-蔚藍海岸"), ("yue_Hans", "普罗旺斯-阿尔卑斯-蔚蓝海岸"), ("zh", "普罗旺斯-阿尔卑斯-蓝色海岸")]),
                        unofficial_name_list: ["PACA", "Provence-Alpes-Côte d'Azur", "Région Sud"].to_vec(),
                    }
                ),
                (
                    "PDL",
                    Subdivision{
                        name: "PDL",
                        country_alpha2: Alpha2::FR,
                        code: "PDL",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Pays de la Loire"), ("ar", "وادي اللوار"), ("az", "Pei dö lə Luar"), ("be", "Землі Луары"), ("bg", "Пеи дьо ла Лоар"), ("bn", "পেই দ\u{9cd}য ল\u{9be} লোয\u{9bc}\u{9be}র"), ("ca", "País del Loira"), ("ccp", "𑄛𑄬\u{1112d}𑄌\u{11134}-𑄓𑄬-𑄣-𑄣\u{11130}𑄢\u{11134}"), ("ceb", "Pays-de-la-Loire"), ("cs", "Pays de la Loire"), ("cy", "Pays de la Loire"), ("da", "Pays de la Loire"), ("de", "Pays de la Loire"), ("el", "Χώρα του Λίγηρα"), ("en", "Pays-de-la-Loire"), ("es", "Países del Loira"), ("et", "Pays de la Loire"), ("eu", "Pays de la Loire"), ("fa", "پیی دو لا لوآر"), ("fi", "Pays de la Loire"), ("fr", "Pays de la Loire"), ("ga", "Pays de la Loire"), ("gl", "País do Loira"), ("he", "פיי דה לה לואר"), ("hi", "प\u{947}स\u{93c} द\u{947} ला लोइर"), ("hr", "Regija Loire"), ("hu", "Loire mente"), ("hy", "Լուարայի երկիր"), ("id", "Pays de la Loire"), ("it", "Paesi della Loira"), ("ja", "ペイ・ド・ラ・ロワール地域圏"), ("jv", "Pays de la Loire"), ("ka", "ლუარის რეგიონი"), ("ko", "페이드라루아르"), ("lt", "Luaros kraštas"), ("lv", "Luāra"), ("mk", "Лоарски Крај"), ("mr", "प\u{947}ई दा ला लोआर"), ("ms", "Pays de la Loire"), ("nb", "Pays de la Loire"), ("nl", "Pays de la Loire"), ("no", "Pays de la Loire"), ("pa", "ਲ\u{a4b}ਆਰ ਦੀ ਧਰਤੀ"), ("pl", "Kraj Loary"), ("pt", "País do Loire"), ("ro", "Pays de la Loire"), ("ru", "Земли Луары"), ("sk", "Pays-de-la-Loire"), ("sl", "Loire"), ("sq", "Pays de la Loire"), ("sr", "Регион Лоара"), ("sr_Latn", "Region Loara"), ("sv", "Pays de la Loire"), ("sw", "Pays de la Loire"), ("th", "แคว\u{e49}นเปอ\u{e35}เดอลาล\u{e31}วร\u{e4c}"), ("tr", "Pays de la Loire"), ("uk", "Пеї-де-ла-Луар"), ("ur", "پئی دو لا لوار"), ("vi", "Pays de la Loire"), ("yue", "盧華爾河區"), ("yue_Hans", "卢华尔河区"), ("zh", "卢瓦尔河地区")]),
                        unofficial_name_list: ["Broioù al Liger", "Pays de la Loire"].to_vec(),
                    }
                ),
                (
                    "PF",
                    Subdivision{
                        name: "PF",
                        country_alpha2: Alpha2::FR,
                        code: "PF",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::OverseasCollectivity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄜\u{11133}𑄢𑄬𑄚\u{11134}𑄌\u{11134} 𑄛\u{11127}𑄣\u{11128}𑄚𑄬𑄥\u{11128}𑄠"), ("en", "French Polynesia"), ("fr", "Polynésie française")]),
                        unofficial_name_list: ["La Polynésie française", "Polynésie française"].to_vec(),
                    }
                ),
                (
                    "PM",
                    Subdivision{
                        name: "PM",
                        country_alpha2: Alpha2::FR,
                        code: "PM",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::OverseasCollectivity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄥𑄬𑄚\u{11134} 𑄛\u{11128}𑄢𑄬 & 𑄟\u{11128}𑄇\u{1112d}\u{1112a}𑄣\u{11127}𑄚\u{11134}"), ("en", "St. Pierre & Miquelon"), ("fr", "Saint Pierre et Miquelon")]),
                        unofficial_name_list: ["Saint Pierre et Miquelon", "St. Pierre and Miquelon"].to_vec(),
                    }
                ),
                (
                    "TF",
                    Subdivision{
                        name: "TF",
                        country_alpha2: Alpha2::FR,
                        code: "TF",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::OverseasCollectivity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄛\u{11133}𑄢𑄬𑄚\u{11134}𑄌\u{11134} 𑄘\u{11127}𑄊\u{11128}𑄚\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄢𑄬𑄎\u{11133}𑄠\u{11127}𑄚\u{11128}"), ("en", "French Southern Territories"), ("fr", "Terres australes et antarctiques françaises")]),
                        unofficial_name_list: ["Terres Australes Françaises"].to_vec(),
                    }
                ),
                (
                    "WF",
                    Subdivision{
                        name: "WF",
                        country_alpha2: Alpha2::FR,
                        code: "WF",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::OverseasCollectivity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄤𑄣\u{11133}𑄦\u{11128}𑄌\u{11134} & 𑄜\u{11128}𑄅\u{1112a}𑄑𑄚"), ("en", "Wallis & Futuna")]),
                        unofficial_name_list: ["Wallis and Futuna", "Wallis et Futuna"].to_vec(),
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
#[cfg(feature = "fr")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::FR,
        alpha3: Alpha3::FRA,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 33,
        currency_code: "EUR",
        gec: Some(GEC::FR),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("FRA"),
        iso_long_name: "The French Republic",
        iso_short_name: "France",
        official_language_list: ["fr"].to_vec(),
        spoken_language_list: ["fr"].to_vec(),
        national_destination_code_length_list: [1].to_vec(),
        national_number_length_list: [9, 10].to_vec(),
        national_prefix: "0",
        nationality: Some("French"),
        number: "250",
        postal_code: true,
        postal_code_format: Some("\\d{2} ?\\d{3}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::WesternEurope),
        un_locode: "FR",
        unofficial_name_list: [
            "France",
            "Frankreich",
            "the French Republic",
            "フランス",
            "Frankrijk",
            "Francia",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "France"),
            ("af", "Frankryk"),
            ("ak", "France"),
            ("am", "France"),
            ("an", "France"),
            ("ar", "فرنسا"),
            ("as", "ফ\u{9cd}ইচ\u{9cd}\u{200c}ল\u{9be}মিক\u{9be}ন\u{9cd}স"),
            ("ay", "France"),
            ("az", "Fransa"),
            ("ba", "France"),
            ("be", "Францыя"),
            ("bg", "Франция"),
            ("bi", "France"),
            ("bn", "ফ\u{9cd}র\u{9be}ন\u{9cd}স"),
            ("bn_IN", "ফ\u{9cd}র\u{9be}ন\u{9cd}স"),
            ("br", "Frañs"),
            ("bs", "Francuska"),
            ("ca", "França"),
            ("ce", "Франци"),
            ("ch", "Francia"),
            ("cs", "Francie"),
            ("cv", "Франци"),
            ("cy", "Ffrainc"),
            ("da", "Frankrig"),
            ("de", "Frankreich"),
            (
                "dv",
                "ފ\u{7a6}ރ\u{7a6}ނ\u{7b0}ސ\u{7ad}ސ\u{7a8}ވ\u{7a8}ލ\u{7a7}ތ\u{7b0}",
            ),
            ("dz", "ཕར\u{f71}ནས\u{f72}།"),
            ("ee", "France"),
            ("el", "Γαλλία"),
            ("en", "France"),
            ("eo", "Francio"),
            ("es", "Francia"),
            ("et", "Prantsusmaa"),
            ("eu", "Frantzia"),
            ("fa", "فرانسه"),
            ("ff", "Faransi"),
            ("fi", "Ranska"),
            ("fo", "Frakland"),
            ("fr", "France"),
            ("fy", "Frankryk"),
            ("ga", "An Fhrainc"),
            ("gl", "Francia"),
            ("gn", "France"),
            ("gu", "ફ\u{acd}રાન\u{acd}સ"),
            ("gv", "Yn Rank"),
            ("ha", "Faransa"),
            ("he", "צרפת"),
            ("hi", "फ\u{93c}\u{94d}रान\u{94d}स"),
            ("hr", "Francuska"),
            ("ht", "Frans"),
            ("hu", "Franciaország"),
            ("hy", "Ֆրանսիա"),
            ("ia", "Francia"),
            ("id", "Perancis"),
            ("io", "Francia"),
            ("is", "Frakkland"),
            ("it", "Francia"),
            ("iu", "France"),
            ("ja", "フランス"),
            ("ka", "საფრანგეთი"),
            ("ki", "Baranja"),
            ("kk", "Франция"),
            ("kl", "France"),
            ("km", "បារា\u{17c6}ង"),
            ("kn", "ಫ\u{ccd}ರಾನ\u{ccd}ಸ\u{ccd}"),
            ("ko", "프랑스"),
            ("ku", "Fransa"),
            ("kv", "Франция"),
            ("kw", "Pow Frynk"),
            ("ky", "Франция"),
            ("lo", "ປະເທດຝະລ\u{eb1}\u{ec8}ງ"),
            ("lt", "Prancūzija"),
            ("lv", "Francija"),
            ("mi", "Wīwī"),
            ("mk", "Франција"),
            ("ml", "ഫ\u{d4d}ര\u{d3e}ന\u{d4d}\u{200d}സ\u{d4d}"),
            ("mn", "Франц"),
            ("mr", "फ\u{94d}रान\u{94d}स"),
            ("ms", "Perancis"),
            ("mt", "Franza"),
            (
                "my",
                "ပြင\u{103a}သစ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Prant"),
            ("nb", "Frankrike"),
            ("ne", "फ\u{94d}रान\u{94d}स"),
            ("nl", "Frankrijk"),
            ("nn", "Frankrike"),
            ("nv", "Dáághahii Dinéʼiʼ Bikéyah"),
            ("oc", "França"),
            ("or", "ଫ\u{b4d}ର\u{b3e}ନ\u{b4d}ସ"),
            ("pa", "ਫਰਾ\u{a02}ਸ"),
            ("pi", "फ\u{94d}रा\u{902}स"),
            ("pl", "Francja"),
            ("ps", "فرانسه"),
            ("pt", "França"),
            ("pt_BR", "França"),
            ("ro", "Franța"),
            ("ru", "Франция"),
            ("rw", "Ubufaransa"),
            ("sc", "Frantza"),
            ("sd", "فرانس"),
            ("si", "ප\u{dca}\u{200d}රංශය"),
            ("sk", "Francúzsko"),
            ("sl", "Francija"),
            ("so", "Faransiis"),
            ("sq", "Francë"),
            ("sr", "Француска"),
            ("sv", "Frankrike"),
            ("sw", "Ufaransa"),
            ("ta", "ஃப\u{bcd}ர\u{bbe}ன\u{bcd}ஸ\u{bcd}"),
            ("te", "ఫ\u{c4d}ర\u{c3e}న\u{c4d}స\u{c4d}"),
            ("tg", "Фаронса"),
            ("th", "ฝร\u{e31}\u{e48}งเศส"),
            ("ti", "ፈረንሳ"),
            ("tk", "Fransiýa"),
            ("tl", "Pransya"),
            ("tr", "Fransa"),
            ("tt", "Франсиа"),
            ("ug", "فىرانسىيە"),
            ("uk", "Франція"),
            ("ur", "فرانس"),
            ("uz", "Fransiya"),
            ("ve", "Fura"),
            ("vi", "Pháp"),
            ("wa", "France"),
            ("wo", "Faraas"),
            ("xh", "Franisi"),
            ("yo", "Fránsì"),
            ("zh_CN", "法国"),
            ("zh_HK", "法國"),
            ("zh_TW", "法國"),
            ("zu", "IFulansi"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
