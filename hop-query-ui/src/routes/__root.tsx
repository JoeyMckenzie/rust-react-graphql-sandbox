import { Link, Outlet, createRootRoute } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";

export const Route = createRootRoute({
    component: () => (
        <>
            <div className="p-2 flex gap-2">
                <Link to="/" className="[&.active]:font-bold">
                    Home
                </Link>{" "}
                <Link to="/about" className="[&.active]:font-bold">
                    About
                </Link>
                <Link to="/beers" className="[&.active]:font-bold">
                    Beers
                </Link>
                <Link to="/breweries" className="[&.active]:font-bold">
                    Breweries
                </Link>
            </div>
            <hr />
            <Outlet />
            <TanStackRouterDevtools />
        </>
    ),
});
