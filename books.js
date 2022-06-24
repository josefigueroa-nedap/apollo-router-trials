const { ApolloServer, gql } = require('apollo-server');
const { buildSubgraphSchema } = require('@apollo/subgraph');

const typeDefs = gql`
extend schema
    @link(url: "https://specs.apollo.dev/federation/v2.0", import: ["@key"])

    interface Node {
        id: ID!
        authorized: Boolean!
    }

    type Author implements Node @key(fields: "id"){
        id: ID!
        authorized: Boolean!
        name: String
    }

    type Book implements Node @key(fields: "id") {
        id: ID!
        authorized: Boolean!
        title: String
        author: Author
    }

#    extend type Review @key(fields: "id"){
#        book: Book
#    }

    type Query {
        node(id: ID!): Node
        books: [Book]
    }
`;


const authors = [
    {
        id:         "Author:katechopin",
        authorized: false,
        name:       'Kate Chopin'
    },
    {
        id:         'Author:paulauster',
        authorized: true,
        name:       'Paul Auster'
    }
];

const books = [
    {
        id:         'Book:awakening',
        authorized: false,    // this should come from permits but just for the example
        title:      'The Awakening',
        author:     'Author:katechopin',
    },
    {
        id:         'Book:glass',
        authorized: true,
        title:      'City of Glass',
        author:     'Author:paulsauster',
    },
];

function parse_id(id){
    var parts = id.match(/(\w+):(\w+)/);

    if (parts){
        return {kind: parts[1], id: parts[2]};
    }
}

function by_id(orig_id){
    return  ((x) => {return x.id == orig_id});
}

const resolvers = {
    Node: {
        __resolveType(obj, context, info){
            console.log("Resolver type: -------------");
            console.log("ob", obj);
            //console.log("context", context);
            //console.log("info", info.fieldNodes[0]);

            const id = obj.id || info.fieldNodes[0].arguments[0].value.value;
            const parsed_id = parse_id(id);

            return parsed_id.kind;
        }
    },
    Author: {
        __resolveReference(author, { fetchAuthorById }){
            return fetchAuthorById(author.id);
        }

    },
    Book: {
        __resolveReference(book, { fetchBookById }){
            console.log("Book/__resolveReference --------------");
            console.log(book, fetchBookById);

            return fetchBookById(book.id);
        },
        author: (obj, args, context) => {
            console.log("Book/author -------------------");
            console.log(obj);

            return authors.find(by_id(obj.author));
        }
    },
    Query: {
        books: () => books,
        node(parent, args, context, info) { 
            console.log("parent: ", parent);
            console.log("args: ", args);
            console.log("context", context);
            console.log("------------------------");

            const id = parse_id(args.id);
            var catalog = null;

            switch(id.kind){
                case 'Book':
                    catalog = books;
                    break;
                case 'Author':
                    catalog = authors;
                    break;
            }

            if (catalog){
                return catalog.find(by_id(args.id));
            }
        }
    },
};

const server = new ApolloServer({
    schema: buildSubgraphSchema([{typeDefs,resolvers}])});

server.listen(4001).then(({ url }) => {
    console.log(`🚀 Server ready at ${url}`);
});

