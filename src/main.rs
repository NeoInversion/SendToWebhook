fn read_to_string(string: &mut String) {
    let _bytes = std::io::stdin().read_line(string);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut url = String::new();

    println!("Enter webhook URL:");
    read_to_string(&mut url);

    loop {
        let mut input = String::new();
        println!("Enter message or command ('STW_EXIT' or 'STW_CHANGE')");
        read_to_string(&mut input);

        if input.trim_end().eq("STW_EXIT") {
            break;
        }
        else if input.trim_end().eq("STW_CHANGE") {
            println!("Enter webhook URL:");
            read_to_string(&mut url);
        }
        else {
            let json = serde_json::json!({
                "content": input,
            });
            let res = client.post(&url)
                .json(&json)
                .send()
                .await?;
                
            // Should probably make this more clear to the user some time,
            // since Discord returns 204 No Content on success (as it should!)
            println!("Received Status: {}", res.status());
        }
    }
    
    Ok(())
}