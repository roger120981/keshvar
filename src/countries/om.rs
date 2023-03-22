// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Sultanate of Oman

#[cfg(all(feature = "om", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{region}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::OM;
    pub const ALPHA3: Alpha3 = Alpha3::OMN;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 968;
    pub const CURRENCY_CODE: &str = "OMR";
    pub const GEC: Option<GEC> = Some(GEC::MU);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("OMA");
    pub const ISO_SHORT_NAME: &str = "Oman";
    pub const ISO_LONG_NAME: &str = "The Sultanate of Oman";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Omani");
    pub const NUMBER: &str = "512";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("(?:PC )?\\d{3}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Sunday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternAsia);
    pub const UN_LOCODE: &str = "OM";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Oman", "عمان", "Omán", "オマーン"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Oman"),
        ("af", "Oman"),
        ("ak", "Oman"),
        ("am", "ኦማን"),
        ("an", "Oman"),
        ("ar", "عمان"),
        ("as", "ওম\u{9be}ন"),
        ("ay", "Oman"),
        ("az", "Oman"),
        ("ba", "Oman"),
        ("be", "Аман"),
        ("bg", "Оман"),
        ("bi", "Oman"),
        ("bn", "ওম\u{9be}ন"),
        ("bn_IN", "ওম\u{9be}ন"),
        ("br", "Oman"),
        ("bs", "Oman"),
        ("ca", "Oman"),
        ("ce", "Оман"),
        ("ch", "Oman"),
        ("cs", "Omán"),
        ("cv", "Оман"),
        ("cy", "Oman"),
        ("da", "Oman"),
        ("de", "Oman"),
        ("dv", "ޢ\u{7aa}މ\u{7a7}ނ\u{7b0}"),
        ("dz", "ཨ\u{f7c}་མ\u{f71}ན།"),
        ("ee", "Oman"),
        ("el", "Ομάν"),
        ("en", "Oman"),
        ("eo", "Omano"),
        ("es", "Omán"),
        ("et", "Omaan"),
        ("eu", "Oman"),
        ("fa", "عمان"),
        ("ff", "Oman"),
        ("fi", "Oman"),
        ("fo", "Oman"),
        ("fr", "Oman"),
        ("fy", "Oman"),
        ("ga", "Oman"),
        ("gl", "Omán"),
        ("gn", "Oman"),
        ("gu", "ઓમાન"),
        ("gv", "Yn Omaan"),
        ("ha", "Oman"),
        ("he", "עומאן"),
        ("hi", "ओमान"),
        ("hr", "Oman"),
        ("ht", "Omàn"),
        ("hu", "Omán"),
        ("hy", "Օման"),
        ("ia", "Oman"),
        ("id", "Oman"),
        ("io", "Oman"),
        ("is", "Óman"),
        ("it", "Oman"),
        ("iu", "Oman"),
        ("ja", "オマーン"),
        ("ka", "ომანი"),
        ("ki", "Oman"),
        ("kk", "Оман"),
        ("kl", "Oman"),
        ("km", "អ\u{17bc}ម\u{17c9}ង\u{17cb}"),
        ("kn", "ಒಮನ\u{ccd}"),
        ("ko", "오만"),
        ("ku", "Uman"),
        ("kv", "Оман"),
        ("kw", "Oman"),
        ("ky", "Оман"),
        ("lo", "Oman"),
        ("lt", "Omanas"),
        ("lv", "Omāna"),
        ("mi", "Ōmana"),
        ("mk", "Оман"),
        ("ml", "ഒമ\u{d3e}ന\u{d4d}\u{200d}"),
        ("mn", "Оман"),
        ("mr", "ओमान"),
        ("ms", "Oman"),
        ("mt", "Oman"),
        (
            "my",
            "အ\u{102d}\u{102f}မန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Oman"),
        ("nb", "Oman"),
        ("ne", "ओमन"),
        ("nl", "Oman"),
        ("nn", "Oman"),
        ("nv", "Oman"),
        ("oc", "Oman"),
        ("or", "ଓମ\u{b3e}ନ"),
        ("pa", "ਓਮਾਨ"),
        ("pi", "ओमान"),
        ("pl", "Oman"),
        ("ps", "عمان"),
        ("pt", "Omã"),
        ("pt_BR", "Omã"),
        ("ro", "Oman"),
        ("ru", "Оман"),
        ("rw", "Omani"),
        ("sc", "Omàn"),
        ("sd", "سلطنت عمان"),
        ("si", "ඕම\u{dcf}නය"),
        ("sk", "Omán"),
        ("sl", "Oman"),
        ("so", "Cumaan"),
        ("sq", "Oman"),
        ("sr", "Оман"),
        ("sv", "Oman"),
        ("sw", "Oman"),
        ("ta", "ஓம\u{bbe}ன\u{bcd}"),
        ("te", "ఓమన\u{c4d}"),
        ("tg", "Умон"),
        ("th", "โอมาน"),
        ("ti", "Oman"),
        ("tk", "Oman"),
        ("tl", "Oman"),
        ("tr", "Umman"),
        ("tt", "Оман"),
        ("ug", "ئومان"),
        ("uk", "Оман"),
        ("ur", "سلطنت عمان"),
        ("uz", "Oman"),
        ("ve", "Oman"),
        ("vi", "Ô-man"),
        ("wa", "Oman"),
        ("wo", "Omaan"),
        ("xh", "Oman"),
        ("yo", "Oman"),
        ("zh_CN", "阿曼"),
        ("zh_HK", "阿曼"),
        ("zh_TW", "阿曼"),
        ("zu", "Oman"),
    ];
    #[cfg(all(feature = "om", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 21.4735329;
        pub const LONGITUDE: f64 = 55.975413;
        pub const MAX_LATITUDE: f64 = 26.4361001;
        pub const MAX_LONGITUDE: f64 = 60.30399999999999;
        pub const MIN_LATITUDE: f64 = 16.4571999;
        pub const MIN_LONGITUDE: f64 = 52.0000019;
        pub const NORTHEAST_LATITUDE: f64 = 26.4361001;
        pub const NORTHEAST_LONGITUDE: f64 = 60.30399999999999;
        pub const SOUTHWEST_LATITUDE: f64 = 16.4571999;
        pub const SOUTHWEST_LONGITUDE: f64 = 52.0000019;
    }
}
#[cfg(all(feature = "om", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 21.4735329,
            longitude: 55.975413,
            max_latitude: 26.4361001,
            max_longitude: 60.30399999999999,
            min_latitude: 16.4571999,
            min_longitude: 52.0000019,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 26.4361001,
                    longitude: 60.30399999999999,
                },
                southwest: CountryGeoBound {
                    latitude: 16.4571999,
                    longitude: 52.0000019,
                },
            },
        }
    }
}

#[cfg(all(feature = "om", feature = "subdivisions"))]
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
                    "BJ",
                    Subdivision{
                        name: "BJ",
                        country_alpha2: Alpha2::OM,
                        code: "BJ",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة جنوب الباطنة"), ("bn", "আল ব\u{9be}তিন\u{9be}হ দক\u{9cd}ষিন গভর\u{9cd}নোরেট"), ("ccp", "𑄎𑄚\u{1112a}𑄛\u{11134} 𑄃𑄣\u{11134} 𑄝𑄑\u{11128}𑄚𑄦\u{11134}"), ("cs", "Jižní al-Batína"), ("da", "Al Batinah South Governorate"), ("de", "Dschanub al-Batina"), ("el", "Αλ Μπατίνα Σάουθ Γκοβερνοράτε"), ("en", "Janub al Batinah"), ("es", "Al Batinah Sur gobernación"), ("eu", "Hego Al Batinah"), ("fa", "استان باطنه جنوبی"), ("fi", "Al-Batinan eteläinen kuvernoraatti"), ("fr", "Al-Batina du Sud"), ("gu", "અલ બટીનાહ સાઉથ ગવર\u{acd}નોર\u{ac7}ટ"), ("hi", "अल बातिनाह दक\u{94d}षिण गवर\u{94d}नर\u{947}ट"), ("id", "Kegubernuran Al-Batinah Selatan"), ("it", "Al Batinah South Governorate"), ("ja", "南バーティナ地方"), ("kn", "ಅಲ\u{ccd} ಬಟ\u{cbf}ನಾ ಸ\u{ccc}ತ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "남바티나 주"), ("lt", "Al Batino pietų gubernija"), ("lv", "Dienvidbātinas muhāfaza"), ("mr", "अल बतिनाह साऊथ गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Al Batinah South Governorate"), ("nb", "Sør-Al Batinah"), ("nl", "Al Batinah"), ("no", "Sør-Al Batinah"), ("pl", "Gubernatorstwo Dżanub al-Batina"), ("pt", "Governamento de Al Batinah Sul"), ("ru", "Северная Эль-Батина"), ("si", "අල\u{dca} බට\u{dd2}න\u{dcf} දක\u{dd4}ණ\u{dd4} පළ\u{dcf}ත"), ("sv", "Södra Al Barinah"), ("ta", "அல\u{bcd} பட\u{bcd}டினஹ\u{bcd} தெற\u{bcd}கு கோவெர\u{bcd}னோரே"), ("te", "అల\u{c4d} బ\u{c3e}ట\u{c3f}న\u{c3e} దక\u{c4d}ష\u{c3f}ణ గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เขตอ\u{e31}ลบาต\u{e34}นะห\u{e4c}"), ("tr", "Al Batinah Güney Yönetimi"), ("uk", "Муніципалітет Південна Ель-Батіна"), ("ur", "ال باتینہ ساؤتھ جوویرنوراتی"), ("vi", "Tỉnh Al Batinah South")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "BS",
                    Subdivision{
                        name: "BS",
                        country_alpha2: Alpha2::OM,
                        code: "BS",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة شمال الباطنة"), ("ccp", "𑄥\u{11133}𑄠𑄟\u{11127}𑄣\u{11134} 𑄃𑄣\u{11134} 𑄝𑄑\u{11128}𑄚𑄦\u{11134}"), ("cs", "Severní al-Batína"), ("de", "Schamal al-Batina"), ("en", "Shamal al Batinah"), ("es", "Al Batinah Norte gobernación"), ("eu", "Ipar Al Batinah"), ("fa", "استان باطنه شمالی"), ("fi", "Al-Batinan pohjoinen kuvernoraatti"), ("fr", "Al-Batina du Nord"), ("id", "Kegubernuran Al-Batinah Utara"), ("it", "Al Batinah North Governorate"), ("ja", "北バーティナ地方"), ("ko", "북바티나 주"), ("nb", "Nord-Al Batinah"), ("no", "Nord-Al Batinah"), ("pl", "Muhafazat Szamal al-Batina"), ("pt", "Batina Setentrional")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "BU",
                    Subdivision{
                        name: "BU",
                        country_alpha2: Alpha2::OM,
                        code: "BU",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة البريمي"), ("bn", "আল ব\u{9c1}র\u{9be}ইমি গভর\u{9cd}নোরেট"), ("ccp", "𑄃𑄣\u{11134} 𑄝\u{1112a}𑄢\u{1112d}𑄟\u{11128}"), ("cs", "Al-Burajmi (guvernorát)"), ("da", "Al Buraimi Governorate"), ("de", "Buraimi (Gouvernement)"), ("el", "Αλ Μπουραΐμι Γκοβερνοράτε"), ("en", "Al Buraimi"), ("es", "Buraimi"), ("eu", "Al Buraimi"), ("fa", "استان بریمی"), ("fi", "Al-Buraimi"), ("fr", "Gouvernorat d’Al Buraymi"), ("gu", "અલ બ\u{ac1}ર\u{ac8}મી ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "אל בוראימי"), ("hi", "अल ब\u{941}र\u{948}मी म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hu", "Burajmi kormányzóság"), ("id", "Kegubernuran Al-Buraimi"), ("it", "governatorato di al-Buraymi"), ("ja", "ブライミ特別行政区"), ("kn", "ಅಲ\u{ccd} ಬುರೈಮ\u{cbf} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "부라이미 주"), ("lt", "Buraimis"), ("lv", "Buraimī muhāfaza"), ("mr", "अल ब\u{941}र\u{947}मी गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Al Buraimi Governorate"), ("nb", "Al Buraimi"), ("nl", "Al Buraimi"), ("no", "Al Buraimi"), ("pl", "Muhafazat al-Burajmi"), ("pt", "Al Buraimi"), ("ru", "Эль-Бурайми"), ("si", "අල\u{dca} බ\u{dd4}රය\u{dd2}ම\u{dd2} පළ\u{dcf}ත"), ("sv", "Al Buraimi"), ("sw", "Al Buraimi"), ("ta", "அல\u{bcd} புரைம\u{bc0} கோவெர\u{bcd}னோரே"), ("te", "అల\u{c4d} బుర\u{c48}మ\u{c3f} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}ล บ\u{e39}ไรม\u{e34} โกเวอโนเรท"), ("tr", "El Buraymi valiliği"), ("uk", "Ель-Бураймі"), ("ur", "محافظہ البریمی"), ("vi", "Buraimi"), ("zh", "布賴米省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "DA",
                    Subdivision{
                        name: "DA",
                        country_alpha2: Alpha2::OM,
                        code: "DA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.3476325), longitude: Some(57.2818625), max_latitude: Some(23.5632319), min_latitude: Some(20.904974), max_longitude: Some(58.34188510000001), min_longitude: Some(56.576047)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة الداخلية"), ("bn", "আদ দ\u{9be}খিলিয\u{9bc}\u{9be} গভর\u{9cd}নোরেট"), ("ccp", "𑄃𑄖\u{11134} 𑄓𑄈\u{1112d}𑄣\u{11128}𑄠𑄦\u{11134}"), ("cs", "Ad-Dáchílija"), ("da", "Ad Dakhiliyah Governorate"), ("de", "Ad-Dachiliyya"), ("el", "Αντ Ντακιλιγιά Γκοβερνοράτε"), ("en", "Ad Dakhiliyah"), ("es", "Interior (Omán)"), ("eu", "Ad Dakhiliyah"), ("fa", "منطقه داخلیه"), ("fi", "Al-Dakhiliyya"), ("fr", "Ad-Dākhilīyah"), ("gu", "એડ ડિખિલીયાહ ગવર\u{acd}નોર\u{ac7}ટ"), ("hi", "अद दाख\u{93c}िलीया म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hu", "Belső régió (Omán)"), ("id", "Kegubernuran Ad-Dakhiliyah"), ("it", "governatorato di al-Dakhiliyya"), ("ja", "ダーヒリーヤ行政区"), ("kn", "ಆಡ\u{ccd} ದಖ\u{cbf}ಲ\u{cbf}ಯಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "다킬리야 주"), ("lt", "Dachilijos regionas"), ("lv", "Dāhilījas mintaka"), ("mr", "एड दखिलियाह गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Wilayah Ad Dakhiliyah"), ("nb", "Ad Dakhiliyah"), ("nl", "Ad Dachiliyah"), ("no", "Ad Dakhiliyah"), ("pl", "Al-Mintakat ad-Dachilijja"), ("pt", "Ad Dakhiliyah"), ("ru", "Эд-Дахилия"), ("si", "අඩ\u{dca} ඩක\u{dd2}ල\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sv", "Ad Dakhiliyah"), ("sw", "Ad Dakhiliyah"), ("ta", "அட டக\u{bcd}ஹிலிய\u{bcd}யஹ\u{bcd} கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "అద\u{c4d} ద\u{c3e}ఖ\u{c3f}ల\u{c4d}య\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เซต\u{e34} โซน"), ("tr", "Ed Dahiliye Bölgesi"), ("uk", "Ед-Дахілія"), ("ur", "محافظہ الداخلیہ"), ("vi", "Tỉnh Ad Dakhiliyah"), ("zh", "內地省")]),
                        unofficial_name_list: ["Ad Dakhiliya"].to_vec(),
                    }
                ),
                (
                    "MA",
                    Subdivision{
                        name: "MA",
                        country_alpha2: Alpha2::OM,
                        code: "MA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(23.61), longitude: Some(58.54), max_latitude: Some(23.6455689), min_latitude: Some(23.5211218), max_longitude: Some(58.6189567), min_longitude: Some(58.2283758)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة مسقط"), ("bn", "ম\u{9c1}স\u{9cd}ক\u{9be}ট গভর\u{9cd}নোরেট"), ("ca", "Governació de Masqat"), ("ccp", "𑄟𑄌\u{11134}𑄇𑄖\u{11134}"), ("cs", "Maskat"), ("da", "Muscat Governorate"), ("de", "Maskat"), ("el", "Μουσκάτ"), ("en", "Muscat"), ("es", "Gobernación Muscat"), ("eu", "Maskat"), ("fa", "استان مسقط"), ("fi", "Muscatin kuvernoraatti"), ("fr", "Gouvernorat de Mascate"), ("gu", "મસ\u{acd}કટ ગવર\u{acd}નોર\u{ac7}ટ"), ("hi", "मस\u{94d}क\u{93c}त म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hu", "Maszkat kormányzóság"), ("id", "Kegubernuran Muskat"), ("it", "governatorato di Mascate"), ("ja", "マスカット特別行政区"), ("kn", "ಮಸ\u{ccd}ಕಟ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "무스카트 주"), ("lt", "Muskato gubernija"), ("lv", "Muskatas muhāfaza"), ("mr", "मस\u{94d}कट गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Muscat Governorate"), ("nb", "Muskat"), ("nl", "Masqat"), ("no", "Muskat"), ("pl", "Muhafazat Maskat"), ("pt", "Governamento de Muscat"), ("ro", "guvernoratul Muscat"), ("ru", "Маскат"), ("si", "මස\u{dca}කට\u{dca} පළ\u{dcf}ත"), ("sv", "Muskat"), ("sw", "Maskat"), ("ta", "மஸ\u{bcd}கட\u{bcd} கோவெர\u{bcd}னோகைட\u{bcd}"), ("te", "మస\u{c4d}కట\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เม\u{e37}องม\u{e31}สก\u{e31}ต"), ("tr", "Maskat valiliği"), ("uk", "Маскат"), ("ur", "محافظہ مسقط"), ("vi", "Tỉnh Muscat"), ("zh", "馬斯喀特省")]),
                        unofficial_name_list: ["Mascate", "Maskat", "Masqaţ"].to_vec(),
                    }
                ),
                (
                    "MU",
                    Subdivision{
                        name: "MU",
                        country_alpha2: Alpha2::OM,
                        code: "MU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(25.9942638), longitude: Some(56.24822769999999), max_latitude: Some(26.385954), min_latitude: Some(25.2419609), max_longitude: Some(56.5376924), min_longitude: Some(56.09789079999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة مسندم"), ("bn", "ম\u{9c1}স\u{9be}ন\u{9cd}দ\u{9be}ম গভর\u{9cd}নোরেট"), ("ca", "Musandam"), ("ccp", "𑄟\u{1112a}𑄥𑄚\u{11134}𑄓𑄟\u{11134}"), ("cs", "Musandam"), ("da", "Musandam Governorate"), ("de", "Musandam"), ("el", "Μουσαντάμ"), ("en", "Musandam"), ("es", "Musandam"), ("et", "Musandami provints"), ("eu", "Musandam"), ("fa", "مسندم"), ("fi", "Musandam"), ("fr", "Musandam"), ("gl", "Península de Musandam"), ("gu", "મ\u{ac1}સાન\u{acd}ડમ ગવર\u{acd}નોર\u{ac7}ટ"), ("hi", "म\u{941}सा\u{902}डाम गवर\u{94d}नर\u{947}ट"), ("hr", "Musandam"), ("hu", "Muszandam kormányzóság"), ("id", "Musandam"), ("it", "Governatorato di Musandam"), ("ja", "ムサンダム特別行政区"), ("kn", "ಮುಸಂದಮ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "무산담 주"), ("lt", "Musandamas"), ("lv", "Musendemas muhāfaza"), ("ml", "മ\u{d41}സന\u{d4d}ധം"), ("mr", "म\u{941}सामदाम गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Musandam"), ("nb", "Musandam"), ("nl", "Musandam"), ("no", "Musandam"), ("pl", "Muhafazat Musandam"), ("pt", "Península de Musandam"), ("ru", "Мусандам"), ("si", "ම\u{dd4}සන\u{dca}ඩම\u{dca} පළ\u{dcf}ත"), ("sl", "Musandam"), ("sr", "Мусандам"), ("sr_Latn", "Musandam"), ("sv", "Musandam"), ("sw", "Musandam"), ("ta", "முஸந\u{bcd}தம\u{bcd} கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "ముసందం గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "ม\u{e39}แซนดำ กอฟเวอโนเลท"), ("tr", "Musandam valiliği"), ("uk", "Мусандам"), ("ur", "محافظہ مسندم"), ("vi", "Musandam"), ("zh", "穆桑代姆省")]),
                        unofficial_name_list: ["Musandam"].to_vec(),
                    }
                ),
                (
                    "SJ",
                    Subdivision{
                        name: "SJ",
                        country_alpha2: Alpha2::OM,
                        code: "SJ",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة جنوب الشرقية"), ("bn", "আশ শ\u{9be}রকিয\u{9bc}\u{9be} দক\u{9cd}ষিন গভর\u{9cd}নোরেট"), ("ccp", "𑄎𑄚\u{1112a}𑄛\u{11134} 𑄃𑄌\u{11134} 𑄥\u{11127}𑄢\u{11134}𑄇\u{1112a}𑄃\u{11128}𑄦\u{11134}"), ("ceb", "Ash Sharqiyah South"), ("cs", "Jižní aš-Šarkíja"), ("da", "Ash Sharqiyah South Governorate"), ("de", "Dschanub asch-Scharqiyya"), ("el", "Ας Σαρκιγιάχ"), ("en", "Janub ash Sharqiyah"), ("es", "Ash Sharqiyah Sur Gobernación"), ("eu", "Hego Ash Sharqiyah"), ("fa", "استان شرقیه جنوبی"), ("fi", "Ash-Šarqiyyan eteläinen kuvernoraatti"), ("fr", "Gouvernorat Ach-Charqiya du Sud"), ("gu", "એશ શાર\u{acd}કિયાહ , સાઉથ ગવર\u{acd}નોર\u{ac7}ટ"), ("hi", "ऐश शार\u{94d}कियाह दक\u{94d}षिण गवर\u{94d}नर\u{947}ट"), ("id", "Kegubernuran Asy-Syarqiyah Selatan"), ("it", "Ash Sharqiyah South Governorate"), ("ja", "南シャルキーヤ地方"), ("kn", "ಆಷ\u{ccd} ಶರ\u{ccd}ಕ\u{cbf}ಯಾ ದಕ\u{ccd}ಷ\u{cbf}ಣ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "남동부 주"), ("lt", "Aš Šarkijos pietinė gubernija"), ("lv", "Dienvidšerkījas muhāfaza"), ("mr", "शशरियाह साऊथ गव\u{94d}हरन\u{947}ट"), ("ms", "Ash Sharqiyah South Governorate"), ("nb", "Sør-Ash Sharqiyah"), ("nl", "Ash Sharqiyah gouvernement"), ("no", "Sør-Ash Sharqiyah"), ("pl", "Prowincja Południowo-Wschodnia (Oman)"), ("pt", "Ash Sharqiyah Sul"), ("ru", "Южная Эш-Шаркия"), ("si", "අශ\u{dca} ශර\u{dca}ක\u{dd2}ය\u{dcf} දක\u{dd4}ණ\u{dd4} පළ\u{dcf}ත"), ("sv", "Södra Ash Sharqiyah"), ("ta", "ஆஷ\u{bcd} ஷ\u{bbe}ர\u{bcd}க\u{bcd}கிய தெற\u{bcd}கு கோவெர\u{bcd}னோர\u{bbe}ட\u{bcd}"), ("te", "ఆష\u{c4d} షర\u{c4d}ఖ\u{c3f}య\u{c3e} దక\u{c4d}ష\u{c3f}ణ గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "แอชชาร\u{e4c}ก\u{e34}ยาห\u{e4c} เซาท\u{e4c} โกเวอโนเนท"), ("tr", "Ash Shargiyah Güney Yönetimi"), ("uk", "Південна Еш-Шаркія"), ("ur", "عیش شارقیاح ساؤتھ جوویرنوراتی"), ("vi", "Tỉnh Nam Ash Sharqiyah"), ("zh", "東南省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SS",
                    Subdivision{
                        name: "SS",
                        country_alpha2: Alpha2::OM,
                        code: "SS",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة شمال الشرقية"), ("bn", "আশ শ\u{9be}রকিয\u{9bc}\u{9be}হ গভর\u{9cd}নোরেট"), ("ccp", "𑄥\u{11133}𑄠𑄟\u{11127}𑄣\u{11134} 𑄃𑄌\u{11134} 𑄥𑄢\u{11134}𑄇\u{1112a}𑄃\u{11128}𑄦\u{11134}"), ("cs", "Severní aš-Šarkíja"), ("da", "Ash Sharqiyah North Governorate"), ("de", "Schamal asch-Scharqiyya"), ("el", "Ας Σαρκίγια Νόρθ Γκοβερνοράτε"), ("en", "Shamal ash Sharqiyah"), ("es", "Gobernación Norte del Ash Sharqiyah"), ("eu", "Ipar Ash Sharqiyah"), ("fa", "استان شرقیه شمالی"), ("fi", "Ash-Šarqiyyan pohjoinen kuvernoraatti"), ("fr", "Ach-Charqiya du Nord"), ("gu", "એશ શારકીયાહ નોર\u{acd}થ ગવર\u{acd}નોર\u{ac7}ટ"), ("hi", "एश शरीकिया उत\u{94d}तरी गवर\u{94d}नर\u{947}ट"), ("id", "Kegubernuran Asy-Syarqiyah Utara"), ("it", "Ash Sharqiyah North Governorate"), ("ja", "北シャルキーヤ地方"), ("kn", "ಆಷ\u{ccd} ಶರ\u{ccd}ಕ\u{cbf}ಯಾ ಉತ\u{ccd}ತರ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "북동부 주"), ("lt", "Aš Šarkijos šiaurinė gubernija"), ("lv", "Aš Šarkijas ziemeļu muhāfaza"), ("mr", "अश शकीयाह नॉर\u{94d}थ गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Pentadbiran Ash Sharqiyah Utara"), ("nb", "Nord-Ash Sharqiyah"), ("nl", "Asj Sjarqiyah"), ("no", "Nord-Ash Sharqiyah"), ("pl", "Ash Sharqiyah North Governorate"), ("pt", "Governamento de Ash Sharqiyah Norte"), ("ru", "Северная Эш-Шаркия"), ("si", "ආශ\u{dca} ශර\u{dca}ක\u{dd2}ය\u{dcf} උත\u{dd4}ර\u{dd4} පළ\u{dcf}ත"), ("sv", "Nord-Ash Sharqiyah"), ("ta", "ஆஷ\u{bcd} ஷ\u{bbe}ர\u{bcd}க\u{bcd}கிய வடக\u{bcd}கு கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "య\u{c3e}ష\u{c4d} షర\u{c4d}ఖ\u{c3f}య\u{c3e} న\u{c3e}ర\u{c4d}త\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เขตการปกครองอาชาร\u{e4c}ก\u{e35}ยะฮ\u{e4c} ตอนเหน\u{e37}อ"), ("tr", "Ash Sharqiyah Kuzey Yönetimi"), ("uk", "Північно-Східна Провінція Еш-Шаркія"), ("ur", "عیش شارقیاح نارتھ جوویرنوراتی"), ("vi", "Tỉnh Bắc Ash Sharqiyah"), ("zh", "東北省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "WU",
                    Subdivision{
                        name: "WU",
                        country_alpha2: Alpha2::OM,
                        code: "WU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.1738655), longitude: Some(56.56164700000001), max_latitude: Some(21.420908), min_latitude: Some(17.9456731), max_longitude: Some(58.3227421), min_longitude: Some(54.9999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة الوسطى"), ("bn", "আল উস\u{9cd}ত\u{9be} গভর\u{9cd}নোরেট"), ("ccp", "𑄃𑄣\u{11134} 𑄃\u{1112e}𑄅\u{1112a}𑄌\u{11134}𑄑"), ("cs", "Al-Wusta"), ("da", "Al Wusta Governorate"), ("de", "Al-Wusta"), ("el", "Αλ Γούστα"), ("en", "Al Wusta"), ("es", "Central"), ("eu", "Al Wusta"), ("fa", "منطقه وسطی"), ("fi", "Al-Wusta"), ("fr", "Al Wusta"), ("gu", "અલ વસ\u{acd}ટા ગવર\u{acd}નોર\u{ac7}ટ"), ("hi", "अल व\u{941}स\u{94d}ता म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hu", "Középső régió"), ("id", "Kegubernuran Al-Wusta (Oman)"), ("it", "governatorato di al-Wusta"), ("ja", "ウスタ行政区"), ("kn", "ಅಲ\u{ccd} ವಸ\u{ccd}ಟಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "중부 주"), ("lt", "Vustos regionas"), ("lv", "Vustas muhāfaza"), ("mr", "अल वस\u{94d}ता गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Wilayah Al Wusta"), ("nb", "Al Wusta"), ("nl", "Al Wusta"), ("no", "Al Wusta"), ("pl", "Al-Mintakat al-Wusta"), ("pt", "Al Wusta"), ("ru", "Эль-Вуста"), ("si", "අල\u{dca} ව\u{dd4}ට\u{dca}ස\u{dcf} පළ\u{dcf}ත"), ("sv", "Al Wusta"), ("sw", "Al Wusta"), ("ta", "அல\u{bcd} வ\u{bcd}உஸ\u{bcd}த\u{bbe} கோவெர\u{bcd}னோரே"), ("te", "అల\u{c4d} వుస\u{c4d}ట\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เขตอ\u{e31}ลว\u{e38}สตะ"), ("tr", "El Vusta Bölgesi"), ("uk", "Ель-Вуста"), ("ur", "محافظہ وسطی (عمان)"), ("vi", "Tỉnh Al Wusta"), ("zh", "中部省")]),
                        unofficial_name_list: ["Al Wustá"].to_vec(),
                    }
                ),
                (
                    "ZA",
                    Subdivision{
                        name: "ZA",
                        country_alpha2: Alpha2::OM,
                        code: "ZA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.5590272), longitude: Some(56.0249982), max_latitude: Some(24.019926), min_latitude: Some(21.2735939), max_longitude: Some(57.116874), min_longitude: Some(55.2069211)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة الظاهرة"), ("bn", "আদ দ\u{9be}হির\u{9be} গভর\u{9cd}নোরেট"), ("ca", "Dhahirah"), ("ccp", "𑄃𑄖\u{11134} 𑄙𑄦\u{11128}𑄢𑄦\u{11134}"), ("cs", "Ad-Zahíra"), ("da", "Ad Dhahirah Governorate"), ("de", "Az-Zahirah"), ("el", "Αμπ Νταχίρα"), ("en", "Ad Dhahirah"), ("es", "Ad Dhahirah"), ("eu", "Ad Dhahirah"), ("fa", "استان ظاهره"), ("fi", "Al-Zahira"), ("fr", "Ad Dhahirah"), ("gu", "એડ ધાહિરાહ ગવર\u{acd}નોર\u{ac7}ટ"), ("hi", "अज\u{93c} ज\u{93c}ाहिराह म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hu", "Záhira régió"), ("id", "Kegubernuran Azh-Zhahirah Utara"), ("it", "governatorato di al-Zahira"), ("ja", "ザーヒラ行政区"), ("kn", "ಆದ\u{cbf} ದಹ\u{cbf}ರಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "다히라 주"), ("lt", "Dahiros regionas"), ("lv", "Zāhiras mintaka"), ("mr", "ऍड धारिह गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Ad Dhahirah Governorate"), ("nb", "Ad Dhahirah"), ("nl", "Az Zahirah"), ("no", "Ad Dhahirah"), ("pl", "Mintakat az-Zahira"), ("pt", "Ad Dhahirah"), ("ru", "Эз-Захира"), ("si", "අඩ\u{dca} ඩය\u{dd2}ර\u{dcf} පළ\u{dcf}ත"), ("sv", "Ad Dhahirah"), ("sw", "Ad Dhahirah"), ("ta", "அட\u{bcd} த\u{bbe}ஹிர\u{bbe} கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "ఆద\u{c4d} దహ\u{c48}ర\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}ดดะค\u{e34}ล\u{e34}ระห\u{e4c} โกเวอโนเรท"), ("tr", "Ez Zahira Bölgesi"), ("uk", "Ез-Захіра"), ("ur", "محافظہ الظاہرہ"), ("vi", "Tỉnh Ad Dhahirah"), ("zh", "扎希拉省")]),
                        unofficial_name_list: ["Adh Dhahirah"].to_vec(),
                    }
                ),
                (
                    "ZU",
                    Subdivision{
                        name: "ZU",
                        country_alpha2: Alpha2::OM,
                        code: "ZU",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة ظفار"), ("bn", "ধোফ\u{9be}র গভর\u{9cd}নোরেট"), ("ca", "Dhofar"), ("ccp", "𑄙\u{1112e}𑄜𑄢\u{11134}"), ("cs", "Dafár"), ("da", "Dhofar"), ("de", "Dhofar"), ("el", "Ντοφάρ"), ("en", "Dhofar"), ("es", "Dhofar"), ("eu", "Dhofar"), ("fa", "استان ظفار"), ("fi", "Dhofar"), ("fr", "Dhofar"), ("gu", "ઢોફાર ગવર\u{acd}નોર\u{ac7}ટ"), ("hi", "ज\u{93c}ोफ\u{93c}ार म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hu", "Zofár kormányzóság"), ("id", "Dhofar"), ("it", "Dhofar"), ("ja", "ドファール特別行政区"), ("kn", "ಧೋಫರ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "도파르 주"), ("lt", "Dofaras"), ("lv", "Dofaras muhāfaza"), ("mr", "ढोफार गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Dhofar Governorate"), ("nb", "Dhofar"), ("nl", "Dhofar"), ("no", "Dhofar"), ("pl", "Muhafazat Zufar"), ("pt", "Zufar"), ("ru", "Дофар"), ("si", "ඩොෆ\u{dcf}ර\u{dca} පළ\u{dcf}ත"), ("sl", "Dofar"), ("sv", "Dhofar"), ("sw", "Dhofar"), ("ta", "தோப\u{bbe}ர\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "ద\u{c4b}ఫ\u{c3e}ర\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "ซ\u{e38}ฟาร\u{e4c}"), ("tr", "Zufar valiliği"), ("uk", "Дофар"), ("ur", "محافظہ ظفار"), ("vi", "Tỉnh Dhofar"), ("zh", "佐法爾省")]),
                        unofficial_name_list: [].to_vec(),
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
#[cfg(feature = "om")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::OM,
        alpha3: Alpha3::OMN,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{region}}\n{{country}}",
        ),
        continent: Continent::Asia,
        country_code: 968,
        currency_code: "OMR",
        gec: Some(GEC::MU),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("OMA"),
        iso_long_name: "The Sultanate of Oman",
        iso_short_name: "Oman",
        official_language_list: ["ar"].to_vec(),
        spoken_language_list: ["ar"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "None",
        nationality: Some("Omani"),
        number: "512",
        postal_code: true,
        postal_code_format: Some("(?:PC )?\\d{3}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Sunday,
        subregion: Some(SubRegion::WesternAsia),
        un_locode: "OM",
        unofficial_name_list: ["Oman", "عمان", "Omán", "オマーン"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Oman"),
            ("af", "Oman"),
            ("ak", "Oman"),
            ("am", "ኦማን"),
            ("an", "Oman"),
            ("ar", "عمان"),
            ("as", "ওম\u{9be}ন"),
            ("ay", "Oman"),
            ("az", "Oman"),
            ("ba", "Oman"),
            ("be", "Аман"),
            ("bg", "Оман"),
            ("bi", "Oman"),
            ("bn", "ওম\u{9be}ন"),
            ("bn_IN", "ওম\u{9be}ন"),
            ("br", "Oman"),
            ("bs", "Oman"),
            ("ca", "Oman"),
            ("ce", "Оман"),
            ("ch", "Oman"),
            ("cs", "Omán"),
            ("cv", "Оман"),
            ("cy", "Oman"),
            ("da", "Oman"),
            ("de", "Oman"),
            ("dv", "ޢ\u{7aa}މ\u{7a7}ނ\u{7b0}"),
            ("dz", "ཨ\u{f7c}་མ\u{f71}ན།"),
            ("ee", "Oman"),
            ("el", "Ομάν"),
            ("en", "Oman"),
            ("eo", "Omano"),
            ("es", "Omán"),
            ("et", "Omaan"),
            ("eu", "Oman"),
            ("fa", "عمان"),
            ("ff", "Oman"),
            ("fi", "Oman"),
            ("fo", "Oman"),
            ("fr", "Oman"),
            ("fy", "Oman"),
            ("ga", "Oman"),
            ("gl", "Omán"),
            ("gn", "Oman"),
            ("gu", "ઓમાન"),
            ("gv", "Yn Omaan"),
            ("ha", "Oman"),
            ("he", "עומאן"),
            ("hi", "ओमान"),
            ("hr", "Oman"),
            ("ht", "Omàn"),
            ("hu", "Omán"),
            ("hy", "Օման"),
            ("ia", "Oman"),
            ("id", "Oman"),
            ("io", "Oman"),
            ("is", "Óman"),
            ("it", "Oman"),
            ("iu", "Oman"),
            ("ja", "オマーン"),
            ("ka", "ომანი"),
            ("ki", "Oman"),
            ("kk", "Оман"),
            ("kl", "Oman"),
            ("km", "អ\u{17bc}ម\u{17c9}ង\u{17cb}"),
            ("kn", "ಒಮನ\u{ccd}"),
            ("ko", "오만"),
            ("ku", "Uman"),
            ("kv", "Оман"),
            ("kw", "Oman"),
            ("ky", "Оман"),
            ("lo", "Oman"),
            ("lt", "Omanas"),
            ("lv", "Omāna"),
            ("mi", "Ōmana"),
            ("mk", "Оман"),
            ("ml", "ഒമ\u{d3e}ന\u{d4d}\u{200d}"),
            ("mn", "Оман"),
            ("mr", "ओमान"),
            ("ms", "Oman"),
            ("mt", "Oman"),
            (
                "my",
                "အ\u{102d}\u{102f}မန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Oman"),
            ("nb", "Oman"),
            ("ne", "ओमन"),
            ("nl", "Oman"),
            ("nn", "Oman"),
            ("nv", "Oman"),
            ("oc", "Oman"),
            ("or", "ଓମ\u{b3e}ନ"),
            ("pa", "ਓਮਾਨ"),
            ("pi", "ओमान"),
            ("pl", "Oman"),
            ("ps", "عمان"),
            ("pt", "Omã"),
            ("pt_BR", "Omã"),
            ("ro", "Oman"),
            ("ru", "Оман"),
            ("rw", "Omani"),
            ("sc", "Omàn"),
            ("sd", "سلطنت عمان"),
            ("si", "ඕම\u{dcf}නය"),
            ("sk", "Omán"),
            ("sl", "Oman"),
            ("so", "Cumaan"),
            ("sq", "Oman"),
            ("sr", "Оман"),
            ("sv", "Oman"),
            ("sw", "Oman"),
            ("ta", "ஓம\u{bbe}ன\u{bcd}"),
            ("te", "ఓమన\u{c4d}"),
            ("tg", "Умон"),
            ("th", "โอมาน"),
            ("ti", "Oman"),
            ("tk", "Oman"),
            ("tl", "Oman"),
            ("tr", "Umman"),
            ("tt", "Оман"),
            ("ug", "ئومان"),
            ("uk", "Оман"),
            ("ur", "سلطنت عمان"),
            ("uz", "Oman"),
            ("ve", "Oman"),
            ("vi", "Ô-man"),
            ("wa", "Oman"),
            ("wo", "Omaan"),
            ("xh", "Oman"),
            ("yo", "Oman"),
            ("zh_CN", "阿曼"),
            ("zh_HK", "阿曼"),
            ("zh_TW", "阿曼"),
            ("zu", "Oman"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
