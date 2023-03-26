// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// Grenada

#[cfg(all(feature = "gd", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::GD;
    pub const ALPHA3: Alpha3 = Alpha3::GRD;
    pub const CONTINENT: Continent = Continent::NorthAmerica;
    pub const COUNTRY_CODE: usize = 1;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::XCD;
    pub const GEC: Option<GEC> = Some(GEC::GJ);
    pub const INTERNATIONAL_PREFIX: &str = "011";
    pub const IOC: Option<IOC> = Some(IOC::GRN);
    pub const ISO_SHORT_NAME: &str = "Grenada";
    pub const ISO_LONG_NAME: &str = "Grenada";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[3];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10];
    pub const NATIONAL_PREFIX: &str = "1";
    pub const NATIONALITY: Option<&str> = Some("Grenadian");
    pub const NUMBER: &str = "308";
    pub const POSTAL_CODE: bool = false;
    pub const POSTAL_CODE_FORMAT: Option<&str> = None;
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::Caribbean);
    pub const UN_LOCODE: &str = "GD";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Grenada", "グレナダ"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Grenada"),
        ("af", "Grenada"),
        ("ak", "Grenada"),
        ("am", "Grenada"),
        ("an", "Grenada"),
        ("ar", "غرينادا"),
        ("as", "গ\u{9cd}ৰেন\u{9be}ড\u{9be}"),
        ("ay", "Grenada"),
        ("az", "Qrenada"),
        ("ba", "Grenada"),
        ("be", "Грэнада"),
        ("bg", "Гренада"),
        ("bi", "Grenada"),
        ("bn", "গ\u{9cd}রেন\u{9be}ড\u{9be}"),
        ("bn_IN", "গ\u{9cd}রেন\u{9be}ড\u{9be}"),
        ("br", "Grenada"),
        ("bs", "Grenada"),
        ("ca", "Grenada"),
        ("ce", "Гренада"),
        ("ch", "Grenada"),
        ("cs", "Grenada"),
        ("cv", "Гренада"),
        ("cy", "Grenada"),
        ("da", "Grenada"),
        ("de", "Grenada"),
        ("dv", "ގ\u{7ac}ރ\u{7ac}ނ\u{7b0}ޑ\u{7a7}"),
        ("dz", "ག\u{f72}ར\u{f72}་ན་ཌ།"),
        ("ee", "Grenada"),
        ("el", "Γρενάδα"),
        ("en", "Grenada"),
        ("eo", "Grenado"),
        ("es", "Granada"),
        ("et", "Grenada"),
        ("eu", "Grenada"),
        ("fa", "گرانادا"),
        ("ff", "Grenada"),
        ("fi", "Grenada"),
        ("fo", "Grenada"),
        ("fr", "Grenade"),
        ("fy", "Grenada"),
        ("ga", "Grenada"),
        ("gl", "Granada"),
        ("gn", "Grenada"),
        ("gu", "ગ\u{acd}ર\u{ac7}ન\u{ac7}ડા"),
        ("gv", "Grenada"),
        ("ha", "Grenada"),
        ("he", "גרנדה"),
        ("hi", "ग\u{94d}र\u{947}नाडा"),
        ("hr", "Grenada"),
        ("ht", "Grenad"),
        ("hu", "Grenada"),
        ("hy", "Գրենադա"),
        ("ia", "Grenada"),
        ("id", "Grenada"),
        ("io", "Grenada"),
        ("is", "Grenada"),
        ("it", "Grenada"),
        ("iu", "Grenada"),
        ("ja", "グレナダ"),
        ("ka", "გრენადა"),
        ("ki", "Grenada"),
        ("kk", "Гренада"),
        ("kl", "Grenada"),
        ("km", "ហ\u{17d2}គ\u{17d2}រ\u{17b8}ណាដា"),
        ("kn", "ಗ\u{ccd}ರ\u{cc6}ನಡಾ"),
        ("ko", "그레나다"),
        ("ku", "Girava Grenada"),
        ("kv", "Гренада"),
        ("kw", "Grenayd"),
        ("ky", "Гренада"),
        ("lo", "Grenada"),
        ("lt", "Grenada"),
        ("lv", "Grenāda"),
        ("mi", "Grenada"),
        ("mk", "Гренада"),
        ("ml", "ഗ\u{d4d}രനഡ"),
        ("mn", "Гренда"),
        ("mr", "ग\u{94d}र\u{947}न\u{947}डा"),
        ("ms", "Grenada"),
        ("mt", "Grenada"),
        ("my", "ဂရ\u{102e}နေဒါန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
        ("na", "Grenada"),
        ("nb", "Grenada"),
        ("ne", "ग\u{94d}रिनाडा"),
        ("nl", "Grenada"),
        ("nn", "Grenada"),
        ("nv", "Grenada"),
        ("oc", "Granada"),
        ("or", "ଗ\u{b4d}ରେନେଡ\u{b3e}"),
        ("pa", "ਗਰੀਨਾਡਾਆ"),
        ("pi", "ग\u{94d}र\u{947}नाडा"),
        ("pl", "Grenada"),
        ("ps", "ګرېنادا"),
        ("pt", "Granada"),
        ("pt_BR", "Granada"),
        ("ro", "Grenada"),
        ("ru", "Гренада"),
        ("rw", "Gerenada"),
        ("sc", "Grenada"),
        ("sd", "Grenada"),
        ("si", "ග\u{dca}\u{200d}රෙන\u{dcf}ඩ\u{dcf}"),
        ("sk", "Grenada"),
        ("sl", "Grenada"),
        ("so", "Giriinaada"),
        ("sq", "Grenadë"),
        ("sr", "Гренада"),
        ("sv", "Grenada"),
        ("sw", "Grenada"),
        ("ta", "கிரன\u{bbe}ட\u{bbe}"),
        ("te", "గ\u{c4d}ర\u{c47}న\u{c47}డ\u{c3e}"),
        ("tg", "Гренада"),
        ("th", "เกรนาดา"),
        ("ti", "Grenada"),
        ("tk", "Grenada"),
        ("tl", "Grenada"),
        ("tr", "Grenada"),
        ("tt", "Gренада"),
        ("ug", "گىرېنادا"),
        ("uk", "Гренада"),
        ("ur", "گریناڈا"),
        ("uz", "Grenada"),
        ("ve", "Grenada"),
        ("vi", "Gợ-rê-na-đa"),
        ("wa", "Grenåde"),
        ("wo", "Grenada"),
        ("xh", "Grenada"),
        ("yo", "Grẹ\u{300}nádà"),
        ("zh_CN", "格林纳达"),
        ("zh_HK", "格林納達"),
        ("zh_TW", "格瑞那達"),
        ("zu", "I-Grenada"),
    ];
    #[cfg(all(feature = "gd", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 12.1165;
        pub const LONGITUDE: f64 = -61.67899999999999;
        pub const MAX_LATITUDE: f64 = 12.5367;
        pub const MAX_LONGITUDE: f64 = -61.3746999;
        pub const MIN_LATITUDE: f64 = 11.9829051;
        pub const MIN_LONGITUDE: f64 = -61.80589999999999;
        pub const NORTHEAST_LATITUDE: f64 = 12.5367;
        pub const NORTHEAST_LONGITUDE: f64 = -61.3746999;
        pub const SOUTHWEST_LATITUDE: f64 = 11.9829051;
        pub const SOUTHWEST_LONGITUDE: f64 = -61.80589999999999;
    }
}
#[cfg(all(feature = "gd", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 12.1165,
            longitude: -61.67899999999999,
            max_latitude: 12.5367,
            max_longitude: -61.3746999,
            min_latitude: 11.9829051,
            min_longitude: -61.80589999999999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 12.5367,
                    longitude: -61.3746999,
                },
                southwest: CountryGeoBound {
                    latitude: 11.9829051,
                    longitude: -61.80589999999999,
                },
            },
        }
    }
}

#[cfg(all(feature = "gd", feature = "subdivisions"))]
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
                        country_alpha2: Alpha2::GD,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.1230598), longitude: Some(-61.64211719999999), max_latitude: Some(12.17327), min_latitude: Some(12.048629), max_longitude: Some(-61.59250969999999), min_longitude: Some(-61.701438)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبرشية سانت أندرو"), ("bn", "সেন\u{9cd}ট আন\u{9cd}ড\u{9cd}র\u{9c1} প\u{9be}রিশ"), ("ccp", "𑄥𑄬𑄚\u{11133}𑄑\u{11134} 𑄃𑄬𑄚\u{11134}𑄓\u{11133}𑄢\u{1112a}"), ("ceb", "Saint Andrew (parokya sa Grenada)"), ("da", "Saint Andrew Parish"), ("de", "Saint Andrew"), ("el", "Άγιος Ανδρέας, Γρενάδα"), ("en", "Saint Andrew"), ("es", "Parroquia de Saint Andrew"), ("eu", "Saint Andrew"), ("fi", "Saint Andrew"), ("fr", "Paroisse de Saint Andrew"), ("gl", "Saint Andrew"), ("gu", "સ\u{ac7}ન\u{acd}ટ એન\u{acd}ડ\u{acd}ર\u{ac1} પ\u{ac5}રિશ"), ("hi", "स\u{947}\u{902}ट ए\u{902}ड\u{94d}र\u{942} प\u{948}रिश, जमाइका"), ("id", "Paroki Saint Andrew"), ("it", "Parrocchia di Saint Andrew"), ("ja", "セント・アンドリューズ"), ("kn", "ಸೇಂಟ\u{ccd} ಆಂಡ\u{ccd}ರ\u{ccd}ಯ\u{cc2} ಪ\u{ccd}ಯಾರ\u{cbf}ಷ\u{ccd}"), ("ko", "세인트앤드루 교구"), ("lt", "Šv. Andriaus parapija"), ("lv", "Sentrendrū pagasts"), ("mr", "स\u{947}\u{902}ट अ\u{901}ड\u{94d}र\u{942} परश"), ("ms", "Saint Andrew Parish"), ("nb", "Saint Andrew prestegjeld"), ("nl", "Saint Andrew"), ("no", "Saint Andrew prestegjeld"), ("pl", "Saint Andrew"), ("pt", "Saint Andrew"), ("ru", "Приход Сент-Эндрю"), ("si", "ශ\u{dcf}න\u{dca}ත ඇන\u{dca}ඩ\u{dca}ර\u{dd6} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Saint Andrew (parish i Grenada)"), ("ta", "செயின\u{bcd}ட\u{bcd} அன\u{bcd}றெவ\u{bcd} பரிஷ\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} ఆండ\u{c4d}ర\u{c4d}యూ ప\u{c3e}ర\u{c3f}ష\u{c4d}"), ("th", "เซนต\u{e4c}แอนดร\u{e39}ว\u{e4c}"), ("tr", "Saint Andrew Parish"), ("uk", "Парафія Сент-Андру"), ("ur", "سینٹ اینڈریو پریش، گریناڈا"), ("vi", "Giáo xứ Saint Andrew"), ("zh", "聖安德魯區 (格林納達)")]),
                        unofficial_name_list: ["Saint Andrew"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::GD,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.0456435), longitude: Some(-61.6888738), max_latitude: Some(12.07608), min_latitude: Some(12.0065127), max_longitude: Some(-61.6356962), min_longitude: Some(-61.71738500000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبرشية القديس ديفيد"), ("bn", "সেন\u{9cd}ট ডেভিড প\u{9cd}য\u{9be}রিশ"), ("ccp", "𑄥𑄬𑄚\u{11133}𑄑\u{11134} 𑄓𑄬𑄞\u{11128}𑄖\u{11134}"), ("ceb", "Saint David (parokya sa Grenada)"), ("da", "Saint David Parish"), ("de", "Saint David"), ("el", "Άγιος Δαβίδ (ενορία)"), ("en", "Saint David"), ("es", "Parroquia de Saint David"), ("eu", "Saint David"), ("fi", "Saint David"), ("fr", "Paroisse de Saint David"), ("gu", "સ\u{ac7}ન\u{acd}ટ ડ\u{ac7}વિડ પ\u{ac5}રિશ"), ("hi", "स\u{947}\u{902}ट ड\u{947}विड प\u{948}रिश"), ("id", "Paroki Saint David"), ("it", "Parrocchia di Saint David"), ("ja", "セント・デイヴィッド郡 (ドミニカ国)"), ("kn", "ಸೇಂಟ\u{ccd} ಡೇವ\u{cbf}ಡ\u{ccd} ಪ\u{ccd}ಯಾರ\u{cbf}ಷ\u{ccd}"), ("ko", "세인트데이비드 교구"), ("lt", "Šv. Dovydo parapija"), ("lv", "Sentdeivida pagasts"), ("mr", "स\u{947}\u{902}ट ड\u{947}व\u{94d}हिड प\u{945}रीश"), ("ms", "Saint David Parish"), ("nb", "Saint David prestegjeld"), ("nl", "Saint David"), ("no", "Saint David prestegjeld"), ("pl", "Saint David"), ("pt", "Saint David"), ("ru", "Приход Сент-Дэвид"), ("si", "ශ\u{dcf}න\u{dca}ත ඩේව\u{dd2}ඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Saint David"), ("ta", "செயின\u{bcd}ட\u{bcd} டேவிட\u{bcd} பரிஷ\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} డ\u{c47}వ\u{c3f}డ\u{c4d} ప\u{c3e}ర\u{c3f}ష\u{c4d}"), ("th", "เซนต\u{e4c}เดว\u{e34}ด"), ("tr", "Saint David Parish"), ("uk", "Парафія Сент-Девід"), ("ur", "سینٹ ڈیوڈ پریش، گریناڈا"), ("vi", "Giáo xứ Saint David"), ("zh", "聖戴維區 (格林納達)")]),
                        unofficial_name_list: ["Saint David"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::GD,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.0714317), longitude: Some(-61.73564330000001), max_latitude: Some(12.1108259), min_latitude: Some(11.9852072), max_longitude: Some(-61.6958989), min_longitude: Some(-61.80193610000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبرشية القديس جورج"), ("bn", "সেন\u{9cd}ট জর\u{9cd}জ প\u{9cd}য\u{9be}রিশ"), ("ccp", "𑄥𑄬𑄚\u{11133}𑄑\u{11134} 𑄎\u{11127}𑄢\u{11134}𑄎\u{11134}"), ("ceb", "Saint George"), ("da", "Saint George Parish"), ("de", "Saint George"), ("el", "Άγιος Γεώργιος, Γρενάδα"), ("en", "Saint George"), ("es", "Parroquia de Saint George"), ("eu", "Saint George parrokia"), ("fi", "Saint George"), ("fr", "Paroisse de Saint George"), ("gl", "Saint George"), ("gu", "સ\u{ac7}ન\u{acd}ટ જ\u{acd}યોર\u{acd}જ પ\u{ac5}રિશ"), ("hi", "स\u{947}\u{902}ट जॉर\u{94d}ज प\u{948}रिश"), ("id", "Paroki Saint George"), ("it", "Parrocchia di Saint George"), ("ja", "セント・ジョージ郡 (グレナダ)"), ("kn", "ಸೇಂಟ\u{ccd} ಜಾರ\u{ccd}ಜ\u{ccd} ಪ\u{ccd}ಯಾರ\u{cbf}ಷ\u{ccd}"), ("ko", "세인트조지 교구"), ("lt", "Šv. Jurgio parapija"), ("lv", "Sentdžordža pagasts"), ("mr", "स\u{947}\u{902}ट जॉर\u{94d}ज प\u{945}रीश"), ("ms", "Saint George Parish"), ("nb", "Saint George prestegjeld"), ("nl", "Saint George"), ("no", "Saint George prestegjeld"), ("pl", "Saint George"), ("pt", "Saint George"), ("ru", "Приход Сент-Джордж"), ("si", "ශ\u{dcf}න\u{dca}ත ජෝජ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Saint George (parish i Grenada)"), ("ta", "செயின\u{bcd}ட\u{bcd} ஜ\u{bbe}ர\u{bcd}ஜ\u{bcd} பரிஷ\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} జ\u{c3e}ర\u{c4d}జ\u{c4d} ప\u{c3e}ర\u{c3f}ష\u{c4d}"), ("th", "เซนต\u{e4c}จอร\u{e4c}จ"), ("tr", "Sain George Parish"), ("uk", "Парафія Сент-Джордж"), ("ur", "سینٹ جارج پریش، گریناڈا"), ("vi", "Giáo xứ Saint George"), ("zh", "聖喬治區 (格林納達)")]),
                        unofficial_name_list: ["Saint George"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::GD,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.130824), longitude: Some(-61.71225699999999), max_latitude: Some(12.177205), min_latitude: Some(12.108849), max_longitude: Some(-61.682125), min_longitude: Some(-61.75262300000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبرشية القديس جون"), ("bn", "সেন\u{9cd}ট জন প\u{9cd}য\u{9be}রিশ"), ("ccp", "𑄥𑄬𑄚\u{11133}𑄑\u{11134} 𑄎\u{11127}𑄚\u{11134}"), ("ceb", "Saint John (parokya sa Grenada)"), ("da", "Saint John Parish"), ("de", "Saint John"), ("el", "Άγιος Ιωάννης"), ("en", "Saint John"), ("es", "Parroquia de Saint John"), ("eu", "Saint John"), ("fi", "Saint John"), ("fr", "Paroisse de Saint John"), ("gu", "સ\u{ac7}\u{a82}ટ જ\u{acd}હોન પ\u{ac5}રિશ"), ("hi", "स\u{947}\u{902}ट जॉन प\u{948}रिश"), ("id", "Paroki Saint John"), ("it", "Parrocchia di Saint John"), ("ja", "セント・ジョン郡 (ドミニカ国)"), ("kn", "ಸೇಂಟ\u{ccd} ಜಾನ\u{ccd} ಪ\u{ccd}ಯಾರ\u{cbf}ಷ\u{ccd}"), ("ko", "세인트존 교구"), ("lt", "Šv. Jono parapija"), ("lv", "Sentdžona pagasts"), ("mr", "स\u{947}\u{902}ट जॉन प\u{945}रीश"), ("ms", "Saint John Parish"), ("nb", "Saint John prestegjeld"), ("nl", "Saint John"), ("no", "Saint John prestegjeld"), ("pl", "Saint John"), ("pt", "Saint John"), ("ru", "Приход Сент-Джон"), ("si", "ශ\u{dcf}න\u{dca}ත ජෝන\u{dca} වසම"), ("sv", "Saint John"), ("ta", "செயின\u{bcd}ட\u{bcd} ஜ\u{bbe}ன\u{bcd} பரிஷ\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} జ\u{c3e}\u{c3e}న\u{c4d} ప\u{c3e}ర\u{c3f}ష\u{c4d}"), ("th", "เซนต\u{e4c} จอน เพล\u{e34}ส"), ("tr", "Saint John Parish"), ("uk", "Парафія Сент-Джон"), ("ur", "سینٹ جان پریش، گریناڈا"), ("vi", "Giáo xứ Saint John"), ("zh", "聖約翰區 (格林納達)")]),
                        unofficial_name_list: ["Saint John"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::GD,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.19023), longitude: Some(-61.6888738), max_latitude: Some(12.219914), min_latitude: Some(12.1516449), max_longitude: Some(-61.6701531), min_longitude: Some(-61.72427709999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبرشية القديس مارك"), ("bn", "সেন\u{9cd}ট ম\u{9be}\u{9be}র\u{9cd}ক প\u{9be}রিশ"), ("ccp", "𑄥𑄬𑄚\u{11133}𑄑\u{11134} 𑄟𑄢\u{11134}𑄇\u{11134}"), ("ceb", "Saint Mark (parokya sa Grenada)"), ("da", "Saint Mark Parish"), ("de", "Saint Mark"), ("el", "Άγιος Μάρκος, Γρενάδα"), ("en", "Saint Mark"), ("es", "Parroquia de Saint Mark"), ("eu", "Saint Mark"), ("fi", "Saint Mark"), ("fr", "Paroisse de Saint Mark"), ("gu", "સ\u{ac7}ન\u{acd}ટ માર\u{acd}ક પ\u{ac5}રિશ"), ("hi", "स\u{947}\u{902}ट मार\u{94d}क प\u{948}रिश"), ("id", "Paroki Saint Mark"), ("it", "Parrocchia di Saint Mark"), ("ja", "セント・マーク (ドミニカ国)"), ("kn", "ಸೇಂಟ\u{ccd} ಮಾರ\u{ccd}ಕ\u{ccd} ಪ\u{ccd}ಯಾರ\u{cbf}ಷ\u{ccd}"), ("ko", "세인트마크 교구"), ("lt", "Šv. Morkaus parapija"), ("lv", "Sentmarka pagasts"), ("mr", "स\u{947}\u{902}ट मार\u{94d}क प\u{945}रीश"), ("ms", "Saint Mark Parish"), ("nb", "Saint Mark Parish"), ("nl", "Saint Mark"), ("no", "Saint Mark Parish"), ("pl", "Saint Mark"), ("pt", "Saint Mark"), ("ru", "Приход Сент-Марка"), ("si", "ශ\u{dcf}න\u{dca}ත ම\u{dcf}ර\u{dca}ක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Saint Mark"), ("ta", "செயின\u{bcd}ட\u{bcd} ம\u{bbe}ர\u{bcd}க\u{bcd} பரிஷ\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} మ\u{c3e}\u{c3e}ర\u{c4d}క\u{c4d} ప\u{c3e}ర\u{c3f}ష\u{c4d}"), ("th", "ตำบลเซนต\u{e4c}มาร\u{e4c}ค"), ("tr", "Saint Mark Parish"), ("uk", "Парафія Сент-Марк"), ("ur", "سینٹ مارک پریش، گریناڈا"), ("vi", "Giáo xứ Saint Mark"), ("zh", "聖馬克區 (格林納達)")]),
                        unofficial_name_list: ["Saint Mark"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::GD,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.2056921), longitude: Some(-61.64211719999999), max_latitude: Some(12.3308232), min_latitude: Some(12.1670921), max_longitude: Some(-61.54397489999999), min_longitude: Some(-61.68279190000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبرشية القديس باتريك"), ("bn", "সেন\u{9cd}ত প\u{9cd}য\u{9be}ট\u{9cd}রিক প\u{9be}রিশ"), ("ccp", "𑄥𑄬𑄚\u{11133}𑄑\u{11134} 𑄛𑄬𑄑\u{11133}𑄢\u{11128}𑄇\u{11134}"), ("ceb", "Saint Patrick (parokya sa Grenada)"), ("da", "Saint Patrick Parish"), ("de", "Saint Patrick"), ("el", "Άγιος Πατρίκιος"), ("en", "Saint Patrick"), ("es", "Parroquia de Saint Patrick"), ("eu", "Saint Patrick"), ("fi", "Saint Patrick"), ("fr", "Paroisse de Saint Patrick"), ("gu", "સ\u{ac7}ન\u{acd}ટ પ\u{ac7}ટ\u{acd}રિક પ\u{ac5}રિશ"), ("hi", "स\u{947}\u{902}ट प\u{948}ट\u{94d}रिक प\u{948}रिश"), ("id", "Paroki Saint Patrick"), ("it", "Parrocchia di Saint Patrick"), ("ja", "セント・パトリック郡 (グレナダ)"), ("kn", "ಸೇಂಟ\u{ccd} ಪ\u{ccd}ಯಾಟ\u{ccd}ರ\u{cbf}ಕ\u{ccd} ಪ\u{ccd}ಯಾರ\u{cbf}ಷ\u{ccd}"), ("ko", "세인트패트릭 교구"), ("lt", "Šv. Patriko parapija"), ("lv", "Sentpatrika pagasts"), ("mr", "स\u{947}\u{902}ट प\u{945}ट\u{94d}रिक प\u{945}रीश"), ("ms", "Saint Patrick Parish"), ("nb", "Saint Patrick"), ("nl", "Saint Patrick"), ("no", "Saint Patrick"), ("pl", "Saint Patrick"), ("pt", "Saint Patrick"), ("ru", "Приход Сент-Патрик"), ("si", "ශ\u{dcf}න\u{dca}ත පැට\u{dca}\u{200d}ර\u{dd2}ක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Saint Patrick"), ("ta", "செயின\u{bcd}ட\u{bcd} பேட\u{bcd}ரிக\u{bcd} பரிஷ\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంంట\u{c4d} ప\u{c3e}ట\u{c4d}ర\u{c3f}క\u{c4d} ప\u{c3e}ర\u{c3f}ష\u{c4d}"), ("th", "ตำบลเซนต\u{e4c}แพทร\u{e34}ค"), ("tr", "Saint Patrick Parish"), ("uk", "Парафія Сент-Патрік"), ("ur", "سینٹ پیٹرک پریش، گریناڈا"), ("vi", "Giáo xứ Saint Patrick"), ("zh", "聖帕特里克區 (格林納達)")]),
                        unofficial_name_list: ["Saint Patrick"].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::GD,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.4785888), longitude: Some(-61.4493842), max_latitude: Some(12.5298576), min_latitude: Some(12.401671), max_longitude: Some(-61.378178), min_longitude: Some(-61.4989469)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Dependency,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كارياكو و بيتي مارتينيك"), ("bn", "ক\u{9cd}য\u{9be}রিয\u{9bc}\u{9be}র\u{9be}ক\u{9c1} এবং পেতিট ম\u{9be}র\u{9cd}টিনিক"), ("ccp", "𑄇\u{11133}𑄠𑄢\u{11128}𑄠𑄇\u{1112e} 𑄃\u{11133}𑄃 𑄛𑄬𑄑\u{11128}𑄖\u{11134} 𑄟𑄢\u{11134}𑄑\u{11128}𑄚\u{11128}𑄇\u{11134}"), ("ceb", "Petite Martinique"), ("da", "Carriacou and Petite Martinique"), ("de", "Carriacou und Petite Martinique"), ("el", "Καρριακού και Μικρή Μαρτινίκα"), ("en", "Carriacou and Petite Martinique"), ("es", "Carriacou et Petite Martinique"), ("eu", "Carriacou eta Martinika Txikia"), ("fi", "Carriacou and Petite Martinique"), ("fr", "Carriacou et Petite Martinique"), ("gu", "ક\u{ac7}રીઆકોઉ એન\u{acd}ડ પ\u{ac7}ટીટ માર\u{acd}ટિનીક"), ("hi", "क\u{948}रिक\u{942} और प\u{947}टाईट मार\u{94d}टिनीक"), ("id", "Carriacou dan Petite Martinique"), ("it", "Carriacou e Petite Martinique"), ("ja", "カリアク島"), ("kn", "ಕ\u{ccd}ಯಾರ\u{cbf}ಯಕ\u{ccc} ಮತ\u{ccd}ತು ಪ\u{cc6}ಟೈಟ\u{ccd} ಮಾರ\u{ccd}ಟ\u{cbf}ನ\u{cbf}ಕ\u{ccd}"), ("ko", "카리아쿠 프티마르티니크"), ("lt", "Karioka"), ("lv", "Kariaku un Mazā Martinika"), ("mr", "क\u{945}रिएकॉ अ\u{901}ड प\u{94d}य\u{941}टाट मार\u{94d}टिनिक"), ("ms", "Carriacou and Petite Martinique"), ("nb", "Carriacou og Petite Martinique"), ("nl", "Carriacou en Petite Martinique"), ("no", "Carriacou og Petite Martinique"), ("pl", "Carriacou i Mała Martynika"), ("pt", "Carriacou e Petite Martinique"), ("ru", "Карриаку и Малый Мартиник"), ("si", "කර\u{dd2}ය\u{dcf}ක\u{dd4} සහ පෙට\u{dd2}ටේ මර\u{dca}ට\u{dd2}න\u{dca}ක\u{dca}"), ("sv", "Carriacou och Petite Martinique"), ("ta", "க\u{bbe}ரியகோவு அண\u{bcd}ட\u{bcd} பெட\u{bcd}டிட\u{bcd} ம\u{bbe}ர\u{bcd}டினியூ"), ("te", "క\u{c3e}ర\u{c3f}య\u{c3e}క\u{c4b} ప\u{c3f}ట\u{c40}ట\u{c4d} మ\u{c3e}ర\u{c4d}ట\u{c3f}న\u{c3f}క\u{c4d}"), ("th", "กาเร\u{e35}ยก\u{e39} แอน พ\u{e35}ไทร\u{e4c} มาต\u{e34}น\u{e35}\u{e48}ค\u{e34}ว"), ("tr", "Carriacou and Petite Martinique"), ("uk", "Карріаку і Малий Мартінік"), ("ur", "کاریاکو اور پیٹیٹے مارٹنیک"), ("vi", "Carriacou và Petite Martinique"), ("zh", "卡里亞庫和小馬提尼克")]),
                        unofficial_name_list: ["Southern Grenadine Islands"].to_vec(),
                    }
                ),
            ]

        )
    }
}
#[allow(unused_imports)]
use crate::{
    Alpha2, Alpha3, Continent, Country, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC,
    IOC,
};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "gd")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::GD,
        alpha3: Alpha3::GRD,
        address_format: None,
        continent: Continent::NorthAmerica,
        country_code: 1,
        currency_code: CurrencyCode::XCD,
        gec: Some(GEC::GJ),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "011",
        ioc: Some(IOC::GRN),
        iso_long_name: "Grenada",
        iso_short_name: "Grenada",
        official_language_list: ["en"].to_vec(),
        spoken_language_list: ["en"].to_vec(),
        national_destination_code_length_list: [3].to_vec(),
        national_number_length_list: [10].to_vec(),
        national_prefix: "1",
        nationality: Some("Grenadian"),
        number: "308",
        postal_code: false,
        postal_code_format: None,
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::Caribbean),
        un_locode: "GD",
        unofficial_name_list: ["Grenada", "グレナダ"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Grenada"),
            ("af", "Grenada"),
            ("ak", "Grenada"),
            ("am", "Grenada"),
            ("an", "Grenada"),
            ("ar", "غرينادا"),
            ("as", "গ\u{9cd}ৰেন\u{9be}ড\u{9be}"),
            ("ay", "Grenada"),
            ("az", "Qrenada"),
            ("ba", "Grenada"),
            ("be", "Грэнада"),
            ("bg", "Гренада"),
            ("bi", "Grenada"),
            ("bn", "গ\u{9cd}রেন\u{9be}ড\u{9be}"),
            ("bn_IN", "গ\u{9cd}রেন\u{9be}ড\u{9be}"),
            ("br", "Grenada"),
            ("bs", "Grenada"),
            ("ca", "Grenada"),
            ("ce", "Гренада"),
            ("ch", "Grenada"),
            ("cs", "Grenada"),
            ("cv", "Гренада"),
            ("cy", "Grenada"),
            ("da", "Grenada"),
            ("de", "Grenada"),
            ("dv", "ގ\u{7ac}ރ\u{7ac}ނ\u{7b0}ޑ\u{7a7}"),
            ("dz", "ག\u{f72}ར\u{f72}་ན་ཌ།"),
            ("ee", "Grenada"),
            ("el", "Γρενάδα"),
            ("en", "Grenada"),
            ("eo", "Grenado"),
            ("es", "Granada"),
            ("et", "Grenada"),
            ("eu", "Grenada"),
            ("fa", "گرانادا"),
            ("ff", "Grenada"),
            ("fi", "Grenada"),
            ("fo", "Grenada"),
            ("fr", "Grenade"),
            ("fy", "Grenada"),
            ("ga", "Grenada"),
            ("gl", "Granada"),
            ("gn", "Grenada"),
            ("gu", "ગ\u{acd}ર\u{ac7}ન\u{ac7}ડા"),
            ("gv", "Grenada"),
            ("ha", "Grenada"),
            ("he", "גרנדה"),
            ("hi", "ग\u{94d}र\u{947}नाडा"),
            ("hr", "Grenada"),
            ("ht", "Grenad"),
            ("hu", "Grenada"),
            ("hy", "Գրենադա"),
            ("ia", "Grenada"),
            ("id", "Grenada"),
            ("io", "Grenada"),
            ("is", "Grenada"),
            ("it", "Grenada"),
            ("iu", "Grenada"),
            ("ja", "グレナダ"),
            ("ka", "გრენადა"),
            ("ki", "Grenada"),
            ("kk", "Гренада"),
            ("kl", "Grenada"),
            ("km", "ហ\u{17d2}គ\u{17d2}រ\u{17b8}ណាដា"),
            ("kn", "ಗ\u{ccd}ರ\u{cc6}ನಡಾ"),
            ("ko", "그레나다"),
            ("ku", "Girava Grenada"),
            ("kv", "Гренада"),
            ("kw", "Grenayd"),
            ("ky", "Гренада"),
            ("lo", "Grenada"),
            ("lt", "Grenada"),
            ("lv", "Grenāda"),
            ("mi", "Grenada"),
            ("mk", "Гренада"),
            ("ml", "ഗ\u{d4d}രനഡ"),
            ("mn", "Гренда"),
            ("mr", "ग\u{94d}र\u{947}न\u{947}डा"),
            ("ms", "Grenada"),
            ("mt", "Grenada"),
            ("my", "ဂရ\u{102e}နေဒါန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
            ("na", "Grenada"),
            ("nb", "Grenada"),
            ("ne", "ग\u{94d}रिनाडा"),
            ("nl", "Grenada"),
            ("nn", "Grenada"),
            ("nv", "Grenada"),
            ("oc", "Granada"),
            ("or", "ଗ\u{b4d}ରେନେଡ\u{b3e}"),
            ("pa", "ਗਰੀਨਾਡਾਆ"),
            ("pi", "ग\u{94d}र\u{947}नाडा"),
            ("pl", "Grenada"),
            ("ps", "ګرېنادا"),
            ("pt", "Granada"),
            ("pt_BR", "Granada"),
            ("ro", "Grenada"),
            ("ru", "Гренада"),
            ("rw", "Gerenada"),
            ("sc", "Grenada"),
            ("sd", "Grenada"),
            ("si", "ග\u{dca}\u{200d}රෙන\u{dcf}ඩ\u{dcf}"),
            ("sk", "Grenada"),
            ("sl", "Grenada"),
            ("so", "Giriinaada"),
            ("sq", "Grenadë"),
            ("sr", "Гренада"),
            ("sv", "Grenada"),
            ("sw", "Grenada"),
            ("ta", "கிரன\u{bbe}ட\u{bbe}"),
            ("te", "గ\u{c4d}ర\u{c47}న\u{c47}డ\u{c3e}"),
            ("tg", "Гренада"),
            ("th", "เกรนาดา"),
            ("ti", "Grenada"),
            ("tk", "Grenada"),
            ("tl", "Grenada"),
            ("tr", "Grenada"),
            ("tt", "Gренада"),
            ("ug", "گىرېنادا"),
            ("uk", "Гренада"),
            ("ur", "گریناڈا"),
            ("uz", "Grenada"),
            ("ve", "Grenada"),
            ("vi", "Gợ-rê-na-đa"),
            ("wa", "Grenåde"),
            ("wo", "Grenada"),
            ("xh", "Grenada"),
            ("yo", "Grẹ\u{300}nádà"),
            ("zh_CN", "格林纳达"),
            ("zh_HK", "格林納達"),
            ("zh_TW", "格瑞那達"),
            ("zu", "I-Grenada"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
