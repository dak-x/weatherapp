# Weather
### A command line tool to fetch weather data of a city. 
It is built on Rust and requires `cargo` *Rust package manager* to build.
Install `cargo` following the instructions at `https://doc.rust-lang.org/cargo/getting-started/installation.html`.


## Setup
### Step 1: Clone the repositry 
Open your terminal and navigate to a folder of your choice *Eg: `~/Documents`*. Clone the repo using : 

        git clone "https://github.com/dak-x/weatherapp" 
### Step 2: Get API Key 
Generate an api key at https://openweathermap.org/api and paste it into the `token.txt` file in the root directory.
### Step 3: Build
Build the bin by running :  

        ./build.sh
Alternatively you can set an Env Variable named `OPENWEATHERAPI = <Your Token>` and then rust 
        
        cargo build --release
        

## Using the App:
You will find the bin file at **`target/release/weather`**. Explore the app via the following command. Copy this binary to your `/bin` folder to call it direcly from your shell. 

        target/release/weather --help

Feel free to contribute !!
