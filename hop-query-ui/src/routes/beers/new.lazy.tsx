import { gql, useMutation } from "@apollo/client";
import { createLazyFileRoute } from "@tanstack/react-router";
import { type FormEventHandler, useState } from "react";

export const Route = createLazyFileRoute("/beers/new")({
    component: () => <div>Hello /beers/new!</div>,
});

const ADD_BEER = gql`
  mutation AddBeer($name: String!, $type: String!, $abv: Float!, $breweryId: ID!) {
    addBeer(name: $name, type: $type, abv: $abv, breweryId: $breweryId) {
      id
      name
      type
      abv
    }
  }
`;

export function AddBeerForm() {
    const [name, setName] = useState("");
    const [type, setType] = useState("");
    const [abv, setAbv] = useState("");
    const [breweryId, setBreweryId] = useState("");

    const [addBeer, { data, loading, error }] = useMutation(ADD_BEER);

    const handleSubmit: FormEventHandler<HTMLFormElement> = (e) => {
        e.preventDefault();

        addBeer({
            variables: { name, type, abv: Number.parseFloat(abv), breweryId },
        });

        // Reset values on success
        setName("");
        setType("");
        setAbv("");
        setBreweryId("");
    };

    return (
        <div className="max-w-md mx-auto mt-10 bg-white p-8 border border-gray-300 rounded-lg shadow-lg">
            <h2 className="text-2xl font-bold mb-6 text-center">
                Add New Beer
            </h2>
            <form onSubmit={handleSubmit} className="space-y-4">
                <div>
                    <label
                        htmlFor="name"
                        className="block text-sm font-medium text-gray-700"
                    >
                        Beer Name
                    </label>
                    <input
                        type="text"
                        id="name"
                        value={name}
                        onChange={(e) => setName(e.target.value)}
                        required
                        className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
                    />
                </div>
                <div>
                    <label
                        htmlFor="type"
                        className="block text-sm font-medium text-gray-700"
                    >
                        Beer Type
                    </label>
                    <input
                        type="text"
                        id="type"
                        value={type}
                        onChange={(e) => setType(e.target.value)}
                        required
                        className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
                    />
                </div>
                <div>
                    <label
                        htmlFor="abv"
                        className="block text-sm font-medium text-gray-700"
                    >
                        ABV (%)
                    </label>
                    <input
                        type="number"
                        id="abv"
                        value={abv}
                        onChange={(e) => setAbv(e.target.value)}
                        required
                        step="0.1"
                        min="0"
                        max="100"
                        className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
                    />
                </div>
                <div>
                    <label
                        htmlFor="breweryId"
                        className="block text-sm font-medium text-gray-700"
                    >
                        Brewery ID
                    </label>
                    <input
                        type="text"
                        id="breweryId"
                        value={breweryId}
                        onChange={(e) => setBreweryId(e.target.value)}
                        required
                        className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
                    />
                </div>
                <div>
                    <button
                        type="submit"
                        className="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                    >
                        Add Beer
                    </button>
                </div>
            </form>
            {loading && (
                <p className="mt-4 text-center text-gray-600">Adding beer...</p>
            )}
            {error && (
                <p className="mt-4 text-center text-red-600">
                    Error: {error.message}
                </p>
            )}
            {data && (
                <p className="mt-4 text-center text-green-600">
                    Beer added successfully!
                </p>
            )}
        </div>
    );
}
