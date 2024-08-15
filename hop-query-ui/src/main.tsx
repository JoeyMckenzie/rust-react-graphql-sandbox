import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { ApolloClient, InMemoryCache, ApolloProvider } from "@apollo/client";
import App from "./App.tsx";
import "./index.css";

const root = document.getElementById("root");

const client = new ApolloClient({
    uri: import.meta.env.VITE_API_BASE_URL,
    cache: new InMemoryCache(),
});

if (root) {
    createRoot(root).render(
        <StrictMode>
            <ApolloProvider client={client}>
                <App />
            </ApolloProvider>
        </StrictMode>,
    );
}
