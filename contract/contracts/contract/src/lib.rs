#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String};

#[contracttype]
#[derive(Clone)]
pub struct WeatherData {
    pub location: String,
    pub temperature: i32,
    pub humidity: u32,
    pub condition: Symbol,
}

#[contract]
pub struct WeatherOracleContract;

#[contractimpl]
impl WeatherOracleContract {

    // Store weather data
    pub fn set_weather(
        env: Env,
        location: String,
        temperature: i32,
        humidity: u32,
        condition: Symbol,
    ) {
        let data = WeatherData {
            location: location.clone(),
            temperature,
            humidity,
            condition,
        };

        env.storage().instance().set(&location, &data);
    }

    // Get weather data
    pub fn get_weather(env: Env, location: String) -> Option<WeatherData> {
        env.storage().instance().get(&location)
    }
}