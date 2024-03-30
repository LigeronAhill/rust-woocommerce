use rust_woocommerce::{
    controllers::products::AttributeDTO, products::Product, ApiClient, BatchObject, Result,
};
#[tokio::main]
async fn main() -> Result<()> {
    let client = ApiClient::from_env()?;
    let medusa = client
        .retrieve::<Product>(rust_woocommerce::Entity::Product, 1307)
        .await?;
    let base_uri = "https://carpetyourlife.com";
    let search = "medusa";
    let search_results_re =
        regex::Regex::new(r"/en/find-your-carpet/(?<article>\w+)/(?<colour>\d{1,3})")?;
    let mut product_links = std::collections::HashSet::new();
    let mut colours = std::collections::HashSet::new();
    for page in 1..5 {
        let search_uri = format!("{base_uri}/en/search?q={search}&Page={page}");
        let body = reqwest::get(&search_uri).await?.text().await?;
        let dom = tl::parse(&body, tl::ParserOptions::default())?;
        let parser = dom.parser();
        let links = dom.query_selector("a[href]").unwrap();
        for link in links {
            let tag = link.get(parser).unwrap().as_tag().unwrap();
            let link_to_product = tag
                .attributes()
                .get("href")
                .flatten()
                .unwrap()
                .as_utf8_str();
            if search_results_re.is_match(&link_to_product) {
                product_links.insert(link_to_product.to_string());
            }
            if let Some(caps) = search_results_re.captures(&link_to_product) {
                let x = &caps["colour"];
                colours.insert(x.to_string());
            }
        }
    }
    let mut pic_links = Vec::new();
    let mut colour_with_link = Vec::new();
    for p_link in product_links {
        let uri = format!("{base_uri}{p_link}");
        let body = reqwest::get(&uri).await?.text().await?;
        let dom = tl::parse(&body, tl::ParserOptions::default())?;
        let parser = dom.parser();
        if let Some(element) = dom.get_element_by_id("application_0_PackshotImage") {
            if let Some(node) = element.get(parser) {
                if let Some(x) = node.as_tag() {
                    if let Some(y) = x.attributes().get("src").flatten() {
                        let s = y.as_utf8_str().replace("medium", "large");
                        pic_links.push(format!("{base_uri}{s}"));
                    }
                }
            }
        }
        if let Some(element) = dom.get_element_by_id("application_0_RoomshotImage") {
            if let Some(node) = element.get(parser) {
                if let Some(x) = node.as_tag() {
                    if let Some(y) = x.attributes().get("src").flatten() {
                        let s = y.as_utf8_str().replace("medium", "large");
                        pic_links.push(format!("{base_uri}{s}"));
                    }
                }
            }
        }
    }
    for col in colours {
        let ls = pic_links
            .iter()
            .filter(|l| l.contains(&col))
            .map(|l| l.to_owned())
            .collect::<Vec<String>>();
        colour_with_link.push(LinkWithColour {
            colour: col,
            link: ls,
        })
    }
    let mut created_prdcts = 0;
    for s in colour_with_link {
        if s.colour == "98" {
            continue;
        } else {
            let name = medusa.name.replace("98", &s.colour);
            let sku = medusa.sku.replace("98", &s.colour);
            let category = medusa.categories[0].id;
            let mut product_to_create = Product::create();
            product_to_create
                .name(name)
                .short_description(&medusa.short_description)
                .sku(sku)
                .regular_price(&medusa.regular_price)
                .manage_stock()
                .backorders(medusa.backorders.clone())
                .weight(&medusa.weight)
                .shipping_class(&medusa.shipping_class)
                .categories(category);
            for l in s.link {
                product_to_create.images(&l);
            }
            for in_attr in &medusa.attributes {
                let at = AttributeDTO::builder()
                    .id(in_attr.id)
                    .name(&in_attr.name)
                    .visible()
                    .options(in_attr.options.clone())
                    .build();
                product_to_create.attribute(at);
            }
            let product_to_create = product_to_create.build();
            let _created: Product = client
                .create(rust_woocommerce::Entity::Product, product_to_create)
                .await?;
            created_prdcts += 1;
        }
    }

    println!("created {} products", created_prdcts);
    // for l in pic_links {
    //     let resp = reqwest::get(&l).await?;
    //     let slice = l.split('/').collect::<Vec<&str>>();
    //     let r = slice.last().unwrap().split('_').collect::<Vec<&str>>();
    //     let x = r[1..].join("_");
    //     std::fs::create_dir_all(&search)?;
    //     let path = format!("{search}/{search}_{x}");
    //     let mut out = std::fs::File::create(&path)?;
    //     let mut content = std::io::Cursor::new(resp.bytes().await?);
    //     std::io::copy(&mut content, &mut out)?;
    // }
    Ok(())
}
#[derive(Debug)]
struct LinkWithColour {
    colour: String,
    link: Vec<String>,
}
