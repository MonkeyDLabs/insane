// #![allow(clippy::missing_errors_doc)]
// #![allow(clippy::unnecessary_struct_initialization)]
// #![allow(clippy::unused_async)]
// use ocol_rs::prelude::*;
// // use serde::{Deserialize, Serialize};

use insane_http::error::Result;
use insane_http::prelude::*;
use insane_http::{context::HttpContext, format, routes::Routes};

pub async fn ping(State(_ctx): State<HttpContext>) -> Result<Response> {
    // load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

pub fn routes() -> Routes {
    Routes::new().prefix("ping_user").add("/", get(ping))
    // .add("/", post(add))
    // .add("/:id", get(get_one))
    // .add("/:id", delete(remove))
    // .add("/:id", post(update))
}
