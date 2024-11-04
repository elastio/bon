// https://vitepress.dev/guide/custom-theme
import type { Theme } from "vitepress";
import DefaultTheme from "vitepress/theme";
import CustomLayout from "./layouts/CustomLayout.vue";

import "./style.css";

// Code is based on the following blog post and its source code as well:
// https://ericgardner.info/notes/blogging-with-vitepress-january-2024
export default {
    extends: DefaultTheme,
    Layout: CustomLayout,
} satisfies Theme;
