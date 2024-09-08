import { all, createLowlight } from "lowlight";

export const registeredLangs = ["html", "css", "javascript", "typescript", "sql", "yaml", "bash", "go", "elixir", "rust", "docker", "lua", "coffeescript"];

const lowlight = createLowlight(all);
registeredLangs.map(async (lang) => {
    const module = await import(`highlight.js/lib/languages/${lang}`);
    lowlight.register(lang, module);
});

export default lowlight;