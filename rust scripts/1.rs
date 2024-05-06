use std::fs::File;
use std::io::prelude::*;
use reqwest::blocking::get;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // URL of the file to download
    let url = "https://github.com/Bartosz8291/BartoszOS-Source-Code/archive/refs/heads/main.zip";
    
    // Perform the GET request to download the file
    let response = get(url)?;

    // Check if the request was successful
    if response.status().is_success() {
        // Extract the response body
        let body = response.bytes()?;
        
        // Create a new file to save the downloaded content
        let mut file = File::create("BartoszOS_Source_Code.zip")?;
        
        // Write the downloaded content to the file
        file.write_all(&body)?;
        
        println!("Download complete.");
    } else {
        // If the request was not successful, print an error message
        println!("Failed to download the file. Status code: {}", response.status());
    }

    Ok(())
}
