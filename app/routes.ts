import { type RouteConfig, index, layout } from "@react-router/dev/routes";

export default [
  layout("../src/core/layouts/FullLayout.tsx", [
    index("routes/home.tsx"),
  ]),
] satisfies RouteConfig;
