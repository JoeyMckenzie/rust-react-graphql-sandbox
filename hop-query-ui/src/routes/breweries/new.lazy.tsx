import { gql, useMutation } from "@apollo/client";
import { createLazyFileRoute, useNavigate } from "@tanstack/react-router";
import { type FormEventHandler, useState } from "react";

export const Route = createLazyFileRoute("/breweries/new")({
	component: AddBreweryForm,
});

const CREATE_BREWERY = gql`
	mutation CreateBrewery($input: CreateBreweryInput!) {
		createBrewery(input: $input) {
			id
			name
			location
			yearEstablished
			description
			website
			createdAt
			updatedAt
		}
	}
`;

export default function AddBreweryForm() {
	const [name, setName] = useState("");
	const [location, setLocation] = useState("");
	const [yearEstablished, setYearEstablished] = useState("");
	const [description, setDescription] = useState("");
	const [website, setWebsite] = useState("");
	const navigate = useNavigate();

	const [createBrewery, { data, loading, error }] = useMutation(CREATE_BREWERY);

	// Note, you're much better off using a *real* form library
	const handleSubmit: FormEventHandler<HTMLFormElement> = (e) => {
		e.preventDefault();

		createBrewery({
			variables: {
				input: {
					name,
					location,
					yearEstablished: yearEstablished
						? Number.parseInt(yearEstablished, 10)
						: null,
					description: description || null,
					website: website || null,
				},
			},
		}).then((result) => {
			if (!result.errors) {
				// Reset form fields after submission, normally all form hooks have some global reset options
				setName("");
				setLocation("");
				setYearEstablished("");
				setDescription("");
				setWebsite("");

				navigate({
					to: "/breweries",
				});
			}
		});
	};

	return (
		<div className="max-w-md mx-auto mt-10 bg-white p-8 border border-gray-300 rounded-lg shadow-lg">
			<h2 className="text-2xl font-bold mb-6 text-center">Add New Brewery</h2>
			<form onSubmit={handleSubmit} className="space-y-4">
				<div>
					<label
						htmlFor="name"
						className="block text-sm font-medium text-gray-700"
					>
						Brewery Name
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
						htmlFor="location"
						className="block text-sm font-medium text-gray-700"
					>
						Location
					</label>
					<input
						type="text"
						id="location"
						value={location}
						onChange={(e) => setLocation(e.target.value)}
						required
						className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
					/>
				</div>
				<div>
					<label
						htmlFor="yearEstablished"
						className="block text-sm font-medium text-gray-700"
					>
						Year Established
					</label>
					<input
						type="number"
						id="yearEstablished"
						value={yearEstablished}
						onChange={(e) => setYearEstablished(e.target.value)}
						min="1000"
						max={new Date().getFullYear()}
						className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
					/>
				</div>
				<div>
					<label
						htmlFor="description"
						className="block text-sm font-medium text-gray-700"
					>
						Description
					</label>
					<textarea
						id="description"
						value={description}
						onChange={(e) => setDescription(e.target.value)}
						rows={3}
						className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
					/>
				</div>
				<div>
					<label
						htmlFor="website"
						className="block text-sm font-medium text-gray-700"
					>
						Website
					</label>
					<input
						type="url"
						id="website"
						value={website}
						onChange={(e) => setWebsite(e.target.value)}
						className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
					/>
				</div>
				<div>
					<button
						type="submit"
						className="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
					>
						Add Brewery
					</button>
				</div>
			</form>
			{loading && (
				<p className="mt-4 text-center text-gray-600">Adding brewery...</p>
			)}
			{error && (
				<p className="mt-4 text-center text-red-600">Error: {error.message}</p>
			)}
			{data && (
				<p className="mt-4 text-center text-green-600">
					Brewery added successfully!
				</p>
			)}
		</div>
	);
}
