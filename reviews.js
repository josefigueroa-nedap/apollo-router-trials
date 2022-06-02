const { ApolloServer, gql } = require('apollo-server');
const { buildSubgraphSchema } = require('@apollo/subgraph');

const typeDefs = gql`
extend schema
    @link(url: "https://specs.apollo.dev/federation/v2.0", import: ["@key", "@shareable", "@external"])


interface Node {
    id: ID!
}

type Review implements Node{
    id: ID!
    body: String
}

extend type Book @key(fields: "id") {
    id: ID! @external
    reviews: [Review]
}

type Query {
    reviews: [Review]
}
`;

const reviews = [
    {
        id:   'Review:awakening1',
        book: 'Book:awakening',
        body: 'Review about Awakening'
    },
    {
        id:   'Review:glass1',
        book: 'Book:glass',
        body: 'Review about City of Glass'
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

