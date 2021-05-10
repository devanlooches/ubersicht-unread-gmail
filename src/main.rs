use std::future::Future;
use std::pin::Pin;
use yup_oauth2::authenticator_delegate::{DefaultInstalledFlowDelegate, InstalledFlowDelegate};

extern crate serde_json;
use serde_json::Value as JsonValue;

extern crate clap;
use clap::{App,Arg};

extern crate reqwest;

#[tokio::main]
async fn main() {
    // Read application secret from a file. Sometimes it's easier to compile it directly into
    // the binary. The clientsecret file contains JSON like `{"installed":{"client_id": ... }}`
    let matches = App::new("Gmail Unread Tracker")
        .version("0.0.1")
        .author("Devan Looches <devan.looches@gmail.com>")
        .about("Shows the number of unread emails you have")
        .arg(
            Arg::with_name("tokencache")
                .short("t")
                .long("tokencache")
                .value_name("FILE")
                .help("Sets custom token cache location")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("credentials")
                .short("c")
                .long("credentials")
                .value_name("FILE")
                .help("Sets custom credentials file location")
                .takes_value(true),
        ).get_matches();

    let tokencache_location = matches.value_of("tokencache").unwrap_or("unreadmail.widget/tokencache.json");
    let credentials_location = matches.value_of("credentials").unwrap_or("unreadmail.widget/credentials.json");
    let token = get_token(credentials_location.to_string(), tokencache_location.to_string()).await;

    let request_url_unread = format!(
        "https://gmail.googleapis.com/gmail/v1/users/me/messages?q={query}&access_token={token}",
        query = "is:unread",
        token = token
    );
    let response_text_unread = reqwest::get(request_url_unread)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let res_unread = serde_json::from_str(&response_text_unread);

    if res_unread.is_ok() {
        let p_unread: JsonValue = res_unread.unwrap();
        let num_of_unread = &p_unread["resultSizeEstimate"];
        println!("{}", num_of_unread);
    } else {
        println!("?");
    }

    let request_url_email = format!(
        "https://gmail.googleapis.com/gmail/v1/users/me/profile?access_token={token}",
        token = token
    );
    let response_text_email = reqwest::get(request_url_email)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let res_email = serde_json::from_str(&response_text_email);

    if res_email.is_ok() {
        let p_email: JsonValue = res_email.unwrap();
        let email_address = &p_email["emailAddress"];
        let email_address = email_address.to_string().replace("\"","");
        let email_address = &email_address[0..6];
        println!("{}", email_address);
    } else {
        println!("?");
    }

}

/// async function to be pinned by the `present_user_url` method of the trait
/// we use the existing `DefaultInstalledFlowDelegate::present_user_url` method as a fallback for
/// when the browser did not open for example, the user still see's the URL.
async fn browser_user_url(url: &str, need_code: bool) -> Result<String, String> {
    webbrowser::open(url).unwrap();
    let def_delegate = DefaultInstalledFlowDelegate;
    def_delegate.present_user_url(url, need_code).await
}

/// our custom delegate struct we will implement a flow delegate trait for:
/// in this case we will implement the `InstalledFlowDelegated` trait
#[derive(Copy, Clone, Debug)]
struct InstalledFlowBrowserDelegate;

/// here we implement only the present_user_url method with the added webbrowser opening
/// the other behaviour of the trait does not need to be changed.
impl InstalledFlowDelegate for InstalledFlowBrowserDelegate {
    /// the actual presenting of URL and browser opening happens in the function defined above here
    /// we only pin it
    fn present_user_url<'a>(
        &'a self,
        url: &'a str,
        need_code: bool,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + 'a>> {
        Box::pin(browser_user_url(url, need_code))
    }
}

async fn get_token(credentials_location: String, tokencache_location: String) -> String {
    // Put your client secret in the working directory!
    let sec = yup_oauth2::read_application_secret(credentials_location)
        .await
        .expect("client secret couldn't be read.");
    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
        sec,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk(tokencache_location)
    // use our custom flow delegate instead of default
    .flow_delegate(Box::new(InstalledFlowBrowserDelegate))
    .build()
    .await
    .expect("InstalledFlowAuthenticator failed to build");

    let scopes = &["https://www.googleapis.com/auth/gmail.readonly"];

    auth.token(scopes).await.unwrap().as_str().to_string()
}
