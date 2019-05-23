#[macro_use]
extern crate serde_derive;
extern crate reqwest;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct Location {
    title: String,
    location_type: String,
    woeid: u32,
    latt_long: String,
}

type LocationCollection = Vec<Location>;

#[derive(Deserialize, Debug)]
struct ConsolidatedWeather {
    id: u64,
    weather_state_name: String,
    weather_state_abbr: String,
    wind_direction_compass: String,
    created: String,
    applicable_date: String,
    min_temp: f64,
    max_temp: f64,
    the_temp: f64,
    wind_speed: f64,
    wind_direction: f64,
    air_pressure: f64,
    humidity: u32,
    visibility: f64,
    predictability: u32,
}

#[derive(Deserialize, Debug)]
struct Weather {
    consolidated_weather: Vec<ConsolidatedWeather>,
    time: String,
    sun_rise: String,
    sun_set: String,
    timezone_name: String,
    parent: Location,
    title: String,
    location_type: String,
    woeid: u32,
    latt_long: String,
    timezone: String,
}


fn main() -> Result<(), Error> {
    // cargo run toronto
    let search = "san";

    let locations = get_locations(search)?;
    let mut forecasts = get_weather_for_locations(locations)?;
    while let Some(forecast) = forecasts.pop() {
        println!("{:?}", weather_report(forecast));
    }

    return Ok(());
}

fn temperature(temp : f64) -> String {
    return format!("{}°C", temp.round());
}

fn consolidated_weather_report(consolidated_weather : &ConsolidatedWeather) -> String {
    return format!("high of {high} and low of {low}, currently sitting at {current} with {state}",
                   high = temperature(consolidated_weather.max_temp),
                   low = temperature(consolidated_weather.min_temp),
                   current = temperature(consolidated_weather.the_temp),
                   state = consolidated_weather.weather_state_name,
                   );
}

fn weather_report(forecast : Weather) -> String {
    return format!("The weather today in {city} will have a {consolidated_weather}",
                   city = forecast.title,
                   consolidated_weather = consolidated_weather_report(&forecast.consolidated_weather[0]),
                   );
}

fn location_search_url(search : &str) -> String {
    return format!("https://www.metaweather.com/api/location/search/?query={}", search);
}

fn get_locations(search : &str) -> Result<LocationCollection, Error> {
    let url = location_search_url(search);
    return reqwest::get(&url)?.json();
}

fn get_weather_for_locations(locations : LocationCollection) -> Result<Vec<Weather>, Error> {
    return locations.into_iter().map(|location| get_weather_for_location(location)).collect();
}

fn location_weather_url(location : &Location) -> String {
    return format!("https://www.metaweather.com/api/location/{}?speed=k", location.woeid);
}

fn get_weather_for_location(location : Location) -> Result<Weather, Error> {
    let url = location_weather_url(&location);
    return reqwest::get(&url)?.json();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn weather_empty() -> Weather {
        return Weather {
            consolidated_weather: vec![],
            time: "".to_string(),
            sun_rise: "".to_string(),
            sun_set: "".to_string(),
            timezone_name: "".to_string(),
            parent: Location {
                title: "".to_string(),
                location_type: "".to_string(),
                woeid: 0,
                latt_long: "".to_string(),
            },
            title: "".to_string(),
            location_type: "".to_string(),
            woeid: 0,
            latt_long: "".to_string(),
            timezone: "".to_string(),
        };
    }

    fn weather_with_title(weather: Weather, title: &str) {
        return Weather {
            title: title.to_string(),
            ..weather
        };
    }

    fn weather_with_temperatures(weather: Weather, min_temp: f64, max_temp: f64, the_temp: f64, weather_state_name : &str) -> Weather {
        return Weather {
            consolidated_weather: vec![
                ConsolidatedWeather {
                    id: 0,
                    weather_state_name: weather_state_name.to_string(),
                    weather_state_abbr: "".to_string(),
                    wind_direction_compass: "".to_string(),
                    created: "".to_string(),
                    applicable_date: "".to_string(),
                    min_temp,
                    max_temp,
                    the_temp,
                    wind_speed: 0.0,
                    wind_direction: 0.0,
                    air_pressure: 0.0,
                    humidity: 0,
                    visibility: 0.0,
                    predictability: 0,
                },
            ],
            ..weather
        };
    }

    fn weather_builder(title: &str, min_temp: f64, max_temp: f64, the_temp: f64, weather_state_name : &str) -> Weather {
        return weather_with_temperatures(
            weather_with_title(
                weather_empty(),
                title,
                ),
            min_temp,
            max_temp,
            the_temp,
            weather_state_name,
            );
    }

    #[test]
    fn weather_report_renders_correct_data() {
        let weather = weather_builder("Toronto", 0.0, 100.0, 50.0, "Light rain");

        assert_eq!(
            weather_report(weather),
            "The weather today in Toronto will have a high of 100°C and low of 0°C, currently sitting at 50°C with Light rain",
        )

    }
}
