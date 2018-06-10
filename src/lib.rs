//! # Automatic Position Reporting System protocol
//!
//! ## What's APRS?
//! Automatic Packet Reporting System (APRS) is a system for real time digital communication 
//! over HAM radio. It is a packet communications protocol for disseminating live data to 
//! everyone on a network enabling radio amateurs to automatically display the positions of radio 
//! stations and other objects on maps. There are also other features such as weather reporting and 
//! messaging.
//! 
//! # What's in this crate?
//! This crate defines Rust data types, traits and symbols per 
//! [APRS v1.01 specification](http://www.aprs.org/doc/APRS101.PDF).
//! 
//! It does not provide any parsing or encoding behavior, there are
//! separate crates for this. The intent here is to enable interoperability 
//! between different implementations and let client code use them as drop-in 
//! replacements for each other.
//!
//! If you're looking for a compartible parser check out the 
//! [`fap`](https://crates.io/crates/aprs-parser) crate, which is a 
//! Rust wrapper around Tapio Aaltonen's 
//! [`libfap`](http://www.pakettiradio.net/libfap/) library.
//!
use std::borrow::Cow;
use std::time::SystemTime;

mod units;
pub use units::{Feet, Meters, Knots, MetersPerSecond, MilesPerHour, KilometersPerHour, 
  Degrees, Fahrenheits, Celsius, Radians};

// Latitude and logitude are technically also degrees, 
// but in APRS packets latlng degrees and course degrees 
// use different ranges
pub struct Position{
    pub latitude: f32,
    pub longitude: f32,
    pub precision: Option<Feet>,
}
impl Position {
    pub fn from_latlng(lat: f32, lng: f32) -> Position {
        Position{
            latitude: lat,
            longitude: lng,
            precision: None,
        }
    }

    pub fn from_latlng_precise(lat: f32, lng: f32, precision: Feet) -> Position  {
        Position{
            latitude: lat,
            longitude: lng,
            precision: Some(precision),
        }
    }

    pub fn set_precision(&mut self, precision : Feet) -> &mut Self {
      self.precision = Some(precision);
      self
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Symbol {
   Sheriff,                       // /! BB    Police, Sheriff
   Emergency,                     // \! OBO   EMERGENCY (and overlays)
   Phone,                         // /$ BE    PHONE
   BankOrAtm,                     // \$ OEO   Bank or ATM  (green box)
   PowerPlant,                    // \% OFO   Power Plant with overlay
   HfGateway,                     // /& BG    HF GATEway
   SmallAircraft,                 // /' BH    Small AIRCRAFT (SSID-11)
   IncidentSite,                  // \' OHO   Crash (& now Incident sites)
   MobileSatelliteStation,        // /( BI    Mobile Satellite Station
   Cloudy,                        // \( OIO   CLOUDY (other clouds w ovrly)
   Handicapped,                   // /) BJ    Wheelchair (handicapped)
   Firenet,                       // \) OJO   Firenet MEO, MODIS Earth Obs.
   Snowmobile,                    // /* BK    SnowMobile
   RedCross,                      // /+ BL    Red Cross
   Church,                        // \+ OL    Church
   BoyScouts,                     // /, BM    Boy Scouts
   GirlScouts,                    // \, OM    Girl Scouts
   HouseQth,                      // /- BN    House QTH (VHF)
   HouseHhf,                      // \- ONO   House (H=HF) (O = Op Present)
   Cross,                         // /. BO    X
   BigQuestionMark,               // \. OO    Ambiguous (Big Question mark)
   RedDot,                        // // BP    Red Dot
   Waypoint,                      // \/ OP    Waypoint Destination
   Circle,                        // \0 A0#   CIRCLE (IRLP/Echolink/WIRES)
   WiFi,                          // \8 A8O   802.11 or other network node
   GasStation,                    // \9 A9    Gas Station (blue pump)
   Fire,                          // /: MR    FIRE
   Campground,                    // /; MS    Campground (Portable ops)
   Park,                          // \; NSO   Park/Picnic + overlay events
   Motorcycle,                    // /< MT    Motorcycle     (SSID-10)
   Advisory,                      // \< NTO   ADVISORY (one WX flag)
   RailroadEngine,                // /= MU    RAILROAD ENGINE
   Car,                           // /> MV    CAR            (SSID-9)
   OverlayedVehicles,             // \> NV#   OVERLAYED CARs & Vehicles
   Server,                        // /? MW    SERVER for Files
   InfoKiosk,                     // \? NW    INFO Kiosk  (Blue box with ?)
   Dot,                           // /@ MX    HC FUTURE predict (dot)
   Huricane,                      // \@ NX    HURICANE/Trop-Storm
   AidStation,                    // /A PA    Aid Station
   BoxOverlay,                    // \A AA#   overlayBOX DTMF & RFID & XO
   Bbs,                           // /B PB    BBS or PBBS
   Canoe,                         // /C PC    Canoe
   CoastGuard,                    // \C AC    Coast Guard
   Depots,                        // \D ADO   DEPOTS (Drizzle ==> ' ovly D)
   Eyeball,                       // /E PE    EYEBALL (Events, etc!)
   Smoke,                         // \E AE    Smoke (& other vis codes)
   Tractor,                       // /F PF    Farm Vehicle (tractor)
   GridSquare,                    // /G PG    Grid Square (6 digit)
   SnowShower,                    // \G AG    AVAIL (Snow Shwr ==> I ovly S)
   Hotel,                         // /H PH    HOTEL (blue bed symbol)
   Haze,                          // \H AHO   \Haze (& Overlay Hazards)
   NetworkStation,                // /I PI    TcpIp on air network stn
   RainShower,                    // \I AI    Rain Shower
   Lightening,                    // \J AJ    AVAIL (Lightening ==> I ovly L)
   School,                        // /K PK    School
   KenwoodHt,                     // \K AK    Kenwood HT (W)
   PcUser,                        // /L PL    PC user (Jan 03)
   Lighthouse,                    // \L AL    Lighthouse
   MacAprs,                       // /M PM    MacAPRS
   Military,                      // \M AMO   MARS (A=Army,N=Navy,F=AF)
   NtsStation,                    // /N PN    NTS Station
   NavigationBuoy,                // \N AN    Navigation Buoy
   Balloon,                       // /O PO    BALLOON        (SSID-11)
   Rocket,                        // \O AO    Overlay Balloon (Rocket = \O)
   Police,                        // /P PP    Police
   Parking,                       // \P AP    Parking
   Quake,                         // \Q AQ    QUAKE
   RecVehicle,                    // /R PR    REC. VEHICLE   (SSID-13)
   Restaurant,                    // \R ARO   Restaurant
   Shuttle,                       // /S PS    SHUTTLE
   Satellite,                     // \S AS    Satellite/Pacsat
   Sstv,                          // /T PT    SSTV
   Thunderstorm,                  // \T AT    Thunderstorm
   Bus,                           // /U PU    BUS            (SSID-2)
   Sunny,                         // \U AU    SUNNY
   Atv,                           // /V PV    ATV
   VortacNavAid,                  // \V AV    VORTAC Nav Aid
   NationalWxServiceSite,         // /W PW    National WX Service Site
   NwsSite,                       // \W AW#   # NWS site (NWS options)
   Helo,                          // /X PX    HELO           (SSID-6)
   Pharmacy,                      // \X AX    Pharmacy Rx (Apothicary)
   Yacht,                         // /Y PY    YACHT (sail)   (SSID-5)
   RadiosAndDevices,              // \Y AYO   Radios and devices
   WinAprs,                       // /Z PZ    WinAPRS
   Person,                        // /[ HS    Human/Person   (SSID-7)
   CloudOrHumansWOvrly,           // \[ DSO   W.Cloud (& humans w Ovrly)
   Triangle,                      // /\ HT    TRIANGLE(DF station)
   GpsSymbol,                     // \\ DTO   New overlayable GPS symbol
   PostOffice,                    // /] HU    MAIL/PostOffice(was PBBS)
   LargeAircraft,                 // /^ HV    LARGE AIRCRAFT
   OtherAircraft,                 // \^ DV#   other Aircraft ovrlys (2014)
   WeatherStation,                // /_ HW    WEATHER Station (blue)
   WxSite,                        // \_ DW#   # WX site (green digi)
   DishAntenna,                   // /` HX    Dish Antenna
   Rain,                          // \` DX    Rain (all types w ovrly)
   Ambulance,                     // /a LA    AMBULANCE     (SSID-1)
   Bike,                          // /b LB    BIKE          (SSID-4)
   IncidentCommandPost,           // /c LC    Incident Command Post
   FireDept,                      // /d LD    Fire dept
   Horse,                         // /e LE    HORSE (equestrian)
   Sleet,                         // \e SE    Sleet (& future ovrly codes)
   FireTruck,                     // /f LF    FIRE TRUCK    (SSID-3)
   FunnelCloud,                   // \f SF    Funnel Cloud
   Glider,                        // /g LG    Glider
   Gale,                          // \g SG    Gale Flags
   Hospital,                      // /h LH    HOSPITAL
   Store,                         // \h SHO   Store. or HAMFST Hh=HAM store
   PointOfInterest,               // \i SI#   BOX or points of Interest
   Jeep,                          // /j LJ    JEEP          (SSID-12)
   Workzone,                      // \j SJ    WorkZone (Steam Shovel)
   Truck,                         // /k LK    TRUCK         (SSID-14)
   SpecialVehicle,                // \k SKO   Special Vehicle SUV,ATV,4x4
   Laptop,                        // /l LL    Laptop (Jan 03)  (Feb 07)
   Areas,                         // \l SL    Areas      (box,circles,etc)
   MicERepeater,                  // /m LM    Mic-E Repeater
   ValueSign,                     // \m SM    Value Sign (3 digit display)
   Node,                          // /n LN    Node (black bulls-eye)
   OverlayTriangle,               // \n SN#   OVERLAY TRIANGLE
   SmallCircle,                   // \o SO    small circle
   Dog,                           // /p LP    ROVER (puppy, or dog)
   Restroom,                      // \r SR    Restrooms
   Boat,                          // /s LS    SHIP (pwr boat)  (SSID-8)
   OverlayBoat,                   // \s SS#   OVERLAY SHIP/boats
   TruckStop,                     // /t LT    TRUCK STOP
   Tornado,                       // \t ST    Tornado
   Truck18Wheeler,                // /u LU    TRUCK (18 wheeler)
   OverlayedTruck,                // \u SU#   OVERLAYED TRUCK
   Van,                           // /v LV    VAN           (SSID-15)
   OverlayedVan,                  // \v SV#   OVERLAYED Van
   WaterStation,                  // /w LW    WATER station
   Flooding,                      // \w SWO   Flooding (Avalanches/Slides)
   Wreck,                         // \x SX    Wreck or Obstruction ->X<-
   Skywarn,                       // \y SY    Skywarn
   OverlayedShelter,              // \z SZ#   OVERLAYED Shelter
   Other,
}
pub const PRIMARY_SYMBOL_TABLE : u8 = 0x2f;
pub const ALT_SYMBOL_TABLE : u8 = 0x5c;
mod symbol;
impl Symbol {
    pub fn from_table(table: u8, code: u8) -> Symbol {
        symbol::from_table(table, code)
    }
}

pub trait Packet {
    fn source(&self) -> Cow<str>;
    fn symbol(&self) -> Symbol;
    fn timestamp(&self) -> Option<SystemTime>;
    fn destination(&self) -> Option<Cow<str>>;
    fn comment(&self) -> Option<Cow<str>>;
    fn position(&self) -> Option<Position>;
    fn speed(&self) -> Option<Knots>;
    fn course(&self) -> Option<Degrees>;
    fn altitude(&self) -> Option<Feet>;
    fn temperature(&self) -> Option<Fahrenheits>;
    fn wind_direction(&self) -> Option<Degrees>;
    fn wind_speed(&self) -> Option<Knots>;

    // fn message_id(&self) -> Option<&str>;
    // fn message_addressee(&self) -> Option<&str>;
    // fn message_text(&self) -> Option<&str>;
    // fn message_ack(&self) -> Option<bool>;
    // fn message_rjc(&self) -> Option<bool>;


    fn latitude(&self) -> Option<f32> {
        self.position().map(|v| v.latitude)
    }

    fn longitude(&self) -> Option<f32> {
        self.position().map(|v| v.longitude)
    }

    fn precision(&self) -> Option<Feet> {
        self.position().and_then(|v| v.precision)
    }
}

// Floating point asserts for tests
#[cfg(test)] #[macro_use] extern crate approx; 