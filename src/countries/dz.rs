// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The People's Democratic Republic of Algeria

#[cfg(all(feature = "dz", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::DZ;
    pub const ALPHA3: Alpha3 = Alpha3::DZA;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 213;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::DZD;
    pub const GEC: Option<GEC> = Some(GEC::AG);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::ALG);
    pub const ISO_SHORT_NAME: &str = "Algeria";
    pub const ISO_LONG_NAME: &str = "The People's Democratic Republic of Algeria";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "7";
    pub const NATIONALITY: Option<&str> = Some("Algerian");
    pub const NUMBER: &str = "012";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Sunday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::NorthernAfrica);
    pub const UN_LOCODE: &str = "DZ";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Algeria",
        "الجزائر",
        "Algerien",
        "Algérie",
        "Argelia",
        "アルジェリア",
        "Algerije",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Algeria"),
        ("af", "Algerië"),
        ("ak", "Algeria"),
        ("am", "ጐሔጄሱ።"),
        ("an", "Algeria"),
        ("ar", "الجزائر"),
        ("as", "আলজেৰিয\u{9be}"),
        ("ay", "Algeria"),
        ("az", "Əlcəzair"),
        ("ba", "Algeria"),
        ("be", "Алжыр"),
        ("bg", "Алжир"),
        ("bi", "Algeria"),
        ("bn", "অ\u{9cd}য\u{9be}লজিরিয়\u{9be}"),
        ("bn_IN", "অ\u{9cd}য\u{9be}লজিরিয়\u{9be}"),
        ("br", "Aljeria"),
        ("bs", "Alžir"),
        ("ca", "Algèria"),
        ("ce", "Алжир"),
        ("ch", "Algeria"),
        ("cs", "Alžírsko"),
        ("cv", "Алжир"),
        ("cy", "Algeria"),
        ("da", "Algeriet"),
        ("de", "Algerien"),
        ("dv", "ޖ\u{7a6}ޒ\u{7a7}އ\u{7a8}ރ\u{7aa}"),
        ("dz", "ཨ\u{f71}ལ་ཇ\u{f72}་ར\u{f72}་ཡ།"),
        ("ee", "Algeria"),
        ("el", "Αλγερία"),
        ("en", "Algeria"),
        ("eo", "Alĝerio"),
        ("es", "Algeria"),
        ("et", "Alžeeria"),
        ("eu", "Aljeria"),
        ("fa", "الجزایر"),
        ("ff", "Aljeri"),
        ("fi", "Algeria"),
        ("fo", "Algeria"),
        ("fr", "Algérie"),
        ("fy", "Algerije"),
        ("ga", "An Ailgéir"),
        ("gl", "Alxeria"),
        ("gn", "Algeria"),
        ("gu", "અલ\u{acd}જ\u{ac7}રિયા"),
        ("gv", "Yn Algear"),
        ("ha", "Aljeriya"),
        ("he", "אלג'יריה"),
        ("hi", "अल\u{94d}जीरिया"),
        ("hr", "Alžir"),
        ("ht", "Aljeri"),
        ("hu", "Algéria"),
        ("hy", "Ալժիր"),
        ("ia", "Algeria"),
        ("id", "Aljazair"),
        ("io", "Aljeria"),
        ("is", "Alsír"),
        ("it", "Algeria"),
        ("iu", "Algeria"),
        ("ja", "アルジェリア"),
        ("ka", "ალჟირი"),
        ("ki", "Algeria"),
        ("kk", "Алжир"),
        ("kl", "Algeria"),
        ("km", "អាល\u{17cb}ហ\u{17d2}សេរ\u{17b8}"),
        ("kn", "ಅಲ\u{ccd}ಜೀರ\u{cbf}ಯ"),
        ("ko", "알제리"),
        ("ku", "Cezayîr"),
        ("kv", "Algeria"),
        ("kw", "Aljeri"),
        ("ky", "Алжир"),
        ("lo", "Algeria"),
        ("lt", "Alžyras"),
        ("lv", "Alžīrija"),
        ("mi", "Algeria"),
        ("mk", "Алжир"),
        ("ml", "അള\u{d4d}\u{200d}ജീരിയ"),
        ("mn", "Алжир"),
        ("mr", "अल\u{94d}जिरिया"),
        ("ms", "Algeria"),
        ("mt", "Alġerija"),
        (
            "my",
            "အယ\u{103a}လ\u{103a}ဂျ\u{102e}းရ\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Ardjiriya"),
        ("nb", "Algerie"),
        ("ne", "अल\u{94d}ज\u{947}रिया"),
        ("nl", "Algerije"),
        ("nn", "Algerie"),
        (
            "nv",
            "Ghą\u{301}ą\u{301}ʼaskʼidii Biłikahii Bikéyah Ntsaaígíí",
        ),
        ("oc", "Argeria"),
        ("or", "ଆଲଜେର\u{b3f}ୟ\u{b3e}"),
        ("pa", "ਅਲਜੀਰੀਆ"),
        ("pi", "अल\u{94d}जीरिया"),
        ("pl", "Algieria"),
        ("ps", "الجزایر"),
        ("pt", "Argélia"),
        ("pt_BR", "Argélia"),
        ("ro", "Algeria"),
        ("ru", "Алжир"),
        ("rw", "Aligeriya"),
        ("sc", "Algeria"),
        ("sd", "الجزائر"),
        ("si", "ඇල\u{dca}ජ\u{dd3}ර\u{dd2}ය\u{dcf}"),
        ("sk", "Alžírsko"),
        ("sl", "Alžirija"),
        ("so", "Aljeeriya"),
        ("sq", "Algjeri"),
        ("sr", "Алжир"),
        ("sv", "Algeriet"),
        ("sw", "Aljeria"),
        ("ta", "அல\u{bcd}ஜ\u{bc0}ரிய\u{bbe}"),
        ("te", "అల\u{c4d}జ\u{c3f}ర\u{c3f}య\u{c3e}"),
        ("tg", "Алҷазоир"),
        ("th", "แอลจ\u{e35}เร\u{e35}ย"),
        ("ti", "አልጄሪያ"),
        ("tk", "Algerýa"),
        ("tl", "Alheriya"),
        ("tr", "Cezayir"),
        ("tt", "Алжыр"),
        ("ug", "ئالجىرىيە"),
        ("uk", "Алжир"),
        ("ur", "الجزائر"),
        ("uz", "Jazoir"),
        ("ve", "Algeria"),
        ("vi", "An-giê-ri"),
        ("wa", "Aldjereye"),
        ("wo", "Aljeeri"),
        ("xh", "Algeria"),
        ("yo", "Àlgéríà"),
        ("zh_CN", "阿尔及利亚"),
        ("zh_HK", "阿爾及利亞"),
        ("zh_TW", "阿爾及利亞"),
        ("zu", "IAljiriya"),
    ];
    #[cfg(all(feature = "dz", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 28.033886;
        pub const LONGITUDE: f64 = 1.659626;
        pub const MAX_LATITUDE: f64 = 37.2216;
        pub const MAX_LONGITUDE: f64 = 11.9999992;
        pub const MIN_LATITUDE: f64 = 18.9681469;
        pub const MIN_LONGITUDE: f64 = -8.6676111;
        pub const NORTHEAST_LATITUDE: f64 = 37.2216;
        pub const NORTHEAST_LONGITUDE: f64 = 11.9999992;
        pub const SOUTHWEST_LATITUDE: f64 = 18.9681469;
        pub const SOUTHWEST_LONGITUDE: f64 = -8.6676111;
    }
}
#[cfg(all(feature = "dz", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 28.033886,
            longitude: 1.659626,
            max_latitude: 37.2216,
            max_longitude: 11.9999992,
            min_latitude: 18.9681469,
            min_longitude: -8.6676111,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 37.2216,
                    longitude: 11.9999992,
                },
                southwest: CountryGeoBound {
                    latitude: 18.9681469,
                    longitude: -8.6676111,
                },
            },
        }
    }
}

#[cfg(all(feature = "dz", feature = "subdivisions"))]
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
                        country_alpha2: Alpha2::DZ,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.866667), longitude: Some(-0.283333), max_latitude: Some(28.1852177), min_latitude: Some(27.7680133), max_longitude: Some(-0.0769043), min_longitude: Some(-0.3310157)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية أدرار"), ("az", "Adrar vilayəti"), ("be", "Адрар"), ("bg", "Адрар"), ("bn", "আদ\u{9cd}র\u{9be}র প\u{9cd}রদেশ"), ("bs", "Adrar"), ("ca", "Província d’Adrar"), ("ccp", "𑄃𑄓\u{11133}𑄢𑄢\u{11134}"), ("ceb", "Adrar (lalawigan sa Arhelya, lat 25,75, long -1,00)"), ("da", "Adrar"), ("de", "Wilaya Adrar"), ("el", "Αντράρ"), ("en", "Adrar"), ("es", "Provincia de Adrar"), ("eu", "Adrar probintzia"), ("fa", "استان ادرار"), ("fi", "Adrarin maakunta"), ("fr", "wilaya d’Adrar"), ("gl", "Provincia de Adrar"), ("gu", "અદ\u{acd}રાર પ\u{acd}રા\u{a82}ત"), ("he", "מחוז אדראר"), ("hi", "अद\u{94d}रर प\u{94d}रा\u{902}त"), ("id", "Provinsi Adrar"), ("it", "provincia di Adrar"), ("ja", "アドラール県"), ("ka", "ადრარის პროვინცია"), ("kn", "ಅಡ\u{ccd}ರಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아드라르 주"), ("lt", "Adraro vilaja"), ("lv", "Adrāras vilāja"), ("mr", "अदरर प\u{94d}रा\u{902}त"), ("ms", "Wilayah Adrar"), ("nb", "Adrar"), ("nl", "Adrar"), ("no", "Adrar"), ("pl", "Prowincja Adrar"), ("pt", "Adrar"), ("ro", "Provincia Adrar"), ("ru", "Адрар"), ("si", "අද\u{dca}ර\u{dcf}ර\u{dca} පළ\u{dcf}ත"), ("sq", "Vilajeti Adrar"), ("sr", "Адрар"), ("sr_Latn", "Adrar"), ("sv", "Adrar"), ("sw", "Jimbo la Adrar"), ("ta", "அட\u{bcd}ரர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అడ\u{c4d}ర\u{c3e}ర\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e31}ดราร\u{e4c}"), ("tr", "Adrar Vilayeti"), ("uk", "Адрар"), ("ur", "صوبہ ادرار"), ("vi", "Adrar"), ("zh", "阿德拉爾省")]),
                        unofficial_name_list: ["Adrar"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::DZ,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.166667), longitude: Some(1.333333), max_latitude: Some(36.2490494), min_latitude: Some(36.0892685), max_longitude: Some(1.4494228), min_longitude: Some(1.2608528)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية الشلف"), ("be", "Правінцыя Эш-Шэліф"), ("bg", "Шлеф"), ("bn", "ক\u{9cd}লেফ প\u{9cd}রদেশ"), ("ca", "Província de Chlef"), ("ccp", "𑄇\u{11133}𑄣𑄬𑄛\u{11134}"), ("ceb", "Wilaya de Chlef"), ("da", "Chlef Province"), ("de", "Chlef"), ("el", "Σλεφ"), ("en", "Chlef"), ("es", "Provincia de Chlef"), ("eu", "Chlef probintzia"), ("fa", "استان الشلف"), ("fi", "Chlefin maakunta"), ("fr", "wilaya de Chlef"), ("gl", "Provincia de Chlef"), ("gu", "ક\u{acd}લ\u{ac7}ફ પ\u{acd}રા\u{a82}ત"), ("he", "מחוז א-שלף"), ("hi", "छल\u{947}फ प\u{94d}रा\u{902}त"), ("id", "Provinsi Syilf"), ("it", "provincia di Chlef"), ("ja", "シュレフ県"), ("ka", "ჩლეფის პროვინცია"), ("kn", "ಕ\u{ccd}ಲ\u{cc6}ಫ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "슐레프 주"), ("lt", "Šelifo vilaja"), ("lv", "Šelīfas vilāja"), ("mr", "क\u{94d}ल\u{947}फ प\u{94d}रा\u{902}त"), ("ms", "Wilayah Chlef"), ("nb", "Chlef"), ("nl", "Chlef"), ("no", "Chlef"), ("pl", "Prowincja Asz-Szalif"), ("pt", "Chlef"), ("ro", "Provincia Chlef"), ("ru", "Эш-Шелифф"), ("si", "චෙල\u{dca}ෆ\u{dca} පළ\u{dcf}ත"), ("sr", "Шлеф"), ("sr_Latn", "Šlef"), ("sv", "Chlef"), ("sw", "Jimbo la Chlef"), ("ta", "செலேபி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "చ\u{c4d}ల\u{c46}ఫ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดชเรฟ"), ("tr", "Şelf Vilayeti"), ("uk", "Шлеф"), ("ur", "صوبہ الشلف"), ("vi", "Chlef"), ("zh", "谢里夫省")]),
                        unofficial_name_list: ["Al Asnam", "Al Asnām", "Chelef", "Chelf", "Chlef", "Chlif", "Ech Cheliff", "El Asnam"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::DZ,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.7972551), longitude: Some(2.8694065), max_latitude: Some(33.8350786), min_latitude: Some(33.7396685), max_longitude: Some(2.947855), min_longitude: Some(2.8309535)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية الأغواط"), ("be", "Правінцыя Агуат"), ("bg", "Лагуат"), ("bn", "ল\u{9be}ঘ\u{9c1}য\u{9bc}\u{9be}ট প\u{9cd}রদেশ"), ("ccp", "𑄣𑄊\u{1112f}𑄖\u{11134}"), ("ceb", "Wilaya de Laghouat"), ("da", "Laghouat Province"), ("de", "Laghouat"), ("el", "Λαγκουάτ"), ("en", "Laghouat"), ("es", "Provincia de Laghouat"), ("eu", "Laghouat probintzia"), ("fa", "استان الاغواط"), ("fi", "Laghouat"), ("fr", "wilaya de Laghouat"), ("gl", "Provincia de Laghouat"), ("gu", "લાગોઆટ પ\u{acd}રા\u{a82}ત"), ("hi", "लाघौत प\u{94d}रा\u{902}त"), ("id", "Provinsi Laghouat"), ("it", "provincia di Laghouat"), ("ja", "ラグアット県"), ("ka", "ლაგუატის პროვინცია"), ("kn", "ಲಘುತ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "라구아트 주"), ("lt", "Laguato vilaja"), ("lv", "Agvātas vilāja"), ("mr", "लगोआट प\u{94d}रा\u{902}त"), ("ms", "Wilayah Laghouat"), ("nb", "Laghouat"), ("nl", "Laghouat"), ("no", "Laghouat"), ("pl", "Prowincja Al-Aghwat"), ("pt", "Laghouat"), ("ro", "Provincia Laghouat"), ("ru", "Лагуат"), ("si", "ලගොඋඅට\u{dca} පළ\u{dcf}ත"), ("sr", "Лагуат"), ("sr_Latn", "Laguat"), ("sv", "Laghouat"), ("sw", "Jimbo la Laghouat"), ("ta", "ல\u{bbe}ஃஹௌஅட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c3e}గ\u{c4d}హ\u{c4b}వ\u{c47} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเลกเฮาท\u{e4c}"), ("tr", "Lagvat Vilayeti"), ("uk", "Лагуат"), ("ur", "صوبہ الاغواط"), ("vi", "Laghouat"), ("zh", "艾格瓦特省")]),
                        unofficial_name_list: ["Laghouat"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::DZ,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.7881449), longitude: Some(7.179025999999999), max_latitude: Some(36.167782), min_latitude: Some(35.4122201), max_longitude: Some(7.932361999999999), min_longitude: Some(6.1793371)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية أم البواقي"), ("az", "Umm-el-Buaqi vilayəti"), ("be", "Правінцыя Ум-эль-Буахі"), ("bg", "Ум ел-Буаги"), ("bn", "ওয\u{9bc}\u{9be}ম এল ব\u{9c1}ঘ\u{9be}ই প\u{9cd}রদেশ"), ("ccp", "𑄃\u{11127}𑄅𑄟\u{11134} 𑄃𑄬𑄣\u{11134} 𑄝\u{1112f}𑄊\u{11129}"), ("ceb", "Oum el Bouaghi (lalawigan)"), ("da", "Oum El Bouaghi Province"), ("de", "Oum el Bouaghi"), ("el", "Ουμ Ελ Μπουάγι"), ("en", "Oum El Bouaghi"), ("es", "Provincia de Oum el-Bouaghi"), ("eu", "Oum-El-Bouaghi probintzia"), ("fa", "استان ام\u{200c}البواقی"), ("fi", "Oum El Bouaghin lääni"), ("fr", "wilaya d’Oum El Bouaghi"), ("gl", "Provincia de Oum El Bouaghi"), ("gu", "ઓમ અલ બૌઆઘી પ\u{acd}રા\u{a82}ત"), ("hi", "ओउम एल बोअघी प\u{94d}रा\u{902}त"), ("id", "Provinsi Oum el Bouaghi"), ("it", "provincia di Oum el-Bouaghi"), ("ja", "ウメル・ブアーギ県"), ("ka", "უმ-ელ-ბუახის პროვინცია"), ("kn", "ಓಮ\u{ccd} ಎಲ\u{ccd} ಬೊವಾಗ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "움엘부아기 주"), ("lt", "Um el Buagio vilaja"), ("lv", "Umm el Bavāgī vilāja"), ("mr", "ओम एल बौआजी प\u{94d}रा\u{902}त"), ("ms", "Wilayah Oum El Bouaghi"), ("nb", "Oum el-Bouaghi"), ("nl", "Oum el-Bouaghi"), ("no", "Oum el-Bouaghi"), ("pl", "Prowincja Umm al-Bawaki"), ("pt", "Oum el Bouaghi"), ("ro", "Provincia Oum el-Bouaghi"), ("ru", "Ум-эль-Буахи"), ("si", "ඕඌම\u{dca} එල\u{dca} බෝග\u{dd3} පළ\u{dcf}ත"), ("sr", "Ум ел Буаги"), ("sr_Latn", "Um el Buagi"), ("sv", "Oum El Bouaghi"), ("sw", "Jimbo ya Oum el-Bouaghi"), ("ta", "ஓம\u{bcd} எல\u{bcd} பௌஅஃஹி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఓవమ\u{c4d} ఎల\u{c4d} బ\u{c4a}వ\u{c3e}ఘ\u{c40} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e39}ม เอล บวกฮ\u{e35}"), ("tr", "Em El Buvaki Vilayeti"), ("uk", "Ум-ель-Буакі"), ("ur", "صوبہ ام البواقی"), ("vi", "Oum El Bouaghi"), ("zh", "乌姆布瓦吉省")]),
                        unofficial_name_list: ["Canrobert", "Oum el Bouaghi"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::DZ,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.55), longitude: Some(6.166666999999999), max_latitude: Some(35.6445679), min_latitude: Some(35.5123258), max_longitude: Some(6.2781502), min_longitude: Some(6.0786152)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية باتنة"), ("az", "Batna vilayəti"), ("be", "Правінцыя Батна"), ("bg", "Батна"), ("bn", "ব\u{9be}টন\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Batna"), ("ccp", "𑄝\u{11127}𑄚\u{11134}𑄑"), ("ceb", "Wilaya de Batna"), ("da", "Batna"), ("de", "Batna"), ("el", "Μπάτνα"), ("en", "Batna"), ("es", "Provincia de Batna"), ("eu", "Batna probintzia"), ("fa", "استان باتنه"), ("fi", "Batnan maakunta"), ("fr", "wilaya de Batna"), ("gl", "Provincia de Batna"), ("gu", "બટના પ\u{acd}રા\u{a82}ત"), ("hi", "बटना प\u{94d}रा\u{902}त"), ("id", "Provinsi Batnah"), ("it", "provincia di Batna"), ("ja", "バトナ県"), ("ka", "ბატნის პროვინცია"), ("kn", "ಬಟ\u{ccd}ನಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "바트나 주"), ("lt", "Batnos vilaja"), ("lv", "Bātinas vilāja"), ("mr", "बाटना प\u{94d}रा\u{902}त"), ("ms", "Wilayah Batna"), ("nb", "Batna"), ("nl", "Batna"), ("no", "Batna"), ("pl", "Prowincja Batina"), ("pt", "Batna"), ("ro", "Provincia Batna"), ("ru", "Батна"), ("si", "බටන\u{dcf} කල\u{dcf}පය"), ("sq", "Batna"), ("sr", "Батна"), ("sr_Latn", "Batna"), ("sv", "Batna"), ("sw", "Jimbo la Batna"), ("ta", "பட\u{bcd}ந\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c3e}ట\u{c4d}న\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแบทนา"), ("tr", "Batna Vilayeti"), ("uk", "Батна"), ("ur", "صوبہ باتنہ"), ("vi", "Batna"), ("zh", "巴特纳省")]),
                        unofficial_name_list: ["Batna"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::DZ,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.7508896), longitude: Some(5.056733299999999), max_latitude: Some(36.833659), min_latitude: Some(36.7055347), max_longitude: Some(5.1074721), min_longitude: Some(4.9087)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية بجاية"), ("az", "Becaya vilayəti"), ("be", "Правінцыя Беджая"), ("bg", "Беджая"), ("bn", "বেজ\u{9be}ইয\u{9bc}\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Bejaïa"), ("ccp", "𑄝𑄬𑄎\u{1112d}𑄠"), ("ceb", "Wilaya de Bejaïa"), ("da", "Béjaïa (provins)"), ("de", "Bejaia"), ("el", "Μπετζάγια"), ("en", "Béjaïa"), ("es", "Provincia de Bujía"), ("eu", "Bejaia probintzia"), ("fa", "استان بجایه"), ("fi", "Béjaïan provinssi"), ("fr", "wilaya de Béjaïa"), ("gl", "Provincia de Béjaïa"), ("gu", "બ\u{ac7}જાઇઆ પ\u{acd}રા\u{a82}ત"), ("hi", "ब\u{947}ज\u{948}अ प\u{94d}रा\u{902}त"), ("id", "Provinsi Bajayah"), ("it", "provincia di Béjaïa"), ("ja", "ベジャイア県"), ("ka", "ბეჯაიის პროვინცია"), ("kn", "ಬೇಜೈಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "베자이아 주"), ("lt", "Bedžajos vilaja"), ("lv", "Bidžājas vilāja"), ("mr", "ब\u{947}जाया प\u{94d}रा\u{902}त"), ("ms", "Wilayah Béjaïa"), ("nb", "Béjaïa"), ("nl", "Béjaïa"), ("no", "Béjaïa"), ("pl", "Prowincja Bidżaja"), ("pt", "Bugia"), ("ro", "Provincia Béjaïa"), ("ru", "Беджая"), ("si", "බෙජය\u{dd2}ය\u{dca}ය\u{dcf} පළ\u{dcf}ත"), ("sr", "Беџаја"), ("sr_Latn", "Bedžaja"), ("sv", "Béjaïa"), ("sw", "Jimbo ya Béjaïa"), ("ta", "பெஜ\u{bbe}இஆ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c47}జ\u{c3e}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเบจาเอ\u{e35}ย"), ("tr", "Becaye Vilayeti"), ("uk", "Беджая"), ("ur", "صوبہ بجایہ"), ("vi", "Béjaïa"), ("zh", "贝贾亚省")]),
                        unofficial_name_list: ["Bejaïa", "Bougie", "Béjaïa"].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::DZ,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.85), longitude: Some(5.733333), max_latitude: Some(34.9172558), min_latitude: Some(34.7857523), max_longitude: Some(5.794816), min_longitude: Some(5.6545258)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية بسكرة"), ("az", "Biskra vilayəti"), ("be", "Правінцыя Біскра"), ("bg", "Бискра (област)"), ("bn", "বিস\u{9cd}ক\u{9cd}র\u{9be}"), ("ca", "Província de Biskra"), ("ccp", "𑄝\u{11128}𑄌\u{11134}𑄇\u{11133}𑄢"), ("ceb", "Wilaya de Biskra"), ("da", "Biskra"), ("de", "Biskra"), ("el", "Μπίσκρα"), ("en", "Biskra"), ("es", "Provincia de Biskra"), ("eu", "Biskra probintzia"), ("fa", "استان بسکره"), ("fi", "Biskra"), ("fr", "wilaya de Biskra"), ("gl", "Provincia de Biskra"), ("gu", "બિસ\u{acd}ક\u{acd}ર\u{acd}રા"), ("hi", "बिस\u{94d}क\u{94d}रा प\u{94d}रा\u{902}त"), ("id", "Provinsi Biskirah"), ("it", "provincia di Biskra"), ("ja", "ビスクラ県"), ("ka", "ბისკრის პროვინცია"), ("kn", "ಬ\u{cbf}ಸ\u{ccd}ಕ\u{ccd}ರಾ"), ("ko", "비스크라 주"), ("lt", "Biskros vilaja"), ("lv", "Biskra"), ("mr", "बिसकरा"), ("ms", "Wilayah Biskra"), ("nb", "Biskra"), ("nl", "Biskra"), ("no", "Biskra"), ("pl", "Prowincja Biskira"), ("pt", "Biskra"), ("ro", "Provincia Biskra"), ("ru", "Бискра"), ("si", "බ\u{dd2}ස\u{dca}ක\u{dca}ර\u{dcf}"), ("sr", "Бискра"), ("sr_Latn", "Biskra"), ("sv", "Biskra"), ("sw", "Jimbo ya Biskra"), ("ta", "பிஸ\u{bcd}க\u{bcd}ர"), ("te", "బ\u{c3f}స\u{c4d}క\u{c4d}ర\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e34}สกรา"), ("tr", "Biskra Vilayeti"), ("uk", "Біскра"), ("ur", "صوبہ بسکرہ"), ("vi", "Biskra"), ("zh", "比斯克拉省")]),
                        unofficial_name_list: ["Beskra", "Biskara", "Biskra"].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "08",
                        country_alpha2: Alpha2::DZ,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.6182492), longitude: Some(-2.2143231), max_latitude: Some(32.166313), min_latitude: Some(31.1824797), max_longitude: Some(-1.352005), min_longitude: Some(-2.9278564)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية بشار"), ("az", "Beşar vilayəti"), ("be", "Правінцыя Бешар"), ("bg", "Бешар"), ("bn", "বেক\u{9be}র প\u{9cd}রদেশ"), ("ca", "Província de Bechar"), ("ccp", "𑄝𑄬𑄌𑄢\u{11134}"), ("ceb", "Wilaya de Béchar"), ("da", "Béchar"), ("de", "Wilaya Bechar"), ("el", "Μπεσάρ"), ("en", "Béchar"), ("es", "Provincia de Béchar"), ("eu", "Bexar probintzia"), ("fa", "استان بشار"), ("fi", "Bécharin maakunta"), ("fr", "wilaya de Béchar"), ("gl", "Provincia de Béchar"), ("gu", "બ\u{ac7}ચર પ\u{acd}રા\u{a82}ત"), ("hi", "ब\u{947}चार प\u{94d}रा\u{902}त"), ("id", "Provinsi Bechar"), ("it", "provincia di Béchar"), ("ja", "ベシャール県"), ("ka", "ბეშარის პროვინცია"), ("kn", "ಬ\u{cc6}ಚಾರ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "베샤르 주"), ("lt", "Bešaro vilaja"), ("lv", "Bešāras vilāja"), ("mr", "ब\u{947}चर प\u{94d}रा\u{902}त"), ("ms", "Wilayah Béchar"), ("nb", "Béchar"), ("nl", "Béchar"), ("no", "Béchar"), ("pl", "Prowincja Baszszar"), ("pt", "Béchar"), ("ro", "Provincia Béchar"), ("ru", "Бешар"), ("si", "බේච\u{dcf}ර\u{dca} පළ\u{dcf}ත"), ("sq", "Bechar"), ("sr", "Бешар"), ("sr_Latn", "Bešar"), ("sv", "Béchar"), ("sw", "Jimbo ya Béchar"), ("ta", "பேச\u{bcd}சர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c46}చ\u{c3e}ర\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเบชา"), ("tr", "Beşar Vilayeti"), ("uk", "Бешар"), ("ur", "صوبہ بشار"), ("vi", "Béchar"), ("zh", "贝沙尔省")]),
                        unofficial_name_list: ["Béchar"].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "09",
                        country_alpha2: Alpha2::DZ,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.4798683), longitude: Some(2.8005677), max_latitude: Some(36.5172071), min_latitude: Some(36.4433975), max_longitude: Some(2.8684831), min_longitude: Some(2.7320336)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية البليدة"), ("az", "Blida vilayəti"), ("be", "Правінцыя Бліда"), ("bg", "Блида"), ("bn", "বিলিড\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Blida"), ("ccp", "𑄝\u{11133}𑄣\u{11128}𑄓"), ("ceb", "Wilaya de Blida"), ("da", "Blida Province"), ("de", "Blida"), ("el", "Μπλίντα"), ("en", "Blida"), ("es", "Provincia de Blida"), ("eu", "Blida probintzia"), ("fa", "استان البلیده"), ("fi", "Blidan provinssi"), ("fr", "wilaya de Blida"), ("gl", "Provincia de Blida"), ("gu", "બ\u{acd}લિડા પ\u{acd}રા\u{a82}ત"), ("hi", "ब\u{94d}लिडा प\u{94d}रा\u{902}त"), ("id", "Provinsi Bulidah"), ("it", "provincia di Blida"), ("ja", "ブリダ県"), ("ka", "ბლიდის პროვინცია"), ("kn", "ಬ\u{ccd}ಲ\u{cbf}ಡಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "블리다 주"), ("lt", "Blidos vilaja"), ("lv", "Blīdas vilāja"), ("mr", "ब\u{94d}लिडा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Blida"), ("nb", "Blida"), ("nl", "Blida"), ("no", "Blida"), ("pl", "Prowincja Al-Bulajda"), ("pt", "Blida"), ("ro", "Provincia Blida"), ("ru", "Блида"), ("si", "බ\u{dca}ල\u{dd2}ඩ\u{dcf} පළ\u{dcf}ත"), ("sr", "Блида"), ("sr_Latn", "Blida"), ("sv", "Blida"), ("sw", "Jimbo ya Blida"), ("ta", "பிளிட\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c4d}ల\u{c3f}డ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบล\u{e34}ดา"), ("tr", "Blida Vilayeti"), ("uk", "Бліда"), ("ur", "صوبہ البلیدہ"), ("vi", "Blida"), ("zh", "卜利达省")]),
                        unofficial_name_list: ["Blida", "El Boulaida"].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::DZ,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.2835299), longitude: Some(3.9878427), max_latitude: Some(36.5897481), min_latitude: Some(35.8270988), max_longitude: Some(4.47439), min_longitude: Some(3.375708)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية البويرة"), ("be", "Правінцыя Буіра"), ("bg", "Буира"), ("bn", "ব\u{9c1}ইর\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Bouira"), ("ccp", "𑄝\u{1112f}𑄃\u{11128}𑄢"), ("ceb", "Wilaya de Bouira"), ("da", "Bouïra Province"), ("de", "Bouira"), ("el", "Μπουίρα"), ("en", "Bouira"), ("es", "Provincia de Bouira"), ("eu", "Bouïra probintzia"), ("fa", "استان بویره"), ("fi", "Bouïran provinssi"), ("fr", "wilaya de Bouira"), ("gl", "Provincia de Bouira"), ("gu", "બોઈરા પ\u{acd}રા\u{a82}ત"), ("hi", "बोउइरा प\u{94d}रा\u{902}त"), ("id", "Provinsi Bouira"), ("it", "provincia di Bouira"), ("ja", "ブイラ県"), ("ka", "ბუირის პროვინცია"), ("kn", "ಬೋಯ\u{cbf}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "부이라 주"), ("lt", "Buiros vilaja"), ("lv", "Būīras vilāja"), ("mr", "बहीरा प\u{94d}रा\u{902}त"), ("ms", "Bouira Province"), ("nb", "Bouira"), ("nl", "Bouira"), ("no", "Bouira"), ("pl", "Prowincja Al-Buwajra"), ("pt", "Bouira"), ("ro", "Provincia Bouira"), ("ru", "Буира"), ("si", "බෞඉර\u{dcf} පළ\u{dcf}ත"), ("sr", "Бујра"), ("sr_Latn", "Bujra"), ("sv", "Bouira"), ("sw", "Jimbo ya Bouira"), ("ta", "பெய\u{bcd}ர\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c3e}య\u{c3f}ర\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e38}ยรา"), ("tr", "Buira Vilayeti"), ("uk", "Буїра"), ("ur", "صوبہ البویرہ"), ("vi", "Bouira"), ("zh", "布维拉省")]),
                        unofficial_name_list: ["Bouira"].to_vec(),
                    }
                ),
                (
                    "11",
                    Subdivision{
                        name: "11",
                        country_alpha2: Alpha2::DZ,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.785), longitude: Some(5.522778), max_latitude: Some(23.6506568), min_latitude: Some(20.8985885), max_longitude: Some(6.662521399999999), min_longitude: Some(4.6884155)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية تمنراست"), ("be", "Правінцыя Таманрасет"), ("bg", "Таманрасет"), ("bn", "ত\u{9be}ম\u{9be}ঙ\u{9cd}ঘ\u{9be}সেট প\u{9cd}রদেশ"), ("ca", "Província de Tamanghasset"), ("ccp", "𑄑𑄟𑄋\u{11134}𑄊𑄥𑄬𑄖\u{11134}"), ("ceb", "Wilaya de Tamanrasset"), ("da", "Tamanghasset Province"), ("de", "Tamanrasset"), ("el", "Ταμαγκασσέτ"), ("en", "Tamanghasset"), ("es", "Provincia de Tamanghasset"), ("et", "Tamanrasseti provints"), ("eu", "Tamanrasset probintzia"), ("fa", "استان تمنراست"), ("fi", "Tamanrassetin maakunta"), ("fr", "wilaya de Tamanrasset"), ("gl", "Provincia de Tamanrasset"), ("gu", "તમ\u{a82}ઘાસ\u{ac7}ટ પ\u{acd}રા\u{a82}ત"), ("hi", "तमान\u{94d}रस\u{94d}स\u{947}ट प\u{94d}रा\u{902}त"), ("id", "Provinsi Tamanghasset"), ("it", "provincia di Tamanrasset"), ("ja", "タマンラセット県"), ("ka", "თამანრასეტის პროვინცია"), ("kn", "ತಮಂಗ\u{ccd}ಹಾಸ\u{cc6}ಟ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "타만라세트 주"), ("lt", "Tamanraseto vilaja"), ("lv", "Tamanrāsetas vilāja"), ("mr", "तमा\u{902}गसास\u{947}ट प\u{94d}रा\u{902}त"), ("ms", "Tamanghasset"), ("nb", "Tamanrasset"), ("nl", "Tamanrasset"), ("no", "Tamanrasset"), ("pl", "Prowincja Tamanrasset"), ("pt", "Tamanghasset"), ("ro", "Provincia Tamanrasset"), ("ru", "Таманрассет"), ("si", "තමන\u{dca}ගස\u{dca}සෙට\u{dca} පළ\u{dcf}ත"), ("sr", "Таманрасет"), ("sr_Latn", "Tamanraset"), ("sv", "Tamanrasset"), ("sw", "Jimbo ya Tamanghasset"), ("ta", "த\u{bbe}மங\u{bcd}அஸெட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ట\u{c3e}మంగ\u{c3e}స\u{c46}ట\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เจเซน\u{e34}เช"), ("tr", "Tamanrasset Vilayeti"), ("uk", "Таманрассет"), ("ur", "صوبہ تمنراست"), ("vi", "Tamanrasset"), ("zh", "塔曼拉塞特省")]),
                        unofficial_name_list: ["Fort-Laperrine", "Tamanghist", "Tamanrasset"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "12",
                        country_alpha2: Alpha2::DZ,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.4), longitude: Some(8.116667000000001), max_latitude: Some(35.4423093), min_latitude: Some(35.3811426), max_longitude: Some(8.170857999999999), min_longitude: Some(8.0502319)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية تبسة"), ("be", "Правінцыя Тэбеса"), ("bg", "Тебеса"), ("bn", "তেবেস\u{9cd}য\u{9be} প\u{9cd}রদেশ"), ("ca", "Tebessa"), ("ccp", "𑄑𑄬𑄝𑄬𑄥"), ("ceb", "Wilaya de Tébessa"), ("da", "Tébessa Province"), ("de", "Tebessa"), ("el", "Τεμπέσσα"), ("en", "Tébessa"), ("es", "Provincia de Tébessa"), ("eu", "Tébessa probintzia"), ("fa", "استان تبسه"), ("fi", "Tébessan lääni"), ("fr", "wilaya de Tébessa"), ("gl", "Provincia de Tébessa"), ("gu", "ટ\u{ac7}બ\u{ac7}સા પ\u{acd}રા\u{a82}ત"), ("hi", "त\u{947}ब\u{947}स\u{94d}स प\u{94d}रा\u{902}त"), ("id", "Provinsi Tebessa"), ("it", "provincia di Tébessa"), ("ja", "テベッサ県"), ("kn", "ಟ\u{cc6}ಬ\u{cc6}ಸ\u{ccd}ಸ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "테베사 주"), ("lt", "Tebesos vilaja"), ("lv", "Tibisas vilāja"), ("mr", "ट\u{947}बस\u{947}सा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Tébessa"), ("nb", "Tébessa"), ("nl", "Tébessa"), ("no", "Tébessa"), ("pl", "Prowincja Tibissa"), ("pt", "Tébessa"), ("ro", "Provincia Tébessa"), ("ru", "Тебесса"), ("si", "ටෙබෙස\u{dca}ස\u{dcf} පළ\u{dcf}ත"), ("sr", "Тебеса"), ("sr_Latn", "Tebesa"), ("sv", "Tébessa"), ("sw", "Wilaya ya Tébessa"), ("ta", "டேபிஎஸ\u{bcd}ஸ\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ట\u{c46}బ\u{c46}స\u{c4d}స\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเทเบซซ\u{e48}า"), ("tr", "Tebesa Vilayeti"), ("uk", "Тебесса"), ("ur", "صوبہ تبسہ"), ("vi", "Tébessa"), ("zh", "泰贝萨省")]),
                        unofficial_name_list: ["Tbessa", "Tébessa"].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "13",
                        country_alpha2: Alpha2::DZ,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.882776), longitude: Some(-1.31667), max_latitude: Some(34.9344265), min_latitude: Some(34.8574818), max_longitude: Some(-1.2593079), min_longitude: Some(-1.3708879)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية تلمسان"), ("az", "Tlemsen vilayəti"), ("be", "Правінцыя Тлемсен"), ("bg", "Тлемсен"), ("bn", "তলেমচেন প\u{9cd}রদেশ"), ("ca", "Província de Tlemcen"), ("ccp", "𑄑\u{11133}𑄣𑄬𑄟𑄥𑄬𑄚\u{11134}"), ("ceb", "Wilaya de Tlemcen"), ("da", "Tlemcen Province"), ("de", "Tlemcen"), ("el", "Τλεμσέν"), ("en", "Tlemcen"), ("es", "Provincia de Tlemecén"), ("eu", "Tlemcen probintzia"), ("fa", "استان تلمسان"), ("fi", "Tlemcenin kunta"), ("fr", "wilaya de Tlemcen"), ("gl", "Provincia de Tlemcen"), ("gu", "ટલ\u{ac7}મસન પ\u{acd}રા\u{a82}ત"), ("hi", "त\u{94d}ल\u{947}म\u{94d}स\u{947}न प\u{94d}रा\u{902}त"), ("hu", "Tilimszán tartomány"), ("id", "Provinsi Tilimsan"), ("it", "provincia di Tlemcen"), ("ja", "トレムセン県"), ("ka", "ტლემსენის პროვინცია"), ("kn", "ಟ\u{ccd}ಲ\u{cc6}ಮ\u{ccd}ಸ\u{cc6}ನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "틀렘센 주"), ("lt", "Tlemseno vilaja"), ("lv", "Tlīmsānas vilāja"), ("mr", "टल\u{947}मसन प\u{94d}रा\u{902}त"), ("ms", "Wilayah Tlemcen"), ("nb", "Tlemcen"), ("nl", "Tlemcen"), ("no", "Tlemcen"), ("pl", "Prowincja Tilimsan"), ("pt", "Tlemcen"), ("ro", "Provincia Tlemcen"), ("ru", "Тлемсен"), ("si", "ට\u{dca}ලෙම\u{dca}සන\u{dca} පළ\u{dcf}ත"), ("sr", "Тлемсен"), ("sr_Latn", "Tlemsen"), ("sv", "Tlemcen"), ("ta", "ட\u{bcd}லேம\u{bcd}ஸன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ట\u{c4d}ల\u{c46}మ\u{c4d}స\u{c46}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเตรมเซน"), ("tr", "Tlemsan Vilayeti"), ("uk", "Тлемсен"), ("ur", "صوبہ تلمسان"), ("vi", "Tlemcen"), ("zh", "特莱姆森省")]),
                        unofficial_name_list: ["Tilimsen", "Tlemcen"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::DZ,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.3673553), longitude: Some(1.3220322), max_latitude: Some(35.4112982), min_latitude: Some(35.3086815), max_longitude: Some(1.4936257), min_longitude: Some(1.2896061)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية تيارت"), ("az", "Tiaret vilayəti"), ("be", "Правінцыя Тыярэт"), ("bg", "Тиарет"), ("bn", "তিয\u{9bc}\u{9be}রেত প\u{9cd}রদেশ"), ("ca", "Província de Tiaret"), ("ccp", "𑄑𑄠𑄢𑄬𑄖\u{11134}"), ("ceb", "Wilaya de Tiaret"), ("da", "Tiaret Province"), ("de", "Tiaret"), ("el", "Τιαρέτ"), ("en", "Tiaret"), ("es", "Provincia de Tiaret"), ("eu", "Tiaret probintzia"), ("fa", "استان تیارت"), ("fi", "Tiaretin provinssi"), ("fr", "wilaya de Tiaret"), ("gl", "Provincia de Tiaret"), ("gu", "તિયાર\u{ac7}ટ પ\u{acd}રા\u{a82}ત"), ("hi", "टिअर\u{947}त प\u{94d}रा\u{902}त"), ("id", "Provinsi Tiaret"), ("it", "provincia di Tiaret"), ("ja", "ティアレット県"), ("ka", "ტიარეტის პროვინცია"), ("kn", "ಟ\u{cbf}ಯಾರ\u{cc6}ಟ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "티아레트 주"), ("lt", "Tiareto vilaja"), ("lv", "Tiaretas vilāja"), ("mr", "तियार\u{947}ट प\u{94d}रा\u{902}त"), ("ms", "Wilayah Tiaret"), ("nb", "Tiaret"), ("nl", "Tiaret"), ("no", "Tiaret"), ("pl", "Prowincja Tijarat"), ("pt", "Tiaret"), ("ro", "Provincia Tiaret"), ("ru", "Тиарет"), ("si", "ටයරෙට\u{dca} පළ\u{dcf}ත"), ("sr", "Тијарет"), ("sr_Latn", "Tijaret"), ("sv", "Tiaret"), ("sw", "Jimbo ya Tiaret"), ("ta", "டிய\u{bbe}ரெட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ట\u{c3f}య\u{c3e}ర\u{c46}ట\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเท\u{e35}ยเรต"), ("tr", "Tiyaret Vilayeti"), ("uk", "Тіарет"), ("ur", "صوبہ تیارت"), ("vi", "Tiaret"), ("zh", "提亚雷特省")]),
                        unofficial_name_list: ["Tiaret", "Tihert"].to_vec(),
                    }
                ),
                (
                    "15",
                    Subdivision{
                        name: "15",
                        country_alpha2: Alpha2::DZ,
                        code: "15",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.716667), longitude: Some(4.05), max_latitude: Some(36.7565764), min_latitude: Some(36.6438373), max_longitude: Some(4.1750836), min_longitude: Some(3.9758921)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية تيزي وزو"), ("az", "Tizi-Uzu vilayəti"), ("be", "Правінцыя Тызі-Узу"), ("bg", "Тизи Узу"), ("bn", "তিজি ওউজ\u{9c1} প\u{9cd}রদেশ"), ("ca", "Província de Tizi Uzu"), ("ccp", "𑄑\u{11128}𑄎\u{11128} 𑄃\u{11127}𑄅\u{1112b}𑄎\u{1112f}"), ("ceb", "Wilaya de Tizi Ouzou"), ("da", "Tizi Ouzou Province"), ("de", "Tizi Ouzou"), ("el", "Τιζί Ουζού"), ("en", "Tizi Ouzou"), ("es", "Provincia de Tizi Ouzou"), ("eu", "Tizi Uzu probintzia"), ("fa", "استان تیزی وزو"), ("fi", "Tizi Ouzoun lääni"), ("fr", "wilaya de Tizi Ouzou"), ("gl", "Provincia de Tizi Ouzou"), ("gu", "ટીઝી ઓઝોઉ\u{a82} પ\u{acd}રા\u{a82}ત"), ("hi", "तिज\u{93c}ि ओउज\u{93c}ोउ प\u{94d}रा\u{902}त"), ("id", "Provinsi Tizi Ouzou"), ("it", "provincia di Tizi Ouzou"), ("ja", "ティジ・ウズー県"), ("ka", "ტიზი-უზუს პროვინცია"), ("kn", "ಟ\u{cbf}ಜ\u{cbf} ಒಜುವ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "티지우주 주"), ("lt", "Tizi Uzu vilaja"), ("lv", "Tīzī Vuzū vilāja"), ("mr", "तीझी औझ\u{941} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Tizi Ouzou"), ("nb", "Tizi Ouzou"), ("nl", "Tizi Ouzou"), ("no", "Tizi Ouzou"), ("pl", "Prowincja Tizi Wuzu"), ("pt", "Tizi Ouzou"), ("ro", "Provincia Tizi Ouzou"), ("ru", "Тизи-Узу"), ("si", "ට\u{dd2}ස\u{dd2} ඕස\u{dd4} පළ\u{dcf}ත"), ("sr", "Тизи Узу"), ("sr_Latn", "Tizi Uzu"), ("sv", "Tizi Ouzou"), ("sw", "Jimbo ya Tizi Ouzou"), ("ta", "டிஜி ஒஉஸ\u{bcd}வு ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ట\u{c3f}జ\u{c3f} ఔజ\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ค\u{e31}นทร\u{e35}\u{e48} เวสม\u{e35}นท\u{e4c}"), ("tr", "Tizi Vuzu Vilayeti"), ("uk", "Тізі-Узу"), ("ur", "صوبہ تیزی وزو"), ("vi", "Tizi Ouzou"), ("zh", "提济乌祖省")]),
                        unofficial_name_list: ["Tizi-Ouzou"].to_vec(),
                    }
                ),
                (
                    "16",
                    Subdivision{
                        name: "16",
                        country_alpha2: Alpha2::DZ,
                        code: "16",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.752887), longitude: Some(3.042048), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية الجزائر"), ("be", "правінцыя Алжыр"), ("bg", "Алжир"), ("ca", "Província d’Alger"), ("ccp", "𑄃𑄣\u{11134}𑄎\u{1112d}𑄠𑄢\u{11134}𑄥\u{11134}"), ("ceb", "Wilaya d’ Alger"), ("da", "Algier"), ("de", "Algier"), ("en", "Algiers"), ("es", "Provincia de Argel"), ("eu", "Aljer probintzia"), ("fa", "ولایة الجزائر"), ("fr", "wilaya d’Alger"), ("gl", "Provincia de Alxer"), ("hi", "अल\u{94d}जीयर\u{94d}स प\u{94d}रा\u{902}त"), ("id", "Provinsi Aljir"), ("it", "provincia di Algeri"), ("ja", "アルジェ県"), ("ka", "ალჟირის პროვინცია"), ("ko", "알제 주"), ("lt", "Alžyro vilaja"), ("ms", "Wilayah Algiers"), ("nb", "Alger"), ("nl", "Algiers"), ("no", "Alger"), ("pl", "Prowincja Algier"), ("pt", "Argel"), ("ro", "Provincia Alger"), ("ru", "Алжир"), ("sr", "Алжир"), ("sr_Latn", "Alžir"), ("sv", "Alger"), ("sw", "Jimbo ya Algiers"), ("tr", "Cezayir Vilayeti"), ("uk", "Алжир"), ("ur", "صوبہ الجزائر"), ("vi", "Algiers"), ("zh", "阿尔及尔省")]),
                        unofficial_name_list: ["Al-Jazair", "Al-Jazaʿir", "Alger", "Algier", "El Djazair", "al-Jazāʿir"].to_vec(),
                    }
                ),
                (
                    "17",
                    Subdivision{
                        name: "17",
                        country_alpha2: Alpha2::DZ,
                        code: "17",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.666667), longitude: Some(3.25), max_latitude: Some(34.7007481), min_latitude: Some(34.6235673), max_longitude: Some(3.2964777), min_longitude: Some(3.201828)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية الجلفة"), ("be", "Правінцыя Джэльфа"), ("bg", "Джелфа"), ("bn", "ডিলফ\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Djelfa"), ("ccp", "𑄎𑄬𑄣\u{11134}𑄜"), ("ceb", "Wilaya de Djelfa"), ("da", "Djelfa Province"), ("de", "Djelfa"), ("el", "Τζέλφα"), ("en", "Djelfa"), ("es", "Provincia de Djelfa"), ("eu", "Djelfa probintzia"), ("fa", "استان جلفه"), ("fi", "Djelfan lääni"), ("fr", "wilaya de Djelfa"), ("gl", "Provincia de Djelfa"), ("gu", "જ\u{ac7}લ\u{acd}ફા પ\u{acd}રા\u{a82}ત"), ("hi", "ज\u{947}ल\u{94d}फा प\u{94d}रा\u{902}त"), ("id", "Provinsi Jalfah"), ("it", "provincia di Djelfa"), ("ja", "ジェルファ県"), ("ka", "ჯელფის პროვინცია"), ("kn", "ಡ\u{cbf}ಜ\u{cc6}ಲ\u{ccd}ಫಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "젤파 주"), ("lt", "Dželfos vilaja"), ("lv", "Džilfas vilāja"), ("mr", "जिल\u{94d}फा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Djelfa"), ("nb", "Djelfa"), ("nl", "Djelfa"), ("no", "Djelfa"), ("pl", "Prowincja Dżilfa"), ("pt", "Djelfa"), ("ro", "Provincia Djelfa"), ("ru", "Джельфа"), ("si", "ඩ\u{dca}ජෙල\u{dca}ෆ\u{dcf} පළ\u{dcf}ත"), ("sr", "Џелфа"), ("sr_Latn", "Dželfa"), ("sv", "Djelfa"), ("sw", "Jimbo ya Djelfa"), ("ta", "ட\u{bcd}ஜெலபிஆ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "జ\u{c46}ల\u{c4d}ఫ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เทศมณฑลบ\u{e35}สตร\u{e35}ตซา-นะเซาด\u{e4c}"), ("tr", "Celfa Vilayeti"), ("uk", "Джельфа"), ("ur", "صوبہ الجلفہ"), ("vi", "Djelfa"), ("zh", "杰勒法省")]),
                        unofficial_name_list: ["Djelfa", "El Djelfa"].to_vec(),
                    }
                ),
                (
                    "18",
                    Subdivision{
                        name: "18",
                        country_alpha2: Alpha2::DZ,
                        code: "18",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.81673869999999), longitude: Some(5.749093299999999), max_latitude: Some(36.8293218), min_latitude: Some(36.7631604), max_longitude: Some(5.822110299999999), min_longitude: Some(5.678882799999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية جيجل"), ("be", "Правінцыя Джыджэль"), ("bg", "Джиджел"), ("bn", "জিয\u{9bc}েল প\u{9cd}রদেশ"), ("ca", "Província de Jijel"), ("ccp", "𑄎\u{11128}𑄎𑄬𑄣\u{11134}"), ("ceb", "Wilaya de Jijel"), ("da", "Jijel Province"), ("de", "Jijel"), ("el", "Τζίτζελ"), ("en", "Jijel"), ("es", "Provincia de Jijel"), ("fa", "استان جیجل"), ("fi", "Jijelin lääni"), ("fr", "wilaya de Jijel"), ("gl", "Provincia de Jijel"), ("gu", "જિજ\u{ac7}લ પ\u{acd}રા\u{a82}ત"), ("hi", "जिज\u{947}ल प\u{94d}रा\u{902}त"), ("id", "Provinsi Jijel"), ("it", "provincia di Jijel"), ("ja", "ジジェル県"), ("ka", "ჯიჯელის პროვინცია"), ("kn", "ಜ\u{cbf}ಜ\u{cc6}ಲ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "지젤 주"), ("lt", "Džidželio vilaja"), ("lv", "Džīdžilas vilāja"), ("mr", "जिझल प\u{94d}रा\u{902}त"), ("ms", "Wilayah Jijel"), ("nb", "Jijel"), ("nl", "Jijel"), ("no", "Jijel"), ("pl", "Prowincja Dżidżal"), ("pt", "Jijel"), ("ro", "Provincia Jijel"), ("ru", "Джиджель"), ("si", "ජ\u{dd2}ජෙල\u{dca} පළ\u{dcf}ත"), ("sr", "Џиџел"), ("sr_Latn", "Džidžel"), ("sv", "Jijel"), ("sw", "Jimbo ya Jijel"), ("ta", "ஜிஜேல\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "జ\u{c3f}జ\u{c46}ల\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e35}เจล"), ("tr", "Cicel Vilayeti"), ("uk", "Джиджель"), ("ur", "صوبہ جیجل"), ("vi", "Jijel"), ("zh", "吉杰勒省")]),
                        unofficial_name_list: ["Djidjel", "Djidjeli", "Djidjelli", "Jijel"].to_vec(),
                    }
                ),
                (
                    "19",
                    Subdivision{
                        name: "19",
                        country_alpha2: Alpha2::DZ,
                        code: "19",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.1969027), longitude: Some(5.4150871), max_latitude: Some(36.2473189), min_latitude: Some(36.1337157), max_longitude: Some(5.507240299999999), min_longitude: Some(5.333186500000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية سطيف"), ("be", "Правінцыя Сетыф"), ("bg", "Сетиф"), ("bn", "সেতিফ প\u{9cd}রদেশ"), ("ca", "Província de Sétif"), ("ccp", "𑄥𑄬𑄑\u{11128}𑄛\u{11134}"), ("ceb", "Wilaya de Sétif"), ("da", "Sétif Province"), ("de", "Sétif"), ("el", "Σετίφ"), ("en", "Sétif"), ("es", "Provincia de Sétif"), ("et", "Sétifi provints"), ("fa", "استان سطيف"), ("fi", "Sétifin provinssi"), ("fr", "wilaya de Sétif"), ("gl", "Provincia de Sétif"), ("gu", "સ\u{ac7}ટીફ પ\u{acd}રા\u{a82}ત"), ("hi", "स\u{947}तिफ प\u{94d}रा\u{902}त"), ("id", "Provinsi Sathif"), ("it", "provincia di Sétif"), ("ja", "セティフ県"), ("ka", "სეტიფის პროვინცია"), ("kn", "ಸ\u{cc6}ಟ\u{cbf}ಫ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "세티프 주"), ("lt", "Setifo vilaja"), ("lv", "Satīfas vilāja"), ("mr", "स\u{948}टफ प\u{94d}रा\u{902}त"), ("ms", "Wilayah Sétif"), ("nb", "Sétif"), ("nl", "Sétif"), ("no", "Sétif"), ("pl", "Prowincja Satif"), ("pt", "Sétif"), ("ro", "Provincia Sétif"), ("ru", "Сетиф"), ("si", "සේට\u{dd2}ෆ\u{dca} පළ\u{dcf}ත"), ("sr", "Сетиф"), ("sr_Latn", "Setif"), ("sv", "Sétif"), ("sw", "Jimbo ya Sétif"), ("ta", "செடிப\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c46}ట\u{c3f}ఫ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เซต\u{e34}ฟ"), ("tr", "Setif Vilayeti"), ("uk", "Сетіф"), ("ur", "صوبہ سطیف"), ("vi", "Sétif"), ("zh", "塞提夫省")]),
                        unofficial_name_list: ["Setif", "Stif", "Sétif"].to_vec(),
                    }
                ),
                (
                    "20",
                    Subdivision{
                        name: "20",
                        country_alpha2: Alpha2::DZ,
                        code: "20",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.8412014), longitude: Some(0.1484305), max_latitude: Some(34.8716372), min_latitude: Some(34.8122529), max_longitude: Some(0.1772404), min_longitude: Some(0.1194763)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية سعيدة"), ("be", "Правінцыя Саіда"), ("bg", "Сайда"), ("bn", "স\u{9be}ইদ\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Saïda"), ("ccp", "𑄥\u{1112d}𑄓"), ("ceb", "Wilaya de Saïda"), ("cs", "Saida"), ("da", "Saïda Province"), ("de", "Saida"), ("el", "Σάιντα"), ("en", "Saïda"), ("es", "Provincia de Saida"), ("fa", "استان سعیده"), ("fi", "Saïdan maakunta"), ("fr", "wilaya de Saïda"), ("gl", "Provincia de Saïda"), ("gu", "સઈદા પ\u{acd}રા\u{a82}ત"), ("hi", "स\u{948}दा प\u{94d}रा\u{902}त"), ("id", "Provinsi Saida"), ("it", "provincia di Saida"), ("ja", "サイダ県"), ("ka", "საიდის პროვინცია"), ("kn", "ಸೈದಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "사이다 주"), ("lt", "Saidos vilaja"), ("lv", "Saīdas vilāja"), ("mr", "सईदा प\u{94d}रा\u{902}त"), ("ms", "Saida Province"), ("nb", "Saida"), ("nl", "Saïda"), ("no", "Saida"), ("pl", "Prowincja Saida"), ("pt", "Saïda"), ("ro", "Provincia Saïda"), ("ru", "Саида"), ("si", "සය\u{dd2}ඩ\u{dcf} පළ\u{dcf}ත"), ("sr", "Сајида"), ("sr_Latn", "Sajida"), ("sv", "Saïda"), ("sw", "Jimbo ya Saida"), ("ta", "சைட\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3e}య\u{c3f}ద\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดไซดา"), ("tr", "Saida Vilayeti"), ("uk", "Саїда"), ("ur", "سیدہ علاقہ"), ("vi", "Saida"), ("zh", "赛伊达省")]),
                        unofficial_name_list: ["Saida", "Saïda"].to_vec(),
                    }
                ),
                (
                    "21",
                    Subdivision{
                        name: "21",
                        country_alpha2: Alpha2::DZ,
                        code: "21",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.866667), longitude: Some(6.899999999999999), max_latitude: Some(36.9170289), min_latitude: Some(36.8385534), max_longitude: Some(7.026958599999999), min_longitude: Some(6.8444823)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية سكيكدة"), ("be", "Правінцыя Скікда"), ("bg", "Скикда"), ("bn", "স\u{9cd}কিকদ\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Skikda"), ("ccp", "𑄇\u{11128}𑄇\u{11134}𑄓"), ("ceb", "Wilaya de Skikda"), ("da", "Skikda Province"), ("de", "Skikda"), ("el", "Σκίκντα"), ("en", "Skikda"), ("es", "Provincia de Skikda"), ("fa", "استان سکیکده"), ("fi", "Skikdan lääni"), ("fr", "wilaya de Skikda"), ("gl", "Provincia de Skikda"), ("gu", "સ\u{acd}કિકડા પ\u{acd}રા\u{a82}ત"), ("hi", "स\u{94d}किक\u{94d}डा प\u{94d}रा\u{902}त"), ("id", "Provinsi Skikda"), ("it", "provincia di Skikda"), ("ja", "スキクダ県"), ("ka", "სკიკდის პროვინცია"), ("kn", "ಸ\u{ccd}ಕ\u{cbf}ಕ\u{ccd}ಡಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "스키크다 주"), ("lt", "Skikdos vilaja"), ("lv", "Skīkidas vilāja"), ("mr", "स\u{94d}किक\u{94d}डा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Skikda"), ("nb", "Skikda"), ("nl", "Skikda"), ("no", "Skikda"), ("pl", "Prowincja Sukajkida"), ("pt", "Skikda"), ("ro", "Provincia Skikda"), ("ru", "Скикда"), ("si", "ස\u{dca}ක\u{dd2}ක\u{dca}ද\u{dcf} පළ\u{dcf}ත"), ("sr", "Скикда"), ("sr_Latn", "Skikda"), ("sv", "Skikda"), ("sw", "Jimbo ya Skikda"), ("ta", "சகிக\u{bcd}கிட ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c4d}క\u{c3f}క\u{c4d}డ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดสก\u{e34}กดา"), ("tr", "Sekikda Vilayeti"), ("uk", "Скікда"), ("ur", "صوبہ سکیکدہ"), ("vi", "Skikda"), ("zh", "斯基克达省")]),
                        unofficial_name_list: ["Skikda"].to_vec(),
                    }
                ),
                (
                    "22",
                    Subdivision{
                        name: "22",
                        country_alpha2: Alpha2::DZ,
                        code: "22",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.2), longitude: Some(-0.641389), max_latitude: Some(35.2516998), min_latitude: Some(35.1609683), max_longitude: Some(-0.5712032), min_longitude: Some(-0.6890702999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية سيدي بلعباس"), ("be", "Правінцыя Сідзі-Бель-Абес"), ("bg", "Сиди Бел Абес"), ("bn", "সিদি বেল আব\u{9cd}বেস প\u{9cd}রদেশ"), ("ca", "Província de Sidi Bel Abbes"), ("ccp", "𑄥\u{11128}𑄓\u{11128} 𑄝𑄬𑄣\u{11134} 𑄃𑄝𑄬𑄌\u{11134}"), ("ceb", "Wilaya de Sidi Bel Abbès"), ("da", "Sidi Bel Abbès Province"), ("de", "Sidi Bel Abbès"), ("el", "Σίντι Μπελ Άμπες"), ("en", "Sidi Bel Abbès"), ("es", "Provincia de Sidi Bel Abbes"), ("fa", "استان سیدی بلعباس"), ("fi", "Sidi Bel Abbèsn provinssi"), ("fr", "wilaya de Sidi Bel Abbès"), ("gl", "Provincia de Sidi Bel Abbes"), ("gu", "સિદી બ\u{ac7}લ અબ\u{acd}બ\u{ac7}સ પ\u{acd}રા\u{a82}ત"), ("hi", "सिदी ब\u{947}ल अब\u{94d}बास प\u{94d}रा\u{902}त"), ("it", "provincia di Sidi Bel Abbes"), ("ja", "シディ・ベル・アッベス県"), ("ka", "სიდი-ბელ-აბესის პროვინცია"), ("kn", "ಸ\u{cbf}ಡ\u{cbf} ಬ\u{cc6}ಲ\u{ccd} ಅಬ\u{ccd}ಬ\u{cc6}ಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "시디벨아베스 주"), ("lt", "Sidi bel Abeso vilaja"), ("lv", "Sīdī Bil Abāsas vilāja"), ("mr", "सिदी ब\u{947}ल अब\u{94d}ब\u{947} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Sidi Bel Abbès"), ("nb", "Sidi bel Abbès"), ("nl", "Sidi-bel-Abbès"), ("no", "Sidi bel Abbès"), ("pl", "Prowincja Sidi Bu-l-Abbas"), ("pt", "Sidi Bel Abbès"), ("ro", "Provincia Sidi Bel Abbès"), ("ru", "Сиди-Бель-Аббес"), ("si", "ස\u{dd2}ඩ\u{dd2} බෙල\u{dca} අබ\u{dca}බේස\u{dca} පළ\u{dcf}ත"), ("sr", "Сиди Бел Абес"), ("sr_Latn", "Sidi Bel Abes"), ("sv", "Sidi Bel Abbès"), ("sw", "Jimbo ya Sidi Bel Abbes"), ("ta", "சிதி பெல\u{bcd} அபிபிஎஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3f}డ\u{c3f} బ\u{c46}ల\u{c4d} అబ\u{c46}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e34}ด\u{e35}เบลอ\u{e31}บแบส"), ("tr", "Sidi Belabbas Vilayeti"), ("uk", "Сіді-Бель-Аббес"), ("ur", "صوبہ سیدی بلعباس"), ("vi", "Sidi Bel Abbes"), ("zh", "西迪贝勒阿巴斯省")]),
                        unofficial_name_list: ["Sidi bel Abbès"].to_vec(),
                    }
                ),
                (
                    "23",
                    Subdivision{
                        name: "23",
                        country_alpha2: Alpha2::DZ,
                        code: "23",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.9), longitude: Some(7.766667), max_latitude: Some(36.9698538), min_latitude: Some(36.8656479), max_longitude: Some(7.793147699999999), min_longitude: Some(7.695407899999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية عنابة"), ("az", "Annaba vilayəti"), ("be", "Правінцыя Анаба"), ("bg", "Анаба"), ("bn", "আন\u{9cd}ন\u{9be}ব\u{9be} প\u{9cd}রদেশ"), ("ca", "Província d’Annaba"), ("ccp", "𑄃𑄚𑄝"), ("ceb", "Annaba (lalawigan)"), ("da", "Annaba"), ("de", "Annaba"), ("el", "Ανάμπα"), ("en", "Annaba"), ("es", "Provincia de Annaba"), ("fa", "استان عنابه"), ("fi", "Annaban maakunta"), ("fr", "wilaya d’Annaba"), ("gl", "Provincia de Annaba"), ("gu", "અનાબા પ\u{acd}રા\u{a82}ત"), ("hi", "अन\u{94d}नाबा प\u{94d}रा\u{902}त"), ("id", "Provinsi ‘Annabah"), ("it", "provincia di Annaba"), ("ja", "アンナバ県"), ("ka", "ანაბის პროვინცია"), ("kn", "ಅನಾಬಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "안나바 주"), ("lt", "Anabos vilaja"), ("lv", "Annābas vilāja"), ("mr", "अन\u{94d}नबा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Annaba"), ("nb", "Annaba"), ("nl", "Annaba"), ("no", "Annaba"), ("pl", "Prowincja Annaba"), ("pt", "Annaba"), ("ro", "Provincia Annaba"), ("ru", "Аннаба"), ("si", "අන\u{dca}නබ\u{dcf} පළ\u{dcf}ත"), ("sr", "Анаба"), ("sr_Latn", "Anaba"), ("sv", "Annaba"), ("sw", "Jimbo ya Annaba"), ("ta", "அன\u{bcd}னப\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అన\u{c4d}న\u{c3e}బ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e31}นนาบะ"), ("tr", "Annaba Vilayeti"), ("uk", "Аннаба"), ("ur", "صوبہ عنابہ"), ("vi", "Annaba"), ("zh", "安纳巴省")]),
                        unofficial_name_list: ["Annaba", "Bona", "Bône"].to_vec(),
                    }
                ),
                (
                    "24",
                    Subdivision{
                        name: "24",
                        country_alpha2: Alpha2::DZ,
                        code: "24",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.45), longitude: Some(7.433332999999999), max_latitude: Some(36.4795519), min_latitude: Some(36.4342657), max_longitude: Some(7.469287), min_longitude: Some(7.397575399999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية قالمة"), ("be", "Правінцыя Гельма"), ("bg", "Гелма"), ("bn", "গ\u{9c1}য\u{9bc}েল\u{9cd}ম\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Guelma"), ("ccp", "𑄉\u{1112a}𑄠𑄬𑄣\u{11134}𑄟"), ("ceb", "Wilaya de Guelma"), ("da", "Guelma Province"), ("de", "Guelma"), ("el", "Γκέλμα"), ("en", "Guelma"), ("es", "Provincia de Guelma"), ("fa", "استان قالمه"), ("fi", "Guelman provinssi"), ("fr", "wilaya de Guelma"), ("gl", "Provincia de Guelma"), ("gu", "ગ\u{ac1}એલ\u{acd}મા પ\u{acd}રા\u{a82}ત"), ("hi", "ग\u{941}एलमा प\u{94d}रा\u{902}त"), ("id", "Provinsi Guelma"), ("it", "provincia di Guelma"), ("ja", "ゲルマ県"), ("ka", "გუელმის პროვინცია"), ("kn", "ಗುಲ\u{ccd}ಮಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "겔마 주"), ("lt", "Gelmos vilaja"), ("lv", "Gālmas vilāja"), ("mr", "ग\u{94d}वाल\u{94d}मा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Guelma"), ("nb", "Guelma"), ("nl", "Guelma"), ("no", "Guelma"), ("pl", "Prowincja Kalima"), ("pt", "Guelma"), ("ro", "Provincia Guelma"), ("ru", "Гельма"), ("si", "ග\u{dd4}එල\u{dca}ම\u{dcf} පළ\u{dcf}ත"), ("sr", "Гелма"), ("sr_Latn", "Gelma"), ("sv", "Guelma"), ("sw", "Jimbo ya Guelma"), ("ta", "கோயில\u{bcd}ம\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గ\u{c4d}వ\u{c46}ల\u{c4d}మ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ก\u{e31}วมา"), ("tr", "Kalime Vilayeti"), ("uk", "Гельма"), ("ur", "صوبہ قالمہ"), ("vi", "Guelma"), ("zh", "盖勒马省")]),
                        unofficial_name_list: ["Guelma"].to_vec(),
                    }
                ),
                (
                    "25",
                    Subdivision{
                        name: "25",
                        country_alpha2: Alpha2::DZ,
                        code: "25",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.35), longitude: Some(6.6), max_latitude: Some(36.44905290000001), min_latitude: Some(36.2589189), max_longitude: Some(6.7665481), min_longitude: Some(6.5284056)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية قسنطينة"), ("be", "Правінцыя Канстанціна"), ("bg", "Константин"), ("bn", "কন\u{9cd}সট\u{9cd}য\u{9be}ট\u{9be}ইন প\u{9cd}রদেশ"), ("ca", "Província de Constantina"), ("ccp", "𑄇\u{11127}𑄚\u{11134}𑄥\u{11134}𑄑𑄚\u{11134}𑄑\u{1112d}𑄚\u{11134}"), ("ceb", "Wilaya de Constantine"), ("da", "Constantine Province"), ("de", "Constantine"), ("el", "Κωνσταντίνη"), ("en", "Constantine"), ("es", "Provincia de Constantina"), ("fa", "استان قسنطینه"), ("fi", "Constantinen lääni"), ("fr", "wilaya de Constantine"), ("gl", "Provincia de Constantina"), ("gu", "કોન\u{acd}સ\u{acd}ટ\u{ac7}ન\u{acd}ટાઇન પ\u{acd}રા\u{a82}ત"), ("hi", "कॉन\u{94d}स\u{94d}ट\u{948}न\u{94d}टाइन प\u{94d}रा\u{902}त"), ("id", "Provinsi Qusnathinah"), ("it", "provincia di Costantina"), ("ja", "コンスタンティーヌ県"), ("ka", "კონსტანტინის პროვინცია"), ("kn", "ಕಾನ\u{ccd}ಸ\u{ccd}ಟಂಟೈನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "콩스탕틴 주"), ("lt", "Konstantinos vilaja"), ("lv", "Konstantinas vilāja"), ("mr", "कॉन\u{94d}स\u{94d}टन\u{94d}टाईन प\u{94d}रा\u{902}त"), ("ms", "Wilayah Constantine"), ("nb", "Constantine"), ("nl", "Constantine"), ("no", "Constantine"), ("pl", "Prowincja Konstantyna"), ("pt", "Constantina"), ("ro", "Provincia Constantine"), ("ru", "Константина"), ("si", "කොන\u{dca}ස\u{dca}ටන\u{dca}ටය\u{dd2}න\u{dca} පළ\u{dcf}ත"), ("sr", "Константин"), ("sr_Latn", "Konstantin"), ("sv", "Constantine"), ("sw", "Jimbo ya Constantine"), ("ta", "க\u{bbe}ன\u{bcd}ஸ\u{bcd}ட\u{bbe}ன\u{bcd}டின\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}న\u{c4d}స\u{c4d}ట\u{c3e}ంట\u{c3f}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดคอนสแตนต\u{e34}น"), ("tr", "Konstantine Vilayeti"), ("uk", "Константіна"), ("ur", "صوبہ قسنطینہ"), ("vi", "Constantine"), ("zh", "君士坦丁省")]),
                        unofficial_name_list: ["Constantine", "Ksontina", "Qacentina", "Qoussantina", "Qusanţīnah"].to_vec(),
                    }
                ),
                (
                    "26",
                    Subdivision{
                        name: "26",
                        country_alpha2: Alpha2::DZ,
                        code: "26",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.266667), longitude: Some(2.75), max_latitude: Some(36.3364236), min_latitude: Some(36.2312581), max_longitude: Some(2.823658), min_longitude: Some(2.7220345)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية المدية"), ("be", "Правінцыя Медэа"), ("bg", "Мостаганем"), ("bn", "মেদিয\u{9bc}\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Médéa"), ("ccp", "𑄟𑄬𑄓𑄬𑄠"), ("ceb", "Wilaya de Médéa"), ("da", "Médéa Province"), ("de", "Medea"), ("el", "Μεντέα"), ("en", "Médéa"), ("es", "Provincia de Médéa"), ("fa", "استان مدیه"), ("fi", "Médéan provinssi"), ("fr", "wilaya de Médéa"), ("gl", "Provincia de Médéa"), ("gu", "મ\u{ac7}દ\u{ac7}યા પ\u{acd}રા\u{a82}ત"), ("hi", "म\u{947}डी प\u{94d}रा\u{902}त"), ("id", "Provinsi Medea"), ("it", "provincia di Médéa"), ("ja", "メデア県"), ("ka", "მედეას პროვინცია"), ("kn", "ಮ\u{cc6}ಡ\u{cc6}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "메데아 주"), ("lt", "Medėjos vilaja"), ("lv", "Medejas vilāja"), ("mr", "म\u{947}द\u{947}या प\u{94d}रा\u{902}त"), ("ms", "Wilayah Médéa"), ("nb", "Medea"), ("nl", "Médéa"), ("no", "Medea"), ("pl", "Prowincja Al-Midija"), ("pt", "Médéa"), ("ro", "Provincia Médéa"), ("ru", "Медеа"), ("si", "මෙද\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sr", "Медеја"), ("sr_Latn", "Medeja"), ("sv", "Médéa"), ("sw", "Jimbo ya Médéa"), ("ta", "ம\u{bc0}ட\u{bc0}ய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c46}డ\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดม\u{e35}เด\u{e35}ย"), ("tr", "Mediye Vilayeti"), ("uk", "Медея"), ("ur", "صوبہ المدیہ"), ("vi", "Médéa"), ("zh", "麦迪亚省")]),
                        unofficial_name_list: ["Lemdiyya", "Médéa", "al-Midyah"].to_vec(),
                    }
                ),
                (
                    "27",
                    Subdivision{
                        name: "27",
                        country_alpha2: Alpha2::DZ,
                        code: "27",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.933333), longitude: Some(0.08333299999999999), max_latitude: Some(36.0394952), min_latitude: Some(35.9107075), max_longitude: Some(0.1765537), min_longitude: Some(0.0536443)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية مستغانم"), ("be", "Правінцыя Мастаганем"), ("bg", "Мостаганем²"), ("bn", "মোস\u{9cd}ত\u{9be}গ\u{9be}নেম প\u{9cd}রদেশ"), ("ca", "Província de Mostaganem"), ("ccp", "𑄟\u{11127}𑄌\u{11134}𑄑𑄉𑄚𑄬𑄟\u{11134}"), ("ceb", "Wilaya de Mostaganem"), ("da", "Mostaganem"), ("de", "Mostaganem"), ("el", "Μοσταγκανέμ"), ("en", "Mostaganem"), ("es", "Provincia de Mostaganem"), ("fa", "استان مستغانم"), ("fi", "Mostaganemin lääni"), ("fr", "wilaya de Mostaganem"), ("gl", "Provincia de Mostaganem"), ("gu", "મોસ\u{acd}ટગન\u{ac7}મ પ\u{acd}રા\u{a82}ત"), ("hi", "मोस\u{94d}तगन\u{947}म प\u{94d}रा\u{902}त"), ("id", "Provinsi Mostaganem"), ("it", "provincia di Mostaganem"), ("ja", "モスタガネム県"), ("ka", "მოსტაგანემის პროვინცია"), ("kn", "ಮೋಸ\u{ccd}ಟ\u{ccd}ಗನೇಮ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "모스타가넴 주"), ("lt", "Mostaganemo vilaja"), ("lv", "Mustegānimas vilāja"), ("mr", "मोस\u{94d}टग\u{947}\u{902}म प\u{94d}रा\u{902}त"), ("ms", "Wilayah Mostaganem"), ("nb", "Mostaganem"), ("nl", "Mostaganem"), ("no", "Mostaganem"), ("pl", "Prowincja Mustaghanam"), ("pt", "Mostaganem"), ("ro", "Provincia Mostaganem"), ("ru", "Мостаганем"), ("si", "මොස\u{dca}ටගනෙම\u{dca} පළ\u{dcf}ත"), ("sr", "Мостаганем"), ("sr_Latn", "Mostaganem"), ("sv", "Mostaganem"), ("sw", "Jimbo ya Mostaganem"), ("ta", "மோஸ\u{bcd}தகனேம\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c4b}స\u{c4d}ట\u{c3e}గ\u{c3e}న\u{c46}మ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมอสตากาเรม"), ("tr", "Mostaganem Vilayeti"), ("uk", "Мостаганем"), ("ur", "صوبہ مستغانم"), ("vi", "Mostaganem"), ("zh", "穆斯塔加奈姆省")]),
                        unofficial_name_list: ["Mestghanem", "Mostaganem", "Mustaghanam", "Mustaghanim", "Mustaġānam"].to_vec(),
                    }
                ),
                (
                    "28",
                    Subdivision{
                        name: "28",
                        country_alpha2: Alpha2::DZ,
                        code: "28",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.701944), longitude: Some(4.547222), max_latitude: Some(35.8723598), min_latitude: Some(35.5699353), max_longitude: Some(4.605503199999999), min_longitude: Some(4.431953399999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية المسيلة"), ("be", "Правінцыя Мсіла"), ("bn", "ম’সিল\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de M’Sila"), ("ccp", "𑄟\u{11133}𑄃𑄥\u{11128}𑄣"), ("ceb", "Wilaya de M’Sila"), ("da", "M’Sila Province"), ("de", "M’Sila"), ("el", "Εμσίλα"), ("en", "M’Sila"), ("es", "Provincia de M’Sila"), ("fa", "استان مسیله"), ("fi", "M’Silan lääni"), ("fr", "wilaya de M’Sila"), ("gl", "Provincia de M’Sila"), ("gu", "મ’સિલા પ\u{acd}રા\u{a82}ત"), ("hi", "म’सिला प\u{94d}रा\u{902}त"), ("id", "Provinsi M’Sila"), ("it", "provincia di M’Sila"), ("ja", "ムシラ県"), ("kn", "ಎಂ ಸ\u{cbf}ಲಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "음실라 주"), ("lt", "Msilos vilaja"), ("lv", "Msīlas vilāja"), ("mr", "एम’सिल प\u{94d}रा\u{902}त"), ("ms", "Wilayah M’Sila"), ("nb", "M’Sila"), ("nl", "M’Sila"), ("no", "M’Sila"), ("pl", "Prowincja Al-Masila"), ("pt", "M’Sila"), ("ro", "Provincia M’Sila"), ("ru", "Мсила"), ("si", "මස\u{dd2}ල\u{dcf} පළ\u{dcf}ත"), ("sr", "Мсила"), ("sr_Latn", "Msila"), ("sv", "M’Sila"), ("sw", "Jimbo ya M’Sila"), ("ta", "ம\u{bcd} ‘சில\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఎంస\u{c3f}ల\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเอ\u{e47}มซ\u{e34}ลา"), ("tr", "M’Sila Vilayeti"), ("uk", "Мсіла"), ("ur", "صوبہ المسیلہ"), ("vi", "M’Sila"), ("zh", "姆西拉省")]),
                        unofficial_name_list: ["Msila", "MʿSila"].to_vec(),
                    }
                ),
                (
                    "29",
                    Subdivision{
                        name: "29",
                        country_alpha2: Alpha2::DZ,
                        code: "29",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.3904125), longitude: Some(0.1494988), max_latitude: Some(35.7647711), min_latitude: Some(34.984219), max_longitude: Some(0.9324479999999999), min_longitude: Some(-0.4987059999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية معسكر"), ("be", "правінцыя Маскара"), ("bg", "Маскара"), ("bn", "ম\u{9be}স\u{9cd}ক\u{9be}র\u{9be}র প\u{9cd}রদেশ"), ("ca", "Província de Mascara"), ("ccp", "𑄟𑄌\u{11134}𑄇𑄢"), ("ceb", "Wilaya de Mascara"), ("da", "Mascara Province"), ("de", "Muaskar"), ("el", "Μάσκαρα"), ("en", "Mascara"), ("es", "Provincia de Muaskar"), ("fa", "استان معسکر"), ("fi", "Mascaran lääni"), ("fr", "wilaya de Mascara"), ("gl", "Provincia de Mascara"), ("gu", "મસ\u{acd}કરા પ\u{acd}રા\u{a82}ત"), ("hi", "मस\u{94d}कारा प\u{94d}रा\u{902}त"), ("id", "Provinsi Mascara"), ("it", "provincia di Mascara"), ("ja", "マスカラ県"), ("ka", "მასკარის პროვინცია"), ("kn", "ಮಸ\u{ccd}ಕರಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "마스카라 주"), ("lt", "Maskaros vilaja"), ("lv", "Maskaras vilāja"), ("mr", "मस\u{94d}करा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Mascara"), ("nb", "Muaskar"), ("nl", "Mascara"), ("no", "Muaskar"), ("pl", "Prowincja Muaskar"), ("pt", "Mascara"), ("ro", "Provincia Mascara"), ("ru", "Маскара"), ("si", "මස\u{dca}ක\u{dcf}ර\u{dcf} පළ\u{dcf}ත"), ("sr", "Маскара"), ("sr_Latn", "Maskara"), ("sv", "Muaskar"), ("sw", "Jimbo ya Mascara"), ("ta", "ம\u{bbe}ஸ\u{bcd}க\u{bcd}க\u{bbe}ர ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మస\u{c4d}క\u{c3e}ర\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมาสการ\u{e48}า"), ("tr", "Muaskar Vilayeti"), ("uk", "Маскара"), ("ur", "صوبہ معسکر"), ("vi", "Mascara"), ("zh", "穆阿斯凯尔省")]),
                        unofficial_name_list: ["Mascara", "Mouaskar"].to_vec(),
                    }
                ),
                (
                    "30",
                    Subdivision{
                        name: "30",
                        country_alpha2: Alpha2::DZ,
                        code: "30",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.95), longitude: Some(5.316667), max_latitude: Some(32.5074455), min_latitude: Some(31.874059), max_longitude: Some(5.4502058), min_longitude: Some(4.463195799999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية ورقلة"), ("az", "Uarqla vilayəti"), ("be", "Правінцыя Уаргла"), ("bg", "Уаргла"), ("bn", "উয\u{9bc}\u{9be}র\u{9cd}গল\u{9be} প\u{9cd}রদেশ"), ("ca", "Província d’Ouargla"), ("ccp", "𑄃\u{1112f}𑄢\u{11134}𑄉\u{11133}𑄣"), ("ceb", "Wilaya de Ouargla"), ("da", "Ouargla Province"), ("de", "Ouargla"), ("el", "Ουέργκλα"), ("en", "Ouargla"), ("es", "Provincia de Ouargla"), ("fa", "استان ورقله"), ("fi", "Ouarglan maakunta"), ("fr", "wilaya d’Ouargla"), ("gl", "Provincia de Ouargla"), ("gu", "ઔઆર\u{acd}ગ\u{acd}લા પ\u{acd}રા\u{a82}ત"), ("hi", "ऑर\u{94d}ग\u{94d}ला प\u{94d}रा\u{902}त"), ("id", "Provinsi Ouargla"), ("it", "provincia di Ouargla"), ("ja", "ワルグラ県"), ("ka", "უარგლის პროვინცია"), ("kn", "ಓರ\u{ccd}ಗಾಲಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "와르글라 주"), ("lt", "Uarglos vilaja"), ("lv", "Varglas vilāja"), ("mr", "औअर\u{94d}गला प\u{94d}रा\u{902}त"), ("ms", "Wilayah Ouargla"), ("nb", "Ouargla"), ("nl", "Ouargla"), ("no", "Ouargla"), ("pl", "Prowincja Warkala"), ("pt", "Ouargla"), ("ro", "Provincia Ouargla"), ("ru", "Уаргла"), ("si", "ඖහම\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sr", "Варгла"), ("sr_Latn", "Vargla"), ("sv", "Ouargla"), ("sw", "Jimbo ya Ouargla"), ("ta", "ஓவர\u{bcd}கள\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఉవ\u{c3e}ర\u{c4d}గల\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e31}วกลา"), ("tr", "Vurkla Vilayeti"), ("uk", "Уаргла"), ("ur", "صوبہ ورقلہ"), ("vi", "Ouargla"), ("zh", "瓦尔格拉省")]),
                        unofficial_name_list: ["Ouargla", "Wargla"].to_vec(),
                    }
                ),
                (
                    "31",
                    Subdivision{
                        name: "31",
                        country_alpha2: Alpha2::DZ,
                        code: "31",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.6976541), longitude: Some(-0.6337376), max_latitude: Some(35.769723), min_latitude: Some(35.6451955), max_longitude: Some(-0.5530071), min_longitude: Some(-0.7298182999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية وهران"), ("be", "Правінцыя Аран"), ("bg", "Оран"), ("bn", "অরণ"), ("ca", "Província d’Orà"), ("ccp", "𑄃\u{11127}𑄢𑄚\u{11134}"), ("ceb", "Oran"), ("da", "Oran"), ("de", "Oran"), ("el", "Οράν"), ("en", "Oran"), ("es", "Provincia de Orán"), ("fa", "استان وهران"), ("fi", "Oranin maakunta"), ("fr", "wilaya d’Oran"), ("gl", "Provincia de Orán"), ("gu", "ઓર\u{ac7}ન"), ("hi", "ओरान प\u{94d}रा\u{902}त"), ("id", "Oran"), ("it", "provincia di Orano"), ("ja", "オラン県"), ("ka", "ორანის პროვინცია"), ("kn", "ಒರಾನ\u{ccd}"), ("ko", "오랑 주"), ("lt", "Orano vilaja"), ("lv", "Orānas vilāja"), ("mr", "ऑरान"), ("ms", "Wilayah Oran"), ("nb", "Oran"), ("nl", "Oran"), ("no", "Oran"), ("pl", "Prowincja Oran"), ("pt", "Orã"), ("ro", "Provincia Oran"), ("ru", "Оран"), ("si", "ඔර\u{dcf}න\u{dca}"), ("sr", "Оран"), ("sr_Latn", "Oran"), ("sv", "Oran"), ("sw", "Jimbo ya Oran"), ("ta", "ஓர\u{bbe}ன\u{bcd}"), ("te", "ఓర\u{c3e}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโอร\u{e31}น"), ("tr", "Oran Vilayeti"), ("uk", "Оран"), ("ur", "صوبہ وھران"), ("vi", "Oran"), ("zh", "奥兰省")]),
                        unofficial_name_list: ["Oran", "Ouahran"].to_vec(),
                    }
                ),
                (
                    "32",
                    Subdivision{
                        name: "32",
                        country_alpha2: Alpha2::DZ,
                        code: "32",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.680278), longitude: Some(1.020278), max_latitude: Some(33.7897053), min_latitude: Some(33.4892988), max_longitude: Some(1.1710738), min_longitude: Some(0.8449173000000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية البيض"), ("be", "Правінцыя Эль-Баяд"), ("bg", "Ел-Баяд"), ("bn", "এল ব\u{9be}য\u{9bc}\u{9be}ধ প\u{9cd}রদেশ"), ("ca", "Província d’El Bayadh"), ("ccp", "𑄃𑄬𑄣\u{11134} 𑄝𑄠𑄖\u{11134}"), ("ceb", "El Bayadh (lalawigan)"), ("da", "El Bayadh"), ("de", "El Bayadh"), ("el", "Ελ Μπάγιαδ"), ("en", "El Bayadh"), ("es", "Provincia de El Bayadh"), ("fa", "استان البیض"), ("fi", "El Bayadhn lääni"), ("fr", "wilaya d’El Bayadh"), ("gl", "Provincia de El Bayadh"), ("gu", "અલ બાયાદ પ\u{acd}રા\u{a82}ત"), ("hi", "एल बयादा प\u{94d}रा\u{902}त"), ("id", "Provinsi El Bayadh"), ("it", "provincia di El Bayadh"), ("ja", "エル・バヤード県"), ("ka", "ელ-ბაიადის პროვინცია"), ("kn", "ಎಲ\u{ccd} ಬಯಾದ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "엘바야드 주"), ("lt", "Bajado vilaja"), ("lv", "Bejedas vilāja"), ("mr", "एल ब\u{947}अद प\u{94d}रा\u{902}त"), ("ms", "Wilayah El Bayadh"), ("nb", "El Bayadh"), ("nl", "El Bayadh"), ("no", "El Bayadh"), ("pl", "Prowincja Al-Bajad"), ("pt", "El Bayadh"), ("ro", "Provincia El Bayadh"), ("ru", "Эль-Баяд"), ("si", "එල\u{dca} බයදෝ පළ\u{dcf}ත"), ("sr", "Ел Бајад"), ("sr_Latn", "El Bajad"), ("sv", "El Bayadh"), ("sw", "Jimbo ya El Bayadh"), ("ta", "எல\u{bcd} பயத\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఎల\u{c4d} బయ\u{c3e}ధ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเอล บาหยาด"), ("tr", "El Beyaz Vilayeti"), ("uk", "Ель-Баяд"), ("ur", "صوبہ البیض"), ("vi", "El Bayadh"), ("zh", "巴亚兹省")]),
                        unofficial_name_list: ["El Bayadh"].to_vec(),
                    }
                ),
                (
                    "33",
                    Subdivision{
                        name: "33",
                        country_alpha2: Alpha2::DZ,
                        code: "33",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(26.1690005), longitude: Some(8.4842465), max_latitude: Some(30.1144791), min_latitude: Some(23.0151409), max_longitude: Some(11.979548), min_longitude: Some(5.8658301)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية إليزي"), ("be", "Правінцыя Ілізі"), ("bg", "Илизи"), ("bn", "ইলিজি প\u{9cd}রদেশ"), ("ca", "Província d’Illizi"), ("ccp", "𑄃\u{11128}𑄣\u{11133}𑄣\u{11128}𑄎\u{11128}"), ("ceb", "Illizi"), ("cs", "Illizi (provincie)"), ("da", "Illizi Province"), ("de", "Illizi"), ("el", "Ιλλίζι"), ("en", "Illizi"), ("es", "Provincia de Illizi"), ("fa", "استان الیزی"), ("fi", "Illizin maakunta"), ("fr", "wilaya d’Illizi"), ("gl", "Provincia de Illizi"), ("gu", "ઇલિઝી પ\u{acd}રા\u{a82}ત"), ("hi", "इल\u{94d}लिज\u{93c}ि प\u{94d}रा\u{902}त"), ("id", "Provinsi Illizi"), ("it", "provincia di Illizi"), ("ja", "イリジ県"), ("ka", "ილიზის პროვინცია"), ("kn", "ಇಲ\u{ccd}ಜ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "일리지 주"), ("lt", "Ilizio vilaja"), ("lv", "Illīzī vilāja"), ("mr", "इलिझी प\u{94d}रा\u{902}त"), ("ms", "Wilayah Illizi"), ("nb", "Illizi"), ("nl", "Illizi"), ("no", "Illizi"), ("pl", "Prowincja Illizi"), ("pt", "Illizi"), ("ro", "Provincia Illizi"), ("ru", "Иллизи"), ("si", "ඉල\u{dca}ල\u{dd2}ස\u{dca} පළ\u{dcf}ත"), ("sr", "Илизи"), ("sr_Latn", "Ilizi"), ("sv", "Illizi"), ("sw", "Jimbo la Illizi"), ("ta", "இல\u{bcd}லிஜி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఇల\u{c4d}ల\u{c3f}జ\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "อ\u{e34}ลล\u{e34}ซ\u{e35}"), ("tr", "İlizi Vilayeti"), ("uk", "Іллізі"), ("ur", "صوبہ الیزی"), ("vi", "Illizi"), ("zh", "伊利济省")]),
                        unofficial_name_list: ["Illizi"].to_vec(),
                    }
                ),
                (
                    "34",
                    Subdivision{
                        name: "34",
                        country_alpha2: Alpha2::DZ,
                        code: "34",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.0704188), longitude: Some(4.7564046), max_latitude: Some(36.1206129), min_latitude: Some(36.0166846), max_longitude: Some(4.848232299999999), min_longitude: Some(4.6901322)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية برج بوعريريج"), ("az", "Borc-Bu-Arreric vilayəti"), ("be", "Правінцыя Бордж-Бу-Арэрыдж"), ("bg", "Бордж Бу Ареридж"), ("bn", "বোর\u{9cd}ডজ ব\u{9c1} প\u{9cd}রদেশ"), ("ca", "Província de Bordj Bou Arreridj"), ("ccp", "𑄝\u{11127}𑄢\u{11133}𑄓\u{11134}𑄎\u{11134} 𑄝\u{1112f} 𑄃𑄢𑄬𑄢\u{11128}𑄖\u{11134}𑄎\u{11134}"), ("ceb", "Wilaya de Bordj Bou Arréridj"), ("da", "Bordj Bou Arréridj"), ("de", "Bordj Bou Arreridj"), ("el", "Μπορντζ Μπου Αρερίντζ"), ("en", "Bordj Bou Arréridj"), ("es", "Provincia de Bordj Bou Arréridj"), ("fa", "استان برج بوعریریج"), ("fi", "Bordj Bou Arréridjin provinssi"), ("fr", "wilaya de Bordj Bou Arreridj"), ("gl", "Provincia de Bordj Bou Arréridj"), ("gu", "બોર\u{acd}ડજ બૌ એર\u{ac7}ગ\u{acd}રીડ પ\u{acd}રા\u{a82}ત"), ("hi", "बोर\u{94d}दज बोउ अर\u{94d}र\u{947}रिदज प\u{94d}रा\u{902}त"), ("id", "Provinsi Bordj Bou Arreridj"), ("it", "provincia di Bordj Bou Arreridj"), ("ja", "ボルジ・ブ・アレリジ県"), ("ka", "ბორჯ-ბუ-არერიჯის პროვინცია"), ("kn", "ಬೋರ\u{ccd}ಜ\u{ccd} ಬ\u{ccc} ಅರ\u{cc6}ರ\u{cbf}ಡ\u{ccd}ಜ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "보르즈부아레리즈 주"), ("lt", "Bordž Bu Areridžo vilaja"), ("lv", "Bordž Bū Arrīridžas vilāja"), ("mr", "बर\u{94d}डज बाऊ एरर\u{947}रिड प\u{94d}रा\u{902}त"), ("ms", "Wilayah Bordj Bou Arréridj"), ("nb", "Bordj Bou Arréridj"), ("nl", "Bordj Bou Arréridj"), ("no", "Bordj Bou Arréridj"), ("pl", "Prowincja Burdż Bu Urajridż"), ("pt", "Bordj Bou Arreridj"), ("ro", "Provincia Bordj Bou Arréridj"), ("ru", "Бордж-Бу-Арреридж"), ("si", "බොර\u{dca}ද\u{dca}ජ\u{dd4} බෞ අර\u{dca}රේර\u{dd2}ඩ\u{dca}ජ\u{dca} පළ\u{dcf}ත"), ("sr", "Борџ Бу Арериџ"), ("sr_Latn", "Bordž Bu Areridž"), ("sv", "Bordj Bou Arréridj"), ("sw", "Jimbo ya Bordj Bou Arréridj"), ("ta", "போர\u{bcd}ட\u{bcd}ஜ\u{bcd} போது அர\u{bcd}ரேரிட\u{bcd}ஜ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c4b}ర\u{c4d}జ\u{c4d} బ\u{c4c} ఆర\u{c3f}డ\u{c4d}జ\u{c46} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "บอร\u{e4c}จ บ\u{e31}ว อาร\u{e4c}เรร\u{e34}ดจ\u{e4c}"), ("tr", "Burc Bu Ariric Vilayeti"), ("uk", "Бордж-Бу-Аррерідж"), ("ur", "صوبہ برج بو عریریج"), ("vi", "Bordj Bou Arréridj"), ("zh", "布阿拉里季堡省")]),
                        unofficial_name_list: ["Bordj Bou Arreridj"].to_vec(),
                    }
                ),
                (
                    "35",
                    Subdivision{
                        name: "35",
                        country_alpha2: Alpha2::DZ,
                        code: "35",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.7675962), longitude: Some(3.7029002), max_latitude: Some(36.9245898), min_latitude: Some(36.526798), max_longitude: Some(4.1178311), min_longitude: Some(3.301881)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية بومرداس"), ("be", "Правінцыя Бумердэс"), ("bg", "Бумердес"), ("bn", "বৌমেরডেস প\u{9cd}রদেশ"), ("ca", "Província de Boumerdès"), ("ccp", "𑄝\u{1112f}𑄟𑄢\u{11134}𑄓𑄬𑄌\u{11134}"), ("ceb", "Wilaya de Boumerdes"), ("da", "Boumerdès Province"), ("de", "Boumerdes"), ("el", "Μπουμερντές"), ("en", "Boumerdès"), ("es", "Provincia de Boumerdès"), ("fa", "استان بومرداس"), ("fi", "Boumerdèsin lääni"), ("fr", "wilaya de Boumerdès"), ("gl", "Provincia de Boumerdès"), ("gu", "બોમ\u{ac7}ર\u{acd}ડ\u{ac7}સ પ\u{acd}રા\u{a82}ત"), ("hi", "बोउम\u{947}र\u{94d}ड\u{947}स प\u{94d}रा\u{902}त"), ("id", "Provinsi Boumerdes"), ("it", "provincia di Boumerdès"), ("ja", "ブメルデス県"), ("ka", "ბუმერდესის პროვინცია"), ("kn", "ಬೊಮ\u{cc6}ರ\u{ccd}ಡ\u{cc6}ಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "부메르데스 주"), ("lt", "Bumerdeso vilaja"), ("lv", "Būmerdāsas vilāja"), ("mr", "बोम\u{947}र\u{94d}ड\u{94d}स प\u{94d}रा\u{902}त"), ("ms", "Wilayah Boumerdès"), ("nb", "Boumerdès"), ("nl", "Boumerdès"), ("no", "Boumerdès"), ("pl", "Prowincja Bumardas"), ("pt", "Boumerdès"), ("ro", "Provincia Boumerdès"), ("ru", "Бумердес"), ("si", "බෞමෙර\u{dca}ඩෙස\u{dca} පළ\u{dcf}ත"), ("sr", "Бумердес"), ("sr_Latn", "Bumerdes"), ("sv", "Boumerdès"), ("sw", "Jimbo ya Boumerdès"), ("ta", "பௌமெர\u{bcd}டேஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c4c}మ\u{c46}ర\u{c4d}డ\u{c46}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e31}วเมอเดส"), ("tr", "Bumerdas Vilayeti"), ("uk", "Бумердес"), ("ur", "صوبہ بومرداس"), ("vi", "Boumerdès"), ("zh", "布米尔达斯省")]),
                        unofficial_name_list: ["Boumerdès"].to_vec(),
                    }
                ),
                (
                    "36",
                    Subdivision{
                        name: "36",
                        country_alpha2: Alpha2::DZ,
                        code: "36",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.7558581), longitude: Some(8.2212979), max_latitude: Some(36.9521553), min_latitude: Some(36.3699299), max_longitude: Some(8.67615), min_longitude: Some(7.730847)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية الطارف"), ("be", "Правінцыя Эль-Тарф"), ("bg", "Ел-Тарф"), ("bn", "এল ট\u{9be}রফ প\u{9cd}রদেশ"), ("ca", "Província d’El Tarf"), ("ccp", "𑄃𑄬𑄣\u{11134} 𑄑𑄢\u{11134}𑄛\u{11134}"), ("ceb", "El Tarf"), ("da", "El Tarf Province"), ("de", "El Tarf"), ("el", "Ελ Ταρφ"), ("en", "El Tarf"), ("es", "Provincia de El Tarf"), ("fa", "استان الطارف"), ("fi", "El Tarfin lääni"), ("fr", "wilaya d’El Tarf"), ("gl", "Provincia de El Taref"), ("gu", "અલ ટાર\u{acd}ફ પ\u{acd}રા\u{a82}ત"), ("hi", "एल तर\u{947}फ प\u{94d}रा\u{902}त"), ("hu", "El Tarf tartomány"), ("id", "Provinsi El Tarf"), ("it", "provincia di El Tarf"), ("ja", "エル・タルフ県"), ("ka", "ელ-ტარეფის პროვინცია"), ("kn", "ಎಲ\u{ccd} ಟ\u{ccd}ಯಾಫ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "엘타르프 주"), ("lt", "Tarefo vilaja"), ("lv", "Tārfas vilāja"), ("mr", "एल तारफ प\u{94d}रा\u{902}त"), ("ms", "El Tarf Province"), ("nb", "El Tarf"), ("nl", "El Tarf"), ("no", "El Tarf"), ("pl", "Prowincja At-Tarif"), ("pt", "El Tarf"), ("ro", "Provincia El Tarf"), ("ru", "Эль-Тарф"), ("si", "එල\u{dca} ට\u{dcf}ර\u{dca}ෆ\u{dca} පළ\u{dcf}ත"), ("sr", "Ел Тарф"), ("sr_Latn", "El Tarf"), ("sv", "El Tarf"), ("sw", "Jimbo ya El Tarf"), ("ta", "எல\u{bcd} தரப\u{bcd}பி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఎల\u{c4d} టర\u{c4d}ఫ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเอล ทาร\u{e4c}ฟ"), ("tr", "El Tarif Vilayeti"), ("uk", "Ат-Тарф"), ("ur", "صوبہ الطارف"), ("vi", "El Taref"), ("zh", "塔里夫省")]),
                        unofficial_name_list: ["El Taref", "El Tarf", "at-Tarf"].to_vec(),
                    }
                ),
                (
                    "37",
                    Subdivision{
                        name: "37",
                        country_alpha2: Alpha2::DZ,
                        code: "37",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.6761012), longitude: Some(-8.1276526), max_latitude: Some(28.9236408), min_latitude: Some(25.5684593), max_longitude: Some(-3.9839172), min_longitude: Some(-8.6666626)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية تندوف"), ("az", "Tinduf vilayəti"), ("be", "Правінцыя Тындуф"), ("bg", "Тиндуф"), ("bn", "তিন\u{9cd}দ\u{9c1}ফ প\u{9cd}রদেশ"), ("ca", "Província de Tindouf"), ("ccp", "𑄑\u{11128}𑄚\u{11134}𑄓\u{1112f}𑄛\u{11134}"), ("ceb", "Wilaya de Tindouf"), ("da", "Tindouf Province"), ("de", "Tindouf"), ("el", "Τιντούφ"), ("en", "Tindouf"), ("es", "Provincia de Tinduf"), ("fa", "استان تندوف"), ("fi", "Tindoufin maakunta"), ("fr", "wilaya de Tindouf"), ("gl", "Provincia de Tindouf"), ("gu", "ટિન\u{acd}ડૌફ પ\u{acd}રા\u{a82}ત"), ("he", "מחוז טינדוף"), ("hi", "टिन\u{94d}डोउफ प\u{94d}रा\u{902}त"), ("id", "Provinsi Tindouf"), ("it", "provincia di Tindouf"), ("ja", "ティンドゥフ県"), ("ka", "ტინდუფის პროვინცია"), ("kn", "ಟ\u{cbf}ಂಡೋಫ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "틴두프 주"), ("lt", "Tindufo vilaja"), ("lv", "Tīndūfas vilāja"), ("mr", "टि\u{902}डौफ प\u{94d}रा\u{902}त"), ("ms", "Tindouf"), ("nb", "Tindouf"), ("nl", "Tindouf"), ("no", "Tindouf"), ("pl", "Prowincja Tinduf"), ("pt", "Tindouf"), ("ro", "Provincia Tindouf"), ("ru", "Тиндуф"), ("si", "ට\u{dd2}න\u{dca}ඩොඋෆ\u{dca} පළ\u{dcf}ත"), ("sr", "Тиндуф"), ("sr_Latn", "Tinduf"), ("sv", "Tindouf"), ("sw", "Jimbo ya Tindouf"), ("ta", "டின\u{bcd}டோவுப\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ట\u{c3f}ండ\u{c4b}ఫ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดท\u{e34}นดอฟ"), ("tr", "Tinduf Vilayeti"), ("uk", "Тіндуф"), ("ur", "صوبہ تندوف"), ("vi", "Tindouf"), ("zh", "廷杜夫省")]),
                        unofficial_name_list: ["Tindouf"].to_vec(),
                    }
                ),
                (
                    "38",
                    Subdivision{
                        name: "38",
                        country_alpha2: Alpha2::DZ,
                        code: "38",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.607778), longitude: Some(1.811111), max_latitude: Some(35.6178491), min_latitude: Some(35.5865321), max_longitude: Some(1.8383218), min_longitude: Some(1.7660523)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية تيسمسيلت"), ("az", "Tissemsilt vilayəti"), ("be", "Правінцыя Тысемсілт"), ("bg", "Тисемсилт"), ("bn", "টিসসেমসিলত প\u{9cd}রদেশ"), ("ca", "Província de Tissemsilt"), ("ccp", "𑄑\u{11128}𑄥𑄬𑄟\u{11134}𑄥\u{11128}𑄣\u{11134}𑄑\u{11134}"), ("ceb", "Wilaya de Tissemsilt"), ("da", "Tissemsilt Province"), ("de", "Tissemsilt"), ("el", "Τισσεμσίλτ"), ("en", "Tissemsilt"), ("es", "Provincia de Tissemsilt"), ("fa", "تسمسیلت"), ("fi", "Tissemsiltin provinssi"), ("fr", "wilaya de Tissemsilt"), ("gl", "Provincia de Tissemsilt"), ("gu", "ટીસ\u{ac7}મ\u{acd}સિલ\u{acd}ટ પ\u{acd}રા\u{a82}ત"), ("hi", "टिस\u{94d}स\u{947}म\u{94d}सिल\u{94d}त प\u{94d}रा\u{902}त"), ("id", "Provinsi Tissemsilt"), ("it", "provincia di Tissemsilt"), ("ja", "ティセムシルト県"), ("ka", "ტისემსილტის პროვინცია"), ("kn", "ಟ\u{cbf}ಸ\u{ccd}ಸ\u{cc6}ಮ\u{ccd}ಸ\u{cbf}ಲ\u{ccd}ಟ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "티셈실트 주"), ("lt", "Tisemsilto vilaja"), ("lv", "Tisemsīltas vilāja"), ("mr", "टीसिम\u{947}ल\u{94d}ल\u{94d}ट प\u{94d}रा\u{902}त"), ("ms", "Wilayah Tissemsilt"), ("nb", "Tissemsilt"), ("nl", "Tissemsilt"), ("no", "Tissemsilt"), ("pl", "Prowincja Tisamsilt"), ("pt", "Tissemsilt"), ("ro", "Provincia Tissemsilt"), ("ru", "Тиссемсилт"), ("si", "ට\u{dd2}සේම\u{dd2}ස\u{dd2}ල\u{dca}ට\u{dca} පළ\u{dcf}ත"), ("sr", "Тисемсилт"), ("sr_Latn", "Tisemsilt"), ("sv", "Tissemsilt"), ("sw", "Jimbo ya Tissemsilt"), ("ta", "டிஸ\u{bcd}ஸம\u{bcd}ஸில\u{bcd}ட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ట\u{c3f}స\u{c46}మ\u{c4d}\u{200c}స\u{c3f}ల\u{c4d}ట\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดท\u{e34}สแสมซอล\u{e4c}ท"), ("tr", "Tissemsilt Vilayeti"), ("uk", "Тіссемсілт"), ("ur", "صوبہ تسمسیلت"), ("vi", "Tissemsilt"), ("zh", "提塞姆西勒特省")]),
                        unofficial_name_list: ["Tissemselt", "Tissemsilt"].to_vec(),
                    }
                ),
                (
                    "39",
                    Subdivision{
                        name: "39",
                        country_alpha2: Alpha2::DZ,
                        code: "39",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.3713397), longitude: Some(6.8479682), max_latitude: Some(33.412323), min_latitude: Some(33.3297394), max_longitude: Some(6.8943501), min_longitude: Some(6.820836099999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية الوادي"), ("be", "Правінцыя Эль-Уэд"), ("bg", "Ел-Уед"), ("bn", "এল ওয\u{9bc}েড প\u{9cd}রদেশ"), ("ca", "Província d’El Oued"), ("ccp", "𑄃𑄬𑄣\u{11134} 𑄃\u{1112e}𑄠𑄬𑄖\u{11134}"), ("ceb", "El Oued"), ("da", "El Oued"), ("de", "El Oued"), ("el", "Ελ Ουέντ"), ("en", "El Oued"), ("es", "Provincia de El Oued"), ("fa", "استان الوادی"), ("fi", "El Oued Province"), ("fr", "wilaya d’El Oued"), ("gl", "Provincia de El Oued"), ("gu", "એલ ઓયડ પ\u{acd}રા\u{a82}ત"), ("hi", "एल ऑड"), ("id", "Provinsi El Oued"), ("it", "provincia di El Oued"), ("ja", "エル・ウェッド県"), ("ka", "ელ-უედის პროვინცია"), ("kn", "ಎಲ\u{ccd} ಔಡ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "엘웨드 주"), ("lt", "Uedo vilaja"), ("lv", "Vādas vilāja"), ("mr", "एल ओयड प\u{94d}रा\u{902}त"), ("ms", "Wilayah El Oued"), ("nb", "El Oued"), ("nl", "El Oued"), ("no", "El Oued"), ("pl", "Prowincja Al-Wadi"), ("pt", "El Oued"), ("ro", "Provincia El Oued"), ("ru", "Эль-Уэд"), ("si", "එල\u{dca} ඔය\u{dd2}ඩ\u{dca} කල\u{dcf}පය"), ("sr", "Ел Ујед"), ("sr_Latn", "El Ujed"), ("sv", "El Oued"), ("sw", "Jimbo ya El Oued"), ("ta", "எல\u{bcd} யது ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఎల\u{c4d} ఓయ\u{c46}ద\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเอลโลด\u{e4c}"), ("tr", "El Vadi Vilayeti"), ("uk", "Ель-Уед"), ("ur", "صوبہ الوادی"), ("vi", "El Oued"), ("zh", "瓦迪省")]),
                        unofficial_name_list: ["El Oued", "El Ouâdi", "El Wad"].to_vec(),
                    }
                ),
                (
                    "40",
                    Subdivision{
                        name: "40",
                        country_alpha2: Alpha2::DZ,
                        code: "40",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.416667), longitude: Some(7.133332999999999), max_latitude: Some(35.4608097), min_latitude: Some(35.3918488), max_longitude: Some(7.1690083), min_longitude: Some(7.113818999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية خنشلة"), ("az", "Xenşela vilayəti"), ("be", "Правінцыя Хеншэла"), ("bg", "Хеншела"), ("bn", "কেনচেল\u{9cd}ল\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Khenchela"), ("ccp", "𑄈𑄬𑄚\u{11134}𑄥𑄬𑄣"), ("ceb", "Wilaya de Khenchela"), ("da", "Khenchela"), ("de", "Khenchela"), ("el", "Χενσέλα"), ("en", "Khenchela"), ("es", "Provincia de Khenchela"), ("fa", "استان خنشله"), ("fi", "Khenchelan lääni"), ("fr", "wilaya de Khenchela"), ("gl", "Provincia de Khenchela"), ("gu", "ખ\u{ac7}\u{a82}ચ\u{ac7}લા પ\u{acd}રા\u{a82}ત"), ("hi", "ख\u{947}न\u{94d}छ\u{947}ला प\u{94d}रा\u{902}त"), ("id", "Provinsi Khenchela"), ("it", "provincia di Khenchela"), ("ja", "ヘンシュラ県"), ("ka", "ხენშელის პროვინცია"), ("kn", "ಖ\u{cc6}ಂಚ\u{cc6}ಲಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "켄셸라 주"), ("lt", "Chenšelos vilaja"), ("lv", "Hanselas vilāja"), ("mr", "कि\u{902}ख\u{947}ला प\u{94d}रा\u{902}त"), ("ms", "Wilayah Khenchela"), ("nb", "Khenchela"), ("nl", "Khenchela"), ("no", "Khenchela"), ("pl", "Prowincja Chanszala"), ("pt", "Khenchela"), ("ro", "Provincia Khenchela"), ("ru", "Хеншела"), ("si", "ඛේන\u{dca}චෙල\u{dcf} පළ\u{dcf}ත"), ("sr", "Хеншела"), ("sr_Latn", "Henšela"), ("sv", "Khenchela"), ("sw", "Jimbo ya Khenchela"), ("ta", "க\u{bcd}ஹெஞ\u{bcd}சேலை ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఖ\u{c46}ంచ\u{c47}ల\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเคนเชลา"), ("tr", "Henşle Vilayeti"), ("uk", "Хеншела"), ("ur", "صوبہ خنشلہ"), ("vi", "Khenchela"), ("zh", "汉舍莱省")]),
                        unofficial_name_list: ["Khenchela", "Khenchla"].to_vec(),
                    }
                ),
                (
                    "41",
                    Subdivision{
                        name: "41",
                        country_alpha2: Alpha2::DZ,
                        code: "41",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.286389), longitude: Some(7.951111000000001), max_latitude: Some(36.3115633), min_latitude: Some(36.2441347), max_longitude: Some(8.0016517), min_longitude: Some(7.916808100000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية سوق أهراس"), ("be", "Правінцыя Сук-Ахрас"), ("bg", "Сук Ахрас"), ("bn", "স\u{9be}ৌক আহ\u{9cd}র\u{9be}স প\u{9cd}রদেশ"), ("ca", "Província de Souk Ahras"), ("ccp", "𑄥\u{1112f}𑄇\u{11134} 𑄃𑄦\u{11133}𑄢𑄌\u{11134}"), ("ceb", "Wilaya de Souk Ahras"), ("da", "Souk Ahras Province"), ("de", "Souk Ahras"), ("el", "Σουκ Αχράς"), ("en", "Souk Ahras"), ("es", "Provincia de Souk Ahras"), ("fa", "استان سوق اهراس"), ("fi", "Souk Ahrasin provinssi"), ("fr", "wilaya de Souk Ahras"), ("gl", "Provincia de Souk Ahras"), ("gu", "સૌક એહરસ પ\u{acd}રા\u{a82}ત"), ("hi", "स\u{942}क अह\u{94d}रस प\u{94d}रा\u{902}त"), ("id", "Provinsi Souk Ahras"), ("it", "provincia di Souk Ahras"), ("ja", "スーク・アフラース県"), ("ka", "სუკ-აჰრასის პროვინცია"), ("kn", "ಸ\u{ccc}ಕ\u{ccd} ಅಹ\u{ccd}ರಾಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "수크아라스 주"), ("lt", "Suk Ahraso vilaja"), ("lv", "Sūk Ehrāsas vilāja"), ("mr", "सौक\u{947} अह\u{94d}रस प\u{94d}रा\u{902}त"), ("ms", "Wilayah Souk Ahras"), ("nb", "Souk Ahras"), ("nl", "Souk Ahras"), ("no", "Souk Ahras"), ("pl", "Prowincja Suk Ahras"), ("pt", "Souk Ahras"), ("ro", "Provincia Souk Ahras"), ("ru", "Сук-Ахрас"), ("si", "ව\u{dd2}ලය\u{dcf}හ\u{dca} සොඋක\u{dca} අහ\u{dca}ර\u{dcf}ස\u{dca}"), ("sr", "Сук Ахрас"), ("sr_Latn", "Suk Ahras"), ("sv", "Souk Ahras"), ("sw", "Jimbo ya Souk Ahras"), ("ta", "சௌக\u{bcd} அஹ\u{bcd}ரஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c4c}క\u{c4d} ఆహ\u{c4d}ర\u{c3e}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e39}การ\u{e31}ส"), ("tr", "Sevk Ahras Vilayeti"), ("uk", "Сук-Ахрас"), ("ur", "صوبہ سوق اھراس"), ("vi", "Souk Ahras"), ("zh", "苏格艾赫拉斯省")]),
                        unofficial_name_list: ["Souk Ahras", "Souq Ahras"].to_vec(),
                    }
                ),
                (
                    "42",
                    Subdivision{
                        name: "42",
                        country_alpha2: Alpha2::DZ,
                        code: "42",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.6178786), longitude: Some(2.3912362), max_latitude: Some(36.6454234), min_latitude: Some(36.5477421), max_longitude: Some(2.5180149), min_longitude: Some(2.3005308)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية تيبازة"), ("az", "Tipaza vilayəti"), ("be", "Правінцыя Тыпаса"), ("bg", "Типаза"), ("bn", "টিপ\u{9be}স\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Tipaza"), ("ccp", "𑄑\u{11128}𑄛𑄥"), ("ceb", "Wilaya de Tipaza"), ("da", "Tipasa Province"), ("de", "Tipaza"), ("el", "Τιπάσα"), ("en", "Tipasa"), ("es", "Provincia de Tipasa"), ("eu", "Tipasa probintzia"), ("fa", "استان تی\u{200c}پازه"), ("fi", "Tipasan provinssi"), ("fr", "wilaya de Tipaza"), ("gl", "Provincia de Tipaza"), ("gu", "ટિપાસા પ\u{acd}રા\u{a82}ત"), ("hi", "टिपज\u{93c}ा प\u{94d}रा\u{902}त"), ("hu", "Tipáza"), ("id", "Provinsi Tipaza"), ("it", "provincia di Tipasa"), ("ja", "ティパザ県"), ("ka", "ტიპასის პროვინცია"), ("kn", "ಟ\u{cbf}ಪಾಸಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "티파자 주"), ("lt", "Tipazos vilaja"), ("lv", "Tibisas vilāja²"), ("mr", "टिपसा प\u{94d}रा\u{902}त"), ("ms", "Tipasa Province"), ("nb", "Tipaza"), ("nl", "Tipaza"), ("no", "Tipaza"), ("pl", "Prowincja Tibaza"), ("pt", "Tipasa"), ("ro", "Provincia Tipaza"), ("ru", "Типаса"), ("si", "ට\u{dd2}පස\u{dcf} පළ\u{dcf}ත"), ("sr", "Типаса"), ("sr_Latn", "Tipasa"), ("sv", "Tipaza"), ("sw", "Jimbo ya Tipasa"), ("ta", "டிப\u{bcd}ஸ\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ట\u{c3f}ప\u{c3e}స\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดท\u{e34}ปาซา"), ("tr", "Tibaze Vilayeti"), ("uk", "Тіпаза"), ("ur", "صوبہ تیبازہ"), ("vi", "Tipaza"), ("zh", "提帕萨省")]),
                        unofficial_name_list: ["Tipaza"].to_vec(),
                    }
                ),
                (
                    "43",
                    Subdivision{
                        name: "43",
                        country_alpha2: Alpha2::DZ,
                        code: "43",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.450278), longitude: Some(6.264444), max_latitude: Some(36.4780338), min_latitude: Some(36.4249427), max_longitude: Some(6.2886429), min_longitude: Some(6.208820299999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية ميلة"), ("be", "Правінцыя Міла"), ("bg", "Мила"), ("bn", "মিল\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Mila"), ("ccp", "𑄟\u{11128}𑄣"), ("ceb", "Wilaya de Mila"), ("da", "Mila Province"), ("de", "Mila"), ("el", "Μίλα"), ("en", "Mila"), ("es", "Provincia de Mila"), ("fa", "استان میله"), ("fi", "Milan provinssi"), ("fr", "wilaya de Mila"), ("gl", "Provincia de Mila"), ("gu", "મિલા પ\u{acd}રા\u{a82}ત"), ("he", "מילה, אלג׳יריה"), ("hi", "मिला प\u{94d}रा\u{902}त"), ("id", "Provinsi Mila"), ("it", "provincia di Mila"), ("ja", "ミラ県"), ("ka", "მილის პროვინცია"), ("kn", "ಮ\u{cbf}ಲಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "밀라 주"), ("lt", "Milos vilaja"), ("lv", "Mīlas vilāja"), ("mr", "मिल प\u{94d}रा\u{902}त"), ("ms", "Wilayah Mila"), ("nb", "Mila"), ("nl", "Mila"), ("no", "Mila"), ("pl", "Prowincja Mila"), ("pt", "Mila"), ("ro", "Provincia Mila"), ("ru", "Мила"), ("si", "ම\u{dd2}ල\u{dcf} පළ\u{dcf}ත"), ("sr", "Мила"), ("sr_Latn", "Mila"), ("sv", "Mila"), ("sw", "Jimbo ya Mila"), ("te", "మ\u{c3f}ల\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดม\u{e34}ลา"), ("tr", "Mila Vilayeti"), ("uk", "Міла"), ("ur", "صوبہ میلہ"), ("vi", "Mila"), ("zh", "米拉省")]),
                        unofficial_name_list: ["Mila"].to_vec(),
                    }
                ),
                (
                    "44",
                    Subdivision{
                        name: "44",
                        country_alpha2: Alpha2::DZ,
                        code: "44",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.0729193), longitude: Some(1.9881527), max_latitude: Some(36.422291), min_latitude: Some(35.834492), max_longitude: Some(2.7431009), min_longitude: Some(1.6114019)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية عين الدفلى"), ("az", "Ayn-Defla vilayəti"), ("be", "Правінцыя Айн-Дэфла"), ("bg", "Айн Дефла"), ("bn", "আইন ডেফ\u{9cd}ল\u{9be} প\u{9cd}রদেশ"), ("ca", "Província d’Ain Defla"), ("ccp", "𑄃\u{1112d}𑄚\u{11134} 𑄓𑄬𑄛\u{11134}𑄣"), ("ceb", "Wilaya de Aïn Defla"), ("da", "Aïn Defla"), ("de", "Ain Defla"), ("el", "Αϊν Ντέφλα"), ("en", "Aïn Defla"), ("es", "Provincia de Aïn Defla"), ("fa", "استان عین الدفلی"), ("fi", "Aïn Deflan provinssi"), ("fr", "wilaya de Aïn Defla"), ("gl", "Provincia de Aïn Defla"), ("gu", "એન ડ\u{ac7}ફ\u{acd}લા પ\u{acd}રા\u{a82}ત"), ("hi", "ऐन द\u{947}ल\u{94d}फ\u{94d}ल प\u{94d}रा\u{902}त"), ("id", "Provinsi Ain Defla"), ("ja", "アインデフラ県"), ("ka", "აინ-დეფლის პროვინცია"), ("kn", "ಏನ\u{ccd} ಡ\u{cc6}ಫ\u{ccd}ಲಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아인데플라 주"), ("lt", "Ain Deflos vilaja"), ("lv", "Deflas vilāja"), ("mr", "ऐन डिफला प\u{94d}रा\u{902}त"), ("ms", "Wilayah Aïn Defla"), ("nb", "Aïn Defla"), ("nl", "Aïn Defla"), ("no", "Aïn Defla"), ("pl", "Prowincja Ajn ad-Dafla"), ("pt", "Aïn Defla"), ("ro", "Provincia Aïn Defla"), ("ru", "Айн-Дефла"), ("si", "අය\u{dd2}න\u{dca} ඩෙෆ\u{dca}ල\u{dcf} පළ\u{dcf}ත"), ("sq", "Ain Defla"), ("sr", "Ајн Делфа"), ("sr_Latn", "Ajn Delfa"), ("sv", "Aïn Defla"), ("sw", "Wilaya ya Aïn Defla"), ("ta", "அய\u{bcd}ன\u{bcd} டேப\u{bcd}பில\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఎయ\u{c3f}న\u{c4d} డ\u{c46}ఫ\u{c4d}ల\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e34}น เดลฟา"), ("tr", "Ayn el Defla Vilayeti"), ("uk", "Айн-Дефля"), ("ur", "صوبہ عین الدفلی"), ("vi", "Aïn Defla"), ("zh", "艾因迪夫拉省")]),
                        unofficial_name_list: ["Aïn Defla", "Aïn Eddefla"].to_vec(),
                    }
                ),
                (
                    "45",
                    Subdivision{
                        name: "45",
                        country_alpha2: Alpha2::DZ,
                        code: "45",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.4350615), longitude: Some(-0.9056623), max_latitude: Some(34.2906), min_latitude: Some(32.108521), max_longitude: Some(0.134101), min_longitude: Some(-1.7384947)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية النعامة"), ("be", "Правінцыя Наама"), ("bg", "Наама"), ("bn", "ন\u{9be}ম\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Naama"), ("ccp", "𑄚𑄟"), ("ceb", "Wilaya de Naama"), ("da", "Naama Province"), ("de", "Naama"), ("el", "Ναάμα"), ("en", "Naama"), ("es", "Naama"), ("fa", "استان نعامه"), ("fi", "Naaman provinssi"), ("fr", "wilaya de Naâma"), ("gl", "Provincia de Naâma"), ("gu", "નામા પ\u{acd}રા\u{a82}ત"), ("hi", "नामा प\u{94d}रा\u{902}त"), ("id", "Provinsi Naama"), ("it", "provincia di Naâma"), ("ja", "ナーマ県"), ("ka", "ნაამის პროვინცია"), ("kn", "ನಾಮಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "나마 주"), ("lt", "Naamos vilaja"), ("lv", "Naāmas vilāja"), ("mr", "नामा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Naama"), ("nb", "Naama"), ("nl", "Naama"), ("no", "Naama"), ("pl", "Prowincja Naama"), ("pt", "Naâma"), ("ro", "Provincia Naâma"), ("ru", "Наама"), ("si", "න\u{dcf}ම\u{dcf} පළ\u{dcf}ත"), ("sr", "Нама"), ("sr_Latn", "Nama"), ("sv", "Naâma"), ("sw", "Jimbo ya Naama"), ("ta", "ந\u{bbe}ம\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "న\u{c3e}మ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดนาอามา"), ("tr", "Niame Vilayeti"), ("uk", "Наама"), ("ur", "صوبہ النعامہ"), ("vi", "Naâma"), ("zh", "纳马省")]),
                        unofficial_name_list: ["Naama"].to_vec(),
                    }
                ),
                (
                    "46",
                    Subdivision{
                        name: "46",
                        country_alpha2: Alpha2::DZ,
                        code: "46",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.3072501), longitude: Some(-1.1424514), max_latitude: Some(35.320658), min_latitude: Some(35.279241), max_longitude: Some(-1.1136531), min_longitude: Some(-1.1580491)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية عين تموشنت"), ("az", "Ayn-Temuşent vilayəti"), ("be", "Правінцыя Айн-Тэмушэнт"), ("bg", "Айн Темушент"), ("bn", "আইন টেম\u{9c1}চেন\u{9cd}ট প\u{9cd}রদেশ"), ("ca", "Província d’Aïn Témouchent"), ("ccp", "𑄃\u{1112d}𑄚\u{11134} 𑄑𑄬𑄟\u{1112f}𑄥𑄬𑄚\u{11134}𑄑\u{11134}"), ("ceb", "Wilaya de Aïn Temouchent"), ("da", "Aïn Témouchent"), ("de", "Ain Temouchent"), ("el", "Αΐν Τεμουχέντ"), ("en", "Aïn Témouchent"), ("es", "Aïn Témouchent"), ("fa", "استان عین تموشنت"), ("fi", "Aïn Témouchentin provinssi"), ("fr", "wilaya d’Aïn Témouchent"), ("gl", "Provincia de Aïn Témouchent"), ("gu", "આઇન ટ\u{ac7}મોઉચ\u{ac7}ન\u{acd}ટ પ\u{acd}રા\u{a82}ત"), ("hi", "ऐन ट\u{947}मॉछ\u{947}न\u{94d}त प\u{94d}रा\u{902}त"), ("id", "Provinsi Ain Temouchent"), ("it", "provincia di Aïn Témouchent"), ("ja", "アイン・ティムシェント県"), ("ka", "აინ-ტემუშენტის პროვინცია"), ("kn", "ಐನ\u{ccd} ಟ\u{cc6}ಮ\u{ccc}ಚ\u{cc6}ಂಟ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아인테무셴트 주"), ("lt", "Ain Temušento vilaja"), ("lv", "Ain Tīmūšentas vilāja"), ("mr", "ऐन ट\u{947}मौच\u{947}\u{902}न\u{94d}ट प\u{94d}रा\u{902}त"), ("ms", "Wilayah Aïn Témouchent"), ("nb", "Aïn Témouchent"), ("nl", "Aïn Témouchent"), ("no", "Aïn Témouchent"), ("pl", "Prowincja Ajn Tumuszanat"), ("pt", "Aïn Témouchent"), ("ro", "Provincia Aïn Témouchent"), ("ru", "Айн-Темушент"), ("si", "අය\u{dd2}න\u{dca} ටෙමොඋචෙන\u{dca}ට\u{dca} පළ\u{dcf}ත"), ("sr", "Ајн Темушент"), ("sr_Latn", "Ajn Temušent"), ("sv", "Aïn Témouchent"), ("sw", "Jimbo ya Aïn Témouchent"), ("ta", "ஆயின\u{bcd} டெமெசேண\u{bcd}ட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఎయ\u{c3f}న\u{c4d} ట\u{c46}మ\u{c4b}చ\u{c46}ంట\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเอนเอม\u{e39}เช\u{e35}ยน"), ("tr", "Ayn Temuşent Vilayeti"), ("uk", "Айн-Темушент"), ("ur", "صوبہ عین تموشنت"), ("vi", "Aïn Témouchent"), ("zh", "艾因泰穆尚特省")]),
                        unofficial_name_list: ["Aïn Temouchent"].to_vec(),
                    }
                ),
                (
                    "47",
                    Subdivision{
                        name: "47",
                        country_alpha2: Alpha2::DZ,
                        code: "47",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.4902246), longitude: Some(3.6738412), max_latitude: Some(32.7046889), min_latitude: Some(32.4574303), max_longitude: Some(3.916626), min_longitude: Some(3.5745049)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية غرداية"), ("be", "Правінцыя Гардая"), ("bg", "Гардая"), ("bn", "ঘর\u{9cd}দ\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Ghardaïa"), ("ccp", "𑄊𑄢\u{11134}𑄓\u{1112d}𑄠"), ("ceb", "Wilaya de Ghardaïa"), ("cs", "Ghardája"), ("da", "Ghardaïa Province"), ("de", "Ghardaia"), ("el", "Γαρντάια"), ("en", "Ghardaïa"), ("es", "Provincia de Ghardaïa"), ("fa", "استان غردایه"), ("fi", "Ghardaïan maakunta"), ("fr", "wilaya de Ghardaïa"), ("gl", "Provincia de Ghardaïa"), ("gu", "ઘારદ\u{ac8}યા પ\u{acd}રા\u{a82}ત"), ("hi", "घर\u{94d}ड\u{948} प\u{94d}रा\u{902}त"), ("id", "Provinsi Ghardaia"), ("it", "provincia di Ghardaïa"), ("ja", "ガルダイア県"), ("ka", "გარდაიის პროვინცია"), ("kn", "ಘಾರ\u{ccd}ಡ\u{cbf}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("lt", "Gardajos vilaja"), ("lv", "Gardājas vilāja"), ("mr", "घारदाई प\u{94d}रा\u{902}त"), ("ms", "Wilayah Ghardaïa"), ("nb", "Ghardaïa"), ("nl", "Ghardaïa"), ("no", "Ghardaïa"), ("pl", "Prowincja Ghardaja"), ("pt", "Ghardaia"), ("ro", "Provincia Ghardaïa"), ("ru", "Гардая"), ("si", "ගර\u{dca}දය\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sr", "Гардаја"), ("sr_Latn", "Gardaja"), ("sv", "Ghardaïa"), ("ta", "க\u{bbe}ர\u{bcd}டையே ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గ\u{c3e}ర\u{c4d}డ\u{c3e}య\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดการ\u{e4c}ดาเอ\u{e35}ย"), ("tr", "Gardaye Vilayeti"), ("uk", "Гардая"), ("ur", "صوبہ غرادیہ"), ("vi", "Ghardaïa"), ("zh", "盖尔达耶省")]),
                        unofficial_name_list: ["Ghardaïa"].to_vec(),
                    }
                ),
                (
                    "48",
                    Subdivision{
                        name: "48",
                        country_alpha2: Alpha2::DZ,
                        code: "48",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.733333), longitude: Some(0.5499999999999999), max_latitude: Some(35.7633683), min_latitude: Some(35.6958176), max_longitude: Some(0.5892706000000001), min_longitude: Some(0.5295754)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية غليزان"), ("be", "Правінцыя Рэлізан"), ("bg", "Релизан"), ("bn", "রেলিজ\u{9be}ন প\u{9cd}রদেশ"), ("ca", "Província de Relizane"), ("ccp", "𑄢𑄬𑄣\u{11128}𑄎𑄚𑄬"), ("ceb", "Wilaya de Relizane"), ("da", "Relizane Province"), ("de", "Relizane"), ("el", "Ρελιζάν"), ("en", "Relizane"), ("es", "Provincia de Relizan"), ("fa", "استان غلیزان"), ("fi", "Relizanen lääni"), ("fr", "wilaya de Relizane"), ("gl", "Provincia de Relizane"), ("gu", "રિલાઈજ\u{ac7}ન પ\u{acd}રા\u{a82}ત"), ("hi", "र\u{947}लिज\u{93c}\u{947}न प\u{94d}रा\u{902}त"), ("id", "Provinsi Relizane"), ("it", "provincia di Relizane"), ("ja", "ルリザンヌ県"), ("ka", "რელიზანის პროვინცია"), ("kn", "ರ\u{cbf}ಲ\u{cbf}ಜ\u{cc6}ನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "렐리잔 주"), ("lt", "Relizano vilaja"), ("lv", "Relīzanas vilāja"), ("mr", "र\u{945}लिझन\u{947} प\u{94d}रा\u{902}त"), ("ms", "Relizane Province"), ("nb", "Relizane"), ("nl", "Relizane"), ("no", "Relizane"), ("pl", "Prowincja Ghulajzan"), ("pt", "Relizane"), ("ro", "Provincia Relizane"), ("ru", "Релизан"), ("si", "රෙල\u{dd2}සනේ පළ\u{dcf}ත"), ("sr", "Релизан"), ("sr_Latn", "Relizan"), ("sv", "Relizane"), ("sw", "Jimbo ya Relizane"), ("ta", "ரில\u{bc0}ச\u{bbe}னே ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ర\u{c3f}\u{c46}ల\u{c3f}జ\u{c47}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เม\u{e37}องเรร\u{e34}เซน"), ("tr", "Relizane Vilayeti"), ("uk", "Релізан"), ("ur", "صوبہ غلیزان"), ("vi", "Relizane"), ("zh", "埃利赞省")]),
                        unofficial_name_list: ["Ghilizane", "Ighil Izane", "Relizane", "Rélizane"].to_vec(),
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
#[cfg(feature = "dz")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::DZ,
        alpha3: Alpha3::DZA,
        address_format: None,
        continent: Continent::Africa,
        country_code: 213,
        currency_code: CurrencyCode::DZD,
        gec: Some(GEC::AG),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::ALG),
        iso_long_name: "The People's Democratic Republic of Algeria",
        iso_short_name: "Algeria",
        official_language_list: ["ar"].to_vec(),
        spoken_language_list: ["ar"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "7",
        nationality: Some("Algerian"),
        number: "012",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Africa),
        start_of_week: WeekDay::Sunday,
        subregion: Some(SubRegion::NorthernAfrica),
        un_locode: "DZ",
        unofficial_name_list: [
            "Algeria",
            "الجزائر",
            "Algerien",
            "Algérie",
            "Argelia",
            "アルジェリア",
            "Algerije",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Algeria"),
            ("af", "Algerië"),
            ("ak", "Algeria"),
            ("am", "ጐሔጄሱ።"),
            ("an", "Algeria"),
            ("ar", "الجزائر"),
            ("as", "আলজেৰিয\u{9be}"),
            ("ay", "Algeria"),
            ("az", "Əlcəzair"),
            ("ba", "Algeria"),
            ("be", "Алжыр"),
            ("bg", "Алжир"),
            ("bi", "Algeria"),
            ("bn", "অ\u{9cd}য\u{9be}লজিরিয়\u{9be}"),
            ("bn_IN", "অ\u{9cd}য\u{9be}লজিরিয়\u{9be}"),
            ("br", "Aljeria"),
            ("bs", "Alžir"),
            ("ca", "Algèria"),
            ("ce", "Алжир"),
            ("ch", "Algeria"),
            ("cs", "Alžírsko"),
            ("cv", "Алжир"),
            ("cy", "Algeria"),
            ("da", "Algeriet"),
            ("de", "Algerien"),
            ("dv", "ޖ\u{7a6}ޒ\u{7a7}އ\u{7a8}ރ\u{7aa}"),
            ("dz", "ཨ\u{f71}ལ་ཇ\u{f72}་ར\u{f72}་ཡ།"),
            ("ee", "Algeria"),
            ("el", "Αλγερία"),
            ("en", "Algeria"),
            ("eo", "Alĝerio"),
            ("es", "Algeria"),
            ("et", "Alžeeria"),
            ("eu", "Aljeria"),
            ("fa", "الجزایر"),
            ("ff", "Aljeri"),
            ("fi", "Algeria"),
            ("fo", "Algeria"),
            ("fr", "Algérie"),
            ("fy", "Algerije"),
            ("ga", "An Ailgéir"),
            ("gl", "Alxeria"),
            ("gn", "Algeria"),
            ("gu", "અલ\u{acd}જ\u{ac7}રિયા"),
            ("gv", "Yn Algear"),
            ("ha", "Aljeriya"),
            ("he", "אלג'יריה"),
            ("hi", "अल\u{94d}जीरिया"),
            ("hr", "Alžir"),
            ("ht", "Aljeri"),
            ("hu", "Algéria"),
            ("hy", "Ալժիր"),
            ("ia", "Algeria"),
            ("id", "Aljazair"),
            ("io", "Aljeria"),
            ("is", "Alsír"),
            ("it", "Algeria"),
            ("iu", "Algeria"),
            ("ja", "アルジェリア"),
            ("ka", "ალჟირი"),
            ("ki", "Algeria"),
            ("kk", "Алжир"),
            ("kl", "Algeria"),
            ("km", "អាល\u{17cb}ហ\u{17d2}សេរ\u{17b8}"),
            ("kn", "ಅಲ\u{ccd}ಜೀರ\u{cbf}ಯ"),
            ("ko", "알제리"),
            ("ku", "Cezayîr"),
            ("kv", "Algeria"),
            ("kw", "Aljeri"),
            ("ky", "Алжир"),
            ("lo", "Algeria"),
            ("lt", "Alžyras"),
            ("lv", "Alžīrija"),
            ("mi", "Algeria"),
            ("mk", "Алжир"),
            ("ml", "അള\u{d4d}\u{200d}ജീരിയ"),
            ("mn", "Алжир"),
            ("mr", "अल\u{94d}जिरिया"),
            ("ms", "Algeria"),
            ("mt", "Alġerija"),
            (
                "my",
                "အယ\u{103a}လ\u{103a}ဂျ\u{102e}းရ\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Ardjiriya"),
            ("nb", "Algerie"),
            ("ne", "अल\u{94d}ज\u{947}रिया"),
            ("nl", "Algerije"),
            ("nn", "Algerie"),
            (
                "nv",
                "Ghą\u{301}ą\u{301}ʼaskʼidii Biłikahii Bikéyah Ntsaaígíí",
            ),
            ("oc", "Argeria"),
            ("or", "ଆଲଜେର\u{b3f}ୟ\u{b3e}"),
            ("pa", "ਅਲਜੀਰੀਆ"),
            ("pi", "अल\u{94d}जीरिया"),
            ("pl", "Algieria"),
            ("ps", "الجزایر"),
            ("pt", "Argélia"),
            ("pt_BR", "Argélia"),
            ("ro", "Algeria"),
            ("ru", "Алжир"),
            ("rw", "Aligeriya"),
            ("sc", "Algeria"),
            ("sd", "الجزائر"),
            ("si", "ඇල\u{dca}ජ\u{dd3}ර\u{dd2}ය\u{dcf}"),
            ("sk", "Alžírsko"),
            ("sl", "Alžirija"),
            ("so", "Aljeeriya"),
            ("sq", "Algjeri"),
            ("sr", "Алжир"),
            ("sv", "Algeriet"),
            ("sw", "Aljeria"),
            ("ta", "அல\u{bcd}ஜ\u{bc0}ரிய\u{bbe}"),
            ("te", "అల\u{c4d}జ\u{c3f}ర\u{c3f}య\u{c3e}"),
            ("tg", "Алҷазоир"),
            ("th", "แอลจ\u{e35}เร\u{e35}ย"),
            ("ti", "አልጄሪያ"),
            ("tk", "Algerýa"),
            ("tl", "Alheriya"),
            ("tr", "Cezayir"),
            ("tt", "Алжыр"),
            ("ug", "ئالجىرىيە"),
            ("uk", "Алжир"),
            ("ur", "الجزائر"),
            ("uz", "Jazoir"),
            ("ve", "Algeria"),
            ("vi", "An-giê-ri"),
            ("wa", "Aldjereye"),
            ("wo", "Aljeeri"),
            ("xh", "Algeria"),
            ("yo", "Àlgéríà"),
            ("zh_CN", "阿尔及利亚"),
            ("zh_HK", "阿爾及利亞"),
            ("zh_TW", "阿爾及利亞"),
            ("zu", "IAljiriya"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
