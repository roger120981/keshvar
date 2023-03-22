// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Collectivity of Saint-Barthélemy

#[cfg(all(feature = "bl", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::BL;
    pub const ALPHA3: Alpha3 = Alpha3::BLM;
    pub const CONTINENT: Continent = Continent::NorthAmerica;
    pub const COUNTRY_CODE: usize = 590;
    pub const CURRENCY_CODE: &str = "EUR";
    pub const GEC: Option<GEC> = Some(GEC::TB);
    pub const INTERNATIONAL_PREFIX: &str = "";
    pub const IOC: Option<&str> = None;
    pub const ISO_SHORT_NAME: &str = "Saint Barthélemy";
    pub const ISO_LONG_NAME: &str = "The Collectivity of Saint-Barthélemy";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["fr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["fr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[];
    pub const NATIONAL_PREFIX: &str = "";
    pub const NATIONALITY: Option<&str> = Some("Saint Barthélemy Islander");
    pub const NUMBER: &str = "652";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("9[78][01]\\d{2}");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::Caribbean);
    pub const UN_LOCODE: &str = "BL";
    pub const UNOFFICIAL_NAME_LIST: &[&str] =
        &["Saint Barthélemy", "Saint-Barthélemy", "サン・バルテルミー"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Saint Barthélemy"),
        ("af", "Sint Barthélemy"),
        ("ak", "Saint Barthélemy"),
        ("am", "Saint Barthélemy"),
        ("an", "Saint Barthélemy"),
        ("ar", "سان بارتليمي"),
        ("as", "চেন\u{9cd}ট ব\u{9be}ৰ\u{9cd}থেলেমি"),
        ("ay", "Saint Barthélemy"),
        ("az", "Saint Barthélemy"),
        ("ba", "Saint Barthélemy"),
        ("be", "Сен-Бартэльмі"),
        ("bg", "Свети Вартоломей"),
        ("bi", "Saint Barthélemy"),
        ("bn", "সেন\u{9cd}ট ব\u{9be}র\u{9cd}থেলেমি"),
        ("bn_IN", "সেন\u{9cd}ট ব\u{9be}র\u{9cd}থেলেমি"),
        ("br", "Saint Barthélemy"),
        ("bs", "Saint Barthélemy"),
        ("ca", "Saint Barthélemy"),
        ("ce", "Saint Barthélemy"),
        ("ch", "Saint Barthélemy"),
        ("cs", "Svatý Bartoloměj"),
        ("cv", "Saint Barthélemy"),
        ("cy", "Saint Barthélemy"),
        ("da", "Sankt Bartolomæus"),
        ("de", "Saint-Barthélemy"),
        ("dv", "Saint Barthélemy"),
        ("dz", "ས\u{f7a}ནཊ\u{f72}་བ\u{f71}རཐ་ལ\u{f7a}་མ\u{f72}།"),
        ("ee", "Saint Barthélemy"),
        ("el", "Άγιος Βαρθολομαίος"),
        ("en", "Saint Barthélemy"),
        ("eo", "Sankta-Bartolomeo"),
        ("es", "San Bartolomé"),
        ("et", "Saint-Barthélemy"),
        ("eu", "Saint Barthélemy"),
        ("fa", "سنت بارتلمی"),
        ("ff", "Saint Barthélemy"),
        ("fi", "Saint Barthélemy"),
        ("fo", "Saint Barthélemy"),
        ("fr", "Saint-Barthélemy"),
        ("fy", "Saint Barthélemy"),
        ("ga", "San Parthalán"),
        ("gl", "San Bartolomé"),
        ("gn", "Saint Barthélemy"),
        ("gu", "સ\u{ac7}ન\u{acd}ટ બાર\u{acd}થ\u{ac7}લ\u{ac7}મિ"),
        ("gv", "Saint Barthélemy"),
        ("ha", "Saint Barthélemy"),
        ("he", "סנט ברתלמי"),
        ("hi", "स\u{947}\u{902}ट बार\u{94d}थ\u{947}ल\u{947}मी"),
        ("hr", "Sveti Bartolomej"),
        ("ht", "Saint Barthélemy"),
        ("hu", "Saint Barthélemy"),
        ("hy", "Սուրբ Բարդուղիմեոսի կղզի"),
        ("ia", "Sancte Barthelemy"),
        ("id", "Saint Barthélemy"),
        ("io", "Saint Barthélemy"),
        ("is", "Sankti Bartelemí"),
        ("it", "Saint Barts"),
        ("iu", "Saint Barthélemy"),
        ("ja", "サンバルテルミ"),
        ("ka", "სენ-ბართლემი"),
        ("ki", "Saint Barthélemy"),
        ("kk", "Сен-Бартельми"),
        ("kl", "Saint Barthélemy"),
        ("km", "សាន\u{200b}បាទេលេម\u{17b8}"),
        ("kn", "Saint Barthélemy"),
        ("ko", "생바르텔레미"),
        ("ku", "Saint Barthélemy"),
        ("kv", "Saint Barthélemy"),
        ("kw", "Saint Barthélemy"),
        ("ky", "Сен-Бартельми"),
        ("lo", "Saint Barthélemy"),
        ("lt", "San Bartelemis"),
        ("lv", "Senbartelmī"),
        ("mi", "Saint Barthélemy"),
        ("mk", "Св. Бартоломеј"),
        (
            "ml",
            "സെയിന\u{d4d}റ\u{d4d} ബ\u{d3e}ര\u{d4d}\u{200d}ത\u{d4d}തെലേമി",
        ),
        ("mn", "Saint Barthélemy"),
        ("mr", "स\u{947}\u{902}ट बर\u{94d}थ\u{947}ल\u{947}मी"),
        ("ms", "Saint Barthélemy"),
        ("mt", "Saint Barthélemy"),
        ("my", "Saint Barthélemy"),
        ("na", "Saint Barthélemy"),
        ("nb", "Saint-Barthélemy"),
        ("ne", "शन\u{94d}त Barthelemy"),
        ("nl", "Saint-Barthélemy"),
        ("nn", "Saint Barthélemy"),
        ("nv", "Saint Barthélemy"),
        ("oc", "Sant Bertomieu"),
        ("or", "ସେଣ\u{b4d}ଟ ବ\u{b3e}ର\u{b4d}ଥେଲେମୀ"),
        ("pa", "ਸ\u{a47}\u{a02}ਟ ਬਰਥੀਲੀਮ\u{a47}"),
        ("pi", "Saint Barthélemy"),
        ("pl", "Saint-Barthélemy"),
        ("ps", "Saint Barthélemy"),
        ("pt", "Saint Barthélemy"),
        ("pt_BR", "São Bartolomeu"),
        ("ro", "Sfântul Bartolomeu"),
        ("ru", "Сен-Бартельми"),
        ("rw", "Saint Barthélemy"),
        ("sc", "Saint Barthélemy"),
        ("sd", "Saint Barthélemy"),
        ("si", "ශ\u{dcf}න\u{dca}ත බර\u{dca}තොලම\u{dd2}ය\u{dd4}"),
        ("sk", "Svätý Bartolomej"),
        ("sl", "Saint Barthelemy"),
        ("so", "Saint Barthélemy"),
        ("sq", "Shën Bartolome"),
        ("sr", "Свети Бартоломеј"),
        ("sv", "Saint-Barthélemy"),
        ("sw", "Saint Barthélemy"),
        ("ta", "செயின\u{bcd}ட\u{bcd} ப\u{bbe}ர\u{bcd}த\u{bcd}லெமி"),
        ("te", "స\u{c48}ంట\u{c4d} బర\u{c4d}త\u{c46}ల\u{c46}మ\u{c3f}"),
        ("tg", "Сент\u{ad}Бартелеми"),
        ("th", "แซงบาร\u{e4c}เตเลอม\u{e35}"),
        ("ti", "Saint Barthélemy"),
        ("tk", "Saint Barthélemy"),
        ("tl", "Saint Barthélemy"),
        ("tr", "Saint Barthélemy"),
        ("tt", "Saint Barthélemy"),
        ("ug", "ساينىت-بارتھېلەمي"),
        ("uk", "Сен-Бартельмі"),
        ("ur", "Saint Barthélemy"),
        ("uz", "Saint Barthélemy"),
        ("ve", "Saint Barthélemy"),
        ("vi", "Saint Barthélemy"),
        ("wa", "Sint Bartelemi"),
        ("wo", "Saint Barthélemy"),
        ("xh", "Saint Barthélemy"),
        ("yo", "Saint Barthélemy"),
        ("zh_CN", "圣巴泰勒米岛"),
        ("zh_HK", "聖巴泰勒米"),
        ("zh_TW", "聖巴瑟米"),
        ("zu", "Saint Barthélemy"),
    ];
    #[cfg(all(feature = "bl", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 17.9;
        pub const LONGITUDE: f64 = -62.833333;
        pub const MAX_LATITUDE: f64 = 17.978;
        pub const MAX_LONGITUDE: f64 = -62.7869;
        pub const MIN_LATITUDE: f64 = 17.8663;
        pub const MIN_LONGITUDE: f64 = -62.9559999;
        pub const NORTHEAST_LATITUDE: f64 = 17.978;
        pub const NORTHEAST_LONGITUDE: f64 = -62.7869;
        pub const SOUTHWEST_LATITUDE: f64 = 17.8663;
        pub const SOUTHWEST_LONGITUDE: f64 = -62.9559999;
    }
}
#[cfg(all(feature = "bl", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 17.9,
            longitude: -62.833333,
            max_latitude: 17.978,
            max_longitude: -62.7869,
            min_latitude: 17.8663,
            min_longitude: -62.9559999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 17.978,
                    longitude: -62.7869,
                },
                southwest: CountryGeoBound {
                    latitude: 17.8663,
                    longitude: -62.9559999,
                },
            },
        }
    }
}

#[cfg(all(feature = "bl", feature = "subdivisions"))]
pub mod subdivisions {
    use crate::Subdivision;
    use std::collections::HashMap;
    // In this state, We do not know if subdivisions have geo or not!
    #[cfg(feature = "geo")]
    #[allow(unused_imports)]
    use crate::{Alpha2, SubdivisionGeo, SubdivisionType};

    pub fn new() -> HashMap<&'static str, Subdivision> {
        HashMap::from([])
    }
}
#[allow(unused_imports)]
use crate::{Alpha2, Alpha3, Continent, Country, Region, SubRegion, WeekDay, WorldRegion, GEC};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "bl")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::BL,
        alpha3: Alpha3::BLM,
        address_format: None,
        continent: Continent::NorthAmerica,
        country_code: 590,
        currency_code: "EUR",
        gec: Some(GEC::TB),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "",
        ioc: None,
        iso_long_name: "The Collectivity of Saint-Barthélemy",
        iso_short_name: "Saint Barthélemy",
        official_language_list: ["fr"].to_vec(),
        spoken_language_list: ["fr"].to_vec(),
        national_destination_code_length_list: [].to_vec(),
        national_number_length_list: [].to_vec(),
        national_prefix: "",
        nationality: Some("Saint Barthélemy Islander"),
        number: "652",
        postal_code: true,
        postal_code_format: Some("9[78][01]\\d{2}"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::Caribbean),
        un_locode: "BL",
        unofficial_name_list: ["Saint Barthélemy", "Saint-Barthélemy", "サン・バルテルミー"]
            .to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Saint Barthélemy"),
            ("af", "Sint Barthélemy"),
            ("ak", "Saint Barthélemy"),
            ("am", "Saint Barthélemy"),
            ("an", "Saint Barthélemy"),
            ("ar", "سان بارتليمي"),
            ("as", "চেন\u{9cd}ট ব\u{9be}ৰ\u{9cd}থেলেমি"),
            ("ay", "Saint Barthélemy"),
            ("az", "Saint Barthélemy"),
            ("ba", "Saint Barthélemy"),
            ("be", "Сен-Бартэльмі"),
            ("bg", "Свети Вартоломей"),
            ("bi", "Saint Barthélemy"),
            ("bn", "সেন\u{9cd}ট ব\u{9be}র\u{9cd}থেলেমি"),
            ("bn_IN", "সেন\u{9cd}ট ব\u{9be}র\u{9cd}থেলেমি"),
            ("br", "Saint Barthélemy"),
            ("bs", "Saint Barthélemy"),
            ("ca", "Saint Barthélemy"),
            ("ce", "Saint Barthélemy"),
            ("ch", "Saint Barthélemy"),
            ("cs", "Svatý Bartoloměj"),
            ("cv", "Saint Barthélemy"),
            ("cy", "Saint Barthélemy"),
            ("da", "Sankt Bartolomæus"),
            ("de", "Saint-Barthélemy"),
            ("dv", "Saint Barthélemy"),
            ("dz", "ས\u{f7a}ནཊ\u{f72}་བ\u{f71}རཐ་ལ\u{f7a}་མ\u{f72}།"),
            ("ee", "Saint Barthélemy"),
            ("el", "Άγιος Βαρθολομαίος"),
            ("en", "Saint Barthélemy"),
            ("eo", "Sankta-Bartolomeo"),
            ("es", "San Bartolomé"),
            ("et", "Saint-Barthélemy"),
            ("eu", "Saint Barthélemy"),
            ("fa", "سنت بارتلمی"),
            ("ff", "Saint Barthélemy"),
            ("fi", "Saint Barthélemy"),
            ("fo", "Saint Barthélemy"),
            ("fr", "Saint-Barthélemy"),
            ("fy", "Saint Barthélemy"),
            ("ga", "San Parthalán"),
            ("gl", "San Bartolomé"),
            ("gn", "Saint Barthélemy"),
            ("gu", "સ\u{ac7}ન\u{acd}ટ બાર\u{acd}થ\u{ac7}લ\u{ac7}મિ"),
            ("gv", "Saint Barthélemy"),
            ("ha", "Saint Barthélemy"),
            ("he", "סנט ברתלמי"),
            ("hi", "स\u{947}\u{902}ट बार\u{94d}थ\u{947}ल\u{947}मी"),
            ("hr", "Sveti Bartolomej"),
            ("ht", "Saint Barthélemy"),
            ("hu", "Saint Barthélemy"),
            ("hy", "Սուրբ Բարդուղիմեոսի կղզի"),
            ("ia", "Sancte Barthelemy"),
            ("id", "Saint Barthélemy"),
            ("io", "Saint Barthélemy"),
            ("is", "Sankti Bartelemí"),
            ("it", "Saint Barts"),
            ("iu", "Saint Barthélemy"),
            ("ja", "サンバルテルミ"),
            ("ka", "სენ-ბართლემი"),
            ("ki", "Saint Barthélemy"),
            ("kk", "Сен-Бартельми"),
            ("kl", "Saint Barthélemy"),
            ("km", "សាន\u{200b}បាទេលេម\u{17b8}"),
            ("kn", "Saint Barthélemy"),
            ("ko", "생바르텔레미"),
            ("ku", "Saint Barthélemy"),
            ("kv", "Saint Barthélemy"),
            ("kw", "Saint Barthélemy"),
            ("ky", "Сен-Бартельми"),
            ("lo", "Saint Barthélemy"),
            ("lt", "San Bartelemis"),
            ("lv", "Senbartelmī"),
            ("mi", "Saint Barthélemy"),
            ("mk", "Св. Бартоломеј"),
            (
                "ml",
                "സെയിന\u{d4d}റ\u{d4d} ബ\u{d3e}ര\u{d4d}\u{200d}ത\u{d4d}തെലേമി",
            ),
            ("mn", "Saint Barthélemy"),
            ("mr", "स\u{947}\u{902}ट बर\u{94d}थ\u{947}ल\u{947}मी"),
            ("ms", "Saint Barthélemy"),
            ("mt", "Saint Barthélemy"),
            ("my", "Saint Barthélemy"),
            ("na", "Saint Barthélemy"),
            ("nb", "Saint-Barthélemy"),
            ("ne", "शन\u{94d}त Barthelemy"),
            ("nl", "Saint-Barthélemy"),
            ("nn", "Saint Barthélemy"),
            ("nv", "Saint Barthélemy"),
            ("oc", "Sant Bertomieu"),
            ("or", "ସେଣ\u{b4d}ଟ ବ\u{b3e}ର\u{b4d}ଥେଲେମୀ"),
            ("pa", "ਸ\u{a47}\u{a02}ਟ ਬਰਥੀਲੀਮ\u{a47}"),
            ("pi", "Saint Barthélemy"),
            ("pl", "Saint-Barthélemy"),
            ("ps", "Saint Barthélemy"),
            ("pt", "Saint Barthélemy"),
            ("pt_BR", "São Bartolomeu"),
            ("ro", "Sfântul Bartolomeu"),
            ("ru", "Сен-Бартельми"),
            ("rw", "Saint Barthélemy"),
            ("sc", "Saint Barthélemy"),
            ("sd", "Saint Barthélemy"),
            ("si", "ශ\u{dcf}න\u{dca}ත බර\u{dca}තොලම\u{dd2}ය\u{dd4}"),
            ("sk", "Svätý Bartolomej"),
            ("sl", "Saint Barthelemy"),
            ("so", "Saint Barthélemy"),
            ("sq", "Shën Bartolome"),
            ("sr", "Свети Бартоломеј"),
            ("sv", "Saint-Barthélemy"),
            ("sw", "Saint Barthélemy"),
            ("ta", "செயின\u{bcd}ட\u{bcd} ப\u{bbe}ர\u{bcd}த\u{bcd}லெமி"),
            ("te", "స\u{c48}ంట\u{c4d} బర\u{c4d}త\u{c46}ల\u{c46}మ\u{c3f}"),
            ("tg", "Сент\u{ad}Бартелеми"),
            ("th", "แซงบาร\u{e4c}เตเลอม\u{e35}"),
            ("ti", "Saint Barthélemy"),
            ("tk", "Saint Barthélemy"),
            ("tl", "Saint Barthélemy"),
            ("tr", "Saint Barthélemy"),
            ("tt", "Saint Barthélemy"),
            ("ug", "ساينىت-بارتھېلەمي"),
            ("uk", "Сен-Бартельмі"),
            ("ur", "Saint Barthélemy"),
            ("uz", "Saint Barthélemy"),
            ("ve", "Saint Barthélemy"),
            ("vi", "Saint Barthélemy"),
            ("wa", "Sint Bartelemi"),
            ("wo", "Saint Barthélemy"),
            ("xh", "Saint Barthélemy"),
            ("yo", "Saint Barthélemy"),
            ("zh_CN", "圣巴泰勒米岛"),
            ("zh_HK", "聖巴泰勒米"),
            ("zh_TW", "聖巴瑟米"),
            ("zu", "Saint Barthélemy"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}