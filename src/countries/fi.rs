// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Finland

#[cfg(all(feature = "fi", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::FI;
    pub const ALPHA3: Alpha3 = Alpha3::FIN;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 358;
    pub const CURRENCY_CODE: &str = "EUR";
    pub const GEC: Option<GEC> = Some(GEC::FI);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("FIN");
    pub const ISO_SHORT_NAME: &str = "Finland";
    pub const ISO_LONG_NAME: &str = "The Republic of Finland";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["fi", "sv"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["fi", "sv"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Finnish");
    pub const NUMBER: &str = "246";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::NorthernEurope);
    pub const UN_LOCODE: &str = "FI";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Finland",
        "Finnland",
        "Finlande",
        "Finlandia",
        "フィンランド",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Finland"),
        ("af", "Finland"),
        ("ak", "Finland"),
        ("am", "ፑንሒን፥"),
        ("an", "Finland"),
        ("ar", "فنلندا"),
        ("as", "ফিনলেণ\u{9cd}ড"),
        ("ay", "Finland"),
        ("az", "Finlandiya"),
        ("ba", "Finland"),
        ("be", "Фінляндыя"),
        ("bg", "Финландия"),
        ("bi", "Finland"),
        ("bn", "ফিনল\u{9cd}য\u{9be}ন\u{9cd}ড"),
        ("bn_IN", "ফিনল\u{9cd}য\u{9be}ন\u{9cd}ড"),
        ("br", "Finland"),
        ("bs", "Finska"),
        ("ca", "Finlàndia"),
        ("ce", "Финлянди"),
        ("ch", "Finlandia"),
        ("cs", "Finsko"),
        ("cv", "Финлянди"),
        ("cy", "Y Ffindir"),
        ("da", "Finland"),
        ("de", "Finnland"),
        ("dv", "ފ\u{7a8}ނ\u{7b0}ލ\u{7ad}ނ\u{7b0}ޑ\u{7aa}"),
        ("dz", "ཕ\u{f72}ན་ལ\u{f7a}ནཌ\u{f72}།"),
        ("ee", "Finland"),
        ("el", "Φινλανδία"),
        ("en", "Finland"),
        ("eo", "Finnlando"),
        ("es", "Finlandia"),
        ("et", "Soome"),
        ("eu", "Finlandia"),
        ("fa", "فنلاند"),
        ("ff", "Finland"),
        ("fi", "Suomi"),
        ("fo", "Finnland"),
        ("fr", "Finlande"),
        ("fy", "Finlân"),
        ("ga", "An Fhionlainn"),
        ("gl", "Finlandia"),
        ("gn", "Finland"),
        ("gu", "ફિનલ\u{ac7}ન\u{acd}ડ"),
        ("gv", "Finnlynn"),
        ("ha", "Finland"),
        ("he", "פינלנד"),
        ("hi", "फ\u{93c}िनल\u{948}ण\u{94d}ड"),
        ("hr", "Finska"),
        ("ht", "Fenlann"),
        ("hu", "Finnország"),
        ("hy", "Ֆինլանդիա"),
        ("ia", "Finlandia"),
        ("id", "Finlandia"),
        ("io", "Finlando"),
        ("is", "Finnland"),
        ("it", "Finlandia"),
        ("iu", "ᐃᓐᓚᓐᑦ"),
        ("ja", "フィンランド"),
        ("ka", "ფინეთი"),
        ("ki", "Finland"),
        ("kk", "Финляндия"),
        ("kl", "Finland"),
        ("km", "ហ\u{17d2}វា\u{17c6}ងឡង\u{17cb}"),
        ("kn", "ಫ\u{cbf}ನ\u{ccd}ಲಂಡ\u{ccd}"),
        ("ko", "핀란드"),
        ("ku", "Fînlandiya"),
        ("kv", "Суоми Му"),
        ("kw", "Pow Finn"),
        ("ky", "Финляндия"),
        ("lo", "ປະເທດແຟງລ\u{eb1}ງ"),
        ("lt", "Suomija"),
        ("lv", "Somija"),
        ("mi", "Hinerangi"),
        ("mk", "Финска"),
        ("ml", "ഫിന\u{d4d}\u{200d}ലന\u{d4d}\u{200d}ഡ\u{d4d}"),
        ("mn", "Финнлянд"),
        ("mr", "फिनल\u{945}\u{902}ड"),
        ("ms", "Finland"),
        ("mt", "Finlandja"),
        (
            "my",
            "ဖင\u{103a}လန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Pinrand"),
        ("nb", "Finland"),
        ("ne", "फिन\u{94d}ल\u{94d}यान\u{94d}ड"),
        ("nl", "Finland"),
        ("nn", "Finland"),
        ("nv", "Nahoditsʼǫʼłání Dineʼé Bikéyah"),
        ("oc", "Finlàndia"),
        ("or", "ଫ\u{b3f}ନଲ\u{b4d}ଯ\u{b3e}ଣ\u{b4d}ଡ"),
        ("pa", "ਫ਼ਿਨਲ\u{a48}\u{a02}ਡ"),
        ("pi", "फिन\u{94d}ल\u{948}\u{902}ड"),
        ("pl", "Finlandia"),
        ("ps", "فنلینډ"),
        ("pt", "Finlândia"),
        ("pt_BR", "Finlândia"),
        ("ro", "Finlanda"),
        ("ru", "Финляндия"),
        ("rw", "Finilande"),
        ("sc", "Finlàndia"),
        ("sd", "فن لينڊ"),
        ("si", "ෆ\u{dd2}න\u{dca}ලන\u{dca}තය"),
        ("sk", "Fínsko"),
        ("sl", "Finska"),
        ("so", "Fiinlaand"),
        ("sq", "Finlandë"),
        ("sr", "Финска"),
        ("sv", "Finland"),
        ("sw", "Finland"),
        ("ta", "பின\u{bcd}ல\u{bbe}ந\u{bcd}து"),
        ("te", "ఫ\u{c3f}న\u{c4d}\u{200c}ల\u{c3e}ండ\u{c4d}"),
        ("tg", "Финляндия"),
        ("th", "ฟ\u{e34}นแลนด\u{e4c}"),
        ("ti", "ፊንላንድ"),
        ("tk", "Finlýandiýa"),
        ("tl", "Finland"),
        ("tr", "Finlandiya"),
        ("tt", "Финланд (Суоми)"),
        ("ug", "فىنلاندىيە"),
        ("uk", "Фінляндія"),
        ("ur", "فن لینڈ"),
        ("uz", "Finlandiya"),
        ("ve", "Finland"),
        ("vi", "Phần Lan"),
        ("wa", "Finlande"),
        ("wo", "Finlaand"),
        ("xh", "Finland"),
        ("yo", "Fínlándì"),
        ("zh_CN", "芬兰"),
        ("zh_HK", "芬蘭"),
        ("zh_TW", "芬蘭"),
        ("zu", "IFinlandi"),
    ];
    #[cfg(all(feature = "fi", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 61.92410999999999;
        pub const LONGITUDE: f64 = 25.7481511;
        pub const MAX_LATITUDE: f64 = 70.0922932;
        pub const MAX_LONGITUDE: f64 = 31.5870999;
        pub const MIN_LATITUDE: f64 = 59.693623;
        pub const MIN_LONGITUDE: f64 = 20.4565002;
        pub const NORTHEAST_LATITUDE: f64 = 70.0922932;
        pub const NORTHEAST_LONGITUDE: f64 = 31.5870999;
        pub const SOUTHWEST_LATITUDE: f64 = 59.693623;
        pub const SOUTHWEST_LONGITUDE: f64 = 20.4565002;
    }
}
#[cfg(all(feature = "fi", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 61.92410999999999,
            longitude: 25.7481511,
            max_latitude: 70.0922932,
            max_longitude: 31.5870999,
            min_latitude: 59.693623,
            min_longitude: 20.4565002,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 70.0922932,
                    longitude: 31.5870999,
                },
                southwest: CountryGeoBound {
                    latitude: 59.693623,
                    longitude: 20.4565002,
                },
            },
        }
    }
}

#[cfg(all(feature = "fi", feature = "subdivisions"))]
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
                        country_alpha2: Alpha2::FI,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(60.177002), longitude: Some(19.915002), max_latitude: Some(60.6547739), min_latitude: Some(59.9686438), max_longitude: Some(21.1111317), min_longitude: Some(19.4613939)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄃𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134} 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄥\u{11134}"), ("en", "Åland Islands")]),
                        unofficial_name_list: ["Åland"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::FI,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كاريليا الجنوبية"), ("bn", "দক\u{9cd}ষিন ক\u{9be}রেলিয\u{9bc}\u{9be}"), ("ca", "Carèlia Meridional"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄇𑄢𑄬𑄣\u{11128}𑄠"), ("ceb", "Etelä-Karjala"), ("cs", "Jižní Karélie"), ("da", "Södra Karelen"), ("de", "Südkarelien"), ("el", "Νότια Καρέλια"), ("en", "South Karelia"), ("es", "Karelia del Sur"), ("et", "Lõuna-Karjala"), ("eu", "Hego Karelia"), ("fa", "کارلیای جنوبی"), ("fi", "Etelä-Karjalan maakunta"), ("fr", "Carélie du Sud"), ("ga", "An Chairéil Theas"), ("gl", "Carelia do Sur"), ("gu", "સાઉથ કાર\u{ac7}લિયા"), ("he", "דרום קרליה"), ("hi", "दक\u{94d}षिण क\u{947}रलिया"), ("hy", "Հարավային Կարելիա"), ("id", "Karelia Selatan"), ("it", "Carelia meridionale"), ("ja", "南カルヤラ県"), ("ka", "სამხრეთი კარელია"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಕರೇಲ\u{cbf}ಯಾ"), ("ko", "남카리알라 지역"), ("lt", "Pietų Karelija"), ("lv", "Dienvidkarēlija"), ("mr", "दक\u{94d}षिण क\u{947}र\u{947}लिया"), ("ms", "Karelia Selatan"), ("nb", "Södra Karelen"), ("nl", "Etelä-Karjala"), ("no", "Södra Karelen"), ("pl", "Karelia Południowa"), ("pt", "Carélia do Sul"), ("ro", "Carelia de Sud"), ("ru", "Южная Карелия"), ("si", "දක\u{dd4}ණ\u{dd4} කරෙල\u{dd2}ය\u{dcf}"), ("sk", "Etelä-Karjala"), ("sq", "Karelia Jugore"), ("sr", "Јужна Карелија"), ("sr_Latn", "Južna Karelija"), ("sv", "Södra Karelen"), ("ta", "தெற\u{bcd}கு கரேலிய\u{bbe}"), ("te", "దక\u{c4d}ష\u{c3f}ణ కర\u{c47}ల\u{c3f}య\u{c3e}"), ("th", "คาเรเล\u{e35}ย"), ("tr", "Güney Karelya"), ("uk", "Південна Карелія"), ("ur", "جنوبی کاریلیا"), ("vi", "Nam Karelia"), ("zh", "南卡累利阿區")]),
                        unofficial_name_list: ["Södra Karelen"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::FI,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بوهيانما الجنوبية"), ("be", "Паўднёвая Астработнія"), ("bn", "দক\u{9cd}ষিণ ওস\u{9cd}ট\u{9cd}রোবোথনিয\u{9bc}\u{9be}"), ("ca", "Ostrobòtnia del Sud"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄃\u{11127}𑄌\u{11134}𑄑\u{11133}𑄢\u{1112e}𑄝\u{1112e}𑄖\u{11134}𑄚\u{11128}𑄠"), ("ceb", "Etelä-Pohjanmaa"), ("cs", "Jižní Pohjanmaa"), ("da", "Södra Österbotten"), ("de", "Südösterbotten"), ("el", "Σάουθερν Οστρομπόρθνια"), ("en", "Southern Ostrobothnia"), ("es", "Ostrobotnia del Sur"), ("et", "Lõuna-Pohjanmaa"), ("eu", "Hego Ostrobotnia"), ("fa", "اوستروبوتنیای جنوبی"), ("fi", "Etelä-Pohjanmaan maakunta"), ("fr", "Ostrobotnie du Sud"), ("gl", "Ostrobotnia do Sur"), ("gu", "સધર\u{acd}ન ઓસ\u{acd}ટ\u{acd}રોબોથનિયા"), ("hi", "दक\u{94d}षिणी ऑस\u{94d}ट\u{94d}रोबोथनिया"), ("hy", "Հարավային Օստրոբոտնիա"), ("id", "Ostrobothnia Selatan"), ("it", "Ostrobotnia meridionale"), ("ja", "南ポフヤンマー県"), ("ka", "სამხრეთი ოსტრობოტნია"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಓಸ\u{ccd}ಟ\u{ccd}ರೊಬೋಥ\u{ccd}ನ\u{cbf}ಯಾ"), ("ko", "남포흐얀마 지역"), ("lt", "Pietų Pohjanma"), ("lv", "Dienvidpohjanmā"), ("mr", "साऊथर\u{94d}न ओस\u{94d}ट\u{94d}रोबॉथनिआ"), ("ms", "Southern Ostrobothnia"), ("nb", "Södra Österbotten"), ("nl", "Etelä-Pohjanmaa"), ("no", "Södra Österbotten"), ("pl", "Etelä-Pohjanmaa"), ("pt", "Ostrobótnia do Sul"), ("ro", "Ostrobotnia de Sud"), ("ru", "Южная Остроботния"), ("si", "දක\u{dd4}ණ\u{dd4} ඔස\u{dca}ට\u{dca}රෝබෝත\u{dca}න\u{dd2}ය\u{dcf}"), ("sk", "Etelä-Pohjanmaa"), ("sq", "Ostrobothnia Jugore"), ("sr", "Јужна Остроботнија"), ("sr_Latn", "Južna Ostrobotnija"), ("sv", "Södra Österbotten"), ("ta", "தெற\u{bcd}கு ஆஸ\u{bcd}ட\u{bcd}ரோபோத\u{bcd}நிய\u{bbe}"), ("te", "దక\u{c4d}ష\u{c3f}ణ ఓస\u{c4d}ట\u{c4d}ర\u{c4b}బ\u{c4b}త\u{c3f}న\u{c3e}"), ("th", "เซาเท\u{e34}ร\u{e4c}น ออสโตรบอธเน\u{e35}ย"), ("tr", "Güney Ostrobothnia"), ("uk", "Південна Погʼянмаа"), ("ur", "جنوبی اوستروبوثنیہ"), ("vi", "Phía Nam Ostrobothnia")]),
                        unofficial_name_list: ["Södra Österbotten"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::FI,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سافو الجنوبية"), ("be", "Паўднёвае Сава"), ("bn", "দক\u{9cd}ষিণ সভোনিয\u{9bc}\u{9be}"), ("ca", "Savònia del Sud"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄥𑄞\u{1112e}𑄚\u{11128}𑄠"), ("ceb", "Southern Savonia"), ("cs", "Jižní Savo"), ("da", "Södra Savolax"), ("de", "Südsavo"), ("el", "Βόρεια Σαβονία"), ("en", "Southern Savonia"), ("es", "Savonia del Sur"), ("et", "Lõuna-Savo"), ("eu", "Hego Savonia"), ("fa", "ساوونیای جنوبی"), ("fi", "Etelä-Savon maakunta"), ("fr", "Savonie du Sud"), ("gl", "Savonia do Sur"), ("gu", "સધર\u{acd}ન સ\u{ac7}વોનિયા"), ("hi", "दक\u{94d}षिणी सवोनिया"), ("hu", "Dél-Savo"), ("hy", "Հարավային Սավո"), ("id", "Savonia Selatan"), ("it", "Savo meridionale"), ("ja", "南サヴォ県"), ("ka", "სამხრეთი სავონია"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಸವೊನ\u{cbf}ಯಾ"), ("ko", "남사보 지역"), ("lt", "Pietų Savas"), ("lv", "Dienvidsavo"), ("mr", "साउदर\u{94d}न सवोनो"), ("ms", "Savonia Selatan"), ("nb", "Södra Savolax"), ("nl", "Etelä-Savo"), ("no", "Södra Savolax"), ("pl", "Etelä-Savo"), ("pt", "Savônia do Sul"), ("ro", "Savonia de Sud"), ("ru", "Южное Саво"), ("si", "දක\u{dd4}ණ\u{dd4} සැවොන\u{dd2}ය\u{dcf}"), ("sk", "Etelä-Savo"), ("sq", "Savonia Jugore"), ("sr", "Јужна Савонија"), ("sr_Latn", "Južna Savonija"), ("sv", "Södra Savolax"), ("ta", "சதன\u{bcd} சவோனிய\u{bbe}"), ("te", "దక\u{c4d}ష\u{c3f}ణ స\u{c3e}వ\u{c4b}న\u{c3f}య\u{c3e}"), ("th", "เซาร\u{e4c}เท\u{e34}ร\u{e4c}น ซาโวเน\u{e35}ย"), ("tr", "Güney Savonya"), ("uk", "Південна Савонія"), ("ur", "سدرن ساوونیا"), ("vi", "Phía Nam Savonia")]),
                        unofficial_name_list: ["Södra Savolax"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::FI,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كاينو"), ("be", "Кайнуу"), ("bn", "ক\u{9be}ইন\u{9cd}য\u{9c1}"), ("ca", "Kainuu"), ("ccp", "𑄇\u{1112d}𑄚\u{1112a}"), ("ceb", "Kainuu"), ("cs", "Kainuu"), ("da", "Kajanaland"), ("de", "Kainuu"), ("el", "Καϊνού"), ("en", "Kainuu"), ("es", "Kainuu"), ("et", "Kainuu"), ("eu", "Kainuu"), ("fa", "کاینو"), ("fi", "Kainuun maakunta"), ("fr", "Kainuu"), ("ga", "Kainuu"), ("gl", "Kainuu"), ("gu", "ક\u{ac8}ન\u{ac1}"), ("he", "קינואו"), ("hi", "क\u{947}न\u{941}उ"), ("hu", "Kainuu"), ("hy", "Կաինուու"), ("id", "Kainuu"), ("it", "Kainuu"), ("ja", "カイヌー県"), ("ka", "კაინუუ"), ("kn", "ಕೈನು"), ("ko", "카이누 지역"), ("lt", "Kainū"), ("lv", "Kainū"), ("mk", "Каину"), ("mr", "काइनी"), ("ms", "Kainuu"), ("nb", "Kajanaland"), ("nl", "Kainuu"), ("no", "Kajanaland"), ("pl", "Kainuu"), ("pt", "Kainuu"), ("ro", "Kainuu"), ("ru", "Кайнуу"), ("si", "කය\u{dd2}න\u{dd6}"), ("sk", "Kainuu"), ("sq", "Kainuu"), ("sr", "Кајину"), ("sr_Latn", "Kajinu"), ("sv", "Kajanaland"), ("ta", "கைனுக\u{bcd}கு"), ("te", "క\u{c48}నూ"), ("th", "ไคน\u{e39}น\u{e4c}"), ("tr", "Kainuu"), ("uk", "Кайнуу"), ("ur", "کاینو"), ("vi", "Kainuu")]),
                        unofficial_name_list: ["Kajanaland"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::FI,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتا هامي"), ("bn", "ত\u{9be}স\u{9cd}তিয\u{9bc}\u{9be} প\u{9cd}রপ\u{9be}র"), ("ca", "Tavastia Pròpia"), ("ccp", "𑄑𑄞𑄌\u{11134}𑄑\u{11128}𑄠 𑄛\u{11133}𑄢\u{1112e}𑄛𑄢\u{11134}"), ("ceb", "Häme"), ("cs", "Kanta-Häme"), ("da", "Egentliga Tavastland"), ("de", "Kanta-Häme"), ("el", "Ταβάστια Πρόπερ"), ("en", "Tavastia Proper"), ("es", "Tavastia Propia"), ("et", "Kanta-Häme"), ("eu", "Jatorrizko Tavastia"), ("fa", "تاواستیای اصلی"), ("fi", "Kanta-Hämeen maakunta"), ("fr", "Kanta-Häme"), ("gl", "Tavastia Propia"), ("gu", "તવાસ\u{acd}તિયા પ\u{acd}રોપર"), ("hi", "तवास\u{94d}टिया प\u{94d}रॉपर"), ("id", "Tavastia Proper"), ("it", "Kanta-Häme"), ("ka", "კანტა-ჰიამე"), ("kn", "ತವಾಸ\u{ccd}ಟ\u{cbf}ಯಾ ಸರ\u{cbf}ಯಾದ"), ("ko", "칸타헤메 지역"), ("lt", "Vidurio Hemė"), ("lv", "Kantaheme"), ("mr", "तवस\u{94d}त\u{94d}रिया प\u{94d}रॉपर"), ("ms", "Tavastia Proper"), ("nb", "Egentliga Tavastland"), ("nl", "Kanta-Häme"), ("no", "Egentliga Tavastland"), ("pl", "Kanta-Häme"), ("pt", "Tavastia Própria"), ("ro", "Kanta - Häme"), ("ru", "Канта-Хяме"), ("si", "ටවස\u{dca}ට\u{dd2}ය\u{dcf} ප\u{dca}\u{200d}රොපර\u{dca}"), ("sk", "Kanta-Häme"), ("sq", "Tavastia Proper"), ("sr", "Ужа Тавастија"), ("sr_Latn", "Uža Tavastija"), ("sv", "Egentliga Tavastland"), ("ta", "தவஸ\u{bcd}திய\u{bbe} ப\u{bcd}ரோபெர\u{bcd}"), ("te", "టవ\u{c3e}స\u{c4d}ట\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}పర\u{c4d}"), ("th", "ทาวาสเท\u{e35}ย โพรเพอ"), ("tr", "Tavastia Proper"), ("uk", "Канта-Гяме"), ("ur", "تاواستیا پروپر"), ("vi", "Tavastia Proper")]),
                        unofficial_name_list: ["Egentliga Tavastland"].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::FI,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بوهيانما الوسطى"), ("bn", "সেন\u{9cd}ট\u{9cd}র\u{9be}ল অস\u{9cd}ট\u{9cd}রোবোথনিয\u{9bc}\u{9be}"), ("ca", "Ostrobòtnia Central"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11133}𑄢𑄣\u{11134} 𑄃\u{11127}𑄌\u{11134}𑄑\u{11133}𑄢\u{1112e}𑄝\u{1112e}𑄖\u{11134}𑄚\u{11128}𑄠"), ("ceb", "Keski-Pohjanmaa"), ("cs", "Střední Pohjanmaa"), ("da", "Mellersta Österbotten"), ("de", "Mittelösterbotten"), ("el", "Κεντρική Οστρομποθνία (Σέντραλ Οστρομποθνία)"), ("en", "Central Ostrobothnia"), ("es", "Ostrobotnia Central"), ("et", "Kesk-Pohjanmaa"), ("eu", "Erdialdeko Ostrobotnia"), ("fa", "اوستروبوتنیای مرکزی"), ("fi", "Keski-Pohjanmaan maakunta"), ("fr", "Ostrobotnie-Centrale"), ("gl", "Ostrobotnia Central"), ("gu", "સ\u{ac7}ન\u{acd}ટ\u{acd}રલ ઑસ\u{acd}ટ\u{acd}રોબોથનિઆ"), ("he", "אוסטרובוטניה המרכזית"), ("hi", "मध\u{94d}य ओस\u{94d}ट\u{94d}रोबोथनिया"), ("hy", "Կենտրոնական Օստրոբոտնիա"), ("id", "Kotamadya Central"), ("it", "Ostrobotnia centrale"), ("ja", "中部ポフヤンマー県"), ("ka", "ცენტრალური ოსტრობოტნია"), ("kn", "ಸ\u{cc6}ಂಟ\u{ccd}ರಲ\u{ccd} ಓಸ\u{ccd}ಟ\u{ccd}ರೊಬೋಥ\u{ccd}ನ\u{cbf}ಯಾ"), ("ko", "중부 포흐얀마 지역"), ("lt", "Vidurio Pohjanma"), ("lv", "Viduspohjanmā"), ("mr", "स\u{947}\u{902}ट\u{94d}रल ओस\u{94d}ट\u{94d}रोबॉथनिआ"), ("ms", "Central Ostrobothnia"), ("nb", "Mellersta Österbotten"), ("nl", "Keski-Pohjanmaa"), ("no", "Mellersta Österbotten"), ("pl", "Keski-Pohjanmaa"), ("pt", "Ostrobótnia Central"), ("ro", "Ostrobotnia Centrală"), ("ru", "Центральная Остроботния"), ("si", "මද\u{dca}\u{200d}යම ඔස\u{dca}ට\u{dca}රෝබොත\u{dca}න\u{dd2}ය\u{dcf}"), ("sk", "Keski-Pohjanmaa"), ("sq", "Ostrobothnia Qendrore"), ("sr", "Средишња Остроботнија"), ("sr_Latn", "Središnja Ostrobotnija"), ("sv", "Mellersta Österbotten"), ("ta", "சென\u{bcd}ட\u{bcd}ரல\u{bcd} ஒஸ\u{bcd}ட\u{bcd}ரோபோதினிற\u{bcd}"), ("te", "స\u{c46}ంట\u{c4d}రల\u{c4d}\u{c4d} ఓస\u{c4d}ట\u{c4d}ర\u{c4b}బ\u{c4b}త\u{c3f}న\u{c3e}"), ("th", "เซ\u{e47}นทร\u{e31}ลออสโตรบอสเน\u{e35}ย"), ("tr", "Orta Ostrobothnia"), ("uk", "Центральна Погʼянмаа"), ("ur", "وسطی اوستروبوثنیہ"), ("vi", "Miền Trung Ostrobothnia")]),
                        unofficial_name_list: ["Mellersta Österbotten"].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "08",
                        country_alpha2: Alpha2::FI,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "فنلندا الوسطى"), ("be", "Цэнтральная Фінляндыя"), ("bn", "সেন\u{9cd}ট\u{9cd}র\u{9be}ল ফিনল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("ca", "Finlàndia Central"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11133}𑄢𑄣\u{11134} 𑄜\u{11128}𑄚\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Keski-Suomi"), ("cs", "Střední Finsko"), ("da", "Keski-Suomi"), ("de", "Mittelfinnland"), ("el", "Κεντρική Φινλανδία"), ("en", "Central Finland"), ("es", "Finlandia Central"), ("et", "Kesk-Soome"), ("eu", "Erdialdeko Finlandia"), ("fa", "فنلاند مرکزی"), ("fi", "Keski-Suomen maakunta"), ("fr", "Finlande-Centrale"), ("gl", "Finlandia Central"), ("gu", "સ\u{ac7}ન\u{acd}ટ\u{acd}રલ ફિનલ\u{ac7}ન\u{acd}ડ"), ("hi", "स\u{947}\u{902}ट\u{94d}रल फिनल\u{948}\u{902}ड"), ("hy", "Կենտրոնական Ֆինլանդիա"), ("id", "Finlandia Tengah"), ("it", "Finlandia centrale"), ("ja", "中央スオミ県"), ("ka", "ცენტრალური ფინეთი"), ("kn", "ಸ\u{cc6}ಂಟ\u{ccd}ರಲ\u{ccd} ಫ\u{cbf}ನ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd}"), ("ko", "중앙수오미 지역"), ("lt", "Vidurio Suomija"), ("lv", "Vidussomija"), ("mr", "स\u{947}\u{902}ट\u{94d}रल फिनल\u{902}ड"), ("ms", "Finland Tengah"), ("nb", "Mellersta Finland"), ("nl", "Keski-Suomi"), ("no", "Mellersta Finland"), ("pl", "Finlandia Środkowa"), ("pt", "Finlândia Central"), ("ro", "Finlanda Centrală"), ("ru", "Центральная Финляндия"), ("si", "මද\u{dca}\u{200d}යම ෆ\u{dd2}න\u{dca}ලන\u{dca}තය"), ("sk", "Keski-Suomi"), ("sq", "Finlanda Qendrore"), ("sr", "Средишња Финска"), ("sr_Latn", "Središnja Finska"), ("sv", "Mellersta Finland"), ("ta", "சென\u{bcd}ட\u{bcd}ரல\u{bcd} பின\u{bcd}ல\u{bbe}ந\u{bcd}து"), ("te", "స\u{c46}ంట\u{c4d}రల\u{c4d} ఫ\u{c3f}న\u{c4d}ల\u{c3e}ండ\u{c4d}"), ("th", "เซนทร\u{e31}ล ฟ\u{e34}นแลนด\u{e4c}"), ("tr", "Orta Finlandiya"), ("uk", "Центральна Фінляндія"), ("ur", "وسطی فن لینڈ"), ("vi", "Miền Trung Phần Lan")]),
                        unofficial_name_list: ["Mellersta Finland"].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "09",
                        country_alpha2: Alpha2::FI,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كومنلاكسو"), ("be", "Кюменлаакса"), ("bg", "Кйменлаксо"), ("bn", "ক\u{9be}য\u{9bc}\u{9be}মেনল\u{9be}কসো"), ("ca", "Vall de Kymi"), ("ccp", "𑄇\u{1112d}𑄟𑄬𑄚\u{11134}𑄣𑄇\u{11134}𑄥\u{1112e}"), ("ceb", "Kymenlaakso"), ("cs", "Kymenlaakso"), ("da", "Kymmenedalen"), ("de", "Kymenlaakso"), ("el", "Κιμενλαάκσο"), ("en", "Kymenlaakso"), ("es", "Kymenlaakso"), ("et", "Kymenlaakso"), ("eu", "Kymenlaakso"), ("fa", "کیمنلاکسو"), ("fi", "Kymenlaakson maakunta"), ("fr", "Vallée de la Kymi"), ("gl", "Kymenlaakso"), ("gu", "કીમ\u{ac7}નલાક\u{acd}સો"), ("hi", "क\u{94d}यम\u{947}\u{902}लाक\u{94d}सो"), ("id", "Kymenlaakso"), ("it", "Kymenlaakso"), ("ja", "キュメンラークソ県"), ("ka", "კიმენლააკსო"), ("kn", "ಕ\u{ccd}ಯುಮ\u{cc6}ನ\u{ccd}ಲಾಕೊ"), ("ko", "퀴멘락소 지역"), ("lt", "Kiumenlaksas"), ("lv", "Kjumenlākso"), ("mr", "क\u{945}म\u{947}\u{902}ल\u{945}स\u{94d}को"), ("ms", "Kymenlaakso"), ("nb", "Kymmenedalen"), ("nl", "Kymenlaakso"), ("no", "Kymmenedalen"), ("pl", "Kymenlaakso"), ("pt", "Kymenlaakso"), ("ro", "Kymenlaakso"), ("ru", "Кюменлааксо"), ("si", "කය\u{dd2}මෙන\u{dca}ල\u{dcf}ක\u{dca}සෝ"), ("sk", "Kymenlaakso"), ("sq", "Kymenlaakso"), ("sr", "Кименска Долина"), ("sr_Latn", "Kimenska Dolina"), ("sv", "Kymmenedalen"), ("ta", "க\u{bcd}கியமெனள\u{bbe}க\u{bcd}ஸோ"), ("te", "క\u{c48}మ\u{c46}న\u{c4d}ల\u{c3e}స\u{c4d}క\u{c4b}"), ("th", "ไคเมนลาค\u{e4c}โซ"), ("tr", "Kymenlaakso"), ("uk", "Кюменлааксо"), ("ur", "کومنلاکسو"), ("vi", "Kymenlaakso")]),
                        unofficial_name_list: ["Kymmenedalen"].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::FI,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(67.9222304), longitude: Some(26.5046438), max_latitude: Some(70.092444), min_latitude: Some(65.497126), max_longitude: Some(30.016886), min_longitude: Some(20.5489762)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Lapland"), ("ar", "إقليم لابي"), ("be", "Лапландыя"), ("bg", "Лапландия"), ("ca", "Província de Lapònia"), ("ccp", "𑄣𑄛\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Lappi (lalawigan)"), ("cs", "Laponsko"), ("da", "Lappi"), ("de", "Lappland"), ("en", "Lapland"), ("es", "Laponia finlandesa"), ("et", "Lapi maakond"), ("eu", "Lappi"), ("fa", "لاپلند (فنلاند)"), ("fi", "Lapin maakunta"), ("fr", "Laponie"), ("gl", "Laponia, Finlandia"), ("hu", "Lappföld"), ("hy", "Լափլանդ"), ("id", "Laplandia, Finlandia"), ("it", "Regione della Lapponia"), ("ja", "ラッピ県"), ("ka", "ლაპლანდია"), ("ko", "라피 지역"), ("lt", "Lapija"), ("lv", "Lapzeme"), ("mk", "Лапонија"), ("ms", "Lapland, Finland"), ("nb", "Lappland"), ("nl", "Lapland"), ("no", "Lappland"), ("pl", "Laponia"), ("pt", "Lapônia"), ("ro", "Laponia"), ("ru", "Лапландия"), ("sk", "Laponsko"), ("sq", "Lapland"), ("sr", "Лапонија"), ("sr_Latn", "Laponija"), ("sv", "Lappland"), ("tr", "Lappi"), ("uk", "Лапландія"), ("ur", "لاپلند"), ("vi", "Lapland")]),
                        unofficial_name_list: ["Laponie", "Lappi", "Lappland", "Lappland"].to_vec(),
                    }
                ),
                (
                    "11",
                    Subdivision{
                        name: "11",
                        country_alpha2: Alpha2::FI,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بيركنما"), ("be", "Пірканмаа"), ("bg", "Пирканма"), ("bn", "স\u{9be}ম\u{9c1}তে সোমখন\u{9cd}দ"), ("ca", "Pirkanmaa"), ("ccp", "𑄛\u{11128}𑄢\u{11134}𑄇𑄚\u{11134}𑄟\u{11133}𑄦"), ("ceb", "Pirkanmaa"), ("cs", "Pirkanmaa"), ("da", "Birkaland"), ("de", "Pirkanmaa"), ("el", "Πιρκάνμαα"), ("en", "Pirkanmaa"), ("es", "Pirkanmaa"), ("et", "Pirkanmaa"), ("eu", "Pirkanmaa"), ("fa", "پیرکانما"), ("fi", "Pirkanmaan maakunta"), ("fr", "Pirkanmaa"), ("ga", "Pirkanmaa"), ("gl", "Pirkanmaa"), ("gu", "પિર\u{acd}કાનમા"), ("hi", "परकनमा"), ("hu", "Pirkanmaa"), ("hy", "Պիրկանմաա"), ("id", "Pirkanmaa"), ("it", "Pirkanmaa"), ("ja", "ピルカンマー県"), ("ka", "პირკანმაა"), ("kn", "ಪ\u{cbf}ರ\u{ccd}ಕಾನ\u{ccd}ಮಾ"), ("ko", "피르칸마 지역"), ("lt", "Pirkanma"), ("lv", "Pirkanmā"), ("mr", "पर\u{94d}कमामा"), ("ms", "Pirkanmaa"), ("nb", "Birkaland"), ("nl", "Pirkanmaa"), ("no", "Birkaland"), ("pl", "Pirkanmaa"), ("pt", "Pirkanmaa"), ("ro", "Pirkanmaa"), ("ru", "Пирканмаа"), ("si", "පර\u{dca}කන\u{dca}ම\u{dcf}"), ("sk", "Pirkanmaa"), ("sq", "Pirkanmaa"), ("sr", "Пирканска земља"), ("sr_Latn", "Pirkanska zemlja"), ("sv", "Birkaland"), ("ta", "பிரகன\u{bcd}ம\u{bbe}"), ("te", "ప\u{c3f}ర\u{c4d}కన\u{c4d}మ\u{c3e}"), ("th", "เพอคานมา"), ("tr", "Pirkanmaa"), ("uk", "Пірканмаа"), ("ur", "پیرکانما"), ("vi", "Pirkanmaa")]),
                        unofficial_name_list: ["Birkaland"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "12",
                        country_alpha2: Alpha2::FI,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بوهيانما"), ("be", "Астработнія"), ("ca", "Regió d’Ostrobòtnia"), ("ccp", "𑄃\u{11127}𑄌\u{11134}𑄑\u{11133}𑄢\u{1112e}𑄝\u{1112e}𑄖\u{11134}𑄚\u{11128}𑄠"), ("ceb", "Pohjanmaa"), ("cs", "Pohjanmaa"), ("da", "Österbotten"), ("de", "Österbotten"), ("en", "Ostrobothnia"), ("es", "Ostrobotnia"), ("et", "Pohjanmaa"), ("eu", "Ostrobotnia eskualdea"), ("fa", "اوستروبوتنیا"), ("fi", "Pohjanmaan maakunta"), ("fr", "Ostrobotnie"), ("gl", "Ostrobotnia"), ("he", "אוסטרובוטניה"), ("hu", "Pohjanmaa"), ("it", "Ostrobotnia"), ("ja", "ポフヤンマー県"), ("ka", "ოსტრობოტნია"), ("ko", "포흐얀마 지역"), ("lt", "Pohjanma"), ("nb", "Österbotten"), ("nl", "Österbotten"), ("no", "Österbotten"), ("pl", "Pohjanmaa"), ("pt", "Ostrobótnia"), ("ro", "Ostrobotnia"), ("ru", "Остроботния"), ("sk", "Pohjanmaa"), ("sq", "Ostrobothnia"), ("sr", "Остроботнија"), ("sr_Latn", "Ostrobotnija"), ("sv", "Österbotten"), ("tr", "Ostrobothnia"), ("uk", "Погʼянмаа"), ("ur", "اوستروبوثنیہ")]),
                        unofficial_name_list: ["Österbotten"].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "13",
                        country_alpha2: Alpha2::FI,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كاريليا الشمالية"), ("bn", "ক\u{9be}রেলিয\u{9bc}\u{9be}"), ("ca", "Carèlia Septentrional"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄇𑄬𑄢𑄬𑄣\u{11128}𑄚"), ("ceb", "Pohjois-Karjala"), ("cs", "Severní Karélie"), ("da", "Norra Karelen"), ("de", "Nordkarelien"), ("el", "Νορθ Καρέλια"), ("en", "North Karelia"), ("es", "Carelia del Norte"), ("et", "Põhja-Karjala"), ("eu", "Ipar Karelia"), ("fa", "کارلیای شمالی"), ("fi", "Pohjois-Karjalan maakunta"), ("fr", "Carélie du Nord"), ("gl", "Carelia do Norte"), ("gu", "નોર\u{acd}થ ક\u{ac7}ર\u{ac7}લિયા"), ("he", "צפון קרליה"), ("hi", "उत\u{94d}तरी कर\u{947}लिया"), ("hu", "Észak-Karjala"), ("hy", "Հյուսիսային Կարելիա"), ("id", "North Karelia"), ("it", "Carelia settentrionale"), ("ja", "北カルヤラ県"), ("ka", "ჩრდილოეთი კარელია"), ("kn", "ಉತ\u{ccd}ತರ ಕರೇಲ\u{cbf}ಯಾ"), ("ko", "북카리알라 지역"), ("lt", "Šiaurės Karelija"), ("lv", "Ziemeļkarēlija"), ("mk", "Северна Карелија"), ("mr", "नॉर\u{94d}थ क\u{947}र\u{947}लिया"), ("ms", "North Karelia"), ("nb", "Norra Karelen"), ("nl", "Pohjois-Karjala"), ("no", "Norra Karelen"), ("pl", "Karelia Północna"), ("pt", "Carélia do Norte"), ("ro", "Carelia de Nord"), ("ru", "Северная Карелия"), ("si", "උත\u{dd4}ර\u{dd4} කරෙල\u{dd2}ය\u{dcf}"), ("sk", "Pohjois-Karjala"), ("sq", "Karelia Veriore"), ("sr", "Северна Карелија"), ("sr_Latn", "Severna Karelija"), ("sv", "Norra Karelen"), ("ta", "வடக\u{bcd}கு கரேலிய\u{bbe}"), ("te", "న\u{c3e}ర\u{c4d}త\u{c4d} కర\u{c47}ల\u{c3f}య\u{c3e}"), ("th", "นอร\u{e4c}ทแคร\u{e4c}เร\u{e35}ย"), ("tr", "Kuzey Karelya"), ("uk", "Північна Карелія"), ("ur", "شمالی کاریلیا"), ("vi", "Bắc Karelia")]),
                        unofficial_name_list: ["Norra Karelen"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::FI,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بوهيانما الشمالية"), ("be", "Паўночная Астработнія"), ("bn", "নর\u{9cd}দ\u{9be}ন ওস\u{9cd}ট\u{9cd}রোবোথনিয\u{9bc}\u{9be}"), ("ca", "Ostrobòtnia del Nord"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄃\u{11127}𑄌\u{11134}𑄑\u{11133}𑄢\u{1112e}𑄝\u{1112e}𑄖\u{11134}𑄚\u{11128}𑄠"), ("ceb", "Pohjois-Pohjanmaa"), ("cs", "Severní Pohjanmaa"), ("da", "Norra Österbotten"), ("de", "Nordösterbotten"), ("el", "Βόρεια Οστροβόθνια"), ("en", "Northern Ostrobothnia"), ("es", "Ostrobotnia del Norte"), ("et", "Põhja-Pohjanmaa"), ("eu", "Ipar Ostrobotnia"), ("fa", "اوستروبوتنیای شمالی"), ("fi", "Pohjois-Pohjanmaan maakunta"), ("fr", "Ostrobotnie du Nord"), ("gl", "Ostrobotnia do Norte"), ("gu", "નધર\u{acd}ન ઓસ\u{acd}ટ\u{acd}રોબોથનિયા"), ("he", "אוסטרובוטניה הצפונית"), ("hi", "उत\u{94d}तरी ओस\u{94d}ट\u{94d}रोबोथनिया"), ("hu", "Észak-Pohjanmaa"), ("hy", "Հյուսիսային Օստրոբոտնիա"), ("id", "Ostrobothnia Utara"), ("it", "Ostrobotnia settentrionale"), ("ja", "北ポフヤンマー県"), ("ka", "ჩრდილოეთი ოსტრობოტნია"), ("kn", "ಉತ\u{ccd}ತರ ಓಸ\u{ccd}ಟ\u{ccd}ರೊಬೋಥ\u{ccd}ನ\u{cbf}ಯಾ"), ("ko", "북포흐얀마 지역"), ("lt", "Šiaurės Pohjanma"), ("lv", "Ziemeļpohjanmā"), ("mk", "Северна Остроботнија"), ("mr", "उत\u{94d}तर ओस\u{94d}ट\u{94d}रोबॉथनिआ"), ("ms", "Ostrobothnia Utara"), ("nb", "Norra Österbotten"), ("nl", "Pohjois-Pohjanmaa"), ("no", "Norra Österbotten"), ("pl", "Pohjois-Pohjanmaa"), ("pt", "Ostrobótnia do Norte"), ("ro", "Ostrobotnia de Nord"), ("ru", "Северная Остроботния"), ("si", "ත\u{dd4}ර\u{dd4} ඔස\u{dca}ත\u{dca}රෝබෝත\u{dca}න\u{dd2}ය\u{dcf}"), ("sk", "Pohjois-Pohjanmaa"), ("sq", "Ostrobothnia Veriore"), ("sr", "Северна Остроботнија"), ("sr_Latn", "Severna Ostrobotnija"), ("sv", "Norra Österbotten"), ("ta", "வடக\u{bcd}கு ஒஸ\u{bcd}ட\u{bcd}ரோபோதினிய\u{bbe}"), ("te", "ఉత\u{c4d}త\u{c4d}తర ఓస\u{c4d}ట\u{c4d}ర\u{c4b}బ\u{c4b}త\u{c4d}న\u{c3f}య\u{c3e}"), ("th", "นอร\u{e4c}ทเท\u{e34}ร\u{e4c}น ออสโทรบอทเน\u{e35}ย"), ("tr", "Kuzey Ostrobothnia"), ("uk", "Північна Погʼянмаа"), ("ur", "شمالی اوستروبوثنیہ"), ("vi", "Phía Bắc Ostrobothnia")]),
                        unofficial_name_list: ["Norra Österbotten"].to_vec(),
                    }
                ),
                (
                    "15",
                    Subdivision{
                        name: "15",
                        country_alpha2: Alpha2::FI,
                        code: "15",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سافو الشمالية"), ("be", "Паўночнае Сава"), ("bn", "নর\u{9cd}দ\u{9be}ন স\u{9cd}য\u{9be}ভোনিয\u{9bc}\u{9be}"), ("ca", "Savònia del Nord"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄥\u{11127}𑄞\u{1112e}𑄚\u{11128}𑄠"), ("ceb", "Pohjois-Savo"), ("cs", "Severní Savo"), ("da", "Norra Savolax"), ("de", "Nordsavo"), ("el", "Βόρεια Σαβονία²"), ("en", "Northern Savonia"), ("es", "Savonia del Norte"), ("et", "Põhja-Savo"), ("eu", "Ipar Savonia"), ("fa", "ساوونیای شمالی"), ("fi", "Pohjois-Savon maakunta"), ("fr", "Savonie du Nord"), ("gl", "Savonia do Norte"), ("gu", "નોર\u{acd}ધન સાવોનિયા"), ("he", "סאבו הצפונית"), ("hi", "उत\u{94d}तरी सवोनिया"), ("hu", "Észak-Savo"), ("hy", "Հյուսիսային Սավո"), ("id", "Savonia Utara"), ("it", "Savo settentrionale"), ("ja", "北サヴォ県"), ("ka", "ჩრდილოეთი სავონია"), ("kn", "ಉತ\u{ccd}ತರ ಸವೊನ\u{cbf}ಯಾ"), ("ko", "북사보 지역"), ("lt", "Šiaurės Savas"), ("lv", "Ziemeļsavo"), ("mk", "Северна Савонија"), ("mr", "नॉर\u{94d}दर\u{94d}न सवोनो"), ("ms", "Northern Savonia"), ("nb", "Norra Savolax"), ("nl", "Pohjois-Savo"), ("no", "Norra Savolax"), ("pl", "Pohjois-Savo"), ("pt", "Savônia do Norte"), ("ro", "Savonia de Nord"), ("ru", "Северное Саво"), ("si", "උත\u{dd4}ර\u{dd4} සවොන\u{dd2}ය\u{dcf}"), ("sk", "Pohjois-Savo"), ("sq", "Savonia Veriore"), ("sr", "Северна Савонија"), ("sr_Latn", "Severna Savonija"), ("sv", "Norra Savolax"), ("ta", "வடக\u{bcd}கு சவோனிய\u{bbe}"), ("te", "ఉత\u{c4d}త\u{c4d}తర సవ\u{c4b}న\u{c3f}య\u{c3e}"), ("th", "นอร\u{e4c}เท\u{e34}ร\u{e4c}นซาโวเน\u{e35}ย"), ("tr", "Kuzey Savonya"), ("uk", "Північна Савонія"), ("ur", "شمالی ساوونیا"), ("vi", "Phía Bắc Savonia")]),
                        unofficial_name_list: ["Norra Savolax"].to_vec(),
                    }
                ),
                (
                    "16",
                    Subdivision{
                        name: "16",
                        country_alpha2: Alpha2::FI,
                        code: "16",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بايات هامي"), ("be", "Пяят-Хямэ"), ("bn", "প\u{9be}ইজ\u{9be}ন ত\u{9be}ভ\u{9be}স\u{9cd}তিয\u{9bc}\u{9be}"), ("ca", "Päijät-Häme"), ("ccp", "𑄛\u{1112d}𑄎𑄚𑄬 𑄑𑄞𑄌\u{11134}𑄑\u{11128}𑄠"), ("ceb", "Päijänne-Tavastland"), ("cs", "Päijät-Häme"), ("da", "Päijänne-Tavastland"), ("de", "Päijät-Häme"), ("el", "Παϊτζάνε Ταβάστια"), ("en", "Päijänne Tavastia"), ("es", "Päijänne Tavastia"), ("et", "Päijät-Häme"), ("eu", "Päijänne Tavastia"), ("fa", "پی\u{200c}ینه تاواستیا"), ("fi", "Päijät-Hämeen maakunta"), ("fr", "Päijät-Häme"), ("ga", "Päijät-Häme"), ("gl", "Päijänne Tavastia"), ("gu", "પ\u{ac7}જન\u{ac7} તાવસ\u{acd}તિયા"), ("hi", "प\u{947}य\u{947}न तवास\u{94d}तिया"), ("hy", "Պյայատ-Հյամե"), ("id", "Päijänne Tavastia"), ("it", "Päijät-Häme"), ("ka", "პიაიატ-ჰიამე"), ("kn", "ಪೈಜೇನ\u{ccd} ಟ\u{cbf}ವಸ\u{ccd}ಟ\u{cbf}ಯಾ"), ("ko", "페이예트헤메 지역"), ("lt", "Peijenės Hemė"), ("lv", "Peijetheme"), ("mr", "प\u{947}जनन तावास\u{94d}तिया"), ("ms", "Paijanne Tavastia"), ("nb", "Päijänne-Tavastland"), ("nl", "Päijät-Häme"), ("no", "Päijänne-Tavastland"), ("pl", "Päijät-Häme"), ("pt", "Päijänne Tavastia"), ("ro", "Päijänne Tavastia"), ("ru", "Пяйят-Хяме"), ("si", "පය\u{dd2}ජන\u{dca}නේ ටවස\u{dca}ට\u{dd2}ය\u{dcf}"), ("sk", "Päijät-Häme"), ("sq", "Päijänne Tavastia"), ("sr", "Пејенска Тавастија"), ("sr_Latn", "Pejenska Tavastija"), ("sv", "Päijänne-Tavastland"), ("ta", "பைஜன\u{bcd}னே டவசதிய\u{bbe}"), ("te", "ప\u{c3e}య\u{c3f}జ\u{c3e}న\u{c4d} ట\u{c3e}వ\u{c3e}స\u{c4d}ట\u{c3f}య\u{c3e}"), ("th", "ไปจานน\u{e35} ทาวาสเท\u{e35}ย"), ("tr", "Päijänne Tavastia"), ("uk", "Пяйят-Гяме"), ("ur", "پائینے تاواستیا"), ("vi", "Päijänne Tavastia")]),
                        unofficial_name_list: ["Päijänne-Tavastland"].to_vec(),
                    }
                ),
                (
                    "17",
                    Subdivision{
                        name: "17",
                        country_alpha2: Alpha2::FI,
                        code: "17",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ستاكونتا"), ("az", "Satakunta"), ("be", "Сатакунта"), ("bg", "Сатакунта"), ("bn", "স\u{9be}ট\u{9be}ক\u{9c1}ন\u{9cd}ট\u{9be}"), ("bs", "Satakunta"), ("ca", "Satakunta"), ("ccp", "𑄥𑄑𑄇\u{1112a}𑄚\u{11134}𑄑"), ("ceb", "Satakunta"), ("cs", "Satakunta"), ("da", "Satakunda"), ("de", "Satakunta"), ("el", "Σατακούντα"), ("en", "Satakunta"), ("es", "Satakunta"), ("et", "Satakunta"), ("eu", "Satakunta"), ("fa", "ساتاکونتا"), ("fi", "Satakunnan maakunta"), ("fr", "Satakunta"), ("ga", "Satakunta"), ("gl", "Satakunta"), ("gu", "સતક\u{ac1}\u{a82}તા"), ("hi", "सताक\u{941}\u{902}ता"), ("hu", "Satakunta tartomány"), ("hy", "Սատակունա"), ("id", "Satakunta"), ("it", "Satakunta"), ("ja", "サタクンタ県"), ("ka", "სატაკუნტა"), ("kn", "ಸತಕುಂತ"), ("ko", "사타쿤타 지역"), ("lt", "Satakunta"), ("lv", "Satakunta"), ("mr", "सातक\u{941}\u{902}डा"), ("ms", "Satakunta"), ("nb", "Satakunta"), ("nl", "Satakunta"), ("no", "Satakunta"), ("pl", "Satakunta"), ("pt", "Satakunta"), ("ro", "Satakunta"), ("ru", "Сатакунта"), ("si", "සටක\u{dd4}න\u{dca}ට\u{dcf}"), ("sk", "Satakunta"), ("sq", "Satakunta"), ("sr", "Сатакунта"), ("sr_Latn", "Satakunta"), ("sv", "Satakunta"), ("ta", "ச\u{bbe}ட\u{bcd}டைக\u{bcd}குண\u{bcd}ட\u{bbe}"), ("te", "స\u{c3e}టుకుంట"), ("th", "ซาตาค\u{e38}นคา"), ("tr", "Satakunta"), ("uk", "Сатакунта"), ("ur", "ساتاکونتا"), ("vi", "Satakunta")]),
                        unofficial_name_list: ["Satakunda"].to_vec(),
                    }
                ),
                (
                    "18",
                    Subdivision{
                        name: "18",
                        country_alpha2: Alpha2::FI,
                        code: "18",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Uusimaa"), ("ar", "أوسيما"), ("be", "Усіма"), ("bg", "Уусимаа"), ("bn", "উসিম\u{9cd}য\u{9be}"), ("ca", "Uusimaa"), ("ccp", "𑄅\u{1112b}\u{1112a}𑄥\u{11128}𑄟\u{11133}𑄦"), ("ceb", "Uusimaa"), ("cs", "Uusimaa"), ("da", "Nyland"), ("de", "Uusimaa"), ("el", "Ουουσίμαα"), ("en", "Uusimaa"), ("es", "Uusimaa"), ("et", "Uusimaa"), ("eu", "Nilandia"), ("fa", "اوسیما"), ("fi", "Uudenmaan maakunta"), ("fr", "Uusimaa"), ("ga", "An Nualainn"), ("gl", "Uusimaa"), ("gu", "ઉસિમા"), ("he", "אוסימה"), ("hi", "उशिमा"), ("hu", "Nyland tartomány"), ("hy", "Ուուսիմաա"), ("id", "Uusimaa"), ("it", "Uusimaa"), ("ja", "ウーシマー県"), ("ka", "უუსიმაა"), ("kn", "ಯುಸ\u{cbf}ಮಾಮಾ"), ("ko", "우시마 지역"), ("lt", "Ūsima"), ("lv", "Ūsimā"), ("mk", "Усима"), ("mr", "उशिमा"), ("ms", "Uusimaa"), ("nb", "Nyland"), ("nl", "Uusimaa"), ("no", "Nyland"), ("pl", "Uusimaa"), ("pt", "Uusimaa"), ("ro", "Uusimaa"), ("ru", "Уусимаа"), ("si", "ඌස\u{dd2}ම\u{dcf}"), ("sk", "Uusimaa"), ("sq", "Uusimaa"), ("sr", "Нова Земља"), ("sr_Latn", "Nova Zemlja"), ("sv", "Nyland"), ("ta", "யூசிம\u{bbe}"), ("te", "యూస\u{c3f}మ\u{c3e}"), ("th", "ย\u{e39}ย\u{e39}ส\u{e34}ม\u{e48}า"), ("tr", "Uusimaa"), ("uk", "Уусімаа"), ("ur", "وسیما"), ("vi", "Uusimaa")]),
                        unofficial_name_list: ["Nyland"].to_vec(),
                    }
                ),
                (
                    "19",
                    Subdivision{
                        name: "19",
                        country_alpha2: Alpha2::FI,
                        code: "19",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "فارسينايس سوومي"), ("be", "Паўднёва-Заходняя Фінляндыя"), ("bn", "ফিনল\u{9cd}য\u{9be}ন\u{9cd}ড প\u{9cd}রপ\u{9be}র"), ("ca", "Finlàndia Pròpia"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄜\u{11128}𑄚\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Varsinais-Suomi"), ("cs", "Vlastní Finsko"), ("da", "Egentliga Finland"), ("de", "Varsinais-Suomi"), ("el", "Νοτιοδυτική Φινλανδία"), ("en", "Southwest Finland"), ("es", "Finlandia Propia"), ("et", "Päris-Soome"), ("eu", "Jatorrizko Finlandia"), ("fa", "فنلاند اصلی"), ("fi", "Varsinais-Suomen maakunta"), ("fr", "Finlande du Sud-Ouest"), ("gl", "Finlandia Propia"), ("gu", "ફિનલ\u{ac7}ન\u{acd}ડ પ\u{acd}રોપર"), ("hi", "फ\u{93c}िनल\u{948}\u{902}ड प\u{94d}रॉपर"), ("id", "Finland Proper"), ("it", "Varsinais-Suomi"), ("ja", "南西スオミ県"), ("ka", "ვარსინაის-სუომი"), ("kn", "ಫ\u{cbf}ನ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd} ಸರ\u{cbf}ಯಾದ"), ("ko", "남서수오미 지역"), ("lt", "Pietvakarių Suomija"), ("lv", "Dienvidrietumsomija"), ("mr", "फिनल\u{902}ड प\u{94d}रॉपर"), ("ms", "Finland Proper"), ("nb", "Egentliga Finland"), ("nl", "Varsinais-Suomi"), ("no", "Egentliga Finland"), ("pl", "Varsinais-Suomi"), ("pt", "Finlândia Própria"), ("ro", "Finlanda Propriu-Zisă"), ("ru", "Варсинайс-Суоми"), ("si", "ෆ\u{dd2}න\u{dca}ලන\u{dca}ත ප\u{dca}\u{200d}රොපර\u{dca}"), ("sk", "Varsinais-Suomi"), ("sq", "Finlanda Jugperëndimore"), ("sr", "Ужа Финска"), ("sr_Latn", "Uža Finska"), ("sv", "Egentliga Finland"), ("ta", "பின\u{bcd}ல\u{bbe}ந\u{bcd}து ப\u{bcd}ரொபேர\u{bcd}"), ("te", "ఫ\u{c3f}న\u{c4d}ల\u{c3e}ండ\u{c4d} ప\u{c4d}ర\u{c3e}పర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดพะเยา"), ("tr", "Güneybatı Finlandiya"), ("uk", "Південно-західна Фінляндія"), ("ur", "جنوب مغربی فن لینڈ"), ("vi", "Finland Proper")]),
                        unofficial_name_list: ["Egentliga Finland"].to_vec(),
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
#[cfg(feature = "fi")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::FI,
        alpha3: Alpha3::FIN,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 358,
        currency_code: "EUR",
        gec: Some(GEC::FI),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("FIN"),
        iso_long_name: "The Republic of Finland",
        iso_short_name: "Finland",
        official_language_list: ["fi", "sv"].to_vec(),
        spoken_language_list: ["fi", "sv"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "0",
        nationality: Some("Finnish"),
        number: "246",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::NorthernEurope),
        un_locode: "FI",
        unofficial_name_list: [
            "Finland",
            "Finnland",
            "Finlande",
            "Finlandia",
            "フィンランド",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Finland"),
            ("af", "Finland"),
            ("ak", "Finland"),
            ("am", "ፑንሒን፥"),
            ("an", "Finland"),
            ("ar", "فنلندا"),
            ("as", "ফিনলেণ\u{9cd}ড"),
            ("ay", "Finland"),
            ("az", "Finlandiya"),
            ("ba", "Finland"),
            ("be", "Фінляндыя"),
            ("bg", "Финландия"),
            ("bi", "Finland"),
            ("bn", "ফিনল\u{9cd}য\u{9be}ন\u{9cd}ড"),
            ("bn_IN", "ফিনল\u{9cd}য\u{9be}ন\u{9cd}ড"),
            ("br", "Finland"),
            ("bs", "Finska"),
            ("ca", "Finlàndia"),
            ("ce", "Финлянди"),
            ("ch", "Finlandia"),
            ("cs", "Finsko"),
            ("cv", "Финлянди"),
            ("cy", "Y Ffindir"),
            ("da", "Finland"),
            ("de", "Finnland"),
            ("dv", "ފ\u{7a8}ނ\u{7b0}ލ\u{7ad}ނ\u{7b0}ޑ\u{7aa}"),
            ("dz", "ཕ\u{f72}ན་ལ\u{f7a}ནཌ\u{f72}།"),
            ("ee", "Finland"),
            ("el", "Φινλανδία"),
            ("en", "Finland"),
            ("eo", "Finnlando"),
            ("es", "Finlandia"),
            ("et", "Soome"),
            ("eu", "Finlandia"),
            ("fa", "فنلاند"),
            ("ff", "Finland"),
            ("fi", "Suomi"),
            ("fo", "Finnland"),
            ("fr", "Finlande"),
            ("fy", "Finlân"),
            ("ga", "An Fhionlainn"),
            ("gl", "Finlandia"),
            ("gn", "Finland"),
            ("gu", "ફિનલ\u{ac7}ન\u{acd}ડ"),
            ("gv", "Finnlynn"),
            ("ha", "Finland"),
            ("he", "פינלנד"),
            ("hi", "फ\u{93c}िनल\u{948}ण\u{94d}ड"),
            ("hr", "Finska"),
            ("ht", "Fenlann"),
            ("hu", "Finnország"),
            ("hy", "Ֆինլանդիա"),
            ("ia", "Finlandia"),
            ("id", "Finlandia"),
            ("io", "Finlando"),
            ("is", "Finnland"),
            ("it", "Finlandia"),
            ("iu", "ᐃᓐᓚᓐᑦ"),
            ("ja", "フィンランド"),
            ("ka", "ფინეთი"),
            ("ki", "Finland"),
            ("kk", "Финляндия"),
            ("kl", "Finland"),
            ("km", "ហ\u{17d2}វា\u{17c6}ងឡង\u{17cb}"),
            ("kn", "ಫ\u{cbf}ನ\u{ccd}ಲಂಡ\u{ccd}"),
            ("ko", "핀란드"),
            ("ku", "Fînlandiya"),
            ("kv", "Суоми Му"),
            ("kw", "Pow Finn"),
            ("ky", "Финляндия"),
            ("lo", "ປະເທດແຟງລ\u{eb1}ງ"),
            ("lt", "Suomija"),
            ("lv", "Somija"),
            ("mi", "Hinerangi"),
            ("mk", "Финска"),
            ("ml", "ഫിന\u{d4d}\u{200d}ലന\u{d4d}\u{200d}ഡ\u{d4d}"),
            ("mn", "Финнлянд"),
            ("mr", "फिनल\u{945}\u{902}ड"),
            ("ms", "Finland"),
            ("mt", "Finlandja"),
            (
                "my",
                "ဖင\u{103a}လန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Pinrand"),
            ("nb", "Finland"),
            ("ne", "फिन\u{94d}ल\u{94d}यान\u{94d}ड"),
            ("nl", "Finland"),
            ("nn", "Finland"),
            ("nv", "Nahoditsʼǫʼłání Dineʼé Bikéyah"),
            ("oc", "Finlàndia"),
            ("or", "ଫ\u{b3f}ନଲ\u{b4d}ଯ\u{b3e}ଣ\u{b4d}ଡ"),
            ("pa", "ਫ਼ਿਨਲ\u{a48}\u{a02}ਡ"),
            ("pi", "फिन\u{94d}ल\u{948}\u{902}ड"),
            ("pl", "Finlandia"),
            ("ps", "فنلینډ"),
            ("pt", "Finlândia"),
            ("pt_BR", "Finlândia"),
            ("ro", "Finlanda"),
            ("ru", "Финляндия"),
            ("rw", "Finilande"),
            ("sc", "Finlàndia"),
            ("sd", "فن لينڊ"),
            ("si", "ෆ\u{dd2}න\u{dca}ලන\u{dca}තය"),
            ("sk", "Fínsko"),
            ("sl", "Finska"),
            ("so", "Fiinlaand"),
            ("sq", "Finlandë"),
            ("sr", "Финска"),
            ("sv", "Finland"),
            ("sw", "Finland"),
            ("ta", "பின\u{bcd}ல\u{bbe}ந\u{bcd}து"),
            ("te", "ఫ\u{c3f}న\u{c4d}\u{200c}ల\u{c3e}ండ\u{c4d}"),
            ("tg", "Финляндия"),
            ("th", "ฟ\u{e34}นแลนด\u{e4c}"),
            ("ti", "ፊንላንድ"),
            ("tk", "Finlýandiýa"),
            ("tl", "Finland"),
            ("tr", "Finlandiya"),
            ("tt", "Финланд (Суоми)"),
            ("ug", "فىنلاندىيە"),
            ("uk", "Фінляндія"),
            ("ur", "فن لینڈ"),
            ("uz", "Finlandiya"),
            ("ve", "Finland"),
            ("vi", "Phần Lan"),
            ("wa", "Finlande"),
            ("wo", "Finlaand"),
            ("xh", "Finland"),
            ("yo", "Fínlándì"),
            ("zh_CN", "芬兰"),
            ("zh_HK", "芬蘭"),
            ("zh_TW", "芬蘭"),
            ("zu", "IFinlandi"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}