mod database;
mod graphql;
mod helpers;
mod model;

use warp::Filter;

#[tokio::main]
async fn main() {
    // Set logger with Fern
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                helpers::format::format_rfc3339(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_secs()
                ),
                record.level(),
                message
            ))
        })
        .level(if cfg!(debug_assertions) {
            log::LevelFilter::Trace
        } else {
            log::LevelFilter::Info
        })
        .level_for("hyper", log::LevelFilter::Error)
        .level_for("juniper_warp::graphiql_filter", log::LevelFilter::Warn)
        .level_for("warp_server", log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    // Read configuration file
    let config = helpers::config::read();

    // Start databases
    let memcached_pool = database::mem::MemPool {
        connection: database::mem::init(&config).unwrap(),
    };

    // Create a warp filter for GraphQL context
    let state = warp::any().map(move || graphql::user::Context {
        memcached: memcached_pool.clone(),
    });

    // Create a filter for the main GraphQL endpoint
    let graphql_filter = juniper_warp::make_graphql_filter(
        graphql::user::schema(),
        state.boxed(),
    );

    warp::serve(
        warp::any()
            .and(warp::options())
            .map(|| "OK")
            .or(warp::post()
                .and(warp::path("graphql").and(graphql_filter))
                .with(warp::log("warp_server"))),
    )
    .run(([127, 0, 0, 1], config.port))
    .await
}
