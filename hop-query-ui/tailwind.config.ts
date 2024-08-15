import forms from "@tailwindcss/forms";
import type { Config } from "tailwindcss";

const config = {
	content: ["./components/**/*.{ts,tsx}", "./src/**/*.{ts,tsx}"],
	plugins: [forms],
} satisfies Config;

export default config;
