import { gql, useQuery } from "@apollo/client";
import "./App.css";

type Brewery = {
    id: number;
    name: string;
    description: string;
    location: string;
    year_establised: number;
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

    return data?.breweries.map(({ id, name, description }) => (
        <div key={id}>
            <h3>{name}</h3>
            <br />
            <b>About this location:</b>
            <p>{description}</p>
            <br />
        </div>
    ));
}

export default function App() {
    return (
        <>
            <h2>My first Apollo app 🚀</h2>
            <br />
            <DisplayBreweries />
        </>
    );
}
