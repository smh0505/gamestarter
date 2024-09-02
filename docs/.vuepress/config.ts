import { viteBundler } from "@vuepress/bundler-vite";
import { defaultTheme } from "@vuepress/theme-default";
import { defineUserConfig } from "vuepress";

export default defineUserConfig({
  bundler: viteBundler({
    viteOptions: {
      server: { port: 9999 },
    },
  }),
  theme: defaultTheme(),
  base: "/gamestarter/",
});
