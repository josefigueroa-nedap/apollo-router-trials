
fn router_service(service) {
    let f_request = |request| {
        eprint("router_service/request:");
        eprint(`${request.body}`);
        eprint("\n\n");

        //eprint(`${"abcXXXabcYYYabc".matches("abc").collect()}`);
        //response.body.data.books.name += " and router";
        //print(response.body.data.books.name);
    };

    let f_response = |response| {
        eprint("router_service/response:");
        eprint(`${response.body}`);
        eprint("\n\n");
        //response.body.data.books.name += " and router";
        //print(response.body.data.books.name);
    };
    // Map our response using our closure
    service.map_request(f_request);
    service.map_response(f_response);
}


/**
fn execution_service(service) {
    let f_request = |request| {
        eprint("execution_service/request:");
        eprint(`${request.body}`);
        eprint("\n\n");
        //response.body.data.books.name += " and router";
        //print(response.body.data.books.name);
    };

    let f_response = |response| {
        eprint("execution_service/response:");
        eprint(`${response.body}`);
        eprint("\n\n");
        //response.body.data.books.name += " and router";
        //print(response.body.data.books.name);
    };
    // Map our response using our closure
    service.map_request(f_request);
    service.map_response(f_response);
}
*/

/*
fn query_planner_service(service) {
    let f_request = |request| {
        eprint("requer_planner_service/request:");
        eprint(`${request.body}`);
        eprint("\n\n");
        //response.body.data.books.name += " and router";
        //print(response.body.data.books.name);
    };

    let f_response = |response| {
        eprint("query_planner_service/response:");
        eprint(`${response.body}`);
        eprint("\n\n");
        //response.body.data.books.name += " and router";
        //print(response.body.data.books.name);
    };
    // Map our response using our closure
    service.map_request(f_request);
    service.map_response(f_response);
}

*/
