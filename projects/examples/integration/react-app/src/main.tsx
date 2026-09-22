import { createClient } from "@wae/client";
import { WaeProvider, useWae } from "@wae/adapter-react";
import { StrictMode, useState } from "react";
import { createRoot } from "react-dom/client";

const client = createClient({ server: { baseUrl: "/api" } });

function Panel() {
    const wae = useWae();
    const [label, setLabel] = useState("WAE + React");
    return (
        <main style={{ fontFamily: "system-ui", padding: "2rem" }}>
            <h1>{label}</h1>
            <button
                type="button"
                onClick={() => {
                    setLabel(`client ok · baseUrl=${wae.server ? "set" : "?"}`);
                }}
            >
                ping client
            </button>
        </main>
    );
}

const el = document.getElementById("root");
if (!el) throw new Error("#root missing");
createRoot(el).render(
    <StrictMode>
        <WaeProvider client={client}>
            <Panel />
        </WaeProvider>
    </StrictMode>,
);
