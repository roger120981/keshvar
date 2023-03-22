// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Zambia

#[cfg(all(feature = "zm", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::ZM;
    pub const ALPHA3: Alpha3 = Alpha3::ZMB;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 260;
    pub const CURRENCY_CODE: &str = "ZMW";
    pub const GEC: Option<GEC> = Some(GEC::ZA);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("ZAM");
    pub const ISO_SHORT_NAME: &str = "Zambia";
    pub const ISO_LONG_NAME: &str = "The Republic of Zambia";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Zambian");
    pub const NUMBER: &str = "894";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternAfrica);
    pub const UN_LOCODE: &str = "ZM";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Zambia", "Sambia", "Zambie", "ザンビア"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Zambia"),
        ("af", "Zambië"),
        ("ak", "Zambia"),
        ("am", "ዛምቢያ"),
        ("an", "Zambia"),
        ("ar", "زامبيا"),
        ("as", "জ\u{9be}ম\u{9cd}বিয়\u{9be}"),
        ("ay", "Zambia"),
        ("az", "Zambiya"),
        ("ba", "Zambia"),
        ("be", "Замбія"),
        ("bg", "Замбия"),
        ("bi", "Zambia"),
        ("bn", "জ\u{9be}ম\u{9cd}বিয়\u{9be}"),
        ("bn_IN", "জ\u{9be}ম\u{9cd}বিয়\u{9be}"),
        ("br", "Zambia"),
        ("bs", "Zambija"),
        ("ca", "Zàmbia"),
        ("ce", "Замби"),
        ("ch", "Zambia"),
        ("cs", "Zambie"),
        ("cv", "Замби"),
        ("cy", "Zambia"),
        ("da", "Zambia"),
        ("de", "Sambia"),
        ("dv", "ޒ\u{7ac}މ\u{7b0}ބ\u{7a8}އ\u{7a7}"),
        ("dz", "ཛམ་བ\u{f72}་ཡ།"),
        ("ee", "Zambia"),
        ("el", "Ζάμπια"),
        ("en", "Zambia"),
        ("eo", "Zambio"),
        ("es", "Zambia"),
        ("et", "Sambia"),
        ("eu", "Zambia"),
        ("fa", "زامبیا"),
        ("ff", "Zambia"),
        ("fi", "Sambia"),
        ("fo", "Sambia"),
        ("fr", "Zambie"),
        ("fy", "Sambia"),
        ("ga", "An tSaimbia"),
        ("gl", "Zambia"),
        ("gn", "Zambia"),
        ("gu", "ઝા\u{a82}બિયા"),
        ("gv", "Yn Tambia"),
        ("ha", "Zambia"),
        ("he", "זמביה"),
        ("hi", "ज\u{93c}ाम\u{94d}बिया"),
        ("hr", "Zambija"),
        ("ht", "Zanbi"),
        ("hu", "Zambia"),
        ("hy", "Զամբիա"),
        ("ia", "Zambia"),
        ("id", "Zambia"),
        ("io", "Zambia"),
        ("is", "Sambía"),
        ("it", "Zambia"),
        ("iu", "Zambia"),
        ("ja", "ザンビア"),
        ("ka", "ზამბია"),
        ("ki", "Zambia"),
        ("kk", "Замбия"),
        ("kl", "Zambia"),
        ("km", "ហ\u{17d2}សា\u{17c6}ប\u{17ca}\u{17b8}"),
        ("kn", "ಝಾಂಬ\u{cbf}ಯಾ"),
        ("ko", "잠비아"),
        ("ku", "Zambiya"),
        ("kv", "Zambia"),
        ("kw", "Zambi"),
        ("ky", "Замбия"),
        ("lo", "Zambia"),
        ("lt", "Zambija"),
        ("lv", "Zambija"),
        ("mi", "Zambia"),
        ("mk", "Замбија"),
        ("ml", "സ\u{d3e}ംബിയ"),
        ("mn", "Замби"),
        ("mr", "झा\u{902}बिया"),
        ("ms", "Zambia"),
        ("mt", "Żambja"),
        (
            "my",
            "ဇမ\u{103a}ဘ\u{102e}ယာန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Zambia"),
        ("nb", "Zambia"),
        ("ne", "जाम\u{94d}बिया"),
        ("nl", "Zambia"),
        ("nn", "Zambia"),
        ("nv", "Zambia"),
        ("oc", "Zambia"),
        ("or", "ଜ\u{b3e}ମ\u{b4d}ବ\u{b3f}ୟ\u{b3e}"),
        ("pa", "ਜ਼\u{a48}ਬੀਆ"),
        ("pi", "जाम\u{94d}बिया"),
        ("pl", "Zambia"),
        ("ps", "زېمبيا"),
        ("pt", "Zâmbia"),
        ("pt_BR", "Zâmbia"),
        ("ro", "Zambia"),
        ("ru", "Замбия"),
        ("rw", "Zambiya"),
        ("sc", "Zàmbia"),
        ("sd", "زيمبيا"),
        ("si", "සැම\u{dca}බ\u{dd2}ය\u{dcf}ව"),
        ("sk", "Zambia"),
        ("sl", "Zambija"),
        ("so", "Saambiya"),
        ("sq", "Zambia"),
        ("sr", "Замбија"),
        ("sv", "Zambia"),
        ("sw", "Zambia"),
        ("ta", "ச\u{bbe}ம\u{bcd}பிய\u{bbe}"),
        ("te", "జ\u{c3e}ంబ\u{c3f}య\u{c3e}"),
        ("tg", "Замбия"),
        ("th", "แซมเบ\u{e35}ย"),
        ("ti", "ዛምቢያ"),
        ("tk", "Zambiýa"),
        ("tl", "Zambia"),
        ("tr", "Zambiya"),
        ("tt", "Замбиа"),
        ("ug", "زامبىيە"),
        ("uk", "Замбія"),
        ("ur", "زیمبیا"),
        ("uz", "Zambiya"),
        ("ve", "Zambia"),
        ("vi", "Xam-bi-a"),
        ("wa", "Zambeye"),
        ("wo", "Saambi"),
        ("xh", "Zambia"),
        ("yo", "Sámbíà"),
        ("zh_CN", "赞比亚"),
        ("zh_HK", "贊比亞"),
        ("zh_TW", "尚比亞"),
        ("zu", "IZambiya"),
    ];
    #[cfg(all(feature = "zm", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -13.133897;
        pub const LONGITUDE: f64 = 27.849332;
        pub const MAX_LATITUDE: f64 = -8.2032838;
        pub const MAX_LONGITUDE: f64 = 33.7090305;
        pub const MIN_LATITUDE: f64 = -18.0774179;
        pub const MIN_LONGITUDE: f64 = 21.999351;
        pub const NORTHEAST_LATITUDE: f64 = -8.2032838;
        pub const NORTHEAST_LONGITUDE: f64 = 33.7090305;
        pub const SOUTHWEST_LATITUDE: f64 = -18.0774179;
        pub const SOUTHWEST_LONGITUDE: f64 = 21.999351;
    }
}
#[cfg(all(feature = "zm", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -13.133897,
            longitude: 27.849332,
            max_latitude: -8.2032838,
            max_longitude: 33.7090305,
            min_latitude: -18.0774179,
            min_longitude: 21.999351,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: -8.2032838,
                    longitude: 33.7090305,
                },
                southwest: CountryGeoBound {
                    latitude: -18.0774179,
                    longitude: 21.999351,
                },
            },
        }
    }
}

#[cfg(all(feature = "zm", feature = "subdivisions"))]
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
                        country_alpha2: Alpha2::ZM,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-15.9454906), longitude: Some(23.3823545), max_latitude: Some(-13.709041), min_latitude: Some(-17.6392799), max_longitude: Some(25.52071), min_longitude: Some(21.999371)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Westelike Provinsie"), ("ar", "المحافظة الغربية، زامبيا"), ("be", "Заходняя правінцыя"), ("bg", "Западна провинция"), ("bn", "ওয\u{9bc}েস\u{9cd}ট\u{9be}র\u{9cd}ন প\u{9cd}রদেশ"), ("ccp", "Western"), ("ceb", "Western Province (lalawigan sa Zambia)"), ("da", "Vestprovinsen"), ("de", "Westprovinz"), ("el", "Δυτική Επαρχία"), ("en", "Western"), ("es", "Provincia del Oeste"), ("et", "Lääneprovints"), ("eu", "Mendebaldea"), ("fi", "Läntinen lääni"), ("fr", "Province Occidentale"), ("gu", "પશ\u{acd}ચિમી પ\u{acd}રા\u{a82}ત"), ("hi", "पश\u{94d}चिमी प\u{94d}रा\u{902}त"), ("hy", "Արևմտյան պրովինցիա"), ("id", "Provinsi Barat, Zambia"), ("it", "provincia Occidentale"), ("ja", "西部州 (ザンビア)"), ("ka", "დასავლეთის პროვინცია"), ("kn", "ಪಶ\u{ccd}ಚ\u{cbf}ಮ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "서부 주"), ("lt", "Vakarų provincija"), ("lv", "Rietumu province"), ("mr", "पश\u{94d}चिम प\u{94d}रा\u{902}त"), ("ms", "Western Province"), ("nb", "Western"), ("nl", "Western"), ("no", "Western"), ("pl", "Prowincja Zachodnia"), ("pt", "Província Ocidental"), ("ro", "Provincia de Vest"), ("ru", "Западная провинция"), ("si", "බටහ\u{dd2}ර පළ\u{dcf}ත"), ("sv", "Western"), ("sw", "Mkoa wa Magharibi"), ("ta", "மேற\u{bcd}கு ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "వ\u{c46}స\u{c4d}ట\u{c4d}రన\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเวสเท\u{e34}ร\u{e4c}น"), ("tr", "Batı Bölgesi"), ("uk", "Західна провінція"), ("ur", "مغربی صوبہ، زیمبیا"), ("vi", "Tỉnh Phía Tây"), ("zh", "西方省")]),
                        unofficial_name_list: ["Western"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::ZM,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-14.3112263), longitude: Some(28.299435), max_latitude: Some(-12.0036589), min_latitude: Some(-15.7121141), max_longitude: Some(31.4494591), min_longitude: Some(25.350383)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sentrale Provinsie (Zambië)"), ("ar", "المحافظة الوسطى، زامبيا"), ("be", "Цэнтральная правінцыя, Замбія"), ("bg", "Централна провинция (Замбия)"), ("bn", "সেন\u{9cd}ট\u{9cd}র\u{9be}ল প\u{9cd}রদেশ"), ("ca", "Província Central (Zàmbia)"), ("ccp", "Central"), ("ceb", "Central Province"), ("cs", "Centrální provincie (Zambie)"), ("da", "Central Province"), ("de", "Zentralprovinz"), ("el", "Κεντρική Επαρχία (Ζάμπια)"), ("en", "Central"), ("es", "Provincia Central (Zambia)"), ("et", "Keskprovints"), ("eu", "Erdialdea (Zambia)"), ("fi", "Central Province"), ("fr", "Province Centrale"), ("gu", "સ\u{ac7}ન\u{acd}ટ\u{acd}રલ પ\u{acd}રા\u{a82}ત"), ("hi", "क\u{947}\u{902}द\u{94d}रीय प\u{94d}रा\u{902}त"), ("id", "Provinsi Central"), ("it", "provincia Centrale"), ("ja", "中央州 (ザンビア)"), ("ka", "ცენტრალური პროვინცია"), ("kn", "ಮಧ\u{ccd}ಯ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "중부 주 (잠비아)"), ("lt", "Centrinė provincija (Zambija)"), ("lv", "Centrālā province"), ("mr", "मध\u{94d}यवर\u{94d}ती प\u{94d}रा\u{902}त"), ("ms", "Central Province"), ("nb", "Central"), ("nl", "Central"), ("no", "Central"), ("pl", "Prowincja Centralna (Zambia)"), ("pt", "Província Central (Zâmbia)"), ("ro", "Provincia Centrală, Zambia"), ("ru", "Центральная провинция"), ("si", "මධ\u{dca}\u{200d}යම පළ\u{dcf}ත"), ("sv", "Central (Zambia)"), ("sw", "Mkoa wa Kati (Zambia)"), ("ta", "சென\u{bcd}ட\u{bcd}ரல\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c46}ంట\u{c4d}రల\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เซ\u{e47}นทร\u{e31}ล โพว\u{e34}\u{e49}น"), ("tr", "Merkez Bölgesi"), ("uk", "Центральна провінція"), ("ur", "وسطی صوبہ، زیمبیا"), ("vi", "Tỉnh Miền Trung"), ("zh", "中央省 (赞比亚)")]),
                        unofficial_name_list: ["Central"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::ZM,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-13.8056187), longitude: Some(31.99280779999999), max_latitude: Some(-10.3251301), min_latitude: Some(-14.997233), max_longitude: Some(33.6895011), min_longitude: Some(29.98652989999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Oostelike Provinsie"), ("ar", "المحافظة الشرقية، زامبيا"), ("be", "Усходняя правінцыя"), ("bg", "Източна провинция"), ("bn", "ইস\u{9cd}ট\u{9be}র\u{9cd}ন প\u{9cd}রদেশ"), ("ccp", "𑄛\u{1112a}𑄇\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬"), ("ceb", "Eastern Province (lalawigan sa Zambia)"), ("da", "Eastern Province"), ("de", "Ostprovinz"), ("el", "Ανατολική Επαρχία"), ("en", "Eastern"), ("es", "Provincia del Este"), ("et", "Idaprovints"), ("fi", "Itäinen maakunta"), ("fr", "Province orientale"), ("gu", "પ\u{ac2}ર\u{acd}વીય પ\u{acd}રા\u{a82}ત"), ("hi", "प\u{942}र\u{94d}वी प\u{94d}रा\u{902}त"), ("id", "Provinsi Timur, Zambia"), ("it", "Zambia orientale"), ("ja", "東部州 (ザンビア)"), ("ka", "აღმოსავლეთის პროვინცია"), ("kn", "ಪ\u{cc2}ರ\u{ccd}ವ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "동부 주"), ("lt", "Rytų provincija"), ("lv", "Austrumu province"), ("mr", "प\u{942}र\u{94d}व प\u{94d}रा\u{902}त"), ("ms", "Eastern Province"), ("nb", "Eastern"), ("nl", "Eastern"), ("no", "Eastern"), ("pl", "Prowincja Wschodnia"), ("pt", "Província Oriental"), ("ro", "Provincia de Est"), ("ru", "Восточная Замбия"), ("si", "නැගෙනහ\u{dd2}ර පළ\u{dcf}ත"), ("sv", "Eastern"), ("sw", "Mkoa wa Mashariki"), ("ta", "கிழக\u{bcd}கு ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "తూర\u{c4d}పు ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "อ\u{e35}สเท\u{e34}ร\u{e4c}น โพว\u{e34}\u{e49}น"), ("tr", "Doğu Bölgesi"), ("uk", "Східна провінція"), ("ur", "مشرقی صوبہ، زیمبیا"), ("vi", "Tỉnh Phía Đông"), ("zh", "东方省")]),
                        unofficial_name_list: ["Eastern"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::ZM,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-11.564831), longitude: Some(29.0459927), max_latitude: Some(-8.375692899999999), min_latitude: Some(-12.471966), max_longitude: Some(30.46612799999999), min_longitude: Some(28.3809699)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Luapula"), ("ar", "لوابولا"), ("be", "Правінцыя Луапула"), ("bg", "Луапула"), ("bn", "ল\u{9c1}য\u{9bc}\u{9be}প\u{9c1}ল\u{9be} প\u{9cd}রদেশ"), ("ca", "província de Luapula"), ("ccp", "𑄣\u{1112a}𑄠𑄛\u{1112a}𑄣"), ("ceb", "Luapula Province"), ("da", "Luapula"), ("de", "Luapula"), ("el", "Λουαπούλα"), ("en", "Luapula"), ("es", "Provincia de Luapula"), ("et", "Luapula provints"), ("fi", "Luapulan provinssi"), ("fr", "Province de Luapula"), ("gu", "લ\u{acd}ય\u{ac1}પ\u{ac1}લા પ\u{acd}રા\u{a82}ત"), ("hi", "ल\u{941}आपला प\u{94d}रा\u{902}त"), ("id", "Provinsi Luapula"), ("it", "provincia di Luapula"), ("ja", "ルアプラ州"), ("ka", "ლუაპულის პროვინცია"), ("kn", "ಲುಪುಪುಲಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "루아풀라 주"), ("lt", "Luapulos provincija"), ("lv", "Luapalas province"), ("mr", "ल\u{941}आपला प\u{94d}रा\u{902}त"), ("ms", "Luapula Province"), ("nb", "Luapula"), ("nl", "Luapula"), ("no", "Luapula"), ("pl", "Prowincja Luapula"), ("pt", "Luapula"), ("ro", "Provincia Luapula, Zambia"), ("ru", "Луапула"), ("si", "ල\u{dd4}ආප\u{dd4}ල\u{dcf} පළ\u{dcf}ත"), ("sv", "Luapula"), ("sw", "Mkoa wa Luapula"), ("ta", "லுப\u{bcd}புல ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "లువ\u{c3e}పుల\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดล\u{e31}วป\u{e39}ลา"), ("tr", "Luapula Bölgesi"), ("uk", "Луапула"), ("ur", "لواپولا صوبہ"), ("vi", "Tỉnh Luapula"), ("zh", "卢阿普拉省")]),
                        unofficial_name_list: ["Luapula"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::ZM,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-9.7670177), longitude: Some(30.8958242), max_latitude: Some(-8.251887), min_latitude: Some(-13.4275721), max_longitude: Some(33.705704), min_longitude: Some(29.10338)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Noordelike Provinsie"), ("ar", "المحافظة الشمالية، زامبيا"), ("be", "Паўночная правінцыя, Замбія"), ("bg", "Северна провинция"), ("bn", "নর\u{9cd}দ\u{9be}ন প\u{9cd}রদেশ"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬"), ("ceb", "Northern Province"), ("da", "Northern Province"), ("de", "Nordprovinz"), ("el", "Βόρεια Επαρχία"), ("en", "Northern"), ("es", "Provincia del Norte"), ("fi", "Pohjoinen lääni"), ("fr", "Province Septentrionale"), ("gu", "નોર\u{acd}ધન પ\u{acd}રા\u{a82}ત"), ("hi", "उत\u{94d}तरी प\u{94d}रा\u{902}त"), ("id", "Provinsi Utara, Zambia"), ("it", "provincia Settentrionale"), ("ja", "北部州 (ザンビア)"), ("ka", "ჩრდილოეთის პროვინცია"), ("kn", "ಉತ\u{ccd}ತರ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "북부 주"), ("lt", "Šiaurės provincija"), ("lv", "Ziemeļu province"), ("mr", "उत\u{94d}तर प\u{94d}रा\u{902}त"), ("ms", "Northern Province"), ("nb", "Northern"), ("nl", "Northern"), ("no", "Northern"), ("pl", "Prowincja Północna"), ("pt", "Província do Norte"), ("ro", "Provincia de Nord, Zambia"), ("ru", "Северная провинция"), ("si", "උත\u{dd4}ර\u{dd4} පළ\u{dcf}ත"), ("sv", "Northern"), ("sw", "Mkoa wa Kaskazini"), ("ta", "வடக\u{bcd}கு ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఉత\u{c4d}తర ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดนอร\u{e4c}ท เท\u{e34}ร\u{e4c}น"), ("tr", "Kuzey Bölgesi"), ("uk", "Північна провінція"), ("ur", "شمالی صوبہ، زیمبیا"), ("vi", "Tỉnh Phía Bắc"), ("zh", "北方省")]),
                        unofficial_name_list: ["Northern"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::ZM,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-13.0050258), longitude: Some(24.9042208), max_latitude: Some(-10.885152), min_latitude: Some(-14.72759), max_longitude: Some(27.6016581), min_longitude: Some(21.999899)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Noordwestelike Provinsie"), ("ar", "المحافظة الشمالية الغربية، زامبيا"), ("be", "Паўночна-Заходняя правінцыя"), ("bg", "Северозападна провинция"), ("bn", "নর\u{9cd}থ-ওয\u{9bc}েস\u{9cd}ট\u{9be}র\u{9cd}ন প\u{9cd}রদেশ"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134}-𑄛\u{11127}𑄏\u{11128}𑄟\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬"), ("ceb", "North-Western Province"), ("da", "Northwestern Province"), ("de", "Nordwestprovinz"), ("el", "Βορειοδυτική Επαρχία"), ("en", "North-Western"), ("es", "Provincia del Noroeste"), ("et", "Loodeprovints"), ("fi", "Luoteisprovinssi"), ("fr", "Province Nord-Occidentale"), ("gu", "ઉત\u{acd}તરપશ\u{acd}ચિમી પ\u{acd}રા\u{a82}ત"), ("hi", "उत\u{94d}तर पश\u{94d}चिमी प\u{94d}रा\u{902}त, जाम\u{94d}बिया"), ("id", "Provinsi Barat Laut, Zambia"), ("it", "provincia Nord-Occidentale"), ("ja", "北西州"), ("ka", "ჩრდილო-დასავლეთის პროვინცია"), ("kn", "ವಾಯವ\u{ccd}ಯ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "북서부 주"), ("lt", "Šiaurės vakarų provincija"), ("lv", "Ziemeļrietumu province"), ("mr", "नॉर\u{94d}थव\u{947}स\u{94d}टर\u{94d}न प\u{94d}रा\u{902}त"), ("ms", "Northwestern Province"), ("nb", "North-Western"), ("nl", "North-Western"), ("no", "North-Western"), ("pl", "Prowincja Północno-Zachodnia"), ("pt", "Noroeste"), ("ro", "Provincia de Nord-Vest"), ("ru", "Северо-Западная провинция"), ("si", "වයඹ පළ\u{dcf}ත"), ("sv", "North-Western"), ("sw", "Mkoa wa Kaskazini-Magharibi"), ("ta", "வடக\u{bcd}கு தவெஸ\u{bcd}டேர\u{bcd}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "వ\u{c3e}యవ\u{c4d}య ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดนอร\u{e4c}ทเวสเท\u{e34}นส\u{e4c}"), ("tr", "Kuzeybatı Bölgesi"), ("uk", "Північно-Західна провінція"), ("ur", "شمال مغربی صوبہ، زیمبیا"), ("vi", "Tỉnh Tây Bắc"), ("yue", "西北省"), ("yue_Hans", "西北省"), ("zh", "西北省")]),
                        unofficial_name_list: ["North-Western"].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::ZM,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-16.9620634), longitude: Some(26.419389), max_latitude: Some(-15.293982), min_latitude: Some(-18.079473), max_longitude: Some(28.912453), min_longitude: Some(24.9715499)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Suidelike Provinsie"), ("ar", "المحافظة الجنوبية، زامبيا"), ("be", "Паўднёвая правінцыя"), ("bg", "Южна провинция"), ("bn", "স\u{9be}উদ\u{9be}র\u{9cd}ন প\u{9cd}রদেশ"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬"), ("ceb", "Southern Province"), ("da", "Southern Province"), ("de", "Südprovinz"), ("el", "Νότια Επαρχία"), ("en", "Southern"), ("es", "Provincia del Sur"), ("et", "Lõunaprovints"), ("eu", "Hegoaldea"), ("fi", "Eteläinen lääni"), ("fr", "Province Méridionale"), ("gu", "દક\u{acd}ષિણી પ\u{acd}રા\u{a82}ત"), ("hi", "दक\u{94d}षिणी प\u{94d}रा\u{902}त"), ("id", "Provinsi Selatan, Zambia"), ("it", "provincia Meridionale"), ("ja", "南部州 (ザンビア)"), ("ka", "სამხრეთის პროვინცია"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "남부 주"), ("lt", "Pietų provincija"), ("lv", "Dienvidu province"), ("mr", "दक\u{94d}षिण प\u{94d}रा\u{902}त"), ("ms", "Southern Province"), ("nb", "Southern"), ("nl", "Southern"), ("no", "Southern"), ("pl", "Prowincja Południowa"), ("pt", "Província do Sul"), ("ro", "Provincia de Sud"), ("ru", "Южная провинция"), ("si", "දක\u{dd4}ණ\u{dd4} කල\u{dcf}පය"), ("sv", "Southern"), ("sw", "Mkoa wa Kusini"), ("ta", "தெற\u{bcd}கு ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "దక\u{c4d}ష\u{c3f}ణ ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เซร\u{e4c}าเท\u{e34}น โพว\u{e34}\u{e49}น"), ("tr", "Güney Bölgesi"), ("uk", "Південна провінція"), ("ur", "جنوبی صوبہ، زیمبیا"), ("vi", "Tỉnh Phía Nam"), ("zh", "南部省")]),
                        unofficial_name_list: ["Southern"].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "08",
                        country_alpha2: Alpha2::ZM,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-13.0570073), longitude: Some(27.5495846), max_latitude: Some(-12.218456), min_latitude: Some(-13.9245529), max_longitude: Some(29.0201341), min_longitude: Some(26.801156)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Koperstreek"), ("ar", "كوبربيلت"), ("be", "Правінцыя Капербелт"), ("bg", "Копърбелт"), ("bn", "কপ\u{9be}রবেল\u{9be}ট প\u{9cd}রদেশ"), ("ca", "província de Copperbelt"), ("ccp", "𑄇\u{1112e}𑄛𑄬𑄢\u{11134}𑄝𑄬𑄣\u{11134}𑄑\u{11134}"), ("ceb", "Copperbelt Province"), ("cs", "Copperbelt"), ("da", "Copperbelt Province"), ("de", "Copperbelt"), ("el", "Κόππερμπελτ"), ("en", "Copperbelt"), ("es", "Provincia de Copperbelt (Zambia)"), ("et", "Copperbelt"), ("fa", "استان کوپربلت"), ("fi", "Copperbeltin provinssi"), ("fr", "Copperbelt"), ("gu", "કૉપરબ\u{ac7}લ\u{acd}ટ પ\u{acd}રા\u{a82}ત"), ("hi", "कॉपरब\u{947}ल\u{94d}ट प\u{94d}रा\u{902}त"), ("id", "Provinsi Copperbelt"), ("it", "provincia di Copperbelt"), ("ja", "カッパーベルト州"), ("ka", "კოპერბელტის პროვინცია"), ("kn", "ಕಾಪರ\u{ccd}ಬ\u{cc6}ಲ\u{ccd}ಟ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "코퍼벨트 주"), ("lt", "Vario Juostos provincija"), ("lv", "Koperbeltas province"), ("mr", "कॉपरब\u{947}ल\u{94d}ट प\u{94d}रा\u{902}त"), ("ms", "Copperbelt Province"), ("nb", "Copperbelt"), ("nl", "Copperbelt"), ("no", "Copperbelt"), ("pl", "Prowincja Pasa Miedzionośnego"), ("pt", "Copperbelt"), ("ro", "Provincia Copperbelt"), ("ru", "Коппербелт"), ("si", "කොපර\u{dca}බෙල\u{dca}ට\u{dca} පළ\u{dcf}ත"), ("sv", "Copperbelt"), ("sw", "Mkoa wa Copperbelt"), ("ta", "க\u{bbe}ப\u{bcd}பெற\u{bcd}பெல\u{bcd}ட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}పర\u{c4d} బ\u{c46}ల\u{c4d}ట\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเปอเบลท\u{e4c}"), ("tr", "Copperbelt Bölgesi"), ("uk", "Коппербелт"), ("ur", "کاپربیلٹ صوبہ"), ("vi", "Tỉnh Copperbelt"), ("zh", "铜带省")]),
                        unofficial_name_list: ["Copperbelt"].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "09",
                        country_alpha2: Alpha2::ZM,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-15.416667), longitude: Some(28.283333), max_latitude: Some(-15.2992532), min_latitude: Some(-15.5344065), max_longitude: Some(28.4887506), min_longitude: Some(28.1987555)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Lusaka"), ("ar", "لوساكا"), ("be", "Правінцыя Лусака"), ("bg", "Лусака"), ("bn", "ল\u{9c1}ক\u{9be}ক\u{9be} প\u{9cd}রদেশ"), ("ccp", "𑄣\u{1112a}𑄥𑄇"), ("ceb", "Lusaka Province"), ("da", "Lusaka Province"), ("de", "Lusaka"), ("el", "Λουσάκα"), ("en", "Lusaka"), ("es", "Provincia de Lusaka"), ("fi", "Lusakan lääni"), ("fr", "Province de Lusaka"), ("gu", "લ\u{ac1}સાકા પ\u{acd}રા\u{a82}ત"), ("hi", "ल\u{941}साका प\u{94d}रा\u{902}त"), ("id", "Provinsi Lusaka"), ("it", "provincia di Lusaka"), ("ja", "ルサカ州"), ("ka", "ლუსაკის პროვინცია"), ("kn", "ಲುಸಾಕಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "루사카 주"), ("lt", "Lusakos provincija"), ("lv", "Lusakas province"), ("mr", "ल\u{941}साका प\u{94d}रा\u{902}त"), ("ms", "Lusaka Province"), ("nb", "Lusaka"), ("nl", "Lusaka"), ("no", "Lusaka"), ("pl", "Prowincja Lusaka"), ("pt", "Lusaka"), ("ro", "Provincia Lusaka"), ("ru", "Лусака"), ("si", "ල\u{dd4}සක\u{dcf} පළ\u{dcf}ත"), ("sv", "Lusaka"), ("sw", "Mkoa wa Lusaka"), ("ta", "லூச\u{bbe}க ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "లుస\u{c3e}క\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดล\u{e39}ซากา"), ("tr", "Lusaka Bölgesi"), ("uk", "Лусака"), ("ur", "لوساکا صوبہ"), ("vi", "Tỉnh Lusaka"), ("zh", "卢萨卡省")]),
                        unofficial_name_list: ["Lusaka"].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::ZM,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة موتشينغا"), ("bn", "মোসিঙ\u{9cd}গ\u{9be} প\u{9cd}রদেশ"), ("ccp", "𑄟\u{1112a}𑄌\u{11128}\u{11101}𑄉"), ("ceb", "Muchinga Province"), ("da", "Muchinga Province"), ("de", "Provinz Muchinga"), ("el", "Μουτσίνγκα"), ("en", "Muchinga"), ("es", "Provincia de Muchinga"), ("fi", "Muchingan lääni"), ("fr", "Province de Muchinga"), ("gu", "મચી\u{a82}ગા પ\u{acd}રા\u{a82}ત"), ("hi", "म\u{941}शि\u{902}गा प\u{94d}रा\u{902}त"), ("id", "Provinsi Muchinga"), ("it", "Provincia di Muchinga"), ("ja", "ムチンガ州"), ("ka", "მუჩინგის პროვინცია"), ("kn", "ಮುಚ\u{cbf}ಂಗ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "무칭가 주"), ("lt", "Mučingos provincija"), ("lv", "Mučingas province"), ("mr", "मचि\u{902}गा प\u{94d}रा\u{902}त"), ("ms", "Muchinga Province"), ("nb", "Muchinga"), ("nl", "Muchinga Province"), ("no", "Muchinga"), ("pl", "Prowincja Muchinga"), ("pt", "Província de Muchinga"), ("ru", "Мучинга"), ("si", "ම\u{dd4}ච\u{dd2}න\u{dca}ග\u{dcf} පළ\u{dcf}ත"), ("sv", "Muchinga"), ("ta", "முச\u{bcd}ச\u{bc0}ங\u{bcd}க ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ముచ\u{c3f}ంగ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดม\u{e39}ช\u{e34}นกา"), ("tr", "Muchinga Bölgesi"), ("uk", "Мучинга"), ("ur", "موچنگا صوبہ"), ("vi", "Tỉnh Muchinga"), ("zh", "穆欽加省")]),
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
#[cfg(feature = "zm")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::ZM,
        alpha3: Alpha3::ZMB,
        address_format: None,
        continent: Continent::Africa,
        country_code: 260,
        currency_code: "ZMW",
        gec: Some(GEC::ZA),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("ZAM"),
        iso_long_name: "The Republic of Zambia",
        iso_short_name: "Zambia",
        official_language_list: ["en"].to_vec(),
        spoken_language_list: ["en"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "0",
        nationality: Some("Zambian"),
        number: "894",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Africa),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternAfrica),
        un_locode: "ZM",
        unofficial_name_list: ["Zambia", "Sambia", "Zambie", "ザンビア"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Zambia"),
            ("af", "Zambië"),
            ("ak", "Zambia"),
            ("am", "ዛምቢያ"),
            ("an", "Zambia"),
            ("ar", "زامبيا"),
            ("as", "জ\u{9be}ম\u{9cd}বিয়\u{9be}"),
            ("ay", "Zambia"),
            ("az", "Zambiya"),
            ("ba", "Zambia"),
            ("be", "Замбія"),
            ("bg", "Замбия"),
            ("bi", "Zambia"),
            ("bn", "জ\u{9be}ম\u{9cd}বিয়\u{9be}"),
            ("bn_IN", "জ\u{9be}ম\u{9cd}বিয়\u{9be}"),
            ("br", "Zambia"),
            ("bs", "Zambija"),
            ("ca", "Zàmbia"),
            ("ce", "Замби"),
            ("ch", "Zambia"),
            ("cs", "Zambie"),
            ("cv", "Замби"),
            ("cy", "Zambia"),
            ("da", "Zambia"),
            ("de", "Sambia"),
            ("dv", "ޒ\u{7ac}މ\u{7b0}ބ\u{7a8}އ\u{7a7}"),
            ("dz", "ཛམ་བ\u{f72}་ཡ།"),
            ("ee", "Zambia"),
            ("el", "Ζάμπια"),
            ("en", "Zambia"),
            ("eo", "Zambio"),
            ("es", "Zambia"),
            ("et", "Sambia"),
            ("eu", "Zambia"),
            ("fa", "زامبیا"),
            ("ff", "Zambia"),
            ("fi", "Sambia"),
            ("fo", "Sambia"),
            ("fr", "Zambie"),
            ("fy", "Sambia"),
            ("ga", "An tSaimbia"),
            ("gl", "Zambia"),
            ("gn", "Zambia"),
            ("gu", "ઝા\u{a82}બિયા"),
            ("gv", "Yn Tambia"),
            ("ha", "Zambia"),
            ("he", "זמביה"),
            ("hi", "ज\u{93c}ाम\u{94d}बिया"),
            ("hr", "Zambija"),
            ("ht", "Zanbi"),
            ("hu", "Zambia"),
            ("hy", "Զամբիա"),
            ("ia", "Zambia"),
            ("id", "Zambia"),
            ("io", "Zambia"),
            ("is", "Sambía"),
            ("it", "Zambia"),
            ("iu", "Zambia"),
            ("ja", "ザンビア"),
            ("ka", "ზამბია"),
            ("ki", "Zambia"),
            ("kk", "Замбия"),
            ("kl", "Zambia"),
            ("km", "ហ\u{17d2}សា\u{17c6}ប\u{17ca}\u{17b8}"),
            ("kn", "ಝಾಂಬ\u{cbf}ಯಾ"),
            ("ko", "잠비아"),
            ("ku", "Zambiya"),
            ("kv", "Zambia"),
            ("kw", "Zambi"),
            ("ky", "Замбия"),
            ("lo", "Zambia"),
            ("lt", "Zambija"),
            ("lv", "Zambija"),
            ("mi", "Zambia"),
            ("mk", "Замбија"),
            ("ml", "സ\u{d3e}ംബിയ"),
            ("mn", "Замби"),
            ("mr", "झा\u{902}बिया"),
            ("ms", "Zambia"),
            ("mt", "Żambja"),
            (
                "my",
                "ဇမ\u{103a}ဘ\u{102e}ယာန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Zambia"),
            ("nb", "Zambia"),
            ("ne", "जाम\u{94d}बिया"),
            ("nl", "Zambia"),
            ("nn", "Zambia"),
            ("nv", "Zambia"),
            ("oc", "Zambia"),
            ("or", "ଜ\u{b3e}ମ\u{b4d}ବ\u{b3f}ୟ\u{b3e}"),
            ("pa", "ਜ਼\u{a48}ਬੀਆ"),
            ("pi", "जाम\u{94d}बिया"),
            ("pl", "Zambia"),
            ("ps", "زېمبيا"),
            ("pt", "Zâmbia"),
            ("pt_BR", "Zâmbia"),
            ("ro", "Zambia"),
            ("ru", "Замбия"),
            ("rw", "Zambiya"),
            ("sc", "Zàmbia"),
            ("sd", "زيمبيا"),
            ("si", "සැම\u{dca}බ\u{dd2}ය\u{dcf}ව"),
            ("sk", "Zambia"),
            ("sl", "Zambija"),
            ("so", "Saambiya"),
            ("sq", "Zambia"),
            ("sr", "Замбија"),
            ("sv", "Zambia"),
            ("sw", "Zambia"),
            ("ta", "ச\u{bbe}ம\u{bcd}பிய\u{bbe}"),
            ("te", "జ\u{c3e}ంబ\u{c3f}య\u{c3e}"),
            ("tg", "Замбия"),
            ("th", "แซมเบ\u{e35}ย"),
            ("ti", "ዛምቢያ"),
            ("tk", "Zambiýa"),
            ("tl", "Zambia"),
            ("tr", "Zambiya"),
            ("tt", "Замбиа"),
            ("ug", "زامبىيە"),
            ("uk", "Замбія"),
            ("ur", "زیمبیا"),
            ("uz", "Zambiya"),
            ("ve", "Zambia"),
            ("vi", "Xam-bi-a"),
            ("wa", "Zambeye"),
            ("wo", "Saambi"),
            ("xh", "Zambia"),
            ("yo", "Sámbíà"),
            ("zh_CN", "赞比亚"),
            ("zh_HK", "贊比亞"),
            ("zh_TW", "尚比亞"),
            ("zu", "IZambiya"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}