use error_chain::error_chain;
use std::io::copy;
use std::fs::File;
use  tempfile::Builder;

error_chain! {
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let target = "https://th.bing.com/th/id/R.c9b8484224b31d940bbdc93870ba74e4?rik=834k6gN0UrC9XQ&pid=ImgRaw&r=0.png";
    let response = reqwest::get(target).await?;
    let mut dest= {
        let alucard = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() {None} else {Some(name)})
        .unwrap_or("tmp.bin");

    println!("file to download: '{}'", alucard);
    let alucard = tmp_dir.path().join(alucard);
    println!("will be located under: '{:?}'", alucard);
    File::create(alucard)?
    };
    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())
}


