// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Argentine Republic

#[cfg(all(feature = "ar", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{region}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::AR;
    pub const ALPHA3: Alpha3 = Alpha3::ARG;
    pub const CONTINENT: Continent = Continent::SouthAmerica;
    pub const COUNTRY_CODE: usize = 54;
    pub const CURRENCY_CODE: &str = "ARS";
    pub const GEC: Option<GEC> = Some(GEC::AR);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("ARG");
    pub const ISO_SHORT_NAME: &str = "Argentina";
    pub const ISO_LONG_NAME: &str = "The Argentine Republic";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["es", "gn"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["es", "gn"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Argentinean");
    pub const NUMBER: &str = "032";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("((?:[A-HJ-NP-Z])?\\d{4})([A-Z]{3})?");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthAmerica);
    pub const UN_LOCODE: &str = "AR";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Argentina",
        "Argentinien",
        "Argentine",
        "アルゼンチン",
        "Argentinië",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Argentina"),
        ("af", "Argentinië"),
        ("ak", "Argentina"),
        ("am", "ጐሴጀንቲና"),
        ("an", "Archentina"),
        ("ar", "الأرجنتين"),
        ("as", "আৰ\u{9cd}জেনটিন\u{9be}"),
        ("ay", "Argentina"),
        ("az", "Argentina"),
        ("ba", "Argentina"),
        ("be", "Аргенціна"),
        ("bg", "Аржентина"),
        ("bi", "Argentina"),
        ("bn", "আর\u{9cd}জেনটিন\u{9be}"),
        ("bn_IN", "আর\u{9cd}জেনটিন\u{9be}"),
        ("br", "Arc'hantina"),
        ("bs", "Argentina"),
        ("ca", "Argentina"),
        ("ce", "Аргентина"),
        ("ch", "Argentina"),
        ("cs", "Argentina"),
        ("cv", "Аргентина"),
        ("cy", "Yr Ariannin"),
        ("da", "Argentina"),
        ("de", "Argentinien"),
        ("dv", "އ\u{7a7}ޖ\u{7ac}ނ\u{7b0}ޓ\u{7a9}ނ\u{7a7}"),
        ("dz", "ཨར་ཇ\u{f7a}ན་ཊ\u{f72}་ན།"),
        ("ee", "Argentina"),
        ("el", "Αργεντινή"),
        ("en", "Argentina"),
        ("eo", "Argentino"),
        ("es", "Argentina"),
        ("et", "Argentina"),
        ("eu", "Argentina"),
        ("fa", "آرژانتین"),
        ("ff", "Argentina"),
        ("fi", "Argentiina"),
        ("fo", "Argentina"),
        ("fr", "Argentine"),
        ("fy", "Argentynje"),
        ("ga", "An Airgintín"),
        ("gl", "Arxentina"),
        ("gn", "Argentina"),
        ("gu", "આર\u{acd}જ\u{ac7}ન\u{acd}ટિના"),
        ("gv", "Yn Argenteen"),
        ("ha", "Argentina"),
        ("he", "ארגנטינה"),
        ("hi", "अर\u{94d}ज\u{947}ण\u{94d}टीना"),
        ("hr", "Argentina"),
        ("ht", "Ajantin"),
        ("hu", "Argentína"),
        ("hy", "Արգենտինա"),
        ("ia", "Argentina"),
        ("id", "Argentina"),
        ("io", "Arjentinia"),
        ("is", "Argentína"),
        ("it", "Argentina"),
        ("iu", "Argentina"),
        ("ja", "アルゼンチン"),
        ("ka", "არგენტინა"),
        ("ki", "Argentina"),
        ("kk", "Аргентина"),
        ("kl", "Argentina"),
        ("km", "អាហ\u{17d2}សង\u{17cb}ទ\u{17b8}ន"),
        ("kn", "ಅರ\u{ccd}ಜ\u{cc6}ಂಟೈನಾ"),
        ("ko", "아르헨티나"),
        ("ku", "Arjantîn"),
        ("kv", "Аргентина"),
        ("kw", "Arghantina"),
        ("ky", "Аргентина"),
        ("lo", "ອາກຊ\u{eb1}ງຕ\u{eb5}ນ"),
        ("lt", "Argentina"),
        ("lv", "Argentīna"),
        ("mi", "Āketina"),
        ("mk", "Аргентина"),
        ("ml", "അര\u{d4d}\u{200d}ജന\u{d4d}റീന"),
        ("mn", "Аргентин"),
        ("mr", "अर\u{94d}ज\u{947}\u{902}र\u{94d}टिना"),
        ("ms", "Argentina"),
        ("mt", "Arġentina"),
        (
            "my",
            "အာဂျင\u{103a}တ\u{102e}းနားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Ardjentina"),
        ("nb", "Argentina"),
        ("ne", "अरज\u{947}नटिना"),
        ("nl", "Argentinië"),
        ("nn", "Argentina"),
        ("nv", "Argentina"),
        ("oc", "Argentina"),
        ("or", "ଅ\u{b3e}ର\u{b4d}ଜେଣ\u{b4d}ଟୀନ\u{b3e}"),
        ("pa", "ਅਰਜਨਟੀਨਾ"),
        ("pi", "अर\u{94d}जन\u{94d}टीना"),
        ("pl", "Argentyna"),
        ("ps", "ارجنټاین"),
        ("pt", "Argentina"),
        ("pt_BR", "Argentina"),
        ("ro", "Argentina"),
        ("ru", "Аргентина"),
        ("rw", "Arijantina"),
        ("sc", "Argentina"),
        ("sd", "Argentina"),
        ("si", "ආජන\u{dca}ට\u{dd2}න\u{dcf}ව"),
        ("sk", "Argentína"),
        ("sl", "Argentina"),
        ("so", "Arjantiina"),
        ("sq", "Argjentinë"),
        ("sr", "Аргентина"),
        ("sv", "Argentina"),
        ("sw", "Argentina"),
        ("ta", "அர\u{bcd}ஜென\u{bcd}டின\u{bbe}"),
        ("te", "అర\u{c4d}జ\u{c47}ంర\u{c4d}ట\u{c3f}న\u{c3e}"),
        ("tg", "Аргентина"),
        ("th", "อาร\u{e4c}เจนต\u{e34}นา"),
        ("ti", "ኣርጀንቲና"),
        ("tk", "Argentina"),
        ("tl", "Arhentina"),
        ("tr", "Arjantin"),
        ("tt", "Арgентина"),
        ("ug", "ئارگېنتىنا"),
        ("uk", "Аргентина"),
        ("ur", "ارجنٹائن"),
        ("uz", "Argentina"),
        ("ve", "Argentina"),
        ("vi", "Á-căn-đình"),
        ("wa", "Årdjintene"),
        ("wo", "Argentiin"),
        ("xh", "Argentina"),
        ("yo", "Argẹntínà"),
        ("zh_CN", "阿根廷"),
        ("zh_HK", "阿根廷"),
        ("zh_TW", "阿根廷"),
        ("zu", "I-Argentina"),
    ];
    #[cfg(all(feature = "ar", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -38.416097;
        pub const LONGITUDE: f64 = -63.61667199999999;
        pub const MAX_LATITUDE: f64 = -21.7810459;
        pub const MAX_LONGITUDE: f64 = -53.637481;
        pub const MIN_LATITUDE: f64 = -55.1250224;
        pub const MIN_LONGITUDE: f64 = -73.5603601;
        pub const NORTHEAST_LATITUDE: f64 = -21.7810459;
        pub const NORTHEAST_LONGITUDE: f64 = -53.637481;
        pub const SOUTHWEST_LATITUDE: f64 = -55.1250224;
        pub const SOUTHWEST_LONGITUDE: f64 = -73.5603601;
    }
}
#[cfg(all(feature = "ar", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -38.416097,
            longitude: -63.61667199999999,
            max_latitude: -21.7810459,
            max_longitude: -53.637481,
            min_latitude: -55.1250224,
            min_longitude: -73.5603601,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: -21.7810459,
                    longitude: -53.637481,
                },
                southwest: CountryGeoBound {
                    latitude: -55.1250224,
                    longitude: -73.5603601,
                },
            },
        }
    }
}

#[cfg(all(feature = "ar", feature = "subdivisions"))]
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
                    "A",
                    Subdivision{
                        name: "A",
                        country_alpha2: Alpha2::AR,
                        code: "A",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-24.7829323), longitude: Some(-65.4121552), max_latitude: Some(-24.7101875), min_latitude: Some(-24.8724718), max_longitude: Some(-65.35461169999999), min_longitude: Some(-65.4991155)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سالتا"), ("be", "Правінцыя Сальта"), ("bg", "Салта"), ("bn", "স\u{9be}লত\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Salta"), ("ccp", "𑄥𑄣\u{11134}𑄑"), ("ceb", "Provincia de Salta"), ("cs", "Salta"), ("da", "Provincia de Salta"), ("de", "Provinz Salta"), ("el", "Σάλτα"), ("en", "Salta"), ("es", "Provincia de Salta"), ("et", "Salta provints"), ("eu", "Saltako probintzia"), ("fa", "ایالت سالتا"), ("fi", "Saltan maakunta"), ("fr", "province de Salta"), ("gl", "Provincia de Salta"), ("gu", "સાલ\u{acd}ટા પ\u{acd}રા\u{a82}ત"), ("he", "סלטה"), ("hi", "साल\u{94d}टा"), ("hy", "Սալտա"), ("id", "Provinsi Salta"), ("it", "provincia di Salta"), ("ja", "サルタ州"), ("ka", "სალტის პროვინცია"), ("kn", "ಸಾಲ\u{ccd}ಟಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "살타 주"), ("lt", "Saltos provincija"), ("lv", "Saltas province"), ("mk", "Салта"), ("mr", "साल\u{94d}ता"), ("ms", "Wilayah Salta"), ("nb", "Salta"), ("nl", "Salta"), ("no", "Salta"), ("pl", "Salta"), ("pt", "Salta"), ("ro", "Provincia Salta"), ("ru", "Сальта"), ("si", "සැල\u{dca}ට\u{dcf} පළ\u{dcf}ත"), ("sk", "Salta"), ("sr", "Салта"), ("sr_Latn", "Salta"), ("sv", "Salta"), ("sw", "Mkoa wa Salta"), ("ta", "ச\u{bbe}ல\u{bcd}ட\u{bcd}ட\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3e}ల\u{c4d}జ\u{c4d}\u{200c}ట\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐซ\u{e31}ลตา"), ("tr", "Salta eyaleti"), ("uk", "Сальта"), ("ur", "صوبہ سالتا"), ("vi", "Salta"), ("zh", "萨尔塔省")]),
                        unofficial_name_list: ["Salta"].to_vec(),
                    }
                ),
                (
                    "B",
                    Subdivision{
                        name: "B",
                        country_alpha2: Alpha2::AR,
                        code: "B",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-34.6037232), longitude: Some(-58.3815931), max_latitude: Some(-34.5265464), min_latitude: Some(-34.7051589), max_longitude: Some(-58.33518840000001), min_longitude: Some(-58.5314522)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Buenos Aires"), ("ar", "بوينس آيرس"), ("be", "правінцыя Буэнас-Айрэс"), ("bg", "Буенос Айрес"), ("bn", "ব\u{9c1}য\u{9bc}েনোস আয\u{9bc}\u{9be}রস প\u{9cd}রদেশ"), ("ca", "Província de Buenos Aires"), ("ccp", "𑄝\u{11128}𑄅\u{1112a}𑄚\u{11127}𑄌\u{11134} 𑄃𑄬𑄠𑄢\u{11134}𑄥\u{11134} 𑄛\u{11133}𑄢\u{11127}\u{1112e}𑄞\u{11128}𑄚\u{11134}𑄥\u{11134}"), ("ceb", "Provincia de Buenos Aires"), ("cs", "Buenos Aires"), ("cy", "Talaith Buenos Aires"), ("da", "Provincia de Buenos Aires"), ("de", "Provinz Buenos Aires"), ("el", "Μπουένος Άιρες"), ("en", "Buenos Aires Province"), ("es", "provincia de Buenos Aires"), ("et", "Buenos Airese provints"), ("eu", "Buenos Airesko probintzia"), ("fa", "ایالت بوئنوس آیرس"), ("fi", "Buenos Airesin maakunta"), ("fr", "province de Buenos Aires"), ("gl", "Provincia de Bos Aires"), ("gu", "બ\u{acd}ય\u{ac1}નોસ એર\u{ac7}સ પ\u{acd}રા\u{a82}ત"), ("he", "בואנוס איירס"), ("hi", "ब\u{94d}य\u{942}नर\u{94d}स आयर\u{94d}स (प\u{94d}रान\u{94d}त)"), ("hr", "Buenos Aires"), ("hu", "Buenos Aires tartomány"), ("hy", "Բուենոս Այրես"), ("id", "Provinsi Buenos Aires"), ("it", "provincia di Buenos Aires"), ("ja", "ブエノスアイレス州"), ("ka", "ბუენოს-აირესის პროვინცია"), ("kn", "ಬ\u{ccd}ಯ\u{cc2}ನಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "부에노스아이레스 주"), ("lt", "Buenos Airės"), ("lv", "Buenosairesas province"), ("mk", "Буенос Аирес"), ("mn", "Буэнос-Айрес муж"), ("mr", "ब\u{941}एनोस आइर\u{947}स प\u{94d}रा\u{902}त"), ("ms", "Provinsi Buenos Aires"), ("nb", "Buenos Aires"), ("nl", "Buenos Aires"), ("no", "Buenos Aires"), ("pl", "Buenos Aires"), ("pt", "Buenos Aires"), ("ro", "Provincia Buenos Aires"), ("ru", "Буэнос-Айрес"), ("si", "බ\u{dd4}වනෝස\u{dca} අයර\u{dca}ස\u{dca} පළ\u{dcf}ත"), ("sk", "Buenos Aires"), ("sr", "Буенос Ајрес"), ("sr_Latn", "Buenos Ajres"), ("sv", "Buenos Aires"), ("sw", "Mkoa wa Buenos Aires"), ("ta", "பஏனோஸ\u{bcd} ஐரிஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బుయ\u{c46}న\u{c4b}స\u{c4d} ఎయ\u{c3f}ర\u{c4d}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐบ\u{e31}วโนสไอเรส"), ("tr", "Buenos Aires eyaleti"), ("uk", "Буенос-Айрес"), ("ur", "صوبہ بیونس آئرس"), ("vi", "Buenos Aires"), ("yue", "布宜諾斯艾利斯省"), ("yue_Hans", "布宜诺斯艾利斯省"), ("zh", "布宜诺斯艾利斯省")]),
                        unofficial_name_list: ["Buenos Aires"].to_vec(),
                    }
                ),
                (
                    "C",
                    Subdivision{
                        name: "C",
                        country_alpha2: Alpha2::AR,
                        code: "C",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-34.6037232), longitude: Some(-58.3815931), max_latitude: Some(-34.5265464), min_latitude: Some(-34.7051589), max_longitude: Some(-58.33518840000001), min_longitude: Some(-58.5314522)}),
                        comments: None,
                        subdivision_type: SubdivisionType::City,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Buenos Aires²"), ("am", "ብዌኖስ አይሬስ"), ("ar", "بوينس آيرس²"), ("az", "Buenos Ayres"), ("be", "Буэнас-Айрэс"), ("bg", "Буенос Айрес²"), ("bn", "ব\u{9c1}য\u{9bc}েনোস আইরেস"), ("bs", "Buenos Aires"), ("ca", "Buenos Aires"), ("ccp", "𑄝\u{11128}𑄅\u{1112a}𑄚\u{11127}𑄌\u{11134} 𑄃𑄬𑄠𑄢\u{11134}𑄥\u{11134}"), ("ceb", "Buenos Aires"), ("cs", "Buenos Aires²"), ("cy", "Buenos Aires"), ("da", "Buenos Aires"), ("de", "Buenos Aires"), ("el", "Μπουένος Άιρες²"), ("en", "Buenos Aires"), ("es", "Buenos Aires"), ("et", "Buenos Aires"), ("eu", "Buenos Aires"), ("fa", "بوئنوس آیرس"), ("fi", "Buenos Aires"), ("fr", "Buenos Aires"), ("ga", "Buenos Aires"), ("gl", "Buenos Aires"), ("gu", "બ\u{acd}ય\u{ac1}નોસ એર\u{ac7}સ"), ("ha", "Buenos Aires"), ("ha_NE", "Buenos Aires"), ("he", "בואנוס איירס²"), ("hi", "ब\u{94d}य\u{942}नस आयर\u{94d}स"), ("hr", "Buenos Aires²"), ("hu", "Buenos Aires"), ("hy", "Բուենոս Այրես²"), ("id", "Buenos Aires"), ("is", "Búenos Aíres"), ("it", "Buenos Aires"), ("ja", "ブエノスアイレス"), ("jv", "Buenos Aires"), ("ka", "ბუენოს-აირესი"), ("kk", "Буэнос-Айрес"), ("kn", "ಬ\u{ccd}ಯ\u{cc2}ನಸ\u{ccd} ಐರ\u{cbf}ಸ\u{ccd}"), ("ko", "부에노스아이레스"), ("ky", "Буэнос-Айрес"), ("lt", "Buenos Airės²"), ("lv", "Buenosairesa"), ("mk", "Буенос Аирес²"), ("ml", "ബ\u{d4d}യ\u{d42}ണസ\u{d4d} അയേഴ\u{d4d}സ\u{d4d}"), ("mn", "Буэнос-Айрес"), ("mr", "ब\u{941}एनोस आइर\u{947}स"), ("ms", "Buenos Aires"), ("my", "ဗျ\u{1030}န\u{102d}\u{102f}အေးရ\u{102d}စ\u{103a}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Buenos Aires²"), ("ne", "ब\u{94d}य\u{942}नस आयर\u{94d}स"), ("nl", "Buenos Aires²"), ("no", "Buenos Aires²"), ("or", "ବ\u{b41}ଏନ\u{b4d}ସ ଏଆରସ"), ("pa", "ਬ\u{a41}ਏਨਸ ਆਇਰਸ"), ("pl", "Buenos Aires²"), ("ps", "بوئنوس آيرز"), ("pt", "Buenos Aires²"), ("ro", "Buenos Aires"), ("ru", "Буэнос-Айрес²"), ("si", "බ\u{dd4}වනොස\u{dca} අය\u{dd2}ර\u{dd3}ස\u{dca}"), ("sk", "Buenos Aires²"), ("sl", "Buenos Aires"), ("so", "Buenos Aires"), ("sq", "Buenos Aires"), ("sr", "Буенос Ајрес²"), ("sr_Latn", "Buenos Ajres²"), ("sv", "Buenos Aires²"), ("sw", "Buenos Aires"), ("ta", "புவெனஸ\u{bcd} ஐரிஸ\u{bcd}"), ("te", "బ\u{c4d}యూనస\u{c4d} ఏర\u{c40}స\u{c4d}"), ("th", "บ\u{e31}วโนสไอเรส"), ("tk", "Buenos-Aýres"), ("tr", "Buenos Aires"), ("uk", "Буенос-Айрес²"), ("ur", "بیونس آئرس"), ("uz", "Buenos Ayres"), ("vi", "Buenos Aires²"), ("yo", "Buenos Aires"), ("yo_BJ", "Buenos Aires"), ("yue", "布宜諾斯艾利斯"), ("yue_Hans", "布宜诺斯艾利斯"), ("zh", "布宜諾斯艾利斯"), ("zu", "Buenos Aires")]),
                        unofficial_name_list: ["Capital federal"].to_vec(),
                    }
                ),
                (
                    "D",
                    Subdivision{
                        name: "D",
                        country_alpha2: Alpha2::AR,
                        code: "D",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-33.3022202), longitude: Some(-66.3367976), max_latitude: Some(-33.2583568), min_latitude: Some(-33.3412384), max_longitude: Some(-66.2345254), min_longitude: Some(-66.38436519999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سان لويس"), ("be", "Правінцыя Сан-Луіс"), ("bg", "Сан Луис"), ("bn", "স\u{9cd}কোয\u{9be}ন ল\u{9c1}ই প\u{9cd}রদেশ"), ("ca", "Província de San Luis"), ("ccp", "𑄥𑄚\u{11134} 𑄣\u{1112a}𑄃\u{11128}𑄌\u{11134}"), ("ceb", "Provincia de San Luis"), ("cs", "San Luis"), ("da", "San Luis Province"), ("de", "Provinz San Luis"), ("el", "Σαν Λουίς"), ("en", "San Luis"), ("es", "Provincia de San Luis"), ("eu", "San Luisko probintzia"), ("fa", "ایالت سان لوئیز"), ("fi", "San Luisin maakunta"), ("fr", "province de San Luis"), ("gl", "Provincia de San Luis"), ("gu", "સાન લ\u{ac1}ઈસ પ\u{acd}રા\u{a82}ત"), ("he", "סן לואיס"), ("hi", "स\u{948}न ल\u{941}ई प\u{94d}रान\u{94d}त"), ("hy", "Սան Լուիս"), ("id", "Provinsi San Luis"), ("it", "provincia di San Luis"), ("ja", "サンルイス州"), ("jv", "Provinsi San Luis"), ("ka", "სან-ლუისის პროვინცია"), ("kn", "ಸ\u{ccd}ಯಾನ\u{ccd} ಲ\u{cc2}ಯ\u{cbf}ಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "산루이스 주"), ("lt", "San Luiso provincija"), ("lv", "Sanluisas province"), ("mk", "Сан Луис"), ("mr", "सान ल\u{941}ईस"), ("ms", "San Luis"), ("nb", "San Luis"), ("nl", "San Luis"), ("no", "San Luis"), ("pl", "San Luis"), ("pt", "San Luis"), ("ro", "Provincia San Luis"), ("ru", "Сан-Луис"), ("si", "සැන\u{dca} ල\u{dd4}ව\u{dd2}ස\u{dca} පළ\u{dcf}ත"), ("sk", "San Luis"), ("sr", "Сан Луис"), ("sr_Latn", "San Luis"), ("sv", "San Luis"), ("sw", "Mkoa wa San Luis"), ("ta", "ச\u{bbe}ன\u{bcd} லுயிஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3e}న\u{c4d} లూయ\u{c40} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐซานล\u{e38}ยส\u{e4c}"), ("tr", "San Luis eyaleti"), ("uk", "Сан-Луїс"), ("ur", "صوبہ سان لوئیس"), ("vi", "San Luis"), ("zh", "圣路易省")]),
                        unofficial_name_list: ["San Luis"].to_vec(),
                    }
                ),
                (
                    "E",
                    Subdivision{
                        name: "E",
                        country_alpha2: Alpha2::AR,
                        code: "E",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-32.5175643), longitude: Some(-59.1041758), max_latitude: Some(-30.1576867), min_latitude: Some(-34.0391276), max_longitude: Some(-57.80086299999999), min_longitude: Some(-60.7680611)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إنتري ريوس"), ("az", "Entre-Rios"), ("be", "правінцыя Энтрэ-Рыас"), ("bg", "Ентре Риос"), ("bn", "এন\u{9cd}ত\u{9cd}রে রিও প\u{9cd}রদেশ"), ("ca", "Entre Ríos"), ("ccp", "𑄃𑄬𑄚\u{11134}𑄑\u{11133}𑄢\u{11128} 𑄢\u{11128}𑄠\u{1112e}𑄌\u{11134}"), ("ceb", "Provincia de Entre Ríos"), ("cs", "Entre Ríos"), ("cy", "Entre Ríos"), ("da", "Entre Ríos"), ("de", "Entre Ríos"), ("el", "Έντρε Ρίος"), ("en", "Entre Ríos"), ("es", "Provincia de Entre Ríos"), ("eu", "Entre Ríosko probintzia"), ("fa", "ایالت انتره ریوز"), ("fi", "Entre Ríosin maakunta"), ("fr", "province d’Entre Ríos"), ("gl", "Provincia de Entre Ríos"), ("gu", "એન\u{acd}ટ\u{acd}ર\u{ac7} રિયોસ પ\u{acd}રા\u{a82}ત"), ("he", "אנטרה ריוס"), ("hi", "एन\u{94d}ट\u{94d}र\u{947} रियोस"), ("hu", "Entre Ríos tartomány"), ("hy", "Էնտրե Ռիոս"), ("id", "Provinsi Entre Ríos"), ("it", "provincia di Entre Ríos"), ("ja", "エントレ・リオス州"), ("ka", "ენტრე-რიოსის პროვინცია"), ("kn", "ಎಂಟ\u{ccd}ರ\u{cc6} ರೈಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "엔트레리오스 주"), ("lt", "Entre Rioso provincija"), ("lv", "Entreriosas province"), ("mk", "Ентре Риос"), ("mr", "ए\u{902}त\u{94d}र\u{947} रियोस"), ("ms", "Wilayah Entre Ríos"), ("nb", "Entre Ríos"), ("nl", "Entre Ríos"), ("no", "Entre Ríos"), ("pl", "Entre Ríos"), ("pt", "Entre Ríos"), ("ro", "Provincia Entre Ríos"), ("ru", "Энтре-Риос"), ("si", "එන\u{dca}ට\u{dca}\u{200d}ර\u{dd2} රයොස\u{dca} පළ\u{dcf}ත"), ("sk", "Entre Ríos"), ("sl", "Entre Rios"), ("sr", "Ентре Риос"), ("sr_Latn", "Entre Rios"), ("sv", "Entre Ríos"), ("sw", "Mkoa wa Entre Ríos"), ("ta", "இன\u{bcd}றே ரியோஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఎంట\u{c4d}ర\u{c3f} ర\u{c3f}య\u{c4b}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐเอนเตรร\u{e35}โอส"), ("tr", "Entre Ríos eyaleti"), ("uk", "Ентре-Ріос"), ("ur", "صوبہ انترے ریوس"), ("vi", "Entre Ríos"), ("zh", "恩特雷里奥斯省")]),
                        unofficial_name_list: ["Entre Ríos"].to_vec(),
                    }
                ),
                (
                    "F",
                    Subdivision{
                        name: "F",
                        country_alpha2: Alpha2::AR,
                        code: "F",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-29.4128001), longitude: Some(-66.8559803), max_latitude: Some(-29.3799026), min_latitude: Some(-29.4568857), max_longitude: Some(-66.7890698), min_longitude: Some(-66.9253657)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لا ريوخا"), ("be", "Правінцыя Ла-Рыёха, Аргенціна"), ("bg", "Ла Риоха"), ("bn", "ল\u{9be} রিওজ\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de La Rioja"), ("ccp", "𑄣 𑄢\u{11128}𑄃\u{1112e}𑄎"), ("ceb", "Provincia de La Rioja (lalawigan)"), ("cs", "La Rioja"), ("da", "Provincia de La Rioja"), ("de", "Provinz La Rioja"), ("el", "Λα Ριότζα"), ("en", "La Rioja"), ("es", "Provincia de La Rioja"), ("eu", "La Riojako probintzia"), ("fa", "ایالت لاریوخا، آرژانتین"), ("fi", "La Riojan maakunta"), ("fr", "province de La Rioja"), ("gl", "Provincia de La Rioja"), ("gu", "લા રિયોજા પ\u{acd}રા\u{a82}ત"), ("he", "לה ריוחה"), ("hi", "ला रियोजा"), ("hr", "La Rioja"), ("hy", "Լա Ռիոխա"), ("id", "Provinsi La Rioja"), ("it", "provincia di La Rioja"), ("ja", "ラ・リオハ州"), ("ka", "ლა-რიოხის პროვინცია"), ("kn", "ಲಾ ರ\u{cbf}ಜಜ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "라리오하 주"), ("lt", "La Riochos provincija"), ("lv", "Larjohas province"), ("mk", "Ла Риоха"), ("mr", "ला रियोजा प\u{94d}रा\u{902}त"), ("ms", "La Rioja Province"), ("nb", "La Rioja"), ("nl", "La Rioja"), ("no", "La Rioja"), ("pl", "La Rioja"), ("pt", "Rioja"), ("ro", "Provincia La Rioja"), ("ru", "Ла-Риоха"), ("si", "ල\u{dcf} ර\u{dd2}යෝජ\u{dcf} පළ\u{dcf}ත"), ("sk", "La Rioja"), ("sr", "Риоха"), ("sr_Latn", "Rioha"), ("sv", "La Rioja"), ("sw", "Mkoa wa La Rioja"), ("ta", "ல\u{bbe} ரியோஜ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c3e} ర\u{c3f}య\u{c4b}జ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐลาร\u{e35}โอคา"), ("tr", "La Rioja Eyaleti"), ("uk", "Ла-Ріоха"), ("ur", "لا ریوجا صوبہ، ارجنٹین"), ("vi", "La Rioja"), ("zh", "拉里奥哈省")]),
                        unofficial_name_list: ["La Rioja"].to_vec(),
                    }
                ),
                (
                    "G",
                    Subdivision{
                        name: "G",
                        country_alpha2: Alpha2::AR,
                        code: "G",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-27.78442), longitude: Some(-64.26728059999999), max_latitude: Some(-27.7426275), min_latitude: Some(-27.8551652), max_longitude: Some(-64.2216859), min_longitude: Some(-64.3114394)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة سانتياغو ديل استيرو"), ("bg", "Сантяго дел Естеро"), ("bn", "স\u{9cd}য\u{9be}ন\u{9cd}টিয\u{9bc}\u{9be}গো ডেল এস\u{9cd}তেরো প\u{9cd}রদেশ"), ("ca", "Província de Santiago del Estero"), ("ccp", "𑄥𑄚\u{11134}𑄑\u{11128}𑄠𑄉\u{1112e} 𑄓𑄬𑄣\u{11134} 𑄃𑄬𑄌\u{11134}𑄑𑄢\u{1112e}"), ("ceb", "Provincia de Santiago del Estero"), ("cs", "Santiago del Estero"), ("cy", "Talaith Santiago del Estero"), ("da", "Provincia de Santiago del Estero"), ("de", "Santiago del Estero (Provinz)"), ("el", "Σαντιάγκο ντελ Εστέρο"), ("en", "Santiago del Estero"), ("es", "Provincia de Santiago del Estero"), ("eu", "Santiago del Esteroko probintzia"), ("fa", "ایالت سانتیاگو دل استرو"), ("fi", "Santiago del Esteron maakunta"), ("fr", "Santiago del Estero"), ("gl", "Provincia de Santiago del Estero"), ("gu", "સ\u{ac5}\u{a82}ટિયાગો ડ\u{ac7}લ ઍસ\u{acd}ટરો પ\u{acd}રા\u{a82}ત"), ("he", "סנטיאגו דל אסטרו"), ("hi", "स\u{948}न\u{94d}टियागो ड\u{947}ल एस\u{94d}त\u{94d}रो प\u{94d}रान\u{94d}त"), ("id", "Provinsi Santiago del Estero"), ("it", "provincia di Santiago del Estero"), ("ja", "サンティアゴ・デル・エステロ州"), ("ka", "სანტიაგო-დელ-ესტეროს პროვინცია"), ("kn", "ಸ\u{ccd}ಯಾಂಟ\u{cbf}ಯಾಗೊ ಡ\u{cc6}ಲ\u{ccd} ಎಸ\u{ccd}ಟರೋ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "산티아고델에스테로 주"), ("lt", "Santjago del Estero provincija"), ("lv", "Santjado del Estero province"), ("mk", "Сантјаго дел Естеро"), ("mn", "Сантьяго-дель-Эстеро муж"), ("mr", "सा\u{902}तियागो द\u{947}ल एस\u{94d}त\u{947}रो"), ("ms", "Wilayah Santiago del Estero"), ("nb", "Santiago del Estero"), ("nl", "Santiago del Estero"), ("no", "Santiago del Estero"), ("pl", "Santiago del Estero"), ("pt", "Santiago del Estero"), ("ro", "Provincia Santiago del Estero"), ("ru", "Сантьяго-дель-Эстеро"), ("si", "සැන\u{dca}ට\u{dd2}ය\u{dcf}ගෝ ඩෙල\u{dca} එස\u{dca}ටෙරෝ පළ\u{dcf}ත"), ("sk", "Santiago del Estero"), ("sr", "Сантијаго дел Естеро"), ("sr_Latn", "Santijago del Estero"), ("sv", "Santiago del Estero"), ("sw", "Mkoa wa Santiago del Estero"), ("ta", "ச\u{bbe}ண\u{bcd}டிய\u{bbe}கோ டேல\u{bcd} எஸ\u{bcd}டெரோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3e}ంట\u{c3f}య\u{c3e}గ\u{c4b} డ\u{c46}ల\u{c4d} ఎస\u{c4d}ట\u{c46}ర\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐซานเต\u{e35}ยโกเดลเอสเตโร"), ("tr", "Santiago del Estero eyaleti"), ("uk", "Сантьяго-дель-Естеро"), ("ur", "صوبہ سانتیاگو دل استرو"), ("vi", "Santiago del Estero"), ("zh", "圣地亚哥－德尔埃斯特罗省")]),
                        unofficial_name_list: ["Santiago del Estero"].to_vec(),
                    }
                ),
                (
                    "H",
                    Subdivision{
                        name: "H",
                        country_alpha2: Alpha2::AR,
                        code: "H",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-26.5857656), longitude: Some(-60.9540073), max_latitude: Some(-24.087868), min_latitude: Some(-27.9955354), max_longitude: Some(-58.36362680000001), min_longitude: Some(-63.42735869999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شاكو"), ("be", "Правінцыя Чака"), ("bg", "Чако"), ("bn", "ক\u{9cd}য\u{9be}কো প\u{9cd}রদেশ"), ("ca", "Província del Chaco"), ("ccp", "𑄍𑄇\u{1112e}"), ("ceb", "Provincia del Chaco"), ("cs", "Chaco"), ("cy", "Talaith Chaco"), ("da", "Chaco"), ("de", "Provinz Chaco"), ("el", "Τσάκο"), ("en", "Chaco"), ("es", "Provincia del Chaco"), ("et", "Chaco provints"), ("eu", "Chacoko probintzia"), ("fa", "ایالت چاکو"), ("fi", "Chacon maakunta"), ("fr", "province du Chaco"), ("gl", "Provincia do Chaco"), ("gu", "ચાકો પ\u{acd}રા\u{a82}ત"), ("he", "צ׳אקו"), ("hi", "चाको"), ("hu", "Chaco tartomány"), ("hy", "Չակո"), ("id", "Provinsi Chaco"), ("it", "provincia del Chaco"), ("ja", "チャコ州"), ("ka", "ჩაკოს პროვინცია"), ("kn", "ಚಾಕೋ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "차코 주"), ("lt", "Čiako provincija"), ("lv", "Čako province"), ("mk", "Чако"), ("mr", "चाको"), ("ms", "Wilayah Chaco"), ("nb", "Chaco"), ("ne", "चाको क\u{94d}ष\u{947}त\u{94d}र"), ("nl", "Chaco"), ("no", "Chaco"), ("pl", "Chaco"), ("pt", "Chaco"), ("ro", "Provincia Chaco"), ("ru", "Чако"), ("si", "චකෝ පළ\u{dcf}ත"), ("sk", "Chaco"), ("sl", "Chaco"), ("sr", "Чако"), ("sr_Latn", "Čako"), ("sv", "Chaco"), ("sw", "Mkoa wa Chaco"), ("ta", "ச\u{bbe}கோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "చ\u{c3e}క\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐชาโก"), ("tr", "Chaco eyaleti"), ("uk", "Чако"), ("ur", "صوبہ چاکو"), ("vi", "Chaco"), ("yue", "查哥省"), ("yue_Hans", "查哥省"), ("zh", "查科省")]),
                        unofficial_name_list: ["Chaco"].to_vec(),
                    }
                ),
                (
                    "J",
                    Subdivision{
                        name: "J",
                        country_alpha2: Alpha2::AR,
                        code: "J",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-31.5272732), longitude: Some(-68.5214081), max_latitude: Some(-31.4894698), min_latitude: Some(-31.6017076), max_longitude: Some(-68.4631344), min_longitude: Some(-68.644147)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سان خوان"), ("be", "Правінцыя Сан-Хуан"), ("bg", "Сан Хуан"), ("bn", "স\u{9be}ন জ\u{9c1}য\u{9bc}\u{9be}ন প\u{9cd}রদেশ"), ("ca", "Província de San Juan"), ("ccp", "𑄥𑄚\u{11134} 𑄎\u{1112a}𑄠𑄚\u{11134}"), ("ceb", "Provincia de San Juan"), ("cs", "San Juan"), ("cy", "Talaith San Juan"), ("da", "San Juan Province"), ("de", "Provinz San Juan"), ("el", "Σαν Χουάν"), ("en", "San Juan"), ("es", "Provincia de San Juan"), ("eu", "San Juango probintzia"), ("fa", "ایالت سن خوآن"), ("fi", "San Juanin maakunta"), ("fr", "province de San Juan"), ("gl", "Provincia de San Juan"), ("gu", "સ\u{ac7}ન જ\u{ac1}આન પ\u{acd}રા\u{a82}ત"), ("he", "סן חואן"), ("hi", "स\u{948}न ज\u{941}आन प\u{94d}रान\u{94d}त"), ("hu", "San Juan tartomány"), ("id", "Provinsi San Juan, Argentina"), ("it", "provincia di San Juan"), ("ja", "サンフアン州"), ("ka", "სან-ხუანის პროვინცია"), ("kn", "ಸ\u{ccd}ಯಾನ\u{ccd} ಜುವಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "산후안 주"), ("lt", "San Chuano provincija"), ("lv", "Sanhuanas province"), ("mk", "Сан Хуан"), ("mr", "सान ह\u{941}आन प\u{94d}रा\u{902}त"), ("ms", "Wilayah San Juan"), ("nb", "San Juan"), ("nl", "San Juan"), ("no", "San Juan"), ("pl", "San Juan"), ("pt", "San Juan"), ("ro", "Provincia San Juan"), ("ru", "Сан-Хуан"), ("si", "සැන\u{dca} ජ\u{dd4}ව\u{dcf}න\u{dca} පළ\u{dcf}ත"), ("sk", "San Juan"), ("sr", "Сан Хуан"), ("sr_Latn", "San Huan"), ("sv", "San Juan"), ("sw", "Mkoa wa San Juan"), ("ta", "ச\u{bbe}ன\u{bcd} ஜுவ\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3e}న\u{c4d} జువ\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐซานควน"), ("tr", "San Juan eyaleti"), ("uk", "Сан-Хуан"), ("ur", "صوبہ سان خوآن، ارجنٹائن"), ("vi", "San Juan"), ("zh", "圣胡安省")]),
                        unofficial_name_list: ["San Juan"].to_vec(),
                    }
                ),
                (
                    "K",
                    Subdivision{
                        name: "K",
                        country_alpha2: Alpha2::AR,
                        code: "K",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-28.4689906), longitude: Some(-65.77897159999999), max_latitude: Some(-28.4203217), min_latitude: Some(-28.5101532), max_longitude: Some(-65.7289342), min_longitude: Some(-65.8248732)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كاتاماركا"), ("be", "Правінцыя Катамарка"), ("bg", "Катамарка"), ("bn", "ক\u{9be}ত\u{9be}ম\u{9be}রক\u{9be} প\u{9cd}রদেশ"), ("bs", "Catamarca"), ("ca", "Catamarca"), ("ccp", "𑄇\u{11133}𑄠𑄑𑄟𑄢\u{11134}𑄇"), ("ceb", "Provincia de Catamarca"), ("cs", "Catamarca"), ("cy", "Talaith Catamarca"), ("da", "Catamarca"), ("de", "Provinz Catamarca"), ("el", "Καταμάρκα"), ("en", "Catamarca"), ("es", "Catamarca"), ("et", "Catamarca provints"), ("eu", "Catamarcako probintzia"), ("fa", "ایالت کاتامارکا"), ("fi", "Catamarcan maakunta"), ("fr", "province de Catamarca"), ("gl", "Provincia de Catamarca"), ("gu", "કાટામાર\u{acd}કા પ\u{acd}રા\u{a82}ત"), ("he", "קטמרקה"), ("hi", "क\u{948}टमार\u{94d}का"), ("hu", "Catamarca tartomány"), ("hy", "Կատամարկա"), ("id", "Provinsi Catamarca"), ("it", "provincia di Catamarca"), ("ja", "カタマルカ州"), ("ka", "კატამარკის პროვინცია"), ("kn", "ಕ\u{ccd}ಯಾಟಮಾರ\u{ccd}ಕಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "카타마르카 주"), ("lt", "Katamarkos provincija"), ("lv", "Katamarkas province"), ("mk", "Катамарка"), ("mr", "कातामार\u{94d}का"), ("ms", "Wilayah Catamarca"), ("nb", "Catamarca"), ("nl", "Catamarca"), ("no", "Catamarca"), ("pl", "Catamarca"), ("pt", "Catamarca"), ("ro", "Provincia Catamarca"), ("ru", "Катамарка"), ("si", "කටමර\u{dca}ස\u{dcf} පළ\u{dcf}ත"), ("sk", "Catamarca"), ("sr", "Катамарка"), ("sr_Latn", "Katamarka"), ("sv", "Catamarca"), ("sw", "Mkoa wa Catamarca"), ("ta", "க\u{bbe}ட\u{bcd}ம\u{bbe}ர\u{bcd}க\u{bcd}க\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}టమ\u{c3e}ర\u{c4d}స\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐกาตามาร\u{e4c}กา"), ("tr", "Catamarca eyaleti"), ("uk", "Катамарка"), ("ur", "صوبہ کاتامارکا"), ("vi", "Catamarca"), ("yue", "卡塔馬卡省"), ("yue_Hans", "卡塔马卡省"), ("zh", "卡塔马卡省")]),
                        unofficial_name_list: ["Catamarca"].to_vec(),
                    }
                ),
                (
                    "L",
                    Subdivision{
                        name: "L",
                        country_alpha2: Alpha2::AR,
                        code: "L",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-37.8956594), longitude: Some(-65.0957792), max_latitude: Some(-34.9923158), min_latitude: Some(-39.3161456), max_longitude: Some(-63.38682430000001), min_longitude: Some(-68.29545)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لا بامبا"), ("be", "Правінцыя Ла-Пампа"), ("bg", "Ла Пампа"), ("bn", "ল\u{9be} প\u{9be}ম\u{9cd}প\u{9be} প\u{9cd}রদেশ"), ("ca", "La Pampa"), ("ccp", "𑄣 𑄛𑄟\u{11134}𑄛"), ("ceb", "Provincia de La Pampa"), ("cs", "La Pampa"), ("cy", "La Pampa"), ("da", "La Pampa"), ("de", "Provinz La Pampa"), ("el", "Λα Πάμπα"), ("en", "La Pampa"), ("es", "Provincia de La Pampa"), ("eu", "La Pampako probintzia"), ("fa", "ایالت لا پامپا"), ("fi", "La Pampan maakunta"), ("fr", "province de La Pampa"), ("gl", "Provincia de La Pampa"), ("gu", "લા પામ\u{acd}પા પ\u{acd}રા\u{a82}ત"), ("he", "לה פמפה"), ("hi", "ला पम\u{94d}पा"), ("id", "Provinsi La Pampa"), ("it", "provincia di La Pampa"), ("ja", "ラ・パンパ州"), ("ka", "ლა-პამპის პროვინცია"), ("kn", "ಲಾ ಪಂಪ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "라팜파 주"), ("lt", "La Pampos provincija"), ("lv", "Lapampas province"), ("mk", "Ла Пампа"), ("mr", "ला पा\u{902}पा"), ("ms", "Wilayah La Pampa"), ("nb", "La Pampa"), ("nl", "La Pampa"), ("no", "La Pampa"), ("pl", "La Pampa"), ("pt", "La Pampa"), ("ro", "Provincia La Pampa"), ("ru", "Ла-Пампа"), ("si", "ල\u{dcf} පම\u{dca}ප\u{dcf} පළ\u{dcf}ත"), ("sk", "La Pampa"), ("sr", "Пампа"), ("sr_Latn", "Pampa"), ("sv", "La Pampa"), ("sw", "Mkoa wa La Pampa"), ("ta", "ல\u{bbe} பம\u{bcd}ப\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c3e} ప\u{c3e}ంప\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐลาป\u{e31}มปา"), ("tr", "La Pampa eyaleti"), ("uk", "Ла-Пампа"), ("ur", "لا پامپا صوبہ"), ("vi", "La Pampa"), ("zh", "拉潘帕省")]),
                        unofficial_name_list: ["Pampa"].to_vec(),
                    }
                ),
                (
                    "M",
                    Subdivision{
                        name: "M",
                        country_alpha2: Alpha2::AR,
                        code: "M",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-32.890183), longitude: Some(-68.8440498), max_latitude: Some(-32.8091398), min_latitude: Some(-33.0437567), max_longitude: Some(-68.7269211), min_longitude: Some(-68.900637)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مندوسا"), ("be", "правінцыя Мендоса"), ("bg", "Мендоса"), ("bn", "মেন\u{9cd}ডোজ\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Mendoza"), ("ccp", "𑄟𑄬𑄚\u{11134}𑄘\u{1112e}𑄎"), ("ceb", "Provincia de Mendoza"), ("cs", "Mendoza"), ("cy", "Talaith Mendoza"), ("da", "Mendoza"), ("de", "Provinz Mendoza"), ("el", "Μεντόζα"), ("en", "Mendoza"), ("es", "Provincia de Mendoza"), ("eu", "Mendozako probintzia"), ("fa", "ایالت مندوزا"), ("fi", "Mendozan maakunta"), ("fr", "province de Mendoza"), ("gl", "Provincia de Mendoza"), ("gu", "મ\u{ac7}ન\u{acd}ડોઝા પ\u{acd}રા\u{a82}ત"), ("he", "מנדוסה"), ("hi", "म\u{947}न\u{94d}दोज\u{93c}ा प\u{94d}रान\u{94d}त"), ("hr", "Mendoza"), ("hu", "Mendoza tartomány"), ("id", "Provinsi Mendoza"), ("it", "provincia di Mendoza"), ("ja", "メンドーサ州"), ("jv", "Provinsi Mendoza"), ("ka", "მენდოსის პროვინცია"), ("kn", "ಮ\u{cc6}ಂಡೋಜ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "멘도사 주"), ("lt", "Mendosos provincija"), ("lv", "Mendosas province"), ("mk", "Мендоза"), ("mr", "म\u{947}न\u{94d}दोसा"), ("ms", "Wilayah Mendoza"), ("nb", "Mendoza"), ("nl", "Mendoza"), ("no", "Mendoza"), ("pl", "Mendoza"), ("pt", "Mendoza (província)"), ("ro", "Provincia Mendoza"), ("ru", "Мендоса"), ("si", "මෙන\u{dca}ඩෝස\u{dcf} පල\u{dcf}ත"), ("sk", "Mendoza"), ("sl", "Mendoza"), ("sr", "Мендоза"), ("sr_Latn", "Mendoza"), ("sv", "Mendoza"), ("sw", "Mkoa wa Mendoza"), ("ta", "மென\u{bcd}டோஜ\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c46}ండ\u{c4b}జ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐเมนโดซา"), ("tr", "Mendoza eyaleti"), ("uk", "Мендоса"), ("ur", "صوبہ مندوسا"), ("vi", "Mendoza"), ("yue", "曼度莎省"), ("yue_Hans", "曼度莎省"), ("zh", "门多萨省")]),
                        unofficial_name_list: ["Mendoza"].to_vec(),
                    }
                ),
                (
                    "N",
                    Subdivision{
                        name: "N",
                        country_alpha2: Alpha2::AR,
                        code: "N",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-26.9377146), longitude: Some(-54.4342138), max_latitude: Some(-25.4954899), min_latitude: Some(-28.1633594), max_longitude: Some(-53.6385579), min_longitude: Some(-56.0595043)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ميسيونس"), ("be", "правінцыя Місьёнес"), ("bg", "Мисионес"), ("bn", "মিশনেস প\u{9cd}রদেশ"), ("ca", "Província de Misiones"), ("ccp", "𑄟\u{1112d}𑄥\u{11128}𑄠\u{1112e}𑄚\u{11128}𑄌\u{11134}"), ("ceb", "Provincia de Misiones"), ("cs", "Misiones"), ("cy", "Talaith Misiones"), ("da", "Provincia de Misiones"), ("de", "Misiones"), ("el", "Μισιόνες"), ("en", "Misiones"), ("es", "Provincia de Misiones"), ("eu", "Misionesko probintzia"), ("fa", "ایالت میسیونز"), ("fi", "Misionesin maakunta"), ("fr", "province de Misiones"), ("gl", "Misiones, Arxentina"), ("gu", "મિસીયોન\u{ac7}સ પ\u{acd}રા\u{a82}ત"), ("he", "מיסיונס"), ("hi", "मिसियोन\u{947}स"), ("hu", "Misiones tartomány"), ("hy", "Միսիոնես"), ("id", "Provinsi Misiones"), ("it", "provincia di Misiones"), ("ja", "ミシオネス州"), ("ka", "მისიონესის პროვინცია"), ("kn", "ಮ\u{cbf}ಷ\u{cc6}ನ\u{ccd}ಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "미시오네스 주"), ("lt", "Misioneso provincija"), ("lv", "Misjonesas province"), ("mk", "Мисионес"), ("mr", "मिस\u{94d}योन\u{947}स"), ("ms", "Wilayah Misiones"), ("nb", "Misiones"), ("nl", "Misiones"), ("no", "Misiones"), ("pl", "Misiones"), ("pt", "Misiones"), ("ro", "Provincia Misiones"), ("ru", "Мисьонес"), ("si", "ම\u{dd2}ස\u{dd2}යෝන\u{dca}ස\u{dca} පළ\u{dcf}ත"), ("sk", "Misiones"), ("sr", "Мисионес"), ("sr_Latn", "Misiones"), ("sv", "Misiones"), ("sw", "Mkoa wa Misiones"), ("ta", "மிசியோன\u{bcd}ஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c3f}స\u{c3f}య\u{c4b}న\u{c46}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐม\u{e35}ซ\u{e35}โอเนส"), ("tr", "Misiones eyaleti"), ("uk", "Місьйонес"), ("ur", "صوبہ میسیونس"), ("vi", "Misiones"), ("zh", "米西奧內斯省")]),
                        unofficial_name_list: ["Misiones"].to_vec(),
                    }
                ),
                (
                    "P",
                    Subdivision{
                        name: "P",
                        country_alpha2: Alpha2::AR,
                        code: "P",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-26.185201), longitude: Some(-58.1753697), max_latitude: Some(-26.124033), min_latitude: Some(-26.2202789), max_longitude: Some(-58.1419959), min_longitude: Some(-58.24794379999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "فورموسا"), ("be", "Правінцыя Фармоса"), ("bg", "Формоса"), ("bn", "ফ\u{9be}রমোস\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Formosa"), ("ccp", "𑄜\u{11127}𑄢\u{11134}𑄟\u{11127}𑄥"), ("ceb", "Provincia de Formosa"), ("cs", "Formosa"), ("cy", "Talaith Formosa"), ("da", "Provincia de Formosa"), ("de", "Provinz Formosa"), ("el", "Φορμόσα"), ("en", "Formosa"), ("es", "Provincia de Formosa"), ("eu", "Formosako probintzia"), ("fa", "ایالت فرموزا"), ("fi", "Formosan maakunta"), ("fr", "province de Formosa"), ("gl", "Provincia de Formosa"), ("gu", "ફોર\u{acd}મોસા પ\u{acd}રા\u{a82}ત"), ("he", "פורמוסה"), ("hi", "फ\u{93c}ॉरमोसा प\u{94d}रान\u{94d}त"), ("hu", "Formosa tartomány"), ("id", "Provinsi Formosa"), ("it", "provincia di Formosa"), ("ja", "フォルモサ州"), ("ka", "ფორმოსის პროვინცია"), ("kn", "ಫಾರ\u{ccd}ಮಾಸಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "포르모사 주"), ("lt", "Formosos provincija"), ("lv", "Formosas province"), ("mk", "Формоза"), ("mr", "फोर\u{94d}मोसा"), ("ms", "Wilayah Formosa"), ("nb", "Formosa"), ("nl", "Formosa"), ("no", "Formosa"), ("pl", "Formosa"), ("pt", "Formosa"), ("ro", "Provincia Formosa"), ("ru", "Формоса"), ("si", "ෆෝමෝස\u{dcf} පළ\u{dcf}ත"), ("sk", "Formosa"), ("sr", "Формоса"), ("sr_Latn", "Formosa"), ("sv", "Formosa"), ("sw", "Mkoa wa Formosa"), ("ta", "போர\u{bcd}மொஸ\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఫ\u{c4b}ర\u{c4d}మ\u{c4b}స\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐฟอร\u{e4c}โมซา"), ("tr", "Formosa eyaleti"), ("uk", "Формоса"), ("ur", "صوبہ فورموسا"), ("vi", "Formosa"), ("zh", "福爾摩沙省")]),
                        unofficial_name_list: ["Formosa"].to_vec(),
                    }
                ),
                (
                    "Q",
                    Subdivision{
                        name: "Q",
                        country_alpha2: Alpha2::AR,
                        code: "Q",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-38.9524444), longitude: Some(-68.0641389), max_latitude: Some(-38.893459), min_latitude: Some(-38.986537), max_longitude: Some(-68.0147382), min_longitude: Some(-68.1929919)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "نيوكوين"), ("be", "Правінцыя Неўкен"), ("bg", "Неукен"), ("bn", "নিউক\u{9c1}য\u{9bc}েন প\u{9cd}রদেশ"), ("ca", "Província del Neuquén"), ("ccp", "𑄚\u{11128}𑄅\u{1112a}𑄇\u{1112a}𑄠𑄬𑄚\u{11134}"), ("ceb", "Provincia del Neuquén"), ("cs", "Neuquén"), ("cy", "Talaith Neuquén"), ("da", "Provincia de Neuquén"), ("de", "Provinz Neuquén"), ("el", "Επαρχία του Νεουκέν"), ("en", "Neuquén"), ("es", "Provincia de Neuquén"), ("eu", "Neuquengo probintzia"), ("fa", "ایالت نئوکن"), ("fi", "Neuquénin maakunta"), ("fr", "province de Neuquén"), ("gl", "Provincia de Neuquén"), ("gu", "ન\u{ac7}ઉક\u{acd}વ\u{ac7}ન પ\u{acd}રા\u{a82}ત"), ("he", "נאוקן"), ("hi", "न\u{94d}य\u{942}क\u{94d}वीन प\u{94d}रान\u{94d}त"), ("hr", "Neuquén"), ("hu", "Neuquén tartomány"), ("hy", "Նեուկեն"), ("id", "Provinsi Neuquén"), ("it", "provincia di Neuquén"), ("ja", "ネウケン州"), ("ka", "ნეუკენის პროვინცია"), ("kn", "ನ\u{ccd}ಯ\u{cc2}ಕ\u{ccd}ವ\u{cc6}ನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "네우켄 주"), ("lt", "Neukeno provincija"), ("lv", "Neukenas province"), ("mk", "Неуквен"), ("mr", "न\u{947}उक\u{947}न"), ("ms", "Wilayah Neuquén"), ("nb", "Neuquén"), ("nl", "Neuquén"), ("no", "Neuquén"), ("pl", "Neuquén"), ("pt", "Neuquén"), ("ro", "Provincia Neuquén"), ("ru", "Неукен"), ("si", "න\u{dd2}ය\u{dd4}ක\u{dd4}එන\u{dca} පළ\u{dcf}ත"), ("sk", "Neuquén"), ("sr", "Неукен"), ("sr_Latn", "Neuken"), ("sv", "Neuquén"), ("sw", "Mkoa wa Neuquén"), ("ta", "னேஉயூன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "న\u{c4d}యూక\u{c4d}వ\u{c46}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐเนวเกน"), ("tr", "Neuquén eyaleti"), ("uk", "Неукен"), ("ur", "صوبہ نیوکوین"), ("vi", "Neuquén"), ("yue", "紐昆省"), ("yue_Hans", "纽昆省"), ("zh", "内乌肯省")]),
                        unofficial_name_list: ["Neuquén"].to_vec(),
                    }
                ),
                (
                    "R",
                    Subdivision{
                        name: "R",
                        country_alpha2: Alpha2::AR,
                        code: "R",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-40.7344343), longitude: Some(-66.6176455), max_latitude: Some(-37.5729141), min_latitude: Some(-42.0024984), max_longitude: Some(-62.79108240000001), min_longitude: Some(-71.902908)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ريو نيغرو"), ("be", "Правінцыя Рыа-Негра"), ("bg", "Рио Негро"), ("bn", "রিও নিগ\u{9cd}রো প\u{9cd}রদেশ"), ("ca", "Província de Río Negro"), ("ccp", "𑄢\u{11128}𑄃\u{1112e} 𑄚𑄬𑄉\u{11133}𑄢\u{11127}\u{1112e}"), ("ceb", "Provincia de Río Negro"), ("cs", "Río Negro"), ("cy", "Río Negro"), ("da", "Río Negro"), ("de", "Provinz Río Negro"), ("el", "Επαρχία Ρίο Νέγκρο"), ("en", "Río Negro"), ("es", "Provincia de Río Negro"), ("eu", "Río Negroko probintzia"), ("fa", "ایالت ریو نگرو"), ("fi", "Río Negron maakunta"), ("fr", "province de Río Negro"), ("gl", "Provincia de Río Negro"), ("gu", "રીયો ન\u{ac7}ગ\u{acd}રો પ\u{acd}રા\u{a82}ત"), ("he", "ריו נגרו"), ("hi", "रियो न\u{947}ग\u{94d}रो"), ("hu", "Río Negro tartomány"), ("hy", "Ռիո Նեգրո պրովինցիա"), ("id", "Provinsi Río Negro"), ("it", "provincia di Río Negro"), ("ja", "リオネグロ州"), ("ka", "რიო-ნეგროს პროვინცია"), ("kn", "ರ\u{cbf}ಯೊ ನ\u{cc6}ಗ\u{ccd}ರೋ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "리오네그로 주"), ("lt", "Rio Negro provincija"), ("lv", "Rionegro province"), ("mk", "Рио Негро"), ("mr", "रियो न\u{947}ग\u{94d}रो"), ("ms", "Wilayah Río Negro"), ("nb", "Río Negro"), ("nl", "Río Negro"), ("no", "Río Negro"), ("pl", "Río Negro"), ("pt", "Río Negro"), ("ro", "Provincia Río Negro"), ("ru", "Рио-Негро"), ("si", "ර\u{dd2}යෝ නෙග\u{dca}රෝ පළ\u{dcf}ත"), ("sk", "Río Negro"), ("sr", "Рио Негро"), ("sr_Latn", "Rio Negro"), ("sv", "Río Negro"), ("sw", "Mkoa wa Río Negro"), ("ta", "ரியோ நெகிறோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ర\u{c3f}య\u{c4b} న\u{c46}గ\u{c4d}ర\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐร\u{e35}โอเนโกร"), ("tr", "Río Negro eyaleti"), ("uk", "Ріо-Неґро"), ("ur", "صوبہ ریو نیگرو"), ("vi", "Río Negro"), ("zh", "内格罗河省")]),
                        unofficial_name_list: ["Río Negro"].to_vec(),
                    }
                ),
                (
                    "S",
                    Subdivision{
                        name: "S",
                        country_alpha2: Alpha2::AR,
                        code: "S",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-31.6323891), longitude: Some(-60.6994591), max_latitude: Some(-31.5685461), min_latitude: Some(-31.6730042), max_longitude: Some(-60.66380530000001), min_longitude: Some(-60.73878)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانتا في"), ("be", "правінцыя Санта-Фэ"), ("bg", "Санта Фе"), ("bn", "সয\u{9bc}\u{9be} স\u{9cd}য\u{9be}ন\u{9cd}ট\u{9be} ফে প\u{9cd}রদেশ"), ("bs", "Santa Fe"), ("ca", "Província de Santa Fe"), ("ccp", "𑄥𑄚\u{11134}𑄑 𑄜𑄬"), ("ceb", "Provincia de Santa Fe"), ("cs", "Santa Fé"), ("cy", "Talaith Santa Fe"), ("da", "Santa Fe"), ("de", "Provinz Santa Fe"), ("el", "Σάντα Φε (επαρχία)"), ("en", "Santa Fe"), ("es", "Provincia de Santa Fe"), ("eu", "Santa Feko probintzia"), ("fa", "ایالت سانتا فه"), ("fi", "Santa Fen maakunta"), ("fr", "province de Santa Fe"), ("gl", "Provincia de Santa Fe"), ("gu", "સા\u{a82}તા ફ\u{ac7} પ\u{acd}રા\u{a82}ત"), ("he", "סנטה פה"), ("hi", "स\u{948}न\u{94d}टा फ\u{948} प\u{94d}रान\u{94d}त"), ("hy", "Սանտա Ֆե նահանգ"), ("id", "Provinsi Santa Fe"), ("it", "provincia di Santa Fe"), ("ja", "サンタフェ州"), ("ka", "სანტა-ფეს პროვინცია"), ("kn", "ಸಾಂತಾ ಫ\u{cc6} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "산타페 주"), ("lt", "Santa Fė provincija"), ("lv", "Santafē province"), ("mk", "Санта Фе"), ("mr", "सा\u{902}ता फ\u{947}"), ("ms", "Wilayah Santa Fe"), ("nb", "Santa Fe"), ("nl", "Santa Fe"), ("no", "Santa Fe"), ("pl", "Santa Fe"), ("pt", "Santa Fé"), ("ro", "Provincia Santa Fe"), ("ru", "Санта-Фе"), ("si", "සැන\u{dca}ට\u{dcf} ෆෙ පළ\u{dcf}ත"), ("sk", "Santa Fe"), ("sr", "Санта Фе"), ("sr_Latn", "Santa Fe"), ("sv", "Santa Fe"), ("sw", "Mkoa wa Santa Fe"), ("ta", "ச\u{bbe}ண\u{bcd}ட\u{bbe} பி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3e}ంట\u{c3e} ఫ\u{c46} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐซานตาเฟ"), ("tr", "Santa Fe eyaleti"), ("uk", "Санта-Фе"), ("ur", "صوبہ سانتا فے"), ("vi", "Santa Fe"), ("zh", "聖大非省")]),
                        unofficial_name_list: ["Santa Fe"].to_vec(),
                    }
                ),
                (
                    "T",
                    Subdivision{
                        name: "T",
                        country_alpha2: Alpha2::AR,
                        code: "T",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-26.8082848), longitude: Some(-65.2175903), max_latitude: Some(-26.7636021), min_latitude: Some(-26.8935681), max_longitude: Some(-65.16676939999999), min_longitude: Some(-65.3363281)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "توكومان"), ("be", "правінцыя Тукуман"), ("bg", "Тукуман"), ("bn", "ত\u{9c1}ক\u{9c1}ম\u{9cd}য\u{9be}ন প\u{9cd}রদেশ"), ("ca", "Tucumán"), ("ccp", "𑄑𑄇\u{11128}𑄅\u{1112a}𑄟\u{11133}𑄠𑄚\u{11134}"), ("ceb", "Provincia de Tucumán"), ("cs", "Tucumán"), ("da", "Provincia de Tucumán"), ("de", "Provinz Tucumán"), ("el", "Τουκουμάν"), ("en", "Tucumán"), ("es", "Provincia de Tucumán"), ("eu", "Tucumángo probintzia"), ("fa", "ایالت توکومان"), ("fi", "Tucumánin maakunta"), ("fr", "province de Tucumán"), ("gl", "Provincia de Tucumán"), ("gu", "ત\u{ac1}ક\u{ac1}માન પ\u{acd}રા\u{a82}ત"), ("he", "טוקומאן"), ("hi", "ट\u{941}क\u{941}म\u{947}न"), ("id", "Provinsi Tucumán"), ("it", "provincia di Tucumán"), ("ja", "トゥクマン州"), ("ka", "ტუკუმანის პროვინცია"), ("kn", "ಟುಕುಮಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "투쿠만 주"), ("lt", "Tukumano provincija"), ("lv", "Tukumanas province"), ("mk", "Тукуман"), ("mr", "त\u{941}क\u{941}मान"), ("ms", "Wilayah Tucumán"), ("nb", "Tucumán"), ("nl", "Tucumán"), ("no", "Tucumán"), ("pl", "Tucumán"), ("pt", "Tucumán"), ("ro", "Provincia Tucumán"), ("ru", "Тукуман"), ("si", "ට\u{dd4}ක\u{dd4}ම\u{dcf}න\u{dca} පළ\u{dcf}ත"), ("sk", "Tucumán"), ("sl", "Provinca Tucumán"), ("sr", "Тукуман"), ("sr_Latn", "Tukuman"), ("sv", "Tucumán"), ("sw", "Mkoa wa Tucumán"), ("ta", "டுக\u{bcd}குமன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "టూకుమన\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐต\u{e39}ก\u{e39}ม\u{e31}น"), ("tr", "Tucumán eyaleti"), ("uk", "Тукуман"), ("ur", "صوبہ توکومان"), ("vi", "Tucumán"), ("zh", "图库曼省")]),
                        unofficial_name_list: ["Tucumán"].to_vec(),
                    }
                ),
                (
                    "U",
                    Subdivision{
                        name: "U",
                        country_alpha2: Alpha2::AR,
                        code: "U",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-43.6846192), longitude: Some(-69.2745537), max_latitude: Some(-32.9517763), min_latitude: Some(-46.0022302), max_longitude: Some(-58.0880073), min_longitude: Some(-72.1973387)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Chubut"), ("ar", "تشوبوت"), ("be", "Правінцыя Чубут"), ("bg", "Чубут"), ("bn", "চ\u{9c1}ব\u{9c1}ট প\u{9cd}রদেশ"), ("ca", "Chubut"), ("ccp", "𑄌\u{1112a}𑄝𑄖\u{11134}"), ("ceb", "Provincia del Chubut"), ("cs", "Chubut"), ("cy", "Talaith Chubut"), ("da", "Chubut"), ("de", "Provinz Chubut"), ("el", "Επαρχία του Τσουμπούτ"), ("en", "Chubut"), ("es", "Provincia del Chubut"), ("eu", "Chubuteko probintzia"), ("fa", "ایالت چوبوت"), ("fi", "Chubutin maakunta"), ("fr", "province de Chubut"), ("gl", "Provincia de Chubut"), ("gu", "ચ\u{ac1}બ\u{ac1}ટ પ\u{acd}રા\u{a82}ત"), ("he", "צ׳ובוט"), ("hi", "च\u{941}ब\u{941}ट"), ("hu", "Chubut tartomány"), ("hy", "Չուբուտ"), ("id", "Provinsi Chubut"), ("is", "Chubut-fylki"), ("it", "provincia di Chubut"), ("ja", "チュブ州"), ("ka", "ჩუბუტის პროვინცია"), ("kn", "ಚುಬುಟ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "추부트 주"), ("lt", "Čiubuto provincija"), ("lv", "Čubutas province"), ("mk", "Чубут"), ("mr", "च\u{941}ब\u{941}त"), ("ms", "Wilayah Chubut"), ("nb", "Chubut"), ("nl", "Chubut"), ("no", "Chubut"), ("pl", "Chubut"), ("pt", "Chubut"), ("ro", "Provincia Chubut"), ("ru", "Чубут"), ("si", "ච\u{dd4}බ\u{dd4}ට\u{dca} පළ\u{dcf}ත"), ("sk", "Chubut"), ("sl", "Chubut"), ("sr", "Чубут"), ("sr_Latn", "Čubut"), ("sv", "Chubut"), ("sw", "Mkoa wa Chubut"), ("ta", "சுபுட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఛుబుట\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐช\u{e39}บ\u{e38}ต"), ("tr", "Chubut eyaleti"), ("uk", "Чубут"), ("ur", "صوبہ چوبوت"), ("vi", "Chubut"), ("yue", "朱拔省"), ("yue_Hans", "朱拔省"), ("zh", "丘布特省")]),
                        unofficial_name_list: ["Chubut"].to_vec(),
                    }
                ),
                (
                    "V",
                    Subdivision{
                        name: "V",
                        country_alpha2: Alpha2::AR,
                        code: "V",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-54.3083548), longitude: Some(-67.7451565), max_latitude: Some(-52.658766), min_latitude: Some(-55.05719070000001), max_longitude: Some(-64.11057989999999), min_longitude: Some(-68.611864)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة تييرا ديل فويغو"), ("be", "Вогненная Зямля, Антарктыда і астравы Паўднёвай Атлантыкі"), ("bg", "Огнена земя"), ("bn", "টিয\u{9bc}ের\u{9be} দেল ফ\u{9c1}য\u{9bc}েগো\u{981}"), ("ca", "Terra del Foc, Antàrtida i Illes de l’Atlàntic Sud"), ("ccp", "𑄑\u{1112d}𑄢 𑄓𑄬𑄣\u{11134} 𑄜\u{11128}𑄅\u{1112a}𑄉\u{1112e}"), ("ceb", "Provincia de Tierra del Fuego (lalawigan)"), ("cs", "Tierra del Fuego"), ("cy", "Tierra del Fuego, Antarctica ac Ynysoedd De’r Iwerydd"), ("da", "Tierra del Fuego Province"), ("de", "Provinz Tierra del Fuego"), ("el", "Τιέρα ντελ Φουέγκο"), ("en", "Tierra del Fuego"), ("es", "Provincia de Tierra del Fuego, Antártida e islas del Atlántico Sur"), ("et", "Tulemaa provints"), ("eu", "Suaren Lurraldea, Antartika eta Hego Atlantikoko Uharteak"), ("fa", "ایالت تیرا دل فوئگو"), ("fi", "Tulimaan, Etelämantereen ja Etelä-Atlantin saarten provinssi"), ("fr", "Terre de Feu, Antarctique et Îles de l’Atlantique Sud"), ("gl", "Terra do Fogo"), ("gu", "ટિએરા ડ\u{ac7}લ ફ\u{ac1}એગો પ\u{acd}રા\u{a82}ત"), ("he", "טיירה דל פואגו"), ("hi", "टिएरा ड\u{947}ल फ\u{93c}\u{941}एगो"), ("hy", "Հրո Երկիր, Անտարկտիդա և Հարավային Ատլանտիկայի կղզիներ"), ("id", "Provinsi Tierra del Fuego"), ("it", "provincia di Terra del Fuoco, Antartide e Isole dell’Atlantico del Sud"), ("ja", "ティエラ・デル・フエゴ州"), ("ka", "ცეცხლოვანი მიწის პროვინცია"), ("kn", "ಟ\u{cbf}ಯ\u{cc6}ರಾ ಡ\u{cc6}ಲ\u{ccd} ಫ\u{ccd}ಯ\u{cc2}ಗೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "티에라델푸에고 주"), ("lt", "Ugnies Žemės, Antarktidos ir Pietų Atlanto salų provincija"), ("lv", "Tjerra del Fuego province"), ("mk", "Огнена Земја, Антарктика и Јужни Атлантски Острови"), ("mr", "तिएरा द\u{947}ल फ\u{94d}व\u{947}गो"), ("ms", "Wilayah Tierra del Fuego"), ("nb", "Tierra del Fuego, Antártida e Islas del Atlántico Sur"), ("nl", "Vuurland, Antarctica en Zuid-Atlantische eilanden"), ("no", "Tierra del Fuego, Antártida e Islas del Atlántico Sur"), ("pl", "Ziemia Ognista"), ("pt", "Terra do Fogo, Antártica e Ilhas do Atlântico Sul"), ("ro", "Provincia Tierra del Fuego"), ("ru", "Огненная Земля, Антарктида и острова Южной Атлантики"), ("si", "ට\u{dd2}යෙර\u{dcf} ඩෙල\u{dca} ෆ\u{dd4}එගෝ පළ\u{dcf}ත"), ("sk", "Tierra del Fuego, Antártida e Islas del Atlántico Sur"), ("sl", "Tierra del Fuego, Antártida e Islas del Atlántico Sur"), ("sr", "Огњена Земља"), ("sr_Latn", "Ognjena Zemlja"), ("sv", "Eldslandet"), ("sw", "Tierra del Fuego, Antaktiki na Visiwa vya Atlantiki Kusini"), ("ta", "டியெர\u{bcd}ர\u{bbe} டெல\u{bcd} பியூயகோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ట\u{c3f}య\u{c46}ర\u{c3e} డ\u{c46}ల\u{c4d} పుయ\u{c46}గ\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐเต\u{e35}ยร\u{e4c}ราเดลฟวยโก"), ("tr", "Tierra del Fuego"), ("uk", "Вогняна Земля, Антарктида та острови Південної Атлантики"), ("ur", "صوبہ تیئرا دل فوئگو، ارجنٹائن"), ("vi", "Tierra del Fuego"), ("zh", "火地岛省")]),
                        unofficial_name_list: ["Tierra del Fuego"].to_vec(),
                    }
                ),
                (
                    "W",
                    Subdivision{
                        name: "W",
                        country_alpha2: Alpha2::AR,
                        code: "W",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-27.4712257), longitude: Some(-58.83958440000001), max_latitude: Some(-27.4386008), min_latitude: Some(-27.5327967), max_longitude: Some(-58.74749370000001), min_longitude: Some(-58.8572739)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كوريينتس"), ("be", "Правінцыя Карыентэс"), ("bg", "Кориентес"), ("bn", "ক\u{9be}রিয\u{9bc}েন\u{9cd}তেস"), ("ca", "Província de Corrientes"), ("ccp", "𑄇\u{11127}𑄢\u{11128}𑄠𑄬𑄚\u{11134}𑄑𑄌\u{11134}"), ("ceb", "Provincia de Corrientes"), ("cs", "Corrientes"), ("cy", "Talaith Corrientes"), ("da", "Provincia de Corrientes"), ("de", "Provinz Corrientes"), ("el", "Κορριέντες"), ("en", "Corrientes"), ("es", "Provincia de Corrientes"), ("eu", "Corrientesko probintzia"), ("fa", "ایالت کورینتس"), ("fi", "Corrientesin maakunta"), ("fr", "province de Corrientes"), ("gl", "Provincia de Corrientes"), ("gu", "કોરિએન\u{acd}ટ\u{ac7}સ"), ("he", "קוריינטס"), ("hi", "कोरिय\u{947}न\u{94d}ट\u{947}स प\u{94d}रान\u{94d}त"), ("hu", "Corrientes tartomány"), ("id", "Provinsi Corrientes"), ("it", "provincia di Corrientes"), ("ja", "コリエンテス州"), ("ka", "კორიენტესის პროვინცია"), ("kn", "ಕೊರ\u{cbf}ಯ\u{cc6}ಂಟಸ\u{ccd}"), ("ko", "코리엔테스 주"), ("lt", "Korienteso provincija"), ("lv", "Korrjentesas province"), ("mk", "Кориентес"), ("mr", "कोरिय\u{947}न\u{94d}त\u{947}स"), ("ms", "Wilayah Corrientes"), ("nb", "Corrientes"), ("nl", "Corrientes"), ("no", "Corrientes"), ("pl", "Corrientes"), ("pt", "Corrientes"), ("ro", "Provincia Corrientes"), ("ru", "Корриентес"), ("si", "කොර\u{dd2}යෙන\u{dca}ට\u{dd2}ස\u{dca}"), ("sk", "Corrientes"), ("sr", "Коријентес"), ("sr_Latn", "Korijentes"), ("sv", "Corrientes"), ("sw", "Mkoa wa Corrientes"), ("ta", "கரிஎண\u{bcd}ட\u{bcd}ஸ\u{bcd}"), ("te", "క\u{c4b}ర\u{c3f}య\u{c46}ంట\u{c46}స\u{c4d}"), ("th", "ร\u{e31}ฐกอร\u{e4c}เร\u{e35}ยนเตส"), ("tr", "Corrientes eyaleti"), ("uk", "Коррієнтес"), ("ur", "صوبہ کورینتس"), ("vi", "Corrientes"), ("zh", "科连特斯省")]),
                        unofficial_name_list: ["Corrientes"].to_vec(),
                    }
                ),
                (
                    "X",
                    Subdivision{
                        name: "X",
                        country_alpha2: Alpha2::AR,
                        code: "X",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-31.3989296), longitude: Some(-64.1821289), max_latitude: Some(-31.3062938), min_latitude: Some(-31.4912166), max_longitude: Some(-64.0621309), min_longitude: Some(-64.31158649999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كوردوبا"), ("be", "правінцыя Кордава"), ("bg", "Кордоба"), ("bn", "কর\u{9cd}ডোব\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Córdoba"), ("ccp", "𑄇\u{11127}𑄢\u{11134}𑄓\u{1112e}𑄝"), ("ceb", "Provincia de Córdoba"), ("cs", "Córdoba"), ("cy", "Talaith Córdoba"), ("da", "Provincia de Córdoba"), ("de", "Córdoba"), ("el", "Κόρντομπα"), ("en", "Córdoba"), ("es", "Provincia de Córdoba"), ("et", "Córdoba provints"), ("eu", "Córdobako probintzia"), ("fa", "ایالت کوردوبا"), ("fi", "Córdoban maakunta"), ("fr", "province de Córdoba"), ("gl", "Provincia de Córdoba, Arxentina"), ("gu", "કોર\u{acd}ડોબા પ\u{acd}રા\u{a82}ત"), ("he", "קורדובה"), ("hi", "कोर\u{94d}डोबा"), ("hu", "Córdoba tartomány"), ("hy", "Կորդովա"), ("id", "Provinsi Córdoba"), ("it", "provincia di Córdoba"), ("ja", "コルドバ州"), ("ka", "კორდოვის პროვინცია"), ("kn", "ಕೊರ\u{ccd}ಡೊಬಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "코르도바 주"), ("lt", "Kordobos provincija"), ("lv", "Kordovas province"), ("mk", "Кордоба"), ("mr", "कोर\u{94d}दोबा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Córdoba"), ("nb", "Córdoba"), ("nl", "Córdoba"), ("no", "Córdoba"), ("pl", "Córdoba"), ("pt", "Córdova (província da Argentina)"), ("ro", "Provincia Córdoba"), ("ru", "Кордова"), ("si", "කොර\u{dca}ඩොබ\u{dcf} පළ\u{dcf}ත"), ("sk", "Córdoba"), ("sr", "Кордоба"), ("sr_Latn", "Kordoba"), ("sv", "Córdoba"), ("sw", "Mkoa wa Córdoba"), ("ta", "கோர\u{bcd}டோப\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}ర\u{c4d}డ\u{c4b}బ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐกอร\u{e4c}โดบา"), ("tr", "Córdoba eyaleti"), ("uk", "Кордова"), ("ur", "صوبہ کوردوبا، ارجنٹائن"), ("vi", "Córdoba"), ("yue", "哥多華省"), ("yue_Hans", "哥多华省"), ("zh", "科尔多瓦省")]),
                        unofficial_name_list: ["Córdoba"].to_vec(),
                    }
                ),
                (
                    "Y",
                    Subdivision{
                        name: "Y",
                        country_alpha2: Alpha2::AR,
                        code: "Y",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-24.1857864), longitude: Some(-65.2994767), max_latitude: Some(-24.1502348), min_latitude: Some(-24.2557641), max_longitude: Some(-65.2336132), min_longitude: Some(-65.37630039999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "خوخوي"), ("be", "правінцыя Жужуй"), ("bg", "Хухуй"), ("bn", "জ\u{9c1}জ\u{9c1}য\u{9bc} প\u{9cd}রদেশ"), ("ca", "Jujuy"), ("ccp", "𑄎\u{1112a}𑄎\u{11127}𑄠\u{11134}"), ("ceb", "Provincia de Jujuy"), ("cs", "Jujuy"), ("da", "Jujuy"), ("de", "Provinz Jujuy"), ("el", "Τζουτζούι"), ("en", "Jujuy"), ("es", "Provincia de Jujuy"), ("et", "Jujuy"), ("eu", "Jujuyko probintzia"), ("fa", "ایالت خوخوی"), ("fi", "Jujuyn maakunta"), ("fr", "province de Jujuy"), ("gl", "Provincia de Jujuy"), ("gu", "જ\u{ac1}જ\u{ac1}ય પ\u{acd}રા\u{a82}ત"), ("he", "חוחוי"), ("hi", "ज\u{94d}य\u{942}ज\u{941}ई"), ("hu", "Jujuy tartomány"), ("hy", "Խուխույ"), ("id", "Jujuy"), ("it", "provincia di Jujuy"), ("ja", "フフイ州"), ("ka", "ხუხუის პროვინცია"), ("kn", "ಜುಜುಯ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "후후이 주"), ("lt", "Chuchujaus provincija"), ("lv", "Huhujas province"), ("mk", "Хухуј"), ("mr", "ज\u{941}ज\u{941}य"), ("ms", "Wilayah Jujuy"), ("nb", "Jujuy"), ("nl", "Jujuy"), ("no", "Jujuy"), ("pl", "Jujuy"), ("pt", "Jujuy"), ("ro", "Provincia Jujuy"), ("ru", "Жужуй"), ("si", "ජ\u{dd4}ජ\u{dd4}ය\u{dd2} පළ\u{dcf}ත"), ("sk", "Jujuy"), ("sr", "Хухуј"), ("sr_Latn", "Huhuj"), ("sv", "Jujuy"), ("sw", "Mkoa wa Jujuy"), ("ta", "ஜூஜூய\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "జుజూయ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐค\u{e39}ค\u{e38}ย"), ("tr", "Jujuy eyaleti"), ("uk", "Жужуй"), ("ur", "صوبہ خوخوئی"), ("vi", "Jujuy"), ("zh", "胡胡伊省")]),
                        unofficial_name_list: ["Jujuy"].to_vec(),
                    }
                ),
                (
                    "Z",
                    Subdivision{
                        name: "Z",
                        country_alpha2: Alpha2::AR,
                        code: "Z",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-48.7736825), longitude: Some(-69.1917167), max_latitude: Some(-45.9915403), min_latitude: Some(-52.3975018), max_longitude: Some(-65.7171603), min_longitude: Some(-73.5948315)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانتا كروز"), ("be", "Правінцыя Санта-Крус"), ("bg", "Санта Крус"), ("bn", "স\u{9cd}য\u{9be}ন\u{9cd}ট\u{9be} ক\u{9cd}র\u{9c1}য প\u{9cd}রদেশ"), ("ca", "Província de Santa Cruz"), ("ccp", "𑄥𑄚\u{11134}𑄑 𑄇\u{11133}𑄢\u{1112a}𑄌\u{11134}"), ("ceb", "Provincia de Santa Cruz"), ("cs", "Santa Cruz"), ("cy", "Talaith Santa Cruz"), ("da", "Santa Cruz"), ("de", "Provinz Santa Cruz"), ("el", "Επαρχία Σάντα Κρους"), ("en", "Santa Cruz"), ("es", "Provincia de Santa Cruz"), ("et", "Santa Cruzi provints"), ("eu", "Santa Cruzko probintzia"), ("fa", "ایالت سانتا کروس، آرژانتین"), ("fi", "Santa Cruzin maakunta"), ("fr", "province de Santa Cruz"), ("gl", "Provincia de Santa Cruz"), ("gu", "સાન\u{acd}તા ક\u{acd}ર\u{ac2}ઝ પ\u{acd}રા\u{a82}ત"), ("he", "סנטה קרוס"), ("hi", "स\u{948}न\u{94d}ता क\u{94d}र\u{941}ज प\u{94d}रान\u{94d}त"), ("hu", "Santa Cruz tartomány"), ("hy", "Սանտա Կրուս"), ("id", "Provinsi Santa Cruz"), ("it", "provincia di Santa Cruz"), ("ja", "サンタクルス州"), ("ka", "სანტა-კრუსის პროვინცია"), ("kn", "ಸಾಂಟಾ ಕ\u{ccd}ರ\u{cc2}ಜ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "산타크루스 주"), ("lt", "Santa Kruso provincija"), ("lv", "Santakrusas province"), ("mk", "Санта Круз"), ("mr", "सा\u{902}ता क\u{94d}र\u{941}झ"), ("ms", "Wilayah Santa Cruz"), ("nb", "Santa Cruz"), ("nl", "Santa Cruz"), ("no", "Santa Cruz"), ("pl", "Santa Cruz"), ("pt", "Santa Cruz"), ("ro", "Provincia Santa Cruz"), ("ru", "Санта-Крус"), ("si", "සැන\u{dca}ට\u{dcf} කෘස\u{dca} පළ\u{dcf}ත"), ("sk", "Santa Cruz"), ("sl", "Santa Cruz"), ("sr", "Санта Круз"), ("sr_Latn", "Santa Kruz"), ("sv", "Santa Cruz"), ("sw", "Mkoa wa Santa Cruz, Argentina"), ("ta", "ச\u{bbe}ண\u{bcd}ட\u{bbe} கிருஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "శ\u{c3e}ంట\u{c3e} క\u{c4d}రజ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐซานตากร\u{e38}ซ"), ("tr", "Santa Cruz"), ("uk", "Санта-Крус"), ("ur", "صوبہ سانتا کروز، ارجنٹائن"), ("vi", "Santa Cruz"), ("yue", "聖古絲省"), ("yue_Hans", "圣古丝省"), ("zh", "圣克鲁斯省")]),
                        unofficial_name_list: ["Santa Cruz"].to_vec(),
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
#[cfg(feature = "ar")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::AR,
        alpha3: Alpha3::ARG,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{region}}\n{{country}}",
        ),
        continent: Continent::SouthAmerica,
        country_code: 54,
        currency_code: "ARS",
        gec: Some(GEC::AR),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("ARG"),
        iso_long_name: "The Argentine Republic",
        iso_short_name: "Argentina",
        official_language_list: ["es", "gn"].to_vec(),
        spoken_language_list: ["es", "gn"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8, 9].to_vec(),
        national_prefix: "0",
        nationality: Some("Argentinean"),
        number: "032",
        postal_code: true,
        postal_code_format: Some("((?:[A-HJ-NP-Z])?\\d{4})([A-Z]{3})?"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthAmerica),
        un_locode: "AR",
        unofficial_name_list: [
            "Argentina",
            "Argentinien",
            "Argentine",
            "アルゼンチン",
            "Argentinië",
        ]
        .to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Argentina"),
            ("af", "Argentinië"),
            ("ak", "Argentina"),
            ("am", "ጐሴጀንቲና"),
            ("an", "Archentina"),
            ("ar", "الأرجنتين"),
            ("as", "আৰ\u{9cd}জেনটিন\u{9be}"),
            ("ay", "Argentina"),
            ("az", "Argentina"),
            ("ba", "Argentina"),
            ("be", "Аргенціна"),
            ("bg", "Аржентина"),
            ("bi", "Argentina"),
            ("bn", "আর\u{9cd}জেনটিন\u{9be}"),
            ("bn_IN", "আর\u{9cd}জেনটিন\u{9be}"),
            ("br", "Arc'hantina"),
            ("bs", "Argentina"),
            ("ca", "Argentina"),
            ("ce", "Аргентина"),
            ("ch", "Argentina"),
            ("cs", "Argentina"),
            ("cv", "Аргентина"),
            ("cy", "Yr Ariannin"),
            ("da", "Argentina"),
            ("de", "Argentinien"),
            ("dv", "އ\u{7a7}ޖ\u{7ac}ނ\u{7b0}ޓ\u{7a9}ނ\u{7a7}"),
            ("dz", "ཨར་ཇ\u{f7a}ན་ཊ\u{f72}་ན།"),
            ("ee", "Argentina"),
            ("el", "Αργεντινή"),
            ("en", "Argentina"),
            ("eo", "Argentino"),
            ("es", "Argentina"),
            ("et", "Argentina"),
            ("eu", "Argentina"),
            ("fa", "آرژانتین"),
            ("ff", "Argentina"),
            ("fi", "Argentiina"),
            ("fo", "Argentina"),
            ("fr", "Argentine"),
            ("fy", "Argentynje"),
            ("ga", "An Airgintín"),
            ("gl", "Arxentina"),
            ("gn", "Argentina"),
            ("gu", "આર\u{acd}જ\u{ac7}ન\u{acd}ટિના"),
            ("gv", "Yn Argenteen"),
            ("ha", "Argentina"),
            ("he", "ארגנטינה"),
            ("hi", "अर\u{94d}ज\u{947}ण\u{94d}टीना"),
            ("hr", "Argentina"),
            ("ht", "Ajantin"),
            ("hu", "Argentína"),
            ("hy", "Արգենտինա"),
            ("ia", "Argentina"),
            ("id", "Argentina"),
            ("io", "Arjentinia"),
            ("is", "Argentína"),
            ("it", "Argentina"),
            ("iu", "Argentina"),
            ("ja", "アルゼンチン"),
            ("ka", "არგენტინა"),
            ("ki", "Argentina"),
            ("kk", "Аргентина"),
            ("kl", "Argentina"),
            ("km", "អាហ\u{17d2}សង\u{17cb}ទ\u{17b8}ន"),
            ("kn", "ಅರ\u{ccd}ಜ\u{cc6}ಂಟೈನಾ"),
            ("ko", "아르헨티나"),
            ("ku", "Arjantîn"),
            ("kv", "Аргентина"),
            ("kw", "Arghantina"),
            ("ky", "Аргентина"),
            ("lo", "ອາກຊ\u{eb1}ງຕ\u{eb5}ນ"),
            ("lt", "Argentina"),
            ("lv", "Argentīna"),
            ("mi", "Āketina"),
            ("mk", "Аргентина"),
            ("ml", "അര\u{d4d}\u{200d}ജന\u{d4d}റീന"),
            ("mn", "Аргентин"),
            ("mr", "अर\u{94d}ज\u{947}\u{902}र\u{94d}टिना"),
            ("ms", "Argentina"),
            ("mt", "Arġentina"),
            (
                "my",
                "အာဂျင\u{103a}တ\u{102e}းနားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Ardjentina"),
            ("nb", "Argentina"),
            ("ne", "अरज\u{947}नटिना"),
            ("nl", "Argentinië"),
            ("nn", "Argentina"),
            ("nv", "Argentina"),
            ("oc", "Argentina"),
            ("or", "ଅ\u{b3e}ର\u{b4d}ଜେଣ\u{b4d}ଟୀନ\u{b3e}"),
            ("pa", "ਅਰਜਨਟੀਨਾ"),
            ("pi", "अर\u{94d}जन\u{94d}टीना"),
            ("pl", "Argentyna"),
            ("ps", "ارجنټاین"),
            ("pt", "Argentina"),
            ("pt_BR", "Argentina"),
            ("ro", "Argentina"),
            ("ru", "Аргентина"),
            ("rw", "Arijantina"),
            ("sc", "Argentina"),
            ("sd", "Argentina"),
            ("si", "ආජන\u{dca}ට\u{dd2}න\u{dcf}ව"),
            ("sk", "Argentína"),
            ("sl", "Argentina"),
            ("so", "Arjantiina"),
            ("sq", "Argjentinë"),
            ("sr", "Аргентина"),
            ("sv", "Argentina"),
            ("sw", "Argentina"),
            ("ta", "அர\u{bcd}ஜென\u{bcd}டின\u{bbe}"),
            ("te", "అర\u{c4d}జ\u{c47}ంర\u{c4d}ట\u{c3f}న\u{c3e}"),
            ("tg", "Аргентина"),
            ("th", "อาร\u{e4c}เจนต\u{e34}นา"),
            ("ti", "ኣርጀንቲና"),
            ("tk", "Argentina"),
            ("tl", "Arhentina"),
            ("tr", "Arjantin"),
            ("tt", "Арgентина"),
            ("ug", "ئارگېنتىنا"),
            ("uk", "Аргентина"),
            ("ur", "ارجنٹائن"),
            ("uz", "Argentina"),
            ("ve", "Argentina"),
            ("vi", "Á-căn-đình"),
            ("wa", "Årdjintene"),
            ("wo", "Argentiin"),
            ("xh", "Argentina"),
            ("yo", "Argẹntínà"),
            ("zh_CN", "阿根廷"),
            ("zh_HK", "阿根廷"),
            ("zh_TW", "阿根廷"),
            ("zu", "I-Argentina"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}