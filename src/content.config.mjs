import { defineCollection, z } from "astro:content";
import { file, glob } from "astro/loaders";

const projects = defineCollection({
  loader: file("src/content/projects.json"),
  schema: z.object({
    id: z.string(),
    name: z.string(),
    description: z.string(),
    status: z.enum(["Pre-alpha", "Alpha", "Beta", "Release"]),
  }),
});

const devlogs = defineCollection({
  loader: glob({ pattern: "**/devlogs/*.{md,mdx}", base: "./src/content" }),
  schema: z.object({
    title: z.string(),
    date: z.coerce.date(),
    project: z.string(),
  }),
});

export const collections = { projects, devlogs };
