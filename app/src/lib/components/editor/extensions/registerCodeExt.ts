import { all, createLowlight } from "lowlight";
import rust from "highlight.js/lib/languages/rust";
import css from "highlight.js/lib/languages/css";
import scss from "highlight.js/lib/languages/scss";
import xml from "highlight.js/lib/languages/xml";
import dart from "highlight.js/lib/languages/dart";
import java from "highlight.js/lib/languages/java";
import javascript from "highlight.js/lib/languages/javascript";
import typescript from "highlight.js/lib/languages/typescript";
import sql from "highlight.js/lib/languages/sql";
import go from "highlight.js/lib/languages/go";
import elixir from "highlight.js/lib/languages/elixir";
import dockerfile from "highlight.js/lib/languages/dockerfile";
import python from "highlight.js/lib/languages/python";
import json from "highlight.js/lib/languages/json";
import yaml from "highlight.js/lib/languages/yaml";
import c from "highlight.js/lib/languages/c";
import cpp from "highlight.js/lib/languages/cpp";
import csharp from "highlight.js/lib/languages/csharp";
import bash from "highlight.js/lib/languages/bash";

export const registeredLangs = [
	"css",
	"scss",
	"xml",
	"javascript",
	"typescript",
	"c",
	"cpp",
	"csharp",
	"sql",
	"yaml",
	"bash",
	"go",
	"elixir",
	"rust",
	"dockerfile",
	"dart",
	"python",
	"json",
	"java",
];

const lowlight = createLowlight(all);
lowlight.register("css", css);
lowlight.register("rust", rust);
lowlight.register("java", java);
lowlight.register("typescript", typescript);
lowlight.register("javascript", javascript);
lowlight.register("yaml", yaml);
lowlight.register("dockerfile", dockerfile);
lowlight.register("bash", bash);
lowlight.register("c", c);
lowlight.register("cpp", cpp);
lowlight.register("csharp", csharp);
lowlight.register("json", json);
lowlight.register("python", python);
lowlight.register("go", go);
lowlight.register("sql", sql);
lowlight.register("elixir", elixir);
lowlight.register("dart", dart);
lowlight.register("xml", xml);
lowlight.register("scss", scss);

export default lowlight;
