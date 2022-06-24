fn router_service(service) {
    let f = |response| {
        if type_of(response.body.data.node) == "map" && type_of(response.body.data.node.authorized) == "bool"{
            if response.body.data.node.authorized == false {
                response.body.data.node = ();
            }else{
                response.body.data.node.remove("authorized");
            }
        }else{  // // `authorized` field not requested or not a map, which is wrong anyway
            response.body.data.node = ();
        }

        let nodes_query = "nodes";

        if type_of(response.body.data[nodes_query]) == "array"{
            let collection = response.body.data[nodes_query];

            response.body.data[nodes_query] = collection.map(|node|
         
            if type_of(node) == "map" && type_of(node.authorized) == "bool"{
                if node.authorized == false {
                    ()
                }else{
                    node.remove("authorized");
                    node
                }
            }else{
                ()      // `authorized` field not requested or not a map, which is wrong anyway
            });
        }
    };

    service.map_response(f);
}
