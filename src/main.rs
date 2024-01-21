use reqwest::blocking;

fn main() {
    let response = blocking::get("http://naver.com/");
    let data = response.unwrap().text().unwrap();
    println!("{data}");
}
