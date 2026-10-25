import { type RouteConfig, index, layout, route } from "@react-router/dev/routes";

export default [
  layout("layouts/FullLayout.tsx", [
    index("routes/home.tsx"),
    route("chat/:id", "routes/chat.tsx"),
    route("project", "routes/project.tsx"),
    route("project/:projectId", "routes/project-detail.tsx"),
    route("project/:projectId/chapter/:chapterId", "routes/chapter-detail.tsx"),
    route("character", "routes/character.tsx"),
    route("character/:characterId", "routes/character-detail.tsx"),
    route("worldbuilding/:sub", "routes/worldbuilding.tsx"),
    route("config", "routes/config.tsx"),
    route("recycle-bin", "routes/recycle-bin.tsx"),
  ]),
] satisfies RouteConfig;
