use Symbol;

pub fn from_table(table: u8, code: u8) -> Symbol {
  match (table, code) {
    (0x2f, 0x21) => Symbol::Sheriff,                       // /!   BB     Police, Sheriff
    (0x5c, 0x21) => Symbol::Emergency,                     // \!   OBO    EMERGENCY (and overlays)
    (0x2f, 0x24) => Symbol::Phone,                         // /$   BE     PHONE
    (0x5c, 0x24) => Symbol::BankOrAtm,                     // \$   OEO    Bank or ATM  (green box)
    (0x5c, 0x25) => Symbol::PowerPlant,                    // \%   OFO    Power Plant with overlay
    (0x2f, 0x26) => Symbol::HfGateway,                     // /&   BG     HF GATEway
    (0x2f, 0x27) => Symbol::SmallAircraft,                 // /'   BH     Small AIRCRAFT (SSID-11)
    (0x5c, 0x27) => Symbol::IncidentSite,                  // \'   OHO    Crash (& now Incident sites)
    (0x2f, 0x28) => Symbol::MobileSatelliteStation,        // /(   BI     Mobile Satellite Station
    (0x5c, 0x28) => Symbol::Cloudy,                        // \(   OIO    CLOUDY (other clouds w ovrly)
    (0x2f, 0x29) => Symbol::Handicapped,                   // /)   BJ     Wheelchair (handicapped)
    (0x5c, 0x29) => Symbol::Firenet,                       // \)   OJO    Firenet MEO, MODIS Earth Obs.
    (0x2f, 0x2a) => Symbol::Snowmobile,                    // /*   BK     SnowMobile
    (0x2f, 0x2b) => Symbol::RedCross,                      // /+   BL     Red Cross
    (0x5c, 0x2b) => Symbol::Church,                        // \+   OL     Church
    (0x2f, 0x2c) => Symbol::BoyScouts,                     // /,   BM     Boy Scouts
    (0x5c, 0x2c) => Symbol::GirlScouts,                    // \,   OM     Girl Scouts
    (0x2f, 0x2d) => Symbol::HouseQth,                      // /-   BN     House QTH (VHF)
    (0x5c, 0x2d) => Symbol::HouseHhf,                      // \-   ONO    House (H=HF) (O = Op Present)
    (0x2f, 0x2e) => Symbol::Cross,                         // /.   BO     X
    (0x5c, 0x2e) => Symbol::BigQuestionMark,               // \.   OO     Ambiguous (Big Question mark)
    (0x2f, 0x2f) => Symbol::RedDot,                        // //   BP     Red Dot
    (0x5c, 0x2f) => Symbol::Waypoint,                      // \/   OP     Waypoint Destination
    (0x5c, 0x30) => Symbol::Circle,                        // \0   A0#    CIRCLE (IRLP/Echolink/WIRES)
    (0x5c, 0x38) => Symbol::WiFi,                          // \8   A8O    802.11 or other network node
    (0x5c, 0x39) => Symbol::GasStation,                    // \9   A9     Gas Station (blue pump)
    (0x2f, 0x3a) => Symbol::Fire,                          // /:   MR     FIRE
    (0x2f, 0x3b) => Symbol::Campground,                    // /;   MS     Campground (Portable ops)
    (0x5c, 0x3b) => Symbol::Park,                          // \;   NSO    Park/Picnic + overlay events
    (0x2f, 0x3c) => Symbol::Motorcycle,                    // /<   MT     Motorcycle     (SSID-10)
    (0x5c, 0x3c) => Symbol::Advisory,                      // \<   NTO    ADVISORY (one WX flag)
    (0x2f, 0x3d) => Symbol::RailroadEngine,                // /=   MU     RAILROAD ENGINE
    (0x2f, 0x3e) => Symbol::Car,                           // />   MV     CAR            (SSID-9)
    (0x5c, 0x3e) => Symbol::OverlayedVehicles,             // \>   NV#    OVERLAYED CARs & Vehicles
    (0x2f, 0x3f) => Symbol::Server,                        // /?   MW     SERVER for Files
    (0x5c, 0x3f) => Symbol::InfoKiosk,                     // \?   NW     INFO Kiosk  (Blue box with ?)
    (0x2f, 0x40) => Symbol::Dot,                           // /@   MX     HC FUTURE predict (dot)
    (0x5c, 0x40) => Symbol::Huricane,                      // \@   NX     HURICANE/Trop-Storm
    (0x2f, 0x41) => Symbol::AidStation,                    // /A   PA     Aid Station
    (0x5c, 0x41) => Symbol::BoxOverlay,                    // \A   AA#    overlayBOX DTMF & RFID & XO
    (0x2f, 0x42) => Symbol::Bbs,                           // /B   PB     BBS or PBBS
    (0x2f, 0x43) => Symbol::Canoe,                         // /C   PC     Canoe
    (0x5c, 0x43) => Symbol::CoastGuard,                    // \C   AC     Coast Guard
    (0x5c, 0x44) => Symbol::Depots,                        // \D   ADO    DEPOTS (Drizzle ==> ' ovly D)
    (0x2f, 0x45) => Symbol::Eyeball,                       // /E   PE     EYEBALL (Events, etc!)
    (0x5c, 0x45) => Symbol::Smoke,                         // \E   AE     Smoke (& other vis codes)
    (0x2f, 0x46) => Symbol::Tractor,                       // /F   PF     Farm Vehicle (tractor)
    (0x2f, 0x47) => Symbol::GridSquare,                    // /G   PG     Grid Square (6 digit)
    (0x5c, 0x47) => Symbol::SnowShower,                    // \G   AG     AVAIL (Snow Shwr ==> I ovly S)
    (0x2f, 0x48) => Symbol::Hotel,                         // /H   PH     HOTEL (blue bed symbol)
    (0x5c, 0x48) => Symbol::Haze,                          // \H   AHO    \Haze (& Overlay Hazards)
    (0x2f, 0x49) => Symbol::NetworkStation,                // /I   PI     TcpIp on air network stn
    (0x5c, 0x49) => Symbol::RainShower,                    // \I   AI     Rain Shower
    (0x5c, 0x4a) => Symbol::Lightening,                    // \J   AJ     AVAIL (Lightening ==> I ovly L)
    (0x2f, 0x4b) => Symbol::School,                        // /K   PK     School
    (0x5c, 0x4b) => Symbol::KenwoodHt,                     // \K   AK     Kenwood HT (W)
    (0x2f, 0x4c) => Symbol::PcUser,                        // /L   PL     PC user (Jan 03)
    (0x5c, 0x4c) => Symbol::Lighthouse,                    // \L   AL     Lighthouse
    (0x2f, 0x4d) => Symbol::MacAprs,                       // /M   PM     MacAPRS
    (0x5c, 0x4d) => Symbol::Military,                      // \M   AMO    MARS (A=Army,N=Navy,F=AF)
    (0x2f, 0x4e) => Symbol::NtsStation,                    // /N   PN     NTS Station
    (0x5c, 0x4e) => Symbol::NavigationBuoy,                // \N   AN     Navigation Buoy
    (0x2f, 0x4f) => Symbol::Balloon,                       // /O   PO     BALLOON        (SSID-11)
    (0x5c, 0x4f) => Symbol::Rocket,                        // \O   AO     Overlay Balloon (Rocket = \O)
    (0x2f, 0x50) => Symbol::Police,                        // /P   PP     Police
    (0x5c, 0x50) => Symbol::Parking,                       // \P   AP     Parking
    (0x5c, 0x51) => Symbol::Quake,                         // \Q   AQ     QUAKE
    (0x2f, 0x52) => Symbol::RecVehicle,                    // /R   PR     REC. VEHICLE   (SSID-13)
    (0x5c, 0x52) => Symbol::Restaurant,                    // \R   ARO    Restaurant
    (0x2f, 0x53) => Symbol::Shuttle,                       // /S   PS     SHUTTLE
    (0x5c, 0x53) => Symbol::Satellite,                     // \S   AS     Satellite/Pacsat
    (0x2f, 0x54) => Symbol::Sstv,                          // /T   PT     SSTV
    (0x5c, 0x54) => Symbol::Thunderstorm,                  // \T   AT     Thunderstorm
    (0x2f, 0x55) => Symbol::Bus,                           // /U   PU     BUS            (SSID-2)
    (0x5c, 0x55) => Symbol::Sunny,                         // \U   AU     SUNNY
    (0x2f, 0x56) => Symbol::Atv,                           // /V   PV     ATV
    (0x5c, 0x56) => Symbol::VortacNavAid,                  // \V   AV     VORTAC Nav Aid
    (0x2f, 0x57) => Symbol::NationalWxServiceSite,         // /W   PW     National WX Service Site
    (0x5c, 0x57) => Symbol::NwsSite,                       // \W   AW#    # NWS site (NWS options)
    (0x2f, 0x58) => Symbol::Helo,                          // /X   PX     HELO           (SSID-6)
    (0x5c, 0x58) => Symbol::Pharmacy,                      // \X   AX     Pharmacy Rx (Apothicary)
    (0x2f, 0x59) => Symbol::Yacht,                         // /Y   PY     YACHT (sail)   (SSID-5)
    (0x5c, 0x59) => Symbol::RadiosAndDevices,              // \Y   AYO    Radios and devices
    (0x2f, 0x5a) => Symbol::WinAprs,                       // /Z   PZ     WinAPRS
    (0x2f, 0x5b) => Symbol::Person,                        // /[   HS     Human/Person   (SSID-7)
    (0x5c, 0x5b) => Symbol::CloudOrHumansWOvrly,           // \[   DSO    W.Cloud (& humans w Ovrly)
    (0x2f, 0x5c) => Symbol::Triangle,                      // /\   HT     TRIANGLE(DF station)
    (0x5c, 0x5c) => Symbol::GpsSymbol,                     // \\   DTO    New overlayable GPS symbol
    (0x2f, 0x5d) => Symbol::PostOffice,                    // /]   HU     MAIL/PostOffice(was PBBS)
    (0x2f, 0x5e) => Symbol::LargeAircraft,                 // /^   HV     LARGE AIRCRAFT
    (0x5c, 0x5e) => Symbol::OtherAircraft,                 // \^   DV#    other Aircraft ovrlys (2014)
    (0x2f, 0x5f) => Symbol::WeatherStation,                // /_   HW     WEATHER Station (blue)
    (0x5c, 0x5f) => Symbol::WxSite,                        // \_   DW#    # WX site (green digi)
    (0x2f, 0x60) => Symbol::DishAntenna,                   // /`   HX     Dish Antenna
    (0x5c, 0x60) => Symbol::Rain,                          // \`   DX     Rain (all types w ovrly)
    (0x2f, 0x61) => Symbol::Ambulance,                     // /a   LA     AMBULANCE     (SSID-1)
    (0x2f, 0x62) => Symbol::Bike,                          // /b   LB     BIKE          (SSID-4)
    (0x2f, 0x63) => Symbol::IncidentCommandPost,           // /c   LC     Incident Command Post
    (0x2f, 0x64) => Symbol::FireDept,                      // /d   LD     Fire dept
    (0x2f, 0x65) => Symbol::Horse,                         // /e   LE     HORSE (equestrian)
    (0x5c, 0x65) => Symbol::Sleet,                         // \e   SE     Sleet (& future ovrly codes)
    (0x2f, 0x66) => Symbol::FireTruck,                     // /f   LF     FIRE TRUCK    (SSID-3)
    (0x5c, 0x66) => Symbol::FunnelCloud,                   // \f   SF     Funnel Cloud
    (0x2f, 0x67) => Symbol::Glider,                        // /g   LG     Glider
    (0x5c, 0x67) => Symbol::Gale,                          // \g   SG     Gale Flags
    (0x2f, 0x68) => Symbol::Hospital,                      // /h   LH     HOSPITAL
    (0x5c, 0x68) => Symbol::Store,                         // \h   SHO    Store. or HAMFST Hh=HAM store
    (0x5c, 0x69) => Symbol::PointOfInterest,               // \i   SI#    BOX or points of Interest
    (0x2f, 0x6a) => Symbol::Jeep,                          // /j   LJ     JEEP          (SSID-12)
    (0x5c, 0x6a) => Symbol::Workzone,                      // \j   SJ     WorkZone (Steam Shovel)
    (0x2f, 0x6b) => Symbol::Truck,                         // /k   LK     TRUCK         (SSID-14)
    (0x5c, 0x6b) => Symbol::SpecialVehicle,                // \k   SKO    Special Vehicle SUV,ATV,4x4
    (0x2f, 0x6c) => Symbol::Laptop,                        // /l   LL     Laptop (Jan 03)  (Feb 07)
    (0x5c, 0x6c) => Symbol::Areas,                         // \l   SL     Areas      (box,circles,etc)
    (0x2f, 0x6d) => Symbol::MicERepeater,                  // /m   LM     Mic-E Repeater
    (0x5c, 0x6d) => Symbol::ValueSign,                     // \m   SM     Value Sign (3 digit display)
    (0x2f, 0x6e) => Symbol::Node,                          // /n   LN     Node (black bulls-eye)
    (0x5c, 0x6e) => Symbol::OverlayTriangle,               // \n   SN#    OVERLAY TRIANGLE
    (0x5c, 0x6f) => Symbol::SmallCircle,                   // \o   SO     small circle
    (0x2f, 0x70) => Symbol::Dog,                           // /p   LP     ROVER (puppy, or dog)
    (0x5c, 0x72) => Symbol::Restroom,                      // \r   SR     Restrooms
    (0x2f, 0x73) => Symbol::Boat,                          // /s   LS     SHIP (pwr boat)  (SSID-8)
    (0x5c, 0x73) => Symbol::OverlayBoat,                   // \s   SS#    OVERLAY SHIP/boats
    (0x2f, 0x74) => Symbol::TruckStop,                     // /t   LT     TRUCK STOP
    (0x5c, 0x74) => Symbol::Tornado,                       // \t   ST     Tornado
    (0x2f, 0x75) => Symbol::Truck18Wheeler,                // /u   LU     TRUCK (18 wheeler)
    (0x5c, 0x75) => Symbol::OverlayedTruck,                // \u   SU#    OVERLAYED TRUCK
    (0x2f, 0x76) => Symbol::Van,                           // /v   LV     VAN           (SSID-15)
    (0x5c, 0x76) => Symbol::OverlayedVan,                  // \v   SV#    OVERLAYED Van
    (0x2f, 0x77) => Symbol::WaterStation,                  // /w   LW     WATER station
    (0x5c, 0x77) => Symbol::Flooding,                      // \w   SWO    Flooding (Avalanches/Slides)
    (0x5c, 0x78) => Symbol::Wreck,                         // \x   SX     Wreck or Obstruction ->X<-
    (0x5c, 0x79) => Symbol::Skywarn,                       // \y   SY     Skywarn
    (0x5c, 0x7a) => Symbol::OverlayedShelter,              // \z   SZ#    OVERLAYED Shelter
    _ => Symbol::Other,        
  }
}

#[cfg(test)]
mod tests {
    use Symbol;
    use PRIMARY_SYMBOL_TABLE;
    use ALT_SYMBOL_TABLE;

    #[test]
    fn test_symbol_from_table() {
         assert_eq!( Symbol::from_table(PRIMARY_SYMBOL_TABLE, 0x3f), Symbol::Server );
         assert_eq!( Symbol::from_table(ALT_SYMBOL_TABLE, 0x3f), Symbol::InfoKiosk );
    }

    #[test]
    fn test_unknown_symbol() {
         assert_eq!( Symbol::from_table(ALT_SYMBOL_TABLE, 0xff), Symbol::Other );
    }    

    #[test]
    fn test_symbol_debug_format() {
         assert_eq!( format!("{:?}", Symbol::InfoKiosk ), "InfoKiosk" );
    }
}
