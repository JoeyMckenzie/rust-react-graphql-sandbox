import { ApolloClient, ApolloProvider, InMemoryCache } from "@apollo/client";
import { RouterProvider, createRouter } from "@tanstack/react-router";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";

// Import the generated route tree
import { routeTree } from "./routeTree.gen";

// Create a new router instance
const router = createRouter({ routeTree });

// Register the router instance for type safety
declare module "@tanstack/react-router" {
    interface Register {
        router: typeof router;
    }
}

const root = document.getElementById("root");
const client = new ApolloClient({
    uri: import.meta.env.VITE_API_BASE_URL,
    cache: new InMemoryCache(),
});

if (root) {
    createRoot(root).render(
        <StrictMode>
            <ApolloProvider client={client}>
                <RouterProvider router={router} />
            </ApolloProvider>
        </StrictMode>,
    );
}
