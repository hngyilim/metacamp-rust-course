
/// Create Country enum with simple variants of UnitedStates, Canada,
/// UnitedKingdom, Germany, France, Japan, Australia, China, Brazil, SouthKorea,
/// Ireland, Spain, India, Switzerland. Each variant does not need to encapsulate
/// any data.
#[derive(PartialEq, Debug)]
pub enum Country {
    UnitedStates, 
    Canada,
    UnitedKingdom, 
    Germany, 
    France, 
    Japan, 
    Australia, 
    China, 
    Brazil, 
    SouthKorea,
    Ireland, 
    Spain, 
    India, 
    Switzerland
}

/// Similarly, create Continent enum with simple variants of NorthAmerica, Europe,
/// Asia, Oceania, SouthAmerica.
#[derive(PartialEq, Debug)]
pub enum Continent {
    NorthAmerica, 
    Europe,
    Asia, 
    Oceania, 
    SouthAmerica
}
use std::fmt;

/// allow continents to be converted to strings
impl std::fmt::Display for Continent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Continent::NorthAmerica => write!(f, "NorthAmerica"),
            Continent::Europe => write!(f, "Europe"),
            Continent::Asia => write!(f, "Asia"),
            Continent::Oceania => write!(f, "Oceania"),
            Continent::SouthAmerica => write!(f, "SouthAmerica"),
        }
    }
}


/// implement on Country
/// - define country_to_continent method which takes in a referenced self and return a
/// Continent
/// - match on self and return an appropriate Continent variant
impl Country {
    pub fn country_to_continent(&self) -> Continent {
        match self {
            Country::UnitedStates | Country::Canada => Continent::NorthAmerica,
            Country::UnitedKingdom | Country::Germany | Country::France | Country::Ireland | Country::Spain | Country::Switzerland => Continent::Europe,
            Country::Japan | Country::China | Country::SouthKorea | Country::India => Continent::Asia,
            Country::Australia => Continent::Oceania,
            Country::Brazil => Continent::SouthAmerica,
        }
    }
}

/// implement std::str::FromStr trait on Country
/// - define Err type as referenced static string
/// - Define from_str method which takes in a referenced string and returns a Result of
/// <Country, Country::Err>
/// - match on self and return an Ok encapsulating an appropriate Country variant
impl std::str::FromStr for Country {
    type Err = &'static str;
    fn from_str(country_string: &str) -> Result<Country, Self::Err> {
        match country_string {
            "USA" => Ok(Country::UnitedStates),
            "Canada" => Ok(Country::Canada),
            "UK" => Ok(Country::UnitedKingdom),
            "Germany" => Ok(Country::Germany),
            "France" => Ok(Country::France),
            "Japan" => Ok(Country::Japan),
            "Australia" => Ok(Country::Australia),
            "China" => Ok(Country::China),
            "Brazil" => Ok(Country::Brazil),
            "South Korea" => Ok(Country::SouthKorea),
            "Ireland" => Ok(Country::Ireland),
            "Spain" => Ok(Country::Spain),
            "India" => Ok(Country::India),
            "Switzerland" => Ok(Country::Switzerland),
            _ => Err("Invalid Country name"),
        }
    }
}
