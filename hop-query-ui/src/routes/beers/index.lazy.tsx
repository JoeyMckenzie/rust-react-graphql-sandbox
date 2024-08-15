import { createLazyFileRoute } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/beers/")({
    component: Beers,
});

function Beers() {
    return (
        <div className="p-2">
            <h3>Beers</h3>
        </div>
    );
}
