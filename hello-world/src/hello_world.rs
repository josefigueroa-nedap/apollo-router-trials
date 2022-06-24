use apollo_router::plugin::Plugin;
use apollo_router::register_plugin;
use apollo_router::services::ResponseBody;
use apollo_router::services::{ExecutionRequest, ExecutionResponse};
use apollo_router::services::{QueryPlannerRequest, QueryPlannerResponse};
use apollo_router::services::{RouterRequest, RouterResponse};
use apollo_router::services::{SubgraphRequest, SubgraphResponse};
use apollo_router::Request;
use apollo_router::Response;
use futures::stream::BoxStream;
use regex::Regex;
use schemars::JsonSchema;
use serde::Deserialize;
use tower::util::BoxService;
use tower::{BoxError, ServiceBuilder, ServiceExt};

#[derive(Debug)]
struct HelloWorld {
    #[allow(dead_code)]
    configuration: Conf,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
struct Conf {
    // Put your plugin configuration here. It will automatically be deserialized from JSON.
    name: String, // The name of the entity you'd like to say hello to
}

// This is a bare bones plugin that can be duplicated when creating your own.
#[async_trait::async_trait]
impl Plugin for HelloWorld {
    type Config = Conf;

    async fn new(configuration: Self::Config) -> Result<Self, BoxError> {
        Ok(HelloWorld { configuration })
    }

    fn router_service(
        &mut self,
        service: BoxService<
            RouterRequest,
            RouterResponse<BoxStream<'static, ResponseBody>>,
            BoxError,
        >,
    ) -> BoxService<RouterRequest, RouterResponse<BoxStream<'static, ResponseBody>>, BoxError> {
        // Say hello when our service is added to the router_service
        // stage of the router plugin pipeline.
        #[cfg(test)]
        println!("Hello {}", self.configuration.name);
        #[cfg(not(test))]
        tracing::info!("Hello {}", self.configuration.name);
        // Always use service builder to compose your plugins.
        // It provides off the shelf building blocks for your plugin.
        ServiceBuilder::new()
            .map_request(|request: RouterRequest|{

               let new_orig_request = request.originating_request.map(|req| {

                    let re        = Regex::new(r"(?P<node_call>\s?node[s]?\s?\(.+\)\s?\{)").unwrap();
                    let old_query = &req.query.as_ref().unwrap();
                    let caps      = re.captures(old_query);

                    let new_query = match caps {
                        Some(found) => {
                            println!("OPERATION NAME: {:?}", found.name("node_call").unwrap().as_str());
                            re.replace_all(old_query, "$node_call authorized ").into_owned()
                        },
                        None => {
                            println!("NOTHING FOUND");
                            req.query.unwrap().clone()
                        }
                    };

                    Request::builder()
                        .query(new_query)
                        .operation_name(req.operation_name.unwrap().clone())
                        .variables(req.variables)
                        .extensions(req.extensions)
                        .build()
                }).expect("Problemas cambiando el query");

                println!("Request query again: {:?}", new_orig_request.body().query);
                //let (parts, old_body) = request.originating_request.into_parts();
                //let mut req =request.originating_request;
                // *req = new_gql_req;

                RouterRequest::from(new_orig_request)
            
            })



















            //.map_response(|response: RouterResponse<BoxStream<'static, ResponseBody>>|{
            //    println!("RouteService/response");
            //    //println!("response body: {:?}", response.response());
            //    //
            //    response.map(|res|{
            //        println!("Esta es la respuesta!! {:?}", res.data());

            //        res
            //    })
            // }
            //
            //
            //
            //
            //
            //

            // .rate_limit()
            // .checkpoint()
            // .timeout()
            .service(service)
            .boxed()
    }

    fn query_planning_service(
        &mut self,
        service: BoxService<QueryPlannerRequest, QueryPlannerResponse, BoxError>,
    ) -> BoxService<QueryPlannerRequest, QueryPlannerResponse, BoxError> {
        // This is the default implementation and does not modify the default service.
        // The trait also has this implementation, and we just provide it here for illustration.
        service
    }

    fn execution_service(
        &mut self,
        service: BoxService<
            ExecutionRequest,
            ExecutionResponse<BoxStream<'static, Response>>,
            BoxError,
        >,
    ) -> BoxService<ExecutionRequest, ExecutionResponse<BoxStream<'static, Response>>, BoxError>
    {
        //This is the default implementation and does not modify the default service.
        // The trait also has this implementation, and we just provide it here for illustration.
        service
    }

    // Called for each subgraph
    fn subgraph_service(
        &mut self,
        _name: &str,
        service: BoxService<SubgraphRequest, SubgraphResponse, BoxError>,
    ) -> BoxService<SubgraphRequest, SubgraphResponse, BoxError> {
        // Always use service builder to compose your plugins.
        // It provides off the shelf building blocks for your plugin.
        ServiceBuilder::new()
            // .map_request()
            // .map_response()
            // .rate_limit()
            // .checkpoint()
            // .timeout()
            .service(service)
            .boxed()
    }
}

// This macro allows us to use it in our plugin registry!
// register_plugin takes a group name, and a plugin name.
//
// In order to keep the plugin names consistent,
// we use using the `Reverse domain name notation`
register_plugin!("example", "hello_world", HelloWorld);

#[cfg(test)]
mod tests {
    use super::{Conf, HelloWorld};

    use apollo_router::plugin::test::IntoSchema::Canned;
    use apollo_router::plugin::test::PluginTestHarness;
    use apollo_router::plugin::Plugin;

    #[tokio::test]
    async fn plugin_registered() {
        apollo_router::plugin::plugins()
            .get("example.hello_world")
            .expect("Plugin not found")
            .create_instance(&serde_json::json!({"name" : "Bob"}))
            .await
            .unwrap();
    }

    // If we run this test as follows: cargo test -- --nocapture
    // we will see the message "Hello Bob" printed to standard out
    #[tokio::test]
    async fn display_message() {
        // Define a configuration to use with our plugin
        let conf = Conf {
            name: "Bob".to_string(),
        };

        // Build an instance of our plugin to use in the test harness
        let plugin = HelloWorld::new(conf).await.expect("created plugin");

        // Build a test harness. Usually we'd use this and send requests to
        // it, but in this case it's enough to build the harness to see our
        // output when our service registers.
        let _test_harness = PluginTestHarness::builder()
            .plugin(plugin)
            .schema(Canned)
            .build()
            .await
            .expect("building harness");
    }
}
///////////////////////////////////////



/*
use apollo_router::plugin::Plugin;
use apollo_router::{
    register_plugin, ExecutionRequest, ExecutionResponse, QueryPlannerRequest,
    QueryPlannerResponse, Response, ResponseBody, RouterRequest, RouterResponse, SubgraphRequest,
    SubgraphResponse //, Request
};
use futures::stream::BoxStream;
use schemars::JsonSchema;
use serde::Deserialize;
use tower::util::BoxService;
use tower::{BoxError, ServiceBuilder, ServiceExt};

//use regex::Regex;

#[derive(Debug)]
struct HelloWorld {
    #[allow(dead_code)]
    configuration: Conf,
}
*/

/*
#[derive(Debug, Default, Deserialize, JsonSchema)]
struct Conf {
    // Put your plugin configuration here. It will automatically be deserialized from JSON.
    name: String, // The name of the entity you'd like to say hello to
}
*/

/*

// This is a bare bones plugin that can be duplicated when creating your own.
#[async_trait::async_trait]
impl Plugin for HelloWorld {
    type Config = Conf;

    async fn new(configuration: Self::Config) -> Result<Self, BoxError> {
        Ok(HelloWorld { configuration })
    }

    fn router_service(
        &mut self,
        service: BoxService<
            RouterRequest,
            RouterResponse<BoxStream<'static, ResponseBody>>,
            BoxError,
        >,
    ) -> BoxService<RouterRequest, RouterResponse<BoxStream<'static, ResponseBody>>, BoxError> {
        // Say hello when our service is added to the router_service
        // stage of the router plugin pipeline.
        #[cfg(test)]
        println!("Hello {}", self.configuration.name);
        #[cfg(not(test))]
        tracing::info!("Hello {}", self.configuration.name);
        // Always use service builder to compose your plugins.
        // It provides off the shelf building blocks for your plugin.
        ServiceBuilder::new()
            .map_request(|request: RouterRequest|{
/*
                println!("Request part: {:?}", request.originating_request.body().query);
                println!("Request vars: {:?}", request.originating_request.body().variables);

                println!("Request vars: {:?}", request.originating_request.uri());


                let new_orig_request = request.originating_request.map(|req| {

                    let re = Regex::new(r"\s?query\s+(?P<operation_name>[\w_]+)\s?\{").unwrap();
                    let caps = re.captures("query HOLA_Mundo_query { field_a field_b }");
                    match caps {
                        Some(found) => println!("OPERATION NAME: {:?}", found.name("operation_name").unwrap().as_str()),
                        None => println!("NO SE ENCONTRO")
                    }

                    Request::builder()
                        .query("query getBooks{ books {id title}}".to_owned())
                        .operation_name("getBooks".to_owned())
                        .variables(req.variables)
                        .extensions(req.extensions)
                        .build()
                }).expect("Problemas cambiando el query");

                println!("Request query again: {:?}", new_orig_request.body().query);
                //let (parts, old_body) = request.originating_request.into_parts();
                //let mut req =request.originating_request;
                // *req = new_gql_req;

                RouterRequest::from(new_orig_request)
*/
                request
            })
        //
        //
        //
        //
        //
        //
        //
        //
        //
        //
        //
        //
        //
        //
        //
        //
        //
        //
        //
        //
             .map_response(|response: RouterResponse<BoxStream<'static, ResponseBody>>|{
                println!("RouteService/response");
                //println!("response body: {:?}", response.response());
                //
                response.map(|res|{
                    println!("Esta es la respuesta!! {:?}", res.data());

                    res
                })
             })
            // .rate_limit()
            // .checkpoint()
            // .timeout()
            .service(service)
            .boxed()
    }

    fn query_planning_service(
        &mut self,
        service: BoxService<QueryPlannerRequest, QueryPlannerResponse, BoxError>,
    ) -> BoxService<QueryPlannerRequest, QueryPlannerResponse, BoxError> {
        // This is the default implementation and does not modify the default service.
        // The trait also has this implementation, and we just provide it here for illustration.
        service
    }

    fn execution_service(
        &mut self,
        service: BoxService<
            ExecutionRequest,
            ExecutionResponse<BoxStream<'static, Response>>,
            BoxError,
        >,
    ) -> BoxService<ExecutionRequest, ExecutionResponse<BoxStream<'static, Response>>, BoxError>
    {
        //This is the default implementation and does not modify the default service.
        // The trait also has this implementation, and we just provide it here for illustration.
        service
    }

    // Called for each subgraph
    fn subgraph_service(
        &mut self,
        _name: &str,
        service: BoxService<SubgraphRequest, SubgraphResponse, BoxError>,
    ) -> BoxService<SubgraphRequest, SubgraphResponse, BoxError> {
        // Always use service builder to compose your plugins.
        // It provides off the shelf building blocks for your plugin.
        ServiceBuilder::new()
            // .map_request()
            // .map_response()
            // .rate_limit()
            // .checkpoint()
            // .timeout()
            .service(service)
            .boxed()
    }
}

// This macro allows us to use it in our plugin registry!
// register_plugin takes a group name, and a plugin name.
//
// In order to keep the plugin names consistent,
// we use using the `Reverse domain name notation`
register_plugin!("example", "hello_world", HelloWorld);

#[cfg(test)]
mod tests {
    use super::{Conf, HelloWorld};

    use apollo_router::utils::test::IntoSchema::Canned;
    use apollo_router::utils::test::PluginTestHarness;
    use apollo_router::Plugin;

    #[tokio::test]
    async fn plugin_registered() {
        apollo_router::plugins()
            .get("example.hello_world")
            .expect("Plugin not found")
            .create_instance(&serde_json::json!({"name" : "Bob"}))
            .await
            .unwrap();
    }

    // If we run this test as follows: cargo test -- --nocapture
    // we will see the message "Hello Bob" printed to standard out
    #[tokio::test]
    async fn display_message() {
        // Define a configuration to use with our plugin
        let conf = Conf {
            name: "Bob".to_string(),
        };

        // Build an instance of our plugin to use in the test harness
        let plugin = HelloWorld::new(conf).await.expect("created plugin");

        // Build a test harness. Usually we'd use this and send requests to
        // it, but in this case it's enough to build the harness to see our
        // output when our service registers.
        let _test_harness = PluginTestHarness::builder()
            .plugin(plugin)
            .schema(Canned)
            .build()
            .await
            .expect("building harness");
    }
}


*/



