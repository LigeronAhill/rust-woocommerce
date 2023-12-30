struct Auth {
    ck: String,
    cs: String,
}
struct ModelController {
    auth: Auth,
    base_url: String,
    client: reqwest::Client,
}
