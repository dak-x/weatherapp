# Weather
### A command line tool to fetch weather data of a city. 
It is built on Rust and requires `cargo` *Rust package manager* to build.
Install `cargo` following the instructions at `https://doc.rust-lang.org/cargo/getting-started/installation.html`.


## Setup
### Step 1: Clone the repositry 
Open your terminal and navigate to a folder of your choice *Eg: `~/Documents`*

        git clone "https://github.com/dak-x/weatherapp" 
### Step 2: Get API Key 
Generate an api key at https://openweathermap.org/api and paste it into the `token.txt ` file in the root directory.
### Step 3: Build
Build the bin by running 

        ./build.sh
Alternatively you can append the following code in `src/lib.rs` 
        
        const APP_ID: &str = <Token Key generated>
and then run from your terminal 
        
        cargo build --release

## Using the App:
You will find the bin file @ **`target/release/weather`**. Explore the app via the following command. Copy this binary to your `/bin` folder to call it direcly from your shell. 

        target/release/weather --help

Feel free to contribute !!