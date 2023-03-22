// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Federal Republic of Nigeria

#[cfg(all(feature = "ng", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::NG;
    pub const ALPHA3: Alpha3 = Alpha3::NGA;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 234;
    pub const CURRENCY_CODE: &str = "NGN";
    pub const GEC: Option<GEC> = Some(GEC::NI);
    pub const INTERNATIONAL_PREFIX: &str = "009";
    pub const IOC: Option<&str> = Some("NGR");
    pub const ISO_SHORT_NAME: &str = "Nigeria";
    pub const ISO_LONG_NAME: &str = "The Federal Republic of Nigeria";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7, 8];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Nigerian");
    pub const NUMBER: &str = "566";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{6}");
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternAfrica);
    pub const UN_LOCODE: &str = "NG";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Nigeria",
        "Nigéria",
        "the Federal Republic of Nigeria",
        "ナイジェリア",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Nigeria"),
        ("af", "Nigerië"),
        ("ak", "Nigeria"),
        ("am", "ና፤ጄሱ።"),
        ("an", "Nigeria"),
        ("ar", "نيجيريا"),
        ("as", "ন\u{9be}ইজেৰিয়\u{9be}"),
        ("ay", "Nigeria"),
        ("az", "Nigeriya"),
        ("ba", "Nigeria"),
        ("be", "Нігерыя"),
        ("bg", "Нигерия"),
        ("bi", "Nigeria"),
        ("bn", "ন\u{9be}ইজেরিয়\u{9be}"),
        ("bn_IN", "ন\u{9be}ইজেরিয়\u{9be}"),
        ("br", "Nigeria"),
        ("bs", "Nigerija"),
        ("ca", "Nigèria"),
        ("ce", "Нигери"),
        ("ch", "Nigeria"),
        ("cs", "Nigérie"),
        ("cv", "Нигери"),
        ("cy", "Nigeria"),
        ("da", "Nigeria"),
        ("de", "Nigeria"),
        ("dv", "ނ\u{7a6}އ\u{7a8}ޖ\u{7a9}ރ\u{7a8}އ\u{7a7}"),
        ("dz", "ནའ\u{f72}་ཇ\u{f72}་ར\u{f72}་ཡ།"),
        ("ee", "Nigeria"),
        ("el", "Νιγηρία"),
        ("en", "Nigeria"),
        ("eo", "Niĝerio"),
        ("es", "Nigeria"),
        ("et", "Nigeeria"),
        ("eu", "Nigeria"),
        ("fa", "نیجریه"),
        ("ff", "Niiseriya"),
        ("fi", "Nigeria"),
        ("fo", "Nigeria"),
        ("fr", "Nigeria"),
        ("fy", "Nigearia"),
        ("ga", "An Nigéir"),
        ("gl", "Nixeria"),
        ("gn", "Nigeria"),
        ("gu", "નાઇજ\u{ac7}રિયા"),
        ("gv", "Yn Naigeer"),
        ("ha", "Nijeriya"),
        ("he", "ניגריה"),
        ("hi", "नाईजीरिया"),
        ("hr", "Nigerija"),
        ("ht", "Nijerya"),
        ("hu", "Nigéria"),
        ("hy", "Նիգերիա"),
        ("ia", "Nigeria"),
        ("id", "Nigeria"),
        ("io", "Nigeria"),
        ("is", "Nígería"),
        ("it", "Nigeria"),
        ("iu", "Nigeria"),
        ("ja", "ナイジェリア"),
        ("ka", "ნიგერია"),
        ("ki", "Nigeria"),
        ("kk", "Нигерия"),
        ("kl", "Nigeria"),
        ("km", "ន\u{17b8}ហ\u{17d2}សេរ\u{17b8}យ\u{17c9}ា"),
        ("kn", "ನೈಜೀರ\u{cbf}ಯಾ"),
        ("ko", "나이지리아"),
        ("ku", "Nîjerya"),
        ("kv", "Nigeria"),
        ("kw", "Nijeri"),
        ("ky", "Нигерия"),
        ("lo", "Nigeria"),
        ("lt", "Nigerija"),
        ("lv", "Nigērija"),
        ("mi", "Nigeria"),
        ("mk", "Нигерија"),
        ("ml", "നൈജീരിയ"),
        ("mn", "Нигерь"),
        ("mr", "नायज\u{947}रिया"),
        ("ms", "Nigeria"),
        ("mt", "Niġerja"),
        (
            "my",
            "န\u{102d}\u{102f}င\u{103a}ဂျ\u{102e}းရ\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Nigeria"),
        ("nb", "Nigeria"),
        ("ne", "नाइज\u{947}रिया"),
        ("nl", "Nigeria"),
        ("nn", "Nigeria"),
        ("nv", "Naakaii Łizhinii Biʼéénézí Bikéyah"),
        ("oc", "Nigèria"),
        ("or", "ନ\u{b3f}ଈଜୀର\u{b3f}ୟ\u{b3e}"),
        ("pa", "ਨੀਜ\u{a3c}ੀਰਆ"),
        ("pi", "न\u{948}जीरिया"),
        ("pl", "Nigeria"),
        ("ps", "نایجیریا"),
        ("pt", "Nigéria"),
        ("pt_BR", "Nigéria"),
        ("ro", "Nigeria"),
        ("ru", "Нигерия"),
        ("rw", "Nigeriya"),
        ("sc", "Nigèria"),
        ("sd", "نائيجيريا"),
        ("si", "නය\u{dd2}ජ\u{dd3}ර\u{dd2}ය\u{dcf}"),
        ("sk", "Nigéria"),
        ("sl", "Nigerija"),
        ("so", "Nayjeeriya"),
        ("sq", "Nigeri"),
        ("sr", "Нигерија"),
        ("sv", "Nigeria"),
        ("sw", "Nigeria"),
        ("ta", "நைஜ\u{bc0}ரிய\u{bbe}"),
        ("te", "న\u{c48}జ\u{c40}ర\u{c3f}య\u{c3e}"),
        ("tg", "Нигерия"),
        ("th", "ไนจ\u{e35}เร\u{e35}ย"),
        ("ti", "ናይጄሪያ"),
        ("tk", "Nigeriýa"),
        ("tl", "Nigeria"),
        ("tr", "Nijerya"),
        ("tt", "Ниgериа"),
        ("ug", "نىگېرىيە"),
        ("uk", "Нігерія"),
        ("ur", "نائجیریا"),
        ("uz", "Nigeriya"),
        ("ve", "Nigeria"),
        ("vi", "Ni-giê-ri-a"),
        ("wa", "Nidjeria"),
        ("wo", "Nijeeria"),
        ("xh", "Nigeria"),
        ("yo", "Nàìjíríà"),
        ("zh_CN", "尼日利亚"),
        ("zh_HK", "尼日利亞"),
        ("zh_TW", "奈及利亞"),
        ("zu", "INigeria"),
    ];
    #[cfg(all(feature = "ng", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 9.081999;
        pub const LONGITUDE: f64 = 8.675277;
        pub const MAX_LATITUDE: f64 = 13.8856449;
        pub const MAX_LONGITUDE: f64 = 14.677982;
        pub const MIN_LATITUDE: f64 = 4.1821001;
        pub const MIN_LONGITUDE: f64 = 2.676932;
        pub const NORTHEAST_LATITUDE: f64 = 13.8856449;
        pub const NORTHEAST_LONGITUDE: f64 = 14.677982;
        pub const SOUTHWEST_LATITUDE: f64 = 4.1821001;
        pub const SOUTHWEST_LONGITUDE: f64 = 2.676932;
    }
}
#[cfg(all(feature = "ng", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 9.081999,
            longitude: 8.675277,
            max_latitude: 13.8856449,
            max_longitude: 14.677982,
            min_latitude: 4.1821001,
            min_longitude: 2.676932,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 13.8856449,
                    longitude: 14.677982,
                },
                southwest: CountryGeoBound {
                    latitude: 4.1821001,
                    longitude: 2.676932,
                },
            },
        }
    }
}

#[cfg(all(feature = "ng", feature = "subdivisions"))]
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
                    "AB",
                    Subdivision{
                        name: "AB",
                        country_alpha2: Alpha2::NG,
                        code: "AB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(5.430892099999999), longitude: Some(7.524724300000001), max_latitude: Some(6.0191921), min_latitude: Some(4.810874), max_longitude: Some(7.9630091), min_longitude: Some(7.150823)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية أبيا"), ("bg", "Абия"), ("bn", "আবিয\u{9bc}\u{9be} অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Abia"), ("ccp", "𑄃𑄝\u{11128}𑄠"), ("ceb", "Abia State"), ("da", "Abia"), ("de", "Abia"), ("el", "Αμπία"), ("en", "Abia"), ("es", "Abia (estado)"), ("et", "Abia osariik"), ("fa", "اراضی ابیا"), ("fi", "Abia"), ("fr", "État d’Abia"), ("gl", "Estado de Abia"), ("gu", "અબિયા સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Abiya"), ("ha_NE", "Abiya"), ("hi", "आबिया राज\u{94d}य"), ("hu", "Abia állam"), ("id", "Abia (negara bagian Nigeria)"), ("ig", "Ȯra Abia"), ("it", "Abia"), ("ja", "アビア州"), ("ka", "აბიის შტატი"), ("kn", "ಅಬ\u{cbf}ಯಾ ರಾಜ\u{ccd}ಯ"), ("ko", "아비아 주"), ("lt", "Abija"), ("lv", "Abija"), ("mr", "अबिया राज\u{94d}य"), ("ms", "Abia State"), ("nb", "Abia"), ("nl", "Abia"), ("no", "Abia"), ("pl", "Abia (stan)"), ("pt", "Abia (estado)"), ("ro", "Abia"), ("ru", "Абия (штат)"), ("si", "අබ\u{dd2}ය\u{dcf} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Абија"), ("sr_Latn", "Abija"), ("sv", "Abia"), ("sw", "Abia (jimbo)"), ("ta", "அபிய\u{bbe} ம\u{bbe}நிலம\u{bcd}"), ("te", "ఆబ\u{c3f}య\u{c3e} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "ร\u{e31}ฐอาเบ\u{e35}ย"), ("tr", "Abia Eyaleti"), ("uk", "Абія"), ("ur", "ابیا ریاست"), ("vi", "Bang Abia"), ("yo", "Ìpínlẹ\u{300} Ábíá"), ("yo_BJ", "Ìpínlɛ\u{300} Ábíá"), ("zh", "阿比亚州")]),
                        unofficial_name_list: ["Abia"].to_vec(),
                    }
                ),
                (
                    "AD",
                    Subdivision{
                        name: "AD",
                        country_alpha2: Alpha2::NG,
                        code: "AD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.3250497), longitude: Some(12.4380581), max_latitude: Some(10.943588), min_latitude: Some(7.452592), max_longitude: Some(13.6924919), min_longitude: Some(11.392936)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية آدماوة"), ("bg", "Адамауа"), ("bn", "আড\u{9be}ম\u{9be}ওয\u{9bc}\u{9be} অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Adamaua"), ("ccp", "𑄃𑄓𑄟\u{11134}𑄤"), ("ceb", "Adamawa State"), ("da", "Adamawa"), ("de", "Adamawa"), ("el", "Ανταμάβα"), ("en", "Adamawa"), ("es", "Adamawa"), ("et", "Adamawa osariik"), ("fa", "اراضی اداماوا"), ("fi", "Adamawa"), ("fr", "État d’Adamawa"), ("gl", "Adamawa"), ("gu", "અદામાવા સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Adamawa"), ("ha_NE", "Adamawa"), ("hi", "अदामावा राज\u{94d}य"), ("id", "Adamawa"), ("ig", "Ȯra Adamawa"), ("it", "Adamawa"), ("ja", "アダマワ州"), ("ka", "ადამავის შტატი"), ("kn", "ಆದಾಮಾ ರಾಜ\u{ccd}ಯ"), ("ko", "아다마와 주"), ("lt", "Adamava"), ("lv", "Adamavas štats"), ("mr", "आदामावा राज\u{94d}य"), ("ms", "Adamawa State"), ("nb", "Adamawa"), ("nl", "Adamawa"), ("no", "Adamawa"), ("pl", "Adamawa"), ("pt", "Adamawa"), ("ro", "Statul Adamawa"), ("ru", "Адамава"), ("si", "අඩම\u{dcf}ව\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sr", "Адамава"), ("sr_Latn", "Adamava"), ("sv", "Adamawa"), ("sw", "Adamawa"), ("ta", "அடமவ\u{bbe} ம\u{bbe}நிலம\u{bcd}"), ("te", "ఆడమ\u{c3e}వ\u{c3e} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "ร\u{e31}ฐอาดามาวา"), ("tr", "Adamawa Eyaleti"), ("uk", "Адамава"), ("ur", "اداماوا ریاست"), ("vi", "Bang Adamawa"), ("yo", "Ìpínlẹ\u{300} Adámáwá"), ("yo_BJ", "Ìpínlɛ\u{300} Adámáwá"), ("zh", "阿達馬瓦")]),
                        unofficial_name_list: ["Adamaoua", "Gongola"].to_vec(),
                    }
                ),
                (
                    "AK",
                    Subdivision{
                        name: "AK",
                        country_alpha2: Alpha2::NG,
                        code: "AK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(4.929986899999999), longitude: Some(7.872159999999999), max_latitude: Some(5.530245), min_latitude: Some(4.4780049), max_longitude: Some(8.340371), min_longitude: Some(7.4605409)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية اكوا ايبوم"), ("bg", "Аква Ибом"), ("bn", "আকি\u{9be}উ আইব\u{9c1}ম অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Akwa Ibom"), ("ccp", "𑄃\u{11127}𑄇\u{11134}𑄤 𑄃\u{11128}𑄝\u{11127}𑄟\u{11134}"), ("ceb", "Akwa Ibom State"), ("da", "Akwa Ibom"), ("de", "Akwa Ibom"), ("el", "Όκβα Ίμπορν"), ("en", "Akwa Ibom"), ("es", "Akwa Ibom"), ("et", "Akwa Ibomi osariik"), ("fa", "اکوا ایبوم استیت"), ("fi", "Akwa Ibom"), ("fr", "État d’Akwa Ibom"), ("gl", "Akwa Ibom"), ("gu", "અકવા ઇબોમ સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Akwa Ibom"), ("ha_NE", "Akwa Ibom"), ("hi", "अक\u{94d}वा इबोम राज\u{94d}य"), ("id", "Akwa Ibom"), ("ig", "Ȯra Akwa Ibom"), ("it", "Akwa Ibom"), ("ja", "アクワ・イボム州"), ("ka", "აკვა-იბომის შტატი"), ("kn", "ಅಕ\u{ccd}ವಾ ಇಬೊಮ\u{ccd} ರಾಜ\u{ccd}ಯ"), ("ko", "아콰이봄 주"), ("lt", "Ava Ibomo valstija"), ("lv", "Akvas Ibomas štats"), ("mr", "अक\u{94d}वा इबोम राज\u{94d}य"), ("ms", "Negeri Akwa Ibom"), ("nb", "Akwa Ibom"), ("nl", "Akwa Ibom"), ("no", "Akwa Ibom"), ("pl", "Akwa Ibom"), ("pt", "Akwa Ibom (estado)"), ("ro", "Statul Akwa Ibom"), ("ru", "Аква-Ибом"), ("si", "අක\u{dca}ව\u{dcf} ඉබොම\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sr", "Аква Ибом"), ("sr_Latn", "Akva Ibom"), ("sv", "Akwa Ibom"), ("sw", "Akwa Ibom (jimbo)"), ("ta", "அகவ இபோம\u{bcd} ம\u{bbe}நிலம\u{bcd}"), ("te", "అక\u{c4d}వ\u{c3e} ఐబ\u{c4b}మ\u{c4d} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "ร\u{e31}ฐอะกวาอ\u{e34}บอม"), ("tr", "Akwa Ibom Eyaleti"), ("uk", "Аква-Ібом"), ("ur", "اکوا ایبوم ریاست"), ("vi", "Bang Akwa Ibom"), ("yo", "Ìpínlẹ\u{300} Akwa Íbọm"), ("yo_BJ", "Ìpínlɛ\u{300} Akwa Íbɔm"), ("zh", "阿夸伊博姆州")]),
                        unofficial_name_list: ["Akwa Ibom"].to_vec(),
                    }
                ),
                (
                    "AN",
                    Subdivision{
                        name: "AN",
                        country_alpha2: Alpha2::NG,
                        code: "AN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.2757656), longitude: Some(7.0068393), max_latitude: Some(6.7795999), min_latitude: Some(5.692615000000001), max_longitude: Some(7.355934), min_longitude: Some(6.613086)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية أنمبرة"), ("bg", "Анамбра"), ("bn", "আন\u{9be}ম\u{9cd}ব\u{9cd}র\u{9be} অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Anambra"), ("ccp", "𑄃𑄚𑄟\u{11134}𑄝\u{11133}𑄢"), ("ceb", "Anambra State"), ("da", "Anambra"), ("de", "Anambra"), ("el", "Ανάμπρα"), ("en", "Anambra"), ("es", "Anambra"), ("et", "Anambra osariik"), ("fa", "انامبرا استیت"), ("fi", "Anambra"), ("fr", "État d’Anambra"), ("gl", "Anambra"), ("gu", "અનામ\u{acd}બ\u{acd}રા સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Anambra"), ("ha_NE", "Anambra"), ("hi", "अनम\u{94d}ब\u{94d}रा राज\u{94d}य"), ("id", "Anambra"), ("ig", "Ȯra Anambra"), ("it", "Anambra"), ("ja", "アナンブラ州"), ("ka", "ანამბრის შტატი"), ("kn", "ಅನಂಬ\u{ccd}ರಾ ರಾಜ\u{ccd}ಯ"), ("ko", "아남브라 주"), ("lt", "Anambros valstija"), ("lv", "Anambras štats"), ("mr", "अ\u{902}\u{902}न\u{94d}याब\u{94d}रा राज\u{94d}य"), ("ms", "Anambra State"), ("nb", "Anambra"), ("nl", "Anambra"), ("no", "Anambra"), ("pl", "Anambra"), ("pt", "Anambra (estado)"), ("ro", "Statul Anambra"), ("ru", "Анамбра"), ("si", "අනම\u{dca}බ\u{dca}\u{200d}ර\u{dcf} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Анамбра"), ("sr_Latn", "Anambra"), ("sv", "Anambra"), ("sw", "Jimbo la Anambra"), ("ta", "அநம\u{bcd}பற\u{bbe} ம\u{bbe}நிலம\u{bcd}"), ("te", "అన\u{c3e}ంబ\u{c3e} ర\u{c3e}ష\u{c4d}ట\u{c4d}రం"), ("th", "ร\u{e31}ฐอะน\u{e31}มบรา"), ("tr", "Anambra Eyaleti"), ("uk", "Анамбра"), ("ur", "انامبرا ریاست"), ("vi", "Bang Anambra"), ("yo", "Ìpínlẹ\u{300} Anámbra"), ("yo_BJ", "Ìpínlɛ\u{300} Anámbra"), ("yue", "阿南布拉州"), ("yue_Hans", "阿南布拉州"), ("zh", "阿南布拉州")]),
                        unofficial_name_list: ["Anambra"].to_vec(),
                    }
                ),
                (
                    "BA",
                    Subdivision{
                        name: "BA",
                        country_alpha2: Alpha2::NG,
                        code: "BA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.315833), longitude: Some(9.844166999999999), max_latitude: Some(10.3398292), min_latitude: Some(10.255381), max_longitude: Some(9.864006), min_longitude: Some(9.779892)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية باوتشي"), ("bg", "Баучи"), ("bn", "ব\u{9be}উচি অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat de Bauchi"), ("ccp", "𑄝\u{1112f}𑄌\u{11128}"), ("ceb", "Bauchi State"), ("da", "Bauchi"), ("de", "Bauchi"), ("el", "Μποτσί"), ("en", "Bauchi"), ("es", "Bauchi"), ("et", "Bauchi osariik"), ("fa", "ایالت باوچی"), ("fi", "Bauchi"), ("fr", "État de Bauchi"), ("gl", "Estado de Bauchi"), ("gu", "બાઉચી સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Bauchi"), ("ha_NE", "Bauchi"), ("hi", "बाउची राज\u{94d}य"), ("id", "Bauchi"), ("ig", "Ȯra Bauchi"), ("is", "Bauchi-fylki"), ("it", "Bauchi"), ("ja", "バウチ州"), ("ka", "ბაუჩის შტატი"), ("kn", "ಬಾಚು ರಾಜ\u{ccd}ಯ"), ("ko", "바우치 주"), ("lt", "Baučio valstija"), ("lv", "Bauči štats"), ("mr", "बाऊची राज\u{94d}य"), ("ms", "Bauchi State"), ("nb", "Bauchi"), ("nl", "Bauchi"), ("no", "Bauchi"), ("pl", "Bauczi"), ("pt", "Bauchi"), ("ro", "Statul Bauchi"), ("ru", "Баучи"), ("si", "බව\u{dd4}ච\u{dd2} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Баучи"), ("sr_Latn", "Bauči"), ("sv", "Bauchi"), ("sw", "Jimbo la Bauchi"), ("ta", "ப\u{bbe}வ\u{bcd}ச\u{bcd}சி ம\u{bbe}நிலம\u{bcd}"), ("te", "బ\u{c4c}చ\u{c3f} ర\u{c3e}ష\u{c4d}ట\u{c4d}రం"), ("th", "ร\u{e31}ฐเบาช\u{e35}"), ("tr", "Bauchi Eyaleti"), ("uk", "Баучі"), ("ur", "باوچی ریاست"), ("vi", "Bang Bauchi"), ("yo", "Ìpínlẹ\u{300} Bauchi"), ("yo_BJ", "Ìpínlɛ\u{300} Bauchi"), ("zh", "包奇州")]),
                        unofficial_name_list: ["Bauchi"].to_vec(),
                    }
                ),
                (
                    "BE",
                    Subdivision{
                        name: "BE",
                        country_alpha2: Alpha2::NG,
                        code: "BE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.350825899999999), longitude: Some(8.8362755), max_latitude: Some(8.149299), min_latitude: Some(6.4427789), max_longitude: Some(9.918301), min_longitude: Some(7.4893301)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية بينو"), ("bg", "Бенуе"), ("bn", "বেন\u{9c1} র\u{9be}জ\u{9cd}য"), ("ca", "Estat de Benue"), ("ccp", "𑄝𑄬𑄚\u{1112a}𑄠\u{11128}"), ("ceb", "Benue State"), ("da", "Benue"), ("de", "Benue"), ("el", "Μπένιου"), ("en", "Benue"), ("es", "Benue"), ("et", "Benue osariik"), ("fa", "ایالت بنوئه"), ("fi", "Benue"), ("fr", "État de Benue"), ("gl", "Estado de Benue"), ("gu", "બ\u{ac7}ન\u{acd}ય\u{ac1} સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Benue"), ("ha_NE", "Benue"), ("hi", "ब\u{947}न\u{941}ए राज\u{94d}य"), ("id", "Benue"), ("ig", "Ȯra Benue"), ("it", "Benue"), ("ja", "ベヌエ州"), ("ka", "ბენუეს შტატი"), ("kn", "ಬ\u{cc6}ನ\u{ccd}ಯು ರಾಜ\u{ccd}ಯ"), ("ko", "베누에 주"), ("lt", "Benujės valstija"), ("lv", "Benue štats"), ("mr", "ब\u{947}न\u{942} राज\u{94d}य"), ("ms", "Benue State"), ("nb", "Benue"), ("nl", "Benue"), ("no", "Benue"), ("pl", "Benue"), ("pt", "Benue"), ("ro", "Statul Benue"), ("ru", "Бенуэ"), ("si", "බෙන\u{dd4}ය\u{dd2} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Бенуе"), ("sr_Latn", "Benue"), ("sv", "Benue"), ("sw", "Jimbo la Benue"), ("ta", "பேணுவே ம\u{bbe}நிலம\u{bcd}"), ("te", "బ\u{c46}న\u{c4d}యూ ర\u{c3e}ష\u{c4d}ట\u{c4d}రం"), ("th", "เบน\u{e34}ว"), ("tr", "Benue Eyaleti"), ("uk", "Бенуе"), ("ur", "بینوے ریاست"), ("vi", "Bang Benue"), ("yo", "Ìpínlẹ\u{300} Bẹ\u{301}núé"), ("yo_BJ", "Ìpínlɛ\u{300} Bɛ\u{301}núé"), ("zh", "贝努埃州")]),
                        unofficial_name_list: ["Benue"].to_vec(),
                    }
                ),
                (
                    "BO",
                    Subdivision{
                        name: "BO",
                        country_alpha2: Alpha2::NG,
                        code: "BO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.5097479), longitude: Some(12.9789121), max_latitude: Some(13.7144441), min_latitude: Some(10.0291549), max_longitude: Some(14.680073), min_longitude: Some(11.6286559)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية بورنو"), ("bg", "Борно"), ("bn", "বর\u{9cd}নো অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat de Borno"), ("ccp", "𑄝\u{11127}𑄢\u{11134}𑄚\u{1112e}"), ("ceb", "Borno State"), ("cs", "Borno"), ("da", "Borno"), ("de", "Borno"), ("el", "Μπόρνο"), ("en", "Borno"), ("es", "Borno"), ("et", "Borno"), ("eu", "Borno"), ("fa", "ایالت بورنو"), ("fi", "Borno"), ("fr", "État de Borno"), ("gl", "Estado de Borno"), ("gu", "બોર\u{acd}નો સ\u{acd}ટ\u{ac7}ટ"), ("ha", "jihar Borno"), ("ha_NE", "jihar Borno"), ("he", "בורנו"), ("hi", "बोरनो राज\u{94d}य"), ("id", "Borno"), ("ig", "Ȯra Borno"), ("it", "Borno"), ("ja", "ボルノ州"), ("ka", "ბორნოს შტატი"), ("kn", "ಬೊರ\u{ccd}ನೊ ರಾಜ\u{ccd}ಯ"), ("ko", "보르노 주"), ("lt", "Borno valstija"), ("lv", "Borno štats"), ("mk", "Борно"), ("mr", "बोर\u{94d}नो राज\u{94d}य"), ("ms", "Borno State"), ("nb", "Borno"), ("nl", "Borno"), ("no", "Borno"), ("pl", "Borno"), ("pt", "Borno"), ("ro", "Statul Borno"), ("ru", "Борно"), ("si", "බොර\u{dca}නෝ ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Борно"), ("sr_Latn", "Borno"), ("sv", "Borno"), ("sw", "Borno"), ("ta", "போர\u{bcd}னோ ம\u{bbe}நிலம\u{bcd}"), ("te", "బ\u{c4b}ర\u{c4d}న\u{c4b} ర\u{c3e}ష\u{c4d}ట\u{c4d}రం"), ("th", "ร\u{e31}ฐโบร\u{e4c}โน"), ("tr", "Borno Eyaleti"), ("uk", "Борно"), ("ur", "بورنو ریاست"), ("vi", "Borno"), ("yo", "Ìpínlẹ\u{300} Bọ\u{300}rnó"), ("yo_BJ", "Ìpínlɛ\u{300} Bɔ\u{300}rnó"), ("zh", "博尔诺州")]),
                        unofficial_name_list: ["Borno"].to_vec(),
                    }
                ),
                (
                    "BY",
                    Subdivision{
                        name: "BY",
                        country_alpha2: Alpha2::NG,
                        code: "BY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(4.867776699999999), longitude: Some(5.898713900000001), max_latitude: Some(5.3933139), min_latitude: Some(4.2771439), max_longitude: Some(6.724865899999999), min_longitude: Some(5.370120500000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية بايلسا"), ("bg", "Байелса"), ("bn", "বেলস\u{9be} অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Bayelsa"), ("ccp", "𑄝𑄬𑄠𑄬𑄣\u{11134}𑄥"), ("ceb", "Bayelsa State"), ("da", "Bayelsa"), ("de", "Bayelsa"), ("el", "Μπαγιέλσα"), ("en", "Bayelsa"), ("es", "Bayelsa"), ("et", "Bayelsa"), ("fa", "ایالت بایلسا"), ("fi", "Bayelsa"), ("fr", "État de Bayelsa"), ("gl", "Bayelsa"), ("gu", "બાયલ\u{acd}સા સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Bayelsa"), ("ha_NE", "Bayelsa"), ("hi", "बाय\u{947}ल\u{94d}सा राज\u{94d}य"), ("id", "Bayelsa"), ("ig", "Ȯra Bayelsa"), ("it", "Bayelsa"), ("ja", "バイエルサ州"), ("ka", "ბაიელსის შტატი"), ("kn", "ಬೇಯ\u{cc6}ಲ\u{ccd}ಸಾ ರಾಜ\u{ccd}ಯ"), ("ko", "바이엘사 주"), ("lt", "Bajelsos valstija"), ("lv", "Bajelsas štats"), ("mr", "बायल\u{94d}स राज\u{94d}य"), ("ms", "Bayelsa State"), ("nb", "Bayelsa"), ("nl", "Bayelsa"), ("no", "Bayelsa"), ("pl", "Bayelsa"), ("pt", "Bayelsa"), ("ro", "Statul Bayelsa"), ("ru", "Байельса"), ("si", "බයෙල\u{dca}ස\u{dcf} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Бајелса"), ("sr_Latn", "Bajelsa"), ("sv", "Bayelsa"), ("sw", "Jimbo la Bayelsa"), ("ta", "பஎல\u{bcd}ஸ\u{bbe} ம\u{bbe}நிலம\u{bcd}"), ("te", "బ\u{c47}య\u{c46}ల\u{c4d}స\u{c3e} ర\u{c3e}ష\u{c4d}ట\u{c4d}రం"), ("th", "เขตหงสาวด\u{e35}"), ("tr", "Bayelsa Eyaleti"), ("uk", "Байельса"), ("ur", "بایلسا ریاست"), ("vi", "Bang Bayelsa"), ("yo", "Ìpínlẹ\u{300} Bàyélsà"), ("yo_BJ", "Ìpínlɛ\u{300} Bàyélsà"), ("zh", "巴耶尔萨州")]),
                        unofficial_name_list: ["Bayelsa"].to_vec(),
                    }
                ),
                (
                    "CR",
                    Subdivision{
                        name: "CR",
                        country_alpha2: Alpha2::NG,
                        code: "CR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.167031499999999), longitude: Some(8.6600586), max_latitude: Some(6.899680999999999), min_latitude: Some(4.690596000000001), max_longitude: Some(9.472486), min_longitude: Some(7.863165999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية كروس ريفر"), ("bg", "Крос Ривър"), ("bn", "ক\u{9cd}রস রিভ\u{9be}র অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Cross River"), ("ccp", "𑄇\u{11133}𑄢\u{11127}𑄌\u{11134} 𑄢\u{11128}𑄞𑄢\u{11134}"), ("ceb", "Cross River State"), ("da", "Cross River"), ("de", "Cross River"), ("el", "Κρος Ρίβερ"), ("en", "Cross River"), ("es", "Cross River"), ("et", "Cross Riveri osariik"), ("fa", "ایالت کراس ریور"), ("fi", "Cross River"), ("fr", "État de Cross River"), ("gl", "Cross River"), ("gu", "ક\u{acd}રોસ રિવર સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Cross River"), ("ha_NE", "Cross River"), ("hi", "क\u{94d}रॉस रिवर राज\u{94d}य"), ("id", "Cross River"), ("ig", "Ȯra Cross River"), ("it", "Cross River"), ("ja", "クロスリバー州"), ("ka", "კროს-რივერის შტატი"), ("kn", "ಕ\u{ccd}ರಾಸ\u{ccd} ರ\u{cbf}ವರ\u{ccd} ಸ\u{ccd}ಟೇಟ\u{ccd}"), ("ko", "크로스리버 주"), ("lt", "Kros Riverso valstija"), ("lv", "Krosriveras štats"), ("mr", "क\u{94d}रॉस रिवर राज\u{94d}य"), ("ms", "Cross River State"), ("nb", "Cross River"), ("nl", "Cross River"), ("no", "Cross River"), ("pl", "Cross River"), ("pt", "Cross River"), ("ro", "Statul Cross River"), ("ru", "Кросс-Ривер"), ("si", "ක\u{dca}\u{200d}රොස\u{dca} ර\u{dd2}වර\u{dca} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Крос Ривер"), ("sr_Latn", "Kros River"), ("sv", "Cross River"), ("sw", "Jimbo la Cross River"), ("ta", "கிர\u{bbe}ஸ\u{bcd} ரிவேர\u{bcd} ம\u{bbe}நிலம\u{bcd}"), ("te", "క\u{c4d}ర\u{c3e}స\u{c4d} ర\u{c3f}వర\u{c4d} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "แม\u{e48}น\u{e49}ำครอส"), ("tr", "Cross River Eyaleti"), ("uk", "Крос-Рівер"), ("ur", "کراس ریور ریاست"), ("vi", "Bang Cross River"), ("yo", "Ìpínlẹ\u{300} Cross River"), ("yo_BJ", "Ìpínlɛ\u{300} Cross River"), ("zh", "克里斯河州")]),
                        unofficial_name_list: ["Cross River"].to_vec(),
                    }
                ),
                (
                    "DE",
                    Subdivision{
                        name: "DE",
                        country_alpha2: Alpha2::NG,
                        code: "DE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(5.5324624), longitude: Some(5.898713900000001), max_latitude: Some(6.5248195), min_latitude: Some(5.024351999999999), max_longitude: Some(6.7653911), min_longitude: Some(5.000000099999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية دلتا"), ("be", "штат Дэльта"), ("bg", "Делта"), ("bn", "ডেল\u{9cd}ট\u{9be} অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat del Delta"), ("ccp", "𑄓𑄬𑄣\u{11134}𑄑"), ("ceb", "Delta State"), ("da", "Delta"), ("de", "Delta"), ("el", "Δέλτα (Ντέλτα)"), ("en", "Delta"), ("es", "Delta"), ("et", "Delta osariik"), ("fa", "ایالت دلتا"), ("fi", "Delta"), ("fr", "état du Delta"), ("gl", "Estado do Delta"), ("gu", "ડ\u{ac7}લ\u{acd}ટા સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Delta"), ("ha_NE", "Delta"), ("he", "מדינת דלתא"), ("hi", "ड\u{947}ल\u{94d}टा राज\u{94d}य"), ("hy", "Դելտա"), ("id", "Delta"), ("ig", "Ȯra Delta"), ("it", "Delta"), ("ja", "デルタ州"), ("ka", "დელტის შტატი"), ("kn", "ಡ\u{cc6}ಲ\u{ccd}ಟಾ ರಾಜ\u{ccd}ಯ"), ("ko", "델타 주"), ("lt", "Deltos valstija"), ("lv", "Deltas štats"), ("mr", "ड\u{947}ल\u{94d}टा राज\u{94d}य"), ("ms", "Delta State"), ("nb", "Delta"), ("nl", "Delta"), ("no", "Delta"), ("pl", "Delta"), ("pt", "Delta"), ("ro", "Statul Delta"), ("ru", "Дельта"), ("si", "ඩෙල\u{dca}ට\u{dcf} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Делта"), ("sr_Latn", "Delta"), ("sv", "Delta"), ("sw", "Delta"), ("ta", "டெல\u{bcd}ட\u{bbe} ம\u{bbe}நிலம\u{bcd}"), ("te", "డ\u{c46}ల\u{c4d}ట\u{c3e} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "เดลทา สเตจ"), ("tr", "Delta Eyaleti"), ("uk", "Дельта"), ("ur", "ڈیلٹا ریاست"), ("vi", "Bang Delta"), ("yo", "Ìpínlẹ\u{300} Dẹ\u{301}ltà"), ("yo_BJ", "Ìpínlɛ\u{300} Dɛ\u{301}ltà"), ("zh", "三角州")]),
                        unofficial_name_list: ["Delta"].to_vec(),
                    }
                ),
                (
                    "EB",
                    Subdivision{
                        name: "EB",
                        country_alpha2: Alpha2::NG,
                        code: "EB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.177973), longitude: Some(7.959286299999999), max_latitude: Some(6.807093), min_latitude: Some(5.6873539), max_longitude: Some(8.4470269), min_longitude: Some(7.5251131)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية إبونيه"), ("bg", "Ебони"), ("bn", "ইবনি অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Ebonyi"), ("ccp", "𑄃\u{11128}𑄝\u{11127}𑄚\u{11128}"), ("ceb", "Ebonyi State"), ("da", "Ebonyi"), ("de", "Ebonyi"), ("el", "Εμπόνι"), ("en", "Ebonyi"), ("es", "Ebonyi"), ("et", "Ebonyi osariik"), ("fa", "ایالت ابونیی"), ("fi", "Ebonyi"), ("fr", "État d’Ebonyi"), ("gl", "Estado de Ebonyi"), ("gu", "ઇબોની સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Ebonyi"), ("ha_NE", "Ebonyi"), ("hi", "एबोन\u{94d}यी राज\u{94d}य"), ("id", "Ebonyi"), ("ig", "Ȯra Ebonyi"), ("it", "Ebonyi"), ("ja", "エボニ州"), ("ka", "ებონიის შტატი"), ("kn", "ಎಬೊನ\u{cbf} ರಾಜ\u{ccd}ಯ"), ("ko", "에보니 주"), ("lt", "Ebondžo valstija"), ("lv", "Eboņi štats"), ("mr", "एबोनाई राज\u{94d}य"), ("ms", "Negeri Ebonyi"), ("nb", "Ebonyi"), ("nl", "Ebonyi"), ("no", "Ebonyi"), ("pl", "Ebonyi"), ("pt", "Ebonyi"), ("ro", "Statul Ebonyi"), ("ru", "Эбоньи"), ("si", "එබෝන\u{dd2} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Ебоњи"), ("sr_Latn", "Ebonji"), ("sv", "Ebonyi"), ("sw", "Jimbo la Ebonyi"), ("ta", "எபோனியி ம\u{bbe}நிலம\u{bcd}"), ("te", "ఇబ\u{c4b}న\u{c4d}య\u{c40} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "อ\u{e35}บอนย\u{e35}"), ("tr", "Ebonyi Eyaleti"), ("uk", "Ебоньі"), ("ur", "عبونئی ریاست"), ("vi", "Bang Ebonyi"), ("zh", "埃邦伊州")]),
                        unofficial_name_list: ["Ebonyi"].to_vec(),
                    }
                ),
                (
                    "ED",
                    Subdivision{
                        name: "ED",
                        country_alpha2: Alpha2::NG,
                        code: "ED",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.5438101), longitude: Some(5.898713900000001), max_latitude: Some(7.5721479), min_latitude: Some(5.7386799), max_longitude: Some(6.707891000000001), min_longitude: Some(4.975229)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية إدو"), ("be", "штат Эда"), ("bg", "Едо"), ("bn", "এদো অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat Edo"), ("ccp", "𑄃\u{11128}𑄓\u{1112e}"), ("ceb", "Edo"), ("da", "Edo"), ("de", "Edo"), ("el", "Έντο Στέιτ"), ("en", "Edo"), ("es", "Edo"), ("et", "Edo osariik"), ("fa", "ایالت ادو"), ("fi", "Edo"), ("fr", "état d’Edo"), ("gl", "Estado de Edo"), ("gu", "ઇડો સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Edo"), ("ha_NE", "Edo"), ("hi", "एडो राज\u{94d}य"), ("id", "Edo"), ("ig", "Ȯra Edo"), ("is", "Edo-fylki"), ("it", "Edo"), ("ja", "エド州"), ("ka", "ედოს შტატი"), ("kn", "ಎಡೊ ರಾಜ\u{ccd}ಯ"), ("ko", "에도 주"), ("lt", "Edo valstija"), ("lv", "Edo štats"), ("mr", "ईदो राज\u{94d}य"), ("ms", "Edo State"), ("nb", "Edo"), ("nl", "Edo"), ("no", "Edo"), ("pl", "Edo"), ("pt", "Edo"), ("ro", "Statul Edo"), ("ru", "Эдо"), ("si", "එඩෝ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sr", "Едо"), ("sr_Latn", "Edo"), ("sv", "Edo"), ("sw", "Jimbo la Edo"), ("ta", "எடோ ம\u{bbe}நிலம\u{bcd}"), ("te", "ఎడ\u{c4b} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "ร\u{e31}ฐอ\u{e35}โด"), ("tr", "Edo Eyaleti"), ("uk", "Едо"), ("ur", "عدو ریاست"), ("vi", "Bang Edo"), ("yo", "Ìpínlẹ\u{300} Ẹdó"), ("yo_BJ", "Ìpínlɛ\u{300} Ɛdó"), ("zh", "埃多州")]),
                        unofficial_name_list: ["Bendel"].to_vec(),
                    }
                ),
                (
                    "EK",
                    Subdivision{
                        name: "EK",
                        country_alpha2: Alpha2::NG,
                        code: "EK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.665581299999999), longitude: Some(5.3102505), max_latitude: Some(8.0674911), min_latitude: Some(7.272215999999999), max_longitude: Some(5.8048959), min_longitude: Some(4.9103081)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية إكيتي"), ("bg", "Екити"), ("bn", "একি অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat Ekiti"), ("ccp", "𑄃\u{11128}𑄇\u{11128}𑄑\u{11128}"), ("ceb", "Ekiti State"), ("da", "Ekiti"), ("de", "Ekiti"), ("el", "Εκίτι"), ("en", "Ekiti"), ("es", "Ekiti"), ("et", "Ekiti osariik"), ("fa", "ایالت اکیتی"), ("fi", "Ekiti"), ("fr", "État d’Ekiti"), ("gl", "Estado de Ekiti"), ("gu", "એકિતી સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Ekiti"), ("ha_NE", "Ekiti"), ("hi", "एकिटि राज\u{94d}य"), ("id", "Ekiti"), ("ig", "Ȯra Ekiti"), ("it", "Ekiti"), ("ja", "エキティ州"), ("ka", "ეკიტის შტატი"), ("kn", "ಏಕ\u{cbf}ಟ\u{cbf} ರಾಜ\u{ccd}ಯ"), ("ko", "에키티 주"), ("lt", "Ekičio valstija"), ("lv", "Ekiti štats"), ("mr", "इकिती राज\u{94d}य"), ("ms", "Negeri Ekiti"), ("nb", "Ekiti"), ("nl", "Ekiti"), ("no", "Ekiti"), ("pl", "Ekiti"), ("pt", "Ekiti"), ("ro", "Statul Ekiti"), ("ru", "Экити"), ("si", "එක\u{dd2}ට\u{dd2} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Екити"), ("sr_Latn", "Ekiti"), ("sv", "Ekiti"), ("sw", "Jimbo la Ekiti"), ("ta", "எகிடி ம\u{bbe}நிலம\u{bcd}"), ("te", "ఇక\u{c3f}ట\u{c3f} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "เอกก\u{e34}ต\u{e34} สเตจ"), ("tr", "Ekiti Eyaleti"), ("uk", "Екіті"), ("ur", "عکیتی ریاست"), ("vi", "Bang Ekiti"), ("yo", "Ìpínlẹ\u{300} Èkìtì"), ("yo_BJ", "Ìpínlɛ\u{300} Èkìtì"), ("zh", "埃基蒂州")]),
                        unofficial_name_list: ["Ekiti"].to_vec(),
                    }
                ),
                (
                    "EN",
                    Subdivision{
                        name: "EN",
                        country_alpha2: Alpha2::NG,
                        code: "EN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.452667), longitude: Some(7.510332999999998), max_latitude: Some(6.5155669), min_latitude: Some(6.360852), max_longitude: Some(7.618160199999999), min_longitude: Some(7.458772700000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية إينوغو"), ("be", "Энугу"), ("bg", "Енугу"), ("bn", "এন\u{9be}\u{9c1}গ\u{9c1} অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat d’Enugu"), ("ccp", "𑄃𑄬𑄚\u{1112a}𑄉\u{1112a}"), ("ceb", "Enugu (kapital sa estado sa Nigeria)"), ("da", "Enugu (delstat)"), ("de", "Enugu"), ("el", "Ενούγκου"), ("en", "Enugu"), ("es", "Enugu"), ("et", "Enugu osariik"), ("fa", "ایالت انوگو"), ("fi", "Enugu"), ("fr", "État d’Enugu"), ("gl", "Estado de Enugu"), ("gu", "એન\u{ac1}ગ\u{ac1} સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Enugu"), ("ha_NE", "Enugu"), ("hi", "एन\u{941}ग\u{941} राज\u{94d}य"), ("id", "Enugu"), ("ig", "Ȯra Enugu"), ("it", "Enugu"), ("ja", "エヌグ州"), ("ka", "ენუგუს შტატი"), ("kn", "ಎನ\u{ccd}ಯುಗು ರಾಜ\u{ccd}ಯ"), ("ko", "에누구 주"), ("lt", "Enugo valstija"), ("lv", "Enugu štats"), ("mr", "एन\u{941}ग\u{941} राज\u{94d}य"), ("ms", "Enugu State"), ("nb", "Enugu"), ("nl", "Enugu"), ("no", "Enugu"), ("pl", "Enugu"), ("pt", "Enugu"), ("ro", "Statul Enugu"), ("ru", "Энугу"), ("si", "එන\u{dd4}ග\u{dd4} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Енугу"), ("sr_Latn", "Enugu"), ("sv", "Enugu"), ("sw", "Jimbo la Enugu"), ("ta", "அணுகு ம\u{bbe}நிலம\u{bcd}"), ("te", "ఇనుగు ర\u{c3e}ష\u{c4d}ట\u{c4d}రం"), ("th", "อ\u{e34}น\u{e39}ก\u{e39}"), ("tr", "Enugu Eyaleti"), ("uk", "Енугу"), ("ur", "عنوگو ریاست"), ("vi", "Bang Enugu"), ("yo", "Ìpínlẹ\u{300} Ẹnúgu"), ("yo_BJ", "Ìpínlɛ\u{300} Ɛnúgu"), ("zh", "埃努古州")]),
                        unofficial_name_list: ["Enugu"].to_vec(),
                    }
                ),
                (
                    "FC",
                    Subdivision{
                        name: "FC",
                        country_alpha2: Alpha2::NG,
                        code: "FC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.8556838), longitude: Some(7.179025999999999), max_latitude: Some(9.3574219), min_latitude: Some(8.396675), max_longitude: Some(7.617400000000001), min_longitude: Some(6.749135)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CapitalTerritory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة العاصمة الإتحادية لنيجيريا"), ("be", "Федэральная сталічная тэрыторыя"), ("bn", "ফেড\u{9be}রেল ক\u{9cd}য\u{9be}পিট\u{9be}ল টেরিটরি"), ("ca", "Territori de la Capital Federal"), ("ccp", "𑄜𑄬𑄓𑄢𑄬𑄣\u{11134} 𑄇\u{11133}𑄠𑄛\u{11128}𑄑𑄣\u{11134} 𑄑𑄬𑄢\u{11128}𑄑\u{1112e}𑄢\u{11128}"), ("ceb", "Federal Capital Territory"), ("da", "Federal Capital Territory"), ("de", "Federal Capital Territory"), ("el", "Φεουδαρχικό Καπιταλιστικό Έδαφος, Νιγηρία"), ("en", "Federal Capital Territory"), ("es", "Territorio de la Capital Federal"), ("et", "Föderaalne pealinnaala"), ("fa", "حوزه فدرال مرکزی"), ("fi", "Pääkaupunkiterritorio"), ("fr", "Territoire de la capitale fédérale du Nigeria"), ("gl", "Territorio da Capital Federal de Nixeria"), ("gu", "ફ\u{ac7}ડરલ ક\u{ac7}પિટલ ટ\u{ac7}રિટરી"), ("hi", "स\u{902}घीय राजधानी क\u{94d}ष\u{947}त\u{94d}र, नाइजीरिया"), ("id", "Wilayah Ibu Kota Federal"), ("it", "Abuja Federal Capital Territory"), ("ja", "連邦首都地区"), ("ka", "დედაქალაქის ფედერალური ტერიტორია"), ("kn", "ಫ\u{cc6}ಡರಲ\u{ccd} ಕ\u{ccd}ಯಾಪ\u{cbf}ಟಲ\u{ccd} ಟ\u{cc6}ರ\u{cbf}ಟರ\u{cbf}"), ("ko", "연방 수도 지구"), ("lt", "Federalinė sostinės teritorija"), ("lv", "Federālā galvaspilsētas teritorija"), ("mr", "फ\u{947}डरल क\u{945}पिटल ट\u{947}रिटरी"), ("ms", "Federal Capital Territory"), ("nb", "Federal Capital Territory"), ("nl", "Federal Capital Territory"), ("no", "Federal Capital Territory"), ("pl", "Federalne Terytorium Stołeczne"), ("pt", "Território da Capital Federal da Nigéria"), ("ro", "Teritoriul Capitalei Federale"), ("ru", "Федеральная столичная территория"), ("si", "ෆෙඩරල\u{dca} ප\u{dca}\u{200d}රධ\u{dcf}න භ\u{dd6}ම\u{dd2}ය"), ("sv", "Federal Capital Territory"), ("sw", "Federal Capital Territory"), ("ta", "பெடரல\u{bcd} கேப\u{bcd}பிடல\u{bcd} டெரிட\u{bbe}ரி"), ("te", "ఫ\u{c46}డరల\u{c4d} క\u{c4d}య\u{c3e}ప\u{c3f}టల\u{c4d} ట\u{c46}ర\u{c3f}టర\u{c40}"), ("th", "หม\u{e39}\u{e48}เกาะแชท\u{e31}ม"), ("tr", "Federal Başkent Bölgesi"), ("uk", "Федеральна столична територія"), ("ur", "وفاقی دارالحکومت علاقہ، نائجیریا"), ("vi", "Lãnh thổ Thủ đô liên bang"), ("yo", "Agbègbè Olú-ìlú Ìjọba Àpapọ\u{300} Nàíjíríà"), ("yo_BJ", "Agbègbè Olú-ìlú Ìjɔba Àpapɔ\u{300} Nàíjíríà"), ("zh", "聯邦首都特區")]),
                        unofficial_name_list: ["Abuja Capital Territory"].to_vec(),
                    }
                ),
                (
                    "GO",
                    Subdivision{
                        name: "GO",
                        country_alpha2: Alpha2::NG,
                        code: "GO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.283333), longitude: Some(11.166667), max_latitude: Some(10.3128497), min_latitude: Some(10.264117), max_longitude: Some(11.2147019), min_longitude: Some(11.1424112)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية غومبي"), ("bg", "Гомбе"), ("bn", "গোম\u{9cd}বে অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat de Gombe"), ("ccp", "𑄉\u{1112e}𑄟\u{11134}𑄝𑄬"), ("ceb", "Gombe (kapital sa estado)"), ("da", "Gombe"), ("de", "Gombe"), ("el", "Γκόμπε"), ("en", "Gombe"), ("es", "Gombe"), ("et", "Gombe osariik"), ("fa", "ایالت گومبه"), ("fi", "Gombe"), ("fr", "État de Gombe"), ("gl", "Estado de Gombe"), ("gu", "ગોમ\u{acd}બ\u{ac7} સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Gombe"), ("ha_NE", "Gombe"), ("hi", "गोम\u{94d}ब\u{947} राज\u{94d}य"), ("hr", "Gombe, nigerijska država"), ("id", "Gombe"), ("ig", "Ȯra Gombe"), ("it", "Gombe"), ("ja", "ゴンベ州"), ("ka", "გომბეს შტატი"), ("kn", "ಗೊಂಬ\u{cc6} ರಾಜ\u{ccd}ಯ"), ("ko", "곰베 주"), ("lt", "Gombės valstija"), ("lv", "Gombes štats"), ("mr", "गोम\u{947} राज\u{94d}य"), ("ms", "Gombe State"), ("nb", "Gombe"), ("nl", "Gombe"), ("no", "Gombe"), ("pl", "Gombe"), ("pt", "Gombe"), ("ro", "Statul Gombe"), ("ru", "Гомбе"), ("si", "ගොම\u{dca}බෙ ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Гомбе"), ("sr_Latn", "Gombe"), ("sv", "Gombe"), ("sw", "Jimbo la Gombe"), ("ta", "கொம\u{bcd}பே ம\u{bbe}நிலம\u{bcd}"), ("te", "గ\u{c4b}ంబ\u{c46} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "กอมเบ"), ("tr", "Gombe Eyaleti"), ("uk", "Гомбе"), ("ur", "گومبے ریاست"), ("vi", "Bang Gombe"), ("yo", "Ìpínlẹ\u{300} Gòmbè"), ("yo_BJ", "Ìpínlɛ\u{300} Gòmbè"), ("zh", "贡贝州")]),
                        unofficial_name_list: ["Gombe"].to_vec(),
                    }
                ),
                (
                    "IM",
                    Subdivision{
                        name: "IM",
                        country_alpha2: Alpha2::NG,
                        code: "IM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(5.5214533), longitude: Some(6.920913499999999), max_latitude: Some(5.928465), min_latitude: Some(5.179824000000001), max_longitude: Some(7.404109), min_longitude: Some(6.622309899999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية إيمو"), ("bg", "Имо"), ("bn", "ইমো অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Imo"), ("ccp", "𑄃\u{11128}𑄟\u{1112e}"), ("ceb", "Imo State"), ("da", "Imo"), ("de", "Imo"), ("el", "Ίμο"), ("en", "Imo"), ("es", "Imo"), ("et", "Imo osariik"), ("fa", "ایالت ایمو"), ("fi", "Imo"), ("fr", "État d’Imo"), ("gl", "Estado de Imo"), ("gu", "ઇમો સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Imo"), ("ha_NE", "Imo"), ("he", "אימו"), ("hi", "इमो राज\u{94d}य"), ("hy", "Իմո"), ("id", "Imo"), ("ig", "Ȯra Imo"), ("it", "Imo"), ("ja", "イモ州"), ("ka", "იმოს შტატი"), ("kn", "ಇಮೋ ರಾಜ\u{ccd}ಯ"), ("ko", "이모 주"), ("lt", "Imo valstija"), ("lv", "Imo štats"), ("mr", "इमो राज\u{94d}य"), ("ms", "Imo State"), ("nb", "Imo"), ("nl", "Imo"), ("no", "Imo"), ("pl", "Imo"), ("pt", "Imo"), ("ro", "Statul Imo"), ("ru", "Имо"), ("si", "ඉමෝ ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Имо"), ("sr_Latn", "Imo"), ("sv", "Imo"), ("sw", "Jimbo la Imo"), ("ta", "இமோ ம\u{bbe}நிலம\u{bcd}"), ("te", "ఇమ\u{c4b} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "อ\u{e35}โม"), ("tr", "Imo Eyaleti"), ("uk", "Імо"), ("ur", "امو ریاست"), ("vi", "Bang Imo"), ("yo", "Ìpínlẹ\u{300} Ímò"), ("yo_BJ", "Ìpínlɛ\u{300} Ímò"), ("zh", "伊莫州")]),
                        unofficial_name_list: ["Imo"].to_vec(),
                    }
                ),
                (
                    "JI",
                    Subdivision{
                        name: "JI",
                        country_alpha2: Alpha2::NG,
                        code: "JI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.5700315), longitude: Some(8.9400589), max_latitude: Some(12.5708993), min_latitude: Some(12.5688205), max_longitude: Some(8.9410364), min_longitude: Some(8.9387779)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية جيغاوة"), ("bg", "Джигава"), ("bn", "জিগ\u{9be}ব\u{9be} অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Jigawa"), ("ccp", "𑄎\u{11128}𑄉𑄤"), ("ceb", "Jigawa State"), ("da", "Jigawa"), ("de", "Jigawa"), ("el", "Τζιγκουάβα"), ("en", "Jigawa"), ("es", "Jigawa"), ("et", "Jigawa"), ("fa", "ایالت جیگاوا"), ("fi", "Jigawa"), ("fr", "État de Jigawa"), ("gl", "Jigawa"), ("gu", "જિગાવા સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Jigawa"), ("ha_NE", "Jigawa"), ("hi", "जिगावा राज\u{94d}य"), ("id", "Jigawa"), ("ig", "Ȯra Jigawa"), ("it", "Jigawa"), ("ja", "ジガワ州"), ("ka", "ჯიგავის შტატი"), ("kn", "ಜ\u{cbf}ಗಾವಾ ರಾಜ\u{ccd}ಯ"), ("ko", "지가와 주"), ("lt", "Džigavos valstija"), ("lv", "Džigavas štats"), ("mr", "जिगावा राज\u{94d}य"), ("ms", "Jigawa State"), ("nb", "Jigawa"), ("nl", "Jigawa"), ("no", "Jigawa"), ("pl", "Jigawa"), ("pt", "Jigawa"), ("ro", "Statul Jigawa"), ("ru", "Джигава"), ("si", "ජ\u{dd2}ගව\u{dcf} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Џигава"), ("sr_Latn", "Džigava"), ("sv", "Jigawa"), ("sw", "Jigawa"), ("ta", "ஜிகவ\u{bbe} ம\u{bbe}நிலம\u{bcd}"), ("te", "జ\u{c3f}గ\u{c3e}వ\u{c3e} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "จ\u{e34}กาวา"), ("tr", "Jigawa Eyaleti"), ("uk", "Джигава"), ("ur", "جیگاوا ریاست"), ("vi", "Bang Jigawa"), ("yo", "Ìpínlẹ\u{300} Jígàwà"), ("yo_BJ", "Ìpínlɛ\u{300} Jígàwà"), ("zh", "吉加瓦州")]),
                        unofficial_name_list: ["Jigawa"].to_vec(),
                    }
                ),
                (
                    "KD",
                    Subdivision{
                        name: "KD",
                        country_alpha2: Alpha2::NG,
                        code: "KD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.516667), longitude: Some(7.433332999999999), max_latitude: Some(10.6169963), min_latitude: Some(10.3971566), max_longitude: Some(7.508812000000001), min_longitude: Some(7.349789099999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية كادونا"), ("bn", "ক\u{9be}দ\u{9c1}ন\u{9be} অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat de Kaduna"), ("ccp", "𑄇𑄓\u{1112a}𑄚"), ("ceb", "Kaduna State"), ("da", "Kaduna"), ("de", "Kaduna"), ("el", "Καντούνα"), ("en", "Kaduna"), ("es", "Kaduna"), ("et", "Kaduna osariik"), ("fa", "ایالت کادونا"), ("fi", "Kaduna"), ("fr", "État de Kaduna"), ("gl", "Estado de Kaduna"), ("gu", "કડ\u{ac1}ના સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Kaduna"), ("ha_NE", "Kaduna"), ("he", "קדונה (מדינה)"), ("hi", "कड\u{942}ना राज\u{94d}य"), ("id", "Kaduna"), ("ig", "Ȯra Kaduna"), ("it", "Kaduna"), ("ja", "カドゥナ州"), ("ka", "კადუნის შტატი"), ("kn", "ಕಡುನ ರಾಜ\u{ccd}ಯ"), ("ko", "카두나 주"), ("lt", "Kadunos valstija"), ("lv", "Kadunas štats"), ("mr", "कड\u{941}न राज\u{94d}य"), ("ms", "Kaduna State"), ("nb", "Kaduna (delstat)"), ("nl", "Kaduna"), ("no", "Kaduna (delstat)"), ("pl", "Kaduna"), ("pt", "Kaduna"), ("ro", "Statul Kaduna"), ("ru", "Кадуна"), ("si", "කඩ\u{dd4}න\u{dcf} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Кадуна"), ("sr_Latn", "Kaduna"), ("sv", "Kaduna"), ("sw", "Kaduna"), ("ta", "க\u{bbe}ட\u{bcd}டுன\u{bbe} ம\u{bbe}நிலம\u{bcd}"), ("te", "కడూన\u{c3e} ర\u{c3e}ష\u{c4d}ట\u{c4d}రం"), ("th", "ร\u{e31}ฐคาด\u{e39}นา"), ("tr", "Kaduna Eyaleti"), ("uk", "Кадуна"), ("ur", "کادونا اسٹیٹ"), ("vi", "Bang Kaduna"), ("yo", "Ìpínlẹ\u{300} Kàdúná"), ("yo_BJ", "Ìpínlɛ\u{300} Kàdúná"), ("zh", "卡杜纳州")]),
                        unofficial_name_list: ["Kaduna"].to_vec(),
                    }
                ),
                (
                    "KE",
                    Subdivision{
                        name: "KE",
                        country_alpha2: Alpha2::NG,
                        code: "KE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.6781241), longitude: Some(4.0695454), max_latitude: Some(13.232542), min_latitude: Some(10.0931591), max_longitude: Some(6.027123), min_longitude: Some(3.4958741)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية كبي"), ("bg", "Кебби"), ("bn", "কেব\u{9cd}বি অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat de Kebbi"), ("ccp", "𑄇𑄬𑄝\u{11133}𑄦\u{11128}"), ("ceb", "Kebbi State"), ("da", "Kebbi"), ("de", "Kebbi"), ("el", "Κέμπι"), ("en", "Kebbi"), ("es", "Kebbi"), ("et", "Kebbi osariik"), ("fa", "ایالت کبی"), ("fi", "Kebbi"), ("fr", "État de Kebbi"), ("gl", "Kebbi"), ("gu", "ક\u{ac7}બી સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Kebbi"), ("ha_NE", "Kebbi"), ("hi", "क\u{947}बी राज\u{94d}य"), ("id", "Kebbi"), ("ig", "Ȯra Kebbi"), ("it", "Kebbi"), ("ja", "ケビ州"), ("ka", "კების შტატი"), ("kn", "ಕ\u{cc6}ಬ\u{cbf}ಬ\u{cbf} ರಾಜ\u{ccd}ಯ"), ("ko", "케비 주"), ("lt", "Kebio valstija"), ("lv", "Kebi štats"), ("mr", "क\u{947}ब\u{94d}बी राज\u{94d}य"), ("ms", "Kebbi State"), ("nb", "Kebbi"), ("nl", "Kebbi"), ("no", "Kebbi"), ("pl", "Kebbi"), ("pt", "Kebbi"), ("ro", "Statul Kebbi"), ("ru", "Кебби"), ("si", "කෙබ\u{dd2} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Кеби"), ("sr_Latn", "Kebi"), ("sv", "Kebbi"), ("sw", "Kebbi"), ("ta", "கேப\u{bcd}பி ம\u{bbe}நிலம\u{bcd}"), ("te", "క\u{c46}బ\u{c4d}బ\u{c40} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "ร\u{e31}ฐเคบบ\u{e34}"), ("tr", "Kebbi Eyaleti"), ("uk", "Кеббі"), ("ur", "کیبی ریاست"), ("vi", "Bang Kebbi"), ("yo", "Ìpínlẹ\u{300} Kébbí"), ("yo_BJ", "Ìpínlɛ\u{300} Kébbí"), ("zh", "凯比州")]),
                        unofficial_name_list: ["Kebbi"].to_vec(),
                    }
                ),
                (
                    "KN",
                    Subdivision{
                        name: "KN",
                        country_alpha2: Alpha2::NG,
                        code: "KN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.0021794), longitude: Some(8.591956099999999), max_latitude: Some(12.0829016), min_latitude: Some(11.912873), max_longitude: Some(8.6704765), min_longitude: Some(8.411321599999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية كانو"), ("bn", "ক\u{9be}নো অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat de Kano"), ("ccp", "𑄇𑄚\u{1112e}"), ("ceb", "Kano State"), ("cs", "Kano"), ("da", "Kano"), ("de", "Kano"), ("el", "Πολιτεία του Κάνο"), ("en", "Kano"), ("es", "Estado de Kano"), ("et", "Kano osariik"), ("fa", "ایالت کانو"), ("fi", "Kano"), ("fr", "État de Kano"), ("gl", "Estado de Kano"), ("gu", "કાનો સ\u{acd}ટ\u{ac7}ટ"), ("ha", "jihar Kano"), ("ha_NE", "jihar Kano"), ("he", "קנו (מדינה)"), ("hi", "कानो राज\u{94d}य"), ("id", "Kano"), ("ig", "Nkeji Ochíchííwu Kano"), ("it", "Kano"), ("ja", "カノ州"), ("ka", "კანოს შტატი"), ("kn", "ಕ\u{ccd}ಯಾನೊ ರಾಜ\u{ccd}ಯ"), ("ko", "카노 주"), ("lt", "Kano valstija"), ("lv", "Kano štats"), ("mr", "कानो राज\u{94d}य"), ("ms", "Negeri Kano"), ("nb", "Kano"), ("nl", "Kano"), ("no", "Kano"), ("pl", "Kano"), ("pt", "Kano"), ("ro", "Statul Kano"), ("ru", "Кано"), ("si", "ක\u{dcf}නෝ ර\u{dcf}ජ\u{dca}\u{200d}යය"), ("sr", "Кано"), ("sr_Latn", "Kano"), ("sv", "Kano"), ("sw", "Kano"), ("ta", "கனோ ம\u{bbe}நிலம\u{bcd}"), ("te", "క\u{c3e}న\u{c4b} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "ร\u{e31}ฐคาโน"), ("tr", "Kano Devleti"), ("uk", "Кано"), ("ur", "کانو ریاست"), ("vi", "Bang Kano"), ("yo", "Ìpínlẹ\u{300} Kánò"), ("yo_BJ", "Ìpínlɛ\u{300} Kánò"), ("zh", "卡诺州")]),
                        unofficial_name_list: ["Kano"].to_vec(),
                    }
                ),
                (
                    "KO",
                    Subdivision{
                        name: "KO",
                        country_alpha2: Alpha2::NG,
                        code: "KO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.561891), longitude: Some(6.5783387), max_latitude: Some(8.7320238), min_latitude: Some(6.528274199999999), max_longitude: Some(7.8822179), min_longitude: Some(5.340376)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية كوجي"), ("bg", "Коги"), ("bn", "কোজি অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat de Kogi"), ("ccp", "𑄇\u{1112e}𑄉\u{11128}"), ("ceb", "Kogi State"), ("da", "Kogi"), ("de", "Kogi"), ("el", "Κόγκι"), ("en", "Kogi"), ("es", "Kogi"), ("et", "Kogi"), ("fa", "ایالت کوگی"), ("fi", "Kogi"), ("fr", "État de Kogi"), ("gl", "Estado de Kogi"), ("gu", "કોગી સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Kogi"), ("ha_NE", "Kogi"), ("hi", "कोगी राज\u{94d}य"), ("id", "Kogi"), ("ig", "Ȯra Kogi"), ("it", "Kogi"), ("ja", "コギ州"), ("ka", "კოგის შტატი"), ("kn", "ಕೋಗ\u{cbf} ರಾಜ\u{ccd}ಯ"), ("ko", "코기 주"), ("lt", "Kogio valstija"), ("lv", "Kogi štats"), ("mr", "कोगी राज\u{94d}य"), ("ms", "Kogi State"), ("nb", "Kogi"), ("nl", "Kogi"), ("no", "Kogi"), ("pl", "Kogi"), ("pt", "Kogi"), ("ro", "Statul Kogi"), ("ru", "Коги"), ("si", "කොග\u{dd3} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Коги"), ("sr_Latn", "Kogi"), ("sv", "Kogi"), ("sw", "Kogi"), ("ta", "கோகி ம\u{bbe}நிலம\u{bcd}"), ("te", "క\u{c4b}గ\u{c3f} ర\u{c3e}ష\u{c4d}ట\u{c4d}రం"), ("th", "ร\u{e31}ฐโคก\u{e35}"), ("tr", "Kogi Eyaleti"), ("uk", "Когі"), ("ur", "کوگی ریاست"), ("vi", "Bang Kogi"), ("yo", "Ìpínlẹ\u{300} Kogí"), ("yo_BJ", "Ìpínlɛ\u{300} Kogí"), ("zh", "科吉州")]),
                        unofficial_name_list: ["Kogi"].to_vec(),
                    }
                ),
                (
                    "KT",
                    Subdivision{
                        name: "KT",
                        country_alpha2: Alpha2::NG,
                        code: "KT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.983333), longitude: Some(7.6), max_latitude: Some(13.0357094), min_latitude: Some(12.934095), max_longitude: Some(7.676266000000001), min_longitude: Some(7.5706298)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية كاتسينا"), ("be", "штат Кацына"), ("bg", "Катсина"), ("bn", "ক\u{9cd}য\u{9be}টসিন\u{9be} অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat de Katsina"), ("ccp", "𑄇𑄖\u{11134}𑄥\u{11128}𑄚"), ("ceb", "Katsina State"), ("da", "Katsina"), ("de", "Katsina"), ("el", "Κατσίνα"), ("en", "Katsina"), ("es", "Katsina"), ("et", "Katsina osariik"), ("fa", "ایالت کاتسینا"), ("fi", "Katsina"), ("fr", "État de Katsina"), ("gl", "Estado de Katsina"), ("gu", "કાત\u{acd}સિના સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Jihar Katsina"), ("ha_NE", "Jihar Katsina"), ("hi", "कात\u{94d}सिना राज\u{94d}य"), ("id", "Katsina"), ("ig", "Ȯra Katsina"), ("it", "Katsina"), ("ja", "カツィナ州"), ("ka", "კატსინის შტატი"), ("kn", "ಕಟ\u{ccd}ಸ\u{cbf}ನಾ ಸ\u{ccd}ಟೇಟ\u{ccd}"), ("ko", "카치나 주"), ("lt", "Katsinos valstija"), ("lv", "Kacinas štats"), ("mr", "कत\u{94d}सीना राज\u{94d}य"), ("ms", "Katsina State"), ("nb", "Katsina"), ("nl", "Katsina"), ("no", "Katsina"), ("pl", "Katsina"), ("pt", "Katsina"), ("ro", "Statul Katsina"), ("ru", "Кацина"), ("si", "කට\u{dca}ස\u{dd2}න\u{dcf} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Кацина"), ("sr_Latn", "Kacina"), ("sv", "Katsina"), ("sw", "Katsina"), ("ta", "க\u{bbe}ட\u{bcd}ச\u{bcd}சின\u{bbe} ம\u{bbe}நிலம\u{bcd}"), ("te", "క\u{c3e}ట\u{c4d}స\u{c3f}న\u{c3e} ర\u{c3e}ష\u{c4d}ట\u{c4d}రం"), ("th", "ร\u{e31}ฐค\u{e31}ตซ\u{e34}นา"), ("tr", "Katsina Eyaleti"), ("uk", "Кацина"), ("ur", "کاتسینا ریاست"), ("vi", "Bang Katsina"), ("yo", "Ìpínlẹ\u{300} Kàtsínà"), ("yo_BJ", "Ìpínlɛ\u{300} Kàtsínà"), ("zh", "卡齐纳州")]),
                        unofficial_name_list: ["Katsina"].to_vec(),
                    }
                ),
                (
                    "KW",
                    Subdivision{
                        name: "KW",
                        country_alpha2: Alpha2::NG,
                        code: "KW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.9847995), longitude: Some(4.5624426), max_latitude: Some(10.152084), min_latitude: Some(7.966079000000001), max_longitude: Some(6.2142591), min_longitude: Some(2.728413)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية كوارة"), ("bg", "Квара"), ("bn", "কওড\u{9bc}\u{9be} র\u{9be}জ\u{9cd}য"), ("ca", "Kwara"), ("ccp", "𑄇\u{11127}𑄤𑄢"), ("ceb", "Kwara State"), ("da", "Kwara"), ("de", "Kwara"), ("el", "Κβάρα"), ("en", "Kwara"), ("es", "Kwara"), ("et", "Kwara osariik"), ("fa", "ایالت کوارا"), ("fi", "Kwara"), ("fr", "État de Kwara"), ("gl", "Kwara"), ("gu", "કવારા સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Kwara"), ("ha_NE", "Kwara"), ("he", "מדינת קווארה"), ("hi", "क\u{94d}वारा राज\u{94d}य"), ("id", "Kwara"), ("ig", "Ȯra Kwara"), ("it", "Kwara"), ("ja", "クワラ州"), ("ka", "კვარის შტატი"), ("kn", "ಕ\u{ccd}ವಾರಾ ರಾಜ\u{ccd}ಯ"), ("ko", "콰라 주"), ("lt", "Kvaros valstija"), ("lv", "Kvaras štats"), ("mr", "क\u{941}वा राज\u{94d}य"), ("ms", "Kwara State"), ("nb", "Kwara"), ("nl", "Kwara"), ("no", "Kwara"), ("pl", "Kwara"), ("pt", "Kwara"), ("ro", "Statul Kwara"), ("ru", "Квара"), ("si", "ක\u{dca}ව\u{dcf}ර\u{dcf} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Квара"), ("sr_Latn", "Kvara"), ("sv", "Kwara"), ("sw", "Kwara"), ("ta", "கவர\u{bbe} ம\u{bbe}நிலம\u{bcd}"), ("te", "క\u{c4d}వ\u{c3e}ర\u{c3e} ర\u{c3e}ష\u{c4d}ట\u{c4d}రం"), ("th", "ร\u{e31}ฐควารา"), ("tr", "Kwara Eyaleti"), ("uk", "Квара"), ("ur", "کوارا ریاست"), ("vi", "Bang Kwara"), ("yo", "Ìpínlẹ\u{300} Kúárà"), ("yo_BJ", "Ìpínlɛ\u{300} Kúárà"), ("zh", "夸拉州")]),
                        unofficial_name_list: ["Kwara"].to_vec(),
                    }
                ),
                (
                    "LA",
                    Subdivision{
                        name: "LA",
                        country_alpha2: Alpha2::NG,
                        code: "LA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.5243793), longitude: Some(3.3792057), max_latitude: Some(6.7027984), min_latitude: Some(6.3936419), max_longitude: Some(3.696727799999999), min_longitude: Some(3.0982732)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية لاغوس"), ("be", "штат Лагас"), ("bg", "Лагос"), ("ca", "Estat de Lagos"), ("ccp", "𑄣𑄉\u{1112e}𑄌\u{11134}"), ("ceb", "Lagos State"), ("da", "Lagos"), ("de", "Lagos"), ("en", "Lagos"), ("es", "Estado de Lagos"), ("et", "Lagose osariik"), ("fa", "ایالت لاگوس"), ("fi", "Lagos"), ("fr", "État de Lagos"), ("gl", "Estado de Lagos"), ("ha", "Lagos"), ("ha_NE", "Lagos"), ("he", "לאגוס (מדינה)"), ("hi", "ल\u{947}गोस राज\u{94d}य"), ("hy", "Լագոս"), ("id", "Lagos"), ("ig", "Ȯra Lagos"), ("it", "Lagos"), ("ja", "レゴス州"), ("ka", "ლაგოსის შტატი"), ("ko", "라고스 주"), ("nb", "Lagos"), ("nl", "Lagos"), ("no", "Lagos"), ("pl", "Lagos"), ("pt", "Lagos"), ("ro", "Statul Lagos"), ("ru", "Лагос"), ("sr", "Лагос"), ("sr_Latn", "Lagos"), ("sv", "Lagos"), ("sw", "Jimbo la Lagos"), ("th", "ร\u{e31}ฐเลกอส"), ("tr", "Lagos Eyaleti"), ("uk", "Лагос"), ("ur", "لاگوس ریاست"), ("yo", "Èkó"), ("yo_BJ", "Èkó"), ("zh", "拉各斯州")]),
                        unofficial_name_list: ["Lagos"].to_vec(),
                    }
                ),
                (
                    "NA",
                    Subdivision{
                        name: "NA",
                        country_alpha2: Alpha2::NG,
                        code: "NA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.5705151), longitude: Some(8.3088441), max_latitude: Some(9.365964000000002), min_latitude: Some(7.769181), max_longitude: Some(9.605724), min_longitude: Some(6.924008)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية نصراوة"), ("bg", "Насарауа"), ("bn", "ন\u{9be}স\u{9be}র\u{9be}ও অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat de Nasarawa"), ("ccp", "𑄚𑄥𑄢\u{11127}𑄤"), ("ceb", "Nasarawa State"), ("da", "Nassarawa"), ("de", "Nassarawa"), ("el", "Νασαράβα"), ("en", "Nasarawa"), ("es", "Nasarawa"), ("et", "Nasarawa"), ("fa", "ایالت ناساراوا"), ("fi", "Nasarawa"), ("fr", "État de Nassarawa"), ("gl", "Estado de Nasarawa"), ("gu", "નાસારવા સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Nasarawa"), ("ha_NE", "Nasarawa"), ("hi", "नासरवा राज\u{94d}य"), ("hu", "Nasarawa"), ("id", "Nassarawa"), ("ig", "Ȯra Nasarawa"), ("it", "Nassarawa"), ("ja", "ナサラワ州"), ("ka", "ნასარავის შტატი"), ("kn", "ನಸರಾವಾ ರಾಜ\u{ccd}ಯ"), ("ko", "나사라와 주"), ("lt", "Nasaravos valstija"), ("lv", "Nasaravas štats"), ("mr", "नासर\u{94d}व राज\u{94d}य"), ("ms", "Nasarawa State"), ("nb", "Nasarawa"), ("nl", "Nassarawa"), ("no", "Nasarawa"), ("pl", "Nassarawa"), ("pt", "Nasarawa"), ("ro", "Statul Nassarawa"), ("ru", "Насарава"), ("si", "නසරව\u{dcf} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Насарава"), ("sr_Latn", "Nasarava"), ("sv", "Nasarawa"), ("sw", "Jimbo la Nasarawa"), ("ta", "நசரவ\u{bbe} ம\u{bbe}நிலம\u{bcd}"), ("te", "నసర\u{c3e}వ\u{c3e} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "นาซซาราวา"), ("tr", "Nasarawa Eyaleti"), ("uk", "Насарава"), ("ur", "نصراوا ریاست"), ("vi", "Bang Nasarawa"), ("yo", "Ìpínlẹ\u{300} Násáráwá"), ("yo_BJ", "Ìpínlɛ\u{300} Násáráwá"), ("zh", "纳萨拉瓦州")]),
                        unofficial_name_list: ["Nasarawa"].to_vec(),
                    }
                ),
                (
                    "NI",
                    Subdivision{
                        name: "NI",
                        country_alpha2: Alpha2::NG,
                        code: "NI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.081999), longitude: Some(8.675277), max_latitude: Some(13.885645), min_latitude: Some(4.269857099999999), max_longitude: Some(14.677982), min_longitude: Some(2.676932)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية نيجر"), ("bg", "Нигер"), ("bn", "ন\u{9be}ইজ\u{9be}র অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat del Níger"), ("ccp", "𑄚\u{1112d}𑄎𑄢\u{11134}"), ("ceb", "Niger State"), ("da", "Niger"), ("de", "Niger"), ("el", "Νίγκερ"), ("en", "Niger"), ("es", "Níger"), ("et", "Nigeri osariik"), ("fa", "ایالت نیجر"), ("fi", "Niger"), ("fr", "État de Niger"), ("gl", "Estado de Níxer"), ("gu", "નાઇજર સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Neja"), ("ha_NE", "Neja"), ("hi", "नाइजर राज\u{94d}य"), ("id", "Niger"), ("ig", "Ȯra Niger"), ("it", "Niger"), ("ja", "ナイジャ州"), ("ka", "ნიგერის შტატი"), ("kn", "ನೈಜರ\u{ccd} ರಾಜ\u{ccd}ಯ"), ("ko", "나이저 주"), ("lt", "Nigerio valstija"), ("lv", "Nigēras štats"), ("mr", "नायजर राज\u{94d}य"), ("ms", "Niger State"), ("nb", "Niger"), ("nl", "Niger"), ("no", "Niger"), ("pl", "Niger"), ("pt", "Níger"), ("ro", "Statul Niger"), ("ru", "Нигер"), ("si", "නය\u{dd2}ජර\u{dca} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Нигер"), ("sr_Latn", "Niger"), ("sv", "Niger"), ("sw", "Jimbo la Niger"), ("ta", "நைஜர\u{bcd} ம\u{bbe}நிலம\u{bcd}"), ("te", "న\u{c48}గర\u{c4d} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "ไนเจอร"), ("tr", "Niger Eyaleti"), ("uk", "Нігер"), ("ur", "نائیجر ریاست"), ("vi", "Bang Niger"), ("yo", "Ìpínlẹ\u{300} Niger"), ("yo_BJ", "Ìpínlɛ\u{300} Niger"), ("zh", "尼日尔州")]),
                        unofficial_name_list: ["Niger"].to_vec(),
                    }
                ),
                (
                    "OG",
                    Subdivision{
                        name: "OG",
                        country_alpha2: Alpha2::NG,
                        code: "OG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.9098333), longitude: Some(3.2583626), max_latitude: Some(7.974634999999999), min_latitude: Some(6.315346099999999), max_longitude: Some(4.5990951), min_longitude: Some(2.668432)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية أوغون"), ("bg", "Огун"), ("bn", "ওগ\u{9c1}ন অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Ogun"), ("ccp", "𑄃\u{1112e}𑄉𑄚\u{11134}"), ("ceb", "Ogun State"), ("da", "Ogun"), ("de", "Ogun"), ("el", "Όγκουν"), ("en", "Ogun"), ("es", "Ogun"), ("et", "Oguni osariik"), ("fa", "ایالت اوگون"), ("fi", "Ogun"), ("fr", "État d’Ogun"), ("gl", "Estado de Ogun"), ("gu", "ઓગ\u{ac1}ન સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Ogun"), ("ha_NE", "Ogun"), ("he", "אוגון"), ("hi", "ओग\u{941}न राज\u{94d}य"), ("hy", "Օգուն"), ("id", "Ogun"), ("ig", "Ȯra Ogun"), ("it", "Ogun"), ("ja", "オグン州"), ("ka", "ოგუნის შტატი"), ("kn", "ಓಗುನ\u{ccd} ರಾಜ\u{ccd}ಯ"), ("ko", "오군 주"), ("lt", "Oguno valstija"), ("lv", "Oguno štats"), ("mr", "ओग\u{941}न राज\u{94d}य"), ("ms", "Ogun"), ("nb", "Ogun"), ("nl", "Ogun"), ("no", "Ogun"), ("pl", "Ogun"), ("pt", "Ogun"), ("ro", "Statul Ogun"), ("ru", "Огун"), ("si", "ඔග\u{dd4}න\u{dca} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Огун"), ("sr_Latn", "Ogun"), ("sv", "Ogun"), ("sw", "Jimbo la Ogun"), ("ta", "ஓகுன\u{bcd} ம\u{bbe}நிலம\u{bcd}"), ("te", "ఓగున\u{c4d} ర\u{c3e}ష\u{c4d}ట\u{c4d}రం"), ("th", "โอก\u{e31}น"), ("tr", "Ogun Eyaleti"), ("uk", "Огун"), ("ur", "اوگون ریاست"), ("vi", "Bang Ogun"), ("yo", "Ìpínlẹ\u{300} Ògùn"), ("yo_BJ", "Ìpínlɛ\u{300} Ògùn"), ("zh", "奥贡州")]),
                        unofficial_name_list: ["Ogun"].to_vec(),
                    }
                ),
                (
                    "ON",
                    Subdivision{
                        name: "ON",
                        country_alpha2: Alpha2::NG,
                        code: "ON",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.083333), longitude: Some(4.833333), max_latitude: Some(7.1249106), min_latitude: Some(7.0529381), max_longitude: Some(4.871921599999999), min_longitude: Some(4.7963046)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية أوندو"), ("bg", "Ондо"), ("bn", "অন\u{9cd}দো অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat d’Ondo"), ("ccp", "𑄃𑄚\u{11134}𑄓\u{1112e}"), ("ceb", "Ondo State"), ("da", "Ondo"), ("de", "Ondo"), ("el", "Πολιτεία Όντο"), ("en", "Ondo"), ("es", "Ondo"), ("et", "Ondo osariik"), ("fa", "ایالت اوندو"), ("fi", "Ondo"), ("fr", "État d’Ondo"), ("gl", "Estado de Ondo"), ("gu", "ઓન\u{acd}ડો સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Ondo"), ("ha_NE", "Ondo"), ("he", "אונדו"), ("hi", "ओन\u{94d}दो राज\u{94d}य"), ("id", "Ondo"), ("ig", "Ȯra Ondo"), ("it", "Ondo"), ("ja", "オンド州"), ("ka", "ონდოს შტატი"), ("kn", "ಒಂಡೋ ರಾಜ\u{ccd}ಯ"), ("ko", "온도 주"), ("lt", "Ondo valstija"), ("lv", "Ondo"), ("mr", "ओन\u{94d}डो राज\u{94d}य"), ("ms", "Ondo State"), ("nb", "Ondo"), ("nl", "Ondo"), ("no", "Ondo"), ("pl", "Ondo"), ("pt", "Ondo"), ("ro", "Statul Ondo"), ("ru", "Ондо"), ("si", "ඕන\u{dca}ඩෝ ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Ондо"), ("sr_Latn", "Ondo"), ("sv", "Ondo"), ("sw", "Jimbo la Ondo"), ("ta", "ஒண\u{bcd}டோ ம\u{bbe}நிலம\u{bcd}"), ("te", "ఓండ\u{c4b} ర\u{c3e}ష\u{c4d}ట\u{c4d}రం"), ("th", "ร\u{e31}ฐออนโด"), ("tr", "Ondo Eyaleti"), ("uk", "Ондо"), ("ur", "اوندو ریاست"), ("vi", "Bang Ondo"), ("yo", "Ìpínlẹ\u{300} Òndó"), ("yo_BJ", "Ìpínlɛ\u{300} Òndó"), ("zh", "翁多州")]),
                        unofficial_name_list: ["Ondo"].to_vec(),
                    }
                ),
                (
                    "OS",
                    Subdivision{
                        name: "OS",
                        country_alpha2: Alpha2::NG,
                        code: "OS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.5875843), longitude: Some(4.5624426), max_latitude: Some(8.088640999999999), min_latitude: Some(6.969976), max_longitude: Some(5.0647019), min_longitude: Some(4.020612)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية أوشون"), ("bg", "Осун"), ("bn", "ওস\u{9c1}ন অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat d’Osun"), ("ccp", "𑄃\u{1112e}𑄥𑄚\u{11134}"), ("ceb", "Osun State"), ("da", "Osun"), ("de", "Osun"), ("el", "Οσούν"), ("en", "Osun"), ("es", "Osun"), ("et", "Osuni osariik"), ("fa", "ایالت اوسون"), ("fi", "Osun"), ("fr", "État d’Osun"), ("gl", "Estado de Osun"), ("gu", "ઓસ\u{ac1}ન સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Osun"), ("ha_NE", "Osun"), ("he", "מדינת אוסון"), ("hi", "ओश\u{941}न राज\u{94d}य"), ("id", "Osun"), ("ig", "Ȯra Osun"), ("it", "Osun"), ("ja", "オスン州"), ("ka", "ოსუნის შტატი"), ("kn", "ಓಸ\u{ccd}ಸನ\u{ccd} ರಾಜ\u{ccd}ಯ"), ("ko", "오순 주"), ("lt", "Osuno valstija"), ("lv", "Osunas štats"), ("mr", "ओस\u{941}न राज\u{94d}य"), ("ms", "Osun State"), ("nb", "Osun"), ("nl", "Osun"), ("no", "Osun"), ("pl", "Osun"), ("pt", "Osun"), ("ro", "Statul Osun"), ("ru", "Осун"), ("si", "ඔස\u{dd4}න\u{dca} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Осун"), ("sr_Latn", "Osun"), ("sv", "Osun"), ("sw", "Jimbo la Osun"), ("ta", "ஓசன\u{bcd} ம\u{bbe}நிலம\u{bcd}"), ("te", "ఓసున\u{c4d} ర\u{c3e}ష\u{c4d}ట\u{c4d}రం"), ("th", "โอซ\u{e38}น"), ("tr", "Osun Eyaleti"), ("uk", "Осун"), ("ur", "اوسون ریاست"), ("vi", "Bang Osun"), ("zh", "奥孙州")]),
                        unofficial_name_list: ["Osun"].to_vec(),
                    }
                ),
                (
                    "OY",
                    Subdivision{
                        name: "OY",
                        country_alpha2: Alpha2::NG,
                        code: "OY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.85), longitude: Some(3.933), max_latitude: Some(7.8899933), min_latitude: Some(7.790425099999999), max_longitude: Some(3.9751624), min_longitude: Some(3.8832377)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية أويو"), ("bg", "Ойо"), ("ca", "Oyo"), ("ccp", "𑄃\u{1112e}𑄠\u{1112e}"), ("ceb", "Oyo State"), ("da", "Oyo"), ("de", "Oyo"), ("el", "πολιτεία Όγιο"), ("en", "Oyo"), ("es", "Oyo"), ("et", "Oyo osariik"), ("fa", "ایالت اویو"), ("fi", "Oyo"), ("fr", "État d’Oyo"), ("gl", "Estado de Oyo"), ("ha", "Oyo"), ("ha_NE", "Oyo"), ("he", "אויו"), ("hi", "ओयो राज\u{94d}य"), ("id", "Oyo"), ("ig", "Ȯra Ọyọ"), ("it", "Oyo"), ("ja", "オヨ州"), ("ka", "ოიოს შტატი"), ("ko", "오요 주"), ("nb", "Oyo"), ("nl", "Oyo"), ("no", "Oyo"), ("pl", "Oyo"), ("pt", "Oyo"), ("ro", "Statul Oyo"), ("ru", "Ойо"), ("sr", "Ојо"), ("sr_Latn", "Ojo"), ("sv", "Oyo"), ("sw", "Jimbo la Oyo"), ("tr", "Oyo Eyaleti"), ("uk", "Ойо (штат)"), ("ur", "اویو ریاست"), ("zh", "奥约州")]),
                        unofficial_name_list: ["Oyo"].to_vec(),
                    }
                ),
                (
                    "PL",
                    Subdivision{
                        name: "PL",
                        country_alpha2: Alpha2::NG,
                        code: "PL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.2350817), longitude: Some(9.7232673), max_latitude: Some(10.3447241), min_latitude: Some(8.350639), max_longitude: Some(10.6606639), min_longitude: Some(8.544271)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية بلاتو"), ("bg", "Плато"), ("bn", "প\u{9cd}লেট\u{9c1} অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat de Plateau"), ("ccp", "𑄛\u{11133}𑄣𑄑\u{11128}𑄅\u{1112a}"), ("ceb", "Plateau State"), ("cs", "Plateau"), ("da", "Plateau"), ("de", "Plateau"), ("el", "Πλατό"), ("en", "Plateau"), ("es", "Plateau"), ("et", "Plateau osariik"), ("fa", "ایالت پلاتو"), ("fi", "Plateau"), ("fr", "État du Plateau"), ("gl", "Plateau"), ("gu", "પ\u{acd}લ\u{ac7}ટીઓ સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Plateau"), ("ha_NE", "Plateau"), ("hi", "प\u{94d}ल\u{948}टो राज\u{94d}य"), ("id", "Plateau"), ("ig", "Ȯra Plateau"), ("it", "Plateau"), ("ja", "プラトー州"), ("ka", "პლატოს შტატი"), ("kn", "ಪ\u{ccd}ರಸ\u{ccd}ಥಭ\u{cc2}ಮ\u{cbf} ರಾಜ\u{ccd}ಯ"), ("ko", "플래토 주"), ("lt", "Plato valstija"), ("lv", "Plato štats"), ("mr", "प\u{94d}ल\u{945}टय\u{942} राज\u{94d}य"), ("ms", "Plateau State"), ("nb", "Plateau"), ("nl", "Plateau"), ("no", "Plateau"), ("pl", "Plateau"), ("pt", "Plateau"), ("ro", "Statul Plateau"), ("ru", "Плато"), ("si", "පල\u{dcf}ට\u{dd4} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Плато"), ("sr_Latn", "Plato"), ("sv", "Plateau"), ("sw", "Jimbo la Plateau"), ("ta", "பிள\u{bbe}ட\u{bcd}டோ ம\u{bbe}நிலம\u{bcd}"), ("te", "ప\u{c4d}ల\u{c47}టూ స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "ร\u{e31}ฐแพทโท"), ("tr", "Plateau Eyaleti"), ("uk", "Плато"), ("ur", "پلیٹئو ریاست"), ("vi", "Bang Plateau"), ("yo", "Ìpínlẹ\u{300} Plateau"), ("yo_BJ", "Ìpínlɛ\u{300} Plateau"), ("zh", "高原州")]),
                        unofficial_name_list: ["Plateau"].to_vec(),
                    }
                ),
                (
                    "RI",
                    Subdivision{
                        name: "RI",
                        country_alpha2: Alpha2::NG,
                        code: "RI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(4.8580767), longitude: Some(6.920913499999999), max_latitude: Some(5.694652899999999), min_latitude: Some(4.347035099999999), max_longitude: Some(7.591592899999998), min_longitude: Some(6.382472000000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية ريفرز"), ("bg", "Ривърс"), ("bn", "রিভ\u{9be}র অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Rivers"), ("ccp", "𑄢\u{11128}𑄞𑄢\u{11134}𑄌\u{11134}"), ("ceb", "Rivers State"), ("cs", "Rivers"), ("da", "Rivers"), ("de", "Rivers"), ("el", "Ρίβερς"), ("en", "Rivers"), ("es", "Rivers"), ("et", "Riversi osariik"), ("fa", "ایالت ریورز"), ("fi", "Rivers"), ("fr", "État de Rivers"), ("gl", "Rivers"), ("gu", "રીવર\u{acd}સ સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Rivers"), ("ha_NE", "Rivers"), ("hi", "रिवर\u{94d}ज\u{93c} राज\u{94d}य"), ("hu", "Rivers állam"), ("hy", "Ռիվերս նահանգ"), ("id", "Rivers"), ("ig", "Ȯra Rivers"), ("it", "Rivers"), ("ja", "リバーズ州"), ("ka", "რივერსის შტატი"), ("kn", "ನದ\u{cbf}ಗಳ ರಾಜ\u{ccd}ಯ"), ("ko", "리버스 주"), ("lt", "Riverso valstija"), ("lv", "Riversas štats"), ("mr", "रिव\u{94d}हर\u{94d}स राज\u{94d}य"), ("ms", "Rivers State"), ("nb", "Rivers"), ("nl", "Rivers"), ("no", "Rivers"), ("pl", "Rivers"), ("pt", "Rivers"), ("ro", "Statul Rivers"), ("ru", "Риверс"), ("si", "ර\u{dd2}වර\u{dca}ස\u{dca} ස\u{dca}ටේට\u{dca}"), ("sr", "Риверс"), ("sr_Latn", "Rivers"), ("sv", "Rivers"), ("sw", "Rivers State"), ("ta", "ரிவர\u{bcd}ஸ\u{bcd} ம\u{bbe}நிலம\u{bcd}"), ("te", "ర\u{c3f}వర\u{c4d}స\u{c4d} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "ร\u{e34}เวอส\u{e4c}"), ("tr", "Rivers Eyaleti"), ("uk", "Ріверс"), ("ur", "ریورز ریاست"), ("vi", "Bang Rivers"), ("yo", "Ìpínlẹ\u{300} Rivers"), ("yo_BJ", "Ìpínlɛ\u{300} Rivers"), ("zh", "河流州")]),
                        unofficial_name_list: ["Rivers"].to_vec(),
                    }
                ),
                (
                    "SO",
                    Subdivision{
                        name: "SO",
                        country_alpha2: Alpha2::NG,
                        code: "SO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.066667), longitude: Some(5.233333), max_latitude: Some(13.0880893), min_latitude: Some(12.9764148), max_longitude: Some(5.2881144), min_longitude: Some(5.1676084)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية صكتو"), ("bg", "Сокото"), ("bn", "সোকোটো অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat de Sokoto"), ("ccp", "𑄥\u{1112e}𑄇\u{1112e}𑄑\u{1112e}"), ("ceb", "Sokoto State"), ("da", "Sokoto"), ("de", "Sokoto"), ("el", "Σοκότο"), ("en", "Sokoto"), ("es", "Sokoto"), ("et", "Sokoto osariik"), ("eu", "Sokoto"), ("fa", "ایالت سوکوتو"), ("fi", "Sokoto"), ("fr", "État de Sokoto"), ("gl", "Estado de Sokoto"), ("gu", "સોકોટો સ\u{acd}ટ\u{ac7}ટ"), ("ha", "jihar Sokoto"), ("ha_NE", "jihar Sokoto"), ("hi", "सोकोटो राज\u{94d}य"), ("id", "Sokoto"), ("ig", "Ȯra Sokoto"), ("it", "Sokoto"), ("ja", "ソコト州"), ("ka", "სოკოტოს შტატი"), ("kn", "ಸೊಕೊಟೊ ರಾಜ\u{ccd}ಯ"), ("ko", "소코토 주"), ("lt", "Sokoto valstija"), ("lv", "Sokoto štats"), ("mr", "सोकोटो राज\u{94d}य"), ("ms", "Sokoto State"), ("nb", "Sokoto"), ("nl", "Sokoto"), ("no", "Sokoto"), ("pl", "Sokoto"), ("pt", "Sokoto"), ("ro", "Statul Sokoto"), ("ru", "Сокото"), ("si", "සොකොටෝ ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Сокото"), ("sr_Latn", "Sokoto"), ("sv", "Sokoto"), ("sw", "Jimbo la Sokoto"), ("ta", "சொகொட\u{bcd}டோ ம\u{bbe}நிலம\u{bcd}"), ("te", "స\u{c4b}క\u{c4b}ట\u{c4b} ర\u{c3e}ష\u{c4d}ట\u{c4d}రం"), ("th", "ร\u{e31}ฐโซโกโตะ"), ("tr", "Sokoto Eyaleti"), ("uk", "Сокото"), ("ur", "سوکوتو ریاست"), ("vi", "Bang Sokoto"), ("yo", "Ìpínlẹ\u{300} Sókótó"), ("yo_BJ", "Ìpínlɛ\u{300} Sókótó"), ("zh", "索科托州")]),
                        unofficial_name_list: ["Sokoto"].to_vec(),
                    }
                ),
                (
                    "TA",
                    Subdivision{
                        name: "TA",
                        country_alpha2: Alpha2::NG,
                        code: "TA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.9868755), longitude: Some(10.9807003), max_latitude: Some(9.604444899999999), min_latitude: Some(6.502453999999999), max_longitude: Some(11.981274), min_longitude: Some(9.1043009)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية ترابة"), ("bg", "Тараба"), ("bn", "ত\u{9be}র\u{9be}ব\u{9be} অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat de Taraba"), ("ccp", "𑄑𑄢𑄝"), ("ceb", "Taraba State"), ("da", "Taraba"), ("de", "Taraba"), ("el", "Ταράμπα"), ("en", "Taraba"), ("es", "Taraba"), ("et", "Taraba osariik"), ("fa", "ایالت تارابا"), ("fi", "Taraba"), ("fr", "État de Taraba"), ("gl", "Taraba"), ("gu", "ટરબા સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Taraba"), ("ha_NE", "Taraba"), ("hi", "टराबा राज\u{94d}य"), ("id", "Taraba"), ("ig", "Ȯra Taraba"), ("it", "Taraba"), ("ja", "タラバ州"), ("ka", "ტარაბის შტატი"), ("kk", "Тараба"), ("kn", "ತಾರಾಬಾ ರಾಜ\u{ccd}ಯ"), ("ko", "타라바 주"), ("lt", "Tarabos valstija"), ("lv", "Tarabas štats"), ("mr", "ताराबा राज\u{94d}य"), ("ms", "Taraba State"), ("nb", "Taraba"), ("nl", "Taraba"), ("no", "Taraba"), ("pl", "Taraba"), ("pt", "Taraba"), ("ro", "Statul Taraba"), ("ru", "Тараба"), ("si", "ටරබ\u{dcf} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Тараба"), ("sr_Latn", "Taraba"), ("sv", "Taraba"), ("sw", "Jimbo la Taraba"), ("ta", "தர\u{bbe}ப\u{bbe} ம\u{bbe}நிலம\u{bcd}"), ("te", "టర\u{c3e}బ\u{c3e} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "แคว\u{e49}นเร\u{e35}ยซ\u{e31}น"), ("tr", "Taraba Eyaleti"), ("uk", "Тараба"), ("ur", "تارابا ریاست"), ("vi", "Taraba"), ("yo", "Ìpínlẹ\u{300} Tàràbà"), ("yo_BJ", "Ìpínlɛ\u{300} Tàràbà"), ("zh", "塔拉巴州")]),
                        unofficial_name_list: ["Taraba"].to_vec(),
                    }
                ),
                (
                    "YO",
                    Subdivision{
                        name: "YO",
                        country_alpha2: Alpha2::NG,
                        code: "YO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.1871412), longitude: Some(11.7068294), max_latitude: Some(13.376133), min_latitude: Some(10.5932052), max_longitude: Some(12.494047), min_longitude: Some(9.665973100000002)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية يوبي"), ("bg", "Йобе"), ("bn", "ইয\u{9bc}ব অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Yobe"), ("ccp", "𑄃\u{11128}𑄠\u{1112e}𑄝𑄬"), ("ceb", "Yobe State"), ("cs", "Yobe"), ("da", "Yobe"), ("de", "Yobe"), ("el", "Γιόμπε"), ("en", "Yobe"), ("es", "Yobe"), ("et", "Yobe"), ("fa", "ایالت یوبه"), ("fi", "Yobe"), ("fr", "État de Yobe"), ("gl", "Yobe"), ("gu", "યોબ\u{ac7} સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Yobe"), ("ha_NE", "Yobe"), ("hi", "योब\u{947} राज\u{94d}य"), ("id", "Yobe"), ("ig", "Ȯra Yobe"), ("it", "Yobe"), ("ja", "ヨベ州"), ("ka", "იობეს შტატი"), ("kn", "ಯೊಬ\u{cc6} ರಾಜ\u{ccd}ಯ"), ("ko", "요베 주"), ("lt", "Jobės valstija"), ("lv", "Jobes štats"), ("mr", "याब\u{947} राज\u{94d}य"), ("ms", "Yobe State"), ("nb", "Yobe"), ("nl", "Yobe"), ("no", "Yobe"), ("pl", "Yobe"), ("pt", "Yobe"), ("ro", "Statul Yobe"), ("ru", "Йобе"), ("si", "යෝබේ ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Јобе"), ("sr_Latn", "Jobe"), ("sv", "Yobe"), ("sw", "Jimbo la Yobe"), ("ta", "யோபெ ம\u{bbe}நிலம\u{bcd}"), ("te", "య\u{c4b}బ\u{c4d} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "โยบ\u{e35}"), ("tr", "Yobe Eyaleti"), ("uk", "Йобе"), ("ur", "یوبے اسٹیٹ"), ("vi", "Bang Yobe"), ("yo", "Ìpínlẹ\u{300} Yòbè"), ("yo_BJ", "Ìpínlɛ\u{300} Yòbè"), ("zh", "约贝州")]),
                        unofficial_name_list: ["Yobe"].to_vec(),
                    }
                ),
                (
                    "ZA",
                    Subdivision{
                        name: "ZA",
                        country_alpha2: Alpha2::NG,
                        code: "ZA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.1844159), longitude: Some(6.2375947), max_latitude: Some(13.207291), min_latitude: Some(10.854616), max_longitude: Some(7.245729999999999), min_longitude: Some(5.021007099999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية زامفارة"), ("bg", "Замфара"), ("bn", "জ\u{9be}ম\u{9cd}প\u{9be}র\u{9be} অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat de Zamfara"), ("ccp", "𑄎𑄟\u{11134}𑄜𑄢"), ("ceb", "Zamfara State"), ("da", "Zamfara"), ("de", "Zamfara"), ("el", "Ζαμφάρα"), ("en", "Zamfara"), ("es", "Zamfara"), ("et", "Zamfara osariik"), ("fa", "ایالت زامفارا"), ("fi", "Zamfara"), ("fr", "État de Zamfara"), ("gl", "Zamfara"), ("gu", "ઝ\u{ac7}મ\u{acd}ફારા સ\u{acd}ટ\u{ac7}ટ"), ("ha", "Zamfara"), ("ha_NE", "Zamfara"), ("hi", "ज\u{93c}ामफ\u{93c}ारा राज\u{94d}य"), ("id", "Zamfara"), ("ig", "Ȯra Zamfara"), ("it", "Zamfara"), ("ja", "ザムファラ州"), ("ka", "ზამფარის შტატი"), ("kn", "ಜಮ\u{ccd}ಫರಾ ಸ\u{ccd}ಟೇಟ\u{ccd}"), ("ko", "잠파라 주"), ("lt", "Zamfaros valstija"), ("lv", "Zamfaras štats"), ("mr", "जामफर राज\u{94d}य"), ("ms", "Zamfara State"), ("nb", "Zamfara"), ("nl", "Zamfara"), ("no", "Zamfara"), ("pl", "Zamfara"), ("pt", "Zamfara (estado)"), ("ro", "Statul Zamfara"), ("ru", "Замфара"), ("si", "සම\u{dca}ෆර\u{dcf} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Замфара"), ("sr_Latn", "Zamfara"), ("sv", "Zamfara"), ("sw", "Jimbo la Zamfara"), ("ta", "சப\u{bcd}மப\u{bbe}ர\u{bbe} ம\u{bbe}நிலம\u{bcd}"), ("te", "జంఫ\u{c3e}ర\u{c3e} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "แซมฟารา"), ("tr", "Zamfara Eyaleti"), ("uk", "Замфара"), ("ur", "زمفارا ریاست"), ("vi", "Bang Zamfara"), ("yo", "Ìpínlẹ\u{300} Zamfara"), ("yo_BJ", "Ìpínlɛ\u{300} Zamfara"), ("zh", "扎姆法拉州")]),
                        unofficial_name_list: ["Zamfara"].to_vec(),
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
#[cfg(feature = "ng")]
pub fn new() -> Country {
    Country{
        alpha2: Alpha2::NG,
        alpha3: Alpha3::NGA,
        address_format: None,
        continent: Continent::Africa,
        country_code: 234,
        currency_code: "NGN",
        gec: Some(GEC::NI),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "009",
        ioc: Some("NGR"),
        iso_long_name: "The Federal Republic of Nigeria",
        iso_short_name: "Nigeria",
        official_language_list: ["en"].to_vec(),
        spoken_language_list: ["en"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7, 8].to_vec(),
        national_prefix: "0",
        nationality: Some("Nigerian"),
        number: "566",
        postal_code: true,
        postal_code_format: Some("\\d{6}"),
        region: Some(Region::Africa),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::WesternAfrica),
        un_locode: "NG",
        unofficial_name_list: ["Nigeria", "Nigéria", "the Federal Republic of Nigeria", "ナイジェリア"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([("ab", "Nigeria"), ("af", "Nigerië"), ("ak", "Nigeria"), ("am", "ና፤ጄሱ።"), ("an", "Nigeria"), ("ar", "نيجيريا"), ("as", "ন\u{9be}ইজেৰিয়\u{9be}"), ("ay", "Nigeria"), ("az", "Nigeriya"), ("ba", "Nigeria"), ("be", "Нігерыя"), ("bg", "Нигерия"), ("bi", "Nigeria"), ("bn", "ন\u{9be}ইজেরিয়\u{9be}"), ("bn_IN", "ন\u{9be}ইজেরিয়\u{9be}"), ("br", "Nigeria"), ("bs", "Nigerija"), ("ca", "Nigèria"), ("ce", "Нигери"), ("ch", "Nigeria"), ("cs", "Nigérie"), ("cv", "Нигери"), ("cy", "Nigeria"), ("da", "Nigeria"), ("de", "Nigeria"), ("dv", "ނ\u{7a6}އ\u{7a8}ޖ\u{7a9}ރ\u{7a8}އ\u{7a7}"), ("dz", "ནའ\u{f72}་ཇ\u{f72}་ར\u{f72}་ཡ།"), ("ee", "Nigeria"), ("el", "Νιγηρία"), ("en", "Nigeria"), ("eo", "Niĝerio"), ("es", "Nigeria"), ("et", "Nigeeria"), ("eu", "Nigeria"), ("fa", "نیجریه"), ("ff", "Niiseriya"), ("fi", "Nigeria"), ("fo", "Nigeria"), ("fr", "Nigeria"), ("fy", "Nigearia"), ("ga", "An Nigéir"), ("gl", "Nixeria"), ("gn", "Nigeria"), ("gu", "નાઇજ\u{ac7}રિયા"), ("gv", "Yn Naigeer"), ("ha", "Nijeriya"), ("he", "ניגריה"), ("hi", "नाईजीरिया"), ("hr", "Nigerija"), ("ht", "Nijerya"), ("hu", "Nigéria"), ("hy", "Նիգերիա"), ("ia", "Nigeria"), ("id", "Nigeria"), ("io", "Nigeria"), ("is", "Nígería"), ("it", "Nigeria"), ("iu", "Nigeria"), ("ja", "ナイジェリア"), ("ka", "ნიგერია"), ("ki", "Nigeria"), ("kk", "Нигерия"), ("kl", "Nigeria"), ("km", "ន\u{17b8}ហ\u{17d2}សេរ\u{17b8}យ\u{17c9}ា"), ("kn", "ನೈಜೀರ\u{cbf}ಯಾ"), ("ko", "나이지리아"), ("ku", "Nîjerya"), ("kv", "Nigeria"), ("kw", "Nijeri"), ("ky", "Нигерия"), ("lo", "Nigeria"), ("lt", "Nigerija"), ("lv", "Nigērija"), ("mi", "Nigeria"), ("mk", "Нигерија"), ("ml", "നൈജീരിയ"), ("mn", "Нигерь"), ("mr", "नायज\u{947}रिया"), ("ms", "Nigeria"), ("mt", "Niġerja"), ("my", "န\u{102d}\u{102f}င\u{103a}ဂျ\u{102e}းရ\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}"), ("na", "Nigeria"), ("nb", "Nigeria"), ("ne", "नाइज\u{947}रिया"), ("nl", "Nigeria"), ("nn", "Nigeria"), ("nv", "Naakaii Łizhinii Biʼéénézí Bikéyah"), ("oc", "Nigèria"), ("or", "ନ\u{b3f}ଈଜୀର\u{b3f}ୟ\u{b3e}"), ("pa", "ਨੀਜ\u{a3c}ੀਰਆ"), ("pi", "न\u{948}जीरिया"), ("pl", "Nigeria"), ("ps", "نایجیریا"), ("pt", "Nigéria"), ("pt_BR", "Nigéria"), ("ro", "Nigeria"), ("ru", "Нигерия"), ("rw", "Nigeriya"), ("sc", "Nigèria"), ("sd", "نائيجيريا"), ("si", "නය\u{dd2}ජ\u{dd3}ර\u{dd2}ය\u{dcf}"), ("sk", "Nigéria"), ("sl", "Nigerija"), ("so", "Nayjeeriya"), ("sq", "Nigeri"), ("sr", "Нигерија"), ("sv", "Nigeria"), ("sw", "Nigeria"), ("ta", "நைஜ\u{bc0}ரிய\u{bbe}"), ("te", "న\u{c48}జ\u{c40}ర\u{c3f}య\u{c3e}"), ("tg", "Нигерия"), ("th", "ไนจ\u{e35}เร\u{e35}ย"), ("ti", "ናይጄሪያ"), ("tk", "Nigeriýa"), ("tl", "Nigeria"), ("tr", "Nijerya"), ("tt", "Ниgериа"), ("ug", "نىگېرىيە"), ("uk", "Нігерія"), ("ur", "نائجیریا"), ("uz", "Nigeriya"), ("ve", "Nigeria"), ("vi", "Ni-giê-ri-a"), ("wa", "Nidjeria"), ("wo", "Nijeeria"), ("xh", "Nigeria"), ("yo", "Nàìjíríà"), ("zh_CN", "尼日利亚"), ("zh_HK", "尼日利亞"), ("zh_TW", "奈及利亞"), ("zu", "INigeria")]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
