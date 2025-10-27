import { defineCollection, z } from "astro:content";
import { file } from "astro/loaders";

const projects = defineCollection({
  loader: file("src/content/projects.json"),
  schema: z.object({
    id: z.string(),
    name: z.string(),
    description: z.string(),
    status: z.enum(["Pre-alpha", "Alpha", "Beta", "Release"]),
  }),
});

export const collections = { projects };
