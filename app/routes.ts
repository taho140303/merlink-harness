import { type RouteConfig, index, layout, route } from "@react-router/dev/routes";

export default [
  layout("../src/core/layouts/FullLayout.tsx", [
    index("routes/home.tsx"),
    route("chat/:id", "routes/chat.tsx"),
    route("project", "routes/project.tsx"),
    route("character", "routes/character.tsx"),
    route("worldbuilding/:sub", "routes/worldbuilding.tsx"),
    route("config", "routes/config.tsx"),
  ]),
] satisfies RouteConfig;
