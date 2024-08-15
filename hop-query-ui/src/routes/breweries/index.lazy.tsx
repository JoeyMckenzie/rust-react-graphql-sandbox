import { gql, useQuery } from "@apollo/client";
import { Link, createLazyFileRoute } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/breweries/")({
    component: Breweries,
});

type Brewery = {
    id: number;
    name: string;
    description: string;
    location: string;
    yearEstablished: number;
};

const GET_BREWERIES = gql`
    query GetBreweries {
		breweries {
			id
			name
			description,
			location,
			yearEstablished
		}
    }
`;

function DisplayBreweries() {
    const { loading, error, data } = useQuery<{ breweries: Brewery[] }>(
        GET_BREWERIES,
    );

    if (loading) {
        return <p>Loading...</p>;
    }

    if (error) {
        return <p>Error : {error.message}</p>;
    }

    return (
        <div className="overflow-x-auto shadow-md sm:rounded-lg">
            <table className="w-full text-sm text-left text-zinc-500">
                <thead className="text-xs text-zinc-700 uppercase bg-zinc-50">
                    <tr>
                        <th scope="col" className="px-6 py-3">
                            ID
                        </th>
                        <th scope="col" className="px-6 py-3">
                            Name
                        </th>
                        <th scope="col" className="px-6 py-3">
                            Description
                        </th>
                        <th scope="col" className="px-6 py-3">
                            Location
                        </th>
                        <th scope="col" className="px-6 py-3">
                            Year Established
                        </th>
                    </tr>
                </thead>
                <tbody>
                    {data?.breweries.map(
                        ({
                            id,
                            name,
                            description,
                            location,
                            yearEstablished,
                        }) => (
                            <tr
                                key={`${id}-${yearEstablished}`}
                                className="bg-white border-b hover:bg-zinc-50"
                            >
                                <td className="px-6 py-4">{id}</td>
                                <td className="px-6 py-4 font-medium text-zinc-900 whitespace-nowrap">
                                    {name}
                                </td>
                                <td className="px-6 py-4">{description}</td>
                                <td className="px-6 py-4">{location}</td>
                                <td className="px-6 py-4">{yearEstablished}</td>
                            </tr>
                        ),
                    )}
                </tbody>
            </table>
        </div>
    );
}

function Breweries() {
    return (
        <div className="p-2 flex flex-col space-y-4">
            <h1 className="text-xl font-bold">Breweries</h1>
            <DisplayBreweries />
            <Link
                to="/breweries/new"
                className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded w-24 text-center"
                as="button"
            >
                New
            </Link>
        </div>
    );
}
