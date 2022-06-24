const { ApolloServer, gql } = require('apollo-server');
const { buildSubgraphSchema } = require('@apollo/subgraph');

const typeDefs = gql`
extend schema
    @link(url: "https://specs.apollo.dev/federation/v2.0", import: ["@key", "@external", "@requires"])


interface Node {
    id: ID!
    authorized: Boolean!
}

type Review implements Node {
    id: ID!
    authorized: Boolean!
    body: String
}

extend type Book @key(fields: "id") {
    id: ID! @external
    authorized: Boolean! @external
    reviews: [Review] @requires(fields: "authorized")
}

type Query {
    reviews: [Review]
}
`;

const reviews = [
    {
        id:         'Review:awakening1',
        authorized: false,
        book:       'Book:awakening',
        body:       'Review about Awakening'
    },
    {
        id:         'Review:glass1',
        authorized: true,
        book:       'Book:glass',
        body:       'Review about City of Glass'
    },
];

function by_id(orig_id){
    return  ((x) => {return x.id == orig_id});
}

const resolvers = {
    Book: {
        reviews: (obj, args, context) => {
            console.log("Book/reviews -------------");
            console.log(obj);

            return reviews.filter((review) => {return review.book == obj.id});
        }
    },
    Review: {
        __resolveReference(review){
            console.log("Review/__resolveReference -----------");
            console.log(review);

            return reviews.find(by_id(review.id));
        }

    },
    Query: {
        reviews: () => reviews
    }
};

const server = new ApolloServer({
    schema: buildSubgraphSchema([{typeDefs,resolvers}])});

server.listen(4002).then(({ url }) => {
    console.log(`🚀 Server ready at ${url}`);
});

