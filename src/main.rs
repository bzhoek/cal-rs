use yup_oauth2::ServiceAccountAuthenticator;

#[tokio::main]
async fn main() {
  let homedir = std::env::var("HOME").unwrap();
  let filepath = format!("{}/.dotfiles/jirest-5aa8b78efeef.json", homedir);
  let creds = yup_oauth2::read_service_account_key(filepath)
    .await
    .unwrap();
  let sa = ServiceAccountAuthenticator::builder(creds)
    .build()
    .await
    .unwrap();
  let scopes = &["https://www.googleapis.com/auth/pubsub"];

  let tok = sa.token(scopes).await.unwrap();
  println!("token is: {:?}", tok);
  let tok = sa.token(scopes).await.unwrap();
  println!("cached token is {:?} and should be identical", tok);
}