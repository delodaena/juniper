use std::env;

use iron::prelude::*;
use juniper::{
    tests::fixtures::starwars::schema::{Database, Query},
    DefaultScalarValue, EmptyMutation, EmptySubscription,
};
use juniper_iron::{GraphQLHandler, GraphiQLHandler};
use logger::Logger;
use mount::Mount;

fn context_factory(_: &mut Request) -> IronResult<Database> {
    Ok(Database::new())
}

fn main() {
    let mut mount = Mount::new();

    let graphql_endpoint = <GraphQLHandler<_, _, _, _, _, DefaultScalarValue>>::new(
        context_factory,
        Query,
        EmptyMutation::<Database>::new(),
        EmptySubscription::<Database>::new(),
    );
    let graphiql_endpoint = GraphiQLHandler::new("/graphql", None);

    mount.mount("/", graphiql_endpoint);
    mount.mount("/graphql", graphql_endpoint);

    let (logger_before, logger_after) = Logger::new(None);

    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    let host = env::var("LISTEN").unwrap_or_else(|_| "0.0.0.0:8080".into());
    println!("GraphQL server started on {host}");
    Iron::new(chain).http(host.as_str()).unwrap();
}
